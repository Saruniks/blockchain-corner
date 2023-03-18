use block::blockchain::BlockValidationErr;
use block::{block::Block, blockchain::Blockchain};
use block::{transaction::{Transaction, Output}};
use chrono::Utc;
use block::hashable::Hashable;

fn main() -> Result<(), BlockValidationErr> {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut genesis_block = Block::new(
        0, Utc::now().timestamp(), vec![0; 32], vec![
            Transaction { 
                inputs: vec![], outputs: vec![
                    Output {
                        to_addr: "Alice".to_string(),
                        value: 50,
                    },
                    Output {
                        to_addr: "Bob".to_string(),
                        value: 7,
                    }
                ] }
        ], difficulty
    );

    genesis_block.mine();
    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block)?;

    let mut block = Block::new(
        1, Utc::now().timestamp(), last_hash, vec![
            Transaction { 
                inputs: vec![
                    blockchain.blocks[0].transactions[0].outputs[0].clone(),
                ], outputs: vec![
                    Output {
                        to_addr: "Nobody".to_string(),
                        value: 36,
                    },
                    Output {
                        to_addr: "Bob".to_string(),
                        value: 12,
                    }
                ] }
        ], difficulty
    );

    block.mine();
    println!("Mined block {:?}", &block);

    blockchain.update_with_block(block)?;
    Ok(())
}
