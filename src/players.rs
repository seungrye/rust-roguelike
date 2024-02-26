use crate::{Map, Position, TileType, Viewshed};
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
    let map = ecs.read_resource::<Map>();
    let mut ppos = ecs.write_resource::<Point>();

    // 만약 Position과 Player를 모두 가진 엔티티가 있다면, for 구문을 실행합니다.
    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if !map.blocked[destination_idx] {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
            viewshed.dirty = true;
        }

        ppos.x = pos.x;
        ppos.y = pos.y;
    }
}
