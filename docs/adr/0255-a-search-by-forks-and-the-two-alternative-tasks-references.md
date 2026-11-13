---
status: proposed
date: 2026-11-13
depends-on: ADR-0254
title: A search by forks, and the two-alternative task's references
kind: decision
summary: ADR-0254's first order in two steps. Brief 088 builds a search by forks in the inhibition tests, held by fast tests, and measures nothing. Exploration 039 then uses it to read the two-alternative task's floor and ceiling beside the engine on H-46's first schedule.
decision-makers: Menteion maintainers
---

# ADR-0255: A search by forks, and the two-alternative task's references

**In brief**
- **Decision.** Brief 088 builds a search by forks and a writer of the four stimulus–readout couplings, a module of the `inhibition` test binary held by tests that run with the workspace's; it measures nothing and dispatches nothing. After it merges, exploration 039 uses them on H-42's three networks under the running configuration: two floors, simple learners that see only the readouts' counts, and a ceiling, the search over the four couplings within H-25's bound, beside the engine on H-46's first schedule from both assignments. No hypothesis is written and no constant moves. A session runs each when the maintainers say so.
- **Why.** [ADR-0254](0254-the-design-review-planned.md) planned a search by forks with a task's references first, and the maintainers chose (2026-11-13) the two-alternative task, where the answer is roughly known, so that the search is checked on it. A reference predicts nothing, and a question with no prediction is an exploration ([ADR-0193](0193-an-exploration-lane.md)); the tool, which order 3 takes for H-47's task, enters `main` by a round.
- **Left open.** The references of every other task, H-47's among them; a search over anything but the four couplings; a reference that is evidence, which a round reads under its own protocol; whether the task is replaced, which the decision after exploration 039 weighs; the hidden-layer task's references.

## Context and Problem Statement

ADR-0254's first order asks what the two-alternative task asks of a learner: a floor, the simplest learner that sees what the readout sees, and a ceiling, the best the same network reaches with its weights found by a search that ignores locality. It also asks for the search, which the tree lacks, and leaves open where it lives and how it runs under [ADR-0184](0184-the-runs-off-the-developer-machine.md).

The maintainers chose, as ADR-0254 records, to run a search by forks with a task's references as the first round; this ADR takes that choice in a round and an exploration, as ADR-0193 sends a question with no prediction to the exploration lane, and the maintainers see the change before either session starts.

This ADR's first draft had one round build the search and pin the references. Two reviews on 2026-11-13 found that it named H-33's derivation where the running configuration trains H-42's networks, cited H-25's speeds for an engine that is H-46's arm, specified floors that would never move, and asked a protocol, a calibration and a reproduction of readings that predict nothing.

What the tree holds for it, read at `main` `7d8496a`:
- **The engine is near the task's top.** The running configuration is H-46's arm ([ADR-0233](0233-a-running-configuration-built.md)). On H-20's schedule ([ADR-0109](0109-a-schedule-of-reversals.md)), a first mapping of 24 blocks and three reversals of 32, its lowest of any mapping's last 128 trials over sixteen schedules was 122 to 125, and its longest reversal 10 to 13 blocks ([H-46](../research/H-46.md)). In its first schedule from H-42's image the first mapping passed 40 of 64 by block 1 to 7, the first reversal 9 to 13 blocks after its flip (`ZERO_ROWS` in `tests/inhibition/pins/h46.rs`). A read's spread at that level is 0.6 to 1.2 correct trials a block ([exploration 017](../explorations/017-what-a-read-spreads.md)).
- **The readouts' own selection is not always at chance.** The frozen block of each network's derivation, with nothing learned and under the derivation's calibration rather than the running configuration, reads 31, 20 and 47 of 64 correct under the assignment, with 9, 5 and 1 ties (`PROBE_FROZEN` in `tests/inhibition/pins/h42.rs`), so 24, 39 and 16 under the mirrored assignment. Seeds 23 and 24 lean to one mapping; seed 22 is near chance. A floor that sees the two summed counts may pass 40 of 64 on one mapping with little learning, and each readout unit's count may separate the stimuli further. A feature that includes a stimulus unit is the stimulus itself.
- **A search with no bound is trivial.** Static weights that favour each stimulus's answer should approach 64 of 64, and cannot reverse. So the ceiling is read within H-25's clause 2, no coupling past 1.30 of its image's, read as coupling × 100 ≤ image × 130, one mapping at a time. In units of 1/256 the largest factor within it is 332; the lower bound, 0.5, is this ADR's.
- **A withheld read is not frozen.** With the reward withheld the inhibitory rule runs at its baseline: the inhibitory sums fell 3.5 to 4.4 % over 16 blocks after a first mapping, the rate's rule moved the gain, and the couplings moved a little (exploration 017; [exploration 036](../explorations/036-what-reaches-the-readout-units-on-both-cells.md)).
- **The pieces exist**: `Task` and its flips; `run_on_presenting_from` with its per-trial `observe`; `ValueCritic`, pure and public, with `made.critic`'s constants, a shift of 9 and a scale of 2 on all three networks; forks by `Image::encode` of a quiescent engine once and `Image::decode` a candidate; writes by `Executor::blocks_mut` over a unit's `fan_out`; `Executor::coupling_sum` for the zero sum's groups, which the host sets at each load; `Lcg` and `mix64`; and `probe_made`, H-42's image of each network. `probe_made` and the helpers that run H-46's arm are private to `tests/inhibition.rs`, and `zero_groups` asserts H-42's couplings, so a written engine's groups are set from its own sums.
- **Cost.** A read of 64 trials takes 5.6 to 6.1 s on the AArch64 runners at 1 024 units, two engines at a time (exploration 036). An arms job's bound is 240 minutes, and [ADR-0179](0179-the-bounds-rule-restated.md)'s rule holds a job to 60 per cent of it, 144 minutes.

