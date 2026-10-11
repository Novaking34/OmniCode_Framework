#!/usr/bin/env bash
# =============================================================================
# CreativeWorkzStudio LLC - OmniCode Platform
# Automated Native Terminal & Language OS Distribution Packager
# =============================================================================
# Compiles the optimized release binary and bundles a standalone native
# distribution archive with zero runtime dependencies.
#
# Output:
#   dist/omnicode-terminal-linux-x86_64.tar.gz
#   dist/omnicode-terminal-latest.tar.gz
# =============================================================================

set -e

SCRIPT_DIR="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ -f "$SCRIPT_DIR/../../Cargo.toml" ]; then
    BASE_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
else
    BASE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
fi
DIST_DIR="$BASE_DIR/infra/dist"
STAGING_DIR="/tmp/omnicode_dist_staging"

# ANSI styling
BOLD="\033[1m"
GREEN="\033[0;32m"
BLUE="\033[0;34m"
CYAN="\033[0;36m"
NC="\033[0m"

echo -e "${BOLD}${CYAN}=====================================================================${NC}"
echo -e "${BOLD}${CYAN}  📦 OmniCode Native Distribution Packager                           ${NC}"
echo -e "${BOLD}${CYAN}  CreativeWorkzStudio LLC — Standalone Native OS Shell               ${NC}"
echo -e "${BOLD}${CYAN}=====================================================================${NC}"

# 1. Compile release binary locally
echo -e "\n${BLUE}[1/4] Compiling optimized release binary with cargo...${NC}"
(cd "$BASE_DIR" && cargo build --release)

# 2. Prepare staging directory
echo -e "\n${BLUE}[2/4] Assembling standalone distribution files...${NC}"
rm -rf "$STAGING_DIR"
mkdir -p "$STAGING_DIR/bin"
mkdir -p "$STAGING_DIR/lib/core"
mkdir -p "$STAGING_DIR/examples"
mkdir -p "$STAGING_DIR/editors"
mkdir -p "$DIST_DIR"

# Copy binary
cp "$BASE_DIR/target/release/omnicode" "$STAGING_DIR/bin/omnicode"
chmod +x "$STAGING_DIR/bin/omnicode"

# Copy standard libraries and manifest
cp -r "$BASE_DIR/lib/core/"*.omni "$STAGING_DIR/lib/core/"
cp "$BASE_DIR/manifest.omni" "$STAGING_DIR/"
cp -r "$BASE_DIR/examples/"*.omni "$STAGING_DIR/examples/"

# Copy VS Code extension if available
if [ -d "$BASE_DIR/apps/editors/vscode" ]; then
    cp "$BASE_DIR/apps/editors/vscode/"*.vsix "$STAGING_DIR/editors/" 2>/dev/null || true
fi

# Create turnkey install script
cat << 'EOF' > "$STAGING_DIR/install.sh"
#!/usr/bin/env bash
# CreativeWorkzStudio LLC - OmniCode Standalone Installer
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Installing OmniCode Native Terminal..."

# Target install directory
if [ "$EUID" -eq 0 ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

cp "$DIR/bin/omnicode" "$INSTALL_DIR/omnicode"
chmod +x "$INSTALL_DIR/omnicode"

echo "✓ OmniCode installed successfully to $INSTALL_DIR/omnicode"
echo "Testing installation:"
"$INSTALL_DIR/omnicode" demo

echo ""
echo "🎉 OmniCode Native Shell is ready! Type 'omnicode' to enter interactive terminal."
EOF
chmod +x "$STAGING_DIR/install.sh"

# Create Distribution README
cat << 'EOF' > "$STAGING_DIR/README.md"
# OmniCode Native Terminal & Language Core
CreativeWorkzStudio LLC — Autonomous Neural-Symbolic Intelligence

## Quick Start
Run turnkey installer:
  ./install.sh

Or run binary directly:
  ./bin/omnicode

Launch interactive userspace shell:
  ./bin/omnicode terminal
EOF

# 3. Create compressed archives
echo -e "\n${BLUE}[3/4] Creating standalone distribution archives in dist/...${NC}"
ARCHIVE_NAME="omnicode-terminal-linux-x86_64.tar.gz"
(cd "/tmp" && tar -czf "$DIST_DIR/$ARCHIVE_NAME" -C "$STAGING_DIR" .)
cp "$DIST_DIR/$ARCHIVE_NAME" "$DIST_DIR/omnicode-terminal-latest.tar.gz"

# 4. Report
echo -e "\n${BLUE}[4/4] Package Verification:${NC}"
ls -lh "$DIST_DIR/$ARCHIVE_NAME"
sha256sum "$DIST_DIR/$ARCHIVE_NAME"

echo -e "\n${BOLD}${GREEN}=====================================================================${NC}"
echo -e "${BOLD}${GREEN}  ✅ Standalone Native Distribution Package Created Successfully!     ${NC}"
echo -e "${BOLD}${GREEN}  Output: $DIST_DIR/$ARCHIVE_NAME                                     ${NC}"
echo -e "${BOLD}${GREEN}=====================================================================${NC}"
