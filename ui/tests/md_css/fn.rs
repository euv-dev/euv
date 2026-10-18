use super::*;

fn rule_pairs(sheet: &str) -> Vec<(String, String)> {
    let mut heads: Vec<&str> = Vec::new();
    let mut bodies: Vec<&str> = Vec::new();
    let mut depth: usize = 0;
    let mut start: usize = 0;
    for (index, character) in sheet.char_indices() {
        match character {
            '{' => {
                if depth == 0 {
                    heads.push(sheet[start..index].trim());
                }
                start = index + 1;
                depth += 1;
            }
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    bodies.push(sheet[start..index].trim());
                    start = index + 1;
                }
            }
            _ => {}
        }
    }
    heads
        .into_iter()
        .zip(bodies)
        .map(|(head, body): (&str, &str)| (head.to_string(), body.to_string()))
        .collect()
}

fn head_covers(head: &str, selector: &str) -> bool {
    head.split(',').any(|part: &str| part.trim() == selector)
}

fn rule_declarations(sheet: &str, selector: &str) -> Option<String> {
    rule_pairs(sheet)
        .into_iter()
        .find(|(head, _): &(String, String)| head_covers(head, selector))
        .map(|(_, body): (String, String)| body)
}

fn every_rule_declarations(sheet: &str, selector: &str) -> Vec<String> {
    rule_pairs(sheet)
        .into_iter()
        .filter(|(head, _): &(String, String)| head_covers(head, selector))
        .map(|(_, body): (String, String)| body)
        .collect()
}

fn declaration(body: &str, property: &str) -> Option<String> {
    for part in body.split(';') {
        let Some((name, value)) = part.split_once(':') else {
            continue;
        };
        if name.trim() == property {
            return Some(value.trim().to_string());
        }
    }
    None
}

#[test]
fn wrapped_inline_code_chips_separate_across_lines() {
    let sheet: &str = euv_md_css();
    let body: String = rule_declarations(sheet, ".md-body code").expect(
        "the markdown body must style inline code, so the rule has to exist for the wrap defect to have a home",
    );
    assert_eq!(
        declaration(&body, "margin"),
        Some(".15em 0".to_string()),
        "a wrapped paragraph puts two chips on consecutive lines, so the vertical margin is what opens the gap between their borders; .15em top and bottom keeps that gap symmetric so the chip stays upright in its line box, and the 0 horizontal margin is what leaves the line breakable between chips"
    );
}

#[test]
fn inline_code_chips_keep_the_padding_that_separates_them_from_link_underlines() {
    let sheet: &str = euv_md_css();
    let body: String = rule_declarations(sheet, ".md-body code").expect(
        "the markdown body must style inline code, so the rule has to exist for the padding contract to have a home",
    );
    assert_eq!(
        declaration(&body, "padding"),
        Some(".45em .4em".to_string()),
        "the dashed link underline and the chip border resolve to the same token, so the padding is the only thing separating them"
    );
}

#[test]
fn single_box_code_chips_do_not_inherit_the_wrapping_margin() {
    let sheet: &str = euv_md_css();
    for selector in [".md-body td code", ".md-body th code", ".md-body pre code"] {
        let body: String = rule_declarations(sheet, selector).unwrap_or_else(|| {
            panic!("{selector} must keep its own rule, because the margin it zeroes is the one the wrapped inline chips rely on")
        });
        assert_eq!(
            declaration(&body, "margin"),
            Some("0".to_string()),
            "{selector} sits in one unbreakable line next to its cell neighbours \
             (a table cell is nowrap, a pre block never wraps), so the vertical \
             margin that separates wrapped inline chips must not apply to it; the \
             chip's border would otherwise gain outer space it does not need. \
             Body was {body:?}"
        );
    }
}

#[test]
fn table_cell_code_chips_shrink_to_their_own_text() {
    let sheet: &str = euv_md_css();
    for selector in [".md-body td code", ".md-body th code"] {
        let bodies: Vec<String> = every_rule_declarations(sheet, selector);
        assert!(
            !bodies.is_empty(),
            "{selector} must keep its own rule: it is the only thing that \
             overrides the inline `.md-body code` chip, so dropping it hands the \
             cell back the prose chip contract"
        );
        for body in &bodies {
            let display: Option<String> = declaration(body, "display");
            assert_ne!(
                display.as_deref(),
                Some("block"),
                "a block code chip is a box on its own line, so a bordered chip \
                 inside a table cell stretches to the full cell width, forces a \
                 line break after it and grows the row to hold every chip \
                 stacked; the cell is already nowrap, so an inline chip wraps \
                 nothing and needs no block. Body was {body:?}"
            );
            assert_eq!(
                display.as_deref(),
                Some("inline-block"),
                "the cell chip has to stay an inline chip that shrink-wraps its \
                 own text and keeps its border hugging the glyphs; this is the \
                 exact declaration the report measured as the full cell width. \
                 Body was {body:?}"
            );
        }
    }
}

#[test]
fn table_cell_code_chips_keep_the_inline_chip_geometry() {
    let sheet: &str = euv_md_css();
    let base: String = rule_declarations(sheet, ".md-body code").expect(
        "the markdown body must style inline code, so the cell chip inherits a chip contract to keep",
    );
    for selector in [".md-body td code", ".md-body th code"] {
        for body in every_rule_declarations(sheet, selector) {
            for property in [
                "width",
                "padding",
                "border",
                "background",
                "font-size",
                "font-family",
                "text-decoration",
            ] {
                assert_eq!(
                    declaration(&body, property),
                    None,
                    "{selector} only overrides the display and the margin, so it \
                     must not declare {property}; a width here re-stretches the \
                     chip to the cell width it just stopped inheriting, and the \
                     chip box is the same box the prose renders. Base chip \
                     declares it as {:?}, body was {body:?}",
                    declaration(&base, property)
                );
            }
        }
    }
    assert_eq!(
        declaration(&base, "padding"),
        Some(".45em .4em".to_string()),
        "the dashed link underline and the chip border resolve to the same token, \
         so the padding is the only thing separating them, in cells as in prose"
    );
}

#[test]
fn the_markdown_accessor_returns_a_stable_static_slice() {
    let first: &str = euv_md_css();
    let second: &str = euv_md_css();
    assert!(
        !first.is_empty(),
        "the markdown component ships its own css, so an empty sheet renders unstyled"
    );
    assert_eq!(
        first, second,
        "the accessor returns a static slice, so two calls cannot hand back different sheets"
    );
    assert_eq!(
        first.as_ptr(),
        second.as_ptr(),
        "a static slice is not rebuilt per call, so the pointer has to match too"
    );
}

#[test]
fn the_markdown_sheet_has_balanced_braces_and_styles_code_blocks() {
    let sheet: &str = euv_md_css();
    let opens: usize = sheet.matches('{').count();
    let closes: usize = sheet.matches('}').count();
    assert_eq!(
        opens, closes,
        "a stylesheet with unbalanced braces is dropped by the browser in whole"
    );
    assert!(
        sheet.contains("pre"),
        "the markdown component renders code blocks, so the sheet has to style them"
    );
}

#[test]
fn the_global_injector_also_ships_the_markdown_sheet_and_stays_safe_without_a_window() {
    let sheet: &str = euv_md_css();
    assert!(!sheet.is_empty(), "the sheet must survive being injected");
    inject_app_global_css();
    assert!(
        !sheet.is_empty(),
        "reaching the end means the injector degraded instead of panicking with no document"
    );
}
