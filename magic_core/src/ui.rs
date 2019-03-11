use crate::action::{Target, TargetDescription};
use crate::state::State;

pub trait UserInterface {
    fn choose_target(&mut self, state: &State, description: TargetDescription) -> Target;
}
