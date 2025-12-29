use clap::{Parser, Subcommand};
use std::io::Write;
use std::path::PathBuf;

use crate::config::Config;
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
    1seed age pub                    Show age public key
    1seed --realm work ssh add       Add work SSH key to agent
    echo secret | 1seed age encrypt  Encrypt to self
    1seed derive password github.com Derive password

ENVIRONMENT:
    SEED_FILE    Path to seed file
    SEED_REALM   Default realm (default: \"default\")
")]
pub struct Cli {
    #[arg(long, global = true, env = "SEED_REALM")]
    pub realm: Option<String>,

    #[arg(short = 'f', long, global = true, env = "SEED_FILE")]
    pub seed_file: Option<PathBuf>,

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

    /// Show status, configuration sources, and derived keys
    Status,

    /// Update to latest release from GitHub
    Update {
        #[arg(long)]
        check: bool,
    },

    /// Set configuration value
    Set { key: String, value: String },

    /// Get configuration value
    Get { key: String },
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
        if let Some(ref r) = self.realm {
            return r.clone();
        }
        if let Ok(config) = Config::load() {
            if let Some(r) = config.realm {
                return r;
            }
        }
        "default".to_string()
    }

    pub fn get_seed_file(&self) -> Option<PathBuf> {
        if let Some(ref f) = self.seed_file {
            return Some(f.clone());
        }
        if let Ok(config) = Config::load() {
            return config.seed_file;
        }
        None
    }
}

fn get_seed(cli: &Cli) -> Result<Seed, Box<dyn std::error::Error>> {
    if let Some(path) = cli.get_seed_file() {
        Seed::from_file(&path)
    } else {
        eprint!("passphrase: ");
        std::io::stderr().flush()?;
        let passphrase = rpassword::read_password()?;
        Seed::from_passphrase(&passphrase)
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

        Commands::Update { check } => {
            update::update(check)?;
        }

        Commands::Set { key, value } => {
            let mut config = Config::load().unwrap_or_default();
            match key.as_str() {
                "realm" => config.realm = Some(value),
                "seed-file" => config.seed_file = Some(PathBuf::from(value)),
                _ => return Err(format!("unknown key: {key}").into()),
            }
            config.save()?;
        }

        Commands::Get { key } => {
            let config = Config::load()?;
            match key.as_str() {
                "realm" => {
                    if let Some(v) = config.realm {
                        println!("{v}");
                    }
                }
                "seed-file" => {
                    if let Some(v) = config.seed_file {
                        println!("{}", v.display());
                    }
                }
                _ => return Err(format!("unknown key: {key}").into()),
            }
        }

        Commands::Status => {
            let config = Config::load().unwrap_or_default();
            let env_realm = std::env::var("SEED_REALM").ok();
            let env_seed_file = std::env::var("SEED_FILE").ok();

            println!("1seed {}", env!("CARGO_PKG_VERSION"));
            println!();

            println!("Configuration (priority: flag > env > config > default)");

            let realm_source = if cli.realm.is_some() && env_realm.is_none() {
                "flag"
            } else if env_realm.is_some() {
                "env"
            } else if config.realm.is_some() {
                "config"
            } else {
                "default"
            };
            println!("  realm:     {:<20} [{}]", realm, realm_source);

            if let Some(seed_path) = cli.get_seed_file() {
                let seed_source = if cli.seed_file.is_some() && env_seed_file.is_none() {
                    "flag"
                } else if env_seed_file.is_some() {
                    "env"
                } else {
                    "config"
                };
                let status = if seed_path.exists() {
                    "found"
                } else {
                    "missing"
                };
                println!(
                    "  seed-file: {:<20} [{}] ({})",
                    seed_path.display(),
                    seed_source,
                    status
                );
            } else {
                println!("  seed-file: {:<20} [default] (prompt)", "(none)");
            }

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
            match Config::path() {
                Ok(path) => {
                    if path.exists() {
                        println!("Config File: {} (exists)", path.display());
                        if let Some(r) = &config.realm {
                            println!("  realm:     {}", r);
                        } else {
                            println!("  realm:     (not set)");
                        }
                        if let Some(f) = &config.seed_file {
                            println!("  seed-file: {}", f.display());
                        } else {
                            println!("  seed-file: (not set)");
                        }
                    } else {
                        println!("Config File: {} (not found)", path.display());
                    }
                }
                Err(e) => {
                    println!("Config File: (error: {})", e);
                }
            }

            if let Ok(seed) = get_seed(&cli) {
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

            if let Ok(binary) = std::env::current_exe() {
                println!();
                println!("Binary: {}", binary.display());
            }
        }
    }

    Ok(())
}
