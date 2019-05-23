use crate::card::Card;
use crate::instance::{Instance, InstanceID};
use crate::player::{Player, PlayerID};
use crate::zone::Zone;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;

pub struct State {
    players: Vec<Player>,
    pub active_player: PlayerID,
    cards: BTreeSet<CardByName>,
    instances: Vec<Instance>,
}

impl State {
    pub fn new(health: u32, decks: Vec<Vec<Card>>) -> State {
        let mut state = State {
            players: Vec::new(),
            active_player: PlayerID(0),
            cards: BTreeSet::new(),
            instances: Vec::new(),
        };

        let health = health as i32;
        let mut id = 0;
        for deck in decks {
            let mut player_instances = Vec::new();
            for card in deck {
                let owner = PlayerID(state.players.len());
                state.cards.insert(CardByName(card.clone()));
                state.instances.push(Instance::new(card, owner, Zone::Deck));
                player_instances.push(InstanceID(id));
                id += 1;
            }
            state.players.push(Player::new(health, player_instances));
        }
        state
    }

    pub fn add_instance(&mut self, instance: Instance) -> InstanceID {
        let instance_id = InstanceID(self.instances.len());
        self.instances.push(instance);
        instance_id
    }

    pub fn num_players(&self) -> usize {
        self.players.len()
    }

    pub fn players(&self) -> Vec<PlayerID> {
        (0..self.players.len()).map(PlayerID).collect()
    }

    pub fn player(&self, player: PlayerID) -> &Player {
        &self.players[player.0]
    }

    pub fn player_mut(&mut self, player: PlayerID) -> &mut Player {
        &mut self.players[player.0]
    }

    // pub(crate) fn card(&self, name: &str) -> Option<&Card> {
    //     self.cards.get(name).map(|cbn| &cbn.0)
    // }

    pub(crate) fn instance(&self, instance: InstanceID) -> &Instance {
        &self.instances[instance.0]
    }

    pub fn instance_mut(&mut self, instance: InstanceID) -> &mut Instance {
        &mut self.instances[instance.0]
    }

    pub fn instances(&self) -> impl Iterator<Item = InstanceID> {
        (0..self.instances.len()).map(InstanceID)
    }
}

#[derive(PartialEq, Eq)]
struct CardByName(Card);

impl Borrow<str> for CardByName {
    fn borrow(&self) -> &str {
        &self.0.name
    }
}

impl PartialOrd for CardByName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.name.partial_cmp(&other.0.name)
    }
}

impl Ord for CardByName {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.name.cmp(&other.0.name)
    }
}

impl fmt::Debug for CardByName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let state = State::new(20, vec![]);
        assert_eq!(state.players, vec![]);
        assert_eq!(state.active_player, PlayerID(0));
        assert_eq!(state.cards, BTreeSet::new());
        assert_eq!(state.instances, vec![]);
    }

    #[test]
    #[should_panic]
    fn test_player_out_of_bounds() {
        let state = State::new(20, vec![]);
        state.player(PlayerID(0));
    }

    #[test]
    fn test_player_in_bounds() {
        let state = State::new(20, vec![vec![]]);
        assert_eq!(*state.player(PlayerID(0)), Player::new(20, vec![]));
    }
}
