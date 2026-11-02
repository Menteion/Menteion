---
status: exploratory
date: 2026-11-03
---

# 020: What a replay costs A

**Branch**: `explore/020-what-a-replay-costs`
**Commit**: `1f2d1b3`, which adds tests and nothing under `src/`, on `2e52059`, `main` at brief 087's merge, with ADR-0235's three parts and
H-47's harness.
**Runs**: [41500459562](https://github.com/Menteion/Menteion/actions/runs/41500459562) (the harness's trial, and seed 23's first mapping from
the mirrored assignment) and [41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777) (the other three cells), both at
`scope=arms`. The rows of the awake gap and of plain nights on both mappings are read from H-47's run
[41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613), not run again.

[H-47](../research/H-47.md) read no at step 5 of its stopping rule ([ADR-0237](../adr/0237-learning-that-sleep-keeps-measured.md)). In ten cells
of twelve, plain nights on A's patterns kept more of A at block 36 than nights without replay, by more than twice the paired SE. In two they
did not. On seed 23, from a first mapping and the mirrored assignment, the night without replay kept A at 53.1 and plain nights at 48.3,
−4.9 ± 1.09, while A's margin stood 2.88 M higher under plain nights. On seed 24, from the eighth image and the assignment, the difference
was +1.8 ± 1.79 at block 36 and −5.9 ± 1.85 at block 24. This note runs H-47's two forms with and without replay again on those two cells,
and on seed 22's two cells of the same starts, where plain nights helped (+5.0 and +2.2). It asks:

1. whether A's patterns point at the answer the mapping in force asks for, and where a replay's spikes land;
2. what a replay moves each night, against a night without replay on the same image: A's couplings, the kinds of unit, the sums, the gain and
   the pairings, tick by tick;
3. why, on seed 23, the margin and the count part, checkpoint by checkpoint, and what the pair takes on the shared units.

The note was to weigh three readings of the two cells: a pattern that points at the wrong answer, a replay that moves the wrong synapses,
and the conflict on the shared units. It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each
test on the repository's AArch64 host, the mean over three paths unless a paired SE is given. Those are H-47's terms, and the runs are H-47's
runs bit for bit.

**In brief**
- **The runs are H-47's.** Each cell's 30 rows equal the rows H-47 pinned, held inside the test. Each job's log holds every one of H-47's
  lines of the same job, label aside, but H-47's two "not pinned" notices. All 96 nights watched read `wrong 0`.
- **The patterns point at the right answer, in all four cells.** Every target of A's 102 patterns lies on the answer's readout under the
  mapping in force; none lies on the other readout. A shared unit's pattern points at A's answer, which is the other readout from the one the
  pair asks of that unit. In a replay's ripple, beyond the pattern's own units, 11.3 to 11.7 spikes land on the answer's readout and 12.2 to
  13.8 on the other, the other more by 0.79 to 2.13 in every row, in the two cells that failed as in the two that did not.
- **On A's and B's units, every plain night moves the margin A's way, in all four cells.** Each of the four kinds of A's and B's units rose
  in each of the 48 path-nights, by 0.03 to 0.43 M. The pairings put weight onto the units' own patterns, +388 714 to +735 248 over four nights
  a class, and took it from the other readout in every class of every cell. Summed over the four nights, A's margin rose 3.33 to 3.56 M under
  plain nights and −0.22 to +0.48 M under nights without replay.
- **Beyond A's and B's units, the same nights take weight from the readouts and add inhibition, in all four cells.** Over four plain nights
  the readouts' excitatory units lost 19.75 to 23.59 M of weight, 13.26 to 15.64 M of it from the units A's patterns reach. The largest
  single parts, 2.99 to 3.74 M each way, lay between the two readouts' pattern targets. Inhibitory weight rose 17.56 to 27.05 M, 5.90 to
  9.69 M of it onto each readout's targets. The pair's blocks then lowered the inhibitory weight by 7.41 to 16.90 M and moved none of the
  excitatory. Matched by start, the cell that failed lost a little more of that excitatory weight and gained a little more of that
  inhibitory weight than seed 22's. At block 36 a read's readout spikes on
  A's trials stood 20 % to 35 % under the night without replay's.
- **The pair takes the shared units' margin about equally under both forms.** Over its blocks it took 0.98 to 2.05 M of A and C's margin
  under plain nights and 0.83 to 2.20 M under nights without replay. Plain nights gave 0.91 to 1.18 M back, nights without replay 0.05 to
  0.21 M.
- **So none of the three readings separates the two cells that failed from the two that did not.** Each is present in all four, and of
  similar size.
- **The cells part by stimulus.** At block 36, plain less without replay on stimulus A's trials read −5.2 ± 1.50 on seed 23, +0.2 ± 0.76 on
  seed 24, and −0.4 ± 0.83 and −1.6 ± 0.88 on seed 22's two cells; on B's trials +0.3 ± 0.88, +1.6 ± 1.50, +5.4 ± 1.47 and +3.8 ± 0.81. At
  block 6 every cell read more on A's trials under plain nights, +0.6 to +2.5.
- **On seed 23 the cost lies on A's trials.** Its 2.88 M of margin are spread over all four kinds, +0.68 to +0.74 M each. Its −4.9 lie on
  A's trials, where A read 24.1 against 29.3 and wrong answers 5.8 against 2.0. On B's trials it read 24.2 against 23.9.
- **One reading of the nights sets seed 23 apart, and it lies outside A's margin.** C's own units, the pair's stimulus at place 5 inside
  readout 0, lost margin onto A's answer in each of seed 23's twelve plain path-nights and in each of its twelve without replay. In the
  other three cells they lost it in 0 to 5 of the twelve plain ones and 6 to 9 of the twelve without replay. At block 36, plain less without
  replay, C's own stood −0.26 M on seed 23 and +0.10 to +0.21 M on the other three cells; D's own −0.29 M, and −0.06 to +0.13 M.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), on
`main`: `Config::runs_a_settled_image()`, the slow-wave baseline at 1/2, H-25's learning configuration, and the zero sum's six groups, each
kind of unit held apart at its sum in the start image. The pair shares 26 of A's 51 source units, conflicting. Gaps follow blocks 6, 12, 18
and 24, then 12 blocks with none to block 36, on paths 0, 5 and 6. Each checkpoint reads A and the pair sixteen times each on forks.

**Four cells.** H-47's two that failed clause 1, seed 23 from a first mapping and the mirrored assignment and seed 24 from the eighth image
and the assignment, and seed 22's two of the same starts. In all four the mapping in force is the mirrored one, so A's answer is readout 1
and B's readout 0. C, which shares A's units, answers readout 0, and D, which shares B's, answers readout 1.

**Two forms a cell, in one job**: the night without replay and the plain night on A's patterns, as H-47's `nights_job` runs them, every row
held to the row H-47 pinned (`pins/h47.rs`). The derivation is run again and held to H-42's pinned tables, and a first mapping is held to its
CRC-64. The awake gap's rows and those of plain nights on both mappings are H-47's logs, read as they are.

