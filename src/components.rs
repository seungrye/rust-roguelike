use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Viewshed {
    pub(crate) visible_tiles: Vec<rltk::Point>,
    pub(crate) range: i32,
    pub(crate) dirty: bool,
}
