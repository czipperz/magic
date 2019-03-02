use crate::card::Card;
use crate::player::*;
use crate::user_interface::UserInterface;
use std::sync::{Arc, Mutex};

pub struct State {
    players: Vec<Player>,
    stack: Vec<Arc<Mutex<Card>>>,
    ui: UserInterface,
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
        unimplemented!()
    }

    pub fn make_player_draw_cards(&mut self, player: PlayerNumber, cards: usize) {
        unimplemented!()
    }

    pub fn is_any_card_targetable_by(
        &self,
        controller: PlayerNumber,
        predicate: &impl Fn(&State, &Card) -> bool,
    ) -> bool {
        unimplemented!()
    }

    pub fn select_target_card(
        &mut self,
        controller: PlayerNumber,
        predicate: &impl Fn(&State, &Card) -> bool,
    ) -> Option<Arc<Mutex<Card>>> {
        unimplemented!()
    }
}
