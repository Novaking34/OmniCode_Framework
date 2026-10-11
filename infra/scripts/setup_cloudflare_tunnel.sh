#!/usr/bin/env bash
# =============================================================================
# CreativeWorkzStudio LLC - OmniCode Platform
# Cloudflare Zero Trust Secure HTTPS Tunnel Connector
# =============================================================================
# Provides secure edge HTTPS routing to the local OmniCode Studio:
# 1. Quick Tunnel (Free, zero-config ephemeral HTTPS endpoint via trycloudflare.com):
#    Useful for local staging, development, and remote access.
# 2. Named Tunnel (Custom domain via Cloudflare Zero Trust Tunnel Token):
#    Production 24/7 tunnel mapped to your custom domain.
#
# Usage:
#   ./scripts/setup_cloudflare_tunnel.sh            # Auto-detects token from .env or starts quick
#   ./scripts/setup_cloudflare_tunnel.sh --quick    # Starts instant ephemeral quick tunnel
#   ./scripts/setup_cloudflare_tunnel.sh --token <TOKEN> # Starts named tunnel with specified token
#   ./scripts/setup_cloudflare_tunnel.sh --port <PORT>   # Specify local port (default: 8080)
# =============================================================================

set -e

# Visual formatting constants
BOLD="\033[1m"
GREEN="\033[0;32m"
BLUE="\033[0;34m"
CYAN="\033[0;36m"
YELLOW="\033[1;33m"
RED="\033[0;31m"
NC="\033[0m"

# Canonical directory resolution
SCRIPT_DIR="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ -f "$SCRIPT_DIR/../../Cargo.toml" ]; then
    BASE_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
else
    BASE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
fi
ENV_FILE="$BASE_DIR/.env"

PORT="8080"
MODE="auto"
CLI_TOKEN=""

echo -e "${BOLD}${CYAN}=====================================================================${NC}"
echo -e "${BOLD}${CYAN}  🌐 OmniCode Cloudflare Tunnel Connector                            ${NC}"
echo -e "${BOLD}${CYAN}  CreativeWorkzStudio LLC — Edge Security & Gateway                 ${NC}"
echo -e "${BOLD}${CYAN}=====================================================================${NC}"

# Ensure cloudflared is installed
ensure_cloudflared() {
    if command -v cloudflared >/dev/null 2>&1; then
        echo -e "${GREEN}[+] cloudflared binary verified: $(cloudflared --version | head -n 1)${NC}"
        return 0
    fi

    echo -e "${BLUE}[*] cloudflared not found in PATH. Checking system architecture...${NC}"
    ARCH="$(uname -m)"
    case "$ARCH" in
        x86_64)
            CLOUDFLARED_URL="https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64"
            ;;
        aarch64|arm64)
            CLOUDFLARED_URL="https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-arm64"
            ;;
        armv7l)
            CLOUDFLARED_URL="https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-arm"
            ;;
        *)
            echo -e "${RED}[-] Unsupported architecture: $ARCH. Please install cloudflared manually.${NC}"
            exit 1
            ;;
    esac

    echo -e "${BLUE}[*] Downloading cloudflared binary for $ARCH...${NC}"
    TMP_BIN="/tmp/cloudflared"
    curl -fsSL "$CLOUDFLARED_URL" -o "$TMP_BIN"
    chmod +x "$TMP_BIN"

    if [ "$EUID" -eq 0 ]; then
        mv "$TMP_BIN" /usr/local/bin/cloudflared
    elif sudo -n true 2>/dev/null; then
        sudo mv "$TMP_BIN" /usr/local/bin/cloudflared
    else
        mkdir -p "$BASE_DIR/bin"
        mv "$TMP_BIN" "$BASE_DIR/bin/cloudflared"
        export PATH="$BASE_DIR/bin:$PATH"
    fi

    echo -e "${GREEN}[+] cloudflared successfully installed.${NC}"
}

# Parse CLI arguments
while [ $# -gt 0 ]; do
    case "$1" in
        --quick)
            MODE="quick"
            shift
            ;;
        --token)
            MODE="token"
            CLI_TOKEN="$2"
            shift 2
            ;;
        --port)
            PORT="$2"
            shift 2
            ;;
        *)
            shift
            ;;
    esac
