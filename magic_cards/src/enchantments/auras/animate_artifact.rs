use super::aura::aura_permanent;
use magic_core::card::{Card, Subtype, Type};
use magic_core::effect::Effect;
use magic_core::instance::InstanceID;
use magic_core::mana::ManaCost;
use magic_core::state::State;

pub fn animate_artifact() -> Card {
    aura_permanent(is_artifact, AnimateArtifactEffect)
        .with_name("Animate Artifact")
        .with_mana_cost(ManaCost::new().with_blue(1).with_generic(3))
        .with_type(Type::Enchantment)
        .with_subtype(Subtype::Aura)
        .build()
}

fn is_artifact(state: &State, instance: InstanceID) -> bool {
    instance.get(state).types.contains(&Type::Artifact)
}

#[derive(Debug)]
struct AnimateArtifactEffect;
impl Effect for AnimateArtifactEffect {
    fn affect(&self, _: &State, _: InstanceID, card: &mut Card) {
        if !card.types.contains(&Type::Creature) {
            card.types.push(Type::Creature);
            let cmc = card.mana_cost().converted() as isize;
            card.power = Some(cmc);
            card.toughness = Some(cmc);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_constructor() {
        let card = animate_artifact();
        assert_eq!(card.name, "Animate Artifact");
        assert_eq!(
            *card.mana_cost(),
            ManaCost::new().with_blue(1).with_generic(3)
        );
        assert_eq!(card.types, &[Type::Enchantment]);
        assert_eq!(card.subtypes, &[Subtype::Aura]);
    }
}
