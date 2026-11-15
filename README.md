

# Torbox Companion

An alternative UI for TorBox. Manage downloads, streaming, and automation rules.

> Complete rewrite of the original [TorBox Manager](https://github.com/jittarao/torbox-app) in Rust for a new UI, better performance and memory safety.

---

## Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (nightly; the Leptos `nightly` feature is enabled)
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos)
- Docker (optional)

### Install cargo-leptos

```bash
cargo install cargo-leptos --locked
```

### Run

**Development:**
```bash
cargo leptos watch
```
Open `http://localhost:3000` in your browser.

**Docker:**
```bash
docker compose up -d
```

**Production:**
```bash
cargo leptos build --release
```

## Requirements

- Rust 1.70+
- TorBox API key ([get one here](https://torbox.app/settings))

## Features

- Rust-based Leptos UI for TorBox
- Torrents, web downloads, and usenet list/control
- Search (IMDB and text) with add-to-library
- Streaming player
- Cloud upload to Google Drive, Dropbox, and OneDrive
- Automation rules for scheduled torrent management
- Docker support
- Status polling while the dashboard tab is visible

## API Features

TorBox coverage in the UI focuses on day-to-day download management. The HTTP client also wraps additional TorBox endpoints (RSS, notifications, referrals, cached checks, and more) that are not all exposed in the dashboard yet.

Shipped in the UI:

- User profile (`/user/me`)
- Torrent / webdl / usenet create, list, control, download links
- Queued download listing
- Search API (torrents and usenet)
- Stream create + playback
- Cloud upload (Google Drive, Dropbox, OneDrive)
- Server-side automation against torrent control

## Tech Stack

- Leptos (full-stack Rust framework)
- Axum (async web server)
- Custom SCSS
- Tokio (async runtime)
- Docker (multi-stage builds)

## Automation Rules

Schedule automated actions on torrents based on conditions.

### Features

- Multiple conditions (seeding time, ratio, stalled time, file size, progress, etc.)
- Cron expressions or interval triggers (minimum 30 minutes)
- Actions: stop seeding, delete, stop, resume, restart, reannounce, force start
- Execution logs with success/failure status
- Force run on demand
- Bulk rule management

### Creating a Rule

1. Go to Automations tab
2. Click "Create Rule"
3. Set trigger (cron or interval)
4. Add conditions (e.g., "SeedingTime > 24 hours")
5. Choose action (e.g., "Stop Seeding")
6. Save

### Presets

- **Delete Inactive Torrents** - Removes failed, expired, stalled torrents
- **Delete Stalled Torrents** - Removes torrents stalled for specified time

### Server Administration

**Delete all rules and logs:**

```bash
sqlite3 data/torbox.db "DELETE FROM rule_execution_log; DELETE FROM automation_rules;"
```


**Delete rules for a specific user:**

```sql
DELETE FROM rule_execution_log WHERE api_key_hash = 'api_key_hash_here';
DELETE FROM automation_rules WHERE api_key_hash = 'api_key_hash_here';
```

Database location: `data/torbox.db` (or your mounted data volume)

## Configuration

### Environment Variables

```bash
LEPTOS_SITE_ADDR=0.0.0.0:3000
TORBOX_MAX_RULES_PER_USER=100
TORBOX_LOG_RETENTION_DAYS=90
TORBOX_RULE_EXECUTION_TIMEOUT_SECS=130
TORBOX_PROXY_RATE_LIMIT=300
TORBOX_PROXY_MAX_BODY_MB=100
```

**Automation settings:**

- `TORBOX_MAX_RULES_PER_USER` - Limits rules per user to prevent resource exhaustion (default: 100)
- `TORBOX_LOG_RETENTION_DAYS` - How long to keep execution logs before cleanup (default: 90)
- `TORBOX_RULE_EXECUTION_TIMEOUT_SECS` - Maximum execution time before timeout (default: 130)

**Site / proxy:**

- `SITE_URL` - Optional. Your public origin if you want absolute SEO tags. Leave unset for self-hosted defaults (relative assets, no hardcoded host).
- `TORBOX_PROXY_RATE_LIMIT` - Max proxied TorBox API requests per client IP per minute (default: 300). Behind a reverse proxy, set `X-Forwarded-For` or `X-Real-IP` to the visitor. Those headers are trusted when the TCP peer is local/private, or when `TORBOX_TRUST_PROXY=1` (required for public CDN/proxy peers such as Cloudflare). If headers are present but not trusted, rate limits fall back to a per-API-key bucket instead of the proxy IP.
- `TORBOX_TRUST_PROXY` - Set to `1` when a public reverse proxy/CDN sets forwarded client IP headers. Without this, public proxy peers are not treated as the visitor.
- `TORBOX_PROXY_MAX_BODY_MB` - Max upload body size through the TorBox proxy in MiB (default: 100). Oversized bodies return HTTP 413.

**Optional analytics (host-controlled):**

- `RYBBIT_SITE_ID` and `RYBBIT_SCRIPT_SRC` - when both are set, the SSR shell injects the analytics script
- Leave unset for no analytics

## Security

**API Key Storage:**

- Automation rules: Encrypted with AES-256-GCM, stored on server with SHA-256 hashing
- Regular usage: Stored locally in browser only
- Download and automation APIs accept `Authorization: Bearer` or `X-Api-Key` only (not query strings)

API keys are never sent to third parties by this app. Built with Rust for memory safety.

## Privacy

**API Keys:**

- Automation rules: Encrypted (AES-256-GCM), stored on server with SHA-256 hashing
- Regular usage: Stored locally in browser only

Automation execution logs may include rule names and torrent item names for the configured retention period.

Optional analytics: if the host sets `RYBBIT_SITE_ID` and `RYBBIT_SCRIPT_SRC`, a privacy-friendly analytics script is injected. Self-hosted users control this.

**Public Instance Logging:**
Server logs contain error messages and technical details (paths, codes, timestamps) for debugging. Auth headers are redacted. No API keys are intentionally logged.

## Performance

Status lists refresh about every 10 seconds while the dashboard tab is visible, and pause while the tab is hidden.

---

## Contributing

PRs welcome. For major changes, open an issue first.

1. Fork the repo
2. Create a feature branch
3. Make your changes
4. Submit a PR

## Support

If you find this useful, consider [buying me a coffee](https://buymeacoffee.com/crazy1).

## Legal

This is a management tool that does not host, store, or distribute content. It interfaces with existing services for file management.

**What it does:**
- UI for managing downloads
- Interfaces with cloud storage services
- API proxy for authorized services

**What it doesn't do:**
- Host or store copyrighted content
- Distribute files
- Cache media content
- Provide unauthorized access

Users are responsible for compliance with applicable laws. No liability assumed by developers.

**DMCA:** Not applicable. No content hosted or cached. All delivery occurs directly between user and service provider.

## License

[AGPL-3.0](https://choosealicense.com/licenses/agpl-3.0/)

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

## Acknowledgments

Complete rewrite of the original [TorBox Manager](https://github.com/jittarao/torbox-app) in Rust. Thanks to the TorBox team for the API.
