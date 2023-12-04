use super::block::Block;

pub struct Chain {
    pub current_block: Block,
}


impl Chain {
    pub fn new() -> Chain {
        Chain {
            current_block: {
                Block {
                    index: 0,
                    timestamp: None,
                    prev_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                    hash: None,
                    proof: None,
                    transactions: vec![],
                }
            },
        }
    }
}