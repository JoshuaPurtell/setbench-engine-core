# EX expansion folders and held-out-set tasks

Date: 2026-09-12
Status: **cutover proven in worktrees** (not merged). No paid Luna run.

Worktrees:

- Engine: `/Users/joshuapurtell/GitHub/worktrees/expansion-folders-engine` (branch `expansion-folders`)
- CardCodeBench: `/Users/joshuapurtell/GitHub/worktrees/expansion-folders-cardcodebench` (branch `expansion-folders`)

## Implementation status (2026-09-13)

Done in the engine worktree:

- `tcg_core` no longer has `cg_engine` or DF-87/89/1 collector-number leaks.
- New hooks: `energy_units`, `retreat_cost_override`, `treats_pokemon_as_delta`, `prevents_special_conditions`.
- `create_hooks(set)` installs one expansion. Tests: DF Holon Veil applies; CG hooks do not.
- DF `engine.rs` (Boost/Scramble units, Holon Veil, Holon Energy WP/GL).
- Dewgong δ and Heracross δ bodies live in per-card modules. `runtime.rs` remains the merge facade.
- CG `engine.rs` holds DRE / Memory Berry / Safeguard / Holon Circle.
- Public `scaffold/src/cg/cards/` has 21 remaining-PvP **stubs** (not gold). `create_hooks("CG")` merges engine + those fragments.

Done in the CardCodeBench worktree:

