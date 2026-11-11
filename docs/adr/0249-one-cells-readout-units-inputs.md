---
status: proposed
date: 2026-11-11
depends-on: ADR-0248
title: One cell's readout units' inputs
kind: decision
summary: On seed 24's eighth image from the assignment every replayed night lowered both readouts' spikes on both stimuli's trials, and which readout fell more went with A's correct trials. Exploration 034 reads the weights onto each readout unit, by the presynaptic unit's class, across each night.
decision-makers: Menteion maintainers
---

# ADR-0249: One cell's readout units' inputs

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 034 reads the same cell as exploration 033, seed 24's eighth image from the assignment, under its six forms. On each image it reads, for each of the two readouts' 918 units, the weights onto the unit summed by the class of the presynaptic unit, and reads what each night and each stretch of blocks changed in them, the units that carried each readout's drop in exploration 033 beside the rest. Two jobs run the six forms. The branch adds its harness only, on an engine that is `main`'s, and `main` takes nothing from it. It runs once this ADR merges.
- **Why.** Right after each single replayed night both readouts fired less on both stimuli's trials, by 18 to 33 per cent, and on A's trials readout 1 less readout 0 read +0.48 ± 0.29 after block 6 alone and −1.46 ± 0.22 to −3.73 ± 0.16 after block 12, 18 or 24 alone. Exploration 033 counted the units' spikes, not what they receive, so it does not say why A's answer's readout fell more after a later night than after the first.
- **Left open.** Whether a hypothesis follows exploration 034, which needs its own ADR, brief and round; any other cell, read unit by unit or by inputs, seed 24's eighth image from the mirrored assignment among them; a unit's membrane, threshold or delays; the inputs of the units outside the readouts' sets; replay on two of the four nights on any cell, or on every night but one on any cell but seed 23's first mapping from the mirrored assignment; the pair's reads unit by unit; the night held onto the inhibitory and outside units without replay, or split finer; the part held across, or exploration 024's hold, beside any part; what the next day does with the pairings a night leaves; what keeps the night held within below the excitatory night on seed 24's eighth image from the assignment; what moves A's trials below the plain night's on seed 23's eighth image from the mirrored assignment; why after block 12 alone lies below the night without replay on seed 23's eighth image from the assignment; plain nights on both and the awake gap night by night; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0248](0248-one-cells-readouts-unit-by-unit.md) ordered exploration 033: on seed 24's eighth image from the assignment, the night without replay, H-47's plain night on A's patterns after block 6, 12, 18 or 24 alone and on all four nights, each readout unit's spikes counted on each of A's trials. Its readings are exploratory, not evidence ([exploration 033](../explorations/033-one-cells-readouts-unit-by-unit.md)):

- **Every row and read is the logs'**: each form's rows and reads are those of H-47's, exploration 031's and exploration 032's jobs, so A's count is those notes'.
- **Right after each single replayed night both readouts fired less on both stimuli's trials**, each beyond 2 SE: on A's trials readout 1 by 7.45 to 10.19 spikes a trial and readout 0 by 6.03 to 7.92, on B's trials readout 0 by 6.27 to 6.77 and readout 1 by 5.31 to 6.21, 18 to 33 per cent of the night without replay's spikes. Every class of both readouts fired less, each beyond 2 SE. On this cell A asks readout 1, the even places, of stimulus A and readout 0, the odd places, of B.
- **The answer's readout less the other**: on A's trials readout 1 less readout 0 read +0.48 ± 0.29 against the night without replay after block 6 alone, within 2 SE, and −1.46 ± 0.22, −2.96 ± 0.19 and −3.73 ± 0.16 after block 12, 18 and 24 alone, beyond 2 SE; its sign is the sign of A's trials less the night without replay in each comparison.
- **Few units carry the drop**: 95 to 136 of a readout's 459 units moved down beyond twice their own SE and 0 to 5 up, where about 23 would by chance; 13 to 19 carry half of each readout's drop, about as many as carry half of the night without replay's spikes. By class on A's trials, after block 12, 18 or 24 alone readout 1's pattern targets fired 4.63 to 5.14 less a trial and its inhibitory units 3.75 to 4.12 less, where after block 6 alone 3.70 and 2.86 less.
- **At block 36** each single night still lay below the night without replay on both readouts and both stimuli's trials, by 1.28 to 4.49, each beyond 2 SE, having come back by 3.10 to 5.84 since right after its replayed night.
- **By group**, each replayed night moved the class matrix on that cell about as H-47's first plain night did (explorations 031 and 032).
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
- **Its input read**: on each image whose class matrix exploration 033's job writes, the start image, the image before each night and after it, and block 36's, for each unit of the two readouts' sets, each synapse onto it summed by the class of its presynaptic unit as exploration 020's class matrix names it, fourteen sums a unit; an inhibitory unit's weight negated, so a + is more inhibition, as in the class matrix. Summed over the units of a target class, each presynaptic class's sums are that class matrix's entry, held inside the test.
- **Its branch** starts from `main` with exploration 033's tag merged in, for exploration 033's harness; that tag changes nothing under `src/`, so the engine is `main`'s. The branch adds its harness only; if reading a unit's inputs cannot be done without a change under `src/`, it stops and says why.

