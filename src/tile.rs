use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Adjacent(u8),
    Bomb,
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::Bomb => "B".to_string(),
            Tile::Adjacent(val) => val.to_string(),
        }
    }
}
