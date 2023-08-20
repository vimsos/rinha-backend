FROM rust:1.71.1-slim-bullseye as build

WORKDIR /source
COPY Cargo.toml .
COPY Cargo.lock .
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