---
status: proposed
date: 2026-11-10
depends-on: ADR-0246
title: Each later night's replay alone, on twelve cells
kind: decision
summary: Replay after block 6 alone lay above the night without replay at block 36 beyond 2 SE in five of H-47's twelve cells, below in one and within 2 SE in six. Exploration 032 reads replay after block 12, 18 or 24 alone on the same cells.
decision-makers: Menteion maintainers
---

# ADR-0247: Each later night's replay alone, on twelve cells

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 032 reads three forms on all twelve of H-47's cells, each H-47's plain night on A's patterns after one block alone, 12, 18 or 24, with the night without replay after the other three, beside the night without replay. One job a cell on eleven cells runs the four forms; on seed 23's first mapping from the mirrored assignment exploration 030 already ran them, and they are read from its job. Exploration 031's form replaying after block 6 alone is read beside them from its jobs. The branch adds its harness only, on an engine that is `main`'s, and `main` takes nothing from it. It runs once this ADR merges and the maintainers start it.
- **Why.** At block 36, replay after block 6 alone lay above the night without replay beyond 2 SE in five cells, +1.7 ± 0.78 to +7.3 ± 0.65, below on seed 24's eighth image from the assignment, −2.0 ± 0.88, and within 2 SE in six. Only on seed 23's first mapping from the mirrored assignment has any other single night's replay been read, so these runs do not say whether the night after block 6 does it or any one replayed night would.
- **Left open.** Whether a hypothesis follows exploration 032, which would need an ADR, a brief and a round of its own; replay on two of the four nights, or on every night but one, on any cell but seed 23's first mapping from the mirrored assignment; what the blocks after the replayed night move on A's and B's trials on seed 24's eighth image from the assignment; what the twelve blocks after block 24's night move on A's trials under replay after block 24 alone; a read unit by unit; the night held onto the inhibitory units and the units outside the readouts without replay, or split finer; the part held across, or exploration 024's hold, beside any part; the synapses no hold reached; what the next day does with the pairings a night leaves; what keeps the night held within below the excitatory night on seed 24's eighth image from the assignment, and what moves A's trials below the plain night's on seed 23's eighth image from the mirrored assignment, each on that cell alone; exploration 024's hold on the other eight cells; plain nights on both and the awake gap night by night; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0246](0246-the-first-nights-replay-on-twelve-cells.md) ordered exploration 031: H-47's plain night on A's patterns after block 6 alone, the later nights without replay, beside the night without replay on all twelve of H-47's cells. Its readings are exploratory, not evidence ([exploration 031](../explorations/031-the-first-nights-replay-on-twelve-cells.md)):

- **At block 6**, where the form's rows are H-47's plain night's own, it lies above the night without replay beyond 2 SE in eight cells and within 2 SE in four.
- **At block 36, less the night without replay**, it lies above beyond 2 SE in five cells, +1.7 ± 0.78 to +7.3 ± 0.65: four of the six where A answers readout 0, and seed 23's first mapping from the mirrored assignment, where A answers readout 1. It lies below beyond 2 SE on seed 24's eighth image from the assignment, −2.0 ± 0.88, where H-47's clause 1 lies within 2 SE, and within 2 SE in the other six, −0.9 ± 1.07 to +5.0 ± 2.61. Of the ten cells where H-47's clause 1 holds, it lies above beyond 2 SE in four and within 2 SE in six.
- **Less H-47's plain night at block 36**, beyond twice exploration 018's unpaired SE, it lies below in seven cells, −2.8 ± 1.01 to −13.1 ± 1.09, within in four, and above on seed 23's first mapping from the mirrored assignment, +6.6 ± 0.90.
- **By stimulus**, of the five cells where it lies above the night without replay beyond 2 SE at block 36, the difference lies beyond 2 SE on B's trials alone in three, on A's alone in one and on both in one. The side at block 36 falls on H-47's stimulus in ten cells.
- **The later nights.** Each of its nights after blocks 12, 18 and 24, without replay after the replayed night, left each of the class matrix's four inhibitory columns lower than H-47's night without replay after the same block did, in all 36, and moved A's margin by −0.21 to +0.15 M, where H-47's later plain nights raised it by 0.54 to 1.17 M.
- **The other single nights** have been read only on seed 23's first mapping from the mirrored assignment ([exploration 030](../explorations/030-one-cell-night-by-night.md)): at block 36, less the night without replay, replay after block 12 alone read −0.5 ± 1.11, after block 18 alone +0.3 ± 0.83 and after block 24 alone −2.4 ± 0.86, below beyond 2 SE; the four single nights sum to −0.9, where H-47's four nights read −4.9 ± 1.09.

