use crate::card::*;
use crate::mana_cost::ManaCost;
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
            state.make_player_draw_cards(target_player, 3);
            true
        })),
    )
}
