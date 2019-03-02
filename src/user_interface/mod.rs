use crate::card::Card;
use crate::player::PlayerNumber;
use crate::state::State;
use std::sync::{Arc, Mutex};

pub trait UserInterface {
    fn select_player(&mut self, state: &State) -> Option<PlayerNumber>;

    fn select_card(
        &mut self,
        state: &State,
        predicate: &Fn(&State, &Card) -> bool,
    ) -> Option<Arc<Mutex<Card>>>;
}
