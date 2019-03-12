use crate::action::{Target, TargetDescription};
use crate::source::Source;
use crate::state::State;

pub trait UserInterface {
    fn choose_target(
        &mut self,
        state: &State,
        source: &Source,
        description: TargetDescription,
    ) -> Target;
}
