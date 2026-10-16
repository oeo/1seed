use sha2::Digest;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::TempDir;

struct TestContext {
    _dir: TempDir,
    seed_file: std::path::PathBuf,
    _config_dir: std::path::PathBuf,
}

impl TestContext {
    fn new() -> Self {
        let _dir = TempDir::new().unwrap();
        let seed_file = _dir.path().join("seed");
        // stored-seed format, so spawned binaries never run production scrypt
        let mut seed = b"1SED2".to_vec();
        seed.extend_from_slice(&[7u8; 32]);
        std::fs::write(&seed_file, seed).unwrap();

        // canonicalize to resolve any symlinks (important on macOS where /tmp -> /private/tmp)
        let seed_file = seed_file.canonicalize().unwrap();

        // create isolated config directory (not used anymore, but kept for compatibility)
        let config_dir = _dir.path().join("config");
        std::fs::create_dir_all(&config_dir).unwrap();
        let _config_dir = config_dir.canonicalize().unwrap();

        TestContext {
            _dir,
            seed_file,
            _config_dir,
        }
    }

    fn cmd(&self) -> Command {
        self.cmd_realm("default")
    }

    fn cmd_realm(&self, realm: &str) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_1seed"));
        // use SEED_FILE env to override keyring for tests
        cmd.env("SEED_FILE", &self.seed_file);
        // always pass explicit realm
        cmd.arg("--realm");
        cmd.arg(realm);
        cmd
    }
}

#[test]
fn pub_deterministic() {
    let ctx = TestContext::new();

    let out1 = ctx.cmd().args(["age", "pub"]).output().unwrap();
    let out2 = ctx.cmd().args(["age", "pub"]).output().unwrap();

    assert!(out1.status.success());
    assert_eq!(out1.stdout, out2.stdout);
    assert!(String::from_utf8_lossy(&out1.stdout).starts_with("age1"));
}

#[test]
fn different_realms_different_keys() {
    let ctx = TestContext::new();

    let out1 = ctx
        .cmd_realm("realm1")
        .args(["age", "pub"])
        .output()
        .unwrap();
    let out2 = ctx
        .cmd_realm("realm2")
        .args(["age", "pub"])
        .output()
        .unwrap();

    assert!(out1.status.success());
    assert!(out2.status.success());
    assert_ne!(out1.stdout, out2.stdout);
}

#[test]
fn ssh_pub_format() {
    let ctx = TestContext::new();

    let out = ctx.cmd().args(["ssh", "pub"]).output().unwrap();
    assert!(out.status.success());

    let key = String::from_utf8_lossy(&out.stdout);
    assert!(key.starts_with("ssh-ed25519 "));
    assert!(key.contains("1seed:"));
}

