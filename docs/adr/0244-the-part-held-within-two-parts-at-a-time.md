---
status: proposed
date: 2026-11-09
depends-on: ADR-0243
title: The part held within, two parts at a time
kind: decision
summary: Exploration 028 read A's count under each part of the part held within, held alone, beyond 2 SE of H-47's plain night's at 3, 6 and 13 of 60 checkpoints, where exploration 027's night held within lay beyond at 17. No hypothesis is written; exploration 029 holds the three parts two at a time.
decision-makers: Menteion maintainers
---

# ADR-0244: The part held within, two parts at a time

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 029 holds exploration 028's three parts of the part held within two at a time, each pair being the part held within with one part left to consolidate, through H-47's plain nights on A's patterns, on all twelve of H-47's cells. Its branch builds on exploration 028's tag, whose hold already takes any set of parts, and adds only its harness; `main` takes nothing from it. It runs once this ADR merges.
- **Why.** Under each part held alone A's count lay beyond 2 SE of H-47's plain night's at 3 to 13 of the 60 checkpoints, and none of the three lay within 2 SE of exploration 027's night held within at more than 52. No job of exploration 028 holds more than one part, so it does not say whether what the night held within read needs two of its parts held, or all three.
- **Left open.** Whether a hypothesis follows exploration 029; the part held across held beside any part; what keeps the night held within and every single part's night below the excitatory night at block 36 on seed 24's eighth image from the mirrored assignment and on seed 23's and seed 24's eighth images from the assignment, read on those cells alone; what moves A's trials below the plain night's on seed 23's eighth image from the mirrored assignment, read on that cell alone; exploration 024's hold on the other eight cells; what the next day does with the pairings a hold leaves; a finer split, such as the inhibitory units by readout or the units outside the readouts by class; exploration 024's hold beside any part; the inhibitory synapses onto the readouts' inhibitory units and the excitatory synapses onto the readouts from A's, B's and the pair's own units, which no hold reaches; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0243](0243-the-part-held-within-split-by-target.md) ordered exploration 028: exploration 027's part held within split in three by target, the held units' synapses onto their own group's excitatory units, onto the inhibitory units and onto the excitatory units outside the two readouts, each part held alone through H-47's plain nights on A's patterns, on all twelve of H-47's cells. Its readings are exploratory, not evidence ([exploration 028](../explorations/028-the-part-held-within-by-target.md)):

- **Each hold held its part.** In all 432 held nights every synapse of the held part ended the night at the weight it started with, its trace cleared, while the other parts' synapses moved, and the quiet ticks after the night moved none. The parts are, by seed, 6 891 to 7 030 synapses onto the own group, 4 487 to 4 543 onto the inhibitory units and 2 339 to 2 366 onto the units outside the readouts, together exploration 027's part held within. The night without replay ran H-47's, row for row.
- **Less H-47's plain night**, A's count lies beyond twice exploration 018's unpaired SE at 3 of the 60 checkpoints under the night held onto the own group, 1 above and 2 below; at 6 under the night held onto the units outside the readouts, all above; and at 13 under the night held onto the inhibitory units, 7 above and 6 below. Exploration 027's night held within lay beyond at 17 and the excitatory night at 25.
- **Where the excitatory night lies beyond the plain night**, 25 checkpoints, the night held onto the inhibitory units lies beyond it in the same direction at 12, and 12 of its own 13 are among them; the night held onto the units outside the readouts does so at 4 and the night held onto the own group at 1.
- **Less exploration 027's night held within**, A's count lies beyond 2 SE at 8 of the 60 checkpoints under the night held onto the inhibitory units, all below; at 13 under the night held onto the units outside the readouts, 4 above and 9 below; and at 17 under the night held onto the own group, 3 above and 14 below. Less the excitatory night it lies beyond at 11, 19 and 20. Each of the three lies below the excitatory night beyond 2 SE at block 36 on seed 24's eighth image from the mirrored assignment and on seed 23's and seed 24's eighth images from the assignment.
- **The side under each held night is H-47's at 55 to 58 of the 60 checkpoints**; at block 36 under the night held onto the inhibitory units it is the excitatory night's in eleven of the twelve cells.
- **What the other parts moved.** Each part held alone left the other two parts and the part held across moving within 0.2 M a group of the plain night's change over four nights. The inhibition onto each group's excitatory units rose by 0.16 to 0.96 M more than under the plain night under the night held onto the inhibitory units, by −0.04 to +0.24 M under the night held onto the own group and by −0.03 to +0.11 M under the night held onto the units outside the readouts, where exploration 027's night held within's rose by 0.33 to 1.11 M more.
- **The held nights ran H-47's plain night's replays and depotentiations in 140 to 142 of their 144 nights each**, all others on seed 23's eighth image from the mirrored assignment.

