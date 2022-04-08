use vblockchain::*;
use vblockchain::transaction::Transaction;
fn main() {
    let mut blockchain = blockchain::Blockchain::new();
    let genesis_block = block::Block::new(vec![Transaction{
        sender:String::from("AmirMahdi"),
        receiver:String::from("Mahdi"),
        amount:10.0
    }]);

    blockchain.add_block(genesis_block);
    println!("{:#?}",blockchain);
}
