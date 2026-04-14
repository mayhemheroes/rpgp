# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly AS builder

RUN cargo install cargo-fuzz

ADD . /src
WORKDIR /src

RUN cargo +nightly fuzz build && \
    mv fuzz/target/x86_64-unknown-linux-gnu/release/packet_parsing /packet_parsing

RUN RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" \
    cargo +nightly build --release --manifest-path fuzz/Cargo.toml && \
    mv fuzz/target/release/packet_parsing /packet_parsing_no_inst

# Package Stage
FROM ubuntu:22.04

COPY --from=builder /packet_parsing /packet_parsing_no_inst /
