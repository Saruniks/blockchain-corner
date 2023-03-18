use block::{block::Block, blockchain::Blockchain};
use chrono::Utc;
use block::hashable::Hashable;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut block = Block::new(
        0, 0, Utc::now().timestamp(), vec![0; 32], "Genesis block".to_owned(), difficulty
    );

    block.mine();
    println!("Mined genesis block {:?}", &block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block]
    };

    println!("Verify: {}", &blockchain.verify());

    for i in 1..=10 {
        let mut block = Block::new(
            i, 0, Utc::now().timestamp(), last_hash, "Another block".to_owned(), difficulty
        );
    
        block.mine();
        println!("Mined block {:?}", &block);

        last_hash = block.hash.clone();
    
        blockchain.blocks.push(block);

        println!("Verify: {}", &blockchain.verify());
    }

    blockchain.blocks[3].payload = "Nope".to_string();

    println!("Verify: {}", &blockchain.verify());
}
