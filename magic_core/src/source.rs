use crate::instance::InstanceID;
use crate::player::PlayerID;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Source {
    pub controller: PlayerID,
    pub instance: InstanceID,
}
