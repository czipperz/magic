use crate::action::Action;
use crate::event::Event;
use crate::instance::InstanceNumber;
use crate::state::State;

pub trait Trigger {
    fn respond(&self, state: &State, this_card: InstanceNumber, event: &Event) -> Option<Action>;
}
