#!/usr/bin/env bash
set -euo pipefail

# swift-rs 1.0.x passes iOS target flags through -Xswiftc/-Xcc. Swift 6.4's
# new default SwiftBuild backend appends macOS flags after them. Preserve the
# native SPM backend expected by swift-rs, while executing the real Swift tool.
REAL_SWIFT="${EUV_REAL_SWIFT:-/usr/bin/swift}"
if [ "${1:-}" = build ]; then
    shift
    exec "$REAL_SWIFT" build --build-system native "$@"
fi
exec "$REAL_SWIFT" "$@"
