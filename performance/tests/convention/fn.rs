use super::*;

fn collect_sources(directory: &Path, sources: &mut Vec<PathBuf>) {
    let entries: fs::ReadDir =
        fs::read_dir(directory).expect("benchmark source directory must be readable");
    for entry in entries {
        let entry: fs::DirEntry = entry.expect("source entry must be readable");
        let path: PathBuf = entry.path();
        if path.is_dir() {
            collect_sources(&path, sources);
        } else if path.extension() == Some(OsStr::new("rs")) {
            sources.push(path);
        }
    }
}

#[test]
fn benchmark_business_code_uses_accessors_instead_of_direct_field_access() {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut sources: Vec<PathBuf> = Vec::new();
    collect_sources(&root, &mut sources);
    assert!(!sources.is_empty());
    sources.sort();
    for source in sources {
        let text: String =
            fs::read_to_string(&source).expect("benchmark source must be valid text");
        let file: File = parse_file(&text).expect("benchmark source must parse as Rust");
        let mut audit: FieldAccessAudit = FieldAccessAudit::default();
        audit.visit_file(&file);
        assert!(
            audit.get_violations().is_empty(),
            "{}: {:?}",
            source.display(),
            audit.get_violations()
        );
    }
}

#[test]
fn field_access_audit_detects_fields_inside_format_macros_too() {
    let file: File = parse_file(r#"fn inspect(input: &Value) -> u64 { println!("{}", input.value); if matches!(input.value, Value::Present) {} input.value }"#).unwrap();
    let mut audit: FieldAccessAudit = FieldAccessAudit::default();
    audit.visit_file(&file);
    assert_eq!(audit.get_violations().len(), 3);
}

fn constant_violations(source: &str) -> Vec<String> {
    let File { items, .. }: File = parse_file(source).expect("constant source must parse as Rust");
    let lines: Vec<&str> = source.lines().collect();
    let mut previous_end: Option<usize> = None;
    let mut violations: Vec<String> = Vec::new();
    for item in items {
        let Item::Const(constant): Item = item else {
            previous_end = None;
            continue;
        };
        let ItemConst {
            attrs,
            ident,
            const_token,
            semi_token,
            ..
        }: ItemConst = constant;
        let documented: bool = attrs.iter().any(|attribute: &Attribute| {
            let Attribute { meta, .. }: &Attribute = attribute;
            match meta {
                Meta::NameValue(MetaNameValue {
                    path,
                    value:
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(text),
                            ..
                        }),
                    ..
                }) => path.is_ident("doc") && !text.value().trim().is_empty(),
                _ => false,
            }
        });
        if !documented {
            violations.push(format!("{ident}: missing documentation"));
        }
        let LineColumn { line: start, .. }: LineColumn = attrs
            .first()
            .map(Spanned::span)
            .unwrap_or_else(|| const_token.span())
            .start();
        let LineColumn { line: end, .. }: LineColumn = semi_token.span().end();
        if let Some(previous) = previous_end {
            let separator: Option<&[&str]> = lines.get(previous..start.saturating_sub(1));
            if !separator.is_some_and(|gap: &[&str]| gap.len() == 1 && gap[0].trim().is_empty()) {
                violations.push(format!("{ident}: expected exactly one blank separator"));
            }
        }
        previous_end = Some(end);
    }
    violations
}

#[test]
fn benchmark_constants_have_docs_and_exactly_one_blank_separator() {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut sources: Vec<PathBuf> = Vec::new();
    collect_sources(&root, &mut sources);
    assert!(!sources.is_empty());
    for source in sources {
        let text: String = fs::read_to_string(&source).unwrap();
        let violations: Vec<String> = constant_violations(&text);
        assert!(
            violations.is_empty(),
            "{}: {violations:?}",
            source.display()
        );
    }
}

#[test]
fn constant_convention_guard_rejects_missing_docs_and_wrong_separator_counts() {
    let valid: &str = "/// First constant.\nconst FIRST: u8 = 1;\n\n/// Second constant.\nconst SECOND: u8 = 2;\n";
    assert!(constant_violations(valid).is_empty());
    assert!(!constant_violations(&valid.replacen("/// First constant.\n", "", 1)).is_empty());
    assert!(!constant_violations(&valid.replace(";\n\n///", ";\n///")).is_empty());
    assert!(!constant_violations(&valid.replace(";\n\n///", ";\n\n\n///")).is_empty());
}
