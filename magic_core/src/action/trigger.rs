use crate::action::Action;
use crate::event::Event;
use crate::instance::InstanceID;
use crate::state::State;
use std::fmt::Debug;

pub trait Trigger: Debug {
    fn respond(&self, state: &State, this_card: InstanceID, event: &Event) -> Option<Action>;
}
