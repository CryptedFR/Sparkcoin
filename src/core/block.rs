use crate::core::Transaction;
use serde::Serialize;

#[derive(Serialize)]
pub struct Hash([u8; 32]);
#[derive(Serialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: Option<u64>,
    pub prev_hash: String,
    pub hash: Option<Hash>,
    pub proof: Option<u64>,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn add_transaction(&self, transaction: Transaction) {
        if !transaction.verify_signature() {
            panic!("Invalid signature");
        }
    }
}