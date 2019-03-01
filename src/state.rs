use crate::card::Card;
use crate::location::Location;
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
        predicate: &impl Fn(&State, &Card, PlayerNumber, Location) -> bool,
    ) -> bool {
        unimplemented!()
    }

    pub fn is_target_card_valid(
        &self,
        card: &Card,
        controller: PlayerNumber,
        predicate: &impl Fn(&State, &Card, PlayerNumber, Location) -> bool,
    ) -> bool {
        unimplemented!()
    }

    pub fn select_target_card(
        &mut self,
        controller: PlayerNumber,
        predicate: &impl Fn(&State, &Card, PlayerNumber, Location) -> bool,
    ) -> Option<Arc<Mutex<Card>>> {
        unimplemented!()
    }

    pub fn move_card(
        &mut self,
        card: Arc<Mutex<Card>>,
        old_controller: PlayerNumber,
        old_location: Location,
        new_controller: PlayerNumber,
        new_location: Location,
    ) {
        unimplemented!()
    }
}
