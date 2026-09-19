mod r#const;
mod r#fn;
mod r#struct;

pub use {r#const::*, r#fn::*, r#struct::*};

use std::{
    env,
    iter::Skip,
    path::{Path, PathBuf},
    process::{Command, ExitCode, exit},
};

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