## Decision Drivers

- **ADR-0254's order and the maintainers' choice** (2026-11-13): the two-alternative task, the tool checked where the answer is roughly known.
- **ADR-0193**: a question with no prediction is an exploration, and its note decides nothing.
- **The search decides nothing**: what it finds is a reference, read again on streams it never saw, and never a constant of the engine or of a later round.
- **Determinism**: every search, floor and read repeats bit for bit from its seed.
- **The engine does not change.**

## Considered Options

1. **The tool by a round, the references by an exploration.**
2. **One round that builds the tool and pins the references**, this ADR's first draft.
3. **H-47's task's references first.**
4. **The tool alone, checked on a synthetic objective.**

## Decision Outcome

**Option 1.**

### Brief 088: the search (Specified)

- **Where it lives**: `runtime/menteion-runtime/tests/instrument/fork_search.rs`, a module the `inhibition` binary includes by path, as it includes the harness; `search` is a name the tree already holds. No crate is added, so §1.6's count stands; nothing under `src/` changes and no helper leaves `tests/inhibition.rs`. Brief 088 adds no long read: those that use the tool run as `#[ignore]`d tests by dispatch (ADR-0184), exploration 039's first.
- **What it takes**: genes, each a named integer with inclusive bounds and a start value; an objective, a whole-number score of the genes on a stream, which reads its candidate from a fork; P, G, E, M, S and a seed.
- **What it does**, in integers from `Lcg` seeded by the seed:
  - generation 0 holds the start genes and P − 1 candidates drawn uniformly within the bounds;
  - each new candidate is scored on streams 0 to S − 1 and ranked by the sum, ties by its index in the generation, the kept first in their rank and the children in the order made;
  - the E best are kept with their scores and not read again;
  - each other place takes a child of two parents, each drawn by a tournament of two members drawn uniformly, the better ranked taken. Each gene comes from either parent with probability 1/2, then moves with probability 1/4 by a step drawn uniformly from −M to M, held within its bounds;
  - P + (G − 1)(P − E) candidates are scored, and the best of the last generation is read again on streams S to 2S − 1.
- **What it prints**: every candidate scored, with its generation, index, genes and score on each stream; then the best's scores on the streams it never saw.
- **The writer** scales each synapse that `coupling` counts, from an excitatory unit of one stimulus's set onto any unit of one readout's set, by a factor in units of 1/256, widened to `i64`, rounded to the nearest with a half away from zero and held within ±32 767. At 256 it leaves the engine as it was.
- **Held by tests that run with the workspace's**:
  - on synthetic objectives whose best is known, one at a corner of a small box and one inside it, the search reaches it from the seed the test fixes at P = 16, G = 10, E = 4, M = 16 and S = 3;
  - one seed prints the same table twice, and another seed another;
  - no gene leaves its bounds, and the count of candidates scored is P + (G − 1)(P − E);
  - on a small engine the writer at 256 leaves its image byte for byte, and at 128 and 332 writes each synapse of the coupling as the rounding says and no other synapse.
  
  Every loop ends by construction.
- **Its runs**: it measures nothing, pins nothing and dispatches nothing. `npm run round:additive -- origin/main <its last commit>` exits 0, so no test the sweep runs has changed, and the pull request's gate is its evidence; a change that cannot only add stops the round. Its ADR records the tool built.

### Exploration 039: the references (Specified; it decides nothing)

