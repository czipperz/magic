use crate::card::Instance;
use crate::event::{Event, SourcedEvent};
use crate::game_state::GameState;
use crate::source::Source;
use crate::ui::UserInterface;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

pub trait Trigger: Debug {
    fn mutate_event(&self, _game_state: &GameState, _source: &Source, _event: &mut Event) -> bool {
        false
    }

    fn respond_to_event(
        &self,
        _game_state: &GameState,
        _ui: &mut UserInterface,
        _event: &SourcedEvent,
        _this_card: Arc<Mutex<Instance>>,
    ) -> Option<SourcedEvent> {
        None
    }
}
