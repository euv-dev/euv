mod r#const;
mod r#fn;

pub use r#fn::*;

pub(crate) use {
    r#const::*,
    r#fn::{
        deduplicate_slug, heading_level_num, load_doc_site_excluding, section_sort_rank,
        should_skip_dir, slugify,
    },
};

use super::*;
