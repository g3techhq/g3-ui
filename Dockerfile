FROM rust:1.98.0-bookworm AS builder

ARG DIOXUS_CLI_VERSION=0.7.9

RUN apt-get update \
    && apt-get install --yes --no-install-recommends clang \
    && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN cargo install dioxus-cli --version "${DIOXUS_CLI_VERSION}" --locked

WORKDIR /workspace
COPY . .

WORKDIR /workspace/playground
RUN dx bundle --web --release --locked

FROM nginxinc/nginx-unprivileged:stable-alpine

LABEL org.opencontainers.image.source="https://github.com/g3techhq/g3-ui"
LABEL org.opencontainers.image.description="Hosted g3-ui Dioxus component playground"
LABEL org.opencontainers.image.licenses="MIT OR Apache-2.0"

COPY deploy/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /workspace/playground/dist/public/ /usr/share/nginx/html/

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
  CMD wget --quiet --output-document=- http://127.0.0.1:8080/healthz || exit 1
