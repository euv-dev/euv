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

/// Objective-C class name for `NSCursor`.
pub(crate) const OBJC_CLS_NS_CURSOR: &str = "NSCursor\0";

/// Objective-C class name for `NSPasteboard`.
pub(crate) const OBJC_CLS_NS_PASTEBOARD: &str = "NSPasteboard\0";

/// Objective-C class name for `NSProcessInfo`.
pub(crate) const OBJC_CLS_NS_PROCESS_INFO: &str = "NSProcessInfo\0";

/// Objective-C selector for `processInfo` on `NSProcessInfo`.
pub(crate) const OBJC_SEL_PROCESS_INFO: &str = "processInfo\0";

/// Objective-C selector for `isLowPowerModeEnabled` on `NSProcessInfo`.
pub(crate) const OBJC_SEL_IS_LOW_POWER_MODE: &str = "isLowPowerModeEnabled\0";

/// Objective-C selector for `isActive` on `NSApplication`.
pub(crate) const OBJC_SEL_IS_ACTIVE: &str = "isActive\0";

/// Objective-C selector for `isKeyWindow` on `NSWindow`.
pub(crate) const OBJC_SEL_IS_KEY_WINDOW: &str = "isKeyWindow\0";

/// Objective-C selector for `isMiniaturized` on `NSWindow`.
pub(crate) const OBJC_SEL_IS_MINIATURIZED: &str = "isMiniaturized\0";

/// Objective-C selector for `changeCount` on `NSPasteboard`.
pub(crate) const OBJC_SEL_CHANGE_COUNT: &str = "changeCount\0";

/// Objective-C selector for `generalPasteboard` on `NSPasteboard`.
pub(crate) const OBJC_SEL_GENERAL_PASTEBOARD: &str = "generalPasteboard\0";

/// Objective-C selector for `clearContents` on `NSPasteboard`.
pub(crate) const OBJC_SEL_CLEAR_CONTENTS: &str = "clearContents\0";

/// Objective-C selector for `setString:forType:` on `NSPasteboard`.
pub(crate) const OBJC_SEL_SET_STRING_FOR_TYPE: &str = "setString:forType:\0";

/// UTI string identifier for UTF-8 plain text on macOS `NSPasteboard`.
pub(crate) const MACOS_PASTEBOARD_TYPE_UTF8: &str = "public.utf8-plain-text";

/// Objective-C selector for `arrowCursor`.
pub(crate) const OBJC_SEL_ARROW_CURSOR: &str = "arrowCursor\0";

/// Objective-C selector for `pointingHandCursor`.
pub(crate) const OBJC_SEL_POINTING_HAND_CURSOR: &str = "pointingHandCursor\0";

/// Objective-C selector for `IBeamCursor`.
pub(crate) const OBJC_SEL_IBEAM_CURSOR: &str = "IBeamCursor\0";

/// Objective-C selector for `openHandCursor`.
pub(crate) const OBJC_SEL_OPEN_HAND_CURSOR: &str = "openHandCursor\0";

/// Objective-C selector for `closedHandCursor`.
pub(crate) const OBJC_SEL_CLOSED_HAND_CURSOR: &str = "closedHandCursor\0";

/// Objective-C selector for `operationNotAllowedCursor`.
pub(crate) const OBJC_SEL_NOT_ALLOWED_CURSOR: &str = "operationNotAllowedCursor\0";

/// Objective-C selector for `set` on `NSCursor`.
pub(crate) const OBJC_SEL_SET_CURSOR: &str = "set\0";

/// Objective-C selector for `hasPreciseScrollingDeltas` on `NSEvent`.
pub(crate) const OBJC_SEL_HAS_PRECISE_SCROLL: &str = "hasPreciseScrollingDeltas\0";

/// Objective-C selector for `sharedApplication`.
pub(crate) const OBJC_SEL_SHARED_APP: &str = "sharedApplication\0";

/// Objective-C selector for `mainScreen`.
pub(crate) const OBJC_SEL_MAIN_SCREEN: &str = "mainScreen\0";

/// Objective-C selector for `screen` on `NSWindow`.
pub(crate) const OBJC_SEL_SCREEN: &str = "screen\0";

/// Objective-C selector for `maximumFramesPerSecond` on `NSScreen`.
pub(crate) const OBJC_SEL_MAX_FPS: &str = "maximumFramesPerSecond\0";

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

/// Objective-C selector for `setContentsScale:`.
pub(crate) const OBJC_SEL_SET_CONTENTS_SCALE: &str = "setContentsScale:\0";

/// Objective-C selector for `setActivationPolicy:`.
pub(crate) const OBJC_SEL_SET_ACTIVATION_POLICY: &str = "setActivationPolicy:\0";

/// Objective-C selector for `finishLaunching`.
pub(crate) const OBJC_SEL_FINISH_LAUNCHING: &str = "finishLaunching\0";

/// Objective-C selector for `activateIgnoringOtherApps:`.
pub(crate) const OBJC_SEL_ACTIVATE_IGNORING: &str = "activateIgnoringOtherApps:\0";

/// Objective-C selector for `center`.
pub(crate) const OBJC_SEL_CENTER: &str = "center\0";

/// Objective-C selector for `updateWindows`.
pub(crate) const OBJC_SEL_UPDATE_WINDOWS: &str = "updateWindows\0";

/// Objective-C selector for `nextEventMatchingMask:untilDate:inMode:dequeue:`.
pub(crate) const OBJC_SEL_NEXT_EVENT: &str = "nextEventMatchingMask:untilDate:inMode:dequeue:\0";

/// Objective-C selector for `sendEvent:`.
pub(crate) const OBJC_SEL_SEND_EVENT: &str = "sendEvent:\0";

