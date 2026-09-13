# SetBench Engine Core

Public Rust workspace for SetBench engine tasks.

This repo contains:

- `tcg_core/`: shared Pokemon TCG engine primitives and runtime hooks (no set collector numbers)
- `tcg_rules_ex/`: rules extensions used by the engine
- `scaffold/src/df/`: Dragon Frontiers expansion (`engine.rs` + per-card modules + `runtime.rs` merge facade)
- `scaffold/src/cg/`: Crystal Guardians expansion add-on (`engine.rs`) plus card stubs
- `scaffold/src/hp/`: Holon Phantoms placeholder

Install one expansion per match:

```rust
game.set_hooks(tcg_expansions::create_hooks("DF"));
```

This repo intentionally does not contain:

- hidden evaluation fixtures
- held-out decklists or policy fixtures
- task-specific train data

## Build

```bash
cargo check --package tcg_expansions
cargo test --workspace
```
