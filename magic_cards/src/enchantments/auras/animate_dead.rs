use super::aura::aura;
use magic_core::action::{
    Action, ActionResolver, ActivatedAction, Target, TargetDescription, Trigger,
};
use magic_core::card::{Card, Subtype, Type};
use magic_core::effect::Effect;
use magic_core::event::*;
use magic_core::instance::InstanceNumber;
use magic_core::mana::ManaCost;
use magic_core::permanent::Permanent;
use magic_core::state::State;
use magic_core::zone::Zone;

pub fn animate_dead() -> Card {
    aura(
        "Animate Dead",
        ManaCost::new().with_black(1).with_generic(1),
        TargetDescription::graveyard(1, is_creature),
    )
    .with_type(Type::Enchantment)
    .with_subtype(Subtype::Aura)
    .with_trigger(AnimateDeadEnterTheBattlefieldTrigger)
    .with_effect(AnimateDeadEffect)
}

fn is_creature(state: &State, instance: InstanceNumber) -> bool {
    instance.card(state).types.contains(&Type::Creature)
}

#[derive(Debug)]
struct AnimateDeadEnterTheBattlefieldTrigger;
impl Trigger for AnimateDeadEnterTheBattlefieldTrigger {
    fn respond(&self, _state: &State, instance: InstanceNumber, event: &Event) -> Option<Action> {
        match event {
            Event::State(
                _,
                StateEvent::Card(card, CardEvent::MoveTo(_, _, _, Zone::Battlefield)),
            ) if *card == instance => Some(Action::from(AnimateDeadEnterTheBattlefieldAction)),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct AnimateDeadEnterTheBattlefieldAction;
impl ActionResolver for AnimateDeadEnterTheBattlefieldAction {
    fn resolve(&self, state: &State, action: ActivatedAction) -> Vec<Event> {
        let aura_instance = action.source.instance;
        let aura = aura_instance.permanent(state).unwrap();
        match &aura.affecting {
            Some(Target::Graveyard(creatures)) => {
                assert_eq!(creatures.len(), 1);
                let creature = creatures[0];
                vec![
                    Event::move_to_zone(state, action.source.clone(), creature, Zone::Battlefield),
                    Event::State(
                        action.source,
                        StateEvent::Card(
                            aura_instance,
                            CardEvent::AttachTo(Target::Permanent(vec![creature])),
                        ),
                    ),
                ]
            }
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct AnimateDeadEffect;
impl Effect for AnimateDeadEffect {
    fn affect(&self, _: &State, permanent: &mut Permanent) {
        permanent.power.as_mut().map(|power| *power -= 1);
    }
}
