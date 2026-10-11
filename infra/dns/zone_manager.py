#!/usr/bin/env python3
# =============================================================================
# BLOCK 1: METADATA BLOCK
# =============================================================================
"""
Module: dns.zone_manager
Architecture: Systems Engineering & Network Edge Routing (OmniCode Platform)
Organization: CreativeWorkzStudio LLC
Description:
    Authoritative BIND RFC 1035 Zone File Generator, Validator, and Synchronizer.
    Manages DNS routing rules for Cloudflare Zero Trust Tunnels, Oracle Cloud
    Compute VM (147.224.147.30), Local Gateway (71.11.99.168), and DuckDNS endpoints.

Exported CLI & API:
    - python3 dns/zone_manager.py validate [file.zone]
    - python3 dns/zone_manager.py inspect [file.zone]
    - python3 dns/zone_manager.py render [--domain DOMAIN] [--out PATH]
"""

import os
import re
import sys
from dataclasses import dataclass
from typing import Dict, List, Optional, Tuple

_CURRENT_DIR = os.path.dirname(os.path.abspath(__file__))
_PROJECT_ROOT = os.path.dirname(_CURRENT_DIR)

DEFAULT_ZONE_FILE_PATH: str = os.path.join(_CURRENT_DIR, "omnicodestudio.com.zone")
ENV_FILE_PATH: str = os.path.join(_PROJECT_ROOT, ".env")


# =============================================================================
# BLOCK 2: OPENING BLOCK (Data Structures & Models)
# =============================================================================

@dataclass
class DNSZoneRecord:
    name: str
    record_type: str
    target: str
    ttl: int = 3600
    comment: str = ""

    def to_bind_line(self) -> str:
        base = f"{self.name:<8} IN  {self.record_type:<7} {self.target}"
        if self.comment:
            return f"{base:<50} ; {self.comment}"
        return base


def parse_env_file(path: str) -> Dict[str, str]:
    values = {}
    if not os.path.isfile(path):
        return values
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            if "=" in line:
                k, v = line.split("=", 1)
                values[k.strip()] = v.strip().strip("\"'")
    return values


# =============================================================================
# BLOCK 3: BODY BLOCK (Renderer, Validator & Inspector)
# =============================================================================

def render_zone_file(
    domain: str = "omnicodestudio.com",
    oracle_ip: str = "147.224.147.30",
    local_ip: str = "71.11.99.168",
    duckdns_host: str = "omnicodestudio.duckdns.org",
    tunnel_id: str = "c346aa27-b4c1-41ba-815c-f66b2b84243b",
    ttl: int = 3600,
) -> str:
    """Renders a standard RFC 1035 / BIND 9 zone file string for Cloudflare import."""
    tunnel_target = f"{tunnel_id}.cfargotunnel.com."
    duck_target = f"{duckdns_host.rstrip('.') }."

    lines = [
        "; =============================================================================",
        f"; BIND RFC 1035 Zone File for {domain}",
        "; CreativeWorkzStudio LLC — OmniCode Platform Infrastructure",
        "; Compatible with Cloudflare DNS Zone File Import (DNS -> Records -> Import)",
        "; =============================================================================",
        f"$ORIGIN {domain.rstrip('.')}.",
        f"$TTL {ttl}",
        "",
        "; =============================================================================",
        "; 1. PRIMARY WEB STUDIO & REST API (HTTP/HTTPS - Port 8080)",
        "; =============================================================================",
        "; Option A: Cloudflare Zero Trust Named Tunnel (Recommended)",
        f"; Tunnel ID: {tunnel_id}",
        DNSZoneRecord("@", "CNAME", tunnel_target, ttl, "Root Apex -> Cloudflare Tunnel").to_bind_line(),
        DNSZoneRecord("www", "CNAME", tunnel_target, ttl, "WWW Subdomain -> Cloudflare Tunnel").to_bind_line(),
        DNSZoneRecord("studio", "CNAME", tunnel_target, ttl, "OmniCode Studio Web Interface").to_bind_line(),
        DNSZoneRecord("api", "CNAME", tunnel_target, ttl, "OmniCode REST API Endpoint").to_bind_line(),
        "",
        "; Option B: Oracle Cloud Compute 24/7 Instance",
        DNSZoneRecord("cloud", "A", oracle_ip, ttl, "Oracle Cloud VM Direct Ingress").to_bind_line(),
        DNSZoneRecord("vm", "A", oracle_ip, ttl, "Oracle Cloud VM Alias").to_bind_line(),
        "",
        "; =============================================================================",
        "; 2. CREATIVEWORKZSTUDIO LLC LOCAL GATEWAY (WORKSTATION IP)",
        "; =============================================================================",
        DNSZoneRecord("local", "A", local_ip, ttl, "CreativeWorkzStudio LLC Workstation WAN").to_bind_line(),
        DNSZoneRecord("gateway", "A", local_ip, ttl, "CreativeWorkzStudio LLC Gateway IP").to_bind_line(),
        "",
        "; =============================================================================",
        "; 3. DYNAMIC DNS (DuckDNS) FALLBACK ALIASES",
        "; =============================================================================",
        DNSZoneRecord("duck", "CNAME", duck_target, ttl, "DuckDNS Dynamic Host Endpoint").to_bind_line(),
        DNSZoneRecord("edge", "CNAME", duck_target, ttl, "DuckDNS Edge Alias").to_bind_line(),
        "",
        "; =============================================================================",
        "; 4. UNIFIED PLATFORM CROSS-LINKS (Yu-Gi-Oh! Server & Simulator)",
        "; =============================================================================",
        DNSZoneRecord("cards", "CNAME", tunnel_target, ttl, "Card Catalog & Lore Web Dashboard").to_bind_line(),
        DNSZoneRecord("play", "A", oracle_ip, ttl, "Live Duel Simulator (Raw TCP 7911 - Grey Cloud)").to_bind_line(),
        DNSZoneRecord("sim", "A", oracle_ip, ttl, "Game Room Manager (Port 7922)").to_bind_line(),
        "",
    ]
    return "\n".join(lines)


