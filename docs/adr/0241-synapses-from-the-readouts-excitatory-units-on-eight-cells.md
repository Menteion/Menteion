---
status: proposed
date: 2026-11-08
depends-on: ADR-0240
title: Synapses from the readouts' excitatory units held on H-47's other eight cells
kind: decision
summary: Exploration 025 held the synapses from the readouts' excitatory units through plain nights, and A's count stood above the plain night's beyond 2 SE at 8 of 20 checkpoints, in three of four cells. No hypothesis is written; exploration 026 runs that hold on H-47's other eight cells.
decision-makers: Menteion maintainers
---

# ADR-0241: Synapses from the readouts' excitatory units held on H-47's other eight cells

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 026 runs H-47's plain nights on A's patterns with exploration 025's excitatory hold set, on H-47's eight cells that explorations 020 to 025 did not run. Its branch builds on exploration 025's tag, and `main` takes nothing from it. It runs once this ADR merges.
- **Why.** Under exploration 025's excitatory night A's count stood within 2 SE of H-47's plain night's or above it at every checkpoint of its four cells, above at 8 of the 20. Those four are the cells where stimulus A answers readout 1, H-47's two that failed clause 1 among them. In H-47's other eight cells clause 1 held, and no hold has run there.
- **Left open.** Whether a hypothesis follows exploration 026; the held units' synapses split by target; exploration 024's hold on the other eight cells; what the next day does with the pairings a hold leaves; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0240](0240-synapses-from-the-readouts-excitatory-units-held.md) ordered exploration 025: H-47's plain night on A's patterns with every synapse from the readouts' excitatory units held, by presynaptic unit, the pair's own units excepted, from the night's first tick to its last. It ran alone and with exploration 024's hold, on exploration 020's four cells. Its readings are exploratory, not evidence ([exploration 025](../explorations/025-synapses-from-the-readouts-excitatory-units.md)):

- **The hold held.** In all 144 nights with it, each of the 21 248 held synapses ended the night at the weight it started with, its trace cleared, and the quiet ticks after the night moved none of them. The night without replay ran H-47's, row for row.
- **Under the excitatory night A's count less H-47's plain night's** lies between −0.3 and +7.1, beyond twice exploration 018's unpaired SE at 8 of the 20 checkpoints, all above. Seed 24's eighth image from the assignment is above from block 12 on, seed 23's first mapping from the mirrored assignment at blocks 18 and 36, and seed 22's eighth image at blocks 12 and 24. Seed 22's first mapping lies within 2 SE throughout.
- **At block 36 the excitatory night less the night without replay** is −1.7 ± 0.42, +8.8 ± 1.28, +4.7 ± 1.52 and +4.6 ± 0.66 on those four cells, where H-47's clause 1 read −4.9 ± 1.09, +1.8 ± 1.79, +5.0 ± 1.24 and +2.2 ± 0.65.
- **The side is H-47's at 19 of the 20 checkpoints.** Each held night ran the plain night's replays and depotentiations, night by night.
- **Exploration 024's hold added** moved A's count by −2.4 to +0.6 from the excitatory night's, beyond 2 SE at three checkpoints, all below.
- **Without replay the excitatory hold** lay between −6.7 and +4.4 from the night without replay, below it beyond 2 SE on seed 24's eighth image at blocks 24 and 36.
- **What it left**: under the excitatory night the inhibition onto each group's excitatory units rose by 0.73 to 1.52 M more than under the plain night, and pending gains sit on about half the held synapses. The note does not part the hold's share from the rest.

**H-47's other eight cells.** Clause 1 held in each, plain on A's less without replay at block 36 from +3.8 ± 1.57 to +19.6 ± 1.06 ([H-47](../research/H-47.md)). Stimulus A answers readout 0 in six of them, and readout 1 on seed 23's eighth image from the assignment and seed 24's first mapping from the mirrored assignment. [Exploration 021](../explorations/021-where-the-gain-falls.md) ran exploration 020's job on all eight, so their class matrices are in its logs. Where A answers readout 0, each readout's excitatory units onto all lost 7.54 to 11.10 M over four plain nights, and the inhibition leaned toward A's readout by 1.35 to 3.71 M.

## Decision Drivers

- **The maintainers' choices** (2026-11-08): run exploration 025's excitatory hold on H-47's other eight cells, and run it once this ADR merges.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A held night is another form, so a hypothesis on it needs an ADR of its own.
- **H-47 asked its clauses of twelve cells.** The excitatory hold has run on four, all where A answers readout 1.

## Considered Options

