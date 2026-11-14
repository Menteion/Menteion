---
status: accepted
date: 2026-11-15
depends-on: ADR-0257
amends: [ADR-0223]
title: Development paused after a closing round
kind: decision
summary: The maintainers pause Menteion's development after this closing round. Exploration 040 and H-48 are parked unrun and the weekly schedule is suspended. A kernel is ported to Mente around its M5; the engine resumes, in Mente or by a reopen condition in Rust, only by an ADR.
decision-makers: Menteion maintainers
---

# ADR-0258: Development paused after a closing round

**In brief**
- **Decision.** After this closing round, no new round, ADR, brief, hypothesis, exploration or build starts in Menteion until an ADR resumes or reopens its development. The tree is frozen at this ADR's merge, tagged `menteion-paused-2026-11-15`; its code is `58228c9`'s. Exploration 040 is parked unrun at its harness, brief 089 stays live and H-48 open, and the weekly schedule is suspended. [A document for Mente](../mente-port.md) names the kernels it may port and what the engine asks of a language. The restart is staged: a kernel in Mente around Mente's M5; after Mente's M7 the maintainers weigh resuming the engine; a reopen condition can resume it in Rust sooner.
- **Why.** The maintainers' model quota and decision time are shared with Mente. A mechanism built in Rust now would be ported again, while research verdicts and ADRs carry over in any language. A frozen engine is a stable workload, and a bit-for-bit reference for Mente's port.
- **Left open.** The ADR that resumes or reopens; the date in the slip condition; and the way of working at the restart, of which this ADR records a review as a proposal.

## Context and Problem Statement

On 2026-11-15, in the session that manages Mente, the maintainers decided to pause Menteion's development after one closing round, done by Menteion's own sessions. They confirmed it in this repository's session the same day, with three choices: exploration 040 parked, brief 089 and H-48 parked, and the weekly schedule suspended.

The facts they weighed:
- the weekly model quota is shared by all the maintainers' projects, and was 55 % used seven hours after its reset; their own time for decisions is shared too;
- Mente, a systems language, is at the second of its milestones M0 to M9. Records of fixed layout come at its M3, fixed point and its hardware contracts at M5, and workers and concurrency at M7. Its register already names a Menteion kernel as its systems workload;
- every mechanism added to the engine in Rust now would be ported again later, while research verdicts and ADRs carry over whatever the language;
- the organization's hosted runners are one pool for both repositories.

**Where the work stands.** [ADR-0257](0257-a-night-kept-by-the-hosts-probe.md) merged with H-48 written before any run and predicted no. Exploration 040's harness was pushed and its trial dispatched, run [43621107487](https://github.com/Menteion/Menteion/actions/runs/43621107487). M11 is open after eighteen explorations since H-47 ([ADR-0237](0237-learning-that-sleep-keeps-measured.md)). [ADR-0254](0254-the-design-review-planned.md)'s plan has done its first order and written its third.

## Decision Drivers

- **The maintainers' decision and choices** (2026-11-15).
- **The repository wins over the document**: what the pause parks, what keeps running and how to resume are in the tree, not in anyone's memory.
- **A frozen tree is a reference**: its pins and determinism tests are what a port is held to, so the tree must not move under it.
- **Latest ≠ Newest** is unchanged, and so is every other principle of `CLAUDE.md`.

## Considered Options

1. **The pause**: (a) after a closing round; (b) at once, with nothing written; (c) no pause.
2. **The work in flight**: (a) parked, each with how to resume it; (b) finished first.
3. **The weekly schedule**: (a) suspended; (b) monthly; (c) kept.
4. **The restart**: (a) staged, a kernel in Mente around its M5, the engine weighed after its M7, and reopen conditions in Rust; (b) only when Mente can develop Menteion; (c) on a date, in Rust.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**

### What pauses, and what may still merge

