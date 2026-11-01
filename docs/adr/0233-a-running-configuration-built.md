---
status: proposed
date: 2026-11-01
depends-on: ADR-0232
title: A running configuration for a settled image, built
kind: built
summary: '`Config::runs_a_settled_image()` is the default with a quarter''s exponent, `ExponentBesideRead` refuses that exponent beside a read, and `Executor::coupling_sum` reads a group''s value. H-46''s arm runs under the name, and every pin held.'
decision-makers: Menteion maintainers
---

# ADR-0233: A running configuration for a settled image, built

**In brief**
- **Decision.** [ADR-0232](0232-a-running-configuration-and-m12-closed.md)'s configuration, boundary and reader are built: `Config::runs_a_settled_image()`, `ConfigError::ExponentBesideRead` and `Executor::coupling_sum`. H-46's arm decodes under the name with its kept addresses unset, and each group's value is read through the reader.
- **Why.** A host needs one name for what H-46 ran beside the groups, and a refusal between a read and an exponent no run has read. One dispatch of every test and seed 22's two first halves held every pin they list on `e130794`.
- **Left open.** The arms' ten other long halves read the name's form unrun, and no public call lays H-25's learning configuration on an image.

## Context and Problem Statement

ADR-0232 specified a name, two refusals and a reader. It left brief 086 the names, the reader's form, how `zero_from` reaches the name and where the second refusal sits.

What was read first (principle 2), on 2026-11-01 at `main` `5d67ecf`:
1. **What a decode keeps of the name.** `Image::decode` calls `Executor::new` with the read, its flag and the probe unset, and sets them from the image. `set_modulation_baseline` and `set_homeostasis` put the image's baseline, gain and step on the executor and leave `Shared`'s `modulation`, `modulation_at_rest` and `gain` at the configuration's until `Executor::tick` publishes all three before the first barrier, so no turn reads the configuration's. The exponent is the one field of the name that reaches a run.
2. **The zero sum's walk.** `zero_sum_round` walks a group's chains and sums its synapses onto the targets, which is the reading ADR-0232 names.
3. **The path at age zero.** The gate holds `keep_address` then `reward_kept(0)` as `address_drawn` then `reward`, as ADR-0232 says, and `drawn_close` and `Task::trial` take the plain path when `Executor::kept_addresses` is zero.
4. **`coupling`** in the harness counts the same synapses as the walk: an excitatory source's onto a readout set, and the two readout sets are apart.

## Decision Drivers

- ADR-0232's decision, and brief 086's standing directives: no record, no section and no format move, and no pinned number moves.
- **A reader the rule cannot disagree with**, since the value it reads is the value a round scales to.
- **No engine built for an image it refuses**: `Executor::new` spawns the workers.
- **Every mutant in the changed lines caught** ([ADR-0030](0030-verification-governance.md)), and each refusal over the lattice.

## Considered Options

1. **The reader's form**: (a) lists of units, refused as a group is refused, its walk `zero_sum_round`'s; (b) `zero_sum_round`'s heads and flags; (c) lists of units, on a walk of its own, reading any unit.
2. **The second refusal**: (a) in `Image::decode`, before `Executor::new`; (b) in the loader's `set_target_read` and `set_step_probe`.

## Decision Outcome

**Options 1(a) and 2(a).** ADR-0232's names stand, and the error is `ExponentBesideRead`, which claims no bound.

- *Why not 1(b)*: heads and flags are the executor's, and a host holds units.
- *Why not 1(c)*: a sum `set_zero_sum` would refuse as a group is no group's value, and a second walk is a second place to disagree.
- *Why not 2(b)*: one rule in two places, reached with an engine already built.

### The configuration (Implemented)

| Item | What it is |
| :--- | :--- |
| `Config::runs_a_settled_image()` | `Config::reads_its_targets()` with `depression_exponent` at `DepressionExponent::Quarter`, and no other field |
| Its comment | the exponent alone is not what was measured; with no groups set an engine runs a state no run has read, and nothing refuses it |
| `Config::default()`, `Config::reads_its_targets()` | unchanged, so every image is born as it was |

