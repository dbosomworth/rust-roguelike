use crate::game::object::Object;
use crate::game::map::Map;

pub fn is_location_blocked(x: i32, y: i32, map: &Map, objects: &[Object]) -> bool{
    if map[x as usize][y as usize].blocked {
        return true;
    }

    return objects.iter().any(|object| object.blocks && object.pos() == (x, y))
}

pub fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    use std::cmp;

    assert!(first_index != second_index);

    let split_at_index = cmp::max(first_index, second_index);

    let (first_slice, second_slice) = items.split_at_mut(split_at_index);

    if first_index < second_index {
        return (&mut first_slice[first_index], &mut second_slice[0]);
    } else {
        return (&mut second_slice[0], &mut first_slice[second_index]);
    }
}