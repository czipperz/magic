use super::{Action, Attribute, Cost, ResolveAction, Subtype, TargetDescription, Type};
use crate::mana::{Color, ManaCost};
use crate::player::PlayerNumber;
use crate::trigger::Trigger;
use std::sync::Arc;

/// A `Card` represents the information written on a physical card.
///
/// To information about where the `Card` is located, see `Instance`.
#[derive(Debug)]
pub struct Card {
    name: String,
    owner: PlayerNumber,

    cast_action: Action,
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

// constructors
impl Card {
    pub fn new(
        name: String,
        owner: PlayerNumber,
        mana_cost: ManaCost,
        types: Vec<Type>,
        on_resolve: impl ResolveAction + 'static,
    ) -> Self {
        assert!(!types.is_empty());
        let colors = mana_cost.colors();
        Card {
            name,
            owner,
            cast_action: Action::new(on_resolve).with_mandatory_cost(Cost::Mana(mana_cost)),
            activated_abilities: Vec::new(),
            colors,
            types,
            subtypes: Vec::new(),
            attributes: Vec::new(),
            triggers: Vec::new(),
            color_words: Vec::new(),
            power: None,
            toughness: None,
        }
    }
}

// getters
impl Card {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn owner(&self) -> PlayerNumber {
        self.owner
    }

    pub fn cast_action(&self) -> &Action {
        &self.cast_action
    }

    pub fn mana_cost(&self) -> &ManaCost {
        match &self.cast_action.mandatory_costs[0] {
            Cost::Mana(mc) => mc,
            _ => panic!("Cast Action must have the first cost be the mana cost"),
        }
    }

    pub fn activated_abilities(&self) -> &Vec<Action> {
        &self.activated_abilities
    }

    pub fn colors(&self) -> &Vec<Color> {
        &self.colors
    }

    pub fn types(&self) -> &Vec<Type> {
        &self.types
    }

    pub fn subtypes(&self) -> &Vec<Subtype> {
        &self.subtypes
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }

    pub fn triggers(&self) -> &Vec<Arc<Trigger>> {
        &self.triggers
    }

    pub fn color_words(&self) -> &Vec<Color> {
        &self.color_words
    }

    pub fn power(&self) -> Option<isize> {
        self.power.clone()
    }

    pub fn toughness(&self) -> Option<isize> {
        self.toughness.clone()
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

    pub fn with_subtype(mut self, subtype: Subtype) -> Self {
        self.subtypes.push(subtype);
        self
    }

    pub fn with_attribute(mut self, attribute: Attribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    pub fn with_trigger(mut self, trigger: impl Trigger + 'static) -> Self {
        self.triggers.push(Arc::new(trigger));
        self
    }

    pub fn with_activated_ability(mut self, ability: Action) -> Self {
        self.activated_abilities.push(ability);
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
