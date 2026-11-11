---
status: accepted
date: 2026-11-11
depends-on: ADR-0247
title: One cell's readouts, unit by unit
kind: decision
summary: On seed 24's eighth image from the assignment each later night's replay alone left A's count 7.3 to 12.3 below the night without replay right after it, while its class matrix by group moved about as H-47's first plain night's did there. Exploration 033 reads that cell's readout units one by one.
decision-makers: Menteion maintainers
---

# ADR-0248: One cell's readouts, unit by unit

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 033 reads one cell, seed 24's eighth image from the assignment, under six forms: the night without replay, H-47's plain night on A's patterns after block 6, 12, 18 or 24 alone, and on all four nights. In each of A's reads it counts each readout unit's spikes on each trial, from the executor's own spike train, and reads which units moved under each form. Two jobs run the six forms. The branch adds its harness only, on an engine that is `main`'s, and `main` takes nothing from it. It runs once this ADR merges.
- **Why.** Right after its replayed night, replay after block 12, 18 or 24 alone lay below the night without replay beyond 2 SE on that cell, −7.3 ± 0.88, −12.1 ± 0.63 and −12.3 ± 0.58, more on A's trials than on B's, where in six or seven cells of twelve each lay above. Its class matrix by group moved over each replayed night about as H-47's first plain night's did there. A read gives each readout's count, not which of its units fired.
- **Left open.** Whether a hypothesis follows exploration 033, which needs its own ADR, brief and round; a read unit by unit on any other cell, seed 24's eighth image from the mirrored assignment among them; replay on two of the four nights on any cell, or on every night but one on any cell but seed 23's first mapping from the mirrored assignment; the pair's reads unit by unit; the units outside the readouts' sets; the night held onto the inhibitory and outside units without replay, or split finer; the part held across, or exploration 024's hold, beside any part; the synapses no hold reached; what the next day does with the pairings a night leaves; what keeps the night held within below the excitatory night on seed 24's eighth image from the assignment; what moves A's trials below the plain night's on seed 23's eighth image from the mirrored assignment; why after block 12 alone lies below the night without replay on seed 23's eighth image from the assignment; exploration 024's hold on the other eight cells; plain nights on both and the awake gap night by night; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0247](0247-each-later-nights-replay-alone-on-twelve-cells.md) ordered exploration 032: H-47's plain night on A's patterns after block 12, 18 or 24 alone, the other nights without replay, beside the night without replay on all twelve of H-47's cells. Its readings are exploratory, not evidence ([exploration 032](../explorations/032-each-later-nights-replay-alone-on-twelve-cells.md)):

- **At block 36, less the night without replay**, after block 12 alone lies above beyond 2 SE in eight cells, within in three and below in one; after block 18 alone above in seven, within in four and below in one; after block 24 alone above in three, within in seven and below in two. After block 6 alone lay above in five, within in six and below in one ([exploration 031](../explorations/031-the-first-nights-replay-on-twelve-cells.md)). Less H-47's plain night each of the three later forms lies below beyond twice exploration 018's unpaired SE in eight to eleven cells.
- **Right after its replayed night**, less the night without replay, after block 12 alone lies above beyond 2 SE in seven cells, after block 18 alone in seven and after block 24 alone in six. On seed 24's eighth image from the assignment each lies below, −7.3 ± 0.88, −12.1 ± 0.63 and −12.3 ± 0.58, on A's trials by −5.2 ± 0.70, −8.1 ± 0.66 and −10.0 ± 0.33 and on B's by −2.1 ± 0.59, −4.0 ± 0.57 and −2.3 ± 0.62, each beyond 2 SE. There H-47's first plain night, replay after block 6 alone at block 6, read +0.3 ± 0.64, within 2 SE. The one other cell where a single later night lay below right after it is seed 23's eighth image from the assignment, after block 12 alone, −1.9 ± 0.88.
- **On that cell at block 36**, after block 6 alone reads −2.0 ± 0.88 and after block 18 and 24 alone −5.5 ± 0.95 and −4.6 ± 0.73, below beyond 2 SE, and after block 12 alone −2.1 ± 1.78, within. H-47's plain night less its night without replay reads −5.9 ± 1.53 and −5.9 ± 1.85 at blocks 18 and 24 and +1.8 ± 1.79 at block 36, the one cell where H-47's clause 1 lies within 2 SE. A answers readout 1, the even places, there.
- **What the replayed nights moved there**: over each, the excitatory units of each group lost 3.49 to 4.53 M of weight onto all and the inhibition onto each group's excitatory units rose by 3.55 to 3.84 M, where over H-47's first plain night on that cell they lost 4.33 and 4.90 M and the inhibition rose by 3.72 and 3.79 M. In every cell each replayed night moved the readouts' synapses by group about as H-47's first plain night did.
- **What a read gives** is each readout's spike count on each trial; which of a readout's units fired is in no log (exploration 032, "What did not run").