**What the job adds, all of it reading and none of it acting:**
- **A's patterns**: each one's first unit, its kind, and where each target lies.
- **Before each gap**, the image's kinds and a split; **after each gap and at block 36**, the split. The split takes each kind's weight onto
  six parts of A's readouts: readout 0 less C's own units and the targets, the targets, C's own; then the same of readout 1 with D's own.
  The **targets** are the units A's patterns reach past their first, 539 to 560 a cell. **C's own** units are C's 25 at place 5, inside
  readout 0, and **D's own** D's 25 at place 16, inside readout 1.
- **In each of those images**, a matrix of fourteen classes of unit, each class's weight onto each class: the six kinds, each readout's
  targets and its other excitatory units, the other excitatory units, and the inhibitory units of each readout and elsewhere. It is held to the
  engine's two sums.
- **At every night**, `ReplayWatch`, exploration 015's `PairWatch` on A's and B's 102 stimulus units, the first unit of each of A's patterns.
  It reads every pairing of those units tick by tick, whether the night tags the patterns or not, so a night without replay is read on the
  same synapses. In a ripple that replays one of A's patterns, every spike is classed against it.

**A kind's margin** is its weight onto its stimulus's answer readout less onto the other. **A's margin**, H-47's, is the sum of four kinds: A
alone, A and C, B alone, and B and D. C's own and D's own lie outside it.

**Nothing under `src/` changed.** The job differs from H-47's only by its dumps.

## The runs are H-47's

| Cell | Rows | Equal to H-47's |
| :--- | ---: | ---: |
| Seed 23, first mapping, mirrored | 30 | 30 |
| Seed 24, eighth, assignment | 30 | 30 |
| Seed 22, first mapping, mirrored | 30 | 30 |
| Seed 22, eighth, assignment | 30 | 30 |

- **Every row held inside the test**: the image's CRC-64, A's and the pair's sixteen counts, A's couplings, their highest and the gain.
- **Every line of H-47's job**, label aside, is in the cell's log: 1 908 of 1 910, 1 907 of 1 909, 1 908 of 1 910 and 1 907 of 1 909. The
  missing two in each are H-47's notices that its rows were not yet pinned, which this job replaces by "every row is H-47's".
- **The trial** ran the first night of each form on path 0 of seed 23's cell, both watched, and the night without replay again unwatched. Each
  image, its read as it was of A, its couplings and its gain held H-47's row.
- **The watch** read 4 994 688 to 5 317 600 pairings a cell over its 24 nights, `wrong 0` in all 96.
- **The nights.** Every night without replay read `replays 0` and `depotentiations 0`, and each plain night 1 129 to 1 340 replays and 545 to
  640 depotentiations. Each night without replay woke on its plain night's tick, 48 of 48.

## 1. A's patterns

The targets of each kind are counted once a pattern, so a unit two patterns reach counts twice; the last row of each cell counts each unit
once.

| Cell | Kind of the first unit | Patterns | Targets | On the answer's readout | On the other | Outside | The pair's own | A's answer | The pair's answer |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | A alone | 25 | 188 | 188 | 0 | 0 | 1 | R1 | – |
| Seed 23, first mapping, mirrored | A and C | 26 | 181 | 181 | 0 | 0 | 27 | R1 | R0 |
| Seed 23, first mapping, mirrored | B alone | 25 | 190 | 190 | 0 | 0 | 19 | R0 | – |
| Seed 23, first mapping, mirrored | B and D | 26 | 186 | 186 | 0 | 0 | 4 | R0 | R1 |
| Seed 23, first mapping, mirrored | distinct targets |  | 539 |  |  |  | 37 |  |  |
| Seed 24, eighth, assignment | A alone | 25 | 184 | 184 | 0 | 0 | 10 | R1 | – |
| Seed 24, eighth, assignment | A and C | 26 | 191 | 191 | 0 | 0 | 29 | R1 | R0 |
| Seed 24, eighth, assignment | B alone | 25 | 175 | 175 | 0 | 0 | 27 | R0 | – |
| Seed 24, eighth, assignment | B and D | 26 | 193 | 193 | 0 | 0 | 5 | R0 | R1 |
| Seed 24, eighth, assignment | distinct targets |  | 545 |  |  |  | 48 |  |  |
| Seed 22, first mapping, mirrored | A alone | 25 | 178 | 178 | 0 | 0 | 7 | R1 | – |
| Seed 22, first mapping, mirrored | A and C | 26 | 184 | 184 | 0 | 0 | 21 | R1 | R0 |
| Seed 22, first mapping, mirrored | B alone | 25 | 188 | 188 | 0 | 0 | 29 | R0 | – |
| Seed 22, first mapping, mirrored | B and D | 26 | 202 | 202 | 0 | 0 | 9 | R0 | R1 |
| Seed 22, first mapping, mirrored | distinct targets |  | 557 |  |  |  | 47 |  |  |
| Seed 22, eighth, assignment | A alone | 25 | 178 | 178 | 0 | 0 | 7 | R1 | – |
| Seed 22, eighth, assignment | A and C | 26 | 184 | 184 | 0 | 0 | 21 | R1 | R0 |
| Seed 22, eighth, assignment | B alone | 25 | 188 | 188 | 0 | 0 | 29 | R0 | – |
| Seed 22, eighth, assignment | B and D | 26 | 202 | 202 | 0 | 0 | 9 | R0 | R1 |
| Seed 22, eighth, assignment | distinct targets |  | 560 |  |  |  | 47 |  |  |

- **Every target lies on the answer's readout under the mapping in force**, 745 of 745 on seed 23, 743 of 743 on seed 24,
  and 752 of 752 in each of seed 22's two cells. So does every pattern tagged in every night: the same 102, drawn from the start image.
- **A shared unit's pattern points at A's answer**, readout 1 for A and C, readout 0 for B and D. The pair asks the opposite of those units.
- **Some targets are the pair's own units.** A and C's patterns reach D's own units in readout 1 as 21 to 29 targets, and B alone's reach
  C's own in readout 0 as 19 to 29. Over all 102 patterns they are 37 to 48 distinct units of the pair's 50.
- **The ledger replays each episode about as often in every cell**: 13.4 to 13.7 times at the first night for A's kinds and 12.6 to 12.7 for
  B's, falling to 11.0 to 11.6 and 10.6 at the fourth. For each kind at each night, the three paths pooled, the first unit fired twice or more
  in 99.2 % to 100 % of the replays.

Spikes in a ripple that replays one of A's patterns, by the pattern's stimulus. The ripples are the total over the four plain nights and the
three paths, and each other column a ripple's mean over them:

