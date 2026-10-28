use super::*;

const SCHEME_NAMES: [&str; 13] = [
    "shadcn",
    "monochrome",
    "antdesign",
    "material",
    "bootstrap",
    "cupertino",
    "fluent",
    "radix",
    "mantine",
    "daisyui",
    "heroui",
    "park",
    "geist",
];

const RESERVED_TOKENS: [&str; 10] = [
    "duration-slower",
    "ease-in",
    "gap-page-header",
    "gap-page-title",
    "gap-section-mobile",
    "nav-width-min",
    "side-indent-num",
    "success-container",
    "text-transform-none",
    "warning-container",
];

const HOST_VARIABLES: [&str; 5] = [
    "euv-mobile-safe-top",
    "euv-safe-bottom",
    "euv-safe-left",
    "euv-safe-right",
    "descriptions-columns",
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the workspace root")
        .to_path_buf()
}

fn token_names(path: &Path) -> Vec<String> {
    let text: String = fs::read_to_string(path).expect("a token file");
    let mut names: Vec<String> = Vec::new();
    for line in text.lines() {
        let trimmed: &str = line.trim();
        if !trimmed.ends_with(';') {
            continue;
        }
        let Some(separator) = trimmed.find(':') else {
            continue;
        };
        let name: &str = trimmed[..separator].trim();
        if name.is_empty() || name.contains(' ') || name.contains('(') {
            continue;
        }
        names.push(name.to_string());
    }
    names
}

fn scheme_file(scheme: &str) -> PathBuf {
    repo_root()
        .join("ui")
        .join("src")
        .join("style")
        .join("var")
        .join("scheme")
        .join(scheme)
        .join("fn.rs")
}

fn theme_file() -> PathBuf {
    repo_root()
        .join("ui")
        .join("src")
        .join("style")
        .join("var")
        .join("theme")
        .join("fn.rs")
}

fn read_tokens() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for root in ["ui/src", "example/src", "docs/src"] {
        let base: PathBuf = repo_root().join(root);
        if !base.exists() {
            continue;
        }
        collect_tokens(&base, &mut names);
    }
    names
}

fn collect_tokens(directory: &Path, names: &mut Vec<String>) {
    let entries: Vec<fs::DirEntry> = match fs::read_dir(directory) {
        Ok(entries) => entries.flatten().collect(),
        Err(_) => return,
    };
    for entry in entries {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            collect_tokens(&path, names);
            continue;
        }
        if path
            .extension()
            .is_none_or(|extension: &OsStr| extension != "rs")
        {
            continue;
        }
        let text: String = fs::read_to_string(&path).expect("a source file");
        for line in text.lines().map(strip_comments) {
            let mut rest: &str = line;
            while let Some(start) = rest.find("var!(") {
                rest = &rest[start + 5..];
                let Some(end) = rest.find(')') else {
                    break;
                };
                let argument: String = rest[..end].replace(['"', ' '], "");
                if is_token_name(&argument) {
                    names.push(argument);
                }
                rest = &rest[end + 1..];
            }
            let mut scan: &str = line;
            while let Some(start) = scan.find("var(--") {
                scan = &scan[start + 6..];
                let end: usize = scan
                    .find(|character: char| !character.is_ascii_alphanumeric() && character != '-')
                    .unwrap_or(scan.len());
                let argument: String = scan[..end].to_string();
                if is_token_name(&argument) {
                    names.push(argument);
                }
                scan = &scan[end..];
            }
        }
    }
}

fn strip_comments(line: &str) -> &str {
    let trimmed: &str = line.trim_start();
    if trimmed.starts_with("//") {
        return "";
    }
    match line.find("//") {
        Some(position) => &line[..position],
        None => line,
    }
}

fn is_token_name(name: &str) -> bool {
    if name.len() < 2 || name == "token" || name == "value" || name == "x" {
        return false;
    }
    if name.ends_with('-') {
        return false;
    }
    name.starts_with(|character: char| character.is_ascii_lowercase())
        && name
            .chars()
            .all(|character: char| character.is_ascii_alphanumeric() || character == '-')
}

