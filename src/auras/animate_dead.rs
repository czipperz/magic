use crate::card::*;
use crate::mana::ManaCost;
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
            |state, card, target_card| {
                let mut target_card = target_card.lock().unwrap();
                let player = target_card.owner();
                target_card.move_to(player, Zone::Battlefield);
                target_card.add_aura(card);
                true
            },
        )),
    )
}

fn is_creature_in_graveyard(state: &State, card: &Card) -> bool {
    card.zone() == Zone::Graveyard && card.types().contains(&Type::Creature)
}
