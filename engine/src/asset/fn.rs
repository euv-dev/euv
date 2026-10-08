use super::*;

/// Marks the asset load callback occupying `slot` as having run.
///
/// Called from inside an `onload` / `onerror` closure. The closure cannot
/// drop itself — a `Closure` must outlive the JavaScript call
/// that is currently executing its trampoline — so it only records that it
/// has run; [`AssetLoader::collect`] performs the actual drop later from a
/// context that is not inside a callback.
///
/// The `Weak` upgrade fails once the owning [`AssetLoader`] has been
/// dropped, in which case there is nothing left to retire and the call is a
/// no-op.
///
/// # Arguments
///
/// - `&Weak<EngineCell<AssetClosureStore>>` - Weak back-reference to the
///   loader's closure store.
/// - `usize` - The index of the calling closure's slot.
pub fn mark_asset_closure_settled(store: &Weak<EngineCell<AssetClosureStore>>, slot: usize) {
    let Some(strong) = store.upgrade() else {
        return;
    };
    let store_ref: &mut AssetClosureStore = strong.get_mut();
    if let Some(flag) = store_ref.settled.get_mut(slot) {
        *flag = true;
    }
}
