# Torbox Companion

An alternative UI for TorBox. Manage downloads, streaming, and automation rules.

> Complete rewrite of the original [TorBox Manager](https://github.com/jittarao/torbox-app) in Rust for a new UI, better performance and memory safety.

---

## Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
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

- Rust-based for performance and memory safety
- Full TorBox API coverage (torrents, web downloads, usenet, streaming, search)
- Dark UI optimized for power users
- Automation rules for scheduled torrent management
- Docker support
- Real-time status updates

## API Features

- User management (profile, subscriptions, transactions)
- File management (create, control, monitor, download)
- Web downloads with progress tracking
- Usenet (NZB) processing
- RSS feed monitoring
- Video streaming with subtitles
- Search (metadata, file discovery, usenet)
- Cloud uploads (Google Drive, Dropbox, OneDrive)
- Batch operations

## Tech Stack

- Leptos (full-stack Rust framework)
- Axum (async web server)
- Tailwind CSS + custom SCSS
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
```

**Automation settings:**

- `TORBOX_MAX_RULES_PER_USER` - Limits rules per user to prevent resource exhaustion (default: 100)
- `TORBOX_LOG_RETENTION_DAYS` - How long to keep execution logs before cleanup (default: 90)
- `TORBOX_RULE_EXECUTION_TIMEOUT_SECS` - Maximum execution time before timeout (default: 130)

## Security

**API Key Storage:**

- Automation rules: Encrypted with AES-256-GCM, stored on server with SHA-256 hashing
- Regular usage: Stored locally in browser only

API keys are never sent to third parties. No user data stored on servers (except encrypted API keys for automation). Built with Rust for memory safety.

## Performance

20-30x faster than JavaScript alternatives. Memory efficient with Rust's zero-cost abstractions. Concurrent processing via Tokio async runtime.

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

## Privacy

**API Keys:**
- Automation rules: Encrypted (AES-256-GCM), stored on server with SHA-256 hashing
- Regular usage: Stored locally in browser only

No data sent to third parties. No information collected beyond encrypted API keys for automation. No tracking.

**Public Instance Logging:**
Server logs contain error messages and technical details (paths, codes, timestamps) for debugging. No API keys, user data, or personal information in logs.

## License

[AGPL-3.0](https://choosealicense.com/licenses/agpl-3.0/)

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

## Acknowledgments

Complete rewrite of the original [TorBox Manager](https://github.com/jittarao/torbox-app) in Rust. Thanks to the TorBox team for the API.