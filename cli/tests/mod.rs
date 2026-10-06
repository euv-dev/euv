mod build;
mod fmt;
mod hmr;
mod inline;
mod server;

use euv_cli::*;

use std::{
    env::temp_dir,
    fs::remove_dir_all,
};
