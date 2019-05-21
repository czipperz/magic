use crate::cast::CastPermanent;
use magic_core::card::{Card, CardBuilder, KeywordAbility, Subtype, Type};
use magic_core::mana::ManaCost;

pub fn air_elemental() -> Card {
    CardBuilder::new()
        .with_name("Air Elemental")
        .with_mana_cost(ManaCost::new().with_blue(2).with_generic(3))
        .on_resolve(CastPermanent)
        .with_type(Type::Creature)
        .with_subtype(Subtype::Elemental)
        .with_power(4)
        .with_toughness(4)
        .with_keyword_ability(KeywordAbility::Flying)
        .build()
}
