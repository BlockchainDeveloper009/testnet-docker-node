use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, String::from("Genesis Block"), String::from("0"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            transactions,
            previous_block.hash.clone(),
        );
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate().skip(1) {
            let previous_block = &self.chain[i - 1];
            if block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}