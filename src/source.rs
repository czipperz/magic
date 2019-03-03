use crate::card::Card;
use crate::mana::Color;
use crate::player::PlayerNumber;
use crate::state::State;

pub struct Source {
    pub player: PlayerNumber,
    pub colors: Vec<Color>,
}

impl Source {
    pub fn from_card(state: &State, card: &Card) -> Source {
        Source {
            player: card.controller(state),
            colors: card.colors(state),
        }
    }
}
