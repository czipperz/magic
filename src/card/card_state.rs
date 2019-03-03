use crate::card::*;
use crate::mana::ManaCost;
use crate::player::PlayerNumber;
use crate::triggers::Triggers;

#[derive(Clone, Debug)]
pub struct CardState {
    pub controller: PlayerNumber,
    pub zone: Zone,
    pub mana_cost: ManaCost,
    pub types: Vec<Type>,
    pub subtypes: Vec<Subtype>,
    pub attributes: Vec<Attribute>,
    pub power: usize,
    pub toughness: usize,
    pub triggers: Triggers,
}
