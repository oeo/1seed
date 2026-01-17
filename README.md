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
- Generates site-specific passwords
- Generates BIP39 mnemonic phrases

## What This Does Not Do

- Store secrets (use files, encrypted with 1seed)
- Sync secrets (use git)
- Manage contacts (use a text file)
- Provide cryptographic isolation between credentials
- Replace hardware security keys for high-value assets
- Generate TOTP codes (time-based, not derivable)

## Installation

### Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/oeo/1seed/master/install.sh | bash
```

### From crates.io

```bash
cargo install oneseed
```

### From Source

```bash
cargo install --path .
```

Requires: `ssh-add` (for agent integration)

## Quick Start

```bash
# Initialize with a random seed (recommended)
1seed init --generate

# Or use a memorable passphrase
1seed init --passphrase

# Show your age public key
1seed age pub

# Add SSH key to agent
1seed ssh add

# Encrypt to self
echo "secret" | 1seed age encrypt > secret.age

# Decrypt
1seed age decrypt < secret.age

# Derive a password
1seed derive password github.com

# Check status
1seed status
```

## Seed Storage

**Priority**: `SEED_FILE` env → OS keyring → `~/.1seed` → error

Seeds are stored in OS-native credential stores when available:
- **macOS**: Keychain.app
- **Linux**: Secret Service (GNOME Keyring / KWallet)
- **Windows**: Credential Manager

If keyring is unavailable (headless servers, minimal installs), 1seed automatically falls back to `~/.1seed` (mode 0600).

### Environment Variables

```bash
# Use specific file (bypasses keyring and ~/.1seed)
SEED_FILE=/path/to/seed 1seed age pub

# Force file-only storage (bypass keyring)
SEED_NO_KEYRING=1 1seed init --generate

# Set default realm
export SEED_REALM=work
```

## Commands

### Initialization

```
1seed init [OPTIONS]          Store seed (keyring or ~/.1seed)
  -g, --generate              Generate random 32 bytes (recommended)
  -p, --passphrase            Use memorable passphrase
  --from-file PATH            Import from existing file

1seed forget --confirm        Remove seed from all storage
1seed status                  Show seed location and derived keys
```

### Age Encryption

```
1seed age pub                      Show age public key
1seed age key                      Show age private key
1seed age encrypt [OPTIONS] [FILE]
  -R, --recipient KEY     Add recipient (repeatable)
  -F, --recipients-file   Add recipients from file (repeatable)
  -s, --self              Include self as recipient
  -p, --passphrase        Encrypt with passphrase
  -a, --armor             ASCII armor output
  -o, --output FILE       Output file

1seed age decrypt [OPTIONS] [FILE]
  -k, --key FILE          Key file (instead of derived)
  -p, --passphrase        Decrypt with passphrase
  -o, --output FILE       Output file
```

Default: encrypt to self, decrypt with derived key.

### SSH Keys

```
1seed ssh pub             Show SSH public key
1seed ssh key             Show SSH private key
1seed ssh add [OPTIONS]   Add SSH key to agent
  -t, --lifetime SEC      Key lifetime
  -c, --confirm           Require confirmation
```

### Signing

```
1seed sign pub                 Show signing public key
1seed sign data [OPTIONS] [FILE]
  -o, --output FILE       Output file
  --binary                Binary output (default: base64)

1seed sign verify SIGNATURE [FILE]
  -k, --pubkey KEY        Public key (default: derived)
