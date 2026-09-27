use super::*;

/// An entry in the asset cache containing the loaded data and its state.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct AssetEntry {
    /// The type of this asset.
    #[get(type(copy))]
    pub(crate) asset_type: AssetType,
    /// The current loading state.
    #[get(type(copy))]
    pub(crate) state: AssetState,
    /// The loaded image element, if this is an image asset.
    #[get(type(clone))]
    pub(crate) image: Option<HtmlImageElement>,
    /// The URL this asset was loaded from.
    pub(crate) url: String,
}

/// A cache for storing loaded game assets, keyed by URL.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct AssetCache {
    /// All cached assets keyed by their source URL.
    #[get(pub)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) entries: HashMap<String, AssetEntry>,
}

/// An asynchronous asset loader that fetches resources over HTTP
/// and populates a shared `AssetCache`.
#[derive(Clone, Data, New)]
pub struct AssetLoader {
    /// The shared cache that loaded assets are stored into.
    #[new(skip)]
    pub(crate) cache: Rc<EngineCell<AssetCache>>,
    /// The number of assets currently being loaded.
    #[get(pub, type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) pending_count: u32,
    /// Stored closures keeping `onload`/`onerror` callbacks alive, preventing memory leaks.
    #[new(skip)]
    pub(crate) closures: AssetClosures,
}
