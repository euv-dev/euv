use super::*;

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
