use clap::{Parser, Subcommand};
use std::io::Write;
use std::path::PathBuf;

use crate::seed::Seed;
use crate::{age, derive, password, sign, ssh, update};

#[derive(Parser)]
#[command(name = "1seed")]
#[command(
    author,
    version,
    about = "Deterministic cryptographic keys from a single seed"
)]
#[command(after_help = "EXAMPLES:
    1seed init                       Store seed in OS keychain
    1seed age pub                    Show age public key
    1seed --realm work ssh add       Add work SSH key to agent
    echo secret | 1seed age encrypt  Encrypt to self
    1seed derive password github.com Derive password

ENVIRONMENT:
    SEED_FILE    Override keychain, use file instead (for testing/automation)
    SEED_REALM   Default realm (default: \"default\")

STORAGE:
    Seeds are stored in your OS keychain:
    - macOS: Keychain
    - Linux: Secret Service (GNOME Keyring / KWallet)
    - Windows: Credential Manager
")]
pub struct Cli {
    #[arg(long, global = true, env = "SEED_REALM")]
    pub realm: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Age encryption keys and operations
    Age {
        #[command(subcommand)]
        action: AgeAction,
    },

    /// SSH keys and operations
    Ssh {
        #[command(subcommand)]
        action: SshAction,
    },

    /// Ed25519 signing keys and operations
    Sign {
        #[command(subcommand)]
        action: SignAction,
    },

    /// Derive passwords, mnemonics, and raw bytes
    Derive {
        #[command(subcommand)]
        action: DeriveAction,
    },

    /// Initialize: store seed in OS keychain
    Init {
        #[arg(short, long)]
        passphrase: bool,

        #[arg(short, long)]
        generate: bool,

        #[arg(long)]
        from_file: Option<PathBuf>,
    },

    /// Remove seed from OS keychain
    Forget {
        #[arg(long)]
        confirm: bool,
    },

    /// Show status and derived keys
    Status,

    /// Update to latest release from GitHub
    Update {
        #[arg(long)]
        check: bool,
    },
}

#[derive(Subcommand)]
pub enum AgeAction {
    /// Show age public key
    Pub,

    /// Show age private key
    Key,

    /// Encrypt file with age
    Encrypt {
        #[arg(short = 'R', long = "recipient", action = clap::ArgAction::Append)]
        recipients: Vec<String>,

        #[arg(short = 'F', long = "recipients-file", action = clap::ArgAction::Append)]
        recipient_files: Vec<PathBuf>,

        #[arg(short, long)]
        self_: bool,

        #[arg(short, long)]
        passphrase: bool,

        #[arg(short, long)]
        armor: bool,

        #[arg(short, long)]
        output: Option<PathBuf>,

        file: Option<PathBuf>,
    },

    /// Decrypt file with age
    Decrypt {
        #[arg(short, long)]
        key: Option<PathBuf>,

        #[arg(short, long)]
        passphrase: bool,

        #[arg(short, long)]
        output: Option<PathBuf>,

        file: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
pub enum SshAction {
    /// Show SSH public key
    Pub,

    /// Show SSH private key
    Key,

    /// Add SSH key to agent
    Add {
        #[arg(short, long)]
        lifetime: Option<u32>,

        #[arg(short, long)]
        confirm: bool,
    },
}

#[derive(Subcommand)]
pub enum SignAction {
    /// Show signing public key
    Pub,

    /// Sign data
    Data {
        #[arg(short, long)]
        output: Option<PathBuf>,

        #[arg(long)]
        binary: bool,

        file: Option<PathBuf>,
    },

    /// Verify signature
    Verify {
        /// Signature (base64, or @file)
        signature: String,

        #[arg(short = 'k', long)]
        pubkey: Option<String>,

        file: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
pub enum DeriveAction {
    /// Derive password for site
    Password {
        site: String,

        #[arg(short, long, default_value = "16")]
        length: usize,

        #[arg(long)]
        no_symbols: bool,

        #[arg(long, default_value = "!@#$%^&*")]
        symbols: String,

        #[arg(short = 'n', long, default_value = "1")]
        counter: u32,
    },

    /// Derive BIP39 mnemonic
    Mnemonic {
        #[arg(short, long, default_value = "24")]
        words: usize,
    },

    /// Derive raw bytes
    Raw {
        path: String,

        #[arg(short, long, default_value = "32")]
        length: usize,

        #[arg(long)]
        hex: bool,

        #[arg(long)]
        base64: bool,

        #[arg(long)]
        binary: bool,
    },
}

impl Cli {
    pub fn get_realm(&self) -> String {
        self.realm.clone().unwrap_or_else(|| "default".to_string())
    }
}

fn get_seed(_cli: &Cli) -> Result<Seed, Box<dyn std::error::Error>> {
    // check for SEED_FILE env override (for testing/automation)
    if let Ok(path_str) = std::env::var("SEED_FILE") {
        let path = PathBuf::from(path_str);
        return Seed::from_file(&path);
    }

    // try keyring first
    match Seed::from_keyring() {
        Ok(seed) => Ok(seed),
        Err(e) => {
            // keyring failed, prompt for passphrase
            // (don't print error during normal passphrase prompt)
            if std::env::var("DEBUG").is_ok() {
                eprintln!("debug: keyring access failed: {}", e);
            }
            eprint!("passphrase: ");
            std::io::stderr().flush()?;
            let passphrase = rpassword::read_password()?;
            Seed::from_passphrase(&passphrase)
        }
    }
}

fn prompt_passphrase(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    eprint!("{prompt}: ");
    std::io::stderr().flush()?;
    Ok(rpassword::read_password()?)
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let realm = cli.get_realm();

    match cli.command {
        Commands::Age { ref action } => match action {
            AgeAction::Pub => {
                let seed = get_seed(&cli)?;
                println!("{}", age::derive_recipient(&seed, &realm));
            }

            AgeAction::Key => {
                let seed = get_seed(&cli)?;
                println!("{}", age::derive_identity(&seed, &realm));
            }

            AgeAction::Encrypt {
                ref recipients,
                ref recipient_files,
                self_,
                passphrase,
                armor,
                ref output,
                ref file,
            } => {
                if *passphrase {
                    let pass = prompt_passphrase("passphrase")?;
                    age::encrypt_passphrase(&pass, *armor, file.as_deref(), output.as_deref())?;
                } else {
                    let seed = get_seed(&cli)?;
                    let use_self = *self_ || (recipients.is_empty() && recipient_files.is_empty());

                    let mut all_recipients: Vec<Box<dyn ::age::Recipient + Send>> = vec![];

                    if use_self {
                        let recipient_str = age::derive_recipient(&seed, &realm);
                        all_recipients.push(age::parse_recipient(&recipient_str)?);
                    }

                    for r in recipients {
                        all_recipients.push(age::parse_recipient(r)?);
                    }

                    for f in recipient_files {
                        all_recipients.extend(age::parse_recipients_file(f)?);
                    }

                    age::encrypt(all_recipients, *armor, file.as_deref(), output.as_deref())?;
                }
            }

            AgeAction::Decrypt {
                ref key,
                passphrase,
                ref output,
                ref file,
            } => {
                if *passphrase {
                    let pass = prompt_passphrase("passphrase")?;
                    age::decrypt_passphrase(&pass, file.as_deref(), output.as_deref())?;
                } else if let Some(key_file) = key {
                    age::decrypt_with_file(key_file, file.as_deref(), output.as_deref())?;
                } else {
                    let seed = get_seed(&cli)?;
                    let identity = age::derive_identity(&seed, &realm);
                    age::decrypt(&identity, file.as_deref(), output.as_deref())?;
                }
            }
        },

        Commands::Ssh { ref action } => match action {
            SshAction::Pub => {
                let seed = get_seed(&cli)?;
                println!("{}", ssh::derive_public(&seed, &realm));
            }

            SshAction::Key => {
                let seed = get_seed(&cli)?;
                print!("{}", ssh::derive_private(&seed, &realm));
            }

            SshAction::Add { lifetime, confirm } => {
                let seed = get_seed(&cli)?;
                ssh::add_to_agent(&seed, &realm, *lifetime, *confirm)?;
                eprintln!("added 1seed:{realm} to agent");
            }
        },

        Commands::Sign { ref action } => match action {
            SignAction::Pub => {
                let seed = get_seed(&cli)?;
                println!("{}", sign::derive_public(&seed, &realm));
            }

            SignAction::Data {
                ref output,
                binary,
                ref file,
            } => {
                let seed = get_seed(&cli)?;
                let sig = sign::sign(&seed, &realm, file.as_deref())?;

                if *binary {
                    if let Some(path) = output {
                        std::fs::write(path, &sig)?;
                    } else {
                        std::io::stdout().write_all(&sig)?;
                    }
                } else {
                    use base64::Engine;
                    let encoded = base64::engine::general_purpose::STANDARD.encode(&sig);
                    if let Some(path) = output {
                        std::fs::write(path, &encoded)?;
                    } else {
                        println!("{encoded}");
                    }
                }
            }

            SignAction::Verify {
                ref signature,
                ref pubkey,
                ref file,
            } => {
                let sig_bytes = if let Some(path) = signature.strip_prefix('@') {
                    std::fs::read(path)?
                } else {
                    use base64::Engine;
                    base64::engine::general_purpose::STANDARD.decode(signature)?
                };

                let pubkey_str = if let Some(pk) = pubkey.as_ref() {
                    pk.clone()
                } else {
                    let seed = get_seed(&cli)?;
                    sign::derive_public(&seed, &realm)
                };

                let valid = sign::verify(&pubkey_str, &sig_bytes, file.as_deref())?;

                if valid {
                    eprintln!("valid");
                } else {
                    eprintln!("invalid");
                    std::process::exit(1);
                }
            }
        },

        Commands::Derive { ref action } => match action {
            DeriveAction::Password {
                ref site,
                length,
                no_symbols,
                ref symbols,
                counter,
            } => {
                let seed = get_seed(&cli)?;
                let pw = password::derive(
                    &seed,
                    &realm,
                    site,
                    *counter,
                    *length,
                    !*no_symbols,
                    symbols,
                )?;
                print!("{}", pw.as_str());
            }

            DeriveAction::Mnemonic { words } => {
                eprintln!("WARNING: Cryptocurrency seed phrase");
                eprintln!("  - Same master seed = same mnemonic = same wallets");
                eprintln!("  - Compromise of master seed = loss of funds");
                eprintln!("  - Consider: dedicated realm, hardware wallet");
                eprintln!();

                let seed = get_seed(&cli)?;
                let mnemonic = derive::mnemonic(&seed, &realm, *words)?;
                println!("{}", mnemonic.as_str());
            }

            DeriveAction::Raw {
                ref path,
                length,
                hex: _,
                base64,
                binary,
            } => {
                let seed = get_seed(&cli)?;
                let bytes = derive::raw(&seed, &realm, path, *length);

                if *binary {
                    std::io::stdout().write_all(&bytes)?;
                } else if *base64 {
                    use base64::Engine;
                    println!(
                        "{}",
                        base64::engine::general_purpose::STANDARD.encode(&*bytes)
                    );
                } else {
                    println!("{}", hex::encode(&*bytes));
                }
            }
        },

        Commands::Init {
            passphrase,
            generate,
            from_file,
        } => {
            if Seed::keyring_exists() {
                eprintln!("Seed already exists in keyring");
                eprintln!("Run '1seed forget --confirm' first to remove it");
                std::process::exit(1);
            }

            let seed_data = if generate {
                // generate random 32 bytes
                let mut bytes = [0u8; 32];
                use std::fs::File;
                use std::io::Read;
                File::open("/dev/urandom")?.read_exact(&mut bytes)?;
                eprintln!("Generated 32-byte random seed");
                bytes.to_vec()
            } else if let Some(path) = from_file {
                // read from file
                let bytes = std::fs::read(path)?;
                eprintln!("Read {} bytes from file", bytes.len());
                bytes
            } else if passphrase {
                // prompt for passphrase
                eprint!("Enter passphrase: ");
                std::io::stderr().flush()?;
                let pass = rpassword::read_password()?;
                eprint!("Confirm passphrase: ");
                std::io::stderr().flush()?;
                let confirm = rpassword::read_password()?;
                if pass != confirm {
                    return Err("passphrases do not match".into());
                }
                pass.into_bytes()
            } else {
                return Err("must specify --passphrase, --generate, or --from-file".into());
            };

            Seed::store_in_keyring(&seed_data)?;

            // verify we can read it back (triggers permission dialog on first access)
            match Seed::from_keyring() {
                Ok(_) => {
                    eprintln!("✓ Seed stored in OS keychain and verified");
                    eprintln!("  Run '1seed status' to see derived keys");
                }
                Err(e) => {
                    eprintln!("⚠ Seed stored, but verification failed");
                    eprintln!("  Error: {}", e);
                    eprintln!();
                    eprintln!("  macOS: Grant keychain access when prompted by system dialog");
                    eprintln!("  Linux: Check Secret Service daemon is running");
                    eprintln!("  Windows: Check Credential Manager permissions");
                    eprintln!();
                    eprintln!("  Try: 1seed status");
                    std::process::exit(1);
                }
            }
        }

        Commands::Forget { confirm } => {
            if !confirm {
                eprintln!("This will remove your seed from the keychain");
                eprintln!("You will lose access to all derived keys");
                eprintln!();
                eprintln!("Run with --confirm to proceed:");
                eprintln!("  1seed forget --confirm");
                std::process::exit(1);
            }

            match Seed::remove_from_keyring() {
                Ok(()) => eprintln!("✓ Seed removed from keychain"),
                Err(e) => eprintln!("Failed to remove seed: {}", e),
            }
        }

        Commands::Update { check } => {
            update::update(check)?;
        }

        Commands::Status => {
            let env_realm = std::env::var("SEED_REALM").ok();
            let env_seed_file = std::env::var("SEED_FILE").ok();

            println!("1seed {}", env!("CARGO_PKG_VERSION"));
            println!();

            println!("Configuration");

            let realm_source = if cli.realm.is_some() && env_realm.is_none() {
                "flag"
            } else if env_realm.is_some() {
                "env"
            } else {
                "default"
            };
            println!("  realm: {} [{}]", realm, realm_source);

            println!();
            println!("Environment Variables");
            println!(
                "  SEED_REALM: {}",
                env_realm.as_deref().unwrap_or("(not set)")
            );
            println!(
                "  SEED_FILE:  {}",
                env_seed_file.as_deref().unwrap_or("(not set)")
            );

            println!();
            println!("Seed Storage");

            // try to get seed to determine actual source
            match get_seed(&cli) {
                Ok(seed) => {
                    // successfully got seed, determine how
                    if env_seed_file.is_some() {
                        println!("  source: SEED_FILE env override");
                    } else {
                        // if SEED_FILE is not set and we got a seed,
                        // it came from keyring (or user just entered passphrase)
                        // check if it's in keyring by attempting direct access
                        if Seed::from_keyring().is_ok() {
                            println!("  source: OS keychain");
                            println!("  macOS:   Keychain.app → Search '1seed'");
                            println!("  Linux:   Secret Service (GNOME Keyring / KWallet)");
                            println!("  Windows: Credential Manager");
                        } else {
                            println!("  source: entered passphrase (not stored)");
                            println!();
                            println!("  To store seed in keychain:");
                            println!("    1seed forget --confirm  # clear any partial state");
                            println!("    1seed init --passphrase # store new seed");
                        }
                    }

                    println!();
                    println!("Derived Keys (realm: {})", realm);
                    println!("  age:  {}", age::derive_recipient(&seed, &realm));

                    let ssh_pub = ssh::derive_public(&seed, &realm);
                    let parts: Vec<&str> = ssh_pub.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let key_preview = if parts[1].len() > 20 {
                            format!("{}...", &parts[1][..20])
                        } else {
                            parts[1].to_string()
                        };
                        println!("  ssh:  {} {key_preview}", parts[0]);
                    }

                    println!("  sign: {}", sign::derive_public(&seed, &realm));
                }
                Err(_) => {
                    if env_seed_file.is_some() {
                        println!("  source: SEED_FILE env override (file missing or unreadable)");
                    } else {
                        println!("  source: none");
                        println!();
                        println!("  To store a seed, run:");
                        println!("    1seed init --generate      # random 32 bytes");
                        println!("    1seed init --passphrase    # memorable passphrase");
                        println!("    1seed init --from-file <path>");
                    }
                }
            }

            if let Ok(binary) = std::env::current_exe() {
                println!();
                println!("Binary: {}", binary.display());
            }
        }
    }

    Ok(())
}
