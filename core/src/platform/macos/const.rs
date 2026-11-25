/// Objective-C class name for `NSApplication`.
pub(crate) const OBJC_CLS_NS_APPLICATION: &str = "NSApplication\0";

/// Objective-C class name for `NSWindow`.
pub(crate) const OBJC_CLS_NS_WINDOW: &str = "NSWindow\0";

/// Objective-C class name for `NSScreen`.
pub(crate) const OBJC_CLS_NS_SCREEN: &str = "NSScreen\0";

/// Objective-C class name for `NSString`.
pub(crate) const OBJC_CLS_NS_STRING: &str = "NSString\0";

/// Objective-C class name for `CAMetalLayer`.
pub(crate) const OBJC_CLS_CA_METAL_LAYER: &str = "CAMetalLayer\0";

/// Objective-C selector for `sharedApplication`.
pub(crate) const OBJC_SEL_SHARED_APP: &str = "sharedApplication\0";

/// Objective-C selector for `mainScreen`.
pub(crate) const OBJC_SEL_MAIN_SCREEN: &str = "mainScreen\0";

/// Objective-C selector for `backingScaleFactor`.
pub(crate) const OBJC_SEL_BACKING_SCALE: &str = "backingScaleFactor\0";

/// Objective-C selector for `effectiveAppearance`.
pub(crate) const OBJC_SEL_EFFECTIVE_APPEARANCE: &str = "effectiveAppearance\0";

/// Objective-C selector for `name`.
pub(crate) const OBJC_SEL_NAME: &str = "name\0";

/// Objective-C selector for `UTF8String`.
pub(crate) const OBJC_SEL_UTF8_STRING: &str = "UTF8String\0";

/// Objective-C selector for `alloc`.
pub(crate) const OBJC_SEL_ALLOC: &str = "alloc\0";

/// Objective-C selector for `initWithContentRect:styleMask:backing:defer:`.
pub(crate) const OBJC_SEL_INIT_WINDOW: &str = "initWithContentRect:styleMask:backing:defer:\0";

/// Objective-C selector for `setTitle:`.
pub(crate) const OBJC_SEL_SET_TITLE: &str = "setTitle:\0";

/// Objective-C selector for `stringWithUTF8String:`.
pub(crate) const OBJC_SEL_STRING_WITH_UTF8: &str = "stringWithUTF8String:\0";

/// Objective-C selector for `contentView`.
pub(crate) const OBJC_SEL_CONTENT_VIEW: &str = "contentView\0";

/// Objective-C selector for `setWantsLayer:`.
pub(crate) const OBJC_SEL_SET_WANTS_LAYER: &str = "setWantsLayer:\0";

/// Objective-C selector for `setLayer:`.
pub(crate) const OBJC_SEL_SET_LAYER: &str = "setLayer:\0";

/// Objective-C selector for `layer`.
pub(crate) const OBJC_SEL_LAYER: &str = "layer\0";

/// Objective-C selector for `makeKeyAndOrderFront:`.
pub(crate) const OBJC_SEL_MAKE_KEY_FRONT: &str = "makeKeyAndOrderFront:\0";

/// Objective-C selector for `orderOut:`.
pub(crate) const OBJC_SEL_ORDER_OUT: &str = "orderOut:\0";

/// Objective-C selector for `close`.
pub(crate) const OBJC_SEL_CLOSE: &str = "close\0";

/// Objective-C selector for `setLevel:`.
pub(crate) const OBJC_SEL_SET_LEVEL: &str = "setLevel:\0";

/// Objective-C selector for `setContentSize:`.
pub(crate) const OBJC_SEL_SET_CONTENT_SIZE: &str = "setContentSize:\0";

/// Objective-C selector for `setStyleMask:`.
pub(crate) const OBJC_SEL_SET_STYLE_MASK: &str = "setStyleMask:\0";

/// Objective-C selector for `toggleFullScreen:`.
pub(crate) const OBJC_SEL_TOGGLE_FULLSCREEN: &str = "toggleFullScreen:\0";

/// Objective-C selector for `setContents:`.
pub(crate) const OBJC_SEL_SET_CONTENTS: &str = "setContents:\0";

/// Objective-C dark appearance name substring.
pub(crate) const MACOS_DARK_APPEARANCE_MARKER: &str = "Dark";

/// AppKit style mask bit for titled window.
pub(crate) const NS_WINDOW_STYLE_TITLED: usize = 1 << 0;

/// AppKit style mask bit for closable window.
pub(crate) const NS_WINDOW_STYLE_CLOSABLE: usize = 1 << 1;

/// AppKit style mask bit for miniaturizable window.
pub(crate) const NS_WINDOW_STYLE_MINIATURIZABLE: usize = 1 << 2;

/// AppKit style mask bit for resizable window.
pub(crate) const NS_WINDOW_STYLE_RESIZABLE: usize = 1 << 3;

/// AppKit style mask for borderless window.
pub(crate) const NS_WINDOW_STYLE_BORDERLESS: usize = 0;

/// AppKit buffered backing store constant (`NSBackingStoreBuffered = 2`).
pub(crate) const NS_BACKING_STORE_BUFFERED: usize = 2;

/// AppKit floating window level (`NSFloatingWindowLevel = 3`).
pub(crate) const NS_FLOATING_WINDOW_LEVEL: isize = 3;

/// AppKit normal window level (`NSNormalWindowLevel = 0`).
pub(crate) const NS_NORMAL_WINDOW_LEVEL: isize = 0;

/// CoreGraphics bitmap info flags for RGBA8 premultiplied-last (`kCGBitmapByteOrder32Big | kCGImageAlphaPremultipliedLast`).
pub(crate) const CG_BITMAP_INFO_RGBA8: u32 = (4 << 12) | 1;

/// Error message when `objc_getClass` fails to resolve AppKit class.
pub(crate) const ERR_MACOS_APPKIT_MISSING: &str = "AppKit class resolution failed";
