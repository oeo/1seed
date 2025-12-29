use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::TempDir;

struct TestContext {
    _dir: TempDir,
    seed_file: std::path::PathBuf,
    config_dir: std::path::PathBuf,
}

impl TestContext {
    fn new() -> Self {
        let _dir = TempDir::new().unwrap();
        let seed_file = _dir.path().join("seed");
        std::fs::write(&seed_file, b"test seed phrase for integration tests").unwrap();

        // canonicalize to resolve any symlinks (important on macOS where /tmp -> /private/tmp)
        let seed_file = seed_file.canonicalize().unwrap();

        // create isolated config directory to prevent tests from reading user's config
        let config_dir = _dir.path().join("config");
        std::fs::create_dir_all(&config_dir).unwrap();
        let config_dir = config_dir.canonicalize().unwrap();

        TestContext {
            _dir,
            seed_file,
            config_dir,
        }
    }

    fn cmd(&self) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_1seed"));
        cmd.env("ONESEED_TEST_MODE", "1");
        cmd.env("XDG_CONFIG_HOME", &self.config_dir);
        cmd.arg("-f");
        cmd.arg(&self.seed_file);
        cmd
    }
}

#[test]
fn pub_deterministic() {
    let ctx = TestContext::new();

    let out1 = ctx.cmd().arg("pub").output().unwrap();
    let out2 = ctx.cmd().arg("pub").output().unwrap();

    assert!(out1.status.success());
    assert_eq!(out1.stdout, out2.stdout);
    assert!(String::from_utf8_lossy(&out1.stdout).starts_with("age1"));
}

#[test]
fn different_realms_different_keys() {
    let ctx = TestContext::new();

    let out1 = ctx.cmd().args(["-r", "realm1", "pub"]).output().unwrap();
    let out2 = ctx.cmd().args(["-r", "realm2", "pub"]).output().unwrap();

    assert!(out1.status.success());
    assert!(out2.status.success());
    assert_ne!(out1.stdout, out2.stdout);
}

#[test]
fn ssh_pub_format() {
    let ctx = TestContext::new();

    let out = ctx.cmd().arg("ssh-pub").output().unwrap();
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
        .args(["enc", "-a"])
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
        .arg("dec")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    dec.stdin.as_mut().unwrap().write_all(&ciphertext).unwrap();
    let dec_out = dec.wait_with_output().unwrap();

    if !dec_out.status.success() {
        eprintln!("Decrypt failed!");
        eprintln!("stderr: {}", String::from_utf8_lossy(&dec_out.stderr));
        eprintln!("stdout: {}", String::from_utf8_lossy(&dec_out.stdout));
        panic!("decrypt command failed");
    }

    assert_eq!(dec_out.stdout, plaintext);
}

#[test]
fn password_deterministic() {
    let ctx = TestContext::new();

    let out1 = ctx.cmd().args(["pw", "github.com"]).output().unwrap();
    let out2 = ctx.cmd().args(["pw", "github.com"]).output().unwrap();

    assert!(out1.status.success());
    assert_eq!(out1.stdout, out2.stdout);
    assert_eq!(out1.stdout.len(), 16);
}

#[test]
fn password_counter_changes_output() {
    let ctx = TestContext::new();

    let out1 = ctx.cmd().args(["pw", "site", "-n", "1"]).output().unwrap();
    let out2 = ctx.cmd().args(["pw", "site", "-n", "2"]).output().unwrap();

    assert!(out1.status.success());
    assert!(out2.status.success());
    assert_ne!(out1.stdout, out2.stdout);
}

#[test]
fn password_length() {
    let ctx = TestContext::new();

    let out = ctx.cmd().args(["pw", "site", "-l", "32"]).output().unwrap();
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
        .arg("sign")
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
        .args(["verify", &sig])
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
fn verify_fails_wrong_data() {
    let ctx = TestContext::new();

    // sign
    let mut sign = ctx
        .cmd()
        .arg("sign")
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
        .args(["verify", &sig])
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
        .args(["raw", "test", "-l", "16"])
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
        .args(["raw", "test", "-l", "32", "--base64"])
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
            .args(["mnemonic", "-w", &words.to_string()])
            .output()
            .unwrap();

        assert!(out.status.success());

        let mnemonic = String::from_utf8_lossy(&out.stdout);
        let count = mnemonic.trim().split_whitespace().count();
        assert_eq!(count, words, "expected {words} words, got {count}");
    }
}

#[test]
fn config_set_get() {
    let ctx = TestContext::new();
    let dir = TempDir::new().unwrap();
    let config_dir = dir.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();

    let out = ctx
        .cmd()
        .env("XDG_CONFIG_HOME", &config_dir)
        .args(["config", "set", "realm", "test-realm"])
        .output()
        .unwrap();
    assert!(out.status.success());

    let out = ctx
        .cmd()
        .env("XDG_CONFIG_HOME", &config_dir)
        .args(["config", "get", "realm"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "test-realm");
}

#[test]
fn realms_add_list_rm() {
    let ctx = TestContext::new();
    let dir = TempDir::new().unwrap();
    let config_dir = dir.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();

    // add
    ctx.cmd()
        .env("XDG_CONFIG_HOME", &config_dir)
        .args(["realms", "add", "personal"])
        .output()
        .unwrap();
    ctx.cmd()
        .env("XDG_CONFIG_HOME", &config_dir)
        .args(["realms", "add", "work"])
        .output()
        .unwrap();

    // list
    let out = ctx
        .cmd()
        .env("XDG_CONFIG_HOME", &config_dir)
        .arg("realms")
        .output()
        .unwrap();
    let list = String::from_utf8_lossy(&out.stdout);
    assert!(list.contains("personal"));
    assert!(list.contains("work"));

    // remove
    ctx.cmd()
        .env("XDG_CONFIG_HOME", &config_dir)
        .args(["realms", "rm", "work"])
        .output()
        .unwrap();

    let out = ctx
        .cmd()
        .env("XDG_CONFIG_HOME", &config_dir)
        .arg("realms")
        .output()
        .unwrap();
    let list = String::from_utf8_lossy(&out.stdout);
    assert!(list.contains("personal"));
    assert!(!list.contains("work"));
}
