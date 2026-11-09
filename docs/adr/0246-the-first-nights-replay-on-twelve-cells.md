---
status: proposed
date: 2026-11-10
depends-on: ADR-0245
title: The first night's replay alone, on twelve cells
kind: decision
summary: On seed 23's first mapping from the mirrored assignment, replay after block 6 alone lay above the night without replay at block 36, on B's trials, and of the single nights only replay after block 24 alone lay below it beyond 2 SE. Exploration 031 reads the first on all twelve of H-47's cells.
decision-makers: Menteion maintainers
---

# ADR-0246: The first night's replay alone, on twelve cells

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 031 reads one form on all twelve of H-47's cells: H-47's plain night on A's patterns after block 6 alone, with the night without replay after blocks 12, 18 and 24, beside the night without replay. One job a cell on the other eleven runs the two forms; on seed 23's first mapping from the mirrored assignment exploration 030 already ran the form, and it is read instead. Its branch adds its harness only, on an engine that is `main`'s, and `main` takes nothing from it. It runs once this ADR merges.
- **Why.** On seed 23's first mapping from the mirrored assignment, where H-47's plain night lies below the night without replay at block 36 beyond 2 SE, −4.9 ± 1.09, replay after block 6 alone lay above it beyond 2 SE, +1.7 ± 0.78, all of it on B's trials (A's 0.0 ± 0.56, B's +1.7 ± 0.40). Of the four single nights only replay after block 24 alone lay below it beyond 2 SE, −2.4 ± 0.86, within 2 SE of H-47's plain night, and the other three lay above H-47's plain night beyond 2 SE. No run has read the form after block 6 alone on any other cell.
- **Left open.** Whether a hypothesis follows exploration 031, which would need an ADR, a brief and a round of its own; replay on two of the four nights, on seed 23's first mapping from the mirrored assignment or elsewhere; replay after block 12, 18 or 24 alone, or on every night but one, on any cell but that one, its twin among them; what the twelve blocks after block 24's night move on A's trials under replay after block 24 alone; a read unit by unit; the night held onto the inhibitory units and the units outside the readouts without replay, or split finer; the part held across, or exploration 024's hold, beside any part; the synapses no hold reached; what the next day does with the pairings a night leaves; what keeps the night held within below the excitatory night on seed 24's eighth image from the assignment, and what moves A's trials below the plain night's on seed 23's eighth image from the mirrored assignment, each read on that cell alone; exploration 024's hold on the other eight cells; plain nights on both and the awake gap night by night; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0245](0245-one-cell-night-by-night.md) ordered exploration 030: on seed 23's first mapping from the mirrored assignment, H-47's plain night's replay on one of the four nights alone and on every night but one, each beside the night without replay. Its readings are exploratory, not evidence ([exploration 030](../explorations/030-one-cell-night-by-night.md)):

