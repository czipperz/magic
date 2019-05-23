use super::{KeywordAbility, Subtype, Type};
use crate::action::{Action, Cost, Trigger};
use crate::effect::Effect;
use crate::mana::{Color, ManaCost};
use crate::permission::Permissions;
use crate::replacement_effect::ReplacementEffect;
use by_address::ByAddress;
use std::sync::Arc;

/// A `Card` represents the information written on a physical card.
///
/// To information about where the `Card` is located, see `Instance`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Card {
    pub name: String,
    pub cast_action: Action,
    pub colors: Vec<Color>,
    pub types: Vec<Type>,
    pub subtypes: Vec<Subtype>,

    pub abilities: Vec<Action>,
    pub keyword_abilities: Vec<KeywordAbility>,
    pub triggers: Vec<ByAddress<Arc<Trigger>>>,
    pub replacement_effects: Vec<ByAddress<Arc<ReplacementEffect>>>,
    pub self_effects: Vec<ByAddress<Arc<Effect>>>,
    pub global_effects: Vec<ByAddress<Arc<Effect>>>,
    pub color_words: Vec<Color>,
    pub permissions: Permissions,

    pub power: Option<isize>,
    pub toughness: Option<isize>,
}

impl Card {
    pub fn mana_cost(&self) -> &ManaCost {
        match &self.cast_action.mandatory_costs[0] {
            Cost::Mana(mc) => mc,
            _ => panic!("Cast Action must have the first cost be the mana cost"),
        }
    }
}
