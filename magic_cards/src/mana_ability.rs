use magic_core::action::{Action, ActionResolver, ActivatedAction};
use magic_core::event::{Event, PlayerEvent, StateEvent};
use magic_core::mana::ManaPool;
use magic_core::state::State;
use magic_core::ui::UserInterface;

pub fn add_mana(mp: ManaPool) -> Action {
    Action::from(AddMana(mp)).as_mana_ability()
}

#[derive(Debug)]
struct AddMana(ManaPool);
impl ActionResolver for AddMana {
    fn resolve(&self, _: &State, _: &mut UserInterface, action: ActivatedAction) -> Vec<Event> {
        let controller = action.source.controller;
        vec![Event::State(
            action.source,
            StateEvent::Player(controller, PlayerEvent::AddMana(self.0.clone())),
        )]
    }
}
