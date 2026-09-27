mod r#fn;

pub use std::{rc::Rc, rc::Weak};

/// Marks a callback slot as settled, as an `onload` / `onerror` handler
/// would once its load finished.
///
/// # Arguments
///
/// - `&AssetLoader` - The loader whose closure store holds the slot.
/// - `usize` - The index of the slot to mark settled.
pub fn settle(loader: &AssetLoader, slot: usize) {
    let store: &mut AssetClosureStore = loader.get_closures().get_mut();
    if let Some(flag) = store.settled.get_mut(slot) {
        *flag = true;
    }
}

use super::*;
