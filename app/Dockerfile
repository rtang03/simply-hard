# syntax=docker/dockerfile:1.4

# https://github.com/brainhivenl/rust-docker/tree/main
# https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
# https://github.com/hyperium/tonic/issues/1047
# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
# https://andygrove.io/2020/05/why-musl-extremely-slow/
# https://stackoverflow.com/questions/69607005/cannot-run-executables-with-alpine-and-busybox-docker-images

FROM rust:slim-buster AS builder

ENV SKIP_COMPILE_PROTO=true
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /build

RUN apt-get update && \
    apt-get -y install ca-certificates cmake musl-tools libssl-dev && \
    rm -rf /var/lib/apt/lists/*

RUN rustup default stable && rustup update
RUN rustup target add x86_64-unknown-linux-musl

# create a new empty shell project
RUN USER=root cargo init --name app && \
  mkdir -p src/bin && \
  echo "fn main() {}" > src/bin/cli.rs && \
  echo "fn main() {}" > src/bin/server.rs && \
  rm src/main.rs

# copy over your manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./env.toml ./env.toml

# this build step will cache your dependencies
RUN cargo build --target x86_64-unknown-linux-musl --release

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/app*
RUN cargo build --target x86_64-unknown-linux-musl --release

# Distribute the binary
FROM alpine:latest AS release

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/simply-server ./simply-server
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/simply-cli ./simply-cli
COPY --from=builder /build/env.toml ./env.toml

EXPOSE 50051

CMD ["./simply-server"]
