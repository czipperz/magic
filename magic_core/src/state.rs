use crate::card::{Card, CardID};
use crate::instance::{Instance, InstanceID};
use crate::permanent::{Permanent, PermanentID};
use crate::player::{Player, PlayerID};
use crate::zone::Zone;

pub struct State {
    players: Vec<Player>,
    pub active_player: PlayerID,
    cards: Vec<Card>,
    instances: Vec<Instance>,
    permanents: Vec<Permanent>,
}

impl State {
    pub fn new(health: u32, decks: Vec<Vec<Card>>) -> State {
        let mut state = State {
            players: Vec::new(),
            active_player: PlayerID(0),
            cards: Vec::new(),
            instances: Vec::new(),
            permanents: Vec::new(),
        };

        let health = health as i32;
        let mut id = 0;
        for deck in decks {
            let mut player_instances = Vec::new();
            for card in deck {
                let owner = PlayerID(state.players.len());
                state.cards.push(card);
                state
                    .instances
                    .push(Instance::new(CardID(id), owner, Zone::Deck));
                player_instances.push(InstanceID(id));
                id += 1;
            }
            state.players.push(Player::new(health, player_instances));
        }
        state
    }

    pub fn add_permanent_from(&mut self, instance_id: InstanceID) -> PermanentID {
        self.add_permanent(Permanent::new(instance_id, instance_id.card(self)))
    }

    pub fn add_permanent(&mut self, permanent: Permanent) -> PermanentID {
        let permanent_id = PermanentID(self.permanents.len());

        {
            let instance = self.instance_mut(permanent.instance);
            assert_eq!(instance.permanent, None);
            instance.permanent = Some(permanent_id);
        }

        self.permanents.push(permanent);
        permanent_id
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

    pub(crate) fn card(&self, card: CardID) -> &Card {
        &self.cards[card.0]
    }

    pub(crate) fn instance(&self, instance: InstanceID) -> &Instance {
        &self.instances[instance.0]
    }

    pub fn instance_mut(&mut self, instance: InstanceID) -> &mut Instance {
        &mut self.instances[instance.0]
    }

    pub(crate) fn permanent(&self, permanent: PermanentID) -> &Permanent {
        &self.permanents[permanent.0]
    }

    pub fn permanents(&self) -> impl Iterator<Item = PermanentID> {
        (0..self.permanents.len()).map(PermanentID)
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
        assert_eq!(state.cards, vec![]);
        assert_eq!(state.instances, vec![]);
        assert_eq!(state.permanents, vec![]);
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