- **One night alone.** At block 36, less the night without replay, replay after block 6 alone reads +1.7 ± 0.78, above beyond twice the paired SE; after block 12 alone −0.5 ± 1.11 and after block 18 alone +0.3 ± 0.83, within; and after block 24 alone −2.4 ± 0.86, below. The four sum to −0.9, where H-47's four nights read −4.9 ± 1.09. Less H-47's plain night, after block 6, 12 or 18 alone lie above beyond twice exploration 018's unpaired SE, and after block 24 alone within, +2.5 ± 1.31. Right after its one replayed night, after block 6 alone read +1.9 ± 0.68 and after block 12 alone +1.6 ± 0.55, above beyond 2 SE, and after block 18 alone +0.6 ± 0.88 and after block 24 alone −0.3 ± 0.81, within.
- **Every night but one.** At block 36, less the night without replay, leaving out the night after block 6 reads −1.7 ± 0.54 and after block 18 −2.6 ± 1.10, below beyond 2 SE, and after block 12 −2.5 ± 1.66 and after block 24 −1.3 ± 1.40, within. Each lies above H-47's plain night beyond 2 SE, by +2.3 to +3.6.
- **By stimulus**, at block 36: under replay after block 6 alone A's trials read 0.0 ± 0.56 against the night without replay and B's +1.7 ± 0.40, so its count above the night without replay lies on B's trials, and its side lies on B, the odd places, as H-47's plain night's does. On A's trials each form leaving a night out lies below the night without replay beyond 2 SE, −2.7 to −4.0, where H-47's plain night read −5.2 ± 1.50, and of the single nights only after block 24 alone, −1.1 ± 0.24.
- **What a replayed night moved ran with how many replayed nights came before it**: a form's first replayed night took 3.35 to 3.99 M of the odd places' excitatory weight onto all, after any block; a second 2.59 to 3.00 M, a third 1.77 to 1.95 M, and H-47's fourth 1.53 M. C's own units lost margin onto A's answer over every night and every stretch of every form there, the night without replay's among them; on the twin each of H-47's plain nights raised it.
- **H-47's clause 1 by cell** ([H-47](../research/H-47.md)): plain less without replay at block 36 lies above beyond 2 SE in ten of the twelve cells, +2.2 to +19.6, within 2 SE on seed 24's eighth image from the assignment, +1.8 ± 1.79, and below on seed 23's first mapping from the mirrored assignment, −4.9 ± 1.09.

## Decision Drivers

- **The maintainers' choices** (2026-11-10): read replay after block 6 alone on all twelve cells, and run it once this ADR merges.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A form that replays on one night only is another form, so a hypothesis on it needs an ADR of its own.
- **Exploration 030's harness** already runs a form that replays on any set of nights (`E030Form`), on an engine that is `main`'s, so a form replaying after block 6 alone on another cell needs no change under `src/`.

## Considered Options

1. **Replay after block 6 alone on all twelve cells**, beside the night without replay, one job a cell.
2. **Replay on two of the four nights**, the six pairs, on seed 23's first mapping from the mirrored assignment.
3. **Exploration 030's eight forms on its twin**, seed 23's first mapping from the assignment.
4. **Every single night alone on all twelve cells**, four forms and the night without replay, five a job.
5. **Write a hypothesis on replay after block 6 alone now**, built by a brief and measured by a round.
6. **Pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.

## Decision Outcome

**Option 1.**

### Exploration 031 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **Its cells** are H-47's twelve, the six where A answers readout 0 and the six where it answers readout 1, on H-47's paths 0, 5 and 6, read sixteen times at blocks 6, 12, 18, 24 and 36 in H-47's terms.
- **Its forms** are exploration 030's night without replay, run as the job's control, which holds inside the job that the harness runs H-47's rows on each cell and pairs the form for its SE, and its form replaying after block 6 alone: H-47's plain night on A's patterns after block 6, A's 102 patterns drawn from the start image, tagged, and H-47's night without replay after blocks 12, 18 and 24. No hold is set.
- **Its branch** starts from `main` with exploration 030's tag merged in, for exploration 030's harness on exploration 020's job; that tag changes nothing under `src/`, so the engine is `main`'s. The branch adds its harness only.

**Its jobs**, one a cell on the other eleven, each run the night without replay, its rows held to H-47's, and the form replaying after block 6 alone. On seed 23's first mapping from the mirrored assignment both forms are read from exploration 030's one-night-alone job (run 42621480151). Exploration 020's job, two forms on one cell with every night watched, took 3 874 to 4 110 s on its four cells and on exploration 021's eight, so each job should take about 65 to 69 minutes, within an arms job's 240. Inside each job, every row of the night without replay is held to H-47's, and the form's row at block 6 on each path to H-47's plain night's; the form's night after block 6 reads replays and depotentiations that are not zero, and every other night of both forms reads `replays 0` and `depotentiations 0`. H-47's plain night on A's patterns is read from H-47's run 41441234613, and its class matrices, night lines and watch from exploration 020's runs 41500459562 and 41510955777 for exploration 020's four cells and from exploration 021's run 41591077514 for the other eight.

