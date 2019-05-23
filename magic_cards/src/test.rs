pub use x::*;

#[cfg(not(test))]
mod x {}

#[cfg(test)]
mod x {
    use magic_core::card::{Card, CardBuilder};
    use magic_core::instance::{Instance, InstanceID};
    use magic_core::mana::ManaCost;
    use magic_core::state::State;
    use magic_core::zone::Zone;

    pub fn base_card() -> CardBuilder {
        CardBuilder::new()
            .with_name("")
            .with_mana_cost(ManaCost::new().with_generic(4))
            .on_resolve(crate::cast::CastPermanent)
    }

    pub fn state_with_card(card: Card) -> (State, InstanceID) {
        let mut state = State::new(0, vec![vec![card.clone()]]);
        let instance =
            state.add_instance(Instance::new(card, state.players()[0], Zone::Battlefield));
        (state, instance)
    }
}
