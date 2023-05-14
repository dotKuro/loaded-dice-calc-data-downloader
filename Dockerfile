FROM rust:1.69.0 AS build

RUN mkdir /app
WORKDIR /app
COPY src ./src
COPY Cargo.lock .
COPY Cargo.toml .

RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu

FROM alpine:3.18.0

RUN mkdir /app
WORKDIR /app
COPY --from=build /app/target/release/loaded-dice-calc-data-downloader .

CMD ["./loaded-dice-calc-data-downloader"]
