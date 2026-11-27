# Build Stage
FROM rust:1.92-trixie AS builder
WORKDIR /usr/src

COPY ./Cargo.toml ./Cargo.lock ./

# Cache dependencies
RUN mkdir src \
    && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs \
    && cargo build --release --locked \
    && rm -rf src/

COPY ./src ./src
RUN touch -a -m ./src/main.rs
RUN cargo build --release --locked

FROM debian:trixie-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        git \
        openssh-client \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /repo
COPY --from=builder /usr/src/target/release/monoverse /usr/local/bin/monoverse
ENTRYPOINT ["monoverse"]
