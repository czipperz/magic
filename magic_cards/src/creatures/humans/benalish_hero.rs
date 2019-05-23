use crate::cast::CastPermanent;
use magic_core::card::{Card, CardBuilder, KeywordAbility, Subtype, Type};
use magic_core::mana::ManaCost;

pub fn benalish_hero() -> Card {
    CardBuilder::new()
        .with_name("Benalish Hero")
        .with_mana_cost(ManaCost::new().with_white(1))
        .on_resolve(CastPermanent)
        .with_type(Type::Creature)
        .with_subtype(Subtype::Human)
        .with_subtype(Subtype::Soldier)
        .with_keyword_ability(KeywordAbility::Bands)
        .build()
}
