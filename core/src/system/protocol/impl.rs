use super::*;

/// Validation and platform manifest generation for `ProtocolRegistration`.
impl ProtocolRegistration {
    /// Validates that `scheme` conforms to RFC 3986 and excludes dangerous reserved schemes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Protocol registration reference.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the custom scheme is valid and safe to register.
    #[must_use]
    pub fn is_valid_scheme(&self) -> bool {
        let lower: String = self.get_scheme().to_ascii_lowercase();
        let mut chars: Chars<'_> = lower.chars();
        let Some(first) = chars.next() else {
            return false;
        };
        if !first.is_ascii_alphabetic() {
            return false;
        }
        if !chars.all(|c: char| {
            c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '+' | '-' | '.')
        }) {
            return false;
        }
        !matches!(
            lower.as_str(),
            "http" | "https" | "file" | "javascript" | "data" | "vbscript" | "about"
        )
    }

    /// Generates the native OS protocol registration descriptor for `platform`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Protocol registration definition.
    /// - `PlatformKind` - Target platform.
    ///
    /// # Returns
    ///
    /// - `ProtocolManifestEntry` - Native registration descriptor.
    #[must_use]
    pub fn platform_descriptor(&self, platform: PlatformKind) -> ProtocolManifestEntry {
        let scheme: String = self.get_scheme().to_ascii_lowercase();
        let bundle: &str = self.get_bundle_id().as_str();
        let name: &str = self.get_display_name().as_str();
        let exe: &str = if self.get_executable_path().is_empty() {
            "%EXE%"
        } else {
            self.get_executable_path().as_str()
        };
        match platform {
            PlatformKind::MacOs => ProtocolManifestEntry::new(
                platform,
                "LaunchServices / Info.plist CFBundleURLTypes".to_string(),
                format!(
                    "CFBundleURLName={bundle}; CFBundleURLSchemes=[{scheme}]; LSSetDefaultHandlerForURLScheme({scheme}, {bundle})"
                ),
            ),
            PlatformKind::Ios | PlatformKind::IpadOs => {
                let domains: String = self
                    .get_universal_domains()
                    .iter()
                    .map(|d: &String| format!("applinks:{d}"))
                    .collect::<Vec<String>>()
                    .join(",");
                ProtocolManifestEntry::new(
                    platform,
                    "UIKit CFBundleURLTypes + Associated Domains".to_string(),
                    format!("CFBundleURLSchemes=[{scheme}]; AssociatedDomains=[{domains}]"),
                )
            }
            PlatformKind::Windows => ProtocolManifestEntry::new(
                platform,
                "Win32 HKCU\\Software\\Classes URL Protocol".to_string(),
                format!(
                    "HKCU\\Software\\Classes\\{scheme}\\shell\\open\\command = \"{exe}\" \"%1\" (URL Protocol=\"\", Name=\"{name}\")"
                ),
            ),
            PlatformKind::Linux => ProtocolManifestEntry::new(
                platform,
                "XDG Desktop Entry x-scheme-handler".to_string(),
                format!(
                    "[Desktop Entry]\nName={name}\nExec={exe} %u\nType=Application\nMimeType=x-scheme-handler/{scheme};"
                ),
            ),
            PlatformKind::Android => ProtocolManifestEntry::new(
                platform,
                "AndroidManifest intent-filter VIEW/BROWSABLE".to_string(),
                format!(
                    "<intent-filter><action android:name=\"android.intent.action.VIEW\"/><category android:name=\"android.intent.category.DEFAULT\"/><category android:name=\"android.intent.category.BROWSABLE\"/><data android:scheme=\"{scheme}\"/></intent-filter>"
                ),
            ),
            PlatformKind::Web | PlatformKind::Headless => ProtocolManifestEntry::new(
                platform,
                "Web navigator.registerProtocolHandler".to_string(),
                format!("navigator.registerProtocolHandler(\"web+{scheme}\", \"/?uri=%s\")"),
            ),
        }
    }
}

