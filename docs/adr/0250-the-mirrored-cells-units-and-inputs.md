---
status: proposed
date: 2026-11-12
depends-on: ADR-0249
title: The mirrored cell's units and inputs
kind: decision
summary: On seed 24's eighth image from the assignment each single replayed night moved the readouts' inputs the same way. Exploration 035 reads seed 24's eighth image from the mirrored assignment, where the nights after block 18 and 24 alone raised A right after them, unit by unit and by input.
decision-makers: Menteion maintainers
---

# ADR-0250: The mirrored cell's units and inputs

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 035 reads seed 24's eighth image from the mirrored assignment under exploration 033's six forms. It counts each readout unit's spikes on A's reads, as exploration 033 did, and reads the weights onto each readout unit by presynaptic class on each image, as exploration 034 did, and sets both beside those two explorations' readings of seed 24's eighth image from the assignment, the other arm's. Two jobs run the six forms. The branch adds its harness only, on an engine that is `main`'s, and `main` takes nothing from it. It runs once this ADR merges.
- **Why.** On seed 24's eighth image from the assignment each single replayed night raised the inhibitory input onto both readouts by 4.18 to 4.48 M and moved A's kinds toward A's answer's readout, after the first night and after the later ones alike. Yet on A's trials A's answer's readout less the other lay within 2 SE of the night without replay after the first night, +0.48 ± 0.29, and below it after the later ones, −1.46 to −3.73. On seed 24's eighth image from the mirrored assignment A's count lay above the night without replay right after the nights after block 18 and 24 alone. Whether the same inputs move where A rose is unread.
- **Left open.** Whether a hypothesis follows exploration 035; any other cell; a unit's membrane, threshold or delays; the units outside the readouts' sets, their spikes or inputs, and a weight paired with its presynaptic unit's spikes; replay on two of the four nights on any cell, or on every night but one on any cell but seed 23's first mapping from the mirrored assignment; the pair's reads unit by unit; the night held onto the inhibitory and outside units without replay, or split finer; the part held across, or exploration 024's hold, beside any part; the synapses no hold reached; exploration 024's hold on the other eight cells; what the next day does with the pairings a night leaves; what keeps the night held within below the excitatory night on seed 24's eighth image from the assignment; what moves A's trials below the plain night's on seed 23's eighth image from the mirrored assignment; why after block 12 alone lies below the night without replay on seed 23's eighth image from the assignment; plain nights on both and the awake gap night by night; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04); the 2026-11-11 sweep.

## Context and Problem Statement

[ADR-0249](0249-one-cells-readout-units-inputs.md) ordered exploration 034: on seed 24's eighth image from the assignment, under exploration 033's six forms, the weights onto each of the two readouts' 918 units summed by the class of the presynaptic unit, on each image the jobs read. Its readings are exploratory, not evidence ([exploration 034](../explorations/034-one-cells-readout-units-inputs.md)):

- **Every row, read and unit read is exploration 033's**, label aside, and each of the 191 images' input reads sums to that image's class matrix. On that cell A asks readout 1, the even places, of stimulus A and readout 0, the odd places, of B.
- **Over each single replayed night** the inhibitory input onto each readout rose by 4.18 to 4.48 M and all excitatory input fell by 3.37 to 4.29. Each place group's excitatory units lost more onto the other readout than onto their own. The night without replay's same nights raised the inhibitory input by 0.24 to 0.36 and moved each excitatory group by 0.18 or less. The gain fell by 0.061 to 0.065 over each single replayed night and rose by 0.033 to 0.037 over the night without replay's same nights.
- **A's kinds moved toward readout 1**: right after each single replayed night their input difference onto readout 1 less onto readout 0 read +0.49 to +0.69 M, and A's margin rose by 1.05 to 1.26 M over each. Readout 1's excitatory input less its inhibitory input fell further than readout 0's, by 0.01, 0.48, 0.46 and 0.95 M after block 6, 12, 18 and 24 alone, where the spikes' readout 1 less readout 0 on A's trials read +0.48, −1.46, −2.96 and −3.73 (exploration 033).
- **The units that carried half of each readout's drop** lost input from the trial's stimulus's kinds, −10 330 to −7 226 a unit on A's trials where the rest of their readout had −433 to +1 065, and gained less inhibitory input than the rest, in each of the 16 sets right after. Over each readout's units a unit's input difference from the trial's stimulus's kinds correlated with its spikes' difference by +0.51 to +0.72.
- **By block 36** the inhibitory difference had come back by 42 to 81 per cent, and A's and B's kinds' and each place group's difference had moved by 0.43 M or less.
- **What reaches a unit** was not read: a unit's inputs are weights, the presynaptic units outside the readouts are not counted, and no weight is paired with its presynaptic unit's spikes.

