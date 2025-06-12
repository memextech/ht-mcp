# Multi-stage build for ht-mcp-rust
FROM rust:1.85-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY ht-core/Cargo.toml ./ht-core/

# Create dummy source files to cache dependencies
RUN mkdir src ht-core/src && \
    echo "fn main() {}" > src/main.rs && \
    echo "// dummy" > src/lib.rs && \
    echo "// dummy" > ht-core/src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && \
    rm -rf src ht-core/src target/release/deps/ht_*

# Copy real source code
COPY src ./src
COPY ht-core/src ./ht-core/src

# Build the actual binary
RUN cargo build --release

# Runtime image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/false ht-user

# Copy binary from builder
COPY --from=builder /app/target/release/ht-mcp-rust /usr/local/bin/ht-mcp-rust

# Set proper permissions
RUN chmod +x /usr/local/bin/ht-mcp-rust

# Switch to non-root user
USER ht-user

# Set the entrypoint
ENTRYPOINT ["ht-mcp-rust"]
CMD ["--help"]