The format stays 26, and no state crate changes.

### The boundary (Implemented)

| Refusal | Where | Reads |
| :--- | :--- | :--- |
| `ConfigError::ExponentBesideRead` | `Executor::new`, after the read's own refusals | an exponent other than one beside a read |
| `ImageError::Config(ConfigError::ExponentBesideRead)` | `Image::decode`, once the modulator's record is read and before `Executor::new` | an exponent other than one on an image with a read or a probe under way |

So no engine is born under the name, and no read or probe ends under such an exponent. A lead-in with no read stays legal at any exponent. No image the two dispatches decoded under the name met either refusal: the six short forms' on three networks, and each schedule's of seed 22's two first halves.

### The reader (Implemented)

| Item | What it is |
| :--- | :--- |
| `Executor::coupling_sum(sources, targets)` | the sum of the synapses from `sources` onto `targets`, between ticks; none for a unit outside the arena, a source not excitatory or a source named twice |
| `zero_sum_walk` | the chains' walk, out of `zero_sum_round`, which now scales from its sum; the reader returns that sum |
| `zero_sum_heads`, `zero_sum_marks` | what `set_zero_sum` and the reader check of their units, one code for both |

### H-46's harness under the name

- `zero_from` decodes the arm's form under `Config::runs_a_settled_image()`, by struct update the four sizing fields a decode keeps, `workers`, `nodes_per_worker`, `injector_capacity` and `train_capacity`, and `kept_addresses` left at zero. It asserts the form's exponent, since the name now sets it. The control's form is what it was.
- `zero_groups` reads each value through `coupling_sum`, and its assertion against `made.couplings` is kept.
- The six short forms and the verdict's test are unchanged.

### Its tests

- **The name**: `runs_a_settled_image_is_the_default_with_a_quarter_s_exponent_and_no_other`, field by field against the default.
- **`Executor::new`'s refusal**: `an_exponent_but_one_is_refused_beside_a_read_and_nowhere_else`, every span of `U8_LATTICE` and the edges 15 and 16, beside no period and one, each flag set and unset, at every exponent, against a rule from ADR-0232's text.
- **`Image::decode`'s refusal**: `an_image_with_a_read_or_a_probe_under_way_is_refused_at_any_exponent_but_one`, a read over the lattices' spans and sums with each flag, a probe at every window left over `I32_LATTICE`'s sums, and a settled image loading at each exponent and under the name, every value but the exponent the image's.
- **The reader**: `a_coupling_sum_is_the_sum_by_hand_and_refuses_what_a_group_refuses`, and the sum by hand at the end of each of the twelve walks of `over_a_walk_the_engine_s_zero_sum_is_the_host_s_exact_rule_tick_by_tick`.

### What the dispatches read