```

### Derivation

```
1seed derive password [OPTIONS] SITE
  -l, --length N          Password length (default: 16)
  -n, --counter N         Rotation counter (default: 1)
  --no-symbols            Alphanumeric only
  --symbols SET           Symbol set (default: !@#$%^&*)

1seed derive raw [OPTIONS] PATH
  -l, --length N          Byte length (default: 32)
  --hex                   Output as hex (default)
  --base64                Output as base64
  --binary                Output as raw bytes

1seed derive mnemonic [OPTIONS]
  -w, --words N           Word count: 12/15/18/21/24 (default: 24)
```

### Management

```
1seed update        Update to latest release from GitHub
  --check           Check for updates without installing
```

## Realms

Realms namespace all derived keys. Same seed, different realm = different keys.

Realms are derivation parameters, not security boundaries. Anyone with your seed can derive all realms. Use realms for organization, not isolation.

```bash
1seed --realm personal age pub     # Personal age key
1seed --realm work age pub         # Work age key (different)
1seed --realm work ssh add         # Work SSH key
```

Set a default realm:
```bash
export SEED_REALM=personal
```

## Password Rotation

When a password is compromised:

```bash
1seed derive password github.com -n 2   # Increment counter
```

Same site, different counter = different password.

## Backup

Your backup is the seed itself. Export it securely:

```bash
# Option 1: From keyring
# macOS: Keychain.app → search "1seed" → export
# Linux: secret-tool lookup service 1seed account master-seed
# Windows: Credential Manager → search "1seed"

# Option 2: From file (if using ~/.1seed)
cat ~/.1seed > backup.seed
chmod 600 backup.seed

# Option 3: Use a memorable passphrase
1seed init --passphrase
# Write down the passphrase securely
```

From the seed, everything derives deterministically:
- Same seed + same realm = same keys (always)
- Different seeds or realms = different keys (always)

## Security Model

### Single Point of Failure

The seed is a single point of failure by design. If your seed leaks, everything derived from it is compromised. This is the fundamental tradeoff.

Like a password manager, you trade N secrets for one well-protected secret. The difference:
- Password manager: one password protects N random secrets
- 1seed: one seed deterministically generates N secrets

If the seed is compromised, you must rotate everything. You cannot rotate individual credentials independently.

### When This Model Works

- You trust OS keychain security (hardware-backed where available)
- You need reproducible keys across machines without syncing state
- You understand "one seed compromised = rotate everything"
- Your threat model accepts trading N secrets for 1 well-protected seed
- You're managing development credentials or personal infrastructure

### When This Model Fails

- You need to rotate individual credentials without rotating all
- You need cryptographic isolation between credentials (use separate seeds)
- You're managing production secrets at scale
- You cannot tolerate total blast radius on compromise

### Implementation Details

Keyring storage uses OS-native credential stores with hardware encryption support where available (Secure Enclave on macOS, TPM on Windows/Linux).

Passphrases are processed through scrypt (N=2^20, r=8, p=1), using ~1GB RAM and taking ~1 second. Use a strong passphrase (6+ random words).

Keys are zeroized in memory when dropped, not just freed.

Mnemonic derivation means your cryptocurrency keys share the fate of your master seed. If the seed leaks, your coins are at risk.

## Examples

### Encrypt for team

```bash
# Collect public keys
cat > team.txt << EOF
age1alice...
age1bob...
$(1seed age pub)
EOF

# Encrypt
1seed age encrypt -F team.txt < secrets.json > secrets.json.age
```

### Sign a release

```bash
1seed sign data release.tar.gz > release.tar.gz.sig
1seed sign pub > signing-key.pub

# Others verify
1seed sign verify -k "$(cat signing-key.pub)" "$(cat release.tar.gz.sig)" release.tar.gz
```

### Multiple machines

```bash
# Machine A
1seed ssh pub >> ~/.ssh/authorized_keys

# Machine B (same seed in keychain)
1seed ssh add
ssh user@machine-a  # works
```

## Version History

- **v0.5.0**: Auto-fallback to ~/.1seed, SEED_NO_KEYRING env var
- **v0.4.0**: Keyring-only storage, removed config file
- **v0.3.0**: Self-update command, simplified config
- **v0.2.0**: Domain-based namespaces (`age`, `ssh`, `sign`, `derive`)
- **v0.1.0**: Initial release

## License

MIT
