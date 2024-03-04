use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Viewshed {
    pub(crate) visible_tiles: Vec<rltk::Point>,
    pub(crate) range: i32,
    pub(crate) dirty: bool,
}

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Debug)]
pub struct BlocksTile {}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub(crate) max_hp: i32,
    pub(crate) hp: i32,
    pub(crate) defense: i32,
    pub(crate) power: i32,
}
