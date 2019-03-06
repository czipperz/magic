use crate::card::Instance;
use crate::game_state::GameState;
use crate::permanent::Permanent;
use crate::player::PlayerNumber;
use crate::source::Source;
use std::sync::{Arc, Mutex};

pub trait UserInterface {
    fn choose_player(&mut self, game_state: &GameState, source: &Source) -> Option<PlayerNumber>;

    fn choose_permanent(
        &mut self,
        game_state: &GameState,
        source: &Source,
        predicate: &Fn(&Arc<Mutex<Permanent>>) -> bool,
    ) -> Option<Arc<Mutex<Permanent>>>;

    fn choose_card(
        &mut self,
        state: &GameState,
        source: &Source,
        predicate: &Fn(&Arc<Mutex<Instance>>) -> bool,
    ) -> Option<Arc<Mutex<Instance>>>;
}
