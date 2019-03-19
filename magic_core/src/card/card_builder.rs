use super::{Attribute, Subtype, Type};
use crate::action::{Action, ActionResolver, Cost, TargetDescription, Trigger};
use crate::card::Card;
use crate::effect::{DoNothingEffect, Effect};
use crate::mana::{Color, ManaCost};
use crate::replacement_effect::ReplacementEffect;
use by_address::ByAddress;
use std::sync::Arc;

pub struct CardBuilder {
    name: Option<String>,
    cast_resolve: Option<ByAddress<Arc<ActionResolver>>>,
    cast_target_descriptions: Vec<TargetDescription>,
    cast_mandatory_costs: Vec<Cost>,
    cast_optional_costs: Vec<Cost>,
    colors: Option<Vec<Color>>,
    types: Vec<Type>,
    subtypes: Vec<Subtype>,
    attributes: Vec<Attribute>,

    abilities: Vec<Action>,
    triggers: Vec<ByAddress<Arc<Trigger>>>,
    replacement_effects: Vec<ByAddress<Arc<ReplacementEffect>>>,
    effect: Option<ByAddress<Arc<Effect>>>,
    color_words: Vec<Color>,

    power: Option<isize>,
    toughness: Option<isize>,
}

impl CardBuilder {
    pub fn new() -> Self {
        CardBuilder {
            name: None,
            cast_resolve: None,
            cast_target_descriptions: Vec::new(),
            cast_mandatory_costs: Vec::new(),
            cast_optional_costs: Vec::new(),
            colors: None,
            types: Vec::new(),
            subtypes: Vec::new(),
            attributes: Vec::new(),

            abilities: Vec::new(),
            triggers: Vec::new(),
            replacement_effects: Vec::new(),
            effect: None,
            color_words: Vec::new(),

            power: None,
            toughness: None,
        }
    }

    pub fn build(self) -> Card {
        Card {
            name: self.name.unwrap(),
            cast_action: Action {
                resolve: self.cast_resolve.unwrap(),
                target_descriptions: self.cast_target_descriptions,
                mandatory_costs: self.cast_mandatory_costs,
                optional_costs: self.cast_optional_costs,
                is_mana_ability: false,
            },
            colors: self.colors.unwrap(),
            types: self.types,
            subtypes: self.subtypes,
            attributes: self.attributes,

            abilities: self.abilities,
            triggers: self.triggers,
            replacement_effects: self.replacement_effects,
            effect: self.effect.unwrap_or(ByAddress(Arc::new(DoNothingEffect))),
            color_words: self.color_words,

            power: self.power,
            toughness: self.toughness,
        }
    }
}

// builder
impl CardBuilder {
    pub fn with_name(mut self, name: impl ToString) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn on_resolve(mut self, resolver: impl ActionResolver + 'static) -> Self {
        self.cast_resolve = Some(ByAddress(Arc::new(resolver)));
        self
    }

    pub fn with_mana_cost(mut self, mana_cost: ManaCost) -> Self {
        if self.colors.is_none() {
            self.colors = Some(mana_cost.colors());
        }
        self.cast_mandatory_costs.insert(0, Cost::Mana(mana_cost));
        self
    }

    pub fn with_target(mut self, target_description: TargetDescription) -> Self {
        self.cast_target_descriptions.push(target_description);
        self
    }

    pub fn with_mandatory_cost(mut self, additional_cost: Cost) -> Self {
        self.cast_mandatory_costs.push(additional_cost);
        self
    }

    pub fn with_optional_cost(mut self, additional_cost: Cost) -> Self {
        self.cast_optional_costs.push(additional_cost);
        self
    }

    pub fn with_colors(mut self, colors: Vec<Color>) -> Self {
        self.colors = Some(colors);
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
        self.effect = Some(ByAddress(Arc::new(effect)));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::MockResolve;

    #[test]
    #[should_panic]
    fn test_build_panics_no_inputs() {
        CardBuilder::new().build();
    }

    #[test]
    fn test_mana_cost_empty() {
        let mana_cost = ManaCost::default();
        let builder = CardBuilder::new()
            .with_name("Mock")
            .with_mana_cost(mana_cost.clone())
            .on_resolve(MockResolve);
        assert_eq!(builder.colors, Some(vec![]));
        assert_eq!(
            builder.cast_mandatory_costs,
            vec![Cost::Mana(mana_cost.clone())]
        );
        let card = builder.build();
        assert_eq!(
            card.cast_action.mandatory_costs,
            vec![Cost::Mana(mana_cost)]
        );
    }
}
