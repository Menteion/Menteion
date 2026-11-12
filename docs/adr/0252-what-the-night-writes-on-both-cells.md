---
status: accepted
date: 2026-11-12
depends-on: ADR-0251
title: What the night writes, on both cells
kind: decision
summary: On seed 24's two eighth images the volley's drive from A's kinds onto A's answer's readout less the other, carried by the weights, parted the cells by +187.7 to +197.0 k after the three later single nights. Exploration 037 reads unit by unit what each night writes onto the readout units.
decision-makers: Menteion maintainers
---

# ADR-0252: What the night writes, on both cells

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 037 runs the nights alone on seed 24's two eighth images, from the assignment and from the mirrored assignment, with no read of A or of the pair. On each of H-47's three paths it runs the night without replay's chain to block 24 and, from the image before each of its four nights, the single replayed night. On every night it reads, by readout unit, each unit's spikes in the replays of A's patterns, and what consolidation and the zero sum's rounds move onto it from each of A's and B's kinds. Two jobs, one a cell. The branch adds its harness only, on an engine that is `main`'s, and `main` takes nothing from it. It runs once this ADR merges.
- **Why.** Less the night without replay, after the three later single nights, the mirrored image's two-span drive on A's trials, A's answer's readout less the other, lay above the assignment's by +179.9, +191.1 and +448.0 k. The volley's from A's kinds lay above by +187.7, +195.1 and +197.0 k, by the weights +188.2, +198.1 and +198.4, worked from exploration 036's logs. The logs hold each readout unit's inputs by class before and after every night, and each kind's consolidation by when and by the target's class. They do not hold which readout units fire in the replays, or what consolidation and the zero sum move onto each readout unit.
- **Left open.** Whether a hypothesis follows exploration 037; any other cell; why the presynaptic units fire less in the window after a replayed night; why the cells part at block 36 by the spikes per weight; the order of the sums from read to read within a cell; a spike's arrival after its synapse's delay; a unit's membrane or threshold; the inputs of the units outside the readouts' sets; any read of A or of the pair, and what a night's writing does to them; replay on two of the four nights, or on every night but one; any hold; the twelve blocks after the last night; plain nights on both and the awake gap; what the next day does with the pairings a night leaves; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0251](0251-what-reaches-the-readout-units-on-both-cells.md) ordered exploration 036. On seed 24's two eighth images it read every unit's spikes on A's reads, and the drive: each synapse's weight onto a readout unit times its presynaptic unit's spikes, in the readout window and in the volley before it. Its readings are exploratory, not evidence ([exploration 036](../explorations/036-what-reaches-the-readout-units-on-both-cells.md)):

- **Every row and read is the logs'** of explorations 033 to 035, label aside, under the same six forms on both cells.
- **The presented stimulus's own units fire in the volley**, ticks 0 to 99 of a trial, about once a unit, and almost never in the window.
- **Right after each single replayed night, in the window**, excitatory less inhibitory drive fell onto both readouts on both stimuli's trials by 717.6 to 1 626.1 k a trial, beyond 2 SE in 32 of 32. Of each place group's fall, 74 to 96 per cent lies in the spikes per weight.
- **In the volley the trial's stimulus's kinds' drive** rose onto its answer's readout by 254.0 to 412.6 k and fell onto the other by 236.9 to 399.4 k, each beyond 2 SE, 96 to 103 per cent of each change by the weights.
- **On A's trials, the two spans' sum** of A's answer's readout less the other, less the night without replay, orders the two cells as the spikes do in 12 of 12 distinct comparisons.

On A's trials, A's answer's readout less the other, less the night without replay, right after each single night, from exploration 036's section 4:

| Form | The volley from A's kinds, k: the image from the assignment | of which by the weights | the image from the mirrored assignment | of which by the weights | The two spans' sum, the mirrored image's less the assignment's | of which A's kinds |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone | +685.9 | +687.1 | +670.3 | +667.7 | −298.2 | −15.6 |
| After block 12 alone | +624.3 | +622.0 | +812.0 | +810.2 | +179.9 | +187.9 |
| After block 18 alone | +597.0 | +594.2 | +792.1 | +792.4 | +191.1 | +195.1 |
| After block 24 alone | +490.9 | +488.6 | +687.9 | +687.0 | +448.0 | +197.0 |

After block 12 and 18 alone A's kinds carried more than all of the sum's difference, after block 24 alone 197.0 of 448.0 k, where the window carried 243.1. A's answer's readout is readout 1, the even places, on the image from the assignment and readout 0, the odd places, on the image from the mirrored assignment. Of A's patterns' targets it holds 252 on the first and 280 on the second; the other readout holds 245 and 254.

Each of A's and B's four kinds is a group of the zero sum: H-47 sets six, each kind of unit apart ([ADR-0236](0236-learning-that-sleep-keeps-its-protocol.md)), by the mechanism of [ADR-0226](0226-a-zero-sum-at-the-stimulus.md) and [ADR-0228](0228-a-zero-sum-at-the-stimulus-built.md). After every tick whose consolidations moved one of a group's synapses onto a target, its weights onto the targets, the two readouts' 918 units, inhibitory ones among them, are scaled back to the group's value. So what a kind gains onto one readout it loses onto the other, within the rounds' rounding. Exploration 035 read A's kinds' input onto A's answer's readout less onto the other, less the night without replay, right after each single night: +0.67, +0.81, +0.79 and +0.69 M on the mirrored image, +0.69, +0.62, +0.60 and +0.49 on the image from the assignment.

What the logs hold of a night:
- exploration 034's input read, on the image before and after every night: each readout unit's input from each class;
- the watch's lines, every night: the night's spikes in the ripples that replay one of A's patterns, by stimulus and class, with the ripples' count; and each kind's pairings, with what consolidation took into the weights, by when and by the target's class.

Which readout units fire in the replays, and what consolidation and the zero sum's rounds move onto each readout unit, are not logged.

## Decision Drivers

- **The maintainers' choices** (2026-11-12): read unit by unit what each single replayed night writes onto the readout units on both of seed 24's eighth images, with each readout unit's spikes in the night's replays; and run it once this ADR merges.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A read changes no form, and any hypothesis still needs an ADR of its own.
- **The harness watches every night already**: on the tag `explore/036-what-reaches-the-readout-units-on-both-cells`, exploration 020's `ReplayWatch` ticks each ripple one tick at a time and classes each spike of a ripple that replays one of A's patterns. It works out each pairing of A's and B's kinds by the rule as the block held it, and what consolidation took from the trace into the weight (`absorbed`), held to the trace the engine then holds. Every watched unit is a source of the zero sum, so the weight itself is not checked. The watch keeps a copy of those units' synapse blocks (`shadow`), so each kind's weights when it is built and after its last tick are at hand. It tallies by the target's class, not by unit; summing by readout unit needs no change under `src/`.
- **The images are in the logs**: a checkpoint's row carries the crc64 of the image after its night, its `highest` and its night's gain, none of which needs a read.

## Considered Options

1. **What the night writes on both cells**: the nights alone, each readout unit's spikes in the replays of A's patterns and what consolidation and the zero sum move onto it, two jobs.
2. **The drive with each spike's arrival after its synapse's delay**, exploration 036's read again on both cells.
3. **Exploration 036's read on the other ten cells**, two jobs a cell.
4. **A reading of the logs alone**: each readout unit's input change over each night, and each kind's consolidation by when and by the target's class, with no run.
5. **Write a hypothesis now**, or **pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.

## Decision Outcome

**Option 1.**

