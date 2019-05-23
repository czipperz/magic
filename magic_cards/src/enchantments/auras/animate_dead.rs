use super::aura::aura;
use magic_core::action::{
    Action, ActionResolver, ActivatedAction, Target, TargetDescription, Trigger,
};
use magic_core::card::{Card, Subtype, Type};
use magic_core::effect::Effect;
use magic_core::event::*;
use magic_core::instance::InstanceID;
use magic_core::mana::ManaCost;
use magic_core::state::State;
use magic_core::ui::UserInterface;
use magic_core::zone::Zone;

pub fn animate_dead() -> Card {
    aura(
        TargetDescription::graveyard(1, is_creature),
        AnimateDeadEffect,
    )
    .with_name("Animate Dead")
    .with_mana_cost(ManaCost::new().with_black(1).with_generic(1))
    .with_type(Type::Enchantment)
    .with_subtype(Subtype::Aura)
    .with_trigger(AnimateDeadEnterTheBattlefieldTrigger)
    .build()
}

fn is_creature(state: &State, instance: InstanceID) -> bool {
    instance.get(state).types.contains(&Type::Creature)
}

#[derive(Debug)]
struct AnimateDeadEnterTheBattlefieldTrigger;
impl Trigger for AnimateDeadEnterTheBattlefieldTrigger {
    fn respond(&self, _state: &State, instance: InstanceID, event: &Event) -> Option<Action> {
        match event {
            Event::State(_, StateEvent::Card(card, CardEvent::MoveTo(_, Zone::Battlefield)))
                if *card == instance =>
            {
                Some(Action::from(AnimateDeadEnterTheBattlefieldAction))
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct AnimateDeadEnterTheBattlefieldAction;
impl ActionResolver for AnimateDeadEnterTheBattlefieldAction {
    fn resolve(&self, state: &State, _: &mut UserInterface, action: ActivatedAction) -> Vec<Event> {
        let aura_id = action.source.instance;
        let aura = aura_id.get(state);
        match &aura.attached_to {
            Some(Target::Instance(creatures)) => {
                assert_eq!(creatures.len(), 1);
                let creature = creatures[0];
                vec![
                    Event::move_to_zone(state, action.source.clone(), creature, Zone::Battlefield),
                    Event::State(
                        action.source,
                        StateEvent::Card(
                            aura_id,
                            CardEvent::attach_to(
                                Target::Instance(vec![creature]),
                                AnimateDeadEffect,
                            ),
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
    fn affect(&self, _: &State, _: InstanceID, card: &mut Card) {
        card.power.as_mut().map(|power| *power -= 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;
    use magic_core::player::PlayerID;

    #[test]
    fn test_constructor() {
        let card = animate_dead();
        assert_eq!(card.name, "Animate Dead");
        assert_eq!(
            *card.mana_cost(),
            ManaCost::new().with_black(1).with_generic(1)
        );
        assert_eq!(card.types, &[Type::Enchantment]);
        assert_eq!(card.subtypes, &[Subtype::Aura]);
        assert_eq!(card.triggers.len(), 2);
    }

    fn respond_event(player: PlayerID, instance: InstanceID) -> Event {
        use magic_core::source::Source;
        Event::State(
            Source {
                controller: player,
                instance,
            },
            StateEvent::Card(instance, CardEvent::MoveTo(player, Zone::Battlefield)),
        )
    }

    #[test]
    fn test_respond() {
        let (state, instance) = state_with_card(base_card().build());
        let event = respond_event(state.players()[0], instance);

        let action = AnimateDeadEnterTheBattlefieldTrigger.respond(&state, instance, &event);

        assert!(action.is_some());
    }

    #[test]
    fn test_respond_different_card() {
        let (state, instances) = state_with_cards(vec![base_card().build(), base_card().build()]);
        let event = respond_event(state.players()[0], instances[0]);

        let action = AnimateDeadEnterTheBattlefieldTrigger.respond(&state, instances[1], &event);

        assert!(action.is_none());
    }

    #[test]
    fn test_respond_no_match() {
        let (state, instance) = state_with_card(base_card().build());
        use magic_core::source::Source;
        let event = Event::State(
            Source {
                controller: state.players()[0],
                instance,
            },
            StateEvent::Card(instance, CardEvent::Tap),
        );

        let action = AnimateDeadEnterTheBattlefieldTrigger.respond(&state, instance, &event);

        assert!(action.is_none());
    }
}
