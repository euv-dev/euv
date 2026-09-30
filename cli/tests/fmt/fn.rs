use super::*;

#[test]
fn test_simple_if() {
    let input: &str = "if{a}{b}";
    let expected: &str = "if { a } {\n    b\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_if_with_else() {
    let input: &str = "if{a}{b}else{c}";
    let expected: &str = "if { a } {\n    b\n} else {\n    c\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_if_else_if_else() {
    let input: &str = "if{a}{b}else if{c}{d}else{e}";
    let expected: &str = "if { a } {\n    b\n} else if { c } {\n    d\n} else {\n    e\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_nested_if() {
    let input: &str = "if{a}{if{b}{c}}";
    let expected: &str = "if { a } {\n    if { b } {\n        c\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_match_single_arm() {
    let input: &str = "match{a}{\"b\"=>{c}}";
    let expected: &str = "match { a } {\n    \"b\" => {\n        c\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_match_multiple_arms() {
    let input: &str = "match{a}{\"b\"=>{c}\"d\"=>{e}}";
    let expected: &str =
        "match { a } {\n    \"b\" => {\n        c\n    }\n    \"d\" => {\n        e\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_for_loop_bare_expr() {
    let input: &str = "for i in a.iter(){b}";
    let expected: &str = "for i in a.iter() {\n    b\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_for_loop_bare_expr_chain() {
    let input: &str = "for (i, v) in a.iter().enumerate(){b}";
    let expected: &str = "for (i, v) in a.iter().enumerate() {\n    b\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_for_loop_braced_expr_unchanged() {
    let input: &str = "for i in {a}{b}";
    let expected: &str = "for i in { a } {\n    b\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_simple_div() {
    let input: &str = "div{a}";
    let expected: &str = "div {\n    a\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_nested_div() {
    let input: &str = "div{a{b}c}";
    let expected: &str = "div {\n    a {\n        b\n    }\n    c\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_empty_block() {
    let input: &str = "div{}";
    let expected: &str = "div {}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_bare_empty_block() {
    let input: &str = "{}";
    let expected: &str = "{}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_string_in_body() {
    let input: &str = "div{\"hello\"}";
    let expected: &str = "div {\n    \"hello\"\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_braces_in_string() {
    let input: &str = "div{\"hello { world }\"}";
    let expected: &str = "div {\n    \"hello { world }\"\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_comment_in_body() {
    let input: &str = "div{// comment\n    a\n}";
    let expected: &str = "div {\n    // comment\n    a\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_class_macro() {
    let input: &str = "class!{c_app_root{display:\"flex\"}}";
    let expected: &str = "class!{\n    c_app_root {\n        display: \"flex\"\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_vars_macro() {
    let input: &str = "vars!{a:\"1\";b:\"2\"}";
    let expected: &str = "vars!{\n    a: \"1\";b: \"2\"\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_deeply_nested() {
    let input: &str = "div{a{div{b{div{c}}}}}";
    let expected: &str = concat!(
        "div {\n",
        "    a {\n",
        "        div {\n",
        "            b {\n",
        "                div {\n",
        "                    c\n",
        "                }\n",
        "            }\n",
        "        }\n",
        "    }\n",
        "}"
    );
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_multiple_siblings() {
    let input: &str = "div{a\n    b\n    c}";
    let expected: &str = "div {\n    a\n    b\n    c\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_if_with_complex_condition() {
    let input: &str = "if{a>0}{b}";
    let expected: &str = "if { a>0 } {\n    b\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_watch_macro() {
    let input: &str = "watch!{signal=>{callback}}";
    let expected: &str = "watch!{\n    signal => {\n        callback\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_pseudo_selector() {
    let input: &str = "div{:hover{color:\"red\"}}";
    let expected: &str = "div {\n    :hover {\n        color: \"red\"\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_media_query() {
    let input: &str = "div{@media{(max-width: 767px){display:\"none\"}}}";
    let expected: &str = "div {\n    @media {\n        (max-width: 767px){\n            display: \"none\"\n        }\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_block_comment() {
    let input: &str = "div{a}";
    let expected: &str = "div {\n    a\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_escape_in_string() {
    let input: &str = "div{\"hello \\\"world\\\"\"}";
    let expected: &str = "div {\n    \"hello \\\"world\\\"\"\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_attribute_with_if() {
    let input: &str = "class:if{a}{b}else{c}";
    let expected: &str = "class: if { a } {\n    b\n} else {\n    c\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_nested_match_in_if() {
    let input: &str = "if{a}{match{b}{\"c\"=>{d}}}";
    let expected: &str =
        "if { a } {\n    match { b } {\n        \"c\" => {\n            d\n        }\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_empty_line_preservation() {
    let input: &str = "div{a\n\n    b}";
    let expected: &str = "div {\n    a\n\n    b\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_format_euv_macros_html() {
    let input: &str = "html! {if{a}{b}}";
    let expected: &str = "html! {\n    if { a } {\n        b\n    }\n}";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_format_euv_macros_class() {
    let input: &str = "class! {c{display:\"flex\"}}";
    let expected: &str = "class! {\n    c {\n        display: \"flex\"\n    }\n}";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_format_euv_macros_vars() {
    let input: &str = "vars! {a:\"1\"}";
    let expected: &str = "vars! {\n    a: \"1\"\n}";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_format_euv_macros_watch() {
    let input: &str = "watch! {s => {f}}";
    let expected: &str = "watch! {\n    s => {\n        f\n    }\n}";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_format_euv_macros_no_leading_blank_lines() {
    let input: &str = "    html! {\n            div {\n                class: c_euv_input_wrapper()\n                label {\n                    for: id\n                    class: c_form_label()\n                    label_string\n                }\n                input {\n                    id: id\n                    name: id\n                    type: \"text\"\n                    placeholder: placeholder\n                    value: value\n                    autocomplete: autocomplete\n                    class: c_euv_input()\n                    onfocus: on_focus_scroll_into_view()\n                }\n            }\n        }";
    let expected: &str = "    html! {\n            div {\n                class: c_euv_input_wrapper()\n                label {\n                    for: id\n                    class: c_form_label()\n                    label_string\n                }\n                input {\n                    id: id\n                    name: id\n                    type: \"text\"\n                    placeholder: placeholder\n                    value: value\n                    autocomplete: autocomplete\n                    class: c_euv_input()\n                    onfocus: on_focus_scroll_into_view()\n                }\n            }\n    }";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_sibling_elements() {
    let input: &str = "html! {\n    euv_vconsole_fab {\n        panel_open: panel_open\n        console_signal: console_signal\n    }\n    euv_vconsole_drawer {\n        console_signal: console_signal\n        panel_open: panel_open\n    }\n}";
    let expected: &str = "html! {\n    euv_vconsole_fab {\n        panel_open: panel_open\n        console_signal: console_signal\n    }\n    euv_vconsole_drawer {\n        console_signal: console_signal\n        panel_open: panel_open\n    }\n}";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_sibling_elements_with_indent() {
    let input: &str = "    html! {\n            euv_vconsole_fab {\n                panel_open: panel_open\n                console_signal: console_signal\n            }\n            euv_vconsole_drawer {\n                console_signal: console_signal\n                panel_open: panel_open\n            }\n        }";
    let expected: &str = "    html! {\n            euv_vconsole_fab {\n                panel_open: panel_open\n                console_signal: console_signal\n            }\n            euv_vconsole_drawer {\n                console_signal: console_signal\n                panel_open: panel_open\n            }\n    }";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_sibling_elements_compressed() {
    let input: &str = "html!{euv_vconsole_fab{panel_open:panel_open console_signal:console_signal}euv_vconsole_drawer{console_signal:console_signal panel_open:panel_open}}";
    let expected: &str = "html! {\n    euv_vconsole_fab {\n        panel_open: panel_open\n        console_signal: console_signal\n    }\n    euv_vconsole_drawer {\n        console_signal: console_signal\n        panel_open: panel_open\n    }\n}";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_block_comment_idempotent_in_class_macro() {
    let inner_indent: &str = "    ";
    let body: String = format!(
        "class! {{\n{inner_indent}pub c_test {{\n{inner_indent}{inner_indent}/* `width: 100%` instead of `min-width: 140px` so the menu\n{ws}matches the dropdown container (and therefore the\n{ws}trigger button) width exactly. With only `min-width`,\n{ws}a trigger wider than 140px (e.g. the 206px-wide\n{ws}nav-column locale switcher button) leaves a large\n{ws}empty gap on the left of the menu, since `right: 0`\n{ws}anchors only the right edge to the container. With\n{ws}`width: 100%`, the menu's left edge lines up with the\n{ws}trigger's left edge. The default minimum content width\n{ws}is still guaranteed by the inner items' own padding,\n{ws}so we keep the rule without an explicit min. */\n{inner_indent}{inner_indent}color: \"red\";\n{inner_indent}}}\n}}",
        ws = " ".repeat(135),
    );
    let once: String = format_euv_macros(&body);
    let twice: String = format_euv_macros(&once);
    assert_eq!(
        once, twice,
        "format_euv_macros must be idempotent on block comments with internal whitespace"
    );
}

#[test]
fn test_block_comment_idempotent_in_macro_body() {
    let input: &str = "class! {c_test {/* line one\n                                                                                                                                                           line two\n                                                                                                                                                           line three */\n        color: \"red\";}}";
    let once: String = format_macro_body(input);
    let twice: String = format_macro_body(&once);
    assert_eq!(
        once, twice,
        "format_macro_body must be idempotent on block comments with internal whitespace"
    );
}

#[test]
fn test_block_comment_continuation_lines_normalized() {
    let inner_indent: &str = "    ";
    let body: String = format!(
        "class! {{\n{inner_indent}c_test {{\n{inner_indent}{inner_indent}/* short\n{ws}continuation\n{ws}lines */\n{inner_indent}{inner_indent}color: \"red\";\n{inner_indent}}}\n}}",
        ws = " ".repeat(135),
    );
    let once: String = format_euv_macros(&body);
    let twice: String = format_euv_macros(&once);
    let thrice: String = format_euv_macros(&twice);
    assert_eq!(
        once, twice,
        "format_euv_macros must be idempotent (run 1 vs 2)"
    );
    assert_eq!(
        twice, thrice,
        "format_euv_macros must be idempotent (run 2 vs 3)"
    );
}
