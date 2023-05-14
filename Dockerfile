FROM rust:1.69.0-alpine AS build

RUN mkdir /app
WORKDIR /app
COPY src .
COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo build --release

FROM alpine:3.18.0

RUN mkdir /app
WORKDIR /app
COPY --from=build /app/target/release/loaded-dice-calc-data-downloader .

CMD ["loaded-dice-calc-data-downloader"]
