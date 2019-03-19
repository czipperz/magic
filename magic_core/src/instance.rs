use crate::card::{Card, CardNumber};
use crate::permanent::{Permanent, PermanentNumber};
use crate::player::{Player, PlayerNumber};
use crate::state::State;
use crate::zone::Zone;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct InstanceNumber {
    pub(crate) number: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instance {
    pub card: CardNumber,
    pub permanent: Option<PermanentNumber>,
    pub owner: PlayerNumber,
    pub controller: PlayerNumber,
    pub zone: Zone,
}

impl InstanceNumber {
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
