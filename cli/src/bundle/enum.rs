use super::*;

/// A native platform the euv CLI can build a client for.
///
/// Every variant names one deliverable the native builders in
/// `native/tauri` produce, and every variant declares the hosts it can be
/// built on so an incompatible request fails before any heavy work
/// starts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum NativePlatform {
    /// The macOS desktop application bundle and disk image.
    Macos,
    /// The Linux desktop application packages.
    Linux,
    /// The Windows desktop application packages.
    Windows,
    /// The iOS device application archive.
    Ios,
    /// The iOS simulator application bundle.
    IosSimulator,
    /// The Android application package.
    Android,
}

/// The host operating systems the native builders support.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostOs {
    /// A macOS host, which builds the Apple and Android deliverables.
    Macos,
    /// A Linux host, which builds the Linux and Android deliverables.
    Linux,
    /// A Windows host, which builds the Windows deliverable.
    Windows,
    /// A host with no supported native packaging toolchain.
    Unsupported,
}

/// The kind of work a native platform needs from the native builders.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeTask {
    /// The platform is built by the Android build script.
    Android,
    /// The platform is built by the per-platform client builder.
    Client,
}
