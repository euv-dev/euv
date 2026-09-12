use super::*;

/// Implementation of `ElementExt` for `Element`.
///
/// Provides convenient methods for manipulating DOM element attributes and properties,
/// handling special cases for form elements like inputs, textareas, and selects.
impl ElementExt for Element {
    /// Removes an attribute or property from the element.
    ///
    /// Handles special cases for form element properties (`value`, `checked`, `disabled`,
    /// `selected`, `readonly`, `multiple`) by setting them to their default values
    /// rather than removing the attribute.
    ///
    /// # Arguments
    ///
    /// - `&str` - The name of the attribute or property to remove.
    fn remove_attribute_or_property(&self, name: &str) {
        if name == ATTR_VALUE {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_value(EMPTY_STRING);
                return;
            }
            if let Some(textarea) = self.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_value(EMPTY_STRING);
                return;
            }
            if let Some(select) = self.dyn_ref::<HtmlSelectElement>() {
                select.set_value(EMPTY_STRING);
                return;
            }
        }
        if name == ATTR_CHECKED
            && let Some(input) = self.dyn_ref::<HtmlInputElement>()
        {
            input.set_checked(false);
            return;
        }
        if name == ATTR_DISABLED {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_disabled(false);
                return;
            }
            if let Some(button) = self.dyn_ref::<HtmlButtonElement>() {
                button.set_disabled(false);
                return;
            }
            if let Some(select) = self.dyn_ref::<HtmlSelectElement>() {
                select.set_disabled(false);
                return;
            }
            if let Some(textarea) = self.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_disabled(false);
                return;
            }
        }
        if name == ATTR_SELECTED
            && let Some(option) = self.dyn_ref::<HtmlOptionElement>()
        {
            option.set_selected(false);
            return;
        }
        if name == ATTR_READONLY {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_read_only(false);
                return;
            }
            if let Some(textarea) = self.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_read_only(false);
                return;
            }
        }
        if name == ATTR_MULTIPLE {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_multiple(false);
                return;
            }
            if let Some(select) = self.dyn_ref::<HtmlSelectElement>() {
                select.set_multiple(false);
                return;
            }
        }
        let _: Result<(), JsValue> = self.remove_attribute(name);
    }

    /// Sets an attribute or property on the element.
    ///
    /// Handles special cases for form element properties (`value`, `checked`, `disabled`,
    /// `selected`, `readonly`, `multiple`) by setting the corresponding DOM property
    /// rather than the HTML attribute, ensuring proper two-way binding behavior.
    ///
    /// # Arguments
    ///
    /// - `&str` - The name of the attribute or property to set.
    /// - `&str` - The value to assign.
    fn set_attribute_or_property(&self, name: &str, value: &str) {
        if name == ATTR_VALUE {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_value(value);
                return;
            }
            if let Some(textarea) = self.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_value(value);
                return;
            }
            if let Some(select) = self.dyn_ref::<HtmlSelectElement>() {
                select.set_value(value);
                return;
            }
        }
        if name == ATTR_CHECKED
            && let Some(input) = self.dyn_ref::<HtmlInputElement>()
        {
            input.set_checked(value == BOOL_TRUE);
            return;
        }
        if name == ATTR_DISABLED {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_disabled(value == BOOL_TRUE);
                return;
            }
            if let Some(button) = self.dyn_ref::<HtmlButtonElement>() {
                button.set_disabled(value == BOOL_TRUE);
                return;
            }
            if let Some(select) = self.dyn_ref::<HtmlSelectElement>() {
                select.set_disabled(value == BOOL_TRUE);
                return;
            }
            if let Some(textarea) = self.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_disabled(value == BOOL_TRUE);
                return;
            }
        }
        if name == ATTR_SELECTED
            && let Some(option) = self.dyn_ref::<HtmlOptionElement>()
        {
            option.set_selected(value == BOOL_TRUE);
            return;
        }
        if name == ATTR_READONLY {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_read_only(value == BOOL_TRUE);
                return;
            }
            if let Some(textarea) = self.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_read_only(value == BOOL_TRUE);
                return;
            }
        }
        if name == ATTR_MULTIPLE {
            if let Some(input) = self.dyn_ref::<HtmlInputElement>() {
                input.set_multiple(value == BOOL_TRUE);
                return;
            }
            if let Some(select) = self.dyn_ref::<HtmlSelectElement>() {
                select.set_multiple(value == BOOL_TRUE);
                return;
            }
        }
        let _: Result<(), JsValue> = self.set_attribute(name, value);
    }

    /// Returns the element's `data-euv-id`, assigning a fresh one if absent.
    ///
    /// # Returns
    ///
    /// - `usize` - The element's `data-euv-id` value.
    fn ensure_euv_id(&self) -> usize {
        match self.get_attribute(DATA_EUV_ID) {
            Some(id_str) => id_str.parse::<usize>().unwrap_or_else(|_| {
                let new_id: usize = NEXT_EUV_ID.fetch_add(1, Ordering::Relaxed);
                let _: Result<(), JsValue> = self.set_attribute(DATA_EUV_ID, &new_id.to_string());
                new_id
            }),
            None => {
                let new_id: usize = NEXT_EUV_ID.fetch_add(1, Ordering::Relaxed);
                let _: Result<(), JsValue> = self.set_attribute(DATA_EUV_ID, &new_id.to_string());
                new_id
            }
        }
    }
}
