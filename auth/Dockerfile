FROM rust:latest AS builder

RUN USER=root cargo new --bin auth
WORKDIR /auth

COPY ./Cargo.lock ./Cargo.lock
COPY ./auth/Cargo.toml ./Cargo.toml

RUN cargo build --bin auth --release 
RUN rm src/*.rs

COPY ./auth/src ./src
COPY .env .

RUN rm ./target/release/deps/auth*
RUN cargo install --path .

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /auth/target/release/auth /usr/local/bin/auth
COPY --from=builder /auth/.env /app/.env

EXPOSE 3001
CMD ["auth"]