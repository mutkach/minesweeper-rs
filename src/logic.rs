use std::collections::HashMap;

use crate::{level::Level, tile::Tile};

pub struct Logic {}

impl Logic {
    pub fn new() -> Self {
        Logic {}
    }

    pub fn pop_cell(self, level: &Level, start_tile: &mut Tile) -> &Level {
        let mut queue = Vec::new();
        //let mut visted = Vec::new().resize_with(level.width * level.height, || false);
        let mut visited = HashMap::new();

        queue.push(start_tile);

        while !queue.is_empty() {
            if let Some(t) = queue.pop() {
            } else {
                unreachable!("we check that queue is not empty")
            }
        }

        level
    }
}
