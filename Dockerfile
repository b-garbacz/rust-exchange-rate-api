FROM rust:latest as builder

WORKDIR /usr/home/myapp

COPY . .

ARG API_KEY

RUN cargo build

RUN  cargo test

