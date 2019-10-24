use crate::game::colors::{COLOR_RED_BLOOD, COLOR_GREEN_BLOOD};
use crate::game::object::Object;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeathCallback {
    Player,
    Monster,
}

impl DeathCallback {
    pub fn callback(self, object: &mut Object) {
        use DeathCallback::*;
        let callback: fn(&mut Object) = match self {
            Player => player_death,
            Monster => monster_death,
        };
        callback(object);
    }
}

fn player_death(player: &mut Object){
    println!("You died!");

    player.char = '%';
    player.color = COLOR_RED_BLOOD;
}

fn monster_death(monster: &mut Object){
    println!("{} is dead!", monster.name);
    monster.char = '%';
    monster.color = COLOR_GREEN_BLOOD;
    monster.blocks = false;
    monster.fighter = None;
    monster.ai = None;
    monster.name = format!("remains of {}.", monster.name);
}