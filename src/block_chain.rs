use crate::block::Block;

#[derive(Debug)]
pub struct BlockChian{
    blocks: Vec<Block>,
}

impl BlockChian {
    pub fn new() -> Self {
        BlockChian {
            blocks: vec![],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn genesis() -> Block {
        let body = vec!["from era".to_string()];
        Block::new(0, "0".to_string(), body)
    }
}