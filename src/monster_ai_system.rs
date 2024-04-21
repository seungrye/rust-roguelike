use crate::{Map, Monster, Name, Position, RunState, Viewshed, WantsToMelee};
use rltk::{console, Point};
use specs::prelude::*;

pub struct MonsterAI {}
impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToMelee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            runstate,
            entities,
            player_pos,
            player_entity,
            mut viewshared,
            monster,
            name,
            mut position,
            mut wants_to_melee,
        ) = data;

        if *runstate != RunState::MonsterTurn {
            return;
        }

        for (entity, viewshed, _monster, name, pos) in
            (&entities, &mut viewshared, &monster, &name, &mut position).join()
        {
            if viewshed.visible_tiles.contains(&*player_pos) {
                let distance =
                    rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
                if distance < 1.5 {
                    let _ = wants_to_melee.insert(
                        entity,
                        WantsToMelee {
                            target: *player_entity,
                        },
                    );
                    continue;
                } else if viewshed.visible_tiles.contains(&*player_pos) {
                    let path = rltk::a_star_search(
                        map.xy_idx(pos.x, pos.y),
                        map.xy_idx(player_pos.x, player_pos.y),
                        &*map,
                    );
                    if path.success && path.steps.len() > 1 {
                        let mut idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[idx] = false;

                        // player 쪽으로 1칸 이동할 (step[0] 는 현재 좌표) 좌표 획득
                        pos.x = path.steps[1] as i32 % map.width;
                        pos.y = path.steps[1] as i32 / map.width;

                        idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[idx] = true;

                        viewshed.dirty = true;
                    }
                }
            }
        }
    }
}
