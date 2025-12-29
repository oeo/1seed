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
