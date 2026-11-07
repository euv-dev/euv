mod code_block;
mod doc_page;
mod header;
mod home_page;
mod layout;
mod markdown;
mod not_found;
mod password_gate;
mod site_footer;

pub(crate) use {
    code_block::*, doc_page::*, header::*, home_page::*, layout::*, markdown::*, not_found::*,
    password_gate::*, site_footer::*,
};

use super::*;
