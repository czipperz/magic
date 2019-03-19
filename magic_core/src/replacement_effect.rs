use crate::event::Event;
use crate::instance::InstanceNumber;
use crate::state::State;
use std::fmt::Debug;

pub trait ReplacementEffect: Debug {
    fn replace(
        &self,
        state: &State,
        this_card: InstanceNumber,
        event: &Event,
    ) -> Option<Vec<Event>>;
}