| Cell | Stimulus | Ripples | The first unit | The rest of the pattern | Pair's own in the answer's readout | Pair's own in the other readout | The answer's readout | The other readout | Another stimulus unit | Elsewhere |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | A | 7 560 | 3.1 | 21.9 | 0.4 | 0.6 | 11.3 | 13.4 | 2.1 | 0.1 |
| Seed 23, first mapping, mirrored | B | 7 147 | 3.1 | 22.2 | 0.5 | 0.6 | 11.4 | 12.2 | 2.1 | 0.1 |
| Seed 24, eighth, assignment | A | 7 506 | 3.1 | 22.4 | 0.4 | 0.7 | 11.7 | 13.8 | 2.0 | 0.1 |
| Seed 24, eighth, assignment | B | 7 170 | 3.1 | 21.9 | 0.4 | 0.5 | 11.3 | 13.0 | 2.0 | 0.1 |
| Seed 22, first mapping, mirrored | A | 7 560 | 3.1 | 21.5 | 0.5 | 0.7 | 11.4 | 12.3 | 2.1 | 0.1 |
| Seed 22, first mapping, mirrored | B | 7 147 | 3.0 | 23.1 | 0.5 | 0.7 | 11.6 | 12.5 | 2.2 | 0.2 |
| Seed 22, eighth, assignment | A | 7 508 | 3.1 | 21.5 | 0.5 | 0.7 | 11.7 | 13.1 | 1.9 | 0.2 |
| Seed 22, eighth, assignment | B | 7 169 | 3.0 | 23.2 | 0.4 | 0.7 | 11.7 | 13.1 | 2.0 | 0.2 |

- **Beyond the pattern, a replay drives both readouts, the other a little more.** On the answer's readout 11.3 to 11.7 spikes a ripple, on
  the other 12.2 to 13.8, the other more by 0.79 to 2.13 in each of the eight rows.
- **The pattern's first unit fires about three times a ripple**, 3.04 to 3.07, and the rest of the pattern 21.5 to 23.2 times.
- **Its pairings onto the other readout lose weight.** In a unit's own replay, a pairing with a post in the same tick is 32.4 % to 34.6 % of
  those onto its own pattern, and 0.0 % to 0.1 % of those onto the other readout. Over four nights those onto the other readout moved −133 557
  to −507 432 into the weights a class and cell, and those onto the pattern +155 063 to +484 926.

**On a pattern that points at the wrong answer**, this note finds nothing in any of the four cells. Each pattern's targets lie on A's answer.
The replay's spikes off the pattern land on both readouts, a little more on the other, in all four. The pairings onto the other readout lose
weight in all four.

## 2. What a replay moves, night by night

Each night's change from its start to its end, the mean over the three paths. A's margin is H-47's, and each kind's margin is onto A's
answer. The kinds are read from the image before the gap and the image after it, the sums and the gain from the night's start and end:

| Cell | Gap | Form | A's margin, M | A alone | A and C | B alone | B and D | Excitatory sum, M | Inhibitory sum, M | Gain |
| :--- | ---: | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | no-replay | +0.15 | +0.04 | +0.06 | +0.02 | +0.03 | −0.48 | −1.19 | +1 991 |
| Seed 23, first mapping, mirrored | 6 | plain-first | +1.21 | +0.26 | +0.33 | +0.25 | +0.36 | −7.72 | +8.85 | −4 327 |
| Seed 23, first mapping, mirrored | 12 | no-replay | +0.05 | −0.03 | +0.03 | +0.06 | −0.01 | +0.37 | −1.40 | +1 972 |
| Seed 23, first mapping, mirrored | 12 | plain-first | +1.01 | +0.19 | +0.29 | +0.24 | +0.29 | −5.84 | +5.81 | −3 789 |
| Seed 23, first mapping, mirrored | 18 | no-replay | +0.07 | +0.01 | +0.05 | +0.01 | +0.01 | +0.64 | −1.43 | +1 978 |
| Seed 23, first mapping, mirrored | 18 | plain-first | +0.75 | +0.15 | +0.24 | +0.19 | +0.17 | −3.93 | +3.79 | −3 392 |
| Seed 23, first mapping, mirrored | 24 | no-replay | +0.10 | −0.02 | +0.08 | +0.05 | −0.01 | +1.02 | −1.33 | +1 991 |
| Seed 23, first mapping, mirrored | 24 | plain-first | +0.54 | +0.08 | +0.12 | +0.19 | +0.15 | −2.89 | +2.62 | −2 803 |
| Seed 24, eighth, assignment | 6 | no-replay | −0.05 | −0.07 | 0.00 | −0.07 | +0.10 | −0.47 | +0.32 | +2 134 |
| Seed 24, eighth, assignment | 6 | plain-first | +1.26 | +0.24 | +0.38 | +0.23 | +0.42 | −9.36 | +8.80 | −4 075 |
| Seed 24, eighth, assignment | 12 | no-replay | −0.01 | −0.06 | +0.05 | −0.10 | +0.11 | +0.10 | +0.25 | +2 175 |
| Seed 24, eighth, assignment | 12 | plain-first | +0.95 | +0.23 | +0.23 | +0.13 | +0.36 | −6.68 | +7.16 | −3 721 |
| Seed 24, eighth, assignment | 18 | no-replay | −0.08 | −0.12 | +0.02 | −0.06 | +0.08 | +0.43 | +0.32 | +2 448 |
| Seed 24, eighth, assignment | 18 | plain-first | +0.56 | +0.08 | +0.15 | +0.13 | +0.20 | −4.45 | +5.92 | −2 997 |
| Seed 24, eighth, assignment | 24 | no-replay | −0.08 | −0.06 | +0.03 | −0.10 | +0.06 | +0.34 | +0.44 | +2 413 |
| Seed 24, eighth, assignment | 24 | plain-first | +0.58 | +0.12 | +0.24 | +0.03 | +0.19 | −3.38 | +5.16 | −2 768 |
| Seed 22, first mapping, mirrored | 6 | no-replay | +0.23 | +0.13 | +0.04 | +0.03 | +0.03 | −0.44 | −1.32 | +1 995 |
| Seed 22, first mapping, mirrored | 6 | plain-first | +1.20 | +0.32 | +0.34 | +0.21 | +0.33 | −7.55 | +7.36 | −4 331 |
| Seed 22, first mapping, mirrored | 12 | no-replay | +0.10 | +0.06 | +0.01 | −0.01 | +0.04 | +0.09 | −1.39 | +1 792 |
| Seed 22, first mapping, mirrored | 12 | plain-first | +0.88 | +0.20 | +0.27 | +0.20 | +0.21 | −5.75 | +4.75 | −3 872 |
| Seed 22, first mapping, mirrored | 18 | no-replay | +0.15 | +0.06 | +0.03 | +0.02 | +0.04 | +0.40 | −1.39 | +1 884 |
| Seed 22, first mapping, mirrored | 18 | plain-first | +0.65 | +0.14 | +0.16 | +0.13 | +0.22 | −3.85 | +3.21 | −3 515 |
| Seed 22, first mapping, mirrored | 24 | no-replay | 0.00 | +0.01 | −0.04 | +0.01 | +0.03 | +0.37 | −1.40 | +2 141 |
| Seed 22, first mapping, mirrored | 24 | plain-first | +0.60 | +0.15 | +0.13 | +0.16 | +0.15 | −2.96 | +2.24 | −2 864 |
| Seed 22, eighth, assignment | 6 | no-replay | +0.08 | −0.01 | +0.03 | −0.02 | +0.08 | −0.61 | +0.28 | +2 019 |
| Seed 22, eighth, assignment | 6 | plain-first | +1.23 | +0.21 | +0.38 | +0.25 | +0.40 | −9.17 | +8.35 | −4 301 |
| Seed 22, eighth, assignment | 12 | no-replay | +0.01 | −0.06 | +0.04 | −0.05 | +0.09 | −0.11 | +0.23 | +2 034 |
| Seed 22, eighth, assignment | 12 | plain-first | +1.00 | +0.21 | +0.29 | +0.18 | +0.32 | −6.30 | +6.67 | −3 912 |
| Seed 22, eighth, assignment | 18 | no-replay | +0.04 | −0.02 | +0.06 | −0.06 | +0.06 | −0.06 | +0.24 | +2 419 |
| Seed 22, eighth, assignment | 18 | plain-first | +0.74 | +0.16 | +0.27 | +0.14 | +0.17 | −4.29 | +5.74 | −3 311 |
| Seed 22, eighth, assignment | 24 | no-replay | 0.00 | −0.02 | +0.05 | −0.06 | +0.04 | +0.23 | +0.28 | +2 133 |
| Seed 22, eighth, assignment | 24 | plain-first | +0.59 | +0.12 | +0.24 | +0.07 | +0.17 | −3.19 | +4.92 | −2 813 |

