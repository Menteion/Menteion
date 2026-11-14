---
status: archived
date: 2026-11-13
affectedFiles: [runtime/menteion-runtime/tests/instrument/fork_search.rs, runtime/menteion-runtime/tests/inhibition.rs, docs/**, briefs/**, CHANGELOG.md]
protectedFiles: [crates/**, runtime/menteion-runtime/src/**, runtime/menteion-runtime/tests/instrument/harness.rs, runtime/menteion-runtime/tests/inhibition/pins/**, runtime/menteion-runtime/tests/assembly/**, runtime/menteion-runtime/tests/h46/**, docs/explorations/0*.md, docs/research/**, docs/benchmarks/results/**, benches/**, .github/workflows/**, .cargo/**, Cargo.toml, Cargo.lock, rust-toolchain.toml, clippy.toml, .spec-brief.json, package.json, package-lock.json, scripts/**]
integrity: sha256-d989d6fa3f252a3315db31dc9fb9416a5744590f31e0ddf724ab04383fdc1bd7
---

<!-- spec-brief:banner -->
> **Executed 2026-11-14 (UTC) in pull request [#308](https://github.com/Menteion/Menteion/pull/308).** Built ADR-0255's search by forks and the writer of the four stimulus-readout couplings as runtime/menteion-runtime/tests/instrument/fork_search.rs, a module the inhibition binary alone includes by path in two lines, held by eight fast tests; ADR-0256 records the tool built. The round measured nothing, pinned nothing and dispatched nothing; round:additive exits 0, and cargo-mutants makes no mutant in a test target. The body below describes the tree before execution and is not maintained.
<!-- /spec-brief:banner -->

# Brief 088: A search by forks

## Mission

**This brief builds [ADR-0255](../../docs/adr/0255-a-search-by-forks-and-the-two-alternative-tasks-references.md)'s search by forks and its writer of the four stimulus–readout couplings.**
It is the first step of the first order of [ADR-0254](../../docs/adr/0254-the-design-review-planned.md)'s plan; exploration 039, which reads the two-alternative task's references with the tool, is the second, and is not this brief's.

When the round is done, the tree holds:
- `runtime/menteion-runtime/tests/instrument/fork_search.rs`, included by path by the `inhibition` binary: the search and the writer, every result repeatable bit for bit from its seed;
- the tests that hold them, running with the workspace's;
- an ADR at the next free number (`ls docs/adr`) that records the tool built, `docs/STATUS.md` rewritten and a changelog entry.

The scope is the front matter's ([ADR-0191](../../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)): one new test module, the two lines that include it, and documents.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no tool and no version of either. The search is the tree's own code, from the tree's own generators.
- **The engine does not change.** Nothing under `src/` or `crates/` changes, no helper leaves `tests/inhibition.rs`, and no test that ran before changes or moves a pinned number.
- **The round measures nothing.** It adds no `#[ignore]`d test, pins no table and makes no dispatch. The search decides nothing, and nothing it finds is a constant of the engine or of a later round.
- **No hypothesis is written.**
- **The code rules hold**: no `unsafe`; no `f32` or `f64`; checked or saturating arithmetic, as `clippy::arithmetic_side_effects` requires in a test crate root; every loop ends by construction ([ADR-0062](../../docs/adr/0062-the-first-complete-sweeps-list.md)).
- **GitHub through `~/.claude/menteion-tools/ghapi.sh` alone**, never `gh`.
- **The branch** is `claude/088-a-search-by-forks` from `main`. Commits are Conventional Commits with a real body ([CONTRIBUTING.md](../../CONTRIBUTING.md#commit-messages)). The branch takes `main` by a merge, never a rebase or a force-push ([ADR-0186](../../docs/adr/0186-merge-commits.md)).
- **The pull request opens as a draft with the round's first push** ([ADR-0223](../../docs/adr/0223-the-gate-shortened-and-explorations-kept-as-tags.md)), and leaves draft with the commit that asks for the merge, which sets the round's ADR to `accepted`.
- **New text meets [ADR-0197](../../docs/adr/0197-one-place-for-each-kind-of-fact.md)'s shape**, which `npm run spec:shape` holds.
- **No push notification.** The maintainers asked for none; a question that stops the round is written at the end of the session's turn.
- **The sweep**: the schedule's sweep of 2026-11-18 at 20:17Z is recorded by the first ADR that merges after it ends ([briefs/README.md](../README.md#evidencing-a-round), item 7). If it has ended when this round's ADR merges, that ADR records it.

---

## Context

Re-derived on 2026-11-13 at `main` `7d8496a`. Line numbers move; the names are what to re-derive.

- **The harness** (`runtime/menteion-runtime/tests/instrument/harness.rs`) is included by path by six binaries (`active`, `arrival`, `assembly`, `everywhere`, `inhibition`, `instrument`); `tests/inhibition.rs` includes it as `#[path = "instrument/harness.rs"] mod harness;` and its pins the same way. A module included by `inhibition` alone runs its tests once, and reaches the harness's names and `tests/inhibition.rs`'s private items through `use crate::*;`, as `tests/inhibition/pins/h46.rs` does.
- **Generators**: `Lcg` (`testkit/prop.rs` at the repository root, included by the harness) and `mix64`.
- **Forks**: `Image::encode` of a quiescent engine (`quiet()`) once, refused on one that is not, and `Image::decode` a candidate; a run repeats bit for bit on any count of workers.
- **Writes to weights**: `Executor::blocks_mut()` over a unit's `fan_out(exec.blocks())`, which names each synapse's block, slot, target and weight; a precedent is in `tests/inhibition.rs`, where a test writes synapses this way. The geometry of `PERIOD` 20 gives the stimulus and readout sets (stimulus A at place 0, B at 11; readout 0 the odd places but 11, readout 1 the even places but 0), and `coupling`/`couplings` read them. A weight is an `i16`; consolidation holds its magnitude to the rail, 32 767.
- **`round:additive`** (`scripts/check-additive.mjs`): an inserted top-level item may not be a `use` or take a name the sources its crate sees already hold. `search` is one (`crates/menteion-arithmetic`); `fork_search` is not. rustfmt puts `#[path]` and `mod` on two lines.
- **Gate traps**: a `pub(crate)` item no test uses is `dead_code` under `-D warnings`; more than seven arguments trips `clippy::too_many_arguments`; MSRV 1.85 has no `let` chains; a `const` longer than 24 lines in `tests/inhibition.rs` fails `spec:pins`.
- **The couplings H-25 bounds**: clause 2 reads a coupling against its image's as `coupling × 100 ≤ image × 130`; 332/256 is the largest factor in 1/256 within it.
- **What uses the tool next**: exploration 039 ([ADR-0255](../../docs/adr/0255-a-search-by-forks-and-the-two-alternative-tasks-references.md)), on H-42's three networks; then order 3, on H-47's task.

---

## Deliverables

### The search

- [x] `runtime/menteion-runtime/tests/instrument/fork_search.rs`, and two lines in `tests/inhibition.rs` that include it by path as `mod fork_search`, with no `use`. It takes genes, each a named integer with inclusive bounds and a start value; an objective, a whole-number score of the genes on a stream; P, G, E, M, S and a seed.
- [x] ADR-0255's algorithm, in integers from `Lcg` seeded by the seed:
  - generation 0 is the start genes and P − 1 candidates drawn uniformly within the bounds;
  - each new candidate is scored on streams 0 to S − 1 and ranked by the sum, ties by its index in the generation, the kept first in their rank and the children in the order made;
  - the E best are kept with their scores and not read again;
  - each other place takes a child of two parents, each drawn by a tournament of two members drawn uniformly, the better ranked taken. Each gene comes from either parent with probability 1/2, then moves with probability 1/4 by a step drawn uniformly from −M to M, held within its bounds;
  - after G generations the best of the last is read again on streams S to 2S − 1.
- [x] Printed: every candidate scored, with its generation, index, genes and score on each stream; then the best's scores on the streams it never saw.

### The writer

- [x] A function that scales each synapse `coupling` counts, from an excitatory unit of one stimulus's set onto any unit of one readout's set, by a factor in units of 1/256, the product widened to `i64`, rounded to the nearest with a half away from zero and held within ±32 767, and touches no other synapse.

### The tests

- [x] On synthetic objectives whose best is known, one at a corner of a small box and one inside it, the search at P = 16, G = 10, E = 4, M = 16 and S = 3 reaches the best from the seed the test fixes.
- [x] One seed prints the same table twice, and another seed another.
- [x] No gene leaves its bounds, and the count of candidates scored is P + (G − 1)(P − E).
- [x] On a small engine, the writer at 256 leaves the encoded image byte for byte; at 128 and at 332 it writes each synapse of the coupling as the rounding says, and no other synapse moves.

### The documents

- [x] An ADR at the next free number: the tool as built, its API and the tests that hold it, and `npm run round:additive -- origin/main <the last commit>` quoted.
- [x] `docs/STATUS.md` rewritten, the changelog's entry, the index (`node scripts/adr-index.mjs`), and the whitepaper's version, which the ADR's row in §9 changes.
- [x] This brief archived with `spec-brief archive 088 --strict`, every box dispositioned.

---

## Not empowered

- Changing anything under `src/` or `crates/`, the harness, or any rule, constant or configuration of the engine.
- Moving a helper out of `tests/inhibition.rs`, or changing a test that ran before.
- An `#[ignore]`d test, a pinned table, a dispatch, or any read of a task's references, which are exploration 039's.
- A hypothesis, a stopping rule or a verdict on a mechanism.
- A dependency, a crate or a tool from outside the tree.

---

## Architectural empowerment

The round may, recording each in its ADR:
- choose the names inside the module, its types and how an objective is passed, so long as it takes and prints what ADR-0255 says;
- choose the synthetic objectives and the small engine of the writer's tests, so long as the tests are fast in a debug build;
- put the tests in `fork_search.rs` or in `tests/inhibition.rs`, which reaches the module's items by path;
- widen `affectedFiles` within its mission, in a commit of its own that says why ([briefs/README.md](../README.md#archiving-one)).

It may not reach the whitepaper's invariants or `CLAUDE.md`'s constraints.

---

## Verification

Before each push, on the developer machine:

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
npm ci
npm run spec
```

The pull request's gate runs those and the rest: the release tests, the benchmarks once, the MSRV, AArch64 and the mutation gate on the changed lines. Every check passes on the last commit.

```bash
npm run round:additive -- origin/main HEAD
node scripts/adr-index.mjs
npx --no-install @spec-family/spec-brief archive 088 --strict --pr <n> --summary "<one paragraph>"
```

- `round:additive` exits 0: the change only adds. If it does not, the round renames what it inserted; a change that cannot only add stops the round, and the session says why.
- No dispatch is made: the round measures nothing.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

---

## Report

The closing message states:
- the tool as built, its API, and the tests that hold it;
- `round:additive`'s exit;
- whether the sweep of 2026-11-18 had ended when the ADR merged, and if so that the ADR records it;
- any widening of `affectedFiles`, and why;
- that the pull request is ready for review;
- what was not done and why.
