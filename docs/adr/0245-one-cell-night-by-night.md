---
status: proposed
date: 2026-11-09
depends-on: ADR-0244
title: One cell, night by night
kind: decision
summary: Exploration 029's night holding the inhibitory and outside parts lay within 2 SE of the night held within at all 60 checkpoints and, like it, ends below the night without replay at block 36 on seed 23's first mapping from the mirrored assignment. Exploration 030 reads that cell night by night.
decision-makers: Menteion maintainers
---

# ADR-0245: One cell, night by night

**In brief**
- **Decision.** No hypothesis is written, no constant moves, and the hold split by target stops at exploration 029. Exploration 030 reads one cell of H-47's twelve, seed 23's first mapping from the mirrored assignment, with H-47's plain night's replay on one night alone and on every night but one: eight forms, each beside the night without replay, in two jobs. It reads what each night and each stretch of blocks after a night moved there, and what each replayed night held. Its branch adds its harness only, on an engine that is `main`'s, and `main` takes nothing from it. It runs once this ADR merges.
- **Why.** On this cell A's count under H-47's plain night lies below the night without replay at blocks 24 and 36, beyond 2 SE at block 36, and the loss lies on stimulus A's trials, beyond 2 SE at both; every held night run on it since in H-47's layout ends below the night without replay at block 36, beyond 2 SE in all but one. Under exploration 029's night held onto the inhibitory units and the units outside the readouts it is the one cell of twelve below at block 36. No run has replayed on some of its nights and not on others, so none says whether one night's replay is enough for the loss, or whether H-47's run needs any one night's.
- **Left open.** Whether a hypothesis follows exploration 030; the same reading on seed 23's first mapping from the assignment, on which this cell's places exchanged ran bit for bit, or on any other cell; a read unit by unit, which readout units fire on A's trials and on B's; replay on two of the four nights; the night held onto the inhibitory units and the units outside the readouts without replay, or split finer; the part held across beside any part, and exploration 024's hold beside any part; the inhibitory synapses onto the readouts' inhibitory units and the excitatory synapses onto the readouts from A's, B's and the pair's own units, which no hold reached; what the next day does with the pairings a hold leaves; what keeps the night held within below the excitatory night on seed 24's eighth image from the assignment, and what moves A's trials below the plain night's on seed 23's eighth image from the mirrored assignment, each read on that cell alone; exploration 024's hold on the other eight cells; plain nights on both and the awake gap night by night; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0244](0244-the-part-held-within-two-parts-at-a-time.md) ordered exploration 029: exploration 028's three parts of the part held within held two at a time through H-47's plain nights on A's patterns, on all twelve of H-47's cells. Its readings and those before it are exploratory, not evidence ([exploration 029](../explorations/029-the-part-held-within-two-parts-at-a-time.md)):

- **The night held onto the inhibitory units and the units outside the readouts**, the part held within with the own group's part left to consolidate, read A's count within twice exploration 018's unpaired SE of exploration 027's night held within at all 60 checkpoints, and of the excitatory night at 54. The other two pairs lay beyond 2 SE of the night held within at 7 and 13. At block 36 it lies within 2 SE of the excitatory night on the three cells where each part held alone lay below it, so ADR-0244's question on those cells stops with the hold split. At block 36, less the night without replay, the night held onto the inhibitory units and the units outside the readouts lies above beyond 2 SE in eleven of the twelve cells and below in one, seed 23's first mapping from the mirrored assignment, −2.7 ± 0.53, as exploration 027's night held within did there, −2.6 ± 0.65.
- **That cell under H-47** ([H-47](../research/H-47.md), [exploration 020](../explorations/020-what-a-replay-costs.md), section 4): A's count under the night without replay reads 55.0, 52.4, 52.0, 53.5 and 53.1 at blocks 6, 12, 18, 24 and 36, and under the plain night on A's patterns 56.9, 54.4, 52.4, 50.9 and 48.3. Plain less without replay reads +1.9 ± 0.68, +2.0 ± 1.26, +0.3 ± 1.17, −2.6 ± 1.30 and −4.9 ± 1.09, H-47's clause 1 at block 36. On A's trials it reads −5.0 ± 1.25 at block 24 and −5.2 ± 1.50 at block 36; on B's +2.4 ± 0.38 and +0.3 ± 0.88. Without replay A's trials rose from 28.5 at block 18 to 30.0 at block 24 ([exploration 021](../explorations/021-where-the-gain-falls.md), section 1).
- **Every held night run on that cell since in H-47's layout**, less the night without replay at block 36: exploration 024's night with the inhibition held, −4.8 ± 1.45; exploration 025's excitatory night, −1.7 ± 0.42, and its night under both holds, −1.7 ± 0.50; exploration 027's night held within, −2.6 ± 0.65, and its night held across, −2.7 ± 1.61, the one within 2 SE; exploration 028's parts held alone, onto the own group −4.6 ± 0.95, onto the inhibitory units −3.0 ± 0.78 and onto the units outside the readouts −4.7 ± 0.66; and exploration 029's pairs, onto the own group and the inhibitory units −2.9 ± 0.96, onto the own group and the units outside the readouts −4.0 ± 1.10, and onto the inhibitory units and the units outside the readouts −2.7 ± 0.53. At blocks 24 and 36 the side lies on B, the odd places, under H-47's plain night and under every one of these nights.
- **Its twin.** On this cell, the readouts' places exchanged and the first mapping learned again ran H-47's seed 23 first mapping from the assignment, bit for bit, the index exchanged; and on seed 23's first mapping H-47 puts the side on the odd places from block 18 from either arm, on B from the mirrored assignment and on A from the assignment ([exploration 023](../explorations/023-the-readouts-index-or-its-group.md)). On that arm H-47's clause 1 reads +13.4 ± 1.56.
- **What exploration 020 read there.** C's own units, the pair's stimulus at place 5 inside readout 0, lost margin onto A's answer in each of this cell's twelve plain path-nights and in each of its twelve without replay; at block 36, plain less without replay, C's own stood −0.26 M and D's own −0.29 M. Exploration 020 found that none of its three readings separates the two cells that failed from the two that did not, and listed among its questions the pair's own units' weights read night by night and stretch by stretch on seed 23.

