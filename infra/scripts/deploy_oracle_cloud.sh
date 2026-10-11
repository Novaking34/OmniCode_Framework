#!/usr/bin/env bash
# =============================================================================
# CreativeWorkzStudio LLC - OmniCode Platform
# Oracle Cloud Ubuntu VM Turnkey Provisioning & Deployment Script
# =============================================================================
# Turnkey setup script for deploying OmniCode 24/7 on an Oracle Cloud Compute Instance:
# 1. Installs system build prerequisites, Rust/Cargo compiler, and git.
# 2. Configures OS firewall & iptables to allow Port 8080 (Web Studio & Cognitive API).
# 3. Builds the optimized OmniCode release binary (`cargo build --release`).
# 4. Installs and registers the systemd 24/7 service (`omnicode-web.service`).
# 5. Verifies service health and edge availability.
#
# Usage (run as root or with sudo on the Oracle Cloud VM):
#   sudo ./scripts/deploy_oracle_cloud.sh
# =============================================================================

set -e

# Visual styling
BOLD="\033[1m"
GREEN="\033[0;32m"
BLUE="\033[0;34m"
CYAN="\033[0;36m"
YELLOW="\033[1;33m"
RED="\033[0;31m"
NC="\033[0m"

echo -e "${BOLD}${CYAN}=====================================================================${NC}"
echo -e "${BOLD}${CYAN}  🌌 CreativeWorkzStudio LLC — OmniCode Platform                     ${NC}"
echo -e "${BOLD}${CYAN}  Oracle Cloud VM Turnkey Deployment & Systemd Provisioner           ${NC}"
echo -e "${BOLD}${CYAN}=====================================================================${NC}"

if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}[!] Error: This script must be run as root or via sudo.${NC}"
    echo -e "    Please run: sudo $0"
    exit 1
fi

REAL_USER="${SUDO_USER:-$USER}"
REAL_HOME="$(eval echo "~$REAL_USER")"
SCRIPT_DIR="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BASE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo -e "${BLUE}[*] Provisioning for host user: ${GREEN}$REAL_USER${NC}"
echo -e "${BLUE}[*] OmniCode base directory:    ${GREEN}$BASE_DIR${NC}"

# Detect Architecture
ARCH="$(uname -m)"
echo -e "${BLUE}[*] Target CPU architecture:    ${GREEN}$ARCH${NC}"

# -----------------------------------------------------------------------------
# STEP 1: System Packages and Build Essentials
# -----------------------------------------------------------------------------
echo ""
echo -e "${BOLD}${BLUE}[1/5] Updating packages and installing build prerequisites...${NC}"
export DEBIAN_FRONTEND=noninteractive
apt-get update -y
apt-get install -y \
    build-essential \
    curl \
    git \
    pkg-config \
    libssl-dev \
    iptables \
    iptables-persistent \
    netfilter-persistent \
    ca-certificates

# Ensure Rust & Cargo are installed for the target user
if ! sudo -u "$REAL_USER" command -v cargo >/dev/null 2>&1; then
    echo -e "${BLUE}[*] Rust compiler not found. Installing Rust toolchain via rustup...${NC}"
    sudo -u "$REAL_USER" curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sudo -u "$REAL_USER" sh -s -- -y
    echo -e "${GREEN}[+] Rust toolchain installed.${NC}"
else
    echo -e "${GREEN}[+] Rust toolchain already available: $(sudo -u "$REAL_USER" cargo --version)${NC}"
fi

# -----------------------------------------------------------------------------
# STEP 2: Configure Oracle Cloud Firewall & iptables
# -----------------------------------------------------------------------------
echo ""
echo -e "${BOLD}${BLUE}[2/5] Configuring OS Firewall & iptables for Port 8080...${NC}"
PORTS=(22 8080)

for PORT in "${PORTS[@]}"; do
    if ! iptables -C INPUT -p tcp --dport "$PORT" -j ACCEPT 2>/dev/null; then
        echo -e "${BLUE}[*] Opening TCP Port $PORT in iptables...${NC}"
        iptables -I INPUT 1 -p tcp --dport "$PORT" -j ACCEPT
    else
        echo -e "${GREEN}[+] TCP Port $PORT already allowed in iptables.${NC}"
    fi