/// Objective-C selector for `isVisible`.
pub(crate) const OBJC_SEL_IS_VISIBLE: &str = "isVisible\0";

/// Objective-C selector for `setAcceptsMouseMovedEvents:`.
pub(crate) const OBJC_SEL_SET_ACCEPTS_MOUSE_MOVED: &str = "setAcceptsMouseMovedEvents:\0";

/// Objective-C selector for `type`.
pub(crate) const OBJC_SEL_TYPE: &str = "type\0";

/// Objective-C selector for `window`.
pub(crate) const OBJC_SEL_WINDOW: &str = "window\0";

/// Objective-C selector for `locationInWindow`.
pub(crate) const OBJC_SEL_LOCATION_IN_WINDOW: &str = "locationInWindow\0";

/// Objective-C selector for `scrollingDeltaX`.
pub(crate) const OBJC_SEL_SCROLLING_DELTA_X: &str = "scrollingDeltaX\0";

/// Objective-C selector for `scrollingDeltaY`.
pub(crate) const OBJC_SEL_SCROLLING_DELTA_Y: &str = "scrollingDeltaY\0";

/// Objective-C selector for `keyCode`.
pub(crate) const OBJC_SEL_KEY_CODE: &str = "keyCode\0";

/// Objective-C selector for `modifierFlags`.
pub(crate) const OBJC_SEL_MODIFIER_FLAGS: &str = "modifierFlags\0";

/// Objective-C selector for `characters`.
pub(crate) const OBJC_SEL_CHARACTERS: &str = "characters\0";

/// Objective-C selector for `charactersIgnoringModifiers`.
pub(crate) const OBJC_SEL_CHARS_IGNORING_MODS: &str = "charactersIgnoringModifiers\0";

/// Objective-C selector for `setTitlebarAppearsTransparent:`.
pub(crate) const OBJC_SEL_SET_TITLEBAR_TRANSPARENT: &str = "setTitlebarAppearsTransparent:\0";

/// Objective-C selector for `setTitleVisibility:`.
pub(crate) const OBJC_SEL_SET_TITLE_VISIBILITY: &str = "setTitleVisibility:\0";

/// Objective-C selector for `bounds`.
pub(crate) const OBJC_SEL_BOUNDS: &str = "bounds\0";

/// AppKit `NSEventTypeLeftMouseDown` constant.
pub(crate) const NS_EVENT_LEFT_MOUSE_DOWN: usize = 1;

/// AppKit `NSEventTypeLeftMouseUp` constant.
pub(crate) const NS_EVENT_LEFT_MOUSE_UP: usize = 2;

/// AppKit `NSEventTypeRightMouseDown` constant.
pub(crate) const NS_EVENT_RIGHT_MOUSE_DOWN: usize = 3;

/// AppKit `NSEventTypeRightMouseUp` constant.
pub(crate) const NS_EVENT_RIGHT_MOUSE_UP: usize = 4;

/// AppKit `NSEventTypeMouseMoved` constant.
pub(crate) const NS_EVENT_MOUSE_MOVED: usize = 5;

/// AppKit `NSEventTypeLeftMouseDragged` constant.
pub(crate) const NS_EVENT_LEFT_MOUSE_DRAGGED: usize = 6;

/// AppKit `NSEventTypeKeyDown` constant.
pub(crate) const NS_EVENT_KEY_DOWN: usize = 10;

/// AppKit `NSEventTypeKeyUp` constant.
pub(crate) const NS_EVENT_KEY_UP: usize = 11;

/// AppKit `NSEventTypeScrollWheel` constant.
pub(crate) const NS_EVENT_SCROLL_WHEEL: usize = 22;

/// AppKit `NSEventModifierFlagShift` bit.
pub(crate) const NS_MOD_SHIFT: usize = 1 << 17;

/// AppKit `NSEventModifierFlagControl` bit.
pub(crate) const NS_MOD_CTRL: usize = 1 << 18;

/// AppKit `NSEventModifierFlagOption` bit.
pub(crate) const NS_MOD_ALT: usize = 1 << 19;

/// AppKit `NSEventModifierFlagCommand` bit.
pub(crate) const NS_MOD_CMD: usize = 1 << 20;

/// Objective-C default RunLoop mode name.
pub(crate) const NS_DEFAULT_RUN_LOOP_MODE: &str = "kCFRunLoopDefaultMode";

/// AppKit event mask matching any event (`NSEventMaskAny`).
pub(crate) const NS_EVENT_MASK_ANY: usize = usize::MAX;

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

/// AppKit style mask bit for immersive full-size content view behind transparent titlebar.
pub(crate) const NS_WINDOW_STYLE_FULL_SIZE_CONTENT: usize = 1 << 15;

/// AppKit style mask for borderless window.
pub(crate) const NS_WINDOW_STYLE_BORDERLESS: usize = 0;

/// AppKit buffered backing store constant (`NSBackingStoreBuffered = 2`).
pub(crate) const NS_BACKING_STORE_BUFFERED: usize = 2;

/// AppKit floating window level (`NSFloatingWindowLevel = 3`).
pub(crate) const NS_FLOATING_WINDOW_LEVEL: isize = 3;

/// AppKit normal window level (`NSNormalWindowLevel = 0`).
pub(crate) const NS_NORMAL_WINDOW_LEVEL: isize = 0;

/// CoreGraphics bitmap info flags for RGBA8 straight-alpha (`kCGBitmapByteOrder32Big | kCGImageAlphaLast`).
pub(crate) const CG_BITMAP_INFO_RGBA8: u32 = (4 << 12) | 3;

/// Error message when `objc_getClass` fails to resolve AppKit class.
pub(crate) const ERR_MACOS_APPKIT_MISSING: &str = "AppKit class resolution failed";
