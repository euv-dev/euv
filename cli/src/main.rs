use euv_cli::*;

use clap::Parser;

/// Entry point for the euv CLI.
///
/// Parses command-line arguments and dispatches to the appropriate
/// mode (run/build/fmt).
///
/// - `run` — build + file watcher + dev server
/// - `build` — build only
/// - `fmt` — format euv macro invocations
///
/// # Returns
///
/// - `Result<(), EuvError>` - `Ok(())` after the selected mode finished.
#[tokio::main]
pub async fn main() -> Result<(), EuvError> {
    Logger::init(log::LevelFilter::Info);
    let cli: Cli = Cli::parse();
    match cli.get_command() {
        Mode::Run(mode_args) => {
            run_mode(mode_args.clone()).await?;
        }
        Mode::Build(mode_args) => {
            build_mode(mode_args.clone()).await?;
        }
        Mode::Fmt(fmt_args) => {
            fmt_mode(fmt_args.clone()).await?;
        }
    }
    Ok(())
}
