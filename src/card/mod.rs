use crate::mana::ManaCost;
use crate::player::PlayerNumber;
use crate::state::State;
use crate::triggers::Triggers;
use std::sync::{Arc, Mutex};

mod zone;
pub use self::zone::*;

mod attrs;
pub use self::attrs::*;

#[derive(Clone, Debug)]
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
    auras: Vec<Arc<Mutex<Card>>>,
}

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

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn owner(&self) -> PlayerNumber {
        self.owner
    }
    pub fn controller(&self) -> PlayerNumber {
        self.controller
    }
    pub fn zone(&self) -> Zone {
        self.zone
    }
    pub fn mana_cost(&self) -> ManaCost {
        self.base_mana_cost.clone()
    }
    pub fn types(&self) -> Vec<Type> {
        self.base_types.clone()
    }
    pub fn subtypes(&self) -> Vec<Subtype> {
        self.base_subtypes.clone()
    }
    pub fn attributes(&self) -> Vec<Attribute> {
        self.base_attributes.clone()
    }
    pub fn power(&self) -> usize {
        self.base_power
    }
    pub fn toughness(&self) -> usize {
        self.base_toughness
    }

    pub fn is_spell(&self) -> bool {
        !self.types().contains(&Type::Land)
    }
    pub fn cast_allows_responses(&self) -> bool {
        self.is_spell()
    }
    pub fn is_valid_target(
        &self,
        state: &State,
        controller: PlayerNumber,
        predicate: &impl Fn(&State, &Card) -> bool,
    ) -> bool {
        predicate(state, self)
    }

    pub fn move_to(&mut self, controller: PlayerNumber, zone: Zone) {
        if controller != self.owner() {
            assert_eq!(zone, Zone::Battlefield);
        }
        self.controller = controller;
        self.zone = zone;
    }
    pub fn add_aura(&mut self, aura: Arc<Mutex<Card>>) {
        assert!(aura.lock().unwrap().subtypes().contains(&Subtype::Aura));
        self.auras.push(aura);
    }

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
