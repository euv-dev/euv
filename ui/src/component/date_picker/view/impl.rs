use super::*;

/// Inherent implementation of [`MonthCaption`].
impl MonthCaption {
    /// Returns the caption of a month, leaking it at most once per month.
    ///
    /// # Arguments
    ///
    /// - `i64` - The year.
    /// - `u32` - The month, `1` through `12`.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The caption, formatted as `YYYY-MM`.
    pub(crate) fn caption(&self, year: i64, month: u32) -> &'static str {
        let mut slot: RefMut<Option<(i64, u32, &'static str)>> = self.get_cached().borrow_mut();
        if let Some((cached_year, cached_month, caption)) = *slot
            && cached_year == year
            && cached_month == month
        {
            return caption;
        }
        let caption: &'static str = Box::leak(format!("{year:04}-{month:02}").into_boxed_str());
        *slot = Some((year, month, caption));
        caption
    }
}
