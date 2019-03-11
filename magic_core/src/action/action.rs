use super::{Cost, Payment, Target, TargetDescription};
use crate::event::Event;
use crate::source::Source;
use crate::state::State;
use std::sync::Arc;

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
#[derive(Clone)]
pub struct Action {
    pub resolve: Arc<ActionResolver>,
    pub target_descriptions: Vec<TargetDescription>,
    pub mandatory_costs: Vec<Cost>,
    pub optional_costs: Vec<Cost>,
}

pub trait ActionResolver {
    fn resolve(&self, state: &State, action: ActivatedAction) -> Vec<Event>;
}

pub struct ActivatedAction {
    pub action_type: ActionType,
    pub source: Source,
    pub targets: Vec<Target>,
    pub mandatory_payments: Vec<Payment>,
    pub optional_payments: Vec<Option<Payment>>,
}

pub enum ActionType {
    Spell,
    ActivatedAbility,
    TriggeredAbility,
}

impl<R> From<R> for Action
where
    R: ActionResolver + 'static,
{
    fn from(resolve: R) -> Self {
        Action {
            resolve: Arc::new(resolve),
            mandatory_costs: Vec::new(),
            optional_costs: Vec::new(),
            target_descriptions: Vec::new(),
        }
    }
}

impl Action {
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
