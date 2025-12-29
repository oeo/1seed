use crate::seed::Seed;
use std::io::{Read, Write};
use std::path::Path;

pub fn derive_recipient(seed: &Seed, realm: &str) -> String {
    let raw = seed.derive_32(realm, "age");
    let secret = x25519_dalek::StaticSecret::from(*raw);
    let public = x25519_dalek::PublicKey::from(&secret);

    // encode recipient using age's format
    use bech32::{ToBase32, Variant};
    let data = public.as_bytes().to_base32();
    bech32::encode("age", data, Variant::Bech32).expect("valid bech32")
}

pub fn derive_identity(seed: &Seed, realm: &str) -> String {
    let raw = seed.derive_32(realm, "age");

    // encode identity using age's format
    use bech32::{ToBase32, Variant};
    let data = raw.to_base32();
    let encoded = bech32::encode("age-secret-key-", data, Variant::Bech32).expect("valid bech32");
    encoded.to_uppercase()
}

pub fn encrypt(
    recipients: Vec<Box<dyn ::age::Recipient + Send>>,
    armor: bool,
    input: Option<&Path>,
    output: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    if recipients.is_empty() {
        return Err("no recipients specified".into());
    }

    let plaintext = read_input(input)?;

    let encryptor =
        age::Encryptor::with_recipients(recipients).ok_or("failed to create encryptor")?;

    let mut ciphertext = vec![];

    if armor {
        let mut writer = encryptor.wrap_output(age::armor::ArmoredWriter::wrap_output(
            &mut ciphertext,
            age::armor::Format::AsciiArmor,
        )?)?;
        writer.write_all(&plaintext)?;
        writer.finish().and_then(|w| w.finish())?;
    } else {
        let mut writer = encryptor.wrap_output(&mut ciphertext)?;
        writer.write_all(&plaintext)?;
        writer.finish()?;
    }

    write_output(output, &ciphertext)?;
    Ok(())
}

pub fn encrypt_passphrase(
    passphrase: &str,
    armor: bool,
    input: Option<&Path>,
    output: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let plaintext = read_input(input)?;

    let encryptor = age::Encryptor::with_user_passphrase(passphrase.to_string().into());

    let mut ciphertext = vec![];

    if armor {
        let mut writer = encryptor.wrap_output(age::armor::ArmoredWriter::wrap_output(
            &mut ciphertext,
            age::armor::Format::AsciiArmor,
        )?)?;
        writer.write_all(&plaintext)?;
        writer.finish().and_then(|w| w.finish())?;
    } else {
        let mut writer = encryptor.wrap_output(&mut ciphertext)?;
        writer.write_all(&plaintext)?;
        writer.finish()?;
    }

    write_output(output, &ciphertext)?;
    Ok(())
}

pub fn decrypt(
    identity: &str,
    input: Option<&Path>,
    output: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let ciphertext = read_input(input)?;

    let identity: age::x25519::Identity = identity.parse()?;

    let decryptor = match age::Decryptor::new(&ciphertext[..])? {
        age::Decryptor::Recipients(d) => d,
        age::Decryptor::Passphrase(_) => return Err("encrypted with passphrase, use -p".into()),
    };

    let mut plaintext = vec![];
    let mut reader = decryptor.decrypt(std::iter::once(&identity as &dyn age::Identity))?;
    reader.read_to_end(&mut plaintext)?;

    write_output(output, &plaintext)?;
    Ok(())
}

pub fn decrypt_with_file(
    key_file: &Path,
    input: Option<&Path>,
    output: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let ciphertext = read_input(input)?;

    let key_content = std::fs::read_to_string(key_file)?;
    let identity: age::x25519::Identity = key_content.trim().parse()?;

    let decryptor = match age::Decryptor::new(&ciphertext[..])? {
        age::Decryptor::Recipients(d) => d,
        age::Decryptor::Passphrase(_) => return Err("encrypted with passphrase, use -p".into()),
    };

    let mut plaintext = vec![];
    let mut reader = decryptor.decrypt(std::iter::once(&identity as &dyn age::Identity))?;
    reader.read_to_end(&mut plaintext)?;

    write_output(output, &plaintext)?;
    Ok(())
}

pub fn decrypt_passphrase(
    passphrase: &str,
    input: Option<&Path>,
    output: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let ciphertext = read_input(input)?;

    let decryptor = match age::Decryptor::new(&ciphertext[..])? {
        age::Decryptor::Recipients(_) => return Err("not encrypted with passphrase".into()),
        age::Decryptor::Passphrase(d) => d,
    };

    let mut plaintext = vec![];
    let mut reader = decryptor.decrypt(&passphrase.to_string().into(), None)?;
    reader.read_to_end(&mut plaintext)?;

    write_output(output, &plaintext)?;
    Ok(())
}

pub fn parse_recipient(
    s: &str,
) -> Result<Box<dyn ::age::Recipient + Send>, Box<dyn std::error::Error>> {
    let recipient: age::x25519::Recipient = s.parse()?;
    Ok(Box::new(recipient))
}

pub fn parse_recipients_file(
    path: &Path,
) -> Result<Vec<Box<dyn ::age::Recipient + Send>>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let mut recipients: Vec<Box<dyn ::age::Recipient + Send>> = vec![];

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let recipient: age::x25519::Recipient = line.parse()?;
        recipients.push(Box::new(recipient));
    }

    Ok(recipients)
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

fn write_output(path: Option<&Path>, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    match path {
        Some(p) => Ok(std::fs::write(p, data)?),
        None => Ok(std::io::stdout().write_all(data)?),
    }
}
