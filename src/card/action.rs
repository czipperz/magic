use super::Instance;
use crate::event::Event;
use crate::game_state::GameState;
use crate::mana::{ManaCost, ManaPool};
use crate::permanent::Permanent;
use crate::player::PlayerNumber;
use crate::spell::{Spell, Target};
use crate::ui::UserInterface;
use std::fmt;
use std::sync::{Arc, Mutex};

/// This structure represents the actions a `Card` has.
///
/// An action is either casting the card or using the activated
/// abilities of the card.
///
/// `Action`s can be added by enchantments and thus a `Permanent` may
/// have different `Action`s than the `Card` base.
///
/// An `Action` consists of `Cost`s and an `Event` that will be
/// created when sufficient `Payment`s are supplied.
///
/// # Examples
///
/// Casting is implemented via an `Action`.  Because casting is
/// complicated, it is automatically generated for `Card`s using
/// the mana cost of a `Card`.  Other costs can be added via the
/// builder methods in `Card`.
///
/// An example of an activated ability that uses an `Action` is Cycle.
/// Another is Equip.
#[derive(Debug)]
pub struct Action {
    pub execute: Arc<ActionEvent>,
    pub mandatory_costs: Vec<Cost>,
    pub optional_costs: Vec<Cost>,
    pub target_descriptions: Vec<TargetDescription>,
}

pub trait ActionEvent: fmt::Debug {
    fn execute(
        &self,
        state: &mut GameState,
        ui: &mut UserInterface,
        card: Arc<Mutex<Instance>>,
        mandatory_payments: Vec<Payment>,
        optional_payments: Vec<Option<Payment>>,
        targets: Vec<Target>,
    ) -> Event;
}

impl Action {
    pub fn new(execute: impl ActionEvent + 'static) -> Self {
        Action {
            execute: Arc::new(execute),
            mandatory_costs: Vec::new(),
            optional_costs: Vec::new(),
            target_descriptions: Vec::new(),
        }
    }

    pub fn with_mandatory_cost(mut self, cost: Cost) -> Self {
        self.mandatory_costs.push(cost);
        self
    }

    pub fn with_optional_cost(mut self, cost: Cost) -> Self {
        self.optional_costs.push(cost);
        self
    }

    pub fn with_target(mut self, target: TargetDescription) -> Self {
        self.target_descriptions.push(target);
        self
    }
}

/// The payment supplied to satisfy the `Cost`.
#[derive(Debug)]
pub enum Payment {
    Mana(ManaPool),
    Sacrifice(Vec<Arc<Mutex<Permanent>>>),
    Payments(Vec<Payment>),
}

/// The cost needed to perform an `Action`.
pub enum Cost {
    Mana(ManaCost),
    Sacrifice(Count, Box<Fn(&GameState, &Permanent) -> bool>),
    Costs(Vec<Cost>),
}

impl fmt::Debug for Cost {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cost::Mana(mc) => write!(fmt, "Mana({:?})", mc),
            Cost::Sacrifice(count, _) => write!(fmt, "Sacrifice({:?}, {:?})", count, ".."),
            Cost::Costs(costs) => write!(fmt, "Costs({:?})", costs),
        }
    }
}

pub enum TargetDescription {
    Player(Count, Box<Fn(&GameState, PlayerNumber) -> bool>),
    Permanent(Count, Box<Fn(&GameState, &Permanent) -> bool>),
    Card(Count, Box<Fn(&GameState, &Instance) -> bool>),
    Spell(Count, Box<Fn(&GameState, &Spell) -> bool>),
}

impl fmt::Debug for TargetDescription {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TargetDescription::Player(count, _) => write!(fmt, "Player({:?}, {:?})", count, ".."),
            TargetDescription::Permanent(count, _) => {
                write!(fmt, "Permanent({:?}, {:?})", count, "..")
            }
            TargetDescription::Card(count, _) => write!(fmt, "Card({:?}, {:?})", count, ".."),
            TargetDescription::Spell(count, _) => write!(fmt, "Spell({:?}, {:?})", count, ".."),
        }
    }
}

/// The number of things to select.
#[derive(Debug)]
pub struct Count {
    pub minimum: usize,
    pub maximum: Option<usize>,
}
