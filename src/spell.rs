use crate::card::{Instance, Subtype};
use crate::mana::Color;
use crate::permanent::Permanent;
use crate::player::PlayerNumber;
use std::sync::{Arc, Mutex};

pub struct Spell {
    card: Arc<Mutex<Instance>>,
    targets: Vec<Target>,
    color_words: Vec<Color>,
    creature_types: Vec<Subtype>,
}

pub enum Target {
    Player(PlayerNumber),
    Permanent(Arc<Mutex<Permanent>>),
    Card(Arc<Mutex<Instance>>),
    Spell(Arc<Mutex<Spell>>),
}
