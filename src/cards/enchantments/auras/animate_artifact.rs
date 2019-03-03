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
                    .add_aura(card, AnimateArtifactDecoration);
                true
            },
        )),
    )
}

fn is_artifact_on_battlefield(state: &State, card: &Card) -> bool {
    card.zone(state) == Zone::Battlefield && card.types(state).contains(&Type::Artifact)
}

#[derive(Debug)]
struct AnimateArtifactDecoration;

impl CardDecoration for AnimateArtifactDecoration {
    fn decorate_types(&self, state: &State, card: &Card, mut types: Vec<Type>) -> Vec<Type> {
        if !types.contains(&Type::Creature) {
            types.push(Type::Creature);
        }
        types
    }

    fn decorate_power(&self, state: &State, card: &Card, power: usize) -> usize {
        if !card.types(state).contains(&Type::Creature) {
            card.converted_mana_cost(state)
        } else {
            power
        }
    }

    fn decorate_toughness(&self, state: &State, card: &Card, toughness: usize) -> usize {
        if !card.types(state).contains(&Type::Creature) {
            card.converted_mana_cost(state)
        } else {
            toughness
        }
    }
}
