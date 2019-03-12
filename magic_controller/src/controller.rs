use crate::stack::Stack;
use magic_core::action::SourcedAction;
use magic_core::state::State;
use magic_core::ui::UserInterface;

pub struct Controller {
    pub(crate) ui: Box<UserInterface>,
    pub(crate) state: State,
    pub(crate) actions: Vec<SourcedAction>,
    pub(crate) stack: Stack,
}

impl Controller {
    pub fn new(ui: impl UserInterface + 'static, state: State) -> Self {
        Controller {
            ui: Box::new(ui),
            state,
            actions: Vec::new(),
            stack: Stack::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            for player in self.state.players() {
                self.state.active_player = player;
                self.turn();
            }
        }
    }
}

impl Controller {
    pub(super) fn cycle_priority(&mut self) {
        unimplemented!()
    }

    pub(super) fn untap(&mut self) {
        unimplemented!()
    }

    pub(super) fn draw(&mut self) {
        unimplemented!()
    }

    pub(super) fn main(&mut self) {
        unimplemented!()
    }

    pub(super) fn declare_attackers(&mut self) -> bool {
        unimplemented!()
    }

    pub(super) fn declare_blockers(&mut self) {
        unimplemented!()
    }

    pub(super) fn combat_damage(&mut self) {
        unimplemented!()
    }

    pub(super) fn cleanup(&mut self) {
        unimplemented!()
    }
}
