---
status: proposed
date: 2026-11-12
depends-on: ADR-0251
title: What the night writes, on both cells
kind: decision
summary: On seed 24's two eighth images the cells parted after the three later single nights mostly by the volley's drive from A's kinds onto A's answer's readout less the other, a part carried by the weights. Exploration 037 reads unit by unit what each night writes onto the readout units.
decision-makers: Menteion maintainers
---

# ADR-0252: What the night writes, on both cells

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 037 runs the nights alone on seed 24's two eighth images, from the assignment and from the mirrored assignment. On each of H-47's three paths it runs the night without replay's chain to block 24 and, from the image before each of its four nights, the single replayed night. On every night it reads each readout unit's spikes in the ripples that replay A's patterns, and the weight the pairings of A's and B's kinds write onto each readout unit, by when they fall. No read of A or of the pair runs. Two jobs, one a cell. The branch adds its harness only, on an engine that is `main`'s, and `main` takes nothing from it. It runs once this ADR merges.
- **Why.** After the three later single nights the mirrored image's sum of the two spans' drive, A's answer's readout less the other on A's trials, lay above the assignment's by +179.9 to +448.0 k. A's kinds carried +187.9 to +197.0 k of it, in the volley, by the weights. Explorations 034 to 036 read the weights before and after each night by class, but not where on each readout they land, which readout units fire in the replays, or when in the night the weight is written.
- **Left open.** Whether a hypothesis follows exploration 037; any other cell; why the presynaptic units fire less in the window after a replayed night; a spike's arrival after its synapse's delay; a unit's membrane or threshold; the inputs of the units outside the readouts' sets; the pair's reads unit by unit; replay on two of the four nights, or on every night but one; any hold; the twelve blocks after the last night; plain nights on both and the awake gap; what the next day does with the pairings a night leaves; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

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

A's answer's readout is readout 1, the even places, on the image from the assignment and readout 0, the odd places, on the image from the mirrored assignment. Of A's patterns' targets it holds 252 on the first and 280 on the second; the other readout holds 245 and 254. Over each single replayed night, exploration 035 read A's kinds' input onto A's answer's readout rising by 0.35 to 0.42 M on the mirrored image and 0.23 to 0.31 on the image from the assignment. What sets the larger rise after the three later nights is not read: an input read sums a readout's units by class, before and after a night, and says nothing of which units gain, which units fire in the replays, or when in the night the weight moves.

## Decision Drivers

- **The maintainers' choices** (2026-11-12): read unit by unit what each single replayed night writes onto the readout units on both of seed 24's eighth images, with each readout unit's spikes in the night's replays; and run it once this ADR merges.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A read changes no form, and any hypothesis still needs an ADR of its own.
- **The harness watches every night already**: on the tag `explore/036-what-reaches-the-readout-units-on-both-cells`, exploration 020's `ReplayWatch` ticks each ripple one tick at a time. It classes each spike of a ripple that replays one of A's patterns, and works out each pairing of A's and B's kinds by the rule as the block held it, its weight moved (`absorbed`) against what the engine wrote. It sums these by class only; summing them by readout unit needs no change under `src/`.
- **The images are in the logs**: a checkpoint's row carries the crc64 of the image after its night, and explorations 034 to 036 read the inputs onto each readout unit before and after every night.

## Considered Options

1. **What the night writes on both cells**: the nights alone, with each readout unit's spikes in A's patterns' replays and the weight the pairings write onto it, two jobs.
2. **The drive with each spike's arrival after its synapse's delay**, exploration 036's read again on both cells.
3. **Exploration 036's read on the other ten cells**, two jobs a cell.
4. **A reading of the logs alone**: each readout unit's change over each night from the input reads already logged, with no run.
5. **Write a hypothesis now**, or **pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.

## Decision Outcome

**Option 1.**

