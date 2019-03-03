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
    controller: PlayerNumber,
    zone: Zone,
    base_mana_cost: ManaCost,
    base_types: Vec<Type>,
    base_subtypes: Vec<Subtype>,
    base_attributes: Vec<Attribute>,
    base_power: usize,
    base_toughness: usize,
    base_triggers: Triggers,
    auras: Vec<Aura>,
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
            controller: owner,
            zone: Zone::Deck,
            base_mana_cost,
            base_types,
            base_subtypes: Vec::new(),
            base_attributes: Vec::new(),
            base_power: 0,
            base_toughness: 0,
            base_triggers: Triggers::new(),
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
    pub fn controller(&self, state: &State) -> PlayerNumber {
        let mut controller = self.controller;
        for aura in &self.auras {
            controller = aura.decoration.decorate_controller(state, self, controller);
        }
        controller
    }
    pub fn zone(&self, state: &State) -> Zone {
        let mut zone = self.zone;
        for aura in &self.auras {
            zone = aura.decoration.decorate_zone(state, self, zone);
        }
        zone
    }
    pub fn mana_cost(&self, state: &State) -> ManaCost {
        let mut mana_cost = self.base_mana_cost.clone();
        for aura in &self.auras {
            mana_cost = aura.decoration.decorate_mana_cost(state, self, mana_cost);
        }
        mana_cost
    }
    pub fn types(&self, state: &State) -> Vec<Type> {
        let mut types = self.base_types.clone();
        for aura in &self.auras {
            types = aura.decoration.decorate_types(state, self, types);
        }
        types
    }
    pub fn subtypes(&self, state: &State) -> Vec<Subtype> {
        let mut subtypes = self.base_subtypes.clone();
        for aura in &self.auras {
            subtypes = aura.decoration.decorate_subtypes(state, self, subtypes);
        }
        subtypes
    }
    pub fn attributes(&self, state: &State) -> Vec<Attribute> {
        let mut attributes = self.base_attributes.clone();
        for aura in &self.auras {
            attributes = aura.decoration.decorate_attributes(state, self, attributes);
        }
        attributes
    }
    pub fn power(&self, state: &State) -> usize {
        let mut power = self.base_power;
        for aura in &self.auras {
            power = aura.decoration.decorate_power(state, self, power);
        }
        power
    }
    pub fn toughness(&self, state: &State) -> usize {
        let mut toughness = self.base_toughness;
        for aura in &self.auras {
            toughness = aura.decoration.decorate_toughness(state, self, toughness);
        }
        toughness
    }
}

// colors
impl Card {
    pub fn converted_mana_cost(&self, state: &State) -> usize {
        let mana_cost = self.mana_cost(state);
        mana_cost.blue + mana_cost.white + mana_cost.green + mana_cost.red + mana_cost.black
    }

    pub fn colors(&self, state: &State) -> Vec<Color> {
        let mana_cost = self.mana_cost(state);
        let mut colors = Vec::new();
        if mana_cost.blue != 0 {
            colors.push(Color::Blue);
        }
        if mana_cost.white != 0 {
            colors.push(Color::White);
        }
        if mana_cost.green != 0 {
            colors.push(Color::Green);
        }
        if mana_cost.red != 0 {
            colors.push(Color::Red);
        }
        if mana_cost.black != 0 {
            colors.push(Color::Black);
        }
        colors
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
        source: &Source,
        predicate: &impl Fn(&State, &Card) -> bool,
    ) -> bool {
        predicate(state, self)
    }

    pub fn as_source(&self, state: &State) -> Source {
        Source::from_card(state, self)
    }
}

// modifiers
impl Card {
    pub fn move_to(&mut self, controller: PlayerNumber, zone: Zone) {
        if controller != self.owner() {
            assert_eq!(zone, Zone::Battlefield);
        }
        self.controller = controller;
        self.zone = zone;
    }

    pub fn add_aura(&mut self, card: Arc<Mutex<Card>>, decoration: impl CardDecoration + 'static) {
        // assert!(card
        //     .lock()
        //     .unwrap()
        //     .subtypes()
        //     .contains(&Subtype::Aura));
        self.auras.push(Aura {
            card,
            decoration: Box::new(decoration),
        });
    }
}

// builder
impl Card {
    pub fn with_base_subtypes(mut self, subtypes: Vec<Subtype>) -> Self {
        self.base_subtypes = subtypes;
        self
    }
    pub fn with_base_attributes(mut self, attributes: Vec<Attribute>) -> Self {
        self.base_attributes = attributes;
        self
    }
    pub fn with_base_power(mut self, power: usize) -> Self {
        self.base_power = power;
        self
    }
    pub fn with_base_toughness(mut self, toughness: usize) -> Self {
        self.base_toughness = toughness;
        self
    }
    pub fn with_base_triggers(mut self, triggers: Triggers) -> Self {
        self.base_triggers = triggers;
        self
    }
}
