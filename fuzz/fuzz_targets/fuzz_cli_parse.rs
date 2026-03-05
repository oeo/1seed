#![no_main]
use libfuzzer_sys::fuzz_target;
use oneseed::cli::Cli;

fuzz_target!(|data: &[u8]| {
    let input = match std::str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => return,
    };

    // split on whitespace to simulate argv
    let args: Vec<&str> = std::iter::once("1seed")
        .chain(input.split_whitespace())
        .collect();

    // just test parsing, not execution
    let _ = <Cli as clap::Parser>::try_parse_from(args);
});
