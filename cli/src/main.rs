use std::process::exit;

use {
    clap::Parser,
    euv_cli::{Cli, execute_cli},
};

/// Body of the `main` free function.
fn main() {
    let cli: Cli = Cli::parse();
    match execute_cli(&cli) {
        Ok(output) => {
            println!("{output}");
        }
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    }
}
