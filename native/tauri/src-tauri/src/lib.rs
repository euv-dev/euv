//! euv-app
//!
//! A Tauri-based application with offline cache and remote resource synchronization.

mod bridge;
mod cache;
mod resource;
mod session;

pub use cache::*;

pub(crate) use {bridge::*, resource::*, session::*};

pub(crate) use std::{
    borrow::Cow,
    collections::{HashSet, VecDeque},
    error, fmt,
    fs::Metadata,
    io,
    path::{Path, PathBuf},
    process::{exit, id},
    str::FromStr,
    sync::{Arc, OnceLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub(crate) use {
    http::{Request, StatusCode, response},
    lombok_macros::*,
    reqwest::{Client, Error, Response, Url},
    scraper::{Html, Selector, error::SelectorErrorKind},
    serde::{Serialize, Serializer},
    swc_common::{FileName, SourceFile, SourceMap, comments::SingleThreadedComments, sync::Lrc},
    swc_ecma_ast::{
        ArrowFunctionBody, Callee, Decl, EsVersion, Expr, Lit, MemberProp, ModuleDecl, ModuleItem,
        Program, Prop, PropOrSpread, Stmt, VarDeclOrExpr,
    },
    swc_ecma_parser::{Syntax, parse_file_as_program},
    tauri::{
        App, AppHandle, Builder, Emitter, Manager, RunEvent, State, UriSchemeContext,
        UriSchemeResponder, Webview, Wry,
        async_runtime::spawn,
        generate_context, generate_handler,
        webview::{PageLoadEvent, PageLoadPayload},
    },
    tokio::{
        fs::{
            File, create_dir_all, metadata, read, read_dir, read_to_string, remove_dir_all,
            remove_file, rename, write,
        },
        io::AsyncWriteExt,
        sync::{
            Mutex, MutexGuard,
            watch::{self, Receiver, Sender, error::RecvError},
        },
    },
};

#[macro_export]
macro_rules! euv_log {
    ($($arg:tt)*) => {{
        let msg: String = format!($($arg)*);
        ::log::info!("{}", msg);
        #[cfg(debug_assertions)]
        {
            if let Some(handle) = $crate::APP_HANDLE.get() {

                let _: Option<()> = handle.emit("euv://debug-log", msg).ok();
            }
        }
    }};
}
