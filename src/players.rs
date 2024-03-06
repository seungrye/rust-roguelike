use crate::{CombatStats, Map, Position, TileType, Viewshed, WantsToMelee};
use rltk::Point;
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

#[derive(Component, Debug)]
pub struct Player {}

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let entities = ecs.entities();
    let map = ecs.read_resource::<Map>();
    let mut ppos = ecs.write_resource::<Point>();

    // 만약 다음의 항목들을 모두 가진 엔티티가 있다면, for 구문을 실행합니다.
    for (entity, _player, pos, viewshed) in
        (&entities, &mut players, &mut positions, &mut viewsheds).join()
    {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[destination_idx].iter() {
            if let Some(target) = combat_stats.get(*potential_target) {
                wants_to_melee
                    .insert(
                        entity, // ??? 그냥 player 를 넣으면 되는게 아닌가??
                        WantsToMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("Add target failed");
                return;
            }
        }

        if !map.blocked[destination_idx] {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
            viewshed.dirty = true;
        }

        ppos.x = pos.x;
        ppos.y = pos.y;
    }
}
