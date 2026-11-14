---
status: proposed
date: 2026-11-14
depends-on: ADR-0254
title: A night kept by the host's probe, and H-47's task's references
kind: decision
hypothesis: H-48
summary: ADR-0254's third order. H-48 asks on H-47's twelve cells whether nights the host keeps by a probe read after the next stretch keep the first mapping at least as well as nights without replay. Exploration 040 reads H-47's task's references; exploration 038 is replaced.
decision-makers: Menteion maintainers
---

# ADR-0257: A night kept by the host's probe, and H-47's task's references

**In brief**
- **Decision.** Nothing in the engine changes. At each of H-47's four gaps the host runs both of H-47's nights in forks, the night without replay and the plain night on A's patterns. Each fork runs the pair's next stretch, both are read on three streams H-47's reads never use, and the fork that reads more is kept, the plain night on a tie. H-48 asks, on H-47's twelve cells, whether such gated nights keep A at least as well as nights without replay in every cell, and more in at least ten. It is predicted yes. Brief 089 builds the gate in the tests and runs H-48 once. Exploration 040 reads H-47's task's references: exploration 039's floor on each readout unit's count, and a ceiling by the search over A's four couplings. Exploration 038 is replaced by H-48 and does not run.
- **Why.** ADR-0254 makes a night kept by a gate the host runs M11's next decision, with order 1's tool for H-47's task's references. Explorations 030 to 032 read that a replayed night's cost can appear after the next blocks, not right after the night.
- **Left open.** DR-25, the replay's order set by the host's tags, order 3's other candidate; chained episodes compiled in sleep (UPG-04); whether the slow-wave baseline enters the running configuration; the order of the plan's orders 4 to 7, which an ADR sets after H-48's verdict.

## Context and Problem Statement

[ADR-0254](0254-the-design-review-planned.md)'s third order asks of M11 whether a night kept only when the host's probe shows retention keeps a mapping better than an ungated night and a night without replay, on what the probe did not see. It names DR-24, a gate the host runs, and DR-25 as candidates, puts order 1's tool for H-47's task's references first, and leaves exploration 038 to this ADR. The probe and the mappings are the host's; the engine's rules and the policy amendment do not change.

**What H-47 read** ([ADR-0237](0237-learning-that-sleep-keeps-measured.md)): plain nights on A's patterns kept more of A than nights without replay at block 36, beyond twice the paired SE, in ten of twelve cells, by +2.2 ± 0.65 to +19.6 ± 1.06. They did not on seed 23's first mapping from the mirrored assignment, −4.9 ± 1.09, below, or on seed 24's eighth image from the assignment, +1.8 ± 1.79, within. The pair stood at 61.8 to 63.4 under plain nights, and A's highest coupling under any form at 1.141 to 1.223 of H-42's image's.

**When a night's cost shows.** Explorations 030 to 032 read single replayed nights against the night without replay, each from the night without replay's chain. Their readings are exploratory, not evidence:
- on seed 23's first mapping from the mirrored assignment, right after the night, the night after block 6 alone read +1.9 ± 0.68 and the night after block 12 alone +1.6 ± 0.55, both above beyond 2 SE, while H-47's four plain nights read −4.9 at block 36 and leaving any one of them out shrank the loss ([exploration 030](../explorations/030-one-cell-night-by-night.md));
- on seed 24's eighth image from the assignment, H-47's plain night at block 6 stood +0.3 ± 0.64 over the night without replay right after it, within 2 SE, and the form that replays on that night alone −2.0 ± 0.88 at block 36 ([exploration 031](../explorations/031-the-first-nights-replay-on-twelve-cells.md)); the nights after blocks 12, 18 and 24 alone read −7.3 ± 0.88, −12.1 ± 0.63 and −12.3 ± 0.58 right after ([exploration 032](../explorations/032-each-later-nights-replay-alone-on-twelve-cells.md));
- right after the night, single replayed nights stood above the night without replay beyond 2 SE in eight, seven, seven and six of the twelve cells after blocks 6, 12, 18 and 24.

A probe read right after each night would keep the first cell's early nights. Read after the next stretch of the pair's blocks, it reads part of what follows.

