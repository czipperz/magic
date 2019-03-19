use super::{Attribute, Subtype, Type};
use crate::action::{Action, Cost, TargetDescription, Trigger};
use crate::effect::{DoNothingEffect, Effect};
use crate::mana::{Color, ManaCost};
use crate::replacement_effect::ReplacementEffect;
use crate::state::State;
use by_address::ByAddress;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct CardNumber {
    pub(crate) number: usize,
}

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
    pub attributes: Vec<Attribute>,

    pub abilities: Vec<Action>,
    pub triggers: Vec<ByAddress<Arc<Trigger>>>,
    pub replacement_effects: Vec<ByAddress<Arc<ReplacementEffect>>>,
    pub effect: ByAddress<Arc<Effect>>,
    pub color_words: Vec<Color>,

    pub power: Option<isize>,
    pub toughness: Option<isize>,
}

impl CardNumber {
    pub fn get<'a>(self, state: &'a State) -> &'a Card {
        state.card(self)
    }
}

// constructors
impl Card {
    pub fn new(name: impl ToString, mana_cost: ManaCost, cast_action: impl Into<Action>) -> Self {
        let colors = mana_cost.colors();
        Card {
            name: name.to_string(),
            cast_action: {
                let mut cast_action = cast_action.into();
                cast_action.mandatory_costs.insert(0, Cost::Mana(mana_cost));
                cast_action
            },
            colors,
            types: Vec::new(),
            subtypes: Vec::new(),
            attributes: Vec::new(),

            abilities: Vec::new(),
            triggers: Vec::new(),
            replacement_effects: Vec::new(),
            effect: ByAddress(Arc::new(DoNothingEffect)),
            color_words: Vec::new(),

            power: None,
            toughness: None,
        }
    }
}

impl Card {
    pub fn mana_cost(&self) -> &ManaCost {
        match &self.cast_action.mandatory_costs[0] {
            Cost::Mana(mc) => mc,
            _ => panic!("Cast Action must have the first cost be the mana cost"),
        }
    }
}

// builder
impl Card {
    pub fn with_mandatory_cost(mut self, additional_cost: Cost) -> Self {
        self.cast_action.mandatory_costs.push(additional_cost);
        self
    }

    pub fn with_optional_cost(mut self, additional_cost: Cost) -> Self {
        self.cast_action.optional_costs.push(additional_cost);
        self
    }

    pub fn with_target(mut self, target_description: TargetDescription) -> Self {
        self.cast_action
            .target_descriptions
            .push(target_description);
        self
    }

    pub fn with_colors(mut self, colors: Vec<Color>) -> Self {
        self.colors = colors;
        self
    }

    pub fn with_type(mut self, tp: Type) -> Self {
        self.types.push(tp);
        self
    }

    pub fn with_subtype(mut self, subtype: Subtype) -> Self {
        self.subtypes.push(subtype);
        self
    }

    pub fn with_attribute(mut self, attribute: Attribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    pub fn with_ability(mut self, ability: Action) -> Self {
        self.abilities.push(ability);
        self
    }

    pub fn with_trigger(mut self, trigger: impl Trigger + 'static) -> Self {
        self.triggers.push(ByAddress(Arc::new(trigger)));
        self
    }

    pub fn with_replacement_effect(
        mut self,
        replacement_effect: impl ReplacementEffect + 'static,
    ) -> Self {
        self.replacement_effects
            .push(ByAddress(Arc::new(replacement_effect)));
        self
    }

    pub fn with_effect(mut self, effect: impl Effect + 'static) -> Self {
        self.effect = ByAddress(Arc::new(effect));
        self
    }

    pub fn with_color_words(mut self, color_words: Vec<Color>) -> Self {
        self.color_words = color_words;
        self
    }

    pub fn with_power(mut self, power: isize) -> Self {
        self.power = Some(power);
        self
    }

    pub fn with_toughness(mut self, toughness: isize) -> Self {
        self.toughness = Some(toughness);
        self
    }
}
