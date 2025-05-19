# Base build stage & install system dependencies
FROM rust:latest AS base
RUN apt-get update && \
    apt-get install -y make pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .
RUN make install

# Development stage
FROM base AS dev
CMD ["make", "cargo-run-dev-watch"]

# Production build stage
FROM base AS builder
RUN cargo build --release

# Final production image
FROM debian:bookworm-slim AS prod

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/hk-smart-home-axum-auth-service .

# Copy any necessary files
COPY --from=builder /app/src/config/jwt-private.key ./src/config/jwt-private.key
COPY --from=builder /app/src/config/jwt-public.key ./src/config/jwt-public.key

CMD ["./hk-smart-home-axum-auth-service"]