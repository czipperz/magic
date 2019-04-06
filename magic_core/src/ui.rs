use crate::action::{SourcedAction, Target, TargetDescription};
use crate::instance::InstanceID;
use crate::source::Source;
use crate::state::State;

pub trait UserInterface {
    /// Let the user choose a target fitting the description.
    ///
    /// The `Source` should be the card that is choosing or targetting a
    /// permanent.
    ///
    /// The user is able to either choose a valid target or cancel the parent
    /// action.  If they accept, `choose_target` will return the `Target`.  If
    /// there is no possible valid target or the user cancels the parent action,
    /// this will return `None`.
    ///
    /// The individual targets inside the returned `Target` will be unique.
    fn choose_target(
        &mut self,
        state: &State,
        source: &Source,
        description: TargetDescription,
    ) -> Option<Target>;

    /// Let the user possibly trigger mana abilities.
    ///
    /// This returns `Some(action)` if the user triggered a mana ability.
    /// This returns `None` if the user selected to stop triggering mana
    /// abilities.
    fn maybe_trigger_mana_ability(&mut self, state: &State) -> Option<SourcedAction>;

    /// Let the user hit yes or no.  This allows for optional actions.
    fn read_bool(&mut self, state: &State, instance: InstanceID) -> bool;

    fn display(&mut self, state: &State);
}
