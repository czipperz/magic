use crate::action::{ActionResolver, ActivatedAction};
use crate::event::Event;
use crate::state::State;
use crate::ui::UserInterface;

#[derive(Debug)]
pub struct MockResolve;
impl ActionResolver for MockResolve {
    fn resolve(&self, _: &State, _: &mut UserInterface, _: ActivatedAction) -> Vec<Event> {
        panic!()
    }
}
