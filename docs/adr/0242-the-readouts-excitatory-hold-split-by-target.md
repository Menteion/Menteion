---
status: proposed
date: 2026-11-09
depends-on: ADR-0241
title: The readouts' excitatory hold split by target
kind: decision
summary: Explorations 025 and 026 read A's count at block 36 under the excitatory hold within 2 SE of the plain night's or above it in all six cells where A answers readout 1, and below it beyond 2 SE in three of six where it answers readout 0. No hypothesis is written; exploration 027 splits the hold.
decision-makers: Menteion maintainers
---

# ADR-0242: The readouts' excitatory hold split by target

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 027 splits exploration 025's excitatory hold in two by target and runs each part alone through H-47's plain nights on A's patterns, on all twelve of H-47's cells. One part holds the held units' synapses onto the other readout's excitatory units, the other part the rest of them. Its branch builds on exploration 026's tag, and `main` takes nothing from it. It runs once this ADR merges.
- **Why.** Under H-47's plain night the held units' synapses lost most onto the other readout's excitatory units, in every one of the twelve cells (explorations 025 and 026). Under the whole hold, as both explorations read it, A's count stood within 2 SE of the plain night's or above it at every checkpoint of the six cells where A answers readout 1, and below it beyond 2 SE at block 36 in three of the six where it answers readout 0, and above it beyond 2 SE in two of them. Neither exploration holds one target and not another, so neither says which held synapses carry the rise or the fall.
- **Left open.** Whether a hypothesis follows exploration 027; exploration 024's hold on the other eight cells; what moves A's trials below the plain night's on seed 23's eighth image from the mirrored assignment, read on that cell alone; the inhibitory synapses onto the readouts' inhibitory units and the excitatory synapses onto the readouts from A's, B's and the pair's own units, which no hold reaches; what the next day does with the pairings a hold leaves; a finer split by target, such as the synapses onto the inhibitory units alone; exploration 024's hold beside either part; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0241](0241-synapses-from-the-readouts-excitatory-units-on-eight-cells.md) ordered exploration 026: exploration 025's excitatory hold, unchanged, on H-47's eight cells outside exploration 020's four. The hold keeps every synapse from the readouts' excitatory units, by presynaptic unit, the pair's own units excepted as sources, from consolidating from a night's first tick to its last, and clears its trace at the last. Its readings are exploratory, not evidence ([exploration 026](../explorations/026-the-excitatory-hold-on-eight-cells.md)):

- **The hold held.** In all 192 held nights each of the 21 248 held synapses ended the night at the weight it started with, its trace cleared, and the quiet ticks after the night moved none of them. The night without replay ran H-47's, row for row.
- **Under the excitatory night A's count less H-47's plain night's** lies between −4.0 and +8.1 on the eight cells, beyond twice exploration 018's unpaired SE at 17 of the 40 checkpoints, 11 above and 6 below. The six below are all where A answers readout 0: seed 23's eighth image from the mirrored assignment from block 12 on, and seed 22's eighth image from the mirrored assignment and first mapping from the assignment at block 36.
- **At block 36, over H-47's twelve cells** (exploration 025's job for exploration 020's four cells, 026's for the other eight): less H-47's plain night, the excitatory night lies beyond 2 SE below in three cells, all where A answers readout 0, and above in five, two where A answers readout 0 and three where it answers readout 1. In all six where A answers readout 1 it lies within 2 SE or above. Less the night without replay it lies beyond 2 SE above in eleven cells and below in one, seed 23's first mapping from the mirrored assignment, −1.7 ± 0.42.
- **The side under the excitatory night is H-47's at 35 of the 40 checkpoints.** The excitatory night ran the plain night's replays and depotentiations in 94 of its 96 nights; the two others, on one path of seed 23's eighth image from the mirrored assignment, each read one replay or one depotentiation more.
- **What the plain night moved in the held units' synapses**, over four nights, the mean over the three paths: −4.73 to −6.98 M of each group's onto the other readout's excitatory units in the twelve cells, the largest part in each (explorations 025 and 026). On exploration 026's eight cells the rest was −0.02 to −2.39 M onto their own group, −1.45 to −2.71 M onto the inhibitory units, −0.97 to +0.12 M onto A's and B's units and −0.06 to +0.06 M onto the other excitatory units.
- **What it left**: under the excitatory night the inhibition onto each group's excitatory units rose by 0.69 to 1.92 M more than under the plain night, and pending gains sit on about half the held synapses. The note does not part the hold's share from the rest.

## Decision Drivers

- **The maintainers' choices** (2026-11-09): split the hold by target into two parts, each run alone, on all twelve cells, and run it once this ADR merges.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A held night is another form, so a hypothesis on it needs an ADR of its own.
- **Exploration 026's note** lists first among the questions it suggests the held units' synapses held by target, the part onto the other readout's excitatory units apart from the rest; neither exploration 025 nor 026 holds one target and not another.

## Considered Options

1. **Split the hold by target into two parts, each alone, on all twelve cells**, one job a cell.
2. **Hold only the part onto the other readout's excitatory units** on all twelve cells, one job a cell of two forms.
3. **Read what moves A's trials below the plain night's on seed 23's eighth image from the mirrored assignment**, on that cell alone.
4. **Pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.
5. **Write a hypothesis on the excitatory night now**, built by a brief and measured by a round.

## Decision Outcome

**Option 1.**

