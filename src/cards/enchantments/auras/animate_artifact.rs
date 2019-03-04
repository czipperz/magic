use super::add_aura::*;
use crate::card::*;
use crate::mana::ManaCost;
use crate::player::PlayerNumber;
use crate::state::State;
use crate::triggers::*;
use std::sync::{Arc, Mutex};

pub fn animate_artifact(owner: PlayerNumber) -> Arc<Mutex<Card>> {
    Arc::new(Mutex::new(
        Card::new(
            "Animate Artifact".to_string(),
            owner,
            ManaCost::new().with_blue(1).with_generic(3),
            vec![Type::Enchantment],
        )
        .with_base_subtypes(vec![Subtype::Aura])
        .with_base_triggers(Triggers::new().on(
            TriggerType::Cast,
            TriggerTargettingCreature::new(
                is_artifact_on_battlefield,
                add_aura(|_, _, card_state| {
                    if !card_state.types.contains(&Type::Creature) {
                        card_state.types.push(Type::Creature);
                        card_state.power = card_state.mana_cost.converted();
                        card_state.toughness = card_state.mana_cost.converted();
                    }
                }),
            ),
        )),
    ))
}

fn is_artifact_on_battlefield(state: &State, card: &Card) -> bool {
    card.zone(state) == Zone::Battlefield && card.types(state).contains(&Type::Artifact)
}
