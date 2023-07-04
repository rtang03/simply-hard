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

RUN --mount=type=cache,target=/root/.cache cargo build --release

# Distribute the binary
# https://stackoverflow.com/questions/69607005/cannot-run-executables-with-alpine-and-busybox-docker-images
FROM alpine:latest AS release

RUN apt-get update && \
  apt-get install -y extra-runtime-dependencies && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /dist

# Use server as entrypoint
COPY --link --from=builder /build/target/release/simply-server ./simply-server
COPY --link --from=builder /build/target/release/simply-cli ./simply-cli
COPY --link --from=builder /build/env.toml ./env.toml

# https://stackoverflow.com/questions/66963068/docker-alpine-executable-binary-not-found-even-if-in-path/66974607
# https://stackoverflow.com/questions/68010688/docker-run-error-loading-shared-library-libstdc-so-6-and-libgcc-s-so-1
# RUN apk add libc6-compat libgcc

EXPOSE 50051

CMD ["./simply-server"]

################################
# syntax=docker/dockerfile:1.4

# see https://github.com/brainhivenl/rust-docker/tree/main
# https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
# https://github.com/hyperium/tonic/issues/1047

FROM rust:slim-buster AS builder

ENV SKIP_COMPILE_PROTO=true

RUN apt-get update && \
    apt-get -y install ca-certificates cmake musl-tools libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /build

COPY . .

RUN rustup default stable && rustup update
RUN rustup target add x86_64-unknown-linux-musl

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo build --target x86_64-unknown-linux-musl --release

# Distribute the binary
# https://stackoverflow.com/questions/69607005/cannot-run-executables-with-alpine-and-busybox-docker-images
FROM alpine:latest AS release

WORKDIR /dist

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/simply-server ./simply-server
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/simply-cli ./simply-cli
COPY --from=builder /build/env.toml ./env.toml

EXPOSE 50051

CMD ["./simply-server"]