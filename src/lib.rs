pub mod block;
pub mod blockchain;
pub mod transaction;
use chrono::{self, Timelike};
use transaction::Transaction;

const DIFFICULT_LEVEL:i32 = 2;
pub fn now()->u64{
    u64::from(chrono::Local::now().second())
}
// nonce:&u64
pub fn calculate_hash(pre_hash:&String,transaction:&Vec<Transaction>,timestamp:&u64,)->String{
    let mut bytes = vec![];
    bytes.extend(&timestamp.to_ne_bytes());
    bytes.extend(&transaction.iter().flat_map(|transaction|
        transaction.bytes()).collect::<Vec<u8>>());
    bytes.extend(pre_hash.as_bytes());
    // bytes.extend(&nonce.to_ne_bytes());
    crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
}
pub fn get_difficualt_string() ->String{
    let mut s = String::new();
    for _i in 0..DIFFICULT_LEVEL{
        s.push_str("0")
    };
    s
}