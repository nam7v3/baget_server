FROM rust:latest

WORKDIR /usr/src/app

COPY . .

RUN cargo install diesel-cli

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/baget_server"]