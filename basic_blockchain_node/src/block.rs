use chrono::Utc;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: i64,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let (nonce, hash) = mine_block(index, timestamp, &transactions, &previous_hash);
        
        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            nonce,
            transactions,
        }
    }
}

fn mine_block(index: u32, timestamp: i64, transactions: &[Transaction], previous_hash: &str) -> (u64, String) {
    let mut nonce = 0;
    loop {
        let hash = calculate_hash(index, timestamp, transactions, previous_hash, nonce);
        if hash.starts_with("0000") {
            return (nonce, hash);
        }
        nonce += 1;
    }
}

fn calculate_hash(index: u32, timestamp: i64, transactions: &[Transaction], previous_hash: &str, nonce: u64) -> String {
    let transactions_str = serde_json::to_string(transactions).unwrap_or_default();
    let input = format!("{}{}{}{}{}", index, timestamp, transactions_str, previous_hash, nonce);
    let mut hasher = Sha256::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}
