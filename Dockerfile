# Build Stage
FROM rust:1.85-alpine AS builder
WORKDIR /usr/src/
# Install required build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev libc-dev gcc

# - Install dependencies
RUN USER=root cargo new backup-buddy
WORKDIR /usr/src/backup-buddy
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# - Copy source
COPY migrations ./migrations
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Runtime Stage
FROM alpine:latest AS runtime
WORKDIR /app
# Install runtime dependencies if needed
# RUN apk add --no-cache ca-certificates

COPY --from=builder /usr/src/backup-buddy/target/release/backup-buddy ./backup-buddy
USER 1000
CMD ["./backup-buddy"]
