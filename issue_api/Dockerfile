FROM rust:latest AS builder

COPY ./auth /auth
COPY ./board_api /board_api

RUN USER=root cargo new --bin issue_api
WORKDIR /issue_api

COPY ./Cargo.lock ./Cargo.lock
COPY ./issue_api/Cargo.toml ./Cargo.toml

RUN cargo build --bin issue_api --release 
RUN rm src/*.rs

COPY ./issue_api/src ./src
COPY .env .

RUN rm ./target/release/deps/issue_api*
RUN cargo install --path .

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /issue_api/target/release/issue_api /usr/local/bin/issue_api
COPY --from=builder /issue_api/.env /app/.env

EXPOSE 3003
CMD ["issue_api"]