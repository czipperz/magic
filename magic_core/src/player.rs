use crate::instance::InstanceNumber;
use crate::state::State;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PlayerNumber {
    pub(crate) number: usize,
}

pub struct Player {
    pub health: i32,
    pub deck: Vec<InstanceNumber>,
    pub hand: Vec<InstanceNumber>,
    pub battlefield: Vec<InstanceNumber>,
    pub graveyard: Vec<InstanceNumber>,
    pub exile: Vec<InstanceNumber>,
}

impl PlayerNumber {
    pub fn get<'a>(self, state: &'a State) -> &'a Player {
        state.player(self)
    }
}

impl Player {
    pub fn new(health: i32, deck: Vec<InstanceNumber>) -> Self {
        Player {
            health,
            deck,
            hand: Vec::new(),
            battlefield: Vec::new(),
            graveyard: Vec::new(),
            exile: Vec::new(),
        }
    }
}
