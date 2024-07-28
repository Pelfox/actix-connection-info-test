FROM rust:1.80.0-slim-bullseye

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/actix-connection-info-test"]
