---
status: proposed
date: 2026-11-14
depends-on: ADR-0255
title: A search by forks, built
kind: built
summary: ADR-0255's search and the writer of the four couplings are built as a module of the inhibition tests, `fork_search.rs`, held by eight fast tests. It measures nothing, and `round:additive` exits 0.
decision-makers: Menteion maintainers
---

# ADR-0256: A search by forks, built

**In brief**
- **Decision.** [ADR-0255](0255-a-search-by-forks-and-the-two-alternative-tasks-references.md)'s search and its writer are built in `runtime/menteion-runtime/tests/instrument/fork_search.rs`, which `tests/inhibition.rs` includes by path in two lines. Eight fast tests that run with the workspace's hold them.
- **Why.** Exploration 039 reads the two-alternative task's ceiling with the search, and order 3 takes it for H-47's task. A tool that decides nothing still enters `main` by a round whose gate is its evidence.
- **Left open.** How often a seed reaches an inner best, at the tests' sizes or at exploration 039's; a cache of reads, which an objective keeps itself; any read of a task's references.

## Context and Problem Statement

ADR-0255 specified what the search takes, does and prints, the writer's rounding, and four tests. It left brief 088 the module's names, how an objective is passed, where the tests live, and the synthetic objectives and small engine of the tests.

What was read first (principle 2), on 2026-11-14 at `main` `c4d1548`:
1. **A module of one binary.** `tests/inhibition/pins/h46.rs` reaches the harness and the root's items through `use crate::*;`. A module declared after the root's `eprintln!` macro prints through it, so a twin listening would keep its lines.
2. **What `coupling` counts.** It sums the synapses an excitatory unit of `from` sends onto a unit of `into`. The four sets are apart, so a synapse is in one coupling at most, and `fan_out` names its block and slot.
3. **A unit is a control record.** `DendriticSuperNeuron` holds atomics and derives neither `Clone` nor `PartialEq`, so a test holds the units unmoved by the encoded image, not by comparing them.
4. **The mutation gate makes no mutant here.** `cargo mutants --workspace --list --in-diff` on this change lists none: cargo-mutants 27.1.0 does not mutate a test target. The tests below are the tool's only check, written to fail on each rule.
5. **A step of 16 in a small box.** In 0..=8 a step of M = 16 is held at an edge most times it moves, so an inner best comes more from the draws and the crossover than from the steps. A first draft's four genes in 0..=31 reached neither its corner nor its inner point from seed 0.

## Decision Drivers

- ADR-0255's decision, and brief 088's standing directives: nothing under `src/` or `crates/` changes, no test that ran before changes, nothing is measured, pinned or dispatched.
- **Determinism**: a search repeats bit for bit from its seed.
- **Only adding** ([ADR-0215](0215-the-calibration-a-change-needs.md)): every line a Rust build reads is a new file or whole items inserted under names the tree does not hold.

## Considered Options

1. **The objective**: (a) `&mut dyn FnMut(&[i32], u32) -> i64`; (b) a trait the caller implements; (c) a generic parameter.
2. **The tests' place**: (a) in `fork_search.rs`; (b) in `tests/inhibition.rs`.
3. **The printed table in a test**: (a) the lines returned as printed; (b) caught through the binary's `LISTENING`.

## Decision Outcome

**Options 1(a), 2(a) and 3(a).**

- *Why not 1(b) or 1(c)*: a closure keeps what a caller needs, a fork's image or a cache of reads by genes and stream, and one function serves every caller.
- *Why not 2(b)*: each test would be a top-level item of the root, whose name `round:additive` must clear. In the module the root gains two lines.
- *Why not 3(b)*: `LISTENING` is one static for the binary, and the tests run in parallel.

### The search (Implemented)

<!-- @assert-count target="runtime/menteion-runtime/tests/instrument/fork_search.rs" symbol="fn search_by_forks" min="1" reason="ADR-0256: the search by forks, a module of the inhibition tests" -->

| Item | What it is |
| :--- | :--- |
| `Gene` | `name`, inclusive `min` and `max`, and `start`, the value generation 0 starts from |
| `Breeding` | `population` P, `generations` G, `kept` E, `step` M, `streams` S and `seed` |
| `search_by_forks(genes, breeding, label, objective)` | ADR-0255's algorithm, every draw `Lcg`'s from the seed; returns `Searched` |
| `Scored` | a candidate as scored: its generation, its index there, its genes and its score on each stream |
| `Searched` | every `Scored` in the order scored, the best of the last generation, its scores on streams S to 2S − 1, and the lines printed |

