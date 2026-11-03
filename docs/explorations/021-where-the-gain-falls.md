---
status: exploratory
date: 2026-11-03
---

# 021: Where a replay's gain falls

**Branch**: `explore/021-where-the-gain-falls`
**Commit**: `6c89a39`, which adds tests and nothing under `src/`, on `71e52df`, `main` at ADR-0238's merge, with exploration 020's tag
merged in (`0597735`).
**Runs**: [41582250345](https://github.com/Menteion/Menteion/actions/runs/41582250345) (the harness's trial),
[41583848509](https://github.com/Menteion/Menteion/actions/runs/41583848509) (A's and B's roles exchanged, four cells) and
[41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) (exploration 020's job on H-47's other eight cells), all at
`scope=arms`. Questions 1 and 2 read H-47's run [41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613), whose
reproduction [41468257319](https://github.com/Menteion/Menteion/actions/runs/41468257319) is the same bit for bit, and questions 4 and 5 read
exploration 020's runs [41500459562](https://github.com/Menteion/Menteion/actions/runs/41500459562) and
[41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777) for its four cells. None of those was run again.

[ADR-0238](../adr/0238-what-keeps-a-mapping-where-the-replay-did-not.md) read H-47's rows leaning by the readout A answers: clause 1 read
+3.8 to +19.6 in the six cells where A answers readout 0 and −4.9 to +6.0 in the six where it answers readout 1, both failures among the
latter, the two ranges overlapping. [Exploration 020](020-what-a-replay-costs.md)'s four cells, all of the second kind, read plain nights
on A's patterns less nights without replay at −5.2 to +0.2 on stimulus A's trials at block 36, none above twice its SE, and +0.3 to +5.4 on
B's; at block 6, +0.6 to +2.5 on A's and −2.3 to +1.3 on B's. This note asks where a replay's gain
falls: on which stimulus's trials, and whether it goes with the stimulus or with the readout that stimulus answers. It asks, at blocks 6,
12, 18, 24 and 36:

1. on each of H-47's twelve cells, each stimulus's correct trials under the four forms, plain nights on A's patterns less nights without
   replay on each stimulus's trials, and the side;
2. what a night without replay, and a plain night, cost each stimulus against the awake gap;
3. whether the side moves when A's and B's roles are exchanged within one start;
4. where the pair's own units lie, against the side;
5. what plain nights take from each readout's own synapses, by orientation.

**The side** is ADR-0238's: at a checkpoint, the stimulus whose plain less without replay difference is the larger of the two. The note
writes no prediction and decides nothing. **Every figure here is exploratory**: from one run of each test on the repository's AArch64
host, in H-47's terms. A count is a mean over a cell's 48 reads, sixteen on each of three paths, and a difference carries its paired SE
unless another is named.

**In brief**
- **The runs are H-47's.** The eight new cells of exploration 020's job held all their rows to H-47's inside the test, and the trial held
  H-47's rows with the roles as H-47 has them. The exchanged runs share H-47's lines up to the zero sum's first two groups, the
  derivation among them, and none after.
- **At blocks 6 and 12 the side lies on stimulus A in 10 cells of 12**, five of each orientation, so on readout 0 in six and readout 1 in
  six. Every cell whose A's less B's is beyond twice its SE puts it on A but one, at block 12.
- **From block 18 it lies on readout 0 in 9, 9 and 8 cells of 12**: on A where A answers readout 0, 5, 4 and 4 of six, and on B where A
  answers readout 1, 4, 5 and 4 of six. Beyond twice the SE, 7 of 9, 6 of 8 and 6 of 7 lie on readout 0.
- **Stimulus or readout**: in the six cells where A answers readout 1, the only ones that tell them apart, the side goes with the stimulus
  in 5, 5, 2, 1 and 2 cells at blocks 6 to 36 and with the readout in 1, 1, 4, 5 and 4.
- **A night without replay pays against the awake gap where A answers readout 0**, on A's trials: below by more than twice the SE in 5
  cells of six at blocks 18, 24 and 36, down to −10.1. Where A answers readout 1 it stood above the awake gap on A's trials in 3, 3 and 4
  cells and below in 0, 0 and 1.
- **Exchanging A's and B's roles within a start does not move the side from block 18.** In all four cells it stays on the same units and
  readout, which now play the other role. At blocks 6 and 12 it moves with the role in 2 and 3 cells, none beyond twice the SE in both runs.
- **The pair's own units lie where the layout puts them in all twelve cells**, C's inside readout 0 and D's inside readout 1. So they lie
  in the readout their own stimulus answers where A answers readout 1, and in the other where A answers readout 0.
- **Plain nights add more inhibition onto stimulus A's readout's excitatory units in all twelve cells**, by 1.35 to 3.71 M where A answers
  readout 0 and 0.10 to 1.41 M where it answers readout 1. Within each readout they take more where A answers readout 1, 3.40 to 5.27 M
  over both readouts against 0.44 to 1.87 M. The readout each reading moves more is the side's at block 36 in 6 and 5 cells of 12.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
exploration 020 restates them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C sharing
26 of A's 51 source units and D 26 of B's, its answers the other way. **The orientation** is the mapping in force: under the assignment A
answers readout 0 and B readout 1, under the mirrored one the other way. It is the harness's `nights_mirrored`. The six cells of each
orientation are:

| A answers | Mapping in force | Its six cells |
| :--- | :--- | :--- |
| Readout 0 | the assignment | seed 22, 23 and 24 from the eighth image and the mirrored assignment, and from a first mapping and the assignment |
| Readout 1 | the mirrored one | seed 22, 23 and 24 from the eighth image and the assignment, and from a first mapping and the mirrored assignment |

A cell is named, as in H-47, by its seed, its image and the arm it starts from, not by the mapping in force: "Seed 22, eighth, mirrored" is
seed 22's eighth image from the mirrored assignment, where the mapping in force is the assignment.

**A stimulus's trials.** A read's line carries its 64 trials as three bit masks, `stimuli`, `right` and `tied`, bit *i* for trial *i*. A set
`stimuli` bit is a trial that presented stimulus B, the second of A's two sets: in every one of H-47's 23 040 reads, A's and the pair's, the
second field of the read's block, the trials that presented its first stimulus, is 64 less the set bits. A trial is correct when its `right`
bit is set, a tie when its `tied` bit is, and wrong otherwise. A read of A presents 28 to 37 trials of stimulus A, 32.25 on average. Split
this way, exploration 020's four cells read here as its note gave them, all 20 rows of its section 4 to the last digit.

**The SEs.** A stimulus's plain less without replay difference is H-47's paired SE over its 48 differences, the two forms run in one job. So
is the side's own, A's difference less B's, over its 48 differences of differences; a side "within 2 SE" is one whose A's less B's is within
twice that SE. Against the awake gap, run in the other job, the SE is exploration 018's unpaired one: the mean over the paths of each path's
difference of means, ± √((the night's stream variance + the awake gap's)/16 + (the paths' differences' SD)²/3), each stream variance from
its own form's split, a negative one as zero. Every count of cells above, within or below twice an SE is taken on the unrounded mean and
SE, so a printed figure on the edge can read the other way: three cells of question 2's tables do (named there).

**Joining the logs.** Each of H-47's twelve cells ran in two jobs, the night without replay and the plain night on A's patterns in one, plain
nights on both and the awake gap in the other. Each job ran its own head, the pair's first six blocks on each path. The head is read in no
table here; each form's reads come from its one job, and no read is taken twice.

**A's and B's roles exchanged (question 3).** `with_roles`, on the branch, lays a start out as H-47's `with_nights` does, but with A's two
sets exchanged. Stimulus A presents the 51 units of H-47's stimulus B, at place 11, and stimulus B those of H-47's A, at place 0. The mapping
in force is named the other way, so that each set of units answers the readout it answers in H-47 and in the start image. The pair is built
on the exchanged sets: C shares 26 of the new A's units, which are H-47's B's, and D 26 of H-47's A's. C's own units stay at place 5 inside
readout 0 and D's at place 16 inside readout 1. So, against H-47:
- **What stays**: the derivation, the start image and its couplings, the zero sum's six groups and their values (in the exchanged order),
  A's 102 patterns as a set (held inside the trial, on seed 23's cell; the jobs draw them by the same call), each set of units' answer,
  the forms, the paths and the reads' streams.
- **What moves**:
  - **Which units a trial presents.** The sequence of stimuli is H-47's by index, so a trial that presented H-47's A's units presents its
    B's. In every read the `stimuli` mask equals H-47's, in all 3 840 exchanged reads; the pair's blocks draw their trials from the same
    seeds.
  - **Which of the pair's own units go with which shared units.** The pair's stimulus that shares H-47's B's units has its own at place 5
    instead of 16. The pairs of a unit C presents and one D presents within a local synapse's reach on the ring (`PRIOR_WINDOW`) fall from
    H-47's 25 to none, in all four cells.
  - **The order of A's patterns**: the new A's first.
- **Two forms ran**, the night without replay and the plain night on A's patterns, which the side needs, on four cells (below). Their rows
  are held to nothing: H-47 ran none of them.

The four cells, with H-47's side in each: seed 23's first mapping from the mirrored assignment, on B's trials (readout 0) from block 18;
seed 23's first mapping from the assignment, on A's (readout 0) at every checkpoint; seed 24's eighth image from the mirrored assignment,
on B's (readout 1) at every checkpoint; and seed 22's eighth image from the assignment, on A's (readout 1) at blocks 6 and 12 and on B's
(readout 0) from block 18. In the exchanged runs each stimulus's trials are read by the units they presented, so that a side is compared
with H-47's on the same units.

**Exploration 020's job on the other eight cells (questions 4 and 5).** `e020_job`, as exploration 020 ran it on its four cells: the night
without replay and the plain night on A's patterns, every row held to H-47's inside the test, with the split, the class matrix and
`ReplayWatch` at every night. Questions 4 and 5 read these eight logs with exploration 020's four, each cell from its one job, so each
head is read once.

**Nothing under `src/` changed.**

## The runs are H-47's

- **The trial** ran seed 23's first mapping from the mirrored assignment on path 0: the pair's head, the first night without replay and
  the first plain night on A's patterns, each image read once as it was of A and of the pair, with the roles as H-47 has them and then
  exchanged. As H-47 has them, each image's CRC-64, the two reads, A's couplings and the gain held H-47's row at block 6, inside the test,
  and 110 of its 112 lines are lines of H-47's job, label aside; the other two are its notices that it held them. Exchanged, the zero sum's
  six groups held H-47's values in the exchanged order and A's patterns H-47's with the new A's first, both inside the test.
- **The exchanged jobs** share with H-47's job of the same cell 38 lines of 1 910 from a first mapping and 37 of 1 909 from the eighth
  image: the derivation and its calibration, the zero sum's two groups of 51 units and their values, and a first mapping's line. From the
  pair's six groups on, every line differs, as the layout does: the groups' line holds the same values in the exchanged order.
- **Exploration 020's job on the other eight cells** held each cell's 30 rows to H-47's inside the test, and printed "every row is H-47's"
  for both forms. Each log holds every one of H-47's lines of the same job, label aside, but its two notices that the rows were not yet
  pinned: 1 908 of 1 910 from a first mapping and 1 907 of 1 909 from the eighth image. The watch read `wrong 0` in all 192 nights. Every
  night without replay read `replays 0` and `depotentiations 0`, and each plain night 1 128 to 1 340 replays and 545 to 640
  depotentiations.

## 1. The side, cell by cell

Each stimulus's correct trials, of the trials a read presents of it (28 to 37 of the 64 for A, so 27 to 36 for B), under the four forms
in the order without replay, plain on A's, plain on both and awake; then plain on A's less without replay on each stimulus's trials, and
A's less B's, ± their paired SEs. Where A answers readout 0:

| Cell | Block | Stimulus A's trials (readout 0): without replay, plain on A's, plain on both, awake | Stimulus B's trials (readout 1): the same | A's: plain less without replay | B's: plain less without replay | A's less B's | Side |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
| Seed 22, eighth, mirrored | 6 | 24.8, 29.0, 28.8, 28.0 | 21.5, 24.4, 22.5, 21.2 | +4.2 ± 0.89 | +2.9 ± 1.07 | +1.4 ± 1.74 | A, readout 0 |
| Seed 22, eighth, mirrored | 12 | 11.3, 19.1, 16.7, 16.7 | 18.9, 23.2, 21.4, 17.4 | +7.8 ± 1.13 | +4.3 ± 0.84 | +3.5 ± 1.80 | A, readout 0 |
| Seed 22, eighth, mirrored | 18 | 9.5, 18.1, 13.3, 15.6 | 16.8, 23.7, 23.4, 17.6 | +8.6 ± 1.79 | +6.9 ± 1.22 | +1.7 ± 2.57 | A, readout 0 |
| Seed 22, eighth, mirrored | 24 | 8.6, 14.7, 10.7, 15.0 | 14.0, 25.5, 23.7, 17.0 | +6.1 ± 1.14 | +11.6 ± 1.20 | −5.5 ± 1.80 | B, readout 1 |
| Seed 22, eighth, mirrored | 36 | 4.7, 13.9, 9.5, 14.8 | 11.9, 22.3, 21.3, 16.1 | +9.2 ± 0.86 | +10.4 ± 0.54 | −1.2 ± 0.97 | B, readout 1 |
| Seed 22, first mapping, assignment | 6 | 25.5, 29.6, 29.2, 26.7 | 26.9, 26.9, 27.1, 25.1 | +4.1 ± 0.45 | 0.0 ± 0.50 | +4.1 ± 0.49 | A, readout 0 |
| Seed 22, first mapping, assignment | 12 | 21.1, 24.9, 23.9, 22.6 | 23.6, 22.7, 22.3, 19.6 | +3.8 ± 0.48 | −0.9 ± 0.40 | +4.7 ± 0.68 | A, readout 0 |
| Seed 22, first mapping, assignment | 18 | 16.4, 24.8, 22.6, 22.5 | 23.7, 22.2, 22.4, 18.8 | +8.3 ± 0.81 | −1.6 ± 0.69 | +9.9 ± 1.00 | A, readout 0 |
| Seed 22, first mapping, assignment | 24 | 15.5, 23.8, 20.2, 20.4 | 23.4, 24.3, 24.5, 16.9 | +8.3 ± 1.55 | +0.8 ± 0.85 | +7.5 ± 2.09 | A, readout 0 |
| Seed 22, first mapping, assignment | 36 | 13.2, 21.5, 19.5, 19.6 | 23.1, 22.7, 22.7, 17.2 | +8.4 ± 1.10 | −0.4 ± 0.83 | +8.8 ± 1.88 | A, readout 0 |
| Seed 23, eighth, mirrored | 6 | 20.0, 22.5, 21.8, 22.6 | 14.4, 13.6, 13.2, 14.9 | +2.5 ± 0.98 | −0.8 ± 0.90 | +3.4 ± 1.46 | A, readout 0 |
| Seed 23, eighth, mirrored | 12 | 15.3, 21.5, 18.6, 17.7 | 4.2, 7.0, 6.6, 9.4 | +6.1 ± 0.90 | +2.8 ± 0.44 | +3.3 ± 0.93 | A, readout 0 |
| Seed 23, eighth, mirrored | 18 | 12.3, 24.4, 20.5, 17.5 | 2.6, 3.2, 6.8, 7.7 | +12.1 ± 1.05 | +0.6 ± 0.57 | +11.5 ± 1.34 | A, readout 0 |
| Seed 23, eighth, mirrored | 24 | 8.9, 25.8, 19.1, 16.1 | 1.7, 3.5, 8.3, 5.6 | +16.9 ± 1.43 | +1.8 ± 0.26 | +15.1 ± 1.26 | A, readout 0 |
| Seed 23, eighth, mirrored | 36 | 7.4, 23.3, 18.0, 15.6 | 0.8, 2.3, 4.7, 5.1 | +15.9 ± 1.05 | +1.5 ± 0.28 | +14.4 ± 1.09 | A, readout 0 |
| Seed 23, first mapping, assignment | 6 | 29.2, 30.9, 30.3, 29.5 | 25.0, 25.4, 24.7, 24.6 | +1.6 ± 0.46 | +0.4 ± 0.57 | +1.3 ± 0.67 | A, readout 0 |
| Seed 23, first mapping, assignment | 12 | 24.2, 30.1, 26.9, 25.6 | 9.3, 7.4, 8.1, 10.7 | +6.0 ± 0.87 | −1.8 ± 0.85 | +7.8 ± 1.24 | A, readout 0 |
| Seed 23, first mapping, assignment | 18 | 19.5, 30.1, 21.6, 26.1 | 7.6, 4.9, 5.9, 7.5 | +10.6 ± 1.65 | −2.8 ± 1.24 | +13.4 ± 1.44 | A, readout 0 |
| Seed 23, first mapping, assignment | 24 | 17.3, 30.4, 21.3, 25.0 | 7.5, 4.3, 5.1, 6.5 | +13.2 ± 1.45 | −3.2 ± 0.67 | +16.4 ± 1.23 | A, readout 0 |
| Seed 23, first mapping, assignment | 36 | 15.7, 29.2, 20.6, 25.6 | 4.0, 3.9, 4.2, 5.6 | +13.5 ± 1.09 | −0.1 ± 0.62 | +13.6 ± 0.84 | A, readout 0 |
| Seed 24, eighth, mirrored | 6 | 19.1, 17.0, 14.4, 15.4 | 17.3, 16.8, 14.4, 15.8 | −2.1 ± 0.99 | −0.5 ± 1.17 | −1.6 ± 1.69 | B, readout 1 |
| Seed 24, eighth, mirrored | 12 | 15.3, 12.1, 9.9, 12.0 | 16.3, 16.6, 15.5, 14.3 | −3.2 ± 0.72 | +0.3 ± 0.77 | −3.5 ± 0.91 | B, readout 1 |
| Seed 24, eighth, mirrored | 18 | 9.2, 9.9, 10.0, 11.9 | 11.0, 18.6, 15.1, 14.7 | +0.7 ± 0.25 | +7.6 ± 1.52 | −6.9 ± 1.32 | B, readout 1 |
| Seed 24, eighth, mirrored | 24 | 7.4, 9.0, 8.3, 10.6 | 9.0, 16.8, 14.1, 13.2 | +1.6 ± 1.33 | +7.8 ± 1.47 | −6.2 ± 2.32 | B, readout 1 |
| Seed 24, eighth, mirrored | 36 | 5.3, 6.9, 8.8, 9.3 | 7.2, 16.9, 11.6, 13.0 | +1.5 ± 0.74 | +9.7 ± 0.98 | −8.2 ± 1.45 | B, readout 1 |
| Seed 24, first mapping, assignment | 6 | 28.8, 30.7, 30.2, 27.5 | 29.5, 30.6, 30.0, 28.3 | +1.9 ± 0.57 | +1.1 ± 0.38 | +0.8 ± 0.65 | A, readout 0 |
| Seed 24, first mapping, assignment | 12 | 26.4, 28.6, 27.9, 23.1 | 25.6, 26.2, 22.9, 21.2 | +2.2 ± 0.43 | +0.6 ± 0.99 | +1.6 ± 1.22 | A, readout 0 |
| Seed 24, first mapping, assignment | 18 | 24.9, 28.9, 27.5, 22.5 | 22.6, 22.5, 20.5, 17.5 | +4.0 ± 0.73 | −0.1 ± 1.55 | +4.1 ± 1.97 | A, readout 0 |
| Seed 24, first mapping, assignment | 24 | 22.3, 28.4, 26.9, 21.6 | 21.4, 24.0, 20.9, 14.8 | +6.1 ± 1.25 | +2.6 ± 2.15 | +3.5 ± 1.53 | A, readout 0 |
| Seed 24, first mapping, assignment | 36 | 21.2, 24.5, 24.8, 20.8 | 18.8, 19.3, 17.3, 14.4 | +3.3 ± 0.54 | +0.5 ± 1.44 | +2.8 ± 1.52 | A, readout 0 |

Where A answers readout 1:

| Cell | Block | Stimulus A's trials (readout 1): without replay, plain on A's, plain on both, awake | Stimulus B's trials (readout 0): the same | A's: plain less without replay | B's: plain less without replay | A's less B's | Side |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
| Seed 22, eighth, assignment | 6 | 24.4, 26.4, 26.1, 24.1 | 25.9, 25.8, 27.0, 24.7 | +2.0 ± 0.46 | −0.1 ± 0.58 | +2.1 ± 0.83 | A, readout 1 |
| Seed 22, eighth, assignment | 12 | 22.9, 25.4, 26.9, 22.1 | 24.8, 25.3, 27.1, 23.4 | +2.5 ± 0.45 | +0.5 ± 0.81 | +2.0 ± 0.92 | A, readout 1 |
| Seed 22, eighth, assignment | 18 | 24.4, 23.8, 28.1, 21.0 | 22.8, 26.8, 28.1, 22.5 | −0.6 ± 0.88 | +4.0 ± 0.58 | −4.6 ± 1.07 | B, readout 0 |
| Seed 22, eighth, assignment | 24 | 26.0, 25.8, 28.7, 21.3 | 21.3, 24.4, 27.5, 22.4 | −0.3 ± 0.99 | +3.1 ± 1.17 | −3.4 ± 1.97 | B, readout 0 |
| Seed 22, eighth, assignment | 36 | 24.7, 23.1, 27.1, 20.9 | 19.5, 23.2, 26.9, 22.1 | −1.6 ± 0.88 | +3.8 ± 0.81 | −5.4 ± 1.57 | B, readout 0 |
| Seed 22, first mapping, mirrored | 6 | 30.6, 31.4, 31.4, 29.8 | 26.0, 26.4, 26.1, 24.5 | +0.7 ± 0.36 | +0.4 ± 0.54 | +0.4 ± 0.68 | A, readout 1 |
| Seed 22, first mapping, mirrored | 12 | 28.2, 29.3, 28.7, 27.0 | 21.3, 23.1, 24.5, 21.9 | +1.1 ± 0.68 | +1.8 ± 1.42 | −0.6 ± 1.10 | B, readout 0 |
| Seed 22, first mapping, mirrored | 18 | 28.0, 27.6, 28.3, 25.9 | 21.0, 25.1, 24.9, 20.2 | −0.4 ± 1.04 | +4.1 ± 1.42 | −4.5 ± 1.13 | B, readout 0 |
| Seed 22, first mapping, mirrored | 24 | 27.5, 27.6, 27.0, 24.3 | 18.4, 23.0, 23.5, 20.7 | +0.1 ± 1.09 | +4.6 ± 1.47 | −4.5 ± 1.31 | B, readout 0 |
| Seed 22, first mapping, mirrored | 36 | 27.8, 27.4, 26.8, 24.4 | 16.4, 21.8, 22.0, 18.8 | −0.4 ± 0.83 | +5.4 ± 1.47 | −5.8 ± 2.05 | B, readout 0 |
| Seed 23, eighth, assignment | 6 | 24.3, 26.9, 25.3, 24.3 | 23.1, 23.4, 23.5, 22.7 | +2.6 ± 0.75 | +0.4 ± 0.66 | +2.3 ± 1.04 | A, readout 1 |
| Seed 23, eighth, assignment | 12 | 18.3, 21.2, 18.9, 17.3 | 22.6, 22.2, 22.6, 20.7 | +2.9 ± 0.87 | −0.4 ± 0.64 | +3.3 ± 1.21 | A, readout 1 |
| Seed 23, eighth, assignment | 18 | 15.6, 20.8, 18.9, 16.5 | 21.7, 24.4, 20.9, 22.9 | +5.2 ± 0.90 | +2.7 ± 0.71 | +2.6 ± 1.35 | A, readout 1 |
| Seed 23, eighth, assignment | 24 | 16.3, 19.6, 20.4, 15.4 | 21.7, 24.1, 20.0, 22.1 | +3.3 ± 1.29 | +2.5 ± 0.79 | +0.8 ± 1.67 | A, readout 1 |
| Seed 23, eighth, assignment | 36 | 13.0, 15.3, 15.0, 14.3 | 23.2, 25.0, 21.8, 22.1 | +2.3 ± 0.88 | +1.8 ± 0.84 | +0.5 ± 0.92 | A, readout 1 |
| Seed 23, first mapping, mirrored | 6 | 30.4, 31.0, 30.7, 30.1 | 24.6, 25.9, 26.6, 23.6 | +0.6 ± 0.31 | +1.3 ± 0.56 | −0.7 ± 0.61 | B, readout 0 |
| Seed 23, first mapping, mirrored | 12 | 28.5, 29.6, 28.0, 28.0 | 23.8, 24.8, 25.5, 21.5 | +1.1 ± 0.70 | +1.0 ± 1.17 | +0.1 ± 1.46 | A, readout 1 |
| Seed 23, first mapping, mirrored | 18 | 28.5, 27.6, 26.8, 26.0 | 23.5, 24.8, 23.9, 21.3 | −1.0 ± 0.92 | +1.3 ± 0.69 | −2.3 ± 1.12 | B, readout 0 |
| Seed 23, first mapping, mirrored | 24 | 30.0, 25.0, 27.6, 25.4 | 23.5, 25.9, 24.8, 21.1 | −5.0 ± 1.25 | +2.4 ± 0.38 | −7.4 ± 1.33 | B, readout 0 |
| Seed 23, first mapping, mirrored | 36 | 29.3, 24.1, 25.7, 23.8 | 23.9, 24.2, 24.5, 19.8 | −5.2 ± 1.50 | +0.3 ± 0.88 | −5.5 ± 2.21 | B, readout 0 |
| Seed 24, eighth, assignment | 6 | 21.3, 23.8, 21.2, 21.9 | 17.5, 15.3, 15.7, 13.9 | +2.5 ± 0.77 | −2.3 ± 0.71 | +4.8 ± 1.34 | A, readout 1 |
| Seed 24, eighth, assignment | 12 | 18.4, 18.8, 18.8, 20.0 | 16.4, 15.4, 13.7, 14.1 | +0.4 ± 1.31 | −1.0 ± 1.51 | +1.4 ± 2.64 | A, readout 1 |
| Seed 24, eighth, assignment | 18 | 18.0, 14.4, 17.3, 18.7 | 16.5, 14.1, 12.1, 14.7 | −3.6 ± 0.86 | −2.4 ± 1.34 | −1.2 ± 1.65 | B, readout 0 |
| Seed 24, eighth, assignment | 24 | 19.2, 15.3, 16.0, 17.9 | 16.3, 14.3, 14.4, 14.0 | −3.9 ± 0.78 | −2.0 ± 1.34 | −1.9 ± 1.19 | B, readout 0 |
| Seed 24, eighth, assignment | 36 | 12.9, 13.1, 14.3, 17.6 | 12.7, 14.3, 15.3, 12.0 | +0.2 ± 0.76 | +1.6 ± 1.50 | −1.4 ± 1.64 | B, readout 0 |
| Seed 24, first mapping, mirrored | 6 | 28.1, 30.1, 29.4, 26.8 | 26.2, 27.6, 26.2, 25.2 | +2.0 ± 0.50 | +1.4 ± 0.59 | +0.7 ± 0.76 | A, readout 1 |
| Seed 24, first mapping, mirrored | 12 | 24.6, 27.3, 26.9, 22.4 | 21.7, 23.8, 21.6, 21.6 | +2.6 ± 0.61 | +2.1 ± 1.01 | +0.6 ± 0.73 | A, readout 1 |
| Seed 24, first mapping, mirrored | 18 | 21.7, 27.0, 25.0, 20.6 | 22.3, 24.8, 22.3, 19.6 | +5.3 ± 0.46 | +2.5 ± 0.95 | +2.9 ± 1.20 | A, readout 1 |
| Seed 24, first mapping, mirrored | 24 | 22.1, 25.8, 25.3, 19.8 | 22.3, 26.2, 22.9, 19.0 | +3.7 ± 0.86 | +4.0 ± 0.60 | −0.3 ± 0.68 | B, readout 0 |
| Seed 24, first mapping, mirrored | 36 | 22.0, 25.2, 24.1, 17.3 | 20.8, 23.5, 19.4, 18.0 | +3.3 ± 0.67 | +2.7 ± 0.99 | +0.5 ± 0.67 | A, readout 1 |

The side, the six cells of each orientation side by side:

| Cell | Mapping in force | Block 6 | Block 12 | Block 18 | Block 24 | Block 36 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Seed 22, eighth, mirrored | assignment (A to readout 0) | A, R0 (within 2 SE) | A, R0 (within 2 SE) | A, R0 (within 2 SE) | B, R1 | B, R1 (within 2 SE) |
| Seed 22, first mapping, assignment | assignment (A to readout 0) | A, R0 | A, R0 | A, R0 | A, R0 | A, R0 |
| Seed 23, eighth, mirrored | assignment (A to readout 0) | A, R0 | A, R0 | A, R0 | A, R0 | A, R0 |
| Seed 23, first mapping, assignment | assignment (A to readout 0) | A, R0 (within 2 SE) | A, R0 | A, R0 | A, R0 | A, R0 |
| Seed 24, eighth, mirrored | assignment (A to readout 0) | B, R1 (within 2 SE) | B, R1 | B, R1 | B, R1 | B, R1 |
| Seed 24, first mapping, assignment | assignment (A to readout 0) | A, R0 (within 2 SE) | A, R0 (within 2 SE) | A, R0 | A, R0 | A, R0 (within 2 SE) |
| Seed 22, eighth, assignment | mirrored (A to readout 1) | A, R1 | A, R1 | B, R0 | B, R0 (within 2 SE) | B, R0 |
| Seed 22, first mapping, mirrored | mirrored (A to readout 1) | A, R1 (within 2 SE) | B, R0 (within 2 SE) | B, R0 | B, R0 | B, R0 |
| Seed 23, eighth, assignment | mirrored (A to readout 1) | A, R1 | A, R1 | A, R1 (within 2 SE) | A, R1 (within 2 SE) | A, R1 (within 2 SE) |
| Seed 23, first mapping, mirrored | mirrored (A to readout 1) | B, R0 (within 2 SE) | A, R1 (within 2 SE) | B, R0 | B, R0 | B, R0 |
| Seed 24, eighth, assignment | mirrored (A to readout 1) | A, R1 | A, R1 (within 2 SE) | B, R0 (within 2 SE) | B, R0 (within 2 SE) | B, R0 (within 2 SE) |
| Seed 24, first mapping, mirrored | mirrored (A to readout 1) | A, R1 (within 2 SE) | A, R1 (within 2 SE) | A, R1 | B, R0 (within 2 SE) | A, R1 (within 2 SE) |

Counted:

| Block | A answers readout 0: side on A (readout 0) / on B (readout 1) | of them beyond 2 SE | A answers readout 1: side on A (readout 1) / on B (readout 0) | of them beyond 2 SE | The twelve: on A / on B | The twelve: on readout 0 / on readout 1 | Where A's less B's is beyond 2 SE: on readout 0 / on readout 1 |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | 5 / 1 | 2 / 0 | 5 / 1 | 3 / 0 | 10 / 2 | 6 / 6 | 2 / 3 of 5 |
| 12 | 5 / 1 | 3 / 1 | 5 / 1 | 2 / 0 | 10 / 2 | 6 / 6 | 3 / 3 of 6 |
| 18 | 5 / 1 | 4 / 1 | 2 / 4 | 1 / 3 | 7 / 5 | 9 / 3 | 7 / 2 of 9 |
| 24 | 4 / 2 | 4 / 2 | 1 / 5 | 0 / 2 | 5 / 7 | 9 / 3 | 6 / 2 of 8 |
| 36 | 4 / 2 | 3 / 1 | 2 / 4 | 0 / 3 | 6 / 6 | 8 / 4 | 6 / 1 of 7 |

- **At blocks 6 and 12 the side lies on stimulus A in 10 cells of 12**, five of each orientation, so on readout 0 in six and readout 1 in
  six. Where A's less B's is beyond twice its SE, it lies on A in all 5 such cells at block 6 and in 5 of 6 at block 12, the other seed
  24's eighth image from the mirrored assignment.
- **From block 18 the side lies on readout 0 in 9, 9 and 8 cells of 12.** Where A answers readout 0 it lies on A in 5, 4 and 4 of the six;
  where A answers readout 1, on B in 4, 5 and 4. Where A's less B's is beyond twice its SE, 9, 8 and 7 cells, it lies on readout 0 in 7, 6
  and 6.
- **Stimulus or readout, counted.** Only the six cells where A answers readout 1 tell the two apart. There the side goes with the stimulus,
  on A, in 5, 5, 2, 1 and 2 cells at blocks 6, 12, 18, 24 and 36, and with readout 0, on B, in 1, 1, 4, 5 and 4. In the six where A answers
  readout 0 the two agree: the side lies on A and readout 0 in 5, 5, 5, 4 and 4, and on B and readout 1 in 1, 1, 1, 2 and 2.
- **At block 36 the side lies on readout 1 in four cells**: seed 22's and seed 24's eighth images from the mirrored assignment, on B, and
  seed 23's eighth image from the assignment and seed 24's first mapping from the mirrored assignment, on A. Only seed 24's eighth image from
  the mirrored assignment stands beyond twice its SE, B's +9.7 ± 0.98 against A's +1.5 ± 0.74.
- **At block 36, by orientation**: where A answers readout 0, plain nights on A's patterns less nights without replay read +1.5 to +15.9 on
  A's trials and −0.4 to +10.4 on B's; where A answers readout 1, −5.2 to +3.3 on A's and +0.3 to +5.4 on B's. The ranges on A's trials
  overlap from +1.5 to +3.3.
- **Where A answers readout 0, A's trials fall without replay.** From block 6 to block 36 they fell by 7.6 to 20.1 in each of the six cells,
  to 4.7 to 21.2, and under the awake gap by 3.9 to 13.2. Where A answers readout 1 they moved by −11.3 to +0.3 without replay, to 12.9 to
  29.3, and by −10.0 to −3.3 under the awake gap.

## 2. Against the awake gap

Each night's mean less the awake gap's on each stimulus's trials, ± exploration 018's unpaired SE. Where A answers readout 0:

| Cell | Block | Without replay less awake: A's trials (readout 0) | B's trials (readout 1) | Plain on A's less awake: A's trials | B's trials |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | −3.3 ± 1.11 | +0.3 ± 0.72 | +1.0 ± 0.96 | +3.2 ± 0.88 |
| Seed 22, eighth, mirrored | 12 | −5.4 ± 1.14 | +1.5 ± 0.80 | +2.4 ± 0.93 | +5.8 ± 0.93 |
| Seed 22, eighth, mirrored | 18 | −6.1 ± 2.06 | −0.8 ± 1.17 | +2.5 ± 0.78 | +6.1 ± 1.24 |
| Seed 22, eighth, mirrored | 24 | −6.4 ± 1.01 | −3.0 ± 0.92 | −0.4 ± 0.74 | +8.6 ± 1.47 |
| Seed 22, eighth, mirrored | 36 | −10.1 ± 0.91 | −4.2 ± 0.85 | −0.9 ± 0.74 | +6.2 ± 0.89 |
| Seed 22, first mapping, assignment | 6 | −1.2 ± 0.85 | +1.8 ± 0.99 | +2.9 ± 0.83 | +1.8 ± 0.73 |
| Seed 22, first mapping, assignment | 12 | −1.5 ± 1.07 | +4.0 ± 1.05 | +2.3 ± 0.92 | +3.1 ± 1.01 |
| Seed 22, first mapping, assignment | 18 | −6.0 ± 1.38 | +5.0 ± 1.17 | +2.3 ± 1.16 | +3.4 ± 1.31 |
| Seed 22, first mapping, assignment | 24 | −5.0 ± 2.11 | +6.6 ± 0.93 | +3.3 ± 1.10 | +7.4 ± 0.74 |
| Seed 22, first mapping, assignment | 36 | −6.4 ± 1.53 | +6.0 ± 1.34 | +1.9 ± 0.95 | +5.5 ± 0.86 |
| Seed 23, eighth, mirrored | 6 | −2.6 ± 1.19 | −0.5 ± 0.79 | −0.1 ± 1.08 | −1.3 ± 0.84 |
| Seed 23, eighth, mirrored | 12 | −2.4 ± 0.77 | −5.2 ± 1.45 | +3.8 ± 1.17 | −2.4 ± 1.70 |
| Seed 23, eighth, mirrored | 18 | −5.3 ± 0.95 | −5.1 ± 0.72 | +6.8 ± 1.15 | −4.5 ± 0.98 |
| Seed 23, eighth, mirrored | 24 | −7.2 ± 0.87 | −4.0 ± 0.31 | +9.7 ± 1.49 | −2.2 ± 0.18 |
| Seed 23, eighth, mirrored | 36 | −8.2 ± 1.04 | −4.3 ± 0.49 | +7.7 ± 0.99 | −2.8 ± 0.40 |
| Seed 23, first mapping, assignment | 6 | −0.3 ± 0.86 | +0.4 ± 1.05 | +1.4 ± 0.82 | +0.8 ± 1.06 |
| Seed 23, first mapping, assignment | 12 | −1.4 ± 1.23 | −1.5 ± 0.82 | +4.6 ± 1.04 | −3.3 ± 0.89 |
| Seed 23, first mapping, assignment | 18 | −6.6 ± 1.82 | +0.2 ± 1.30 | +4.0 ± 1.00 | −2.6 ± 0.86 |
| Seed 23, first mapping, assignment | 24 | −7.8 ± 1.79 | +1.1 ± 0.60 | +5.4 ± 0.99 | −2.1 ± 0.63 |
| Seed 23, first mapping, assignment | 36 | −9.9 ± 1.48 | −1.6 ± 0.71 | +3.6 ± 0.78 | −1.8 ± 0.45 |
| Seed 24, eighth, mirrored | 6 | +3.7 ± 0.89 | +1.5 ± 1.21 | +1.6 ± 1.01 | +1.0 ± 1.03 |
| Seed 24, eighth, mirrored | 12 | +3.3 ± 0.99 | +2.0 ± 0.95 | +0.1 ± 0.97 | +2.3 ± 0.93 |
| Seed 24, eighth, mirrored | 18 | −2.7 ± 0.59 | −3.7 ± 1.14 | −2.0 ± 0.50 | +3.9 ± 2.31 |
| Seed 24, eighth, mirrored | 24 | −3.2 ± 1.20 | −4.2 ± 0.65 | −1.6 ± 0.86 | +3.6 ± 1.30 |
| Seed 24, eighth, mirrored | 36 | −4.0 ± 0.79 | −5.8 ± 0.96 | −2.5 ± 0.84 | +4.0 ± 0.68 |
| Seed 24, first mapping, assignment | 6 | +1.3 ± 0.90 | +1.2 ± 0.97 | +3.1 ± 0.93 | +2.3 ± 0.91 |
| Seed 24, first mapping, assignment | 12 | +3.3 ± 1.18 | +4.4 ± 1.29 | +5.5 ± 1.07 | +5.0 ± 0.92 |
| Seed 24, first mapping, assignment | 18 | +2.4 ± 1.54 | +5.1 ± 1.28 | +6.4 ± 1.30 | +5.0 ± 2.38 |
| Seed 24, first mapping, assignment | 24 | +0.7 ± 1.28 | +6.6 ± 0.92 | +6.8 ± 1.16 | +9.2 ± 2.42 |
| Seed 24, first mapping, assignment | 36 | +0.5 ± 1.54 | +4.4 ± 2.45 | +3.7 ± 1.41 | +4.9 ± 1.40 |

Where A answers readout 1:

| Cell | Block | Without replay less awake: A's trials (readout 1) | B's trials (readout 0) | Plain on A's less awake: A's trials | B's trials |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, assignment | 6 | +0.3 ± 0.65 | +1.2 ± 1.21 | +2.3 ± 0.68 | +1.1 ± 1.16 |
| Seed 22, eighth, assignment | 12 | +0.8 ± 1.26 | +1.4 ± 0.98 | +3.3 ± 1.31 | +1.9 ± 1.04 |
| Seed 22, eighth, assignment | 18 | +3.4 ± 1.44 | +0.3 ± 0.87 | +2.8 ± 1.95 | +4.3 ± 0.67 |
| Seed 22, eighth, assignment | 24 | +4.7 ± 1.18 | −1.1 ± 1.11 | +4.4 ± 1.55 | +2.0 ± 1.28 |
| Seed 22, eighth, assignment | 36 | +3.9 ± 1.09 | −2.7 ± 1.13 | +2.3 ± 1.24 | +1.1 ± 1.43 |
| Seed 22, first mapping, mirrored | 6 | +0.8 ± 0.94 | +1.5 ± 0.99 | +1.5 ± 0.86 | +1.9 ± 0.76 |
| Seed 22, first mapping, mirrored | 12 | +1.2 ± 1.07 | −0.6 ± 1.73 | +2.4 ± 1.06 | +1.2 ± 0.91 |
| Seed 22, first mapping, mirrored | 18 | +2.1 ± 0.88 | +0.8 ± 1.93 | +1.7 ± 1.18 | +4.9 ± 0.97 |
| Seed 22, first mapping, mirrored | 24 | +3.2 ± 1.18 | −2.3 ± 1.64 | +3.3 ± 0.76 | +2.3 ± 1.19 |
| Seed 22, first mapping, mirrored | 36 | +3.3 ± 0.73 | −2.4 ± 1.66 | +2.9 ± 0.99 | +3.0 ± 0.97 |
| Seed 23, eighth, assignment | 6 | 0.0 ± 1.01 | +0.4 ± 0.99 | +2.6 ± 1.18 | +0.7 ± 1.08 |
| Seed 23, eighth, assignment | 12 | +1.0 ± 0.76 | +1.9 ± 0.99 | +3.9 ± 1.21 | +1.5 ± 0.91 |
| Seed 23, eighth, assignment | 18 | −0.9 ± 0.97 | −1.2 ± 1.45 | +4.4 ± 0.94 | +1.5 ± 1.78 |
| Seed 23, eighth, assignment | 24 | +0.9 ± 1.22 | −0.4 ± 1.87 | +4.2 ± 1.12 | +2.1 ± 2.17 |
| Seed 23, eighth, assignment | 36 | −1.3 ± 1.04 | +1.1 ± 2.15 | +1.0 ± 0.99 | +2.9 ± 1.38 |
| Seed 23, first mapping, mirrored | 6 | +0.4 ± 1.03 | +1.0 ± 1.06 | +1.0 ± 1.03 | +2.3 ± 1.11 |
| Seed 23, first mapping, mirrored | 12 | +0.5 ± 1.33 | +2.4 ± 1.38 | +1.6 ± 1.09 | +3.3 ± 1.07 |
| Seed 23, first mapping, mirrored | 18 | +2.6 ± 0.81 | +2.2 ± 1.10 | +1.6 ± 1.22 | +3.5 ± 1.37 |
| Seed 23, first mapping, mirrored | 24 | +4.7 ± 1.10 | +2.4 ± 1.16 | −0.3 ± 1.14 | +4.8 ± 1.26 |
| Seed 23, first mapping, mirrored | 36 | +5.4 ± 1.21 | +4.1 ± 1.58 | +0.2 ± 1.01 | +4.4 ± 1.20 |
| Seed 24, eighth, assignment | 6 | −0.6 ± 0.90 | +3.6 ± 1.14 | +2.0 ± 0.90 | +1.3 ± 0.86 |
| Seed 24, eighth, assignment | 12 | −1.5 ± 1.04 | +2.3 ± 1.34 | −1.1 ± 1.69 | +1.3 ± 1.58 |
| Seed 24, eighth, assignment | 18 | −0.7 ± 1.52 | +1.7 ± 0.93 | −4.3 ± 1.47 | −0.6 ± 1.31 |
| Seed 24, eighth, assignment | 24 | +1.3 ± 1.26 | +2.3 ± 1.41 | −2.6 ± 1.12 | +0.3 ± 0.55 |
| Seed 24, eighth, assignment | 36 | −4.7 ± 1.11 | +0.6 ± 1.37 | −4.5 ± 1.34 | +2.3 ± 1.19 |
| Seed 24, first mapping, mirrored | 6 | +1.3 ± 1.10 | +1.0 ± 1.01 | +3.3 ± 1.06 | +2.4 ± 0.96 |
| Seed 24, first mapping, mirrored | 12 | +2.2 ± 1.13 | +0.1 ± 1.20 | +4.8 ± 1.02 | +2.1 ± 1.18 |
| Seed 24, first mapping, mirrored | 18 | +1.1 ± 0.90 | +2.7 ± 1.75 | +6.5 ± 1.04 | +5.2 ± 1.24 |
| Seed 24, first mapping, mirrored | 24 | +2.3 ± 1.40 | +3.3 ± 1.42 | +6.0 ± 1.05 | +7.2 ± 1.23 |
| Seed 24, first mapping, mirrored | 36 | +4.7 ± 1.15 | +2.8 ± 1.39 | +7.9 ± 0.74 | +5.5 ± 0.76 |

Counted, the cells above, within and below twice the SE, on the unrounded figures. Three cells print on the edge and read the other way
from their printed figures: seed 24's first mapping from the mirrored assignment, B's trials without replay at block 36, +2.8 ± 1.39, is
within (+2.750 against twice 1.394); seed 22's first mapping from the assignment, A's trials under plain nights at block 36, +1.9 ± 0.95,
and seed 23's first mapping from the mirrored assignment, B's trials without replay at block 18, +2.2 ± 1.10, are above.

| Difference | Block | A answers readout 0: A's trials (readout 0), cells above, within, below 2 SE | B's trials (readout 1) | A answers readout 1: A's trials (readout 1) | B's trials (readout 0) |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Without replay less awake | 6 | 1, 3, 2 | 0, 6, 0 | 0, 6, 0 | 1, 5, 0 |
| Without replay less awake | 12 | 2, 2, 2 | 3, 2, 1 | 0, 6, 0 | 0, 6, 0 |
| Without replay less awake | 18 | 0, 1, 5 | 2, 2, 2 | 3, 3, 0 | 1, 5, 0 |
| Without replay less awake | 24 | 0, 1, 5 | 2, 1, 3 | 3, 3, 0 | 2, 4, 0 |
| Without replay less awake | 36 | 0, 1, 5 | 1, 1, 4 | 4, 1, 1 | 1, 4, 1 |
| Plain on A's less awake | 6 | 2, 4, 0 | 3, 3, 0 | 4, 2, 0 | 3, 3, 0 |
| Plain on A's less awake | 12 | 5, 1, 0 | 4, 1, 1 | 4, 2, 0 | 1, 5, 0 |
| Plain on A's less awake | 18 | 4, 1, 1 | 3, 1, 2 | 2, 3, 1 | 4, 2, 0 |
| Plain on A's less awake | 24 | 4, 2, 0 | 4, 0, 2 | 4, 1, 1 | 2, 4, 0 |
| Plain on A's less awake | 36 | 4, 1, 1 | 4, 0, 2 | 2, 3, 1 | 4, 2, 0 |

- **Where A answers readout 0, a night without replay costs A's trials against the awake gap.** From block 18 A's trials, on readout 0,
  stand below it by more than twice the SE in 5 cells of the six, at −6.6 to +2.4, −7.8 to +0.7 and −10.1 to +0.5 at blocks 18, 24 and 36.
  B's trials, on readout 1, stand below in 2, 3 and 4 and above in 2, 2 and 1.
- **Where A answers readout 1, a night without replay costs neither stimulus's trials as often.** A's trials, on readout 1, stand above
  the awake gap in 3, 3 and 4 cells from block 18 and below in 0, 0 and 1, seed 24's eighth image from the assignment at block 36, −4.7 ±
  1.11. B's trials, on readout 0, stand within it in 5, 4 and 4, and below in one, at block 36.
- **At block 36 a plain night on A's patterns stands above the awake gap more often than below it**, in both orientations and on both
  stimuli. Where A answers readout 0, A's trials stand above in 4 cells, within in 1 and below in 1, and B's in 4, 0 and 2; where A answers
  readout 1, A's in 2, 3 and 1, and B's in 4, 2 and 0.
- **At blocks 6 and 12 a night without replay stands within twice the SE of the awake gap** in 20 and 16 of the 24 readings, two
  stimuli in each of twelve cells.

## 3. A's and B's roles exchanged

Plain on A's less without replay on the trials that presented each set of units, H-47's and exchanged, ± their paired SEs, and the side
named by its units, the role they play and the readout they answer:

| Cell | Block | H-47: plain less without replay on H-47's A's units' trials | on H-47's B's units' trials | H-47: the side | Exchanged: on H-47's A's units' trials (now B's) | on H-47's B's units' trials (now A's) | Exchanged: the side |
| :--- | ---: | ---: | ---: | :--- | ---: | ---: | :--- |
| Seed 23, first mapping, mirrored | 6 | +0.6 ± 0.31 | +1.3 ± 0.56 | H-47's B's units, as B, readout 0 (within 2 SE) | +2.4 ± 0.77 | +2.2 ± 0.66 | H-47's A's units, as B, readout 1 (within 2 SE) |
| Seed 23, first mapping, mirrored | 12 | +1.1 ± 0.70 | +1.0 ± 1.17 | H-47's A's units, as A, readout 1 (within 2 SE) | +0.7 ± 1.87 | +2.7 ± 1.33 | H-47's B's units, as A, readout 0 (within 2 SE) |
| Seed 23, first mapping, mirrored | 18 | −1.0 ± 0.92 | +1.3 ± 0.69 | H-47's B's units, as B, readout 0 | −3.0 ± 2.08 | +2.8 ± 1.37 | H-47's B's units, as A, readout 0 |
| Seed 23, first mapping, mirrored | 24 | −5.0 ± 1.25 | +2.4 ± 0.38 | H-47's B's units, as B, readout 0 | −6.6 ± 1.33 | +3.0 ± 1.50 | H-47's B's units, as A, readout 0 |
| Seed 23, first mapping, mirrored | 36 | −5.2 ± 1.50 | +0.3 ± 0.88 | H-47's B's units, as B, readout 0 | −6.6 ± 1.09 | +1.9 ± 1.19 | H-47's B's units, as A, readout 0 |
| Seed 23, first mapping, assignment | 6 | +1.6 ± 0.46 | +0.4 ± 0.57 | H-47's A's units, as A, readout 0 (within 2 SE) | +2.5 ± 0.53 | +0.3 ± 0.57 | H-47's A's units, as B, readout 0 |
| Seed 23, first mapping, assignment | 12 | +6.0 ± 0.87 | −1.8 ± 0.85 | H-47's A's units, as A, readout 0 | +5.4 ± 0.63 | −4.1 ± 1.05 | H-47's A's units, as B, readout 0 |
| Seed 23, first mapping, assignment | 18 | +10.6 ± 1.65 | −2.8 ± 1.24 | H-47's A's units, as A, readout 0 | +7.9 ± 0.58 | −3.1 ± 0.78 | H-47's A's units, as B, readout 0 |
| Seed 23, first mapping, assignment | 24 | +13.2 ± 1.45 | −3.2 ± 0.67 | H-47's A's units, as A, readout 0 | +9.2 ± 0.65 | −2.4 ± 0.60 | H-47's A's units, as B, readout 0 |
| Seed 23, first mapping, assignment | 36 | +13.5 ± 1.09 | −0.1 ± 0.62 | H-47's A's units, as A, readout 0 | +6.7 ± 0.83 | −0.1 ± 0.65 | H-47's A's units, as B, readout 0 |
| Seed 24, eighth, mirrored | 6 | −2.1 ± 0.99 | −0.5 ± 1.17 | H-47's B's units, as B, readout 1 (within 2 SE) | −1.4 ± 0.38 | −0.1 ± 0.64 | H-47's B's units, as A, readout 1 |
| Seed 24, eighth, mirrored | 12 | −3.2 ± 0.72 | +0.3 ± 0.77 | H-47's B's units, as B, readout 1 | +1.0 ± 0.70 | +0.9 ± 0.85 | H-47's A's units, as B, readout 0 (within 2 SE) |
| Seed 24, eighth, mirrored | 18 | +0.7 ± 0.25 | +7.6 ± 1.52 | H-47's B's units, as B, readout 1 | +1.8 ± 0.73 | +4.9 ± 0.56 | H-47's B's units, as A, readout 1 |
| Seed 24, eighth, mirrored | 24 | +1.6 ± 1.33 | +7.8 ± 1.47 | H-47's B's units, as B, readout 1 | +5.8 ± 1.32 | +6.1 ± 0.78 | H-47's B's units, as A, readout 1 (within 2 SE) |
| Seed 24, eighth, mirrored | 36 | +1.5 ± 0.74 | +9.7 ± 0.98 | H-47's B's units, as B, readout 1 | +3.9 ± 1.62 | +6.9 ± 0.85 | H-47's B's units, as A, readout 1 (within 2 SE) |
| Seed 22, eighth, assignment | 6 | +2.0 ± 0.46 | −0.1 ± 0.58 | H-47's A's units, as A, readout 1 | −0.5 ± 0.65 | +0.9 ± 0.47 | H-47's B's units, as A, readout 0 (within 2 SE) |
| Seed 22, eighth, assignment | 12 | +2.5 ± 0.45 | +0.5 ± 0.81 | H-47's A's units, as A, readout 1 | +0.1 ± 1.04 | +1.1 ± 0.55 | H-47's B's units, as A, readout 0 (within 2 SE) |
| Seed 22, eighth, assignment | 18 | −0.6 ± 0.88 | +4.0 ± 0.58 | H-47's B's units, as B, readout 0 | +0.2 ± 1.04 | +3.3 ± 1.06 | H-47's B's units, as A, readout 0 (within 2 SE) |
| Seed 22, eighth, assignment | 24 | −0.3 ± 0.99 | +3.1 ± 1.17 | H-47's B's units, as B, readout 0 (within 2 SE) | −1.5 ± 1.02 | +4.5 ± 1.19 | H-47's B's units, as A, readout 0 |
| Seed 22, eighth, assignment | 36 | −1.6 ± 0.88 | +3.8 ± 0.81 | H-47's B's units, as B, readout 0 | −1.7 ± 0.32 | +8.0 ± 0.11 | H-47's B's units, as A, readout 0 |

