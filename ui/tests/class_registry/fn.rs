use super::*;

const SCALE: &[f32] = &[
    0.0, 2.0, 4.0, 8.0, 9.0, 10.0, 12.0, 16.0, 20.0, 24.0, 28.0, 32.0, 40.0, 80.0,
];

fn registries() -> Vec<PathBuf> {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the workspace root")
        .to_path_buf();
    vec![
        root.join("ui").join("src").join("style").join("class"),
        root.join("example").join("src").join("style").join("class"),
    ]
}

fn class_files() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    for registry in registries() {
        collect(&registry, &mut files);
    }
    files.sort();
    files
}

fn collect(directory: &Path, files: &mut Vec<PathBuf>) {
    let entries: Vec<fs::DirEntry> = match fs::read_dir(directory) {
        Ok(entries) => entries.flatten().collect(),
        Err(_) => return,
    };
    for entry in entries {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            collect(&path, files);
            continue;
        }
        if path
            .extension()
            .is_some_and(|extension: &OsStr| extension == "rs")
        {
            files.push(path);
        }
    }
}

struct Rule {
    name: String,
    body: String,
    file: String,
}

fn rules() -> Vec<Rule> {
    let mut found: Vec<Rule> = Vec::new();
    for file in class_files() {
        let text: String = fs::read_to_string(&file).expect("a class file");
        let name: String = file
            .file_name()
            .and_then(|name: &OsStr| name.to_str())
            .unwrap_or_default()
            .to_string();
        let lines: Vec<&str> = text.lines().collect();
        let mut index: usize = 0;
        while index < lines.len() {
            let trimmed: &str = lines[index].trim();
            let start: &str = match trimmed.strip_prefix("pub ") {
                Some(rest) => rest,
                None => match trimmed.strip_prefix("pub(crate) ") {
                    Some(rest) => rest,
                    None => {
                        index += 1;
                        continue;
                    }
                },
            };
            let rule_name: String = start
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .trim_end_matches('{')
                .to_string();
            let indent: usize = lines[index].len() - lines[index].trim_start().len();
            let mut cursor: usize = index + 1;
            let mut body: Vec<&str> = Vec::new();
            while cursor < lines.len() {
                let line: &str = lines[cursor];
                let line_indent: usize = line.len() - line.trim_start().len();
                if line.trim() == "}" && line_indent == indent {
                    break;
                }
                body.push(line);
                cursor += 1;
            }
            found.push(Rule {
                name: rule_name,
                body: body.join("\n"),
                file: name.clone(),
            });
            index = cursor + 1;
        }
    }
    found
}
fn declarations(body: &str) -> Vec<(String, String)> {
    let code: String = body
        .lines()
        .map(|line: &str| match line.find("//") {
            Some(position) => &line[..position],
            None => line,
        })
        .collect::<Vec<&str>>()
        .join("\n");
    let mut out: Vec<(String, String)> = Vec::new();
    let mut rest: &str = &code;
    while let Some(position) = rest.find(':') {
        let head: &str = &rest[..position];
        let property: String = head
            .rsplit(|character: char| !character.is_alphanumeric() && character != '-')
            .next()
            .unwrap_or_default()
            .to_string();
        let tail: &str = &rest[position + 1..];
        let end: usize = tail.find(';').unwrap_or(tail.len());
        let value: String = tail[..end].trim().to_string();
        if !property.is_empty() {
            out.push((property, value));
        }
        rest = &tail[(end + 1).min(tail.len())..];
    }
    out
}
fn numbers(value: &str) -> Vec<f32> {
    let mut out: Vec<f32> = Vec::new();
    let characters: Vec<char> = value.chars().collect();
    let mut index: usize = 0;
    while index < characters.len() {
        let starts_number: bool = characters[index].is_ascii_digit()
            || (characters[index] == '-'
                && index + 1 < characters.len()
                && characters[index + 1].is_ascii_digit());
        if !starts_number {
            index += 1;
            continue;
        }
        let start: usize = index;
        if characters[index] == '-' {
            index += 1;
        }
        while index < characters.len()
            && (characters[index].is_ascii_digit() || characters[index] == '.')
        {
            index += 1;
        }
        let text: String = characters[start..index].iter().collect();
        if let Ok(number) = text.parse::<f32>() {
            out.push(number);
        }
    }
    out
}
fn is_spacing(property: &str) -> bool {
    let named: bool = property == "gap" || property == "row-gap" || property == "column-gap";
    let padded: bool = property.starts_with("padding") || property.starts_with("margin");
    named || padded && property.len() > "padding".len() - 1
}
#[test]
fn a_class_rule_never_hardcodes_a_colour() {
    let mut offenders: Vec<String> = Vec::new();
    for rule in rules() {
        for (property, value) in declarations(&rule.body) {
            if property == "color"
                || property == "background"
                || property == "background-color"
                || property == "border"
                || property == "border-color"
                || property == "border-bottom"
                || property == "border-top"
                || property == "border-left"
                || property == "border-right"
                || property == "outline-color"
                || property == "fill"
                || property == "stroke"
            {
                let lower: String = value.to_lowercase();
                let literal: bool = lower.contains('#')
                    || lower.contains("rgb(")
                    || lower.contains("hsl(")
                    || lower.contains("rgba(")
                    || lower.contains("hsla(");
                if literal {
                    offenders.push(format!(
                        "{}.{}: {} — a colour literal, use var!(…)",
                        rule.file, rule.name, value
                    ));
                }
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "a colour belongs in the token layer:\n{}",
        offenders.join("\n")
    );
}
#[test]
fn a_class_rule_never_writes_the_top_safe_area_itself() {
    let mut offenders: Vec<String> = Vec::new();
    for file in class_files() {
        let text: String = fs::read_to_string(&file).expect("a class file");
        for (number, line) in text.lines().enumerate() {
            if line.contains("env(safe-area-inset-top)") {
                offenders.push(format!(
                    "{}:{}: {}",
                    file.file_name()
                        .and_then(|name: &OsStr| name.to_str())
                        .unwrap_or_default(),
                    number + 1,
                    line.trim()
                ));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "the top safe area is a variable contract, not env():\n{}\n\
         write var(--euv-mobile-safe-top, 0px) instead — a static rule cannot \
         tell a letterboxed browser from an immersive WebView.",
        offenders.join("\n")
    );
}
#[test]
fn a_class_rule_never_uses_a_bare_hover_block() {
    let mut offenders: Vec<String> = Vec::new();
    for rule in rules() {
        for line in rule.body.lines() {
            let trimmed: &str = line.trim();
            if trimmed.starts_with("hover") && !trimmed.contains(':') {
                offenders.push(format!("{}.{}: {}", rule.file, rule.name, trimmed));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "a bare `hover {{ … }}` block emits the selector `.c_xxxhover` and the \
         hover style dies silently:\n{}",
        offenders.join("\n")
    );
}
#[test]
fn a_hardcoded_spacing_literal_is_on_the_scale_or_carries_a_reason() {
    let mut offenders: Vec<String> = Vec::new();
    for rule in rules() {
        let has_comment: bool = rule.body.contains("//");
        for (property, value) in declarations(&rule.body) {
            if !is_spacing(&property) {
                continue;
            }
            if value.contains("var!") || value.contains("calc(") {
                continue;
            }
            for number in numbers(&value) {
                let on_scale: bool = SCALE.iter().any(|step: &f32| (*step - number).abs() < 0.01);
                if on_scale {
                    continue;
                }
                if has_comment {
                    continue;
                }
                offenders.push(format!(
                    "{}.{}: {} = {} — a spacing literal that is neither on the scale \
                     nor explained by a comment",
                    rule.file, rule.name, property, value
                ));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "a spacing value is either a token or a recorded decision:\n{}",
        offenders.join("\n")
    );
}
fn value_of(declarations: &[(String, String)], property: &str) -> Option<String> {
    declarations
        .iter()
        .find(|entry: &&(String, String)| entry.0 == property)
        .map(|entry: &(String, String)| entry.1.trim_matches('"').to_string())
}
#[test]
fn a_bordered_inline_box_is_never_an_atomic_inline_flex() {
    let mut offenders: Vec<String> = Vec::new();
    for rule in rules() {
        let declarations: Vec<(String, String)> = declarations(&rule.body);
        let display: String = match value_of(&declarations, "display") {
            Some(display) => display,
            None => continue,
        };
        if !matches!(
            display.as_str(),
            "inline" | "inline-flex" | "inline-grid" | "inline-block"
        ) {
            continue;
        }
        let bordered: bool = declarations.iter().any(|entry: &(String, String)| {
            let property: &str = entry.0.as_str();
            let value: &str = entry.1.trim_matches('"');
            property.starts_with("border") && !value.is_empty() && value != "none"
        });
        if !bordered {
            continue;
        }
        let nowrap: bool = value_of(&declarations, "white-space").as_deref() == Some("nowrap");
        let atom: bool = display == "inline-block"
            && value_of(&declarations, "vertical-align").is_some()
            && value_of(&declarations, "line-height").as_deref() == Some("1")
            && value_of(&declarations, "text-align").as_deref() == Some("center")
            && value_of(&declarations, "box-sizing").as_deref() == Some("border-box");
        if nowrap || atom {
            continue;
        }
        offenders.push(format!(
            "{}.{}: display: {display} with a border and no `white-space: nowrap` — an inline \
             flex box is atomic, so a wrapped label left-aligns while the border stays at the \
             parent width and the right border reads as missing. Either use the wrapping-atom \
             recipe (inline-block, vertical-align: middle, line-height: 1, text-align: center, \
             box-sizing: border-box) or make the box nowrap",
            rule.file, rule.name
        ));
    }
    assert!(
        offenders.is_empty(),
        "the two inline recipes are never mixed:\n{}",
        offenders.join("\n")
    );
}
#[test]
fn the_ring_token_is_not_an_orphan() {
    let mut used: usize = 0;
    for file in class_files() {
        let text: String = fs::read_to_string(&file).expect("a class file");
        used += text.matches("var!(ring)").count();
    }
    let global: String = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("style")
            .join("css")
            .join("const.rs"),
    )
    .expect("the global stylesheet");
    used += global.matches("var(--ring)").count();
    assert!(
        used > 0,
        "the `ring` token holds the ink of the theme and is used for the focus ring. In a \
         monochrome system --border, --accent and --ring are the same colour, so a focus that \
         only changes the border colour changes nothing on screen: the focus was invisible on \
         every form control."
    );
}
#[test]
fn the_cards_are_separated_by_the_block_gap() {
    let card: Option<Rule> = rules()
        .into_iter()
        .find(|rule: &Rule| rule.name == "c_card");
    let card: Rule = card.expect("the card class");
    let declarations: BTreeMap<String, String> = declarations(&card.body).into_iter().collect();
    let margin: String = declarations.get("margin-top").cloned().unwrap_or_default();
    assert!(
        margin.contains("page-block-gap"),
        "c_card separates the cards with `{}` — the page template asks for the \
         `page-block-gap` token, and without it the gap is whatever the card \
         title's margin happens to be",
        margin
    );
}
