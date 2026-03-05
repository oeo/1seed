#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 3 {
        return;
    }

    let seed = oneseed::seed::Seed::from_passphrase("fuzz-seed").unwrap();

    // use first byte for length (clamped to valid range)
    let length = (data[0] as usize % 125) + 4; // 4..128
    let counter = data[1] as u32;
    let site = std::str::from_utf8(&data[2..]).unwrap_or("site");

    // test with and without symbols
    let _ = oneseed::password::derive(&seed, "realm", site, counter, length, true, "");
    let _ = oneseed::password::derive(&seed, "realm", site, counter, length, false, "");

    // test with custom symbol set
    let symbols = std::str::from_utf8(&data[2..]).unwrap_or("");
    let _ = oneseed::password::derive(&seed, "realm", "site", 1, 16, true, symbols);
});