- **Every plain night raises each of A's and B's four kinds**, in each of the 48 path-nights a kind, by 0.073 to 0.330 M for A alone, 0.096
  to 0.399 for A and C, 0.029 to 0.276 for B alone and 0.133 to 0.428 for B and D. The first night raised A's margin by 1.20 to 1.26 M, and the
  fourth by 0.54 to 0.60 M.
- **A night without replay moves them by −0.127 to +0.130 M**, and A's margin by −0.110 to +0.238 M on one path.
- **Every plain night lowers the excitatory sum**, by 2.83 to 9.41 M on one path, raises the inhibitory sum by 2.23 to 8.89 M, and lowers
  the gain by 2 745 to 4 436. Every night without replay raises the gain, by 1 716 to 2 580.

Summed over the four nights:

| Cell | Form | A to readout 0, M | A to readout 1 | B to readout 0 | B to readout 1 | A's margin | A alone | A and C | B alone | B and D | C's own | D's own | Excitatory sum, M | Inhibitory sum, M | Gain |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | no-replay | −0.11 | +0.11 | +0.08 | −0.08 | +0.37 | 0.00 | +0.21 | +0.13 | +0.03 | −0.16 | +0.02 | +1.6 | −5.3 | +7 932 |
| Seed 23, first mapping, mirrored | plain-first | −0.83 | +0.83 | +0.92 | −0.92 | +3.50 | +0.68 | +0.98 | +0.87 | +0.97 | −0.34 | −0.06 | −20.4 | +21.1 | −14 311 |
| Seed 24, eighth, assignment | no-replay | +0.11 | −0.11 | 0.00 | 0.00 | −0.22 | −0.32 | +0.09 | −0.33 | +0.34 | 0.00 | −0.25 | +0.4 | +1.3 | +9 170 |
| Seed 24, eighth, assignment | plain-first | −0.83 | +0.83 | +0.85 | −0.85 | +3.35 | +0.67 | +0.99 | +0.52 | +1.17 | +0.09 | −0.16 | −23.9 | +27.1 | −13 562 |
| Seed 22, first mapping, mirrored | no-replay | −0.15 | +0.15 | +0.09 | −0.09 | +0.48 | +0.25 | +0.05 | +0.05 | +0.14 | −0.11 | −0.13 | +0.4 | −5.5 | +7 813 |
| Seed 22, first mapping, mirrored | plain-first | −0.85 | +0.85 | +0.81 | −0.81 | +3.33 | +0.80 | +0.91 | +0.70 | +0.91 | +0.21 | −0.19 | −20.1 | +17.6 | −14 581 |
| Seed 22, eighth, assignment | no-replay | −0.03 | +0.03 | +0.03 | −0.03 | +0.13 | −0.12 | +0.18 | −0.20 | +0.27 | 0.00 | −0.13 | −0.5 | +1.0 | +8 604 |
| Seed 22, eighth, assignment | plain-first | −0.93 | +0.93 | +0.84 | −0.84 | +3.56 | +0.69 | +1.18 | +0.63 | +1.06 | +0.28 | −0.22 | −23.0 | +25.7 | −14 337 |

- **A's four couplings move as one**: what A loses onto readout 0 it gains onto readout 1, and B the other way, since the zero sum holds each
  kind's weight onto both readouts. Plain nights moved each by 0.81 to 0.93 M, nights without replay by at most 0.15 M.
- **C's own and D's own move little, and not with the others.** Under plain nights C's own moved −0.34 M on seed 23 and +0.09 to +0.28 M on
  the other three cells; D's own −0.06 to −0.22 M on all four.

What the pairings of A's and B's units put into the weights, summed over the four nights, the mean over the three paths, by the unit's class
and the target's. The pair's own takes its units in both readouts, and elsewhere the other stimulus units and the units outside the
readouts:

| Cell | Class | Without replay: its own pattern | the answer's readout | the other readout | the pair's own | elsewhere | Plain: its own pattern | the answer's readout | the other readout | the pair's own | elsewhere |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | A alone | −98 115 | +37 517 | −42 389 | −25 037 | −6 583 | +501 684 | −91 013 | −256 228 | −32 118 | −11 630 |
| Seed 23, first mapping, mirrored | A and C | +34 574 | +113 592 | −40 896 | +10 918 | −9 620 | +529 601 | −143 412 | −516 376 | +66 570 | −19 026 |
| Seed 23, first mapping, mirrored | B alone | −20 106 | +59 793 | −140 523 | +4 554 | −9 363 | +517 237 | −56 523 | −369 487 | +54 803 | −17 589 |
| Seed 23, first mapping, mirrored | B and D | +89 210 | +88 796 | +199 904 | −7 590 | −17 404 | +566 175 | +22 522 | −340 285 | −16 584 | −25 144 |
| Seed 24, eighth, assignment | A alone | +3 865 | +121 157 | +379 537 | −18 805 | −15 187 | +689 810 | −139 096 | −167 636 | −22 594 | −22 822 |
| Seed 24, eighth, assignment | A and C | +289 785 | +82 819 | +374 106 | +4 400 | +6 551 | +723 749 | −50 381 | −177 312 | +99 711 | −31 103 |
| Seed 24, eighth, assignment | B alone | −37 387 | +13 959 | +242 913 | +50 089 | −13 618 | +505 138 | −7 529 | −23 814 | +87 034 | −13 334 |
| Seed 24, eighth, assignment | B and D | +263 318 | +386 474 | +347 893 | −3 608 | −10 855 | +734 044 | +65 441 | −345 156 | +18 186 | −33 886 |
| Seed 22, first mapping, mirrored | A alone | +8 865 | +121 362 | −156 069 | +21 011 | −9 314 | +565 462 | −279 729 | −429 017 | −32 195 | −16 594 |
| Seed 22, first mapping, mirrored | A and C | −68 542 | +202 189 | +98 063 | +10 267 | −12 430 | +499 693 | −118 313 | −448 052 | +73 522 | −34 391 |
| Seed 22, first mapping, mirrored | B alone | −102 733 | +15 973 | −136 016 | −38 080 | −6 404 | +388 714 | −38 394 | −288 490 | +66 859 | −11 650 |
| Seed 22, first mapping, mirrored | B and D | +4 093 | +78 999 | −58 298 | −6 882 | −5 574 | +512 811 | −10 850 | −397 392 | +17 653 | −13 424 |
| Seed 22, eighth, assignment | A alone | +30 732 | +78 935 | +139 530 | +27 904 | −4 722 | +721 903 | −256 744 | −258 744 | −1 190 | −19 798 |
| Seed 22, eighth, assignment | A and C | −10 012 | +358 541 | +258 547 | +54 921 | −13 959 | +735 248 | +102 647 | −255 850 | +84 398 | −26 928 |
| Seed 22, eighth, assignment | B alone | −13 308 | +81 927 | +240 848 | +8 445 | −6 176 | +476 384 | −21 774 | −152 480 | +65 845 | −9 795 |
| Seed 22, eighth, assignment | B and D | +168 851 | +162 511 | +82 207 | −4 556 | −2 767 | +713 238 | +42 591 | −279 660 | +18 599 | −9 751 |

- **Under plain nights every class puts weight onto its own pattern and takes it from the other readout**, in every cell. The answer's
  readout off the pattern loses weight in 12 of 16 class-and-cell rows.
- **Under nights without replay the pairings put weight onto the other readout in 10 of the 16 rows.**
- The pairings are not all a night moves: the zero sum's rounds hold each kind's weight onto the readouts, and REM's depotentiations were not
  counted.

The readouts' weights, by the class matrix, summed over the four nights and over the pair's blocks (the head, the three stretches between
nights, and the 12 blocks to block 36), the mean over the three paths, in millions:

| Cell | Form | Over | Readout 0's targets onto readout 1's | Readout 1's targets onto readout 0's | The readouts' excitatory units onto all | Inhibitory onto readout 0's targets | onto readout 1's targets | Inhibitory onto all | A's and B's units onto all |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | no-replay | four nights | −0.11 | −0.12 | +1.69 | −1.22 | −1.25 | −5.35 | −0.04 |
| Seed 23, first mapping, mirrored | no-replay | the pair's blocks | 0.00 | 0.00 | 0.00 | −5.05 | −4.86 | −12.54 | 0.00 |
| Seed 23, first mapping, mirrored | plain-first | four nights | −3.58 | −3.03 | −20.10 | +7.74 | +7.90 | +21.08 | −0.07 |
| Seed 23, first mapping, mirrored | plain-first | the pair's blocks | 0.00 | 0.00 | −0.01 | −6.07 | −5.80 | −16.90 | 0.00 |
| Seed 24, eighth, assignment | no-replay | four nights | +0.27 | +0.01 | +0.63 | +0.83 | +0.68 | +1.33 | −0.03 |
| Seed 24, eighth, assignment | no-replay | the pair's blocks | 0.00 | 0.00 | −0.01 | +0.06 | −0.24 | +2.27 | 0.00 |
| Seed 24, eighth, assignment | plain-first | four nights | −3.46 | −3.70 | −23.59 | +9.40 | +9.69 | +27.05 | −0.10 |
| Seed 24, eighth, assignment | plain-first | the pair's blocks | 0.00 | 0.00 | +0.01 | −2.73 | −3.31 | −7.75 | 0.00 |
| Seed 22, first mapping, mirrored | no-replay | four nights | −0.40 | −0.52 | +0.35 | −1.31 | −1.54 | −5.50 | −0.03 |
| Seed 22, first mapping, mirrored | no-replay | the pair's blocks | 0.00 | 0.00 | 0.00 | −4.95 | −5.43 | −12.95 | 0.00 |
| Seed 22, first mapping, mirrored | plain-first | four nights | −2.99 | −3.52 | −19.75 | +5.90 | +6.42 | +17.56 | −0.08 |
| Seed 22, first mapping, mirrored | plain-first | the pair's blocks | 0.00 | 0.00 | 0.00 | −5.62 | −5.97 | −16.50 | 0.00 |
| Seed 22, eighth, assignment | no-replay | four nights | −0.35 | −0.51 | −0.62 | +0.71 | +0.56 | +1.03 | −0.03 |
| Seed 22, eighth, assignment | no-replay | the pair's blocks | 0.00 | 0.00 | −0.01 | −0.36 | −0.35 | +1.27 | 0.00 |
| Seed 22, eighth, assignment | plain-first | four nights | −3.56 | −3.74 | −22.69 | +8.80 | +9.21 | +25.68 | −0.07 |
| Seed 22, eighth, assignment | plain-first | the pair's blocks | 0.00 | 0.00 | +0.01 | −2.72 | −3.07 | −7.41 | 0.00 |

- **Plain nights take the readouts' excitatory weight**, 19.75 to 23.59 M over four nights, 13.26 to 15.64 M of it from the units A's
  patterns reach. The largest single parts lie between the two readouts' pattern targets, −2.99 to −3.74 M each way. Nights without replay
  moved it by −0.62 to +1.69 M.
- **Plain nights add inhibitory weight**, 17.56 to 27.05 M, of it 5.90 to 9.69 M onto each readout's targets. Nights without replay moved it
  by −5.50 to +1.33 M.
- **The pair's blocks move no excitatory weight of the readouts' units**, at most 0.01 M either way. They lowered the inhibitory weight by
  7.41 to 16.90 M under plain nights. Under nights without replay they lowered it by 12.54 and 12.95 M from a first mapping and raised it by
  2.27 and 1.27 M from the eighth image. So the blocks' fall is not only the nights' rise given back.
- **A's and B's units' weight onto everything stays where the zero sum holds it**, −0.03 to −0.10 M over the four nights.
- **Matched by start, the cell that failed lost a little more and gained a little more than seed 22's**: the readouts' excitatory loss 20.10 against
  19.75 M and 23.59 against 22.69 M, the inhibitory rise 21.08 against 17.56 M and 27.05 against 25.68 M. Two pairs of cells cannot say
  whether that order means anything.

