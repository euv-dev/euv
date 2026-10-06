mod build;
mod fmt;
mod hmr;
mod inline;
mod server;

use euv_cli::*;

use std::{
    env::temp_dir,
    fs::{create_dir_all, remove_dir_all, write},
    io,
    path::PathBuf,
};

use clap::Parser;
