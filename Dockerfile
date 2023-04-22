FROM rust:slim as base
ENV TARGET x86_64-unknown-linux-musl

RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add "$TARGET"


FROM base as builder
WORKDIR /build/

COPY ./ /build/
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/build/target \
    cargo build --release --target x86_64-unknown-linux-musl

RUN ls -la /build/target


FROM scratch

COPY --from=builder \
    /build/target/x86_64-unknown-linux-musl/release/kitten_dispenser\
    /kitten_dispenser

ENTRYPOINT ["/kitten_dispenser"]
