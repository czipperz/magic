use crate::action::{Action, Target, Trigger};
use crate::card::{Attribute, Subtype, Type};
use crate::instance::{Instance, InstanceNumber};
use crate::mana::Color;
use crate::replacement_effect::ReplacementEffect;
use crate::state::State;
use by_address::ByAddress;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PermanentNumber {
    pub(crate) number: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Permanent {
    pub instance: InstanceNumber,
    pub effects: Vec<InstanceNumber>,
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

impl PermanentNumber {
    pub fn get<'a>(self, state: &'a State) -> &'a Permanent {
        state.permanent(self)
    }

    pub fn instance<'a>(self, state: &'a State) -> &'a Instance {
        self.get(state).instance(state)
    }
}

impl Permanent {
    pub fn instance<'a>(&self, state: &'a State) -> &'a Instance {
        self.instance.get(state)
    }
}
