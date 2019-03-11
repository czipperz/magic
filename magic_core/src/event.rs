use crate::action::{ActivatedAction, Target};
use crate::instance::InstanceNumber;
use crate::player::PlayerNumber;
use crate::source::Source;
use crate::state::State;
use crate::turn::{Phase, Step};
use crate::zone::Zone;

pub enum Event {
    State(Source, StateEvent),
    Action(PlayerNumber, UserEvent),
    Turn(PlayerNumber, TurnEvent),
}

pub enum StateEvent {
    Card(InstanceNumber, CardEvent),
    Player(PlayerNumber, PlayerEvent),
}

pub enum CardEvent {
    TakeDamage(usize),
    MoveTo(PlayerNumber, Zone, PlayerNumber, Zone),
    AttachTo(Target),
}

pub enum PlayerEvent {
    TakeDamage(usize),
    DrawCards(usize),
}

pub enum UserEvent {
    PlayLand(InstanceNumber),
    Activate(ActivatedAction),
}

pub enum TurnEvent {
    BeginPhase(Phase),
    EndPhase(Phase),
    BeginStep(Step),
    EndStep(Step),
}

impl Event {
    pub fn move_to_zone(
        state: &State,
        source: Source,
        instance_number: InstanceNumber,
        zone: Zone,
    ) -> Self {
        let instance = instance_number.get(state);
        Event::State(
            source,
            StateEvent::Card(
                instance_number,
                CardEvent::MoveTo(
                    instance.controller,
                    instance.zone,
                    instance.controller,
                    zone,
                ),
            ),
        )
    }
}
