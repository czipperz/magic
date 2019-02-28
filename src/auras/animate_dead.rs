use super::base_aura::*;
use crate::card::*;
use crate::location::Location;
use crate::mana_cost::ManaCost;
use crate::player::PlayerNumber;
use crate::triggers::Triggers;

pub fn animate_dead(owner: PlayerNumber) -> Card {
    Card::new(
        "Animate Dead".to_string(),
        owner,
        ManaCost::new().with_black(1).with_generic(1),
        vec![Type::Enchantment],
    )
    .with_base_subtypes(vec![Subtype::Aura])
    .with_base_triggers(creature_aura(Triggers::new(), is_creature_in_graveyard))
}

fn is_creature_in_graveyard(card: &Card, controller: PlayerNumber, location: Location) -> bool {
    location == Location::Graveyard && card.types().contains(&Type::Creature)
}
