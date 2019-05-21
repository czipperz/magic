use crate::cast::CastPermanent;
use magic_core::card::{Card, CardBuilder, Type};
use magic_core::effect::Effect;
use magic_core::mana::{Color, ManaCost};
use magic_core::permanent::Permanent;
use magic_core::state::State;

pub fn bad_moon() -> Card {
    CardBuilder::new()
        .with_name("Bad Moon")
        .with_mana_cost(ManaCost::new().with_black(1).with_generic(1))
        .with_type(Type::Enchantment)
        .on_resolve(CastPermanent)
        .with_global_effect(BadMoonEffect)
        .build()
}

#[derive(Debug)]
struct BadMoonEffect;
impl Effect for BadMoonEffect {
    fn affect(&self, _: &State, permanent: &mut Permanent) {
        if permanent.colors.contains(&Color::Black) {
            match (&mut permanent.power, &mut permanent.toughness) {
                (Some(power), Some(toughness)) => {
                    *power += 1;
                    *toughness += 1;
                }
                _ => (),
            }
        }
    }
}
