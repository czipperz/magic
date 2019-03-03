use crate::card::*;
use crate::mana::ManaCost;
use crate::player::PlayerNumber;
use crate::state::State;
use crate::triggers::*;

pub fn animate_artifact(owner: PlayerNumber) -> Card {
    Card::new(
        "Animate Artifact".to_string(),
        owner,
        ManaCost::new().with_blue(1).with_generic(3),
        vec![Type::Enchantment],
    )
    .with_base_subtypes(vec![Subtype::Aura])
    .with_base_triggers(
        Triggers::new().with_cast_triggers(TriggerTargettingCreature::new(
            is_artifact_on_battlefield,
            |_, card, target_card| {
                target_card
                    .lock()
                    .unwrap()
                    .add_aura(card, |state, card, card_state| {
                        if !card_state.types.contains(&Type::Creature) {
                            card_state.types.push(Type::Creature);
                            card_state.power = card_state.mana_cost.converted();
                            card_state.toughness = card_state.mana_cost.converted();
                        }
                    });
                true
            },
        )),
    )
}

fn is_artifact_on_battlefield(state: &State, card: &Card) -> bool {
    card.zone(state) == Zone::Battlefield && card.types(state).contains(&Type::Artifact)
}