### Exploration 037 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **Its cells** are seed 24's eighth image from the assignment and from the mirrored assignment, H-25's two arms, on H-47's paths 0, 5 and 6. **A's patterns** are H-47's 102, one starting at each of the 102 stimulus units, A's and B's kinds; a pattern's stimulus is its first unit's.
- **Its nights**, on each path of each cell: the chain, H-47's blocks from the start to block 24 with the night without replay after block 6, 12, 18 and 24; and, from the image before each of those four nights, a single replayed night, H-47's plain night on A's patterns, with nothing run after it. Eight nights a path, twenty-four a cell. Until its night a single night's form ran the chain's nights on the chain's images, so each replayed night is that form's night. No block runs after block 24, and no read of A or of the pair runs.
- **Held inside the test**: each replayed night reads replays and depotentiations that are not zero, and each chain night none (`e030_night`). Each night's image after it, by crc64, its `highest` and its night's gain are the row's that the logs give for that night, path and cell:
  - the chain's nights, H-47's pinned row of the night without replay at that block;
  - the night after block 6 alone, H-47's plain night's at block 6;
  - the nights after block 12, 18 and 24 alone, the rows exploration 033 printed on the image from the assignment and exploration 035 on the mirrored image, pinned in the harness.
  A night whose image differs fails its job, and the job says which.
- **Its read**, on every night, in the watch, with no change under `src/`, each summed by unit over the arena and printed for the readout units:
  - each unit's spikes in the ripples that replay one of A's patterns, by the pattern's stimulus;
  - for each unit and each of the four kinds, what consolidation took into the weights of that kind's synapses onto it (`absorbed`), summed over the night in four bins of the pairing's ordinal: 0 to 3 together, the presynaptic unit's own replay; `PAIR_AFTER`, its next spike after it; `PAIR_ELSE_SWS`, any other in slow-wave sleep; `PAIR_ELSE`, any other. A bin says when a trace is consolidated, not when it was written: a consolidation takes its share of the whole trace the synapse then holds, half of it at the slow-wave baseline, earlier pairings' and the day's among it;
  - for each unit and each kind, the zero sum's rounds: that kind's weights onto the unit after the watch's last tick, before the quiet ticks, less their weights when the watch was built, less consolidation onto it;
  - the count of pairings read at a modulation below zero, where the watch checks nothing and `absorbed` would carry the weight's move with the other sign.
- **Held on every night**: the watch's misses (`wrong`) and the pairings read below zero are none; the rounds onto every target outside the two readouts are zero; and the read, totalled over units with each spike and pairing under the class the watch gave it, is the watch's lines. Spikes by stimulus and class are its ripple-spikes line; consolidation by kind, target class and ordinal, ordinals 0 to 3 and the lines' `spikes` summed, is its pairings lines' `absorbed`. A night that fails one fails its job, which says which.
- **Beside it**, exploration 034's input read on the image before and after each night, as explorations 034 to 036 ran it. A readout unit's change from a kind over the night, less its consolidation and its rounds, is what the watch does not see: the ticks before it is built and the quiet ticks after the night. It is printed, not held to zero. A unit's class is the start image's (`e020_class`).
- **Its branch**, `explore/037-what-the-night-writes-on-both-cells`, starts from `main` with exploration 036's tag merged in; that tag changes nothing under `src/`, so the engine is `main`'s. The branch adds its harness only; if the read cannot be taken without a change under `src/`, it stops and says why.

**Its jobs**, two, one a cell, each its three paths' 72 blocks and 24 nights, the paths run two at a time as the harness runs its chains. They are expected to be far shorter than exploration 036's: its longer job ran twelve chains, 48 nights and 960 reads of A. The trial runs one path whole, so each job's projection is the trial's time for that cell's path, twice. If it passes an arms job's 240 minutes, each cell runs one job a path, its test named by cell and path; if a path passes too, the exploration stops before any job and says why.

**Its trial**, one test, runs path 0 of each cell whole, as a job runs it: the chain's 24 blocks and four nights and the four replayed nights, with the input reads around each night and the read, each night held as above, so every pinned row of path 0 is checked before a job runs.