### Exploration 037 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **Its cells** are seed 24's eighth image from the assignment and from the mirrored assignment, H-25's two arms, on H-47's paths 0, 5 and 6.
- **Its nights**, on each path of each cell: the night without replay's chain from the start to block 24, H-47's blocks with a night without replay after block 6, 12, 18 and 24; and, from the image before each of those four nights, the night after that block alone runs, H-47's plain night on A's patterns. Eight nights a path, twenty-four a cell. A single night's form and the night without replay ran the same nights on the same images until then, so each replayed night starts from the image its night without replay starts from. No block runs after block 24, and no read of A or of the pair runs.
- **Held inside the test**: each night reads replays and depotentiations as its form asks (`e030_night`). The night without replay's image after each night carries the crc64 of H-47's pinned row of the night without replay at that block on that path, and the night after block 6 alone's that of H-47's plain night at block 6. The images after the nights after block 12, 18 and 24 alone are printed and set beside explorations 033's and 035's rows by the note.
- **Its reads**, on every night, in the watch, with no change under `src/`:
  - each readout unit's spikes in the ripples that replay one of A's patterns, by the pattern's stimulus;
  - for each readout unit and each of the four kinds of A's and B's units, the weight their pairings move onto it (`absorbed`, the watch's reading), summed over the night by when the pairing falls: in the presynaptic unit's own replay, at its next spike after it, otherwise in slow-wave sleep, otherwise;
  - exploration 034's input read on the image before and after each night, as the jobs of explorations 034 to 036 ran it, so each readout unit's change by presynaptic class over the night.
- **What it prints**: after each night, the spikes and the moved weights unit by unit, and the watch's lines as before. The part of a unit's change from a kind that its pairings do not account for, the zero sum's rounds among it, is the input reads' less the pairings', printed beside them and not held to zero. A unit's class is the start image's (`e020_class`).
- **Its branch**, `explore/037-what-the-night-writes-on-both-cells`, starts from `main` with exploration 036's tag merged in; that tag changes nothing under `src/`, so the engine is `main`'s. The branch adds its harness only; if the read cannot be taken without a change under `src/`, it stops and says why.

**Its jobs**, two, one a cell, each the three paths' twenty-four nights. With no reads they run far shorter than exploration 036's. The trial times a path's first two nights with the read and without it, and each job's projection scales that to twenty-four nights and its blocks; if a projection passes an arms job's 240 minutes, each cell's paths split over two jobs.

**Its trial**, one test, runs path 0 of each cell to block 6 and both nights after it with the read, each night's image held as above.

It reads, on each cell, with no SE: a night's figures are the mean over the three paths of one night each, with the paths' range beside it. Each readout is named by the stimulus whose answer it is, as exploration 035 named them:
1. **The nights against the logs**: each night's image, replays, depotentiations and watch lines beside explorations 033 to 036's for the same night, path and form, label aside, so that every input figure is those notes'.
2. **What each replayed night wrote onto each readout**, less the night without replay from the same image: the weight from each of A's and B's kinds onto each readout's units, by the unit's class (C's or D's own, A's patterns' targets, other excitatory, inhibitory) and A's answer's readout less the other.
3. **When it was written**: the pairings' moved weight by when they fell, onto each readout and class, beside the input reads' change and the part the pairings do not account for.
4. **Which readout units fired in the replays**: each readout unit's spikes in A's patterns' replays of each stimulus, by class, and over each readout's units the correlation of a unit's spikes in the replays of a stimulus's patterns with its gain from that stimulus's kinds.
5. **The two cells side by side**, each of those by stimulus, the same night and path.

Of exploration 036's questions it reads what sets the volley's larger rise from A's kinds onto A's answer's readout less the other on the mirrored image after the three later single nights, as far as the night's writing goes. Why the presynaptic units fire less in the window, the drive with each spike's arrival after its synapse's delay, why the cells part at block 36 by the spikes per weight, and the order on the other ten cells stay open.

Before it spends a job it states whether any night must run bit for bit as one already in the logs: every night's images, replays and watch lines are in explorations 033 to 036's logs, but no log holds a readout unit's spikes in the replays or the pairings' weight by readout unit, so each night runs, and its images and lines are set beside those logs'.
It does not run any other cell; replay on two nights or on every night but one; any hold; H-47's plain night on all four nights; the twelve blocks after the last night; plain nights on both or the awake gap; any read of A or of the pair; the inputs of the units outside the readouts' sets; a spike's arrival after its synapse's delay; a unit's membrane or threshold; any path but H-47's three; any part of a night apart; or the next day's reading of the pairings a night leaves.

## Consequences

- **Good**: the part of the drive that parts the two cells, the weights from A's kinds, is read where it is written, unit by unit and by the pairing's time, on the two cells whose spikes, inputs and drive are already read.
- **Good**: with no reads the jobs are short, and every night's image is checked against the logs.
- **Bad**: two cells of twelve; a contrast between them says nothing of the other ten.
- **Bad**: the watch follows the pairings of A's and B's kinds alone; the other units' weights onto the readout units are read only as each night's change, and the zero sum's rounds only as a remainder.
- **Bad**: a figure is the mean of three nights, one a path, with no SE.
- **Bad**: the harness change is on a branch that `main` does not take.
- **Bad**: M11 stays open; any hypothesis needs its own ADR, brief and round.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, the drive after each synapse's delay**: it checks exploration 036's split between the spans, which carries the cells' parting in the weights from A's kinds; it does not ask where those weights come from.
- **Option 3, the other ten cells**: it asks whether the order of the sums holds elsewhere before anything says why the two cells read part.
- **Option 4, the logs alone**: they hold each readout unit's change over a night, but not which readout units fire in the replays nor when in the night the weight moves.
- **Option 5, a hypothesis now or a pause**: what sets the larger rise on the mirrored image is unread, so a prediction would rest on a difference with no mechanism read, and the read is one short exploration away.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 037's note, when it merges, holds what it read.
