---
status: proposed
date: 2026-11-03
depends-on: ADR-0237
title: What keeps a mapping where the replay did not, and M11's next exploration
kind: decision
summary: H-47's step 5 answered from exploration 020. On seed 23 the night without replay kept more of A than the awake gap, and the replay cost A's own trials; on seed 24 neither night kept A as the awake gap did. No hypothesis is written, and an exploration reads where a replay's gain falls.
decision-makers: Menteion maintainers
---

# ADR-0238: What keeps a mapping where the replay did not, and M11's next exploration

**In brief**
- **Decision.** H-47's step 5 is answered. On seed 23 from a first mapping and the mirrored assignment, the rest of the night kept A and the replay cost A's own trials. On seed 24 from the eighth image and the assignment, neither night kept A as the awake gap did. No hypothesis is written and no constant moves. An exploration reads, on H-47's twelve cells, on which stimulus and which readout a replay's gain falls.
- **Why.** Exploration 020 read that the three readings it was asked to weigh do not separate the two cells that failed from the two that held. The cells parted by stimulus: on all four, the replay helped B's trials more than A's.
- **Left open.** Whether a new hypothesis follows the exploration; chained episodes compiled in sleep (UPG-04); the schedule's sweep of 2026-11-04.

## Context and Problem Statement

[ADR-0237](0237-learning-that-sleep-keeps-measured.md) read H-47 as no at step 5. In ten cells of twelve, plain nights on A's patterns kept more of A at block 36 than nights without replay, by more than twice the paired SE. On seed 23 from a first mapping and the mirrored assignment they read −4.9 ± 1.09, and on seed 24 from the eighth image and the assignment +1.8 ± 1.79. Step 5 names an ADR on what keeps A there, the replay or the rest of the night.

[Exploration 020](../explorations/020-what-a-replay-costs.md) ran H-47's two forms again on those two cells and on seed 22's two of the same starts, bit for bit H-47's runs. It is exploratory, not evidence:
- **The three readings.** In all four cells A's patterns point at the answer of the mapping in force, and every plain night moves A's margin A's way. The pair takes the shared units' margin about equally under both forms. None of the three separates the cells that failed.
- **By stimulus**, plain less without replay at block 36: on A's trials −5.2 ± 1.50, +0.2 ± 0.76, −0.4 ± 0.83 and −1.6 ± 0.88; on B's +0.3 ± 0.88, +1.6 ± 1.50, +5.4 ± 1.47 and +3.8 ± 0.81. The order is seed 23, seed 24, then seed 22's two cells.
- **Seed 23.** A read 53.1 without replay, 48.3 under plain nights and 43.6 after the awake gap. On A's trials it read 29.3, 24.1 and 23.8.
- **Seed 24.** A read 25.6 without replay, 27.4 under plain nights and 29.6 after the awake gap. On A's trials it read 12.9, 13.1 and 17.6.
- **In all four cells** the mapping in force is the mirrored one, so A's answer is readout 1 and B's readout 0.

**What the engine holds today**: ADR-0235's slow-wave baseline, the ledger spent and a trial's own units, each unset and in no named configuration.

## Decision Drivers

- **The maintainers' choices** (2026-11-03): explore the asymmetry between A and B before any new hypothesis, and run it once this ADR merges.
- **H-47's stopping rule**: step 5 asks for this ADR, and step 8 allows no second attempt and no constant moved.
- **Each read's trials by stimulus are in H-47's logs**, in its `stimuli` and `right` fields, so part of the question needs no new run.

## Considered Options

1. **Explore the asymmetry between A and B**, on H-47's twelve cells.
2. **Pause M11** and turn to M13, ADR-0187's next milestone.
3. **Explore chained episodes compiled in sleep** (UPG-04), M11's named candidate.

## Decision Outcome

**Option 1.**

### Step 5's answer

- **On seed 23** the rest of the night kept A, 53.1 against the awake gap's 43.6. The replay cost A's own trials, 24.1 against 29.3, and moved B's little, 24.2 against 23.9.
- **On seed 24** neither night kept A as the awake gap did. On A's trials the two nights stood 4.7 and 4.5 under it, and the replay moved them little.
- **H-47 stays no.** No constant moves, there is no second attempt, and M11 stays open.
- **ADR-0235's three parts stay unset**, and the slow-wave baseline enters no named configuration.

### Exploration 021 (Specified; it decides nothing)

It writes no prediction and decides nothing. It reads, on all twelve of H-47's cells:
- **on which stimulus a replay's gain falls**, and on which readout that stimulus answers, from H-47's logs first;
- **what in the night without replay costs a stimulus** against the awake gap, by stimulus and readout;
- **whether the side follows the stimulus or the readout**, with the roles of A and B, or of the readouts, exchanged on some cells;
- **where the pair's own units sit**, C's inside readout 0 and D's inside readout 1, against the side that gains.

### The sweep

The schedule's run of 2026-11-04 at 20:17Z has not begun when this is written. The first ADR that merges after it ends records it.

## Consequences

- **Good**: step 5 is answered from runs that are H-47's bit for bit, and its no stands as written.
- **Good**: the next exploration starts from logs already in hand.
- **Bad**: M11 stays open past its first hypothesis, and no hypothesis is in view.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, pause M11**: the replay helped in ten cells of twelve, and why it helped one stimulus more than the other is unread.
- **Option 3, UPG-04**: its layout is undecided ([ADR-0187](0187-the-upgrade-suite-planned.md)), and the plain replay's own asymmetry is not yet understood.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 021's note, when it merges, holds what it read.
