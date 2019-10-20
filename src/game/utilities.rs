use crate::game::object::Object;
use crate::game::map::Map;

pub fn is_location_blocked(x: i32, y: i32, map: &Map, objects: &[Object]) -> bool{
    if map[x as usize][y as usize].blocked {
        return true;
    }

    return objects.iter().any(|object| object.blocks && object.pos() == (x, y))
}