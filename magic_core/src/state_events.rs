use crate::action::{ActionType, SourcedAction};
use crate::event::Event;
use crate::source::Source;
use crate::state::State;

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

    pub fn replacement_effects(&self, initial_events: Vec<Event>) -> Vec<Event> {
        initial_events
    }
}
