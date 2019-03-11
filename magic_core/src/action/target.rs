use crate::action::ActivatedAction;
use crate::count::Count;
use crate::instance::InstanceNumber;
use crate::permanent::PermanentNumber;
use crate::player::PlayerNumber;
use crate::state::State;
use std::sync::Arc;

pub enum Target {
    Player(Vec<PlayerNumber>),
    Permanent(Vec<InstanceNumber>),
    Graveyard(Vec<InstanceNumber>),
    Spell(Vec<InstanceNumber>),
}

#[derive(Clone)]
pub enum TargetDescription {
    Player(Count, Arc<Fn(&State, PlayerNumber) -> bool>),
    Permanent(Count, Arc<Fn(&State, PermanentNumber) -> bool>),
    Graveyard(Count, Arc<Fn(&State, InstanceNumber) -> bool>),
    ActivatedAction(Count, Arc<Fn(&State, &ActivatedAction) -> bool>),
}

impl TargetDescription {
    pub fn player(
        count: impl Into<Count>,
        predicate: impl Fn(&State, PlayerNumber) -> bool + 'static,
    ) -> Self {
        TargetDescription::Player(count.into(), Arc::new(predicate))
    }

    pub fn permanent(
        count: impl Into<Count>,
        predicate: impl Fn(&State, PermanentNumber) -> bool + 'static,
    ) -> Self {
        TargetDescription::Permanent(count.into(), Arc::new(predicate))
    }

    pub fn graveyard(
        count: impl Into<Count>,
        predicate: impl Fn(&State, InstanceNumber) -> bool + 'static,
    ) -> Self {
        TargetDescription::Graveyard(count.into(), Arc::new(predicate))
    }

    pub fn spell(
        count: impl Into<Count>,
        predicate: impl Fn(&State, &ActivatedAction) -> bool + 'static,
    ) -> Self {
        TargetDescription::ActivatedAction(count.into(), Arc::new(predicate))
    }
}
