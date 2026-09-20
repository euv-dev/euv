use super::*;

/// Shared application state.
///
/// Holds the generated HTML, reload channel, build lock, and CLI arguments
/// for coordination between the HTTP server and file watcher.
#[derive(Data, New)]
pub(crate) struct AppState {
    /// The generated HTML with injected reload script.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) html_content: RwLock<String>,
    /// Broadcast channel for reload events.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) reload_tx: broadcast::Sender<ReloadEvent>,
    /// Whether a build is currently in progress.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_building: RwLock<bool>,
    /// CLI arguments.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) args: ModeArgs,
}

/// Configuration for HTML generation.
///
/// Groups all parameters needed by `generate_html` into a single struct
/// to reduce parameter count and improve maintainability.
#[derive(Data, New)]
pub(crate) struct HtmlConfig {
    /// The directory where `index.html` will be written.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) serving_root: PathBuf,
    /// The JS import path relative to the serving root.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) import_path: String,
    /// Whether to use the release template (no live-reload).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_release: bool,
    /// Optional path to a custom index.html template file.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) custom_index_html: Option<PathBuf>,
}

/// Request middleware that injects cache-control headers.
///
/// Sets `Cache-Control: no-cache, no-store, must-revalidate`, `Pragma: no-cache`,
/// and `Expires: 0` on every response to prevent stale WASM assets during development.
#[derive(Data, New)]
pub(crate) struct RequestMiddleware;

/// Response middleware that writes the serialized response to the stream.
///
/// Builds the HTTP response bytes and sends them through the connection stream,
/// closing the stream if the send fails.
#[derive(Data, New)]
pub(crate) struct ResponseMiddleware;

/// Route handler for the root path serving the injected development HTML.
///
/// When the request targets `index.html`, returns the in-memory HTML
/// that has the live-reload script injected. For all other files,
/// reads the content from disk with path-traversal protection.
#[derive(Data, New)]
pub(crate) struct IndexRoute;

/// Route handler for the bare root path (`/` and `/index.html`).
///
/// Serves the same in-memory HTML as `IndexRoute`'s `index.html` case
/// but is registered at the URL root so that users landing on
/// `http://host:port/` (the URL printed by most browsers by default)
/// get the app instead of a blank 200. Without this route the dev
/// server only matches `{serving_route_prefix}/{path:.*}` and a request
/// to `/` falls through to hyperlane's default empty response — the
/// "white screen" symptom users reported.
///
/// The actual application path prefix (e.g. `www`) is still registered
/// separately via `IndexRoute` so the original URL continues to work.
#[derive(Data, New)]
pub(crate) struct RootRoute;

/// Route handler for the reload endpoint using long-polling.
///
/// Holds the connection open until a reload event is broadcast, then returns
/// a single JSON response so the client can distinguish between a successful
/// rebuild and an error.
#[derive(Data, New)]
pub(crate) struct ReloadRoute;