Counted, the exchanged side against H-47's:

| Block | On the same units and readout, the other role (beyond 2 SE in both runs) | On the other units, the same role (beyond 2 SE in both) | Cells |
| ---: | ---: | ---: | ---: |
| 6 | 2 (0) | 2 (0) | 4 |
| 12 | 1 (1) | 3 (0) | 4 |
| 18 | 4 (3) | 0 (0) | 4 |
| 24 | 4 (2) | 0 (0) | 4 |
| 36 | 4 (3) | 0 (0) | 4 |

- **From block 18 the side stays on the same units, and so on the same readout, in all four cells.** Exchanged, those units play the other
  role, so the side moves from B to A in three cells and from A to B in one. A's less B's stands beyond twice its SE in both runs in 3, 2
  and 3 of the four at blocks 18, 24 and 36.
- **At blocks 6 and 12 it moves with the role in 2 and 3 cells of the four**, and stays on the same units in 2 and 1. None of the cells
  where it moves with the role stands beyond twice its SE in both runs.
- **At block 36 the side's units gain in either role**: +0.3 ± 0.88 in H-47 and +1.9 ± 1.19 exchanged on seed 23's first mapping
  from the mirrored assignment, +13.5 ± 1.09 and +6.7 ± 0.83 from the assignment, +9.7 ± 0.98 and +6.9 ± 0.85 on seed 24, and
  +3.8 ± 0.81 and +8.0 ± 0.11 on seed 22. The other units read −5.2 ± 1.50 and −6.6 ± 1.09, −0.1 ± 0.62 and −0.1 ± 0.65, +1.5 ± 0.74
  and +3.9 ± 1.62, and −1.6 ± 0.88 and −1.7 ± 0.32.

