use sha2::Sha256;
use sha2::Digest;

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
    fn calc_hash(&self) -> String {
        let header = &self.block_header;
        let body = &self.block_body;

        let concated_str = vec![header.height.to_string(),
        header.prev_hash.to_string(),
        header.timestamp.to_string(),
        body.concat()].concat();
        let mut hasher = Sha256::new();
        hasher.update(concated_str.as_bytes());//  [u8; 32]
        hex::encode(hasher.finalize().as_slice()) // encode it into a string of length 64
    }
}

#[test]
fn test_calc_hash() {
    let block = Block {
        block_header: BlockHeader {
            hash: "".to_string(),
            height: 0,
            prev_hash: "".to_string(),
            timestamp: 0,
        },
        block_body: vec!["hello".to_string(), "world".to_string()],
    };
    println!("{}", block.calc_hash())
}