use crate::controller::Controller;
use magic_core::action::*;
use magic_core::event::{Event, TurnEvent};
use magic_core::instance::InstanceID;
use magic_core::mana::{ManaCost, ManaPayment, ManaPool};
use magic_core::player::PlayerID;
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
                self.stack.push((resolve.0, activated));
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
        &action.source,
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
            events.append(&mut resolve(state, ui, &**resolver, activated));
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
    let events = resolver.resolve(state, ui, activated);
    state.replacement_effects(ui, events)
}

fn select_payment(
    ui: &mut UserInterface,
    state: &State,
    source: &Source,
    mandatory_cost: Cost,
) -> Option<Payment> {
    match mandatory_cost {
        Cost::Tap => {
            if source.instance.get(state).tapped {
                None
            } else {
                Some(Payment::Tap)
            }
        }
        Cost::Mana(mana_cost) => select_mana(ui, state, mana_cost),
        Cost::Sacrifice(count, predicate) => select_sacrifice(
            ui,
            state,
            source,
            TargetDescription::permanent(count, {
                let controller = source.controller;
                move |state, instance_id| {
                    let instance = instance_id.get(state);
                    instance.controller == controller && predicate(state, instance_id)
                }
            }),
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
            Target::Instance(permanents) => Payment::Sacrifice(permanents),
            _ => unreachable!(),
        })
}

fn pay_payments<'a, I: Iterator<Item = &'a Payment>>(
    state: &mut State,
    source: &Source,
    payments: I,
) -> bool {
    if let Some(payments) = unify_payments(payments) {
        for payment in payments {
            match payment {
                Payment::Tap => pay_tap(state, source.instance),
                Payment::Mana(payment) => {
                    if !pay_mana_payment(state, source.controller, payment) {
                        return false;
                    }
                }
                Payment::Sacrifice(sacrifices) => {
                    pay_sacrifices(state, source.controller, sacrifices)
                }
            }
        }
        true
    } else {
        false
    }
}

fn pay_tap(state: &mut State, instance: InstanceID) {
    let instance = state.instance_mut(instance);
    assert!(!instance.tapped);
    instance.tapped = true;
}

fn pay_mana_payment(state: &mut State, player: PlayerID, payment: ManaPayment) -> bool {
    let ManaPayment {
        pool:
            ManaPool {
                blue,
                white,
                green,
                red,
                black,
                colorless,
            },
        mut generic,
    } = payment;

    // Ensure have enough colored mana.
    let player_mana_pool = &mut state.player_mut(player).floating_mana;
    let mut mana_pool = player_mana_pool.clone();
    for (pooled, required) in vec![
        (&mut mana_pool.blue, blue),
        (&mut mana_pool.white, white),
        (&mut mana_pool.green, green),
        (&mut mana_pool.red, red),
        (&mut mana_pool.black, black),
        (&mut mana_pool.colorless, colorless),
    ]
    .into_iter()
    {
        if *pooled < required {
            return false;
        }
        *pooled -= required;
    }

    // Ensure have enough mana to pay for generic
    if mana_pool.converted() < generic {
        return false;
    }

    // Pay for generic mana
    for pool in vec![
        &mut mana_pool.blue,
        &mut mana_pool.white,
        &mut mana_pool.green,
        &mut mana_pool.red,
        &mut mana_pool.black,
        &mut mana_pool.colorless,
    ]
    .into_iter()
    {
        if generic > *pool {
            generic -= *pool;
            *pool = 0;
        } else {
            *pool -= generic;
            break;
        }
    }

    *player_mana_pool = mana_pool;
    true
}

fn pay_sacrifices(_state: &mut State, _player: PlayerID, sacrifices: Vec<InstanceID>) {
    for _sacrifice in sacrifices {
        unimplemented!()
    }
}

fn unify_payments<'a, I: Iterator<Item = &'a Payment>>(payments: I) -> Option<Vec<Payment>> {
    use std::collections::HashSet;
    let mut tapping = false;
    let mut mana_payment = ManaPayment::default();
    let mut sacrifice_payment = HashSet::new();
    for payment in payments {
        match payment {
            Payment::Tap => {
                if tapping {
                    return None;
                } else {
                    tapping = true;
                }
            }
            Payment::Mana(mana) => {
                mana_payment += mana;
            }
            Payment::Sacrifice(sacrifices) => {
                for sacrifice in sacrifices {
                    if !sacrifice_payment.insert(*sacrifice) {
                        // It already is sacrificing this permanent
                        return None;
                    }
                }
            }
        }
    }

    let mut payments = vec![
        Payment::Mana(mana_payment),
        Payment::Sacrifice(sacrifice_payment.into_iter().collect()),
    ];
    if tapping {
        payments.push(Payment::Tap);
    }
    Some(payments)
}
