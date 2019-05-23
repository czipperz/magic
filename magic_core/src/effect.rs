use crate::card::Card;
use crate::instance::InstanceID;
use crate::state::State;
use std::fmt::Debug;

pub trait Effect: Debug {
    fn affect(&self, state: &State, instance: InstanceID, card: &mut Card);
}
