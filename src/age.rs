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
    let mut ciphertext = read_input(input)?;

    let identity: age::x25519::Identity = identity.parse()?;

    // handle armored input by de-armoring first
    if ciphertext.starts_with(b"-----BEGIN AGE ENCRYPTED FILE-----") {
        let armored_reader = age::armor::ArmoredReader::new(&ciphertext[..]);
        ciphertext = std::io::Read::bytes(armored_reader).collect::<Result<Vec<u8>, _>>()?;
    }

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
    let mut ciphertext = read_input(input)?;

    let key_content = std::fs::read_to_string(key_file)?;
    let identity: age::x25519::Identity = key_content.trim().parse()?;

    // handle armored input by de-armoring first
    if ciphertext.starts_with(b"-----BEGIN AGE ENCRYPTED FILE-----") {
        let armored_reader = age::armor::ArmoredReader::new(&ciphertext[..]);
        ciphertext = std::io::Read::bytes(armored_reader).collect::<Result<Vec<u8>, _>>()?;
    }

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
    let mut ciphertext = read_input(input)?;

    // handle armored input by de-armoring first
    if ciphertext.starts_with(b"-----BEGIN AGE ENCRYPTED FILE-----") {
        let armored_reader = age::armor::ArmoredReader::new(&ciphertext[..]);
        ciphertext = std::io::Read::bytes(armored_reader).collect::<Result<Vec<u8>, _>>()?;
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seed::Seed;

    #[test]
    fn recipient_format() {
        let seed = Seed::from_passphrase("test").unwrap();
        let recipient = derive_recipient(&seed, "realm");
        assert!(recipient.starts_with("age1"));
    }

    #[test]
    fn identity_format() {
        let seed = Seed::from_passphrase("test").unwrap();
        let identity = derive_identity(&seed, "realm");
        assert!(identity.starts_with("AGE-SECRET-KEY-1"));
    }

    #[test]
    fn recipient_deterministic() {
        let seed1 = Seed::from_passphrase("test").unwrap();
        let seed2 = Seed::from_passphrase("test").unwrap();
        assert_eq!(
            derive_recipient(&seed1, "realm"),
            derive_recipient(&seed2, "realm")
        );
    }

    #[test]
    fn different_realms_different_recipients() {
        let seed = Seed::from_passphrase("test").unwrap();
        assert_ne!(derive_recipient(&seed, "a"), derive_recipient(&seed, "b"));
    }

    #[test]
    fn identity_roundtrips_to_recipient() {
        let seed = Seed::from_passphrase("test").unwrap();
        let identity = derive_identity(&seed, "realm");
        let recipient = derive_recipient(&seed, "realm");

        // parse the identity and derive its public key
        let parsed: ::age::x25519::Identity = identity.parse().unwrap();
        let derived_recipient = parsed.to_public().to_string();

        assert_eq!(recipient, derived_recipient);
    }

    #[test]
    fn encrypt_decrypt_in_memory() {
        let seed = Seed::from_passphrase("test").unwrap();
        let recipient_str = derive_recipient(&seed, "realm");
        let identity_str = derive_identity(&seed, "realm");

        let recipient = parse_recipient(&recipient_str).unwrap();
        let plaintext = b"hello world";

        // encrypt to a temp file
        let dir = tempfile::TempDir::new().unwrap();
        let ct_path = dir.path().join("ct");
        let pt_path = dir.path().join("pt");

        // write plaintext to a file for input
        let in_path = dir.path().join("in");
        std::fs::write(&in_path, plaintext).unwrap();

        encrypt(vec![recipient], false, Some(&in_path), Some(&ct_path)).unwrap();
        decrypt(&identity_str, Some(&ct_path), Some(&pt_path)).unwrap();

        assert_eq!(std::fs::read(&pt_path).unwrap(), plaintext);
    }

    #[test]
    fn parse_recipient_rejects_garbage() {
        assert!(parse_recipient("not-a-valid-recipient").is_err());
    }
}
