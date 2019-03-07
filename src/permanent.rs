use crate::card::{Attribute, Instance, Subtype, Type};
use crate::mana::Color;
use crate::trigger::Trigger;
use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
pub struct Permanent {
    card: Arc<Mutex<Instance>>,
    effects: Vec<Arc<Mutex<Permanent>>>,
    effecting: Weak<Mutex<Permanent>>,

    colors: Vec<Color>,
    types: Vec<Type>,
    subtypes: Vec<Subtype>,
    attributes: Vec<Attribute>,
    triggers: Vec<Arc<Trigger>>,
    power: Option<isize>,
    toughness: Option<isize>,
}

impl From<Arc<Mutex<Instance>>> for Permanent {
    fn from(card: Arc<Mutex<Instance>>) -> Self {
        let (colors, types, subtypes, attributes, triggers, power, toughness) = {
            let card_locked = card.lock().unwrap();
            (
                card_locked.colors().clone(),
                card_locked.types().clone(),
                card_locked.subtypes().clone(),
                card_locked.attributes().clone(),
                card_locked.triggers().clone(),
                card_locked.power(),
                card_locked.toughness(),
            )
        };
        Permanent {
            card,
            effects: Vec::new(),
            effecting: Weak::new(),

            colors,
            types,
            subtypes,
            attributes,
            triggers,
            power,
            toughness,
        }
    }
}
