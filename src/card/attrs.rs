#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Artifact,
    Creature,
    Enchantment,
    Instant,
    Land,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Subtype {
    Aura,
    Elemental,
    Wall,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Attribute {
    Defender,
    Flying,
    Hexproof,
    Shroud,
}
