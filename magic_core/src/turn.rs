pub enum Phase {
    Beginning,
    PrecombatMain,
    Combat,
    PostcombatMain,
    Ending,
}

pub enum Step {
    Untap,
    Upkeep,
    Draw,

    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndOfCombat,

    EndingPhase,
    End,
    Cleanup,
}
