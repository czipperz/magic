use magic_core::action::{Target, TargetDescription};
use magic_core::source::Source;
use magic_core::state::State;
use magic_core::ui::UserInterface;

pub struct GraphicalUserInterface {}

impl GraphicalUserInterface {
    pub fn new() -> Self {
        GraphicalUserInterface {}
    }
}

impl UserInterface for GraphicalUserInterface {
    fn choose_target(
        &mut self,
        _state: &State,
        _source: &Source,
        _target_description: TargetDescription,
    ) -> Option<Target> {
        unimplemented!()
    }

    fn display(&mut self, _state: &State) {
        unimplemented!()
    }
}
