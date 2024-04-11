use specs::prelude::*;

use crate::{CombatStats, Player, SufferDamage};
pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut combat_stats, mut damages) = data;
        for (stats, damage) in (&mut combat_stats, &damages).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }

        damages.clear();
    }
}

pub fn delete_the_dead(ecs: &mut World) {
    let mut dead = Vec::new();
    // using a scope to make the borrow checker happy
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let players = ecs.read_storage::<Player>();
        let entities = ecs.entities();
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 {
                let player = players.get(entity);
                match player {
                    None => dead.push(entity),
                    Some(_) => rltk::console::log("You are dead"),
                }
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}
