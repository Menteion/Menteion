---
status: proposed
date: 2026-11-09
depends-on: ADR-0242
title: The part held within split by target
kind: decision
summary: Exploration 027 read A's count under the night held within no further than 2 SE from the excitatory night's at 56 of 60 checkpoints, and under the night held across no further than 2 SE from H-47's plain night's at 55. No hypothesis is written; exploration 028 splits the part held within by target.
decision-makers: Menteion maintainers
---

# ADR-0243: The part held within split by target

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 028 splits exploration 027's part held within in three by target and runs each part alone through H-47's plain nights on A's patterns, on all twelve of H-47's cells: the held units' synapses onto their own group's excitatory units, onto the inhibitory units, and onto every excitatory unit outside the two readouts, A's and B's units and the other excitatory units. Its branch builds on exploration 027's tag, and `main` takes nothing from it. It runs once this ADR merges.
- **Why.** Under the night held within, A's count lay beyond 2 SE of the excitatory night's at 4 of exploration 027's 60 checkpoints; its side was the excitatory night's at 56 and H-47's at 54. The part held within holds four targets together, and exploration 027 holds none of them apart, so it does not say which of them, alone or together, carries the excitatory night's count.
- **Left open.** Whether a hypothesis follows exploration 028; the part held across held beside any part of the part held within; what keeps the night held within below the excitatory night on seed 24's eighth image from the assignment, read on that cell alone; what moves A's trials below the plain night's on seed 23's eighth image from the mirrored assignment, read on that cell alone; exploration 024's hold beside any part; a finer split, such as the inhibitory units by readout; what the next day does with the pairings a hold leaves; the inhibitory synapses onto the readouts' inhibitory units and the excitatory synapses onto the readouts from A's, B's and the pair's own units, which no hold reaches; exploration 024's hold on the other eight cells; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0242](0242-the-readouts-excitatory-hold-split-by-target.md) ordered exploration 027: exploration 025's excitatory hold split by target into two parts, each held alone through H-47's plain nights on A's patterns, on all twelve of H-47's cells. The part held across is each held unit's synapses onto the other readout's excitatory units; the part held within is its other synapses. Its readings are exploratory, not evidence ([exploration 027](../explorations/027-the-excitatory-hold-by-target.md)):

