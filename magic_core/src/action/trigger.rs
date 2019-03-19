use crate::action::Action;
use crate::event::Event;
use crate::instance::InstanceNumber;
use crate::state::State;
use std::fmt::Debug;

pub trait Trigger: Debug {
    fn respond(&self, state: &State, this_card: InstanceNumber, event: &Event) -> Option<Action>;
}
