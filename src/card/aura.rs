use crate::card::Card;
use crate::card::CardDecoration;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Aura {
    pub card: Arc<Mutex<Card>>,
    pub decoration: Box<CardDecoration>,
}
