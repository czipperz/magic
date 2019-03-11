use crate::cast::CastPermanent;
use magic_core::action::{Action, ActivatedAction, ResolveAction, Trigger};
use magic_core::card::{Card, Type};
use magic_core::event::*;
use magic_core::instance::InstanceNumber;
use magic_core::mana::ManaCost;
use magic_core::player::PlayerNumber;
use magic_core::state::State;
use magic_core::zone::Zone;

pub fn ankh_of_mishra() -> Card {
    Card::new(
        "Ankh of Mishra",
        ManaCost::new().with_generic(2),
        CastPermanent,
    )
    .with_type(Type::Artifact)
    .with_trigger(AnkhOfMishraTrigger)
}

struct AnkhOfMishraTrigger;
impl Trigger for AnkhOfMishraTrigger {
    fn respond(&self, state: &State, _ankh: InstanceNumber, event: &Event) -> Option<Action> {
        match event {
            Event::State(
                _,
                StateEvent::Card(card, CardEvent::MoveTo(_, old_zone, _, Zone::Battlefield)),
            ) if *old_zone != Zone::Battlefield && card.card(state).types.contains(&Type::Land) => {
                Some(Action::from(AnkhOfMishraAction {
                    controller: card.get(state).controller,
                }))
            }
            _ => None,
        }
    }
}

struct AnkhOfMishraAction {
    controller: PlayerNumber,
}
impl ResolveAction for AnkhOfMishraAction {
    fn resolve(&self, _: &State, action: ActivatedAction) -> Vec<Event> {
        vec![Event::State(
            action.source,
            StateEvent::Player(self.controller, PlayerEvent::TakeDamage(2)),
        )]
    }
}
