use crate::bundle::*;
use crate::card::*;
use crate::location::Location;
use crate::mana_cost::ManaCost;
use crate::player::PlayerNumber;
use crate::state::State;
use crate::triggers::Triggers;
use std::sync::{Arc, Mutex};

pub fn animate_dead(owner: PlayerNumber) -> Card {
    Card::new(
        "Animate Dead".to_string(),
        owner,
        ManaCost::new().with_black(1).with_generic(1),
        vec![Type::Enchantment],
    )
    .with_base_subtypes(vec![Subtype::Aura])
    .with_base_triggers(
        Triggers::new()
            .with_can_cast(can_cast)
            .with_try_cast(try_cast)
            .with_on_cast(on_cast),
    )
}

fn is_creature_in_graveyard(card: &Card, controller: PlayerNumber, location: Location) -> bool {
    location == Location::Graveyard && card.types().contains(&Type::Creature)
}

// all of this code is the exact same as Animate Artifact (except the predicate)
fn can_cast(
    state: &State,
    bundle: &Bundle,
    card: Arc<Mutex<Card>>,
    controller: PlayerNumber,
    location: Location,
) -> bool {
    state.is_any_card_targetable_by(controller, is_creature_in_graveyard)
}

fn try_cast(
    state: &State,
    bundle: &mut Bundle,
    card: Arc<Mutex<Card>>,
    controller: PlayerNumber,
    location: Location,
) -> bool {
    match state.select_target_card(controller, is_creature_in_graveyard) {
        Some(target_card) => {
            bundle
                .map
                .insert("target_card", BundleItem::Card(target_card));
            true
        }
        None => false,
    }
}

fn on_cast(
    state: &mut State,
    bundle: &mut Bundle,
    card: Arc<Mutex<Card>>,
    controller: PlayerNumber,
    location: Location,
) -> bool {
    let target_card = bundle.map["target_card"].unwrap_card();
    let mut target_card = target_card.lock().unwrap();
    if state.is_target_card_valid(&*target_card, controller, is_creature_in_graveyard) {
        target_card.add_aura(card);
        true
    } else {
        false
    }
}