**Its trial** runs the pair's head and each form's first night on path 0, on seed 23's first mapping from the assignment, a cell that runs a job: each holds, bit for bit, H-47's row at block 6 on path 0, the image's CRC-64, its reads, A's couplings, their highest over the head and the gain, the night without replay H-47's night without replay's and the form H-47's plain night's.

It reads at blocks 6, 12, 18, 24 and 36, in H-47's terms. At block 6 the form's rows are H-47's plain night's, and from block 12 on they part from both of H-47's forms:
1. **A's count and each stimulus's trials** under the form: less the night without replay, with H-47's paired SE, the forms run in one job; and less H-47's plain night, with exploration 018's unpaired SE.
2. **The side** under the form, by stimulus and by group of units, against H-47's side on the same cell.
3. **The pair** under the form and A's highest coupling, as clauses 2 and 3 read them.
4. **What each night and each stretch moved**, from the class matrix by group, as exploration 030 reads it, beside H-47's plain night and night without replay on the same cell from exploration 020's and 021's jobs.
5. **The engine's two sums** over each night, the replays, depotentiations and the watch's wrong answers, and the ties.
6. **The twelve cells side by side**: at block 36, the form less the night without replay with the job's paired SE, and less H-47's plain night with exploration 018's unpaired SE, each with its stimulus split, beside H-47's clause 1, the rows ordered by the readout A answers. No figure is pooled across cells.

Before it spends a job it states whether any form but the night without replay must run bit for bit as one already in the logs, and runs none that does: the form on seed 23's first mapping from the mirrored assignment is exploration 030's, so that cell runs no job; on the other eleven, no log holds a night without replay after a replayed night, so the form parts from both of H-47's forms at the night after block 12. The replayed night after block 6 is H-47's first plain night on the same image, so what it holds is exploration 020's and 021's reading of that night, and the note sets it beside them rather than reading it anew.
It does not run replay after block 12, 18 or 24 alone, or on every night but one, or on two nights, on any cell; any hold; plain nights on both or the awake gap; any read unit by unit; any path but H-47's three; any part of a night apart, such as slow-wave sleep from REM; or the next day's reading of the pairings a night leaves.

## Consequences

- **Good**: whether replay after block 6 alone, the only one of exploration 030's eight forms above the night without replay beyond 2 SE at block 36 on seed 23's first mapping from the mirrored assignment, lies above it on each of H-47's twelve cells is read, in H-47's terms, beside H-47's clause 1.
- **Bad**: no other single night runs on the eleven cells, so where the form lies above, the reading does not say whether the night after block 6 does it or any one replayed night would.
- **Bad**: one form; a cell where it lies within 2 SE says nothing on why.
- **Bad**: H-47's plain night is read from another job, so the form less it carries an unpaired SE.
- **Bad**: the form is a change of the harness on a branch that `main` does not take.
- **Bad**: eleven jobs of about 65 to 69 minutes each, about half of each the night without replay, run as the job's control though its rows are H-47's.
- **Bad**: M11 stays open; any hypothesis on the form needs its own ADR, brief and round.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, two nights on the one cell**: it would finish that cell's sixteen forms, and say nothing of the other eleven.
- **Option 3, the twin**: it would read how H-47's plain night builds its +13.4 ± 1.56 there, and not whether the form lies above the night without replay on the other ten cells.
- **Option 4, every single night on all twelve cells**: four forms and the night without replay a job, eleven jobs of about 160 minutes, as exploration 030's one-night-alone job took, about two and a half times option 1's, where at block 36 only the night after block 6 alone read above the night without replay beyond 2 SE on seed 23's first mapping from the mirrored assignment.
- **Option 5, a hypothesis now**: one cell of twelve has read the form, so a prediction would rest on that cell alone.
- **Option 6, pause M11**: a form that lay above the night without replay on seed 23's first mapping from the mirrored assignment, where H-47's plain night lies below it, is not yet read on the other cells.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 031's note, when it merges, holds what it read.
