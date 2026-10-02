mod r#fn;

pub fn settle(loader: &AssetLoader, slot: usize) {
    let store: &mut AssetClosureStore = loader.get_closures().get_mut();
    if let Some(flag) = store.settled.get_mut(slot) {
        *flag = true;
    }
}

use super::*;
