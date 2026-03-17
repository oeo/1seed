# 1seed

[![Rust](https://img.shields.io/badge/rust-%23E34F26.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/oneseed.svg?style=for-the-badge)](https://crates.io/crates/oneseed)

Deterministic cryptographic keys (and other derivations) from a single seed.

## What This Does

- Stores seed securely in OS keychain with automatic fallback to ~/.1seed
- Derives age, SSH, and signing keys from one master secret
- Encrypts and decrypts files using age
- Signs and verifies data using Ed25519
- Generates site-specific passwords, deterministic integers, UUIDs, and BIP39 mnemonics

## Installation

```bash
# From crates.io
cargo install oneseed

# From source
git clone https://github.com/oeo/1seed && cd 1seed
make install

# Generate man page and shell completions
make generate
```

## Quick Start

```bash
1seed init --generate              # Initialize master seed
1seed age pub                      # Show public key
1seed ssh add                      # Add SSH key to agent
echo "secret" | 1seed age encrypt  # Encrypt to self
1seed derive password github.com   # Generate password
1seed status                       # Check status
```

## Seed Storage

**Priority**: `SEED_FILE` env → OS keyring → `~/.1seed`.

1seed uses OS-native credential stores (Keychain, Secret Service, Credential Manager) where available. If unavailable, it automatically falls back to `~/.1seed` (mode 0600).

```bash
# Override seed location
SEED_FILE=/path/to/seed 1seed age pub

# Set default realm
export SEED_REALM=work
```

## Commands

### Initialization
- `1seed init [-g | -p | --from-file FILE]` : Store seed (generate random, passphrase, or file).
- `1seed forget --confirm` : Remove seed from all storage.
- `1seed status` : Show seed location and public keys.

### Age Encryption
- `1seed age [pub | key]` : Show keys.
- `1seed age encrypt [-s] [-a] [-o FILE]` : Encrypt (default to self).
- `1seed age decrypt [-k FILE] [-o FILE]` : Decrypt using derived key.

### SSH & Signing
- `1seed ssh [pub | key | add]` : SSH key management and agent integration.
- `1seed sign [pub | data | verify]` : Ed25519 signatures.

### Derivation
- `1seed derive password SITE [-l 16] [-n 1]` : Site-specific passwords.
- `1seed derive int PATH [--min 0] [--max 100]` : Deterministic integers.
- `1seed derive uuid PATH` : Deterministic UUIDs.
- `1seed derive mnemonic [-w 24]` : BIP39 word phrases.
- `1seed derive raw PATH [--hex | --base64]` : Deterministic raw bytes.

## Realms
Realms namespace all derived keys. Same seed, different realm = different keys.

```bash
1seed --realm work ssh add
export SEED_REALM=personal
```

## Security Model

**Single Point of Failure**: The seed is the "Master Key". If it leaks, everything derived from it is compromised. 1seed trades N secrets for one well-protected secret.

- **Storage**: Uses hardware-backed keychain where available (Secure Enclave, TPM).
- **KDF**: HKDF-SHA256 for derivation; scrypt for passphrases (~1GB RAM).
- **Memory**: Keys are zeroized when dropped.

## Version History

- **v0.7.0**: Upgraded age crate to 0.11, expanded age encryption test coverage
- **v0.6.0**: Removed self-update, added man page and fuzz targets
- **v0.5.2**: Code formatting fixes
- **v0.5.1**: Deterministic Integer and UUID derivation, improved documentation
- **v0.5.0**: Auto-fallback to ~/.1seed, SEED_NO_KEYRING env var
- **v0.4.0**: Keyring-only storage, removed config file
- **v0.3.0**: Simplified config
- **v0.2.0**: Domain-based namespaces (`age`, `ssh`, `sign`, `derive`)
- **v0.1.0**: Initial release

## License
MIT