## Decision Drivers

- **The maintainers' choices** (2026-11-09): hold the three parts two at a time, all three pairs, on all twelve cells, and run it once this ADR merges.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A held night is another form, so a hypothesis on it needs an ADR of its own.
- **Exploration 028's note** lists first among the questions it suggests two parts held together, the part onto the inhibitory units with each of the other two. With the third pair, the own group's part with the part onto the units outside the readouts, the three pairs are the part held within less each of its parts in turn, so that each part is read both alone (exploration 028) and as the one part left free (here).

## Considered Options

1. **Hold the three parts two at a time, all three pairs, on all twelve cells**, one job a cell.
2. **Hold only the two pairs with the part onto the inhibitory units**, as exploration 028's note names them, on all twelve cells.
3. **Read what keeps every single part's night below the excitatory night** on seed 24's eighth image from the mirrored assignment and seed 23's and seed 24's eighth images from the assignment, on those cells alone.
4. **Read what the next day does with the pairings a hold leaves**, on the parts exploration 028 held.
5. **Pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.
6. **Write a hypothesis on a held night now**, built by a brief and measured by a round.

## Decision Outcome

**Option 1.**

### Exploration 029 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **The held units, the targets and the three parts** are exploration 028's: every excitatory unit of H-47's two readouts but the pair's own, 664 in each cell, each with its readout's group; the targets with a group, every excitatory unit of the two readouts, the pair's own among them, 714 in each cell; and the part held within split by target into the part held onto the own group, the part held onto the inhibitory units and the part held onto the units outside the readouts, as the class matrix's columns hold them in exploration 028.
- **The three pairs**: the own group's part with the inhibitory units' part, which leaves the units outside the readouts' part to consolidate; the own group's part with the units outside the readouts' part, which leaves the inhibitory units' part; and the inhibitory units' part with the units outside the readouts' part, which leaves the own group's part. On seed 23's first mapping from the assignment they hold 11 406, 9 285 and 6 853 synapses.
- **A pair's hold**, in each form that sets it, is exploration 028's hold with both of its parts set: from the night's first tick to its last no synapse of either part consolidates, slot by slot, and each one's trace is cleared at the night's last tick. Every other synapse, the part left free's and the part held across's among them, consolidates as in H-47 and keeps its trace at the night's last tick. Each held night holds, inside the test, every synapse of both parts at its first weight with its trace cleared, and a synapse of the part left free moved; it prints how many synapses moved of the part left free and of the part held across. That none of the held units is a source of the zero sum is held inside the test, as explorations 025, 027 and 028 held it.
- **Its branch** starts from `main` with exploration 028's tag merged in, which carries explorations 024's, 025's, 027's and 028's changes under `src/`; exploration 028's `Executor::set_night_hold_by_target_parts` already holds any set of parts together, so the branch adds its harness and nothing under `src/`. `main` takes nothing from it.
- **Its cells** are H-47's twelve, the six where A answers readout 0 and the six where it answers readout 1.

**Its job, one a cell**, runs four forms, as exploration 028's did, whose jobs took 7 315 to 7 791 s, so each should take about 120 to 135 minutes, within an arms job's 240 minutes:
- **the night without replay**, its rows held to H-47's. It runs H-47's night bit for bit, and runs to hold the branch to H-47's rows over the whole run and to pair the held forms with;
- **the night held onto the own group and the inhibitory units**: H-47's plain night on A's patterns with that pair held;
- **the night held onto the own group and the units outside the readouts**: the same with that pair held;
- **the night held onto the inhibitory units and the units outside the readouts**: the same with that pair held.

H-47's plain night is read from H-47's logs, its class matrices and night lines from exploration 020's runs 41500459562 and 41510955777 for exploration 020's four cells and from exploration 021's run 41591077514 for the other eight. The excitatory night is read from exploration 025's job (run 42469251250) for exploration 020's four cells and exploration 026's (run 42489793510) for the other eight, exploration 027's night held within from its job (run 42514033935), and each part held alone from exploration 028's job (run 42552893754).

