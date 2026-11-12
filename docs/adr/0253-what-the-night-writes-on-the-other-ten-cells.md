---
status: proposed
date: 2026-11-13
depends-on: ADR-0252
title: What the night writes, on the other ten cells
kind: decision
summary: On seed 24's two eighth images the cells parted after the later single nights in what consolidation in the units' own replays wrote and in which readout units it reached. Exploration 038 reads what each night writes on H-47's other ten cells, beside their A's counts.
decision-makers: Menteion maintainers
---

# ADR-0253: What the night writes, on the other ten cells

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 038 runs exploration 037's read on H-47's other ten cells: on each of H-47's three paths, the night without replay's chain to block 24 and, from the image before each of its four nights, the single replayed night, with no read of A or of the pair. It sets each cell's figures beside A's count less the night without replay right after the same night, from explorations 031 and 032. Ten jobs, one a cell. The branch adds its harness only, on an engine that is `main`'s, and `main` takes nothing from it. It runs after this ADR merges, when the maintainers start it.
- **Why.** On seed 24's two eighth images the cells parted, after the three later single nights, by +188.8, +198.6 and +198.6 k in A's kinds' weights onto A's answer's readout less B's. Consolidation in the units' own replays made +230.0, +276.0 and +207.9 of it, and on the mirrored image less of A's kinds' change fell on A's patterns' targets. Two cells say nothing of the other ten, where A's count right after a single replayed night lay above the night without replay beyond 2 SE in six to eight cells.
- **Also.** It records that exploration 037 read the pairings below zero instead of holding them at none, as ADR-0252 had ordered.
- **Left open.** Whether a hypothesis follows exploration 038; the trace a night starts with, and how much of each bin is the day's; the dopamine term at a night's start; the order in which readout units fire after a replayed pattern's first unit; why the presynaptic units fire less in the window after a replayed night; a spike's arrival after its synapse's delay; a unit's membrane or threshold; the inputs of the units outside the readouts' sets; any read of A or of the pair; replay on two of the four nights, or on every night but one; any hold; the twelve blocks after the last night; plain nights on both and the awake gap; what the next day does with the pairings a night leaves; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0252](0252-what-the-night-writes-on-both-cells.md) ordered exploration 037. On seed 24's two eighth images it ran the nights alone and read, by readout unit, each unit's spikes in the replays of A's patterns, what consolidation moved onto it from each of A's and B's kinds by when, and the zero sum's rounds. Its readings are exploratory, not evidence ([exploration 037](../explorations/037-what-the-night-writes-on-both-cells.md)):

- **Every night is the logs'**: on each cell all 24 nights held their row inside the test, and every line each night printed, but the harness's own, is the logs' night's.
- **On both cells each single replayed night moved A's kinds' weights toward A's answer's readout and away from B's, by the same zero sum**: each kind's change onto one readout is the negation of its change onto the other within 121 of the class matrix's units, on every night and path.
- **Most of it was consolidation**, in the presynaptic unit's own replay and at its next spike after it; the zero sum's rounds and what the watch does not see moved little of each difference.
- **The cells part in the own replays and in where the weights go**: on the mirrored image more is consolidated in the own replays and less at the other spikes in slow-wave sleep, on all four nights; and of A's answer's readout less B's, less falls on A's patterns' targets and more on the other excitatory and inhibitory units.

From A's kinds onto A's answer's readout less B's, less the night without replay from the same image, the image from the mirrored assignment's less the image from the assignment's, from exploration 037's section 5:

| Night | The inputs' change, k | Consolidation in the own replay | at the other spikes in slow-wave sleep | The zero sum's rounds | By class: A's patterns' targets | other excitatory | inhibitory |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone | −19.5 | +145.9 | −168.3 | +1.2 | −208.7 | +57.8 | +120.2 |
| After block 12 alone | +188.8 | +230.0 | −35.1 | +1.1 | −42.3 | +80.4 | +133.9 |
| After block 18 alone | +198.6 | +276.0 | −120.6 | −19.4 | −74.2 | +78.5 | +182.3 |
| After block 24 alone | +198.6 | +207.9 | −69.0 | +10.8 | −84.3 | +93.9 | +145.3 |