## Decision Drivers

- **The maintainers' choices** (2026-11-11): read seed 24's eighth image from the assignment unit by unit, and run it once this ADR merges.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A read that counts units changes no form, and any hypothesis still needs an ADR of its own.
- **The executor's train is public**: `Executor::train` gives the last spikes as `(tick, unit)` between ticks, the train each readout counts its window from (`Readout::count_window`), and the harness's per-trial observer receives the executor, `&mut`, after each trial, as `train` asks. The harness's night engine keeps a train of the arena's units times `spikes_per_unit` of a trial's ticks, one trial's bound, and `Task::check` refuses a smaller one (`TaskError::TrainTooSmall`), so a trial's window lies in the train at the trial's end. So each unit's spikes in a trial's window can be counted in the harness with no change under `src/`.

## Considered Options

1. **Seed 24's eighth image from the assignment, unit by unit**, six forms, two jobs.
2. **The same, with seed 24's eighth image from the mirrored assignment**, the same seed and image, where after block 18 and after block 24 alone lay above the night without replay right after their nights and after block 12 alone within 2 SE, four jobs.
3. **Replay on two of the four nights**, the six pairs, on all twelve cells.
4. **Write a hypothesis on replay after block 12 alone now**, built by a brief and measured by a round.
5. **Pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.

## Decision Outcome

**Option 1.**

### Exploration 033 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **Its cell** is seed 24's eighth image from the assignment alone, on H-47's paths 0, 5 and 6, read sixteen times at blocks 6, 12, 18, 24 and 36 in H-47's terms.
- **Its forms** are exploration 030's night without replay, run in each job as its control; `alone-6`, `alone-12`, `alone-18` and `alone-24`, H-47's plain night on A's patterns after that block alone, A's 102 patterns drawn from the start image, tagged, and the night without replay after the other blocks; and H-47's plain night on all four nights, `plain-first`, an `E030Form` replaying on every night. No hold is set.
- **Its unit read**: in each of A's reads, the read of the mapping in force at the start, for every trial and every unit of the two readouts' sets, the unit's spikes in that trial's readout window, counted from the executor's train; summed over the read's trials of each stimulus, and divided by that stimulus's trials in the read, the same in every form, for a unit's spikes a trial. The pair's reads are read as before.
- **Its branch** starts from `main` with exploration 032's tag merged in, for exploration 032's harness on exploration 031's, 030's and 020's; that tag changes nothing under `src/`, so the engine is `main`'s. The branch adds its harness only.

**Its jobs**, two: one runs the night without replay, after block 6 alone and H-47's plain night, nine chains; the other the night without replay and after block 12, 18 or 24 alone, twelve chains, as exploration 032's job on that cell did in 7 872 s. Chains run two at a time, five rounds in the first and six in the second, so the first should take about 110 minutes and the second about 131, within an arms job's 240; exploration 031's job of three rounds on that cell took 3 911 s. Neither estimate counts the unit read's dumps.

**Inside each job**, every row of the night without replay and of H-47's plain night is held to H-47's row of that form, after block 6 alone's row at block 6 to H-47's plain night's, and each later form's rows before its replayed night's block to H-47's night without replay's (`held_to`). Each form's replayed nights read replays and depotentiations that are not zero and every other night `replays 0` and `depotentiations 0` (`e030_night`). On each trial of each unit read, as a guard, the train still holds the whole readout window: it has let go no spike, or its oldest spike is from a tick before the window opens; and each readout's units' spikes sum to that readout's count of the trial (`Outcome::counts`).

