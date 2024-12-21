FROM rust:latest
WORKDIR /usr/src/blockchain
COPY . .
RUN cargo build --release
CMD ["./target/release/basic_blockchain_node"]