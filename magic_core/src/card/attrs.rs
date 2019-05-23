#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    Artifact,
    Creature,
    Enchantment,
    Instant,
    Land,
    Sorcery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Subtype {
    Aura,
    Elemental,
    Forest,
    Human,
    Soldier,
    Wall,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KeywordAbility {
    Banding,
    Defender,
    Flying,
    Hexproof,
    Shroud,
}
