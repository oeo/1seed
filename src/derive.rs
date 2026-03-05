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

pub fn integer(
    seed: &Seed,
    realm: &str,
    path: &str,
    min: i64,
    max: i64,
) -> Result<i64, Box<dyn std::error::Error>> {
    if min > max {
        return Err("min must be less than or equal to max".into());
    }

    let range = (max as i128 - min as i128 + 1) as u128;

    // We need enough bytes to cover the range.
    // 8 bytes (u64) covers up to 1.8e19, which fits i64 range.
    // To be perfectly uniform via rejection sampling or unbiased modulo,
    // we take 8 bytes chunks.

    let key_type = format!("int/{path}");

    // We derive a long stream to have enough attempts for rejection sampling
    // 256 bytes = 32 attempts of u64, highly unlikely to fail all.
    let raw = seed.derive(realm, &key_type, 256);

    let chunk_size = 8;
    let limit = u64::MAX as u128 - (u64::MAX as u128 % range);

    for chunk in raw.chunks(chunk_size) {
        if chunk.len() < chunk_size {
            break;
        }

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(chunk);
        let val = u64::from_be_bytes(bytes) as u128;

        if val < limit {
            return Ok((min as i128 + (val % range) as i128) as i64);
        }
    }

    Err("failed to derive uniform integer (astronomically unlikely)".into())
}

pub fn uuid(seed: &Seed, realm: &str, path: &str) -> String {
    let key_type = format!("uuid/{path}");
    let mut bytes = *seed.derive_32(realm, &key_type); // derive 32, use 16

    // UUID v4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
    // Version 4 (random)
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    // Variant 1 (RFC 4122)
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    // Use the first 16 bytes
    let b = &bytes[0..16];

    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        b[0], b[1], b[2], b[3],
        b[4], b[5],
        b[6], b[7],
        b[8], b[9],
        b[10], b[11], b[12], b[13], b[14], b[15]
    )
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

    #[test]
    fn integer_deterministic() {
        let seed = Seed::from_passphrase("test").unwrap();
        let i1 = integer(&seed, "realm", "count", 0, 100).unwrap();
        let i2 = integer(&seed, "realm", "count", 0, 100).unwrap();
        assert_eq!(i1, i2);
    }

    #[test]
    fn integer_in_range() {
        let seed = Seed::from_passphrase("test").unwrap();
        for i in 0..100 {
            let val = integer(&seed, "realm", &format!("count{i}"), 10, 20).unwrap();
            assert!(val >= 10 && val <= 20);
        }
    }

    #[test]
    fn uuid_format() {
        let seed = Seed::from_passphrase("test").unwrap();
        let u = uuid(&seed, "realm", "id");
        assert_eq!(u.len(), 36);
        assert_eq!(u.chars().nth(14), Some('4')); // Version 4
        let variant = u.chars().nth(19).unwrap();
        assert!(variant == '8' || variant == '9' || variant == 'a' || variant == 'b'); // Variant 1

        let parts: Vec<&str> = u.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);
    }

    #[test]
    fn uuid_deterministic() {
        let seed = Seed::from_passphrase("test").unwrap();
        assert_eq!(uuid(&seed, "realm", "id"), uuid(&seed, "realm", "id"));
    }

    #[test]
    fn uuid_different_paths() {
        let seed = Seed::from_passphrase("test").unwrap();
        assert_ne!(uuid(&seed, "realm", "a"), uuid(&seed, "realm", "b"));
    }

    #[test]
    fn raw_deterministic() {
        let seed = Seed::from_passphrase("test").unwrap();
        let r1 = raw(&seed, "realm", "key", 32);
        let r2 = raw(&seed, "realm", "key", 32);
        assert_eq!(r1.as_slice(), r2.as_slice());
    }

    #[test]
    fn raw_respects_length() {
        let seed = Seed::from_passphrase("test").unwrap();
        for len in [1, 16, 32, 64, 128] {
            assert_eq!(raw(&seed, "realm", "key", len).len(), len);
        }
    }

    #[test]
    fn mnemonic_rejects_invalid_word_count() {
        let seed = Seed::from_passphrase("test").unwrap();
        assert!(mnemonic(&seed, "realm", 13).is_err());
        assert!(mnemonic(&seed, "realm", 0).is_err());
        assert!(mnemonic(&seed, "realm", 25).is_err());
    }

    #[test]
    fn integer_rejects_min_greater_than_max() {
        let seed = Seed::from_passphrase("test").unwrap();
        assert!(integer(&seed, "realm", "x", 100, 0).is_err());
    }

    #[test]
    fn integer_single_value_range() {
        let seed = Seed::from_passphrase("test").unwrap();
        let val = integer(&seed, "realm", "x", 42, 42).unwrap();
        assert_eq!(val, 42);
    }

    #[test]
    fn integer_negative_range() {
        let seed = Seed::from_passphrase("test").unwrap();
        let val = integer(&seed, "realm", "x", -100, -50).unwrap();
        assert!(val >= -100 && val <= -50);
    }
}