**Its jobs**, two, exploration 033's: one runs the night without replay, after block 6 alone and H-47's plain night, nine chains, as exploration 033's first job did in 5 850 s; the other the night without replay and after block 12, 18 or 24 alone, twelve chains, 7 429 s there. The input reads add their dumps to that; each job should stay within an arms job's 240 minutes. Inside each job the rows are held to H-47's as in exploration 033's jobs (`held_to`), each form's replayed nights read replays and depotentiations that are not zero and its other nights none (`e030_night`), and the input read's sums are held as above. A's reads are counted unit by unit as in exploration 033. Every row, read and unit read is set beside exploration 033's in the note.

**Its trial** runs the pair's head and each form's first night on path 0 of that cell, as exploration 033's trial did: each holds H-47's row at block 6 on path 0 of the form whose first night it ran, and each input read's sums are the class matrix's entries.

It reads, on that cell, each image's inputs, the mean over the three paths, in the class matrix's units, sums of Q1.15 weights, written in millions (M) where they are large:
1. **The rows, reads and unit reads against the logs**: each form's, beside exploration 033's, so that every count and spike is that note's and is not read anew.
2. **What each replayed night changed in the inputs** of each readout's units, the image after the night less the image before it, by presynaptic class, beside the night without replay's same night on the same image: on readout 1's and readout 0's units, and on the units that carried half of each readout's drop on A's trials and on B's in exploration 033 against the readout's other units.
3. **Whether the inputs' change lines up with the spikes'**: for each readout and stimulus, the correlation over the readout's units between a unit's change in input from each group of presynaptic classes and its spikes' difference right after the replayed night in exploration 033; and whether readout 1's units lost more input from stimulus A's kinds, A alone and A and C, than readout 0's after block 12, 18 or 24 alone, and not after block 6 alone, as their spikes on A's trials did.
4. **The blocks after the replayed night**: what each stretch of blocks changed in the same inputs, to block 36, beside exploration 033's units' recovery by block 36.
5. **H-47's plain night** on all four nights the same way, its nights so far together at blocks 6 to 24 and at block 36.
No SE is claimed for an input: an image is one per path.

Before it spends a job it states whether any form must run bit for bit as one already in the logs: the six forms' images, rows and reads are exploration 033's, but no log holds a unit's inputs, so each form runs, and its rows and reads are set beside exploration 033's.
It does not run any other cell; replay on two nights or on every night but one; any hold; plain nights on both or the awake gap; the pair's reads unit by unit; the inputs of the units outside the readouts' sets; a unit's membrane, threshold or delays; any path but H-47's three; any part of a night apart, such as slow-wave sleep from REM; or the next day's reading of the pairings a night leaves.

## Consequences

- **Good**: on the cell where a later single replayed night lowered A's answer's readout more than the other, what each night changed in the weights onto each readout unit is read, by presynaptic class, beside the units' spikes.
- **Good**: the units that carried each readout's drop are read against the rest of their readout.
- **Bad**: one cell; an input pattern read there says nothing of the cells where single nights lay above.
- **Bad**: a unit's weights are not its drive: its membrane, threshold and the spikes of its presynaptic units are not read, so a weight's change does not say how many spikes it moved.
- **Bad**: the forms run a third time to read inputs, though their images are in the logs by checksum.
- **Bad**: the harness change is on a branch that `main` does not take.
- **Bad**: M11 stays open; any hypothesis needs its own ADR, brief and round.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, the mirrored cell unit by unit**: it would read whether the same few units fall where A rose, not what the night changed in what they receive.
- **Option 3, pause M11**: the cell where replay lowers A's answer's readout more than the other after a later night is read down to its units' spikes, and one step short of what changed them.
- **Option 4, a hypothesis now**: no single replayed night lies above the night without replay at block 36 in all twelve cells, so a prediction would rest on a split reading.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 034's note, when it merges, holds what it read.
