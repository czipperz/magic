use super::aura::aura_permanent;
use magic_core::card::{Card, Subtype, Type};
use magic_core::effect::Effect;
use magic_core::mana::ManaCost;
use magic_core::permanent::{Permanent, PermanentID};
use magic_core::permission::Permission;
use magic_core::state::State;

pub fn animate_wall() -> Card {
    aura_permanent(is_wall, AnimateWallEffect)
        .with_name("Animate Wall")
        .with_mana_cost(ManaCost::new().with_white(1))
        .with_type(Type::Enchantment)
        .with_subtype(Subtype::Aura)
        .build()
}

fn is_wall(state: &State, instance: PermanentID) -> bool {
    instance.get(state).subtypes.contains(&Subtype::Wall)
}

#[derive(Debug)]
struct AnimateWallEffect;
impl Effect for AnimateWallEffect {
    fn affect(&self, _: &State, permanent: &mut Permanent) {
        permanent.permissions.add(Permission::Attack);
    }
}
