use crate::instance::InstanceNumber;
use crate::player::PlayerNumber;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Source {
    pub controller: PlayerNumber,
    pub instance: InstanceNumber,
}