**On a replay that moves the wrong synapses**, the answer has two parts. On A's and B's synapses the replay moves the margin A's way in
every night of every cell. Beyond them it takes excitatory weight from the readouts' units, most of it from those its patterns reach, and
adds inhibition onto them, in every cell. Neither part separates the two cells, beyond the small order matched by start above.

## 3. The shared units and the pair

What each stretch moved of four kinds' margins onto A's answer, summed over the pair's blocks and over the four nights, the mean over the
three paths, in millions:

| Cell | Form | A and C: the pair's blocks | the nights | B and D: the pair's blocks | the nights | C's own: the pair's blocks | the nights | D's own: the pair's blocks | the nights |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | no-replay | −1.12 | +0.21 | −0.87 | +0.03 | −1.23 | −0.16 | −0.88 | +0.02 |
| Seed 23, first mapping, mirrored | plain-first | −1.16 | +0.98 | −1.08 | +0.97 | −1.31 | −0.34 | −1.09 | −0.06 |
| Seed 24, eighth, assignment | no-replay | −2.20 | +0.09 | −2.22 | +0.34 | −0.95 | 0.00 | −0.82 | −0.25 |
| Seed 24, eighth, assignment | plain-first | −2.05 | +0.99 | −2.07 | +1.17 | −0.84 | +0.09 | −0.78 | −0.16 |
| Seed 22, first mapping, mirrored | no-replay | −0.83 | +0.05 | −1.01 | +0.14 | −0.95 | −0.11 | −0.78 | −0.13 |
| Seed 22, first mapping, mirrored | plain-first | −0.98 | +0.91 | −1.06 | +0.91 | −1.17 | +0.21 | −0.78 | −0.19 |
| Seed 22, eighth, assignment | no-replay | −1.23 | +0.18 | −1.46 | +0.27 | −0.79 | 0.00 | −0.78 | −0.13 |
| Seed 22, eighth, assignment | plain-first | −1.47 | +1.18 | −1.39 | +1.06 | −0.88 | +0.28 | −0.73 | −0.22 |

- **The pair takes A and C's margin under both forms about alike**: 0.98 to 2.05 M over its blocks under plain nights and 0.83 to 2.20 M
  under nights without replay. B and D's: 1.06 to 2.07 M and 0.87 to 2.22 M.
- **Plain nights give the shared kinds back more**: A and C 0.91 to 1.18 M, B and D 0.91 to 1.17 M. Nights without replay give 0.05 to
  0.21 M and 0.03 to 0.34 M.
- **C's own lose margin onto A's answer over the pair's blocks**, 0.79 to 1.31 M, and D's own 0.73 to 1.09 M.

**On the conflict on the shared units**, the pair takes as much under one form as under the other, in all four cells, and plain nights give
more back. That does not separate the two cells either.

## 4. Why the margin and the count part

A's count, plain nights less nights without replay, the 48 differences of a checkpoint ± their paired SE, split by the stimulus a trial
presented, with the ties on both stimuli's trials and the readouts' spikes on A's trials:

| Cell | Block | A's count | Correct on A | Correct on B | Ties | Readout spikes on A's trials |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | +1.9 ± 0.68 | +0.6 ± 0.31 | +1.3 ± 0.56 | −0.6 ± 0.42 | −229.0 ± 10.55 |
| Seed 23, first mapping, mirrored | 12 | +2.0 ± 1.26 | +1.1 ± 0.70 | +1.0 ± 1.17 | −0.6 ± 0.56 | −334.6 ± 13.11 |
| Seed 23, first mapping, mirrored | 18 | +0.3 ± 1.17 | −1.0 ± 0.92 | +1.3 ± 0.69 | +0.4 ± 0.43 | −341.7 ± 20.91 |
| Seed 23, first mapping, mirrored | 24 | −2.6 ± 1.30 | −5.0 ± 1.25 | +2.4 ± 0.38 | +0.6 ± 0.36 | −378.0 ± 23.20 |
| Seed 23, first mapping, mirrored | 36 | −4.9 ± 1.09 | −5.2 ± 1.50 | +0.3 ± 0.88 | +1.7 ± 0.21 | −225.9 ± 20.69 |
| Seed 24, eighth, assignment | 6 | +0.3 ± 0.64 | +2.5 ± 0.77 | −2.3 ± 0.71 | +0.5 ± 0.38 | −494.6 ± 10.74 |
| Seed 24, eighth, assignment | 12 | −0.6 ± 1.03 | +0.4 ± 1.31 | −1.0 ± 1.51 | +0.3 ± 0.24 | −645.7 ± 12.44 |
| Seed 24, eighth, assignment | 18 | −5.9 ± 1.53 | −3.6 ± 0.86 | −2.4 ± 1.34 | +2.0 ± 0.40 | −807.3 ± 16.54 |
| Seed 24, eighth, assignment | 24 | −5.9 ± 1.85 | −3.9 ± 0.78 | −2.0 ± 1.34 | +1.2 ± 0.27 | −902.7 ± 21.00 |
| Seed 24, eighth, assignment | 36 | +1.8 ± 1.79 | +0.2 ± 0.76 | +1.6 ± 1.50 | +0.4 ± 0.37 | −666.9 ± 27.54 |
| Seed 22, first mapping, mirrored | 6 | +1.1 ± 0.63 | +0.7 ± 0.36 | +0.4 ± 0.54 | +0.2 ± 0.37 | −229.3 ± 9.32 |
| Seed 22, first mapping, mirrored | 12 | +2.9 ± 1.93 | +1.1 ± 0.68 | +1.8 ± 1.42 | +0.1 ± 0.56 | −321.2 ± 11.02 |
| Seed 22, first mapping, mirrored | 18 | +3.7 ± 2.22 | −0.4 ± 1.04 | +4.1 ± 1.42 | −0.6 ± 0.45 | −340.6 ± 13.36 |
| Seed 22, first mapping, mirrored | 24 | +4.7 ± 2.24 | +0.1 ± 1.09 | +4.6 ± 1.47 | −0.4 ± 0.81 | −377.6 ± 16.72 |
| Seed 22, first mapping, mirrored | 36 | +5.0 ± 1.24 | −0.4 ± 0.83 | +5.4 ± 1.47 | −0.7 ± 0.35 | −233.0 ± 10.87 |
| Seed 22, eighth, assignment | 6 | +1.9 ± 0.65 | +2.0 ± 0.46 | −0.1 ± 0.58 | −0.3 ± 0.33 | −360.9 ± 12.94 |
| Seed 22, eighth, assignment | 12 | +3.0 ± 0.93 | +2.5 ± 0.45 | +0.5 ± 0.81 | 0.0 ± 0.45 | −482.9 ± 14.73 |
| Seed 22, eighth, assignment | 18 | +3.4 ± 1.05 | −0.6 ± 0.88 | +4.0 ± 0.58 | −0.1 ± 0.45 | −548.4 ± 13.30 |
| Seed 22, eighth, assignment | 24 | +2.8 ± 0.89 | −0.3 ± 0.99 | +3.1 ± 1.17 | 0.0 ± 0.24 | −590.9 ± 12.79 |
| Seed 22, eighth, assignment | 36 | +2.2 ± 0.65 | −1.6 ± 0.88 | +3.8 ± 0.81 | −0.1 ± 0.18 | −438.4 ± 10.48 |

