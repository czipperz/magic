use crate::card::*;
use crate::mana::ManaCost;
use crate::player::PlayerNumber;
use crate::triggers::*;

pub fn ancestral_recall(owner: PlayerNumber) -> Card {
    Card::new(
        "Ancestral Recall".to_string(),
        owner,
        ManaCost::new().with_blue(1),
        vec![Type::Instant],
    )
    .with_base_triggers(
        Triggers::new().with_cast_triggers(TriggerTargettingPlayer::new(|state, target_player| {
            state.player(target_player).lock().unwrap().draw_cards(state, 3).unwrap();
            true
        })),
    )
}
