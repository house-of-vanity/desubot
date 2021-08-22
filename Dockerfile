# syntax=docker/dockerfile:1

FROM rust:latest AS builder
WORKDIR /desubot
ADD ./ /desubot/
RUN cargo build --release

FROM ubuntu:latest
WORKDIR /storage
COPY --from=builder /desubot/target/release/desubot /usr/bin/
COPY mystem /usr/bin/
RUN apt update && apt install -y fontconfig openssl ca-certificates && rm -rf /var/lib/apt/lists/*
ENTRYPOINT desubot