- **Its draws, in order**: generation 0's P − 1 candidates gene by gene, each uniform within its bounds. Each child then draws its two tournaments, each two places in the ranked generation, the lower taken. Then, gene by gene, a coin for its parent, a one in four, and on that one a step uniform in −M..=M.
- **Its rank**: by the sum of a candidate's S scores, the larger first, by a stable sort of the generation in its order, the kept in their rank and then the children as made.
- **What it refuses**: P below 2, E not below P, G or S at 0, M below 0, a start outside its bounds, and bounds wider than a `u32` draw.
- **What it prints**, each line as made: a head, `DUMP search <label> genes [...] bounds [...] start [...] P G E M S seed`; a line a candidate scored, `DUMP search <label> g <generation> i <index> genes [...] scores [...] sum`; and the best's, `DUMP search <label> best g i genes [...] scores [...] unseen [...] sum`, the sum the unseen scores'.

### The writer (Implemented)

<!-- @assert-count target="runtime/menteion-runtime/tests/instrument/fork_search.rs" symbol="fn scale_coupling" min="1" reason="ADR-0256: the writer of a stimulus-readout coupling" -->

| Item | What it is |
| :--- | :--- |
| `scaled_weight(weight, factor)` | the product widened to `i64`, rounded to the nearest with a half away from zero and held within ±32 767 |
| `scale_coupling(exec, from, into, factor)` | each synapse `coupling` counts from `from` into `into` written by `scaled_weight`, and no other; returns the count written |
| `scale_couplings(exec, sets, factors)` | the four in `couplings`' order, A→R0, A→R1, B→R0, B→R1, each by its own factor; returns the four counts |

### Its tests

All eight are in `fork_search.rs`, at ADR-0255's sizes, P 16, G 10, E 4, M 16 and S 3, unless a row says otherwise.

| Test | What it holds |
| :--- | :--- |
| `the_search_reaches_the_best_at_a_corner_and_inside_a_small_box_from_its_seed` | two genes in 0..=8 from 4, seed 16: the corner (8, 8) and the inner point (3, 5), each reached by a child, and the best's unseen scores and line |
| `one_seed_prints_the_same_table_twice_and_another_seed_another` | seed 0 twice gives one `Searched`; seed 1 another table; the head as written and every candidate's line in the order scored |
| `no_gene_leaves_its_bounds_and_the_count_scored_is_p_plus_g_less_one_times_p_less_e` | bounds narrower than a step, across zero and of one value, at three sizes: every read within bounds, P + (G − 1)(P − E) scored, the kept unread, streams 0 to S − 1 then S to 2S − 1, each generation's new places in order |
| `a_tie_ranks_by_the_index_so_the_kept_come_first_and_the_start_is_best` | a constant objective: the best is the start, generation 0's index 0 |
| `the_best_is_the_first_scored_of_the_largest_sum_since_the_kept_keep_their_order` | one gene scored, many ties, three seeds: the best is the first scored of the largest sum |
| `a_weight_scales_by_256ths_rounded_half_away_from_zero_and_held_within_the_rail` | `scaled_weight` at 17 pairs: each half, each side of zero, a negative factor and both rails |
| `the_writer_leaves_the_image_at_256_and_writes_each_coupling_s_synapses_and_no_other` | the instrument's network at 256 units: at 256 the image byte for byte; at 128 and 332, for each coupling, each synapse as the rounding by hand says, the image otherwise unmoved, and the other three couplings equal |
| `the_four_couplings_scale_each_by_its_own_factor` | at 256 the image byte for byte; at 128, 332, 200 and 300 each coupling by its own factor |

Two genes in 0..=8 are a smaller box than the first draft's, for the reason of item 5. The objectives add a noise of 0 or 1 a stream, and a step of one from either best costs 4 a stream, so each best is unique.

### Its runs

- **`round:additive`**: `npm run round:additive -- origin/main 1a31725` exits 0, *"c4d1548...1a31725 only adds: 2 file(s) a Rust build reads, each whole items inserted into a package's tests/ (ADR-0215)"*.
- **Nothing is dispatched**: the round measures nothing, adds no `#[ignore]`d test and pins nothing. The pull request's gate is its evidence.

### The sweep

The schedule's sweep of 2026-11-18 at 20:17Z had not run when this was written, on 2026-11-14. The first ADR that merges after it ends records it. [ADR-0251](0251-what-reaches-the-readout-units-on-both-cells.md) recorded 2026-11-11's.

## Consequences

- Good: exploration 039 and order 3 have a search that repeats bit for bit, and a writer whose rounding is held at its edges.
- Good: the change only adds, so no number an earlier round pinned can move.
- Bad: no mutant checks the module, since cargo-mutants does not mutate test targets; its tests are its only check.
- Bad: the tests' inner best is reached from a fixed seed in a box of 81 points. They say nothing of how often the search reaches an inner best in exploration 039's box of four genes in 128..=332.
- Neutral: no file under `crates/` or `src/` changes.

## Confirmation

- `cargo test --workspace --locked` runs the eight tests in the gate.
- `npm run round:additive -- origin/main <the last commit>` exits 0.
- `npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
