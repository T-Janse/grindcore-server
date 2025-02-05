FROM rust:1.80-slim-bookworm
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
EXPOSE 5000
CMD cargo run --quiet
