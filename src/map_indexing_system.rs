use crate::{BlocksTile, Map, Position};
use rltk::console;
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        map.populate_blocked();
        map.clear_content_index();

        for (position, entity) in (&position, &entities).join() {
            let idx = map.xy_idx(position.x, position.y);

            // if is has some, update blocking list
            if let Some(_) = blockers.get(entity) {
                map.blocked[idx] = true;
            }

            // console::log(format!("{:#?}", entity));

            // push the entity to the appropriate index slot.
            // it is a Copy type, so we do not need to clone it
            // (we want to avoid moving it out of the ECS)
            map.tile_content[idx].push(entity)
        }
    }
}