**Its trial** runs the pair's head and each form's first night on path 0, on seed 23's first mapping from the assignment:
- with no hold set, the night without replay and the plain night hold H-47's row at block 6 on path 0, bit for bit;
- each pair is exactly its two parts, synapse by synapse, its count printed, and the three parts are disjoint and together exactly the synapses exploration 027's part held within names, 13 772 on this cell;
- with each pair's hold set, every synapse of both parts ends the night at the weight it started with and its trace cleared, a synapse of the part left free moves, and at the night's last tick some synapse of the held units outside the pair still carries a trace that is not zero;
- with the three parts held together, the plain night on A's patterns holds the row exploration 027's trial printed for the night held within at block 6 on path 0 (run 42512634982: the image's CRC-64, A's first count of 57, its couplings and the gain), bit for bit, as exploration 028's trial held it;
- each held form's row at block 6 on path 0 is printed beside H-47's plain night's, exploration 027's trial's night held within's and exploration 028's trial's rows for the two parts held alone (run 42551926531) on the same cell.

It reads at blocks 6, 12, 18, 24 and 36, in H-47's terms:
1. **A's count and each stimulus's trials** under each held form: less the night without replay and less each other held form, with H-47's paired SE, the forms run in one job; and less H-47's plain night, less exploration 027's night held within, less the excitatory night and less exploration 028's night for each of the pair's two parts held alone, each with exploration 018's unpaired SE.
2. **The side under each held form**, by stimulus and by group of units, against H-47's side, the excitatory night's, exploration 027's night held within's and exploration 028's nights' on the same cell.
3. **The pair** under each held form and A's highest coupling, as clauses 2 and 3 read them.
4. **What each night moved**, from the class matrix by group as exploration 028 reads it: the held units' synapses broken down by target, the two held parts' change shown nil and the part left free's and the part held across's read beside the plain night's, and the inhibition onto each group's excitatory units.
5. **What the hold leaves**, as exploration 028 read it: what the quiet ticks after each night move, and the pairings still pending on the held pair's synapses, the gain and the loss apart.
6. **The twelve cells side by side**: at block 36, each held form less the night without replay with the job's paired SE, and less H-47's plain night, less exploration 027's night held within, less the excitatory night and less exploration 028's night for each of the pair's two parts held alone with 018's unpaired SE, the rows ordered by the readout A answers. No figure is pooled across cells.

Before it spends a job it states whether any form but the night without replay must run bit for bit as one already in the logs, and runs none that does (none can: each pair holds synapses that the plain night, exploration 027's night held across, exploration 024's held night and each of exploration 028's three nights move, and lets consolidate synapses that exploration 027's night held within, the excitatory night and exploration 025's night under both holds hold, as the trial's moved synapses show; the held forms without replay in the logs run no replay).
It does not run any part held alone again, the three parts together but in the trial, the part held across alone or beside any part, exploration 024's hold, exploration 027's night held within or the excitatory night again, any part held in the night without replay, any finer split, plain nights on both, the awake gap, or a hold through the quiet ticks.

## Consequences

- **Good**: each of the three pairs, the part held within less one part, is read on all twelve cells, against exploration 027's night held within, exploration 028's parts held alone and the plain night on the same cells.
- **Bad**: the hold is a change under `src/` that `main` does not take, so what the exploration reads is not the engine's.
- **Bad**: exploration 027's night held within, exploration 028's parts held alone and the excitatory night are read from other jobs, so a held form less any of them carries an unpaired SE.
- **Bad**: with no held form without replay, a pair's hold is not read apart from the replay.
- **Bad**: twelve jobs of about 120 to 135 minutes each.
- **Bad**: M11 stays open, and no hypothesis is in view.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, the two pairs with the inhibitory units' part**: it would save a form a job, but leave unread the part held within less the inhibitory units' part, so it would not say whether what the night held within read needs that part, the part whose night, held alone, lay nearest the excitatory night's.
- **Option 3, the three cells alone**: they are where every single part's night lies below the excitatory night at block 36, and they would not say whether two parts held together carry the night held within's count on the other nine.
- **Option 4, the next day's reading of the pending pairings**: it reads what a hold leaves in the trace, not which parts together move A's count.
- **Option 5, pause M11**: what the night held within read is not yet parted between two of its parts and all three.
- **Option 6, a hypothesis now**: no reading yet says which parts held together, if any short of all three, move A's count as the night held within does, so a prediction would name none.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 029's note, when it merges, holds what it read.
