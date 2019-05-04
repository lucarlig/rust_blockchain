use super::*;

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
    pub difficulty: u128,
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: BlockHash,
        nonce: u64,
        payload: String,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0, 32],
            prev_block_hash,
            nonce,
            payload,
            difficulty,
        }
    }
    pub fn mine(&mut self) {
        for nonce_attempt in 0..u64::max_value() {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&self.index.to_le_bytes());
        bytes.extend(&self.timestamp.to_le_bytes());
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&self.nonce.to_le_bytes());
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&self.difficulty.to_le_bytes());
        bytes
    }
}

pub fn check_difficulty(hash: &BlockHash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}
