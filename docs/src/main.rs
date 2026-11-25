use std::{env, path::Path};

use euv_docs::{
    DOCS_FLAG_GENERATE, DOCS_FLAG_HEADLESS, DOCS_FLAG_INTERACTIVE, DocSite, GeneratedSiteReport,
    generate_static_docs, load_doc_site, render_docs_app,
};

const DEFAULT_DOCS_DIR: &str = "docs/docs/en";
const DEFAULT_OUT_DIR: &str = "dist/docs";

/// Body of the `main` free function.
fn main() {
    let args: Vec<String> = env::args().collect();
    let docs_dir: &str = args
        .get(2)
        .filter(|_: &&String| args.get(1).map(String::as_str) == Some(DOCS_FLAG_GENERATE))
        .map(String::as_str)
        .unwrap_or(DEFAULT_DOCS_DIR);
    let site: DocSite = match load_doc_site(Path::new(docs_dir)) {
        Ok(loaded) => loaded,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };
    if args.iter().any(|arg: &String| arg == DOCS_FLAG_GENERATE) {
        let out_dir: &str = args.get(3).map(String::as_str).unwrap_or(DEFAULT_OUT_DIR);
        match generate_static_docs(&site, Path::new(out_dir)) {
            Ok(report) => print_report(&report),
            Err(err) => eprintln!("{err}"),
        }
        return;
    }
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

/// Body of the `print_report` free function.
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
