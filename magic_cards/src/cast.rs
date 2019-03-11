use magic_core::action::{ActivatedAction, ResolveAction};
use magic_core::event::*;
use magic_core::source::Source;
use magic_core::state::State;
use magic_core::zone::Zone;

pub fn put_on_battlefield(state: &State, source: Source) -> Event {
    let instance = source.instance;
    Event::move_to_zone(state, source, instance, Zone::Battlefield)
}

pub struct CastPermanent;
impl ResolveAction for CastPermanent {
    fn resolve(&self, state: &State, action: ActivatedAction) -> Vec<Event> {
        vec![put_on_battlefield(state, action.source)]
    }
}
