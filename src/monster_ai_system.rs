use specs::prelude::*;

use crate::{Map, Monster, Position, Viewshed};

pub struct MonsterAI {}
impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Monster>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, monster) = data;
        for (ent, viewshed, pos, monster) in (&entities, &viewshed, &pos, &monster).join() {}
    }
}
