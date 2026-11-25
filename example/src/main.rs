use std::{env, thread::sleep, time::Duration};

use {
    euv::{FrameStats, RenderMode},
    euv_example::{
        FLAG_HARDWARE, FLAG_HEADLESS, FLAG_HYBRID, FLAG_INTERACTIVE, FLAG_SOFTWARE, ROUTE_GAME,
        create_showcase_app, run_headless_smoke,
    },
};

/// Body of the `main` free function.
fn main() {
    let args: Vec<String> = env::args().collect();
    let headless: bool = args.iter().any(|arg: &String| arg == FLAG_HEADLESS);
    let interactive: bool = args.iter().any(|arg: &String| arg == FLAG_INTERACTIVE);
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
            Ok((mut app, state)) => {
                if interactive {
                    let mut last_stats: FrameStats = FrameStats::default();
                    while app.get_running() {
                        if state.get_route().get_untracked() == ROUTE_GAME {
                            let _: u32 = state.get_game_host().step(1.0 / 60.0);
                        }
                        match app.step_game_frame(1.0 / 60.0) {
                            Ok(stats) => last_stats = stats,
                            Err(err) => {
                                eprintln!("{err}");
                                break;
                            }
                        }
                        sleep(Duration::from_millis(8));
                    }
                    print_stats(&last_stats);
                } else if let Ok(stats) = app.run_frames(Some(1)) {
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
