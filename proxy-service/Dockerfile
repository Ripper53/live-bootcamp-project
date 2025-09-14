FROM rust:1.89-alpine AS chef
USER root
RUN apk update; apk add --no-cache build-base clang cmake g++ musl-dev && cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY ./proxy-service .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./proxy-service .
RUN chmod +x define_constants.sh && ./define_constants.sh
RUN cargo build --release

FROM alpine:3.22 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/proxy-service /usr/local/bin
ENTRYPOINT ["/usr/local/bin/proxy-service"]
