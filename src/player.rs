use crate::card::Instance;
use crate::card::{Card, Zone};
use crate::permanent::Permanent;
use crate::source::Source;
use std::sync::{Arc, Mutex};

pub type PlayerNumber = usize;

pub struct Player {
    health: isize,
    deck: Vec<Arc<Mutex<Instance>>>,
    hand: Vec<Arc<Mutex<Instance>>>,
    battlefield: Vec<Arc<Mutex<Permanent>>>,
    graveyard: Vec<Arc<Mutex<Instance>>>,
    exile: Vec<Arc<Mutex<Instance>>>,
}

fn build_cards(vec: Vec<Card>, zone: Zone) -> Vec<Arc<Mutex<Instance>>> {
    vec.into_iter()
        .map(|card| {
            let owner = card.owner();
            Arc::new(Mutex::new(Instance::new(card, owner, zone)))
        })
        .collect()
}

impl Player {
    pub fn new(health: isize, deck: Vec<Card>) -> Self {
        Player {
            health,
            deck: build_cards(deck, Zone::Deck),
            hand: Vec::new(),
            battlefield: Vec::new(),
            graveyard: Vec::new(),
            exile: Vec::new(),
        }
    }

    pub fn inject(
        health: isize,
        deck: Vec<Card>,
        hand: Vec<Card>,
        battlefield: Vec<Permanent>,
        graveyard: Vec<Card>,
        exile: Vec<Card>,
    ) -> Self {
        Player {
            health,
            deck: build_cards(deck, Zone::Deck),
            hand: build_cards(hand, Zone::Hand),
            battlefield: battlefield
                .into_iter()
                .map(Mutex::new)
                .map(Arc::new)
                .collect(),
            graveyard: build_cards(graveyard, Zone::Graveyard),
            exile: build_cards(exile, Zone::Exile),
        }
    }

    pub fn battlefield(&self) -> &[Arc<Mutex<Permanent>>] {
        &self.battlefield
    }

    pub fn graveyard(&self) -> &[Arc<Mutex<Instance>>] {
        &self.graveyard
    }

    pub fn exile(&self) -> &[Arc<Mutex<Instance>>] {
        &self.exile
    }

    pub fn is_valid_target(&self, _source: &Source) -> bool {
        // TODO: implement hexproof and shroud for players
        true
    }

    pub(crate) fn resolve_draw_cards(&mut self, num: usize) -> Result<(), ()> {
        // TODO: implement draw card triggers
        // TODO: implement losing when out of cards (Err returned)
        for _ in 0..num {
            self.hand.push(self.deck.pop().ok_or(())?);
        }
        Ok(())
    }

    pub(crate) fn resolve_take_damage(&mut self, damage: usize) -> Result<(), ()> {
        self.health -= damage as isize;
        if self.health > 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
impl Player {
    pub fn inject_into_battlefield(&mut self, card: Arc<Mutex<Permanent>>) {
        self.battlefield.push(card);
    }
}
