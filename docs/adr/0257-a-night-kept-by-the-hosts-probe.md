---
status: proposed
date: 2026-11-14
depends-on: ADR-0254
title: A night kept by the host's probe, and H-47's task's references
kind: decision
hypothesis: H-48
summary: ADR-0254's third order. H-48 asks on H-47's twelve cells whether nights the host keeps by a probe six blocks on keep the first mapping at least as well as nights without replay, and more in ten; predicted no. Exploration 040 reads H-47's task's references; exploration 038 is replaced.
decision-makers: Menteion maintainers
---

# ADR-0257: A night kept by the host's probe, and H-47's task's references

**In brief**
- **Decision.** Nothing in the engine changes. At each of H-47's four gaps the host runs both of H-47's nights in forks, the night without replay and the plain night on A's patterns. Each fork runs the pair's next 6 blocks, A is read on six streams H-47's reads never use, and the fork that reads more is kept, the plain night on a tie. H-48 asks, on H-47's twelve cells, whether such gated nights keep A at least as well as nights without replay in every cell, and more in at least ten. It is predicted no: on its second clause, and on its first too where the gate is expected to keep a night whose cost comes late. Exploration 040 first reads H-47's task's references: exploration 039's floor on each readout unit's count, and a ceiling by the search over A's four couplings. Brief 089 then builds the gate in the tests and runs H-48 once. Exploration 038 is replaced by H-48 and does not run.
- **Why.** ADR-0254 makes a night kept by a gate the host runs M11's next decision, with order 1's tool for H-47's task's references. Explorations 030 to 032 read that a replayed night's cost can appear blocks after the night. Six blocks on, their nearest readings leave clause 2 short. The round is still worth running: nothing else reads the chains a gate runs, or whether it removes replay's cost where replay had one.
- **Left open.** When the plan's order 2 runs; DR-25, the replay's order set by the host's tags, order 3's other candidate; chained episodes compiled in sleep (UPG-04); whether the slow-wave baseline enters the running configuration.

## Context and Problem Statement

[ADR-0254](0254-the-design-review-planned.md)'s third order asks of M11 whether a night kept only when the host's probe shows retention keeps a mapping better than an ungated night and a night without replay, on what the probe did not see. It names DR-24, a gate the host runs, and DR-25 as candidates. It puts order 1's tool for H-47's task's references first, and leaves exploration 038 to this ADR. The probe and the mappings are the host's; the engine's rules and the policy amendment do not change.

**What H-47 read** ([ADR-0237](0237-learning-that-sleep-keeps-measured.md)): plain nights on A's patterns kept more of A than nights without replay at block 36, beyond twice the paired SE, in ten of twelve cells, by +2.2 ± 0.65 to +19.6 ± 1.06. They did not on seed 23's first mapping from the mirrored assignment, −4.9 ± 1.09, below, or on seed 24's eighth image from the assignment, +1.8 ± 1.79, within. The pair stood at 61.8 to 63.4 under plain nights, and A's highest coupling under any form at 1.141 to 1.223 of H-42's image's.

**When a night's cost shows.** Explorations 030 to 032 read single replayed nights, each from the night without replay's chain, and forms leaving one plain night out. Their readings are exploratory, not evidence:
- on seed 23's first mapping from the mirrored assignment, the nights after blocks 6 and 12 alone read +1.9 ± 0.68 and +1.6 ± 0.55 right after them, both above beyond 2 SE. H-47's four plain nights read −4.9 at block 36, and leaving any one of them out shrank the loss ([exploration 030](../explorations/030-one-cell-night-by-night.md));
- on seed 24's eighth image from the assignment, the first plain night stood +0.3 ± 0.64 over the night without replay right after it, and the form that replays on it alone −2.0 ± 0.88 at block 36 ([exploration 031](../explorations/031-the-first-nights-replay-on-twelve-cells.md));
- right after the night, single replayed nights stood above the night without replay beyond 2 SE in eight, seven, seven and six of the twelve cells after blocks 6, 12, 18 and 24 ([exploration 032](../explorations/032-each-later-nights-replay-alone-on-twelve-cells.md)).

