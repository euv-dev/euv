/// Cross-platform semantic accessibility role mapped to native OS screen readers and ARIA.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum A11yRole {
    /// Generic grouping container.
    #[default]
    Group,
    /// Top-level application window.
    Window,
    /// Landmark region.
    Region,
    /// Primary navigation landmark.
    Navigation,
    /// Section heading.
    Heading,
    /// Static readable text.
    Text,
    /// Interactive push button.
    Button,
    /// Editable text input field.
    TextInput,
    /// Two-state checkbox control.
    Checkbox,
    /// Two-state toggle switch control.
    Switch,
    /// Continuous numeric range slider.
    Slider,
    /// Individual tab trigger.
    Tab,
    /// Tab strip container.
    TabList,
    /// Ordered or virtualized list container.
    List,
    /// Single row inside a list.
    ListItem,
    /// Modal dialog or sheet overlay.
    Dialog,
    /// Live status alert banner.
    Alert,
    /// Progress indicator bar.
    ProgressBar,
    /// Bitmap or vector image.
    Image,
    /// Scrollable viewport container.
    ScrollView,
    /// Hyperlink or route navigation trigger.
    Link,
    /// Popup menu container.
    Menu,
    /// Actionable menu item.
    MenuItem,
    /// Custom graphics or game canvas.
    Canvas,
}

/// Screen reader live announcement politeness level.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum A11yLiveRegion {
    /// No automatic screen reader announcement.
    #[default]
    Off,
    /// Announced when the screen reader finishes current speech.
    Polite,
    /// Interrupts current speech immediately for urgent alerts.
    Assertive,
}

/// Semantic assistive-technology action invoked by VoiceOver, TalkBack, UIA, AT-SPI2, or ARIA.
#[derive(Clone, Debug, PartialEq)]
pub enum A11yAction {
    /// Activate or press the element.
    Click,
    /// Move keyboard and assistive focus to the element.
    Focus,
    /// Remove focus from the element.
    Blur,
    /// Replace editable text value directly.
    SetValue(String),
    /// Increment an adjustable control such as a slider or stepper.
    Increment,
    /// Decrement an adjustable control such as a slider or stepper.
    Decrement,
    /// Scroll a viewport by logical `(dx, dy)` deltas.
    ScrollBy(f32, f32),
    /// Dismiss an active modal, sheet, or popover overlay.
    Dismiss,
}
