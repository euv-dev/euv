#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../env/common.sh
source "${SCRIPT_DIR}/../env/common.sh"
load_local_env

cd "${REPO_ROOT}"
log_info "1/5 Checking formatting & clippy..."
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings

log_info "2/5 Running full workspace unit & E2E tests..."
cargo test --workspace --all-features
cargo test --workspace --no-default-features
RUSTFLAGS="-D warnings" cargo test -p euv-automation --release

log_info "3/5 Verifying all 7 cross-platform target triples..."
for t in wasm32-unknown-unknown aarch64-apple-darwin aarch64-apple-ios aarch64-apple-ios-sim aarch64-linux-android x86_64-pc-windows-msvc x86_64-unknown-linux-gnu; do
  RUSTFLAGS="-D warnings" cargo check --workspace --target "${t}"
done

log_info "4/5 Proving Dev-Only AI Schema stripping in Release builds..."
cargo build -p euv-example --bin euv-example
cargo build --release -p euv-example --bin euv-example
DEV_MATCHES="$(strings target/debug/euv-example | grep -c "euv.ai.schema.v1" || true)"
REL_MATCHES="$(strings target/release/euv-example | grep -c "euv.ai.schema.v1" || true)"
log_info "Native AI schema matches: debug=${DEV_MATCHES}, release=${REL_MATCHES}"
[ "${DEV_MATCHES}" -ge 1 ] && [ "${REL_MATCHES}" -eq 0 ]

log_info "5/5 Running release performance benchmark suite..."
cargo run --release -p euv-performance
log_info "All E2E checks passed."
