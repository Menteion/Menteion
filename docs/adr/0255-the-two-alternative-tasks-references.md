---
status: proposed
date: 2026-11-14
depends-on: ADR-0254
title: The two-alternative task's references
kind: decision
summary: ADR-0254's first order. A round builds a search by forks in the test harness and reads the two-alternative task's references on the three reference networks, a floor by two simplest learners and a ceiling by the search, beside the engine on the same trials. No hypothesis is written.
decision-makers: Menteion maintainers
---

# ADR-0255: The two-alternative task's references

**In brief**
- **Decision.** Brief 088 builds a search by forks as a module of the test harness, with no new crate and nothing under `src/`, and reads the two-alternative task's references on the three reference networks. The floor is two learners that see only what a readout sees; the ceiling is the search over the four addressed couplings within H-25's bound. Both are read beside the engine on the same trials and schedule. No hypothesis is written and no constant moves.
- **Why.** [ADR-0254](0254-the-design-review-planned.md) planned a task's references with a search by forks first, and the maintainers chose (2026-11-14) to read them on the two-alternative task, where the answer is roughly known, so that the search is checked on it.
- **Left open.** The references of every other task, H-47's among them, which order 3 takes from this tool; a search over anything but the four couplings; whether the task is replaced, which the next decision weighs; the hidden-layer task's references.

## Context and Problem Statement

ADR-0254's first order asks what the two-alternative task asks of a learner: a floor, the simplest learner that sees what the readout sees, and a ceiling, the best the same network reaches with its weights found by a search that ignores locality. It also asks for the search itself, which the tree lacks.

