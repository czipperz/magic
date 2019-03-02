use crate::card::*;
use crate::mana::ManaCost;
use crate::player::PlayerNumber;
use crate::state::State;
use std::fmt::Debug;

pub trait CardDecoration: Debug {
    fn decorate_controller(
        &self,
        state: &State,
        card: &Card,
        controller: PlayerNumber,
    ) -> PlayerNumber {
        controller
    }

    fn decorate_zone(&self, state: &State, card: &Card, zone: Zone) -> Zone {
        zone
    }

    fn decorate_mana_cost(&self, state: &State, card: &Card, mana_cost: ManaCost) -> ManaCost {
        mana_cost
    }

    fn decorate_types(&self, state: &State, card: &Card, types: Vec<Type>) -> Vec<Type> {
        types
    }

    fn decorate_subtypes(
        &self,
        state: &State,
        card: &Card,
        subtypes: Vec<Subtype>,
    ) -> Vec<Subtype> {
        subtypes
    }

    fn decorate_attributes(
        &self,
        state: &State,
        card: &Card,
        attributes: Vec<Attribute>,
    ) -> Vec<Attribute> {
        attributes
    }

    fn decorate_power(&self, state: &State, card: &Card, power: usize) -> usize {
        power
    }

    fn decorate_toughness(&self, state: &State, card: &Card, toughness: usize) -> usize {
        toughness
    }
}
