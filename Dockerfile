FROM rust:1.80-slim-bookworm
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
CMD cargo run --quiet
