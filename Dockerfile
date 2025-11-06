# Multi-stage build for production-ready Rust application with distroless runtime
FROM rust:1.90-slim AS base

ARG BUILD_DATE
ARG VCS_REF
ARG VERSION
ARG TARGETARCH

# Install system dependencies in a single layer
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    unzip \
    build-essential \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Install Rust tools with caching
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    cargo install cargo-chef --version ^0.1 && \
    cargo install cargo-leptos --locked && \
    cargo install wasm-pack --version 0.12.1 && \
    rustup target add wasm32-unknown-unknown

ENV CARGO_INCREMENTAL=1
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

WORKDIR /app

# Planner stage: Analyze dependencies
FROM base AS planner
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    cargo chef prepare --recipe-path recipe.json

# Cacher stage: Pre-compile dependencies
FROM base AS cacher
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/app/target \
    cargo chef cook --release --recipe-path recipe.json

# Builder stage: Build application
FROM base AS builder
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY style ./style
COPY public ./public

ENV LEPTOS_OUTPUT_NAME=torbox-companion
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV LEPTOS_ENV=PROD
ENV RUST_BACKTRACE=1

RUN if [ "$TARGETARCH" = "arm64" ]; then \
        echo "ARM64 detected, disabling wasm-opt" && \
        if ! grep -qE "^wasm-opt-features\s*=\s*\[\]" Cargo.toml; then \
            awk '/^\[package\.metadata\.leptos\]/ { \
                print; \
                print "wasm-opt-features = []"; \
                next \
            } \
            { print }' Cargo.toml > Cargo.toml.tmp && \
            mv Cargo.toml.tmp Cargo.toml; \
        fi && \
        if ! grep -qE "^wasm-opt\s*=\s*false" Cargo.toml; then \
            awk '/^\[package\.metadata\.leptos\]/ { \
                print; \
                print "wasm-opt = false"; \
                next \
            } \
            { print }' Cargo.toml > Cargo.toml.tmp && \
            mv Cargo.toml.tmp Cargo.toml; \
        fi && \
        echo "Verifying wasm-opt configuration:" && \
        grep -E "^wasm-opt" Cargo.toml || echo "No wasm-opt settings found"; \
    else \
        echo "TARGETARCH is $TARGETARCH, using wasm-opt optimization"; \
    fi

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/app/target \
    sh -c 'CARGO_BUILD_JOBS=$(nproc) cargo leptos build --release && \
    echo "Build completed, checking for WASM file..." && \
    if [ ! -f target/site/pkg/torbox-companion.wasm ]; then \
        echo "ERROR: cargo leptos build failed to create torbox-companion.wasm"; \
        echo "Contents of target/site/pkg:"; \
        ls -la target/site/pkg/ || echo "Directory does not exist"; \
        exit 1; \
    fi && \
    echo "WASM file found, creating symlink..." && \
    cd target/site/pkg && \
    ln -sf torbox-companion.wasm torbox-companion_bg.wasm && \
    echo "Build verification complete"'

RUN --mount=type=cache,target=/app/target \
    sh -c 'mkdir -p /app/build-output && \
    cp -r target/release/torbox-companion /app/build-output/ && \
    cp -r target/site /app/build-output/ && \
    echo "Build artifacts copied to filesystem"'

FROM gcr.io/distroless/cc-debian12

ARG BUILD_DATE
ARG VCS_REF
ARG VERSION

LABEL org.opencontainers.image.title="Torbox Companion"
LABEL org.opencontainers.image.description="A modern, high-performance alternative to the default TorBox UI."
LABEL org.opencontainers.image.url="https://github.com/crazycacti/torbox-companion"
LABEL org.opencontainers.image.source="https://github.com/crazycacti/torbox-companion"
LABEL org.opencontainers.image.version="${VERSION}"
LABEL org.opencontainers.image.created="${BUILD_DATE}"
LABEL org.opencontainers.image.revision="${VCS_REF}"
LABEL org.opencontainers.image.licenses="AGPL-3.0"

WORKDIR /app
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
# Enable request logging by default (set to "false" to disable)
ENV ENABLE_LOGGING=true
# Request log level: "errors" (errors only, default), "full" (all requests for debugging only)
ENV LOG_LEVEL=errors
# Rust log level: trace, debug, info, warn, error (default: info)
ENV RUST_LOG=info

COPY --from=builder /app/build-output/torbox-companion /app/torbox-companion
COPY --from=builder /app/build-output/site ./target/site
COPY --from=builder /app/Cargo.toml ./

EXPOSE 3000
CMD ["./torbox-companion"]