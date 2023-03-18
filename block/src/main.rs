use block::block::Block;
use chrono::Utc;
use block::hashable::Hashable;

fn main() {
    let mut block = Block::new(
        0, 1266122, Utc::now().timestamp(), vec![0; 32], "Genesis block".to_owned(), 0x00000fffffffffffffffffffffffffff
    );

    block.hash = block.hash();
    
    println!("{:?}", &block);

    block.mine();

    println!("{:?}", &block);
}
