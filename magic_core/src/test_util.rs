use crate::action::{ActionResolver, ActivatedAction};
use crate::event::Event;
use crate::state::State;

#[derive(Debug)]
pub struct MockResolve;
impl ActionResolver for MockResolve {
    fn resolve(&self, _: &State, _: ActivatedAction) -> Vec<Event> {
        panic!()
    }
}
