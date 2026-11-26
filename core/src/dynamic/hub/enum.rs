use super::*;

/// Serializable native UI node primitive used in component-level and page-level dynamic updates.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DynamicNodeKind {
    /// Vertical flexbox container.
    #[default]
    Column,
    /// Horizontal flexbox container.
    Row,
    /// Uniform-column grid container.
    Grid,
    /// Generic box container.
    View,
    /// Styled elevated surface card container.
    Card,
    /// Native text run node.
    Text,
    /// Interactive clickable button dispatching a `DynamicActionEvent`.
    Button,
    /// Compact pill status badge.
    Badge,
    /// Horizontal hairline separator.
    Divider,
}

/// Granularity scope of a dynamic update operation.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DynamicUpdateScope {
    /// Surgical update scoped to a single named component slot.
    #[default]
    Component,
    /// Page-level update scoped to a route (override existing or inject new route).
    Page,
    /// Atomic multi-target bundle updating components and pages together.
    Bundle,
}

/// Validation and runtime errors returned by `DynamicUpdateHub` operations.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DynamicUpdateError {
    /// Target component ID or page route was empty or whitespace-only.
    EmptyTargetId,
    /// Supplied checksum did not match the deterministic FNV-1a digest of the update payload.
    ChecksumMismatch {
        /// Target component ID or route path.
        target: String,
        /// Checksum declared by the incoming manifest.
        expected: u64,
        /// Deterministically computed checksum of the payload.
        actual: u64,
    },
    /// Incoming version was lower than or equal to the active version without `allow_downgrade`.
    VersionRegression {
        /// Target component ID or route path.
        target: String,
        /// Currently active version number.
        current: u64,
        /// Rejected incoming version number.
        incoming: u64,
    },
    /// Remote JSON manifest failed deserialization.
    InvalidJson(String),
    /// Rollback requested on a component or page with no applied dynamic revision.
    NoHistoryToRollback(String),
}

/// Internal representation of a mounted dynamic update payload (SDUI descriptor or native Rust closure).
#[derive(Clone, CustomDebug)]
pub(crate) enum DynamicPayload {
    /// Declarative cross-platform server-driven UI tree specification.
    Spec(DynamicNodeSpec),
    /// Hot-swapped native Rust builder closure.
    Native(#[debug(skip)] DynamicBuilderFn),
}
