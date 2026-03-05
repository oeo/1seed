use crate::seed::Seed;
use ed25519_dalek::SigningKey;
use ssh_key::{LineEnding, PrivateKey};
use std::io::Write;
use std::process::{Command, Stdio};

pub fn derive_private(seed: &Seed, realm: &str) -> String {
    let raw = seed.derive_32(realm, "ssh");
    let signing_key = SigningKey::from_bytes(&raw);
    let verifying_key = signing_key.verifying_key();

    let keypair = ssh_key::private::Ed25519Keypair {
        public: ssh_key::public::Ed25519PublicKey::from(verifying_key),
        private: ssh_key::private::Ed25519PrivateKey::from(signing_key),
    };

    let private_key = PrivateKey::from(keypair);
    private_key
        .to_openssh(LineEnding::LF)
        .expect("valid key")
        .to_string()
}

pub fn derive_public(seed: &Seed, realm: &str) -> String {
    let raw = seed.derive_32(realm, "ssh");
    let signing_key = SigningKey::from_bytes(&raw);
    let verifying_key = signing_key.verifying_key();

    let ed25519_pubkey = ssh_key::public::Ed25519PublicKey::from(verifying_key);
    let public_key = ssh_key::PublicKey::from(ed25519_pubkey);
    format!(
        "{} 1seed:{}",
        public_key.to_openssh().expect("valid key"),
        realm
    )
}

pub fn add_to_agent(
    seed: &Seed,
    realm: &str,
    lifetime: Option<u32>,
    confirm: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let private_key = derive_private(seed, realm);

    let mut args = vec!["-".to_string()];

    if let Some(t) = lifetime {
        args.push("-t".to_string());
        args.push(t.to_string());
    }

    if confirm {
        args.push("-c".to_string());
    }

    let mut child = Command::new("ssh-add")
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .ok_or("failed to open stdin")?
        .write_all(private_key.as_bytes())?;

    let status = child.wait()?;

    if !status.success() {
        return Err("ssh-add failed".into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seed::Seed;

    #[test]
    fn public_key_format() {
        let seed = Seed::from_passphrase("test").unwrap();
        let pubkey = derive_public(&seed, "realm");
        assert!(pubkey.starts_with("ssh-ed25519 "));
        assert!(pubkey.contains("1seed:realm"));
    }

    #[test]
    fn public_key_deterministic() {
        let seed1 = Seed::from_passphrase("test").unwrap();
        let seed2 = Seed::from_passphrase("test").unwrap();
        assert_eq!(
            derive_public(&seed1, "realm"),
            derive_public(&seed2, "realm")
        );
    }

    #[test]
    fn private_key_openssh_format() {
        let seed = Seed::from_passphrase("test").unwrap();
        let privkey = derive_private(&seed, "realm");
        assert!(privkey.starts_with("-----BEGIN OPENSSH PRIVATE KEY-----"));
        assert!(privkey
            .trim_end()
            .ends_with("-----END OPENSSH PRIVATE KEY-----"));
    }

    #[test]
    fn different_realms_different_keys() {
        let seed = Seed::from_passphrase("test").unwrap();
        assert_ne!(derive_public(&seed, "a"), derive_public(&seed, "b"));
    }

    #[test]
    fn private_key_matches_public_key() {
        let seed = Seed::from_passphrase("test").unwrap();
        let privkey_str = derive_private(&seed, "realm");
        let pubkey_str = derive_public(&seed, "realm");

        // parse the private key and extract its public half
        let privkey = PrivateKey::from_openssh(&privkey_str).unwrap();
        let derived_pub = privkey.public_key().to_openssh().unwrap();

        // the public key string includes a comment, strip it for comparison
        let pubkey_no_comment = pubkey_str
            .split_whitespace()
            .take(2)
            .collect::<Vec<_>>()
            .join(" ");

        assert_eq!(derived_pub, pubkey_no_comment);
    }
}
