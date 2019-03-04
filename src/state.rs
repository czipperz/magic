use crate::card::Card;
use crate::player::*;
use crate::source::Source;
use crate::ui::UserInterface;
use std::sync::{Arc, Mutex};

pub struct State {
    players: Vec<Arc<Mutex<Player>>>,
    stack: Vec<Arc<Mutex<Card>>>,
    ui: Arc<Mutex<UserInterface>>,
}

impl State {
    pub fn new(decks: Vec<Vec<Arc<Mutex<Card>>>>, ui: impl UserInterface + 'static) -> Self {
        State {
            players: decks.into_iter().map(|p| Arc::new(Mutex::new(Player::new(p)))).collect(),
            stack: Vec::new(),
            ui: Arc::new(Mutex::new(ui)),
        }
    }

    pub fn player(&self, player: PlayerNumber) -> Arc<Mutex<Player>> {
        self.players[player].clone()
    }

    pub fn is_any_player_targetable_by(&self, source: &Source) -> bool {
        self.players.iter().any(|player| player.lock().unwrap().is_valid_target(self, source))
    }

    pub fn select_target_player(&mut self, source: &Source) -> Option<PlayerNumber> {
        self.ui.lock().unwrap().select_player(self, source)
    }

    pub fn is_any_permanent_targetable_by(
        &self,
        source: &Source,
        predicate: &impl Fn(&State, &Card) -> bool,
    ) -> bool {
        for player in &self.players {
            if player.lock().unwrap().is_any_permanent_targetable_by(self, source, predicate) {
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
        self.ui.lock().unwrap().select_card(self, source, predicate)
    }
}
