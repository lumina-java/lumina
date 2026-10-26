# ─── Stage 1: Build Stage ───────────────────────────────────────────────────────────
FROM rust:1.80-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Build dengan Docker BuildKit Cache Mounts agar kompilasi ulang hanya memakan hitungan detik
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    (cp /app/target/release/lumina_clean-server /app/server 2>/dev/null || \
     cp /app/target/release/lumina_clean /app/server 2>/dev/null || \
     find /app/target/release/ -maxdepth 1 -type f -executable ! -name "*.so" ! -name "*.d" -exec cp {} /app/server \; )

# ─── Stage 2: Runtime Stage ──────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/server ./server
COPY --from=builder /app/resources ./resources
COPY --from=builder /app/public ./public
COPY --from=builder /app/database ./database
COPY --from=builder /app/storage ./storage
COPY --from=builder /app/.env.example ./.env

ENV APP_ENV=production
ENV APP_PORT=8000

EXPOSE 8000
CMD ["./server"]
