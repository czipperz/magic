use crate::card::Card;
use crate::player::*;
use crate::source::Source;
use crate::ui::UserInterface;
use std::sync::{Arc, Mutex};

pub struct State {
    players: Vec<Player>,
    stack: Vec<Arc<Mutex<Card>>>,
    ui: Arc<Mutex<UserInterface>>,
}

impl State {
    pub fn new(decks: Vec<Vec<Arc<Mutex<Card>>>>, ui: impl UserInterface + 'static) -> Self {
        State {
            players: decks.into_iter().map(Player::new).collect(),
            stack: Vec::new(),
            ui: Arc::new(Mutex::new(ui)),
        }
    }

    pub fn is_any_player_targetable_by(&self, source: &Source) -> bool {
        // TODO: implement hexproof and shroud for players
        true
    }

    pub fn is_target_player_valid(&self, source: &Source, target: PlayerNumber) -> bool {
        // TODO: implement hexproof and shroud for players
        true
    }

    pub fn select_target_player(&mut self, source: &Source) -> Option<PlayerNumber> {
        self.ui.lock().unwrap().select_player(self, source)
    }

    pub fn make_player_draw_cards(&mut self, player: PlayerNumber, cards: usize) {
        // TODO: implement draw card triggers
        // TODO: implement losing when out of cards (Err returned)
        self.players[player].draw_cards(cards).unwrap();
    }

    pub fn is_any_permanent_targetable_by(
        &self,
        source: &Source,
        predicate: &impl Fn(&State, &Card) -> bool,
    ) -> bool {
        for i in 0..self.players.len() {
            if self.players[i].is_any_permanent_targetable_by(self, source, predicate) {
                return true;
            }
        }
        false
    }

    pub fn select_target_card(
        &mut self,
        source: &Source,
        predicate: &impl Fn(&State, &Card) -> bool,
    ) -> Option<Arc<Mutex<Card>>> {
        // TODO: implement hexproof and shroud for cards
        self.ui.lock().unwrap().select_card(self, source, predicate)
    }
}
