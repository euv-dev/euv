#!/usr/bin/env python3
"""Exercise production release code without building Tauri or website artifacts."""
import os
from pathlib import Path
import subprocess
import tempfile

root = Path(__file__).resolve().parents[2] / "src-tauri"
with tempfile.TemporaryDirectory(prefix="euv-release-tests-", dir=os.environ["TMPDIR"]) as temporary:
    project = Path(temporary)
    (project / "src").mkdir()
    (project / "Cargo.toml").write_text('''[package]
name = "euv-release-tests"
version = "0.0.0"
edition = "2024"
[workspace]
[dependencies]
reqwest = { version = "=0.12.28", default-features = false, features = ["rustls-tls-webpki-roots"] }
serde = { version = "=1.0.228", features = ["derive"] }
serde_json = "=1.0.150"
sha2 = "=0.10.9"
tokio = { version = "=1.52.3", features = ["fs", "rt", "net", "sync", "time", "io-util"] }
''')
    (project / "src/lib.rs").write_text(f'''use std::{{collections::HashSet, path::{{Path, PathBuf}}}};
use serde::Deserialize;
use std::fmt;
#[cfg(test)] use serde_json::{{Value, json}};
use sha2::{{Digest, Sha256}};
use reqwest::{{Client, Response, Url}};
use tokio::fs::{{read, write, create_dir_all}};
#[path = "{root / 'src/release/mod.rs'}"] mod release;
use release::*;
#[cfg(test)] use std::{{env, fs, process::id, net::SocketAddr}};
#[cfg(test)] use tokio::{{net::TcpListener, task::JoinHandle, io::{{AsyncReadExt, AsyncWriteExt}}, runtime::{{Runtime, Builder}}}};
#[cfg(test)]
#[path = "{root / 'tests/release_regression/mod.rs'}"] mod release_regression;
''')
    environment = dict(os.environ)
    environment.setdefault("CARGO_TARGET_DIR", str(Path(os.environ["TMPDIR"]) / "euv-release-test-target"))
    subprocess.run(["cargo", "test", "--lib", "--offline", "--manifest-path", str(project / "Cargo.toml")], env=environment, check=True)
