use crate::game_state::GameState;
use crate::ui::UserInterface;

pub struct Controller {
    pub ui: Box<UserInterface>,
    pub game_state: GameState,
}

impl Controller {
    pub fn new(ui: impl UserInterface + 'static, game_state: GameState) -> Self {
        Controller {
            ui: Box::new(ui),
            game_state,
        }
    }
}
