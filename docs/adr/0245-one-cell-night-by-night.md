---
status: proposed
date: 2026-11-09
depends-on: ADR-0244
title: One cell, night by night
kind: decision
summary: Exploration 029's night holding the inhibitory and outside parts read as the night held within at all 60 checkpoints, yet on seed 23's first mapping from the mirrored assignment it ends below the night without replay. Exploration 030 reads that cell one night's replay at a time.
decision-makers: Menteion maintainers
---

# ADR-0245: One cell, night by night

**In brief**
- **Decision.** No hypothesis is written, no constant moves, and the hold split by target stops at exploration 029. Exploration 030 reads one cell of H-47's twelve, seed 23's first mapping from the mirrored assignment, with H-47's plain night's replay on one night at a time: four forms, each replaying A's patterns on one of the four nights and on no other, beside the night without replay, in one job. It reads what each night and each stretch of blocks between the nights moved there, and what the night replayed held. Its branch adds its harness only, and `main` takes nothing from it. It runs once this ADR merges.
- **Why.** On this cell A's count under H-47's plain night lies below the night without replay at blocks 24 and 36, beyond 2 SE at block 36, on A's trials; and every night with replay run on it since, held or not, ends below the night without replay at block 36, beyond 2 SE in all but one. Under exploration 029's night held onto the inhibitory units and the units outside the readouts it is the one cell of twelve below at block 36. No run has replayed on some of its nights and not on others, so none says which night's replay, or which stretch after it, carries the loss.
- **Left open.** Whether a hypothesis follows exploration 030; the same reading on seed 23's first mapping from the assignment, on which this cell's places exchanged ran bit for bit, or on any other cell; a read unit by unit, which readout units fire on A's trials and on B's; replay on more than one night but all four; the night held onto the inhibitory units and the units outside the readouts without replay, or split finer; what the next day does with the pairings a hold leaves; what keeps the night held within below the excitatory night on seed 24's eighth image from the assignment, and what moves A's trials below the plain night's on seed 23's eighth image from the mirrored assignment, each read on that cell alone; exploration 024's hold on the other eight cells; plain nights on both and the awake gap night by night; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0244](0244-the-part-held-within-two-parts-at-a-time.md) ordered exploration 029: exploration 028's three parts of the part held within held two at a time through H-47's plain nights on A's patterns, on all twelve of H-47's cells. Its readings and those before it are exploratory, not evidence ([exploration 029](../explorations/029-the-part-held-within-two-parts-at-a-time.md)):

