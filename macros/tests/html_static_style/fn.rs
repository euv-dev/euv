use super::*;

#[test]
fn literal_style_compiles_as_static_text_attribute_value() {
    let _node: ::euv::VirtualNode = html! {
    "div" {
        style: {
            color: "red";
        }
        "x"
    }
    };
}

const STATIC_CLASS: &str = "static-class";

fn static_class_fn() -> &'static str {
    STATIC_CLASS
}

#[test]
fn static_class_constant_needs_no_braces_before_a_text_node_child() {
    let _node: ::euv::VirtualNode = html! {
    div {
        class: STATIC_CLASS
        {
            "text"
        }
    }
    };
}

#[test]
fn static_class_constant_still_accepts_explicit_braces() {
    let _node: ::euv::VirtualNode = html! {
    div {
        class: { STATIC_CLASS }
        {
            "text"
        }
    }
    };
}

#[test]
fn struct_literal_attribute_value_is_still_parsed_as_one_expression() {
    let _node: ::euv::VirtualNode = html! {
    div {
        style: {
            color: "red";
        }
        "x"
    }
    };
}

#[test]
fn call_expression_attribute_value_is_unchanged() {
    let _node: ::euv::VirtualNode = html! {
    div {
        class: static_class_fn()
        {
            "text"
        }
    }
    };
}

#[test]
fn non_class_static_constant_before_text_child_parses() {
    let _node: ::euv::VirtualNode = html! {
    div {
        title: STATIC_CLASS
        {
            "text"
        }
    }
    };
}
