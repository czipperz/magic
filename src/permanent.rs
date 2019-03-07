use crate::card::{Action, Attribute, Instance, Subtype, Type};
use crate::mana::Color;
use crate::spell::Spell;
use crate::trigger::Trigger;
use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
pub struct Permanent {
    card: Arc<Mutex<Instance>>,
    effects: Vec<Arc<Mutex<Permanent>>>,
    effecting: Weak<Mutex<Permanent>>,

    activated_abilities: Vec<Action>,
    colors: Vec<Color>,
    types: Vec<Type>,
    subtypes: Vec<Subtype>,
    attributes: Vec<Attribute>,
    triggers: Vec<Arc<Trigger>>,
    color_words: Vec<Color>,
    power: Option<isize>,
    toughness: Option<isize>,
}

impl From<Spell> for Permanent {
    fn from(spell: Spell) -> Self {
        let (activated_abilities, colors, types, attributes, triggers, power, toughness) = {
            let card_locked = spell.card.lock().unwrap();
            (
                card_locked.activated_abilities().clone(),
                card_locked.colors().clone(),
                card_locked.types().clone(),
                card_locked.attributes().clone(),
                card_locked.triggers().clone(),
                card_locked.power(),
                card_locked.toughness(),
            )
        };
        Permanent {
            card: spell.card,
            effects: Vec::new(),
            effecting: Weak::new(),

            activated_abilities,
            colors,
            types,
            subtypes: spell.subtypes,
            attributes,
            triggers,
            color_words: spell.color_words,
            power,
            toughness,
        }
    }
}
