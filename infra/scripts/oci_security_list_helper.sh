#!/usr/bin/env bash
# =============================================================================
# CreativeWorkzStudio LLC - OmniCode Platform
# Oracle Cloud VCN Ingress/Egress & Firewall Configuration Helper
# =============================================================================
# Helper script to manage VM OS firewall (iptables) and OCI VCN Security List
# for OmniCode Studio (port 8080).
#
# Requirements:
#   1. VM OS Firewall: iptables + netfilter-persistent (Already configured)
#   2. OCI Cloud Firewall: OCI CLI (installed at /usr/local/bin/oci) or OCI Console
# =============================================================================

set -e

BOLD="\033[1m"
GREEN="\033[0;32m"
BLUE="\033[0;34m"
CYAN="\033[0;36m"
YELLOW="\033[1;33m"
NC="\033[0m"

echo -e "${BOLD}${CYAN}=====================================================================${NC}"
echo -e "${BOLD}${CYAN}  🛡️  Oracle Cloud VM Firewall & Ingress/Egress Helper                ${NC}"
echo -e "${BOLD}${CYAN}  CreativeWorkzStudio LLC — OmniCode Platform (Port 8080)             ${NC}"
echo -e "${BOLD}${CYAN}=====================================================================${NC}"

# 1. Verify OS-level iptables rules
echo -e "\n${BLUE}[1/3] Checking OS-level iptables firewall rules for Port 8080...${NC}"
if sudo iptables -L INPUT -n | grep -q "dpt:8080"; then
    echo -e "${GREEN}[✓] Port 8080 is ACCEPT in iptables INPUT chain.${NC}"
else
    echo -e "${YELLOW}[*] Adding Port 8080 to iptables INPUT chain...${NC}"
    sudo iptables -I INPUT 6 -p tcp --dport 8080 -j ACCEPT
fi

# Ensure rules are saved persistently
if command -v netfilter-persistent >/dev/null 2>&1; then
    sudo netfilter-persistent save >/dev/null 2>&1
    echo -e "${GREEN}[✓] iptables rules saved persistently via netfilter-persistent.${NC}"
fi

# 2. Check OCI CLI installation
echo -e "\n${BLUE}[2/3] Checking Oracle Cloud Infrastructure CLI (oci-cli)...${NC}"
if command -v oci >/dev/null 2>&1; then
    OCI_VERSION=$(oci --version)
    echo -e "${GREEN}[✓] OCI CLI is installed at: $(which oci) (version: $OCI_VERSION)${NC}"
else
    echo -e "${YELLOW}[!] OCI CLI not found. Run setup on VM: sudo pip3 install oci-cli${NC}"
fi

# 3. Guidance for OCI VCN Security List (Cloud Layer)
echo -e "\n${BLUE}[3/3] Oracle Cloud VCN Ingress Rule Instructions (Cloud Hypervisor Layer):${NC}"
cat << 'EOF'

In Oracle Cloud Infrastructure (OCI), incoming traffic must pass through TWO layers:
  Layer 1: Linux OS Firewall (iptables) -> ALREADY CONFIGURED & PERSISTED (Port 8080 OPEN)
  Layer 2: OCI VCN Security List        -> Requires Cloud Ingress Rule

Option A: Via OCI Web Console (1-Minute Setup)
  1. Open Oracle Cloud Console (https://cloud.oracle.com)
  2. Navigate to: Networking > Virtual Cloud Networks > Select your VCN
  3. Under Resources, click "Security Lists" > Click "Default Security List for [VCN]"
  4. Click "Add Ingress Rules":
     - Source CIDR:            0.0.0.0/0
     - IP Protocol:            TCP
     - Destination Port Range: 8080
     - Description:           OmniCode Studio & Interactive Terminal GUI
  5. Click "Add Ingress Rules" to save.

Option B: Via OCI CLI (Command Line on VM)
  If you have configured OCI API credentials (`oci setup config`):
  Run:
    oci network security-list update \
      --security-list-id <YOUR_SECURITY_LIST_OCID> \
      --ingress-security-rules file://<(echo '[{"source":"0.0.0.0/0","protocol":"6","tcpOptions":{"destinationPortRange":{"max":8080,"min":8080}},"description":"OmniCode Studio"}]')

Option C: Via Cloudflare Zero Trust Tunnel (Zero Open Ports Required)
  Because cloudflared is already running on the VM, you can route:
  - Hostname: studio.omnicodestudio.com -> HTTP://localhost:8080
  This routes traffic securely via Cloudflare edge without needing any open ports in OCI VCN!
EOF

echo -e "\n${BOLD}${GREEN}=====================================================================${NC}"
echo -e "${BOLD}${GREEN}  ✓ Firewall verification & guidance completed.                      ${NC}"
echo -e "${BOLD}${GREEN}=====================================================================${NC}"
