use crate::controller::Controller;
use crate::stack::Stack;
use magic_core::action::*;
use magic_core::event::{Event, TurnEvent};
use magic_core::mana::{ManaCost, ManaPayment};
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
                        self.actions.push(SourcedAction {
                            action_type: ActionType::TriggeredAbility,
                            source,
                            action,
                        })
                    }
                }
            }
        }
    }

    fn build_stack(&mut self) {
        self.sort_actions();
        let actions = std::mem::replace(&mut self.actions, Vec::new());
        for action in actions {
            let resolve = action.action.resolve.clone();
            if let Some(activated) =
                activate(&mut *self.ui, &mut self.state, &mut self.stack, action)
            {
                self.stack.push(resolve, activated);
            }
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
    stack: &mut Stack,
    action: SourcedAction,
) -> Option<ActivatedAction> {
    // resolve targets
    let mut targets = Vec::new();
    for target_description in action.action.target_descriptions {
        if let Some(target) = ui.choose_target(state, &action.source, target_description) {
            targets.push(target);
        } else {
            return None;
        }
    }

    if action.action_type == ActionType::Spell
        && (action.action.mandatory_costs.iter())
            .chain(action.action.optional_costs.iter())
            .any(|cost| match cost {
                Cost::Mana(_) => true,
                _ => false,
            })
    {
        allow_mana_ability_responses(ui, state, stack);
    }

    // resolve payments
    let mut mandatory_payments = Vec::new();
    for mandatory_cost in action.action.mandatory_costs {
        if let Some(payment) = select_payment(ui, state, &action.source, mandatory_cost) {
            mandatory_payments.push(payment)
        } else {
            return None;
        }
    }

    let mut optional_payments = Vec::new();
    for optional_cost in action.action.optional_costs {
        optional_payments.push(select_payment(ui, state, &action.source, optional_cost));
    }

    if pay_payments(
        state,
        mandatory_payments
            .iter()
            .chain(optional_payments.iter().filter_map(|x| x.as_ref())),
    ) {
        Some(ActivatedAction {
            action_type: action.action_type,
            source: action.source,
            targets,
            mandatory_payments,
            optional_payments,
        })
    } else {
        None
    }
}

fn allow_mana_ability_responses(
    ui: &mut UserInterface,
    state: &mut State,
    stack: &mut Stack,
) -> Vec<Event> {
    let mut events = Vec::new();
    while let Some(ability) = ui.maybe_trigger_mana_ability(state) {
        let resolve = ability.action.resolve.clone();
        if let Some(activated) = activate(ui, state, stack, ability) {
            assert_eq!(activated.action_type, ActionType::ActivatedAbility);
            stack.push(resolve, activated);
            events.append(&mut stack.pop(state).unwrap());
        }
    }
    events
}

fn select_payment(
    ui: &mut UserInterface,
    state: &State,
    source: &Source,
    mandatory_cost: Cost,
) -> Option<Payment> {
    match mandatory_cost {
        Cost::Mana(mana_cost) => select_mana(ui, state, mana_cost),
        Cost::Sacrifice(number, predicate) => select_sacrifice(
            ui,
            state,
            source,
            TargetDescription::Permanent(number, predicate),
        ),
    }
}

fn select_mana(_ui: &mut UserInterface, _state: &State, cost: ManaCost) -> Option<Payment> {
    // must update to support new forms of mana
    let ManaCost { pool, generic } = cost;
    Some(Payment::Mana(ManaPayment { pool, generic }))
}

fn select_sacrifice(
    ui: &mut UserInterface,
    state: &State,
    source: &Source,
    target: TargetDescription,
) -> Option<Payment> {
    ui.choose_target(state, source, target)
        .map(|target| match target {
            Target::Permanent(permanents) => Payment::Sacrifice(permanents),
            _ => unreachable!(),
        })
}

fn pay_payments<'a, I: Iterator<Item = &'a Payment>>(_state: &mut State, _payments: I) -> bool {
    unimplemented!()
}
