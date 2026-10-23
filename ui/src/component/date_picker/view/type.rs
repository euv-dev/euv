use super::*;

/// The cached `(year, month, caption)` triple of a month.
///
/// The caption is a `&'static str`, so it has to be leaked to outlive
/// the render that produced it. Leaking a fresh string on every render
/// would pile up one allocation per signal change, so the caption is
/// leaked once per month and reused until the panel walks to another
/// one.
pub(crate) type CachedCaption = Rc<RefCell<Option<(i64, u32, &'static str)>>>;
