# Base

FROM rust:latest@sha256:223cc32ddca8f3ee90c436dbc94b6b5f052ea7f9653c5201e9612f08ad7d21b8 AS base

RUN BINSTALL_VERSION=v1.27.0 curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall -y cargo-chef@0.1.78 sccache@0.19.0
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

ARG RELEASE
ENV RELEASE=$RELEASE

# Planner

FROM base AS planner

WORKDIR /usr/src/firefly-reports

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

# Builder

FROM base AS builder

RUN cargo binstall -y dioxus-cli@0.7.10

WORKDIR /usr/src/firefly-reports

COPY --from=planner /usr/src/firefly-reports/recipe.json recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    dx bundle --locked --release -p firefly-reports-web --web

# Container

FROM debian:trixie-slim@sha256:a72b4433de1cf7d6bae0c4cee74d0547f58601a643b3375f070c51600d63b6b2

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/firefly-reports/target/dx/firefly-reports-web/release/web/ /usr/local/firefly-reports-web

WORKDIR /usr/local/firefly-reports-web

ENV IP=0.0.0.0
ENV PORT=8080

EXPOSE 8080

CMD ["./server"]
