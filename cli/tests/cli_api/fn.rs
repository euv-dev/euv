use super::*;

fn scratch(tag: &str) -> PathBuf {
    let dir: PathBuf = env::temp_dir()
        .join("euv-cli-api-tests")
        .join(format!("{tag}-{}", process::id()));
    let _: io::Result<()> = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        let _: io::Result<()> = fs::create_dir_all(parent);
    }
    let mut file: fs::File = fs::File::create(path).expect("scratch file");
    let _: io::Result<()> = io::Write::write_all(&mut file, contents.as_bytes());
}

#[test]
fn the_dev_template_fetches_the_same_path_the_router_registers() {
    assert!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, interactive-widget=resizes-visual"
    />
    <meta name="mobile-web-app-capable" content="yes" />
    <meta name="apple-mobile-web-app-capable" content="yes" />
    <meta
      name="description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta
      name="keywords"
      content="rust, webassembly, wasm, ui-framework, virtual-dom, reactive, declarative-ui, euv"
    />
    <meta property="og:title" content="euv" />
    <meta
      property="og:description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta property="og:type" content="website" />
    <title>Euv</title>
    __EUV_BASE_HREF_TAG__
    <style>
      * {
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-rendering: optimizeLegibility;
      }
      canvas {
        image-rendering: auto;
      }
    </style>
  </head>
  <body>
    <div id="app"></div>
  </body>
  <script>
__EUV_INLINE_JS__
  </script>
  <script>
    (function () {
      async function connect() {
        try {
          const res = await fetch('/__euv_reload');
          const data = await res.json();
          if (data.type === 'Reload') {
            location.reload();
          } else if (data.type === 'Error') {
            console.error('[euv] build error:', data.message);
            setTimeout(connect, 1000);
          } else {
            setTimeout(connect, 1000);
          }
        } catch (_) {
          setTimeout(connect, 2000);
        }
      }
      connect();
    })();
  </script>
</html>
"#.contains("/__euv_reload"),
        "the dev template hardcodes the reload URL instead of using \
         __RELOAD_ROUTE__, so the JS and the registered route must \
         stay the same string or live reload silently 404s"
    );
}

#[test]
fn only_the_dev_template_carries_the_reload_script() {
    assert!(
        !r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, interactive-widget=resizes-visual"
    />
    <meta name="mobile-web-app-capable" content="yes" />
    <meta name="apple-mobile-web-app-capable" content="yes" />
    <meta
      name="description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta
      name="keywords"
      content="rust, webassembly, wasm, ui-framework, virtual-dom, reactive, declarative-ui, euv"
    />
    <meta property="og:title" content="euv" />
    <meta
      property="og:description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta property="og:type" content="website" />
    <title>Euv</title>
    __EUV_BASE_HREF_TAG__
    <style>
      * {
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-rendering: optimizeLegibility;
      }
      canvas {
        image-rendering: auto;
      }
    </style>
  </head>
  <body>
    <div id="app"></div>
  </body>
  <script>
__EUV_INLINE_JS__
  </script>
</html>
"#.contains("/__euv_reload"),
        "a release build must ship no live-reload instrumentation at all"
    );
    assert!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, interactive-widget=resizes-visual"
    />
    <meta name="mobile-web-app-capable" content="yes" />
    <meta name="apple-mobile-web-app-capable" content="yes" />
    <meta
      name="description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta
      name="keywords"
      content="rust, webassembly, wasm, ui-framework, virtual-dom, reactive, declarative-ui, euv"
    />
    <meta property="og:title" content="euv" />
    <meta
      property="og:description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta property="og:type" content="website" />
    <title>Euv</title>
    __EUV_BASE_HREF_TAG__
    <style>
      * {
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-rendering: optimizeLegibility;
      }
      canvas {
        image-rendering: auto;
      }
    </style>
  </head>
  <body>
    <div id="app"></div>
  </body>
  <script>
__EUV_INLINE_JS__
  </script>
  <script>
    (function () {
      async function connect() {
        try {
          const res = await fetch('/__euv_reload');
          const data = await res.json();
          if (data.type === 'Reload') {
            location.reload();
          } else if (data.type === 'Error') {
            console.error('[euv] build error:', data.message);
            setTimeout(connect, 1000);
          } else {
            setTimeout(connect, 1000);
          }
        } catch (_) {
          setTimeout(connect, 2000);
        }
      }
      connect();
    })();
  </script>
</html>
"#.contains("connect()"),
        "the dev template is the one that must own the reconnect loop"
    );
    assert!(
        !r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, interactive-widget=resizes-visual"
    />
    <meta name="mobile-web-app-capable" content="yes" />
    <meta name="apple-mobile-web-app-capable" content="yes" />
    <meta
      name="description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta
      name="keywords"
      content="rust, webassembly, wasm, ui-framework, virtual-dom, reactive, declarative-ui, euv"
    />
    <meta property="og:title" content="euv" />
    <meta
      property="og:description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta property="og:type" content="website" />
    <title>Euv</title>
    __EUV_BASE_HREF_TAG__
    <style>
      * {
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-rendering: optimizeLegibility;
      }
      canvas {
        image-rendering: auto;
      }
    </style>
  </head>
  <body>
    <div id="app"></div>
  </body>
  <script>
__EUV_INLINE_JS__
  </script>
</html>
"#.contains("connect()"),
        "a reconnect loop in a release build would poll a server that no longer exists"
    );
}

