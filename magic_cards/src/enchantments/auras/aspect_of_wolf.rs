use super::aura::aura_permanent;
use magic_core::card::{Card, Subtype, Type};
use magic_core::effect::Effect;
use magic_core::mana::ManaCost;
use magic_core::permanent::{Permanent, PermanentID};
use magic_core::state::State;

pub fn aspect_of_wolf() -> Card {
    aura_permanent(is_creature, AspectOfWolfEffect)
        .with_name("Aspect of Wolf")
        .with_mana_cost(ManaCost::new().with_green(1).with_generic(1))
        .with_type(Type::Enchantment)
        .with_subtype(Subtype::Aura)
        .build()
}

fn is_creature(state: &State, permanent: PermanentID) -> bool {
    permanent.get(state).types.contains(&Type::Creature)
}

#[derive(Debug)]
struct AspectOfWolfEffect;
impl Effect for AspectOfWolfEffect {
    fn affect(&self, state: &State, permanent: &mut Permanent) {
        let controller = permanent.instance(state).controller;
        let num_forests = state
            .permanents()
            .map(|pid| (pid.get(state), pid.instance(state)))
            .filter(|(permanent, instance)| {
                instance.controller == controller && permanent.subtypes.contains(&Subtype::Forest)
            })
            .count();
        match (&mut permanent.power, &mut permanent.toughness) {
            (Some(power), Some(toughness)) => {
                *power += num_forests as isize / 2;
                *toughness += (num_forests + 1) as isize / 2;
            }
            _ => (),
        }
    }
}
