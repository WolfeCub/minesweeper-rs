use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::Rng;

use crate::tile::*;
use crate::position::*;

pub struct Board {
    pub height: usize,
    pub width: usize,
    pub sprite_size: f32,
    pub grid: Vec<Vec<Tile>>,
    pub covered: HashMap<Position, Entity>,
    pub flags: HashMap<Position, Entity>,
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
}

impl ToString for Board {
    fn to_string(&self) -> String {
        self.grid.iter().map(|row| {
            row.iter().map(|item| item.to_string()).collect::<Vec<String>>().join(",")
        }).collect::<Vec<String>>().join("\n")
    }
}

