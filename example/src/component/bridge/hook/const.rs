/// The default global bridge object key on `window`.
///
/// The euv-app renames `window.__TAURI__` to `window.bridge` via the
/// link interceptor script, so the bridge runtime is accessed as
/// `window.bridge.core.invoke`.
pub(crate) const BRIDGE_DEFAULT_GLOBAL_KEY: &str = "bridge";

/// The default core module key on the bridge object.
pub(crate) const BRIDGE_DEFAULT_CORE_KEY: &str = "core";

/// The default invoke function key on the core module.
pub(crate) const BRIDGE_DEFAULT_INVOKE_KEY: &str = "invoke";

/// The argument key for specifying a bridge group name.
pub(crate) const BRIDGE_GROUP_KEY: &str = "group";

/// The invoke command name for resolving bridge group permissions.
pub(crate) const INVOKE_RESOLVE_BRIDGE_GROUP_PERMISSIONS: &str = "resolve_bridge_group_permissions";

/// The bridge group name for querying all permissions.
pub(crate) const BRIDGE_GROUP_ALL: &str = "all";

/// Error text returned when a bridge command is invoked outside a browser
/// context, i.e. where the `window` global does not exist at all.
pub(crate) const ERROR_NO_GLOBAL_WINDOW: &str = "no global window exists";

/// String tag emitted by native on a successful `update_cache`.
pub(crate) const UPDATE_RESULT_SUCCESS: &str = "success";

/// String tag emitted by native on a failed `update_cache`.
pub(crate) const UPDATE_RESULT_FAILED: &str = "failed";
