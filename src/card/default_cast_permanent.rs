use super::{ResolveAction, Zone};
use crate::event::{Event, SourcedEvent};
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
    ) -> SourcedEvent {
        Event::move_to_zone(stack_item.card.clone(), Zone::Battlefield)
            .with_source(stack_item.card.into())
    }
}
