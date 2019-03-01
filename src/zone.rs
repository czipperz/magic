/// The location of a Card.
///
/// A card usually progresses from a player's hard, to the stack, to
/// the battlefield and then to the graveyard.
///
/// A card can alter the order in many different ways.  Sorceries and
/// instants go directly to the graveyard and skip the battlefield.
/// Certain spells can exile other cards.  Other spells bring cards
/// back from the graveyard or exile to another state.
///
/// These variants other than `Stack`, correspond to the fields in
/// `Player` where the `Card` is located.  The variant `Stack`
/// corresponds to the global spell stack, which is located in
/// `State`.
///
/// A card in the locations `Stack` and `Battlefield` can have a
/// different controller and owner.  In the other locations, the
/// controller must be the owner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Zone {
    Deck,
    Hand,
    Stack,
    Battlefield,
    Graveyard,
    Exile,
}
