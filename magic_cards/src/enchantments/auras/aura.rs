use crate::cast::put_on_battlefield;
use magic_core::action::*;
use magic_core::card::CardBuilder;
use magic_core::effect::Effect;
use magic_core::event::*;
use magic_core::instance::InstanceID;
use magic_core::state::State;
use magic_core::ui::UserInterface;
use magic_core::zone::Zone;
use std::sync::Arc;

pub fn aura_permanent(
    predicate: impl Fn(&State, InstanceID) -> bool + 'static,
    effect: impl Effect + 'static,
) -> CardBuilder {
    aura(TargetDescription::permanent(1, predicate), effect)
}

pub fn aura(target: TargetDescription, effect: impl Effect + 'static) -> CardBuilder {
    let effect = Arc::new(effect);
    CardBuilder::new()
        .on_resolve(CastAura {
            effect: effect.clone(),
        })
        .with_target(target.clone())
        .with_trigger(EnterTheBattlefieldAttachIfNot { target, effect })
}

fn attach(action: ActivatedAction, effect: Arc<Effect>) -> Event {
    let aura = action.source.instance;
    let target = action.targets.into_iter().next().unwrap();
    Event::State(
        action.source,
        StateEvent::Card(aura, CardEvent::AttachTo(target, effect)),
    )
}

#[derive(Debug)]
struct CastAura {
    effect: Arc<Effect>,
}
impl ActionResolver for CastAura {
    fn resolve(&self, state: &State, _: &mut UserInterface, action: ActivatedAction) -> Vec<Event> {
        vec![
            put_on_battlefield(state, action.source.clone()),
            attach(action, self.effect.clone()),
        ]
    }
}

#[derive(Debug)]
struct EnterTheBattlefieldAttachIfNot {
    target: TargetDescription,
    effect: Arc<Effect>,
}
impl Trigger for EnterTheBattlefieldAttachIfNot {
    fn respond(&self, state: &State, instance: InstanceID, event: &Event) -> Option<Action> {
        match event {
            Event::State(_, StateEvent::Card(card, CardEvent::MoveTo(_, Zone::Battlefield)))
                if *card == instance =>
            {
                let instance = instance.get(state);
                if instance.attached_to.is_none() {
                    return Some(
                        Action::from(AttachAura {
                            effect: self.effect.clone(),
                        })
                        .with_target(self.target.clone()),
                    );
                }
            }
            _ => (),
        }
        None
    }
}

#[derive(Debug)]
struct AttachAura {
    effect: Arc<Effect>,
}
impl ActionResolver for AttachAura {
    fn resolve(
        &self,
        _state: &State,
        _: &mut UserInterface,
        action: ActivatedAction,
    ) -> Vec<Event> {
        vec![attach(action, self.effect.clone())]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use magic_core::card::Card;

    #[derive(Debug)]
    struct DoNothingEffect;
    impl Effect for DoNothingEffect {
        fn affect(&self, _: &State, _: InstanceID, _: &mut Card) {}
    }

    #[test]
    fn test_aura_permanent() {
        let card = aura_permanent(|_, _| false, DoNothingEffect)
            .with_name("")
            .with_colors(vec![])
            .build();
        assert_eq!(card.cast_action.target_descriptions.len(), 1);
        assert_eq!(card.triggers.len(), 1);
    }

    #[test]
    fn test_aura() {
        let card = aura(
            TargetDescription::permanent(1, |_, _| false),
            DoNothingEffect,
        )
        .with_name("")
        .with_colors(vec![])
        .build();
        assert_eq!(card.cast_action.target_descriptions.len(), 1);
        assert_eq!(card.triggers.len(), 1);
    }
}
