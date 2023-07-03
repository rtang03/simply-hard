# syntax=docker/dockerfile:1.4

# see https://github.com/brainhivenl/rust-docker/tree/main
# https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/

FROM rust:slim-buster AS builder

ENV SCCACHE_VERSION=0.5.4
ENV RUSTC_WRAPPER=/usr/local/bin/sccache

RUN apt-get update && \
    apt-get install -y build-essential wget && \
    rm -rf /var/lib/apt/lists/*

# Install and configure sccache to speed up builds
RUN ARCH= && alpineArch="$(dpkg --print-architecture)" \
      && case "${alpineArch##*-}" in \
        amd64) \
          ARCH='x86_64' \
          ;; \
        arm64) \
          ARCH='aarch64' \
          ;; \
        *) ;; \
      esac \
    && wget -O sccache.tar.gz https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-${ARCH}-unknown-linux-musl.tar.gz \
    && tar xzf sccache.tar.gz \
    && mv sccache-v*/sccache /usr/local/bin/sccache \
    && chmod +x /usr/local/bin/sccache

# Pre-compile dependencies
WORKDIR /build

# TODO: Add Cargo.lock
# Fill in the package name in Cargo.toml
RUN USER=root cargo init --name app && \
  mkdir -p src/bin && \
  echo "fn main() {}" > src/bin/cli.rs && \
  echo "fn main() {}" > src/bin/server.rs && \
  rm src/main.rs

# COPY prefetch_cargo.toml Cargo.toml

RUN --mount=type=cache,target=/root/.cache cargo fetch && \
  cargo build --release

# Build the project
COPY src src
COPY env.toml env.toml
COPY Cargo.toml Cargo.toml

# RUN rm ./target/release/deps/app* && \
#   cargo build --release

RUN --mount=type=cache,target=/root/.cache cargo build --release

# Distribute the binary
FROM alpine:latest AS release

WORKDIR /dist

# RUN apk add coreutils

# Use server as entrypoint
COPY --link --from=builder /build/target/release/simply-server ./simply-server

CMD ["/dist/simply-server"]
