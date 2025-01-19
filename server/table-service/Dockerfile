FROM rust:latest

WORKDIR /usr/src/app

COPY Cargo.toml ./
COPY Cargo.lock* ./

COPY src ./src

RUN cargo build

EXPOSE 8080

CMD ["cargo", "run"]
