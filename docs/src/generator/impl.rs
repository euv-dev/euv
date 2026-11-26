use super::*;

/// Default build options enabling clean directory URLs with no base prefix or dev HMR script.
impl Default for DocsBuildOptions {
    /// Returns default `DocsBuildOptions` with `clean_urls = true`.
    ///
    /// # Returns
    ///
    /// - `Self` - Default documentation build options.
    fn default() -> Self {
        Self {
            title: None,
            base_url: String::new(),
            hmr_endpoint: None,
            assets_root: None,
            clean_urls: true,
        }
    }
}
