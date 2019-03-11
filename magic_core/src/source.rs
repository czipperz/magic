use crate::instance::InstanceNumber;
use crate::player::PlayerNumber;

#[derive(Clone)]
pub struct Source {
    pub controller: PlayerNumber,
    pub instance: InstanceNumber,
}
