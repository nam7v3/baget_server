FROM rust:latest

WORKDIR /usr/src/app

COPY . .

RUN chmod +x ./wait-for-it.sh

RUN cargo build --release

EXPOSE 8080

CMD ["./wait-for-it.sh", "db:5432", "--", "./target/release/baget_server"]
