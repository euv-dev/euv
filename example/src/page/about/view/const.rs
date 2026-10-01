/// The name of the euv package.
pub(crate) const EUV_PACKAGE_NAME: &str = env!("EUV_PACKAGE_NAME");

/// The current version of the euv package.
pub(crate) const EUV_VERSION: &str = env!("EUV_VERSION");

/// The description of the euv package.
pub(crate) const EUV_DESCRIPTION: &str = env!("EUV_DESCRIPTION");

/// The repository URL of the euv package.
pub(crate) const EUV_REPOSITORY: &str = env!("EUV_REPOSITORY");

/// The repository organization and project name of the euv package.
pub(crate) const EUV_REPOSITORY_NAME: &str = env!("EUV_REPOSITORY_NAME");

/// The authors of the euv package.
pub(crate) const EUV_AUTHORS: &str = env!("EUV_AUTHORS");

/// The license of the euv package.
pub(crate) const EUV_LICENSE: &str = env!("EUV_LICENSE");

/// The Rust edition used by the euv package.
pub(crate) const EUV_EDITION: &str = env!("EUV_EDITION");

/// The build date of the euv package, formatted as YYYY-MM-DD.
pub(crate) const EUV_BUILD_DATE: &str = env!("EUV_BUILD_DATE");

/// The build clock time of the euv package, formatted as HH:MM:SS.
pub(crate) const EUV_BUILD_CLOCK: &str = env!("EUV_BUILD_CLOCK");

/// The build Unix microsecond timestamp of the euv package, as a pure numeric value.
pub(crate) const EUV_BUILD_TIMESTAMP: &str = env!("EUV_BUILD_TIMESTAMP");

/// The label of the native bridge permissions info row.
pub(crate) const ABOUT_BRIDGE_PERMISSIONS_LABEL: &str = "Permissions";

/// The card title of the native bridge integration card.
pub(crate) const ABOUT_CARD_BRIDGE_INTEGRATION_TITLE: &str = "bridge Integration";

/// The heading of the native bridge section.
pub(crate) const ABOUT_SECTION_NATIVE_BRIDGE_TITLE: &str = "Native Bridge";

/// The label of the build timestamp info row.
pub(crate) const ABOUT_BUILD_TIMESTAMP_LABEL: &str = "Timestamp";

/// The label of the build clock time info row.
pub(crate) const ABOUT_BUILD_TIME_LABEL: &str = "Time";

/// The label of the build date info row.
pub(crate) const ABOUT_BUILD_DATE_LABEL: &str = "Date";

/// The card title of the build information card.
pub(crate) const ABOUT_CARD_BUILD_INFORMATION_TITLE: &str = "Build Information";

/// The link target opening an external URL in a new browsing context.
pub(crate) const ABOUT_LINK_TARGET_BLANK: &str = "_blank";

/// The label of the package repository info row.
pub(crate) const ABOUT_INFO_REPOSITORY_LABEL: &str = "Repository";

/// The label of the package authors info row.
pub(crate) const ABOUT_INFO_AUTHORS_LABEL: &str = "Authors";

/// The label of the package license info row.
pub(crate) const ABOUT_INFO_LICENSE_LABEL: &str = "License";

/// The label of the package edition info row.
pub(crate) const ABOUT_INFO_EDITION_LABEL: &str = "Edition";

/// The label of the package version info row.
pub(crate) const ABOUT_INFO_VERSION_LABEL: &str = "Version";

/// The label of the package name info row.
pub(crate) const ABOUT_INFO_NAME_LABEL: &str = "Name";

/// The card title of the project details card.
pub(crate) const ABOUT_CARD_PROJECT_DETAILS_TITLE: &str = "Project Details";

/// The heading of the package info section.
pub(crate) const ABOUT_SECTION_PACKAGE_INFO_TITLE: &str = "Package Info";

/// The description of the cross-platform feature.
pub(crate) const ABOUT_FEATURE_CROSS_PLATFORM_DESC: &str = "Run anywhere with WASM — browsers, servers, and native platforms via the bridge. Share the same Rust codebase across all targets.";

/// The name of the cross-platform feature.
pub(crate) const ABOUT_FEATURE_CROSS_PLATFORM_NAME: &str = "WebAssembly Powered";

/// The card title of the cross-platform feature.
pub(crate) const ABOUT_FEATURE_CROSS_PLATFORM_TITLE: &str = "Cross-Platform";

/// The description of the HTML macros feature.
pub(crate) const ABOUT_FEATURE_HTML_MACROS_DESC: &str = "Write UI with familiar HTML-like macros that compile to efficient Rust at build time. No runtime template engine — just zero-cost abstractions.";

/// The name of the HTML macros feature.
pub(crate) const ABOUT_FEATURE_HTML_MACROS_NAME: &str = "Declarative Syntax";

/// The card title of the HTML macros feature.
pub(crate) const ABOUT_FEATURE_HTML_MACROS_TITLE: &str = "HTML Macros";

/// The description of the virtual DOM feature.
pub(crate) const ABOUT_FEATURE_VIRTUAL_DOM_DESC: &str = "Virtual DOM with optimized reconciliation for smooth 60fps updates. The differ computes the minimal set of DOM operations needed to sync the UI with the latest state.";

/// The name of the virtual DOM feature.
pub(crate) const ABOUT_FEATURE_VIRTUAL_DOM_NAME: &str = "Efficient Diffing";

/// The card title of the virtual DOM feature.
pub(crate) const ABOUT_FEATURE_VIRTUAL_DOM_TITLE: &str = "Virtual DOM";

/// The description of the reactive signals feature.
pub(crate) const ABOUT_FEATURE_REACTIVE_SIGNALS_DESC: &str = "Fine-grained reactive state management with automatic dependency tracking. Signals only notify dependents that read them, avoiding unnecessary re-renders.";

/// The name of the reactive signals feature.
pub(crate) const ABOUT_FEATURE_REACTIVE_SIGNALS_NAME: &str = "Signal-Based Reactivity";

/// The card title of the reactive signals feature.
pub(crate) const ABOUT_FEATURE_REACTIVE_SIGNALS_TITLE: &str = "Reactive Signals";

/// The description below the features section heading.
pub(crate) const ABOUT_SECTION_FEATURES_DESC: &str =
    "Everything you need for declarative cross-platform UI development.";

/// The heading of the features section.
pub(crate) const ABOUT_SECTION_FEATURES_TITLE: &str = "Features";

/// The label of the crates-count home stat card.
pub(crate) const ABOUT_STAT_CRATES_LABEL: &str = "Crates";

/// The label of the VDOM home stat card.
pub(crate) const ABOUT_STAT_ARCHITECTURE_LABEL: &str = "Architecture";

/// The VDOM value of the home stat card.
pub(crate) const ABOUT_STAT_VDOM_VALUE: &str = "VDOM";

/// The label of the Rust home stat card.
pub(crate) const ABOUT_STAT_LANGUAGE_LABEL: &str = "Language";

/// The Rust value of the home stat card.
pub(crate) const ABOUT_STAT_RUST_VALUE: &str = "Rust";

/// The label of the WASM home stat card.
pub(crate) const ABOUT_STAT_RUNTIME_LABEL: &str = "Runtime";

/// The WASM value of the home stat card.
pub(crate) const ABOUT_STAT_WASM_VALUE: &str = "WASM";

/// The label of the button that browses to the next route.
pub(crate) const ABOUT_BROWSE_BUTTON_LABEL: &str = "Browse";

/// The label of the external GitHub repository button.
pub(crate) const ABOUT_GITHUB_BUTTON_LABEL: &str = "GitHub";
