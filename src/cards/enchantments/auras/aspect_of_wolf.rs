use super::add_aura::*;
use crate::card::*;
use crate::mana::ManaCost;
use crate::player::PlayerNumber;
use crate::state::State;
use crate::triggers::*;
use std::sync::{Arc, Mutex};

pub fn aspect_of_wolf(owner: PlayerNumber) -> Arc<Mutex<Card>> {
    Arc::new(Mutex::new(
        Card::new(
            "Aspect of Wolf".to_string(),
            owner,
            ManaCost::new().with_green(1).with_generic(1),
            vec![Type::Enchantment],
        )
        .with_base_subtypes(vec![Subtype::Aura])
        .with_base_triggers(Triggers::new().on(
            TriggerType::Cast,
            TriggerTargettingCreature::new(is_creature_on_battlefield, add_aura(decoration)),
        )),
    ))
}

fn is_creature_on_battlefield(state: &State, card: &Card) -> bool {
    card.zone(state) == Zone::Battlefield && card.types(state).contains(&Type::Creature)
}

fn decoration(state: &State, _: &Card, card_state: &mut CardState) {
    // this causes an infinite loop
    let forests = state
        .player(card_state.controller)
        .lock()
        .unwrap()
        .count_permanents_on_battlefield(&|card| card.types(state).contains(&Type::Land));
    card_state.power += forests / 2;
    card_state.toughness += (forests + 1) / 2;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::TestingUI;

    #[test]
    fn does_nothing_when_no_forests() {
        let state = State::new(20, vec![vec![]], TestingUI::new());
        let creature = crate::cards::air_elemental(0);
        state
            .player(0)
            .lock()
            .unwrap()
            .inject_into_battlefield(creature.clone());
        assert_eq!(creature.lock().unwrap().power(&state), 4);

        let aura = aspect_of_wolf(0);
        creature.lock().unwrap().add_aura(aura, decoration);
        assert_eq!(creature.lock().unwrap().power(&state), 4);
    }
}
