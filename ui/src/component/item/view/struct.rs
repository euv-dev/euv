use super::*;

/// Props for the `euv_item` component.
///
/// An item is one row of a list: a piece of media, a title and a
/// description, and the actions that belong to it.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvItemProps {
    /// The surface of the item.
    #[get(type(copy))]
    pub variant: EuvItemVariant,
    /// The density of the item.
    #[get(type(copy))]
    pub size: EuvItemSize,
}

/// Props for the `euv_item_media` component.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvItemMediaProps {
    /// The kind of media shown beside the item.
    #[get(type(copy))]
    pub variant: EuvItemMediaVariant,
}

/// Props for the `euv_item_title` component.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvItemTitleProps {
    /// The title text.
    #[get(type(copy))]
    pub text: &'static str,
}

/// Props for the `euv_item_description` component.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvItemDescriptionProps {
    /// The description text.
    #[get(type(copy))]
    pub text: &'static str,
}