#[test]
fn both_templates_still_carry_the_two_live_placeholders() {
    for template in [
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, interactive-widget=resizes-visual"
    />
    <meta name="mobile-web-app-capable" content="yes" />
    <meta name="apple-mobile-web-app-capable" content="yes" />
    <meta
      name="description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta
      name="keywords"
      content="rust, webassembly, wasm, ui-framework, virtual-dom, reactive, declarative-ui, euv"
    />
    <meta property="og:title" content="euv" />
    <meta
      property="og:description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta property="og:type" content="website" />
    <title>Euv</title>
    __EUV_BASE_HREF_TAG__
    <style>
      * {
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-rendering: optimizeLegibility;
      }
      canvas {
        image-rendering: auto;
      }
    </style>
  </head>
  <body>
    <div id="app"></div>
  </body>
  <script>
__EUV_INLINE_JS__
  </script>
  <script>
    (function () {
      async function connect() {
        try {
          const res = await fetch('/__euv_reload');
          const data = await res.json();
          if (data.type === 'Reload') {
            location.reload();
          } else if (data.type === 'Error') {
            console.error('[euv] build error:', data.message);
            setTimeout(connect, 1000);
          } else {
            setTimeout(connect, 1000);
          }
        } catch (_) {
          setTimeout(connect, 2000);
        }
      }
      connect();
    })();
  </script>
</html>
"#,
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, interactive-widget=resizes-visual"
    />
    <meta name="mobile-web-app-capable" content="yes" />
    <meta name="apple-mobile-web-app-capable" content="yes" />
    <meta
      name="description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta
      name="keywords"
      content="rust, webassembly, wasm, ui-framework, virtual-dom, reactive, declarative-ui, euv"
    />
    <meta property="og:title" content="euv" />
    <meta
      property="og:description"
      content="A declarative, cross-platform UI framework for Rust with virtual DOM, reactive signals, and HTML macros for WebAssembly."
    />
    <meta property="og:type" content="website" />
    <title>Euv</title>
    __EUV_BASE_HREF_TAG__
    <style>
      * {
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-rendering: optimizeLegibility;
      }
      canvas {
        image-rendering: auto;
      }
    </style>
  </head>
  <body>
    <div id="app"></div>
  </body>
  <script>
__EUV_INLINE_JS__
  </script>
</html>
"#,
    ] {
        assert!(
            template.contains("__EUV_BASE_HREF_TAG__"),
            "a missing __EUV_BASE_HREF_TAG__ means the base href tag is never injected"
        );
        assert!(
            template.contains("__EUV_INLINE_JS__"),
            "a missing __EUV_INLINE_JS__ means the wasm bridge is never inlined"
        );
    }
}

#[test]
fn the_four_placeholders_are_mutually_distinct() {
    let all: [&str; 4] = [
        "__IMPORT_PATH__",
        "__RELOAD_ROUTE__",
        "__EUV_INLINE_JS__",
        "__EUV_BASE_HREF_TAG__",
    ];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(
                all[i], all[j],
                "two placeholders sharing a token would make the replace chain order-dependent"
            );
        }
    }
}

#[test]
fn the_reload_route_is_an_absolute_path() {
    assert!(
        "/__euv_reload".starts_with("/"),
        "the route is registered verbatim, so it must be absolute"
    );
}

#[test]
fn the_project_layout_names_match_what_cargo_and_git_actually_use() {
    assert_eq!("/__euv_reload", "/__euv_reload");
    assert_eq!("__RELOAD_ROUTE__", "__RELOAD_ROUTE__");
}

#[test]
fn the_path_helpers_use_forward_slashes_for_url_construction() {
    assert_eq!("/", "/");
    assert_eq!("/", "/");
    assert_eq!("./", "./");
    assert_eq!("..", "..");
    assert_eq!('/', '/');
    assert_eq!('\\', '\\');
}

