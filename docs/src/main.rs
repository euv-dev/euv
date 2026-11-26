use std::{env, path::PathBuf};

use euv_docs::{
    DOCS_FLAG_GENERATE, DOCS_FLAG_HEADLESS, DOCS_FLAG_INTERACTIVE, DocSite, DocsBuildOptions,
    GeneratedSiteReport, build_docs_bundle, load_doc_site_with_options, render_docs_app,
};

const DEFAULT_DOCS_DIR: &str = "docs/docs/en";
const DEFAULT_OUT_DIR: &str = "dist/docs";
const FLAG_OUT: &str = "--out";
const FLAG_TITLE: &str = "--title";
const FLAG_BASE: &str = "--base";

/// Entry point for the `euv-docs` static site builder and native documentation viewer.
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut src_dir: Option<PathBuf> = None;
    let mut out_dir: PathBuf = PathBuf::from(DEFAULT_OUT_DIR);
    let mut title_override: Option<String> = None;
    let mut base_url: String = String::new();
    let mut generate_mode: bool = false;
    let mut idx: usize = 1;
    while idx < args.len() {
        let arg: &str = args[idx].as_str();
        if arg == DOCS_FLAG_GENERATE {
            generate_mode = true;
            if let Some(dir_val) = args.get(idx + 1)
                && !dir_val.starts_with('-')
            {
                src_dir = Some(PathBuf::from(dir_val));
                idx += 1;
            }
            if let Some(out_val) = args.get(idx + 1)
                && !out_val.starts_with('-')
            {
                out_dir = PathBuf::from(out_val);
                idx += 1;
            }
        } else if arg == FLAG_OUT {
            generate_mode = true;
            if let Some(val) = args.get(idx + 1) {
                out_dir = PathBuf::from(val);
                idx += 1;
            }
        } else if arg == FLAG_TITLE {
            if let Some(val) = args.get(idx + 1) {
                title_override = Some(val.clone());
                idx += 1;
            }
        } else if arg == FLAG_BASE {
            if let Some(val) = args.get(idx + 1) {
                base_url = val.clone();
                idx += 1;
            }
        } else if !arg.starts_with('-') && src_dir.is_none() {
            src_dir = Some(PathBuf::from(arg));
            generate_mode = true;
        }
        idx += 1;
    }
    let resolved_src: PathBuf = src_dir.unwrap_or_else(|| PathBuf::from(DEFAULT_DOCS_DIR));
    if generate_mode {
        let mut opts: DocsBuildOptions = DocsBuildOptions::default();
        opts.set_title(title_override)
            .set_base_url(base_url)
            .set_clean_urls(true);
        match build_docs_bundle(&resolved_src, &out_dir, &opts) {
            Ok(report) => print_report(&report),
            Err(err) => eprintln!("{err}"),
        }
        return;
    }
    let site: DocSite = match load_doc_site_with_options(&resolved_src, title_override.as_deref()) {
        Ok(loaded) => loaded,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };
    let headless: bool = args.iter().any(|arg: &String| arg == DOCS_FLAG_HEADLESS);
    let interactive: bool = args.iter().any(|arg: &String| arg == DOCS_FLAG_INTERACTIVE);
    match render_docs_app(&site, headless, interactive) {
        Ok(stats) => {
            println!(
                "pages={} draw_commands={} frame_hash={:#018x}",
                site.get_pages().len(),
                stats.get_draw_commands(),
                stats.get_frame_hash()
            );
        }
        Err(err) => eprintln!("{err}"),
    }
}

/// Prints a concise build summary for a generated documentation site.
///
/// # Arguments
///
/// - `&GeneratedSiteReport` - Shared reference to a `GeneratedSiteReport`.
fn print_report(report: &GeneratedSiteReport) {
    println!(
        "generated_pages={} index={} timestamp={}",
        report.get_page_count(),
        report.get_index_path(),
        report.get_generated_at()
    );
}
