use std::cell::RefCell;

use super::{account::Account, map::Coordinates};

#[derive(Debug, Default)]
struct Region {
    start: Coordinates,
    end: Coordinates,
}

#[derive(Debug, Default)]
pub struct City {
    owner: RefCell<Account>,
    region: Region,
}
