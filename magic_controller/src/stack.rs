use magic_core::action::{ActionResolver, ActivatedAction};
use magic_core::event::Event;
use magic_core::state::State;
use std::sync::Arc;

#[derive(Default)]
pub struct Stack {
    actions: Vec<(Arc<ActionResolver>, ActivatedAction)>,
}

impl Stack {
    fn pop(&mut self, state: &State) -> Option<Vec<Event>> {
        self.actions
            .pop()
            .map(|(resolve, action)| resolve.resolve(state, action))
    }

    pub fn push(&mut self, resolve_action: Arc<ActionResolver>, action: ActivatedAction) {
        self.actions.push((resolve_action, action));
    }
}