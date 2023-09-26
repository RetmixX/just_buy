FROM rust:latest as build
RUN apt-get update && apt-get install -y \
    libpq-dev

WORKDIR /project

COPY . .
COPY ./.build/.env .env
RUN cargo build --release

CMD ["./target/release/just_buy"]
