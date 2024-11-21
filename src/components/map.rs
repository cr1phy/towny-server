use borsh::BorshSerialize;
use derive_more::derive::{Add, Div, Mul, Sub};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Add, Sub, Mul, Div, Serialize, Deserialize)]
pub struct Coordinates(u64, u64);

#[derive(Debug, Default, BorshSerialize)]
pub struct Relief {}

#[derive(Debug, Default, BorshSerialize)]
pub struct Map {
    seed: u64,
    relief: Relief,
}

impl Map {
    pub fn generate() -> Self {
        let seed: u64 = rand::random();
        Self {
            seed,
            ..Default::default()
        }
    }
}
