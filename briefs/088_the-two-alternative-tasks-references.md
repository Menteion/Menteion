---
status: proposed
date: 2026-11-14
affectedFiles: [runtime/menteion-runtime/tests/**, docs/**, briefs/**, CHANGELOG.md, README.md, scripts/exhaustive-costs.tsv]
protectedFiles: [crates/**, runtime/menteion-runtime/src/**, docs/explorations/0*.md, docs/benchmarks/results/**, benches/**, .github/workflows/**, .cargo/**, Cargo.toml, Cargo.lock, rust-toolchain.toml, clippy.toml, .spec-brief.json, package.json, package-lock.json, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/check-additive.mjs, scripts/check-archive.mjs, scripts/check-scope.mjs, scripts/check-explorations.mjs, scripts/check-doc-shape.mjs, scripts/adr-index.mjs]
---

# Brief 088: The two-alternative task's references

## Mission

**This brief builds [ADR-0255](../docs/adr/0255-the-two-alternative-tasks-references.md)'s search by forks and reads the two-alternative task's references.**
It is the first order of [ADR-0254](../docs/adr/0254-the-design-review-planned.md)'s plan.

When the round is done, the tree holds:
- a search by forks, a module of the test harness, every result repeatable bit for bit from its seed, held by tests that run with the workspace's;
- on the three reference networks, the engine, two floors and the search's ceiling read on the same trials of the two-alternative task, their tables pinned and reproduced;
- an ADR at the next free number (`ls docs/adr`) that records the search built and the references read, `docs/STATUS.md` rewritten and a changelog entry.

The scope is the front matter's ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)): tests and documents, nothing under `src/` or `crates/`.

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no tool, no version of either. The search is the tree's own code, from the tree's own generators.
- **The engine does not change.** Nothing under `src/` or `crates/` changes, and no pinned number of any other test moves; one that moves stops the round as a finding.
- **The search decides nothing.** What it finds is a reference, read again on streams it never saw, and never a constant of a later round.
- **No hypothesis is written.** The readings are references, set beside each other and beside H-25's clause 1; the round predicts nothing and judges no mechanism.
- **No `unsafe` beyond ADR-0023's.** No `f32`/`f64`; every loop ends by construction ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)); the mutation gate catches every mutant in the lines the change touches ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The runs are dispatches** ([briefs/README.md](README.md#evidencing-a-round)): the calibration as `npm run round:additive` reads it, the three jobs at `scope=arms` on the protocol's commit, the calibration read first, and their reproduction as a second dispatch at `scope=arms` after the tables are pinned, as H-47's long jobs were reproduced.
- **The pull request opens as a draft with the round's first push** ([ADR-0223](../docs/adr/0223-the-gate-shortened-and-explorations-kept-as-tags.md)), and leaves draft with the commit that asks for the merge, which sets the round's ADR to `accepted`.
- **No push notification.** The maintainers asked for none; a question that stops the round is written at the end of the session's turn.

---

## Context

Re-derived on 2026-11-14 at `main` `7d8496a`. Line numbers move; the names are what to re-derive.

- **The task**: `Task` (`runtime/menteion-runtime/src/task.rs`): `stimulus_at`, `trial_presenting`, `trial_to`; a trial's readout counts by `Readout::count_window`, the answer by `Readout::select`, a tie no answer and an error; `Outcome` carries the counts, the selection and `correct`. H-25 rewards through the engine's critic (`Executor::reward`), not the task's.
- **The harness** (`runtime/menteion-runtime/tests/instrument/harness.rs`): `BLOCK` 64, `WINDOW` from tick 100 for 500, the geometry of `PERIOD` 20 (stimulus A at place 0, B at 11; readout 0 the odd places but 11, readout 1 the even places but 0; 51 units a stimulus set and 459 a readout set at 1 024 units), `task_over`, `run_on_presenting_from` with `Presented` and its per-trial `observe(exec, trial, start_tick, &outcome)`, and `Lcg` (from `testkit/prop.rs`).
- **The networks**: `derive(seed)` in `tests/inhibition.rs` gives each reference network's start image, H-25's image of it, from which H-47 trains; `zero_from`, `zero_groups` and `night_engine` build an engine of it under the running configuration of [ADR-0233](../docs/adr/0233-a-running-configuration-built.md), the zero sum's groups set again at every decode since they are not in the image. The seeds are 22, 23 and 24.
- **Forks**: `Image::encode` of a quiescent engine once (`quiet()`), `Image::decode` for each candidate; a drive stream is the drive's seed offset, as `ReadFrom` and `spread_read` do. A run repeats bit for bit on any count of workers.
- **Writes to weights**: `Executor::blocks_mut()` over a unit's `fan_out(exec.blocks())`, which names each synapse's block, slot, target and weight. The four addressed couplings are the excitatory synapses from each stimulus's units onto each readout's units; on seed 22 at 1 024 units they number 775, 806, 798 and 809. The prior draws excitatory weights from 6 000 to 12 000, and consolidation holds a magnitude to the rail, 32 767.
- **The critic's rule**: `ValueCritic` (`crates/menteion-neuromod/src/lib.rs`), `value_q16` over pairs of weight and count and `step(weight, error_q16, count)`, pure and public.
- **What the engine reads**: H-25's mappings score 116 to 124 of their last 128 trials ([ADR-0140](../docs/adr/0140-the-address-drawn-measured.md)); the first mapping crosses 40 of 64 by block 4 to 6, a reversal by block 14 to 21. With the reward withheld, the two readouts' summed counts are near chance ([ADR-0065](../docs/adr/0065-the-instrument-recalibrated.md)).
- **A withheld read is not frozen**: under H-25's image the inhibitory rule runs at its baseline over a read.
- **Cost**: one read of 64 trials takes 5.6 to 6.1 s on the AArch64 runners at 1 024 units, two engines at a time ([exploration 036](../docs/explorations/036-what-reaches-the-readout-units-on-both-cells.md)); an arms job's bound is 240 minutes.
- **Where the helpers are**: most of what the round needs is private to `tests/inhibition.rs`. The round moves what it uses into the harness or into its module, unchanged, so that every test that used them runs as it did.

---

## Deliverables

### The search

- [ ] `runtime/menteion-runtime/tests/instrument/search.rs`, included by path where it is used, as the harness is: named integer parameters with inclusive bounds; an objective, a whole-number score of a candidate read from forks of one quiescent image; a seed. ADR-0255's algorithm, in integers from `Lcg`: P candidates over G generations, generation 0 holding the image as it is; each scored on S drive streams and ranked by the sum, ties by index; the E best kept; children of two parents drawn by a tournament of two, each gene from either parent, then moved with probability 1/4 by a step from ±M, held within its bounds.
- [ ] Printed: every candidate scored, with its generation, index, genes and score on each stream; then the best read again on S streams the search never saw.
- [ ] Tests that run with the workspace's: on a synthetic objective with a known best the search reaches it within its budget; two runs from one seed print the same table; no gene leaves its bounds.

### The references

- [ ] On each network, the schedule: the first mapping for 24 blocks, then its first reversal for 24 blocks, on one drive stream.
- [ ] The engine on that schedule, rewarded under the running configuration.
- [ ] The two floors on the same trials: one that sees the two readouts' summed window counts, one that sees each readout unit's window count. Each keeps a weight vector for each answer, answers the larger value, a tie an error, and moves the chosen answer's weights by `ValueCritic`'s step on the error of its value against the reward. Their features come from the network with the reward withheld. No feature includes a stimulus unit.
- [ ] The ceiling: the search over the four addressed couplings' factors, in units of 1/256 from 128 to 333, rounding each weight and holding it on the rail, the zero sum's groups set from the written image's own sums; its objective the correct trials of one withheld block on each of S = 3 streams; P = 16, G = 10, E = 4, M = 16; for the first mapping and, apart, for its reversal.
- [ ] Three jobs, one a network, named by it and kept out of the weekly sweep as H-47's were, and a trial that times one generation on seed 22. Each job's projection, from the trial, is under 240 minutes; past it, G falls first, then P, and the ADR says so.

### The documents

- [ ] The protocol committed before any job ran: the schedule, the learners, the genes, the budget and the streams, pinned in the round's pins file (`tests/inhibition/pins/` if the binary is `inhibition`, as [ADR-0227](../docs/adr/0227-the-pinned-tables-in-a-file-a-round.md) has a round's).
- [ ] The tables written from the jobs' logs and pinned, reproduced by the second dispatch.
- [ ] An ADR at the next free number: the search built; network by network, each learner's and the engine's correct trials a block, the block each first reaches 40 of 64 in each mapping and each mapping's last eight blocks; the ceiling's best on the search's streams and on the three it never saw, with its genes; whether each floor reaches H-25's clause 1 on this schedule, as a reading. If a floor does, the ADR says that the task cannot rank a mechanism, for ADR-0254's next decision.
- [ ] `docs/STATUS.md` rewritten, the changelog's entry, the index (`node scripts/adr-index.mjs`), the whitepaper's version where it changes.
- [ ] This brief archived with `spec-brief archive 088 --strict`, every box dispositioned.

---

## Not empowered

- Changing anything under `src/` or `crates/`, or any rule, constant or configuration of the engine.
- A hypothesis, a stopping rule or a verdict on a mechanism.
- A search over anything but the four couplings, or a ceiling without H-25's bound.
- A floor that reads a stimulus unit, the critic's window or anything the readout does not see.
- Taking anything the search finds as a constant of the engine or of a later round.
- H-47's task, or any other task's references.
- A dependency, a crate or a tool from outside the tree.

---

## Architectural empowerment

The round may, recording each in its ADR:
- choose how the helpers it needs leave `tests/inhibition.rs`, so long as every test that used them runs as it did;
- set P, G, E, M and S other than ADR-0255's where the trial's projection or a synthetic test shows the given ones cannot be met, saying why;
- read the floors' features from one withheld read shared by both, so long as each learner sees only its own features;
- widen `affectedFiles` within its mission, in a commit of its own that says why ([briefs/README.md](README.md#archiving-one)).

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

By dispatch, never on the developer machine. A `ci.yml` dispatch is `POST /repos/Menteion/Menteion/actions/workflows/ci.yml/dispatches` with a JSON body, through `ghapi.sh`:

```bash
npm run round:additive -- origin/main <the protocol's commit>
# ci.yml on this round's branch: the calibration round:additive names, and inputs {"scope":"arms","tests":"<the trial and the three jobs>"}
# after the tables are pinned: inputs {"scope":"arms","tests":"<the three jobs>"}
node scripts/adr-index.mjs
npx --no-install @spec-family/spec-brief archive 088 --strict --pr <n> --summary "<one paragraph>"
```

- The calibration is green before any job's log is read.
- The three jobs are green, and their reproduction holds every pinned table.
- Every commit a dispatch ran on is an ancestor of the pull request's head, and no file under `runtime/` changes after the reproduction's commit.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

---

## Report

The closing message states:
- the search as built and the tests that hold it;
- network by network, the engine's, the two floors' and the ceiling's readings, and whether each floor reaches H-25's clause 1;
- each dispatch with its run id and commit, and each job's time against its bound;
- any change to P, G, E, M or S, and why;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine;
- that the pull request is ready for review;
- what was not done and why.
