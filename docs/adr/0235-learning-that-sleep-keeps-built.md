---
status: accepted
date: 2026-11-02
depends-on: ADR-0234
title: Learning that sleep keeps, built
kind: built
summary: 'The slow-wave baseline is a configuration''s, the ledger is spent on a call, and a trial presents the units a host gives, each unset the engine as it is. The documents give the replay''s drive as it arrives, and F-47 is resolved.'
decision-makers: Menteion maintainers
---

# ADR-0235: Learning that sleep keeps, built

**In brief**
- **Decision.** [ADR-0234](0234-learning-that-sleep-keeps.md)'s three parts are built: `Config::slow_wave_baseline_q16`, `Executor::spend_episodes` and `Task::trial_presenting`. Unset, or given each set's own units, the engine and the trial are what they were, bit for bit, over the lattice. The documents outside the state crates give the replay's drive as it arrives, 4.375 at a gain of 1.75, and F-47 is resolved.
- **Why.** H-47 needs a night whose replays reach excitatory synapses, a night that replays only what it tags, and a pair that shares half of A's units. Each part enters unset, so no pinned number moves.
- **Left open.** The comment on `RIPPLE_SHIFT` in `menteion-hippocampus`, a state crate's, outside brief 087's scope. Whether the slow-wave baseline enters the running configuration is ADR-0234's next decision.

## Context and Problem Statement

ADR-0234 specified three parts and left brief 087 their names, whether a trial's units are a call's argument or a field of the stimulus, and how the explorations' forms are built.

What was read first (principle 2), on 2026-11-02 at `main` `754c966`:
1. **The modulation.** `Executor::tick` builds `Modulations::of(&self.modulator, self.modulation_baseline_q16, …)` before the barrier, and the stage is `homeostasis.sleep_stage`. The baseline reaches every excitatory synapse, and every inhibitory one while the inhibitory baseline is unset.
2. **The ledger.** `Episode::is_spent` is a tag of zero, where REM's `depotentiate` leaves an episode at the end. A decoded engine holds the image's episodes and the configuration's room beside them (`Image::decode`), so an image a night wrote carries its episodes into the next night.
3. **A stimulus.** It sends its messages and its cancel through `send` over its set. The addressed delivery's sources are its set's units, and `Task::check` reads the sets alone.
4. **The explorations' forms**, on the tags `explore/014-what-a-night-keeps` and `explore/016-what-a-shared-source-takes`: an `Option<i32>` read in `tick` and refused nowhere; a loop setting every tag to zero; and `Stimulus::inject_into`, `cancel_into` and `Task::trial_presenting` taking `Option<&[Vec<u32>; 2]>`, with no refusal and the addressed delivery's sources left the set's.

## Decision Drivers

- ADR-0234's decision and brief 087's directives: no record, section, format or named configuration moves, and no pinned number moves.
- **Principle 5**: each part unset is the engine as it is, held by property tests over the lattice.
- **Fail closed**: a value or a unit the rule cannot read is refused before it reaches the engine.
- **Every mutant in the changed lines caught** ([ADR-0030](0030-verification-governance.md)).

## Considered Options

1. **The units a trial presents**: (a) an argument of a `Task` call, `[&[u32]; 2]`; (b) a field of the stimulus; (c) the tag's `Option<&[Vec<u32>; 2]>`.
2. **The units given**: (a) refused as `check` refuses a set, before anything is injected; (b) taken as given.
3. **The addressed delivery under units given**: (a) they are its sources; (b) the set's stay its sources.
4. **A slow-wave baseline outside $[0, 1]$**: (a) refused, as the inhibitory baseline is; (b) clamped where the modulations are built.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).** ADR-0234's names stand.

- *Why not 1(b)*: borrowed units in a field would give `Stimulus` and `Task` a lifetime and take their `Copy` away, and the units are the host's for a run.
- *Why not 1(c)*: a `Vec` in the signature asks every caller for an allocation the trial never needs, and `None` is `trial`.
- *Why not 2(b)*: the injector refuses a unit outside the arena only after part of the stimulus is sent, and a unit in a readout would drive the count it is read by.
- *Why not 3(b)*: the address would name units the trial did not present.
- *Why not 4(b)*: a configured value the engine changes silently is a value no run read.

### The slow-wave baseline (Implemented)

| Item | What it is |
| :--- | :--- |
| `Config::slow_wave_baseline_q16: Option<i32>` | unset in `Config::default()`, `Config::reads_its_targets()` and `Config::runs_a_settled_image()` |
| `tick_baseline` | in slow-wave sleep while set, the slow-wave baseline; in every other stage and while unset, the engine's. A pattern on the stage, so no comparison is mutated |
| `Executor::tick` | builds its modulations under `tick_baseline`, so the slow-wave baseline reaches what the baseline reaches |
| `ConfigError::SlowWaveBaselineOutOfRange` | `Executor::new`, and so `Image::decode`, refuse one set outside $[0, 1]$ |
| `Executor::slow_wave_baseline_q16()` | the configuration's; never the image's, which does not hold it |

### The ledger spent (Implemented)

`Executor::spend_episodes(&mut self)`, between ticks: every episode of the ledger's length takes a tag of zero. Its pattern, its tick, its count, the ledger's length, the hippocampal record and the room are unchanged, and a ledger spent already or empty is unchanged.

### A trial's own units (Implemented)