done

# Persist firewall rules
netfilter-persistent save || true

# Check UFW if active
if command -v ufw >/dev/null 2>&1 && ufw status 2>/dev/null | grep -q "Status: active"; then
    echo -e "${BLUE}[*] UFW firewall active. Permitting port 8080...${NC}"
    ufw allow 22/tcp
    ufw allow 8080/tcp
    ufw reload
fi
echo -e "${GREEN}[+] OS firewall configured for OmniCode Web & Cognitive Studio (Port 8080).${NC}"

# -----------------------------------------------------------------------------
# STEP 3: Compile OmniCode Release Binary
# -----------------------------------------------------------------------------
echo ""
echo -e "${BOLD}${BLUE}[3/5] Compiling OmniCode release binary with Cargo...${NC}"
export PATH="$REAL_HOME/.cargo/bin:$PATH"
sudo -u "$REAL_USER" env PATH="$REAL_HOME/.cargo/bin:$PATH" cargo build --release --manifest-path "$BASE_DIR/Cargo.toml"
echo -e "${GREEN}[+] OmniCode release binary built at: $BASE_DIR/target/release/omnicode${NC}"

# -----------------------------------------------------------------------------
# STEP 4: Install and Enable Systemd Service
# -----------------------------------------------------------------------------
echo ""
echo -e "${BOLD}${BLUE}[4/5] Registering and enabling systemd 24/7 service...${NC}"
SERVICE_SRC="$BASE_DIR/systemd/omnicode-web.service"
SERVICE_DEST="/etc/systemd/system/omnicode-web.service"

sed -e "s|/home/ubuntu/OmniCode|$BASE_DIR|g" \
    -e "s|User=ubuntu|User=$REAL_USER|g" \
    "$SERVICE_SRC" > "$SERVICE_DEST"

systemctl daemon-reload
systemctl enable omnicode-web.service
systemctl restart omnicode-web.service
echo -e "${GREEN}[+] omnicode-web.service installed and restarted.${NC}"

# -----------------------------------------------------------------------------
# STEP 5: Verification and Endpoint Summary
# -----------------------------------------------------------------------------
echo ""
echo -e "${BOLD}${BLUE}[5/5] Verifying service health and endpoints...${NC}"
sleep 2

STATUS=$(systemctl is-active omnicode-web.service || echo "failed")
PUBLIC_IP="$(curl -s -4 https://ifconfig.me || curl -s -4 https://api.ipify.org || echo "ORACLE_VM_PUBLIC_IP")"

echo ""
echo -e "${BOLD}${GREEN}=====================================================================${NC}"
echo -e "${BOLD}${GREEN}  🎉 OMNICODE ORACLE CLOUD 24/7 DEPLOYMENT COMPLETE!                ${NC}"
echo -e "${BOLD}${GREEN}  CreativeWorkzStudio LLC                                           ${NC}"
echo -e "${BOLD}${GREEN}=====================================================================${NC}"
echo -e "Service Status:         ${BOLD}${GREEN}$STATUS${NC}"
echo -e "Public Server IP:       ${BOLD}${YELLOW}$PUBLIC_IP${NC}"
echo -e "Web Studio & API:       ${BOLD}${CYAN}http://$PUBLIC_IP:8080${NC}"
echo ""
echo -e "${BOLD}Management Commands:${NC}"
echo -e "  Check service status:  ${BLUE}sudo systemctl status omnicode-web${NC}"
echo -e "  Restart service:       ${BLUE}sudo systemctl restart omnicode-web${NC}"
echo -e "  View live logs:        ${BLUE}sudo journalctl -u omnicode-web -f${NC}"
echo ""
echo -e "${BOLD}Cloudflare Integration:${NC}"
echo -e "  To route edge traffic through Cloudflare, launch the tunnel connector:"
echo -e "  ${BLUE}$BASE_DIR/scripts/setup_cloudflare_tunnel.sh --quick${NC}"
echo -e "${BOLD}${GREEN}=====================================================================${NC}"
