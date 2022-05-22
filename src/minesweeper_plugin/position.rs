use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

impl Position {
    pub fn get_surrounding(&self, width: usize, height: usize) -> Vec<Position> {
        let irow = self.y as i32;
        let icol = self.x as i32;

        let mut result: Vec<Position> = Vec::with_capacity(8);

        for r in irow-1..=irow+1 {
            for c in icol-1..=icol+1 {
                if r < 0 || r >= height as i32 || c < 0 || c >= width as i32 {
                    continue;
                }

                result.push(Position::new(c as usize, r as usize));
            }
        }
        result
    }
}
