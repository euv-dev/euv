/// Objective-C class name for `UIDevice`.
pub(crate) const UIKIT_CLS_UI_DEVICE: &str = "UIDevice\0";

/// Objective-C class name for `UIScreen`.
pub(crate) const UIKIT_CLS_UI_SCREEN: &str = "UIScreen\0";

/// Objective-C class name for `UIWindow`.
pub(crate) const UIKIT_CLS_UI_WINDOW: &str = "UIWindow\0";

/// Objective-C class name for `UIView`.
pub(crate) const UIKIT_CLS_UI_VIEW: &str = "UIView\0";

/// Objective-C class name for `CAMetalLayer`.
pub(crate) const UIKIT_CLS_CA_METAL_LAYER: &str = "CAMetalLayer\0";

/// Objective-C selector for `currentDevice`.
pub(crate) const UIKIT_SEL_CURRENT_DEVICE: &str = "currentDevice\0";

/// Objective-C selector for `userInterfaceIdiom`.
pub(crate) const UIKIT_SEL_USER_INTERFACE_IDIOM: &str = "userInterfaceIdiom\0";

/// Objective-C selector for `mainScreen`.
pub(crate) const UIKIT_SEL_MAIN_SCREEN: &str = "mainScreen\0";

/// Objective-C selector for `nativeScale`.
pub(crate) const UIKIT_SEL_NATIVE_SCALE: &str = "nativeScale\0";

/// Objective-C selector for `traitCollection`.
pub(crate) const UIKIT_SEL_TRAIT_COLLECTION: &str = "traitCollection\0";

/// Objective-C selector for `userInterfaceStyle`.
pub(crate) const UIKIT_SEL_USER_INTERFACE_STYLE: &str = "userInterfaceStyle\0";

/// Objective-C selector for `alloc`.
pub(crate) const UIKIT_SEL_ALLOC: &str = "alloc\0";

/// Objective-C selector for `init`.
pub(crate) const UIKIT_SEL_INIT: &str = "init\0";

/// Objective-C selector for `layer`.
pub(crate) const UIKIT_SEL_LAYER: &str = "layer\0";

/// Objective-C selector for `setContents:`.
pub(crate) const UIKIT_SEL_SET_CONTENTS: &str = "setContents:\0";

/// `UIUserInterfaceIdiomPad` enum integer value (`1`).
pub(crate) const UI_USER_INTERFACE_IDIOM_PAD: isize = 1;

/// `UIUserInterfaceStyleDark` enum integer value (`2`).
pub(crate) const UI_USER_INTERFACE_STYLE_DARK: isize = 2;

/// Default iOS phone top safe-area inset (Dynamic Island / status bar) in logical points.
pub(crate) const IOS_DEFAULT_SAFE_TOP: f32 = 47.0;

/// Default iOS phone bottom home-indicator safe-area inset in logical points.
pub(crate) const IOS_DEFAULT_SAFE_BOTTOM: f32 = 34.0;

/// Default iPadOS top status-bar safe-area inset in logical points.
pub(crate) const IPADOS_DEFAULT_SAFE_TOP: f32 = 24.0;

/// Default iPadOS bottom gesture-bar safe-area inset in logical points.
pub(crate) const IPADOS_DEFAULT_SAFE_BOTTOM: f32 = 20.0;

/// CoreGraphics RGBA8 bitmap info flags.
pub(crate) const IOS_CG_BITMAP_INFO_RGBA8: u32 = (4 << 12) | 1;
