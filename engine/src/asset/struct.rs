use super::*;

/// An entry in the asset cache containing the loaded data and its state.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct AssetEntry {
    /// The type of this asset.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) asset_type: AssetType,
    /// The current loading state.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) state: AssetState,
    /// The loaded image element, if this is an image asset.
    #[get(type(clone))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) image: Option<HtmlImageElement>,
    /// The URL this asset was loaded from.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) url: String,
}

/// A cache for storing loaded game assets, keyed by URL.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct AssetCache {
    /// All cached assets keyed by their source URL.
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
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cache: Rc<EngineCell<AssetCache>>,
    /// The shared counter of loads that have been requested but have not
    /// settled yet. Read through [`AssetLoader::pending_count`].
    #[new(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pending: AssetPending,
    /// Stored closures keeping `onload`/`onerror` callbacks alive, preventing memory leaks.
    #[new(skip)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) closures: AssetClosures,
}

/// The backing store behind [`AssetClosures`].
///
/// Slots are appended in pairs by [`AssetLoader::load_image`] — even indices
/// hold an `onload` closure, odd indices the matching `onerror` closure — so
/// a callback can identify its own slot with one captured index.
///
/// The fields are public because the type is reachable from the public
/// [`AssetClosures`] alias: a private-field type exposed through a public
/// alias would be unusable to callers, and the retained-slot count is
/// genuinely useful as a leak diagnostic.
#[derive(Debug, Default)]
pub struct AssetClosureStore {
    /// The load callbacks, in registration order. A `None` slot is one that
    /// has already been released by [`AssetLoader::collect`].
    pub slots: Vec<Option<Closure<dyn FnMut()>>>,
    /// Whether the closure in the slot at the same index has run. A settled
    /// closure cannot free itself (see [`AssetClosures`]) but is no longer
    /// keeping anything alive that the cache still needs.
    pub settled: Vec<bool>,
}