- **Both ran on `e130794`**, the round's last commit under `src/` and `tests/`, once its gate was green. No such commit follows them: the commits after it change the documents, the brief and the cost table.
- **`scope=exhaustive`, nothing excluded**: run [41246805299](https://github.com/Menteion/Menteion/actions/runs/41246805299). 18 of 18 shards green; the 227 tests the listing names, each exit 0 with one test reported, none excluded, 97 186 s summed.
- **`scope=arms`, seed 22's two first halves**: run [41246807640](https://github.com/Menteion/Menteion/actions/runs/41246807640), on AArch64. Each job green, 104.7 and 102.6 minutes, 43.6 % and 42.8 % of their bound of 240. Each test held its eight pinned rows, and each row of `arm.log` is in `pins/h46.rs` verbatim. Each eighth image's CRC-64, `0xedcd0f2e2d3fe331` and `0x07dab2259bdba1f9`, is the one its first half printed, so the second halves' input has not moved.

| Shard | Job | Of its bound | Tests | Their seconds summed | Tests' wall time |
| :--- | ---: | ---: | ---: | ---: | ---: |
| 0 | 42.8 min | 17.8 % | 11 | 4 926 s | 2 494 s |
| 1 | 39.0 min | 16.3 % | 11 | 4 474 s | 2 253 s |
| 2 | 41.2 min | 17.2 % | 11 | 4 757 s | 2 383 s |
| 3 | 34.5 min | 14.4 % | 12 | 3 866 s | 1 986 s |
| 4 | 36.6 min | 15.3 % | 12 | 4 216 s | 2 119 s |
| 5 | 34.0 min | 14.2 % | 10 | 3 909 s | 1 959 s |
| 6 | 57.1 min | 23.8 % | 12 | 6 259 s | 3 330 s |
| 7 | 54.8 min | 22.8 % | 13 | 6 236 s | 3 181 s |
| 8 | 47.8 min | 19.9 % | 13 | 5 566 s | 2 793 s |
| 9 | 59.9 min | 25.0 % | 13 | 6 551 s | 3 502 s |
| 10 | 36.9 min | 15.4 % | 14 | 4 276 s | 2 149 s |
| 11 | 40.5 min | 16.9 % | 13 | 4 700 s | 2 359 s |
| 12 | 56.9 min | 23.7 % | 13 | 6 387 s | 3 318 s |
| 13 | 61.4 min | 25.6 % | 13 | 7 156 s | 3 585 s |
| 14 | 45.5 min | 19.0 % | 13 | 5 284 s | 2 643 s |
| 15 | 54.7 min | 22.8 % | 13 | 6 325 s | 3 182 s |
| 16 | 53.8 min | 22.4 % | 15 | 6 088 s | 3 128 s |
| 17 | 54.3 min | 22.6 % | 15 | 6 210 s | 3 170 s |

- The bound is 240 minutes a job ([ADR-0179](0179-the-bounds-rule-restated.md)). **No job passed 60 % of it**: the longest, shard 13's, took 61.4 minutes, 25.6 %.
- Fourteen shards started at 22:08Z, and shards 7, 8, 13 and 14 waited for runners until 22:30Z to 22:38Z. The last ended at 23:33:17Z, 1 h 25 min after the dispatch.
- `scripts/exhaustive-costs.tsv` is regenerated from its artifacts with `--keep`. Its 227 lines are the shards' 227 tests, and its six new lines are H-46's short forms, which had none.

**The name's form beyond those runs.** The arms' ten other long halves read the name's form unrun: seed 22's two second halves, and the four halves each of seeds 23 and 24. The twelve control halves keep the form they ran. Their rows rest on the path's equality, which `a_reward_kept_at_age_zero_is_the_drawn_reward_bit_for_bit` and `on_time_under_kept_addresses_a_trial_is_the_trial_it_was_bit_for_bit` hold at the gate, and which the two jobs and the six short forms read under both rules.

### The sweep

The schedule's sweep of 2026-11-04 at 20:17Z had not run when this was written, on 2026-11-01. The first ADR that merges after it ends records it. [ADR-0227](0227-the-pinned-tables-in-a-file-a-round.md) recorded the one before it.

## Consequences

- Good: a host has a name for a settled image, a reader for each group's value, and a refusal between either and a read.
- Good: the reader is the rule's own sum, so a value read is the value a round holds an unmoved group at.
- Bad: every comment that warns of the name without its groups is a description, as ADR-0232 said it would be.
- Bad: `set_zero_sum` and the reader allocate a unit's flags each call, between ticks; a host calls them at a load.
- Neutral: `zero_sum_round`'s walk moved into a function of its own, and the hot path is unchanged in what it does.

## Confirmation

- `cargo test --workspace --locked` runs the tests above in the gate.
- The pull request's mutation gate on `e130794`, run 41245334737, made 29 mutants in the changed lines and caught all 25 that built; 4 were unviable.
- The two dispatches above hold the pins, and `scripts/exhaustive-costs.tsv` is regenerated from the first.
- `npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
