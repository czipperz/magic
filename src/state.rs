use crate::card::Card;
use crate::player::*;
use crate::user_interface::UserInterface;
use std::sync::{Arc, Mutex};

pub struct State {
    players: Vec<Player>,
    stack: Vec<Arc<Mutex<Card>>>,
    ui: Arc<Mutex<UserInterface>>,
}

impl State {
    pub fn is_any_player_targetable_by(&self, controller: PlayerNumber) -> bool {
        // TODO: implement hexproof and shroud for players
        true
    }

    pub fn is_target_player_valid(&self, controller: PlayerNumber, target: PlayerNumber) -> bool {
        // TODO: implement hexproof and shroud for players
        true
    }

    pub fn select_target_player(&mut self, controller: PlayerNumber) -> Option<PlayerNumber> {
        self.ui.lock().unwrap().select_player(self)
    }

    pub fn make_player_draw_cards(&mut self, player: PlayerNumber, cards: usize) {
        // TODO: implement draw card triggers
        // TODO: implement losing when out of cards (Err returned)
        self.players[player].draw_cards(cards).unwrap();
    }

    pub fn is_any_permanent_targetable_by(
        &self,
        controller: PlayerNumber,
        predicate: &impl Fn(&State, &Card) -> bool,
    ) -> bool {
        for i in 0..self.players.len() {
            if self.players[i].is_any_permanent_targetable_by(self, controller, predicate) {
                return true;
            }
        }
        false
    }

    pub fn select_target_card(
        &mut self,
        controller: PlayerNumber,
        predicate: &impl Fn(&State, &Card) -> bool,
    ) -> Option<Arc<Mutex<Card>>> {
        // TODO: implement hexproof and shroud for cards
        self.ui.lock().unwrap().select_card(self, predicate)
    }
}
