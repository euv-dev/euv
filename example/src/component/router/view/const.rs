/// Route path for the gesture recognition demo page.
///
/// Declared here rather than inline so the route, the nav entry and the page
/// registration all reference one spelling.
pub(crate) const GESTURE_ROUTE: &str = "/gesture";

/// Route path table for every page `page_router` can resolve.
///
/// `page_router` matches the current hash route against these constants with a
/// `match` over string patterns. A `match` pattern must be a literal, so each arm
/// references a `const` rather than repeating the path inline — one spelling per
/// route, shared with `NAV_ITEMS` in the nav component.
pub(crate) const ROUTE_ABOUT: &str = "/about";
/// Route path of the home page, which renders the about page.
pub(crate) const ROUTE_HOME: &str = "/";
/// Route path for the animation demo page.
pub(crate) const ROUTE_ANIMATION: &str = "/animation";
/// Route path for the custom-attributes demo page.
pub(crate) const ROUTE_CUSTOM_ATTRS: &str = "/custom-attrs";
/// Route path for the badge demo page.
pub(crate) const ROUTE_BADGE: &str = "/badge";
/// Route path for the component-binding demo page.
pub(crate) const ROUTE_COMPONENT_BINDING: &str = "/component-binding";
/// Route path for the browser demo page.
pub(crate) const ROUTE_BROWSER: &str = "/browser";
/// Route path for the camera demo page.
pub(crate) const ROUTE_CAMERA: &str = "/camera";
/// Route path for the canvas demo page.
pub(crate) const ROUTE_CANVAS: &str = "/canvas";
/// Route path for the conditional-rendering demo page.
pub(crate) const ROUTE_CONDITIONAL: &str = "/conditional";
/// Route path for the counter demo page.
pub(crate) const ROUTE_COUNTER: &str = "/counter";
/// Route path for the dynamic-component demo page.
pub(crate) const ROUTE_DYNAMIC_COMPONENT: &str = "/dynamic-component";
/// Route path for the event demo page.
pub(crate) const ROUTE_EVENT: &str = "/event";
/// Route path for the form demo page.
pub(crate) const ROUTE_FORM: &str = "/form";
/// Route path for the timing-hooks demo page.
pub(crate) const ROUTE_HOOKS_TIMING: &str = "/hooks-timing";
/// Route path for the async-hooks demo page.
pub(crate) const ROUTE_HOOKS_ASYNC: &str = "/hooks-async";
/// Route path for the guarded-hooks demo page.
pub(crate) const ROUTE_HOOKS_PROTECT: &str = "/hooks-protect";
/// Route path for the i18n-hooks demo page.
pub(crate) const ROUTE_HOOKS_I18N: &str = "/hooks-i18n";
/// Route path for the 2D game demo page.
pub(crate) const ROUTE_GAME_2D: &str = "/game-2d";
/// Route path for the 3D game demo page.
pub(crate) const ROUTE_GAME_3D: &str = "/game-3d";
/// Route path for the keep-alive demo page.
pub(crate) const ROUTE_KEEP_ALIVE: &str = "/keep-alive";
/// Route path for the lifecycle demo page.
pub(crate) const ROUTE_LIFECYCLE: &str = "/lifecycle";
/// Route path for the lighting demo page.
pub(crate) const ROUTE_LIGHTING: &str = "/lighting";
/// Route path for the list demo page.
pub(crate) const ROUTE_LIST: &str = "/list";
/// Route path for the modal demo page.
pub(crate) const ROUTE_MODAL: &str = "/modal";
/// Route path for the observer demo page.
pub(crate) const ROUTE_OBSERVER: &str = "/observer";
/// Route path for the ray-tracing demo page.
pub(crate) const ROUTE_RAYTRACE: &str = "/raytrace";
/// Route path for the server-sent-events demo page.
pub(crate) const ROUTE_SSE: &str = "/sse";
/// Route path for the select demo page.
pub(crate) const ROUTE_SELECT: &str = "/select";
/// Route path for the timer demo page.
pub(crate) const ROUTE_TIMER: &str = "/timer";
/// Route path for the file-upload demo page.
pub(crate) const ROUTE_FILE_UPLOAD: &str = "/file-upload";
/// Route path for the virtual-list demo page.
pub(crate) const ROUTE_VIRTUAL_LIST: &str = "/virtual-list";
/// Route path for the websocket demo page.
pub(crate) const ROUTE_WEBSOCKET: &str = "/websocket";
