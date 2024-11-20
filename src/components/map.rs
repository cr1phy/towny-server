use borsh::BorshSerialize;

#[derive(Debug)]
pub struct Coordinates {}

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
