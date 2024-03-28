# Build Stage
FROM rust:1.76 as builder
WORKDIR /usr/src

COPY ./Cargo.toml ./Cargo.toml

# Cache dependencies
RUN mkdir src \
    && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs \
    && cargo build --release \
    && rm -rf src/

COPY ./src ./src
RUN touch -a -m ./src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev git
WORKDIR /repo
COPY --from=builder /usr/src/target/release/monoverse /usr/local/bin/monoverse
ENTRYPOINT ["monoverse"]
