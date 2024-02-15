FROM rust:latest as builder

WORKDIR /var/build
COPY ./friday ./friday
WORKDIR /var/build/friday

RUN cargo build --release


FROM debian:bookworm

WORKDIR ./usr/src/app
EXPOSE 5000

RUN apt-get update
RUN apt-get -y install openssl libssl-dev

COPY --from=builder /var/build/friday/target/release/friday ./friday
CMD ["./friday"]
