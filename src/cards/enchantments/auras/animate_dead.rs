use crate::card::*;
use crate::mana::ManaCost;
use crate::player::PlayerNumber;
use crate::state::State;
use crate::triggers::*;
use std::sync::{Arc, Mutex};

pub fn animate_dead(owner: PlayerNumber) -> Arc<Mutex<Card>> {
    Arc::new(Mutex::new(
        Card::new(
            "Animate Dead".to_string(),
            owner,
            ManaCost::new().with_black(1).with_generic(1),
            vec![Type::Enchantment],
        )
        .with_base_subtypes(vec![Subtype::Aura])
        .with_base_triggers(Triggers::new().on(
            TriggerType::Cast,
            TriggerTargettingCreature::new(is_creature_in_graveyard, |state, card, target_card| {
                let mut target_card = target_card.lock().unwrap();
                let player = card.lock().unwrap().controller(state);
                target_card.move_to(player, Zone::Battlefield);
                target_card.add_aura(card, |_, _, card_state| {
                    card_state.power = card_state.power.saturating_sub(1);
                });
                true
            }),
        )),
    ))
}

fn is_creature_in_graveyard(state: &State, card: &Card) -> bool {
    card.zone(state) == Zone::Graveyard && card.types(state).contains(&Type::Creature)
}
