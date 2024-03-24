FROM rust:latest as builder

WORKDIR /usr/home/rust-exchange-rate-api

COPY . .

RUN cargo build --release

WORKDIR /usr/home/rust-exchange-rate-api/target/release/

RUN chmod +x exchange_rate_api

ENTRYPOINT ["./exchange_rate_api"]
