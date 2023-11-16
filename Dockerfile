FROM rust:latest as builder

WORKDIR /builder

RUN apt-get update && \
    apt-get install -y libclang-dev clang

COPY . /builder

RUN cargo build --release

FROM ubuntu:latest
WORKDIR /app

COPY ./upload upload
COPY ./Rocket.toml Rocket.toml
COPY ./static static
COPY --from=builder /builder/target/release/giftaku_api .

EXPOSE 8000

CMD ["./giftaku_api"]