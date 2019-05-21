use crate::permanent::Permanent;
use crate::state::State;
use std::fmt::Debug;

pub trait Effect: Debug {
    fn affect(&self, state: &State, permanent: &mut Permanent);
}
