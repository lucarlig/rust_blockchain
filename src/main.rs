use blockchainlib::*;

fn main() {
    let mut block = Block::new(
        0,
        0,
        vec![0; 32],
        0,
        "Genesis block!".to_owned(),
        0x0000_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
    );

    block.hash = block.hash();

    println!("{:x?}", &block);

    block.mine();

    println!("{:x?}", &block);
}
