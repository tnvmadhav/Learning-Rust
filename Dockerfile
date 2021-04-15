FROM rust:1.31

WORKDIR /
COPY . .

RUN cargo install --path .
RUN cargo build

WORKDIR /target/debug/

CMD ["Learning-Rust"]