A's count, both stimuli, under each of the two forms and plain less without replay, H-47's and exchanged:

| Cell | Block | H-47: A's count without replay, plain on A's | plain less without replay | Exchanged: the same | plain less without replay |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | 55.0, 56.9 | +1.9 ± 0.68 | 51.1, 55.8 | +4.7 ± 1.11 |
| Seed 23, first mapping, mirrored | 12 | 52.4, 54.4 | +2.0 ± 1.26 | 42.8, 46.1 | +3.4 ± 2.48 |
| Seed 23, first mapping, mirrored | 18 | 52.0, 52.4 | +0.3 ± 1.17 | 44.3, 44.1 | −0.2 ± 2.74 |
| Seed 23, first mapping, mirrored | 24 | 53.5, 50.9 | −2.6 ± 1.30 | 46.4, 42.9 | −3.5 ± 2.74 |
| Seed 23, first mapping, mirrored | 36 | 53.1, 48.3 | −4.9 ± 1.09 | 43.0, 38.3 | −4.7 ± 2.26 |
| Seed 23, first mapping, assignment | 6 | 54.2, 56.3 | +2.0 ± 0.79 | 52.5, 55.3 | +2.8 ± 0.82 |
| Seed 23, first mapping, assignment | 12 | 33.4, 37.6 | +4.1 ± 1.18 | 39.7, 41.0 | +1.3 ± 1.22 |
| Seed 23, first mapping, assignment | 18 | 27.2, 35.0 | +7.9 ± 2.54 | 38.7, 43.4 | +4.7 ± 1.04 |
| Seed 23, first mapping, assignment | 24 | 24.8, 34.8 | +10.0 ± 1.90 | 36.3, 43.2 | +6.8 ± 0.77 |
| Seed 23, first mapping, assignment | 36 | 19.7, 33.0 | +13.4 ± 1.56 | 34.3, 40.9 | +6.6 ± 1.25 |
| Seed 24, eighth, mirrored | 6 | 36.4, 33.8 | −2.6 ± 1.35 | 43.6, 42.1 | −1.5 ± 0.93 |
| Seed 24, eighth, mirrored | 12 | 31.5, 28.6 | −2.9 ± 1.19 | 39.2, 41.1 | +1.9 ± 1.14 |
| Seed 24, eighth, mirrored | 18 | 20.2, 28.6 | +8.3 ± 1.70 | 32.0, 38.6 | +6.6 ± 1.04 |
| Seed 24, eighth, mirrored | 24 | 16.4, 25.8 | +9.4 ± 1.58 | 24.4, 36.4 | +12.0 ± 1.16 |
| Seed 24, eighth, mirrored | 36 | 12.5, 23.8 | +11.3 ± 0.96 | 23.6, 34.4 | +10.8 ± 1.47 |
| Seed 22, eighth, assignment | 6 | 50.3, 52.3 | +1.9 ± 0.65 | 43.0, 43.4 | +0.4 ± 0.49 |
| Seed 22, eighth, assignment | 12 | 47.6, 50.6 | +3.0 ± 0.93 | 43.9, 45.0 | +1.1 ± 0.90 |
| Seed 22, eighth, assignment | 18 | 47.2, 50.6 | +3.4 ± 1.05 | 41.5, 45.1 | +3.5 ± 1.18 |
| Seed 22, eighth, assignment | 24 | 47.3, 50.2 | +2.8 ± 0.89 | 36.3, 39.2 | +3.0 ± 1.69 |
| Seed 22, eighth, assignment | 36 | 44.2, 46.4 | +2.2 ± 0.65 | 32.6, 38.8 | +6.2 ± 0.23 |

