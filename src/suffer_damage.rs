use specs::prelude::*;
use specs::{Entity, WriteStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount)
        } else {
            let damage = SufferDamage {
                amount: vec![amount],
            };
            store
                .insert(victim, damage)
                .expect("Unable to insert damage");
        }
    }
}
