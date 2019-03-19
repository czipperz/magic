use super::aura::aura_permanent;
use magic_core::card::{Attribute, Card, Subtype, Type};
use magic_core::effect::Effect;
use magic_core::mana::ManaCost;
use magic_core::permanent::Permanent;
use magic_core::permanent::PermanentNumber;
use magic_core::state::State;

pub fn animate_wall() -> Card {
    aura_permanent("Animate Wall", ManaCost::new().with_white(1), is_wall)
        .with_type(Type::Enchantment)
        .with_subtype(Subtype::Aura)
        .with_effect(AnimateWallEffect)
}

fn is_wall(state: &State, instance: PermanentNumber) -> bool {
    instance.get(state).subtypes.contains(&Subtype::Wall)
}

#[derive(Debug)]
struct AnimateWallEffect;
impl Effect for AnimateWallEffect {
    fn affect(&self, _: &State, permanent: &mut Permanent) {
        permanent.ignored_attributes.push(Attribute::Defender)
    }
}
