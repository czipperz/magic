use super::{Instance, ResolveAction, Zone};
use crate::event::Event;
use crate::game_state::GameState;
use crate::spell::StackItem;
use crate::ui::UserInterface;

/// `DefaultCastPermanent` handles the default implementation of
/// putting a permanent on the battlefield.
///
/// If your card is
#[derive(Debug)]
pub struct DefaultCastPermanent;

impl ResolveAction for DefaultCastPermanent {
    fn resolve(
        &self,
        _game_state: &mut GameState,
        _ui: &mut UserInterface,
        stack_item: StackItem,
    ) -> Event {
        Event {
            source: stack_item.card.clone().into(),
            v: Instance::move_to_zone(stack_item.card, Zone::Battlefield),
        }
    }
}
