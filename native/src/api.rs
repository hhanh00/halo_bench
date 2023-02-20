use crate::build_tx::build_tx;
use crate::tx_types::Tx;
use crate::SEED;
use lazy_static::lazy_static;
use orchard::circuit::{ProvingKey, VerifyingKey};

pub const TXDESC_BYTES: &[u8] = include_bytes!("../tx.bin");
pub const TX_BYTES: &[u8] = include_bytes!("../out.bin");

lazy_static! {
    static ref PK: ProvingKey = ProvingKey::build();
    static ref VK: VerifyingKey = VerifyingKey::build();
    static ref TXDESC: Tx = bincode::deserialize(TXDESC_BYTES).unwrap();
}

pub const TX_SIZE: usize = 9165;

pub fn test_from_seed(seed: u32) -> bool {
    let res = build_tx(&TXDESC, SEED, seed, &PK);
    if seed < 20 {
        let offset = seed as usize * TX_SIZE;
        let expected = &TX_BYTES[offset..offset + TX_SIZE];
        return expected == &*res;
    }
    true
}