- **The exchanged runs read A differently from H-47's from block 6 on**, under both forms: at block 36, without replay, 43.0 against 53.1
  on seed 23 from the mirrored assignment, 34.3 against 19.7 from the assignment, 23.6 against 12.5 on seed 24 and 32.6 against 44.2 on
  seed 22.
- **Plain less without replay at block 36 keeps its sign in all four**: −4.7 ± 2.26 against H-47's −4.9 ± 1.09, +6.6 ± 1.25 against +13.4
  ± 1.56, +10.8 ± 1.47 against +11.3 ± 0.96, and +6.2 ± 0.23 against +2.2 ± 0.65.

## 4. Where the pair's own units lie

Each cell's start split, the readout each stimulus answers, the side at block 36, and what four plain nights on A's patterns moved of the
pair's own units' weight onto each readout's excitatory units, the mean over the three paths:

| Cell | A answers | C's own lie in | C answers | D's own lie in | D answers | Splits asserted | Side at block 36 | Plain nights: C's own onto readout 0's excitatory, M | onto readout 1's | D's own onto readout 0's | onto readout 1's |
| :--- | :--- | :--- | :--- | :--- | :--- | ---: | :--- | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0 | readout 0 (25 units) | readout 1 | readout 1 (25 units) | readout 0 | 55 | B, readout 1 | +0.18 | −0.17 | −0.22 | +0.30 |
| Seed 22, first mapping, assignment | readout 0 | readout 0 (25 units) | readout 1 | readout 1 (25 units) | readout 0 | 55 | A, readout 0 | +0.22 | −0.23 | −0.20 | +0.23 |
| Seed 23, eighth, mirrored | readout 0 | readout 0 (25 units) | readout 1 | readout 1 (25 units) | readout 0 | 55 | A, readout 0 | +0.16 | −0.20 | −0.19 | +0.16 |
| Seed 23, first mapping, assignment | readout 0 | readout 0 (25 units) | readout 1 | readout 1 (25 units) | readout 0 | 55 | A, readout 0 | +0.20 | −0.26 | −0.25 | +0.18 |
| Seed 24, eighth, mirrored | readout 0 | readout 0 (25 units) | readout 1 | readout 1 (25 units) | readout 0 | 55 | B, readout 1 | +0.29 | −0.30 | −0.15 | +0.13 |
| Seed 24, first mapping, assignment | readout 0 | readout 0 (25 units) | readout 1 | readout 1 (25 units) | readout 0 | 55 | A, readout 0 | +0.37 | −0.36 | −0.10 | +0.10 |
| Seed 22, eighth, assignment | readout 1 | readout 0 (25 units) | readout 0 | readout 1 (25 units) | readout 1 | 55 | B, readout 0 | −0.06 | +0.09 | 0.00 | +0.07 |
| Seed 22, first mapping, mirrored | readout 1 | readout 0 (25 units) | readout 0 | readout 1 (25 units) | readout 1 | 55 | B, readout 0 | −0.02 | +0.03 | −0.01 | +0.06 |
| Seed 23, eighth, assignment | readout 1 | readout 0 (25 units) | readout 0 | readout 1 (25 units) | readout 1 | 55 | A, readout 1 | +0.19 | −0.16 | −0.03 | +0.08 |
| Seed 23, first mapping, mirrored | readout 1 | readout 0 (25 units) | readout 0 | readout 1 (25 units) | readout 1 | 55 | B, readout 0 | +0.20 | −0.16 | −0.04 | +0.08 |
| Seed 24, eighth, assignment | readout 1 | readout 0 (25 units) | readout 0 | readout 1 (25 units) | readout 1 | 55 | B, readout 0 | +0.07 | +0.03 | +0.02 | −0.04 |
| Seed 24, first mapping, mirrored | readout 1 | readout 0 (25 units) | readout 0 | readout 1 (25 units) | readout 1 | 55 | A, readout 1 | +0.11 | −0.02 | −0.03 | −0.11 |

