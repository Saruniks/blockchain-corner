use std::collections::HashSet;
use crate::{Hash, transaction};
use crate::{block::{Block, check_difficulty}, hashable::Hashable};

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
} 

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![], unspent_outputs: HashSet::new() }
    }
    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr>{
        let i = self.blocks.len();
  
        if block.index != i as u32 {
            println!("Index mismatch {} != {}",
                &block.index,
                &i,
            );
            return Err(BlockValidationErr::MismatchedIndex);
        } else if !check_difficulty(&block.hash(), block.difficulty) {
            println!("Difficulty fail");
            return Err(BlockValidationErr::InvalidHash);
        } else if i != 0 {
            // Not genesis block
            let prev_block = &self.blocks[i - 1];
            if prev_block.timestamp > block.timestamp {
                println!("Timestamp verification fail");
                return Err(BlockValidationErr::AchronologicalTimestamp);
            } else if block.prev_block_hash != prev_block.hash {
                println!("Hash mismatch");
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        } else {
            // Genesis block
            if block.prev_block_hash != vec![0; 32] {
                println!("Genesis block prev_block_hash invalid");
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                println!("!coinbase.is_coinbase()");
                return Err(BlockValidationErr::InvalidCoinbaseTransaction)
            }
            let mut block_spent = HashSet::<Hash>::new();
            let mut block_created = HashSet::<Hash>::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                if !(&input_hashes - &self.unspent_outputs).is_empty() || !(&input_hashes & &block_spent).is_empty() {
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;

                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if coinbase.output_value() < total_fee {
                println!("coinbase.output_value() < total_fee");
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        self.blocks.push(block);

        Ok(())
    }
}

