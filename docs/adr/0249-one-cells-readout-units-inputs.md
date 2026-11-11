---
status: proposed
date: 2026-11-11
depends-on: ADR-0248
title: One cell's readout units' inputs
kind: decision
summary: On seed 24's eighth image from the assignment every replayed night lowered both readouts' spikes on both stimuli's trials, readout 1 more than readout 0 on A's trials after a later single night. Exploration 034 reads the weights onto each readout unit by presynaptic class across each night.
decision-makers: Menteion maintainers
---

# ADR-0249: One cell's readout units' inputs

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 034 reads the same cell as exploration 033, seed 24's eighth image from the assignment, under its six forms. On each image it reads, for each of the two readouts' 918 units, the weights onto the unit summed by the class of the presynaptic unit, and reads what each night and each stretch of blocks changed in them, the units that carried each readout's drop in exploration 033 beside the rest. Two jobs run the six forms. The branch adds its harness only, on an engine that is `main`'s, and `main` takes nothing from it. It runs once this ADR merges.
- **Why.** Right after each single replayed night both readouts fired less on both stimuli's trials, by 18 to 33 per cent, and on A's trials readout 1 less readout 0, less the night without replay, read +0.48 ± 0.29 after block 6 alone, within 2 SE, and −1.46 ± 0.22 to −3.73 ± 0.16 after block 12, 18 or 24 alone. A's margin, each of A's and B's kinds' weight onto its stimulus's answer readout less onto the other, rose by 1.05 to 1.26 M over each single replayed night, 1.26 over the first. Exploration 033 counted the units' spikes, not what each receives, so it does not say why A's answer's readout fell more after a later single night than after the first.
- **Left open.** Whether a hypothesis follows exploration 034; any other cell, seed 24's eighth image from the mirrored assignment among them; a unit's membrane, threshold or delays; the units outside the readouts' sets, their spikes or their inputs; replay on two of the four nights on any cell, or on every night but one on any cell but seed 23's first mapping from the mirrored assignment; the pair's reads unit by unit; the night held onto the inhibitory and outside units without replay, or split finer; the part held across, or exploration 024's hold, beside any part; the synapses no hold reached; exploration 024's hold on the other eight cells; what the next day does with the pairings a night leaves; what keeps the night held within below the excitatory night on seed 24's eighth image from the assignment; what moves A's trials below the plain night's on seed 23's eighth image from the mirrored assignment; why after block 12 alone lies below the night without replay on seed 23's eighth image from the assignment; plain nights on both and the awake gap night by night; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0248](0248-one-cells-readouts-unit-by-unit.md) ordered exploration 033: on seed 24's eighth image from the assignment, the night without replay, H-47's plain night on A's patterns after block 6, 12, 18 or 24 alone and on all four nights, each readout unit's spikes counted on each of A's trials. Its readings are exploratory, not evidence ([exploration 033](../explorations/033-one-cells-readouts-unit-by-unit.md)):

- **Every row and read is the logs'**: each form's rows and reads are those of H-47's, exploration 031's and exploration 032's jobs, so A's count is those notes'.
- **Right after each single replayed night both readouts fired less on both stimuli's trials**, each beyond 2 SE: on A's trials readout 1 by 7.45 to 10.19 spikes a trial and readout 0 by 6.03 to 7.92, on B's trials readout 0 by 6.27 to 6.77 and readout 1 by 5.31 to 6.21, 18 to 33 per cent of the night without replay's spikes. Every class of both readouts fired less, each beyond 2 SE. On this cell A asks readout 1, the even places, of stimulus A and readout 0, the odd places, of B.
- **The answer's readout less the other**: on A's trials readout 1 less readout 0 read +0.48 ± 0.29 against the night without replay after block 6 alone, within 2 SE, and −1.46 ± 0.22, −2.96 ± 0.19 and −3.73 ± 0.16 after block 12, 18 and 24 alone, beyond 2 SE; its sign is the sign of A's correct trials on stimulus A, less the night without replay's, in each comparison, as the note found it.
- **Many units moved, and few carry half the change**: 95 to 136 of a readout's 459 units moved down beyond twice their own SE and 0 to 5 up, where about 23 would by chance in the two directions together, 11 or 12 each way; 13 to 19 carry half of each readout's drop, about as many as carry half of the night without replay's spikes. By class on A's trials, after block 12, 18 or 24 alone readout 1's pattern targets fired 4.63 to 5.14 less a trial and its inhibitory units 3.75 to 4.12 less, where after block 6 alone 3.70 and 2.86 less.
- **At block 36** each single night still lay below the night without replay on both readouts and both stimuli's trials, by 1.28 to 4.49, each beyond 2 SE, having come back by 3.10 to 5.84 since right after its replayed night.
- **By group**, each single replayed night moved the class matrix on that cell about as H-47's first plain night did (explorations 031 and 032), and A's margin rose by 1.05 to 1.26 M over each single replayed night, after block 6 alone too; each readout's sums by kind are in the logs' splits and were not read.
- **What a unit receives** was not read: exploration 033 counts spikes, not a unit's inputs, so it does not say why a unit fired less.

