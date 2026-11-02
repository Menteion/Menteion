---
status: proposed
date: 2026-11-02
affectedFiles: [runtime/menteion-runtime/src/**, runtime/menteion-runtime/tests/**, testkit/**, docs/**, briefs/**, CHANGELOG.md, CLAUDE.md, README.md, scripts/exhaustive-costs.tsv]
protectedFiles: [crates/**, docs/explorations/0*.md, docs/benchmarks/results/**, benches/**, .github/workflows/**, .cargo/**, Cargo.toml, Cargo.lock, rust-toolchain.toml, clippy.toml, .spec-brief.json, package.json, package-lock.json, scripts/arm-run.sh, scripts/exhaustive-shard.sh, scripts/exhaustive-costs.mjs, scripts/check-additive.mjs, scripts/check-archive.mjs, scripts/check-scope.mjs, scripts/check-explorations.mjs, scripts/check-doc-shape.mjs, scripts/adr-index.mjs]
---

# Brief 087: Learning that sleep keeps

<!-- @assert-absence target="runtime/menteion-runtime/src/executor.rs" symbol="fn spend_episodes" reason="brief 087 precondition: main spends no ledger on a call; the round that builds it archives this brief in the same pull request" -->

## Mission

**This brief builds [ADR-0234](../docs/adr/0234-learning-that-sleep-keeps.md)'s three parts and runs H-47 once.**
M11 asks for a learned mapping held through a sleep with replay, against one without ([Appendix C](../docs/WHITEPAPER.md#appendix-c-roadmap)).

When the round is done, the tree holds:
- the slow-wave baseline as a configuration's, the ledger spent on a call, and a trial that presents the units the host gives, each unset the engine as it is, held by property tests;
- the documents outside the state crates giving the replay's drive as it arrives, and F-47 resolved;
- H-47's protocol committed before any arm ran, its 24 jobs run, its verdict, and a short form of each of its twelve cells in the weekly job.

It writes a built ADR, a protocol ADR and a measured ADR, adds its readings to [H-47's report](../docs/research/H-47.md) and rewrites `docs/STATUS.md` ([briefs/README.md](README.md#a-rounds-documents)).
The scope is the front matter's ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)).

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no tool, no version of either.
- **Unset is the engine as it is.** With the three parts unset no number moves, and the format stays 26. A pinned number that moves stops the round as a finding (H-47's stopping rule, step 2).
- **H-47 is ADR-0234's**: its clauses, its constants and its five predicted readings are committed in the protocol ADR and pinned in the tests before any arm runs, and nothing among them moves after.
- **The replay stays plain.** The twin, its drive divided by the gain, is built nowhere: it stays on the explorations' tags.
- **No `unsafe` beyond ADR-0023's.** No `f32`/`f64`; every loop ends by construction ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)); the mutation gate catches every mutant in the lines the change touches ([ADR-0030](../docs/adr/0030-verification-governance.md)); a new rule carries a test over the lattice.
- **The runs are dispatches** ([briefs/README.md](README.md#evidencing-a-round)): the calibration as `npm run round:additive` reads it, and the 24 jobs and the twin at `scope=arms`, dispatched together on the protocol's commit, the calibration and the twin read first. ADR-0234 reproduces the long jobs at `scope=arms` on the reproduction's commit, as ADR-0226 did, since `scope=exhaustive` cannot list them. No whole-domain test runs on the developer machine.
- **The pull request opens as a draft with the round's first push** ([ADR-0223](../docs/adr/0223-the-gate-shortened-and-explorations-kept-as-tags.md)), and leaves draft with the commit that asks for the merge, which sets the round's ADRs to `accepted`.
- **The branch is named `claude/brief-087-…`**, never rebased or force-pushed once a dispatch has run on it.
- **The documents are ADR-0197's**: the new ADRs in its shape, the tables written by `node scripts/adr-index.mjs`, new text within `npm run spec:shape`'s limits.
- **H-47's pinned tables go in `tests/inhibition/pins/h47.rs`**, a row a line, declared in its `mod.rs` ([ADR-0227](../docs/adr/0227-the-pinned-tables-in-a-file-a-round.md)).
- **No shard of the weekly job passes 60 per cent of its bound**, 240 minutes ([ADR-0179](../docs/adr/0179-the-bounds-rule-restated.md)). One that does is reported with its job's minutes.
- **GitHub without gh.** Dispatches, the pull request, logs and artifacts go through the REST API with the repository's own token (`~/.claude/menteion-tools/ghapi.sh` and its README); `git push` uses the same token. The token is never printed or put on a command line.
- **The maintainer is notified** as `briefs/README.md` says: when a question only they can answer stops the round, and when the pull request is ready for review.
- Conventional Commits with a real body; never commit on `main`.

## Context

Re-derived on 2026-11-02 against `main` at `8641246`. Line numbers move; the symbols are what to re-derive.

1. **The replay** (`runtime/menteion-runtime/src/executor.rs`): `REPLAY_MESSAGES` messages of `REPLAY_DRIVE_Q16`, 1.25 each, built as `REPLAY_MESSAGE` and delivered on the ripple's cadence by `deliver`. The turn scales the summed batch by the tick's gain in `scaled`, an injected message as a synapse's.
2. **The documents that give the replay's drive before the gain**:
   - in `executor.rs`, the comment on `REPLAY_DRIVE_Q16` ("about twelve ticks on");
   - in `task.rs`, the module's comment ("about twelve ticks on, as a replay does") and the comment on `CUE_Q16` in its tests;
   - in the whitepaper, §6.6 step 2, §8.8's row for the complementary learning systems ("a drive of 2.5"), and [F-47](../docs/WHITEPAPER.md#11-risks-and-technical-debt)'s row;
   - the comment on `RIPPLE_SHIFT` in `crates/menteion-hippocampus/src/lib.rs` ("one replay is one spike of each unit of the pattern"), which is a state crate's and protected here: the built ADR names it as left.
   An accepted ADR's body is never edited.
3. **The modulation**: the tick builds `Modulations::of(&self.modulator, self.modulation_baseline_q16, …)` before the barrier. The sleep stage is `homeostasis.sleep_stage`, `STAGE_SWS` in slow-wave sleep (`menteion-homeostasis`).
4. **The ledger**: `Executor::tag_episode`, `episodes`, `episode_room`, and `Episode::is_spent` (`menteion-hippocampus`). A ripple replays the episode under the hand that is not spent.
5. **The task** (`runtime/menteion-runtime/src/task.rs`): `Stimulus::inject` and `cancel_at` send to the set's units through `send(set, …)`, and `Task::trial` presents the trial's stimulus.
6. **The explorations' code**, written for explorations and not reviewed:
   - the tag `explore/014-what-a-night-keeps`: `Config::slow_wave_baseline_q16`, `Executor::spend_episodes`, and the twin `Config::replay_after_gain`, which this round does not build;
   - the tag `explore/016-what-a-shared-source-takes`: `Stimulus::inject_into`, `cancel_into` and `Task::trial_presenting`;
   - their harness on the tag `explore/019-a-night-without-replay`, in `tests/inhibition.rs`: `night_engine`, `night_read` and its `tag` switch, `idle_on`, `NightsGap`, `what_nights_keep`, `NightFrom`, `ReadFrom` and the `nights_without_replay_…` tests.
7. **The starts**: H-46's eighth images, `tests/h46/zero<seed>-<arm>-image-8.image` with their CRC-64, and a first mapping learned from H-42's image as exploration 016 learned it.
8. **The running configuration** ([ADR-0233](../docs/adr/0233-a-running-configuration-built.md)): `Config::runs_a_settled_image()`, the groups read once with `Executor::coupling_sum` and set with `Executor::set_zero_sum` at each load.
9. **The explorations' readings and their logs**: exploration 019's runs 41408442255 and 41413189972, and 018's 41367828454 and 41374137568, hold seed 22's rows for the twin of step 2. Their artifacts expire, so the round downloads them first.
   - The tag's `night_engine` decodes under the harness's `config`, with kept addresses and the decoded exponent, not under `Config::runs_a_settled_image()`.
   - [ADR-0233](../docs/adr/0233-a-running-configuration-built.md) held H-46's arm to the name with the kept addresses unset. Here the twin is what holds the name to the explorations' logs.
10. **The process**: eighteen weekly shards, the bound 240 minutes; a `…_paused` test runs only by name at `scope=arms` ([ADR-0127](../docs/adr/0127-the-paused-line-leaves-the-weekly.md)); `spec-brief archive --strict` measured from `origin/main`.
11. **The sweep**: the schedule runs on 2026-11-04 at 20:17Z. If it has ended when this round's first ADR merges, that ADR records it ([briefs/README.md](README.md#evidencing-a-round)).

## Deliverables

- [ ] **The three parts, built, in a new ADR at the next free number (`ls docs/adr`), kind `built`.**
  - The slow-wave baseline as a `Config` field, unset by default, in no named configuration and not in the image.
  - The ledger spent on a call between ticks.
  - A trial that presents the units the host gives, its messages and its cancel to them.
  - A test over `testkit/prop.rs`'s lattice for each: unset, or given each set's own units, the engine and the trial are what they were bit for bit.
  - The documents of Context 2 outside the state crates giving the replay's drive as it arrives, 4.375 at a gain of 1.75, and F-47's row resolved with the `RIPPLE_SHIFT` comment named as left.
- [ ] **H-47's protocol, in a new ADR at the next free number, kind `protocol`, committed before any arm runs.**
  - The harness, its 24 `…_paused` jobs and 12 `…_exhaustive` short forms, `tests/inhibition/pins/h47.rs` with its placeholder tables, and the verdict's test over them.
  - Every constant of the harness listed with its value, and each term of [H-47's report](../docs/research/H-47.md#its-terms) tied to the code that reads it.
  - `npm run round:additive`'s reading quoted, and the calibration it names.
  - The twin of H-47's step 2 as a `…_paused` test of its own. Its lines are pinned by hash in `h47.rs` from the explorations' logs, the label aside: the `PIN` lines, the groups' `kinds`, the head's `run` lines, each night's `start`, `window` and `end` lines, each image's `kinds` and `held`, and the 32 reads of each image.
- [ ] **The dispatches.**
  - The calibration and the 24 jobs together on the protocol's commit; the calibration and the twin read first.
  - The tables pinned from the jobs' logs, then the reproduction: the short forms at `scope=exhaustive -f only`, and the 24 jobs again at `scope=arms`.
- [ ] **H-47's verdict, in a new ADR at the next free number, kind `measured`.**
  - The verdict per network, start and clause in one table; each predicted reading held or missed in a line; the stopping rule's step reached and the next decision it names.
  - Whether the schedule's sweep of 2026-11-04 has ended, and its record if it has.
- [ ] **The documents, in the same pull request.**
  - [H-47's report](../docs/research/H-47.md) with its readings' tables and their dispatches.
  - `docs/STATUS.md` rewritten whole.
  - Whitepaper: the slow-wave baseline beside the modulation's baseline with an `@assert-count`, §6.6, Appendix C's M11 row, and the version in both declarations with its date.
  - The tables written by `node scripts/adr-index.mjs`, the cost table regenerated, and one `CHANGELOG.md` entry of at most 400 characters.
- [ ] **The brief archived** with `spec-brief archive 087 --strict`, every deliverable dispositioned. The round's ADRs are set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says.
- **No record, section or format changes**, and no state crate changes.
- **No value of `Config::default()`, `Config::reads_its_targets()` or `Config::runs_a_settled_image()` changes.** The slow-wave baseline enters none of them.
- **The twin is not built**, in `src/` or in the tests.
- **No constant of H-47's moves** after the protocol's commit, and no pinned number moves.
- **No derivation, lead-in or arm is run beyond the dispatches above**, and no whole-domain test runs on the developer machine.
- No float anywhere, no dependency, no `unsafe` beyond ADR-0023's, and no rebase of the branch once a dispatch has run on it.

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the three parts' names, and whether the units a trial presents are a `Task` call's argument or a field of the stimulus;
- how the harness is built from the tags' code, the twin of step 2 held;
- each short form's length, the twelve within 3 600 s together on x86-64 as the reproduction reads them alone;
- splitting a cell's two jobs further, within the job's bound;
- a file the round must change and its scope does not hold, within this brief's mission, widened as `briefs/README.md` says.

It may not:
- write into a protected file, or shrink `protectedFiles`;
- change a value of a named configuration, or what `Config::default()` builds;
- build the twin, or change the replay's drive;
- change H-47's clauses, constants or readings after the protocol's commit;
- run a derivation, a lead-in or an arm beyond the dispatches above, or any whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If the twin of step 2 is not the explorations' logs, or a dispatch moves a pinned number, the round stops there: the finding is written, and nothing is merged until it is understood.

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
# ci.yml on this round's branch: the calibration round:additive names, and inputs {"scope":"arms","tests":"<the 24 jobs and the twin>"}
# after the tables are pinned: inputs {"scope":"exhaustive","only":"<the 12 short forms>"} and {"scope":"arms","tests":"<the 24 jobs>"}
node scripts/exhaustive-costs.mjs from <the downloaded shards' artifacts> --run '<the calibration> and <the reproduction>' --keep scripts/exhaustive-costs.tsv > costs.new
mv costs.new scripts/exhaustive-costs.tsv
node scripts/adr-index.mjs
npx --no-install @spec-family/spec-brief archive 087 --strict --pr <n> --summary "<one paragraph>"
```

- The calibration is green and the twin holds its pinned hashes, before any arm's log is read.
- The 24 jobs are green, and their reproduction holds every pinned table.
- Every commit a dispatch ran on is an ancestor of the pull request's head, and no file under `runtime/`, `crates/` or `benches/` changes after the reproduction's commit.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

## Report

The closing message states:
- the three parts as built, and that the named configurations build the engines they built;
- the documents rewritten for F-47, and F-47 resolved;
- H-47's verdict per network, start and clause, and each predicted reading held or missed;
- each dispatch with its run id and commit, and each job's time against its bound;
- the regenerated cost table and the short forms' seconds;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine;
- that the pull request is ready for review;
- what was not done and why.
