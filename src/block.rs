use crate::transaction::Transaction;
use super::*;
#[derive(Debug)]
pub struct Block{
    pub timestamp:u64,
    pub hash:String,
    pub pre_hash:String,
    pub transaction:Vec<Transaction>,
    // pub nonce :u64,
}

impl Block{
    pub fn new(transaction:Vec<Transaction>)->Self{
        let time = now();
        let empty_string = String::new();
        // let nonce = 0u64;
        Block {
            timestamp: time, 
            hash: empty_string.clone(),
            pre_hash: empty_string.clone(), 
            transaction: transaction, 
            // nonce:nonce,
        }
    }
    pub fn set_hash(&mut self){
        // &self.nonce
        self.hash = calculate_hash(&self.pre_hash, &self.transaction, &self.timestamp )
    }
    pub fn set_pre_hash(&mut self,pre_hash:String){
        self.pre_hash = pre_hash;
    }
    // pub fn mine(&mut self){
    //     let target = get_difficualt_string();
    //     while &self.hash[..DIFFICULT_LEVEL as usize] != target {
    //         // self.nonce += 1;
    //         // &self.nonce
    //         self.hash = calculate_hash(&self.pre_hash, &self.transaction, &self.timestamp)
    //     }
    //     println!("Block MINED");
    // }
}
