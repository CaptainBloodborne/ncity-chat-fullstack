# Stage 1: Builder
FROM rust:1.92 AS builder

WORKDIR /app
COPY . .

# Build release version
RUN cargo build --release

# Stage 2: Runner (Use debian:slim or alpine if you compiled with musl)
FROM debian:bookworm-slim
WORKDIR /app
# Install OpenSSL/Standard libs if your app needs them
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
# Copy the binary from builder
COPY --from=builder /app/target/release/ncity-chat-network ./server
CMD ["./server"]