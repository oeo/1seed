use hkdf::Hkdf;
use keyring::Entry;
use scrypt::{scrypt, Params};
use sha2::Sha256;
#[cfg(unix)]
use std::io::Write;
use std::path::{Path, PathBuf};
use zeroize::{Zeroize, Zeroizing};

const SCRYPT_R: u32 = 8;
const SCRYPT_P: u32 = 1;
const VERSION: &str = "v1";
pub(crate) const MAGIC: &[u8] = b"1SED2";
const MASTER_LEN: usize = 32;

pub struct Seed {
    master: Zeroizing<[u8; 32]>,
}

pub enum SeedSource {
    EnvFile(PathBuf),
    Keyring,
    DefaultFile(PathBuf),
}

impl Seed {
    fn default_file_path() -> PathBuf {
        std::env::var_os("HOME")
            .filter(|home| !home.is_empty())
            .map(PathBuf::from)
            .or_else(dirs::home_dir)
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".1seed")
    }

    pub fn from_passphrase(passphrase: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if passphrase.is_empty() {
            return Err("passphrase must not be empty".into());
        }
        let scrypt_n = if cfg!(test) { 12 } else { 20 };
        let params = Params::new(scrypt_n, SCRYPT_R, SCRYPT_P, 32)?;
        let mut master = Zeroizing::new([0u8; 32]);
        scrypt(passphrase.as_bytes(), b"1seed", &params, master.as_mut())?;
        Ok(Self { master })
    }

    pub fn master_from_passphrase(
        passphrase: &str,
    ) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        Ok(*Seed::from_passphrase(passphrase.trim())?.master)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if bytes.len() >= MASTER_LEN && bytes.iter().any(|&b| !(32..=127).contains(&b)) {
            // Text this long was historically read as a raw key, skipping scrypt.
            // A random key is almost never valid UTF-8, so refuse to guess.
            if std::str::from_utf8(bytes).is_ok() {
                return Err(
                    "ambiguous legacy seed: text of 32+ bytes with a control or \
                    non-ASCII character was read as a raw key. To keep the current keys, run \
                    'init --from-file' on its first 32 bytes; to use it as a passphrase \
                    (different keys), run 'init --passphrase'"
                        .into(),
                );
            }
            let mut master = Zeroizing::new([0u8; 32]);
            master.copy_from_slice(&bytes[..MASTER_LEN]);
            Ok(Self { master })
        } else {
            let passphrase = String::from_utf8_lossy(bytes);
            let passphrase = passphrase.trim();
            Self::from_passphrase(passphrase)
        }
    }

    fn decode_storage(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if bytes.starts_with(MAGIC) {
            if bytes.len() != MAGIC.len() + MASTER_LEN {
                return Err("stored seed is corrupt: wrong length".into());
            }
            let mut master = Zeroizing::new([0u8; 32]);
            master.copy_from_slice(&bytes[MAGIC.len()..]);
            return Ok(Self { master });
        }
        Self::from_bytes(bytes)
    }

    fn encode_master(master: &[u8; 32]) -> Vec<u8> {
        let mut out = Vec::with_capacity(MAGIC.len() + MASTER_LEN);
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(master);
        out
    }

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let bytes = std::fs::read(path)?;
        Self::decode_storage(&bytes)
    }

    pub fn load() -> Result<(Self, SeedSource), Box<dyn std::error::Error>> {
        if let Ok(path_str) = std::env::var("SEED_FILE") {
            let path = PathBuf::from(path_str);
            return Ok((Self::from_file(&path)?, SeedSource::EnvFile(path)));
        }

        Self::load_stored(Self::keyring_secret(), Self::default_file_path())
    }

    fn load_stored(
        keyring: Result<Option<Vec<u8>>, Box<dyn std::error::Error>>,
        default_file: PathBuf,
    ) -> Result<(Self, SeedSource), Box<dyn std::error::Error>> {
        let keyring = match keyring {
            Err(e) if default_file.exists() => {
                eprintln!("warning: {e}");
                eprintln!(
                    "warning: using {}, which may hold a different seed",
                    default_file.display()
                );
                None
            }
            other => other?,
        };

        if let Some(bytes) = keyring {
            let seed = Self::decode_storage(&bytes)?;
            // The Linux keyring (kernel keyutils) does not survive a reboot:
            // give seeds stored there by earlier versions a durable copy.
            if cfg!(target_os = "linux") && !default_file.exists() {
                let data = Self::encode_master(&seed.master);
                if let Err(e) = write_seed_file(&default_file, &data) {
                    eprintln!(
                        "warning: seed exists only in the kernel keyring and could not be copied to {}: {e}",
                        default_file.display()
                    );
                }
            }
            return Ok((seed, SeedSource::Keyring));
        }

        if default_file.exists() {
            return Ok((
                Self::from_file(&default_file)?,
                SeedSource::DefaultFile(default_file),
            ));
        }

        Err("no seed found, run '1seed init --generate'".into())
    }

    fn keyring_secret() -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        if std::env::var("SEED_NO_KEYRING").is_ok() {
            return Ok(None);
        }
        keyring_found(Entry::new("1seed", "master-seed").and_then(|e| e.get_secret())).map_err(
            |e| {
                format!(
                    "keyring unavailable ({e}); a seed may be stored there. \
                    Set SEED_NO_KEYRING=1 to ignore the keyring and use ~/.1seed only"
                )
                .into()
            },
        )
    }

    pub fn store(master: &[u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
        let data = Self::encode_master(master);
        // The Linux keyring (kernel keyutils) does not survive a reboot.
        let use_file_only = cfg!(target_os = "linux") || std::env::var("SEED_NO_KEYRING").is_ok();

        if use_file_only {
            write_seed_file(&Self::default_file_path(), &data)?;
        } else {
            Entry::new("1seed", "master-seed")
                .and_then(|e| e.set_secret(&data))
                .map_err(|e| {
                    format!(
                        "keyring unavailable ({e}); seed not stored. \
                        Set SEED_NO_KEYRING=1 to store it in ~/.1seed"
                    )
                })?;
        }
        Ok(())
    }

    pub fn remove() -> Result<(), Box<dyn std::error::Error>> {
        Self::remove_stored(
            keyring_found(Entry::new("1seed", "master-seed").and_then(|e| e.delete_credential())),
            &Self::default_file_path(),
        )
    }

    fn remove_stored(
        keyring: keyring::Result<Option<()>>,
        default_file: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let removed_file = default_file.exists();
        if removed_file {
            std::fs::remove_file(default_file)?;
        }
        // A write killed midway can leave the temp file holding the seed.
        let _ = std::fs::remove_file(default_file.with_extension("tmp"));

        match keyring {
            Err(e) => Err(format!("keyring unavailable ({e}); a seed may remain there").into()),
            Ok(None) if !removed_file => Err("no seed found to remove".into()),
            Ok(_) => Ok(()),
        }
    }

    pub fn exists() -> Result<bool, Box<dyn std::error::Error>> {
        Ok(std::env::var("SEED_FILE").is_ok()
            || Self::keyring_secret()?.is_some()
            || Self::default_file_path().exists())
    }

    pub fn derive(&self, realm: &str, key_type: &str, length: usize) -> Zeroizing<Vec<u8>> {
        let path = format!("{VERSION}/{realm}/{key_type}");
        let hk = Hkdf::<Sha256>::new(None, self.master.as_ref());
        let mut output = Zeroizing::new(vec![0u8; length]);
        hk.expand(path.as_bytes(), output.as_mut_slice())
            .expect("length should be valid");
        output
    }

    pub fn derive_32(&self, realm: &str, key_type: &str) -> Zeroizing<[u8; 32]> {
        let bytes = self.derive(realm, key_type, 32);
        let mut arr = Zeroizing::new([0u8; 32]);
        arr.copy_from_slice(&bytes);
        arr
    }
}

