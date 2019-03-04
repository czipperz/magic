use crate::card::Card;
use crate::source::Source;
use crate::state::State;
use std::sync::{Arc, Mutex};

pub type PlayerNumber = usize;

pub struct Player {
    deck: Vec<Arc<Mutex<Card>>>,
    hand: Vec<Arc<Mutex<Card>>>,
    battlefield: Vec<Arc<Mutex<Card>>>,
    graveyard: Vec<Arc<Mutex<Card>>>,
    exile: Vec<Arc<Mutex<Card>>>,
}

impl Player {
    pub fn new(deck: Vec<Arc<Mutex<Card>>>) -> Self {
        Player {
            deck,
            hand: Vec::new(),
            battlefield: Vec::new(),
            graveyard: Vec::new(),
            exile: Vec::new(),
        }
    }

    pub fn is_valid_target(&self, _state: &State, _source: &Source) -> bool {
        // TODO: implement hexproof and shroud for players
        true
    }

    pub fn draw_cards(&mut self, _state: &State, num: usize) -> Result<(), ()> {
        // TODO: implement draw card triggers
        // TODO: implement losing when out of cards (Err returned)
        for _ in 0..num {
            self.hand.push(self.deck.pop().ok_or(())?);
        }
        Ok(())
    }

    pub fn is_any_permanent_targetable_by(
        &self,
        state: &State,
        source: &Source,
        predicate: &impl Fn(&State, &Card) -> bool,
    ) -> bool {
        self.battlefield
            .iter()
            .chain(self.graveyard.iter())
            .chain(self.exile.iter())
            .any(|card| {
                card.lock()
                    .unwrap()
                    .is_valid_target(state, source, predicate)
            })
    }
}