| Item | What it is |
| :--- | :--- |
| `Task::trial_presenting(exec, trial, presenting: [&[u32]; 2])` | the trial, its stimulus's messages and its cancel going to the units given for the stimulus drawn, in their order |
| Its refusals, before anything is injected | `TaskError::EmptySet` for no unit given; `SetOutsideArena` for a unit at or past the arena's end; `SetsOverlap` for a unit in a readout's set or given for both stimuli |
| `Task::trial` | `trial_to` with no units given: the set's path, call for call |
| The addressed delivery | its sources are the units presented, the set's or the host's (`addressed`) |
| `Stimulus::inject`, `cancel_at` | unchanged, through `inject_to` and `cancel_to` on the set |

### F-47's documents

| Where | Now |
| :--- | :--- |
| `REPLAY_DRIVE_Q16`'s comment, `executor.rs` | 2.5 times the gain: at 1.0 the band ADR-0038 read, at 1.75 4.375, fired twice at 5 and 206 ticks |
| `task.rs`'s module comment | the stimulus's messages scaled by the gain as a replay's are, 4.375 at 1.75 |
| `CUE_Q16` in `task.rs`'s tests | once at the gain of 1.0 those tests run at |
| `STIMULUS_Q16` in `tests/learning.rs` | at that file's gain of 2.0 they arrive as 5.0; found beside Context 2's list |
| Whitepaper §5.2.15 | the executor's replay as it arrives, its paragraph split under the limits of `npm run spec:shape` |
| Whitepaper §6.6 step 2 and §8.8's row | 2.5 times the gain, 4.375 at 1.75 |
| F-47's row | resolved, naming the `RIPPLE_SHIFT` comment as left |

**Left**: the comment on `RIPPLE_SHIFT` in `crates/menteion-hippocampus/src/lib.rs` still reads one replay as one spike of each unit, which holds at a gain near 1.0. It is a state crate's, protected by brief 087.

### Its tests

- **The slow-wave baseline**: `a_tick_s_baseline_is_the_slow_wave_baseline_in_slow_wave_sleep_alone`, over `I32_LATTICE` squared by every stage and `U8_LATTICE`, and 4 000 seeded cases. `the_slow_wave_baseline_is_unset_by_default_and_in_no_named_configuration_and_refused_outside_the_unit_interval`, both edges and the lattice, by `new` and the loader, and an image the same bytes set or unset. In `tests/sleep.rs`, `a_slow_wave_tick_consolidates_under_the_slow_wave_baseline_and_every_other_tick_under_the_engine_s`: in each stage over the lattice's values in $[0, 1]$ and three seeded ones, an engine that sets it is bit for bit the engine whose own baseline is it in slow-wave sleep and zero elsewhere.
- **The ledger spent**: `spending_the_ledger_leaves_every_episode_spent_and_nothing_else_moved`, over 200 seeded ledgers with priorities from `U8_LATTICE`. In `tests/sleep.rs`, `a_night_after_the_ledger_is_spent_replays_what_is_tagged_after_and_nothing_before`, at every priority of the lattice, against the episode in a ledger of its own.
- **A trial's own units**: `a_trial_presented_to_each_stimulus_s_own_units_is_the_trial_it_was`, over 64 seeded tasks with the trial's index from `U32_LATTICE`; `a_trial_presented_to_other_units_is_the_trial_of_sets_of_those_units`, under every delivery that needs no critic; and `units_given_are_refused_where_a_set_would_be_before_anything_is_injected`, each refusal at its edge.
- `tests/contention.rs` and `tests/differential.rs` spell every field, so each names the new one unset.

### What the dispatches read

- **The calibration ran on `5e5946d`**, the protocol's commit, which carries every line of this ADR's code. Dispatch [41441229981](https://github.com/Menteion/Menteion/actions/runs/41441229981) at `scope=exhaustive`, the twelve short forms of H-47 excluded: 18 of 18 shards green, the 227 tests the listing names each exit 0 with one test reported, 104 599 s summed. Every pin they hold held, with the three parts unset.
- **The longest shard** took 67 minutes, 28 % of its bound of 240 ([ADR-0179](0179-the-bounds-rule-restated.md)); the shortest 39. No job passed 60 %.
- **The twin** in dispatch [41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613) ran these parts as H-47 sets them, on seed 22, and held every line it reads to explorations 018's and 019's logs ([ADR-0236](0236-learning-that-sleep-keeps-its-protocol.md)).
- **The mutation gate** on `978391f`, run 41436922503, and on `5e5946d`, run 41441219858, made 41 mutants in the changed lines each time and caught all 29 that built; 12 were unviable.

### The sweep

The schedule's sweep of 2026-11-04 at 20:17Z had not run when this was written, on 2026-11-02.

## Consequences

- Good: a host can hold a night's replays to what it tags, reach the excitatory synapses in slow-wave sleep, and present a stimulus no periodic set names.
- Good: the documents give the replay's drive as it arrives.
- Bad: while set, the modulation's baseline depends on the stage, outside the image: an engine decoded from an image takes it from its configuration, as ADR-0234 said.
- Bad: `trial_presenting` checks the units given at every trial, a pass over them and the readouts' sets.
- Neutral: `trial` runs through `trial_to` with the set's path unchanged.

## Confirmation

- `cargo test --workspace --locked` runs the tests above in the gate.
- The pull request's mutation gate reads every mutant in the changed lines.
- `npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
