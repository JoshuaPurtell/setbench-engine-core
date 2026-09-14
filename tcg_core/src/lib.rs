mod action;
mod api;
mod card_meta;
mod combat;
mod custom_abilities;
#[cfg(test)]
mod e2_tests;
mod effects;
mod event;
mod game;
mod ids;
mod markers;
mod modifiers;
mod player;
mod power_locks;
mod prompt;
mod replacements;
mod restrictions;
pub mod runtime_hooks;
mod selectors;
mod serialization;
mod special_conditions;
pub mod timers;
mod triggers;
mod types;
mod view;
mod zone;

pub use action::{can_execute, execute, Action, ActionError};
pub use card_meta::{CardMeta, CardMetaMap};
pub use combat::{
    apply_damage_modifier, calculate_damage, calculate_damage_for_types,
    calculate_damage_with_flags, check_knockouts, check_knockouts_all,
    check_knockouts_all_with_cause, execute_attack, Attack, AttackCost, CombatError,
    KnockoutResult,
};
pub use effects::{
    execute_effect, execute_effect_with_source, execute_effect_with_source_and_targets,
    ContinuousTiming, EffectAst, EffectError, EffectOutcome, Predicate, Target, TargetPlayer,
};
pub use event::GameEvent;
pub use game::{BetweenTurnsEffect, PendingAttack, ToolDiscardSchedule};
pub use game::{GameState, PendingPrompt, StepResult, Turn};
pub use ids::{CardDefId, CardInstanceId, PlayerId, PokemonId};
pub use markers::Marker;
pub use modifiers::{
    AttackSelector, DamageModifierEntry, StackingRule, StatModifierEntry, StatModifierKind,
    StatModifierValue,
};
pub use player::{PlayerState, PokemonSlot};
pub use power_locks::{PowerLock, PowerLockKind};
pub use prompt::{Prompt, RevealedCard, SelectionDestination};
pub use replacements::{KnockoutCause, ReplacementEffect, ReplacementTrigger};
pub use restrictions::{Restriction, RestrictionKind, RestrictionTarget};
pub use selectors::{CardSelector, PokemonScope, PokemonSelector};
pub use serialization::PlayerStateSnapshot;
pub use serialization::{GameStateSnapshot, SnapshotError};
pub use special_conditions::{execute_between_turns, SpecialConditionError};
pub use tcg_rules_ex::Phase;
pub use tcg_rules_ex::SpecialCondition;
pub use types::{EnergyAttachmentSource, Resistance, Type, Weakness};
/// Alias for Type, used in energy card implementations.
pub type EnergyType = Type;
pub use api::Game;
pub use custom_abilities::{
    after_attack, after_retreat, apply_attack_overrides, apply_between_turns_custom,
    apply_post_attack_custom, apply_tool_stadium_effects, attack_cost_modifier, before_damage,
    can_attach_tool, can_use_pokepower_override, card_has_power_or_body, energy_provides_override,
    execute_custom_power, is_pokebody_active_override, on_energy_attached, on_tool_attached,
    power_effect_id_for, power_is_once_per_turn, register_card_triggers, resolve_custom_prompt,
    AttackOverrides,
};
pub use game::TriggerHandler;
pub use timers::{PlayerTimerState, TimerConfig, TimerView};
pub use triggers::{
    TriggerBus, TriggerEvent, TriggerKind, TriggerPredicate, TriggerSubscription, TriggeredEffect,
};
pub use types::Stage;
pub use view::{ActionHints, GameView, PokemonView};
pub use zone::{CardInstance, Zone, ZoneKind, ZoneRef};
