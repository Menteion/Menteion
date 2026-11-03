---
status: proposed
date: 2026-11-03
depends-on: ADR-0237
title: What keeps a mapping where the replay did not, and M11's next exploration
kind: decision
summary: H-47's step 5 answered from its own rows. On seed 23 A stood higher after nights without replay than after the awake gap, and the replay cost it. On seed 24 A fell under every form. No hypothesis is written, and an exploration reads where a replay's gain falls, by stimulus and readout.
decision-makers: Menteion maintainers
---

# ADR-0238: What keeps a mapping where the replay did not, and M11's next exploration

**In brief**
- **Decision.** H-47's step 5 is answered from its pinned rows. On seed 23 A stood higher after nights without replay than after the awake gap, and plain nights on A's patterns cost it. On seed 24 A fell under every form, to near half of 64. No hypothesis is written and no constant moves. An exploration reads, on H-47's twelve cells, where a replay's gain falls, by stimulus and by readout.
- **Why.** H-47's rows part its cells by readout. Where A answers readout 0, clause 1 read +3.8 to +19.6. Where A answers readout 1, it read −4.9 to +6.0, and both failures lie there. Exploration 020 read that none of the three readings it weighed separates the cells.
- **Left open.** Whether a hypothesis follows the exploration; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04); the schedule's sweep of 2026-11-04.

## Context and Problem Statement

[ADR-0237](0237-learning-that-sleep-keeps-measured.md) read H-47 as no at step 5. Clause 1 held in ten cells of twelve and failed on seed 23 from a first mapping and the mirrored assignment, −4.9 ± 1.09, and on seed 24 from the eighth image and the assignment, +1.8 ± 1.79. Step 5 names an ADR on what keeps A there, the replay or the rest of the night.

**H-47's rows by the readout A answers**, clause 1's difference at block 36, plain nights on A's patterns less nights without replay ([H-47's report](../research/H-47.md)). The mapping in force is the harness's `nights_mirrored`:

| A's answer | The mapping in force | Clause 1 in its six cells |
| :--- | :--- | :--- |
| Readout 0 | the assignment | +3.8, +7.9, +11.3, +13.4, +17.4, +19.6 |
| Readout 1 | the mirrored one | −4.9, +1.8, +2.2, +4.1, +5.0, +6.0 |

**H-47's rows on the two cells that failed**, A's mean count at blocks 6, 24 and 36:

| Cell | Awake gap | Without replay | Plain on A's | Plain on both |
| :--- | :--- | :--- | :--- | :--- |
| Seed 23, first mapping, mirrored | 53.7, 46.5, 43.6 | 55.0, 53.5, 53.1 | 56.9, 50.9, 48.3 | 57.3, 52.4, 50.1 |
| Seed 24, eighth image, assignment | 35.8, 32.0, 29.6 | 38.8, 35.6, 25.6 | 39.1, 29.6, 27.4 | 36.9, 30.5, 29.6 |

[Exploration 020](../explorations/020-what-a-replay-costs.md) ran H-47's two nights again on those two cells and on seed 22's two of the same starts, bit for bit H-47's runs. In all four the mapping in force is the mirrored one. Its readings are exploratory, not evidence:
- **The three readings it weighed.** A's patterns point at the answer in force, and every plain night moves A's margin A's way. The pair takes the shared units' margin about equally under both forms. None of the three separates the cells that failed.
- **Beyond A's and B's units**, four plain nights took 19.75 to 23.59 M of the readouts' excitatory weight and added 17.56 to 27.05 M of inhibitory weight. At block 36 the readouts fired 20 % to 35 % less on A's trials than without replay.
- **By stimulus**, plain on A's less without replay:

