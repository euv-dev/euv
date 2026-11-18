use super::*;

class! {
    c_functional_combinator_regression {
        : is("[class]") > * {
            margin-block: "0px";
            : is("[class]") > * {
                color: "red";
            }
        }
        : is("[class]") + slot {
            color: "blue";
        }
        : is("[class]") ~ slot {
            color: "green";
        }
    }
    c_combinator_regression {
        > * {
            margin-block: "0px";
        }
        > slot > * {
            margin-block: "0px";
        }
        > slot > slot > * {
            margin-block: "0px";
        }
        @media (width > 400px) {
            > slot > * {
                margin-block: "0px";
            }
        }
    }
}

#[test]
fn functional_pseudo_combinators_emit_complete_selectors() {
    let css: &Css = c_functional_combinator_regression();
    let rules: String = format!("{:?}", css.pseudo_rules);
    assert!(rules.contains(":is([class])>*"), "{rules}");
    assert!(rules.contains(":is([class])+slot"), "{rules}");
    assert!(rules.contains(":is([class])~slot"), "{rules}");
    assert!(rules.contains("color: red;"), "{rules}");
    assert!(css.style.is_empty());
}

#[test]
fn class_combinator_macro_emits_unmangled_rules() {
    let css: &Css = c_combinator_regression();
    assert_eq!(css.style, "");
    assert_eq!(css.pseudo_rules.len(), 3);
    for (rule, selector) in css
        .pseudo_rules
        .iter()
        .zip([">*", ">slot>*", ">slot>slot>*"])
    {
        assert_eq!(
            format!("{rule:?}"),
            format!(
                "{:?}",
                PseudoRule::new(selector.to_string(), "margin-block: 0px; ".to_string())
            )
        );
    }
    let media: String = format!("{:?}", css.media_rules);
    assert!(media.contains(">slot>*"));
    assert!(media.contains("margin-block: 0px; "), "{media}");
    assert!(
        media.contains("style: \"\", pseudo_rules: [PseudoRule"),
        "{media}"
    );
}