## Decision Drivers

- **The maintainers' choices** (2026-11-11): read the inputs of the units that carried the drop on that cell, and run it once this ADR merges.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A read of weights changes no form, and any hypothesis still needs an ADR of its own.
- **The harness reads every synapse**: exploration 020's class matrix (`e020_classes`) walks each unit's fan-out (`exec.units()`, `unit.fan_out(exec.blocks())`), each synapse's target and weight, and sums them by the presynaptic unit's class and the target's. Summing them by target unit instead needs no change under `src/`.

## Considered Options

1. **The readout units' inputs on that cell**, six forms, two jobs.
2. **Seed 24's eighth image from the mirrored assignment, unit by unit**, the same seed and image, where after block 18 and 24 alone lay above the night without replay right after their nights, two jobs.
3. **Pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.
4. **Write a hypothesis now**, built by a brief and measured by a round.

## Decision Outcome

**Option 1.**

### Exploration 034 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **Its cell** is seed 24's eighth image from the assignment alone, on H-47's paths 0, 5 and 6, read sixteen times at blocks 6, 12, 18, 24 and 36 in H-47's terms.
- **Its forms** are exploration 033's six: the night without replay in each job, `alone-6`, `alone-12`, `alone-18` and `alone-24`, and `plain-first`, H-47's plain night on all four nights. No hold is set.
- **Its input read**: on each image whose class matrix exploration 033's job writes, the start image, the image before each night and after it, and block 36's, for each unit of the two readouts' sets, each synapse onto it summed by the class of its presynaptic unit as exploration 020's class matrix names it, fourteen sums a unit; an inhibitory unit's weight negated, so a + is more inhibition, as in the class matrix. Summed over the units of each of the eight classes the readouts' units make up, C's own, D's own, and each readout's pattern targets, other excitatory and inhibitory units, each presynaptic class's sums are that class matrix's entry, 112 entries an image, held inside the test.
- **Its branch** starts from `main` with exploration 033's tag merged in, for exploration 033's harness; that tag changes nothing under `src/`, so the engine is `main`'s. The branch adds its harness only; if reading a unit's inputs cannot be done without a change under `src/`, it stops and says why.

**Its jobs**, two, exploration 033's: one runs the night without replay, after block 6 alone and H-47's plain night, nine chains, as exploration 033's first job did in 5 850 s; the other the night without replay and after block 12, 18 or 24 alone, twelve chains, 7 429 s there. Each chain writes nine class matrices and each job one start image, so the input read dumps 82 images in the first job and 109 in the second, 12 852 sums an image, about 1.05 and 1.40 million sums, one more walk of the fan-out an image beside the class matrix's; at about seven characters a sum that is some 7 and 10 MB more of logs, which the job's artifact keeps. Exploration 033's jobs used at most 51.6 per cent of an arms job's 240 minutes and wrote 6.6 and 7.8 MB of logs.

**Inside each job** the rows are held to H-47's as in exploration 033's jobs (`held_to`), each form's replayed nights read replays and depotentiations that are not zero and its other nights none (`e030_night`), and the input read's sums are held as above. A's reads are counted unit by unit as in exploration 033. Every row, read and unit read is set beside exploration 033's in the note, and every line of exploration 033's jobs, label aside, is looked for in exploration 034's, which ties the class matrices to exploration 033's.

**Its trial** runs the pair's head and each form's first night on path 0 of that cell, as exploration 033's trial did: each holds H-47's row at block 6 on path 0 of the form whose first night it ran, and each input read's sums are the class matrix's entries.

