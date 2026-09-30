use super::*;

#[component]
fn my_component() -> u32 {
    42
}

#[component]
pub fn pub_component() -> &'static str {
    "hello"
}

#[component]
fn generic_component<T: Default + Clone + 'static>() -> T {
    T::default()
}

#[test]
fn component_attribute_preserves_function_signature_no_args_returning_u32() {
    let result: u32 = my_component();
    assert_eq!(result, 42);
}

#[test]
fn component_attribute_preserves_pub_visibility() {
    let result: &'static str = pub_component();
    assert_eq!(result, "hello");
}

#[test]
fn component_attribute_preserves_generic_function() {
    let result: i32 = generic_component::<i32>();
    assert_eq!(result, 0);
}

#[test]
fn component_attribute_does_not_alter_function_name() {
    assert_eq!(my_component(), 42);
}
