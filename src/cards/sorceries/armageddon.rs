use crate::card::*;
use crate::mana::ManaCost;
use crate::player::PlayerNumber;
use crate::triggers::*;
use std::sync::{Arc, Mutex};

pub fn armageddon(owner: PlayerNumber) -> Arc<Mutex<Card>> {
    Arc::new(Mutex::new(
        Card::new(
            "Armageddon".to_string(),
            owner,
            ManaCost::new().with_blue(1),
            vec![Type::Instant],
        )
        .with_base_triggers(Triggers::new().on(
            TriggerType::Cast,
            TriggerOnExecute::new(|state, _| {
                state
                    .destroy_all_permanents(&|state, card| card.types(state).contains(&Type::Land));
                true
            }),
        )),
    ))
}
