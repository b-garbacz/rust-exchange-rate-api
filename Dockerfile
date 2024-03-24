FROM rust:latest as builder

WORKDIR /usr/home/rust-exchange-rate-api

COPY . .

RUN cargo build --release
