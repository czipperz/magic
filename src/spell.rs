use crate::card::Payment;
use crate::card::{Instance, Subtype};
use crate::event::{Event, EventV};
use crate::mana::Color;
use crate::permanent::Permanent;
use crate::player::PlayerNumber;
use std::sync::{Arc, Mutex};

pub struct Spell {
    card: Arc<Mutex<Instance>>,
    payment: Payment,
    targets: Vec<Target>,
    color_words: Vec<Color>,
    subtypes: Vec<Subtype>,
}

pub enum Target {
    Player(PlayerNumber),
    Permanent(Arc<Mutex<Permanent>>),
    Card(Arc<Mutex<Instance>>),
    Spell(Arc<Mutex<Spell>>),
}

impl Spell {
    pub fn from_card(card: Arc<Mutex<Instance>>, payment: Payment, targets: Vec<Target>) -> Self {
        let (color_words, subtypes) = {
            let card_locked = card.lock().unwrap();
            (
                card_locked.color_words().clone(),
                card_locked.subtypes().clone(),
            )
        };
        Spell {
            card,
            payment,
            targets,
            color_words,
            subtypes,
        }
    }

    pub fn cast(card: Arc<Mutex<Instance>>, payment: Payment, targets: Vec<Target>) -> Event {
        Event {
            source: card.clone().into(),
            v: EventV::Cast(Spell::from_card(card, payment, targets)),
        }
    }
}
