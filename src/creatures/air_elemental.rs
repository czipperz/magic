use crate::card::*;
use crate::mana_cost::ManaCost;
use crate::player::PlayerNumber;

pub fn air_elemental(player: PlayerNumber) -> Card {
    Card::new(
        "Air Elemental".to_string(),
        player,
        ManaCost::new().with_blue(2).with_generic(3),
        vec![Type::Creature],
    )
    .with_base_subtypes(vec![Subtype::Elemental])
    .with_base_power(4)
    .with_base_toughness(4)
    .with_base_attributes(vec![Attribute::Flying])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let air_elemental = air_elemental(1);
        assert_eq!(air_elemental.name(), "Air Elemental");
        assert_eq!(air_elemental.owner(), 1);
        assert_eq!(air_elemental.mana_cost().blue, 2);
        assert_eq!(air_elemental.mana_cost().generic, 3);
        assert_eq!(air_elemental.types(), vec![Type::Creature]);
        assert_eq!(air_elemental.subtypes(), vec![Subtype::Elemental]);
        assert_eq!(air_elemental.power(), 4);
        assert_eq!(air_elemental.toughness(), 4);
    }
}
