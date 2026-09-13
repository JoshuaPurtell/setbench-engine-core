//! Runtime hooks registry.
//!
//! `create_hooks(set)` installs **one** expansion. A Crystal Guardians match must
//! not run Dragon Frontiers bodies.

use tcg_core::runtime_hooks::RuntimeHooks;

/// Create the RuntimeHooks vtable for a single expansion (`"CG"`, `"DF"`, or `"HP"`).
pub fn create_hooks(set: &str) -> RuntimeHooks {
    match set {
        "CG" | "cg" | "crystal_guardians" => crate::cg::hooks::create(),
        "DF" | "df" | "dragon_frontiers" => crate::df::hooks::create(),
        "HP" | "hp" | "holon_phantoms" => crate::hp::hooks::create(),
        other => panic!("unknown expansion set {other:?}; expected CG, DF, or HP"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tcg_core::runtime_hooks::def_id_matches;
    use tcg_core::{
        CardDefId, CardInstance, CardMeta, CardMetaMap, GameState, PlayerId, PokemonSlot, Stage,
        Type,
    };
    use tcg_rules_ex::RulesetConfig;

    fn holon_veil_game() -> (GameState, tcg_core::CardInstanceId) {
        let mut card_meta = CardMetaMap::new();
        card_meta.insert(
            CardDefId::new("DF-1"),
            CardMeta {
                name: "Ampharos Delta".to_string(),
                is_basic: false,
                is_tool: false,
                is_stadium: false,
                is_pokemon: true,
                is_energy: false,
                hp: 0,
                energy_kind: None,
                provides: Vec::new(),
                trainer_kind: None,
                is_ex: false,
                is_star: false,
                is_delta: true,
                stage: Stage::Stage2,
                types: vec![Type::Lightning],
                weakness: None,
                resistance: None,
                retreat_cost: None,
                trainer_effect: None,
                evolves_from: Some("Flaaffy".to_string()),
                attacks: Vec::new(),
                card_type: String::new(),
                delta_species: false,
            },
        );
        card_meta.insert(
            CardDefId::new("CG-050"),
            CardMeta {
                name: "Non-Delta".to_string(),
                is_basic: true,
                is_tool: false,
                is_stadium: false,
                is_pokemon: true,
                is_energy: false,
                hp: 0,
                energy_kind: None,
                provides: Vec::new(),
                trainer_kind: None,
                is_ex: false,
                is_star: false,
                is_delta: false,
                stage: Stage::Basic,
                types: vec![Type::Colorless],
                weakness: None,
                resistance: None,
                retreat_cost: None,
                trainer_effect: None,
                evolves_from: None,
                attacks: Vec::new(),
                card_type: String::new(),
                delta_species: false,
            },
        );
        let mut game = GameState::new_with_card_meta(
            Vec::new(),
            Vec::new(),
            5,
            RulesetConfig::default(),
            card_meta,
        );
        let ampharos = PokemonSlot::new(CardInstance::new(CardDefId::new("DF-1"), PlayerId::P1));
        let other = PokemonSlot::new(CardInstance::new(CardDefId::new("CG-050"), PlayerId::P1));
        let other_id = other.card.id;
        game.players[0].active = Some(ampharos);
        game.players[0].bench.push(other);
        (game, other_id)
    }

    #[test]
    fn df_hooks_apply_holon_veil() {
        let (mut game, other_id) = holon_veil_game();
        game.set_hooks(create_hooks("DF"));
        assert!(game.pokemon_is_delta(other_id));
        assert!(def_id_matches(&CardDefId::new("DF-1"), "DF", 1));
    }

    #[test]
    fn cg_hooks_do_not_apply_holon_veil() {
        let (mut game, other_id) = holon_veil_game();
        game.set_hooks(create_hooks("CG"));
        assert!(!game.pokemon_is_delta(other_id));
    }
}
