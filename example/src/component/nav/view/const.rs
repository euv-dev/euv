/// Static navigation route targets — every page in the
/// demo registers here so the sidebar can iterate.
///
/// Entries are sorted alphabetically by their display label
/// so the sidebar reads top-to-bottom in `A → Z` order. Each
/// tuple is `(icon, label, target)` — the icon is a short
/// emoji, the label is the human-readable name, and the
/// target is the hash-routed path that `page_router` resolves.
pub(crate) const NAV_ITEMS: &[(&str, &str, &str)] = &[
    ("ℹ️", "About", "/"),
    ("🎬", "Animation", "/animation"),
    ("🌐", "Async", "/hooks-async"),
    ("⚙️", "Attrs", "/custom-attrs"),
    ("🏷️", "Badge", "/badge"),
    ("🔗", "Binding", "/component-binding"),
    ("🌐", "Browser", "/browser"),
    ("📷", "Camera", "/camera"),
    ("🎨", "Canvas", "/canvas"),
    ("🔀", "Condition", "/conditional"),
    ("🔢", "Counter", "/counter"),
    ("🏷️", "DynTag", "/dynamic-component"),
    ("🎯", "Event", "/event"),
    ("📄", "Form", "/form"),
    ("🎮", "Game2D", "/game-2d"),
    ("🎲", "Game3D", "/game-3d"),
    ("🌍", "i18n", "/hooks-i18n"),
    ("💚", "KeepAlive", "/keep-alive"),
    ("♻️", "Lifecycle", "/lifecycle"),
    ("💡", "Lighting", "/lighting"),
    ("📝", "List", "/list"),
    ("💬", "Modal", "/modal"),
    ("👁️", "Observer", "/observer"),
    ("🛡️", "Protect", "/hooks-protect"),
    ("🔦", "Ray Trace", "/raytrace"),
    ("📡", "SSE", "/sse"),
    ("📋", "Select", "/select"),
    ("⏲️", "Timing", "/hooks-timing"),
    ("⏱️", "Timer", "/timer"),
    ("📁", "Upload", "/file-upload"),
    ("📊", "VList", "/virtual-list"),
    ("🔌", "WebSocket", "/websocket"),
];
