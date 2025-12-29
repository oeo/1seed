mod cli;
mod config;
mod seed;
mod age;
mod ssh;
mod sign;
mod password;
mod derive;

use std::process::ExitCode;

fn main() -> ExitCode {
    match cli::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