/// URI parser for `DeepLinkEvent`.
impl DeepLinkEvent {
    /// Parses a raw custom-scheme or universal link URL into a structured `DeepLinkEvent`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Raw incoming URI string (e.g. `"euv://workspace/render?mode=metal"`).
    ///
    /// # Returns
    ///
    /// - `Result<Self, PlatformError>` - Structured deep link event or validation error.
    pub fn parse(raw_url: &str) -> Result<Self, PlatformError> {
        let trimmed: &str = raw_url.trim();
        let Some((scheme_raw, rest)) = trimmed.split_once("://") else {
            return Err(PlatformError::Unsupported(
                "Invalid deep link URI: missing '://' separator".to_string(),
            ));
        };
        let scheme: String = scheme_raw.to_ascii_lowercase();
        if scheme.is_empty() {
            return Err(PlatformError::Unsupported(
                "Invalid deep link URI: empty scheme".to_string(),
            ));
        }
        let (authority_and_path, query_raw): (&str, &str) = match rest.split_once('?') {
            Some((left, right)) => (left, right),
            None => (rest, ""),
        };
        let (host, path): (String, String) = match authority_and_path.split_once('/') {
            Some((h, p)) => (h.to_string(), format!("/{p}")),
            None => (authority_and_path.to_string(), "/".to_string()),
        };
        let mut query: HashMap<String, String> = HashMap::new();
        if !query_raw.is_empty() {
            for pair in query_raw.split('&') {
                if pair.is_empty() {
                    continue;
                }
                match pair.split_once('=') {
                    Some((k, v)) => {
                        let _: Option<String> = query.insert(k.to_string(), v.to_string());
                    }
                    None => {
                        let _: Option<String> = query.insert(pair.to_string(), String::new());
                    }
                }
            }
        }
        Ok(Self::new(trimmed.to_string(), scheme, host, path, query))
    }
}

/// Registration and incoming link queuing for `ProtocolRegistry`.
impl ProtocolRegistry {
    /// Validates and registers `registration` for `platform`, returning its OS manifest descriptor.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable protocol registry.
    /// - `ProtocolRegistration` - Custom protocol specification.
    /// - `PlatformKind` - Active platform kind.
    ///
    /// # Returns
    ///
    /// - `Result<ProtocolManifestEntry, PlatformError>` - Generated platform registration descriptor.
    pub fn register(
        &mut self,
        registration: ProtocolRegistration,
        platform: PlatformKind,
    ) -> Result<ProtocolManifestEntry, PlatformError> {
        if !registration.is_valid_scheme() {
            return Err(PlatformError::Unsupported(format!(
                "Invalid or reserved custom URL scheme: {}",
                registration.get_scheme()
            )));
        }
        let entry: ProtocolManifestEntry = registration.platform_descriptor(platform);
        let scheme_lower: String = registration.get_scheme().to_ascii_lowercase();
        self.get_mut_registrations()
            .retain(|existing: &ProtocolRegistration| {
                existing.get_scheme().to_ascii_lowercase() != scheme_lower
            });
        self.get_mut_registrations().push(registration);
        Ok(entry)
    }

    /// Returns `true` if `scheme` has been registered.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Protocol registry reference.
    /// - `&str` - Scheme name to check.
    ///
    /// # Returns
    ///
    /// - `bool` - Registration status.
    #[must_use]
    pub fn is_registered(&self, scheme: &str) -> bool {
        let target: String = scheme.to_ascii_lowercase();
        self.get_registrations()
            .iter()
            .any(|reg: &ProtocolRegistration| reg.get_scheme().to_ascii_lowercase() == target)
    }

    /// Parses and queues a raw incoming deep link URI.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable protocol registry.
    /// - `&str` - Raw incoming URI string.
    ///
    /// # Returns
    ///
    /// - `Result<DeepLinkEvent, PlatformError>` - Parsed deep link event.
    pub fn push_raw_url(&mut self, raw_url: &str) -> Result<DeepLinkEvent, PlatformError> {
        let parsed: DeepLinkEvent = DeepLinkEvent::parse(raw_url)?;
        self.get_mut_pending_links().push(parsed.clone());
        Ok(parsed)
    }

    /// Drains all queued deep link events.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable protocol registry.
    ///
    /// # Returns
    ///
    /// - `Vec<DeepLinkEvent>` - Drained deep links in arrival order.
    pub fn drain_links(&mut self) -> Vec<DeepLinkEvent> {
        take(self.get_mut_pending_links())
    }
}