1. **Explore the excitatory hold on H-47's other eight cells**, one job a cell.
2. **Explore the held units' synapses split by target** on exploration 025's four cells, those onto the other readout's excitatory units apart from the rest.
3. **Write a hypothesis on the excitatory night now**, built by a brief and measured by a round.
4. **Pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.

## Decision Outcome

**Option 1.**

### Exploration 026 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **The hold** is exploration 025's excitatory hold, unchanged: from the night's first tick to its last no synapse from a held unit consolidates, onto whatever target, and each held synapse's trace is cleared at the night's last tick. The held units are every excitatory unit of H-47's two readouts but the pair's own, counted and printed in each cell, none a source of the zero sum. Every synapse outside the hold consolidates as in H-47.
- **Its branch** starts from `main` with exploration 025's tag merged in, which carries 024's and 025's changes under `src/`. It changes nothing there; `main` takes nothing from it.
- **Its cells** are H-47's eight that explorations 020 to 025 did not run: seed 22's eighth image from the mirrored assignment and its first mapping from the assignment; seed 23's eighth image from the assignment and from the mirrored assignment, and its first mapping from the assignment; seed 24's eighth image from the mirrored assignment, and its first mapping from the assignment and from the mirrored assignment.

**Its job, one a cell**, runs three forms. On exploration 025's figures, 7 441 to 7 704 s for four, three take about 5 600 to 5 800 s, within an arms job's 240 minutes:
- **the night without replay**, its rows held to H-47's. It runs H-47's night bit for bit, and runs to hold the branch to H-47's rows over the whole run, as 024's and 025's did;
- **the excitatory night**: H-47's plain night on A's patterns with the excitatory hold set;
- **the excitatory night without replay**, to read the hold apart from the replay.

H-47's plain night is read from H-47's logs, and its class matrices and night lines from exploration 021's job on these eight cells.

**Its trial** runs the pair's head and each form's first night on path 0:
- on seed 23's first mapping from the assignment, with no hold set, the night without replay and the plain night hold H-47's row at block 6 on path 0, bit for bit; with the excitatory hold set, every held synapse ends the night at the weight it started with and its trace cleared, and an excitatory synapse from a unit outside the hold moves;
- on seed 23's first mapping from the mirrored assignment, the excitatory night holds the row exploration 025's trial printed at block 6 on path 0 (run 42468595193), bit for bit.

It reads at blocks 6, 12, 18, 24 and 36, in H-47's terms:
1. **A's count and each stimulus's trials** under each held form: less the night without replay, with H-47's paired SE, the forms run in one job; and the excitatory night less H-47's plain night, with exploration 018's unpaired SE.
2. **The side under each held form**: the stimulus whose difference from the night without replay is the larger, by stimulus and by group of units, against H-47's side on the same cell.
3. **The pair** under each held form and A's highest coupling, as clauses 2 and 3 read them.
4. **What each night moved**, from the class matrix by group as exploration 025 reads it, the held units' synapses split by target and their change shown nil.
5. **What the hold leaves**, as exploration 025 read it: what the quiet ticks after each night move, and the pairings still pending, the gain and the loss apart.
6. **All twelve cells together**: exploration 025's four beside these eight at block 36, the excitatory night less the night without replay and less H-47's plain night, by the readout A answers.

Before it spends a job it states whether any form but the night without replay must run bit for bit as one already in the logs, and runs none that does.
It does not run exploration 024's hold on these cells, alone or with 025's, nor plain nights on both, the awake gap, a hold through the quiet ticks, or the held units' synapses split by target.

## Consequences

- **Good**: the hold is read on all twelve of H-47's cells, among them every cell where A answers readout 0 and every cell where clause 1 held.
- **Bad**: the hold is a change under `src/` that `main` does not take, so what the exploration reads is not the engine's.
- **Bad**: exploration 024's hold, alone or with 025's, stays read on four cells only.
- **Bad**: eight jobs of about 95 minutes each.
- **Bad**: M11 stays open, and no hypothesis is in view.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, the held units' synapses split by target**: it would read which part of the hold carries the excitatory night's gain on four cells, all where A answers readout 1, and leave the other eight unread under any hold.
- **Option 3, a hypothesis now**: four cells are read under the hold, none where A answers readout 0. On seed 23's first mapping from the mirrored assignment the excitatory night still stood 1.7 ± 0.42 under the night without replay at block 36 (025).
- **Option 4, pause M11**: the excitatory night stood above the plain night beyond 2 SE in three of four cells (025), and what it does where replay already helped is unread.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 026's note, when it merges, holds what it read.
