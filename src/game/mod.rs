pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;
pub const ROOM_MAX_SIZE: i32 = 10;
pub const ROOM_MIN_SIZE: i32 = 6;
pub const MAX_ROOMS: i32 = 30;
pub const MAX_ROOM_MONSTERS: i32 = 3;
pub const CHANCE_FOR_BREAKABLE_TUNNEL: f32 = 0.25;
pub const PLAYER_INDEX: usize = 0;

pub mod tcod;
pub mod utilities;
pub mod map;
pub mod actions;

pub struct Game {
    pub map: map::Map,
}

pub mod object;
pub mod tile;
pub mod colors;
pub mod fov;
pub mod ai;
pub mod fighter;
pub mod deathcallback;