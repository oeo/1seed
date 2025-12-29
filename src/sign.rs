use crate::seed::Seed;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use std::io::Read;
use std::path::Path;

pub fn derive_public(seed: &Seed, realm: &str) -> String {
    let raw = seed.derive_32(realm, "sign");
    let signing_key = SigningKey::from_bytes(&raw);
    let verifying_key = signing_key.verifying_key();

    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(verifying_key.as_bytes())
}

pub fn sign(seed: &Seed, realm: &str, input: Option<&Path>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let raw = seed.derive_32(realm, "sign");
    let signing_key = SigningKey::from_bytes(&raw);

    let data = read_input(input)?;
    let signature = signing_key.sign(&data);

    Ok(signature.to_bytes().to_vec())
}

pub fn verify(
    pubkey_b64: &str,
    signature: &[u8],
    input: Option<&Path>,
) -> Result<bool, Box<dyn std::error::Error>> {
    use base64::Engine;

    let pubkey_bytes = base64::engine::general_purpose::STANDARD.decode(pubkey_b64)?;
    let pubkey_array: [u8; 32] = pubkey_bytes
        .try_into()
        .map_err(|_| "invalid public key length")?;

    let sig_array: [u8; 64] = signature
        .try_into()
        .map_err(|_| "invalid signature length")?;

    let verifying_key = VerifyingKey::from_bytes(&pubkey_array)?;
    let signature = Signature::from_bytes(&sig_array);

    let data = read_input(input)?;

    Ok(verifying_key.verify(&data, &signature).is_ok())
}

fn read_input(path: Option<&Path>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match path {
        Some(p) => Ok(std::fs::read(p)?),
        None => {
            let mut buf = vec![];
            std::io::stdin().read_to_end(&mut buf)?;
            Ok(buf)
        }
    }
}
