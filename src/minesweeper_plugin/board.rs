use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::Rng;

use crate::minesweeper_plugin::tile::*;
use crate::minesweeper_plugin::position::*;

pub struct Board {
    pub height: usize,
    pub width: usize,
    pub sprite_size: f32,
    grid: Vec<Vec<Tile>>,
    covered: HashMap<Position, Entity>,
    flags: HashMap<Position, Entity>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            sprite_size: 50., /* TODO: Configurable? */
            grid: vec![vec![Tile::Adjacent(0); width]; height],
            covered: HashMap::new(),
            flags: HashMap::new(),
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Tile> {
        Some(self.grid.get(row)?.get(col)?.clone())
    }

    pub fn get_covered(&self, row: usize, col: usize) -> Option<&Entity> {
        self.covered.get(&Position::new(col, row))
    }

    pub fn get_covered_by_pos(&self, pos: &Position) -> Option<&Entity> {
        self.covered.get(pos)
    }

    pub fn add_covered(&mut self, row: usize, col: usize, value: Entity) {
        self.covered.insert(Position::new(col, row), value);
    }

    pub fn remove_covered_by_pos(&mut self, pos: &Position) {
        self.covered.remove(pos);
    }

    pub fn get_flagged(&self, row: usize, col: usize) -> Option<&Entity> {
        self.flags.get(&Position::new(col, row))
    }

    pub fn get_flagged_by_pos(&self, pos: &Position) -> Option<&Entity> {
        self.flags.get(pos)
    }

    pub fn add_flagged(&mut self, row: usize, col: usize, value: Entity) {
        self.flags.insert(Position::new(col, row), value);
    }

    pub fn remove_flagged(&mut self, row: usize, col: usize) {
        self.flags.remove(&Position::new(col, row));
    }

    pub fn add_bomb(&mut self) {
        let mut rng = rand::thread_rng();
        let mut row;
        let mut col;

        loop {
            row = rng.gen_range(0..self.height);
            col = rng.gen_range(0..self.width);

            if let Tile::Adjacent(_) = self.grid[row][col] {
                self.grid[row][col] = Tile::Bomb;
                break;
            }
        }

        for p in Position::new(col, row).get_surrounding(self.width, self.height) {
            self.get(p.y, p.x).map(|t| {
                self.grid[p.y][p.x] = match t {
                    Tile::Adjacent(val) => Tile::Adjacent(val + 1),
                    other => other
                };
            });
        }
    }

    pub fn iter(&self) -> BoardIter {
        BoardIter {
            grid: &self.grid,
            width: self.width,
            height: self.height,
            counter: 0,
        }
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        self.grid.iter().map(|row| {
            row.iter().map(|item| item.to_string()).collect::<Vec<String>>().join(",")
        }).collect::<Vec<String>>().join("\n")
    }
}

pub struct BoardIter<'a> {
    grid: &'a Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    counter: usize,
}

impl<'a> Iterator for BoardIter<'a> {
    type Item = (Position, &'a Tile);

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.counter / self.height;
        let col = self.counter % self.width;

        let tile = self.grid.get(row)?.get(col)?;
        self.counter += 1;

        Some((Position::new(col, row), tile))
    }
}
