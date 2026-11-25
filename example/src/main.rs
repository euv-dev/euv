use std::env;

use {
    euv::{FrameStats, RenderMode},
    euv_example::{
        FLAG_HARDWARE, FLAG_HEADLESS, FLAG_HYBRID, FLAG_SOFTWARE, create_showcase_app,
        run_headless_smoke,
    },
};

/// Body of the `main` free function.
fn main() {
    let args: Vec<String> = env::args().collect();
    let headless: bool = args.iter().any(|arg: &String| arg == FLAG_HEADLESS);
    let mode: RenderMode = if args.iter().any(|arg: &String| arg == FLAG_SOFTWARE) {
        RenderMode::Software
    } else if args.iter().any(|arg: &String| arg == FLAG_HARDWARE) {
        RenderMode::Hardware
    } else if args.iter().any(|arg: &String| arg == FLAG_HYBRID) {
        RenderMode::Hybrid
    } else {
        RenderMode::Auto
    };
    if headless {
        match run_headless_smoke(mode) {
            Ok(stats) => {
                print_stats(&stats);
            }
            Err(err) => {
                eprintln!("{err}");
            }
        }
    } else {
        match create_showcase_app(false, mode) {
            Ok((mut app, _)) => {
                if let Ok(stats) = app.step_frame() {
                    print_stats(&stats);
                }
            }
            Err(err) => {
                eprintln!("{err}");
            }
        }
    }
}

/// Body of the `print_stats` free function.
///
/// # Arguments
///
/// - `&FrameStats` - Shared reference to a `FrameStats`.
fn print_stats(stats: &FrameStats) {
    println!(
        "backend={:?} draw_commands={} draw_batches={} layout_nodes={} frame_hash={:#018x}",
        stats.get_active_backend(),
        stats.get_draw_commands(),
        stats.get_draw_batches(),
        stats.get_layout_nodes(),
        stats.get_frame_hash()
    );
}
