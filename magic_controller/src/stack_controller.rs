use crate::controller::Controller;
use magic_core::action::*;
use magic_core::event::{Event, TurnEvent};
use magic_core::source::Source;
use magic_core::state::State;
use magic_core::ui::UserInterface;

impl Controller {
    pub(super) fn trigger_register(&mut self, event: TurnEvent) {
        let event = Event::Turn(self.state.active_player, event);
        self.trigger_event(event);
    }

    pub(super) fn trigger(&mut self, event: TurnEvent) {
        self.trigger_register(event);
        self.build_stack();
        self.resolve_stack();
    }

    pub(super) fn trigger_yield(&mut self, event: TurnEvent) {
        self.trigger(event);
        self.cycle_priority();
    }

    fn trigger_event(&mut self, event: Event) {
        for player_number in self.state.players() {
            let player = player_number.get(&self.state);
            for instance in &player.battlefield {
                let permanent = instance.get(&self.state).permanent(&self.state).unwrap();
                for trigger in &permanent.triggers {
                    if let Some(action) = trigger.respond(&self.state, *instance, &event) {
                        let source = Source {
                            controller: player_number,
                            instance: *instance,
                        };
                        self.actions
                            .push((ActionType::TriggeredAbility, source, action))
                    }
                }
            }
        }
    }

    fn build_stack(&mut self) {
        self.sort_actions();
        let actions = std::mem::replace(&mut self.actions, Vec::new());
        for (action_type, source, action) in actions {
            let resolve = action.resolve.clone();
            let activated = activate(&mut *self.ui, &mut self.state, action_type, source, action);
            self.stack.push(resolve, activated);
        }
    }

    fn sort_actions(&mut self) {
        // self.ui.sort_actions(&mut self.actions);
        unimplemented!()
    }

    fn resolve_stack(&mut self) {
        unimplemented!()
    }
}

fn activate(
    ui: &mut UserInterface,
    state: &mut State,
    action_type: ActionType,
    source: Source,
    action: Action,
) -> ActivatedAction {
    // resolve targets
    let mut targets = Vec::new();
    for target_description in action.target_descriptions {
        targets.push(ui.choose_target(state, target_description));
    }

    // resolve payments
    let mut mandatory_payments = Vec::new();
    for mandatory_cost in action.mandatory_costs {
        mandatory_payments.push(pay(ui, state, mandatory_cost))
    }

    let mut optional_payments = Vec::new();
    for optional_cost in action.optional_costs {
        optional_payments.push(maybe_pay(ui, state, optional_cost))
    }

    ActivatedAction {
        action_type,
        source,
        targets,
        mandatory_payments,
        optional_payments,
    }
}

fn pay(ui: &mut UserInterface, state: &mut State, mandatory_cost: Cost) -> Payment {
    unimplemented!()
}

fn maybe_pay(ui: &mut UserInterface, state: &mut State, mandatory_cost: Cost) -> Option<Payment> {
    unimplemented!()
}