#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub explored: bool,
    pub breakable: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile { blocked: false, block_sight: false, explored: false, breakable: false}
    }

    pub fn wall() -> Self {
        Tile { blocked: true, block_sight: true, explored: false, breakable: false}
    }

    pub fn breakable_wall() -> Self {
        Tile { blocked: true, block_sight: true, explored: false, breakable: true}
    }
}
