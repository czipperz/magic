use super::aura::Aura;
use super::*;
use crate::mana::*;
use crate::player::PlayerNumber;
use crate::source::Source;
use crate::state::State;
use crate::triggers::Triggers;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Card {
    name: String,
    owner: PlayerNumber,
    auras: Vec<Aura>,
    base_state: CardState,
}

// constructors
impl Card {
    pub fn new(
        name: String,
        owner: PlayerNumber,
        base_mana_cost: ManaCost,
        base_types: Vec<Type>,
    ) -> Self {
        assert!(!base_types.is_empty());
        Card {
            name,
            owner,
            base_state: CardState {
                controller: owner,
                zone: Zone::Deck,
                mana_cost: base_mana_cost,
                types: base_types,
                subtypes: Vec::new(),
                attributes: Vec::new(),
                power: 0,
                toughness: 0,
                triggers: Triggers::new(),
            },
            auras: Vec::new(),
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

    pub fn state(&self, state: &State) -> CardState {
        let mut card_state = self.base_state.clone();
        for aura in &self.auras {
            aura.decorate_card_state(state, self, &mut card_state);
        }
        card_state
    }

    pub fn controller(&self, state: &State) -> PlayerNumber {
        self.state(state).controller
    }
    pub fn zone(&self, state: &State) -> Zone {
        self.state(state).zone
    }
    pub fn mana_cost(&self, state: &State) -> ManaCost {
        self.state(state).mana_cost
    }
    pub fn types(&self, state: &State) -> Vec<Type> {
        self.state(state).types
    }
    pub fn subtypes(&self, state: &State) -> Vec<Subtype> {
        self.state(state).subtypes
    }
    pub fn attributes(&self, state: &State) -> Vec<Attribute> {
        self.state(state).attributes
    }
    pub fn power(&self, state: &State) -> usize {
        self.state(state).power
    }
    pub fn toughness(&self, state: &State) -> usize {
        self.state(state).toughness
    }
}

// colors
impl Card {
    pub fn colors(&self, state: &State) -> Vec<Color> {
        self.mana_cost(state).colors()
    }
}

// predicates
impl Card {
    pub fn is_spell(&self, state: &State) -> bool {
        !self.types(state).contains(&Type::Land)
    }

    pub fn cast_allows_responses(&self, state: &State) -> bool {
        self.is_spell(state)
    }

    pub fn is_valid_target(
        &self,
        state: &State,
        _source: &Source,
        predicate: &impl Fn(&State, &Card) -> bool,
    ) -> bool {
        // TODO: implement hexproof and shroud for cards
        predicate(state, self)
    }
}

// modifiers
impl Card {
    pub fn move_to(&mut self, controller: PlayerNumber, zone: Zone) {
        if controller != self.owner() {
            assert_eq!(zone, Zone::Battlefield);
        }
        self.base_state.controller = controller;
        self.base_state.zone = zone;
    }

    pub fn add_aura(
        &mut self,
        card: Arc<Mutex<Card>>,
        decoration: impl Fn(&State, &Card, &mut CardState) + 'static,
    ) {
        assert!(card
            .lock()
            .unwrap()
            .base_state
            .subtypes
            .contains(&Subtype::Aura));
        self.auras.push(Aura {
            card,
            decoration: Box::new(decoration),
        });
    }
}

// builder
impl Card {
    pub fn with_base_subtypes(mut self, subtypes: Vec<Subtype>) -> Self {
        self.base_state.subtypes = subtypes;
        self
    }
    pub fn with_base_attributes(mut self, attributes: Vec<Attribute>) -> Self {
        self.base_state.attributes = attributes;
        self
    }
    pub fn with_base_power(mut self, power: usize) -> Self {
        self.base_state.power = power;
        self
    }
    pub fn with_base_toughness(mut self, toughness: usize) -> Self {
        self.base_state.toughness = toughness;
        self
    }
    pub fn with_base_triggers(mut self, triggers: Triggers) -> Self {
        self.base_state.triggers = triggers;
        self
    }
}
