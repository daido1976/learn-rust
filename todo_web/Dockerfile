FROM rust:latest

WORKDIR /todo_web

COPY Cargo.toml Cargo.toml
COPY ./src ./src
COPY ./templates ./templates

RUN cargo build --release

RUN cargo install --path .

CMD ["todo_web"]
