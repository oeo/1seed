# 1seed

[![Rust](https://img.shields.io/badge/rust-%23E34F26.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/oneseed.svg?style=for-the-badge)](https://crates.io/crates/oneseed)
[![zread](https://img.shields.io/badge/Ask_Zread-_.svg?style=for-the-badge&color=00b0aa&labelColor=000000&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB3aWR0aD0iMTYiIGhlaWdodD0iMTYiIHZpZXdCb3g9IjAgMCAxNiAxNiIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTQuOTYxNTYgMS42MDAxSDIuMjQxNTZDMS44ODgxIDEuNjAwMSAxLjYwMTU2IDEuODg2NjQgMS42MDE1NiAyLjI0MDFWNC45NjAxQzEuNjAxNTYgNS4zMTM1NiAxLjg4ODEgNS42MDAxIDIuMjQxNTYgNS42MDAxSDQuOTYxNTZDNS4zMTUwMiA1LjYwMDEgNS42MDE1NiA1LjMxMzU2IDUuNjAxNTYgNC45NjAxVjIuMjQwMUM1LjYwMTU2IDEuODg2NjQgNS4zMTUwMiAxLjYwMDEgNC45NjE1NiAxLjYwMDFaIiBmaWxsPSIjZmZmIi8%2BCjxwYXRoIGQ9Ik00Ljk2MTU2IDEwLjM5OTlIMi4yNDE1NkMxLjg4ODEgMTAuMzk5OSAxLjYwMTU2IDEwLjY4NjQgMS42MDE1NiAxMS4wMzk5VjEzLjc1OTlDMS42MDE1NiAxNC4xMTM0IDEuODg4MSAxNC4zOTk5IDIuMjQxNTYgMTQuMzk5OUg0Ljk2MTU2QzUuMzE1MDIgMTQuMzk5OSA1LjYwMTU2IDE0LjExMzQgNS42MDE1NiAxMy43NTk5VjExLjAzOTlDNS42MDE1NiAxMC42ODY0IDUuMzE1MDIgMTAuMzk5OSA0Ljk2MTU2IDEwLjM5OTlaIiBmaWxsPSIjZmZmIi8%2BCjxwYXRoIGQ9Ik0xMy43NTg0IDEuNjAwMUgxMS4wMzg0QzEwLjY4NSAxLjYwMDEgMTAuMzk4NCAxLjg4NjY0IDEwLjM5ODQgMi4yNDAxVjQuOTYwMUMxMC4zOTg0IDUuMzEzNTYgMTAuNjg1IDUuNjAwMSAxMS4wMzg0IDUuNjAwMUgxMy43NTg0QzE0LjExMTkgNS42MDAxIDE0LjM5ODQgNS4zMTM1NiAxNC4zOTg0IDQuOTYwMVYyLjI0MDFDMTQuMzk4NCAxLjg4NjY0IDE0LjExMTkgMS42MDAxIDEzLjc1ODQgMS42MDAxWiIgZmlsbD0iI2ZmZiIvPgo8cGF0aCBkPSJNNCAxMkwxMiA0TDQgMTJaIiBmaWxsPSIjZmZmIi8%2BCjxwYXRoIGQ9Ik00IDEyTDEyIDQiIHN0cm9rZT0iI2ZmZiIgc3Ryb2tlLXdpZHRoPSIxLjUiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIvPgo8L3N2Zz4K&logoColor=ffffff)](https://zread.ai/oeo/1seed)

Deterministic cryptographic keys from a single seed.

## What This Does

- Stores seed securely in OS keychain with automatic fallback to ~/.1seed
- Derives age, SSH, and signing keys from one master secret
- Encrypts and decrypts files using age
- Signs and verifies data using Ed25519
- Generates site-specific passwords, deterministic integers, UUIDs, and BIP39 mnemonics

## Installation

```bash
# Quick Install
curl -fsSL https://raw.githubusercontent.com/oeo/1seed/master/install.sh | bash

# From crates.io
cargo install oneseed
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
- `1seed init [-g | -p]` : Store seed (generate random or use passphrase).
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

### Management
- `1seed update [--check]` : Update to latest release from GitHub.

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

- **v0.5.1**: Deterministic Integer and UUID derivation, improved documentation
- **v0.5.0**: Auto-fallback to ~/.1seed, SEED_NO_KEYRING env var
- **v0.4.0**: Keyring-only storage, removed config file
- **v0.3.0**: Self-update command, simplified config
- **v0.2.0**: Domain-based namespaces (`age`, `ssh`, `sign`, `derive`)
- **v0.1.0**: Initial release

## License
MIT