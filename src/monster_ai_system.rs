use crate::{Map, Monster, Name, Position, Viewshed};
use rltk::{console, Point};
use specs::prelude::*;

pub struct MonsterAI {}
impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut _map, _entities, player_pos, mut viewshed, monster, name, mut position) = data;
        for (viewshed, _monster, name, position) in
            (&mut viewshed, &monster, &name, &mut position).join()
        {
            if viewshed.visible_tiles.contains(&*player_pos) {
                let distance = rltk::DistanceAlg::Pythagoras
                    .distance2d(Point::new(position.x, position.y), *player_pos);
                if distance < 1.5 {
                    console::log(format!("{} shouts insults", name.name));
                    continue;
                }

                let path = rltk::a_star_search(
                    _map.xy_idx(position.x, position.y),
                    _map.xy_idx(player_pos.x, player_pos.y),
                    &*_map,
                );
                if path.success && path.steps.len() > 1 {
                    position.x = path.steps[1] as i32 % _map.width;
                    position.y = path.steps[1] as i32 / _map.width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
