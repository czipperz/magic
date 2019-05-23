use crate::action::Target;
use crate::card::Card;
use crate::player::{Player, PlayerID};
use crate::state::State;
use crate::zone::Zone;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct InstanceID(pub(crate) usize);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instance {
    pub owner: PlayerID,
    pub controller: PlayerID,
    pub zone: Zone,

    pub attachments: Vec<InstanceID>,
    pub attached_to: Option<Target>,
    pub tapped: bool,

    pub card: Card,
}

impl InstanceID {
    pub fn get<'a>(self, state: &'a State) -> &'a Instance {
        state.instance(self)
    }

    pub fn owner<'a>(self, state: &'a State) -> &'a Player {
        self.get(state).owner(state)
    }

    pub fn controller<'a>(self, state: &'a State) -> &'a Player {
        self.get(state).controller(state)
    }
}

impl Instance {
    pub fn new(card: Card, owner: PlayerID, zone: Zone) -> Self {
        Instance {
            owner,
            controller: owner,
            zone,

            attachments: Vec::new(),
            attached_to: None,
            tapped: false,

            card,
        }
    }

    pub fn owner<'a>(&self, state: &'a State) -> &'a Player {
        self.owner.get(state)
    }

    pub fn controller<'a>(&self, state: &'a State) -> &'a Player {
        self.controller.get(state)
    }
}

impl Deref for Instance {
    type Target = Card;

    fn deref(&self) -> &Card {
        &self.card
    }
}

impl DerefMut for Instance {
    fn deref_mut(&mut self) -> &mut Card {
        &mut self.card
    }
}
