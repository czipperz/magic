use crate::card::Card;
use crate::event::Event;
use crate::player::Player;
use std::sync::{Arc, Mutex};

pub struct GameState {
    players: Vec<Arc<Mutex<Player>>>,
    stack: Vec<Arc<Mutex<Event>>>,
}

impl GameState {
    pub fn new(default_health: isize, decks: Vec<Vec<Card>>) -> Self {
        GameState {
            players: decks
                .into_iter()
                .map(|deck| Arc::new(Mutex::new(Player::new(default_health, deck))))
                .collect(),
            stack: Vec::new(),
        }
    }

    pub fn inject(players: Vec<Arc<Mutex<Player>>>) -> Self {
        GameState {
            players,
            stack: Vec::new(),
        }
    }

    pub fn players(&self) -> &[Arc<Mutex<Player>>] {
        &self.players
    }
}