done

ensure_cloudflared

# Resolve Token from .env if auto
ENV_TOKEN=""
if [ -f "$ENV_FILE" ]; then
    ENV_TOKEN=$(grep -E '^CLOUDFLARE_TUNNEL_TOKEN=' "$ENV_FILE" 2>/dev/null | cut -d '=' -f 2- | tr -d '"' | tr -d "'" || true)
fi

# Fallback: check yugioh-server .env if available
if [ -z "$ENV_TOKEN" ] && [ -f "/home/professorseanex/yugioh-server/.env" ]; then
    ENV_TOKEN=$(grep -E '^CLOUDFLARE_TUNNEL_TOKEN=' "/home/professorseanex/yugioh-server/.env" 2>/dev/null | cut -d '=' -f 2- | tr -d '"' | tr -d "'" || true)
fi

if [ "$MODE" = "token" ] && [ -n "$CLI_TOKEN" ]; then
    ACTIVE_TOKEN="$CLI_TOKEN"
elif [ "$MODE" = "auto" ] && [ -n "$ENV_TOKEN" ] && [ "$ENV_TOKEN" != "your_cloudflare_tunnel_token_here" ]; then
    ACTIVE_TOKEN="$ENV_TOKEN"
    MODE="token"
else
    MODE="quick"
fi

# Execution
if [ "$MODE" = "token" ]; then
    echo -e "${BLUE}[*] Starting Cloudflare Named Tunnel using configured token...${NC}"
    echo -e "    Local Target  : http://localhost:$PORT (OmniCode Studio & API)"
    echo -e "    Token         : ${ACTIVE_TOKEN:0:15}... (Truncated for security)"
    echo ""
    echo -e "${YELLOW}[!] Tunnel running in foreground. Press Ctrl+C to terminate.${NC}"
    echo ""
    exec cloudflared tunnel --no-autoupdate run --token "$ACTIVE_TOKEN"
else
    echo -e "${BLUE}[*] Starting Cloudflare Quick Tunnel (Free, zero-config HTTPS edge)...${NC}"
    echo -e "    Local Target : http://localhost:$PORT"
    echo ""

    LOG_FILE="/tmp/omnicode_cloudflared_quick.log"
    rm -f "$LOG_FILE"

    cloudflared tunnel --url "http://localhost:$PORT" > "$LOG_FILE" 2>&1 &
    CF_PID=$!

    cleanup() {
        echo ""
        echo -e "${YELLOW}[*] Shutting down Cloudflare tunnel (PID: $CF_PID)...${NC}"
        kill "$CF_PID" 2>/dev/null || true
        exit 0
    }
    trap cleanup SIGINT SIGTERM EXIT

    echo -e "${BLUE}[*] Provisioning edge tunnel and acquiring public HTTPS URL...${NC}"
    URL=""
    for i in {1..30}; do
        if [ -f "$LOG_FILE" ]; then
            URL=$(grep -o 'https://[-a-zA-Z0-9\.]*\.trycloudflare\.com' "$LOG_FILE" | head -n 1 || true)
            if [ -n "$URL" ]; then
                break
            fi
        fi
        sleep 1
    done

    if [ -z "$URL" ]; then
        echo -e "${RED}[-] Timeout: Failed to obtain public tunnel URL within 30 seconds.${NC}"
        echo -e "    Log output:"
        cat "$LOG_FILE"
        exit 1
    fi

    echo ""
    echo -e "${BOLD}${GREEN}=====================================================================${NC}"
    echo -e "${BOLD}${GREEN}  🎉 OMNICODE CLOUDFLARE HTTPS TUNNEL IS LIVE!                       ${NC}"
    echo -e "${BOLD}${GREEN}=====================================================================${NC}"
    echo -e "  Public HTTPS URL:  ${BOLD}${CYAN}${URL}${NC}"
    echo -e "  Local Destination: http://localhost:$PORT"
    echo -e "  Organization:      CreativeWorkzStudio LLC"
    echo ""
    echo -e "${YELLOW}[!] Keep this terminal open. Tunnel will close when stopped.${NC}"
    echo -e "    Press Ctrl+C to terminate."
    wait "$CF_PID"
fi
