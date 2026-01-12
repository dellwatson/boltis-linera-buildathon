# ==============================
# Stage 1: Build Linera + WASM app
# ==============================
FROM rust:1.86.0-slim AS builder

# Install dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    protobuf-compiler \
    libprotobuf-dev \
    clang \
    make \
    libssl-dev \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Add wasm target
RUN rustup target add wasm32-unknown-unknown

# Install Linera binaries (Conway testnet version 0.15.5)
RUN cargo install --locked linera-storage-service@0.15.7 && \
    cargo install --locked linera-service@0.15.7

# Set up working directory
WORKDIR /app

# Copy project files
COPY . .

# Build the WASM target (Counter only)
RUN cargo build --release --target wasm32-unknown-unknown -p counter

# ==============================
# Stage 2: Runtime image
# ==============================
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    openssl \
    libgcc-s1 \
    && rm -rf /var/lib/apt/lists/* 

# Set up working directory
WORKDIR /app

# Copy Linera binaries from builder
COPY --from=builder /usr/local/cargo/bin/linera /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/linera-server /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/linera-proxy /usr/local/bin/

# Copy the Counter WASM artifacts and deploy script
COPY --from=builder /app/target/wasm32-unknown-unknown/release/counter_*.wasm ./counter/
COPY --from=builder /app/deploy_counter_canonical.sh ./deploy_counter_canonical.sh

RUN chmod +x ./deploy_counter_canonical.sh

EXPOSE 8080

ENTRYPOINT ["/app/deploy_counter_canonical.sh"]
