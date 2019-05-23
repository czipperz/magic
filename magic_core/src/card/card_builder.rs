use super::{KeywordAbility, Subtype, Type};
use crate::action::{Action, ActionResolver, Cost, TargetDescription, Trigger};
use crate::card::Card;
use crate::effect::Effect;
use crate::mana::{Color, ManaCost};
use crate::permission::{Permission, Permissions};
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

    abilities: Vec<Action>,
    keyword_abilities: Vec<KeywordAbility>,
    triggers: Vec<ByAddress<Arc<Trigger>>>,
    replacement_effects: Vec<ByAddress<Arc<ReplacementEffect>>>,
    self_effects: Vec<ByAddress<Arc<Effect>>>,
    global_effects: Vec<ByAddress<Arc<Effect>>>,
    color_words: Vec<Color>,
    permissions: Permissions,

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

            abilities: Vec::new(),
            keyword_abilities: Vec::new(),
            triggers: Vec::new(),
            replacement_effects: Vec::new(),
            self_effects: Vec::new(),
            global_effects: Vec::new(),
            color_words: Vec::new(),
            permissions: Permissions::default(),

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

            abilities: self.abilities,
            keyword_abilities: self.keyword_abilities,
            triggers: self.triggers,
            replacement_effects: self.replacement_effects,
            self_effects: self.self_effects,
            global_effects: self.global_effects,
            color_words: self.color_words,
            permissions: self.permissions,

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

    pub fn with_ability(mut self, ability: Action) -> Self {
        self.abilities.push(ability);
        self
    }

    pub fn with_keyword_ability(mut self, keyword_ability: KeywordAbility) -> Self {
        self.keyword_abilities.push(keyword_ability);
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

    pub fn with_self_effect(mut self, effect: impl Effect + 'static) -> Self {
        self.self_effects.push(ByAddress(Arc::new(effect)));
        self
    }

    pub fn with_global_effect(mut self, effect: impl Effect + 'static) -> Self {
        self.global_effects.push(ByAddress(Arc::new(effect)));
        self
    }

    pub fn with_color_words(mut self, color_words: Vec<Color>) -> Self {
        self.color_words = color_words;
        self
    }

    pub fn with_permission(mut self, permission: Permission) -> Self {
        self.permissions.add(permission);
        self
    }

    pub fn without_permission(mut self, permission: &Permission) -> Self {
        self.permissions.remove(permission);
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
        assert_eq!(card.colors, vec![]);
        assert_eq!(
            card.cast_action.mandatory_costs,
            vec![Cost::Mana(mana_cost)]
        );
    }

    #[test]
    fn test_mana_cost_colored() {
        let mana_cost = ManaCost::default().with_red(1).with_blue(1);
        let builder = CardBuilder::new()
            .with_name("Mock")
            .with_mana_cost(mana_cost.clone())
            .on_resolve(MockResolve);

        assert_eq!(builder.colors, Some(vec![Color::Blue, Color::Red]));
        assert_eq!(
            builder.cast_mandatory_costs,
            vec![Cost::Mana(mana_cost.clone())]
        );

        let card = builder.build();
        assert_eq!(card.colors, vec![Color::Blue, Color::Red]);
        assert_eq!(
            card.cast_action.mandatory_costs,
            vec![Cost::Mana(mana_cost)]
        );
    }

    #[test]
    fn test_mana_cost_colored_before_custom_card_colors() {
        let mana_cost = ManaCost::default().with_red(1).with_blue(1);
        let builder = CardBuilder::new()
            .with_name("Mock")
            .with_mana_cost(mana_cost.clone())
            .with_colors(vec![])
            .on_resolve(MockResolve);

        assert_eq!(builder.colors, Some(vec![]));
        assert_eq!(
            builder.cast_mandatory_costs,
            vec![Cost::Mana(mana_cost.clone())]
        );

        let card = builder.build();
        assert_eq!(card.colors, vec![]);
        assert_eq!(
            card.cast_action.mandatory_costs,
            vec![Cost::Mana(mana_cost)]
        );
    }

    #[test]
    fn test_mana_cost_colored_after_custom_card_colors() {
        let mana_cost = ManaCost::default().with_red(1).with_blue(1);
        let builder = CardBuilder::new()
            .with_name("Mock")
            .with_colors(vec![])
            .with_mana_cost(mana_cost.clone())
            .on_resolve(MockResolve);

        assert_eq!(builder.colors, Some(vec![]));
        assert_eq!(
            builder.cast_mandatory_costs,
            vec![Cost::Mana(mana_cost.clone())]
        );

        let card = builder.build();
        assert_eq!(card.colors, vec![]);
        assert_eq!(
            card.cast_action.mandatory_costs,
            vec![Cost::Mana(mana_cost)]
        );
    }
}
