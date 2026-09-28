// euv-ui component registry.
//
// Each component lives in its own `<name>/` module directory. A component
// module has this shape:
//
//   <name>/mod.rs      — declares `mod view;` / `mod hook;` and re-exports
//   <name>/view/*.rs   — props struct, enums, and the `#[component]` fns
//   <name>/hook/*.rs   — optional state/logic helpers consumed by the view
//
// New components are added by adding a module here and listing it in the
// `pub use` block below, keeping this file alphabetical.

mod alert;
mod avatar;
mod badge;
mod breadcrumb;
mod browser;
mod button;
mod calendar;
mod camera;
mod card;
mod checkbox;
mod collapse;
mod debug;
mod divider;
mod doc_layout;
mod drawer;
mod dropdown;
mod feature_grid;
mod field;
mod header;
mod hero;
mod icon;
mod info;
mod input;
mod layout;
mod loading;
mod logo;
mod markdown;
mod modal;
mod nav;
mod navbar;
mod pagination;
mod panel;
mod popover;
mod progress;
mod radio;
mod rating;
mod result;
mod router;
mod sidebar;
mod skeleton;
mod slider;
mod space;
mod stat;
mod steps;
mod switch;
mod table;
mod tabs;
mod tag;
mod theme;
mod timeline;
mod toc;
mod tooltip;
mod touch;
mod upload;
mod vconsole;
mod virtual_list;

pub use {
    alert::*, avatar::*, badge::*, breadcrumb::*, browser::*, button::*, calendar::*, camera::*,
    card::*, checkbox::*, collapse::*, debug::*, divider::*, doc_layout::*, drawer::*, dropdown::*,
    feature_grid::*, field::*, header::*, hero::*, icon::*, info::*, input::*, layout::*,
    loading::*, logo::*, markdown::*, modal::*, nav::*, navbar::*, pagination::*, panel::*,
    popover::*, progress::*, radio::*, rating::*, result::*, router::*, sidebar::*, skeleton::*,
    slider::*, space::*, stat::*, steps::*, switch::*, table::*, tabs::*, tag::*, theme::*,
    timeline::*, toc::*, tooltip::*, touch::*, upload::*, vconsole::*, virtual_list::*,
};

use super::*;