- **C's own units lie inside readout 0 and D's inside readout 1 in all twelve cells**, 25 units each. Every split of every image held it,
  55 a cell: the job asserts that the pair's own units lie in A's readout before it splits.
- **Where they lie against the pair's answers goes with the orientation.** Where A answers readout 0, C answers readout 1 and D readout 0,
  so each pair's own units lie in the readout their stimulus does not answer: C's in A's answer, D's in B's. Where A answers readout 1, C
  answers readout 0 and D readout 1, so each lies in its own answer: C's in B's answer, D's in A's.
- **Against the side**: since C's own lie in readout 0 in every cell, the side lies on the readout that holds C's own units in 6, 6, 9, 9
  and 8 cells at blocks 6, 12, 18, 24 and 36, and on the one that holds D's own in 6, 6, 3, 3 and 4. The exchange of question 3 keeps C's own
  at place 5, inside readout 0, and moves which shared units go with them; from block 18 its side stayed on readout 0 in the three cells
  where H-47's lay there.
- **Plain nights move the pair's own units' weight toward the readout they lie in, in all six cells where A answers readout 0**: C's own
  onto readout 0's excitatory units +0.16 to +0.37 M and onto readout 1's −0.36 to −0.17 M, D's own onto readout 1's +0.10 to +0.30 M and
  onto readout 0's −0.25 to −0.10 M. Where A answers readout 1 they move less and not one way: C's own −0.06 to +0.20 M onto readout 0's
  and −0.16 to +0.09 M onto readout 1's, D's own −0.04 to +0.02 M and −0.11 to +0.08 M.