On H-47's twelve cells A's count right after a single replayed night is read ([exploration 031](../explorations/031-the-first-nights-replay-on-twelve-cells.md), [exploration 032](../explorations/032-each-later-nights-replay-alone-on-twelve-cells.md)). After block 6 alone it lay above the night without replay at block 6 beyond twice the paired SE in eight cells and within 2 SE in four. After block 12, 18 and 24 alone it lay above beyond 2 SE in seven, seven and six cells, and below on seed 24's eighth image from the assignment, −7.3 ± 0.88, −12.1 ± 0.63 and −12.3 ± 0.58; after block 12 alone also on seed 23's eighth image from the assignment, −1.9 ± 0.88. What each night writes is read on two of the twelve.

**What exploration 037 changed.** ADR-0252 held the pairings read at a modulation below zero at none, and the harness's first trial stopped on 896 of them on the mirrored image's path 0, on the night without replay after block 18. A night's engine is decoded from an image, and a decoded executor addresses every unit ([ADR-0068](0068-the-reward-addressed.md)). So an excitatory synapse consolidates under the baseline plus the dopamine term, which under the signed gate ([ADR-0094](0094-the-signed-gate-built.md)) can fall below zero and moves the weight against its trace's sign. Exploration 037 reads such a pairing by that rule, counts each miss in the watch's `wrong`, held at zero, and counts its weight's move with its own sign in a fifth bin. The count is printed each night instead of held. In its jobs two of the 48 nights read any, both from the mirrored image's path 0 image before the night after block 18. This ADR records the change, and exploration 038 keeps it.

## Decision Drivers

- **The maintainers' choices** (2026-11-13): run exploration 037's read on H-47's other ten cells, beside each cell's A's count from explorations 031 and 032; and start it after this ADR merges, when they say so.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A read changes no form, and any hypothesis still needs an ADR of its own.
- **The harness reads any cell**: on the tag `explore/037-what-the-night-writes-on-both-cells`, `e037_cell` runs a cell's three paths given the rows its nights hold. H-47 pinned the night without replay's rows and its plain night's on all twelve cells. The rows of the nights after block 12, 18 and 24 alone are in exploration 032's jobs and, on seed 23's first mapping from the mirrored assignment, in exploration 030's one-night-alone job.
- **The nights alone are short**: exploration 037's two jobs took 893 and 866 s.

## Considered Options

1. **What the night writes on the other ten cells**, exploration 037's read, ten jobs, beside each cell's A's count.
2. **The trace a night starts with**, by kind and readout unit, and the dopamine term, on seed 24's two eighth images.
3. **The order in which the readout units fire** after a replayed pattern's first unit, on the same two.
4. **Write a hypothesis now**, or **pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.

## Decision Outcome

**Option 1.**

