use hkdf::Hkdf;
use keyring::Entry;
use scrypt::{scrypt, Params};
use sha2::Sha256;
use std::path::{Path, PathBuf};
use zeroize::{Zeroize, Zeroizing};

const SCRYPT_R: u32 = 8;
const SCRYPT_P: u32 = 1;
const VERSION: &str = "v1";

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
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".1seed")
    }

    pub fn from_passphrase(passphrase: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let scrypt_n = if std::env::var("ONESEED_TEST_MODE").is_ok() {
            12
        } else {
            20
        };
        let params = Params::new(scrypt_n, SCRYPT_R, SCRYPT_P, 32)?;
        let mut master = Zeroizing::new([0u8; 32]);
        scrypt(passphrase.as_bytes(), b"1seed", &params, master.as_mut())?;
        Ok(Self { master })
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if bytes.len() >= 32 && bytes.iter().any(|&b| !(32..=127).contains(&b)) {
            let mut master = Zeroizing::new([0u8; 32]);
            master.copy_from_slice(&bytes[..32]);
            Ok(Self { master })
        } else {
            let passphrase = String::from_utf8_lossy(bytes);
            let passphrase = passphrase.trim();
            Self::from_passphrase(passphrase)
        }
    }

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(&bytes)
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
        Self::from_bytes(&bytes)
    }

    pub fn store(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let use_file_only = std::env::var("SEED_NO_KEYRING").is_ok();

        if use_file_only {
            let path = Self::default_file_path();
            std::fs::write(&path, data)?;
            #[cfg(unix)]
            {
                use std::fs::Permissions;
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&path, Permissions::from_mode(0o600))?;
            }
            return Ok(());
        }

        match Entry::new("1seed", "master-seed").and_then(|e| e.set_secret(data)) {
            Ok(()) => Ok(()),
            Err(_) => {
                let path = Self::default_file_path();
                std::fs::write(&path, data)?;
                #[cfg(unix)]
                {
                    use std::fs::Permissions;
                    use std::os::unix::fs::PermissionsExt;
                    std::fs::set_permissions(&path, Permissions::from_mode(0o600))?;
                }
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
}