- **Each hold held its part.** In all 288 held nights every synapse of the held part ended the night at the weight it started with, its trace cleared, and the quiet ticks after the night moved no synapse of either part. The two parts are 7 345 to 7 476 synapses held across and 13 772 to 13 903 held within, by seed, together exploration 025's 21 248. The night without replay ran H-47's, row for row.
- **Under the night held within**, A's count less the excitatory night's (explorations 025 and 026) lies beyond twice exploration 018's unpaired SE at 4 of the 60 checkpoints, all below and all on seed 24's eighth image from the assignment, from block 12 on. Less H-47's plain night's it lies beyond 2 SE at 17, 13 above and 4 below. Of the 25 checkpoints where explorations 025 and 026 read the excitatory night beyond 2 SE of the plain night, it lies beyond in the same direction at 14: 11 of exploration 026's 17 and 3 of exploration 025's 8.
- **Under the night held across**, A's count less H-47's plain night's lies beyond 2 SE at 5 of the 60 checkpoints, all above, and less the excitatory night's at 15, 3 above and 12 below.
- **The side under the night held within is the excitatory night's at 56 of the 60 checkpoints and H-47's at 54**, and the excitatory night's at block 36 in all twelve cells; under the night held across it is H-47's at 56 and the excitatory night's at 54. The night held within ran the excitatory night's replays and depotentiations in all 144 of its nights, and the night held across the plain night's in all 144; the two differ on two nights of one path of seed 23's eighth image from the mirrored assignment, by one replay or one depotentiation.
- **What a part moved with the other held**, a group's change over four nights, the mean over the three paths: under the night held within the part held across moved by 0.97 to 1.03 times the plain night's, and under the night held across the part held within by 1.04 to 1.20 times it. The inhibition onto each group's excitatory units rose by 0.33 to 1.11 M more than under the plain night under either.
- **What the plain night moved in the part held within, by target** (exploration 027's section 4): −0.02 to −2.79 M of each group's onto its own group's excitatory units, −1.45 to −2.71 M onto the inhibitory units, −0.97 to +0.12 M onto A's and B's units and −0.06 to +0.06 M onto the other excitatory units, over the twelve cells.

## Decision Drivers

- **The maintainers' choices** (2026-11-09): split the part held within in three by target, each part run alone, on all twelve cells, and run it once this ADR merges.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A held night is another form, so a hypothesis on it needs an ADR of its own.
- **Exploration 027's note** lists first among the questions it suggests the part held within split by target, its synapses onto their own group, onto the inhibitory units and onto A's and B's units each held alone. The other excitatory units, onto which the plain night moved −0.06 to +0.06 M a group, go with A's and B's units, so the three parts cover the part held within. Nothing in exploration 027 holds one of its targets and not another.

## Considered Options

1. **Split the part held within in three by target, each alone, on all twelve cells**, one job a cell.
2. **Hold only the synapses onto the inhibitory units**, the part held within's largest target under the plain night in 17 of the twelve cells' 24 groups, on all twelve cells.
3. **Read what keeps the night held within below the excitatory night on seed 24's eighth image from the assignment**, on that cell alone.
4. **Pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.
5. **Write a hypothesis on the night held within now**, built by a brief and measured by a round.

## Decision Outcome

**Option 1.**

### Exploration 028 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **The held units** are exploration 025's and 027's: every excitatory unit of H-47's two readouts but the pair's own, 664 in each cell, each with its readout's group. **A group's excitatory units**, as targets, are every excitatory unit of that readout, the pair's own units in it among them, as exploration 027 split the held synapses by target.
- **The three parts** split exploration 027's part held within by target, as the class matrix's columns name them. The part held onto the own group holds each held unit's synapses onto its own group's excitatory units. The part held onto the inhibitory units holds its synapses onto every inhibitory unit, those of each readout and elsewhere. The part held outside the readouts holds its synapses onto every excitatory unit outside the two readouts: A's and B's units and the other excitatory units. Together they are exploration 027's part held within in each cell.
- **A part's hold**, in each form that sets it, is exploration 027's: from the night's first tick to its last no synapse of that part consolidates, slot by slot, and each one's trace is cleared at the night's last tick. Every synapse outside the part, the other two parts' and the part held across's among them, consolidates as in H-47 and keeps its trace at the night's last tick. Each held night holds, inside the test, every synapse of its part at its first weight with its trace cleared, and a synapse of the held units outside the part moved; it prints how many synapses of each other part moved. That none of the held units is a source of the zero sum is held inside the test, as explorations 025 and 027 held it.
- **Its branch** starts from `main` with exploration 027's tag merged in, which carries explorations 024's, 025's and 027's changes under `src/`, and adds the three parts there. `main` takes nothing from it.
- **Its cells** are H-47's twelve, the six where A answers readout 0 and the six where it answers readout 1.

**Its job, one a cell**, runs four forms. Exploration 027's three-form jobs took 5 718 to 6 153 s, and a fourth form added 27 to 28 % where it was last added (exploration 025's four-form jobs took 7 441 to 7 704 s on the cells where exploration 024's three-form jobs took 5 796 to 6 057 s), so each four-form job should take about 120 to 137 minutes, within an arms job's 240 minutes:
- **the night without replay**, its rows held to H-47's. It runs H-47's night bit for bit, and runs to hold the branch to H-47's rows over the whole run and to pair the held forms with;
- **the night held onto the own group**: H-47's plain night on A's patterns with that part held;
- **the night held onto the inhibitory units**: H-47's plain night on A's patterns with that part held;
- **the night held outside the readouts**: H-47's plain night on A's patterns with that part held.

H-47's plain night is read from H-47's logs, its class matrices and night lines from exploration 020's runs 41500459562 and 41510955777 for exploration 020's four cells and from exploration 021's run 41591077514 for the other eight. The excitatory night is read from exploration 025's job (run 42469251250) for exploration 020's four cells and exploration 026's (run 42489793510) for the other eight, and exploration 027's two held nights from its job (run 42514033935).

**Its trial** runs the pair's head and each form's first night on path 0, on seed 23's first mapping from the assignment:
- with no hold set, the night without replay and the plain night hold H-47's row at block 6 on path 0, bit for bit;
- with each part's hold set, every synapse of that part ends the night at the weight it started with and its trace cleared, a synapse of each other part moves, and at the night's last tick some synapse of the held units outside the part still carries a trace that is not zero;
- the three parts are disjoint and together are exactly the synapses exploration 027's part held within names, 13 772 on this cell, synapse by synapse, each part's count printed;
- with the three parts held together, the plain night on A's patterns holds the row exploration 027's trial printed for the night held within at block 6 on path 0 (run 42512634982: the image's CRC-64, A's first count of 57, its couplings and the gain), bit for bit;
- each held form's row at block 6 on path 0 is printed beside H-47's plain night's and exploration 027's trial's night held within's on the same cell.

It reads at blocks 6, 12, 18, 24 and 36, in H-47's terms:
1. **A's count and each stimulus's trials** under each held form: less the night without replay and less each other held form, with H-47's paired SE, the forms run in one job; and less H-47's plain night, less exploration 027's night held within and less the excitatory night, each with exploration 018's unpaired SE.
2. **The side under each held form**, by stimulus and by group of units, against H-47's side, the excitatory night's and exploration 027's night held within's on the same cell.
3. **The pair** under each held form and A's highest coupling, as clauses 2 and 3 read them.
4. **What each night moved**, from the class matrix by group as exploration 027 reads it: the held units' synapses broken down by target, the held part's change shown nil and each other part's change read beside the plain night's, and the inhibition onto each group's excitatory units.
5. **What the hold leaves**, as exploration 027 read it: what the quiet ticks after each night move, and the pairings still pending on the held part's synapses, the gain and the loss apart.
6. **The twelve cells side by side**: at block 36, each held form less the night without replay with the job's paired SE, and less H-47's plain night, less exploration 027's night held within and less the excitatory night with 018's unpaired SE, the rows ordered by the readout A answers. No figure is pooled across cells.

Before it spends a job it states whether any form but the night without replay must run bit for bit as one already in the logs, and runs none that does (none can: each holds synapses the plain night and exploration 027's night held across move, and lets consolidate synapses exploration 027's night held within and the excitatory night hold, as the trial's moved synapses show).
It does not run exploration 024's hold, the part held across alone or beside any part, exploration 027's night held within or the excitatory night again, any part held in the night without replay, any finer split, such as the inhibitory units by readout, plain nights on both, the awake gap, or a hold through the quiet ticks.

## Consequences

- **Good**: each of the three parts of the part held within is read alone on all twelve cells, against exploration 027's night held within and the plain night on the same cells.
- **Bad**: the three parts are a change under `src/` that `main` does not take, so what the exploration reads is not the engine's.
- **Bad**: exploration 027's night held within and the excitatory night are read from other jobs, so a held form less either carries an unpaired SE.
- **Bad**: with no held form without replay, a part's hold is not read apart from the replay.
- **Bad**: twelve jobs of about 120 to 137 minutes each.
- **Bad**: M11 stays open, and no hypothesis is in view.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, the synapses onto the inhibitory units alone**: it would save two forms a job, but leave unread what the part held onto the own group and the part held outside the readouts do alone, though the plain night moved the own group's by up to 2.79 M in a group.
- **Option 3, seed 24's eighth image from the assignment alone**: it is the one cell where exploration 027 read the night held within below the excitatory night beyond 2 SE, and it would not say which target carries the count on the other eleven.
- **Option 4, pause M11**: what the part held within read in exploration 027 is not yet parted between its targets.
- **Option 5, a hypothesis now**: no reading yet names the target of the part held within that carries the excitatory night's count, so a prediction would name none.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 028's note, when it merges, holds what it read.
