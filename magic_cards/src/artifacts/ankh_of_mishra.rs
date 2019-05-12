use crate::cast::CastPermanent;
use magic_core::action::{Action, ActionResolver, ActivatedAction, Trigger};
use magic_core::card::{Card, CardBuilder, Type};
use magic_core::event::*;
use magic_core::instance::InstanceID;
use magic_core::mana::ManaCost;
use magic_core::player::PlayerID;
use magic_core::state::State;
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
        match event {
            Event::State(
                _,
                StateEvent::Card(instance, CardEvent::MoveTo(_, Zone::Battlefield)),
            ) if instance.get(state).zone != Zone::Battlefield
                && instance.card(state).types.contains(&Type::Land) =>
            {
                Some(Action::from(AnkhOfMishraAction {
                    controller: instance.get(state).controller,
                }))
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct AnkhOfMishraAction {
    controller: PlayerID,
}
impl ActionResolver for AnkhOfMishraAction {
    fn resolve(&self, _: &State, action: ActivatedAction) -> Vec<Event> {
        vec![Event::State(
            action.source,
            StateEvent::Player(self.controller, PlayerEvent::TakeDamage(2)),
        )]
    }
}
