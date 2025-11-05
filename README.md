# Torbox Companion

An alternative to the default TorBox UI. Manage downloads, streaming, and more.

> **Note**: This is a complete rewrite of the original [TorBox Manager](https://github.com/jittarao/torbox-app) built with Rust instead of Next.js for superior performance and memory safety.

## Features

- **Fast & Secure**: Built with Rust for maximum performance and security
- **Complete TorBox Integration**: Full API coverage for torrents, web downloads, usenet, streaming, and search
- **Modern Dark UI**: Clean, responsive interface with dark theme optimized for power users
- **API Key Authentication**: Secure API key management with local storage
- **Docker Ready**: Containerized for easy deployment
- **Lightweight**: Minimal dependencies for optimal performance
- **Real-time Updates**: Live status monitoring and automatic refresh

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos)
- [Docker](https://www.docker.com/) (optional)

### Installing cargo-leptos

```bash
cargo install cargo-leptos --locked
```

### Running the project

#### Development

```bash
cargo leptos watch
```

This will watch for changes and automatically recompile. Open your browser to `http://localhost:3000`.

#### Docker

```bash
docker compose up -d
```

#### Production Build

```bash
cargo leptos build --release
```

## Requirements

- Rust 1.70 or later
- A running TorBox instance with API access
- Valid TorBox API key (get yours from [TorBox Settings](https://torbox.app/settings))

## Tech Stack

- **Framework**: Leptos (full-stack Rust web framework)
- **Backend**: Axum (async web framework)
- **Styling**: Tailwind CSS with custom SCSS
- **Runtime**: Tokio (async runtime)
- **Containerization**: Docker with multi-stage builds
- **HTTP Client**: Reqwest for API communication

## API Features

### Complete TorBox API Coverage

- **User Management**: Profile, subscriptions, transactions, referrals
- **File Management**: Create, control, monitor, and download files
- **Web Downloads**: Direct download from URLs with progress tracking
- **Usenet Downloads**: NZB file processing and management
- **RSS Feeds**: Automated file monitoring and downloading
- **Streaming**: Video streaming with subtitle and audio track support
- **Search Integration**: Metadata search, file discovery, usenet search
- **Cloud Integration**: Google Drive, Dropbox, OneDrive uploads
- **Notifications**: Real-time status updates and alerts

### Advanced Features

- **Batch Operations**: Upload multiple files simultaneously
- **Smart Downloads**: Cherry-pick specific files across multiple downloads
- **Real-time Monitoring**: Live status updates and progress tracking
- **Customizable Interface**: Tailor the workflow to match your needs

## Development

### Adding Dependencies

Add new dependencies to `Cargo.toml`:

```toml
[dependencies]
your-crate = "1.0"
```

### Building for Production

```bash
cargo leptos build --release
```

The server will be available at `target/release/torbox-companion`.

### Docker Deployment

#### Build and Run

```bash
docker compose up -d
```

#### Custom Configuration

The application uses environment variables for configuration:

```bash
LEPTOS_SITE_ADDR=0.0.0.0:3000  # Server address
```

## Security

- API keys are stored locally and never transmitted
- No user data is stored on servers
- Built with Rust's memory safety guarantees
- Minimal attack surface with focused dependencies
- Secure by default with modern web security practices

## Performance

- **20-30x faster** than JavaScript-based alternatives
- **Memory efficient** with Rust's zero-cost abstractions
- **Concurrent processing** with Tokio async runtime
- **Optimized builds** with size-optimized WASM bundles
- **Fast startup** with minimal dependencies

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Fork the repository
2. Clone your fork
3. Create a feature branch
4. Make your changes
5. Add tests if applicable
6. Submit a pull request

## Support

If you find this project helpful and would like to support its development, consider buying me a coffee! Your support helps me continue improving Torbox Companion and building more open-source tools.

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-☕-yellow.svg)](https://buymeacoffee.com/crazy1)

## Legal Disclaimer

**IMPORTANT LEGAL NOTICE**

This application is a **management tool** and **does not host, store, or distribute any content**. It is designed to interface with existing services and APIs for legitimate file management purposes only.

### What This Application Does:
- Provides a user interface for managing existing downloads
- Interfaces with legitimate cloud storage services
- Offers file organization and management tools
- Connects to authorized APIs for data synchronization
- Acts as an API proxy to forward requests to authorized services

### What This Application Does NOT Do:
- Host or store any copyrighted content
- Distribute or share any files
- Transmit or proxy streaming content (streams load directly from service provider's CDN)
- Provide access to unauthorized content
- Facilitate copyright infringement
- Store or cache any user data beyond API keys
- Cache or buffer any media content (video/audio files)

### User Responsibility:
Users are solely responsible for ensuring their use of this application complies with all applicable laws and regulations in their jurisdiction. This includes but is not limited to:
- Respecting copyright laws
- Obtaining proper authorization for any content
- Complying with local regulations
- Using the application for legitimate purposes only

### No Liability:
The developers of this application assume no responsibility for how users utilize this tool. Users must ensure their activities are legal and authorized in their jurisdiction.

## DMCA Compliance

Since no content is hosted, stored, transmitted, or cached on our servers, DMCA notices are not applicable to this application. All content delivery (including streaming) occurs directly between the user and the service provider's infrastructure. This application serves only as an interface and API proxy, forwarding requests without touching any actual media content.

## Terms of Service

By using this application, you agree to use it only for legitimate file management purposes and comply with all applicable laws.

## Privacy Policy

- **API Keys**: Stored locally in your browser only
- **No Data Transmission**: No user data is sent to our servers
- **No Information Collection**: We do not collect or store any user information
- **Local Storage Only**: All data remains on your device
- **No Tracking**: No analytics or tracking mechanisms

### Public Instance Logging

If using a publicly available instance of this application:
- **Error Logging**: Server logs may contain error messages and debugging information
- **No Personal Data**: Logs do not include API keys, user data, or personal information
- **Technical Information Only**: Logs may include technical details like request paths, error codes, and timestamps
- **Debugging Purposes**: Logging is used solely for application maintenance and error resolution

## License

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0/)

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

## Acknowledgments

- **Original TorBox App**: This project is a complete rewrite of the original [TorBox Manager](https://github.com/jittarao/torbox-app) built with Next.js
- **TorBox Team**: For providing the comprehensive API that makes this possible

---

**Built with ❤️ using Rust for the power user who demands performance and reliability.**