use std::process::ExitCode;

fn main() -> ExitCode {
    match oneseed::cli::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
