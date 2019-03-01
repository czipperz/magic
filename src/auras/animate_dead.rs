use crate::card::*;
use crate::location::Location;
use crate::mana_cost::ManaCost;
use crate::player::PlayerNumber;
use crate::state::State;
use crate::triggers::*;

pub fn animate_dead(owner: PlayerNumber) -> Card {
    Card::new(
        "Animate Dead".to_string(),
        owner,
        ManaCost::new().with_black(1).with_generic(1),
        vec![Type::Enchantment],
    )
    .with_base_subtypes(vec![Subtype::Aura])
    .with_base_triggers(
        Triggers::new().with_cast_triggers(TriggerTargettingCreature::new(
            is_creature_in_graveyard,
            |state, card, target_card, player, location| {
                state.move_card(
                    target_card.clone(),
                    player,
                    location,
                    player,
                    Location::Battlefield,
                );
                target_card.lock().unwrap().add_aura(card);
                true
            },
        )),
    )
}

fn is_creature_in_graveyard(
    state: &State,
    card: &Card,
    controller: PlayerNumber,
    location: Location,
) -> bool {
    location == Location::Graveyard && card.types().contains(&Type::Creature)
}
