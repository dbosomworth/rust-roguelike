use crate::game::MAP_HEIGHT;
use crate::game::MAP_WIDTH;
use crate::game::ROOM_MAX_SIZE;
use crate::game::ROOM_MIN_SIZE;
use crate::game::MAX_ROOMS;
use crate::game::PLAYER_INDEX;
use crate::game::tile::Tile as Tile;    
use crate::game::object::{Object as Object, place_objects};

use rand::Rng;

pub type Map = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
   pub x1: i32,
   pub y1: i32,
   pub x2: i32,
   pub y2: i32
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self{
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    //returns a touple of the center coords
    pub fn center(&self) -> (i32, i32) {
        let center_x: i32 = (self.x1 + self.x2) / 2;
        let center_y: i32 = (self.y1 + self.y2) / 2;
        return (center_x, center_y);
    }

    //calculates if the rect intersects with another rect
    pub fn intersects_with(&self, other: &Rect) -> bool {
        return (self.x1 <= other.x2) &&
                (self.x2 >= other.x1) &&
                (self.y1 <= other.y2) &&
                (self.y2 >= other.y1)
    }

}

fn create_room(room: Rect, map: &mut Map){
    //for in A..B inclusive at the start, exclusive at the end
    //1..5 is 1,2,3,4
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}


fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map){
    use std::cmp;
    for x in cmp::min(x1, x2)..(cmp::max(x1,x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}
fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map){
    use std::cmp;
    for y in cmp::min(y1, y2)..(cmp::max(y1,y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}
//generate map
pub fn make_map(objects: &mut Vec<Object>) -> Map {
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let mut rooms = vec![];


    for _ in 0..MAX_ROOMS {
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);

        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);
    
        let new_room = Rect::new(x, y, w, h);

        let failed = rooms.iter()
                        .any(|other_room| new_room.intersects_with(other_room));
    
        let (new_x, new_y) = new_room.center();

        if !failed {
            
            create_room(new_room, &mut map);
            place_objects(new_room, &map, objects);

            if rooms.is_empty() {
                objects[PLAYER_INDEX].set_pos(new_x, new_y);
            } else {

                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                if rand::random() {
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }            
            }

            rooms.push(new_room);
        }
    }
    return map;
}