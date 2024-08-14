use std::time::{SystemTime, UNIX_EPOCH};

use sha2::Digest;
use sha2::Sha256;

pub struct Block {
    block_header: BlockHeader,
    block_body: BlockBody,
}

type BlockBody = Vec<String>;

struct BlockHeader {
    // the hash is part of this struct, simply serialising this struct won't work. exclude this from others when doing calculations
    hash: String,
    // the serial number of this block. a new block is added to the blockchain with an increasing height
    height: u64,
    // the most interesting, it stores the hash eld value of the previous block
    prev_hash: String,
    // when this block was created, related to the local machine
    timestamp: u64,
}

impl Block {
    fn new(height: u64, prev_hash: String, body: BlockBody) -> Self {
        let block_header = BlockHeader::new(height, prev_hash);
        let mut block = Self {
            block_header,
            block_body: body,
        };
        block.block_header.hash = block.calc_hash();
        block
    }
    fn calc_hash(&self) -> String {
        let header = &self.block_header;
        let body = &self.block_body;

        let concated_str = vec![
            header.height.to_string(),
            header.prev_hash.to_string(),
            header.timestamp.to_string(),
            body.concat(),
        ]
        .concat();
        let mut hasher = Sha256::new();
        hasher.update(concated_str.as_bytes()); //  [u8; 32]
        hex::encode(hasher.finalize().as_slice()) // encode it into a string of length 64
    }
}

impl BlockHeader {
    fn new(height: u64, prev_hash: String) -> Self {
        BlockHeader {
            hash: "".to_string(),
            height,
            prev_hash,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

#[test]
fn test_calc_hash() {
    let block_body= vec!["hello".to_string(), "world".to_string()];
    let block = Block::new(100, "aaaaaaaaaaaaaaaa".to_string(), block_body);
        
    println!("{}", block.block_header.hash)
}
