use super::*;

/// Native OS role mapping for `A11yRole` across all 7 platforms.
impl A11yRole {
    /// Resolves the canonical native accessibility role or trait identifier for `platform`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Semantic accessibility role.
    /// - `PlatformKind` - Target execution platform.
    ///
    /// # Returns
    ///
    /// - `&'static str` - Native OS role identifier (NSAccessibility, UIAccessibility, UIA, AT-SPI2, Android, or ARIA).
    #[must_use]
    pub const fn native_role_name(&self, platform: PlatformKind) -> &'static str {
        match platform {
            PlatformKind::MacOs => match *self {
                Self::Window => "NSAccessibilityWindowRole",
                Self::Region | Self::Navigation | Self::Group => ROLE_MAC_GROUP,
                Self::Heading | Self::Text => ROLE_MAC_STATIC_TEXT,
                Self::Button => "NSAccessibilityButtonRole",
                Self::TextInput => "NSAccessibilityTextFieldRole",
                Self::Checkbox | Self::Switch => "NSAccessibilityCheckBoxRole",
                Self::Slider => "NSAccessibilitySliderRole",
                Self::Tab => "NSAccessibilityRadioButtonRole",
                Self::TabList => "NSAccessibilityTabGroupRole",
                Self::List => "NSAccessibilityListRole",
                Self::ListItem => "NSAccessibilityRowRole",
                Self::Dialog | Self::Alert => "NSAccessibilitySheetRole",
                Self::ProgressBar => "NSAccessibilityProgressIndicatorRole",
                Self::Image | Self::Canvas => "NSAccessibilityImageRole",
                Self::ScrollView => "NSAccessibilityScrollAreaRole",
                Self::Link => "NSAccessibilityLinkRole",
                Self::Menu => "NSAccessibilityMenuRole",
                Self::MenuItem => "NSAccessibilityMenuItemRole",
            },
            PlatformKind::Ios | PlatformKind::IpadOs => match *self {
                Self::Button | Self::Tab | Self::MenuItem => "UIAccessibilityTraitButton",
                Self::Heading => "UIAccessibilityTraitHeader",
                Self::Link => "UIAccessibilityTraitLink",
                Self::Image | Self::Canvas => "UIAccessibilityTraitImage",
                Self::Slider | Self::ProgressBar => "UIAccessibilityTraitAdjustable",
                Self::Text | Self::Alert => "UIAccessibilityTraitStaticText",
                _ => "UIAccessibilityTraitNone",
            },
            PlatformKind::Windows => match *self {
                Self::Window | Self::Dialog => "UIA_WindowControlTypeId",
                Self::Button => "UIA_ButtonControlTypeId",
                Self::TextInput => "UIA_EditControlTypeId",
                Self::Checkbox | Self::Switch => "UIA_CheckBoxControlTypeId",
                Self::Slider => "UIA_SliderControlTypeId",
                Self::Tab => "UIA_TabItemControlTypeId",
                Self::TabList => "UIA_TabControlTypeId",
                Self::List => "UIA_ListControlTypeId",
                Self::ListItem => "UIA_ListItemControlTypeId",
                Self::ProgressBar => "UIA_ProgressBarControlTypeId",
                Self::Image | Self::Canvas => "UIA_ImageControlTypeId",
                Self::Link => "UIA_HyperlinkControlTypeId",
                Self::Menu => "UIA_MenuControlTypeId",
                Self::MenuItem => "UIA_MenuItemControlTypeId",
                Self::Text | Self::Heading | Self::Alert => "UIA_TextControlTypeId",
                Self::ScrollView => "UIA_PaneControlTypeId",
                Self::Group | Self::Region | Self::Navigation => "UIA_GroupControlTypeId",
            },
            PlatformKind::Linux => match *self {
                Self::Window => "ATK_ROLE_FRAME",
                Self::Dialog => "ATK_ROLE_DIALOG",
                Self::Alert => "ATK_ROLE_ALERT",
                Self::Button => "ATK_ROLE_PUSH_BUTTON",
                Self::TextInput => "ATK_ROLE_ENTRY",
                Self::Checkbox => "ATK_ROLE_CHECK_BOX",
                Self::Switch => "ATK_ROLE_TOGGLE_BUTTON",
                Self::Slider => "ATK_ROLE_SLIDER",
                Self::Tab => "ATK_ROLE_PAGE_TAB",
                Self::TabList => "ATK_ROLE_PAGE_TAB_LIST",
                Self::List => "ATK_ROLE_LIST",
                Self::ListItem => "ATK_ROLE_LIST_ITEM",
                Self::ProgressBar => "ATK_ROLE_PROGRESS_BAR",
                Self::Image | Self::Canvas => "ATK_ROLE_IMAGE",
                Self::ScrollView => "ATK_ROLE_SCROLL_PANE",
                Self::Link => "ATK_ROLE_LINK",
                Self::Menu => "ATK_ROLE_MENU",
                Self::MenuItem => "ATK_ROLE_MENU_ITEM",
                Self::Heading => "ATK_ROLE_HEADING",
                Self::Text => "ATK_ROLE_LABEL",
                Self::Group | Self::Region | Self::Navigation => "ATK_ROLE_PANEL",
            },
            PlatformKind::Android => match *self {
                Self::Button | Self::MenuItem => "android.widget.Button",
                Self::TextInput => "android.widget.EditText",
                Self::Checkbox => "android.widget.CheckBox",
                Self::Switch => "android.widget.Switch",
                Self::Slider => "android.widget.SeekBar",
                Self::ProgressBar => "android.widget.ProgressBar",
                Self::Image | Self::Canvas => "android.widget.ImageView",
                Self::ScrollView => "android.widget.ScrollView",
                Self::List => "android.widget.ListView",
                Self::Text | Self::Heading | Self::Alert | Self::Link => "android.widget.TextView",
                _ => "android.view.View",
            },
            PlatformKind::Web | PlatformKind::Headless => match *self {
                Self::Window => "application",
                Self::Region | Self::ScrollView => ROLE_ARIA_REGION,
                Self::Navigation => "navigation",
                Self::Heading => "heading",
                Self::Text => "paragraph",
                Self::Button => "button",
                Self::TextInput => "textbox",
                Self::Checkbox => "checkbox",
                Self::Switch => "switch",
                Self::Slider => "slider",
                Self::Tab => "tab",
                Self::TabList => "tablist",
                Self::List => "list",
                Self::ListItem => "listitem",
                Self::Dialog => "dialog",
                Self::Alert => "alert",
                Self::ProgressBar => "progressbar",
                Self::Image | Self::Canvas => "img",
                Self::Link => "link",
                Self::Menu => "menu",
                Self::MenuItem => "menuitem",
                Self::Group => "group",
            },
        }
    }
}

