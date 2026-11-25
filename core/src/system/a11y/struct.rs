use super::*;

/// Declarative accessibility metadata attached to any retained `SceneNode`.
#[derive(Clone, Data, Debug, Default, New, PartialEq)]
pub struct A11yProps {
    /// Semantic assistive role.
    #[get(type(copy))]
    pub(crate) role: A11yRole,
    /// Primary accessible name spoken by screen readers.
    pub(crate) label: String,
    /// Supplemental usage hint or description.
    #[new(skip)]
    pub(crate) hint: String,
    /// Current human-readable value (e.g. input text or slider percentage).
    #[new(skip)]
    pub(crate) value: String,
    /// Checked or toggled state for checkboxes, switches, and toggle buttons.
    #[new(skip)]
    #[get(skip)]
    pub(crate) checked: Option<bool>,
    /// Whether the control is currently disabled.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) disabled: bool,
    /// Whether the subtree is hidden from assistive technology.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) hidden: bool,
    /// Live region announcement policy.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) live_region: A11yLiveRegion,
    /// Heading level (`1..=6`) when `role == A11yRole::Heading`.
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) heading_level: u8,
}

/// Compiled snapshot node in the cross-platform `A11yTree`.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct A11yNode {
    /// Corresponding scene node identifier.
    #[get(type(copy))]
    pub(crate) id: NodeId,
    /// Resolved semantic accessibility properties.
    pub(crate) props: A11yProps,
    /// Laid-out screen bounds in logical coordinates.
    #[get(type(copy))]
    pub(crate) bounds: Rect,
    /// Whether this node currently holds keyboard focus.
    #[get(type(copy))]
    pub(crate) focused: bool,
    /// Ordered accessible child node handles.
    pub(crate) children: Vec<NodeId>,
}

/// Compiled semantic accessibility tree bridging `SceneTree` to OS screen readers.
#[derive(Clone, Data, Debug, Default, New, PartialEq)]
pub struct A11yTree {
    /// Root accessible node handle.
    #[get(type(copy))]
    pub(crate) root: NodeId,
    /// Flat preorder list of exposed accessible nodes.
    pub(crate) nodes: Vec<A11yNode>,
    /// Currently focused accessible node handle, if any.
    #[get(skip)]
    pub(crate) focused_id: Option<NodeId>,
}

/// Live accessibility announcement queued for VoiceOver, TalkBack, Narrator, Orca, or ARIA live regions.
#[derive(Clone, Data, Debug, Eq, New, PartialEq)]
pub struct A11yAnnouncement {
    /// Spoken message text.
    pub(crate) message: String,
    /// Announcement priority level.
    #[get(type(copy))]
    pub(crate) politeness: A11yLiveRegion,
}