- Remaining-PvP vendor unhooked the same way.
- Canonical remaining-PvP gold stays in the private CardCodeBench overlay and is never copied into this public repository.
- Remaining overlay still compiles `src/cards.rs` for Harbor, and also writes `expansions/cg/cards/<task>.rs`.
- Set grader: `src/cardcodebench/set_grade.py` + `tools/grade_set.py`. One submission → 21 remaining-PvP packs, `--workers >= 8`, `--host` cargo-runs without Docker/Luna. Headline `cards_correct / 21`.
- Set-task stubs + instruction: `tasks/pokemon/crystal_guardians/set-pvp/`.
- Host gold/stub gates (`tools/grade_set.py --host --workers 8`): reference **21/21**, stub **0/21**. Aggron `split-bomb-bench-p2-s5` needed Charmander δ Scratch/Bite as printed damage + `NoOp` (catalog `DealDamage` was double-applying and KO'ing Lairon before Split Bomb). Do not bulk-rewrite other packs' support catalogs; that broke Swampert Energy Recycle.

Not done (out of this cutover):

- Rest of DF `runtime.rs` split into per-card modules.
- Paid Luna set episode.

This document does not authorize a paid Luna episode.

This continues [cardbench/docs/PROPOSAL_CARDCODEBENCH_2026-09-07.md](/Users/joshuapurtell/GitHub/cardbench/docs/PROPOSAL_CARDCODEBENCH_2026-09-07.md) tracks `single_card`, `card_family`, and `mechanic_cards`. It names the missing layout those tracks need: a core engine, then one folder per expansion (engine add-on + cards), then a task that holds out one whole folder while showing the others.

## 1. Decision

Keep three layers. Do not invent a fourth crate or a second hook system.

| Layer | Owns | Must not contain |
| --- | --- | --- |
| Core (`tcg_core` + `tcg_rules_ex`) | Turn loop, zones, combat, `EffectAst`, prompts, prizes, era-wide rules | `CG-95`, `DF-87`, Crystal Shard, Dual Armor |
| Expansion add-on | Set-wide rules for that set only | Per-Pokémon attack bodies |
| Cards | Printed attacks + that card’s power/body | DRE discard policy, stadium rules, Boost Energy units |

Remaining-PvP gold already is the card layer (`attacks()` + `hooks()`). `remaining_engine_overlay` and CardBench `cg_engine.rs` are already expansion add-ons, unnamed and patched in. The work is to put those files in the folders the engine was designed for, then grade a whole folder.

CardBench Crystal Guardians `set_engine` already tells the agent that reference sets are in the workspace and CG files are stubs. That is the right *task*, on the wrong *files*: two monoliths (`all_cards.rs` + `tcg_core/src/cg_engine.rs`), and `create_hooks()` always merges CG+DF+HP.

## 2. Why this, not more remaining-PvP only

Single-card remaining-PvP is the right unit for authoring and for RL on one card. It does not measure “can you implement an EX expansion from card text, given working examples of the era.”

The held-out-set task is that measurement:

- Agent reads complete gold for N−1 expansions.
- Agent writes the held-out expansion folder.
- Grader installs **only** that set’s hooks and runs remaining-PvP packs for that set.

Shown-others is few-shot on the *pattern* (Dewgong δ Delta Protection → Camerupt Delta Protection). Set-specific cards (Crystal Beach, Dual Armor, Dual Crystal Type) are not in the sibling folder. That is the hard part.

## 3. Current seams (do not replace these)

**Hook vtable.** `tcg_core::runtime_hooks::RuntimeHooks` is fn pointers. `AttackOverrides::merge` already composes bodies. `execute_power` returns bool (first true wins). Remaining-PvP already does `game.set_hooks(cards::hooks())`.

**Leak into core** (must move before the folders are honest):

| Call site | File | What it names |
| --- | --- | --- |
| `cg_engine::energy_units` | `tcg_core/src/game.rs` | DRE unit count |
| Boost Energy / Scramble Energy | `game.rs` | `DF-87`, `DF-89` |
| `cg_engine::discard_double_rainbow_if_basic` | `game.rs` | DRE |
| `cg_engine::prevents_attack_effects` | `game.rs` | Safeguard-style |
| `cg_engine::tool_discard_timing_override` | `game.rs`, `action.rs` | Memory Berry |
| Holon Veil | `game.rs` `player_has_holon_veil` | `DF-1` Ampharos |

Several of those already have `RuntimeHooks` fields (`is_double_rainbow`, `prevents_attack_effects`, `tool_discard_timing_override`). Core just does not call them. Energy *units* (DRE provides 2) may need a new hook; `energy_provides_override` only covers types.

**Remaining-PvP overlay** (`src/cardcodebench/remaining.py` `remaining_engine_overlay`) is generic-by-contract (fails if `CG-13`, Crystal Shard, Dual Armor, … appear). It still string-patches the **task** vendor copy. Long-term those patches belong in `setbench-engine-core`, not in the overlay. Overlay must stay free of set names.

**DF “per-card” files are identifiers.** `scaffold/src/df/cards/df_015_dewgong.rs` has SET/NUMBER/NAME. Delta Protection still lives as `def_id_matches(..., "DF", 15)` in `df/runtime.rs`. Splitting that runtime is stage 3, not stage 0.

**HP `set_engine` is blocked.** CardBench `holon_phantoms/family.toml`: no real gold. Do not stub a held-out HP set until gold exists.

**CG remaining-PvP coverage (2026-09-12):** 21 cards are 60-card two-player remaining-PvP. 27 leftover Pokémon are still board-setup. 52 cards live in older frozen groups. Whole-set gold is not 100/100 remaining-PvP.

**Exposure.** CardBench already shipped CG gold (`all_cards.rs`, `cg_engine.rs`) and a public set_engine instruction. Remaining-PvP golds in `.private/` are local, not a clean unpublished held-out set. CG can prototype the *format*. It cannot be relabeled a private unseen full-set test.

## 4. First tree (stage 0)

Do not add 16 EX folders. One set, one real card, `create_hooks("CG")`.

```text
setbench-engine-core/
  tcg_core/                         # no mod cg_engine; no DF collector numbers
  tcg_rules_ex/
  expansions/                       # today’s tcg_expansions / scaffold/
    src/lib.rs
    src/registry.rs                  # create_hooks(set: &str) — never CG+DF+HP
    cg/
      mod.rs
      engine.rs                     # DRE, tool discard, stadium — set-wide
      hooks.rs                      # merge engine + card fragments
      catalog.json                  # printed HP / W/R / retreat; attacks empty
      cards/
        mod.rs
        cg_095_kyogre_ex.rs         # only real card in stage 0
    df/                             # leave in place in the first PR
    hp/                             # placeholder until gold exists
```

Kyogre gold moves as-is from `.private/remaining/cg-095-kyogre-ex/reference/src/cards.rs`:

```rust
// expansions/cg/cards/cg_095_kyogre_ex.rs
pub const DEF: &str = "CG-95";
pub fn attacks() -> Vec<Attack> { /* Hydro Shot AST */ }
pub fn retreat_cost_override(game, id, base) -> i32 { /* Flotation */ }
```

`expansions/cg/hooks.rs` is the merge site. A CG match does `game.set_hooks(expansions::create_hooks("CG"))`. DF is not in that vtable.

CardCodeBench remaining-PvP for Kyogre still grades **one** allowed file. The path becomes `expansions/cg/cards/cg_095_kyogre_ex.rs` (or a thin `src/cards.rs` re-export during migration). Driver, catalog slice, and overlay stay harness.

**Stage 0 gate:** existing Kyogre remaining-PvP gold=1 / stub=0 on the current seed pack. If that pack regresses, stop. Do not continue the folder split on a broken grade.

## 5. Card contract

Every card module, every set, same shape. Copy remaining-PvP gold; do not invent a trait object.

```rust
pub const DEF: &str = "CG-3";
pub fn attacks() -> Vec<Attack>;

// Optional fragments; default no-ops. hooks.rs merges.
pub fn attack_overrides(...) -> AttackOverrides { Default::default() }
pub fn retreat_cost_override(...) -> i32 { base }
pub fn execute_power(...) -> bool { false }
pub fn card_has_power_or_body(id: &CardDefId) -> bool { id.as_str() == DEF }
```

Catalog (`catalog.json`) keeps printed identity: name, HP, types, W/R, retreat, stage, δ/ex/★. Attacks and bodies live in the card file so the agent has one place to edit.

Loader writes `cards::attacks()` onto that def id at match start (today: `catalog_with_attacks(hero, cards::attacks())` for one hero). A set match walks every module.

**Where a rule belongs**

1. Core / `tcg_rules_ex`: Active-only W/R, setup coin, evolve-from-hand, generic attack-name lock (today remaining overlay).
2. Set `engine.rs`: DRE, Crystal Beach, Holon Circle, Memory Berry, δ identification if it is set policy.
3. Card module: Delta Protection, Fluffy Fur, Dual Armor, Echo Draw, Flotation.

If two cards need the same hook, `hooks.rs` merges. Do not special-case a card name in `tcg_core`.

## 6. Registry

Today:

```rust
pub fn create_hooks() -> RuntimeHooks  // always CG + DF + HP
```

Replace with:

```rust
pub fn create_hooks(set: &str) -> RuntimeHooks
```

- `"CG"` → `cg::engine` + `cg::cards::*` only.
- `"DF"` → DF only.
- Unknown set → fail closed, empty hooks are not a silent fallback for a named set.

A CG remaining-PvP pack must not run Feraligatr Battle Aura. Shown DF gold is **readable** in a held-out-set workspace; it is not installed into the CG `GameState`.

## 7. Remaining-PvP and the set task share gold

Do not maintain two Kyogre implementations.

| Surface | Allowed files | Frozen | Grade |
| --- | --- | --- | --- |
| Single card (today) | one `expansions/cg/cards/cg_NNN.rs` | core + `cg/engine.rs` + other card stubs | that card’s remaining-PvP pack |
| Whole set, in-expansion | `expansions/cg/**` | core only; no sibling sets | union of remaining-PvP packs |
| Held-out set (goal) | `expansions/cg/**` | core + sibling expansion gold | same union; DF/HP not in vtable |

Headline SCORE stays binary AND across required packs (or a versioned partial policy that is never the headline). Report a per-card vector as diagnostics.

Mixed-hero cells (Crystal Shard on Kingler, Stadium vs Luvdisc) need the **full** catalog and the relevant trainer/stadium in `engine.rs` or those card modules. Per-task catalog slices stay for single-card remaining-PvP.

Do not treat CardBench `train_scenarios.json` substring markers as remaining-PvP-quality checks.

## 8. Stages

| Stage | Work | Done when |
| --- | --- | --- |
| 0 | Core call sites → hooks; delete `mod cg_engine` from `tcg_core`; land remaining overlay patches that are era-wide; Kyogre module under `expansions/cg/cards/` | Kyogre pack gold=1 / stub=0; overlay still bans named CG cards |
| 1 | Move the other 20 remaining-PvP golds into `cg/cards/` | each pack still passes; one remaining-PvP driver can `create_hooks("CG")` with only that card non-stub |
| 2 | DRE + stadiums/tools that are set-wide into `cg/engine.rs`; convert leftover Pokémon only as remaining-PvP, not board fixtures | mixed-hero packs that need Beach/Shard have gold |
| 3 | Split `df/runtime.rs` into `df/cards/` with the same contract. Dewgong δ is the copy-from example for Camerupt | DF still grades; per-card files own Delta Protection |
| 4 | HP only after real gold exists | HP `set_engine` unblocked |
| 5 | Next English EX folders (`pk/`, `ds/`, …) one at a time | each folder has `engine.rs` + card modules + remaining-PvP or equivalent packs |
| 6 | Held-out-set Harbor task: stub one folder, show the others | reference=1 / stub=0 on the union; overlap group recorded |

English EX (RS → PK, plus POP/etc.) is a backlog of folders with this contract, not a rewrite. Earlier sets (Team Magma, Rocket, Holon Energy) will force new primitives. Era-wide primitives go in core. Set-only rules go in that `engine.rs`.

**Do not** convert the remaining 27 leftover Pokémon as a prerequisite for stage 0. Stage 0 is Kyogre + unhooked core.

**Do not** run serial paid contrast or a whole-set agent episode from this document.

## 9. Held-out-set workspace (stage 6)

```text
/app/
  tcg_core/                         # frozen
  tcg_rules_ex/
  expansions/
    src/registry.rs                 # frozen; create_hooks(set)
    df/                             # FULL GOLD, readable
      engine.rs
      cards/df_015_dewgong.rs
      ...
    hp/                             # omit until gold exists
    cg/                             # HELD OUT
      engine.rs                     # RuntimeHooks::empty() + TODO
      hooks.rs                      # wires modules; fragments empty
      catalog.json                  # identity only
      cards/cg_003_camerupt.rs       # empty attacks() / hooks
      cards/cg_095_kyogre_ex.rs
      ...
  data/cg/card_text_bundle.json
```

Allowed writes: `expansions/cg/**` only.

Grade: `create_hooks("CG")` only, then every remaining-PvP pack that has gold for that set.

**Shown**

- Core and EX-era rules
- Sibling expansion gold (pattern)
- Held-out catalog + card text
- Optional public train packs (0pct/30pct style), never the private traces

**Hidden**

- Held-out gold `engine.rs` and card modules
- Remaining-PvP expected traces / seeds

## 10. Splits and contamination

Same overlap-graph rule as the 2026-09-07 proposal.

- Single-card `cg-095` remaining-PvP and “implement all of CG” are **one split group**. Training on Kyogre burns CG-as-heldout.
- Published CardBench CG gold cannot be a private unseen set. Use CG as the format prototype (DF shown, CG stubbed) and treat those scores as engineering.
- First **clean** full-set heldout is a set never published as gold: HP after private gold, or PK/DS never shown.
- Rotate later: same snapshot, three tasks (`holdout=CG|DF|HP`). Report separately. Do not average.
- Dewgong in the shown DF folder does **not** put Camerupt in train. It does put “δ damage reduction by 40” in the few-shot. Do not claim unseen-mechanic transfer for Delta Protection on a CG-holdout task that showed DF Dewgong.
- Reprints and identical bodies across sets are grouping edges.

Training and screening may use only train tasks. Do not freeze a transfer split in this draft.

## 11. What this document does not do

- Authorize a paid Harbor / Tinker / Luna job.
- Relabel remaining-PvP or CardBench CG gold as a fresh held-out set.
- Require implementing all 100 CG cards before stage 0.
- Load DF/HP into a CG match “for convenience.”
- Treat CardBench train event-log substring checks as remaining-PvP.
- Invent fixture worlds if gold/GameBench is down.
- Change remaining-PvP’s public-crate-is-grade-crate contract until a dedicated migration PR says so.

## 12. First PR (when implementation is authorized)

Scope is only stage 0. Fail closed rather than a half-split.

1. Route `energy_units`, DRE discard, prevent-effects, tool timing, Holon Veil, Boost/Scramble through `RuntimeHooks` (add `energy_units` if needed).
2. Remove `mod cg_engine` from `tcg_core`. Put DRE in `expansions/cg/engine.rs`.
3. Land remaining overlay patches that are era-wide into `setbench-engine-core` (Active-only W/R, retreat-cost hook, generic attack-name lock, ChoosePokemonTargets, catalog tool types). Overlay shrinks; still bans named CG cards.
4. Add `create_hooks("CG")`. Stop merging DF/HP by default.
5. Move Kyogre gold to `expansions/cg/cards/cg_095_kyogre_ex.rs`. Remaining-PvP driver installs CG hooks, not `cards::hooks()` as a one-off vtable, unless a re-export is required for the allowlist.
6. Re-run Kyogre remaining-PvP reference/stub. If either gate fails, revert.

Out of first PR: leftover-20 remaining-PvP authoring, DF runtime split, HP gold, whole-set Harbor task, any paid panel.

## 13. Open questions

- **Allowlist during migration.** Keep `src/cards.rs` as a re-export of the Kyogre module until Harbor instruction/lock hashes update, or cut over in the same PR.
- **`energy_units` vs `energy_provides_override`.** DRE is two colorless units on a Basic, discarded if attached to a Basic at end of turn. Confirm one hook vs two.
- **Catalog ownership.** Full CG `catalog.json` in the expansion folder vs today’s per-task PvP slices. Set task needs the union; single-card can keep slices generated from the full catalog.
- **Which set is the first advertised held-out-set task.** Engineering prototype: CG with DF shown. Authority: not CG.
- **Partial credit.** Headline remains AND. A per-card vector is diagnostics / a separately versioned train reward, never the published set score.
- **Trainer/stadium modules.** Same `cards/` folder vs `engine.rs`. Prefer a module per printed card (`cg_075_crystal_beach.rs`) so the set task allowlist is the folder, not a special engine file plus 90 Pokémon.

## 14. Related records

- Remaining-PvP coverage: `CG_PVP_COVERAGE_2026-09-12.md`
- Remaining overlay: `src/cardcodebench/remaining.py` (`remaining_engine_overlay`, `public_overlay`)
- Public engine pin: `/Users/joshuapurtell/GitHub/setbench-engine-core`
- CardBench CG set_engine: `cardbench/varieties/pokemon/sets/crystal_guardians/family.toml` (grades `all_cards.rs` + `tcg_core/src/cg_engine.rs`)
- CardBench DF: grades `import_specs.rs` + `runtime.rs`, not the per-card modules
- CardBench HP: `readiness = blocked`
