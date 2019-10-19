pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

pub const ROOM_MAX_SIZE: i32 = 10;
pub const ROOM_MIN_SIZE: i32 = 6;
pub const MAX_ROOMS: i32 = 30;

pub mod map;

pub struct Game {
    pub map: map::Map
}

pub mod object;
pub mod tile;
pub mod colors;
pub mod fov;