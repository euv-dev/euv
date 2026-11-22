use super::*;

/// Renders the public catalog of stored projects.
///
/// The catalog is fetched from the artifact API and rendered in Rust, so
/// the documentation home page needs no script and no hand-written
/// stylesheet: the cards carry the framework's own `c_catalog*` classes.
///
/// The cards are built in plain Rust and handed to the grid through the
/// component's children. A `html!` loop needs static data, and the
/// catalog is runtime data, so the cards travel as children instead.
///
/// # Arguments
///
/// - `VirtualNode<DocsCatalogProps>` - The props node carrying the
///   artifacts and the route.
///
/// # Returns
///
/// - `VirtualNode` - The rendered catalog.
#[component]
pub(crate) fn docs_catalog(node: VirtualNode<DocsCatalogProps>) -> VirtualNode {
    let DocsCatalogProps {
        artifacts,
        route_signal,
    }: DocsCatalogProps = node.try_get_props().unwrap_or_default();
    // The locale is resolved once, before the cards are built. Reading
    // the signal inside the loop would make every card register its own
    // dependency on the route.
    let locale: String = route_signal.get();
    let files_label: String = locale_text(&locale, DOCS_KEY_CATALOG_FILES);
    let size_label: String = locale_text(&locale, DOCS_KEY_CATALOG_SIZE);
    let updated_label: String = locale_text(&locale, DOCS_KEY_CATALOG_UPDATED);
    let mut cards: Vec<VirtualNode> = Vec::with_capacity(artifacts.len());
    for artifact in artifacts.iter() {
        cards.push(docs_catalog_card(
            artifact.clone(),
            files_label.clone(),
            size_label.clone(),
            updated_label.clone(),
        ));
    }
    let children: VirtualNode = VirtualNode::Fragment(cards);
    html! {
        div {
            class: c_catalog()
            div {
                class: c_catalog_grid()
                children
            }
        }
    }
}

/// Renders one catalog card.
///
/// The card carries both the framework's `c_catalog_card` class and the
/// documentation site's own `docs-catalog-card` class: the framework owns
/// the styling, and the site's class is the contract the documentation
/// runtime validator asserts on.
///
/// # Arguments
///
/// - `DocsCatalogArtifact` - The artifact the card describes.
/// - `String` - The localized label of the file count field.
/// - `String` - The localized label of the total size field.
/// - `String` - The localized label of the update time field.
///
/// # Returns
///
/// - `VirtualNode` - The rendered card.
fn docs_catalog_card(
    artifact: DocsCatalogArtifact,
    files_label: String,
    size_label: String,
    updated_label: String,
) -> VirtualNode {
    let link: String = CATALOG_LINK_TEMPLATE.replace("{name}", &artifact.get_name());
    let name: String = artifact.get_name().clone();
    let digest: String = artifact.get_sha256().clone();
    let files: String = artifact.get_file_count().to_string();
    let size: String = format_bytes(artifact.get_total_bytes());
    let updated: String = format_timestamp(artifact.get_updated_at());
    html! {
        article {
            class: c_catalog_card()
            class: CATALOG_CARD_CLASS
            h3 {
                class: c_catalog_card_title()
                a {
                    class: c_catalog_card_title_link()
                    href: link.clone()
                    {
                        name.clone()
                    }
                }
            }
            dl {
                class: c_catalog_card_details()
                dt {
                    class: c_catalog_card_details_label()
                    {
                        files_label.clone()
                    }
                }
                dd {
                    class: c_catalog_card_details_value()
                    {
                        files.clone()
                    }
                }
                dt {
                    class: c_catalog_card_details_label()
                    {
                        size_label.clone()
                    }
                }
                dd {
                    class: c_catalog_card_details_value()
                    {
                        size.clone()
                    }
                }
                dt {
                    class: c_catalog_card_details_label()
                    {
                        updated_label.clone()
                    }
                }
                dd {
                    class: c_catalog_card_details_value()
                    time {
                        datetime: updated.clone()
                        {
                            updated.clone()
                        }
                    }
                }
            }
            code {
                class: c_catalog_card_digest()
                {
                    digest.clone()
                }
            }
        }
    }
}

/// Fetches the catalog and parses it.
///
/// # Returns
///
/// - `Result<Vec<DocsCatalogArtifact>, String>` - The stored projects, or
///   the reason they could not be read.
#[cfg(target_arch = "wasm32")]
pub(crate) async fn load_catalog() -> Result<Vec<DocsCatalogArtifact>, String> {
    let response: FetchResponse = fetch(CATALOG_URL)
        .await
        .map_err(|error: FetchError| format!("{ERROR_CATALOG_SHAPE}: {error}"))?;
    if !response.is_ok() {
        return Err(ERROR_CATALOG_SHAPE.to_string());
    }
    Ok(parse_catalog(&response.bytes())?.get_artifacts())
}

