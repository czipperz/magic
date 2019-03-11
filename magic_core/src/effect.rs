use crate::permanent::Permanent;
use crate::state::State;

pub trait Effect {
    fn affect(&self, state: &State, permanent: &mut Permanent);
}

pub struct DoNothingEffect;
impl Effect for DoNothingEffect {
    fn affect(&self, _: &State, _: &mut Permanent) {}
}
