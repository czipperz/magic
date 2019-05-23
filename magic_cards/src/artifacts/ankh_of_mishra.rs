use crate::cast::CastPermanent;
use magic_core::action::{Action, ActionResolver, ActivatedAction, Trigger};
use magic_core::card::{Card, CardBuilder, Type};
use magic_core::event::*;
use magic_core::instance::InstanceID;
use magic_core::mana::ManaCost;
use magic_core::player::PlayerID;
use magic_core::state::State;
use magic_core::ui::UserInterface;
use magic_core::zone::Zone;

pub fn ankh_of_mishra() -> Card {
    CardBuilder::new()
        .with_name("Ankh of Mishra")
        .with_mana_cost(ManaCost::new().with_generic(2))
        .on_resolve(CastPermanent)
        .with_type(Type::Artifact)
        .with_trigger(AnkhOfMishraTrigger)
        .build()
}

#[derive(Debug)]
struct AnkhOfMishraTrigger;
impl Trigger for AnkhOfMishraTrigger {
    fn respond(&self, state: &State, _ankh: InstanceID, event: &Event) -> Option<Action> {
        if let Event::State(
            _,
            StateEvent::Card(instance, CardEvent::MoveTo(_, Zone::Battlefield)),
        ) = event
        {
            let instance = instance.get(state);
            if instance.zone != Zone::Battlefield && instance.types.contains(&Type::Land) {
                return Some(Action::from(AnkhOfMishraAction {
                    controller: instance.controller,
                }));
            }
        }
        None
    }
}

#[derive(Debug)]
struct AnkhOfMishraAction {
    controller: PlayerID,
}
impl ActionResolver for AnkhOfMishraAction {
    fn resolve(&self, _: &State, _: &mut UserInterface, action: ActivatedAction) -> Vec<Event> {
        vec![Event::State(
            action.source,
            StateEvent::Player(self.controller, PlayerEvent::TakeDamage(2)),
        )]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_construction() {
        let card = ankh_of_mishra();
        assert_eq!(card.name, "Ankh of Mishra");
        assert_eq!(*card.mana_cost(), ManaCost::new().with_generic(2));
        assert_eq!(card.types, &[Type::Artifact]);
        assert_eq!(card.triggers.len(), 1);
    }
}
