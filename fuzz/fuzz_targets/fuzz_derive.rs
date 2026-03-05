#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }

    // split fuzz input into realm, path, and length
    let split = data.iter().position(|&b| b == 0).unwrap_or(1);
    let realm = std::str::from_utf8(&data[..split]).unwrap_or("r");
    let rest = &data[split.saturating_add(1)..];
    let path = std::str::from_utf8(rest).unwrap_or("p");

    // use a fixed seed so the fuzzer explores input handling, not seed variation
    let seed = oneseed::seed::Seed::from_passphrase("fuzz-seed").unwrap();

    // exercise derivation paths
    let _ = oneseed::derive::raw(&seed, realm, path, 32);
    let _ = oneseed::derive::uuid(&seed, realm, path);
    let _ = oneseed::derive::integer(&seed, realm, path, 0, 1000);
});
