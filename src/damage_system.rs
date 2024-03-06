use specs::prelude::*;

use crate::{CombatStats, SufferDamage};
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
        let entities = ecs.entities();
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 {
                dead.push(entity);
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}
