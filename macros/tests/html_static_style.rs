use euv::*;

///
/// OPT-10 regression test (recovered from v0.18.59..v0.20.6 silent E0308).
///
/// Locks in the macro expansion of an all-literal `style:` block so a
/// future change that re-wraps `AttributeValue::StaticText` as
/// `AttributeValue::Text(String)` fails to compile here, not five
/// versions later in a downstream crate.
///
/// Before the fix this exact call failed with `expected String, found
/// AttributeValue` (see R2 in references/euv-perf-findings-0.20.6.md).
///
/// The test only asserts that the macro accepts the syntax and emits a
/// `VirtualNode`; a deeper shape check would require
/// `AttributeEntry::get_value`, which is `pub(crate)` and therefore not
/// reachable from this crate. The compile-only check is sufficient to
/// lock the macro output type — that is what silently broke between
/// v0.18.58 and v0.20.6.
///
#[test]
fn literal_style_compiles_as_static_text_attribute_value() {
    let _node: ::euv::VirtualNode = html! {
    div {
        style: {
            color: "red";
        }
        "x"
    }
    };
}
