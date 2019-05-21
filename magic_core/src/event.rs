use crate::action::{ActivatedAction, Target};
use crate::effect::Effect;
use crate::instance::InstanceID;
use crate::player::PlayerID;
use crate::source::Source;
use crate::state::State;
use crate::turn::{Phase, Step};
use crate::zone::Zone;
use std::sync::Arc;

pub enum Event {
    State(Source, StateEvent),
    Action(PlayerID, UserEvent),
    Turn(PlayerID, TurnEvent),
}

pub enum StateEvent {
    Card(InstanceID, CardEvent),
    Player(PlayerID, PlayerEvent),
}

pub enum CardEvent {
    TakeDamage(usize),
    MoveTo(PlayerID, Zone),
    AttachTo(Target, Arc<Effect>),
}

pub enum PlayerEvent {
    TakeDamage(usize),
    DrawCards(usize),
}

pub enum UserEvent {
    PlayLand(InstanceID),
    Activate(ActivatedAction),
}

pub enum TurnEvent {
    BeginPhase(Phase),
    EndPhase(Phase),
    BeginStep(Step),
    EndStep(Step),
}

impl Event {
    pub fn move_to(source: Source, instance_id: InstanceID, player: PlayerID, zone: Zone) -> Self {
        Event::State(
            source,
            StateEvent::Card(instance_id, CardEvent::MoveTo(player, zone)),
        )
    }

    pub fn move_to_zone(
        state: &State,
        source: Source,
        instance_id: InstanceID,
        zone: Zone,
    ) -> Self {
        let instance = instance_id.get(state);
        Self::move_to(source, instance_id, instance.controller, zone)
    }
}

impl CardEvent {
    pub fn attach_to(target: Target, effect: impl Effect + 'static) -> Self {
        CardEvent::AttachTo(target, Arc::new(effect))
    }
}
