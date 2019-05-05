use blockchainlib::*;

fn main() {
    let difficulty = 0x0000_0fff_ffff_ffff_ffff_ffff_ffff_ffff;

    let mut my_blockchain = Blockchain::new();

    for i in 1..=10 {
        my_blockchain.add_block(format!("this number is {}", i), difficulty);
    }
}
