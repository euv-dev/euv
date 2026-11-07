use super::*;

/// Resolves a bridge group name to the list of permission strings.
///
/// # Arguments
///
/// - `&str` - The group name (e.g., "camera", "microphone", "all").
///
/// # Returns
///
/// - `Result<Vec<String>, String>` - The list of permission strings, or an error if the group name is unknown.
#[tauri::command]
pub(crate) fn resolve_bridge_group_permissions(group: &str) -> Result<Vec<String>, String> {
    let bridge_group: BridgeGroup = group
        .parse::<BridgeGroup>()
        .map_err(|error: String| error)?;
    Ok(bridge_group
        .permissions()
        .iter()
        .map(|permission: &&str| permission.to_string())
        .collect())
}
