/// The reason the catalog response could not be read as UTF-8.
pub const ERROR_CATALOG_ENCODING: &str = "The catalog response is not UTF-8";

/// The reason the catalog response does not have the expected shape.
pub const ERROR_CATALOG_SHAPE: &str = "The catalog response has an unexpected shape";

/// The path of the public catalog API.
pub const CATALOG_URL: &str = "/api/page-artifacts";

/// The class the catalog cards carry.
///
/// The name is the contract the documentation runtime validator asserts
/// on, so it stays stable even though the styling lives in the shared
/// `c_catalog*` classes of the framework.
pub const CATALOG_CARD_CLASS: &str = "docs-catalog-card";

/// The template of a published project page link.
pub const CATALOG_LINK_TEMPLATE: &str = "/pages/{name}/";
