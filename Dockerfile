FROM rust:1.80.0-alpine3.20

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/actix-connection-info-test"]
