use magic_core::action::{ActivatedAction, ResolveAction, Target, TargetDescription};
use magic_core::card::Card;
use magic_core::event::*;
use magic_core::mana::ManaCost;
use magic_core::state::State;

pub fn ancestral_recall() -> Card {
    Card::new(
        "Ancestral Recall",
        ManaCost::new().with_blue(1),
        CastAncestralRecall,
    )
    .with_target(TargetDescription::player(1, |_, _| true))
}

struct CastAncestralRecall;

impl ResolveAction for CastAncestralRecall {
    fn resolve(&self, _: &State, action: ActivatedAction) -> Vec<Event> {
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
