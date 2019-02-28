use crate::bundle::*;
use crate::card::*;
use crate::location::Location;
use crate::mana_cost::ManaCost;
use crate::player::PlayerNumber;
use crate::state::State;
use crate::triggers::Triggers;
use std::sync::{Arc, Mutex};

pub fn ancestral_recall(owner: PlayerNumber) -> Card {
    Card::new(
        "Ancestral Recall".to_string(),
        owner,
        ManaCost::new().with_blue(1),
        vec![Type::Instant],
    )
    .with_base_triggers(
        Triggers::new()
            .with_can_cast(can_cast)
            .with_try_cast(try_cast)
            .with_on_cast(on_cast),
    )
}

fn can_cast(
    state: &State,
    bundle: &Bundle,
    card: Arc<Mutex<Card>>,
    controller: PlayerNumber,
    location: Location,
) -> bool {
    state.is_any_player_targetable_by(controller)
}

fn try_cast(
    state: &State,
    bundle: &mut Bundle,
    card: Arc<Mutex<Card>>,
    controller: PlayerNumber,
    location: Location,
) -> bool {
    match state.select_target_player(controller) {
        Some(target_player) => {
            bundle
                .map
                .insert("target_player", BundleItem::Player(target_player));
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
    let target_player = bundle.map["target_player"].unwrap_player();
    if state.is_target_player_valid(target_player, controller) {
        state.make_player_draw_cards(target_player, 3);
        true
    } else {
        false
    }
}