A survey of the harness on 2026-11-14 found what the round can use and what makes the references hard to read:
- **The engine is near the task's top.** H-25's mappings score 116 to 124 of their last 128 trials ([ADR-0140](0140-the-address-drawn-measured.md)). The first mapping crosses 40 of 64 by block 4 to 6, a reversal by block 14 to 21. A read's spread at that level is 0.6 to 1.2 correct trials a block ([exploration 017](../explorations/017-what-a-read-spreads.md)).
- **What the floor sees decides it.** With the reward withheld, the two readouts' summed counts are near chance on the reference network ([ADR-0065](0065-the-instrument-recalibrated.md)'s calibration, 27 of 64 with 8 to 11 ties). Each readout unit's count may separate the stimuli, since some places hear only one of them. A feature that includes the stimulus units is the stimulus itself, so no floor may read one.
- **A search with no bound is trivial.** Static weights that favour each stimulus's answer should approach 64 of 64 on a network that does not learn, and cannot reverse. So the ceiling is read within H-25's clause 2 bound, 1.30 times each coupling of the image, one mapping at a time.
- **A withheld read is not frozen**: under H-25's image the inhibitory baseline keeps the inhibitory rule running. The round reads the floor's features and the ceiling under the same configuration with the reward withheld, and says so.
- **The pieces exist**: the task (`Task`, `run_on_presenting_from`, its per-trial `observe`), the critic's rule (`ValueCritic`, pure and public), forks by `Image::encode` once and `Image::decode` per candidate, the host's writes to weights (`Executor::blocks_mut` over a unit's fan-out), seeded generators (`Lcg`, `mix64`), and the three networks' start images (`derive`). Most are private functions of `tests/inhibition.rs`, so the round moves those it needs into the harness.
- **One read of 64 trials takes 5.6 to 6.1 s** on the AArch64 runners at 1 024 units ([exploration 036](../explorations/036-what-reaches-the-readout-units-on-both-cells.md)), so an arms job of 240 minutes holds about 4 000 such reads, two engines at a time.

## Decision Drivers

- **ADR-0254's order and the maintainers' choice** (2026-11-14): the two-alternative task, the tool checked where the answer is roughly known.
- **The search decides nothing**: what it finds is a reference, read again on streams it never saw; it is never a constant of a later round.
- **Determinism**: every search, floor and read repeats bit for bit from its seed.
- **One change a round**: the engine and its rules do not change; the round is tests and documents.

## Considered Options

1. **The two-alternative task's references, with the search built and checked on them.**
2. **H-47's task's references directly.**
3. **The search alone, checked on a synthetic objective.**

## Decision Outcome

**Option 1.** Brief 088 runs it.

### The search

- **Where it lives**: a module of the test harness, `runtime/menteion-runtime/tests/instrument/search.rs`, included by path as the harness is. No crate is added (§1.6's count stands), and nothing under `src/` changes.
- **What it takes**: named integer parameters, each with inclusive bounds; an objective, a whole-number score of a candidate read from forks of one quiescent image; and a seed.
- **What it does**, all in integers from `Lcg` seeded by the search's seed: a population of P candidates over G generations, generation 0 holding the image as it is; each candidate scored on S drive streams and ranked by the sum, ties by its index; the E best kept; the rest replaced by children of two parents drawn by a tournament of two, each gene taken from either parent, then moved with probability 1/4 by a step drawn from ±M and held within its bounds.
- **What it prints**: every candidate scored, with its generation, index, genes and score on each stream; then the best read again on S streams the search never saw.
- **Held by tests that run with the workspace's**: on a synthetic objective with a known best, the search reaches it within its budget; two runs from one seed print the same table; no gene leaves its bounds; every loop ends by construction.

### The references

- **The networks**: the start images of the three reference networks, seeds 22, 23 and 24, from which H-47 trains (`derive`), under the running configuration of [ADR-0233](0233-a-running-configuration-built.md), each engine built as H-47's harness builds it.
- **The schedule**: the first mapping for 24 blocks of 64 trials, then its first reversal for 24 blocks, on one drive stream for the engine and the floors.
- **The engine**: the configuration itself, rewarded, read on that schedule: correct trials a block, the block it first reaches 40 of 64 in each mapping, and the mean of each mapping's last eight blocks.
- **The floor**, two learners, each read on the same schedule and the same trials:
  - one that sees the two readouts' summed window counts;
  - one that sees each readout unit's window count, 918 of them.
  
  Each keeps one weight vector for each answer, answers the larger value, a tie being no answer and an error as in the task, and moves the chosen answer's weights by `ValueCritic`'s step on the error of its value against the reward. Its features come from the network with the reward withheld, so that the floor does all the learning.
- **The ceiling**: the search over four genes, the factor each addressed coupling's synapses are scaled by (stimulus A onto readout 0 and readout 1, B onto both), in units of 1/256 from 128 to 333, 0.5 to 1.30 times the image's, rounding each weight and holding it on the rail. Its objective is the correct trials of one withheld block on each of S = 3 streams, for the first mapping and, apart, for its reversal. P = 16, G = 10, E = 4 and M = 16: 480 reads a mapping. The zero sum's groups, which would scale a written coupling back, are set from the written image's own sums.
- **What is read**, network by network: each learner's and the engine's correct trials a block, the block each first reaches 40 of 64 and each mapping's last eight; the ceiling's best on the search's streams and on the three it never saw, with its genes; and whether each floor reaches H-25's clause 1 on this schedule, written as a reading. A floor that does is the reading that the task is too easy to rank a mechanism on, which ADR-0254's ladder weighs next.

### The round

- **Its tests**: three jobs, one a network, each the engine, the two floors and the two searches, named by network and kept out of the weekly sweep, as H-47's were; and a trial that times one generation on seed 22, from which each job's time is projected. If a projection passes an arms job's 240 minutes, G falls first, then P, and the brief says so.
- **Its runs** follow [`briefs/README.md`](../../briefs/README.md#evidencing-a-round): a calibration as `npm run round:additive` reads it, the three jobs at `scope=arms` on the protocol's commit, and their reproduction as a second dispatch at `scope=arms`, as H-47's long jobs were reproduced.
- **What it writes**: an ADR at the next free number that records the search built and the references read, its tables from the logs; `docs/STATUS.md`; the changelog.

## Consequences

- **Good**: the tree gains a search it can repeat bit for bit, checked where its answer is roughly known, and the two-alternative task gains the floor and ceiling every later reading on it can be set beside.
- **Good**: a floor that passes says, from a measurement, that the task cannot rank mechanisms, before one is measured on a harder rung.
- **Bad**: the ceiling covers four genes of one network's couplings, not every weight; a richer genome is a later round's.
- **Bad**: a withheld read is not frozen, so the ceiling and the floors' features carry the inhibitory rule's drift over a read.
- **Bad**: three jobs of up to four hours, and their reproduction.
- **Neutral**: no file under `crates/` or `src/` changes.

## Alternatives considered and why rejected

- **Option 2, H-47's task first**: its references need a search the tree has not yet checked anywhere, on a task of nights and two mappings that is longer and harder to read.
- **Option 3, the search alone**: a tool checked only on a synthetic objective says nothing about a task, and the two-alternative task's references are cheap beside it.

## Confirmation

- Brief 088 carries the round; its ADR records what was built and read.
- `npm run spec` holds the links, the tables and the version.
