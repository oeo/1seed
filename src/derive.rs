use crate::seed::Seed;
use zeroize::Zeroizing;

pub fn raw(seed: &Seed, realm: &str, path: &str, length: usize) -> Zeroizing<Vec<u8>> {
    let key_type = format!("raw/{path}");
    seed.derive(realm, &key_type, length)
}

pub fn mnemonic(
    seed: &Seed,
    realm: &str,
    words: usize,
) -> Result<Zeroizing<String>, Box<dyn std::error::Error>> {
    let entropy_bytes = match words {
        12 => 16, // 128 bits
        15 => 20, // 160 bits
        18 => 24, // 192 bits
        21 => 28, // 224 bits
        24 => 32, // 256 bits
        _ => return Err("word count must be 12, 15, 18, 21, or 24".into()),
    };

    let entropy = seed.derive(realm, "mnemonic", entropy_bytes);
    let mnemonic = bip39::Mnemonic::from_entropy(&entropy)?;

    Ok(Zeroizing::new(mnemonic.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mnemonic_deterministic() {
        let seed = Seed::from_passphrase("test").unwrap();

        let m1 = mnemonic(&seed, "realm", 24).unwrap();
        let m2 = mnemonic(&seed, "realm", 24).unwrap();

        assert_eq!(m1.as_str(), m2.as_str());
    }

    #[test]
    fn mnemonic_valid_words() {
        let seed = Seed::from_passphrase("test").unwrap();

        for words in [12, 15, 18, 21, 24] {
            let m = mnemonic(&seed, "realm", words).unwrap();
            assert_eq!(m.split_whitespace().count(), words);
        }
    }
}
