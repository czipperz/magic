use super::aura::aura_permanent;
use magic_core::card::{Card, Subtype, Type};
use magic_core::effect::Effect;
use magic_core::mana::ManaCost;
use magic_core::permanent::{Permanent, PermanentNumber};
use magic_core::state::State;

pub fn animate_artifact() -> Card {
    aura_permanent(
        "Animate Artifact",
        ManaCost::new().with_blue(1).with_generic(3),
        is_artifact,
    )
    .with_type(Type::Enchantment)
    .with_subtype(Subtype::Aura)
    .with_effect(AnimateArtifactEffect)
}

fn is_artifact(state: &State, permanent: PermanentNumber) -> bool {
    permanent.get(state).types.contains(&Type::Artifact)
}

#[derive(Debug)]
struct AnimateArtifactEffect;
impl Effect for AnimateArtifactEffect {
    fn affect(&self, state: &State, permanent: &mut Permanent) {
        if !permanent.types.contains(&Type::Creature) {
            permanent.types.push(Type::Creature);
            let cmc = permanent
                .instance(state)
                .card(state)
                .mana_cost()
                .converted() as isize;
            permanent.power = Some(cmc);
            permanent.toughness = Some(cmc);
        }
    }
}
