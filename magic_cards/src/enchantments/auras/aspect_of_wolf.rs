use super::aura::aura_permanent;
use magic_core::card::{Card, Subtype, Type};
use magic_core::effect::Effect;
use magic_core::instance::InstanceID;
use magic_core::mana::ManaCost;
use magic_core::state::State;

pub fn aspect_of_wolf() -> Card {
    aura_permanent(is_creature, AspectOfWolfEffect)
        .with_name("Aspect of Wolf")
        .with_mana_cost(ManaCost::new().with_green(1).with_generic(1))
        .with_type(Type::Enchantment)
        .with_subtype(Subtype::Aura)
        .build()
}

fn is_creature(state: &State, instance: InstanceID) -> bool {
    instance.get(state).types.contains(&Type::Creature)
}

#[derive(Debug)]
struct AspectOfWolfEffect;
impl Effect for AspectOfWolfEffect {
    fn affect(&self, state: &State, instance: InstanceID, card: &mut Card) {
        let controller = instance.get(state).controller;
        let num_forests = state
            .instances()
            .map(|id| id.get(state))
            .filter(|instance| {
                instance.controller == controller && instance.subtypes.contains(&Subtype::Forest)
            })
            .count();
        match (&mut card.power, &mut card.toughness) {
            (Some(power), Some(toughness)) => {
                *power += num_forests as isize / 2;
                *toughness += (num_forests + 1) as isize / 2;
            }
            _ => (),
        }
    }
}