/// Optional accessor helpers for `A11yProps`.
impl A11yProps {
    /// Returns the optional checked state (`Some(true)`, `Some(false)`, or `None`).
    ///
    /// # Arguments
    ///
    /// - `&Self` - Accessibility properties reference.
    ///
    /// # Returns
    ///
    /// - `Option<bool>` - Checked state if applicable.
    #[must_use]
    pub fn checked_state(&self) -> Option<bool> {
        let Self { checked, .. } = self;
        *checked
    }
}

/// Tree compilation and lookup methods for `A11yTree`.
impl A11yTree {
    /// Compiles an `A11yTree` snapshot directly from a laid-out `SceneTree`.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained UI scene tree.
    ///
    /// # Returns
    ///
    /// - `Self` - Compiled accessibility tree ready for native OS bridges.
    #[must_use]
    pub fn from_scene(tree: &SceneTree) -> Self {
        let root: NodeId = tree.get_root();
        let focused_id: Option<NodeId> = tree.get_focus().focused_node();
        let mut nodes: Vec<A11yNode> = Vec::new();
        Self::collect_node(tree, root, focused_id, &mut nodes);
        Self::new(root, nodes, focused_id)
    }

    /// Recursively collects visible, non-hidden nodes into the flat accessibility list.
    ///
    /// # Arguments
    ///
    /// - `&SceneTree` - Retained UI scene tree.
    /// - `NodeId` - Current node handle.
    /// - `Option<NodeId>` - Active focused node identifier.
    /// - `&mut Vec<A11yNode>` - Output accessible node vector.
    fn collect_node(
        tree: &SceneTree,
        node_id: NodeId,
        focused_id: Option<NodeId>,
        out: &mut Vec<A11yNode>,
    ) {
        let Some(node) = tree.get_node(node_id) else {
            return;
        };
        if node.get_style().get_layout().get_display() == Display::None {
            return;
        }
        let props: A11yProps = node.a11y_props().unwrap_or_else(|| {
            let mut inferred: A11yProps = match node.get_kind() {
                NodeKind::Text(t) => A11yProps::new(A11yRole::Text, t.clone()),
                NodeKind::Input(state) => {
                    let mut p: A11yProps =
                        A11yProps::new(A11yRole::TextInput, state.get_placeholder().clone());
                    p.set_value(state.get_value().clone())
                        .set_disabled(state.get_disabled());
                    p
                }
                NodeKind::Image(_) => A11yProps::new(A11yRole::Image, String::new()),
                NodeKind::Canvas => A11yProps::new(A11yRole::Canvas, String::new()),
                NodeKind::ScrollView => A11yProps::new(A11yRole::ScrollView, String::new()),
                NodeKind::Portal => A11yProps::new(A11yRole::Dialog, String::new()),
                NodeKind::View | NodeKind::Fragment => {
                    let role: A11yRole = if node_id == tree.get_root() {
                        A11yRole::Window
                    } else if node
                        .get_handlers()
                        .handler_for(&UiEventKind::Click(Point::default()))
                        .is_some()
                    {
                        A11yRole::Button
                    } else {
                        A11yRole::Group
                    };
                    A11yProps::new(role, String::new())
                }
            };
            if node.get_interaction().get_focused() {
                inferred.set_live_region(A11yLiveRegion::Polite);
            }
            inferred
        });
        if props.get_hidden() {
            return;
        }
        let visible_children: Vec<NodeId> = node
            .get_children()
            .iter()
            .copied()
            .filter(|cid: &NodeId| {
                tree.get_node(*cid).is_some_and(|c: &SceneNode| {
                    c.get_style().get_layout().get_display() != Display::None
                        && !c.a11y_props().is_some_and(|p: A11yProps| p.get_hidden())
                })
            })
            .collect();
        out.push(A11yNode::new(
            node_id,
            props,
            node.get_layout_box().get_bounds(),
            focused_id == Some(node_id),
            visible_children.clone(),
        ));
        for child_id in visible_children.into_iter() {
            Self::collect_node(tree, child_id, focused_id, out);
        }
    }

