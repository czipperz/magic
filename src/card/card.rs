use super::{Attribute, Subtype, Type};
use crate::action::Cost;
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

    mana_cost: ManaCost,
    additional_costs: Vec<Cost>,
    colors: Vec<Color>,
    types: Vec<Type>,
    subtypes: Vec<Subtype>,
    attributes: Vec<Attribute>,
    triggers: Vec<Arc<Trigger>>,
    power: Option<isize>,
    toughness: Option<isize>,
}

// constructors
impl Card {
    pub fn new(name: String, owner: PlayerNumber, mana_cost: ManaCost, types: Vec<Type>) -> Self {
        assert!(!types.is_empty());
        let colors = mana_cost.colors();
        Card {
            name,
            owner,
            mana_cost,
            additional_costs: Vec::new(),
            colors,
            types,
            subtypes: Vec::new(),
            attributes: Vec::new(),
            triggers: Vec::new(),
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

    pub fn mana_cost(&self) -> ManaCost {
        self.mana_cost.clone()
    }

    pub fn additional_costs(&self) -> &Vec<Cost> {
        &self.additional_costs
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

    pub fn power(&self) -> Option<isize> {
        self.power.clone()
    }

    pub fn toughness(&self) -> Option<isize> {
        self.toughness.clone()
    }
}

// builder
impl Card {
    pub fn with_additional_cost(mut self, additional_cost: Cost) -> Self {
        self.additional_costs.push(additional_cost);
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

    pub fn with_power(mut self, power: isize) -> Self {
        self.power = Some(power);
        self
    }

    pub fn with_toughness(mut self, toughness: isize) -> Self {
        self.toughness = Some(toughness);
        self
    }
}
