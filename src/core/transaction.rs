use serde::Serialize;

#[derive(Serialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub fee: f64,
    pub signature: String,
}

impl Transaction {
    pub fn verify_signature(&self) -> bool {
        true
    }
}