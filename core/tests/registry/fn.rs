use super::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn repeated_registration_keeps_only_one_shared_cell_reference() {
    let entry: NodeRefEntry = Rc::new(UnsafeCell::new(None));
    for _ in 0..REF_REGISTRATION_COUNT {
        Registry::register_noderef(REPEATED_REF_ID, Rc::clone(&entry));
    }
    assert_eq!(Rc::strong_count(&entry), 2);
    Registry::cleanup_noderefs(REPEATED_REF_ID);
    assert_eq!(Rc::strong_count(&entry), 1);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn distinct_cells_for_one_element_remain_registered_until_cleanup() {
    let first: NodeRefEntry = Rc::new(UnsafeCell::new(None));
    let second: NodeRefEntry = Rc::new(UnsafeCell::new(None));
    Registry::register_noderef(DISTINCT_REF_ID, Rc::clone(&first));
    Registry::register_noderef(DISTINCT_REF_ID, Rc::clone(&second));
    Registry::register_noderef(DISTINCT_REF_ID, Rc::clone(&first));
    assert_eq!(Rc::strong_count(&first), 2);
    assert_eq!(Rc::strong_count(&second), 2);
    Registry::cleanup_noderefs(DISTINCT_REF_ID);
    assert_eq!(Rc::strong_count(&first), 1);
    assert_eq!(Rc::strong_count(&second), 1);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn cleanup_clears_every_distinct_registered_cell() {
    let first: NodeRefEntry = Rc::new(UnsafeCell::new(Some(JsValue::NULL)));
    let second: NodeRefEntry = Rc::new(UnsafeCell::new(Some(JsValue::TRUE)));
    Registry::register_noderef(CLEANUP_REF_ID, Rc::clone(&first));
    Registry::register_noderef(CLEANUP_REF_ID, Rc::clone(&second));
    Registry::cleanup_noderefs(CLEANUP_REF_ID);
    assert!(unsafe { (*first.get()).is_none() });
    assert!(unsafe { (*second.get()).is_none() });
}
