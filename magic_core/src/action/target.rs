use crate::action::ActivatedAction;
use crate::count::Count;
use crate::instance::InstanceID;
use crate::player::PlayerID;
use crate::state::State;
use crate::zone::Zone;
use by_address::ByAddress;
use std::fmt;
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Target {
    Player(Vec<PlayerID>),
    Instance(Vec<InstanceID>),
    Spell(Vec<InstanceID>),
}

#[derive(Clone, Eq, PartialEq)]
pub enum TargetDescription {
    Player(Count, ByAddress<Arc<Fn(&State, PlayerID) -> bool>>),
    Instance(Count, ByAddress<Arc<Fn(&State, InstanceID) -> bool>>),
    ActivatedAction(Count, ByAddress<Arc<Fn(&State, &ActivatedAction) -> bool>>),
}

impl TargetDescription {
    pub fn player(
        count: impl Into<Count>,
        predicate: impl Fn(&State, PlayerID) -> bool + 'static,
    ) -> Self {
        TargetDescription::Player(count.into(), ByAddress(Arc::new(predicate)))
    }

    pub fn instance(
        count: impl Into<Count>,
        predicate: impl Fn(&State, InstanceID) -> bool + 'static,
    ) -> Self {
        TargetDescription::Instance(count.into(), ByAddress(Arc::new(predicate)))
    }

    pub fn permanent(
        count: impl Into<Count>,
        predicate: impl Fn(&State, InstanceID) -> bool + 'static,
    ) -> Self {
        Self::instance(count, move |state, instance_id| {
            instance_id.get(state).zone == Zone::Battlefield && predicate(state, instance_id)
        })
    }

    pub fn graveyard(
        count: impl Into<Count>,
        predicate: impl Fn(&State, InstanceID) -> bool + 'static,
    ) -> Self {
        Self::instance(count, move |state, instance_id| {
            instance_id.get(state).zone == Zone::Graveyard && predicate(state, instance_id)
        })
    }

    pub fn spell(
        count: impl Into<Count>,
        predicate: impl Fn(&State, &ActivatedAction) -> bool + 'static,
    ) -> Self {
        TargetDescription::ActivatedAction(count.into(), ByAddress(Arc::new(predicate)))
    }
}

impl fmt::Debug for TargetDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TargetDescription::")?;
        match self {
            TargetDescription::Player(count, _) => write!(f, "Player({:?}, {})", count, ".."),
            TargetDescription::Instance(count, _) => write!(f, "Instance({:?}, {})", count, ".."),
            TargetDescription::ActivatedAction(count, _) => {
                write!(f, "Spell({:?}, {})", count, "..")
            }
        }
    }
}
