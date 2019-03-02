use crate::card::Card;
use crate::mana::Color;
use crate::player::PlayerNumber;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A `Bundle` describes the information specified on a card.
///
/// It is up to the implementation of a card to dictate what the order
/// of the values mean.
///
/// # Examples
///
/// Take for example the instant Ancestral Recall.  It reads "Target
/// player draws three cards".  To implement this, we need to target a
/// player when we play it.  Then we have to allow for responses (such
/// as other instants or abilities).  Then we finally have that target
/// player draw three cards.  The `Bundle` for this card stores the
/// target player.  This extracts the state of the computations out
/// from the computations themselves.  This also allows for more
/// advanced effects to interact with each-other in a standard format.
///
/// Let's take for example the card Alter Reality.  It states "Change
/// the text of target spell or permanent by replacing all instances
/// of one color word with another".  Consider Absolute Grace.  It is
/// an enchantment that reads "All creatures have protection from
/// black".  To have these cards correctly interact, Absolute Grade
/// must expose a `Bundle` with the color `Black`.  Then `Alter
/// Reality` can simply change the color to fit the new standard.
#[derive(Clone, Debug, Default)]
pub struct Bundle {
    pub map: HashMap<&'static str, BundleItem>,
}

#[derive(Clone, Debug)]
pub enum BundleItem {
    // Should this be Weak?
    Card(Arc<Mutex<Card>>),
    Color(Color),
    Player(PlayerNumber),
    List(Vec<BundleItem>),
}

impl BundleItem {
    pub fn unwrap_card(&self) -> Arc<Mutex<Card>> {
        if let BundleItem::Card(c) = self {
            c.clone()
        } else {
            unreachable!()
        }
    }
    pub fn unwrap_color(&self) -> Color {
        if let BundleItem::Color(c) = self {
            *c
        } else {
            unreachable!()
        }
    }
    pub fn unwrap_player(&self) -> PlayerNumber {
        if let BundleItem::Player(p) = self {
            *p
        } else {
            unreachable!()
        }
    }
}
