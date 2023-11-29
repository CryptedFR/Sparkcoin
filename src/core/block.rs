struct Hash([u8; 32]);

pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub hash: Hash,
    pub prev_hash: Hash,
    pub proof: u64,
    pub transactions: Vec<Transaction>,
}