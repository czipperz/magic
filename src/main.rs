use magic_cards::*;
use magic_controller::Controller;
use magic_core::state::State;
use magic_graphics::GraphicalUserInterface;

fn main() {
    let health = 20;
    let deck1 = vec![
        creatures::air_elemental(),
        instants::ancestral_recall(),
        enchantments::animate_artifact(),
    ];
    let deck2 = vec![];
    let decks = vec![deck1, deck2];
    let mut controller = Controller::new(GraphicalUserInterface::new(), State::new(health, decks));
    controller.run();
}
