mod convention;
mod suite;

use super::*;

use std::{ffi::OsStr, path::Path};

use serde_json::{Value, from_str};

use proc_macro2::LineColumn;

use syn::{
    Attribute, Error, Expr, ExprField, ExprLit, File, Item, ItemConst, Lit, Macro, Meta,
    MetaNameValue, Token,
    parse::Parser,
    parse_file, parse_str, parse2,
    punctuated::Punctuated,
    spanned::Spanned,
    visit::{self, Visit},
};
