FROM rust:latest

WORKDIR /todo_web

# this build step will cache your dependencies
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release

# build for release
COPY ./src ./src
COPY ./templates ./templates
RUN rm ./target/release/deps/todo_web*
RUN cargo build --release

# run the binary
RUN cargo install --path .
CMD ["todo_web"]