- **The night held onto the inhibitory units and the units outside the readouts**, the part held within with the own group's part left to consolidate, read A's count within twice exploration 018's unpaired SE of exploration 027's night held within at all 60 checkpoints, and of the excitatory night at 54. The other two pairs lay beyond 2 SE of the night held within at 7 and 13. At block 36, less the night without replay, the night held onto the inhibitory units and the units outside the readouts lies above beyond 2 SE in eleven of the twelve cells and below in one, seed 23's first mapping from the mirrored assignment, −2.7 ± 0.53, as exploration 027's night held within did there, −2.6 ± 0.65.
- **That cell under H-47** ([H-47](../research/H-47.md), [exploration 020](../explorations/020-what-a-replay-costs.md), section 6): A's count under the night without replay reads 55.0, 52.4, 52.0, 53.5 and 53.1 at blocks 6, 12, 18, 24 and 36, and under the plain night on A's patterns 56.9, 54.4, 52.4, 50.9 and 48.3. Plain less without replay reads +1.9 ± 0.68, +2.0 ± 1.26, +0.3 ± 1.17, −2.6 ± 1.30 and −4.9 ± 1.09, H-47's clause 1 at block 36. On A's trials it reads −5.0 ± 1.25 at block 24 and −5.2 ± 1.50 at block 36; on B's +2.4 ± 0.38 and +0.3 ± 0.88. Without replay A's trials rose from 28.5 at block 18 to 30.0 at block 24 ([exploration 021](../explorations/021-where-the-gain-falls.md), section 2).
- **Every night with replay run on that cell since**, less the night without replay at block 36: exploration 024's night with the inhibition held, −4.8 ± 1.45; exploration 025's excitatory night, −1.7 ± 0.42; exploration 027's nights held within, −2.6 ± 0.65, and held across, −2.7 ± 1.61, within 2 SE; exploration 028's three parts held alone, −4.6 ± 0.95, −3.0 ± 0.78 and −4.7 ± 0.66; and exploration 029's three pairs, −2.9 ± 0.96, −4.0 ± 1.10 and −2.7 ± 0.53. At blocks 24 and 36 the side lies on B, the odd places, under H-47's plain night and under every one of these nights.
- **Its twin.** On this cell, the readouts' places exchanged and the first mapping learned again ran H-47's seed 23 first mapping from the assignment, bit for bit, the index exchanged; and on seed 23's first mapping H-47 puts the side on the odd places from block 18 from either arm, on B from the mirrored assignment and on A from the assignment ([exploration 023](../explorations/023-the-readouts-index-or-its-group.md)). On that arm H-47's clause 1 reads +13.4 ± 1.56.
- **What exploration 020 read there.** C's own units, the pair's stimulus at place 5 inside readout 0, lost margin onto A's answer in each of this cell's twelve plain path-nights and in each of its twelve without replay; at block 36, plain less without replay, C's own stood −0.26 M and D's own −0.29 M. Exploration 020 found that none of its three readings separates the two cells that failed from the two that did not, and listed among its questions the pair's own units' weights read night by night and stretch by stretch on seed 23.

## Decision Drivers

- **The maintainers' choices** (2026-11-09): stop the hold split by target, diagnose seed 23's first mapping from the mirrored assignment alone, and run it once this ADR merges.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A night that replays on some nights and not others is another form, so a hypothesis on it needs an ADR of its own.
- **H-47's harness sets each gap's night apart**: its night without replay is the plain night with nothing tagged, chosen gap by gap, so a form that replays on one night only needs no change under `src/`.

## Considered Options

1. **One night's replay at a time on this cell**, with each night's and each stretch's change and the replayed night's content, in one job.
2. **A read unit by unit on this cell**, which readout units fire on A's trials and on B's under each form.
3. **The same forms on this cell and on its twin**, seed 23's first mapping from the assignment, one job each.
4. **More paths on this cell** under H-47's two forms, for firmer figures.
5. **The night held onto the inhibitory units and the units outside the readouts without replay**, as exploration 029's note asks first.
6. **Write a hypothesis on that held night now**, built by a brief and measured by a round.
7. **Pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.

## Decision Outcome

**Option 1.**

### Exploration 030 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **Its cell** is seed 23's first mapping from the mirrored assignment, on H-47's paths 0, 5 and 6, read sixteen times at blocks 6, 12, 18, 24 and 36 in H-47's terms. A answers readout 1, the even places, and B readout 0, the odd places.
- **A night replayed** is H-47's plain night on A's patterns: A's 102 patterns, drawn from the start image, tagged. Every other night of a form is H-47's night without replay, the same night with nothing tagged. No hold is set in any form.
- **Its branch** starts from `main` with exploration 029's tag merged in, for H-47's and exploration 020's harness, the class matrix and `ReplayWatch`; it sets none of the holds on it and adds its harness only, nothing under `src/`. `main` takes nothing from it.

**Its job**, one, runs five forms:
- **the night without replay**, its rows held to H-47's;
- **replay after block 6 only**: the plain night after block 6, the night without replay after blocks 12, 18 and 24;
- **replay after block 12 only**;
- **replay after block 18 only**;
- **replay after block 24 only**.

