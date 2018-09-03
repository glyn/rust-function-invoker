FROM rust:1.28.0 as builder

ARG COMPONENT

WORKDIR /rust-invoker

COPY . .

RUN cargo build --release

###########

FROM debian:wheezy-slim

# The following line forces the creation of a /tmp directory
WORKDIR /tmp

WORKDIR /

COPY --from=builder /rust-invoker/target/release/rust_invoker /rust_invoker