**What a fork costs.** The nights already run on forks: each of H-47's forms decodes the image the pair's stretch wrote, and each of H-47's reads is a fork. One H-47 job ran two forms on three paths in 61.2 to 68.6 minutes on AArch64 ([H-47's report](../research/H-47.md#the-dispatches)).

**The references.** On the two-alternative task a learner on each readout unit's count kept pace with the engine, and the best of the coupling box's corners stood above the search's best ([exploration 039](../explorations/039-the-two-alternative-tasks-references.md)). H-47's task has no reference yet.

## Decision Drivers

- **The maintainers' choices** (2026-11-14): a rejected night replaced by the night without replay, not by the image as it was; the probe read after the next stretch; H-48 as a round with exploration 040's references beside it; for the references, exploration 039's floor on each readout unit's count and a ceiling by the search over A's four couplings; exploration 038 replaced by H-48.
- **H-47's stopping rule**, step 8: no constant of H-47 moves and nothing of it runs again. H-48 is a new hypothesis on H-47's cells, with its own constants committed before any run.
- **The host's side** (ADR-0254): the gate is the host's reading of its own mappings on forks. It is neither the policy amendment's trial, which commits only on equal behaviour hashes, nor a gate the engine sets on its own amendments.
- **One change a round**: the gate alone; DR-25 waits.
- **Latest ≠ Newest**: the tree's own code, and no dependency.

## Considered Options

1. **The rejected night**: (a) the night without replay, both nights run in forks; (b) the image as it was, DR-24's text; (c) episode by episode.
2. **The probe's time**: (a) after the next stretch of the pair's blocks; (b) right after the night; (c) both, one a reading.
3. **The order**: (a) H-48 as a round, exploration 040's references beside it; (b) an exploration of the gate first, a hypothesis after; (c) H-48 alone.
4. **The references**: (a) exploration 039's floor on each readout unit's count, and the search's ceiling over A's four couplings; (b) those and the best of the sixteen ways to keep or reject four nights, read after the fact.
5. **Exploration 038**: (a) replaced by H-48; (b) paused until H-48's verdict; (c) run.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).**

### The gated night (Specified; brief 089 builds it in the tests)

On each of H-47's paths, from H-47's start image, the pair's head of 6 blocks runs as H-47's does. Then at each gap, after blocks 6, 12, 18 and 24:
- **Two forks.** From the image the pair's stretch wrote, H-47's night without replay and its plain night on A's patterns each run, as H-47's forms run them.
- **Each fork's next stretch.** Each fork runs the pair's next stretch as H-47's forms do: 6 blocks after the gaps at blocks 6, 12 and 18, and the 12 blocks to block 36 after the gap at block 24.
- **The probe.** Each fork's image after its stretch is read on streams 16, 17 and 18 (`nights_read_from`), one read of A and one of the pair on each, the reward withheld, as H-47 reads. Its score is the sum of the six reads' correct trials, of 384.
- **The choice.** The plain night's fork is kept when its score is at least the night without replay's; otherwise that fork is kept. The kept fork's night image is the gated chain's image at the gap, and its stretch's image the chain's from there.

The probe reads no image H-47 reads at blocks 6 to 24, and none of H-47's sixteen streams at any block. The gap at block 24's probe reads the image at block 36, on its own three streams.
- **Its reads**: H-47's, at blocks 6, 12, 18, 24 and 36, sixteen of A and sixteen of the pair at each checkpoint, on the gated chain's images.
- **Its comparisons**: H-47's pinned rows of the night without replay and of the plain night on A's patterns, on the same paths and streams, paired by path and stream ([H-47's terms](../research/H-47.md#its-terms)). No form of H-47's runs again.
- **Checked inside the test**: while the gate has kept one form at every gap so far, the chain's image and reads at each checkpoint are H-47's pinned row of that form. A gated chain that is not stops the round as a finding.
- **Twelve `…_paused` jobs**, one a cell, its three paths two at a time; from H-47's jobs, under 60 minutes each on AArch64.
- **A short form of each cell** is a weekly `…_exhaustive` test. On path 0 it runs the head, both nights at block 6, each fork's stretch and the probe, held to the long run's pinned choice and images. The twelve take at most 4 800 s together on x86-64, as the reproduction reads them alone.
- **The reproduction**: the short forms at `scope=exhaustive`, and the twelve jobs again at `scope=arms` on the reproduction's commit, every pinned table held, as H-47's were.

### H-48 (Hypothesis; written before any run)

The text, with its clauses, readings, terms and stopping rule, is [H-48's report](../research/H-48.md).
- **Clause 1**: in each of the twelve cells, A's mean count at block 36 under gated nights is not below that under nights without replay by more than twice its paired SE.
- **Clause 2**: in at least ten of the twelve cells, A's mean count at block 36 under gated nights exceeds that under nights without replay by more than twice its paired SE.
- **Clause 3**: in each cell, the pair's mean count at block 36 under gated nights is at least 56 of 64.
- **Clause 4**: in each cell, none of A's four couplings passes 1.30 of H-42's image's on its network. It is read at each block's end of every stretch the gated chain runs, the forks the gate does not keep among them, and at each image read.
- **Yes** when all four hold; otherwise **no**, with the network, the start and the clause.

A mean count and the paired SE are H-47's, of 48 reads, three paths by sixteen streams.

**Predicted: yes.** In the ten cells where H-47's plain night kept more at block 36, it kept more at block 24 too, and single replayed nights stood above beyond 2 SE right after them in six to eight cells a night: the gate is expected to keep most plain nights there. Where a replay cost A, the probe after the next stretch is expected to read the cost and keep the night without replay. The prediction rests on explorations 030 to 032, whose single nights ran from the night without replay's chain and not from a gated one.

**Four readings are predicted:**
1. on seed 23's first mapping from the mirrored assignment, A's mean count at block 36 under gated nights over that under plain nights, positive;
2. A's mean count at block 36 under gated nights not below that under plain nights by more than twice its paired SE, in at least 8 of the 10 cells where H-47's clause 1 held;
3. the plain night kept at more than 72 of the 144 gaps, twelve cells by three paths by four gaps;
4. on seed 24's eighth image from the assignment, the night without replay kept at at least 7 of its 12 gaps.

**What would refute it**: a cell where gated nights keep less of A than nights without replay; fewer than ten cells where they keep more; the pair unlearned; a coupling past 1.30.

### H-48's stopping rule

1. **One round**: brief 089 builds the gated night in the tests, with its ADR, then runs H-48 once. Its clauses, constants and readings are committed before any arm runs.
2. **A calibration stops the round**, and is a finding, if a pinned number moves. So does a gated chain that, where the gate has kept one form at every gap so far, is not H-47's pinned row of that form.
3. **Yes**: M11's exit test reads yes. The next decisions are whether M11 closes and whether the slow-wave baseline enters the running configuration.
4. **No on clause 4**: M12 reopens ([ADR-0232](0232-a-running-configuration-and-m12-closed.md)), and an ADR on the zero sum through the form that crossed.
5. **No on clause 1**: an ADR on what the probe after one stretch does not see on that network.
6. **No on clause 2**: an ADR on what the gate gives up where replay kept more.
7. **No on clause 3**: an ADR on what the gated nights cost the pair.
8. **Where several steps apply**, the first in this order is taken. After any verdict, an ADR orders the plan's orders 4 to 7 beside M13, M14, M8's remainder and M15, as ADR-0254 says.
9. **No constant moves after such a run.** There is no second attempt, and no other probe, stream, tie rule, form, path or network.

### Exploration 040 (Specified; it decides nothing)

H-47's task's references, set beside H-47's forms. It writes no prediction, decides nothing, and expects no sign of any figure.
- **Its cells**: H-47's twelve, under H-47's configuration and groups (`with_nights`).
- **The floor**, exploration 039's learner on each readout unit's count, on paths 0, 5 and 6. A run from the start image, the reward withheld, presents the pair's 36 blocks as H-47's stretches present them on that path. At blocks 6, 12, 18, 24 and 36, forks of it read A and the pair on H-47's sixteen streams each, the reward withheld. Every trial keeps the 918 readout units' window counts. The learner starts at 16 384 on its own readout's features, so that it first answers as the readouts select. It learns on the pair's trials by `ValueCritic::step` at the critic's constants, as exploration 039's did, and answers each read's trials without learning.
- **Checked inside the test**: the withheld run presents the stimuli H-47's chain presents on that path, trial by trial; the units' counts sum to the readouts'; and the learner, unmoved, answers as the readouts select.
- **The ceiling**, on each cell, on path 0's image at block 36 under H-47's night without replay, its chain held to H-47's pinned rows. The search by forks runs over four genes, A's four couplings' factors in 1/256. Each runs from 128 to the largest that keeps its written coupling within 1.30 of H-42's image's, and starts at 256; P 16, G 10, E 4, M 16 and S 3, from exploration 039's seed. Each candidate is written into a fork by brief 088's writer, and the six groups are set from the written engine's `coupling_sum`, so the start's reads are not H-47's.
- **Its score** on stream s is one withheld block of A and one of the pair on H-47's stream s, summed. The search's streams are 1 to 3, its unseen ones 4 to 6, and a read is kept by its genes and stream. The box's sixteen corners are read on streams 1 to 6. Every read prints A's and the pair's counts apart, and any coupling past 1.30 of H-42's image's.
- **Its trial**, one test, on seed 22's first mapping from the assignment, whose start learns its mapping first: the start, path 0's chain to block 36, one generation of the search and the floor on path 0, timed. Each job is projected from it. Past 144 minutes, ADR-0179's 60 per cent of an arms job's bound, a cell runs as two jobs, the floor and the ceiling; if a part still passes, G falls, then P, decided before any job.
- **Its jobs**: twelve, one a cell, each named by its cell and ending `_explore`, in one `scope=arms` dispatch.

It reads, per cell, with exploration 017's spread beside every count and no SE of its own:
1. the floor's A and pair at each checkpoint, beside H-47's four forms', by path and over the three;
2. the ceiling's best genes and its A and pair on the search's streams and the unseen ones, beside the best corner, the start's reads and H-47's forms' reads at block 36 on streams 1 to 6;
3. how many reads passed 1.30 of H-42's image's.

It runs from `main` on `explore/040-what-a-readout-and-the-couplings-keep` and adds its harness only; a read that needs a change under `src/` stops it. Exploration 040 comes first, as ADR-0254's table orders. Brief 089 does not read its note, and the maintainers start each. It does not run the gate, another form, a gene but A's four couplings, the ceiling on paths 5 and 6, a floor that sees a stimulus unit, or the sixteen ways to keep four nights.

### Exploration 038, and the sweep

- **Exploration 038** ([ADR-0253](0253-what-the-night-writes-on-the-other-ten-cells.md)) is replaced by H-48 and does not run. Its question, what each night writes on H-47's other ten cells, is a dissection; a later ADR may take it up if H-48 reads no.
- **The schedule's sweep of 2026-11-18 at 20:17Z** has not begun when this is written. The first ADR that merges after it ends records it.

## Consequences

- **Good**: M11's next step is a mechanism, read on H-47's cells against H-47's own pinned rows, with no form of H-47's run again.
- **Good**: the gate uses what the engine has and tissue lacks, a fork and a comparison, on the host's side, with no change to the engine or its image.
- **Bad**: each gap runs two nights and two stretches. The round runs twelve jobs of under an hour and the same again to reproduce them, and the weekly job gains twelve short forms, at most 4 800 s together on x86-64.
- **Bad**: the probe of the gap at block 24 reads block 36's image on streams the reads do not use, so the gate has seen the image H-48 reads at block 36.
- **Neutral**: the references decide nothing, and H-48's prediction does not rest on them.

## Alternatives considered and why rejected

- **Option 1(b), the image as it was**: a rejected night becomes no sleep at all, a form H-47 never ran, so neither of H-47's forms is its control.
- **Option 1(c), episode by episode**: an order of magnitude more forks a gap.
- **Option 2(b), right after the night**: on seed 23's first mapping from the mirrored assignment it would keep the early nights whose cost appears later (exploration 030).
- **Option 2(c), both**: twice the jobs for one change.
- **Option 3(b), an exploration first**: M11 has run eighteen explorations since H-47, and ADR-0254 makes order 3 a hypothesis.
- **Option 3(c), H-48 alone**: ADR-0254 orders the references first.
- **Option 4(b), the sixteen ways**: about eight times a form's runs a path, past thirty jobs.
- **Option 5(b), paused**: an open line with no condition that closes it.
- **Option 5(c), run**: ten jobs of dissection beside the mechanism.

## Confirmation

`briefs/089_a-night-kept-by-the-hosts-probe.md` builds the gated night and runs H-48 once.
[H-48's report](../research/H-48.md) holds the hypothesis as written before any run.
`npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
