use crate::controller::Controller;
use magic_core::action::*;
use magic_core::event::{Event, TurnEvent};
use magic_core::mana::{ManaCost, ManaPayment};
use magic_core::source::Source;
use magic_core::state::State;
use magic_core::ui::UserInterface;

impl Controller {
    pub(super) fn trigger_register(&mut self, event: TurnEvent) {
        let event = Event::Turn(self.state.active_player, event);
        self.trigger_event(&event);
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

    fn trigger_event(&mut self, event: &Event) {
        self.actions.append(&mut self.state.trigger(event))
    }

    fn build_stack(&mut self) {
        self.sort_actions();
        let actions = std::mem::replace(&mut self.actions, Vec::new());
        for action in actions {
            let resolve = action.action.resolve.clone();
            if let Some(activated) = activate(&mut *self.ui, &mut self.state, action) {
                self.stack.push((resolve, activated));
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
    action: SourcedAction,
) -> Option<ActivatedAction> {
    // Resolve targets
    let mut targets = Vec::new();
    for target_description in action.action.target_descriptions {
        if let Some(target) = ui.choose_target(state, &action.source, target_description) {
            targets.push(target);
        } else {
            return None;
        }
    }

    // Resolve optional payments to be payed.  The ones chosen become mandatory.
    let instance = action.source.instance;
    let optional_costs_selected = action
        .action
        .optional_costs
        .into_iter()
        .map(|cost| {
            if ui.read_bool(state, instance) {
                Some(cost)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Allow mana ability responses if casting a spell and there are mana costs.
    if action.action_type == ActionType::Spell
        && (action.action.mandatory_costs.iter())
            .chain(optional_costs_selected.iter().filter_map(|x| x.as_ref()))
            .any(|cost| match cost {
                Cost::Mana(_) => true,
                _ => false,
            })
    {
        allow_mana_ability_responses(ui, state);
    }

    // Select how to pay costs.  Abort (return None) if payment is not selected.
    let mut mandatory_payments = Vec::new();
    for mandatory_cost in action.action.mandatory_costs {
        if let Some(payment) = select_payment(ui, state, &action.source, mandatory_cost) {
            mandatory_payments.push(payment)
        } else {
            return None;
        }
    }

    let mut optional_payments = Vec::new();
    for optional_cost in optional_costs_selected {
        if let Some(cost) = optional_cost {
            if let Some(payment) = select_payment(ui, state, &action.source, cost) {
                optional_payments.push(Some(payment))
            } else {
                return None;
            }
        } else {
            optional_payments.push(None)
        }
    }

    // Attempt to actually pay the payments.  If we can't, don't do anything.
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

fn allow_mana_ability_responses(ui: &mut UserInterface, state: &mut State) -> Vec<Event> {
    let mut events = Vec::new();
    while let Some(ability) = ui.maybe_trigger_mana_ability(state) {
        let resolver = ability.action.resolve.clone();
        if let Some(activated) = activate(ui, state, ability) {
            assert_eq!(activated.action_type, ActionType::ActivatedAbility);
            events.append(&mut resolve(state, ui, &*resolver, activated));
        }
    }
    events
}

fn resolve(
    state: &State,
    ui: &mut UserInterface,
    resolver: &ActionResolver,
    activated: ActivatedAction,
) -> Vec<Event> {
    let events = resolver.resolve(state, activated);
    state.replacement_effects(ui, events)
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
