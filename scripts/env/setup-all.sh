#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=common.sh
source "${SCRIPT_DIR}/common.sh"

"${SCRIPT_DIR}/setup-rust.sh"
"${SCRIPT_DIR}/setup-web.sh"
"${SCRIPT_DIR}/setup-apple.sh"
"${SCRIPT_DIR}/setup-android.sh"
"${SCRIPT_DIR}/doctor.sh"
