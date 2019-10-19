use tcod::console::*;
use tcod::map::Map as FovMap;

//import from the game folder so we can split the files up
mod game;

use game::Game as Game;
use game::MAP_HEIGHT;
use game::MAP_WIDTH;
use game::object::Object as Object;
use game::colors::*;
use game::fov::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

const FONT_PNG: &str = "arial10x10.png";

//object to hold/pass around for tcod
struct Tcod {
    root: Root,
    con: Offscreen,
    fov: FovMap
}

//Render function
fn render_all(tcod: &mut Tcod, game: &mut Game, objects: &[Object], fov_recompute: bool){
    


    if fov_recompute {
        let player = &objects[0];
        tcod.fov.compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGORITHM);
    }

    //Draw the objects in our object array
    for object in objects {
        if tcod.fov.is_in_fov(object.x, object.y) {
            object.draw(&mut tcod.con);
        }
    }

    //loop through our map and draw the tiles
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            let visible = tcod.fov.is_in_fov(x, y);
            let color = match (visible, wall) {
                (false, false) => COLOR_DARK_GROUND,
                (false, true) => COLOR_DARK_WALL,
                (true, false) => COLOR_LIGHT_GROUND,
                (true, true) => COLOR_LIGHT_WALL,                
            };       
            
            //calculate if tile is explored
            let explored = &mut game.map[x as usize][y as usize].explored;

            if visible {
                *explored = true;
            } 
            
            if *explored {
                tcod.con.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }


    //blit the offscreen render to the root
    blit(&tcod.con, (0,0), (MAP_WIDTH, MAP_HEIGHT), &mut tcod.root, (0,0), 1.0, 1.0);
}

//Toggle fullscreen
fn toggle_fullscreen(tcod: &mut Tcod){
    let fullscreen = tcod.root.is_fullscreen();
    tcod.root.set_fullscreen(!fullscreen);
}

//Handle key presses
// &mut is effectively borrowing
fn handle_keys(mut tcod: &mut Tcod, game: &Game, player: &mut Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    
    let key = tcod.root.wait_for_keypress(true);

    match key {
        //.. means ignore other fields in the struct
        Key { code: Enter, alt: true, .. } => toggle_fullscreen(&mut tcod),        
        Key { code: Escape, ..} => return true,
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } =>  player.move_by(0, 1, game),
        Key { code: Left, .. } =>  player.move_by(-1, 0, game),
        Key { code: Right, .. } =>  player.move_by(1, 0, game),

        _ => {}
    }

    return false;
}

//Main function
fn main() {
    
    //setup the root (window etc)
    let root = Root::initializer()
        .font(FONT_PNG, FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    
    //create the tcod object
    let mut tcod = Tcod {
        root, 
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
        fov: FovMap::new(MAP_WIDTH,MAP_HEIGHT)
    };

    //set the maximum fps
    tcod::system::set_fps(LIMIT_FPS);

    //create the player Object
    let player = Object::new(25, 23, '@', COLOR_WHITE);

    //create an NPC Object
    let npc = Object::new(SCREEN_WIDTH /2 - 5, SCREEN_HEIGHT /2, '@', COLOR_YELLOW);

    //make a list of Objects
    let mut objects = [player, npc];

    //create the game object
    let mut game = Game {
        map: game::map::make_map(&mut objects[0]),
    };

    compute_fov_map(&mut tcod.fov, &game.map, MAP_WIDTH, MAP_HEIGHT);

    let mut previous_player_position = (-1, -1);

    //game loop
    while !tcod.root.window_closed() {

        //clear the off-screen buffer
        tcod.con.clear();

        let fov_recompute = previous_player_position != (objects[0].x, objects[0].y);
        //render our game, and blit to root screen
        render_all(&mut tcod, &mut game, &objects, fov_recompute);

        //flush it
        tcod.root.flush();

        //get our player object so we can pass it to handle keys
        let player = &mut objects[0];

        //update previous player position
        previous_player_position = (player.x, player.y);

        //get the next keyboard input
        let exit = handle_keys(&mut tcod, &game, player);

        if exit {
            break;
        }
    }
}
