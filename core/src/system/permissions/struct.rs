use super::*;

/// Resolved platform-specific permission metadata and current authorization status.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct PermissionDescriptor {
    /// Requested unified permission category.
    #[get(type(copy))]
    pub(crate) permission: SystemPermission,
    /// Active target platform.
    #[get(type(copy))]
    pub(crate) platform: PlatformKind,
    /// Native OS manifest key, Info.plist key, WinRT capability, Portal interface, or Web permission name.
    pub(crate) native_key: String,
    /// User-facing privacy rationale string.
    pub(crate) usage_rationale: String,
    /// Current resolved authorization state.
    #[get(type(copy))]
    pub(crate) status: PermissionStatus,
}

/// Cross-platform system permission manager tracking OS authorization state and privacy rationales.
#[derive(Clone, Data, Debug, Default, New)]
pub struct PermissionManager {
    /// Cached or bridged permission statuses keyed by `SystemPermission`.
    #[new(skip)]
    pub(crate) statuses: HashMap<SystemPermission, PermissionStatus>,
    /// Declared user-facing usage rationales keyed by `SystemPermission`.
    #[new(skip)]
    pub(crate) rationales: HashMap<SystemPermission, String>,
}
