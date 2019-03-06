use crate::card::Instance;
use crate::event::Event;
use crate::game_state::GameState;
use crate::mana::ManaCost;
use crate::permanent::Permanent;
use std::sync::{Arc, Mutex};

/// This structure represents the actions a `Card` has.
///
/// The `can_execute` function allows for the user interface to
/// understand if the action can be performed.  The `try_execute`
/// function generates an `Event` to be put on the stack.  If the
/// `Action` is canceled then no `Event` is generated.
///
/// # Examples
///
/// Casting is implemented via an `Action`.  Because casting is
/// complicated, it is automatically generated for all `Card`s using
/// the mana cost of a `Card`.  Other costs can be added via the
/// builder methods in `Card`.
///
/// An example of an activated ability that uses an `Action` is Cycle.
///
/// An activated ability that uses an `Action` is
pub trait Action {
    fn minimum_costs(
        &self,
        state: &GameState,
        card: Arc<Mutex<Instance>>,
        permanent: Option<Arc<Mutex<Permanent>>>,
    ) -> Vec<Cost>;

    fn execute(
        &self,
        state: &mut GameState,
        card: Arc<Mutex<Instance>>,
        permanent: Option<Arc<Mutex<Permanent>>>,
    ) -> Option<Event>;
}

/// The cost needed to perform an `Action`.
#[derive(Debug)]
pub enum Cost {
    Mana(ManaCost),
}
