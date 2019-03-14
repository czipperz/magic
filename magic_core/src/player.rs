use crate::instance::InstanceNumber;
use crate::mana::ManaPool;
use crate::state::State;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PlayerNumber {
    pub(crate) number: usize,
}

pub struct Player {
    pub health: i32,
    pub floating_mana: ManaPool,
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
            floating_mana: ManaPool::default(),
            deck,
            hand: Vec::new(),
            battlefield: Vec::new(),
            graveyard: Vec::new(),
            exile: Vec::new(),
        }
    }
}
