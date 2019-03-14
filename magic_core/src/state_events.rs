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
                let permanent = instance.get(&self).permanent(&self).unwrap();
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
                let permanent = instance.get(&self).permanent(&self).unwrap();
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