It reads, on each cell, with no SE: a night's figure is the mean over the three paths of one night each, with the paths' lowest and highest beside it. Each readout is named by the stimulus whose answer it is, as exploration 035 named them:
1. **The nights against the logs**: each chain night beside the night without replay after the same block on the same path in explorations 033 to 036's logs, and each replayed night beside the night after that block alone's: images, replays, depotentiations and watch lines, label aside; and the input reads before and after each night beside explorations 034 to 036's.
2. **What each replayed night wrote onto each readout**, by the input reads' change, less the night without replay from the same image: from each of A's and B's kinds onto each readout's units, by the unit's class (C's or D's own, A's patterns' targets, other excitatory, inhibitory), and A's answer's readout less the other.
3. **How it was written**: each kind's consolidation in each of its four bins and its zero sum's rounds, onto each readout and class, beside the input reads' change and what the watch did not see.
4. **Which readout units fired in the replays**: each readout unit's spikes in the replays of each stimulus's patterns, by class; and over each readout's 459 units, on the units' three-path means, the correlation of a unit's spikes in the replays of a stimulus's patterns on the replayed night, where the night without replay replays none, with its input change from that stimulus's kinds and with their consolidation onto it, each the replayed night's less the night without replay's.
5. **The two cells side by side**, each of those by stimulus, the same night.

Of exploration 036's questions it reads, as far as the night's writing goes, what sets the larger rise from A's kinds onto A's answer's readout less the other on the mirrored image after the three later single nights. Why the presynaptic units fire less in the window, the drive after each synapse's delay, why the cells part at block 36 by the spikes per weight, and the order of the sums on the other ten cells and from read to read stay open.

Before it spends a job it states whether any night must run bit for bit as one already in the logs. Every night's images, replays and watch lines are in explorations 033 to 036's logs, and its input reads in 034 to 036's. No log holds a readout unit's spikes in the replays, consolidation by readout unit, or the zero sum's rounds, so each night runs, and its images and lines are set beside those logs'.
It does not run any other cell; replay on two nights or on every night but one; any hold; H-47's plain night on all four nights; the twelve blocks after the last night; plain nights on both or the awake gap; any read of A or of the pair; the inputs of the units outside the readouts' sets; a spike's arrival after its synapse's delay; a unit's membrane or threshold; any path but H-47's three; or the next day's reading of the pairings a night leaves. No part of a night runs apart; consolidation is only binned by when it falls.

## Consequences

- **Good**: a part of what parts the two cells, A's kinds' weights onto the readouts, is read where it is written, unit by unit, with the zero sum's share beside consolidation's, on the two cells whose spikes, inputs and drive are already read.
- **Good**: with no reads the jobs are expected to be short, and every night's image is held to the logs inside the test.
- **Bad**: two cells of twelve; a contrast between them says nothing of the other ten.
- **Bad**: the watch follows A's and B's kinds alone; the other units' weights onto the readout units are read only as each night's change, and the ticks the watch does not see only as a remainder.
- **Bad**: a bin says when a trace is consolidated, not when the pairing that wrote it fell.
- **Bad**: a figure is the mean of three nights, one a path, with no SE.
- **Bad**: the harness change is on a branch that `main` does not take.
- **Bad**: M11 stays open; any hypothesis needs its own ADR, brief and round.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, the drive after each synapse's delay**: it checks how exploration 036 split the drive between the spans; the cells' parting it read lies largely in the weights from A's kinds, which this does not trace.
- **Option 3, the other ten cells**: it asks whether the order of the sums holds elsewhere before anything says why the two cells part.
- **Option 4, the logs alone**: they hold each readout unit's change over a night and consolidation by the target's class, but not which readout units fire in the replays, nor what consolidation and the zero sum move onto each unit.
- **Option 5, a hypothesis now or a pause**: what sets the larger rise on the mirrored image is unread, so a prediction would rest on a difference with no mechanism read; the read is one exploration away, with jobs expected short.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 037's note, when it merges, holds what it read.