/// Formats a byte count the way the catalog shows it.
///
/// # Arguments
///
/// - `u64` - The byte count.
///
/// # Returns
///
/// - `String` - The formatted size.
fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    let mut value: f64 = bytes as f64;
    let mut unit: usize = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{value:.2} {}", UNITS[unit])
    }
}

/// Formats a Unix timestamp as an ISO-8601 instant.
///
/// # Arguments
///
/// - `i64` - Seconds since the Unix epoch.
///
/// # Returns
///
/// - `String` - The ISO-8601 instant, or the raw seconds when the instant
///   cannot be represented.
fn format_timestamp(seconds: i64) -> String {
    match chrono::DateTime::from_timestamp(seconds, 0) {
        Some(instant) => instant.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        None => seconds.to_string(),
    }
}

/// Parses the catalog API response.
///
/// The response is `{"success":true,"data":{"count":N,"artifacts":[...]}}`.
/// Every field the cards show is validated here, because a malformed
/// value would otherwise reach the page as a broken card: the name must
/// be a safe path segment, the digest must be a hex SHA-256, and the
/// counts must be non-negative integers.
///
/// # Arguments
///
/// - `&[u8]` - The raw response body.
///
/// # Returns
///
/// - `Result<DocsCatalog, String>` - The parsed catalog, or the reason
///   the response must not be rendered.
pub fn parse_catalog(bytes: &[u8]) -> Result<DocsCatalog, String> {
    let text: &str = from_utf8(bytes).map_err(|_| ERROR_CATALOG_ENCODING.to_string())?;
    let root: Value = serde_json::from_str(text)
        .map_err(|error: Error| format!("{ERROR_CATALOG_SHAPE}: {error}"))?;
    let data: &Value = root.get("data").unwrap_or(&root);
    let artifacts: &Value = data
        .get("artifacts")
        .ok_or_else(|| ERROR_CATALOG_SHAPE.to_string())?;
    let list: &Vec<Value> = artifacts
        .as_array()
        .ok_or_else(|| ERROR_CATALOG_SHAPE.to_string())?;
    let mut parsed: Vec<DocsCatalogArtifact> = Vec::with_capacity(list.len());
    let mut names: Vec<String> = Vec::with_capacity(list.len());
    for item in list {
        let name: String =
            text_field(item, "name").ok_or_else(|| ERROR_CATALOG_SHAPE.to_string())?;
        if !is_safe_artifact_name(&name) || names.contains(&name) {
            return Err(ERROR_CATALOG_SHAPE.to_string());
        }
        names.push(name.clone());
        let sha256: String =
            text_field(item, "sha256").ok_or_else(|| ERROR_CATALOG_SHAPE.to_string())?;
        if !is_sha256(&sha256) {
            return Err(ERROR_CATALOG_SHAPE.to_string());
        }
        parsed.push(DocsCatalogArtifact::new(
            name,
            count_field(item, "fileCount")?,
            count_field(item, "totalBytes")?,
            sha256,
            count_field(item, "updatedAt")? as i64,
        ));
    }
    if let Some(count) = data.get("count").and_then(Value::as_u64)
        && count as usize != parsed.len()
    {
        return Err(ERROR_CATALOG_SHAPE.to_string());
    }
    Ok(DocsCatalog::new(parsed))
}

/// Reads a string field from a catalog entry.
///
/// # Arguments
///
/// - `&Value` - The entry.
/// - `&str` - The field name.
///
/// # Returns
///
/// - `Option<String>` - The value, or `None` when the field is absent or
///   is not a string.
fn text_field(item: &Value, field: &str) -> Option<String> {
    item.get(field).and_then(Value::as_str).map(str::to_string)
}

/// Reads a non-negative integer field from a catalog entry.
///
/// # Arguments
///
/// - `&Value` - The entry.
/// - `&str` - The field name.
///
/// # Returns
///
/// - `Result<u64, String>` - The value, or the reason the entry must not
///   be rendered.
fn count_field(item: &Value, field: &str) -> Result<u64, String> {
    item.get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| ERROR_CATALOG_SHAPE.to_string())
}

/// Whether the name is a safe path segment for a published page.
///
/// # Arguments
///
/// - `&str` - The artifact name.
///
/// # Returns
///
/// - `bool` - `true` when the name can be used in a link.
fn is_safe_artifact_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 63
        && name
            .bytes()
            .all(|byte: u8| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !name.starts_with('-')
        && !name.ends_with('-')
}

/// Whether the digest is a hex SHA-256.
///
/// # Arguments
///
/// - `&str` - The digest.
///
/// # Returns
///
/// - `bool` - `true` when the digest is well formed.
fn is_sha256(digest: &str) -> bool {
    digest.len() == 64
        && digest
            .bytes()
            .all(|byte: u8| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}