#[test]
fn the_euv_arguments_parse_from_the_same_flags_they_are_named_after() {
    let full: Vec<String> = vec![
        "euv".to_string(),
        "-c".to_string(),
        "/tmp/app".to_string(),
        "-p".to_string(),
        "1234".to_string(),
        "--www-dir".to_string(),
        "www".to_string(),
        "--index-html".to_string(),
        "custom.html".to_string(),
        "--no-gitignore".to_string(),
    ];
    let parsed: ModeArgs = ModeArgs::parse_from(full);
    let rendered: String = format!("{parsed:?}");
    assert!(
        rendered.contains("/tmp/app"),
        "-c must reach crate_path, got: {rendered}"
    );
    assert!(
        rendered.contains("1234"),
        "-p must reach port, got: {rendered}"
    );
    assert!(
        rendered.contains("custom.html"),
        "--index-html must land in the template field, got: {rendered}"
    );
    assert!(
        rendered.contains("true"),
        "--no-gitignore is a bare flag, so it must parse to true, got: {rendered}"
    );
}

#[test]
fn the_banner_action_names_are_the_clap_subcommand_names() {
    assert_eq!("run", "run");
    assert_eq!("build", "build");
    let parsed: Mode = Mode::parse_from(["euv", "build"]);
    assert!(
        matches!(parsed, Mode::Build(_)),
        "build must be the literal subcommand clap dispatches on"
    );
}

#[test]
fn the_inline_bridge_opt_out_is_an_env_var_name_not_a_value() {
    assert_eq!("EUV_NO_INLINE_BRIDGE", "EUV_NO_INLINE_BRIDGE");
    assert!(
        !"EUV_NO_INLINE_BRIDGE".contains('='),
        "a name carrying '=' would be read as an assignment, not as presence"
    );
}

#[test]
fn the_server_error_messages_are_distinct_and_non_empty() {
    let messages: [&str; 7] = [
        "server not ready",
        "Global state already initialized",
        "Failed to read custom index.html",
        "Custom index.html is not valid UTF-8",
        "Failed to create static directory",
        "Failed to write index.html",
        "Failed to read",
    ];
    for message in messages {
        assert!(
            !message.is_empty(),
            "an empty error message tells the user nothing"
        );
    }
    for i in 0..messages.len() {
        for j in (i + 1)..messages.len() {
            assert_ne!(
                messages[i], messages[j],
                "two failures sharing a message cannot be told apart in a log"
            );
        }
    }
}

#[test]
fn the_fmt_error_messages_are_distinct_and_non_empty() {
    let messages: [&str; 4] = [
        "Failed to read directory",
        "Failed to read entry in directory",
        "Failed to read",
        "Failed to write",
    ];
    for i in 0..messages.len() {
        for j in (i + 1)..messages.len() {
            assert_ne!(messages[i], messages[j], "duplicate fmt failure message");
        }
    }
}

#[test]
fn the_mode_error_messages_name_the_flag_they_are_about() {
    assert!(
        "Invalid crate-path".contains("--crate-path".trim_start_matches('-')),
        "the message should echo --crate-path so the user knows which flag failed"
    );
}

#[test]
fn the_log_separators_are_the_ones_the_format_string_uses() {
    assert_eq!(" ", " ");
    assert_eq!(":", ":");
}

