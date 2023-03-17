use block::block::Block;
use chrono::Utc;
use block::hashable::Hashable;

fn main() {
    let mut block = Block::new(
        0, 0, Utc::now().timestamp(), vec![0; 32], "Genesis block".to_owned()
    );
    
    println!("{:?}", block);

    let h = block.hash();

    println!("{:?}", &h);

    block.hash = h;
    
    println!("{:?}", &block);
}
