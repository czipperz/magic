use super::{ActionEvent, Instance, Payment};
use crate::event::Event;
use crate::game_state::GameState;
use crate::spell::Target;
use crate::ui::UserInterface;
use std::sync::{Arc, Mutex};

/// CastAction handles the implementation of casting a card.
#[derive(Debug)]
pub struct CastAction;

impl ActionEvent for CastAction {
    fn execute(
        &self,
        _game_state: &mut GameState,
        _ui: &mut UserInterface,
        card: Arc<Mutex<Instance>>,
        mandatory_payment: Vec<Payment>,
        optional_payments: Vec<Option<Payment>>,
        targets: Vec<Target>,
    ) -> Event {
        Event::cast(
            card,
            Payment::Payments(
                mandatory_payment
                    .into_iter()
                    .chain(optional_payments.into_iter().filter_map(|p| p))
                    .collect(),
            ),
            targets,
        )
    }
}
