use crate::card::{Card, CardNumber};
use crate::instance::{Instance, InstanceNumber};
use crate::permanent::{Permanent, PermanentNumber};
use crate::player::{Player, PlayerNumber};
use crate::zone::Zone;

pub struct State {
    players: Vec<Player>,
    pub active_player: PlayerNumber,
    cards: Vec<Card>,
    instances: Vec<Instance>,
    permanents: Vec<Permanent>,
}

impl State {
    pub fn new(health: u32, decks: Vec<Vec<Card>>) -> State {
        let mut state = State {
            players: Vec::new(),
            active_player: PlayerNumber { number: 0 },
            cards: Vec::new(),
            instances: Vec::new(),
            permanents: Vec::new(),
        };

        let health = health as i32;
        let mut number = 0;
        for deck in decks {
            let mut player_instances = Vec::new();
            for card in deck {
                let owner = PlayerNumber {
                    number: state.players.len(),
                };
                state.cards.push(card);
                state.instances.push(Instance {
                    card: CardNumber { number },
                    permanent: None,
                    owner,
                    controller: owner,
                    zone: Zone::Deck,
                });
                player_instances.push(InstanceNumber { number });
                number += 1;
            }
            state.players.push(Player::new(health, player_instances));
        }
        state
    }

    pub fn add_permanent(&mut self, instance_number: InstanceNumber) -> PermanentNumber {
        let permanent_number = PermanentNumber {
            number: self.permanents.len(),
        };

        {
            let instance = self.instance_mut(instance_number);
            assert_eq!(instance.permanent, None);
            instance.permanent = Some(permanent_number);
        }

        let instance = self.instance(instance_number);
        let card = self.card(instance.card);
        let permanent = Permanent {
            instance: instance_number,
            effects: Vec::new(),
            affecting: None,
            tapped: false,

            colors: card.colors.clone(),
            types: card.types.clone(),
            subtypes: card.subtypes.clone(),
            attributes: card.attributes.clone(),
            abilities: card.abilities.clone(),
            triggers: card.triggers.clone(),
            replacement_effects: card.replacement_effects.clone(),
            color_words: card.color_words.clone(),
            power: card.power.clone(),
            toughness: card.toughness.clone(),

            ignored_attributes: Vec::new(),
        };
        self.permanents.push(permanent);
        permanent_number
    }

    pub fn add_instance(
        &mut self,
        card_number: CardNumber,
        owner: PlayerNumber,
        zone: Zone,
    ) -> InstanceNumber {
        let instance_number = InstanceNumber {
            number: self.instances.len(),
        };
        let instance = Instance {
            card: card_number,
            permanent: None,
            owner,
            controller: owner,
            zone,
        };
        self.instances.push(instance);
        instance_number
    }

    pub fn num_players(&self) -> usize {
        self.players.len()
    }

    pub fn players(&self) -> Vec<PlayerNumber> {
        (0..self.players.len())
            .map(|number| PlayerNumber { number })
            .collect()
    }

    pub fn player(&self, player: PlayerNumber) -> &Player {
        &self.players[player.number]
    }

    pub fn player_mut(&mut self, player: PlayerNumber) -> &mut Player {
        &mut self.players[player.number]
    }

    pub(crate) fn card(&self, card: CardNumber) -> &Card {
        &self.cards[card.number]
    }

    pub(crate) fn instance(&self, instance: InstanceNumber) -> &Instance {
        &self.instances[instance.number]
    }

    pub fn instance_mut(&mut self, instance: InstanceNumber) -> &mut Instance {
        &mut self.instances[instance.number]
    }

    pub(crate) fn permanent(&self, permanent: PermanentNumber) -> &Permanent {
        &self.permanents[permanent.number]
    }
}
