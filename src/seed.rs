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
        let scrypt_n = if cfg!(test) || std::env::var("ONESEED_TEST_MODE").is_ok() {
            12
        } else {
            20
        };
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
        if bytes.len() == MAGIC.len() + MASTER_LEN && bytes.starts_with(MAGIC) {
            let mut master = Zeroizing::new([0u8; 32]);
            master.copy_from_slice(&bytes[MAGIC.len()..]);
            return Ok(Self { master });
        }
        if bytes.starts_with(MAGIC) && bytes.len() < MAGIC.len() + MASTER_LEN {
            return Err("seed file appears truncated".into());
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

        let use_file_only = std::env::var("SEED_NO_KEYRING").is_ok();

        if !use_file_only {
            if let Ok(seed) = Self::from_keyring() {
                return Ok((seed, SeedSource::Keyring));
            }
        }

        let default_file = Self::default_file_path();
        if default_file.exists() {
            return Ok((
                Self::from_file(&default_file)?,
                SeedSource::DefaultFile(default_file),
            ));
        }

        Err("no seed found, run '1seed init --generate'".into())
    }

    fn from_keyring() -> Result<Self, Box<dyn std::error::Error>> {
        let entry = Entry::new("1seed", "master-seed")?;
        let bytes = entry.get_secret()?;
        Self::decode_storage(&bytes)
    }

    pub fn store(master: &[u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
        let data = Self::encode_master(master);
        let use_file_only = std::env::var("SEED_NO_KEYRING").is_ok();

        if use_file_only {
            write_secure(&Self::default_file_path(), &data)?;
            return Ok(());
        }

        match Entry::new("1seed", "master-seed").and_then(|e| e.set_secret(&data)) {
            Ok(()) => Ok(()),
            Err(_) => {
                write_secure(&Self::default_file_path(), &data)?;
                Ok(())
            }
        }
    }

    pub fn remove() -> Result<(), Box<dyn std::error::Error>> {
        let mut removed_any = false;

        if let Ok(entry) = Entry::new("1seed", "master-seed") {
            if entry.delete_credential().is_ok() {
                removed_any = true;
            }
        }

        let default_file = Self::default_file_path();
        if default_file.exists() {
            std::fs::remove_file(&default_file)?;
            removed_any = true;
        }

        if removed_any {
            Ok(())
        } else {
            Err("no seed found to remove".into())
        }
    }

    pub fn exists() -> bool {
        if std::env::var("SEED_FILE").is_ok() {
            return true;
        }

        let use_file_only = std::env::var("SEED_NO_KEYRING").is_ok();

        if !use_file_only
            && Entry::new("1seed", "master-seed")
                .and_then(|e| e.get_secret())
                .is_ok()
        {
            return true;
        }

        Self::default_file_path().exists()
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
        // treated as raw binary master. decode_storage without a header routes through
        // the legacy sniff (still raw), but init stores an encoded master derived via
        // scrypt. Verify the two representations agree:
        let pass = "correct horse battery staple and secure è"; // >= 32 bytes, non-ascii
        assert!(pass.len() >= 32);
        let master = Seed::master_from_passphrase(pass).unwrap();
        let blob = Seed::encode_master(&master);

        let via_scrypt = Seed::from_passphrase(pass.trim()).unwrap();
        let via_blob = Seed::decode_storage(&blob).unwrap();
        assert_eq!(*via_blob.master, *via_scrypt.master);
        // and it must NOT equal the raw first-32-bytes interpretation
        let raw_master = Seed::from_bytes(pass.as_bytes()).unwrap();
        assert_ne!(*via_blob.master, *raw_master.master);
    }
}
