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


#[cfg(test)]
mod tests {
        use super::*;
        use std::{thread, time};

        #[test]
        fn test_difficulty_bytes_as_u128_1() {
                assert_eq!(0, difficulty_bytes_as_u128(&[0; 32]));
                assert_eq!(
                        6672203272959577714968129557485651205,
                        difficulty_bytes_as_u128(&[5; 32])
                );
                assert_eq!(
                        17347728509694902058917136849462693133,
                        difficulty_bytes_as_u128(&[13; 32])
                );

        }
        #[test]
        fn test_difficulty_bytes_as_u128_2() {
                assert_eq!(
                        6672203272959577714968129557485651205,
                        difficulty_bytes_as_u128(&[5; 32])
                );
                assert_eq!(
                        17347728509694902058917136849462693133,
                        difficulty_bytes_as_u128(&[13; 32])
                );

        }
        #[test]
        fn test_difficulty_bytes_as_u128_3() {
                assert_eq!(
                        17347728509694902058917136849462693133,
                        difficulty_bytes_as_u128(&[13; 32])
                );

        }
        #[test]
        fn test_difficulty_bytes_as_u128_4() {
                assert_eq!(
                        340282366920938463463374607431768211455,
                        difficulty_bytes_as_u128(&[std::u8::MAX; 32])
                );

        }
        #[test]
        fn test_now_1() {
                assert!(now() > 1557022292388);
        }
        #[test]
        fn test_now_21() {
                let now1 = now();
                thread::sleep(time::Duration::from_millis(1));
                let now2 = now();
                assert!(now2 > now1);
        }
}