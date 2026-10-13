# 1seed

[![Rust](https://img.shields.io/badge/rust-%23E34F26.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/oneseed.svg?style=for-the-badge)](https://crates.io/crates/oneseed)

Deterministic cryptographic keys (and other derivations) from a single seed.

## What This Does

- Stores seed in the OS keychain (macOS, Windows) or in ~/.1seed (Linux, or with `SEED_NO_KEYRING=1`)
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

**Priority**: `--seed-file` flag → `SEED_FILE` env → OS keyring → `~/.1seed`.

On macOS and Windows, 1seed uses the OS-native credential store (Keychain, Credential Manager). If the store cannot be reached, `init` stops with an error, `forget` removes `~/.1seed` and then reports that a seed may remain in the store, and other commands read an existing `~/.1seed` with a warning. Set `SEED_NO_KEYRING=1` to store and read the seed in `~/.1seed` (mode 0600) only; `forget` still tries both places.

On Linux, `init` always stores the seed in `~/.1seed` (mode 0600). Earlier versions stored it in the kernel keyring, which does not survive a reboot; a seed still held there is copied to `~/.1seed` the next time it is loaded.

```bash
# Override seed location
SEED_FILE=/path/to/seed 1seed age pub

# Set default realm
export SEED_REALM=work
```

**Storage format**: `init` stores the 32-byte derived master seed prefixed with the magic bytes `1SED2`. The seed file is written mode 0600. Seeds written before this header existed are still readable: a seed with no header is interpreted as before (raw binary, or passphrase text), with three exceptions that now fail with an error instead of deriving keys:

- An empty seed, or an empty passphrase.
- A seed that starts with `1SED2` but is not exactly 37 bytes.
- UTF-8 text of 32 or more bytes that contains a control character (such as a trailing newline) or a non-ASCII character. Such a seed was previously read as a raw key from its first 32 bytes, skipping scrypt. To keep the same keys, run `1seed init --from-file` on a file holding those first 32 bytes. To use the text as a passphrase, run `1seed init --passphrase`; the derived keys will differ. `init` refuses while a seed exists or `SEED_FILE` is set: save a copy of the old seed first, then unset `SEED_FILE` and run `1seed forget --confirm` before `init`.

If the rejected seed is in the OS keychain and a good `~/.1seed` exists, set `SEED_NO_KEYRING=1` to use the file.

Passphrases always use the full scrypt cost. Earlier versions lowered it when the `ONESEED_TEST_MODE` environment variable was set. A passphrase seed file read with that variable set derives different keys now. A seed created by `init --passphrase` with that variable set keeps its keys, but was derived at the reduced cost and should be re-initialized.

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
- `1seed derive int PATH [--min 0] [--max 2147483647]` : Deterministic integers.
- `1seed derive uuid PATH` : Deterministic UUIDs.
- `1seed derive mnemonic [-w 24]` : BIP39 word phrases.
- `1seed derive raw PATH [-l 32] [--hex | --base64 | --binary]` : Deterministic raw bytes (max 8160).

## Realms
Realms namespace all derived keys. Same seed, different realm = different keys.

```bash
1seed --realm work ssh add
export SEED_REALM=personal
```

## Security Model

**Single Point of Failure**: The seed is the "Master Key". If it leaks, everything derived from it is compromised. 1seed trades N secrets for one well-protected secret.

- **Storage**: Uses the OS-native credential store on macOS and Windows (Keychain, Credential Manager), and a mode 0600 file on Linux. These are software credential stores, not hardware-backed (no Secure Enclave or TPM); treat the stored seed with the same care as a password file.
- **KDF**: HKDF-SHA256 for derivation; scrypt for passphrases (~1GB RAM).
- **Memory**: Derived keys are zeroized when dropped.

## Version History

- **v0.8.1**: Seed file written atomically; keyring failures are reported and no longer treated as a missing seed
- **v0.8.0**: Added `--seed-file`, upgraded age to 0.12, refreshed audited dependencies; Linux stores the seed in `~/.1seed`; malformed seeds are rejected
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
