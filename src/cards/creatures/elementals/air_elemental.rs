use crate::card::*;
use crate::mana::ManaCost;
use crate::player::PlayerNumber;
use std::sync::{Arc, Mutex};

pub fn air_elemental(player: PlayerNumber) -> Arc<Mutex<Card>> {
    Arc::new(Mutex::new(
        Card::new(
            "Air Elemental".to_string(),
            player,
            ManaCost::new().with_blue(2).with_generic(3),
            vec![Type::Creature],
        )
        .with_base_subtypes(vec![Subtype::Elemental])
        .with_base_power(4)
        .with_base_toughness(4)
        .with_base_attributes(vec![Attribute::Flying]),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;
    use crate::ui::TestingUI;

    #[test]
    fn test() {
        let state = State::new(20, vec![], TestingUI::new());
        let air_elemental = air_elemental(1);
        let air_elemental = air_elemental.lock().unwrap();
        assert_eq!(air_elemental.name(), "Air Elemental");
        assert_eq!(air_elemental.owner(), 1);
        assert_eq!(air_elemental.mana_cost(&state).blue, 2);
        assert_eq!(air_elemental.mana_cost(&state).generic, 3);
        assert_eq!(air_elemental.types(&state), vec![Type::Creature]);
        assert_eq!(air_elemental.subtypes(&state), vec![Subtype::Elemental]);
        assert_eq!(air_elemental.power(&state), 4);
        assert_eq!(air_elemental.toughness(&state), 4);
    }
}