- **On stimulus A's trials no cell reads more under plain nights at block 36 by as much as its paired SE**: −5.2 ± 1.50, +0.2 ± 0.76,
  −0.4 ± 0.83 and −1.6 ± 0.88. At block 6 every cell read more, +0.6 to +2.5.
- **On stimulus B's trials they read more in seed 22's two cells**, +5.4 ± 1.47 and +3.8 ± 0.81 at block 36, and on seed 24 +1.6 ± 1.50. On
  seed 23 they read +0.3 ± 0.88.
- **The readouts fire less on A's trials under plain nights at every checkpoint of every cell**, by 226 to 903 spikes a read. At block 36 that
  is 20.2 % under on seed 23, 34.8 % on seed 24, and 21.6 % and 25.7 % on seed 22's two cells.
- **Seed 24 reads less on both stimuli at blocks 18 and 24**: on A's trials −3.6 ± 0.86 and −3.9 ± 0.78, beyond twice the SE, and on B's
  −2.4 ± 1.34 and −2.0 ± 1.34, within it.

At block 36, each form, A's count of 64 split by stimulus, the six kinds' margins onto A's answer, A's margin and the sums:

| Cell | Form | A's count | Correct on A | Correct on B | Ties | A alone | A and C | B alone | B and D | C's own | D's own | A's margin | Excitatory sum, M | Inhibitory sum, M |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | no-replay | 53.1 | 29.3 | 23.9 | 3.0 | 0.35 | -0.31 | 1.30 | -0.78 | -1.01 | -0.59 | 0.56 | 214.7 | 124.1 |
| Seed 23, first mapping, mirrored | plain-first | 48.3 | 24.1 | 24.2 | 4.7 | 1.03 | 0.42 | 2.03 | -0.04 | -1.27 | -0.88 | 3.45 | 192.8 | 146.2 |
| Seed 23, first mapping, mirrored | plain-both | 50.1 | 25.7 | 24.5 | 4.3 | 0.85 | -0.44 | 1.64 | -0.72 | -1.36 | -1.10 | 1.32 | 192.2 | 144.3 |
| Seed 23, first mapping, mirrored | awake | 43.6 | 23.8 | 19.8 | 5.2 | 0.35 | -0.37 | 1.16 | -0.82 | -0.81 | -0.66 | 0.32 | 213.2 | 125.1 |
| Seed 24, eighth, assignment | no-replay | 25.6 | 12.9 | 12.7 | 5.3 | 0.63 | -1.10 | -0.17 | -0.34 | -1.03 | -1.29 | -0.97 | 214.0 | 77.4 |
| Seed 24, eighth, assignment | plain-first | 27.4 | 13.1 | 14.3 | 5.7 | 1.62 | -0.05 | 0.69 | 0.65 | -0.82 | -1.17 | 2.90 | 189.8 | 93.1 |
| Seed 24, eighth, assignment | plain-both | 29.6 | 14.3 | 15.3 | 5.2 | 1.42 | -0.81 | 0.36 | -0.02 | -0.99 | -1.60 | 0.95 | 190.8 | 89.9 |
| Seed 24, eighth, assignment | awake | 29.6 | 17.6 | 12.0 | 5.5 | 0.95 | -0.81 | 0.16 | -0.44 | -0.72 | -1.02 | -0.14 | 213.6 | 75.8 |
| Seed 22, first mapping, mirrored | no-replay | 44.1 | 27.8 | 16.4 | 5.0 | 1.23 | -0.06 | 0.68 | -0.23 | -0.86 | -1.27 | 1.62 | 214.0 | 123.6 |
| Seed 22, first mapping, mirrored | plain-first | 49.1 | 27.4 | 21.8 | 4.3 | 1.78 | 0.66 | 1.33 | 0.49 | -0.76 | -1.33 | 4.26 | 193.5 | 143.1 |
| Seed 22, first mapping, mirrored | plain-both | 48.8 | 26.8 | 22.0 | 4.2 | 1.68 | -0.15 | 1.10 | -0.14 | -1.08 | -1.73 | 2.50 | 193.3 | 142.7 |
| Seed 22, first mapping, mirrored | awake | 43.2 | 24.4 | 18.8 | 5.1 | 0.97 | -0.09 | 0.63 | -0.29 | -0.79 | -1.12 | 1.22 | 213.6 | 124.9 |
| Seed 22, eighth, assignment | no-replay | 44.2 | 24.7 | 19.5 | 4.1 | 1.12 | -0.33 | 0.63 | -0.42 | -0.64 | -1.27 | 1.00 | 213.1 | 78.1 |
| Seed 22, eighth, assignment | plain-first | 46.4 | 23.1 | 23.2 | 4.0 | 1.93 | 0.43 | 1.46 | 0.44 | -0.45 | -1.31 | 4.26 | 190.7 | 94.1 |
| Seed 22, eighth, assignment | plain-both | 54.0 | 27.1 | 26.9 | 3.0 | 1.81 | -0.33 | 1.29 | -0.22 | -0.85 | -1.63 | 2.56 | 190.7 | 90.5 |
| Seed 22, eighth, assignment | awake | 43.0 | 20.9 | 22.1 | 5.0 | 1.24 | -0.52 | 0.83 | -0.49 | -0.56 | -1.07 | 1.06 | 213.6 | 77.0 |

- **On seed 23, 2.88 M more of A's margin under plain nights is spread over A's and B's four kinds**: A alone +0.68, A and C +0.73, B alone
  +0.73, B and D +0.74. The count's −4.9 lies on A's trials: correct 24.1 against 29.3, wrong 5.8 against 2.0, ties 2.4 against 1.0. On B's
  trials it read 24.2 against 23.9.
- **The margin does not order the count across cells.** Seed 23 without replay read 53.1 at 0.56 M, and seed 22's first mapping under plain
  nights 49.1 at 4.26 M.
- **On B's trials the two forms end apart on seed 22's first mapping and together on seed 23.** From block 6 to block 36 seed 23 read 24.6
  to 23.9 without replay and 25.9 to 24.2 under plain nights; seed 22's first mapping 26.0 to 16.4 and 26.4 to 21.8.
- **At block 36 the sums stand apart in all four cells**: the excitatory sum 189.8 to 193.5 M under plain nights and 213.1
  to 214.7 M without replay, the inhibitory sum 15.7 to 22.1 M higher under plain nights.