## Decision Drivers

- **The maintainers' choices** (2026-11-09): stop the hold split by target, diagnose seed 23's first mapping from the mirrored assignment alone, in one or two jobs, and run it once this ADR merges.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A night that replays on some nights and not others is another form, so a hypothesis on it needs an ADR of its own.
- **H-47's harness sets each gap's night apart**: its night without replay is the plain night with nothing tagged, chosen gap by gap, so a form that replays on some nights only needs no change under `src/`.
- **One night alone and one night left out answer two questions.** A form that replays on one night after nights without replay reads whether that night's replay is enough for the loss; a form that replays on every night but one reads whether H-47's run needs that night's replay. Neither alone answers both.

## Considered Options

1. **One night's replay alone and every night but one, on this cell**, with each night's and each stretch's change and each replayed night's content, in two jobs.
2. **One night's replay alone only**, in one job.
3. **Every night but one only**, in one job.
4. **A read unit by unit on this cell**, which readout units fire on A's trials and on B's under each form.
5. **The same forms on this cell and on its twin**, seed 23's first mapping from the assignment.
6. **More paths on this cell** under H-47's two forms, for firmer figures.
7. **The night held onto the inhibitory units and the units outside the readouts without replay**, as exploration 029's note asks first.
8. **Write a hypothesis on that held night now**, built by a brief and measured by a round.
9. **Pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.

## Decision Outcome

**Option 1.**

### Exploration 030 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **Its cell** is seed 23's first mapping from the mirrored assignment, on H-47's paths 0, 5 and 6, read sixteen times at blocks 6, 12, 18, 24 and 36 in H-47's terms. A answers readout 1, the even places, and B readout 0, the odd places.
- **A night replayed** is H-47's plain night on A's patterns: A's 102 patterns, drawn from the start image, tagged. Every other night of a form is H-47's night without replay, the same night with nothing tagged. No hold is set.
- **Its branch** starts from `main` with exploration 023's tag merged in, for H-47's and exploration 020's harness, the class matrix and `ReplayWatch`; that tag changes nothing under `src/`, so the engine is `main`'s. The branch adds its harness only.

**Its two jobs** each run the night without replay, its rows held to H-47's, and four forms:
- **one night alone**: replay after block 6 only, after block 12 only, after block 18 only, and after block 24 only;
- **one night left out**: replay after every block but block 6, but block 12, but block 18, and but block 24.

Exploration 028's and 029's four-form jobs took 7 269 to 7 949 s, so each five-form job should take about 150 to 170 minutes, within an arms job's 240. Inside each job, every row a form reads after nights that so far match H-47's night without replay night for night, or its plain night, is held to H-47's row of that form on the same path and block; each night replayed reads replays and depotentiations that are not zero, and every other night reads `replays 0` and `depotentiations 0`. H-47's plain night on A's patterns is read from H-47's run 41441234613, and its class matrices, night lines and watch on this cell from exploration 020's run 41500459562; the twin cell's from exploration 021's run 41591077514. None is run again.

