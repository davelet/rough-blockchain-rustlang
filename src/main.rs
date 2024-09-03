use block::Block;
use block_chain::BlockChian;

mod block;
mod block_chain;

fn main() {
    println!("Hello, world!");
    let block_body= vec!["hello".to_string(), "world".to_string()];
    let block = Block::new(100, "aaaaaaaaaaaaaaaa".to_string(), block_body);
        
    println!("{}", block.block_header.hash);

    let mut block_chain = BlockChian::new();

    let genesis = BlockChian::genesis();
    let pre_hash = genesis.block_header.hash.clone();
    block_chain.add_block(genesis);

    let block = Block::new(1, pre_hash, vec![]);
    let pre_hash = block.block_header.hash.clone();
    block_chain.add_block(block);

    let block = Block::new(2, pre_hash, vec![]);
    let pre_hash = block.block_header.hash.clone();
    block_chain.add_block(block);

    let block = Block::new(3, pre_hash, vec![]);
    block_chain.add_block(block);

    println!("{:?}", block_chain)
}
