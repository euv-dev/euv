use euv_performance::run;
use std::process::ExitCode;

/// Body of the `main` free function.
///
/// # Returns
///
/// - `ExitCode` - A `ExitCode` value.
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
