use crate::seed::Seed;
use zeroize::Zeroizing;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ"; // 24 chars, no I/O
const LOWER: &[u8] = b"abcdefghjkmnpqrstuvwxyz"; // 24 chars, no i/l/o
const DIGIT: &[u8] = b"23456789"; // 8 chars, no 0/1
const SYMBOL: &[u8] = b"!@#$%^&*"; // 8 chars

pub fn derive(
    seed: &Seed,
    realm: &str,
    site: &str,
    counter: u32,
    length: usize,
    use_symbols: bool,
    symbols: &str,
) -> Result<Zeroizing<String>, Box<dyn std::error::Error>> {
    if length < 4 {
        return Err("password length must be at least 4".into());
    }
    if length > 128 {
        return Err("password length must be at most 128".into());
    }

    let key_type = format!("pw/{site}/{counter}");
    let raw = seed.derive(realm, &key_type, length * 2);

    let symbol_set = if use_symbols {
        if symbols.is_empty() {
            SYMBOL
        } else {
            symbols.as_bytes()
        }
    } else {
        &[]
    };

    let charset: Vec<u8> = if use_symbols {
        [UPPER, LOWER, DIGIT, symbol_set].concat()
    } else {
        [UPPER, LOWER, DIGIT].concat()
    };

    // rejection sampling for uniform distribution
    let max_valid = 256 - (256 % charset.len());

    let mut password: Vec<u8> = raw
        .iter()
        .filter(|&&b| (b as usize) < max_valid)
        .take(length)
        .map(|&b| charset[b as usize % charset.len()])
        .collect();

    // if we didn't get enough (rare), fall back to simple modulo
    while password.len() < length {
        let idx = password.len();
        password.push(charset[raw[idx] as usize % charset.len()]);
    }

    // ensure requirements are met
    let fix = seed.derive(realm, &format!("pw/{site}/{counter}/fix"), 8);
    ensure_requirements(&mut password, &fix, use_symbols, symbol_set);

    Ok(Zeroizing::new(String::from_utf8(password)?))
}

fn ensure_requirements(pw: &mut [u8], fix: &[u8], use_symbols: bool, symbol_set: &[u8]) {
    let requirements: Vec<&[u8]> = if use_symbols {
        vec![UPPER, LOWER, DIGIT, symbol_set]
    } else {
        vec![UPPER, LOWER, DIGIT]
    };

    for (i, req) in requirements.iter().enumerate() {
        let has_req = pw.iter().any(|c| req.contains(c));
        if !has_req {
            let pos = fix[i * 2] as usize % pw.len();
            let char_idx = fix[i * 2 + 1] as usize % req.len();
            pw[pos] = req[char_idx];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_meets_requirements() {
        let seed = Seed::from_passphrase("test").unwrap();

        for i in 0..100 {
            let pw = derive(&seed, "realm", &format!("site{i}"), 1, 16, true, "").unwrap();

            assert!(pw.chars().any(|c| c.is_ascii_uppercase()), "missing uppercase");
            assert!(pw.chars().any(|c| c.is_ascii_lowercase()), "missing lowercase");
            assert!(pw.chars().any(|c| c.is_ascii_digit()), "missing digit");
            assert!(pw.chars().any(|c| "!@#$%^&*".contains(c)), "missing symbol");
        }
    }

    #[test]
    fn password_deterministic() {
        let seed = Seed::from_passphrase("test").unwrap();

        let pw1 = derive(&seed, "realm", "site", 1, 16, true, "").unwrap();
        let pw2 = derive(&seed, "realm", "site", 1, 16, true, "").unwrap();

        assert_eq!(pw1.as_str(), pw2.as_str());
    }

    #[test]
    fn counter_changes_password() {
        let seed = Seed::from_passphrase("test").unwrap();

        let pw1 = derive(&seed, "realm", "site", 1, 16, true, "").unwrap();
        let pw2 = derive(&seed, "realm", "site", 2, 16, true, "").unwrap();

        assert_ne!(pw1.as_str(), pw2.as_str());
    }
}