**Its trial** runs the pair's head and each form's first night on path 0 of that cell, each image's read at block 6 counted unit by unit: the night without replay and after block 12, 18 or 24 alone hold H-47's night without replay's row at block 6 on path 0, and after block 6 alone and H-47's plain night its plain night's, the image's CRC-64, its reads, A's couplings, their highest over the head and the gain; and every unit read's sums are its readouts' counts.

It reads at blocks 6, 12, 18, 24 and 36, in H-47's terms:
1. **The reads against the logs**: every read's count and masks under each form beside the read of the same form, path, checkpoint and stream in the logs, H-47's for the night without replay and its plain night, exploration 031's for after block 6 alone and exploration 032's for the other three, so that A's count and its differences are those notes' and are not read anew.
2. **Each readout unit's spikes a trial**, on A's trials and on B's, under each form, the mean over the 48 reads; and each form less the night without replay, with H-47's paired SE over the 48 reads' differences.
3. **Right after each replayed night**, each single night's form less the night without replay at its replayed night's block, and H-47's plain night at blocks 6, 12, 18 and 24, its nights so far together: which units moved on A's trials and on B's, by readout and by class of unit as exploration 020's class matrix names it. On that cell A asks readout 1, the even places, of stimulus A and readout 0, the odd places, of B. On each stimulus's trials, how each of the two readouts' counts moved, with its paired SE. A unit moved where its difference lies beyond twice its own paired SE; about one unit in twenty, some 23 of a readout's 459, would lie there by chance in each comparison, and a count of units moved is read against that. Where a readout's own difference lies beyond 2 SE, the fewest units, ranked by their difference in the direction of its net change, whose differences reach half of it, the units named; beside it the gross change, the sum of the units' differences' magnitudes, and the units that moved against the net change.
4. **At block 36** the same, and each unit's difference there beside its difference right after the replayed night (read 3) and the change between them, taking up ADR-0247's question of what the blocks after the replayed night move on that cell's A's and B's trials. Each form less the night without replay is paired in its job; after block 6 alone less H-47's plain night is paired, both in the first job; and after block 12, 18 or 24 alone less H-47's plain night, across the two jobs, carries exploration 018's unpaired SE. A's count's differences keep the notes' figures and SEs (read 1), and where after block 6 alone and H-47's plain night now share a job, the note gives their count's paired SE beside exploration 031's unpaired one.
5. **Beside each**, the class matrix and the split by group over the replayed night as explorations 031 and 032 read them on that cell, from their logs, not read anew.

Before it spends a job it states whether any form must run bit for bit as one already in the logs: the six forms' images and reads are in the logs, but no log holds a unit's spikes, so each form runs, and its rows and reads are set beside the logs'.
It does not run any other cell; replay on two nights or on every night but one; any hold; plain nights on both or the awake gap; the pair's reads unit by unit; the units outside the readouts' sets; any path but H-47's three; any part of a night apart, such as slow-wave sleep from REM; or the next day's reading of the pairings a night leaves.

## Consequences

- **Good**: on the one cell where each replayed night after block 12, 18 or 24 alone lies below the night without replay right after it, the readout units' spikes are read where A's count fell: which units moved, in which readout and class, and how few carry half of each readout's net change.
- **Good**: H-47's plain night is read unit by unit on the same cell, where its clause 1 lies within 2 SE.
- **Bad**: one cell; a unit pattern read there says nothing of whether it holds on the cells where single nights lay above, seed 24's eighth image from the mirrored assignment among them.
- **Bad**: the forms are run again to read units, though their images and reads are in the logs.
- **Bad**: the units outside the readouts' sets, and the pair's reads, are not read unit by unit.
- **Bad**: the harness change is on a branch that `main` does not take.
- **Bad**: M11 stays open; any hypothesis needs its own ADR, brief and round.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, with seed 24's eighth image from the mirrored assignment**: it would set beside the cell one of the same seed and image where two of the three later single nights lay above the night without replay right after them, at four jobs; the maintainers chose the one cell first.
- **Option 3, two nights**: twenty-four jobs, two a cell, and it would read whether nights add up, not why one cell falls after a single replayed night.
- **Option 4, a hypothesis now**: replay after block 12 alone lay above the night without replay at block 36 in eight cells of twelve and below in one, so a prediction on it would rest on a reading split across the cells.
- **Option 5, pause M11**: the one cell where each later single replayed night lowers A's count at once is not yet read below the readout's count.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 033's note, when it merges, holds what it read.
