use crate::card::Card;
use std::sync::{Arc, Mutex};

pub type PlayerNumber = usize;

pub struct Player {
    deck: Vec<Arc<Mutex<Card>>>,
    hand: Vec<Arc<Mutex<Card>>>,
    battlefield: Vec<Arc<Mutex<Card>>>,
    graveyard: Vec<Arc<Mutex<Card>>>,
    exile: Vec<Arc<Mutex<Card>>>,
}
