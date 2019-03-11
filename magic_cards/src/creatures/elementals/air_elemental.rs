use crate::cast::CastPermanent;
use magic_core::card::{Attribute, Card, Subtype, Type};
use magic_core::mana::ManaCost;

pub fn air_elemental() -> Card {
    Card::new(
        "Air Elemental",
        ManaCost::new().with_blue(2).with_generic(3),
        CastPermanent,
    )
    .with_type(Type::Creature)
    .with_subtype(Subtype::Elemental)
    .with_power(4)
    .with_toughness(4)
    .with_attribute(Attribute::Flying)
}
