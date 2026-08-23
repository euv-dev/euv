//! `NodeRef` — reactive handle to a DOM element set after mount.
//!
//! This module provides the [`NodeRef<T>`] type, which stores an optional
//! reference to a DOM element. The element is set by the renderer after the
//! corresponding virtual node is mounted, and cleared when the node is
//! unmounted. Users can read the element via [`NodeRef::get`] or
//! [`NodeRef::get_cloned`] (the typed variant that `dyn_into`s the raw
//! `JsValue` into the user's `T`).
//!
//! `NodeRef` is the euv equivalent of React's `useRef` / Yew's `NodeRef` /
//! Solid's `createSignal` for DOM elements. The macro integration allows
//! `html! { div { ref: my_ref } }` to wire the rendered element back to
//! the handle without forcing the user to call `document.get_element_by_id`
//! and `dyn_into` by hand.
//!
//! # Why `Rc<UnsafeCell<Option<JsValue>>>` instead of `Rc<RefCell<...>>`
//!
//! The euv event handler registry (`core/src/event/handler/type.rs`) uses
//! the same pattern. WASM is single-threaded, so `RefCell`'s runtime borrow
//! checking only adds overhead without adding safety; `UnsafeCell` skips it.
//! We do not hand out `&mut` references to the inner cell — only
//! `Option<JsValue>` via `take` / `replace` — so there is no aliasing hazard.

mod r#impl;
mod r#struct;

pub use r#struct::*;

use super::*;