## 5. What plain nights take from each readout's own synapses

What each night moved, summed over the four nights, the mean over the three paths, from the class matrix in the images before and after
each gap. A readout's excitatory units are exploration 020's classes inside it: its pattern targets, its other excitatory units, and the
pair's own units there. 020's figure for the readouts' excitatory weight left the pair's own out, and that is the whole difference from
it: over four plain nights C's own moved by at most 0.10 M and D's by at most 0.13 M, both together by at most 0.23 M. "Onto its own" is
onto those same units; a readout's inhibitory units are those inside its set; "inhibitory onto a readout's excitatory units" counts every
inhibitory unit:

| Cell | Form | Side at block 36 | Readout 0's excitatory units onto all, M | Readout 1's | Readout 0's onto its own | Readout 1's onto its own | Readout 0's inhibitory onto all | Readout 1's | Inhibitory onto readout 0's excitatory | onto readout 1's |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | no-replay | B, readout 1 | +0.30 | −0.58 | −0.49 | −0.68 | +0.48 | +0.83 | +1.10 | +0.70 |
| Seed 22, eighth, mirrored | plain-first | B, readout 1 | −9.27 | −10.38 | −0.47 | −0.62 | +15.11 | +15.86 | +14.23 | +12.21 |
| Seed 22, first mapping, assignment | no-replay | A, readout 0 | +0.49 | −0.21 | −0.50 | −0.70 | −2.93 | −2.45 | −1.89 | −2.20 |
| Seed 22, first mapping, assignment | plain-first | A, readout 0 | −8.72 | −9.47 | −0.51 | −0.54 | +10.51 | +11.17 | +9.29 | +7.95 |
| Seed 23, eighth, mirrored | no-replay | A, readout 0 | +1.49 | +0.31 | +0.17 | −0.19 | +1.02 | +0.78 | +1.13 | +0.88 |
| Seed 23, eighth, mirrored | plain-first | A, readout 0 | −10.88 | −10.32 | −0.40 | −1.01 | +16.71 | +15.29 | +14.40 | +12.88 |
| Seed 23, first mapping, assignment | no-replay | A, readout 0 | +1.47 | +0.53 | +0.01 | −0.32 | −2.58 | −2.56 | −1.65 | −1.99 |
| Seed 23, first mapping, assignment | plain-first | A, readout 0 | −8.82 | −8.61 | +0.18 | −0.62 | +11.61 | +10.61 | +9.88 | +8.08 |
| Seed 24, eighth, mirrored | no-replay | B, readout 1 | +0.73 | −0.27 | −0.41 | −0.80 | +0.66 | +0.75 | +0.93 | +0.94 |
| Seed 24, eighth, mirrored | plain-first | B, readout 1 | −8.96 | −11.10 | −0.61 | −1.25 | +15.12 | +14.63 | +14.61 | +11.20 |
| Seed 24, first mapping, assignment | no-replay | A, readout 0 | +1.33 | +0.90 | −0.41 | −0.50 | −2.67 | −2.49 | −1.73 | −1.86 |
| Seed 24, first mapping, assignment | plain-first | A, readout 0 | −7.54 | −9.51 | −0.26 | −1.07 | +10.52 | +10.53 | +10.49 | +6.77 |
| Seed 22, eighth, assignment | no-replay | B, readout 0 | −0.13 | −0.50 | −0.51 | −0.81 | +0.27 | +0.76 | +0.89 | +0.60 |
| Seed 22, eighth, assignment | plain-first | B, readout 0 | −10.68 | −12.13 | −2.55 | −2.72 | +12.47 | +13.21 | +10.61 | +11.30 |
| Seed 22, first mapping, mirrored | no-replay | B, readout 0 | +0.52 | −0.20 | −0.34 | −0.80 | −2.93 | −2.57 | −1.92 | −2.23 |
| Seed 22, first mapping, mirrored | plain-first | B, readout 0 | −9.20 | −10.78 | −2.41 | −2.24 | +8.69 | +8.86 | +6.52 | +7.22 |
| Seed 23, eighth, assignment | no-replay | A, readout 1 | +1.39 | +0.09 | +0.46 | −0.21 | +0.92 | +0.92 | +1.17 | +1.02 |
| Seed 23, eighth, assignment | plain-first | A, readout 1 | −12.01 | −11.01 | −2.19 | −2.10 | +15.52 | +14.06 | +12.07 | +13.48 |
| Seed 23, first mapping, mirrored | no-replay | B, readout 0 | +1.48 | +0.14 | +0.18 | −0.28 | −2.70 | −2.65 | −1.79 | −2.04 |
| Seed 23, first mapping, mirrored | plain-first | B, readout 0 | −10.47 | −9.72 | −1.82 | −1.58 | +11.31 | +9.76 | +8.33 | +8.77 |
| Seed 24, eighth, assignment | no-replay | B, readout 0 | +0.68 | −0.12 | −0.53 | −0.70 | +0.61 | +0.72 | +0.98 | +0.78 |
| Seed 24, eighth, assignment | plain-first | B, readout 0 | −11.23 | −12.33 | −1.78 | −2.75 | +13.53 | +13.52 | +11.38 | +11.76 |
| Seed 24, first mapping, mirrored | no-replay | A, readout 1 | +1.29 | +0.78 | −0.38 | −0.58 | −2.70 | −2.56 | −1.77 | −1.92 |
| Seed 24, first mapping, mirrored | plain-first | A, readout 1 | −9.87 | −10.72 | −1.48 | −2.50 | +9.78 | +9.34 | +7.72 | +7.83 |