- **C's own stands lower under plain nights on seed 23 alone**, −0.26 M against the night without replay, where seed 24 reads +0.21 and
  seed 22's cells +0.10 and +0.19. D's own stands −0.29 M on seed 23, +0.13 on seed 24, and −0.06 and −0.04 on seed 22's cells.

Where A's and B's units put their margin at block 36: onto the readouts' units outside the targets and the pair's own (the rest), onto the
targets, and onto the pair's own, each readout's part onto the answer's less the same part onto the other, in millions:

| Cell | Image | A's units onto the rest | onto the targets | onto the pair's own | B's units onto the rest | onto the targets | onto the pair's own |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | the start | -0.31 | 1.15 | 0.12 | -0.46 | 1.82 | -0.13 |
| Seed 23, first mapping, mirrored | no-replay | -0.71 | 0.61 | 0.14 | -0.74 | 1.42 | -0.16 |
| Seed 23, first mapping, mirrored | plain-first | -0.97 | 2.18 | 0.25 | -0.69 | 2.67 | 0.01 |
| Seed 24, eighth, assignment | the start | 0.39 | 1.46 | 0.11 | 0.28 | 1.31 | 0.11 |
| Seed 24, eighth, assignment | no-replay | -0.09 | -0.47 | 0.10 | -0.60 | 0.05 | 0.05 |
| Seed 24, eighth, assignment | plain-first | -0.32 | 1.66 | 0.22 | -0.45 | 1.60 | 0.18 |
| Seed 22, first mapping, mirrored | the start | 1.00 | 0.56 | 0.14 | -0.32 | 1.46 | 0.13 |
| Seed 22, first mapping, mirrored | no-replay | 0.90 | 0.17 | 0.10 | -0.89 | 1.22 | 0.11 |
| Seed 22, first mapping, mirrored | plain-first | 0.31 | 1.87 | 0.25 | -0.74 | 2.34 | 0.23 |
| Seed 22, eighth, assignment | the start | 1.62 | 0.23 | 0.12 | -0.14 | 1.56 | 0.19 |
| Seed 22, eighth, assignment | no-replay | 1.24 | -0.54 | 0.09 | -0.86 | 0.88 | 0.19 |
| Seed 22, eighth, assignment | plain-first | 0.74 | 1.45 | 0.18 | -0.69 | 2.30 | 0.29 |

- **Plain nights put A's units' margin onto the targets and take it off the rest, in all four cells.** Against the night without replay the
  targets stood +1.57, +2.13, +1.71 and +1.99 M, and the rest −0.26, −0.23, −0.59 and −0.51 M.
- **B's units gain on both**, on the targets +1.11 to +1.56 M and on the rest +0.06 to +0.17 M.
- **The rest's loss does not order the cells by A's trials either.** Seed 22's two cells lost more of it than seed 23 and read −0.4 and −1.6
  on A's trials, against −5.2.

**On the margin and the count**, seed 23's margin rises on every kind of A's and B's units, and its count falls on stimulus A's trials
alone. Its readouts fire a fifth less on those trials, as in the cells that held. Among everything this note read of the nights, what sets
seed 23 apart is C's own units. They lost margin onto A's answer in every path-night of either form, and stood 0.26 M lower under plain
nights at block 36. On seed 24 at blocks 18 and 24 plain nights read less on A's trials beyond twice the SE and on B's within it. Nothing in
the tables above sets its nights apart from seed 22's eighth image.

## What it costs

The trial took 383 s. The four cells took 3 874, 3 898, 3 918 and 4 106 s, at most 28.5 % of a job's 240 minutes; H-47's jobs of the same
forms took 3 935 to 4 010 s. In the trial the first night without replay took 26 s watched and 21 s unwatched, and the plain night 27 s
watched; the three ran two at a time, so the times include sharing the runner. Nothing was cut: every night of every path was watched, and
all four cells ran. A first dispatch, [41499230117](https://github.com/Menteion/Menteion/actions/runs/41499230117) on
`b8f6c65`, was cancelled seven minutes in, its logs unread, to add the class matrix. Seven jobs started, five ran to the end.

## What it suggests

**On these four cells, what the nights moved on A's and B's synapses did not separate the cells that failed.** Every plain night moved the
margin A's way, in the cells that failed as in those that held. The patterns pointed at the right answer, and the pair took the shared
units' margin under both forms alike. The cells differed in the reads: which stimulus's trials gained, and by how much.

**The replay reaches past the patterns.** In every cell four plain nights took 19.75 to 23.59 M of the readouts' excitatory weight, most of it
from the units the patterns reach, and added inhibition onto those units. At block 36 the inhibitory sum stood 15.7 to 22.1 M above the night
without replay's, and the readouts fired a fifth to a third less on A's trials. Whether that costs the count, and on which stimulus, was not
separated from the rest here. A night whose replay drives the readouts' units without moving their own synapses would be one contrast that
separates it.

**The pair's own units sit in A's readouts.** C's own lie in readout 0 and D's own in readout 1, and A's and B's patterns reach some of them.
Their margin onto A's answer was the one reading of the nights that set seed 23 apart. Four cells cannot say whether that is a cause.

Questions this suggests for a later exploration or a round:
- A's reads unit by unit: which readout units fire on A's trials and on B's, under each form, the pair's own units among them.
- A night whose replay leaves the readouts' units' synapses, excitatory and inhibitory, where they stood, read on these four cells.
- The pair's own units' weights, read night by night and stretch by stretch, on seed 23 and on cells where they do not fall.
- The other eight cells of H-47, with the class matrix and the split, to see whether the stimulus that gains follows anything read here.

## What did not run

- **The awake gap and plain nights on both mappings** were read from H-47's logs: no split, no class matrix, no watch for them.
- **The pairings of any unit but A's and B's 102 stimulus units.** The readouts' and the inhibitory units' synapses are read only as sums by
  class, from the images before and after each gap, not pairing by pairing.
- **REM's depotentiations and the zero sum's rounds** are not split out of a night's change.
- **A read unit by unit.** A read's trials give each readout's spike count, not which units fired.
- **H-47's other eight cells**, and any SE but the paired SEs of section 4's differences: the kinds, the split, the matrix and the pairings
  are means over three paths.
- **Any other form, shift, baseline or network.**

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_what_a_replay_costs_explore` | [41500459562](https://github.com/Menteion/Menteion/actions/runs/41500459562) | `1f2d1b3` | 383 |
| `what_a_replay_costs_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [41500459562](https://github.com/Menteion/Menteion/actions/runs/41500459562) | `1f2d1b3` | 3874 |
| `what_a_replay_costs_the_eighth_image_on_seed_24_from_the_assignment_explore` | [41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777) | `1f2d1b3` | 3898 |
| `what_a_replay_costs_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777) | `1f2d1b3` | 4106 |
| `what_a_replay_costs_the_eighth_image_on_seed_22_from_the_assignment_explore` | [41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777) | `1f2d1b3` | 3918 |
