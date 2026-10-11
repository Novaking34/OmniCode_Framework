# 🌐 OmniCode DNS Zone Infrastructure

Authoritative BIND RFC 1035 DNS zone configuration files for **OmniCode Studio & CreativeWorkzStudio LLC**, designed for 1-click import into Cloudflare DNS.

---

## 📁 File Manifest

| File | Standard | Description |
| :--- | :--- | :--- |
| [`omnicodestudio.com.zone`](file:///home/professorseanex/OmniCode/dns/omnicodestudio.com.zone) | RFC 1035 / BIND 9 | Master DNS zone file compatible with Cloudflare DNS Import |
| [`zone_manager.py`](file:///home/professorseanex/OmniCode/dns/zone_manager.py) | Python 3 | CLI generator, syntax validator, and record inspector |

---

## 🚀 How to Import into Cloudflare (Step-by-Step)

1. Log into your [Cloudflare Dashboard](https://dash.cloudflare.com/).
2. Select your domain (e.g. `omnicodestudio.com` or your company domain).
3. In the left navigation sidebar, click **DNS** ➔ **Records**.
4. Scroll to the **Import and Export** card and click **Import**.
5. Select [`omnicodestudio.com.zone`](file:///home/professorseanex/OmniCode/dns/omnicodestudio.com.zone).
6. Click **Import**. Cloudflare will parse and create all records automatically.

---

## 🗺️ Architectural Routing Map

```
Internet Request
       │
       ├── HTTPS Web Studio (Port 8080) ────► Cloudflare Tunnel (c346aa27...) ────► Oracle VM / Local Port 8080
       │     (studio.omnicodestudio.com)
       │
       ├── DuckDNS Dynamic Host ────────────► omnicodestudio.duckdns.org ─────────► 71.11.99.168 (CWS Gateway)
       │     (duck.omnicodestudio.com)
       │
       └── Yu-Gi-Oh! Game Simulator (7911) ─► play.omnicodestudio.com ────────────► 147.224.147.30 (Oracle VM)
             (Raw TCP - Grey Cloud Only!)
```

---

## ⚙️ Proxy Status Rules (Crucial!)

| Record | Type | Target | Cloudflare Proxy Status | Why |
| :--- | :--- | :--- | :--- | :--- |
| `@`, `www` | `CNAME` | `c346aa27...cfargotunnel.com.` | **Proxied (Orange Cloud)** | Free edge SSL/TLS & DDoS mitigation |
| `studio`, `api` | `CNAME` | `c346aa27...cfargotunnel.com.` | **Proxied (Orange Cloud)** | Direct edge routing to OmniCode |
| `local`, `gateway` | `A` | `71.11.99.168` | **DNS-Only (Grey Cloud)** | CreativeWorkzStudio LLC WAN IP |
| `cloud`, `vm` | `A` | `147.224.147.30` | **DNS-Only (Grey Cloud)** | Direct Oracle VM SSH / Administration |
| `duck`, `edge` | `CNAME` | `omnicodestudio.duckdns.org.` | **DNS-Only (Grey Cloud)** | Dynamic IP synchronization |
| `play`, `sim` | `A` | `147.224.147.30` | **DNS-Only (Grey Cloud)** | **MUST BE GREY CLOUD**: Raw TCP port 7911 |

---

## 🛠️ CLI Zone Management

```bash
cd /home/professorseanex/OmniCode

# Validate zone file syntax
python3 dns/zone_manager.py validate

# Inspect all configured records
python3 dns/zone_manager.py inspect

# Re-render with a custom domain
python3 dns/zone_manager.py render --domain yourcompany.com --out dns/yourcompany.com.zone
```
