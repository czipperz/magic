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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Attribute {
    Flying,
}
