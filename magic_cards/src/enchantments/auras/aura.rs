use crate::cast::put_on_battlefield;
use magic_core::action::*;
use magic_core::card::Card;
use magic_core::event::*;
use magic_core::instance::InstanceNumber;
use magic_core::mana::ManaCost;
use magic_core::permanent::PermanentNumber;
use magic_core::state::State;
use magic_core::zone::Zone;

pub fn aura_permanent(
    name: impl ToString,
    mana_cost: ManaCost,
    predicate: impl Fn(&State, PermanentNumber) -> bool + 'static,
) -> Card {
    aura(name, mana_cost, TargetDescription::permanent(1, predicate))
}

pub fn aura(name: impl ToString, mana_cost: ManaCost, target: TargetDescription) -> Card {
    Card::new(name, mana_cost, CastAura)
        .with_target(target.clone())
        .with_trigger(EnterTheBattlefieldAttachIfNot { target })
}

fn attach(action: ActivatedAction) -> Event {
    let aura = action.source.instance;
    let target = action.targets.into_iter().next().unwrap();
    Event::State(
        action.source,
        StateEvent::Card(aura, CardEvent::AttachTo(target)),
    )
}

struct CastAura;
impl ActionResolver for CastAura {
    fn resolve(&self, state: &State, action: ActivatedAction) -> Vec<Event> {
        vec![
            put_on_battlefield(state, action.source.clone()),
            attach(action),
        ]
    }
}

struct EnterTheBattlefieldAttachIfNot {
    target: TargetDescription,
}
impl Trigger for EnterTheBattlefieldAttachIfNot {
    fn respond(&self, state: &State, instance: InstanceNumber, event: &Event) -> Option<Action> {
        match event {
            Event::State(
                _,
                StateEvent::Card(card, CardEvent::MoveTo(_, _, _, Zone::Battlefield)),
            ) if *card == instance => {
                if let Some(permanent) = card.permanent(state) {
                    if permanent.affecting.is_none() {
                        return Some(Action::from(AttachAura).with_target(self.target.clone()));
                    }
                }
            }
            _ => (),
        }
        None
    }
}

struct AttachAura;
impl ActionResolver for AttachAura {
    fn resolve(&self, _state: &State, action: ActivatedAction) -> Vec<Event> {
        vec![attach(action)]
    }
}