#[test]
fn every_scheme_defines_the_same_tokens() {
    let reference: Vec<String> = token_names(&scheme_file("shadcn"));
    assert!(
        reference.len() > 50,
        "the shadcn scheme defines the token contract every other scheme follows"
    );
    for scheme in SCHEME_NAMES {
        let names: Vec<String> = token_names(&scheme_file(scheme));
        assert_eq!(
            names.len(),
            reference.len(),
            "{scheme} defines {} tokens, the shadcn scheme defines {} — a scheme that adds or \
             drops a token leaves a role undefined in that design language",
            names.len(),
            reference.len()
        );
        let missing: Vec<&String> = reference
            .iter()
            .filter(|name: &&String| !names.contains(name))
            .collect();
        let extra: Vec<&String> = names
            .iter()
            .filter(|name: &&String| !reference.contains(name))
            .collect();
        assert!(
            missing.is_empty() && extra.is_empty(),
            "{scheme} does not match the shadcn token contract — missing {missing:?}, extra {extra:?}"
        );
    }
}

#[test]
fn the_theme_and_a_scheme_never_own_the_same_token() {
    let theme: Vec<String> = token_names(&theme_file());
    for scheme in SCHEME_NAMES {
        let names: Vec<String> = token_names(&scheme_file(scheme));
        let overlap: Vec<&String> = theme
            .iter()
            .filter(|name: &&String| names.contains(name))
            .collect();
        assert!(
            overlap.is_empty(),
            "{scheme} and the theme class both define {overlap:?} — the two classes sit on the same \
             root element, so the browser would resolve that token from whichever stylesheet came \
             last"
        );
    }
}

#[test]
fn every_token_the_ui_reads_is_defined() {
    let mut defined: Vec<String> = token_names(&theme_file());
    defined.extend(token_names(&scheme_file("shadcn")));
    let read: Vec<String> = read_tokens();
    let mut missing: Vec<String> = Vec::new();
    for name in read {
        if defined.contains(&name) || HOST_VARIABLES.contains(&name.as_str()) {
            continue;
        }
        if !missing.contains(&name) {
            missing.push(name);
        }
    }
    missing.retain(|name: &String| !name.contains('…'));
    assert!(
        missing.is_empty(),
        "the UI reads tokens that no owner defines: {missing:?} — a token the browser cannot \
         resolve falls back to the initial value, so the style dies silently"
    );
}

#[test]
fn no_token_is_orphaned() {
    let mut defined: Vec<String> = token_names(&theme_file());
    defined.extend(token_names(&scheme_file("shadcn")));
    let read: Vec<String> = read_tokens();
    let mut orphaned: Vec<String> = Vec::new();
    for name in defined {
        if read.contains(&name) || RESERVED_TOKENS.contains(&name.as_str()) {
            continue;
        }
        if !orphaned.contains(&name) {
            orphaned.push(name);
        }
    }
    assert!(
        orphaned.is_empty(),
        "{orphaned:?} are defined but nothing reads them — a reserved token belongs in \
         RESERVED_TOKENS, where the test states that it is on purpose"
    );
}

#[test]
fn the_compiled_scheme_names_itself() {
    let expected: &str = if cfg!(feature = "theme-shadcn") {
        "shadcn"
    } else if cfg!(feature = "theme-monochrome") {
        "monochrome"
    } else if cfg!(feature = "theme-antdesign") {
        "antdesign"
    } else if cfg!(feature = "theme-material") {
        "material"
    } else if cfg!(feature = "theme-bootstrap") {
        "bootstrap"
    } else if cfg!(feature = "theme-cupertino") {
        "cupertino"
    } else if cfg!(feature = "theme-fluent") {
        "fluent"
    } else if cfg!(feature = "theme-radix") {
        "radix"
    } else if cfg!(feature = "theme-mantine") {
        "mantine"
    } else if cfg!(feature = "theme-daisyui") {
        "daisyui"
    } else if cfg!(feature = "theme-heroui") {
        "heroui"
    } else if cfg!(feature = "theme-park") {
        "park"
    } else if cfg!(feature = "theme-geist") {
        "geist"
    } else {
        "shadcn"
    };
    assert_eq!(
        scheme_name(),
        expected,
        "the compiled scheme reports {expected}, the enabled feature selects {expected}"
    );
}