Exploration 028's and 029's four-form jobs took 7 269 to 7 949 s, so the five-form job should take about 150 to 170 minutes, within an arms job's 240. In each form replaying after block 12, 18 or 24, every row before its replayed night is held to H-47's night without replay's, and in the form replaying after block 6 the row at block 6 is held to H-47's plain night's. H-47's plain night on A's patterns is read from H-47's run, and its class matrices, night lines and watch on this cell from exploration 020's runs 41500459562 and 41510955777; the twin cell's from exploration 021's run 41591077514. None is run again.

**Its trial** runs the pair's head and each form's first night on path 0, on this cell: the night without replay and the form replaying after block 6 hold H-47's rows at block 6 for the night without replay and the plain night, bit for bit, and each form replaying after block 12, 18 or 24 holds H-47's night without replay's row at block 6, bit for bit.

It reads at blocks 6, 12, 18, 24 and 36, in H-47's terms:
1. **A's count and each stimulus's trials** under each form: less the night without replay, with H-47's paired SE, the forms run in one job; and less H-47's plain night, with exploration 018's unpaired SE. At block 36 the four forms' differences from the night without replay are set beside the plain night's, as sizes, with no SE claimed for their sum.
2. **The side** under each form, by stimulus and by group of units, against H-47's side.
3. **The pair** under each form and A's highest coupling, as clauses 2 and 3 read them.
4. **What each night and each stretch moved**, from the class matrix by group: the change over each night and over each stretch of blocks between checkpoints, blocks 7 to 12, 13 to 18, 19 to 24 and 25 to 36, under each form; beside them the same night and stretches under H-47's plain night and the night without replay on this cell, and on the twin cell, from exploration 020's and 021's jobs; and C's own and D's own units onto A's answer, night by night and stretch by stretch, as exploration 020 read them over the nights alone.
5. **What the replayed night held**, from `ReplayWatch` as exploration 020 reads it: its ripples' spikes by class and its pairings by class, on each form's replayed night, beside H-47's plain night's same night on this cell and on the twin cell.
6. **The engine's two sums** over each night, the replays, depotentiations and the watch's wrong answers, and the ties, under each form.

Before it spends the job it states whether any form but the night without replay must run bit for bit as one already in the logs, and runs none that does (none can: each replays on one night and not on the other three, where the forms in the logs replay on all four nights or on none, as the trial's rows after block 6 will show).
It does not run the twin cell or any other cell, replay on two or three of the nights, plain nights on both, the awake gap, any hold, any read unit by unit, any path but H-47's three, any part of a night apart, slow-wave sleep from REM, or the next day's reading of the pairings a night leaves.

## Consequences

- **Good**: which night's replay, and which stretch after it, carries A's loss is read on the one cell where every night with replay since H-47 ends below the night without replay.
- **Bad**: one cell of twelve; what it reads may not hold on any other.
- **Bad**: H-47's plain night and the twin cell are read from other jobs, so a form less either carries an unpaired SE, and the four nights' differences are not tested as a sum.
- **Bad**: the forms are a change of the harness on a branch that `main` does not take.
- **Bad**: one job of about 150 to 170 minutes.
- **Bad**: M11 stays open, and no hypothesis is in view.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, a read unit by unit**: no instrument prints it, and it would say which units fire on A's trials at a checkpoint, not which night put them there.
- **Option 3, this cell and its twin**: it would double the job, and the twin's plain night and night without replay are already in exploration 021's logs, beside which this cell's single nights can be read.
- **Option 4, more paths**: firmer figures for a difference already beyond 2 SE at block 36, with nothing on where it comes from.
- **Option 5, the held night without replay**: it would read the hold apart from the replay on all twelve cells, not what moves A's count on this one.
- **Option 6, a hypothesis now**: under the held night that read as the night held within, this cell still ends below the night without replay beyond 2 SE at block 36, so a prediction on H-47's clause 1 would name a cell already read to fail it.
- **Option 7, pause M11**: the one cell that keeps every night read from H-47's clause 1 is not yet read night by night.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 030's note, when it merges, holds what it read.
