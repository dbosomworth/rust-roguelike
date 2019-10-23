use tcod::colors::Color;
use tcod::console::{Console, BackgroundFlag};

use crate::game::Game;
use crate::game::PLAYER_INDEX;
use crate::game::colors::*;
use crate::game::MAX_ROOM_MONSTERS;
use crate::game::map::Rect;
use crate::game::map::Map as Map;
use crate::game::utilities::is_location_blocked;
use crate::game::tile::Tile;

use crate::game::fighter::Fighter;
use crate::game::ai::Ai;

use rand::Rng;

#[derive(Debug)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub char: char,
    pub color: Color,
    pub name: String,
    pub blocks: bool,
    pub alive: bool,
    pub fighter: Option<Fighter>,
    pub ai: Option<Ai>,
}

impl Object {
    pub fn new(x: i32, y:i32, char: char, color: Color, name: String, blocks: bool) -> Self{
        Object { 
            x: x,
            y: y,
            char: char,
            color: color,
            name: name.into(),
            blocks: blocks,
            alive: false,
            fighter: None,
            ai: None
        }
    }

    //move the object by dx dy
    pub fn move_by(id: usize, dx: i32, dy: i32, game: &Game, objects: &mut [Object] ){
        let (x, y) = objects[id].pos();
        if !is_location_blocked(x + dx, y + dy, &game.map, objects){
            objects[id].set_pos(x + dx, y + dy);
        }
    }

    //attempt to move or attack
    pub fn player_move_or_attack(dx: i32, dy: i32, game: &Game, objects: &mut [Object]){
        let x = objects[PLAYER_INDEX].x + dx;
        let y = objects[PLAYER_INDEX].y + dy;

        let target_id = objects.iter().position(|object| object.pos() == (x, y));

        match target_id {
            Some(target_id) => {
                println!("The {} laughs at your puny efforts to attack him!", objects[target_id].name);
            }
            None => {

                if game.map[x as usize][y as usize].breakable == true {
                    println!("You feel a draft coming from the wall.");
                }else{
                    Object::move_by(PLAYER_INDEX, dx, dy, &game, objects);
                }
            }
        }
    }

    //move towards a target    
    //todo:: fix move_by to use map instead of game
    pub fn move_towards(id: usize, target_x: i32, target_y: i32, game: &Game, objects: &mut [Object]) {
        //find the vector
        let dx = target_x - objects[id].x;
        let dy = target_y - objects[id].y;
        let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

        //normalize, round and restrict to grind
        let dx = (dx as f32 / distance).round() as i32;
        let dy = (dy as f32 / distance).round() as i32;
        
        Object::move_by(id, dx, dy, &game, objects);
    }

    //calculates the distance to another object
    pub fn distance_to(&self, other: &Object) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        return ((dx.pow(2) + dy.pow(2)) as f32).sqrt();
    }


    //draw the object to the Console
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    //get the position of the object (x, y)
    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    //set the position of the object (x, y)
    pub fn set_pos(&mut self, x: i32, y: i32){
        self.x = x;
        self.y = y;
    }
}

pub fn place_objects(room: Rect, map: &Map, objects: &mut Vec<Object>) {
    let number_of_monsters = rand::thread_rng().gen_range(0,MAX_ROOM_MONSTERS + 1);

    for _ in 0..number_of_monsters {
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        if !is_location_blocked(x, y, map, objects){
            let mut monster = if rand::random::<f32>() < 0.8 {
                let mut orc = Object::new(x, y, 'o', COLOR_ORC, "orc".to_string(), true);
                orc.fighter = Some(Fighter{
                    max_hp: 10,
                    hp: 10,
                    defense: 0,
                    power: 3,
                });
                orc.ai = Some(Ai::Basic);
                orc
            }else{
                let mut troll = Object::new(x, y, 'T', COLOR_TROLL, "troll".to_string(), true);
                troll.fighter = Some(Fighter {
                    max_hp: 16,
                    hp: 16,
                    defense: 1,
                    power: 4,
                });
                troll.ai = Some(Ai::Basic);
                troll
            };    
        
            monster.alive = true;
            objects.push(monster);
        
        }
    }
}