def validate_zone_file(file_path: str) -> Tuple[bool, List[str]]:
    """Validates zone file syntax and record definitions."""
    errors = []
    if not os.path.isfile(file_path):
        return False, [f"File not found: {file_path}"]

    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()

    has_origin = bool(re.search(r"^\$ORIGIN\s+[\w\.\-]+\.", content, re.MULTILINE))
    if not has_origin:
        errors.append("Missing required directive: $ORIGIN <domain>.")

    has_ttl = bool(re.search(r"^\$TTL\s+\d+", content, re.MULTILINE))
    if not has_ttl:
        errors.append("Missing required directive: $TTL <seconds>")

    record_pattern = re.compile(r"^\s*([\@\w\.\-]+)\s+(?:IN\s+)?(A|AAAA|CNAME|TXT|MX|NS)\s+([^\s;]+)", re.MULTILINE)
    matches = record_pattern.findall(content)
    if not matches:
        errors.append("No valid DNS resource records found in file.")

    return len(errors) == 0, errors


def inspect_zone_file(file_path: str):
    """Parses and prints a formatted summary table of the DNS zone records."""
    if not os.path.isfile(file_path):
        print(f"[!] Error: File '{file_path}' does not exist.")
        sys.exit(1)

    print("=====================================================================")
    print(f"  🌐 DNS Zone Inspector: {os.path.basename(file_path)}")
    print("  CreativeWorkzStudio LLC — OmniCode Platform")
    print("=====================================================================")

    with open(file_path, "r", encoding="utf-8") as f:
        lines = f.readlines()

    origin = "unknown"
    ttl = "unknown"
    records = []

    record_re = re.compile(r"^\s*([\@\w\.\-]+)\s+(?:IN\s+)?(A|AAAA|CNAME|TXT|MX|NS)\s+([^\s;]+)(?:\s*;\s*(.*))?")

    for line in lines:
        stripped = line.strip()
        if stripped.startswith("$ORIGIN"):
            origin = stripped.split()[1]
        elif stripped.startswith("$TTL"):
            ttl = stripped.split()[1]
        elif stripped and not stripped.startswith(";"):
            m = record_re.match(line)
            if m:
                records.append({
                    "name": m.group(1),
                    "type": m.group(2),
                    "target": m.group(3),
                    "comment": m.group(4) or ""
                })

    print(f"• Origin Domain : {origin}")
    print(f"• Default TTL   : {ttl} seconds")
    print(f"• Record Count  : {len(records)}\n")
    print(f"{'NAME':<10} {'TYPE':<8} {'TARGET':<45} {'NOTE'}")
    print("-" * 75)
    for r in records:
        print(f"{r['name']:<10} {r['type']:<8} {r['target']:<45} {r['comment']}")
    print("-" * 75)


# =============================================================================
# BLOCK 4: CLOSING BLOCK (CLI Dispatcher)
# =============================================================================

def main():
    args = sys.argv[1:]
    command = args[0] if args else "inspect"

    target_file = DEFAULT_ZONE_FILE_PATH
    env = parse_env_file(ENV_FILE_PATH)

    if command == "inspect":
        file_to_inspect = args[1] if len(args) > 1 else target_file
        inspect_zone_file(file_to_inspect)

    elif command == "validate":
        file_to_val = args[1] if len(args) > 1 else target_file
        valid, errors = validate_zone_file(file_to_val)
        if valid:
            print(f"[✓] Zone file '{file_to_val}' is syntactically valid RFC 1035 format.")
            sys.exit(0)
        else:
            print(f"[✗] Zone file validation failed with {len(errors)} error(s):")
            for err in errors:
                print(f"    - {err}")
            sys.exit(1)

    elif command == "render":
        domain = "omnicodestudio.com"
        out_file = target_file

        if "--domain" in args:
            idx = args.index("--domain")
            if idx + 1 < len(args):
                domain = args[idx + 1]

        if "--out" in args:
            idx = args.index("--out")
            if idx + 1 < len(args):
                out_file = args[idx + 1]

        oracle_ip = env.get("ORACLE_VM_HOST", "147.224.147.30")
        local_ip = env.get("PUBLIC_WAN_IP", "71.11.99.168")
        duckdns = env.get("PUBLIC_DOMAIN", "omnicodestudio.duckdns.org")
        tunnel_id = env.get("CLOUDFLARE_TUNNEL_ID", "c346aa27-b4c1-41ba-815c-f66b2b84243b")

        rendered = render_zone_file(
            domain=domain,
            oracle_ip=oracle_ip,
            local_ip=local_ip,
            duckdns_host=duckdns,
            tunnel_id=tunnel_id,
        )

        with open(out_file, "w", encoding="utf-8") as f:
            f.write(rendered)

        print(f"[✓] Rendered RFC 1035 zone file to: {out_file}")
        inspect_zone_file(out_file)

    else:
        print("Usage:")
        print("  python3 dns/zone_manager.py inspect [path.zone]")
        print("  python3 dns/zone_manager.py validate [path.zone]")
        print("  python3 dns/zone_manager.py render [--domain DOMAIN] [--out PATH]")


if __name__ == "__main__":
    main()