    /// Finds an `A11yNode` by its `NodeId`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Accessibility tree reference.
    /// - `NodeId` - Target scene node identifier.
    ///
    /// # Returns
    ///
    /// - `Option<&A11yNode>` - Matching accessibility node, if exposed.
    #[must_use]
    pub fn find(&self, id: NodeId) -> Option<&A11yNode> {
        self.get_nodes()
            .iter()
            .find(|node: &&A11yNode| node.get_id() == id)
    }

    /// Returns the currently focused node identifier, if any.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Accessibility tree reference.
    ///
    /// # Returns
    ///
    /// - `Option<NodeId>` - Focused accessible node identifier.
    #[must_use]
    pub fn focused_node(&self) -> Option<NodeId> {
        let Self { focused_id, .. } = self;
        *focused_id
    }

    /// Formats a compact ARIA-compatible semantic summary for Web synchronization and diagnostics.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Accessibility tree reference.
    /// - `PlatformKind` - Platform whose native role identifiers should be emitted.
    ///
    /// # Returns
    ///
    /// - `String` - Newline-delimited semantic role summary.
    #[must_use]
    pub fn format_summary(&self, platform: PlatformKind) -> String {
        let mut out: String = String::new();
        for node in self.get_nodes().iter() {
            let role: &str = node.get_props().get_role().native_role_name(platform);
            let label: &str = node.get_props().get_label().as_str();
            out.push_str(role);
            if !label.is_empty() {
                out.push_str(": ");
                out.push_str(label);
            }
            out.push('\n');
        }
        out
    }
}
