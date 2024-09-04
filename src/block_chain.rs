use crate::block::Block;
use redb::{Database as Db, TableDefinition};
use serde_json;

#[derive(Debug)]
pub struct BlockChian{
    blocks: Vec<Block>,
    db: Db,
}

impl BlockChian {
    pub fn new() -> Self {
        BlockChian {
            blocks: vec![],
            db: Db::create("./db.redb").unwrap(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn genesis() -> Block {
        let body = vec!["from era".to_string()];
        Block::new(0, "0".to_string(), body)
    }

    pub fn persist(&self, table: TableDefinition<&str, &str>, block: Block) {
        // persist to disk
        let ctx = self.db.begin_write().unwrap();
        {
            let mut table = ctx.open_table(table).unwrap();
            let value = serde_json::to_string(&block).unwrap();
            let hash = &*block.block_header.hash;
            table.insert(hash, &*value).unwrap();
            table.insert(&*block.block_header.height.to_string(), hash).unwrap();
            table.insert("LBP", hash).unwrap();
        }
        ctx.commit().unwrap();
    }

    pub fn get_block_by_hash(&self, table: TableDefinition<&str, &str>, hash: &str) -> Option<Block> {
        let ctx = self.db.begin_read().unwrap();
        let table = ctx.open_table(table).unwrap();
        let v = table.get(hash).unwrap();
        match v {
            Some(v) => {
                let block: Block = serde_json::from_str(v.value()).unwrap();
                return Some(block);
            }
            None => return None,
        }
    }

    pub fn restore_chain(&mut self, table: TableDefinition<&str, &str>) -> bool {
        let ctx = self.db.begin_read().unwrap();
        let table = ctx.open_table(table).unwrap();
        let lbp = table.get("LBP").unwrap();
        if lbp.is_none() {
            return false;
        }
        let lbp = lbp.unwrap();
        let lb = table.get(lbp.value()).unwrap().unwrap();
        let mut block: Block = serde_json::from_str(lb.value()).unwrap();
        // println!("restore block:{:?}", block);
        let mut h = block.block_header.height;
        let mut pre_hash = block.block_header.prev_hash.clone();
        self.blocks.insert(0, block);
        while h > 0 {
            block = serde_json::from_str(table.get(&*pre_hash).unwrap().unwrap().value()).unwrap();
            pre_hash = block.block_header.prev_hash.clone();
            h = block.block_header.height;
            self.blocks.insert(0, block);
        }

        true
    }
}

#[test]
fn db() {
    let db = Db::create("./db.redb").unwrap();
    let table = TableDefinition::<&str, &str>::new("table");
    let ctx = db.begin_write().unwrap();
    {
        let mut table = ctx.open_table(table).unwrap();
        table.insert("key", "value").unwrap();

        let mut table = ctx.open_table(TableDefinition::<&str, i32>::new("data")).unwrap();
        table.insert("key",  123).unwrap();
         
    }
    ctx.commit().unwrap();

    let ctx = db.begin_read().unwrap();
    let table = ctx.open_table(table).unwrap();
    let v = table.get("key").unwrap().unwrap();
    println!("table :{:?}", v.value());
    assert!(v.value() == "value");

    let table = ctx.open_table(TableDefinition::<&str, i32>::new("data")).unwrap();
    let v = table.get("key").unwrap().unwrap();
    println!("data :{:?}", v.value());
    assert!(v.value() == 123)
}