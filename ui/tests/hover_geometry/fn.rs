use super::*;

fn hover_tokens(text: &str) -> TokenStream {
    text.parse::<TokenStream>().expect("valid Rust tokens")
}

fn hover_punct(token: &TokenTree, ch: char) -> bool {
    matches!(token, TokenTree::Punct(value) if value.as_char() == ch)
}

fn hover_selector(tokens: &[TokenTree]) -> bool {
    tokens.windows(2).any(|pair: &[TokenTree]| {
        hover_punct(&pair[0], ':') && matches!(&pair[1], TokenTree::Ident(name) if name == "hover")
    }) || tokens.iter().any(|token: &TokenTree| {
        matches!(token, TokenTree::Group(group) if hover_selector(&group.stream().into_iter().collect::<Vec<TokenTree>>()))
    })
}

fn hover_inventory(stream: TokenStream, origin: &str, definitions: &mut Vec<HoverDefinition>) {
    let tokens: Vec<TokenTree> = stream.into_iter().collect();
    let mut index: usize = 0;
    while index < tokens.len() {
        if index + 2 < tokens.len()
            && matches!(&tokens[index], TokenTree::Ident(name) if name == "class")
            && hover_punct(&tokens[index + 1], '!')
            && let TokenTree::Group(group) = &tokens[index + 2]
        {
            let entries: Vec<TokenTree> = group.stream().into_iter().collect();
            let mut name: Option<String> = None;
            for entry in entries {
                match entry {
                    TokenTree::Ident(ident) if ident.to_string().starts_with("c_") => {
                        name = Some(ident.to_string());
                    }
                    TokenTree::Group(body) if body.delimiter() == Delimiter::Brace => {
                        definitions.push(HoverDefinition {
                            name: name.take().expect("class definition has a c_ name"),
                            origin: origin.to_string(),
                            body: body.stream(),
                        });
                    }
                    _ => {}
                }
            }
            assert!(name.is_none(), "unparsed class definition in {origin}");
            index += 3;
            continue;
        }
        if let TokenTree::Group(group) = &tokens[index] {
            hover_inventory(group.stream(), origin, definitions);
        }
        index += 1;
    }
}

fn hover_sources(path: &Path, definitions: &mut Vec<HoverDefinition>) {
    for entry in fs::read_dir(path).expect("UI source directory") {
        let path: PathBuf = entry.expect("source entry").path();
        if path.is_dir() {
            hover_sources(&path, definitions);
        } else if path.extension() == Some(OsStr::new("rs")) {
            let source: String = fs::read_to_string(&path).expect("UTF-8 Rust source");
            hover_inventory(
                hover_tokens(&source),
                &path.display().to_string(),
                definitions,
            );
        }
    }
}

fn hover_paint_property(property: &str) -> bool {
    matches!(
        property,
        "color"
            | "background"
            | "background-color"
            | "background-image"
            | "background-position"
            | "background-size"
            | "background-repeat"
            | "background-clip"
            | "background-origin"
            | "opacity"
            | "box-shadow"
            | "text-shadow"
            | "cursor"
            | "pointer-events"
            | "visibility"
            | "fill"
            | "stroke"
            | "stroke-opacity"
            | "fill-opacity"
            | "text-decoration"
            | "text-decoration-color"
            | "text-decoration-line"
            | "text-decoration-thickness"
            | "text-underline-offset"
    ) || property.starts_with("outline")
        || (property.starts_with("border-")
            && (property.ends_with("-color") || property.ends_with("-radius")))
}

