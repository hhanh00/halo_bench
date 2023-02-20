use serde::{Deserialize, Serialize};

pub type Hash32 = [u8; 32];

#[derive(Serialize, Deserialize)]
pub struct Tx {
    pub anchor_height: u32,
    pub expiry_height: u32,
    pub orchard_anchor: Hash32,
    pub spends: Vec<Spend>,
    pub outputs: Vec<Output>,
}

#[derive(Serialize, Deserialize)]
pub struct Spend {
    pub id: u32,
    pub diversifier: [u8; 11],
    pub rseed: Hash32,
    pub rho: Hash32,
    pub position: u32,
    pub witness: Vec<Hash32>,
    pub amount: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub id: u32,
    pub orchard: Vec<u8>,
    pub amount: u64,
    pub memo: Vec<u8>,
}
