use super::base_aura::*;
use crate::card::*;
use crate::location::Location;
use crate::mana_cost::ManaCost;
use crate::player::PlayerNumber;
use crate::triggers::Triggers;

pub fn animate_artifact(owner: PlayerNumber) -> Card {
    Card::new(
        "Animate Artifact".to_string(),
        owner,
        ManaCost::new().with_blue(1).with_generic(3),
        vec![Type::Enchantment],
    )
    .with_base_subtypes(vec![Subtype::Aura])
    .with_base_triggers(creature_aura(Triggers::new(), is_artifact_on_battlefield))
}

fn is_artifact_on_battlefield(card: &Card, controller: PlayerNumber, location: Location) -> bool {
    location == Location::Battlefield && card.types().contains(&Type::Artifact)
}