impl Drop for Seed {
    fn drop(&mut self) {
        self.master.zeroize();
    }
}

#[cfg(unix)]
pub(crate) fn write_secure(path: &Path, data: &[u8]) -> std::io::Result<()> {
    use std::fs::{OpenOptions, Permissions};
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)?;
    f.set_permissions(Permissions::from_mode(0o600))?;
    f.write_all(data)
}

#[cfg(not(unix))]
pub(crate) fn write_secure(path: &Path, data: &[u8]) -> std::io::Result<()> {
    std::fs::write(path, data)
}

/// Separates "no entry" from a keyring that could not be asked.
fn keyring_found<T>(result: keyring::Result<T>) -> keyring::Result<Option<T>> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(keyring::Error::NoEntry) => Ok(None),
        // Linux never stores the seed in the keyring; only a seed left by an
        // earlier version can be there.
        Err(_) if cfg!(target_os = "linux") => Ok(None),
        Err(e) => Err(e),
    }
}

/// Writes the seed beside its target and renames it into place, so the
/// target is either absent or complete.
fn write_seed_file(path: &Path, data: &[u8]) -> std::io::Result<()> {
    let tmp = path.with_extension("tmp");
    let result = write_secure(&tmp, data)
        .and_then(|()| {
            std::fs::OpenOptions::new()
                .write(true)
                .open(&tmp)?
                .sync_all()
        })
        .and_then(|()| std::fs::rename(&tmp, path));
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_derivation() {
        let seed1 = Seed::from_passphrase("test passphrase").unwrap();
        let seed2 = Seed::from_passphrase("test passphrase").unwrap();

        let key1 = seed1.derive("realm", "type", 32);
        let key2 = seed2.derive("realm", "type", 32);

        assert_eq!(key1.as_slice(), key2.as_slice());
    }

    #[test]
    fn different_realms_different_keys() {
        let seed = Seed::from_passphrase("test").unwrap();

        let key1 = seed.derive("realm1", "age", 32);
        let key2 = seed.derive("realm2", "age", 32);

        assert_ne!(key1.as_slice(), key2.as_slice());
    }

    #[test]
    fn different_types_different_keys() {
        let seed = Seed::from_passphrase("test").unwrap();

        let key1 = seed.derive("realm", "age", 32);
        let key2 = seed.derive("realm", "ssh", 32);

        assert_ne!(key1.as_slice(), key2.as_slice());
    }

    #[test]
    fn different_passphrases_different_keys() {
        let seed1 = Seed::from_passphrase("passphrase one").unwrap();
        let seed2 = Seed::from_passphrase("passphrase two").unwrap();

        let key1 = seed1.derive("realm", "age", 32);
        let key2 = seed2.derive("realm", "age", 32);

        assert_ne!(key1.as_slice(), key2.as_slice());
    }

    #[test]
    fn derive_various_lengths() {
        let seed = Seed::from_passphrase("test").unwrap();

        for len in [16, 32, 48, 64] {
            let key = seed.derive("realm", "type", len);
            assert_eq!(key.len(), len);
        }
    }

    #[test]
    fn from_bytes_binary_uses_raw() {
        // 32 bytes with a non-ascii byte should be treated as raw binary
        let mut raw = [0u8; 32];
        raw[0] = 0xFF;
        let seed = Seed::from_bytes(&raw).unwrap();

        // derive something to prove it works
        let key = seed.derive("r", "t", 32);
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn from_bytes_ascii_uses_passphrase() {
        // pure ascii text should be treated as a passphrase
        let text = b"my passphrase";
        let seed_from_bytes = Seed::from_bytes(text).unwrap();
        let seed_from_passphrase = Seed::from_passphrase("my passphrase").unwrap();

        let key1 = seed_from_bytes.derive("r", "t", 32);
        let key2 = seed_from_passphrase.derive("r", "t", 32);
        assert_eq!(key1.as_slice(), key2.as_slice());
    }

    #[test]
    fn derive_32_matches_derive() {
        let seed = Seed::from_passphrase("test").unwrap();

        let from_derive = seed.derive("realm", "age", 32);
        let from_derive_32 = seed.derive_32("realm", "age");

        assert_eq!(from_derive.as_slice(), from_derive_32.as_slice());
    }

    #[test]
    fn encode_master_roundtrip() {
        let master = [0x5au8; 32];
        let blob = Seed::encode_master(&master);
        assert_eq!(blob.len(), MAGIC.len() + MASTER_LEN);
        assert!(blob.starts_with(MAGIC));

        let seed = Seed::decode_storage(&blob).unwrap();
        assert_eq!(*seed.master, master);
    }

    #[test]
    fn decode_storage_truncated_magic_errors() {
        let blob = Seed::encode_master(&[0u8; 32]);
        assert!(Seed::decode_storage(&blob[..blob.len() - 1]).is_err());
    }

    #[test]
    fn decode_storage_magic_with_trailing_byte_errors() {
        let mut blob = Seed::encode_master(&[0u8; 32]);
        blob.push(b'\n');
        assert!(Seed::decode_storage(&blob).is_err());
    }

    #[test]
    fn decode_storage_empty_errors() {
        assert!(Seed::decode_storage(b"").is_err());
        assert!(Seed::decode_storage(b"  \n").is_err());
    }

    #[test]
    fn decode_storage_long_text_with_newline_errors() {
        // would otherwise load as its raw first 32 bytes, skipping scrypt
        assert!(Seed::decode_storage(b"correct horse battery staple and more words\n").is_err());
        // exactly 32 bytes: 31 characters plus the newline
        assert!(Seed::decode_storage(b"thirty-one character passphrase\n").is_err());
    }

    #[test]
    fn legacy_passphrase_parity() {
        // ASCII passphrase without header derives identical keys to pre-change behavior
        let legacy = Seed::from_bytes(b"my secret passphrase").unwrap();
        let new = Seed::decode_storage(b"my secret passphrase").unwrap();

        assert_eq!(
            legacy.derive("realm", "age", 32).as_slice(),
            new.derive("realm", "age", 32).as_slice()
        );
    }

    #[test]
    fn legacy_raw_binary_parity() {
        // raw >=32B with non-ascii, no header: same as before
        let mut raw = [0u8; 40];
        raw[0] = 0xFF;
        let legacy = Seed::from_bytes(&raw).unwrap();
        let new = Seed::decode_storage(&raw).unwrap();

        assert_eq!(
            legacy.derive("realm", "age", 32).as_slice(),
            new.derive("realm", "age", 32).as_slice()
        );
    }

    #[test]
    fn non_ascii_passphrase_uses_scrypt() {
        // h1 regression: a >=32B passphrase containing a non-ascii byte must NOT be
        // treated as raw binary master. init stores an encoded master derived via
        // scrypt. Verify the two representations agree:
        let pass = "correct horse battery staple and secure è"; // >= 32 bytes, non-ascii
        assert!(pass.len() >= 32);
        let master = Seed::master_from_passphrase(pass).unwrap();
        let blob = Seed::encode_master(&master);

        let via_scrypt = Seed::from_passphrase(pass.trim()).unwrap();
        let via_blob = Seed::decode_storage(&blob).unwrap();
        assert_eq!(*via_blob.master, *via_scrypt.master);
        // and the headerless text is rejected, not read as its raw first 32 bytes
        assert!(Seed::from_bytes(pass.as_bytes()).is_err());
    }

    #[test]
    fn write_seed_file_writes_exact_bytes_and_no_temp() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join(".1seed");
        let blob = Seed::encode_master(&[0x5au8; 32]);

        write_seed_file(&path, &blob).unwrap();

        assert_eq!(std::fs::read(&path).unwrap(), blob);
        assert!(!dir.path().join(".1seed.tmp").exists());
    }

    #[test]
    fn write_seed_file_failure_leaves_no_target() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join(".1seed");
        // the temp path is a directory, so the write cannot start
        std::fs::create_dir(dir.path().join(".1seed.tmp")).unwrap();

        assert!(write_seed_file(&path, &Seed::encode_master(&[0u8; 32])).is_err());
        assert!(!path.exists());
    }

    #[test]
    fn load_stored_corrupt_keyring_entry_errors() {
        // must not fall through to a valid file
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join(".1seed");
        std::fs::write(&path, Seed::encode_master(&[1u8; 32])).unwrap();
        let mut corrupt = Seed::encode_master(&[2u8; 32]);
        corrupt.push(0);

        assert!(Seed::load_stored(Ok(Some(corrupt)), path).is_err());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn load_stored_copies_keyring_seed_to_missing_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join(".1seed");
        let blob = Seed::encode_master(&[3u8; 32]);

        let (seed, source) = Seed::load_stored(Ok(Some(blob.clone())), path.clone()).unwrap();

        assert_eq!(*seed.master, [3u8; 32]);
        assert!(matches!(source, SeedSource::Keyring));
        assert_eq!(std::fs::read(&path).unwrap(), blob);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn load_stored_returns_keyring_seed_when_copy_fails() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("missing").join(".1seed");
        let blob = Seed::encode_master(&[4u8; 32]);

        let (seed, _) = Seed::load_stored(Ok(Some(blob)), path).unwrap();

        assert_eq!(*seed.master, [4u8; 32]);
    }

    #[test]
    fn keyring_found_separates_no_entry_from_failure() {
        assert_eq!(keyring_found(Ok(1)).unwrap(), Some(1));
        assert_eq!(
            keyring_found::<u8>(Err(keyring::Error::NoEntry)).unwrap(),
            None
        );

        let failed = keyring_found::<u8>(Err(keyring::Error::PlatformFailure("locked".into())));
        if cfg!(target_os = "linux") {
            assert_eq!(failed.unwrap(), None);
        } else {
            assert!(matches!(failed, Err(keyring::Error::PlatformFailure(_))));
        }
    }

    fn seed_and_temp(dir: &tempfile::TempDir) -> (PathBuf, PathBuf) {
        let path = dir.path().join(".1seed");
        let temp = dir.path().join(".1seed.tmp");
        std::fs::write(&path, Seed::encode_master(&[6u8; 32])).unwrap();
        std::fs::write(&temp, b"partial").unwrap();
        (path, temp)
    }

    #[test]
    fn remove_stored_keyring_failure_removes_files_and_reports_it() {
        let dir = tempfile::TempDir::new().unwrap();
        let (path, temp) = seed_and_temp(&dir);

        let err = Seed::remove_stored(Err(keyring::Error::PlatformFailure("locked".into())), &path)
            .unwrap_err();

        assert!(err.to_string().contains("a seed may remain"));
        assert!(!path.exists());
        assert!(!temp.exists());
    }

    #[test]
    fn remove_stored_no_entry_removes_files() {
        let dir = tempfile::TempDir::new().unwrap();
        let (path, temp) = seed_and_temp(&dir);

        Seed::remove_stored(Ok(None), &path).unwrap();

        assert!(!path.exists());
        assert!(!temp.exists());
    }

    #[test]
    fn remove_stored_nothing_to_remove_errors() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join(".1seed");

        let err = Seed::remove_stored(Ok(None), &path).unwrap_err();

        assert_eq!(err.to_string(), "no seed found to remove");
    }

    #[test]
    fn remove_stored_keyring_entry_alone_is_removed() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join(".1seed");

        Seed::remove_stored(Ok(Some(())), &path).unwrap();
    }

    #[test]
    fn load_stored_keyring_failure_uses_existing_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join(".1seed");
        std::fs::write(&path, Seed::encode_master(&[5u8; 32])).unwrap();

        let (seed, source) = Seed::load_stored(Err("keyring unavailable".into()), path).unwrap();

        assert_eq!(*seed.master, [5u8; 32]);
        assert!(matches!(source, SeedSource::DefaultFile(_)));
    }

    #[test]
    fn load_stored_keyring_failure_without_file_reports_it() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join(".1seed");

        let err = Seed::load_stored(Err("keyring unavailable".into()), path)
            .err()
            .unwrap();

        assert_eq!(err.to_string(), "keyring unavailable");
    }
}