By orientation, the four plain nights' range over the six cells of each readout, and which readout they move more, cell by cell:

| A answers | Four plain nights on A's patterns | Readout 0, M | Readout 1, M | Cells where readout 0 / readout 1 moves more | Cells where that readout is the side's at block 36 |
| :--- | :--- | ---: | ---: | ---: | ---: |
| readout 0 | Readout's excitatory units onto all | −10.88 to −7.54 | −11.10 to −8.61 | 2 / 4 | 4 of 6 |
| readout 0 | Readout's excitatory units onto its own | −0.61 to +0.18 | −1.25 to −0.54 | 0 / 6 | 2 of 6 |
| readout 0 | Readout's inhibitory units onto all | +10.51 to +16.71 | +10.53 to +15.86 | 3 / 3 | 3 of 6 |
| readout 0 | Inhibitory onto the readout's excitatory units | +9.29 to +14.61 | +6.77 to +12.88 | 6 / 0 | 4 of 6 |
| readout 1 | Readout's excitatory units onto all | −12.01 to −9.20 | −12.33 to −9.72 | 2 / 4 | 2 of 6 |
| readout 1 | Readout's excitatory units onto its own | −2.55 to −1.48 | −2.75 to −1.58 | 3 / 3 | 3 of 6 |
| readout 1 | Readout's inhibitory units onto all | +8.69 to +15.52 | +8.86 to +14.06 | 4 / 2 | 2 of 6 |
| readout 1 | Inhibitory onto the readout's excitatory units | +6.52 to +12.07 | +7.22 to +13.48 | 0 / 6 | 2 of 6 |

