mod r#const;
mod r#fn;
mod r#struct;

pub use lombok_macros::*;
pub use {r#const::*, r#fn::*, r#struct::*};

use std::{
    env,
    iter::Skip,
    path::{Path, PathBuf},
    process::{Command, ExitCode, exit},
};

/// CLI entry point: parses arguments, runs the build, and maps the
/// outcome onto a process exit code (2 for bad usage, 1 for build failure,
/// 0 on success).
///
/// # Returns
///
/// - `ExitCode` - `0` on success, `1` when the build fails, `2` when
///   the command line is unusable.
fn main() -> ExitCode {
    let args: Args = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("euv-docs: {err}");
            print_usage();
            return ExitCode::from(2);
        }
    };
    if let Err(err) = run(&args) {
        eprintln!("euv-docs: {err}");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}