On seed 24's eighth image from the mirrored assignment, H-25's other arm on the same seed, A asks readout 0, the odd places, of stimulus A and readout 1, the even places, of B. There, A's count less the night without replay's, in [exploration 031](../explorations/031-the-first-nights-replay-on-twelve-cells.md) and [exploration 032](../explorations/032-each-later-nights-replay-alone-on-twelve-cells.md):

| Form | Right after its replayed night | At block 36 |
| :--- | ---: | ---: |
| After block 6 alone | −2.6 ± 1.35 | +3.1 ± 1.84 |
| After block 12 alone | +0.8 ± 1.09 | +3.8 ± 0.78 |
| After block 18 alone | +9.1 ± 0.92 | +5.4 ± 1.17 |
| After block 24 alone | +5.4 ± 2.02 | +1.9 ± 1.18 |
| H-47's plain night on all four nights | – | +11.3 ± 0.96 |

Right after their nights after block 18 and 24 alone lie above the night without replay beyond 2 SE, after block 6 and 12 alone within; at block 36 after block 12 and 18 alone lie above, after block 6 and 24 alone within. On seed 24's eighth image from the assignment the same four read +0.3 ± 0.64 right after the night after block 6 alone and −7.3 ± 0.88, −12.1 ± 0.63 and −12.3 ± 0.58 after block 12, 18 and 24 alone, in the same notes. No log holds that cell's readout units one by one or their inputs.

## Decision Drivers

- **The maintainers' choices** (2026-11-12): read the mirrored cell's readout units' spikes and inputs together, and run it once this ADR merges.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A read changes no form, and any hypothesis still needs an ADR of its own.
- **The harness reads both**: exploration 033's unit read and exploration 034's input read are on the tag `explore/034-one-cells-readout-units-inputs` and take the cell as their arguments, as exploration 032's tests did; neither needed a change under `src/`.

## Considered Options

1. **The mirrored cell's units and inputs**, six forms, two jobs.
2. **What reaches each readout unit on the image from the assignment**, each synapse's weight by its presynaptic unit's spikes on each of A's trials, summed by presynaptic class, two jobs.
3. **Write a hypothesis now**, built by a brief and measured by a round.
4. **Pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.

## Decision Outcome

**Option 1.**

### Exploration 035 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **Its cell** is seed 24's eighth image from the mirrored assignment alone, on H-47's paths 0, 5 and 6, read sixteen times at blocks 6, 12, 18, 24 and 36 in H-47's terms.
- **Its forms** are exploration 033's six: the night without replay in each job, `alone-6`, `alone-12`, `alone-18` and `alone-24`, and `plain-first`, H-47's plain night on all four nights. No hold is set.
- **Its reads**: A's reads counted unit by unit as in exploration 033, and on each image whose class matrix the job writes, the start image, the image before each night and after it, and block 36's, the input read of exploration 034, fourteen sums a readout unit, its sums over each of the readouts' eight classes held to that class matrix's 112 entries inside the test.
- **Its branch** starts from `main` with exploration 034's tag merged in, for the harness of explorations 033 and 034; that tag changes nothing under `src/`, so the engine is `main`'s. The branch adds its harness only; if the two reads cannot be pointed at this cell without a change under `src/`, or this cell's readout units do not make up the eight classes the input read holds, it stops and says why.

**Its jobs**, two, as exploration 034's: one runs the night without replay, after block 6 alone and H-47's plain night, nine chains; the other the night without replay and after block 12, 18 or 24 alone, twelve chains. On the image from the assignment exploration 034's took 5 822 and 7 925 s; on this cell exploration 031's job of two forms took 3 863 s, where that image's took 3 911, and exploration 032's of four forms 7 603 s, where that image's took 7 872, so about 96 and 128 minutes, under an arms job's 240.

