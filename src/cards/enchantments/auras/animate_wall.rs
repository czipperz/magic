use super::add_aura::*;
use crate::card::*;
use crate::mana::ManaCost;
use crate::player::PlayerNumber;
use crate::state::State;
use crate::triggers::*;
use std::sync::{Arc, Mutex};

pub fn animate_wall(owner: PlayerNumber) -> Arc<Mutex<Card>> {
    Arc::new(Mutex::new(
        Card::new(
            "Animate Wall".to_string(),
            owner,
            ManaCost::new().with_white(1),
            vec![Type::Enchantment],
        )
        .with_base_subtypes(vec![Subtype::Aura])
        .with_base_triggers(Triggers::new().on(
            TriggerType::Cast,
            TriggerTargettingCreature::new(
                is_wall_on_battlefield,
                add_aura(|_, _, card_state| {
                    if let Some(pos) = card_state
                        .attributes
                        .iter()
                        .position(|x| *x == Attribute::Defender)
                    {
                        card_state.attributes.remove(pos);
                    }
                }),
            ),
        )),
    ))
}

fn is_wall_on_battlefield(state: &State, card: &Card) -> bool {
    card.zone(state) == Zone::Battlefield && card.subtypes(state).contains(&Subtype::Wall)
}
