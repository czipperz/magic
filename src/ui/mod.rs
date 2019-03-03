use crate::card::Card;
use crate::player::PlayerNumber;
use crate::source::Source;
use crate::state::State;
use std::sync::{Arc, Mutex};

pub trait UserInterface {
    fn select_player(&mut self, state: &State, source: &Source) -> Option<PlayerNumber>;

    fn select_card(
        &mut self,
        state: &State,
        source: &Source,
        predicate: &Fn(&State, &Card) -> bool,
    ) -> Option<Arc<Mutex<Card>>>;
}

mod testing;
pub use self::testing::*;