| Cell | A's trials, block 6 | B's trials, block 6 | A's trials, block 36 | B's trials, block 36 |
| :--- | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | +0.6 | +1.3 | −5.2 ± 1.50 | +0.3 ± 0.88 |
| Seed 24, eighth image, assignment | +2.5 | −2.3 | +0.2 ± 0.76 | +1.6 ± 1.50 |
| Seed 22, first mapping, mirrored | +0.7 | +0.4 | −0.4 ± 0.83 | +5.4 ± 1.47 |
| Seed 22, eighth image, assignment | +2.0 | −0.1 | −1.6 ± 0.88 | +3.8 ± 0.81 |

**What the engine holds today**: ADR-0235's slow-wave baseline, a configuration's field, unset and in no named configuration; and its two calls, the ledger spent and a trial's own units.

## Decision Drivers

- **The maintainers' choices** (2026-11-03): explore the asymmetry between A and B before any new hypothesis, and run it once this ADR merges.
- **H-47's stopping rule**: step 5 asks for this ADR. Step 8 allows no second attempt and no constant moved.
- **H-47's logs hold each read's trials** by stimulus, in its `stimuli` and `right` fields, and the mapping in force in its `mirrored` field. Part of the question needs no new run.

## Considered Options

1. **Explore the asymmetry, between A and B or between the readouts**, on H-47's twelve cells.
2. **Pause M11** and turn to M13, ADR-0187's next milestone.
3. **Explore chained episodes compiled in sleep** (UPG-04), M11's named candidate.

## Decision Outcome

**Option 1.**

### Step 5's answer, from H-47's rows

- **On seed 23** A went from 55.0 to 53.1 without replay and from 53.7 to 43.6 after the awake gap, from block 6 to block 36. These are means: no committed rule pairs the two forms, so no SE is given.
- **The replay of A's patterns cost A there.** Plain nights on A's patterns took it from 56.9 to 48.3.
- **On seed 24 A fell under every form.** Right after the last night, at block 24, the night without replay stood highest, 35.6 against the awake gap's 32.0. By block 36 every form stood from 25.6 to 29.6, near half of 64.
- **H-47 stays no.** No constant moves, and there is no second attempt. M11 stays open.
- **The slow-wave baseline stays unset.** Whether it enters the running configuration stays open.

### Exploration 021 (Specified; it decides nothing)

It writes no prediction and decides nothing. A **side** is the stimulus whose trials gain more under plain nights on A's patterns than under nights without replay. It reads, at every checkpoint:
1. **From H-47's logs first**, on all twelve cells: each stimulus's trials under each form, and the readout each stimulus answers. It reads the six cells of each orientation side by side.
2. **What a night without replay costs each stimulus** against the awake gap, by stimulus and by readout.
3. **Whether the side follows the stimulus or the readout.** The readouts' exchange is already in H-47's logs. An exchange of A's and B's roles needs a run of the exploration's own.
4. **Where the pair's own units sit against the side.** In exploration 020's four cells C's own lay inside readout 0 and D's inside readout 1. On the other eight that is read, not assumed.
5. **What plain nights take from each readout's own synapses**, by orientation.

A night whose replay leaves the readouts' synapses where they stood is named and not taken: it needs a change under `src/` on a branch, and the logs come first.

### The sweep

No sweep is left unrecorded: [ADR-0227](0227-the-pinned-tables-in-a-file-a-round.md) recorded the schedule's run of 2026-10-28. The run of 2026-11-04 at 20:17Z has not begun when this is written, and the first ADR that merges after it ends records it.

## Consequences

- **Good**: step 5 is answered from H-47's own rows, and its no stands as written.
- **Good**: the next exploration starts from logs already in hand.
- **Bad**: M11 stays open past its first hypothesis, and no hypothesis is in view.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, pause M11**: the replay helped in ten cells of twelve. Why it helps more where A answers readout 0 is unread.
- **Option 3, UPG-04**: its layout is undecided ([ADR-0187](0187-the-upgrade-suite-planned.md)), and the plain replay's own asymmetry is not yet understood.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 021's note, when it merges, holds what it read.
