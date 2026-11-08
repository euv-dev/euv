use super::*;

/// Props for the [`euv_code_block`] component.
///
/// The block carries the surface it is drawn on and its size. The header,
/// the content and the footer are passed as children.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCodeBlockProps {
    /// The surface the code block is drawn on.
    pub variant: EuvCodeBlockVariant,
    /// The size of the block.
    pub size: EuvCodeBlockSize,
}

/// Props for the [`euv_code_block_header`] component.
///
/// The header takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCodeBlockHeaderProps {}

/// Props for the [`euv_code_block_title`] component.
///
/// The title takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCodeBlockTitleProps {}

/// Props for the [`euv_code_block_actions`] component.
///
/// The actions take no settings of their own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCodeBlockActionsProps {}

/// Props for the [`euv_code_block_copy_button`] component.
///
/// The copy button carries the text it puts on the clipboard.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCodeBlockCopyButtonProps {
    /// The text the button puts on the clipboard.
    pub value: &'static str,
}

/// Props for the [`euv_code_block_content`] component.
///
/// The content carries the two settings the reference publishes: whether
/// the long lines wrap, and whether the lines are numbered.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCodeBlockContentProps {
    /// Whether a long line wraps instead of scrolling.
    pub wrap: bool,
    /// Whether the lines are numbered.
    pub line_numbers: bool,
    /// The number the first line is given.
    pub start_line: i32,
}

/// Props for the [`euv_code_block_line`] component.
///
/// The line carries the state of the line inside the change the snippet
/// shows.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCodeBlockLineProps {
    /// The state of the line.
    pub variant: EuvCodeBlockLineVariant,
}

/// Props for the [`euv_code_block_footer`] component.
///
/// The footer takes no settings of its own.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvCodeBlockFooterProps {}
