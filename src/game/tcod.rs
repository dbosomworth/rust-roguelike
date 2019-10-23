use tcod::console::*;
use tcod::map::Map as FovMap;

//object to hold/pass around for tcod
pub struct Tcod {
    pub root: Root,
    pub con: Offscreen,
    pub fov: FovMap
}