use super::*;

/// Declarative custom URL protocol registration and universal/app link specification.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct ProtocolRegistration {
    /// Custom URI scheme name (e.g. `"euv"`, `"myapp"`).
    pub(crate) scheme: String,
    /// Human-readable handler display name shown in OS chooser dialogs.
    pub(crate) display_name: String,
    /// Reverse-DNS bundle or application identifier (e.g. `"dev.euv.app"`).
    pub(crate) bundle_id: String,
    /// Optional desktop executable path invoked for custom URI launches.
    #[new(skip)]
    pub(crate) executable_path: String,
    /// Optional HTTPS domains for iOS/iPadOS Associated Domains and Android App Links.
    #[new(skip)]
    pub(crate) universal_domains: Vec<String>,
}

/// Parsed incoming custom-scheme or universal deep link event.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct DeepLinkEvent {
    /// Original full URI string.
    pub(crate) raw_url: String,
    /// Normalized lowercase scheme (e.g. `"euv"`).
    pub(crate) scheme: String,
    /// Authority or host segment.
    pub(crate) host: String,
    /// Normalized path segment beginning with `/` when non-empty.
    pub(crate) path: String,
    /// Parsed query key-value pairs.
    pub(crate) query: HashMap<String, String>,
}

/// Platform-specific manifest or OS registration artifact produced for a `ProtocolRegistration`.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct ProtocolManifestEntry {
    /// Target execution platform.
    #[get(type(copy))]
    pub(crate) platform: PlatformKind,
    /// Native OS registration mechanism name.
    pub(crate) mechanism: String,
    /// Ready-to-apply manifest snippet, registry path, desktop entry, or Web API invocation.
    pub(crate) descriptor: String,
}

/// Cross-platform registry managing custom URI protocol handlers and incoming deep links.
#[derive(Clone, Data, Debug, Default, New)]
pub struct ProtocolRegistry {
    /// Registered custom protocol definitions.
    #[new(skip)]
    pub(crate) registrations: Vec<ProtocolRegistration>,
    /// Queue of received deep link events awaiting app consumption.
    #[new(skip)]
    pub(crate) pending_links: Vec<DeepLinkEvent>,
}
