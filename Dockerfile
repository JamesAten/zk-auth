FROM rust:1.70

WORKDIR /zk-auth

COPY . .

RUN apt update
RUN apt install -y protobuf-compiler

RUN cargo build --release --bin server --bin client