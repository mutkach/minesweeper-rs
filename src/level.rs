use std::{
    collections::{HashMap, HashSet, LinkedList},
    error::Error,
    fmt::Debug,
    mem::swap,
};

use crate::tile::{self, Tile, TileState};
use crate::utils::random_sample;
use macroquad::rand::{self, RandomRange};

const TILE_SIZE: u8 = 16;

pub struct Level {
    pub grid: Vec<Tile>,
    pub width: usize,
    pub height: usize,
    n_bombs: usize,
    n_opened: usize,
    marked: HashSet<usize>,
}

impl Level {
    pub fn new(width: usize, height: usize, n_bombs: usize) -> Self {
        let mut tiles = Vec::new();
        for i in 0..height {
            for j in 0..width {
                let tile = Tile::new_default(i, j);
                tiles.push(tile);
            }
        }

        for pos in random_sample(width * height, n_bombs) {
            tiles[pos].bomb = true;
        }

        Level {
            grid: tiles,
            width,
            height,
            n_bombs,
            n_opened: 0,
            marked: HashSet::new(),
        }
    }

    pub fn get_neighbors(&self, idx: usize) -> Vec<usize> {
        let mut neighbors = Vec::<usize>::new();
        let xy_to_idx = |x: usize, y: usize| y * self.width + x;

        let x = idx % self.width;
        let y = idx / self.width;

        let lbx = x == 0;
        let lby = y == 0;
        let ubx = x == self.width - 1;
        let uby = y == self.height - 1;

        if !lbx {
            neighbors.push(xy_to_idx(x - 1, y));
        }

        if !ubx {
            neighbors.push(xy_to_idx(x + 1, y));
        }

        if !lby {
            neighbors.push(xy_to_idx(x, y - 1));
        }

        if !uby {
            neighbors.push(xy_to_idx(x, y + 1));
        }

        if !ubx && !uby {
            neighbors.push(xy_to_idx(x + 1, y + 1));
        }
        if !ubx && !lby {
            neighbors.push(xy_to_idx(x + 1, y - 1));
        }
        if !lbx && !lby {
            neighbors.push(xy_to_idx(x - 1, y - 1));
        }
        if !lbx && !uby {
            neighbors.push(xy_to_idx(x - 1, y + 1));
        }
        neighbors
    }

    pub fn count_bombs(&self, idx: usize) -> usize {
        self.get_neighbors(idx)
            .iter()
            .map(|&x| self.grid[x].bomb)
            .filter(|&x| x == true)
            .count()
    }

    pub fn check_bomb(&self, idx: usize) -> bool {
        self.grid[idx].bomb
    }

    pub fn bfs_cells(&mut self, start_idx: usize) {
        if self.grid[start_idx].state == TileState::Free {
            return;
        }
        let mut queue = Vec::new();
        let mut visited = HashSet::<usize>::new();
        queue.push(start_idx);
        visited.insert(start_idx);

        while !queue.is_empty() {
            let current = queue.pop().expect("queue can't be empty");
            let count_bombs = self.count_bombs(current);
            if count_bombs > 0 {
                continue;
            }

            self.get_neighbors(current)
                .iter()
                .filter(|x| !self.grid[**x].bomb && self.grid[**x].state == TileState::Hidden)
                .for_each(|x| {
                    if visited.insert(*x) {
                        queue.push(*x);
                    }
                });
        }
        for idx in visited {
            let count_bombs = self.count_bombs(idx);
            let current_tile = self.grid.get_mut(idx).expect("checked!");
            current_tile.state = TileState::Free;
            current_tile.number = count_bombs;
            self.n_opened += 1;
        }
    }

    pub fn mark_cell(&mut self, idx: usize) -> bool {
        let Some(_) = self.grid.get_mut(idx).map(|x| match x.state {
            TileState::Hidden => {
                self.marked.insert(idx);
                x.state = TileState::Marked;
            }
            TileState::Marked => {
                self.marked.remove(&idx);
                x.state = TileState::Hidden;
            }
            TileState::Free => {
                x.state = TileState::Free;
            }
        }) else {
            return false;
        };
        return true;
    }

    pub fn check_win(&self) -> bool {
        println!("opened: {}; marked: {}", self.n_opened, self.marked.len());

        self.marked
            .iter()
            .map(|x| self.grid[*x].bomb)
            .all(|x| x == true)
            && (self.marked.len() == self.n_bombs)
            && (self.n_opened == self.width * self.height - self.n_bombs)
    }
}