From this ADR's merge no new round, ADR, brief, hypothesis, exploration or build starts. Three kinds of change may still merge, each by a pull request of its own:
- the ADR that resumes or reopens development;
- the ADR that records the maintainers' signature of the restart below, with Mente's register;
- a change Mente's port needs that changes no behaviour of the engine: a test that pins a kernel already in the tree, or a correction to [docs/mente-port.md](../mente-port.md), which then names it.

**The frozen tree** is this ADR's merge commit, tagged `menteion-paused-2026-11-15`. Its files under `crates/`, `runtime/`, `benches/` and `testkit/` are `58228c9`'s: this round changes documents and `.github/workflows/ci.yml` alone.

### What is parked

- **Exploration 040** (ADR-0257): its harness is at `f15724d`, on the tag `explore/040-what-a-readout-and-the-couplings-keep`. Its trial, run 43621107487, passed on that commit in 1 352 s and projected a whole job at 85 minutes; its twelve jobs were not dispatched and no note was written. To resume, its jobs are dispatched from the tag as ADR-0257 specifies.
- **Brief 089 and H-48** (ADR-0257): the brief stays live and H-48 open. Its directive on the sweep of 2026-11-18 is stale, since that sweep does not run. The ADR that resumes weighs whether it runs as written or is decided again.
- **ADR-0254's plan**: order 1 is done, order 3 is written and parked, orders 2 and 4 to 7 have not started. ADR-0257 dropped exploration 038 in favour of H-48.
- **A question of [ADR-0225](0225-a-unit-served-on-arrival-built.md)**: what a read of the arena on every tick should pay, since it catches every asleep unit up a tick.
- **The milestones**: M11 is open; M13, M14, M8's remainder and M15 have not started (whitepaper Appendix C). M7 waits for a machine of the reference platform.
- **Findings open**: F-54, F-90 and F-91 ([ADR-0254](0254-the-design-review-planned.md)), and F-92, recorded here.
- **The weekly sweep**: the last recorded ran on 2026-11-11 ([ADR-0251](0251-what-reaches-the-readout-units-on-both-cells.md)); 2026-11-18's does not run.

### What keeps running

- **Nothing on a schedule.** `.github/workflows/ci.yml` keeps its triggers on a push to `main`, a pull request and a dispatch, which run only when someone pushes, opens a pull request or dispatches. Its weekly schedule is removed, and a comment says how the resuming ADR restores it.
- **No scheduled task on the developer machine is Menteion's alone.**

### The restart: a draft for the maintainers to sign

The same conditions go into Mente's register, and the maintainers sign the two together, by an ADR of the second kind above. Until then this is a draft.
1. **A kernel in Mente, around Mente's M5.** A kernel of [docs/mente-port.md](../mente-port.md) is ported as Mente's systems workload, held bit for bit to the tests that document names at the frozen tree. It is Mente's work; it changes nothing here but by the second kind of change above.
2. **The engine weighed after Mente's M7.** The maintainers decide, by an ADR, whether the engine resumes in Mente, resumes in Rust, or stays paused.
3. **Reopened in Rust sooner**, by an ADR, when any of these holds:
   - a research question the maintainers judge cannot wait for Mente, named in that ADR;
   - Mente's M5 is not reached by a date the maintainers set when they sign;
   - a defect in the frozen tree that Mente's port depends on, which a change of the second kind cannot meet.
4. **What resuming does first.** The resuming ADR restores the weekly schedule in `ci.yml` and the whitepaper's directive. It calibrates the whole tree at `scope=exhaustive` on its own commit before any round, on whatever toolchain `rust-toolchain.toml` then pins. It rewrites `docs/STATUS.md` and reads the parked list above.

**No research question is held to need a reopening now.** The one round ready to run, H-48, is predicted no; what waits on its verdict is the order of ADR-0254's remaining orders, and that waits with it.

### The way of working at the restart: a proposal

