use super::Controller;
use magic_core::event::TurnEvent;
use magic_core::turn::{Phase, Step};

impl Controller {
    pub(super) fn turn(&mut self) {
        self.beginning_phase();
        self.precombat_main_phase();
        self.combat_phase();
        self.postcombat_main_phase();
        self.ending_phase();
    }

    fn beginning_phase(&mut self) {
        self.trigger(TurnEvent::BeginPhase(Phase::Beginning));
        self.untap_step();
        self.upkeep_step();
        self.draw_step();
        self.trigger(TurnEvent::EndPhase(Phase::Beginning));
    }

    fn untap_step(&mut self) {
        self.untap();
        self.trigger_register(TurnEvent::BeginStep(Step::Untap));
        self.trigger_register(TurnEvent::EndStep(Step::Untap));
    }

    fn upkeep_step(&mut self) {
        self.trigger(TurnEvent::BeginStep(Step::Upkeep));
        self.trigger(TurnEvent::EndStep(Step::Upkeep));
    }

    fn draw_step(&mut self) {
        self.draw();
        self.trigger(TurnEvent::BeginStep(Step::Draw));
        self.trigger(TurnEvent::EndStep(Step::Draw));
    }

    fn precombat_main_phase(&mut self) {
        self.trigger(TurnEvent::BeginPhase(Phase::PrecombatMain));
        self.main();
        self.trigger(TurnEvent::EndPhase(Phase::PrecombatMain));
    }

    fn combat_phase(&mut self) {
        self.trigger_yield(TurnEvent::BeginPhase(Phase::Combat));
        self.beginning_of_combat_step();
        if self.declare_attackers_step() {
            self.declare_blockers_step();
            self.combat_damage_step();
        }
        self.end_of_combat_step();
        self.trigger_yield(TurnEvent::EndPhase(Phase::Combat));
    }

    fn beginning_of_combat_step(&mut self) {
        self.trigger_yield(TurnEvent::BeginStep(Step::BeginningOfCombat));
        self.trigger_yield(TurnEvent::BeginStep(Step::BeginningOfCombat));
    }

    fn declare_attackers_step(&mut self) -> bool {
        self.trigger_yield(TurnEvent::BeginStep(Step::DeclareAttackers));
        let any_declared = self.declare_attackers();
        self.trigger_yield(TurnEvent::EndStep(Step::DeclareAttackers));
        any_declared
    }

    fn declare_blockers_step(&mut self) {
        self.trigger_yield(TurnEvent::BeginStep(Step::DeclareBlockers));
        self.declare_blockers();
        self.trigger_yield(TurnEvent::EndStep(Step::DeclareBlockers));
    }

    fn combat_damage_step(&mut self) {
        self.trigger_yield(TurnEvent::BeginStep(Step::CombatDamage));
        self.combat_damage();
        self.trigger_yield(TurnEvent::EndStep(Step::CombatDamage));
    }

    fn end_of_combat_step(&mut self) {
        self.trigger_yield(TurnEvent::BeginStep(Step::EndOfCombat));
        self.trigger_yield(TurnEvent::EndStep(Step::EndOfCombat));
    }

    fn postcombat_main_phase(&mut self) {
        self.trigger(TurnEvent::BeginPhase(Phase::PostcombatMain));
        self.main();
        self.trigger(TurnEvent::EndPhase(Phase::PostcombatMain));
    }

    fn ending_phase(&mut self) {
        self.trigger(TurnEvent::BeginPhase(Phase::Ending));
        self.end_step();
        self.cleanup_step();
        self.trigger(TurnEvent::EndPhase(Phase::Ending));
    }

    fn end_step(&mut self) {
        self.trigger_yield(TurnEvent::BeginStep(Step::End));
        self.trigger_yield(TurnEvent::EndStep(Step::End));
    }

    fn cleanup_step(&mut self) {
        self.trigger(TurnEvent::BeginStep(Step::Cleanup));
        self.cleanup();
        self.trigger(TurnEvent::EndStep(Step::Cleanup));
    }
}
