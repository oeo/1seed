# 1seed

[![Rust](https://img.shields.io/badge/rust-%23E34F26.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/oneseed.svg?style=for-the-badge)](https://crates.io/crates/oneseed)
[![zread](https://img.shields.io/badge/Ask_Zread-_.svg?style=for-the-badge&color=00b0aa&labelColor=000000&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB3aWR0aD0iMTYiIGhlaWdodD0iMTYiIHZpZXdCb3g9IjAgMCAxNiAxNiIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTQuOTYxNTYgMS42MDAxSDIuMjQxNTZDMS44ODgxIDEuNjAwMSAxLjYwMTU2IDEuODg2NjQgMS42MDE1NiAyLjI0MDFWNC45NjAxQzEuNjAxNTYgNS4zMTM1NiAxLjg4ODEgNS42MDAxIDIuMjQxNTYgNS42MDAxSDQuOTYxNTZDNS4zMTUwMiA1LjYwMDEgNS42MDE1NiA1LjMxMzU2IDUuNjAxNTYgNC45NjAxVjIuMjQwMUM1LjYwMTU2IDEuODg2NjQgNS4zMTUwMiAxLjYwMDEgNC45NjE1NiAxLjYwMDFaIiBmaWxsPSIjZmZmIi8%2BCjxwYXRoIGQ9Ik00Ljk2MTU2IDEwLjM5OTlIMi4yNDE1NkMxLjg4ODEgMTAuMzk5OSAxLjYwMTU2IDEwLjY4NjQgMS42MDE1NiAxMS4wMzk5VjEzLjc1OTlDMS42MDE1NiAxNC4xMTM0IDEuODg4MSAxNC4zOTk5IDIuMjQxNTYgMTQuMzk5OUg0Ljk2MTU2QzUuMzE1MDIgMTQuMzk5OSA1LjYwMTU2IDE0LjExMzQgNS42MDE1NiAxMy43NTk5VjExLjAzOTlDNS42MDE1NiAxMC42ODY0IDUuMzE1MDIgMTAuMzk5OSA0Ljk2MTU2IDEwLjM5OTlaIiBmaWxsPSIjZmZmIi8%2BCjxwYXRoIGQ9Ik0xMy43NTg0IDEuNjAwMUgxMS4wMzg0QzEwLjY4NSAxLjYwMDEgMTAuMzk4NCAxLjg4NjY0IDEwLjM5ODQgMi4yNDAxVjQuOTYwMUMxMC4zOTg0IDUuMzEzNTYgMTAuNjg1IDUuNjAwMSAxMS4wMzg0IDUuNjAwMUgxMy43NTg0QzE0LjExMTkgNS42MDAxIDE0LjM5ODQgNS4zMTM1NiAxNC4zOTg0IDQuOTYwMVYyLjI0MDFDMTQuMzk4NCAxLjg4NjY0IDE0LjExMTkgMS42MDAxIDEzLjc1ODQgMS42MDAxWiIgZmlsbD0iI2ZmZiIvPgo8cGF0aCBkPSJNNCAxMkwxMiA0TDQgMTJaIiBmaWxsPSIjZmZmIi8%2BCjxwYXRoIGQ9Ik00IDEyTDEyIDQiIHN0cm9rZT0iI2ZmZiIgc3Ryb2tlLXdpZHRoPSIxLjUiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIvPgo8L3N2Zz4K&logoColor=ffffff)](https://zread.ai/oeo/1seed)

Deterministic cryptographic keys from a single seed.

## What This Does

- Derives age, SSH, and signing keys from one master secret
- Encrypts and decrypts files using age
- Signs and verifies data using Ed25519
- Generates site-specific passwords
- Generates BIP39 mnemonic phrases

## What This Does Not Do

- Store secrets (use files, encrypted with 1seed)
- Sync secrets (use git)
- Manage contacts (use a text file)
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
# Option 1: Use a seed file (recommended)
dd if=/dev/urandom bs=32 count=1 > ~/.seed
chmod 600 ~/.seed
1seed config set seed-file ~/.seed

# Option 2: Use a passphrase (brainwallet)
echo "your long memorable passphrase here" > ~/.seed
chmod 600 ~/.seed
1seed config set seed-file ~/.seed

# Option 3: No file, prompt every time
# (just don't set seed-file)

# Show your age public key
1seed pub

# Add SSH key to agent
1seed ssh-add

# Encrypt to self
echo "secret" | 1seed enc > secret.age

# Decrypt
1seed dec < secret.age

# Derive a password
1seed pw github.com
```

## Commands

### Key Export

```
1seed pub         Age public key
1seed priv        Age private key
1seed ssh         SSH private key
1seed ssh-pub     SSH public key
1seed sign-pub    Ed25519 signing public key
```

### Encryption

```
1seed enc [OPTIONS] [FILE]
  -R, --recipient KEY     Add recipient (repeatable)
  -F, --recipients-file   Add recipients from file (repeatable)
  -s, --self              Include self as recipient
  -p, --passphrase        Encrypt with passphrase
  -a, --armor             ASCII armor output
  -o, --output FILE       Output file

1seed dec [OPTIONS] [FILE]
  -k, --key FILE          Key file (instead of derived)
  -p, --passphrase        Decrypt with passphrase
  -o, --output FILE       Output file
```

Default: encrypt to self, decrypt with derived key.

### Signing

```
1seed sign [OPTIONS] [FILE]
  -o, --output FILE       Output file
  --binary                Binary output (default: base64)

1seed verify SIGNATURE [FILE]
  -k, --pubkey KEY        Public key (default: derived)
```

### Derivation

```
1seed pw [OPTIONS] SITE
  -l, --length N          Password length (default: 16)
  -n, --counter N         Rotation counter (default: 1)
  --no-symbols            Alphanumeric only
  --symbols SET           Symbol set (default: !@#$%^&*)

1seed raw [OPTIONS] PATH
  -l, --length N          Byte length (default: 32)
  --hex                   Output as hex (default)
  --base64                Output as base64
  --binary                Output as raw bytes

1seed mnemonic [OPTIONS]
  -w, --words N           Word count: 12/15/18/21/24 (default: 24)
```

### SSH Agent

```
1seed ssh-add [OPTIONS]
  -t, --lifetime SEC      Key lifetime
  -c, --confirm           Require confirmation
```

### Configuration

```
1seed config              List config values
1seed config set K V      Set value (realm, seed-file)
1seed config get K        Get value
1seed config path         Show config file path

1seed realms              List known realms
1seed realms add NAME     Track realm
1seed realms rm NAME      Untrack realm

1seed info                Show status and public keys
```

## Realms

Realms namespace all derived keys. Same seed, different realm = different keys.

```bash
1seed -r personal pub     # Personal age key
1seed -r work pub         # Work age key (different)
1seed -r work ssh-add     # Work SSH key

# Track realms for reference
1seed realms add personal
1seed realms add work
1seed realms
```

Set a default:

```bash
1seed config set realm personal
```

## Password Rotation

When a password is compromised:

```bash
1seed pw github.com -n 2   # Increment counter
```

Same site, different counter = different password.

## Backup

Your backup is the seed file (32 bytes) or passphrase.

```bash
# Backup seed file
cp ~/.seed /secure/backup/location/

# Or memorize a passphrase
echo "correct horse battery staple piano umbrella" > ~/.seed
```

From this, everything derives deterministically:
- Same seed + same realm = same keys (always)
- Different seeds or realms = different keys (always)

## Security Notes

**Seed file:** 32 random bytes. Maximum entropy.

**Passphrase:** Processed through scrypt (N=2^20, r=8, p=1). Uses ~1GB RAM, takes ~1 second. Resists brute force, but use a strong passphrase (6+ random words).

**Memory:** Keys are zeroized when dropped.

**Mnemonic warning:** Deriving BIP39 phrases means your cryptocurrency keys share fate with your master seed. Consider using a dedicated realm and understand the risk.

## Examples

### Encrypt for team

```bash
# Collect public keys
cat > team.txt << EOF
age1alice...
age1bob...
$(1seed pub)
EOF

# Encrypt
1seed enc -F team.txt < secrets.json > secrets.json.age
```

### Sign a release

```bash
1seed sign release.tar.gz > release.tar.gz.sig
1seed sign-pub > signing-key.pub

# Others verify
1seed verify -k "$(cat signing-key.pub)" "$(cat release.tar.gz.sig)" release.tar.gz
```

### Multiple machines

```bash
# Machine A
1seed ssh-pub >> ~/.ssh/authorized_keys

# Machine B (same seed)
1seed ssh-add
ssh user@machine-a  # works
```

## Environment Variables

```
SEED_FILE    Path to seed file
SEED_REALM   Default realm
```

## Files

Configuration file location (use `1seed config path` to see yours):
```
Linux:   ~/.config/1seed/config.toml
macOS:   ~/Library/Application Support/1seed/config.toml
Windows: %APPDATA%\1seed\config.toml
```

## License

MIT
