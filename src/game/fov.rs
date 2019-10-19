use tcod::map::{FovAlgorithm, Map as FovMap};
use crate::game::map::Map as GameMap;

pub const FOV_ALGORITHM: FovAlgorithm = FovAlgorithm::Basic;
pub const FOV_LIGHT_WALLS: bool = true;
pub const TORCH_RADIUS: i32 = 10;

pub fn compute_fov_map(fov: &mut FovMap, map: &GameMap, width: i32, height: i32){
    for y in 0..height {
        for x in 0..width {
            fov.set (
                x,
                y,
                !map[x as usize][y as usize].block_sight,
                !map[x as usize][y as usize].blocked
            );
        }
    }
}