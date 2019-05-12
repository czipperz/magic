use crate::action::{Action, Target, Trigger};
use crate::card::{Attribute, Card, Subtype, Type};
use crate::instance::{Instance, InstanceID};
use crate::mana::Color;
use crate::replacement_effect::ReplacementEffect;
use crate::state::State;
use by_address::ByAddress;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PermanentID(pub(crate) usize);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Permanent {
    pub instance: InstanceID,
    pub effects: Vec<InstanceID>,
    pub affecting: Option<Target>,
    pub tapped: bool,

    pub colors: Vec<Color>,
    pub types: Vec<Type>,
    pub subtypes: Vec<Subtype>,
    pub attributes: Vec<Attribute>,
    pub abilities: Vec<Action>,
    pub triggers: Vec<ByAddress<Arc<Trigger>>>,
    pub replacement_effects: Vec<ByAddress<Arc<ReplacementEffect>>>,
    pub color_words: Vec<Color>,
    pub power: Option<isize>,
    pub toughness: Option<isize>,

    pub ignored_attributes: Vec<Attribute>,
}

impl PermanentID {
    pub fn get<'a>(self, state: &'a State) -> &'a Permanent {
        state.permanent(self)
    }

    pub fn instance<'a>(self, state: &'a State) -> &'a Instance {
        self.get(state).instance(state)
    }
}

impl Permanent {
    pub fn new(instance_id: InstanceID, card: &Card) -> Self {
        Permanent {
            instance: instance_id,
            effects: Vec::new(),
            affecting: None,
            tapped: false,

            colors: card.colors.clone(),
            types: card.types.clone(),
            subtypes: card.subtypes.clone(),
            attributes: card.attributes.clone(),
            abilities: card.abilities.clone(),
            triggers: card.triggers.clone(),
            replacement_effects: card.replacement_effects.clone(),
            color_words: card.color_words.clone(),
            power: card.power.clone(),
            toughness: card.toughness.clone(),

            ignored_attributes: Vec::new(),
        }
    }

    pub fn instance<'a>(&self, state: &'a State) -> &'a Instance {
        self.instance.get(state)
    }
}
