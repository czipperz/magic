use super::UserInterface;
use crate::card::Card;
use crate::player::PlayerNumber;
use crate::state::State;
use std::sync::{Arc, Mutex};

pub struct TestingUI {}

impl TestingUI {
    pub fn new() -> Self {
        TestingUI {}
    }
}

impl UserInterface for TestingUI {
    fn select_player(&mut self, state: &State) -> Option<PlayerNumber> {
        unimplemented!()
    }

    fn select_card(
        &mut self,
        state: &State,
        predicate: &Fn(&State, &Card) -> bool,
    ) -> Option<Arc<Mutex<Card>>> {
        unimplemented!()
    }
}
