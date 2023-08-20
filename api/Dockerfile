FROM rustlang/rust:nightly-bookworm-slim as build

WORKDIR /source
COPY Cargo.toml .
COPY Cargo.lock .
COPY rust-toolchain.toml .
COPY .cargo/config.Dockerfile.toml .cargo/config.toml
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY .sqlx .sqlx
COPY src src
RUN touch src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /source/target/release/rinha-vimsos .
ENTRYPOINT ./rinha-vimsos 