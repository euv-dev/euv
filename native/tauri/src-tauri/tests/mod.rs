mod readiness;
#[path = "../src/release/mod.rs"]
mod release;
mod release_regression;
mod resource;
#[path = "../src/resource/const.rs"]
mod resource_extensions;
#[path = "../src/session/mod.rs"]
mod session;
#[path = "../src/snapshot/mod.rs"]
mod snapshot;
mod snapshot_regression;

use {
    lombok_macros::*,
    reqwest::{Client, Error, Response, Url},
    serde::Deserialize,
    serde_json::{Value, json},
    sha2::{Digest, Sha256},
    std::{
        collections::{HashSet, VecDeque},
        env, error, fmt, fs,
        fs::Metadata,
        io,
        iter::once,
        net::SocketAddr,
        path::{Path, PathBuf},
        process::id,
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
        time::{Duration, Instant},
    },
    tokio::{
        fs::{File, OpenOptions, create_dir_all, metadata, read, remove_file, rename, write},
        io::{AsyncReadExt, AsyncWriteExt},
        net::{TcpListener, TcpStream},
        runtime::{Builder, Runtime},
        sync::{
            Mutex, MutexGuard, oneshot,
            watch::{self, Receiver, Sender, error::RecvError},
        },
        task::{JoinHandle, JoinSet},
    },
};
use {release::*, resource_extensions::*, session::*, snapshot::*};
