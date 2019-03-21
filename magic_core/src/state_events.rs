use crate::action::{ActionType, SourcedAction};
use crate::event::Event;
use crate::instance::InstanceNumber;
use crate::source::Source;
use crate::state::State;
use crate::ui::UserInterface;
use std::collections::HashSet;

impl State {
    pub fn trigger(&self, event: &Event) -> Vec<SourcedAction> {
        let mut actions = Vec::new();
        for player_number in self.players() {
            let player = player_number.get(&self);
            for instance in &player.battlefield {
                let permanent = instance.permanent(&self).unwrap();
                for trigger in &permanent.triggers {
                    if let Some(action) = trigger.respond(&self, *instance, &event) {
                        let source = Source {
                            controller: player_number,
                            instance: *instance,
                        };
                        actions.push(SourcedAction {
                            action_type: ActionType::TriggeredAbility,
                            source,
                            action,
                        })
                    }
                }
            }
        }
        actions
    }

    pub fn replacement_effects(
        &self,
        ui: &mut UserInterface,
        initial_events: Vec<Event>,
    ) -> Vec<Event> {
        let mut events = Vec::new();
        let mut history = HashSet::new();
        for event in initial_events {
            self.replacement_effects_(ui, &mut events, &mut history, event);
            history.clear();
        }
        events
    }

    fn replacement_effects_(
        &self,
        ui: &mut UserInterface,
        events: &mut Vec<Event>,
        history: &mut HashSet<(InstanceNumber, usize)>,
        event: Event,
    ) {
        let mut replacements = Vec::new();
        for player_number in self.players() {
            let player = player_number.get(&self);
            for instance in &player.battlefield {
                let permanent = instance.permanent(&self).unwrap();
                for (index, replacement_effect) in permanent.replacement_effects.iter().enumerate()
                {
                    if !history.contains(&(*instance, index)) {
                        if let Some(replacement) =
                            replacement_effect.replace(&self, *instance, &event)
                        {
                            replacements.push((instance, index, replacement));
                        }
                    }
                }
            }
        }

        if replacements.len() == 0 {
            events.push(event);
        } else if replacements.len() == 1 {
            let (instance, index, replacement) = replacements.into_iter().next().unwrap();
            history.insert((*instance, index));
            for rep in replacement {
                self.replacement_effects_(ui, events, history, rep);
            }
        } else {
            // ui choose option
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CardBuilder;
    use crate::event::TurnEvent;
    use crate::player::PlayerNumber;
    use crate::test_util::MockResolve;
    use crate::turn::Phase;

    #[test]
    fn test_trigger_no_cards() {
        let state = State::new(20, vec![vec![]]);
        let event = Event::Turn(
            PlayerNumber { number: 0 },
            TurnEvent::BeginPhase(Phase::Beginning),
        );

        assert_eq!(state.trigger(&event), vec![]);
    }

    fn mock_card() -> CardBuilder {
        CardBuilder::new()
            .with_name("Mock")
            .with_colors(vec![])
            .on_resolve(MockResolve)
    }

    #[test]
    fn test_trigger_card_no_triggers() {
        let card = mock_card().build();
        let state = State::new(20, vec![vec![card]]);
        let event = Event::Turn(
            PlayerNumber { number: 0 },
            TurnEvent::BeginPhase(Phase::Beginning),
        );

        assert_eq!(state.trigger(&event), vec![]);
    }

    use crate::action::{Action, Trigger};
    #[derive(Debug)]
    struct AlwaysTrigger;
    impl Trigger for AlwaysTrigger {
        fn respond(&self, _: &State, _: InstanceNumber, _: &Event) -> Option<Action> {
            Some(Action::from(MockResolve))
        }
    }

    #[test]
    fn test_trigger_card_one_trigger_in_deck() {
        let card = mock_card().with_trigger(AlwaysTrigger).build();
        let state = State::new(20, vec![vec![card]]);
        let event = Event::Turn(
            PlayerNumber { number: 0 },
            TurnEvent::BeginPhase(Phase::Beginning),
        );

        assert_eq!(state.trigger(&event), vec![]);
    }

    #[test]
    fn test_trigger_card_one_trigger_on_battlefield() {
        let card = mock_card().with_trigger(AlwaysTrigger).build();
        let mut state = State::new(20, vec![vec![card]]);

        let player_number = PlayerNumber { number: 0 };
        let player = state.player_mut(player_number);
        let instance = player.deck.pop().unwrap();
        player.battlefield.push(instance);
        state.add_permanent(instance);
        let event = Event::Turn(player_number, TurnEvent::BeginPhase(Phase::Beginning));

        let actions = state.trigger(&event);
        assert_eq!(actions.len(), 1);
        let action = &actions[0];
        assert_eq!(action.action_type, ActionType::TriggeredAbility);
        assert_eq!(
            action.source,
            Source {
                instance,
                controller: player_number,
            }
        );
    }

    #[test]
    fn test_trigger_card_two_triggers_on_battlefield() {
        let card = mock_card().with_trigger(AlwaysTrigger).build();
        let mut state = State::new(20, vec![vec![card.clone()], vec![card]]);
        for number in 0..=1 {
            let player_number = PlayerNumber { number };
            let player = state.player_mut(player_number);
            let instance = player.deck.pop().unwrap();
            player.battlefield.push(instance);
            state.add_permanent(instance);
        }

        let event = Event::Turn(
            PlayerNumber { number: 0 },
            TurnEvent::BeginPhase(Phase::Beginning),
        );
        let actions = state.trigger(&event);
        assert_eq!(actions.len(), 2);
        for i in 0..=1 {
            let action = &actions[i];
            assert_eq!(action.action_type, ActionType::TriggeredAbility);
            assert_eq!(
                action.source,
                Source {
                    instance: InstanceNumber { number: i },
                    controller: PlayerNumber { number: i },
                }
            );
        }
    }

    #[test]
    fn test_trigger_card_mixture_trigger_notrigger_on_battlefield() {
        let card_trigger = mock_card().with_trigger(AlwaysTrigger).build();
        let card_notrigger = mock_card().build();
        let mut state = State::new(
            20,
            vec![
                vec![
                    card_trigger.clone(),
                    card_notrigger.clone(),
                    card_notrigger,
                    card_trigger,
                ],
                vec![],
            ],
        );
        for index in &[3, 0] {
            let player_number = PlayerNumber { number: 0 };
            let player = state.player_mut(player_number);
            let instance = player.deck.remove(*index);
            player.battlefield.push(instance);
            state.add_permanent(instance);
        }

        let event = Event::Turn(
            PlayerNumber { number: 0 },
            TurnEvent::BeginPhase(Phase::Beginning),
        );
        let actions = state.trigger(&event);
        assert_eq!(actions.len(), 2);
        for i in 0..=1 {
            let action = &actions[i];
            assert_eq!(action.action_type, ActionType::TriggeredAbility);
            assert_eq!(
                action.source,
                Source {
                    instance: InstanceNumber { number: [3, 0][i] },
                    controller: PlayerNumber { number: 0 },
                }
            );
        }
    }
}