## Decision Drivers

- **The maintainers' choices** (2026-11-10): read replay after block 12, 18 or 24 alone on all twelve cells, and start it only when they say so once this ADR merges.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A form that replays on one night only is another form, so a hypothesis on it needs an ADR of its own.
- **Exploration 031's harness** runs exploration 030's forms on any cell from a list of forms (`e030_job` and `e030_trial` take the forms; `E030_ALONE` holds the four single nights), on an engine that is `main`'s, so these forms need no change under `src/`.

## Considered Options

1. **Replay after block 12, 18 or 24 alone on all twelve cells**, beside the night without replay, one job a cell.
2. **Seed 24's eighth image from the assignment night by night**, the one cell where replay after block 6 alone lies below the night without replay at block 36.
3. **Replay on two of the four nights**, the six pairs, on one cell or on all twelve.
4. **Every night but one on all twelve cells**, exploration 030's four forms leaving a night out.
5. **Write a hypothesis on replay after block 6 alone now**, built by a brief and measured by a round.
6. **Pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.

## Decision Outcome

**Option 1.**

### Exploration 032 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **Its cells** are H-47's twelve, the six where A answers readout 0 and the six where it answers readout 1, on H-47's paths 0, 5 and 6, read sixteen times at blocks 6, 12, 18, 24 and 36 in H-47's terms.
- **Its forms** are exploration 030's night without replay, run as the job's control, which holds inside the job that the harness runs H-47's rows on each cell and pairs the forms for their SEs, and its three forms replaying after block 12, 18 or 24 alone (`E030_ALONE`'s last three): H-47's plain night on A's patterns after that block, A's 102 patterns drawn from the start image, tagged, and H-47's night without replay after the other three blocks. No hold is set.
- **Beside them** it reads exploration 031's form replaying after block 6 alone, from exploration 031's jobs and, on seed 23's first mapping from the mirrored assignment, exploration 030's. Every read of the night without replay in exploration 031's jobs held H-47's, so the form less the night without replay keeps its paired SE.
- **Its branch** starts from `main` with exploration 031's tag merged in, for exploration 031's harness on exploration 030's and 020's; that tag changes nothing under `src/`, so the engine is `main`'s. The branch adds its harness only.

**Its jobs**, one a cell on the other eleven, each run the night without replay, its rows held to H-47's, and the three forms. On seed 23's first mapping from the mirrored assignment all four are read from exploration 030's one-night-alone job (run 42621480151). Exploration 031's jobs of two forms, every night watched, took 3 863 to 4 236 s, so each job of four should take about 129 to 141 minutes, within an arms job's 240. Inside each job, every row of the night without replay is held to H-47's, and each form's rows at the blocks before its replayed night to H-47's night without replay's; each form's night after its block reads replays and depotentiations that are not zero, and every other night of every form reads `replays 0` and `depotentiations 0`. H-47's plain night on A's patterns is read from H-47's run 41441234613, and its class matrices, night lines and watch from exploration 020's runs 41500459562 and 41510955777 for exploration 020's four cells and from exploration 021's run 41591077514 for the other eight.

