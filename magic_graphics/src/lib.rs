use magic_core::action::{Target, TargetDescription};
use magic_core::state::State;
use magic_core::ui::UserInterface;

pub struct GraphicalUserInterface {}

impl GraphicalUserInterface {
    pub fn new() -> Self {
        GraphicalUserInterface {}
    }
}

impl UserInterface for GraphicalUserInterface {}
