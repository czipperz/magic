use crate::event::Event;
use crate::instance::InstanceID;
use crate::state::State;
use std::fmt::Debug;

pub trait ReplacementEffect: Debug {
    fn replace(
        &self,
        state: &State,
        this_card: InstanceID,
        event: &Event,
    ) -> Option<Vec<Event>>;
}