#[test]
fn encrypt_decrypt_roundtrip() {
    let ctx = TestContext::new();
    let plaintext = b"hello world";

    // encrypt
    let mut enc = ctx
        .cmd()
        .args(["age", "encrypt", "-a"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    enc.stdin.as_mut().unwrap().write_all(plaintext).unwrap();
    let enc_out = enc.wait_with_output().unwrap();
    assert!(enc_out.status.success());

    let ciphertext = enc_out.stdout;
    assert!(String::from_utf8_lossy(&ciphertext).contains("-----BEGIN AGE ENCRYPTED FILE-----"));

    // decrypt
    let mut dec = ctx
        .cmd()
        .args(["age", "decrypt"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    dec.stdin.as_mut().unwrap().write_all(&ciphertext).unwrap();
    let dec_out = dec.wait_with_output().unwrap();
    assert!(dec_out.status.success());

    assert_eq!(dec_out.stdout, plaintext);
}

#[test]
fn password_deterministic() {
    let ctx = TestContext::new();

    let out1 = ctx
        .cmd()
        .args(["derive", "password", "github.com"])
        .output()
        .unwrap();
    let out2 = ctx
        .cmd()
        .args(["derive", "password", "github.com"])
        .output()
        .unwrap();

    assert!(out1.status.success());
    assert_eq!(out1.stdout, out2.stdout);
    assert_eq!(out1.stdout.len(), 16);
}

#[test]
fn password_counter_changes_output() {
    let ctx = TestContext::new();

    let out1 = ctx
        .cmd()
        .args(["derive", "password", "site", "-n", "1"])
        .output()
        .unwrap();
    let out2 = ctx
        .cmd()
        .args(["derive", "password", "site", "-n", "2"])
        .output()
        .unwrap();

    assert!(out1.status.success());
    assert!(out2.status.success());
    assert_ne!(out1.stdout, out2.stdout);
}

#[test]
fn password_length() {
    let ctx = TestContext::new();

    let out = ctx
        .cmd()
        .args(["derive", "password", "site", "-l", "32"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout.len(), 32);
}

#[test]
fn sign_verify_roundtrip() {
    let ctx = TestContext::new();
    let data = b"data to sign";

    // sign
    let mut sign = ctx
        .cmd()
        .args(["sign", "data"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    sign.stdin.as_mut().unwrap().write_all(data).unwrap();
    let sign_out = sign.wait_with_output().unwrap();
    assert!(sign_out.status.success());

    let sig = String::from_utf8(sign_out.stdout)
        .unwrap()
        .trim()
        .to_string();

    // verify
    let mut verify = ctx
        .cmd()
        .args(["sign", "verify", &sig])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    verify.stdin.as_mut().unwrap().write_all(data).unwrap();
    let verify_out = verify.wait_with_output().unwrap();
    assert!(verify_out.status.success());
}

#[test]
fn sign_output_file_verifies() {
    let ctx = TestContext::new();
    let dir = ctx._dir.path();
    let data = dir.join("data");
    std::fs::write(&data, b"data to sign").unwrap();

    let verify = |sig: &std::path::Path| {
        ctx.cmd()
            .args(["sign", "verify", &format!("@{}", sig.display())])
            .arg(&data)
            .output()
            .unwrap()
    };

    for (name, flags) in [("sig.b64", vec![]), ("sig.bin", vec!["--binary"])] {
        let sig = dir.join(name);
        let out = ctx
            .cmd()
            .args(["sign", "data", "-o"])
            .arg(&sig)
            .args(flags)
            .arg(&data)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(verify(&sig).status.success(), "{name} did not verify");
    }

    let junk = dir.join("junk");
    std::fs::write(&junk, b"not a signature").unwrap();
    let out = verify(&junk);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("64 raw bytes or base64 text"));
}

#[test]
fn verify_fails_wrong_data() {
    let ctx = TestContext::new();

    // sign
    let mut sign = ctx
        .cmd()
        .args(["sign", "data"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    sign.stdin.as_mut().unwrap().write_all(b"original").unwrap();
    let sign_out = sign.wait_with_output().unwrap();
    let sig = String::from_utf8(sign_out.stdout)
        .unwrap()
        .trim()
        .to_string();

    // verify with different data
    let mut verify = ctx
        .cmd()
        .args(["sign", "verify", &sig])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    verify
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"tampered")
        .unwrap();
    let verify_out = verify.wait_with_output().unwrap();
    assert!(!verify_out.status.success());
}

#[test]
fn raw_hex_output() {
    let ctx = TestContext::new();

    let out = ctx
        .cmd()
        .args(["derive", "raw", "test", "-l", "16"])
        .output()
        .unwrap();
    assert!(out.status.success());

    let hex = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_eq!(hex.len(), 32); // 16 bytes = 32 hex chars
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn raw_base64_output() {
    let ctx = TestContext::new();

    let out = ctx
        .cmd()
        .args(["derive", "raw", "test", "-l", "32", "--base64"])
        .output()
        .unwrap();
    assert!(out.status.success());

    let b64 = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_eq!(b64.len(), 44); // 32 bytes base64 = 44 chars
}

#[test]
fn mnemonic_word_counts() {
    let ctx = TestContext::new();

    for words in [12, 15, 18, 21, 24] {
        let out = ctx
            .cmd()
            .args(["derive", "mnemonic", "-w", &words.to_string()])
            .output()
            .unwrap();

        assert!(out.status.success());

        let mnemonic = String::from_utf8_lossy(&out.stdout);
        let count = mnemonic.split_whitespace().count();
        assert_eq!(count, words, "expected {words} words, got {count}");
    }
}

#[test]
fn derive_integer_range() {
    let ctx = TestContext::new();

    let out = ctx
        .cmd()
        .args(["derive", "int", "test", "--min", "10", "--max", "20"])
        .output()
        .unwrap();

    assert!(out.status.success());
    let val: i64 = String::from_utf8_lossy(&out.stdout).trim().parse().unwrap();
    assert!((10..=20).contains(&val));
}

#[test]
fn derive_uuid_format() {
    let ctx = TestContext::new();

    let out = ctx.cmd().args(["derive", "uuid", "test"]).output().unwrap();

    assert!(out.status.success());
    let uuid = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_eq!(uuid.len(), 36);

    // Check basic format xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    let parts: Vec<&str> = uuid.split('-').collect();
    assert_eq!(parts.len(), 5);
}

fn init_cmd(dir: &std::path::Path) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_1seed"));
    cmd.env("SEED_NO_KEYRING", "1");
    cmd.env("HOME", dir);
    cmd
}

#[test]
fn init_generate_stores_magic_seed() {
    let dir = tempfile::TempDir::new().unwrap();
    let seed_path = dir.path().join(".1seed");

    let out = init_cmd(dir.path())
        .arg("init")
        .arg("--generate")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let bytes = std::fs::read(&seed_path).unwrap();
    assert!(bytes.starts_with(b"1SED2"));
    assert_eq!(bytes.len(), 37);

    // 0600 even under a permissive umask
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(&seed_path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
    }
}

#[test]
fn init_and_forget_refuse_seed_overrides() {
    let cases: [(&[&str], bool); 6] = [
        (&["init", "--generate"], true),
        (&["--seed-file", "x", "init", "--generate"], false),
        (&["--ask-passphrase", "init", "--generate"], false),
        (&["forget", "--confirm"], true),
        (&["--seed-file", "x", "forget", "--confirm"], false),
        (&["--ask-passphrase", "forget", "--confirm"], false),
    ];
    for (args, via_env) in cases {
        let dir = tempfile::TempDir::new().unwrap();
        let mut cmd = init_cmd(dir.path());
        if via_env {
            cmd.env("SEED_FILE", dir.path().join("missing"));
        }
        let out = cmd.args(args).output().unwrap();
        assert!(!out.status.success(), "{args:?}");
        assert!(
            String::from_utf8_lossy(&out.stderr).contains("acts on the stored seed only"),
            "{args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert_eq!(
            std::fs::read_dir(dir.path()).unwrap().count(),
            0,
            "{args:?}"
        );
    }
}

#[test]
fn ask_passphrase_conflicts_with_seed_file() {
    let out = Command::new(env!("CARGO_BIN_EXE_1seed"))
        .args(["--seed-file", "x", "--ask-passphrase", "derive", "mnemonic"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("cannot be used with"));
}

#[test]
fn init_from_magic_file_rejected() {
    let dir = tempfile::TempDir::new().unwrap();
    let src = dir.path().join("seed.bin");
    let mut blob = Vec::from(&b"1SED2"[..]);
    blob.extend_from_slice(&[0u8; 32]);
    std::fs::write(&src, &blob).unwrap();

    let out = init_cmd(dir.path())
        .arg("init")
        .arg("--from-file")
        .arg(&src)
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("already a stored seed"));
}

#[test]
fn init_from_utf8_passphrase_scrypts() {
    let dir = tempfile::TempDir::new().unwrap();
    let src = dir.path().join("pass.txt");
    let pass = "my secret passphrase that is longer than 32 bytes";
    assert!(pass.len() >= 32);
    std::fs::write(&src, pass).unwrap();

    let out = init_cmd(dir.path())
        .arg("init")
        .arg("--from-file")
        .arg(&src)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let bytes = std::fs::read(dir.path().join(".1seed")).unwrap();
    assert!(bytes.starts_with(b"1SED2"));
    assert_eq!(bytes.len(), 37);

    // derived key equals scrypt-of-passphrase, not raw first-32-bytes
    let cmd = Command::new(env!("CARGO_BIN_EXE_1seed"))
        .env("SEED_FILE", dir.path().join(".1seed"))
        .args(["--realm", "default", "age", "pub"])
        .output()
        .unwrap();
    assert!(cmd.status.success());
}

#[test]
fn test_mode_env_does_not_weaken_kdf() {
    let dir = tempfile::TempDir::new().unwrap();
    let seed = dir.path().join("pass.txt");
    std::fs::write(&seed, "short passphrase").unwrap();

    let age_pub = |test_mode: bool| {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_1seed"));
        cmd.env("SEED_FILE", &seed)
            .env_remove("ONESEED_TEST_MODE")
            .args(["--realm", "default", "age", "pub"]);
        if test_mode {
            cmd.env("ONESEED_TEST_MODE", "1");
        }
        let out = cmd.output().unwrap();
        assert!(out.status.success());
        out.stdout
    };

    assert_eq!(age_pub(false), age_pub(true));
}

#[test]
fn malformed_seed_files_rejected() {
    let dir = tempfile::TempDir::new().unwrap();
    let mut stored = b"1SED2".to_vec();
    stored.extend_from_slice(&[7u8; 33]);

    let cases: [(&str, &[u8]); 3] = [
        ("empty", b""),
        ("trailing", &stored),
        ("text", b"correct horse battery staple and more words\n"),
    ];
    for (name, bytes) in cases {
        let seed = dir.path().join(name);
        std::fs::write(&seed, bytes).unwrap();
        let out = Command::new(env!("CARGO_BIN_EXE_1seed"))
            .env("SEED_FILE", &seed)
            .args(["--realm", "default", "age", "pub"])
            .output()
            .unwrap();
        assert!(!out.status.success(), "{name} seed was accepted");
    }
}

#[test]
fn init_from_first_32_bytes_keeps_legacy_raw_key() {
    let dir = tempfile::TempDir::new().unwrap();
    // rejected as a seed file; older versions used its first 32 bytes as the key
    let legacy = b"correct horse battery staple and more words\n";
    let src = dir.path().join("first32");
    std::fs::write(&src, &legacy[..32]).unwrap();

    let out = init_cmd(dir.path())
        .arg("init")
        .arg("--from-file")
        .arg(&src)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stored = std::fs::read(dir.path().join(".1seed")).unwrap();
    assert_eq!(&stored[..5], b"1SED2");
    assert_eq!(&stored[5..], &legacy[..32]);
}

#[cfg(target_os = "linux")]
#[test]
fn init_on_linux_stores_seed_file_without_keyring_override() {
    let dir = tempfile::TempDir::new().unwrap();

    let out = Command::new(env!("CARGO_BIN_EXE_1seed"))
        .env("HOME", dir.path())
        .env_remove("SEED_NO_KEYRING")
        .env_remove("SEED_FILE")
        .args(["init", "--generate"])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(std::fs::read(dir.path().join(".1seed")).unwrap().len(), 37);
}

#[test]
fn derive_raw_too_long_errors() {
    let ctx = TestContext::new();

    let out = ctx
        .cmd()
        .args(["derive", "raw", "test", "-l", "9000"])
        .output()
        .unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("8160"), "unexpected stderr: {stderr}");
}

#[test]
fn derive_integer_full_range() {
    let ctx = TestContext::new();

    let out = ctx
        .cmd()
        .args([
            "derive",
            "int",
            "test",
            "--min=-9223372036854775808",
            "--max=9223372036854775807",
        ])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout)
        .trim()
        .parse::<i64>()
        .unwrap();
}

fn stdout_of(cmd: &mut Command) -> String {
    let out = cmd.output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

/// Exact outputs captured from v0.8.2. Any change that alters derived keys,
/// encodings, or the KDF fails here; such a change must be a new derivation
/// version, never an edit to these values.
#[test]
fn known_answer_vectors() {
    let ctx = TestContext::new();
    let dir = ctx._dir.path();
    let data = dir.join("data.txt");
    std::fs::write(&data, "known answer data\n").unwrap();
    let data = data.to_str().unwrap();

    let cases: &[(&[&str], &str)] = &[
        (&["age", "pub"], "age1uw84uq6ek9n5kk6mej7rj9qsx6r26wmzjluxdpx6pj6ed74nsvcsmcydvt\n"),
        (&["age", "key"], "AGE-SECRET-KEY-1FMZVF5WNLYAHSTM928Y3H5VC439Y2ZFFMFXZQFAUJUHWCQF35PDQLPL26M\n"),
        (&["ssh", "pub"], "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAII9bp34wTxHzKITMKURXzOBbIeE0RVd1E35rqiBtYMYZ 1seed:default\n"),
        (
            &["ssh", "key"],
            "-----BEGIN OPENSSH PRIVATE KEY-----\n\
             b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW\n\
             QyNTUxOQAAACCPW6d+ME8R8yiEzClEV8zgWyHhNEVXdRN+a6ogbWDGGQAAAIiYY5eEmGOX\n\
             hAAAAAtzc2gtZWQyNTUxOQAAACCPW6d+ME8R8yiEzClEV8zgWyHhNEVXdRN+a6ogbWDGGQ\n\
             AAAEBjeQ1LIC1zBJhjVT9bu+5QOUx/u2V2VWEuwOu0ahWTyo9bp34wTxHzKITMKURXzOBb\n\
             IeE0RVd1E35rqiBtYMYZAAAAAAECAwQF\n\
             -----END OPENSSH PRIVATE KEY-----\n",
        ),
        (&["sign", "pub"], "uKuH78uGCreO6QWmkoCn8pWACr/p94UTorD0039du/A=\n"),
        (&["sign", "data", data], "UUxc1RJUnjZkxxIyKn42ujz0bzl6K6Dg+QxPX4GLfRjuX0EJYanYkoTVpkpmRo2h/FUhbtPWU71mL6k037+EBQ==\n"),
        (&["derive", "password", "example.com"], "Er9CWbRVGRZzM!87"),
        (&["derive", "password", "example.com", "-l", "4", "--no-symbols"], "4rHc"),
        (&["derive", "password", "example.com", "--symbols", "+-", "-n", "2"], "2DAAahXetu2Uj8K+"),
        (&["derive", "mnemonic", "-w", "12"], "initial wedding nephew differ swing affair float off device quality album type\n"),
        (&["derive", "mnemonic", "-w", "15"], "initial wedding nephew differ swing affair float off device quality album turtle razor among general\n"),
        (&["derive", "mnemonic", "-w", "18"], "initial wedding nephew differ swing affair float off device quality album turtle razor among glad puzzle extend nerve\n"),
        (&["derive", "mnemonic", "-w", "21"], "initial wedding nephew differ swing affair float off device quality album turtle razor among glad puzzle extend news tube cost mobile\n"),
        (&["derive", "mnemonic", "-w", "24"], "initial wedding nephew differ swing affair float off device quality album turtle razor among glad puzzle extend news tube cost limb bunker thought code\n"),
        (&["derive", "int", "dice", "--min", "1", "--max", "6"], "6\n"),
        (&["derive", "int", "full", "--min=-9223372036854775808", "--max=9223372036854775807"], "8320348908091761961\n"),
        (&["derive", "uuid", "test"], "06a40a98-4a36-444f-bc6d-af65d587dbb0\n"),
        (&["derive", "raw", "key", "-l", "64"], "4df42929a080f94850e8695e4198a8207047c2c975dd243abb5ac045402c33aa48c0549166ffa33e67a149df1def4abf03cba132290b54098707d7733a9a4c39\n"),
    ];
    for (args, expected) in cases {
        assert_eq!(&stdout_of(ctx.cmd().args(*args)), expected, "{args:?}");
    }

    // the realm defaults to "default" when neither --realm nor SEED_REALM is given
    let mut no_realm = Command::new(env!("CARGO_BIN_EXE_1seed"));
    no_realm
        .env("SEED_FILE", &ctx.seed_file)
        .env_remove("SEED_REALM")
        .args(["age", "pub"]);
    assert_eq!(stdout_of(&mut no_realm), cases[0].1);

    // headerless binary seeds of 32+ bytes are read as a raw master
    let legacy = dir.join("legacy");
    let mut raw = vec![0xffu8];
    raw.extend(1u8..40);
    std::fs::write(&legacy, raw).unwrap();
    let mut legacy_cmd = Command::new(env!("CARGO_BIN_EXE_1seed"));
    legacy_cmd
        .env("SEED_FILE", &legacy)
        .args(["--realm", "default", "age", "pub"]);
    assert_eq!(
        stdout_of(&mut legacy_cmd),
        "age1en8ye6klhvv9s6mm7gewfkrt6pgnt3dxqyt0g7nhy26z360u7qmqsrpyqk\n"
    );

    let wordlist = bip39::Language::English.word_list().join("\n");
    assert_eq!(
        hex::encode(sha2::Sha256::digest(wordlist.as_bytes())),
        "187db04a869dd9bc7be80d21a86497d692c0db6abd3aa8cb6be5d618ff757fae"
    );
}

// The README's offline test vector; keep the two in sync.
const README_PASSPHRASE: &str = "tidy oxygen ripple cobalt swerve banjo";
const README_MNEMONIC: &str = "blush neck grab brass orchard real veteran afraid box empty tuna decade smooth tree other hidden broken increase address seek fox purchase possible roof\n";

/// Pins scrypt at production cost and both passphrase trims: the one a
/// headerless seed file takes and the one init and --ask-passphrase take.
#[test]
fn known_answer_passphrase() {
    let dir = tempfile::TempDir::new().unwrap();
    let mnemonic = |seed: &std::path::Path| {
        stdout_of(
            Command::new(env!("CARGO_BIN_EXE_1seed"))
                .env("SEED_FILE", seed)
                .args(["--realm", "btc", "derive", "mnemonic"]),
        )
    };

    let headerless = dir.path().join("pass.txt");
    std::fs::write(&headerless, format!("  {README_PASSPHRASE}  ")).unwrap();
    assert_eq!(mnemonic(&headerless), README_MNEMONIC);

    // not exactly 32 bytes, which init would read as a raw key
    let typed = dir.path().join("typed.txt");
    std::fs::write(&typed, format!("  {README_PASSPHRASE}  \n")).unwrap();
    let out = init_cmd(dir.path())
        .args(["init", "--from-file"])
        .arg(&typed)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stored = dir.path().join(".1seed");
    assert_eq!(
        hex::encode(std::fs::read(&stored).unwrap()),
        "315345443203dc2b23f29e7430a51e147d88a4b3c029d68f0bf6a96b8a2b24871b87586bca"
    );
    assert_eq!(mnemonic(&stored), README_MNEMONIC);
}