fn hover_statement(
    tokens: &[TokenTree],
    active: bool,
    definitions: &[HoverDefinition],
    trail: &mut Vec<String>,
    violations: &mut Vec<String>,
) {
    if tokens.is_empty() || !active {
        return;
    }
    if let Some(colon) = tokens
        .iter()
        .position(|token: &TokenTree| hover_punct(token, ':'))
    {
        let property: String = tokens[..colon].iter().map(TokenTree::to_string).collect();
        if !hover_paint_property(&property) {
            violations.push(format!(
                "{}: forbidden hover property {property}",
                trail.join(" -> ")
            ));
        }
        return;
    }
    if let [TokenTree::Ident(name), TokenTree::Group(args)] = tokens
        && args.delimiter() == Delimiter::Parenthesis
    {
        let name: String = name.to_string();
        assert!(
            !trail.contains(&name),
            "hover inheritance cycle: {trail:?} -> {name}"
        );
        let parents: Vec<&HoverDefinition> = definitions
            .iter()
            .filter(|definition: &&HoverDefinition| definition.name == name)
            .collect();
        assert!(!parents.is_empty(), "unknown hover inheritance {name}");
        trail.push(name);
        for parent in parents {
            hover_body(parent.body.clone(), true, definitions, trail, violations);
        }
        trail.pop();
        return;
    }
    violations.push(format!(
        "{}: unparsed hover statement {}",
        trail.join(" -> "),
        tokens
            .iter()
            .map(TokenTree::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    ));
}

fn hover_body(
    stream: TokenStream,
    active: bool,
    definitions: &[HoverDefinition],
    trail: &mut Vec<String>,
    violations: &mut Vec<String>,
) {
    let mut pending: Vec<TokenTree> = Vec::new();
    for token in stream {
        if hover_punct(&token, ';') {
            hover_statement(&pending, active, definitions, trail, violations);
            pending.clear();
        } else if let TokenTree::Group(group) = &token
            && group.delimiter() == Delimiter::Brace
        {
            let nested: bool = active
                || (!pending
                    .first()
                    .is_some_and(|token: &TokenTree| hover_punct(token, '@'))
                    && hover_selector(&pending));
            hover_body(group.stream(), nested, definitions, trail, violations);
            pending.clear();
        } else {
            pending.push(token);
        }
    }
    hover_statement(&pending, active, definitions, trail, violations);
}

fn hover_violations(definitions: &[HoverDefinition]) -> Vec<String> {
    let mut violations: Vec<String> = Vec::new();
    for definition in definitions {
        let mut trail: Vec<String> = vec![format!("{} ({})", definition.name, definition.origin)];
        hover_body(
            definition.body.clone(),
            false,
            definitions,
            &mut trail,
            &mut violations,
        );
    }
    violations
}

fn hover_debug_fields(text: &str) -> Vec<TokenTree> {
    let tokens: Vec<TokenTree> = hover_tokens(text).into_iter().collect();
    match tokens.as_slice() {
        [TokenTree::Ident(_), TokenTree::Group(group)] if group.delimiter() == Delimiter::Brace => {
            group.stream().into_iter().collect()
        }
        _ => panic!("unexpected CSS metadata Debug shape: {text}"),
    }
}

fn hover_debug_string(fields: &[TokenTree], key: &str) -> String {
    for field in fields.windows(3) {
        if matches!(&field[0], TokenTree::Ident(name) if name == key)
            && hover_punct(&field[1], ':')
            && let TokenTree::Literal(literal) = &field[2]
        {
            return syn::parse_str::<LitStr>(&literal.to_string())
                .expect("CSS metadata string")
                .value();
        }
    }
    panic!("missing CSS metadata field {key}");
}

fn hover_css_declarations(style: &str, violations: &mut Vec<String>) {
    let mut start: usize = 0;
    let mut quote: Option<char> = None;
    let mut escaped: bool = false;
    let mut depth: usize = 0;
    for (index, ch) in style.char_indices() {
        if escaped {
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if let Some(delimiter) = quote {
            if ch == delimiter {
                quote = None;
            }
        } else if ch == '\'' || ch == '"' {
            quote = Some(ch);
        } else if ch == '(' {
            depth += 1;
        } else if ch == ')' {
            depth = depth.checked_sub(1).expect("balanced CSS parentheses");
        } else if ch == ';' && depth == 0 {
            hover_css_property(&style[start..index], violations);
            start = index + ch.len_utf8();
        }
    }
    assert!(
        quote.is_none() && depth == 0 && !escaped,
        "malformed emitted CSS {style}"
    );
    hover_css_property(&style[start..], violations);
}

fn hover_css_property(declaration: &str, violations: &mut Vec<String>) {
    if declaration.trim().is_empty() {
        return;
    }
    let (property, _): (&str, &str) = declaration
        .split_once(':')
        .expect("CSS declaration has colon");
    if !hover_paint_property(property.trim()) {
        violations.push(property.trim().to_string());
    }
}

fn hover_emitted_pseudo(rule: &PseudoRule, violations: &mut Vec<String>) -> usize {
    let fields: Vec<TokenTree> = hover_debug_fields(&format!("{rule:?}"));
    let selector: String = hover_debug_string(&fields, "selector");
    let style: String = hover_debug_string(&fields, "style");
    assert_eq!(rule, &PseudoRule::new(selector.clone(), style.clone()));
    if hover_selector(
        &hover_tokens(&selector)
            .into_iter()
            .collect::<Vec<TokenTree>>(),
    ) {
        hover_css_declarations(&style, violations);
        return 1;
    }
    0
}

fn hover_emitted_media(rule: &MediaRule, violations: &mut Vec<String>) -> usize {
    let fields: Vec<TokenTree> = hover_debug_fields(&format!("{rule:?}"));
    let query: String = hover_debug_string(&fields, "query");
    let style: String = hover_debug_string(&fields, "style");
    let mut pseudos: Vec<PseudoRule> = Vec::new();
    for field in fields.windows(3) {
        if matches!(&field[0], TokenTree::Ident(name) if name == "pseudo_rules")
            && let TokenTree::Group(list) = &field[2]
        {
            for token in list.stream() {
                if let TokenTree::Group(group) = token {
                    let nested: Vec<TokenTree> = group.stream().into_iter().collect();
                    pseudos.push(PseudoRule::new(
                        hover_debug_string(&nested, "selector"),
                        hover_debug_string(&nested, "style"),
                    ));
                }
            }
        }
    }
    assert_eq!(rule, &MediaRule::new(query, style, pseudos.clone()));
    pseudos
        .iter()
        .map(|pseudo: &PseudoRule| hover_emitted_pseudo(pseudo, violations))
        .sum()
}

#[test]
fn key_factories_emit_only_paint_hover_css_including_inherited_and_media_rules() {
    let factories: [&Css; 20] = [
        c_home_btn_primary(),
        c_home_btn_secondary(),
        c_euv_button(),
        c_euv_button_default(),
        c_euv_button_outline(),
        c_euv_button_ghost(),
        c_euv_dropdown_item(),
        c_euv_dropdown_item_active(),
        c_euv_dropdown_menu_item(),
        c_euv_menubar_item(),
        c_euv_context_menu_item(),
        c_euv_toc_link(),
        c_euv_toc_link_active(),
        c_euv_toc_link_nested_active(),
        c_euv_sidebar_menu_button(),
        c_euv_sidebar_menu_button_active(),
        c_euv_select_option(),
        c_euv_drawer_trigger(),
        c_euv_accordion_trigger(),
        c_euv_sheet_close(),
    ];
    let mut count: usize = 0;
    for factory in factories {
        let mut violations: Vec<String> = Vec::new();
        for rule in factory.get_pseudo_rules() {
            count += hover_emitted_pseudo(rule, &mut violations);
        }
        for rule in factory.get_media_rules() {
            count += hover_emitted_media(rule, &mut violations);
        }
        assert!(
            violations.is_empty(),
            "{} emitted hover geometry: {violations:?}",
            factory.get_name()
        );
    }
    {
        assert!(
            count >= 10,
            "emitted hover coverage unexpectedly empty: {count}"
        );
    }
}

#[test]
fn card_headers_do_not_reserve_an_empty_description_row() {
    let header: String = c_euv_card_header().get_style().split_whitespace().collect();
    assert!(header.contains("grid-template-rows:auto;"), "{header}");
    assert!(!header.contains("grid-template-rows:autoauto;"), "{header}");
    let action: String = c_euv_card_action().get_style().split_whitespace().collect();
    assert!(action.contains("grid-row-end:auto;"), "{action}");
    let rules: String = format!("{:?}", c_euv_card_action().get_pseudo_rules());
    assert!(rules.contains(":has(.c_euv_card_description)"), "{rules}");
    assert!(rules.contains("span 2"), "{rules}");
}

#[test]
fn card_shell_separates_surface_padding_from_content_spacing() {
    let style: String = c_card().get_style().split_whitespace().collect();
    let padding: Option<&str> = style
        .split(';')
        .filter_map(|declaration: &str| declaration.split_once(':'))
        .filter(|(property, _): &(&str, &str)| *property == "padding")
        .map(|(_, value): (&str, &str)| value)
        .next_back();
    let gap: Option<&str> = style
        .split(';')
        .filter_map(|declaration: &str| declaration.split_once(':'))
        .filter(|(property, _): &(&str, &str)| *property == "gap")
        .map(|(_, value): (&str, &str)| value)
        .next_back();
    assert_eq!(padding, Some("var(--card-padding)"), "{style}");
    assert_eq!(gap, Some("var(--card-content-gap)"), "{style}");
    assert!(style.contains("display:flex"));
    let rules: String = format!("{:?}", c_card().get_pseudo_rules());
    let compact_rules: String = rules.split_whitespace().collect();
    for selector in [
        ">:is([class],*)",
        ">slot>:is([class],*)",
        ">slot>slot>:is([class],*)",
    ] {
        assert!(compact_rules.contains(selector), "{rules}");
    }
    assert!(
        !style.contains("margin-block0px"),
        "child declarations leaked into the surface: {style}"
    );
    assert!(
        compact_rules.contains(">:is([class],*)"),
        "card-owned margins must outrank subsequently injected child classes: {rules}"
    );
    assert!(rules.contains("margin-block"));
    assert!(
        rules.contains("data-euv-dynamic-id"),
        "reactive boundaries are not normalized: {rules}"
    );
    assert!(
        !rules.contains("selector: \"*"),
        "unscoped universal selector: {rules}"
    );
    let title: String = c_card_title().get_style().split_whitespace().collect();
    assert!(
        title.contains("margin:0px;"),
        "card title adds an inner gap: {title}"
    );
}

#[test]
fn card_child_margin_priority_covers_mixed_slot_and_dynamic_boundaries() {
    let rules: String = format!("{:?}", c_card().get_pseudo_rules());
    let compact: String = rules.split_whitespace().collect();
    assert!(compact.contains(">:is([class],*)"), "{rules}");
    let boundary: &str = ">:is(slot,[data-euv-dynamic-id])";
    for depth in 1..=4 {
        let selector: String = format!("{}>:is([class],*)", boundary.repeat(depth));
        assert!(
            compact.contains(&selector),
            "missing mixed boundary depth {depth}: {rules}"
        );
    }
    assert!(compact.contains("margin-block:0px"), "{rules}");
}

#[test]
fn feature_grid_preserves_intrinsic_card_height_and_owns_both_axis_gaps() {
    let style: String = c_home_feature_grid()
        .get_style()
        .split_whitespace()
        .collect();

    assert!(style.contains("align-items:start;"), "{style}");
    assert!(style.contains("gap:var(--page-block-gap);"), "{style}");
    assert!(!style.contains("row-gap:0px;"), "{style}");
}

#[test]
fn console_fab_radius_uses_the_current_scheme_token() {
    {
        assert!(
            c_euv_logo_fab()
                .get_style()
                .split_whitespace()
                .collect::<String>()
                .contains("border-radius:var(--radius-md)")
        );
    }
}

#[test]
fn emitted_css_parser_controls_preserve_functions_and_quoted_semicolons() {
    let mut violations: Vec<String> = Vec::new();
    hover_css_declarations(
        "color:rgb(1, 2, 3);background-image:url(\"data:image/svg+xml;a:b\");box-shadow:0 0 3px black",
        &mut violations,
    );
    assert!(violations.is_empty());
    hover_css_declarations(
        "transform:translateX(2px);border:1px solid red;font-size:12px",
        &mut violations,
    );
    assert_eq!(violations, ["transform", "border", "font-size"]);
    let nested: PseudoRule = PseudoRule::new(
        ":is(:hover, :focus) > span".to_string(),
        "padding:1px".to_string(),
    );
    assert_eq!(hover_emitted_pseudo(&nested, &mut violations), 1);
    assert_eq!(violations.last().map(String::as_str), Some("padding"));
    let media: MediaRule = MediaRule::new(
        "(max-width: 767px)".to_string(),
        "transform:translateX(5px)".to_string(),
        vec![nested],
    );
    assert_eq!(hover_emitted_media(&media, &mut violations), 1);
    assert_eq!(
        violations
            .iter()
            .filter(|property: &&String| property.as_str() == "transform")
            .count(),
        1
    );
}

#[test]
fn all_source_class_definitions_including_inactive_cfg_keep_hover_paint_only() {
    let mut definitions: Vec<HoverDefinition> = Vec::new();
    hover_sources(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut definitions,
    );
    hover_sources(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../example/src/style"),
        &mut definitions,
    );
    let hover_classes: usize = definitions
        .iter()
        .filter(|definition: &&HoverDefinition| {
            hover_selector(
                &definition
                    .body
                    .clone()
                    .into_iter()
                    .collect::<Vec<TokenTree>>(),
            )
        })
        .count();
    assert!(
        definitions.len() >= 500,
        "unexpectedly small class inventory: {}",
        definitions.len()
    );
    assert!(
        hover_classes >= 100,
        "unexpectedly small hover inventory: {hover_classes}"
    );
    let violations: Vec<String> = hover_violations(&definitions);
    println!(
        "hover inventory: {} definitions, {hover_classes} hover definitions, {} violations",
        definitions.len(),
        violations.len()
    );
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn source_parser_handles_literals_comments_nested_selectors_media_and_inheritance() {
    let source: String = [
        concat!("/", "/ class! { pub c_comment { :hover { height: \"bad\"; } } }"),
        r##"
        const FAKE: &str = r#"class! {
 pub c_fake {
     :hover {
         transform: "bad";
     }
 }
        }"#;
        #[cfg(any())] class! {
            pub c_base { color: "red"; box-shadow: "0 0 2px black"; }
            pub c_safe { transform: "translateX(5px)"; @media (hover: hover) { transform: "translateY(2px)"; } :is(:hover, :focus) { c_base(); border-radius: "2px"; } }
            pub c_media { @media (hover: hover) { :hover { > span { padding: "1px"; } } } }
            pub c_desc { > :is(button:hover) { font-weight: "bold"; } }
            pub c_bad_base { width: "10px"; }
            pub c_child { :hover { c_bad_base(); } }
        }
    "##,
    ].join("\n");
    let mut definitions: Vec<HoverDefinition> = Vec::new();
    hover_inventory(hover_tokens(&source), "fixture", &mut definitions);
    assert_eq!(definitions.len(), 6);
    let violations: Vec<String> = hover_violations(&definitions);
    assert_eq!(violations.len(), 3, "{violations:?}");
    for property in ["padding", "font-weight", "width"] {
        assert!(
            violations
                .iter()
                .any(|violation: &String| violation.contains(property))
        );
    }
}

#[test]
fn hover_policy_rejects_geometry_even_when_repeated_or_hidden_in_shorthands() {
    for property in [
        "transform",
        "translate",
        "scale",
        "rotate",
        "margin",
        "margin-inline-start",
        "padding",
        "width",
        "min-height",
        "position",
        "top",
        "inset",
        "font",
        "font-size",
        "font-weight",
        "line-height",
        "letter-spacing",
        "display",
        "gap",
        "flex",
        "grid-template-columns",
        "border",
        "border-left",
        "border-width",
        "border-left-width",
        "border-style",
        "animation",
        "transition",
        "--hover-offset",
        "all",
        "zoom",
    ] {
        let mut definitions: Vec<HoverDefinition> = Vec::new();
        hover_inventory(
            hover_tokens(&format!(
                "class! {{ pub c_probe {{ {property}: \"same\"; :hover {{ {property}: \"same\"; }} }} }}"
            )),
            "negative",
            &mut definitions,
        );
        assert_eq!(hover_violations(&definitions).len(), 1, "{property}");
    }
    for property in [
        "color",
        "box-shadow",
        "outline",
        "outline-offset",
        "border-color",
        "border-left-color",
        "border-radius",
        "border-start-start-radius",
    ] {
        assert!(hover_paint_property(property), "{property}");
    }
}