**Its trial** runs the pair's head and each form's first night on path 0, on this cell: each holds, bit for bit, an H-47 row at block 6 on path 0, the image's CRC-64, its reads, A's couplings and the gain: the night without replay, every form replaying after block 12, 18 or 24 alone, and the form leaving out the night after block 6 hold the night without replay's; the form replaying after block 6 alone and every form leaving out the night after block 12, 18 or 24 hold the plain night's.

It reads at blocks 6, 12, 18, 24 and 36, in H-47's terms. A checkpoint is read after the night that follows its block, so at the checkpoint right after a form's one replayed night, that form less the night without replay is that night's replay alone, and on A's count a stretch is read together with the night that ends it; only the class matrix reads them apart.
1. **A's count and each stimulus's trials** under each form: less the night without replay, with H-47's paired SE, the forms run in one job; and less H-47's plain night, with exploration 018's unpaired SE, where a form's rows are H-47's own they are read as H-47's. At block 36 the sum of the four single nights' differences from the night without replay, and the sum of the plain night's four differences from the forms leaving a night out, each signed, are set beside the plain night less the night without replay, −4.9 ± 1.09, with no SE claimed for either sum.
2. **The side** under each form, by stimulus and by group of units, against H-47's side.
3. **The pair** under each form and A's highest coupling, as clauses 2 and 3 read them.
4. **What each night and each stretch moved**, from the class matrix by group: the change over each night and over each stretch of blocks after it, blocks 7 to 12, 13 to 18, 19 to 24 and 25 to 36, under each form; beside them the same nights and stretches under H-47's plain night and the night without replay on this cell, and on the twin cell, from exploration 020's and 021's jobs; and C's own and D's own units onto A's answer, night by night and stretch by stretch, where exploration 020 read them over the nights, and over the pair's blocks only summed.
5. **What each replayed night held**, from `ReplayWatch` as exploration 020 reads it: its ripples' spikes by class and its pairings by class, beside H-47's plain night's same night on this cell and on the twin cell.
6. **The engine's two sums** over each night, the replays, depotentiations and the watch's wrong answers, and the ties, under each form.

Before it spends a job it states whether any form but the night without replay must run bit for bit as one already in the logs, and runs none that does (none can: each replays on one night or on three, where the forms in the logs on this cell replay on all four nights or on none; each runs H-47's night without replay's rows, or H-47's plain night's, at each checkpoint before the first night where it parts from that form, as the job holds, and is held to neither after it).
It does not run the twin cell or any other cell, replay on two of the four nights, plain nights on both, the awake gap, any hold, any read unit by unit, any path but H-47's three, any part of a night apart, such as slow-wave sleep from REM, or the next day's reading of the pairings a night leaves.

## Consequences

- **Good**: on the one cell where every night with replay read since H-47 in H-47's layout ends below the night without replay at block 36, it reads whether one night's replay alone, after nights without replay, is enough for the loss, and whether H-47's run needs any one night's replay, with what each night and each stretch after it moves.
- **Bad**: one cell of twelve; what it reads may not hold on any other.
- **Bad**: no form replays on two nights, so where no night alone is enough and two nights are each needed, it does not say whether those two nights' replay is enough without the others.
- **Bad**: H-47's plain night is read from another job, so a form less it carries an unpaired SE, and no sum of differences is tested; the twin cell is set beside, with no SE.
- **Bad**: the forms are a change of the harness on a branch that `main` does not take.
- **Bad**: two jobs of about 150 to 170 minutes each.
- **Bad**: M11 stays open, and no hypothesis is in view.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, one night alone only**: it reads whether a night's replay is enough for the loss, not whether H-47's run needs it, where each later night replays onto an image the earlier replays changed.
- **Option 3, every night but one only**: it reads whether H-47's run needs a night, not whether that night alone is enough.
- **Option 4, a read unit by unit**: no instrument prints it, and it would say which units fire on A's trials at a checkpoint, not which night put them there.
- **Option 5, this cell and its twin**: it would take two more jobs, and the twin's plain night and night without replay are already in exploration 021's logs, beside which this cell's forms can be read.
- **Option 6, more paths**: firmer figures for a difference already beyond 2 SE at block 36, with nothing on where it comes from.
- **Option 7, the held night without replay**: it would read the hold apart from the replay on all twelve cells, not what moves A's count on this one.
- **Option 8, a hypothesis now**: under the held night within 2 SE of the night held within, this cell still ends below the night without replay beyond 2 SE at block 36, so a prediction on H-47's clause 1 would name a cell already read to fail it.
- **Option 9, pause M11**: the one cell where every night with replay read fails H-47's clause 1 is not yet read night by night.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 030's note, when it merges, holds what it read.
