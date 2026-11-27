#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::{
    env,
    hint::spin_loop,
    thread::sleep,
    time::{Duration, Instant},
};

#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
use euv_automation::{AiAppHooks, AiSchemaProtocol, DEV_AI_INSPECTOR_ADDR, DevAutomationBridge};
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
use euv_example::{FLAG_AI_QUERY, FLAG_AI_SCHEMA};
#[cfg(not(target_arch = "wasm32"))]
use euv_example::{FLAG_CAPTURE_DIR, capture_showcase_screenshots};
use {
    euv::{FrameStats, RenderMode},
    euv_example::{
        FLAG_HARDWARE, FLAG_HEADLESS, FLAG_HYBRID, FLAG_INTERACTIVE, FLAG_SOFTWARE, ROUTE_GAME,
        ROUTE_GAME_3D, create_showcase_app, run_headless_smoke,
    },
};

/// Body of the `main` free function.
fn main() {
    let args: Vec<String> = env::args().collect();
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(pos) = args.iter().position(|arg: &String| arg == FLAG_CAPTURE_DIR)
        && let Some(dir) = args.get(pos + 1)
    {
        match capture_showcase_screenshots(Path::new(dir)) {
            Ok(n) => println!("captured={n} dir={dir}"),
            Err(err) => eprintln!("{err}"),
        }
        return;
    }
    #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
    if args.iter().any(|arg: &String| arg == FLAG_AI_SCHEMA) {
        if let Ok((mut app, state)) = create_showcase_app(true, RenderMode::Software) {
            drop(app.step_frame());
            let hooks: AiAppHooks = state.build_ai_hooks();
            println!(
                "{}",
                AiSchemaProtocol::snapshot_pretty_json_with_hooks(&app, Some(&hooks))
            );
        }
        return;
    }
    #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
    if let Some(pos) = args.iter().position(|arg: &String| arg == FLAG_AI_QUERY)
        && let Some(query_json) = args.get(pos + 1)
    {
        if let Ok((mut app, state)) = create_showcase_app(true, RenderMode::Software) {
            drop(app.step_frame());
            let hooks: AiAppHooks = state.build_ai_hooks();
            println!(
                "{}",
                AiSchemaProtocol::handle_json_with_hooks(&mut app, query_json, Some(&hooks))
            );
        }
        return;
    }
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
                    #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
                    let mut dev_bridge: DevAutomationBridge =
                        DevAutomationBridge::start(DEV_AI_INSPECTOR_ADDR)
                            .with_hooks(state.build_ai_hooks());
                    let mut last_stats: FrameStats = FrameStats::default();
                    let mut prev_tick: Instant = Instant::now();
                    while app.get_running() {
                        let frame_start: Instant = Instant::now();
                        let dt: f32 = frame_start
                            .duration_since(prev_tick)
                            .as_secs_f32()
                            .clamp(0.001, 0.05);
                        prev_tick = frame_start;
                        let on_game_route: bool = {
                            let cur: String = state.get_route().get_untracked();
                            cur == ROUTE_GAME || cur == ROUTE_GAME_3D
                        };
                        match app.step_game_frame(dt) {
                            Ok(stats) => last_stats = stats,
                            Err(err) => {
                                eprintln!("{err}");
                                break;
                            }
                        }
                        #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
                        {
                            let _: usize = dev_bridge.poll(&mut app);
                        }
                        let elapsed: Duration = frame_start.elapsed();
                        let active_anim: bool = app.has_active_animations();
                        let busy_game_or_anim: bool = on_game_route || active_anim;
                        let native_hz: f64 =
                            f64::from(app.display_refresh_rate_hz().clamp(24.0, 480.0));
                        let target_hz: f64 = app
                            .fps_limit()
                            .map(|fps: u32| f64::from(fps.max(1)))
                            .unwrap_or(native_hz);
                        let frame_budget: Duration = Duration::from_micros(
                            (1_000_000.0 / target_hz).round().max(1000.0) as u64,
                        );
                        let target: Duration =
                            if busy_game_or_anim || last_stats.get_pixels_shaded() > 0 {
                                frame_budget
                            } else {
                                frame_budget.max(Duration::from_millis(4))
                            };
                        if let Some(remaining) = target.checked_sub(elapsed) {
                            if busy_game_or_anim {
                                let spin_guard: Duration = Duration::from_micros(1500);
                                if let Some(coarse) = remaining.checked_sub(spin_guard) {
                                    sleep(coarse);
                                }
                                while frame_start.elapsed() < target {
                                    spin_loop();
                                }
                            } else {
                                sleep(remaining);
                            }
                        }
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
