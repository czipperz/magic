use magic_core::action::{ActionResolver, ActivatedAction};
use magic_core::event::*;
use magic_core::source::Source;
use magic_core::state::State;
use magic_core::ui::UserInterface;
use magic_core::zone::Zone;

pub fn put_on_battlefield(state: &State, source: Source) -> Event {
    let instance = source.instance;
    Event::move_to_zone(state, source, instance, Zone::Battlefield)
}

#[derive(Debug)]
pub struct CastPermanent;
impl ActionResolver for CastPermanent {
    fn resolve(&self, state: &State, _: &mut UserInterface, action: ActivatedAction) -> Vec<Event> {
        vec![put_on_battlefield(state, action.source)]
    }
}
