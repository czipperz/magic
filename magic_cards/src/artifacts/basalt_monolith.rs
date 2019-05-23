use magic_core::action::{Action, ActionResolver, ActivatedAction, Cost};
use magic_core::card::{Card, CardBuilder};
use magic_core::event::{CardEvent, Event, PlayerEvent, StateEvent};
use magic_core::instance::InstanceID;
use magic_core::mana::{ManaCost, ManaPool};
use magic_core::replacement_effect::ReplacementEffect;
use magic_core::state::State;
use magic_core::ui::UserInterface;

pub fn basalt_monolith() -> Card {
    CardBuilder::new()
        .with_name("Basalt Monolith")
        .with_mana_cost(ManaCost::new().with_generic(3))
        .with_replacement_effect(BasaltMonolithDoesntUntapInUntapStep)
        .with_ability(
            Action::from(UntapThisPermanent)
                .with_mandatory_cost(Cost::Mana(ManaCost::new().with_generic(3))),
        )
        .with_ability(Action::from(Add3Mana).with_mandatory_cost(Cost::Tap))
        .build()
}

#[derive(Debug)]
struct BasaltMonolithDoesntUntapInUntapStep;
impl ReplacementEffect for BasaltMonolithDoesntUntapInUntapStep {
    fn replace(
        &self,
        _: &State,
        _: &mut UserInterface,
        this_card: InstanceID,
        event: &Event,
    ) -> Option<Vec<Event>> {
        if let Event::State(_, StateEvent::Card(card, CardEvent::Tap)) = event {
            if *card == this_card {
                return Some(vec![]);
            }
        }
        None
    }
}

#[derive(Debug)]
struct UntapThisPermanent;
impl ActionResolver for UntapThisPermanent {
    fn resolve(&self, _: &State, _: &mut UserInterface, action: ActivatedAction) -> Vec<Event> {
        let instance = action.source.instance;
        vec![Event::State(
            action.source,
            StateEvent::Card(instance, CardEvent::Untap),
        )]
    }
}

#[derive(Debug)]
struct Add3Mana;
impl ActionResolver for Add3Mana {
    fn resolve(&self, _: &State, _: &mut UserInterface, action: ActivatedAction) -> Vec<Event> {
        let controller = action.source.controller;
        vec![Event::State(
            action.source,
            StateEvent::Player(
                controller,
                PlayerEvent::AddMana(ManaPool::new().with_colorless(3)),
            ),
        )]
    }
}
