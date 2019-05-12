use magic_core::action::{ActionResolver, ActivatedAction, Target, TargetDescription};
use magic_core::card::{Card, CardBuilder};
use magic_core::event::*;
use magic_core::mana::ManaCost;
use magic_core::state::State;
use magic_core::ui::UserInterface;

pub fn ancestral_recall() -> Card {
    CardBuilder::new()
        .with_name("Ancestral Recall")
        .with_mana_cost(ManaCost::new().with_blue(1))
        .on_resolve(CastAncestralRecall)
        .with_target(TargetDescription::player(1, |_, _| true))
        .build()
}

#[derive(Debug)]
struct CastAncestralRecall;
impl ActionResolver for CastAncestralRecall {
    fn resolve(&self, _: &State, _: &mut UserInterface, action: ActivatedAction) -> Vec<Event> {
        if let Target::Player(players) = &action.targets[0] {
            assert_eq!(players.len(), 1);
            vec![Event::State(
                action.source,
                StateEvent::Player(players[0], PlayerEvent::DrawCards(3)),
            )]
        } else {
            panic!()
        }
    }
}
