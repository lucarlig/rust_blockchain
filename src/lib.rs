mod block;
pub use block::Block;
mod hashable;
pub use hashable::Hashable;

type BlockHash = Vec<u8>;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error getting time since unix")
        .as_millis()
}

pub fn difficulty_bytes_as_u128(v: &[u8]) -> u128 {
    let mut array = [0; 16];
    array.copy_from_slice(&v[16..32]);
    u128::from_le_bytes(array)
}
