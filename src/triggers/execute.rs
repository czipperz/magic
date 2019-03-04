use super::{Bundle, Trigger};
use crate::card::Card;
use crate::state::State;
use std::sync::{Arc, Mutex};

pub struct TriggerOnExecute<C> {
    callback: C,
}

impl<C> TriggerOnExecute<C>
where
    C: Fn(&mut State, Arc<Mutex<Card>>) -> bool,
{
    pub fn new(callback: C) -> Self {
        TriggerOnExecute { callback }
    }
}

impl<C> Trigger for TriggerOnExecute<C>
where
    C: Fn(&mut State, Arc<Mutex<Card>>) -> bool,
{
    fn can_execute(&self, _: &State, _: &Bundle, _: Arc<Mutex<Card>>) -> bool {
        true
    }

    fn try_execute(&self, _: &mut State, _: &mut Bundle, _: Arc<Mutex<Card>>) -> bool {
        true
    }

    fn on_execute(&self, state: &mut State, _: &mut Bundle, card: Arc<Mutex<Card>>) -> bool {
        (self.callback)(state, card)
    }
}
