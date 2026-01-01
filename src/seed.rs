use hkdf::Hkdf;
use keyring::Entry;
use scrypt::{scrypt, Params};
use sha2::Sha256;
use std::path::Path;
use zeroize::{Zeroize, Zeroizing};

const SCRYPT_R: u32 = 8;
const SCRYPT_P: u32 = 1;
const VERSION: &str = "v1";

pub struct Seed {
    master: Zeroizing<[u8; 32]>,
}

impl Seed {
    pub fn from_passphrase(passphrase: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // production: N=20 (~1GB RAM, ~1 sec)
        // testing: N=12 (~4MB RAM, ~10ms) via ONESEED_TEST_MODE=1
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

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let bytes = std::fs::read(path)?;

        if bytes.len() >= 32 && bytes.iter().any(|&b| !(32..=127).contains(&b)) {
            // looks like binary data, use first 32 bytes
            let mut master = Zeroizing::new([0u8; 32]);
            master.copy_from_slice(&bytes[..32]);
            Ok(Self { master })
        } else {
            // treat as passphrase
            let passphrase = String::from_utf8_lossy(&bytes);
            let passphrase = passphrase.trim();
            Self::from_passphrase(passphrase)
        }
    }

    pub fn from_keyring() -> Result<Self, Box<dyn std::error::Error>> {
        if std::env::var("DEBUG").is_ok() {
            eprintln!("debug: attempting to read from keyring");
            eprintln!("debug: service='1seed', account='master-seed'");
        }

        let entry = Entry::new("1seed", "master-seed")?;
        let bytes = entry.get_secret()?;

        if std::env::var("DEBUG").is_ok() {
            eprintln!(
                "debug: successfully read {} bytes from keyring",
                bytes.len()
            );
        }

        if bytes.len() >= 32 && bytes.iter().any(|&b| !(32..=127).contains(&b)) {
            // binary seed (32 bytes)
            if bytes.len() < 32 {
                return Err("seed in keyring is too short (need at least 32 bytes)".into());
            }
            let mut master = Zeroizing::new([0u8; 32]);
            master.copy_from_slice(&bytes[..32]);
            Ok(Self { master })
        } else {
            // passphrase stored in keyring
            let passphrase = String::from_utf8_lossy(&bytes);
            let passphrase = passphrase.trim();
            Self::from_passphrase(passphrase)
        }
    }

    pub fn store_in_keyring(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let entry = Entry::new("1seed", "master-seed")?;
        entry.set_secret(data)?;

        // debug: verify entry was created
        if std::env::var("DEBUG").is_ok() {
            eprintln!("debug: stored {} bytes to keyring", data.len());
            eprintln!("debug: service='1seed', account='master-seed'");
        }

        Ok(())
    }

    pub fn remove_from_keyring() -> Result<(), Box<dyn std::error::Error>> {
        let entry = Entry::new("1seed", "master-seed")?;
        entry.delete_credential()?;
        Ok(())
    }

    pub fn keyring_exists() -> bool {
        Entry::new("1seed", "master-seed")
            .and_then(|e| e.get_secret())
            .is_ok()
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