### Exploration 027 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **The held units** are exploration 025's: every excitatory unit of H-47's two readouts but the pair's own, 664 in each cell. **A readout's excitatory units**, as targets, are every excitatory unit of that readout, the pair's own units in it among them, as explorations 025 and 026 split the held synapses by target.
- **The two parts.** The part held across holds each held unit's synapses onto the other readout's excitatory units. The part held within holds its other synapses: onto its own readout's excitatory units, the inhibitory units, A's and B's units and every other target, as the class matrix's columns name them. Together they are exploration 025's 21 248 held synapses in each cell.
- **A part's hold**, in each form that sets it: from the night's first tick to its last no synapse of that part consolidates, and each one's trace is cleared at the night's last tick, as exploration 025's hold does its synapses. Every synapse outside the part, the other part's among them, consolidates as in H-47 and keeps its trace at the night's last tick. Each held night holds, inside the test, every synapse of its part at its first weight with its trace cleared, and a synapse of the other part moved. That none of the held units is a source of the zero sum is held inside the test, as exploration 025 held it.
- **Its branch** starts from `main` with exploration 026's tag merged in, which carries explorations 024's and 025's changes under `src/`, and adds the hold by target there. `main` takes nothing from it.
- **Its cells** are H-47's twelve: exploration 020's four, where A answers readout 1, and exploration 026's eight, six of them where A answers readout 0.

**Its job, one a cell**, runs three forms. Exploration 026's three-form jobs took 5 702 to 6 129 s, and a form costs about the same with or without replay (exploration 020's two-form jobs, 024's three and 025's four), so each job should take about 95 to 105 minutes, within an arms job's 240 minutes:
- **the night without replay**, its rows held to H-47's. It runs H-47's night bit for bit, and runs to hold the branch to H-47's rows over the whole run and to pair the held forms with;
- **the night held across**: H-47's plain night on A's patterns with the part held across;
- **the night held within**: H-47's plain night on A's patterns with the part held within.

H-47's plain night is read from H-47's logs, its class matrices and night lines from exploration 020's runs 41500459562 and 41510955777 for 020's four cells, and from exploration 021's run 41591077514 of 020's job for the other eight. The excitatory night, exploration 025's whole hold, is read from exploration 025's job (run 42469251250) for exploration 020's four cells and exploration 026's (run 42489793510) for the other eight.

**Its trial** runs the pair's head and each form's first night on path 0, on two cells:
- on seed 23's first mapping from the assignment, with no hold set, the night without replay and the plain night hold H-47's row at block 6 on path 0, bit for bit; with each part's hold set, every synapse of that part ends the night at the weight it started with and its trace cleared, a synapse of the other part moves, at the night's last tick some synapse of the other part still carries a trace that is not zero, and the two parts are disjoint and together are exactly the synapses exploration 025's hold names, 21 248, synapse by synapse, each part's count printed;
- on seed 23's first mapping from the mirrored assignment, the plain night on A's patterns with both parts held together holds the row exploration 025's trial printed at block 6 on path 0 (run 42468595193: the image's CRC-64, A's first count of 58, its couplings and the gain), bit for bit.

It reads at blocks 6, 12, 18, 24 and 36, in H-47's terms:
1. **A's count and each stimulus's trials** under each held form: less the night without replay, with H-47's paired SE, the forms run in one job; less the other held form, paired likewise; and less H-47's plain night and less the excitatory night, each with exploration 018's unpaired SE.
2. **The side under each held form**: the stimulus whose difference from the night without replay is the larger, by stimulus and by group of units, against H-47's side and the excitatory night's on the same cell.
3. **The pair** under each held form and A's highest coupling, as clauses 2 and 3 read them.
4. **What each night moved**, from the class matrix by group as exploration 026 reads it: the held units' synapses broken down by target, the held part's change shown nil and the other part's change read beside the plain night's, and the inhibition onto each group's excitatory units.
5. **What the hold leaves**, as exploration 026 read it: what the quiet ticks after each night move, and the pairings still pending on the held synapses, the gain and the loss apart.
6. **The twelve cells side by side**: at block 36, each held form less the night without replay with the job's paired SE, and less H-47's plain night and less the excitatory night with 018's unpaired SE, the rows ordered by the readout A answers. No figure is pooled across cells.

Before it spends a job it states whether any form but the night without replay must run bit for bit as one already in the logs, and runs none that does (neither held form can: each holds synapses the plain night moves, and lets consolidate synapses the excitatory night holds, as the trial's moved synapse shows).
It does not run exploration 024's hold, alone or with either part, the excitatory night again, either part held in the night without replay, any finer split by target, such as the synapses onto the inhibitory units alone, plain nights on both, the awake gap, or a hold through the quiet ticks.

## Consequences

- **Good**: each part of the hold is read alone on all twelve cells, against the whole hold and the plain night on the same cells.
- **Bad**: the hold by target is a change under `src/` that `main` does not take, so what the exploration reads is not the engine's.
- **Bad**: the excitatory night is read from other jobs, so a held form less it carries an unpaired SE.
- **Bad**: with no held form without replay, a part's hold is not read apart from the replay.
- **Bad**: twelve jobs of about 95 to 105 minutes each.
- **Bad**: M11 stays open, and no hypothesis is in view.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, the part held across alone**: it would save one form a job, but leave unread what the part held within does alone, against the whole hold and the plain night.
- **Option 3, seed 23's eighth image from the mirrored assignment alone**: it is one of the three cells where exploration 026 read the excitatory night below the plain night beyond 2 SE at block 36, and it would not say which part of the hold carries that fall.
- **Option 4, pause M11**: what the whole hold read in explorations 025 and 026 is not yet parted between its targets.
- **Option 5, a hypothesis now**: under the excitatory night A's count stood below H-47's plain night's beyond 2 SE at block 36 in three cells where A answers readout 0, and below the night without replay on seed 23's first mapping from the mirrored assignment, by 1.7 ± 0.42 (025, 026).

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 027's note, when it merges, holds what it read.
