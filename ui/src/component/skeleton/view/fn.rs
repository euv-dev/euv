use super::*;

/// The fewest placeholder lines a skeleton block will render.
const SKELETON_LINES_MIN: usize = 1;

/// The most placeholder lines a skeleton block will render.
///
/// A loading placeholder is a stand-in for a short block of content; past
/// a handful of lines a skeleton stops reading as a placeholder and the
/// height is better owned by an explicit `euv_loading` indicator.
const SKELETON_LINES_MAX: usize = 8;

/// The width applied to skeleton lines when the `width` prop is unusable.
const SKELETON_WIDTH_DEFAULT: &str = "100%";

/// The border radius applied to skeleton lines when `rounded` is true.
///
/// The design system is square by default; rounding is opt-in per block so
/// a caller can match a circular avatar or a pill-shaped chip without the
/// rest of the component library losing its `border-radius: 0px` rule.
const SKELETON_RADIUS: &str = "2px";

/// Clamps a requested line count into the renderable `1..=8` range.
///
/// A `0` request still draws one line: an empty block reads as a layout
/// bug rather than as loading, and the caller who meant to hide it should
/// unmount the component instead of rendering nothing.
///
/// # Arguments
///
/// - `usize` - The requested number of placeholder lines.
///
/// # Returns
///
/// - `usize` - The line count clamped into the `1..=8` range.
pub fn skeleton_line_count(count: usize) -> usize {
    count.clamp(SKELETON_LINES_MIN, SKELETON_LINES_MAX)
}

/// A loading placeholder shown while real content is still being fetched.
///
/// Renders a `c_euv_skeleton` block holding one `c_euv_skeleton_line` per
/// requested line, each carrying an inline `width` (and `height` when one
/// is supplied). The pulse is not animated here: the shipped
/// `c_euv_skeleton_line` class already references the global
/// `euv-pulse` keyframe, so the shimmer runs off the shared animation
/// rather than a per-instance one. The `rounded` signal drives the
/// `border-radius` override through the same inline style.
///
/// # Arguments
///
/// - `VirtualNode<EuvSkeletonProps>` - The props node containing lines, width, height and rounded.
///
/// # Returns
///
/// - `VirtualNode` - The skeleton placeholder virtual DOM tree.
#[component]
pub fn euv_skeleton(node: VirtualNode<EuvSkeletonProps>) -> VirtualNode {
    let EuvSkeletonProps {
        lines,
        width,
        height,
        rounded,
    }: EuvSkeletonProps = node.try_get_props().unwrap_or_default();
    let mut block: Vec<VirtualNode> = Vec::with_capacity(skeleton_line_count(lines));
    for index in 0..skeleton_line_count(lines) {
        block.push(skeleton_line(index, width, height, rounded));
    }
    html! {
        div {
            class: c_euv_skeleton()
            block
        }
    }
}

/// Renders one placeholder line with its inline geometry.
///
/// The declarations are concatenated into a single style string rather
/// than stacked as repeated `style:` attributes: only the top-level
/// attributes of an element are read as attributes, so a declaration
/// nested inside a conditional child block would be dropped. `height` is
/// omitted entirely when the prop is empty, leaving the stylesheet default
/// in charge.
///
/// # Arguments
///
/// - `usize` - The zero-based position of this line, used as its key.
/// - `&'static str` - The requested width, e.g. `"60%"`.
/// - `&'static str` - The requested height, e.g. `"24px"`.
/// - `Signal<bool>` - Whether the line is drawn with rounded corners.
///
/// # Returns
///
/// - `VirtualNode` - The placeholder line virtual DOM tree.
fn skeleton_line(
    index: usize,
    width: &'static str,
    height: &'static str,
    rounded: Signal<bool>,
) -> VirtualNode {
    let mut style: String = String::with_capacity(64);
    let line_width: &str = skeleton_width(width);
    style.push_str("width: ");
    style.push_str(line_width);
    style.push(';');
    if !height.is_empty() {
        style.push_str(" height: ");
        style.push_str(height);
        style.push(';');
    }
    if rounded.get() {
        style.push_str(" border-radius: ");
        style.push_str(SKELETON_RADIUS);
        style.push(';');
    }
    let style_ref: &str = style.as_str();
    html! {
        div {
            key: index.to_string()
            class: c_euv_skeleton_line()
            style: style_ref
        }
    }
}

/// Normalises the `width` prop into a usable CSS length.
///
/// Only percentages are accepted: the skeleton is a placeholder for a
/// block of in-flow text, and an arbitrary length would let it overflow
/// its container. An empty or non-percentage value falls back to
/// `100%`.
///
/// # Arguments
///
/// - `&'static str` - The requested width, e.g. `"60%"`.
///
/// # Returns
///
/// - `&'static str` - The width to apply, ending in `%`.
fn skeleton_width(width: &'static str) -> &'static str {
    let is_percent: bool = match width.strip_suffix('%') {
        Some(digits) => !digits.is_empty() && digits.bytes().all(|byte: u8| byte.is_ascii_digit()),
        None => false,
    };
    if is_percent {
        width
    } else {
        SKELETON_WIDTH_DEFAULT
    }
}