**Six blocks on.** The first checkpoint after six more blocks is the nearest those tables come to what a probe six blocks on reads; for the night after block 24 it is block 36, twelve blocks on. There the form that replays on one night alone stood above the night without replay beyond 2 SE in 5, 9, 7 and 3 of the twelve cells after the nights at blocks 6, 12, 18 and 24, and below in 1, 2, 1 and 2. On seed 24's eighth image from the assignment the first night read +1.4 ± 0.72 at block 12, and the later nights −4.6 ± 1.83, −8.8 ± 0.82 and −4.6 ± 0.73 at blocks 18, 24 and 36. There, the form that replays on the first night alone stood +6.4 ± 1.70 over H-47's plain nights at block 18, under exploration 018's unpaired SE.

**What a probe resolves.** Exploration 017 split a read's SD of 3.52 into a stream's 2.64 and the rest's 2.34 ([exploration 017](../explorations/017-what-a-read-spreads.md)). Worked out and not run, two forks' difference over the same six reads has an SD of about √12 × 2.34 ≈ 8.1 correct trials, 1.35 a read, if they share each stream's part, and up to √12 × 3.52 ≈ 12.2, 2.0 a read, if they do not.

**What a fork costs.** The nights already run on forks: each of H-47's forms decodes the image the pair's stretch wrote, and each of H-47's reads is a fork. One H-47 job ran two forms on three paths in 61.2 to 68.6 minutes on AArch64 ([H-47's report](../research/H-47.md#the-dispatches)).

**The references.** On the two-alternative task a learner on each readout unit's count kept pace with the engine, and the best of the coupling box's corners stood above the search's best ([exploration 039](../explorations/039-the-two-alternative-tasks-references.md)). H-47's task has no reference yet.

## Decision Drivers

- **The maintainers' choices for the gate** (2026-11-14): order 3 before order 2. A rejected night is replaced by the night without replay, not by the image as it was. The probe reads A alone, on six streams, six blocks after every night.
- **The maintainers' choices for the round** (2026-11-14): H-48 as a round, predicted no and run, after exploration 040's references. The references are exploration 039's floor on each readout unit's count and a ceiling by the search over A's four couplings. Exploration 038 is replaced by H-48.
- **ADR-0254's question, narrowed.** It asks for better than an ungated night and a night without replay. A gate that keeps every plain night is the ungated night, so it cannot stand above it in every cell. H-48's clauses compare with the night without replay, as M11's exit test does ([ADR-0234](0234-learning-that-sleep-keeps.md)), and the ungated night is a reading.
- **What the probe does not see**: H-47's streams 0 to 15, and every image H-48's reads read. Six blocks on from a night is the image before the next night, or block 30.
- **H-47's stopping rule**, step 8: "No constant moves after such a run. There is no second attempt, and no other baseline, shift, form, path or network." H-48 is a new hypothesis on H-47's cells, with its own constants committed before any run, and runs no form of H-47's as an arm.
- **The host's side** (ADR-0254): the gate is the host's reading of its own mappings on forks. It is neither the policy amendment's trial, which commits only on equal behaviour hashes, nor a gate the engine sets on its own amendments.
- **One change a round**: the gate alone; DR-25 waits.
- **A predicted no** is worth a round only when the no is a reading nothing else gives ([ADR-0089](0089-the-assignment-reversed.md)).
- **Latest ≠ Newest**: the tree's own code, and no dependency.

## Considered Options

1. **The rejected night**: (a) the night without replay, both nights run in forks; (b) the image as it was, DR-24's text; (c) episode by episode.
2. **The probe's time**: (a) six blocks after the night, at every gap; (b) right after the night; (c) both, each a gated chain of its own.
3. **The probe's reads**: (a) A alone, on six streams; (b) A and the pair, on three streams each.
4. **The order**: (a) H-48 as a round, exploration 040's references first; (b) an exploration of the gate first, a hypothesis after; (c) H-48 alone.
5. **The references**: (a) exploration 039's floor on each readout unit's count, and the search's ceiling over A's four couplings; (b) those and the best of the sixteen ways to keep or reject four nights, read after the fact.
6. **Exploration 038**: (a) replaced by H-48; (b) paused until H-48's verdict; (c) run.
7. **The plan's order**: (a) order 3 before order 2; (b) order 2 first.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a), 6(a) and 7(a).**

### The gated night (Specified; brief 089 builds it in the tests)

On each of H-47's paths, from H-47's start image, the pair's head of 6 blocks runs as H-47's does. Then at each gap, after blocks 6, 12, 18 and 24:
- **Two forks.** From the image the pair's stretch wrote, H-47's night without replay and its plain night on A's patterns each run, as H-47's forms run them.
- **Six blocks on.** Each fork runs the pair's next 6 blocks from its night image, as H-47's stretches run, from the path's first index.
- **The probe.** Each fork's image after those 6 blocks is read on streams 16 to 21 (`nights_read_from`), one read of A on each, the reward withheld, as H-47 reads. Its score is the sum of the six reads' correct trials, of 384.
- **The choice.** The plain night's fork is kept when its score is at least the night without replay's; otherwise that fork is kept.
- **The chain.** The kept fork's night image is the gated chain's image at the gap. After the gaps at blocks 6, 12 and 18, its 6 blocks are the chain's stretch. After the gap at block 24, the kept fork runs H-47's 12 blocks to block 36 from its night image.

Around the gate:
- **Its reads**: H-47's, at blocks 6, 12, 18, 24 and 36, sixteen of A and sixteen of the pair at each checkpoint, on the gated chain's images. The probe reads none of those images and none of H-47's streams.
- **Its comparisons**: H-47's pinned rows of the night without replay and of the plain night on A's patterns, on the same paths and streams, paired by path and stream ([H-47's terms](../research/H-47.md#its-terms)). No form of H-47's runs as an arm or a comparison, and a chain that repeats one is held to its rows.
- **Checked inside the test**: while the gate has kept the same form at every gap so far, the chain's image and reads at each checkpoint are H-47's pinned row of that form. At each gap where it has, that form's fork's night image is H-47's row of the form there, before the choice. A gated chain that is not stops the round as a finding.
- **Its pins**, in `tests/inhibition/pins/h48.rs`: the chain's rows at the five checkpoints, as H-47's rows are. At each gap, for each fork: its night image and its probe image by CRC-64, A's couplings at their highest over its six blocks and in its probe image, and its six counts; and the choice.
- **The calibration test**, which step 2 names: on seed 22's eighth image from the assignment, on path 0, it runs the head and both forks of the gap at block 6, and holds each night's image and reads to H-47's pinned row at block 6. It runs at `scope=arms` beside the jobs, and again beside them in the reproduction. If `round:additive` exits 0 it is the whole calibration; if it exits 1 it runs beside the whole tree's.
- **Twelve `…_paused` jobs**, one a cell, its three paths two at a time and each gap's forks one after the other. Each runs the same 24 nights and 198 blocks as one of H-47's jobs, with 624 reads against H-47's 960: about an hour each on AArch64.
- **A short form of each cell** is a weekly `…_exhaustive` test. On path 0 it runs the head, both forks at block 6, their six blocks and the probe, held to the long run's pinned rows of that gap. The twelve take at most 4 800 s together on x86-64, as the reproduction reads them alone, and keep the cost table's default until a full deal ([ADR-0220](0220-the-cost-of-a-test-run-alone.md)).
- **The reproduction**: the short forms at `scope=exhaustive`, and the calibration test and the twelve jobs again at `scope=arms` on the reproduction's commit, every pinned table held, as H-47's were.

### H-48 (Hypothesis; written before any run)

The text, with its clauses, readings, terms and stopping rule, is [H-48's report](../research/H-48.md).
- **Clause 1**: in each of the twelve cells, A's mean count at block 36 under gated nights is not below that under nights without replay by more than twice its paired SE.
- **Clause 2**: in at least ten of the twelve cells, A's mean count at block 36 under gated nights exceeds that under nights without replay by more than twice its paired SE.
- **Clause 3**: in each cell, the pair's mean count at block 36 under gated nights is at least 56 of 64.
- **Clause 4**: in each cell, none of A's four couplings passes 1.30 of H-42's image's on its network. It is read at each block's end of every stretch the gated chain runs and of every fork's six blocks, and at each image read.
- **Yes** when all four hold; otherwise **no**, with the network, the start and the clause.

A mean count and the paired SE are H-47's, of 48 reads, three paths by sixteen streams.

**Predicted: no.** Six blocks on, the nearest readings put three cells below clause 2's bound, and clause 1 is at risk in two of them:
- on seed 24's eighth image from the assignment the gate is expected to keep the first plain night and the nights without replay after it. On every path that is exploration 031's form that replays on the first night alone, bit for bit, which read −2.0 ± 0.88 at block 36. If the gate runs it on all three paths, clause 1 fails there, and step 5 is taken before step 6;
- on seed 23's eighth image from the assignment the night after block 12 read −2.3 ± 0.76 six blocks on, and no form that replays on one night alone stood above at block 36;
- on seed 23's first mapping from the mirrored assignment the night after block 24 read −0.3 ± 0.81 right after it and −2.4 ± 0.86 at block 36, and the probe reads between them. Of exploration 030's forms with a plain night there, H-47's among them, four of five stood below the night without replay beyond 2 SE at block 36, and none of the four without one; the form leaving that night out read −1.3 ± 1.40, within. Clause 1 there rests on the probe rejecting that night.

**Four readings are predicted:**
1. clause 1's rule holds in at least 10 of the 12 cells;
2. clause 2's rule holds in at least 8 of the 12 cells;
3. on seed 23's first mapping from the mirrored assignment, A's mean count at block 36 under gated nights over that under plain nights, positive;
4. on seed 24's eighth image from the assignment, the plain night kept at the gap at block 6 on at least two of its three paths, and the night without replay at at least seven of its nine later gaps.

**What would refute it**: all four clauses holding, the gated nights above the night without replay beyond 2 SE in ten or more cells. A no read on clause 1 with clause 2 holding keeps the no, on another clause than the one expected.

### H-48's stopping rule

1. **One round**: brief 089 builds the gated night in the tests and writes its protocol ADR, then runs H-48 once. Its clauses, constants and readings are committed before any arm runs.
2. **A calibration stops the round**, and is a finding, if a pinned number moves. So does the calibration test, run beside the jobs at `scope=arms` and in the reproduction, if a fork's image or reads at block 6 are not H-47's pinned row of its form. So does a gated chain that, where the gate has kept the same form at every gap so far, is not H-47's pinned row of that form.
3. **Yes**: M11's exit test reads yes. The next decisions are whether M11 closes, whether the slow-wave baseline enters the running configuration, and whether DR-25 runs.
4. **No on clause 4**: M12 reopens ([ADR-0232](0232-a-running-configuration-and-m12-closed.md)), and an ADR on the zero sum through the form that crossed.
5. **No on clause 1**: an ADR on what the probe six blocks on does not see on that network.
6. **No on clause 2**: an ADR on what the gate gives up where replay kept more.
7. **No on clause 3**: an ADR on what the gated nights cost the pair.
8. **Where several steps apply**, the first in this order is taken. After any verdict an ADR sets when the plan's order 2 runs, and orders its orders 4 to 7 beside M13, M14, M8's remainder and M15, as ADR-0254 says.
9. **No constant moves after such a run.** There is no second attempt, and no other probe, stream, tie rule, form, path or network.

### Exploration 040 (Specified; it decides nothing)

H-47's task's references, set beside H-47's forms. It writes no prediction, decides nothing, and expects no sign of any figure.
- **Its cells**: H-47's twelve, under H-47's configuration and groups (`with_nights`).
- **The floor**, exploration 039's learner on each readout unit's count, on paths 0, 5 and 6. A run from the start image, the reward withheld, presents the pair's 36 blocks as H-47's chain presents them on that path. It restarts at each stretch as H-47's does: blocks of 6, 6, 6, 6 and 12, each from the path's first index, quieted and encoded between them. At blocks 6, 12, 18, 24 and 36, forks of it read A and the pair on H-47's sixteen streams each, the reward withheld. Every trial keeps the 918 readout units' window counts.
- **The learner** starts at 16 384 on its own readout's features, so that it first answers as the readouts select. It learns on the pair's trials by `ValueCritic::step` at the critic's constants, as exploration 039's did, and answers each read's trials without learning.
- **Checked inside the test**: the withheld run presents the stimuli H-47's chain presents on that path, trial by trial; the units' counts sum to the readouts'; and the learner, unmoved, answers as the readouts select.
- **The ceiling**, on each cell, on path 0's image at block 36 under H-47's night without replay, its chain held to H-47's pinned rows. The search by forks runs over four genes, A's four couplings' factors in 1/256, from exploration 039's seed, at P 16, G 10, E 4, M 16 and S 3, each starting at 256.
- **Each gene's top** is found with brief 088's writer on that image: the largest factor, at most 512, whose written coupling times 100 is at most H-42's image's times 130. Each gene runs from 128 to its top.
- **Each candidate** is written into a fork by brief 088's writer, and the six groups are set from the written engine's `coupling_sum`, so the start's reads are not H-47's. Its score on stream s is one withheld block of A and one of the pair on H-47's stream s, summed. The search's streams are 1 to 3, its unseen ones 4 to 6, and a read is kept by its genes and stream. The box's sixteen corners are read on streams 1 to 6. Every read prints A's and the pair's counts apart, and any coupling past 1.30 of H-42's image's.
- **Its trial**, one test ending `_explore`, on seed 22's first mapping from the assignment, whose start learns its mapping first. It runs the start, path 0's chain to block 36, one generation of the search and the floor on path 0, timed, and each job is projected from it. A cell whose projection passes 144 minutes, ADR-0179's 60 per cent of an arms job's bound, runs as two jobs, the floor and the ceiling. If a part still passes, G falls, then P, decided before any job.
- **Its jobs**: twelve, or two for a cell that needs them, each named by its cell and ending `_explore`, in one `scope=arms` dispatch. A job a runner fails is dispatched again.

It reads, per cell, with exploration 017's spread beside every count and no SE of its own:
1. the floor's A and pair at each checkpoint, beside H-47's four forms', by path and over the three;
2. the ceiling's best genes, and its A and pair on the search's streams and the unseen ones, beside the best corner, the start's reads and H-47's forms' reads at block 36 on streams 1 to 6;
3. how many reads passed 1.30 of H-42's image's.

It runs from `main` on `explore/040-what-a-readout-and-the-couplings-keep` and adds its harness only; a read that needs a change under `src/` stops it. It runs before brief 089, which does not read its note; the maintainers start each. It does not run the gate, another form, a gene but A's four couplings, the ceiling on paths 5 and 6, a floor that sees a stimulus unit, or the sixteen ways to keep four nights.

### Exploration 038, and the sweep

- **Exploration 038** ([ADR-0253](0253-what-the-night-writes-on-the-other-ten-cells.md)) is replaced by H-48 and does not run. Its question, what each night writes on H-47's other ten cells, is a dissection; a later ADR may take it up after H-48's verdict.
- **The schedule's sweep of 2026-11-18 at 20:17Z** has not begun when this is written. The first ADR that merges after it ends records it.

## Consequences

- **Good**: M11's next step is a mechanism, read on H-47's cells against H-47's own pinned rows.
- **Good**: the gate is a fork and a comparison on the host's side, an engineered capability stated as a mechanism (DR-01), with no change to the engine or its image. It computes no gradient (DR-21), and H-47's task gains its references, a rung of the ladder (DR-26).
- **Bad**: each gap runs two nights and their blocks. The round runs twelve jobs of about an hour, and the same again to reproduce them, and the weekly job gains twelve short forms, at most 4 800 s together on x86-64.
- **Bad**: it is predicted no. If the no is read, M11 stays open, and the next ADR asks what the probe six blocks on does not see (step 5) or what the gate gives up (step 6).
- **Neutral**: the references decide nothing, and H-48's prediction does not rest on them.

## Alternatives considered and why rejected

- **Option 1(b), the image as it was**: a rejected night becomes no sleep at all, a form H-47 never ran, so neither of H-47's forms is its control.
- **Option 1(c), episode by episode**: an order of magnitude more forks a gap.
- **Option 2(b), right after the night**: on seed 23's first mapping from the mirrored assignment it would keep the early nights whose cost appears later (exploration 030).
- **Option 2(c), both**: twice the chains and jobs for one change.
- **Option 3(b), A and the pair**: the pair reads near its ceiling under every form, so its reads add noise; clause 3 holds it apart.
- **Option 4(b), an exploration first**: M11 has run eighteen explorations since H-47, and ADR-0254 makes order 3 a hypothesis.
- **Option 4(c), H-48 alone**: ADR-0254 orders the references first.
- **Option 5(b), the sixteen ways**: about eight times a form's runs a path, past thirty jobs.
- **Option 6(b), paused**: an open line with no condition that closes it.
- **Option 6(c), run**: ten jobs of dissection beside the mechanism.
- **Option 7(b), order 2 first**: the maintainers took M11's step first; order 2 waits.

## Confirmation

`briefs/089_a-night-kept-by-the-hosts-probe.md` builds the gated night and runs H-48 once.
[H-48's report](../research/H-48.md) holds the hypothesis as written before any run.
`npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