A review of Mente's practices against Menteion's, for the ADR that resumes. It decides nothing now.
- **Menteion already has**: hypotheses and stopping rules committed before any run; the mutation gate on a change's lines and the weekly sweep ([ADR-0030](0030-verification-governance.md)); oracles for the executor, on-arrival service against per-tick service ([ADR-0225](0225-a-unit-served-on-arrival-built.md)), one against two and four workers, and x86-64 against AArch64; every change carrying its checks for code (`CONTRIBUTING.md`'s definition of done, `spec:prove`); a checked exploration lane ([ADR-0193](0193-an-exploration-lane.md)); a declared scope a brief ([ADR-0191](0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)); accepted ADRs frozen byte for byte ([ADR-0197](0197-one-place-for-each-kind-of-fact.md)).
- **Worth adopting at the restart**:
  1. a guard against the class of each problem found. Above all for readings: the scripts that generate and check an exploration's tables live in each session's scratch space, so the same slips recur, an exact half rounded down, a line too long, "every" read from one line. They would move into `scripts/` with tests;
  2. a reopen condition in each decision and process ADR, one line that `spec:shape` checks;
  3. the review by a reader with fresh context written into `CONTRIBUTING.md`, with the rules only review holds;
  4. complete required checks: the mutation gate and the AArch64 job required, a round's merge conditions checked in CI rather than by a script on the developer machine, and a time limit per test, so that a timeout is not counted as a caught mutant.
- **Would slow research**: a merge queue, since two code pull requests are rarely open together and the runners are shared; a second, independent engine as an oracle, since the dynamics are held by the exhaustive tests; isolation of untrusted test code, since nothing here is adversarial.

### The clean-up

- **Worktrees**: sixteen of the developer machine's eighteen, each clean and contained in `main` or in a tag, were removed. Left: the main checkout and this session's.
- **Branches**: on GitHub, `main` alone once this merges; exploration 040's branch is already its tag. On the developer machine, 175 local branches contained in `main` or in a tag were deleted. Left are 85 from rounds merged by rebase before [ADR-0186](0186-merge-commits.md), whose content is on `main` under other hashes, for the maintainers to keep or delete.
- **Sessions**: exploration 040's is archived; this session closes after the merge and the tag.

### Findings

- **F-92**: the runtime crate's description and a comment in `runtime/menteion-runtime/Cargo.toml` name work-stealing deques. [ADR-0102](0102-the-sweep-timed-with-no-census.md) replaced them with a schedule bitmap, ADR-0100's design, and [ADR-0023](0023-executor.md) carries its banner. **Open**: the description changes with the resuming ADR, since this round changes no file under `runtime/`.
- **F-93**: §8.5 stated epoch-based reclamation in the present tense, with no label, while the executive summary and [ADR-0011](0011-epoch-based-reclamation.md) give it as Specified and no code holds it. **Resolved** here: §8.5 labels it Specified.

## Consequences

- **Good**: the quota and the runners go to Mente, and nothing built in Rust now is ported again.
- **Good**: what is parked, what runs and how to resume are in the tree, with a frozen point a port can be held to.
- **Bad**: M11's question waits, with H-48 written and unrun; the verdicts the plan would have read wait with it.
- **Bad**: no sweep reads the tree while it rests, so a runner's change that breaks it is found only when development resumes.
- **Neutral**: the research's ADRs, reports and pins stand as they are, and carry over whatever the language.

## Alternatives considered and why rejected

- **Option 1(b), at once**: what is parked and how to resume would be in nobody's tree.
- **Option 1(c), no pause**: it keeps spending a shared quota on mechanisms a port would build again.
- **Option 2(b), finished first**: exploration 040's jobs and note, and H-48's round, would spend most of the quota the pause saves, for readings only the parked plan waits on.
- **Option 3(b) and 3(c)**: a tree that does not change gains little from a weekly run, which takes runners from Mente.
- **Option 4(b), only when Mente can develop Menteion**: that is Mente's stage 3, far off, with no earlier step.
- **Option 4(c), a date in Rust**: it ties the restart to the calendar and not to what Mente can carry.

## Confirmation

`docs/STATUS.md` says the work is paused. `.github/workflows/ci.yml` holds no schedule, and the whitepaper's directive holds that. [docs/mente-port.md](../mente-port.md) is the document for Mente. `npm run spec` holds the links, the tables and the version.