**Inside each job** the rows are held to H-47's as in exploration 033's jobs (`held_to`), each form's replayed nights read replays and depotentiations that are not zero and its other nights none (`e030_night`), the unit read's guards hold as there, and the input read's sums are held as above. Every row and read is set beside H-47's, exploration 031's and exploration 032's jobs' on this cell in the note, and every line of exploration 031's and exploration 032's jobs on this cell and of exploration 021's job's plain night on it, label aside, is looked for in exploration 035's.

**Its trial** runs the pair's head and each form's first night on path 0 of that cell, as exploration 034's trial did: each holds H-47's row at block 6 on path 0 of the form whose first night it ran, and each input read's sums are the class matrix's entries.

It reads, on that cell, in exploration 033's terms for spikes and exploration 034's for inputs, each paired within a job and unpaired across jobs as exploration 033 did; on this cell A's answer's readout is readout 0, so where the two cells are set side by side each readout is named by the stimulus whose answer it is:
1. **The rows and reads against the logs**: each form's, beside H-47's, exploration 031's and exploration 032's, so that A's count is those notes'.
2. **What each replayed night did to the readouts' spikes**: each readout's spikes a trial on each stimulus's trials under each form, each form less the night without replay right after its replayed night and at block 36; each stimulus's answer's readout less the other; the units that moved beyond twice their own SE and the fewest that carry half of each readout's change; and each class, as exploration 033's sections 1 to 4 read them.
3. **What each replayed night changed in the inputs**: each night's change by presynaptic group and class onto each readout beside the night without replay's same night; the units that carried half of each readout's change against the rest; each image's gain; the image each replayed night starts from; the correlation over each readout's units of a unit's input difference with its spikes' difference; A's margin over each night; each later night and stretch to block 36; and H-47's plain night, as exploration 034's sections 2 to 5 read them.
4. **The two cells side by side**: each of those readings beside exploration 033's and exploration 034's on the image from the assignment, by stimulus, the same form, night and checkpoint, and on the image before each replayed night.

No SE is claimed for an input: an input is a mean of three images, one a path.

Before it spends a job it states whether any form must run bit for bit as one already in the logs: the six forms' images, rows and reads are in H-47's, exploration 021's, exploration 031's and exploration 032's logs, but no log holds this cell's units one by one or their inputs, so each form runs, and its rows and reads are set beside those logs'.
It does not run any other cell; replay on two nights or on every night but one; any hold; plain nights on both or the awake gap; the pair's reads unit by unit; the units outside the readouts' sets, their spikes or their inputs; a synapse's weight paired with its presynaptic unit's spikes; a unit's membrane, threshold or delays; any path but H-47's three; any part of a night apart, such as slow-wave sleep from REM; or the next day's reading of the pairings a night leaves.

### The sweep

No sweep is left unrecorded: [ADR-0239](0239-the-readouts-inhibition-held-through-a-night.md) recorded the schedule's run of 2026-11-04. The run of 2026-11-11 at 20:17Z has not ended when this is written, and the first ADR that merges after it ends records it.

## Consequences

- **Good**: on the same seed, the cell where the single nights after block 18 and 24 raised A right after them is read unit by unit and by input, beside the cell where the later single nights lowered A's answer's readout below the other.
- **Good**: one exploration gives that cell both reads its neighbour took two to get.
- **Bad**: two cells of twelve; a contrast between them says nothing of the other ten.
- **Bad**: a unit's weights are not its drive: the spikes of its presynaptic units outside the readouts, any weight paired with its presynaptic unit's spikes, its membrane and its threshold are not read.
- **Bad**: the forms run again, though their images are in the logs by checksum.
- **Bad**: the harness change is on a branch that `main` does not take.
- **Bad**: M11 stays open; any hypothesis needs its own ADR, brief and round.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, what reaches each unit on the image from the assignment**: it reads one cell further down, where the inputs moved the same way on all four nights, before anything says whether replay moves the inputs the same way on a cell where single later nights raised A.
- **Option 3, a hypothesis now**: no single replayed night lies above the night without replay at block 36 in all twelve cells, so a prediction would rest on a split reading.
- **Option 4, pause M11**: the two eighth images of one seed, where the single later nights moved A's count opposite ways right after them, are one exploration from being read alike.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 035's note, when it merges, holds what it read.
