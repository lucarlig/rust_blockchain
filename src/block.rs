use super::*;

#[derive(Debug)]
pub struct Block {
    pub index: usize,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
    pub difficulty: u128,
}

impl Block {
    pub fn new(
        index: usize,
        timestamp: u128,
        prev_block_hash: BlockHash,
        nonce: u64,
        payload: String,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_1() {
        let block = Block::new(
            0,
            0,
            vec![0; 32],
            0,
            "Genesis block!".to_owned(),
            0x0000_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
        );

        assert_eq!(block.index, 0);
        assert_eq!(block.timestamp, 0);
        assert_eq!(block.hash, vec![0; 32]);
        assert_eq!(block.prev_block_hash, vec![0; 32]);
        assert_eq!(block.nonce, 0);
        assert_eq!(block.payload, "Genesis block!".to_owned());
        assert_eq!(block.difficulty, 0x0000_ffff_ffff_ffff_ffff_ffff_ffff_ffff);
    }
    #[test]
    fn test_bytes_1() {
        let block = Block::new(
            0,
            0,
            vec![0; 32],
            0,
            "Genesis block!".to_owned(),
            0x0000_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
        );
        let bytes = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 71, 101, 110, 101, 115, 105, 115, 32, 98, 108, 111, 99, 107, 33, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0,
        ];
        assert_eq!(block.bytes(), bytes);
    }
    #[test]
    fn test_hash_1() {
        let block = Block::new(
            0,
            0,
            vec![0; 32],
            0,
            "Genesis block!".to_owned(),
            0x0000_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
        );
        let hash = vec![
            190, 132, 42, 149, 178, 111, 155, 240, 196, 99, 49, 131, 135, 132, 80, 111, 224, 96,
            79, 16, 46, 203, 164, 7, 120, 8, 152, 105, 36, 79, 233, 155,
        ];
        assert_eq!(hash, block.hash());
    }
    #[test]
    fn test_hash_2() {
        let block = Block::new(
            0,
            0,
            vec![0; 32],
            118318,
            "Genesis block!".to_owned(),
            0x0000_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
        );
        let hash = vec![
            219, 233, 198, 114, 253, 246, 157, 102, 149, 137, 197, 108, 63, 173, 227, 163, 55, 228,
            179, 140, 84, 141, 118, 251, 118, 20, 47, 19, 235, 65, 0, 0,
        ];
        assert_eq!(hash, block.hash());
    }
    #[test]
    fn test_mine_1() {
        let mut block = Block::new(
            0,
            0,
            vec![0; 32],
            0,
            "Genesis block!".to_owned(),
            0x0000_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
        );
        let hash_mined = [
            219, 233, 198, 114, 253, 246, 157, 102, 149, 137, 197, 108, 63, 173, 227, 163, 55, 228,
            179, 140, 84, 141, 118, 251, 118, 20, 47, 19, 235, 65, 0, 0,
        ];
        block.hash = block.hash();
        block.mine();
        assert_eq!(block.hash, hash_mined);
    }
    #[test]
    fn test_mine_2() {
        let mut block = Block::new(
            0,
            0,
            vec![0; 32],
            118318,
            "Genesis block!".to_owned(),
            0x0000_ffff_ffff_ffff_ffff_ffff_ffff_ffff,
        );
        let hash_mined = [
            219, 233, 198, 114, 253, 246, 157, 102, 149, 137, 197, 108, 63, 173, 227, 163, 55, 228,
            179, 140, 84, 141, 118, 251, 118, 20, 47, 19, 235, 65, 0, 0,
        ];
        block.hash = block.hash();
        block.mine();
        assert_eq!(block.hash, hash_mined);
    }
}