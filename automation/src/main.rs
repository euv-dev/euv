#[cfg(not(target_arch = "wasm32"))]
use euv_automation::run;

/// Executes the automation CLI and preserves failures as a nonzero status.
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

/// Leaves browser-fixture builds free of the native runner.
#[cfg(target_arch = "wasm32")]
fn main() {}
