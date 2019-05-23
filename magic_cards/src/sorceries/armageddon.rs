use magic_core::action::{ActionResolver, ActivatedAction};
use magic_core::card::{Card, CardBuilder, Type};
use magic_core::event::*;
use magic_core::mana::ManaCost;
use magic_core::state::State;
use magic_core::ui::UserInterface;

pub fn armageddon() -> Card {
    CardBuilder::new()
        .with_name("Armageddon")
        .with_mana_cost(ManaCost::new().with_white(1).with_generic(3))
        .with_type(Type::Sorcery)
        .on_resolve(CastArmageddon)
        .build()
}

#[derive(Debug)]
struct CastArmageddon;
impl ActionResolver for CastArmageddon {
    fn resolve(&self, state: &State, _: &mut UserInterface, action: ActivatedAction) -> Vec<Event> {
        state
            .instances()
            .filter(|instance| instance.get(state).types.contains(&Type::Land))
            .map(|instance| {
                Event::State(
                    action.source.clone(),
                    StateEvent::Card(instance, CardEvent::Destroy),
                )
            })
            .collect()
    }
}
