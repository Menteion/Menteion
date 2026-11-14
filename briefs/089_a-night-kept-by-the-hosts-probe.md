---
status: proposed
date: 2026-11-14
affectedFiles: [runtime/menteion-runtime/tests/inhibition.rs, runtime/menteion-runtime/tests/inhibition/pins/h48.rs, runtime/menteion-runtime/tests/inhibition/pins/mod.rs, docs/**, briefs/**, CHANGELOG.md, scripts/exhaustive-costs.tsv]
protectedFiles: [crates/**, runtime/menteion-runtime/src/**, runtime/menteion-runtime/tests/instrument/**, runtime/menteion-runtime/tests/inhibition/pins/h42.rs, runtime/menteion-runtime/tests/inhibition/pins/h46.rs, runtime/menteion-runtime/tests/inhibition/pins/h47.rs, runtime/menteion-runtime/tests/assembly/**, runtime/menteion-runtime/tests/h46/**, testkit/**, docs/explorations/0*.md, docs/research/H-47.md, docs/benchmarks/results/**, benches/**, .github/workflows/**, .cargo/**, Cargo.toml, Cargo.lock, rust-toolchain.toml, clippy.toml, .spec-brief.json, package.json, package-lock.json, scripts/*.mjs, scripts/*.sh]
---

# Brief 089: A night kept by the host's probe

## Mission

**This brief builds [ADR-0257](../docs/adr/0257-a-night-kept-by-the-hosts-probe.md)'s gated night in the tests and runs H-48 once.**
It is M11's step in the third order of [ADR-0254](../docs/adr/0254-the-design-review-planned.md)'s plan. Exploration 040, which reads H-47's task's references, is not this brief's, and this brief does not read its note.

When the round is done, the tree holds:
- the gated night in `tests/inhibition.rs`, the host's: at each of H-47's gaps both of H-47's nights in forks, each fork's next stretch, the probe on streams 16 to 18, and the choice;
- H-48's protocol committed before any arm ran, its twelve jobs run, its verdict, and a short form of each of its twelve cells in the weekly job.

It writes a protocol ADR and a measured ADR, adds its readings to [H-48's report](../docs/research/H-48.md) and rewrites `docs/STATUS.md` ([briefs/README.md](README.md#a-rounds-documents)).
The scope is the front matter's ([ADR-0191](../docs/adr/0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)).

---

## Standing directives

- **Latest ≠ Newest** (whitepaper §2.1, `CLAUDE.md` principle 4). The round depends on nothing new: no crate, no tool and no version of either.
- **The engine does not change.** Nothing under `src/` or `crates/` changes, the harness `tests/instrument/harness.rs` does not change, and no test that ran before changes or moves a pinned number. The gate is the host's, in the tests.
- **H-48 is ADR-0257's**: its clauses, its constants and its four predicted readings are committed in the protocol ADR and pinned in the tests before any arm runs, and nothing among them moves after.
- **H-47's rows are read, never run again**: the comparisons are H-47's pinned rows in `tests/inhibition/pins/h47.rs`, which the round does not change.
- **The code rules hold**: no `unsafe`; no `f32` or `f64`; checked or saturating arithmetic, as `clippy::arithmetic_side_effects` requires in a test crate root; every loop ends by construction ([ADR-0062](../docs/adr/0062-the-first-complete-sweeps-list.md)); the mutation gate catches every mutant in the lines the change touches ([ADR-0030](../docs/adr/0030-verification-governance.md)).
- **The runs are dispatches** ([briefs/README.md](README.md#evidencing-a-round)): the calibration as `npm run round:additive` names it, and the twelve jobs at `scope=arms`, dispatched together on the protocol's commit, the calibration read first. The reproduction runs the short forms at `scope=exhaustive -f only` and the twelve jobs again at `scope=arms`, as H-47's was. No whole-domain test runs on the developer machine.
- **The pull request opens as a draft with the round's first push** ([ADR-0223](../docs/adr/0223-the-gate-shortened-and-explorations-kept-as-tags.md)), and leaves draft with the commit that asks for the merge, which sets the round's ADRs to `accepted`.
- **The branch** is `claude/089-a-night-kept-by-the-hosts-probe` from `main`, never rebased or force-pushed once a dispatch has run on it ([ADR-0186](../docs/adr/0186-merge-commits.md)). Conventional Commits with a real body ([CONTRIBUTING.md](../CONTRIBUTING.md#commit-messages)).
- **The documents are ADR-0197's**: the new ADRs in its shape, the tables written by `node scripts/adr-index.mjs`, new text within `npm run spec:shape`'s limits.
- **H-48's pinned tables go in `tests/inhibition/pins/h48.rs`**, a row a line, declared in its `mod.rs` ([ADR-0227](../docs/adr/0227-the-pinned-tables-in-a-file-a-round.md)).
- **No shard of the weekly job passes 60 per cent of its bound**, 240 minutes ([ADR-0179](../docs/adr/0179-the-bounds-rule-restated.md)). One that does is reported with its job's minutes.
- **GitHub through `~/.claude/menteion-tools/ghapi.sh` alone**, never `gh`; the token is never printed or put on a command line.
- **No push notification.** The maintainers asked for none; a question that stops the round is written at the end of the session's turn.
- **The sweep**: the schedule's sweep of 2026-11-18 at 20:17Z is recorded by the first ADR that merges after it ends ([briefs/README.md](README.md#evidencing-a-round), item 7). If it has ended when this round's first ADR merges, that ADR records it.

---

## Context

Re-derived on 2026-11-14 at `main` `22cb69a`. Line numbers move; the names are what to re-derive.

1. **H-47's harness** (`runtime/menteion-runtime/tests/inhibition.rs`): `with_nights` lays out a cell; `nights_run` runs a stretch of the pair's blocks from an image, each stretch from the path's first index (`path_from`); `nights_gap` runs a gap of one form (`NightsGap::NoReplay`, `NightsGap::PlainFirst`) from an image, A's patterns those of the start image (`mapping_patterns`); `nights_job` is the form loop; `nights_reads` and `spread_read` read an image by `nights_read_from(i)`; `nights_short` is a cell's short form; `two_at_a_time` runs two items at once.
2. **H-47's constants**: `NIGHTS_PATHS` 0, 5 and 6; `NIGHTS_EVERY` 6, `NIGHTS_LAST` 24 and `NIGHTS_ON` 12; `NIGHTS_CHECKPOINTS` 6, 12, 18, 24 and 36; `NIGHTS_READS` 16, the reads on streams 0 to 15; `NIGHTS_PAIR_FLOOR` 56.
3. **H-47's rows**: `NightsRow` (an image's CRC-64, A's and the pair's sixteen counts, A's couplings, their highest, the gap's gain) and `NIGHTS_ROWS[n][s][f]` in `tests/inhibition/pins/h47.rs`, by network, start and form, a path's five checkpoints after another's. The paired SE is `nights_split` over `nights_differences`; clause 3's bound is `nights_bounded`; the verdict's rules test is `the_nights_of_h_47_their_clauses_and_their_predicted_readings_rules`.
4. **H-47's report** ([H-47.md](../docs/research/H-47.md)) holds the terms H-48 takes, and its dispatches the jobs' times: 61.2 to 68.6 minutes for two forms on three paths.
5. **The process**: a `…_paused` test runs only by name at `scope=arms` ([ADR-0127](../docs/adr/0127-the-paused-line-leaves-the-weekly.md)); a round that only adds calibrates with its own calibration tests alone ([ADR-0215](../docs/adr/0215-the-calibration-a-change-needs.md)); a round's pinned tables are in its own file, no `const` in the binary's file longer than 24 lines (`spec:pins`).
6. **Gate traps**: MSRV 1.85 has no `let` chains; a `pub(crate)` item no test uses is `dead_code` under `-D warnings`; more than seven arguments trips `clippy::too_many_arguments`; `round:additive` refuses an inserted item that is a `use` or takes a name the tree holds.
7. **The sweep**: the schedule runs on 2026-11-18 at 20:17Z.

---

## Deliverables

- [ ] **The gated night, in `tests/inhibition.rs`.** On each path, the pair's head as H-47's; at each gap both forks, each fork's stretch, the probe and the choice, as ADR-0257 specifies; the gated chain's images at the five checkpoints read as H-47 reads them.
  - Inside the test: while the gate has kept one form at every gap so far, the chain's image and reads at each checkpoint held to H-47's pinned row of that form; every fork's probe, score and the choice printed at each gap.
  - Twelve `…_paused` jobs, one a cell, and twelve `…_exhaustive` short forms, the twelve within 4 800 s together on x86-64 as the reproduction reads them alone.
  - `tests/inhibition/pins/h48.rs` with its placeholder tables: each cell's rows of the gated chain at the five checkpoints on the three paths, and each gap's two scores and its choice.
  - The verdict's test over hand-made rows, fast, and over the pinned tables once they are written.
- [ ] **H-48's protocol, in a new ADR at the next free number (`ls docs/adr`), kind `protocol`, committed before any arm runs.**
  - Every constant of the harness listed with its value, and each term of [H-48's report](../docs/research/H-48.md#its-terms) tied to the code that reads it.
  - `npm run round:additive`'s reading quoted, and the calibration it names.
- [ ] **The dispatches.**
  - The calibration and the twelve jobs together on the protocol's commit; the calibration read first.
  - The tables pinned from the jobs' logs, then the reproduction: the short forms at `scope=exhaustive -f only`, and the twelve jobs again at `scope=arms`.
- [ ] **H-48's verdict, in a new ADR at the next free number, kind `measured`.**
  - The verdict per network, start and clause in one table; each predicted reading held or missed in a line; the stopping rule's step reached and the next decision it names.
  - Whether the schedule's sweep of 2026-11-18 has ended, and its record if it has.
- [ ] **The documents, in the same pull request.**
  - [H-48's report](../docs/research/H-48.md) with its readings' tables, the gate's choices by cell, path and gap, and the dispatches.
  - `docs/STATUS.md` rewritten whole.
  - Whitepaper: Appendix C's M11 row, and the version in both declarations with its date.
  - The tables written by `node scripts/adr-index.mjs`, the cost table regenerated, and one `CHANGELOG.md` entry of at most 400 characters.
- [ ] **The brief archived** with `spec-brief archive 089 --strict`, every deliverable dispositioned. The round's ADRs are set to `accepted` by the commit that asks for the merge, and the pull request is marked ready for review with it.

---

## Not empowered

- **The round's scope** is the front matter's. It writes only inside `affectedFiles`, and never into `protectedFiles`. A file it must change outside the scope is handled as `briefs/README.md` says.
- **Nothing under `src/` or `crates/` changes**, and no record, section, format or named configuration.
- **No form of H-47's runs again**, and H-47's pinned rows do not change.
- **No constant of H-48's moves** after the protocol's commit, and no pinned number moves.
- **No probe, stream, tie rule, form, path or network** but ADR-0257's.
- **No derivation, lead-in or arm is run beyond the dispatches above**, and no whole-domain test runs on the developer machine.
- No float anywhere, no dependency, no `unsafe`, and no rebase of the branch once a dispatch has run on it.

---

## Architectural empowerment

The executing session may replace any instruction above with a better decision, recorded in its ADR. That covers:
- the names of the gated night's items, and how the form loop is shared with H-47's, so long as no test that ran before changes;
- how each gap's two forks run, one after the other or two at a time, within the job's bound;
- each short form's length, the twelve within 4 800 s together on x86-64;
- splitting a cell's job in two, within the job's bound;
- a file the round must change and its scope does not hold, within this brief's mission, widened as `briefs/README.md` says.

It may not:
- write into a protected file, or shrink `protectedFiles`;
- change the engine, the harness, or a named configuration;
- change H-48's clauses, constants or readings after the protocol's commit;
- run a derivation, a lead-in or an arm beyond the dispatches above, or any whole-domain test on the developer machine;
- reach the standing directives, the whitepaper's invariants or the constraints in `CLAUDE.md`.

If a gated chain that has kept one form at every gap is not H-47's pinned row of that form, or a dispatch moves a pinned number, the round stops there: the finding is written, and nothing is merged until it is understood.

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
# ci.yml on this round's branch: the calibration round:additive names, and inputs {"scope":"arms","tests":"<the twelve jobs>"}
# after the tables are pinned: inputs {"scope":"exhaustive","only":"<the twelve short forms>"} and {"scope":"arms","tests":"<the twelve jobs>"}
node scripts/exhaustive-costs.mjs from <the downloaded shards' artifacts> --run '<the calibration> and <the reproduction>' --keep scripts/exhaustive-costs.tsv > costs.new
mv costs.new scripts/exhaustive-costs.tsv
node scripts/adr-index.mjs
npx --no-install @spec-family/spec-brief archive 089 --strict --pr <n> --summary "<one paragraph>"
```

- The calibration is green before any job's log is read.
- The twelve jobs are green, every check inside them held, and their reproduction holds every pinned table.
- Every commit a dispatch ran on is an ancestor of the pull request's head, and no file under `runtime/`, `crates/` or `benches/` changes after the reproduction's commit.
- The archive passes `--strict`: nothing outside `affectedFiles` and nothing in `protectedFiles` changed.

---

## Report

The closing message states:
- the gated night as built, and that every chain that kept one form held H-47's rows;
- H-48's verdict per network, start and clause, and each predicted reading held or missed;
- the gate's choices, by cell, path and gap;
- each dispatch with its run id and commit, and each job's time against its bound;
- the regenerated cost table and the short forms' seconds;
- whether the sweep of 2026-11-18 had ended, and if so that an ADR records it;
- any widening of `affectedFiles`, and why;
- that no whole-domain test ran on the developer machine;
- that the pull request is ready for review;
- what was not done and why.
