FROM rust:latest AS builder
RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /api
COPY ./Cargo.lock Cargo.lock
COPY ./Cargo.toml Cargo.toml
COPY ./src src
COPY ./migrations migrations
RUN cargo build --release
RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /api
COPY --from=builder /api/target/release/api ./

EXPOSE 8080
CMD ["./api"]