### Exploration 038 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **Its cells** are H-47's ten but seed 24's two eighth images: seeds 22, 23 and 24's first mappings from the assignment and from the mirrored assignment, and seeds 22 and 23's eighth images from both, on H-47's paths 0, 5 and 6.
- **Its nights, its read and its holds** are exploration 037's, as that exploration ran them at `bbf805d`: the chain's four nights without replay and the four single replayed nights a path; each readout unit's spikes in the replays of A's patterns, what consolidation moved onto it from each of A's and B's kinds in five bins, the fifth below zero, the zero sum's rounds, and what the watch does not see; exploration 034's input read before and after each night. Every hold is exploration 037's, the count below zero printed and not held.
- **Its rows**: the night without replay's are H-47's pinned rows, and the night after block 6 alone's H-47's plain night's at block 6. Those of the nights after block 12, 18 and 24 alone are the `PIN row` lines exploration 032's jobs printed, run [42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683). On seed 23's first mapping from the mirrored assignment they are exploration 030's, run [42621480151](https://github.com/Menteion/Menteion/actions/runs/42621480151). They are read from the artifacts by a script and pinned in the harness. A night whose image differs fails its job, which says which; a row that lacks a field holds what it has, and the note says which.
- **Its branch**, `explore/038-what-the-night-writes-on-the-other-ten-cells`, starts from `main` with exploration 037's tag merged in; that tag changes nothing under `src/`, so the engine is `main`'s. The branch adds its harness only: the pinned rows of ten more cells, a trial and ten jobs. If the read cannot be taken without a change under `src/`, it stops and says why.

**Its jobs**, ten, one a cell, each its three paths' 72 blocks and 24 nights, two paths at a time. Exploration 037's took 893 and 866 s. **Its trial**, one test, runs path 0 of seed 23's first mapping from the mirrored assignment whole, the cell whose rows come from another run, and times it. Each job's projection is that path's time twice; past an arms job's 240 minutes, each cell runs one job a path, and if a path passes too the exploration stops before any job and says why.

It reads, with no SE on a night's figure, as exploration 037 did, each readout named by the stimulus whose answer it is:
1. **The nights against the logs**: each night's image, replays, depotentiations and watch lines beside explorations 031's and 032's, or 030's, for the same night, path and cell, label aside; and each image's class matrix and split beside theirs. The logs hold no input read on these ten cells.
2. **On each cell**, exploration 037's readings 2 to 4: what each replayed night wrote onto each readout from each kind, by the units' class; how it was written; and which readout units fired in the replays.
3. **The twelve cells side by side**, seed 24's two from exploration 037: for each cell and single night, A's kinds onto A's answer's readout less B's, less the night without replay, its parts by bin, rounds and class. Beside it, A's count less the night without replay right after the same night, with its paired SE, from exploration 031 at block 6 and exploration 032 at blocks 12, 18 and 24.
4. **Over the twelve cells**, for each single night, the correlation of each of those parts with A's count's difference, beside it where A answers readout 0 and where it answers readout 1, six cells each.

Of exploration 037's questions it reads whether the parting by the own replays holds on the other ten cells. The trace a night starts with, the dopamine term, and the order of the readout units' spikes in a replay stay open.

Before it spends a job it states whether any night must run bit for bit as one already in the logs. Every night's image, replays and watch lines are in explorations 030 to 032's logs, but no log holds a readout unit's spikes in the replays, consolidation by readout unit, or the zero sum's rounds on these cells, so each night runs, and its images and lines are set beside those logs'.
It does not run seed 24's two eighth images again; replay on two nights or on every night but one; any hold; H-47's plain night on all four nights; the twelve blocks after the last night; plain nights on both or the awake gap; any read of A or of the pair; the inputs of the units outside the readouts' sets; a spike's arrival after its synapse's delay; a unit's membrane or threshold; any path but H-47's three; or the next day's reading of the pairings a night leaves. No part of a night runs apart.

## Consequences

- **Good**: what parted seed 24's two cells is read on all twelve, beside the A's counts already read there, with the read and holds exploration 037 proved.
- **Good**: the nights alone are short, about fifteen minutes a cell on exploration 037's jobs.
- **Bad**: a correlation over twelve cells, one figure a cell and night, with no SE, says nothing of cause.
- **Bad**: A's count is the logs', read on a run with the reads of A; this exploration runs none, so A's count and the night's writing are set side by side, not read together.
- **Bad**: the harness change is on a branch that `main` does not take.
- **Bad**: M11 stays open; any hypothesis needs its own ADR, brief and round.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, the trace a night starts with**: it deepens the reading on two cells whose contrast may not hold on the other ten.
- **Option 3, the order of the spikes in a replay**: the same, on the same two cells.
- **Option 4, a hypothesis now or a pause**: what parts seed 24's two cells is read on two of twelve, so a prediction over twelve would rest on two; the read on all twelve is ten short jobs away.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 038's note, when it merges, holds what it read.
