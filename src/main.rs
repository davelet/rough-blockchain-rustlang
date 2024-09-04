use block::Block;
use block_chain::BlockChian;
use redb::{Database, ReadableTable, TableDefinition};

mod block;
mod block_chain;

fn main() {
    println!("Hello, world!");
    let block_body= vec!["hello".to_string(), "world".to_string()];
    let block = Block::new(100, "aaaaaaaaaaaaaaaa".to_string(), block_body);
        
    println!("{}", block.block_header.hash);

    let mut block_chain = BlockChian::new();
    let table = TableDefinition::new("my_chain");

    let genesis = BlockChian::genesis();
    let pre_hash = genesis.block_header.hash.clone();
    // block_chain.add_block(genesis);
    block_chain.persist(table, genesis);

    let block = Block::new(1, pre_hash, vec![]);
    let pre_hash = block.block_header.hash.clone();
    // block_chain.add_block(block);
    block_chain.persist(table, block);

    let block = Block::new(2, pre_hash, vec![]);
    let pre_hash = block.block_header.hash.clone();
    // block_chain.add_block(block);
    block_chain.persist(table, block);

    let block = Block::new(3, pre_hash, vec![]);
    // block_chain.add_block(block);
    block_chain.persist(table, block);

    println!("{:?}", block_chain);
    println!("{:?}", block_chain.get_block_by_hash(table, "123"));
    let restored = block_chain.restore_chain(table);
    println!("restore: {restored} = {:?}", block_chain);

    // let db = Database::create("db.redb").unwrap();
    // let ctx = db.begin_read().unwrap();
    // let table = ctx.open_table(table).unwrap();
    // let a = table.range("0"..);


}
