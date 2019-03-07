use super::Instance;
use crate::event::SourcedEvent;
use crate::game_state::GameState;
use crate::mana::{ManaCost, ManaPool};
use crate::permanent::Permanent;
use crate::player::PlayerNumber;
use crate::spell::{Spell, StackItem};
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
#[derive(Clone, Debug)]
pub struct Action {
    pub resolve: Arc<ResolveAction>,
    pub mandatory_costs: Vec<Cost>,
    pub optional_costs: Vec<Cost>,
    pub target_descriptions: Vec<TargetDescription>,
}

pub trait ResolveAction: fmt::Debug {
    fn resolve(
        &self,
        state: &mut GameState,
        ui: &mut UserInterface,
        stack_item: StackItem,
    ) -> SourcedEvent;
}

impl Action {
    pub fn new(resolve: impl ResolveAction + 'static) -> Self {
        Action {
            resolve: Arc::new(resolve),
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
#[derive(Clone)]
pub enum Cost {
    Mana(ManaCost),
    Sacrifice(Count, Arc<Fn(&GameState, &Permanent) -> bool>),
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

#[derive(Clone)]
pub enum TargetDescription {
    Player(Count, Arc<Fn(&GameState, PlayerNumber) -> bool>),
    Permanent(Count, Arc<Fn(&GameState, &Permanent) -> bool>),
    Card(Count, Arc<Fn(&GameState, &Instance) -> bool>),
    Spell(Count, Arc<Fn(&GameState, &Spell) -> bool>),
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
#[derive(Clone, Debug)]
pub struct Count {
    pub minimum: usize,
    pub maximum: Option<usize>,
}
