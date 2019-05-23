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
        .with_power(1)
        .with_toughness(1)
        .with_keyword_ability(KeywordAbility::Banding)
        .build()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_constructor() {
        let card = benalish_hero();
        assert_eq!(card.name, "Benalish Hero");
        assert_eq!(*card.mana_cost(), ManaCost::new().with_white(1));
        assert_eq!(card.types, &[Type::Creature]);
        assert_eq!(card.subtypes, &[Subtype::Human, Subtype::Soldier]);
        assert_eq!(card.power, Some(1));
        assert_eq!(card.toughness, Some(1));
        assert_eq!(card.keyword_abilities, &[KeywordAbility::Banding]);
    }
}
