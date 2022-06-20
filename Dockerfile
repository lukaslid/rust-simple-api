FROM rust:1.49-slim-buster

RUN USER=root cargo new api
WORKDIR /api

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/api*
RUN cargo install --path .

CMD ["api"]