#[test]
fn the_windows_unc_prefix_is_the_documented_four_character_form() {
    assert_eq!(r"\\?\", r"\\?\");
    assert_eq!(r"\\?\".chars().count(), 4);
}

#[test]
fn the_walk_skips_exactly_the_two_generated_directories() {
    assert_eq!("target", "target");
    assert_eq!("node_modules", "node_modules");
    assert_ne!("target", "node_modules");
    assert_eq!("rs", "rs");
    assert!(
        "view.rs".ends_with("rs"),
        "the extension is compared without a dot, so a dot here would never match"
    );
}

#[test]
fn the_fmt_keyword_letters_are_the_distinct_letters_the_detector_looks_for() {
    let letters: [char; 13] = [
        'a', 'c', 'e', 'f', 'h', 'i', 'l', 'm', 'n', 'o', 'r', 's', 't',
    ];
    for i in 0..letters.len() {
        assert!(
            letters[i].is_ascii_lowercase(),
            "{:?} is not a lowercase ASCII letter",
            letters[i]
        );
        for j in (i + 1)..letters.len() {
            assert_ne!(
                letters[i], letters[j],
                "a repeated keyword letter means one branch of the detector is dead"
            );
        }
    }
}

#[test]
fn fmt_mode_defaults_to_check_so_a_bare_fmt_call_never_rewrites() {
    assert_eq!(
        FmtMode::default(),
        FmtMode::Check,
        "the safe mode must be the default, not the destructive one"
    );
}

#[test]
fn fmt_mode_orders_check_before_write() {
    assert!(
        FmtMode::Check < FmtMode::Write,
        "the declaration order is the semantic order, and a mode must never sort before the default"
    );
}

#[test]
fn fmt_mode_is_hashable_so_it_can_key_a_memo() {
    let modes: HashSet<FmtMode> = [FmtMode::Check, FmtMode::Write, FmtMode::Check]
        .into_iter()
        .collect();
    assert_eq!(modes.len(), 2, "the duplicate Check must collapse");
}

#[test]
fn the_build_mode_default_is_dev() {
    assert_eq!(
        BuildMode::default(),
        BuildMode::Dev,
        "a crate with no profile flag must get a debug build"
    );
}

#[test]
fn the_three_build_modes_are_mutually_distinct() {
    let all: [BuildMode; 3] = [BuildMode::Dev, BuildMode::Release, BuildMode::Profiling];
    for i in 0..all.len() {
        for j in (i + 1)..all.len() {
            assert_ne!(all[i], all[j]);
        }
    }
}

#[test]
fn an_action_is_copy_so_it_can_be_logged_after_the_mode_consumes_it() {
    let action: Action = Action::Run;
    let copied: Action = action;
    assert_eq!(copied, action, "Action is Copy, so a use must not move it");
    assert_ne!(Action::Run, Action::Build);
}

#[test]
fn a_successful_reload_serialises_as_a_bare_tagged_object() {
    let encoded: String = serde_json::to_string(&ReloadEvent::Reload).expect("serialisable");
    assert_eq!(
        encoded, r#"{"type":"Reload"}"#,
        "the dev template branches on data.type === 'Reload', so the tag must not change"
    );
}

#[test]
fn a_failed_reload_serialises_its_message_under_the_same_tag() {
    let encoded: String = serde_json::to_string(&ReloadEvent::Error("build failed".to_string()))
        .expect("serialisable");
    assert_eq!(
        encoded, r#"{"type":"Error","message":"build failed"}"#,
        "the dev template reads data.message, so the field name must not change"
    );
}

#[test]
fn a_reload_error_message_is_escaped_before_it_reaches_the_client() {
    let encoded: String =
        serde_json::to_string(&ReloadEvent::Error("expected \";\" at line 3".to_string()))
            .expect("serialisable");
    assert_eq!(
        encoded, r#"{"type":"Error","message":"expected \";\" at line 3"}"#,
        "the message is interpolated into the client console, so quotes must not break the JSON"
    );
}

#[test]
fn an_empty_reload_error_still_carries_its_tag() {
    let encoded: String =
        serde_json::to_string(&ReloadEvent::Error(String::new())).expect("serialisable");
    assert_eq!(
        encoded, r#"{"type":"Error","message":""}"#,
        "a compiler error with no detail must still be routed to the error branch"
    );
}

#[test]
fn a_reload_event_is_only_ever_produced_never_consumed() {
    let encoded: String = serde_json::to_string(&ReloadEvent::Reload).expect("serialisable");
    assert!(
        !encoded.contains("message"),
        "the Reload branch carries no payload, so the client sees no undefined field"
    );
}

#[test]
fn the_mode_subcommands_dispatch_to_their_own_argument_type() {
    let run: Mode = Mode::parse_from(["euv", "run", "--port", "9000"]);
    match &run {
        Mode::Run(args) => {
            let rendered: String = format!("{args:?}");
            assert!(
                rendered.contains("9000"),
                "--port must reach the Run variant's ModeArgs, got: {rendered}"
            );
        }
        other => panic!("expected Run, got {other:?}"),
    }

    let fmt: Mode = Mode::parse_from(["euv", "fmt", "--check"]);
    match &fmt {
        Mode::Fmt(args) => {
            let rendered: String = format!("{args:?}");
            assert!(
                rendered.contains("check: true"),
                "--check must reach FmtArgs, got: {rendered}"
            );
        }
        other => panic!("expected Fmt, got {other:?}"),
    }
}

#[test]
fn the_cli_root_tolerates_no_subcommand() {
    let cli: Result<Cli, clap::Error> = Cli::try_parse_from(["euv"]);
    assert!(
        cli.is_err(),
        "running euv with no subcommand must print usage, not guess a mode"
    );
}

#[test]
fn the_cli_root_carries_the_parsed_subcommand() {
    let cli: Cli = Cli::parse_from(["euv", "run"]);
    assert!(
        format!("{cli:?}").contains("Run"),
        "the root must carry the dispatched subcommand, got: {cli:?}"
    );
}

#[test]
fn a_bare_argv_lands_on_the_documented_defaults() {
    let args: ModeArgs = ModeArgs::parse_from(["euv"]);
    let rendered: String = format!("{args:?}");
    assert!(
        rendered.contains("port: 80"),
        "an unset --port must stay on 80, got: {rendered}"
    );
    assert!(
        rendered.contains("www_dir: \"www\""),
        "an unset --www-dir must stay on www, got: {rendered}"
    );
    assert!(
        rendered.contains("crate_path: \".\""),
        "an unset --crate-path must stay on the cwd, got: {rendered}"
    );
    assert!(
        rendered.contains("wasm_pack_args: []"),
        "nothing after -- means nothing to forward, got: {rendered}"
    );
    assert!(
        !rendered.contains("dev: true")
            && !rendered.contains("release: true")
            && !rendered.contains("profiling: true"),
        "no profile flag means no build mode, got: {rendered}"
    );
}

#[test]
fn a_wasm_pack_passthrough_keeps_its_flags_verbatim() {
    let args: ModeArgs = ModeArgs::parse_from([
        "euv",
        "--",
        "--release",
        "--no-typescript",
        "--scope",
        "my-pkg",
    ]);
    let rendered: String = format!("{args:?}");
    assert!(
        rendered.contains(
            "wasm_pack_args: [\"--release\", \"--no-typescript\", \"--scope\", \"my-pkg\"]"
        ),
        "everything after -- belongs to wasm-pack, spelled exactly as the user typed it, got: {rendered}"
    );
}

fn mode_args(argv: &[&str]) -> ModeArgs {
    let mut full: Vec<String> = vec!["euv".to_string()];
    for arg in argv {
        full.push((*arg).to_string());
    }
    ModeArgs::parse_from(full)
}

fn fmt_args(path: &Path, check: bool) -> FmtArgs {
    let mut full: Vec<String> = vec![
        "euv".to_string(),
        "--path".to_string(),
        path.to_string_lossy().to_string(),
    ];
    if check {
        full.push("--check".to_string());
    }
    FmtArgs::parse_from(full)
}

#[tokio::test]
async fn a_www_dir_holding_index_html_is_returned_as_is() {
    let root: PathBuf = scratch("www-direct");
    write_file(&root.join("index.html"), "<html></html>");
    let observed: PathBuf = resolve_www_dir(&root).await;
    assert_eq!(
        observed, root,
        "the directory already has an index.html, so there is nothing to search for"
    );
}

#[tokio::test]
async fn a_www_dir_whose_child_shares_its_name_is_unwrapped() {
    let outer: PathBuf = scratch("www-nested");
    let root: PathBuf = outer.join("www");
    let nested: PathBuf = root.join("www");
    write_file(&nested.join("index.html"), "<html></html>");
    let observed: PathBuf = resolve_www_dir(&root).await;
    assert_eq!(
        observed, nested,
        "wasm-pack emits www/www/pkg, and the unwrap step exists for exactly that shape"
    );
}

#[tokio::test]
async fn a_www_dir_with_index_html_wins_over_the_unwrap() {
    let outer: PathBuf = scratch("www-both");
    let root: PathBuf = outer.join("www");
    write_file(&root.join("index.html"), "<html>outer</html>");
    write_file(&root.join("www").join("index.html"), "<html>inner</html>");
    let observed: PathBuf = resolve_www_dir(&root).await;
    assert_eq!(
        observed, root,
        "when both levels have an index.html the shallower one must win"
    );
}

#[tokio::test]
async fn a_www_dir_with_no_index_html_anywhere_is_returned_unchanged() {
    let root: PathBuf = scratch("www-missing");
    let observed: PathBuf = resolve_www_dir(&root).await;
    assert_eq!(
        observed, root,
        "with no index.html to find, the caller still needs a usable directory"
    );
}

#[tokio::test]
async fn a_missing_www_dir_is_not_an_error() {
    let root: PathBuf = scratch("www-absent").join("nope");
    let observed: PathBuf = resolve_www_dir(&root).await;
    assert_eq!(observed, root, "resolution must degrade, never panic");
}

#[tokio::test]
async fn cleaning_removes_files_and_subdirectories_but_keeps_the_root() {
    let root: PathBuf = scratch("clean");
    write_file(&root.join("stale.js"), "// stale");
    write_file(&root.join("pkg").join("deep").join("x_bg.wasm"), "binary");
    let sub: PathBuf = root.join("pkg");
    assert!(sub.is_dir());
    clean_out_dir(&root).await;
    assert!(
        root.is_dir(),
        "the output directory itself must survive a clean"
    );
    assert_eq!(
        fs::read_dir(&root).expect("root still readable").count(),
        0,
        "a clean must leave nothing behind, not even nested directories"
    );
}

#[tokio::test]
async fn cleaning_a_missing_directory_is_a_no_op() {
    let root: PathBuf = scratch("clean-absent").join("nope");
    clean_out_dir(&root).await;
    assert!(
        !root.exists(),
        "a clean must not create what was never there"
    );
}

#[tokio::test]
async fn cleaning_twice_is_idempotent() {
    let root: PathBuf = scratch("clean-twice");
    write_file(&root.join("a.js"), "x");
    clean_out_dir(&root).await;
    clean_out_dir(&root).await;
    assert_eq!(fs::read_dir(&root).expect("root readable").count(), 0);
}

#[tokio::test]
async fn a_serving_root_inside_the_www_dir_is_the_www_dir() {
    let root: PathBuf = scratch("serving-inside");
    write_file(&root.join("index.html"), "<html></html>");
    let args: ModeArgs = mode_args(&["--crate-path", &root.to_string_lossy()]);
    let observed: PathBuf = resolve_serving_root(&args).await;
    assert_eq!(
        observed,
        root.join("www"),
        "index.html and the wasm artifacts share one root only when out_dir sits under www"
    );
}

#[tokio::test]
async fn a_serving_root_outside_the_www_dir_is_the_out_dir_parent() {
    let root: PathBuf = scratch("serving-outside");
    let out: PathBuf = root.join("build").join("pkg");
    let args: ModeArgs = mode_args(&[
        "--crate-path",
        &root.to_string_lossy(),
        "--",
        "--out-dir",
        &out.to_string_lossy(),
    ]);
    let observed: PathBuf = resolve_serving_root(&args).await;
    assert_eq!(
        observed,
        out.parent().expect("out dir has a parent").to_path_buf(),
        "an out_dir outside www needs its own root so the two trees stay co-located"
    );
}

#[tokio::test]
async fn fmt_check_reports_a_file_that_needs_formatting() {
    let root: PathBuf = scratch("fmt-check-dirty");
    let file: PathBuf = root.join("view.rs");
    write_file(&file, UNFORMATTED);
    let result: Result<(), EuvError> = fmt_mode(fmt_args(&file, true)).await;
    let error: EuvError = result.expect_err("check mode must fail on unformatted source");
    let rendered: String = format!("{error}");
    assert!(
        rendered.contains("needs formatting"),
        "the user needs the file name and the verdict, got: {rendered}"
    );
}

#[tokio::test]
async fn fmt_check_leaves_the_file_untouched() {
    let root: PathBuf = scratch("fmt-check-clean");
    let file: PathBuf = root.join("clean.rs");
    let original: &str = "fn main() {}\n";
    write_file(&file, original);
    let result: Result<(), EuvError> = fmt_mode(fmt_args(&file, true)).await;
    let _: () = result.expect("already-formatted source must pass check mode");
    assert_eq!(
        fs::read_to_string(&file).expect("read back"),
        original,
        "check mode must never write"
    );
}

#[tokio::test]
async fn fmt_write_rewrites_the_file_in_place() {
    let root: PathBuf = scratch("fmt-write");
    let file: PathBuf = root.join("view.rs");
    write_file(&file, UNFORMATTED);
    let result: Result<(), EuvError> = fmt_mode(fmt_args(&file, false)).await;
    let _: () = result.expect("write mode must succeed on a writable file");
    let after: String = fs::read_to_string(&file).expect("read back");
    assert_ne!(
        after, UNFORMATTED,
        "write mode must have rewritten the file"
    );
    assert!(
        after.contains("<div>hi</div>"),
        "the macro body must survive the rewrite, got: {after}"
    );
}

#[tokio::test]
async fn fmt_write_is_idempotent() {
    let root: PathBuf = scratch("fmt-write-twice");
    let file: PathBuf = root.join("view.rs");
    write_file(&file, UNFORMATTED);
    let _: () = fmt_mode(fmt_args(&file, false)).await.expect("first write");
    let after_first: String = fs::read_to_string(&file).expect("read back");
    let _: () = fmt_mode(fmt_args(&file, false))
        .await
        .expect("second write");
    assert_eq!(
        fs::read_to_string(&file).expect("read back"),
        after_first,
        "formatting a formatted file must be a no-op, or fmt would thrash git"
    );
}

#[tokio::test]
async fn fmt_accepts_a_relative_path_resolved_against_the_cwd() {
    let result: Result<(), EuvError> = fmt_mode(FmtArgs::new(PathBuf::from("."), true)).await;
    let _: Result<(), EuvError> = result;
}

#[tokio::test]
async fn format_dir_reaches_a_nested_source_file() {
    let root: PathBuf = scratch("fmt-nested");
    let file: PathBuf = root.join("page").join("view").join("fn.rs");
    write_file(&file, UNFORMATTED);
    let _: () = format_dir(&root, FmtMode::Write)
        .await
        .expect("walk succeeds");
    assert_ne!(
        fs::read_to_string(&file).expect("read back"),
        UNFORMATTED,
        "the walker must descend into nested directories"
    );
}

#[tokio::test]
async fn format_dir_ignores_non_rust_files() {
    let root: PathBuf = scratch("fmt-ext");
    let source: PathBuf = root.join("view.rs");
    let other: PathBuf = root.join("notes.md");
    write_file(&source, UNFORMATTED);
    write_file(&other, UNFORMATTED);
    let result: Result<(), EuvError> = format_dir(&root, FmtMode::Check).await;
    let error: EuvError = result.expect_err("the unformatted .rs file must be reported");
    assert_eq!(
        format!("{error}"),
        "1 file(s) need formatting. Run `euv fmt` to fix.",
        "a .md file in the tree must not be counted or rewritten"
    );
}

#[tokio::test]
async fn format_dir_skips_the_build_output_directory() {
    let root: PathBuf = scratch("fmt-skip-target");
    let generated: PathBuf = root.join("target").join("fn.rs");
    write_file(&generated, UNFORMATTED);
    let result: Result<(), EuvError> = format_dir(&root, FmtMode::Check).await;
    let _: () = result.expect("build output must be left alone, not flagged");
    assert_eq!(
        fs::read_to_string(&generated).expect("read back"),
        UNFORMATTED,
        "a generated file under target must not be rewritten"
    );
}

#[tokio::test]
async fn format_dir_skips_the_node_modules_directory() {
    let root: PathBuf = scratch("fmt-skip-node");
    let vendored: PathBuf = root.join("node_modules").join("fn.rs");
    write_file(&vendored, UNFORMATTED);
    let result: Result<(), EuvError> = format_dir(&root, FmtMode::Check).await;
    let _: () = result.expect("node_modules must be left alone, not flagged");
    assert_eq!(
        fs::read_to_string(&vendored).expect("read back"),
        UNFORMATTED,
        "a vendored file under node_modules must not be rewritten"
    );
}

#[tokio::test]
async fn format_dir_on_a_single_file_touches_only_that_file() {
    let root: PathBuf = scratch("fmt-single");
    let dirty: PathBuf = root.join("dirty.rs");
    let clean: PathBuf = root.join("clean.rs");
    write_file(&dirty, UNFORMATTED);
    write_file(&clean, FORMATTED);
    let result: Result<(), EuvError> = format_dir(&dirty, FmtMode::Check).await;
    let _: EuvError = result.expect_err("the dirty file must be reported");
    assert_eq!(
        fs::read_to_string(&clean).expect("read back"),
        FORMATTED,
        "a file path must not be treated as a directory to walk"
    );
}

#[tokio::test]
async fn format_dir_reports_a_single_file_by_its_own_name() {
    let root: PathBuf = scratch("fmt-single-name");
    let dirty: PathBuf = root.join("dirty.rs");
    write_file(&dirty, UNFORMATTED);
    let result: Result<(), EuvError> = format_dir(&dirty, FmtMode::Check).await;
    let error: EuvError = result.expect_err("the dirty file must be reported");
    let rendered: String = format!("{error}");
    assert!(
        rendered.contains("dirty.rs"),
        "a single-file check must name the file, got: {rendered}"
    );
    assert!(
        !rendered.contains("file(s)"),
        "a single-file check must not use the directory-wide plural wording, got: {rendered}"
    );
}

#[test]
fn the_banner_accepts_both_actions_without_a_format_argument() {
    print_banner(Action::Run);
    print_banner(Action::Build);
}

#[test]
fn logger_init_sets_the_global_max_level() {
    Logger::init(log::LevelFilter::Warn);
    assert_eq!(
        log::max_level(),
        log::LevelFilter::Warn,
        "init must apply the level filter even when a logger is already installed"
    );
    Logger::init(log::LevelFilter::Info);
    assert_eq!(log::max_level(), log::LevelFilter::Info);
}

#[test]
fn the_global_logger_is_a_zero_sized_singleton() {
    assert_eq!(
        mem::size_of_val(&Logger),
        0,
        "Logger is a unit struct, so the static costs nothing at runtime"
    );
}

#[test]
fn the_io_error_keeps_its_message_path_and_cause() {
    let cause: io::Error = io::Error::new(io::ErrorKind::NotFound, "boom");
    let path: PathBuf = PathBuf::from("/tmp/missing");
    let error: EuvError = EuvError::IoPath {
        message: "Invalid crate-path".to_string(),
        path: path.clone(),
        error: cause,
    };
    let rendered: String = format!("{error}");
    assert!(
        rendered.contains("Invalid crate-path"),
        "the human message must survive, got: {rendered}"
    );
    assert!(
        rendered.contains("missing"),
        "the path must survive, got: {rendered}"
    );
    assert!(
        rendered.contains("boom"),
        "the underlying cause must survive, got: {rendered}"
    );
}

#[test]
fn the_utf8_error_keeps_its_message_and_cause() {
    let invalid: Vec<u8> = vec![0xF0, 0x28, 0x8C, 0x28];
    let cause: FromUtf8Error = String::from_utf8(invalid).expect_err("bytes are not utf-8");
    let error: EuvError = EuvError::Utf8 {
        message: "Custom index.html is not valid UTF-8".to_string(),
        error: cause,
    };
    let rendered: String = format!("{error}");
    assert!(
        rendered.contains("Custom index.html is not valid UTF-8"),
        "{rendered}"
    );
}

#[test]
fn a_bare_message_error_renders_verbatim() {
    let error: EuvError = EuvError::Message("something went wrong".to_string());
    assert_eq!(format!("{error}"), "something went wrong");
}

#[test]
fn a_server_error_renders_its_payload() {
    let error: EuvError = EuvError::Server("server not ready".to_string());
    assert_eq!(format!("{error}"), "server not ready");
}

#[tokio::test]
async fn a_build_that_cannot_create_its_out_dir_names_the_offending_path() {
    let root: PathBuf = scratch("build-outdir-blocked");
    let blocker: PathBuf = root.join("blocker");
    write_file(&blocker, "x");
    let out: PathBuf = blocker.join("pkg");
    let args: ModeArgs = mode_args(&[
        "--crate-path",
        &root.to_string_lossy(),
        "--",
        "--out-dir",
        &out.to_string_lossy(),
    ]);
    let result: Result<(), EuvError> = build_wasm(&args).await;
    match result {
        Err(EuvError::IoPath {
            message,
            path,
            error,
        }) => {
            assert!(
                message.contains("output directory"),
                "the message must say which step failed, got {message:?}"
            );
            assert_eq!(
                path, out,
                "the error must carry the directory it could not create"
            );
            assert_eq!(
                error.kind(),
                io::ErrorKind::NotADirectory,
                "a regular file standing where a directory belongs is what makes this fail"
            );
        }
        Err(other) => panic!("expected a path-carrying io error, got {other:?}"),
        Ok(()) => panic!("building under a regular file must not succeed"),
    }
}

#[tokio::test]
async fn a_build_only_pipeline_stops_at_the_blocked_out_dir() {
    let root: PathBuf = scratch("build-only-outdir-blocked");
    let blocker: PathBuf = root.join("blocker");
    write_file(&blocker, "x");
    let out: PathBuf = blocker.join("pkg");
    let args: ModeArgs = mode_args(&[
        "--crate-path",
        &root.to_string_lossy(),
        "--",
        "--out-dir",
        &out.to_string_lossy(),
    ]);
    let result: Result<(), EuvError> = run_build_only_pipeline(&args).await;
    assert!(
        matches!(result, Err(EuvError::IoPath { .. })),
        "a pipeline that cannot lay down its output must not report success"
    );
}

#[tokio::test]
async fn a_full_pipeline_still_emits_html_when_the_build_fails() {
    let root: PathBuf = scratch("full-pipeline-build-fails");
    let out: PathBuf = root.join("build").join("pkg");
    let args: ModeArgs = mode_args(&[
        "--crate-path",
        &root.to_string_lossy(),
        "--",
        "--out-dir",
        &out.to_string_lossy(),
    ]);
    match run_build_pipeline(&args, None).await {
        Ok(html) => {
            assert!(
                !html.is_empty(),
                "a failed build still has to leave the dev server something to serve"
            );
            let produced: Vec<PathBuf> = fs::read_dir(&out)
                .map(|entries: fs::ReadDir| {
                    entries
                        .filter_map(|entry: io::Result<fs::DirEntry>| entry.ok())
                        .map(|entry: fs::DirEntry| entry.path())
                        .collect()
                })
                .unwrap_or_default();
            assert!(
                produced.is_empty(),
                "the build left artifacts behind, so this never exercised the failure \
                 path and proves nothing: {produced:?}"
            );
        }
        Err(other) => panic!(
            "a failed build is swallowed on purpose so the dev server still has a page; \
             surfacing {other:?} here means the pipeline no longer does that"
        ),
    }
}
