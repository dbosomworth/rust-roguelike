use crate::game::object::Object;
use crate::game::Game;
use crate::game::PLAYER_INDEX;
use crate::game::tcod::Tcod;
use crate::game::utilities::mut_two;

#[derive(Debug)]
pub enum Ai {
    Basic,
}

pub fn ai_take_turn(monster_id: usize, tcod: &Tcod, game: &Game, objects: &mut [Object]) {

    let (monster_x, monster_y ) = objects[monster_id].pos();

    if tcod.fov.is_in_fov(monster_x, monster_y) {

        if objects[monster_id].distance_to(&objects[PLAYER_INDEX]) >= 2.0 {
            //move
            let (player_x, player_y) = objects[PLAYER_INDEX].pos();
            Object::move_towards(monster_id, player_x, player_y, &game, objects);

        }else if objects[PLAYER_INDEX].fighter.map_or(false, |f| f.hp > 0) {
            //attack
            let (monster, player) = mut_two(monster_id, PLAYER_INDEX, objects);
            monster.attack(player);
        }
    }
}