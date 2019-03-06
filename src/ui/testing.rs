use super::UserInterface;
use crate::card::Instance;
use crate::game_state::GameState;
use crate::permanent::Permanent;
use crate::player::PlayerNumber;
use crate::source::Source;
use std::sync::{Arc, Mutex};

pub struct TestingUI {}

impl TestingUI {
    pub fn new() -> Self {
        TestingUI {}
    }
}

impl UserInterface for TestingUI {
    fn choose_player(&mut self, _game_state: &GameState, _source: &Source) -> Option<PlayerNumber> {
        unimplemented!()
    }

    fn choose_permanent(
        &mut self,
        _game_state: &GameState,
        _source: &Source,
        _predicate: &Fn(&Arc<Mutex<Permanent>>) -> bool,
    ) -> Option<Arc<Mutex<Permanent>>> {
        unimplemented!()
    }

    fn choose_card(
        &mut self,
        _state: &GameState,
        _source: &Source,
        _predicate: &Fn(&Arc<Mutex<Instance>>) -> bool,
    ) -> Option<Arc<Mutex<Instance>>> {
        unimplemented!()
    }
}
