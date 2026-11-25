# Breadator: one binary that serves the pages, the API and the embedded static files.
#   docker build -t breadator .

FROM rust:1.100-slim-trixie AS wasm
RUN rustup target add wasm32-unknown-unknown \
    && cargo install wasm-bindgen-cli --version 0.2.130 --locked
WORKDIR /build
COPY Cargo.toml Cargo.lock rustfmt.toml ./
COPY crates ./crates
RUN cargo build --release --locked --target wasm32-unknown-unknown -p breadator-wasm \
    && wasm-bindgen --target web --out-dir /pkg target/wasm32-unknown-unknown/release/breadator_wasm.wasm

FROM node:24-trixie-slim AS web
WORKDIR /build/web
COPY web/package.json web/package-lock.json ./
RUN npm ci --no-audit --no-fund
COPY web ./
COPY --from=wasm /pkg ./pkg
RUN mkdir -p ../crates/breadator-server/static && npx tsc --noEmit && node build.mjs

FROM rust:1.100-slim-trixie AS server
WORKDIR /build
COPY Cargo.toml Cargo.lock rustfmt.toml ./
COPY crates ./crates
COPY migrations ./migrations
# The knowledge base is compiled in: articles and recipes, and the merged research files.
COPY content ./content
COPY research/data/bibliography.json research/data/claims.jsonl research/data/videos.json ./research/data/
# The islands and the WASM module are embedded in the binary (include_dir!).
COPY --from=web /build/crates/breadator-server/static/dist ./crates/breadator-server/static/dist
RUN cargo build --release --locked -p breadator-server && cp target/release/breadator /breadator

# Litestream: continuous replication of the SQLite database to object storage (Cloudflare R2).
FROM debian:trixie-slim AS litestream
ARG TARGETARCH
ARG LITESTREAM_VERSION=0.5.18
ADD https://github.com/benbjohnson/litestream/releases/download/v${LITESTREAM_VERSION}/litestream-${LITESTREAM_VERSION}-linux-x86_64.tar.gz /tmp/litestream-amd64.tar.gz
ADD https://github.com/benbjohnson/litestream/releases/download/v${LITESTREAM_VERSION}/litestream-${LITESTREAM_VERSION}-linux-arm64.tar.gz /tmp/litestream-arm64.tar.gz
RUN set -eu; \
    case "${TARGETARCH:-amd64}" in \
      amd64) sum=d3687b9fd0f2e9426bbd3da828429ff3f1dfc5c9b74d54fdf028e4ee510e7dea ;; \
      arm64) sum=922a09e6a7ac67f2095571894dc6382e94e1dd4cdd9674883ddf1a8e74ab982d ;; \
      *) echo "unsupported architecture ${TARGETARCH}" >&2; exit 1 ;; \
    esac; \
    echo "${sum}  /tmp/litestream-${TARGETARCH:-amd64}.tar.gz" | sha256sum -c -; \
    tar -xzf "/tmp/litestream-${TARGETARCH:-amd64}.tar.gz" -C /usr/local/bin litestream

FROM debian:trixie-slim AS runtime
# ca-certificates: TLS to Resend, Google, GitHub and the R2 bucket.
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 10001 breadator \
    && mkdir /data && chown breadator:breadator /data
COPY --from=server /breadator /usr/local/bin/breadator
COPY --from=litestream /usr/local/bin/litestream /usr/local/bin/litestream
COPY litestream.yml /etc/litestream.yml
COPY --chmod=0755 docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
# The entrypoint starts as root only to hand the volume (mounted root-owned) to uid 10001, then
# drops to it: the server and Litestream never run as root.
ENV PORT=8080 LOG_FORMAT=json DATABASE_PATH=/data/breadator.sqlite3
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD ["breadator", "healthcheck"]
ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
