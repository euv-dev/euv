/// All supported execution targets across Web, Mobile, Tablet, Desktop, and Headless.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PlatformKind {
    /// Browser WebAssembly platform using HTML5 Canvas / WebGL2 / WebGPU.
    Web,
    /// Android native platform using Android NDK (`ANativeWindow` / `AInput` / Vulkan / GLES).
    Android,
    /// iOS phone native platform using UIKit / CoreGraphics / Metal.
    Ios,
    /// iPadOS tablet native platform using UIKit / Stage Manager / Apple Pencil / Metal.
    IpadOs,
    /// macOS desktop native platform using Cocoa AppKit / CoreGraphics / Metal.
    MacOs,
    /// Windows desktop native platform using Win32 / GDI32 / DWM / Direct3D.
    Windows,
    /// Linux desktop native platform using X11 / Wayland / Vulkan.
    Linux,
    /// Deterministic offscreen headless platform for automated parity tests and benchmarks.
    #[default]
    Headless,
}

/// Physical device form factor used for adaptive layout and input affordances.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeviceFormFactor {
    /// Multi-window desktop workstation.
    #[default]
    Desktop,
    /// Large-screen touch + stylus + optional keyboard tablet (e.g. iPadOS).
    Tablet,
    /// Compact handheld phone (iOS / Android).
    Phone,
    /// Web browser viewport.
    Browser,
    /// Offscreen headless test host.
    Headless,
}

/// Zero-dependency ABI-stable native OS window handle for GPU surface creation.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum RawWindowHandle {
    /// macOS AppKit handle carrying `(ns_window, ns_view, ca_metal_layer)` pointers.
    AppKit {
        /// Pointer to `NSWindow`.
        ns_window: usize,
        /// Pointer to `NSView`.
        ns_view: usize,
        /// Pointer to `CAMetalLayer`.
        metal_layer: usize,
    },
    /// iOS / iPadOS UIKit handle carrying `(ui_window, ui_view, ca_metal_layer, is_ipad)`.
    UiKit {
        /// Pointer to `UIWindow`.
        ui_window: usize,
        /// Pointer to `UIView`.
        ui_view: usize,
        /// Pointer to `CAMetalLayer`.
        metal_layer: usize,
        /// True when running on iPadOS tablet idiom.
        is_ipad: bool,
    },
    /// Windows Win32 handle carrying `(hwnd, hinstance)`.
    Win32 {
        /// Win32 `HWND` window handle.
        hwnd: usize,
        /// Win32 `HINSTANCE` module handle.
        hinstance: usize,
    },
    /// Linux X11 Xlib handle carrying `(window, display)`.
    Xlib {
        /// X11 `Window` identifier.
        window: u64,
        /// X11 `Display*` pointer.
        display: usize,
    },
    /// Linux Wayland handle carrying `(wl_surface, wl_display)`.
    Wayland {
        /// Pointer to `wl_surface`.
        surface: usize,
        /// Pointer to `wl_display`.
        display: usize,
    },
    /// Android NDK handle carrying `ANativeWindow*`.
    AndroidNdk {
        /// Pointer to `ANativeWindow`.
        native_window: usize,
    },
    /// Web browser canvas element identifier.
    WebCanvas {
        /// Canvas registration identifier.
        canvas_id: u32,
    },
    /// Offscreen headless surface with no OS window handle.
    #[default]
    Headless,
}
