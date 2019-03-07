use crate::card::Payment;
use crate::card::{Instance, Subtype};
use crate::mana::Color;
use crate::permanent::Permanent;
use crate::player::PlayerNumber;
use std::sync::{Arc, Mutex};

pub type Spell = StackItem;
pub type Ability = StackItem;

pub struct StackItem {
    pub card: Arc<Mutex<Instance>>,
    pub mandatory_payments: Vec<Payment>,
    pub optional_payments: Vec<Option<Payment>>,
    pub targets: Vec<Target>,
    pub color_words: Vec<Color>,
    pub subtypes: Vec<Subtype>,
}

pub enum Target {
    Player(PlayerNumber),
    Permanent(Arc<Mutex<Permanent>>),
    Card(Arc<Mutex<Instance>>),
    Spell(Arc<Mutex<Spell>>),
}

impl StackItem {
    pub fn new(
        card: Arc<Mutex<Instance>>,
        mandatory_payments: Vec<Payment>,
        optional_payments: Vec<Option<Payment>>,
        targets: Vec<Target>,
    ) -> Self {
        let (color_words, subtypes) = {
            let card_locked = card.lock().unwrap();
            (
                card_locked.color_words().clone(),
                card_locked.subtypes().clone(),
            )
        };
        Spell {
            card,
            mandatory_payments,
            optional_payments,
            targets,
            color_words,
            subtypes,
        }
    }
}
