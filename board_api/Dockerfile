FROM rust:latest AS builder

COPY ./auth /auth

RUN USER=root cargo new --bin board_api
WORKDIR /board_api

COPY ./Cargo.lock ./Cargo.lock
COPY ./board_api/Cargo.toml ./Cargo.toml

RUN cargo build --bin board_api --release 
RUN rm src/*.rs

COPY ./board_api/src ./src
COPY .env .

RUN rm ./target/release/deps/board_api*
RUN cargo install --path .

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /board_api/target/release/board_api /usr/local/bin/board_api
COPY --from=builder /board_api/.env /app/.env

EXPOSE 3002
CMD ["board_api"]