- **Plain nights add more inhibition onto the excitatory units of stimulus A's readout, in all twelve cells**: under plain nights alone by
  1.35 to 3.71 M more than onto the other readout's where A answers readout 0, and by 0.10 to 1.41 M more where A answers readout 1. A night
  without replay moved the inhibitory weight onto each readout's excitatory units by −2.23 to +1.17 M.
- **Within each readout, plain nights take more where A answers readout 1.** There each readout's excitatory units onto its own lost 1.48
  to 2.55 M (readout 0) and 1.58 to 2.75 M (readout 1). Where A answers readout 0 they moved by −0.61 to +0.18 M and −1.25 to −0.54 M, and
  readout 0 lost less than readout 1 in all six. The two readouts together lost 3.40 to 5.27 M against 0.44 to 1.87 M; the ranges do not
  overlap.
- **Onto everything the two orientations overlap.** Each readout's excitatory units lost 7.54 to 11.10 M where A answers readout 0 and
  9.20 to 12.33 M where it answers readout 1, the two readouts together 17.05 to 21.20 M and 19.98 to 23.55 M. Readout 0 lost more than
  readout 1 in 2 cells of each six. Each readout's inhibitory units gained 8.69 to 16.71 M, readout 0's more in 3 of the six where A answers
  readout 0 and in 4 of the six where it answers readout 1.
- **Against the side at block 36**: the readout that gains more inhibition onto its excitatory units, A's in every cell, is the side's
  readout in 4 of the six where A answers readout 0 and in 2 of the six where A answers readout 1, 6 of 12, where the side lies on A.

## What it costs

The trial took 707 s. The four exchanged cells took 3 810 to 4 066 s and the eight of exploration 020's job 3 900 to 4 110 s, at most
28.5 % of a job's 240 minutes; H-47's jobs of the same two forms took 3 819 to 4 049 s. Nothing was cut: questions 1 and 2 read all twelve
cells from H-47's logs, question 3 ran four cells of its four, and questions 4 and 5 ran the eight cells exploration 020 had not, which with
its four make all twelve. Thirteen jobs ran, every one to the end.

## What it suggests

**The side moves with the checkpoint.** At blocks 6 and 12 plain nights gained more on stimulus A's trials in ten cells of twelve, in
both orientations. From block 18 the gain lay on readout 0's stimulus in 9, 9 and 8 cells, A where A answers readout 0 and B where it
answers readout 1.

**From block 18, within one start, the side stayed with the units and their readout, not the role.** In four cells of four the exchange
moved A's and B's roles and, from block 18, left the side on the same units; at blocks 6 and 12 it moved with the role in 2 and 3 cells,
none beyond twice the SE in both runs. That exchange keeps each set of units' answer and keeps the pair's own units
where they were, C's inside readout 0; it does not separate the readout from the units that answer it, nor from where C's own units lie.

**Without replay, A's trials pay where A answers readout 0.** Against the awake gap a night without replay stood below on A's trials in 5
cells of the six from block 18, and where A answers readout 1 above in 3 to 4. At block 36 plain nights stood above the awake gap on A's
trials more often than below in both orientations.

**Two readings of the nights' synapses.** Plain nights add more inhibition onto the excitatory units of stimulus A's readout in every
cell, by 1.35 to 3.71 M where A answers readout 0 and 0.10 to 1.41 M where it answers readout 1, the ranges overlapping. They take more of
each readout's excitatory weight onto itself where A answers readout 1, the two orientations' ranges apart. The readout each moves more is
the side's at block 36 in 6 and 5 cells of 12. Twelve cells and one run of each cannot say which of these, if any, sets where the gain
falls.

Questions this suggests for a later exploration or a round:
- The exchange with the pair's own units moved too, C's to place 16 and D's to 5, so that the readout and the pair's own units part.
- A read unit by unit on the twelve cells, to see which of readout 0's units fire on the side's trials under each form.
- The exchange on H-47's other eight cells, and its awake gap, to read question 2 within one start.

## What did not run

- **A night whose replay leaves the readouts' synapses where they stood**, which ADR-0238 names and does not take.
- **The exchange's other forms**, plain nights on both and the awake gap, so question 2 is not read on the exchanged runs; and the exchange
  on H-47's other eight cells.
- **The split and the class matrix on the exchanged runs**, and on plain nights on both and the awake gap: questions 4 and 5 read the
  night without replay and the plain night on A's patterns, with H-47's roles.
- **A read unit by unit**: a read gives each readout's spike count, not which units fired.
- **REM's depotentiations and the zero sum's rounds** are not split out of a night's change, as in exploration 020.
- **Any SE for questions 4 and 5**: the class matrix and the split are means over the three paths.
- **Any other form, shift, baseline or network.**

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_where_a_replays_gain_falls_explore` | [41582250345](https://github.com/Menteion/Menteion/actions/runs/41582250345) | `6c89a39` | 707 |
| `the_roles_exchanged_a_first_mapping_on_seed_23_from_the_assignment_explore` | [41583848509](https://github.com/Menteion/Menteion/actions/runs/41583848509) | `6c89a39` | 4029 |
| `the_roles_exchanged_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [41583848509](https://github.com/Menteion/Menteion/actions/runs/41583848509) | `6c89a39` | 4066 |
| `the_roles_exchanged_the_eighth_image_on_seed_22_from_the_assignment_explore` | [41583848509](https://github.com/Menteion/Menteion/actions/runs/41583848509) | `6c89a39` | 3810 |
| `the_roles_exchanged_the_eighth_image_on_seed_24_from_the_mirrored_assignment_explore` | [41583848509](https://github.com/Menteion/Menteion/actions/runs/41583848509) | `6c89a39` | 3814 |
| `the_pair_and_the_readouts_a_first_mapping_on_seed_22_from_the_assignment_explore` | [41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) | `6c89a39` | 4056 |
| `the_pair_and_the_readouts_a_first_mapping_on_seed_23_from_the_assignment_explore` | [41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) | `6c89a39` | 4110 |
| `the_pair_and_the_readouts_a_first_mapping_on_seed_24_from_the_assignment_explore` | [41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) | `6c89a39` | 4084 |
| `the_pair_and_the_readouts_a_first_mapping_on_seed_24_from_the_mirrored_assignment_explore` | [41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) | `6c89a39` | 3944 |
| `the_pair_and_the_readouts_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) | `6c89a39` | 3935 |
| `the_pair_and_the_readouts_the_eighth_image_on_seed_23_from_the_assignment_explore` | [41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) | `6c89a39` | 3978 |
| `the_pair_and_the_readouts_the_eighth_image_on_seed_23_from_the_mirrored_assignment_explore` | [41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) | `6c89a39` | 3900 |
| `the_pair_and_the_readouts_the_eighth_image_on_seed_24_from_the_mirrored_assignment_explore` | [41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) | `6c89a39` | 4012 |
