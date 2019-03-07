use crate::card::Instance;
use crate::mana::Color;
use crate::player::PlayerNumber;
use std::sync::{Arc, Mutex};

pub struct Source {
    pub card: Arc<Mutex<Instance>>,
    pub player: PlayerNumber,
    pub colors: Vec<Color>,
}

impl From<Arc<Mutex<Instance>>> for Source {
    fn from(card: Arc<Mutex<Instance>>) -> Self {
        let (player, colors) = {
            let card = card.lock().unwrap();
            (card.controller(), card.colors().clone())
        };
        Source {
            card,
            player,
            colors,
        }
    }
}
