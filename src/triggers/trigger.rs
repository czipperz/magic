use crate::bundle::Bundle;
use crate::card::Card;
use crate::state::State;
use std::sync::{Arc, Mutex};

pub trait Trigger {
    fn can_execute(&self, state: &State, bundle: &Bundle, card: Arc<Mutex<Card>>) -> bool;

    fn try_execute(&self, state: &mut State, bundle: &mut Bundle, card: Arc<Mutex<Card>>) -> bool;

    fn on_execute(&self, state: &mut State, bundle: &mut Bundle, card: Arc<Mutex<Card>>) -> bool;
}
