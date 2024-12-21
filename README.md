To develop a blockchain in Rust and create a testnet with two nodes posting transactions, follow these steps:

1. Set up the Rust development environment:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.bashrc
rustc --version
```

2. Create a new Rust project:
```bash
cargo new blockchain_project
cd blockchain_project
```

3. Add necessary dependencies to your `Cargo.toml`:
```toml
[dependencies]
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sha2 = "0.10.0"
```

4. Implement the core blockchain structure:
- Create `src/block.rs` and `src/blockchain.rs` files
- Implement `Block` and `Blockchain` structs with basic functionality

5. Develop key blockchain components:
- Genesis block creation
- Block mining
- Transaction handling
- Consensus mechanism (e.g., Proof of Work)

6. Create a simple API for interacting with the blockchain:
- Add endpoints for submitting transactions and querying the chain

7. Dockerize your blockchain application:
- Create a `Dockerfile` in your project root:
```dockerfile
FROM rust:latest
WORKDIR /usr/src/blockchain
COPY . .
RUN cargo build --release
CMD ["./target/release/blockchain_project"]
```

8. Build and run Docker containers for two nodes:
```bash
docker build -t blockchain-node .
docker network create blockchain-network
docker run -d --name node1 --network blockchain-network -p 8000:8000 blockchain-node
docker run -d --name node2 --network blockchain-network -p 8001:8000 blockchain-node
```

9. Implement inter-node communication:
- Use a library like `libp2p` for peer-to-peer networking
- Add functionality for nodes to discover each other and sync the blockchain

10. Create a script to post transactions to the nodes:
```rust
use reqwest;

fn main() {
    let transaction = json!({
        "sender": "Alice",
        "recipient": "Bob",
        "amount": 10
    });

    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8000/transaction")
        .json(&transaction)
        .send()
        .await?;

    println!("Transaction sent: {:?}", res.text().await?);
}
```

This setup creates a basic blockchain in Rust with two Docker nodes that can accept transactions. The nodes communicate over a Docker network, simulating a testnet environment[1][4][7].

Citations:
[1] https://dev.to/ecj222/how-to-build-a-blockchain-from-scratch-in-rust-46
[2] https://www.youtube.com/watch?v=4qBM-Ou5Ohc
[3] https://docs.bnbchain.org/bnb-smart-chain/developers/node_operators/docker/
[4] https://www.rapidinnovation.io/post/how-to-build-a-blockchain-with-rust
[5] https://hackernoon.com/building-a-blockchain-in-rust-and-substrate-a-step-by-step-guide-for-developers-kc223ybp
[6] https://github.com/zquestz/docker-bitcoin
[7] https://github.com/ddimaria/rust-blockchain-tutorial
[8] https://docs.scrt.network/secret-network-documentation/development/readme-1/interacting-with-the-testnet
[9] https://www.skeps.com/blog/deploying-blockchain-applications-with-docker
[10] https://webisoft.com/articles/rust-blockchain/
[11] https://www.youtube.com/watch?v=1oJrLNKSVf8


#### manual build
cargo build
cargo run