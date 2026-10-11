#!/usr/bin/env bash
# =============================================================================
# CreativeWorkzStudio LLC — OmniCode Desktop GUI Studio Launcher
# =============================================================================
set -e

SCRIPT_DIR="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BASE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

if command -v python3 >/dev/null 2>&1; then
    exec python3 "$BASE_DIR/gui/desktop_app.py" "$@"
else
    echo "Python 3 is required to launch the OmniCode Desktop GUI."
    exit 1
fi
