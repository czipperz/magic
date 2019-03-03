use crate::card::Card;
use crate::mana::Color;
use crate::player::PlayerNumber;
use crate::state::State;
use std::sync::{Arc, Mutex};

pub struct Source {
    pub player: PlayerNumber,
    pub colors: Vec<Color>,
    pub card: Arc<Mutex<Card>>,
}

impl Source {
    pub fn from_card(state: &State, card: Arc<Mutex<Card>>) -> Source {
        let (player, colors) = {
            let card = card.lock().unwrap();
            (card.controller(state), card.colors(state))
        };
        Source {
            player,
            colors,
            card,
        }
    }
}
