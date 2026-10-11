#!/usr/bin/env bash
# =============================================================================
# CreativeWorkzStudio LLC - OmniCode Platform
# Oracle Cloud VM Automated Synchronization Script
# =============================================================================
# Synchronizes the local OmniCode repository to the Oracle Cloud VM:
# 1. Connects to Oracle Cloud VM (147.224.147.30 / Host 'oracle') via SSH.
# 2. Synchronizes repository source files to /home/ubuntu/OmniCode using rsync.
# 3. Compiles the optimized release binary with cargo on the remote host.
# 4. Restarts systemd service: omnicode-web.service.
# 5. Queries and reports remote service health.
#
# Usage:
#   ./scripts/sync_oracle_vm.sh [optional commit or sync note]
# =============================================================================

set -e

# ANSI Styling
GREEN="\033[0;32m"
BLUE="\033[0;34m"
CYAN="\033[0;36m"
YELLOW="\033[1;33m"
RED="\033[0;31m"
BOLD="\033[1m"
NC="\033[0m"

SCRIPT_DIR="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ -f "$SCRIPT_DIR/../../Cargo.toml" ]; then
    BASE_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
else
    BASE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
fi

# Target Configuration
ORACLE_HOST="${ORACLE_VM_HOST:-147.224.147.30}"
ORACLE_USER="${ORACLE_VM_USER:-ubuntu}"
SSH_KEY="${ORACLE_SSH_KEY:-$HOME/.ssh/oracle_key.pem}"
REMOTE_DIR="/home/ubuntu/OmniCode"

echo -e "${BOLD}${CYAN}=====================================================================${NC}"
echo -e "${BOLD}${CYAN}  🌌 CreativeWorkzStudio LLC — OmniCode Platform                     ${NC}"
echo -e "${BOLD}${CYAN}  Oracle Cloud VM Automated Synchronization Gateway                  ${NC}"
echo -e "${BOLD}${CYAN}  Target: ${ORACLE_USER}@${ORACLE_HOST} (${REMOTE_DIR})              ${NC}"
echo -e "${BOLD}${CYAN}=====================================================================${NC}"

# Check SSH Key
if [ ! -f "$SSH_KEY" ]; then
    echo -e "${RED}[!] SSH Private Key not found at: ${SSH_KEY}${NC}"
    echo -e "    Please specify valid key path via ORACLE_SSH_KEY env var."
    exit 1
fi
chmod 600 "$SSH_KEY" 2>/dev/null || true

# Test SSH Connection
echo -e "\n${BLUE}[1/4] Verifying SSH connectivity to Oracle VM...${NC}"
if ! ssh -i "$SSH_KEY" -o ConnectTimeout=8 -o StrictHostKeyChecking=accept-new "${ORACLE_USER}@${ORACLE_HOST}" "echo 'Connected successfully'" >/dev/null 2>&1; then
    echo -e "${RED}[!] Failed to establish SSH connection to ${ORACLE_USER}@${ORACLE_HOST}.${NC}"
    exit 1
fi
echo -e "${GREEN}[+] Remote Oracle VM host reachable.${NC}"

# Ensure Remote Directory Exists
echo -e "\n${BLUE}[2/4] Ensuring remote workspace directory exists...${NC}"
ssh -i "$SSH_KEY" "${ORACLE_USER}@${ORACLE_HOST}" "mkdir -p $REMOTE_DIR"

# Rsync Files to Oracle VM (excluding target/ and .git/ large artifacts for speed)
echo -e "\n${BLUE}[3/4] Synchronizing repository files via rsync...${NC}"
rsync -avz --delete \
    -e "ssh -i $SSH_KEY -o StrictHostKeyChecking=accept-new" \
    --exclude "target/" \
    --exclude ".git/" \
    --exclude "node_modules/" \
    --exclude "*.log" \
    "$BASE_DIR/" "${ORACLE_USER}@${ORACLE_HOST}:${REMOTE_DIR}/"
echo -e "${GREEN}[+] Repository source synchronization complete.${NC}"

# Ensure release binary exists locally and transfer it
if [ ! -f "$BASE_DIR/target/release/omnicode" ]; then
    echo -e "${BLUE}[*] Compiling local release binary with cargo...${NC}"
    (cd "$BASE_DIR" && cargo build --release)
fi

echo -e "\n${BLUE}[*] Deploying optimized release binary to Oracle VM...${NC}"
ssh -i "$SSH_KEY" "${ORACLE_USER}@${ORACLE_HOST}" "mkdir -p ${REMOTE_DIR}/target/release"
scp -i "$SSH_KEY" "$BASE_DIR/target/release/omnicode" "${ORACLE_USER}@${ORACLE_HOST}:/tmp/omnicode.new"
ssh -i "$SSH_KEY" "${ORACLE_USER}@${ORACLE_HOST}" "mv -f /tmp/omnicode.new ${REMOTE_DIR}/target/release/omnicode && chmod +x ${REMOTE_DIR}/target/release/omnicode"
echo -e "${GREEN}[+] Binary deployment complete.${NC}"

# Remote Service Configuration & Restart
echo -e "\n${BLUE}[4/4] Configuring firewall and restarting omnicode-web.service on Oracle VM...${NC}"
SSH_CMD="ssh -i \"$SSH_KEY\" \"${ORACLE_USER}@${ORACLE_HOST}\""

eval $SSH_CMD << 'EOF'
set -e
GREEN="\033[0;32m"
BLUE="\033[0;34m"
NC="\033[0m"

cd /home/ubuntu/OmniCode

# Install / update systemd service
SERVICE_FILE="infra/systemd/omnicode-web.service"
if [ ! -f "$SERVICE_FILE" ]; then
    SERVICE_FILE="systemd/omnicode-web.service"
fi
if [ -f "$SERVICE_FILE" ]; then
    echo -e "${BLUE}[*] Installing/updating omnicode-web.service from $SERVICE_FILE...${NC}"
    sudo cp "$SERVICE_FILE" /etc/systemd/system/omnicode-web.service
    sudo systemctl daemon-reload
    sudo systemctl enable omnicode-web.service
    sudo systemctl restart omnicode-web.service
fi

# Ensure TCP port 8080 is permitted in iptables
if ! sudo iptables -L INPUT -n | grep -q "dpt:8080"; then
    echo -e "${BLUE}[*] Opening port 8080 in iptables...${NC}"
    sudo iptables -I INPUT 6 -p tcp --dport 8080 -j ACCEPT
    if command -v netfilter-persistent >/dev/null 2>&1; then
        sudo netfilter-persistent save >/dev/null 2>&1 || true
    fi
fi

sleep 2
STATUS=$(sudo systemctl is-active omnicode-web.service || echo "inactive")
echo -e " • omnicode-web.service status: [${STATUS}]"
EOF

echo -e "\n${BOLD}${GREEN}=====================================================================${NC}"
echo -e "${BOLD}${GREEN}  ✅ OmniCode Oracle Cloud VM Synchronization Complete!              ${NC}"
echo -e "${BOLD}${GREEN}  CreativeWorkzStudio LLC                                            ${NC}"
echo -e "${BOLD}${GREEN}=====================================================================${NC}"
