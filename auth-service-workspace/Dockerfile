# Start with image that has the Rust toolchain installed
FROM rust:1.89-alpine AS chef
USER root
# Add cargo-chef to cache dependencies
RUN apk add --no-cache musl-dev & cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY ./auth-service-workspace .
# Capture info needed to build dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY ./auth-service-workspace .
RUN --mount=type=secret,id=jwt_secret JWT_SECRET=$(cat /run/jwt_secret)
WORKDIR /app/auth-service-core
RUN chmod +x define_constants.sh && ./define_constants.sh
WORKDIR /app
RUN cargo build --release --bin auth-service

# We do not need the Rust toolchain to run the binary!
# Start with a minimal image and copy over the binary and assets folder.
FROM alpine:3.22 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/auth-service /usr/local/bin
COPY --from=builder /app/auth-service/assets /app/assets
ENTRYPOINT ["/usr/local/bin/auth-service"]
