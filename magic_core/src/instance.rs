use crate::card::{Card, CardID};
use crate::permanent::{Permanent, PermanentID};
use crate::player::{Player, PlayerID};
use crate::state::State;
use crate::zone::Zone;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct InstanceID(pub(crate) usize);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instance {
    pub card: CardID,
    pub permanent: Option<PermanentID>,
    pub owner: PlayerID,
    pub controller: PlayerID,
    pub zone: Zone,
}

impl InstanceID {
    pub fn get<'a>(self, state: &'a State) -> &'a Instance {
        state.instance(self)
    }

    pub fn card<'a>(self, state: &'a State) -> &'a Card {
        self.get(state).card(state)
    }

    pub fn permanent<'a>(self, state: &'a State) -> Option<&'a Permanent> {
        self.get(state).permanent(state)
    }

    pub fn owner<'a>(self, state: &'a State) -> &'a Player {
        self.get(state).owner(state)
    }

    pub fn controller<'a>(self, state: &'a State) -> &'a Player {
        self.get(state).controller(state)
    }
}

impl Instance {
    pub fn new(card: CardID, owner: PlayerID, zone: Zone) -> Self {
        Instance {
            card,
            permanent: None,
            owner,
            controller: owner,
            zone,
        }
    }

    pub fn card<'a>(&self, state: &'a State) -> &'a Card {
        self.card.get(state)
    }

    pub fn permanent<'a>(&self, state: &'a State) -> Option<&'a Permanent> {
        self.permanent.map(|permanent| permanent.get(state))
    }

    pub fn owner<'a>(&self, state: &'a State) -> &'a Player {
        self.owner.get(state)
    }

    pub fn controller<'a>(&self, state: &'a State) -> &'a Player {
        self.controller.get(state)
    }
}
