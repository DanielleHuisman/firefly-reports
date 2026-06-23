# Base

FROM rust:latest AS base

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall -y cargo-chef@0.1.77 sccache@0.15.0
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

RUN cargo binstall -y dioxus-cli@0.7.9

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

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/firefly-reports/target/dx/firefly-reports-web/release/web/ /usr/local/firefly-reports-web

WORKDIR /usr/local/firefly-reports-web

ENV IP=0.0.0.0
ENV PORT=8080

EXPOSE 8080

CMD ["./server"]