It runs after brief 088 merges, on `explore/039-the-two-alternative-tasks-references` from `main`, and writes no prediction.
- **Its networks** are H-42's three, seeds 22, 23 and 24 at 1 024 units, each from `probe_made`'s image under the running configuration, the zero sum's groups set at each load as H-46's arm sets them.
- **Its schedule** is H-46's first schedule, H-20's: the first mapping for 24 blocks, then three reversals of 32 blocks, 120 blocks from the assignment and 120 from the mirrored assignment, on H-46's trials and drive stream.
- **The engine** is H-46's arm on that schedule, rewarded, run as `zero_short` runs it. Its row holds H-46's pinned row of its network and start (`ZERO_ROWS`): the trace, the reward values and the image by CRC-64, each mapping's last 128 and the block it passed 40 of 64. A row that does not hold stops the exploration.
- **The floors' features** come from one run of the 120 blocks a network with the reward withheld. With the reward withheld only a trial's correctness depends on the mapping, so the one run serves both starts, and a check holds, trial by trial, that it presents each engine run's stimuli. On each trial the features are the two readouts' summed window counts and each of the 918 readout units' window counts. A check holds that no stimulus unit is in a readout's set.
- **The floors** are two learners on each start, each reading every trial's features in order:
  - one sees the two summed counts, the other each readout unit's count;
  - each keeps a vector of `i16` weights for each answer, at first 16 384 on the features of the answer's own readout and 0 elsewhere, so that it starts as the readouts' own selection;
  - it answers the larger of `ValueCritic::value_q16` over its two vectors at `made.critic`'s constants, a tie being no answer and an error, as the task counts one;
  - after an answer it moves that answer's weights by `ValueCritic::step` on the error of `REWARD_Q16` for a correct answer, or its negation for a wrong one, against that answer's value. After no answer nothing moves.
- **The ceiling** is the search over four genes, the factors of the four couplings (A onto readout 0 and onto readout 1, B onto both), from 128 to 332 and starting at 256:
  - each candidate is written into a fork of the start image by brief 088's writer, the zero sum's groups set from the written engine's `coupling_sum`;
  - its objective is the correct trials of one block with the reward withheld on stream s, that block's first trial 64 × s and its drive seed the schedule's plus s, as `nights_read_from` sets a read's start. The search's streams 0 to 2 are streams 1 to 3, and its unseen streams 4 to 6. A read's counts give its correct trials under either mapping, so a read is kept by its genes and stream and serves both searches;
  - at each read's end the four couplings are read against the start image's, and a pass of 1.30 is printed beside the score;
  - it runs for the assignment's mapping and apart for the mirrored one's, each on the start image, at P = 16, G = 10, E = 4 and M = 16, with a seed fixed before the trial;
  - the box's sixteen corners are read on streams 1 to 6; their best on streams 1 to 3 is set beside the search's best, on those streams and on 4 to 6.
- **Its trial and jobs**: a trial times one generation of one search and one start's 120 blocks on seed 22, and each job's time is projected from it before any job runs, two engines at a time. Past 144 minutes G falls first, then P, and the note says so. Three jobs, one a network, each runs the engine from both starts, the withheld run, the floors, the two searches and the corners. The trial and the jobs end in `_explore`.
- **It reads**, network by network and start by start, beside exploration 017's spread and with no SE of its own:
  1. each floor's and the engine's correct trials a block, each mapping's last 128, and the block each first passes 40 of 64 after each flip;
  2. whether each floor holds H-25's clauses 1 and 3 on this schedule, as the engine did, and how often a floor's weight sat on a rail;
  3. for each mapping, the ceiling's best genes and their scores on the search's streams and on the three unseen ones, beside the corners' best and the engine's and each floor's last 128, every score given per 64 trials.
- **It does not read** any other task, any gene but the four, a floor that sees a stimulus unit or the critic's window, or the engine under any other configuration. Nothing under `src/` changes; if the read needs a change there, the exploration stops and says why.

## Consequences

- **Good**: the tree gains a search that repeats bit for bit, held by its tests on `main`, for order 3 and the references after it.
- **Good**: the two-alternative task gains a floor and a ceiling beside the engine, read where the engine's run is pinned.
- **Good**: the round is short and its gate is its evidence; the long reads are three exploratory jobs, each projected at about an hour, two engines at a time.
- **Bad**: a reference read by an exploration is not evidence. A later ADR cites it only in its Context, and a round that sets a verdict beside a reference reads it again under its own protocol.
- **Bad**: the ceiling covers four genes, not every weight, and the box's lower bound is this ADR's.
- **Bad**: a withheld read is not frozen, so the floors' features and the ceiling carry the inhibitory rule's drift.
- **Neutral**: no file under `crates/` or `src/` changes.

## Alternatives considered and why rejected

- **Option 2, one round**: a protocol, a calibration and a reproduction would hold readings that judge nothing, and ADR-0193 gives a question with no prediction its own lane.
- **Option 3, H-47's task first**: its references need a search the tree has not checked anywhere, on a task of nights and two mappings that is longer and harder to read.
- **Option 4, the tool alone**: a tool checked only on a synthetic objective says nothing about a task, and the maintainers chose to check it on one.

## Confirmation

- Brief 088 carries the round, and its ADR records the tool built.
- Exploration 039's note, when it merges, holds what it read.
- `npm run spec` holds the links, the tables and the version.
