use magic_core::action::{ActionResolver, ActivatedAction};
use magic_core::event::Event;
use magic_core::state::State;
use std::sync::Arc;

#[derive(Default)]
pub struct Stack {
    actions: Vec<(Arc<ActionResolver>, ActivatedAction)>,
}

impl Stack {
    pub fn pop(&mut self, state: &State) -> Option<Vec<Event>> {
        if let Some((resolver, action)) = self.actions.pop() {
            let events = resolver.resolve(state, action);
            // TODO: run replacement effects
            Some(events)
        } else {
            None
        }
    }

    pub fn push(&mut self, resolver: Arc<ActionResolver>, action: ActivatedAction) {
        self.actions.push((resolver, action));
    }
}
