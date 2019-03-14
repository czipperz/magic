use crate::event::Event;
use crate::instance::InstanceNumber;
use crate::state::State;

pub trait ReplacementEffect {
    fn replace(
        &self,
        state: &State,
        this_card: InstanceNumber,
        event: &Event,
    ) -> Option<Vec<Event>>;
}
