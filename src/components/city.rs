use super::map::Coordinates;

struct Region {
    start: Coordinates,
    end: Coordinates,
}

pub struct City {
    region: Region,
}
