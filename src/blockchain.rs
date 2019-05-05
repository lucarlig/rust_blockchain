use super::*;
pub struct Blockchain {
    pub blocks: Vec<Block>,
    index: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: Vec::new(),
            index: 0,
        }
    }

    pub fn add_block(&mut self, payload: String, difficulty: u128) {
        if self.blocks.len() == 0 {
            let prev_block_hash = vec![0; 32];
            self.create_block(prev_block_hash, payload, difficulty);
        } else {
            let prev_block_hash = self.blocks[self.index].hash.clone();
            self.index = self.index + 1;
            self.create_block(prev_block_hash, payload, difficulty);
        }
    }

    fn create_block(&mut self, prev_block_hash: Vec<u8>, payload: String, difficulty: u128) {
        self.blocks.push(Block::new(
            self.index,
            now(),
            prev_block_hash,
            0,
            payload,
            difficulty,
        ));
        self.blocks[self.index].mine();
        println!("{:x?}", self.blocks[self.index]);
        println!("Verify: {}", &self.verify());
    }

    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i {
                println!("Index mismatch {} !=  {}", &block.index, &i,);
                return false;
            } else if !block::check_difficulty(&block.hash(), block.difficulty) {
                println!("Difficulty fail");
                return false;
            } else if i != 0 {
                // not genesis
                let prev_block = &self.blocks[i - 1];
                if block.timestamp <= prev_block.timestamp {
                    println!("Time did not increase");
                    return false;
                } else if block.prev_block_hash != prev_block.hash {
                    println!("Hash mismatch");
                }
            } else {
                // genesis
                if block.prev_block_hash != vec![0; 32] {
                    println!("Genesis block prev_block_hash invalid");
                    return false;
                }
            }
        }
        true
    }
}

