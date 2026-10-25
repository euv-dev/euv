use super::*;

#[test]
fn month_caption_formats_a_zero_padded_year_and_month() {
    let caption: MonthCaption = MonthCaption::default();
    assert_eq!(caption.caption(2024, 3), "2024-03");
    assert_eq!(caption.caption(2024, 12), "2024-12");
    assert_eq!(caption.caption(999, 1), "0999-01");
}

#[test]
fn month_caption_reuses_the_leaked_string_for_the_same_month() {
    let caption: MonthCaption = MonthCaption::default();
    let first: &'static str = caption.caption(2024, 3);
    let second: &'static str = caption.caption(2024, 3);
    assert_eq!(first, second);
    assert_eq!(first.as_ptr(), second.as_ptr());
    let other: &'static str = caption.caption(2024, 4);
    assert_eq!(other, "2024-04");
    assert_ne!(first.as_ptr(), other.as_ptr());
}
