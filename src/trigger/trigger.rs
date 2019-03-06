use crate::card::Instance;
use crate::event::Event;
use crate::game_state::GameState;
use crate::ui::UserInterface;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

pub trait Trigger: Debug {
    fn mutate_event(&self, _game_state: &GameState, _event: &mut Event) -> bool {
        false
    }

    fn respond_to_event(
        &self,
        _ui: &mut UserInterface,
        _game_state: &GameState,
        _event: &Event,
        _this_card: Arc<Mutex<Instance>>,
    ) -> Option<Event> {
        None
    }
}