**Its trial** runs the pair's head and each form's first night on path 0, on seed 23's first mapping from the assignment, a cell that runs a job: each form's first night is without replay, so each holds, bit for bit, H-47's night without replay's row at block 6 on path 0, the image's CRC-64, its reads, A's couplings, their highest over the head and the gain.

It reads at blocks 6, 12, 18, 24 and 36, in H-47's terms. A form's rows are H-47's night without replay's up to the block before its replayed night, and from that night on they part from both of H-47's forms:
1. **A's count and each stimulus's trials** under each form: less the night without replay, with H-47's paired SE, the forms run in one job; and less H-47's plain night, with exploration 018's unpaired SE.
2. **The four single nights at block 36** on each cell, replay after block 6 alone among them: each less the night without replay and their sum, beside H-47's plain night less it, by stimulus.
3. **The side** under each form, by stimulus and by group of units, against H-47's side on the same cell.
4. **The pair** under each form and A's highest coupling, as clauses 2 and 3 read them.
5. **What each night and each stretch moved**, from the class matrix by group, as exploration 030 reads it, each replayed night beside H-47's plain night after the same block and night without replay on the same cell from exploration 020's and 021's jobs.
6. **The engine's two sums** over each night, the replays, depotentiations and the watch's wrong answers, the ripples and pairings of each replayed night, and the ties.
7. **The twelve cells side by side**: at block 36, each form less the night without replay with the job's paired SE, and less H-47's plain night with exploration 018's unpaired SE, each with its stimulus split, beside replay after block 6 alone and H-47's clause 1, the rows ordered by the readout A answers. No figure is pooled across cells.

Before it spends a job it states whether any form but the night without replay must run bit for bit as one already in the logs, and runs none that does: the forms on seed 23's first mapping from the mirrored assignment are exploration 030's, so that cell runs no job; on the other eleven, no log holds H-47's plain night on an image the night without replay's chain wrote, so each form parts from both of H-47's forms at its replayed night.
It does not run replay after block 6 alone again, on two nights or on every night but one, on any cell; any hold; plain nights on both or the awake gap; any read unit by unit; any path but H-47's three; any part of a night apart, such as slow-wave sleep from REM; or the next day's reading of the pairings a night leaves.

## Consequences

- **Good**: whether replay on one later night alone lies above the night without replay at block 36 on each of H-47's twelve cells is read, in H-47's terms, beside replay after block 6 alone and H-47's clause 1, and so whether the first night's reading there is its own or any one night's.
- **Good**: on every cell the four single nights are read side by side, and their sum beside H-47's four nights together.
- **Bad**: no form replays on two nights or leaves one out on the eleven cells, so where single nights do not add up to H-47's plain night the reading does not say which nights together do.
- **Bad**: H-47's plain night and replay after block 6 alone are read from other jobs, so a form less H-47's plain night carries an unpaired SE.
- **Bad**: the forms are a change of the harness on a branch that `main` does not take.
- **Bad**: eleven jobs of about 129 to 141 minutes each, a quarter of each the night without replay, run as the job's control though its rows are H-47's.
- **Bad**: M11 stays open; any hypothesis on a form needs its own ADR, brief and round.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, seed 24's eighth image from the assignment**: it would read the one cell where the first night's replay alone lies below the night without replay, and not whether any other single night lies above it on the other eleven.
- **Option 3, two nights**: six forms a cell; on all twelve cells a job of seven forms would take about 225 to 247 minutes, near or past an arms job's 240, and on one cell it says nothing of the other eleven.
- **Option 4, every night but one**: it reads what a night's absence costs where every other night replays, a question after the one asked here, whether one night replayed alone does anything at all.
- **Option 5, a hypothesis now**: replay after block 6 alone lies above the night without replay beyond 2 SE in five cells of twelve and below in one, so a prediction on it would rest on a reading split across the cells.
- **Option 6, pause M11**: a single replayed night that lay above the night without replay in five cells is not yet read after any other block on eleven of them.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 032's note, when it merges, holds what it read.
