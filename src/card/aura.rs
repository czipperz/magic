use crate::card::Card;
use crate::card::CardDecoration;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Aura {
    pub card: Arc<Mutex<Card>>,
    pub decoration: Box<CardDecoration>,
}

impl Aura {
    pub fn new(card: Arc<Mutex<Card>>, decoration: impl CardDecoration + 'static) -> Self {
        Aura {
            card,
            decoration: Box::new(decoration),
        }
    }
}
