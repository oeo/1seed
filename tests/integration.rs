use std::process::{Command, Stdio};
use std::io::Write;
use tempfile::TempDir;

fn seed_cmd() -> Command {
    Command::new(env!("CARGO_BIN_EXE_1seed"))
}

fn setup_seed() -> TempDir {
    let dir = TempDir::new().unwrap();
    let seed_file = dir.path().join("seed");
    std::fs::write(&seed_file, b"test seed phrase for integration tests").unwrap();
    std::env::set_var("SEED_FILE", &seed_file);
    dir
}

#[test]
fn pub_deterministic() {
    let _dir = setup_seed();

    let out1 = seed_cmd().arg("pub").output().unwrap();
    let out2 = seed_cmd().arg("pub").output().unwrap();

    assert!(out1.status.success());
    assert_eq!(out1.stdout, out2.stdout);
    assert!(String::from_utf8_lossy(&out1.stdout).starts_with("age1"));
}

#[test]
fn different_realms_different_keys() {
    let _dir = setup_seed();

    let out1 = seed_cmd().args(["-r", "realm1", "pub"]).output().unwrap();
    let out2 = seed_cmd().args(["-r", "realm2", "pub"]).output().unwrap();

    assert!(out1.status.success());
    assert!(out2.status.success());
    assert_ne!(out1.stdout, out2.stdout);
}

#[test]
fn ssh_pub_format() {
    let _dir = setup_seed();

    let out = seed_cmd().arg("ssh-pub").output().unwrap();
    assert!(out.status.success());

    let key = String::from_utf8_lossy(&out.stdout);
    assert!(key.starts_with("ssh-ed25519 "));
    assert!(key.contains("1seed:"));
}

#[test]
fn encrypt_decrypt_roundtrip() {
    let _dir = setup_seed();
    let plaintext = b"hello world";

    // encrypt
    let mut enc = seed_cmd()
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
    let mut dec = seed_cmd()
        .arg("dec")
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
    let _dir = setup_seed();

    let out1 = seed_cmd().args(["pw", "github.com"]).output().unwrap();
    let out2 = seed_cmd().args(["pw", "github.com"]).output().unwrap();

    assert!(out1.status.success());
    assert_eq!(out1.stdout, out2.stdout);
    assert_eq!(out1.stdout.len(), 16);
}

#[test]
fn password_counter_changes_output() {
    let _dir = setup_seed();

    let out1 = seed_cmd().args(["pw", "site", "-n", "1"]).output().unwrap();
    let out2 = seed_cmd().args(["pw", "site", "-n", "2"]).output().unwrap();

    assert!(out1.status.success());
    assert!(out2.status.success());
    assert_ne!(out1.stdout, out2.stdout);
}

#[test]
fn password_length() {
    let _dir = setup_seed();

    let out = seed_cmd().args(["pw", "site", "-l", "32"]).output().unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout.len(), 32);
}

#[test]
fn sign_verify_roundtrip() {
    let _dir = setup_seed();
    let data = b"data to sign";

    // sign
    let mut sign = seed_cmd()
        .arg("sign")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    sign.stdin.as_mut().unwrap().write_all(data).unwrap();
    let sign_out = sign.wait_with_output().unwrap();
    assert!(sign_out.status.success());

    let sig = String::from_utf8(sign_out.stdout).unwrap().trim().to_string();

    // verify
    let mut verify = seed_cmd()
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
    let _dir = setup_seed();

    // sign
    let mut sign = seed_cmd()
        .arg("sign")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    sign.stdin.as_mut().unwrap().write_all(b"original").unwrap();
    let sign_out = sign.wait_with_output().unwrap();
    let sig = String::from_utf8(sign_out.stdout).unwrap().trim().to_string();

    // verify with different data
    let mut verify = seed_cmd()
        .args(["verify", &sig])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    verify.stdin.as_mut().unwrap().write_all(b"tampered").unwrap();
    let verify_out = verify.wait_with_output().unwrap();
    assert!(!verify_out.status.success());
}

#[test]
fn raw_hex_output() {
    let _dir = setup_seed();

    let out = seed_cmd().args(["raw", "test", "-l", "16"]).output().unwrap();
    assert!(out.status.success());

    let hex = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_eq!(hex.len(), 32); // 16 bytes = 32 hex chars
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn raw_base64_output() {
    let _dir = setup_seed();

    let out = seed_cmd().args(["raw", "test", "-l", "32", "--base64"]).output().unwrap();
    assert!(out.status.success());

    let b64 = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_eq!(b64.len(), 44); // 32 bytes base64 = 44 chars
}

#[test]
fn mnemonic_word_counts() {
    let _dir = setup_seed();

    for words in [12, 15, 18, 21, 24] {
        let out = seed_cmd()
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
    let dir = TempDir::new().unwrap();
    let config_dir = dir.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();

    // override config location via XDG
    std::env::set_var("XDG_CONFIG_HOME", &config_dir);

    let out = seed_cmd()
        .args(["config", "set", "realm", "test-realm"])
        .output()
        .unwrap();
    assert!(out.status.success());

    let out = seed_cmd()
        .args(["config", "get", "realm"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "test-realm");
}

#[test]
fn realms_add_list_rm() {
    let dir = TempDir::new().unwrap();
    let config_dir = dir.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::env::set_var("XDG_CONFIG_HOME", &config_dir);

    // add
    seed_cmd().args(["realms", "add", "personal"]).output().unwrap();
    seed_cmd().args(["realms", "add", "work"]).output().unwrap();

    // list
    let out = seed_cmd().arg("realms").output().unwrap();
    let list = String::from_utf8_lossy(&out.stdout);
    assert!(list.contains("personal"));
    assert!(list.contains("work"));

    // remove
    seed_cmd().args(["realms", "rm", "work"]).output().unwrap();

    let out = seed_cmd().arg("realms").output().unwrap();
    let list = String::from_utf8_lossy(&out.stdout);
    assert!(list.contains("personal"));
    assert!(!list.contains("work"));
}
