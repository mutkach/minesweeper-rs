use macroquad::math::Vec2;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileState {
    Hidden,
    Free,
    Marked,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub state: TileState,
    pub bomb: bool,
    pub number: usize,
}

impl Tile {
    pub fn new_default(x: usize, y: usize) -> Self {
        Tile {
            x,
            y,
            state: TileState::Hidden,
            bomb: false,
            number: 0,
        }
    }

    pub fn new(x: usize, y: usize, bomb: bool) -> Self {
        Tile {
            x,
            y,
            state: TileState::Hidden,
            bomb,
            number: 0,
        }
    }
}
