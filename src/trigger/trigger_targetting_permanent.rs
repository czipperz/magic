use super::Trigger;
use crate::card::Instance;
use crate::event::Event;
use crate::game_state::GameState;
use crate::permanent::Permanent;
use crate::ui::UserInterface;
use std::fmt;
use std::sync::{Arc, Mutex};

struct TriggerTargettingPermanent<EP, PP, C> {
    event_predicate: EP,
    permanent_predicate: PP,
    create_event: C,
}

pub fn target_permanent<EventPredicate, PermanentPredicate, CreateEvent>(
    event_predicate: EventPredicate,
    permanent_predicate: PermanentPredicate,
    create_event: CreateEvent,
) -> TriggerTargettingPermanent<EventPredicate, PermanentPredicate, CreateEvent>
where
    EventPredicate: Fn(&GameState, &Event) -> bool,
    PermanentPredicate: Fn(&GameState, &Event, &Arc<Mutex<Permanent>>) -> bool,
    CreateEvent: Fn(&GameState, &Event, Arc<Mutex<Instance>>, Arc<Mutex<Permanent>>) -> Event,
{
    TriggerTargettingPermanent {
        event_predicate,
        permanent_predicate,
        create_event,
    }
}

impl<EP, PP, C> Trigger for TriggerTargettingPermanent<EP, PP, C>
where
    EP: Fn(&GameState, &Event, &Arc<Mutex<Instance>>) -> bool,
    PP: Fn(&GameState, &Arc<Mutex<Instance>>, &Arc<Mutex<Permanent>>) -> bool,
    C: Fn(&GameState, Arc<Mutex<Instance>>, Arc<Mutex<Permanent>>) -> Event,
{
    fn respond_to_event(
        &self,
        ui: &mut UserInterface,
        game_state: &GameState,
        event: &Event,
        this_card: Arc<Mutex<Instance>>,
    ) -> Option<Event> {
        if (self.event_predicate)(game_state, event, &this_card) {
            let source = this_card.into();
            let predicate = |card| (self.permanent_predicate)(game_state, &this_card, card);
            if let Some(permanent) = ui.choose_permanent(game_state, &source, &predicate) {
                Some((self.create_event)(game_state, this_card, permanent))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<EP, PP, C> fmt::Debug for TriggerTargettingPermanent<EP, PP, C> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "TriggerTargettingPermanent")
    }
}