It reads, on that cell, each image's inputs, the mean over the three paths, in the class matrix's units, sums of Q1.15 weights, written in millions (M) where they are large. **A night's change** in an input is the image after the night less the image before it; **a stretch's** the image before the next night, or block 36's, less the image after the night before it; **an input's difference** is the form's image less the night without replay's on the same path at the same point, a checkpoint or the image before a night. **The presynaptic groups** are A's kinds (classes 0 and 1), B's kinds (2 and 3), the odd places' excitatory units (4, 6 and 7), the even places' (5, 8 and 9), the other excitatory units (10) and the inhibitory units (11 to 13). On this cell class 10 holds only units 1 020 to 1 023, past the last whole period, and class 13 none, every inhibitory unit lying at places 4, 9, 14 or 19 of the readouts, so those two are read for completeness alone:
1. **The rows, reads and unit reads against the logs**: each form's, beside exploration 033's, so that every count and spike is that note's and is not read anew.
2. **What each replayed night changed in the inputs** of each readout's units, by presynaptic group and class, beside the night without replay's same night on the same image: on readout 1's and readout 0's units, and on the units that carried half of each readout's drop under the same form at the same checkpoint on each stimulus's trials, as exploration 033's sections 3 and 4 name them, each of the two sets against the rest of its readout; and the inputs on the image before each replayed night, block 6's beside 12's, 18's and 24's, exploration 033's question of what differs in the image each replayed night starts from. Each image's synaptic gain, which scales every unit's summed input, is set beside its inputs, from the rows and the night lines.
3. **Whether the inputs line up with the spikes**: for each readout and stimulus, the correlation over the readout's 459 units between a unit's input difference from each presynaptic group right after the replayed night and its spikes' difference there in exploration 033, a unit's input difference being the same on both stimuli's trials and only its spikes' difference differing with the stimulus; and whether readout 1's units lost more input from A's kinds than readout 0's after block 12, 18 or 24 alone and not after block 6 alone, as their spikes on A's trials did. The readouts' sums by kind are in the logs' splits, which print the kinds in the order A alone, A and C, C's own, B alone, B and D, D's own, so that classes 2 and 3 are the splits' kinds 3 and 4; that sum is read from them too, as a check of the input read.
4. **The nights and blocks after the replayed night**: what each later night and each stretch of blocks changed in the same inputs, to block 36, beside exploration 033's units' recovery by block 36.
5. **H-47's plain night** on all four nights the same way, its nights so far together at blocks 6 to 24 and at block 36.
No SE is claimed for an input: an input is a mean of three images, one a path.

Before it spends a job it states whether any form must run bit for bit as one already in the logs: the six forms' images, rows and reads are exploration 033's, but no log holds a unit's inputs, so each form runs, and its rows and reads are set beside exploration 033's.
It does not run any other cell; replay on two nights or on every night but one; any hold; plain nights on both or the awake gap; the pair's reads unit by unit; the units outside the readouts' sets, their spikes or their inputs; a unit's membrane, threshold or delays; any path but H-47's three; any part of a night apart, such as slow-wave sleep from REM; or the next day's reading of the pairings a night leaves.

## Consequences

- **Good**: on the cell where a later single replayed night lowered A's answer's readout more than the other, what each night changed in the weights onto each readout unit is read, by presynaptic class, beside the units' spikes.
- **Good**: the units that carried each readout's drop are read against the rest of their readout.
- **Bad**: one cell; an input pattern read there says nothing of the cells where single nights lay above.
- **Bad**: a unit's weights are not its drive: its membrane, its threshold and the spikes of the presynaptic units outside the readouts are not read, and the synaptic gain scales every unit's summed input, so a weight's change does not say how many spikes it moved.
- **Bad**: the forms run again to read inputs, though their images are in the logs by checksum.
- **Bad**: the harness change is on a branch that `main` does not take.
- **Bad**: M11 stays open; any hypothesis needs its own ADR, brief and round.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, the mirrored cell unit by unit**: it would read whether the same few units fall where A rose, not what the night changed in what they receive.
- **Option 3, pause M11**: the cell where replay lowers A's answer's readout more than the other after a later single night is read down to its units' spikes, and one step short of what changed them.
- **Option 4, a hypothesis now**: no single replayed night lies above the night without replay at block 36 in all twelve cells, so a prediction would rest on a split reading.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 034's note, when it merges, holds what it read.
