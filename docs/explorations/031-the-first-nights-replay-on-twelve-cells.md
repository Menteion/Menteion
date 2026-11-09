---
status: exploratory
date: 2026-11-10
---

# 031: The first night's replay alone, on twelve cells

**Branch**: `explore/031-the-first-nights-replay-on-twelve-cells`
**Commit**: `a6819e3`, which adds the harness, on `cb6355c`: `main` at `125ccc3`, ADR-0246's merge, with exploration 030's tag merged
in, which changes nothing under `src/`. This branch changes nothing under `src/`, so the engine is `main`'s.
**Runs**: [42665197682](https://github.com/Menteion/Menteion/actions/runs/42665197682) (the harness's trial) and
[42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780) (eleven jobs, one a cell), both at `scope=arms`. On seed 23's
first mapping from the mirrored assignment both forms are read from exploration 030's one-night-alone job, run
[42621480151](https://github.com/Menteion/Menteion/actions/runs/42621480151). H-47's plain night on A's patterns is read from H-47's run
[41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613), and its class matrices, night lines and watch from exploration
020's runs [41500459562](https://github.com/Menteion/Menteion/actions/runs/41500459562) and
[41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777) for exploration 020's four cells and from exploration 021's run
[41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) for the other eight. None of those was run again.

[ADR-0246](../adr/0246-the-first-nights-replay-on-twelve-cells.md) ordered this exploration. On seed 23's first mapping from the mirrored
assignment, where [H-47](../research/H-47.md)'s plain night on A's patterns lies below the night without replay at block 36 beyond twice
the paired SE, −4.9 ± 1.09, [exploration 030](030-one-cell-night-by-night.md) read H-47's plain night replayed after block 6 alone, every
other night without replay, above it beyond 2 SE, +1.7 ± 0.78. No run had read that form on any other cell. This note reads it beside the
night without replay on all twelve of H-47's cells. It asks, at blocks 6, 12, 18, 24 and 36:

1. A's count and each stimulus's correct trials under the form: less the night without replay, with H-47's paired SE, the two forms run
   in one job; and less H-47's plain night, with [exploration 018](018-what-nights-keep.md)'s unpaired SE, H-47's plain night run in
   another job;
2. the side under the form, by stimulus and by group of units, against H-47's side on the same cell;
3. the pair under the form and A's highest coupling, as H-47's clauses 2 and 3 read them;
4. what each night and each stretch of blocks moved, from the class matrix by group, beside H-47's plain night and night without replay on
   the same cell from exploration 020's and 021's jobs, with C's own and D's own units and A's margin onto A's answer;
5. the engine's two sums over each night, its replays and depotentiations, the watch's wrong answers, and the ties;
6. the twelve cells side by side at block 36: the form less the night without replay with the job's paired SE, and less H-47's plain
   night with exploration 018's unpaired SE, each with its stimulus split, beside H-47's clause 1.

**A checkpoint is read after the night that follows its block**, and block 36's after the twelve blocks that follow the last night. So at
block 6 the form's rows are H-47's plain night's own, the night after block 6 being H-47's first plain night on the same image, and the
form less the night without replay there is H-47's plain night less its night without replay. From block 12 on, its rows part from both
of H-47's forms: its night after block 12 is H-47's night without replay, run on the image the plain night's first night and blocks 7 to
12 wrote.

**The side** is [ADR-0238](../adr/0238-what-keeps-a-mapping-where-the-replay-did-not.md)'s: at a checkpoint, the stimulus whose
difference from the night without replay is the larger of the two, and neither where they are equal. The note writes no prediction and
decides nothing. **Every figure here is exploratory**: from one run of each test on the repository's AArch64 host, in H-47's terms. A count
is a mean over the cell's 48 reads, sixteen on each of three paths, and a difference carries its paired SE unless another is named. A
difference is written "beyond 2 SE" where it lies beyond twice its SE; one written without it carries its sign alone.

**In brief**
- **Each job ran H-47's rows where its nights were H-47's, and the form's night after block 6 is H-47's first plain night.** Inside the
  test every row of the night without replay held H-47's, fifteen a cell, and the form's row at block 6 held H-47's plain night's, three a
  cell. In every cell the form's night after block 6 printed the lines of H-47's first plain night in exploration 020's or 021's job, and
  the trial held both forms' first nights to H-47's row at block 6 on path 0. Only that night read replays.
- **At block 6**, where the form's rows are H-47's plain night's, it lies above the night without replay beyond 2 SE in eight cells and
  within 2 SE in four.
- **At block 36, less the night without replay**, the form lies above beyond 2 SE in five cells, +1.7 ± 0.78 to +7.3 ± 0.65: four of the
  six where A answers readout 0, and seed 23's first mapping from the mirrored assignment, where it answers readout 1. It lies below beyond
  2 SE on seed 24's eighth image from the assignment, −2.0 ± 0.88, and within 2 SE in the other six, −0.9 ± 1.07 to +5.0 ± 2.61.
- **Where H-47's clause 1 holds**, ten cells, the form lies above the night without replay beyond 2 SE in four and within 2 SE in six.
- **Less H-47's plain night at block 36** the form lies below beyond twice exploration 018's unpaired SE in seven cells, −2.8 ± 1.01 to
  −13.1 ± 1.09, within in four, and above on seed 23's first mapping from the mirrored assignment, +6.6 ± 0.90.
- **By stimulus at block 36**, of the five cells where the form lies above the night without replay beyond 2 SE, the difference lies beyond
  2 SE on B's trials alone in three, on A's alone in one and on both in one.
- **The side at block 36** falls on H-47's stimulus in ten cells; it lies on A in six and on B in six.
- **The form's nights after blocks 12, 18 and 24**, without replay after the replayed night, left each inhibitory column of the class
  matrix lower than H-47's night without replay after the same block did, in all 36 of them, and A's margin moved by −0.21 to +0.15 M over
  each, where H-47's later plain nights raised it by 0.54 to 1.17 M.
- **The pair** stands at 61.9 to 63.6 at block 36 under the form, and A's highest coupling at 1.078 to 1.179 of H-42's, below H-47's plain
  night's in all twelve cells.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 020 to 030 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. A cell is named, as in H-47, by its seed, its image and the
arm it starts from, not by the mapping in force. Nights follow blocks 6, 12, 18 and 24, then 12 blocks run with none to block 36, on paths
0, 5 and 6. Each checkpoint reads A and the pair sixteen times each.

**Places.** A period of the ring is 20 units. Stimulus A presents place 0 of each period and B place 11. **The odd places** are the odd
places but 11 and **the even places** the even places but 0: H-47's readout 0 and readout 1, each with its excitatory and its inhibitory
units. C's own units lie at place 5, among the odd places, and D's at place 16, among the even, in every cell
([exploration 021](021-where-the-gain-falls.md), section 4).

**Twelve cells**, all of H-47's, in exploration 028's order: the six where A answers readout 0 first, then the six where it answers the
other readout. A answers readout 0 on the eighth images from the mirrored assignment and on the first mappings from the assignment, as each
cell's reads of A print it:

| Cell | A answers | B answers | H-47: plain less without replay at block 36 (clause 1) | Its two forms | H-47's class matrices and night lines |
| :--- | :--- | :--- | ---: | :--- | :--- |
| Seed 22, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +19.6 ± 1.06 | this exploration's | 021's |
| Seed 22, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +7.9 ± 0.51 | this exploration's | 021's |
| Seed 23, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +17.4 ± 1.08 | this exploration's | 021's |
| Seed 23, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +13.4 ± 1.56 | this exploration's | 021's |
| Seed 24, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +11.3 ± 0.96 | this exploration's | 021's |
| Seed 24, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +3.8 ± 1.57 | this exploration's | 021's |
| Seed 23, eighth, assignment | readout 1, the even places | readout 0, the odd places | +4.1 ± 1.45 | this exploration's | 021's |
| Seed 24, first mapping, mirrored | readout 1, the even places | readout 0, the odd places | +6.0 ± 1.54 | this exploration's | 021's |
| Seed 23, first mapping, mirrored | readout 1, the even places | readout 0, the odd places | −4.9 ± 1.09 | 030's one-night-alone job | 020's |
| Seed 24, eighth, assignment | readout 1, the even places | readout 0, the odd places | +1.8 ± 1.79 | this exploration's | 020's |
| Seed 22, first mapping, mirrored | readout 1, the even places | readout 0, the odd places | +5.0 ± 1.24 | this exploration's | 020's |
| Seed 22, eighth, assignment | readout 1, the even places | readout 0, the odd places | +2.2 ± 0.65 | this exploration's | 020's |

**Two forms a cell**, each chosen night by night (`E030Form`, exploration 030's), every night watched:
- **the night without replay** (`no-replay`), H-47's night without replay after every block, the job's control: inside the job it holds
  that the harness runs H-47's rows on the cell, and it pairs the form for its SE;
- **after block 6 alone** (`alone-6`), exploration 030's form: H-47's plain night on A's patterns after block 6, A's 102 patterns drawn from
  the start image and tagged at priority 200, and H-47's night without replay, the same night with nothing tagged, after blocks 12, 18 and
  24.

No hold is set. **H-47's plain night on A's patterns** (`plain-first`, plain on A's in the tables) is read from H-47's job, its rows and
reads, and from exploration 020's or 021's job on the same cell, its class matrices, night lines and watch; it is not run here.

**One job a cell** (`e030_job`), exploration 030's job with the two forms, on the eleven cells but seed 23's first mapping from the mirrored
assignment, where both forms are exploration 030's and are read from its one-night-alone job. The job writes A's patterns and their
targets, the split and the class matrix of the start image, of the image before each night and of the image after it, and of block 36's
image; it reads each night with `ReplayWatch`. Inside the test (`e030_night`), the form's night after block 6 reads replays and
depotentiations that are not zero, and every other night of both forms reads `replays 0` and `depotentiations 0`. Every row of the night
without replay, and the form's row at block 6 on each path, is held inside the test to H-47's row of that form on the same path and block
(`held_to`). A row is the image's CRC-64, A's and the pair's sixteen counts, A's couplings, their highest over the stretch before the
checkpoint, and the gain at the night's start and end. Beside each form the job prints how many of its fifteen rows are each of H-47's two
forms' rows. Each job ran its own head, the pair's first six blocks on each path, and the head is read once.

**The class matrix** is exploration 020's, read by group as exploration 030 reads it. **The odd places' excitatory units** are classes 4, 6
and 7, C's own and readout 0's two classes, and **the even places'** classes 5, 8 and 9, D's own and readout 1's two; the inhibitory units
are class 11 in the odd places, 12 in the even places and 13 elsewhere. An inhibitory entry is a magnitude, the weight negated, so a + is
more inhibition. **A night's change** is the image after it less the image before it; **a stretch's** the image before the next night, or
block 36's, less the image after the night before it; **the head's** the image before the first night less the start image. Each is the
mean over the three paths.

**A kind's margin onto A's answer** is exploration 020's, from the split: the kind's weight onto the readout mapping A asks of its stimulus
less onto the other, A's kinds and C's own against stimulus A's answer, B's kinds and D's own against B's. **A's margin** is the sum of
four kinds', A alone, A and C, B alone, and B and D.

**The SEs.** The form less the night without replay carries H-47's paired SE over its 48 differences, the night without replay of its own
job. The form less H-47's plain night, run in another job, carries exploration 018's unpaired SE: the mean over the paths of each path's
difference of means, ± √((the two forms' stream variances)/16 + (the paths' differences' SD)²/3), a negative variance as zero. At block 6,
where the form's rows are the plain night's own, that difference is written "= plain". H-47's plain night less its night without replay is
H-47's own, with H-47's paired SE, marked (H-47). A count of cells or checkpoints within or beyond twice an SE is taken on the unrounded
mean and SE.

## Before a job ran

ADR-0246 asks whether any form but the night without replay must run bit for bit as one already in the logs. **On seed 23's first mapping
from the mirrored assignment the form must, and it ran no job there**: exploration 030's one-night-alone job ran both forms on that cell,
on the same engine, harness and paths, and both are read from it. **On the other eleven cells none must.** There the logs hold H-47's four
forms and the nights of explorations 020 to 029, in H-47's layout or another, with or without a hold; each of them replays on all four
nights or on none, and exploration 030 ran on the one cell alone. The form replays after block 6 alone, so from its night after block 12
it runs H-47's night without replay on an image that only the plain night's chain wrote:

| Form | Its rows are H-47's up to block | The first night where it parts from H-47's forms | That night runs on |
| :--- | ---: | :--- | :--- |
| After block 6 alone | 6, the plain night's | the night after block 12, without replay | the plain night's image after blocks 7 to 12 |

At block 6 the two kinds of night, run on the same head, read different rows on every path of every cell in H-47's job: the image's CRC-64
differs on each of the three paths in each of the twelve. No log holds H-47's night without replay on an image the plain night's chain
wrote on any of the eleven cells, so the form's rows after block 6 are in no log there, and the eleven jobs ran.

The night after block 6 under the form is H-47's first plain night on the same image, so what it holds is exploration 020's and 021's
reading of that night; section 5 sets the form's night beside theirs and reads it no further.

## The runs are laid out as written

- **The trial** ran on path 0 of seed 23's first mapping from the assignment, a cell that runs a job, the pair's head and each form's first
  night, both watched, each image split and read once as it was, of A and of the pair. Each image, those two reads, A's couplings, their
  highest over the head and the gain held, inside the test, H-47's row at block 6 on path 0 of the form whose first night it is:

| Form | Its first night | H-47's row at block 6 on path 0 it holds | Image's CRC-64, H-47's row held inside the test | A's read as it was | The pair's | A's couplings | Gain over the night | Replays | Depotentiations | Watch: wrong | Its lines that are 021's job's night of the same kind after block 6 on path 0, label aside |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | :--- | ---: | ---: | ---: | :--- |
| Without replay | without replay | the night without replay's | 1049472092185774887 | 50 | 31 | 7 096 340, 5 666 987, 6 345 105, 6 901 365 | 112 276 to 114 144 | 0 | 0 | 0 | 91 of 91 |
| After block 6 alone | replays | the plain night's | 7582150002090434622 | 57 | 27 | 7 405 970, 5 357 375, 6 066 402, 7 180 052 | 112 276 to 107 421 | 1 339 | 640 | 0 | 420 of 420 |

- Each of the two nights' lines, its start, its windows, its watch and its end, are the lines of exploration 021's job's night of the same
  kind after block 6 on path 0 of the same cell, the label and the form's name aside: 91 of 91 for the night without replay and 420 of 420
  for the night that replays. So is the head's line. The night that replays read 1 339 replays and 640 depotentiations and the night
  without replay none, as the trial prints; inside the test the first was held to read both not zero and the second none of either. The
  watch read `wrong 0` in both.

- **Each of the eleven jobs** ran to the end, its two forms on the three paths. Inside the test, every row of the night without replay held
  H-47's row of its form on the same path and block, fifteen a cell, and the form's row at block 6 on each path held H-47's plain night's,
  three a cell. Beside each form the job prints how many of its fifteen rows are H-47's rows of either form. Set beside H-47's rows here,
  cell by cell, with seed 23's first mapping from the mirrored assignment read from exploration 030's job:

| Cell | Without replay: rows H-47's | After block 6 alone: rows H-47's plain night's, its night without replay's | Without replay: lines in H-47's lines job, label aside | Heads: lines in that job | After block 6 alone, night after block 6: its start, window, watch and end lines that are that job's plain night's after block 6 | Its later nights' end lines among that job's, either form | Reads with H-47's plain night's `stimuli` mask | Lines job |
| :--- | :--- | ---: | :--- | :--- | :--- | :--- | :--- | :--- |
| Seed 22, eighth, mirrored | 15 of 15 | 3, 0 | 1 647 of 1 647 | 3 of 3 | 1 156 of 1 156 | 0 of 9 | 960 of 960 | 021 |
| Seed 22, first mapping, assignment | 15 of 15 | 3, 0 | 1 647 of 1 647 | 3 of 3 | 1 141 of 1 141 | 0 of 9 | 960 of 960 | 021 |
| Seed 23, eighth, mirrored | 15 of 15 | 3, 0 | 1 695 of 1 695 | 3 of 3 | 1 199 of 1 199 | 0 of 9 | 960 of 960 | 021 |
| Seed 23, first mapping, assignment | 15 of 15 | 3, 0 | 1 695 of 1 695 | 3 of 3 | 1 260 of 1 260 | 0 of 9 | 960 of 960 | 021 |
| Seed 24, eighth, mirrored | 15 of 15 | 3, 0 | 1 671 of 1 671 | 3 of 3 | 1 194 of 1 194 | 0 of 9 | 960 of 960 | 021 |
| Seed 24, first mapping, assignment | 15 of 15 | 3, 0 | 1 671 of 1 671 | 3 of 3 | 1 211 of 1 211 | 0 of 9 | 960 of 960 | 021 |
| Seed 23, eighth, assignment | 15 of 15 | 3, 0 | 1 695 of 1 695 | 3 of 3 | 1 218 of 1 218 | 0 of 9 | 960 of 960 | 021 |
| Seed 24, first mapping, mirrored | 15 of 15 | 3, 0 | 1 671 of 1 671 | 3 of 3 | 1 222 of 1 222 | 0 of 9 | 960 of 960 | 021 |
| Seed 23, first mapping, mirrored | 15 of 15 | 3, 0 | 1 695 of 1 695 | 3 of 3 | 1 214 of 1 214 | 0 of 9 | 960 of 960 | 020 |
| Seed 24, eighth, assignment | 15 of 15 | 3, 0 | 1 671 of 1 671 | 3 of 3 | 1 195 of 1 195 | 0 of 9 | 960 of 960 | 020 |
| Seed 22, first mapping, mirrored | 15 of 15 | 3, 0 | 1 647 of 1 647 | 3 of 3 | 1 130 of 1 130 | 0 of 9 | 960 of 960 | 020 |
| Seed 22, eighth, assignment | 15 of 15 | 3, 0 | 1 647 of 1 647 | 3 of 3 | 1 145 of 1 145 | 0 of 9 | 960 of 960 | 020 |

- **The night without replay's lines** are 1 659 to 1 707 a job. Of them 1 647 to 1 695 are the lines of H-47's night without replay in
  exploration 020's or 021's job on the same cell, label aside, all of that job's; the other 12 are the job's notices that each night read
  what its form asks. The pair's heads, three lines a job, are that job's. So the night without replay's class matrices, splits and night
  lines are those jobs', and section 4 lists them once.
- **Each read of the night without replay** carries H-47's read's count and its `stimuli`, `right` and `tied` masks, 480 a cell, and every
  read of both forms carries the `stimuli` mask of H-47's plain night's read of the same path, checkpoint, mapping and stream, 960 a cell,
  so each trial presented the stimulus it presented in H-47.
- **The form's night after block 6** prints on its three paths the lines of that job's plain night after block 6, label and form aside, its
  start, windows, watch and end, 1 130 to 1 260 lines a cell, all of them. Of its nights after blocks 12, 18 and 24, nine a cell, none ends
  on the end line of that job's night without replay or plain night after the same block. Its head, its night after block 6 and its blocks 7
  to 12 read in the class matrix what H-47's plain night's read in that job, in every cell.
- **The rows.** Of the form's fifteen rows in each cell, the three at block 6 are H-47's plain night's and none is its night without
  replay's; the night without replay's fifteen are H-47's night without replay's and none its plain night's, as each job prints and as the
  rows set beside H-47's read.
- **The nights.** The form's 36 nights after block 6, three a cell, read 1 329 to 1 340 replays and 640 depotentiations each, as the jobs
  print, held inside the test to be not zero; the other 252 nights of both forms read `replays 0` and `depotentiations 0`, held inside the
  test, exploration 030's job's among them. The watch read `wrong 0` in all 288 nights, over 57 566 208 pairings, a tally over the twelve
  jobs. Each job's log prints H-42's couplings on its seed as H-47's and the lines job's logs print them.

## 1. A's count and each stimulus's trials under the form

A's count at each checkpoint, the mean of 48 reads, under the night without replay and the form, both this exploration's jobs, and under
H-47's plain night, H-47's job's; then the form less the night without replay, ± the paired SE; the form less H-47's plain night, ± the
unpaired SE, "= plain" at block 6, where the form's rows are the plain night's own; and H-47's plain night less its night without replay,
H-47's own:

| Cell | Block | Without replay | After block 6 alone | Plain on A's (H-47) | After block 6 alone less without replay | After block 6 alone less plain | H-47: plain less without replay |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 46.3 | 53.3 | 53.3 | +7.1 ± 0.91 | = plain | +7.1 ± 0.91 (H-47) |
| Seed 22, eighth, mirrored | 12 | 30.2 | 36.2 | 42.3 | +6.0 ± 1.35 | −6.0 ± 1.24 | +12.1 ± 0.85 (H-47) |
| Seed 22, eighth, mirrored | 18 | 26.3 | 30.2 | 41.8 | +3.9 ± 1.14 | −11.6 ± 1.44 | +15.5 ± 1.67 (H-47) |
| Seed 22, eighth, mirrored | 24 | 22.6 | 28.0 | 40.2 | +5.4 ± 0.47 | −12.3 ± 1.73 | +17.6 ± 1.49 (H-47) |
| Seed 22, eighth, mirrored | 36 | 16.6 | 23.8 | 36.2 | +7.3 ± 0.65 | −12.4 ± 1.05 | +19.6 ± 1.06 (H-47) |
| Seed 22, first mapping, assignment | 6 | 52.4 | 56.5 | 56.5 | +4.1 ± 0.82 | = plain | +4.1 ± 0.82 (H-47) |
| Seed 22, first mapping, assignment | 12 | 44.8 | 45.9 | 47.6 | +1.1 ± 0.49 | −1.7 ± 1.28 | +2.9 ± 0.56 (H-47) |
| Seed 22, first mapping, assignment | 18 | 40.2 | 44.9 | 46.9 | +4.7 ± 0.46 | −2.0 ± 1.32 | +6.8 ± 1.12 (H-47) |
| Seed 22, first mapping, assignment | 24 | 38.9 | 43.0 | 48.0 | +4.1 ± 0.79 | −5.0 ± 1.45 | +9.1 ± 1.37 (H-47) |
| Seed 22, first mapping, assignment | 36 | 36.3 | 40.6 | 44.3 | +4.3 ± 1.05 | −3.6 ± 0.87 | +7.9 ± 0.51 (H-47) |
| Seed 23, eighth, mirrored | 6 | 34.4 | 36.1 | 36.1 | +1.7 ± 1.20 | = plain | +1.7 ± 1.20 (H-47) |
| Seed 23, eighth, mirrored | 12 | 19.5 | 26.2 | 28.5 | +6.7 ± 0.95 | −2.3 ± 1.41 | +9.0 ± 1.07 (H-47) |
| Seed 23, eighth, mirrored | 18 | 14.9 | 23.4 | 27.6 | +8.5 ± 0.85 | −4.2 ± 1.04 | +12.7 ± 1.02 (H-47) |
| Seed 23, eighth, mirrored | 24 | 10.6 | 16.6 | 29.3 | +6.0 ± 1.26 | −12.7 ± 1.00 | +18.7 ± 1.62 (H-47) |
| Seed 23, eighth, mirrored | 36 | 8.2 | 12.5 | 25.6 | +4.3 ± 0.63 | −13.1 ± 1.09 | +17.4 ± 1.08 (H-47) |
| Seed 23, first mapping, assignment | 6 | 54.2 | 56.3 | 56.3 | +2.0 ± 0.79 | = plain | +2.0 ± 0.79 (H-47) |
| Seed 23, first mapping, assignment | 12 | 33.4 | 36.4 | 37.6 | +3.0 ± 1.04 | −1.2 ± 0.87 | +4.1 ± 1.18 (H-47) |
| Seed 23, first mapping, assignment | 18 | 27.2 | 31.1 | 35.0 | +3.9 ± 2.36 | −3.9 ± 1.07 | +7.9 ± 2.54 (H-47) |
| Seed 23, first mapping, assignment | 24 | 24.8 | 27.4 | 34.8 | +2.6 ± 2.48 | −7.4 ± 1.20 | +10.0 ± 1.90 (H-47) |
| Seed 23, first mapping, assignment | 36 | 19.7 | 24.7 | 33.0 | +5.0 ± 2.61 | −8.3 ± 1.52 | +13.4 ± 1.56 (H-47) |
| Seed 24, eighth, mirrored | 6 | 36.4 | 33.8 | 33.8 | −2.6 ± 1.35 | = plain | −2.6 ± 1.35 (H-47) |
| Seed 24, eighth, mirrored | 12 | 31.5 | 28.3 | 28.6 | −3.2 ± 1.07 | −0.3 ± 1.20 | −2.9 ± 1.19 (H-47) |
| Seed 24, eighth, mirrored | 18 | 20.2 | 22.4 | 28.6 | +2.2 ± 1.15 | −6.2 ± 1.69 | +8.3 ± 1.70 (H-47) |
| Seed 24, eighth, mirrored | 24 | 16.4 | 19.0 | 25.8 | +2.6 ± 1.22 | −6.8 ± 1.22 | +9.4 ± 1.58 (H-47) |
| Seed 24, eighth, mirrored | 36 | 12.5 | 15.6 | 23.8 | +3.1 ± 1.84 | −8.2 ± 2.09 | +11.3 ± 0.96 (H-47) |
| Seed 24, first mapping, assignment | 6 | 58.3 | 61.2 | 61.2 | +2.9 ± 0.72 | = plain | +2.9 ± 0.72 (H-47) |
| Seed 24, first mapping, assignment | 12 | 52.0 | 53.6 | 54.8 | +1.6 ± 0.76 | −1.2 ± 0.95 | +2.8 ± 0.93 (H-47) |
| Seed 24, first mapping, assignment | 18 | 47.5 | 49.9 | 51.3 | +2.4 ± 0.61 | −1.4 ± 2.00 | +3.9 ± 1.41 (H-47) |
| Seed 24, first mapping, assignment | 24 | 43.7 | 47.6 | 52.4 | +3.9 ± 0.91 | −4.8 ± 2.58 | +8.7 ± 3.17 (H-47) |
| Seed 24, first mapping, assignment | 36 | 40.1 | 43.9 | 43.8 | +3.9 ± 0.76 | +0.1 ± 1.97 | +3.8 ± 1.57 (H-47) |
| Seed 23, eighth, assignment | 6 | 47.4 | 50.4 | 50.4 | +3.0 ± 0.96 | = plain | +3.0 ± 0.96 (H-47) |
| Seed 23, eighth, assignment | 12 | 40.8 | 41.5 | 43.4 | +0.7 ± 0.89 | −1.9 ± 1.15 | +2.6 ± 0.94 (H-47) |
| Seed 23, eighth, assignment | 18 | 37.3 | 39.9 | 45.2 | +2.5 ± 0.92 | −5.4 ± 1.29 | +7.9 ± 0.90 (H-47) |
| Seed 23, eighth, assignment | 24 | 38.0 | 39.9 | 43.7 | +2.0 ± 1.05 | −3.8 ± 1.06 | +5.7 ± 1.34 (H-47) |
| Seed 23, eighth, assignment | 36 | 36.2 | 37.4 | 40.3 | +1.3 ± 1.04 | −2.8 ± 1.52 | +4.1 ± 1.45 (H-47) |
| Seed 24, first mapping, mirrored | 6 | 54.3 | 57.7 | 57.7 | +3.4 ± 0.79 | = plain | +3.4 ± 0.79 (H-47) |
| Seed 24, first mapping, mirrored | 12 | 46.3 | 48.6 | 51.0 | +2.2 ± 1.56 | −2.5 ± 1.17 | +4.7 ± 1.50 (H-47) |
| Seed 24, first mapping, mirrored | 18 | 44.0 | 48.4 | 51.8 | +4.4 ± 0.74 | −3.4 ± 1.06 | +7.8 ± 0.89 (H-47) |
| Seed 24, first mapping, mirrored | 24 | 44.4 | 46.9 | 52.0 | +2.5 ± 2.54 | −5.1 ± 1.86 | +7.6 ± 1.32 (H-47) |
| Seed 24, first mapping, mirrored | 36 | 42.7 | 46.0 | 48.7 | +3.2 ± 2.06 | −2.8 ± 1.01 | +6.0 ± 1.54 (H-47) |
| Seed 23, first mapping, mirrored | 6 | 55.0 | 56.9 | 56.9 | +1.9 ± 0.68 | = plain | +1.9 ± 0.68 (H-47) |
| Seed 23, first mapping, mirrored | 12 | 52.4 | 53.1 | 54.4 | +0.8 ± 0.44 | −1.2 ± 1.03 | +2.0 ± 1.26 (H-47) |
| Seed 23, first mapping, mirrored | 18 | 52.0 | 51.2 | 52.4 | −0.8 ± 1.51 | −1.1 ± 0.84 | +0.3 ± 1.17 (H-47) |
| Seed 23, first mapping, mirrored | 24 | 53.5 | 54.5 | 50.9 | +1.0 ± 1.08 | +3.6 ± 0.97 | −2.6 ± 1.30 (H-47) |
| Seed 23, first mapping, mirrored | 36 | 53.1 | 54.8 | 48.3 | +1.7 ± 0.78 | +6.6 ± 0.90 | −4.9 ± 1.09 (H-47) |
| Seed 24, eighth, assignment | 6 | 38.8 | 39.1 | 39.1 | +0.3 ± 0.64 | = plain | +0.3 ± 0.64 (H-47) |
| Seed 24, eighth, assignment | 12 | 34.8 | 36.2 | 34.2 | +1.4 ± 0.72 | +2.0 ± 0.88 | −0.6 ± 1.03 (H-47) |
| Seed 24, eighth, assignment | 18 | 34.4 | 34.9 | 28.5 | +0.4 ± 1.21 | +6.4 ± 1.70 | −5.9 ± 1.53 (H-47) |
| Seed 24, eighth, assignment | 24 | 35.6 | 33.0 | 29.6 | −2.6 ± 0.81 | +3.3 ± 2.22 | −5.9 ± 1.85 (H-47) |
| Seed 24, eighth, assignment | 36 | 25.6 | 23.6 | 27.4 | −2.0 ± 0.88 | −3.8 ± 2.14 | +1.8 ± 1.79 (H-47) |
| Seed 22, first mapping, mirrored | 6 | 56.6 | 57.7 | 57.7 | +1.1 ± 0.63 | = plain | +1.1 ± 0.63 (H-47) |
| Seed 22, first mapping, mirrored | 12 | 49.5 | 50.3 | 52.5 | +0.8 ± 2.31 | −2.2 ± 1.30 | +2.9 ± 1.93 (H-47) |
| Seed 22, first mapping, mirrored | 18 | 49.0 | 50.1 | 52.7 | +1.2 ± 2.89 | −2.5 ± 1.16 | +3.7 ± 2.22 (H-47) |
| Seed 22, first mapping, mirrored | 24 | 45.9 | 47.3 | 50.6 | +1.4 ± 2.33 | −3.3 ± 1.12 | +4.7 ± 2.24 (H-47) |
| Seed 22, first mapping, mirrored | 36 | 44.1 | 45.4 | 49.1 | +1.3 ± 2.40 | −3.7 ± 1.74 | +5.0 ± 1.24 (H-47) |
| Seed 22, eighth, assignment | 6 | 50.3 | 52.3 | 52.3 | +1.9 ± 0.65 | = plain | +1.9 ± 0.65 (H-47) |
| Seed 22, eighth, assignment | 12 | 47.6 | 47.5 | 50.6 | −0.1 ± 1.01 | −3.1 ± 1.09 | +3.0 ± 0.93 (H-47) |
| Seed 22, eighth, assignment | 18 | 47.2 | 47.3 | 50.6 | +0.2 ± 0.74 | −3.2 ± 1.39 | +3.4 ± 1.05 (H-47) |
| Seed 22, eighth, assignment | 24 | 47.3 | 45.7 | 50.2 | −1.6 ± 0.75 | −4.5 ± 0.94 | +2.8 ± 0.89 (H-47) |
| Seed 22, eighth, assignment | 36 | 44.2 | 43.3 | 46.4 | −0.9 ± 1.07 | −3.1 ± 1.56 | +2.2 ± 0.65 (H-47) |

- **At block 6 the form's rows are H-47's plain night's**, so the form less the night without replay is H-47's plain night less its night
  without replay there: above beyond twice the paired SE in eight cells, +1.9 ± 0.68 on seed 23's first mapping from the mirrored assignment
  to +7.1 ± 0.91 on seed 22's eighth image from the mirrored assignment, and within 2 SE in four: seed 23's eighth image from the mirrored
  assignment, +1.7 ± 1.20, seed 24's eighth image from the mirrored assignment, −2.6 ± 1.35, seed 24's eighth image from the assignment,
  +0.3 ± 0.64, and seed 22's first mapping from the mirrored assignment, +1.1 ± 0.63.
- **At block 36, less the night without replay**, the form lies above beyond 2 SE in five cells: seed 22's eighth image from the mirrored
  assignment, +7.3 ± 0.65, seed 22's first mapping from the assignment, +4.3 ± 1.05, seed 23's eighth image from the mirrored assignment,
  +4.3 ± 0.63, and seed 24's first mapping from the assignment, +3.9 ± 0.76, where A answers readout 0, and seed 23's first mapping from the
  mirrored assignment, +1.7 ± 0.78, where it answers readout 1. It lies below beyond 2 SE on seed 24's eighth image from the assignment,
  −2.0 ± 0.88, and within 2 SE in the other six: seed 23's first mapping from the assignment, +5.0 ± 2.61, and seed 24's eighth image from
  the mirrored assignment, +3.1 ± 1.84, where A answers readout 0; seed 23's eighth image from the assignment, +1.3 ± 1.04, seed 24's first
  mapping from the mirrored assignment, +3.2 ± 2.06, seed 22's first mapping from the mirrored assignment, +1.3 ± 2.40, and seed 22's
  eighth image from the assignment, −0.9 ± 1.07, where it answers readout 1.
- **From block 12 to block 36**, where the form's rows part from H-47's, it lies above the night without replay beyond 2 SE at 21 of the 48
  checkpoints and below at 4: on seed 24's eighth image from the mirrored assignment at block 12, −3.2 ± 1.07, on seed 24's eighth image
  from the assignment at blocks 24 and 36, −2.6 ± 0.81 and −2.0 ± 0.88, and on seed 22's eighth image from the assignment at block 24,
  −1.6 ± 0.75.
- **Less H-47's plain night at block 36**, the form lies below beyond twice exploration 018's unpaired SE in seven cells, −2.8 ± 1.01 on
  seed 24's first mapping from the mirrored assignment to −13.1 ± 1.09 on seed 23's eighth image from the mirrored assignment; above in one,
  seed 23's first mapping from the mirrored assignment, +6.6 ± 0.90; and within in four: seed 24's first mapping from the assignment,
  +0.1 ± 1.97, seed 23's eighth image from the assignment, −2.8 ± 1.52, seed 24's eighth image from the assignment, −3.8 ± 2.14, and seed
  22's eighth image from the assignment, −3.1 ± 1.56.
- **From block 12 to block 36** it lies below H-47's plain night beyond 2 SE at 27 of the 48 checkpoints and above at 4: on seed 24's eighth
  image from the assignment at blocks 12 and 18, +2.0 ± 0.88 and +6.4 ± 1.70, and on seed 23's first mapping from the mirrored assignment at
  blocks 24 and 36, +3.6 ± 0.97 and +6.6 ± 0.90. These are the two cells where H-47's plain night lies below its night without replay
  beyond 2 SE at block 18 or later: seed 24's eighth image from the assignment at blocks 18 and 24, −5.9 ± 1.53 and −5.9 ± 1.85, and seed
  23's first mapping from the mirrored assignment at block 36, −4.9 ± 1.09.

Each stimulus's correct trials under the form, of the trials a read presents of it, and the ties of both stimuli's trials, the mean of 48
reads; then each stimulus's trials less the night without replay, ± the paired SE, less H-47's plain night, ± the unpaired SE, and H-47's
plain night less its night without replay, H-47's own:

| Cell | Block | After block 6 alone: correct on A's trials | on B's trials | Ties | A's trials less without replay | B's trials less without replay | A's trials less plain | B's trials less plain | H-47: A's trials, plain less without replay | B's trials |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 29.0 | 24.4 | 3.4 | +4.2 ± 0.89 | +2.9 ± 1.07 | = plain | = plain | +4.2 ± 0.89 (H-47) | +2.9 ± 1.07 (H-47) |
| Seed 22, eighth, mirrored | 12 | 14.7 | 21.6 | 5.1 | +3.3 ± 1.42 | +2.7 ± 0.52 | −4.4 ± 1.04 | −1.6 ± 0.91 | +7.8 ± 1.13 (H-47) | +4.3 ± 0.84 (H-47) |
| Seed 22, eighth, mirrored | 18 | 9.3 | 20.9 | 5.2 | −0.2 ± 1.09 | +4.1 ± 0.14 | −8.8 ± 1.29 | −2.8 ± 1.33 | +8.6 ± 1.79 (H-47) | +6.9 ± 1.22 (H-47) |
| Seed 22, eighth, mirrored | 24 | 9.2 | 18.8 | 4.6 | +0.6 ± 0.68 | +4.8 ± 0.56 | −5.5 ± 0.87 | −6.8 ± 1.31 | +6.1 ± 1.14 (H-47) | +11.6 ± 1.20 (H-47) |
| Seed 22, eighth, mirrored | 36 | 5.4 | 18.4 | 4.4 | +0.8 ± 0.55 | +6.5 ± 0.44 | −8.5 ± 1.02 | −3.9 ± 0.69 | +9.2 ± 0.86 (H-47) | +10.4 ± 0.54 (H-47) |
| Seed 22, first mapping, assignment | 6 | 29.6 | 26.9 | 2.8 | +4.1 ± 0.45 | 0.0 ± 0.50 | = plain | = plain | +4.1 ± 0.45 (H-47) | 0.0 ± 0.50 (H-47) |
| Seed 22, first mapping, assignment | 12 | 23.2 | 22.6 | 4.9 | +2.1 ± 0.16 | −1.0 ± 0.50 | −1.7 ± 0.82 | 0.0 ± 1.37 | +3.8 ± 0.48 (H-47) | −0.9 ± 0.40 (H-47) |
| Seed 22, first mapping, assignment | 18 | 21.4 | 23.5 | 5.1 | +4.9 ± 0.78 | −0.2 ± 0.74 | −3.4 ± 1.05 | +1.4 ± 1.04 | +8.3 ± 0.81 (H-47) | −1.6 ± 0.69 (H-47) |
| Seed 22, first mapping, assignment | 24 | 18.7 | 24.3 | 4.4 | +3.2 ± 1.43 | +0.9 ± 0.81 | −5.1 ± 0.87 | +0.1 ± 0.90 | +8.3 ± 1.55 (H-47) | +0.8 ± 0.85 (H-47) |
| Seed 22, first mapping, assignment | 36 | 17.2 | 23.5 | 5.1 | +4.0 ± 1.93 | +0.3 ± 1.05 | −4.4 ± 1.43 | +0.8 ± 0.71 | +8.4 ± 1.10 (H-47) | −0.4 ± 0.83 (H-47) |
| Seed 23, eighth, mirrored | 6 | 22.5 | 13.6 | 5.3 | +2.5 ± 0.98 | −0.8 ± 0.90 | = plain | = plain | +2.5 ± 0.98 (H-47) | −0.8 ± 0.90 (H-47) |
| Seed 23, eighth, mirrored | 12 | 19.0 | 7.2 | 4.3 | +3.7 ± 0.88 | +3.0 ± 0.52 | −2.5 ± 1.04 | +0.2 ± 0.82 | +6.1 ± 0.90 (H-47) | +2.8 ± 0.44 (H-47) |
| Seed 23, eighth, mirrored | 18 | 17.9 | 5.4 | 4.5 | +5.7 ± 0.71 | +2.8 ± 0.65 | −6.4 ± 1.08 | +2.2 ± 0.68 | +12.1 ± 1.05 (H-47) | +0.6 ± 0.57 (H-47) |
| Seed 23, eighth, mirrored | 24 | 12.7 | 3.9 | 4.8 | +3.8 ± 0.96 | +2.2 ± 0.94 | −13.1 ± 1.08 | +0.4 ± 1.02 | +16.9 ± 1.43 (H-47) | +1.8 ± 0.26 (H-47) |
| Seed 23, eighth, mirrored | 36 | 11.1 | 1.4 | 3.6 | +3.7 ± 0.55 | +0.6 ± 0.17 | −12.2 ± 1.13 | −0.9 ± 0.37 | +15.9 ± 1.05 (H-47) | +1.5 ± 0.28 (H-47) |
| Seed 23, first mapping, assignment | 6 | 30.9 | 25.4 | 2.8 | +1.6 ± 0.46 | +0.4 ± 0.57 | = plain | = plain | +1.6 ± 0.46 (H-47) | +0.4 ± 0.57 (H-47) |
| Seed 23, first mapping, assignment | 12 | 28.4 | 8.0 | 3.3 | +4.2 ± 0.80 | −1.2 ± 1.09 | −1.8 ± 0.79 | +0.6 ± 0.96 | +6.0 ± 0.87 (H-47) | −1.8 ± 0.85 (H-47) |
| Seed 23, first mapping, assignment | 18 | 26.5 | 4.5 | 3.4 | +7.0 ± 1.97 | −3.1 ± 0.40 | −3.6 ± 1.18 | −0.4 ± 0.79 | +10.6 ± 1.65 (H-47) | −2.8 ± 1.24 (H-47) |
| Seed 23, first mapping, assignment | 24 | 22.9 | 4.5 | 4.7 | +5.6 ± 2.36 | −3.0 ± 0.44 | −7.5 ± 1.33 | +0.2 ± 0.65 | +13.2 ± 1.45 (H-47) | −3.2 ± 0.67 (H-47) |
| Seed 23, first mapping, assignment | 36 | 22.1 | 2.6 | 4.3 | +6.4 ± 2.30 | −1.4 ± 0.41 | −7.1 ± 1.51 | −1.3 ± 0.56 | +13.5 ± 1.09 (H-47) | −0.1 ± 0.62 (H-47) |
| Seed 24, eighth, mirrored | 6 | 17.0 | 16.8 | 5.8 | −2.1 ± 0.99 | −0.5 ± 1.17 | = plain | = plain | −2.1 ± 0.99 (H-47) | −0.5 ± 1.17 (H-47) |
| Seed 24, eighth, mirrored | 12 | 11.8 | 16.5 | 5.1 | −3.4 ± 0.55 | +0.2 ± 0.67 | −0.3 ± 0.77 | −0.1 ± 1.07 | −3.2 ± 0.72 (H-47) | +0.3 ± 0.77 (H-47) |
| Seed 24, eighth, mirrored | 18 | 9.3 | 13.1 | 4.8 | +0.1 ± 0.70 | +2.1 ± 1.06 | −0.6 ± 0.83 | −5.6 ± 1.21 | +0.7 ± 0.25 (H-47) | +7.6 ± 1.52 (H-47) |
| Seed 24, eighth, mirrored | 24 | 7.4 | 11.6 | 4.5 | 0.0 ± 1.35 | +2.6 ± 0.82 | −1.6 ± 0.89 | −5.1 ± 1.07 | +1.6 ± 1.33 (H-47) | +7.8 ± 1.47 (H-47) |
| Seed 24, eighth, mirrored | 36 | 6.0 | 9.6 | 4.4 | +0.7 ± 0.86 | +2.4 ± 1.06 | −0.9 ± 1.68 | −7.3 ± 0.99 | +1.5 ± 0.74 (H-47) | +9.7 ± 0.98 (H-47) |
| Seed 24, first mapping, assignment | 6 | 30.7 | 30.6 | 1.4 | +1.9 ± 0.57 | +1.1 ± 0.38 | = plain | = plain | +1.9 ± 0.57 (H-47) | +1.1 ± 0.38 (H-47) |
| Seed 24, first mapping, assignment | 12 | 27.5 | 26.1 | 3.3 | +1.1 ± 0.47 | +0.4 ± 0.88 | −1.1 ± 0.95 | −0.1 ± 0.98 | +2.2 ± 0.43 (H-47) | +0.6 ± 0.99 (H-47) |
| Seed 24, first mapping, assignment | 18 | 25.5 | 24.4 | 3.9 | +0.6 ± 0.71 | +1.9 ± 0.89 | −3.4 ± 0.92 | +2.0 ± 1.64 | +4.0 ± 0.73 (H-47) | −0.1 ± 1.55 (H-47) |
| Seed 24, first mapping, assignment | 24 | 23.5 | 24.1 | 4.3 | +1.3 ± 0.53 | +2.6 ± 0.62 | −4.8 ± 1.20 | 0.0 ± 1.77 | +6.1 ± 1.25 (H-47) | +2.6 ± 2.15 (H-47) |
| Seed 24, first mapping, assignment | 36 | 22.0 | 21.9 | 4.9 | +0.8 ± 0.49 | +3.1 ± 0.45 | −2.5 ± 1.07 | +2.6 ± 1.71 | +3.3 ± 0.54 (H-47) | +0.5 ± 1.44 (H-47) |
| Seed 23, eighth, assignment | 6 | 26.9 | 23.4 | 3.9 | +2.6 ± 0.75 | +0.4 ± 0.66 | = plain | = plain | +2.6 ± 0.75 (H-47) | +0.4 ± 0.66 (H-47) |
| Seed 23, eighth, assignment | 12 | 21.2 | 20.3 | 4.1 | +3.0 ± 0.71 | −2.3 ± 0.57 | 0.0 ± 1.41 | −1.9 ± 1.26 | +2.9 ± 0.87 (H-47) | −0.4 ± 0.64 (H-47) |
| Seed 23, eighth, assignment | 18 | 19.5 | 20.4 | 4.0 | +3.9 ± 0.54 | −1.4 ± 0.53 | −1.3 ± 0.95 | −4.0 ± 1.42 | +5.2 ± 0.90 (H-47) | +2.7 ± 0.71 (H-47) |
| Seed 23, eighth, assignment | 24 | 19.1 | 20.9 | 4.5 | +2.8 ± 0.47 | −0.8 ± 0.87 | −0.5 ± 1.36 | −3.3 ± 1.49 | +3.3 ± 1.29 (H-47) | +2.5 ± 0.79 (H-47) |
| Seed 23, eighth, assignment | 36 | 15.4 | 22.0 | 4.7 | +2.4 ± 0.41 | −1.2 ± 1.31 | +0.1 ± 1.48 | −3.0 ± 1.02 | +2.3 ± 0.88 (H-47) | +1.8 ± 0.84 (H-47) |
| Seed 24, first mapping, mirrored | 6 | 30.1 | 27.6 | 2.6 | +2.0 ± 0.50 | +1.4 ± 0.59 | = plain | = plain | +2.0 ± 0.50 (H-47) | +1.4 ± 0.59 (H-47) |
| Seed 24, first mapping, mirrored | 12 | 25.9 | 22.7 | 4.3 | +1.3 ± 0.56 | +1.0 ± 1.10 | −1.4 ± 1.07 | −1.1 ± 0.92 | +2.6 ± 0.61 (H-47) | +2.1 ± 1.01 (H-47) |
| Seed 24, first mapping, mirrored | 18 | 25.4 | 23.0 | 4.2 | +3.7 ± 0.59 | +0.7 ± 1.06 | −1.6 ± 1.17 | −1.8 ± 1.02 | +5.3 ± 0.46 (H-47) | +2.5 ± 0.95 (H-47) |
| Seed 24, first mapping, mirrored | 24 | 24.7 | 22.2 | 5.0 | +2.6 ± 0.86 | −0.1 ± 1.67 | −1.0 ± 1.11 | −4.0 ± 1.74 | +3.7 ± 0.86 (H-47) | +4.0 ± 0.60 (H-47) |
| Seed 24, first mapping, mirrored | 36 | 25.3 | 20.6 | 4.5 | +3.4 ± 0.53 | −0.1 ± 1.54 | +0.1 ± 0.61 | −2.9 ± 1.13 | +3.3 ± 0.67 (H-47) | +2.7 ± 0.99 (H-47) |
| Seed 23, first mapping, mirrored | 6 | 31.0 | 25.9 | 2.4 | +0.6 ± 0.31 | +1.3 ± 0.56 | = plain | = plain | +0.6 ± 0.31 (H-47) | +1.3 ± 0.56 (H-47) |
| Seed 23, first mapping, mirrored | 12 | 28.5 | 24.7 | 3.3 | 0.0 ± 0.56 | +0.8 ± 0.61 | −1.1 ± 0.96 | −0.1 ± 1.00 | +1.1 ± 0.70 (H-47) | +1.0 ± 1.17 (H-47) |
| Seed 23, first mapping, mirrored | 18 | 27.3 | 23.9 | 3.6 | −1.3 ± 0.74 | +0.5 ± 0.89 | −0.3 ± 1.17 | −0.8 ± 1.20 | −1.0 ± 0.92 (H-47) | +1.3 ± 0.69 (H-47) |
| Seed 23, first mapping, mirrored | 24 | 29.5 | 25.0 | 2.5 | −0.5 ± 0.42 | +1.5 ± 0.99 | +4.5 ± 1.25 | −0.9 ± 1.37 | −5.0 ± 1.25 (H-47) | +2.4 ± 0.38 (H-47) |
| Seed 23, first mapping, mirrored | 36 | 29.3 | 25.6 | 2.9 | 0.0 ± 0.56 | +1.7 ± 0.40 | +5.2 ± 1.42 | +1.4 ± 1.31 | −5.2 ± 1.50 (H-47) | +0.3 ± 0.88 (H-47) |
| Seed 24, eighth, assignment | 6 | 23.8 | 15.3 | 5.1 | +2.5 ± 0.77 | −2.3 ± 0.71 | = plain | = plain | +2.5 ± 0.77 (H-47) | −2.3 ± 0.71 (H-47) |
| Seed 24, eighth, assignment | 12 | 19.8 | 16.4 | 4.5 | +1.4 ± 1.16 | 0.0 ± 1.27 | +1.0 ± 0.96 | +1.0 ± 1.13 | +0.4 ± 1.31 (H-47) | −1.0 ± 1.51 (H-47) |
| Seed 24, eighth, assignment | 18 | 19.6 | 15.3 | 4.4 | +1.6 ± 0.86 | −1.2 ± 1.69 | +5.2 ± 1.51 | +1.2 ± 1.05 | −3.6 ± 0.86 (H-47) | −2.4 ± 1.34 (H-47) |
| Seed 24, eighth, assignment | 24 | 17.0 | 15.9 | 5.2 | −2.2 ± 0.37 | −0.4 ± 1.03 | +1.7 ± 0.95 | +1.6 ± 1.83 | −3.9 ± 0.78 (H-47) | −2.0 ± 1.34 (H-47) |
| Seed 24, eighth, assignment | 36 | 12.0 | 11.6 | 4.9 | −1.0 ± 0.63 | −1.0 ± 0.71 | −1.1 ± 1.43 | −2.7 ± 0.99 | +0.2 ± 0.76 (H-47) | +1.6 ± 1.50 (H-47) |
| Seed 22, first mapping, mirrored | 6 | 31.4 | 26.4 | 2.5 | +0.7 ± 0.36 | +0.4 ± 0.54 | = plain | = plain | +0.7 ± 0.36 (H-47) | +0.4 ± 0.54 (H-47) |
| Seed 22, first mapping, mirrored | 12 | 28.1 | 22.2 | 3.6 | −0.1 ± 1.04 | +0.9 ± 1.42 | −1.2 ± 1.24 | −0.9 ± 1.09 | +1.1 ± 0.68 (H-47) | +1.8 ± 1.42 (H-47) |
| Seed 22, first mapping, mirrored | 18 | 27.7 | 22.4 | 3.8 | −0.2 ± 1.33 | +1.4 ± 1.82 | +0.2 ± 0.96 | −2.7 ± 0.99 | −0.4 ± 1.04 (H-47) | +4.1 ± 1.42 (H-47) |
| Seed 22, first mapping, mirrored | 24 | 27.4 | 19.9 | 3.7 | −0.1 ± 1.14 | +1.5 ± 1.49 | −0.2 ± 0.86 | −3.0 ± 1.31 | +0.1 ± 1.09 (H-47) | +4.6 ± 1.47 (H-47) |
| Seed 22, first mapping, mirrored | 36 | 27.9 | 17.5 | 4.7 | +0.1 ± 1.24 | +1.1 ± 1.27 | +0.5 ± 0.95 | −4.3 ± 1.60 | −0.4 ± 0.83 (H-47) | +5.4 ± 1.47 (H-47) |
| Seed 22, eighth, assignment | 6 | 26.4 | 25.8 | 3.4 | +2.0 ± 0.46 | −0.1 ± 0.58 | = plain | = plain | +2.0 ± 0.46 (H-47) | −0.1 ± 0.58 (H-47) |
| Seed 22, eighth, assignment | 12 | 22.9 | 24.6 | 3.5 | 0.0 ± 0.57 | −0.1 ± 0.73 | −2.5 ± 0.94 | −0.6 ± 1.19 | +2.5 ± 0.45 (H-47) | +0.5 ± 0.81 (H-47) |
| Seed 22, eighth, assignment | 18 | 23.8 | 23.5 | 4.0 | −0.6 ± 0.39 | +0.7 ± 0.61 | +0.1 ± 1.29 | −3.3 ± 0.82 | −0.6 ± 0.88 (H-47) | +4.0 ± 0.58 (H-47) |
| Seed 22, eighth, assignment | 24 | 24.2 | 21.5 | 3.8 | −1.8 ± 0.34 | +0.2 ± 0.70 | −1.5 ± 1.30 | −2.9 ± 1.11 | −0.3 ± 0.99 (H-47) | +3.1 ± 1.17 (H-47) |
| Seed 22, eighth, assignment | 36 | 22.7 | 20.5 | 4.4 | −2.0 ± 0.58 | +1.1 ± 1.03 | −0.4 ± 1.13 | −2.7 ± 1.27 | −1.6 ± 0.88 (H-47) | +3.8 ± 0.81 (H-47) |

- **On A's trials at block 36, less the night without replay**, the form lies above beyond 2 SE in five cells: seed 22's first mapping from
  the assignment, +4.0 ± 1.93, seed 23's eighth image from the mirrored assignment, +3.7 ± 0.55, and seed 23's first mapping from the
  assignment, +6.4 ± 2.30, where A answers readout 0, and seed 23's eighth image from the assignment, +2.4 ± 0.41, and seed 24's first
  mapping from the mirrored assignment, +3.4 ± 0.53, where it answers readout 1. It lies below beyond 2 SE on seed 22's eighth image from
  the assignment, −2.0 ± 0.58, and within 2 SE in the other six.
- **On B's trials at block 36** it lies above beyond 2 SE in five cells: seed 22's eighth image from the mirrored assignment, +6.5 ± 0.44,
  seed 23's eighth image from the mirrored assignment, +0.6 ± 0.17, seed 24's eighth image from the mirrored assignment, +2.4 ± 1.06, and
  seed 24's first mapping from the assignment, +3.1 ± 0.45, where A answers readout 0, and seed 23's first mapping from the mirrored
  assignment, +1.7 ± 0.40, where it answers readout 1. It lies below beyond 2 SE on seed 23's first mapping from the assignment,
  −1.4 ± 0.41, and within 2 SE in the other six.
- **Of the five cells where A's count lies above the night without replay beyond 2 SE at block 36**, the difference lies beyond 2 SE on B's
  trials and within on A's on seed 22's eighth image from the mirrored assignment, B's +6.5 ± 0.44 and A's +0.8 ± 0.55, seed 24's first
  mapping from the assignment, +3.1 ± 0.45 and +0.8 ± 0.49, and seed 23's first mapping from the mirrored assignment, +1.7 ± 0.40 and
  0.0 ± 0.56; beyond 2 SE on A's trials and within on B's on seed 22's first mapping from the assignment, A's +4.0 ± 1.93 and B's
  +0.3 ± 1.05; and beyond 2 SE on both on seed 23's eighth image from the mirrored assignment, A's +3.7 ± 0.55 and B's +0.6 ± 0.17. On
  seed 24's eighth image from the assignment, where the count lies below, both stimuli's trials read within 2 SE, A's −1.0 ± 0.63 and
  B's −1.0 ± 0.71.
- **Less H-47's plain night at block 36**, on A's trials the form lies below beyond 2 SE in five cells, all where A answers readout 0: seed
  22's eighth image from the mirrored assignment, −8.5 ± 1.02, seed 22's first mapping from the assignment, −4.4 ± 1.43, seed 23's eighth
  image from the mirrored assignment, −12.2 ± 1.13, seed 23's first mapping from the assignment, −7.1 ± 1.51, and seed 24's first mapping
  from the assignment, −2.5 ± 1.07. It lies above beyond 2 SE on seed 23's first mapping from the mirrored assignment, +5.2 ± 1.42, and
  within in the other six. On B's trials it lies below beyond 2 SE in nine cells and within in three: seed 22's first mapping from the
  assignment, +0.8 ± 0.71, seed 24's first mapping from the assignment, +2.6 ± 1.71, and seed 23's first mapping from the mirrored
  assignment, +1.4 ± 1.31.

## 2. The side under the form

Each stimulus's trials less the night without replay of the form's own job, and A's less B's, ± their paired SEs, with the side: the
stimulus whose difference is the larger, and its group of units, the places the stimulus answers on that cell under A. H-47's side is its
plain night's against its night without replay, H-47's job's:

| Cell | Block | A's trials less without replay | B's trials less without replay | A's less B's | Side | H-47's side |
| :--- | ---: | ---: | ---: | ---: | :--- | :--- |
| Seed 22, eighth, mirrored | 6 | +4.2 ± 0.89 | +2.9 ± 1.07 | +1.4 ± 1.74 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | 12 | +3.3 ± 1.42 | +2.7 ± 0.52 | +0.6 ± 1.66 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | 18 | −0.2 ± 1.09 | +4.1 ± 0.14 | −4.3 ± 1.07 | B, the even places | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | 24 | +0.6 ± 0.68 | +4.8 ± 0.56 | −4.3 ± 1.15 | B, the even places | B, the even places |
| Seed 22, eighth, mirrored | 36 | +0.8 ± 0.55 | +6.5 ± 0.44 | −5.8 ± 0.77 | B, the even places | B, the even places (within 2 SE) |
| Seed 22, first mapping, assignment | 6 | +4.1 ± 0.45 | 0.0 ± 0.50 | +4.1 ± 0.49 | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 12 | +2.1 ± 0.16 | −1.0 ± 0.50 | +3.0 ± 0.55 | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 18 | +4.9 ± 0.78 | −0.2 ± 0.74 | +5.1 ± 1.44 | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 24 | +3.2 ± 1.43 | +0.9 ± 0.81 | +2.3 ± 2.18 | A, the odd places (within 2 SE) | A, the odd places |
| Seed 22, first mapping, assignment | 36 | +4.0 ± 1.93 | +0.3 ± 1.05 | +3.7 ± 2.92 | A, the odd places (within 2 SE) | A, the odd places |
| Seed 23, eighth, mirrored | 6 | +2.5 ± 0.98 | −0.8 ± 0.90 | +3.4 ± 1.46 | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | 12 | +3.7 ± 0.88 | +3.0 ± 0.52 | +0.6 ± 1.09 | A, the odd places (within 2 SE) | A, the odd places |
| Seed 23, eighth, mirrored | 18 | +5.7 ± 0.71 | +2.8 ± 0.65 | +2.8 ± 1.05 | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | 24 | +3.8 ± 0.96 | +2.2 ± 0.94 | +1.6 ± 1.42 | A, the odd places (within 2 SE) | A, the odd places |
| Seed 23, eighth, mirrored | 36 | +3.7 ± 0.55 | +0.6 ± 0.17 | +3.1 ± 0.52 | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 6 | +1.6 ± 0.46 | +0.4 ± 0.57 | +1.3 ± 0.67 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 23, first mapping, assignment | 12 | +4.2 ± 0.80 | −1.2 ± 1.09 | +5.4 ± 1.61 | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 18 | +7.0 ± 1.97 | −3.1 ± 0.40 | +10.1 ± 1.60 | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 24 | +5.6 ± 2.36 | −3.0 ± 0.44 | +8.7 ± 2.32 | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 36 | +6.4 ± 2.30 | −1.4 ± 0.41 | +7.8 ± 2.02 | A, the odd places | A, the odd places |
| Seed 24, eighth, mirrored | 6 | −2.1 ± 0.99 | −0.5 ± 1.17 | −1.6 ± 1.69 | B, the even places (within 2 SE) | B, the even places (within 2 SE) |
| Seed 24, eighth, mirrored | 12 | −3.4 ± 0.55 | +0.2 ± 0.67 | −3.6 ± 0.58 | B, the even places | B, the even places |
| Seed 24, eighth, mirrored | 18 | +0.1 ± 0.70 | +2.1 ± 1.06 | −2.0 ± 1.39 | B, the even places (within 2 SE) | B, the even places |
| Seed 24, eighth, mirrored | 24 | 0.0 ± 1.35 | +2.6 ± 0.82 | −2.7 ± 1.87 | B, the even places (within 2 SE) | B, the even places |
| Seed 24, eighth, mirrored | 36 | +0.7 ± 0.86 | +2.4 ± 1.06 | −1.7 ± 0.53 | B, the even places | B, the even places |
| Seed 24, first mapping, assignment | 6 | +1.9 ± 0.57 | +1.1 ± 0.38 | +0.8 ± 0.65 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 24, first mapping, assignment | 12 | +1.1 ± 0.47 | +0.4 ± 0.88 | +0.7 ± 1.19 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 24, first mapping, assignment | 18 | +0.6 ± 0.71 | +1.9 ± 0.89 | −1.3 ± 1.49 | B, the even places (within 2 SE) | A, the odd places |
| Seed 24, first mapping, assignment | 24 | +1.3 ± 0.53 | +2.6 ± 0.62 | −1.4 ± 0.70 | B, the even places (within 2 SE) | A, the odd places |
| Seed 24, first mapping, assignment | 36 | +0.8 ± 0.49 | +3.1 ± 0.45 | −2.3 ± 0.55 | B, the even places | A, the odd places (within 2 SE) |
| Seed 23, eighth, assignment | 6 | +2.6 ± 0.75 | +0.4 ± 0.66 | +2.3 ± 1.04 | A, the even places | A, the even places |
| Seed 23, eighth, assignment | 12 | +3.0 ± 0.71 | −2.3 ± 0.57 | +5.2 ± 0.92 | A, the even places | A, the even places |
| Seed 23, eighth, assignment | 18 | +3.9 ± 0.54 | −1.4 ± 0.53 | +5.3 ± 0.55 | A, the even places | A, the even places (within 2 SE) |
| Seed 23, eighth, assignment | 24 | +2.8 ± 0.47 | −0.8 ± 0.87 | +3.5 ± 0.92 | A, the even places | A, the even places (within 2 SE) |
| Seed 23, eighth, assignment | 36 | +2.4 ± 0.41 | −1.2 ± 1.31 | +3.6 ± 1.64 | A, the even places | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | 6 | +2.0 ± 0.50 | +1.4 ± 0.59 | +0.7 ± 0.76 | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | 12 | +1.3 ± 0.56 | +1.0 ± 1.10 | +0.3 ± 0.80 | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | 18 | +3.7 ± 0.59 | +0.7 ± 1.06 | +3.0 ± 1.55 | A, the even places (within 2 SE) | A, the even places |
| Seed 24, first mapping, mirrored | 24 | +2.6 ± 0.86 | −0.1 ± 1.67 | +2.7 ± 0.82 | A, the even places | B, the odd places (within 2 SE) |
| Seed 24, first mapping, mirrored | 36 | +3.4 ± 0.53 | −0.1 ± 1.54 | +3.5 ± 1.03 | A, the even places | A, the even places (within 2 SE) |
| Seed 23, first mapping, mirrored | 6 | +0.6 ± 0.31 | +1.3 ± 0.56 | −0.7 ± 0.61 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 12 | 0.0 ± 0.56 | +0.8 ± 0.61 | −0.9 ± 1.08 | B, the odd places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 23, first mapping, mirrored | 18 | −1.3 ± 0.74 | +0.5 ± 0.89 | −1.7 ± 0.63 | B, the odd places | B, the odd places |
| Seed 23, first mapping, mirrored | 24 | −0.5 ± 0.42 | +1.5 ± 0.99 | −2.0 ± 1.07 | B, the odd places (within 2 SE) | B, the odd places |
| Seed 23, first mapping, mirrored | 36 | 0.0 ± 0.56 | +1.7 ± 0.40 | −1.6 ± 0.60 | B, the odd places | B, the odd places |
| Seed 24, eighth, assignment | 6 | +2.5 ± 0.77 | −2.3 ± 0.71 | +4.8 ± 1.34 | A, the even places | A, the even places |
| Seed 24, eighth, assignment | 12 | +1.4 ± 1.16 | 0.0 ± 1.27 | +1.3 ± 2.32 | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, eighth, assignment | 18 | +1.6 ± 0.86 | −1.2 ± 1.69 | +2.8 ± 2.39 | A, the even places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | 24 | −2.2 ± 0.37 | −0.4 ± 1.03 | −1.8 ± 1.32 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | 36 | −1.0 ± 0.63 | −1.0 ± 0.71 | +0.1 ± 1.01 | A, the even places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 22, first mapping, mirrored | 6 | +0.7 ± 0.36 | +0.4 ± 0.54 | +0.4 ± 0.68 | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | 12 | −0.1 ± 1.04 | +0.9 ± 1.42 | −0.9 ± 0.92 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 22, first mapping, mirrored | 18 | −0.2 ± 1.33 | +1.4 ± 1.82 | −1.6 ± 1.33 | B, the odd places (within 2 SE) | B, the odd places |
| Seed 22, first mapping, mirrored | 24 | −0.1 ± 1.14 | +1.5 ± 1.49 | −1.6 ± 1.28 | B, the odd places (within 2 SE) | B, the odd places |
| Seed 22, first mapping, mirrored | 36 | +0.1 ± 1.24 | +1.1 ± 1.27 | −1.0 ± 0.69 | B, the odd places (within 2 SE) | B, the odd places |
| Seed 22, eighth, assignment | 6 | +2.0 ± 0.46 | −0.1 ± 0.58 | +2.1 ± 0.83 | A, the even places | A, the even places |
| Seed 22, eighth, assignment | 12 | 0.0 ± 0.57 | −0.1 ± 0.73 | +0.2 ± 0.83 | A, the even places (within 2 SE) | A, the even places |
| Seed 22, eighth, assignment | 18 | −0.6 ± 0.39 | +0.7 ± 0.61 | −1.3 ± 0.71 | B, the odd places (within 2 SE) | B, the odd places |
| Seed 22, eighth, assignment | 24 | −1.8 ± 0.34 | +0.2 ± 0.70 | −2.0 ± 0.81 | B, the odd places | B, the odd places (within 2 SE) |
| Seed 22, eighth, assignment | 36 | −2.0 ± 0.58 | +1.1 ± 1.03 | −3.1 ± 1.23 | B, the odd places | B, the odd places |

- **At block 6** the side is H-47's in every cell, the form's rows being its plain night's.
- **At block 36** the side falls on the stimulus of H-47's side in ten of the twelve cells. On seed 24's first mapping from the assignment
  it lies on B, the even places, beyond 2 SE, A's less B's −2.3 ± 0.55, where H-47's lies on A, the odd places, within 2 SE; on seed 24's
  eighth image from the assignment it lies on A, the even places, within 2 SE, +0.1 ± 1.01, where H-47's lies on B, the odd places, within 2
  SE.
- **At block 36** the side lies on A in six cells and on B in six, on the odd places in six and on the even places in six; beyond 2 SE in
  nine, and within 2 SE on seed 22's first mapping from the assignment, seed 24's eighth image from the assignment and seed 22's first
  mapping from the mirrored assignment. No checkpoint of any cell reads neither.
- **At blocks 12, 18 and 24** the side falls on H-47's stimulus in eleven, nine and ten cells. It falls on the other stimulus at block 12 on
  seed 23's first mapping from the mirrored assignment; at block 18 on seed 22's eighth image from the mirrored assignment, seed 24's first
  mapping from the assignment and seed 24's eighth image from the assignment; and at block 24 on seed 24's first mapping from the
  assignment and seed 24's first mapping from the mirrored assignment.

## 3. The pair and A's couplings

The pair's count under the form at each checkpoint, the mean of 48 reads, and at block 36 under the night without replay and H-47's plain
night; and A's highest coupling over the same coupling in H-42's image on the cell's seed, in any image read and at any block's end of the
form's 36, as H-47's clauses 2 and 3 read them:

| Cell | After block 6 alone: the pair at block 6 | 12 | 18 | 24 | 36 | Without replay: the pair at 36 | Plain on A's (H-47) at 36 | A's highest coupling over H-42's: after block 6 alone | without replay | plain on A's (H-47) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 45.8 | 61.4 | 61.9 | 62.4 | 63.2 | 63.0 | 62.8 | 1.144 | 1.144 | 1.197 |
| Seed 22, first mapping, assignment | 42.1 | 60.8 | 61.7 | 62.2 | 63.4 | 63.3 | 63.1 | 1.127 | 1.112 | 1.198 |
| Seed 23, eighth, mirrored | 57.8 | 59.8 | 59.0 | 60.4 | 63.2 | 62.6 | 63.1 | 1.147 | 1.147 | 1.211 |
| Seed 23, first mapping, assignment | 27.7 | 56.6 | 61.8 | 61.5 | 63.0 | 63.1 | 61.9 | 1.149 | 1.125 | 1.212 |
| Seed 24, eighth, mirrored | 62.0 | 61.1 | 60.6 | 60.4 | 61.9 | 62.7 | 63.0 | 1.078 | 1.078 | 1.141 |
| Seed 24, first mapping, assignment | 34.8 | 51.6 | 59.8 | 60.4 | 62.5 | 62.1 | 61.8 | 1.095 | 1.085 | 1.162 |
| Seed 23, eighth, assignment | 54.7 | 60.7 | 61.5 | 62.5 | 63.6 | 63.4 | 63.4 | 1.118 | 1.118 | 1.171 |
| Seed 24, first mapping, mirrored | 51.6 | 60.8 | 62.5 | 63.0 | 63.3 | 63.4 | 62.4 | 1.178 | 1.177 | 1.223 |
| Seed 23, first mapping, mirrored | 49.3 | 59.5 | 62.4 | 63.0 | 63.5 | 63.5 | 63.2 | 1.102 | 1.082 | 1.158 |
| Seed 24, eighth, assignment | 61.0 | 59.8 | 58.6 | 57.7 | 62.5 | 62.6 | 62.9 | 1.179 | 1.179 | 1.200 |
| Seed 22, first mapping, mirrored | 54.3 | 60.8 | 62.1 | 62.6 | 63.1 | 63.1 | 62.5 | 1.122 | 1.114 | 1.182 |
| Seed 22, eighth, assignment | 63.1 | 62.2 | 61.5 | 61.0 | 63.0 | 62.6 | 62.9 | 1.133 | 1.133 | 1.184 |

- **The pair stands at 61.9 to 63.6 at block 36 under the form**, the lowest on seed 24's eighth image from the mirrored assignment and the
  highest on seed 23's eighth image from the assignment, above clause 2's floor of 56 in every cell; under the night without replay at 62.1
  to 63.5, and under H-47's plain night at 61.8 to 63.4.
- **A's highest coupling under the form** reads 1.078 to 1.179 of H-42's, the lowest on seed 24's eighth image from the mirrored
  assignment and the highest on seed 24's eighth image from the assignment; none passes clause 3's 1.30. It is the night without replay's
  own on the six eighth images, and above it on the six first mappings by 0.001 to 0.024. It lies below H-47's plain night's, 1.141 to
  1.223, in all twelve.

## 4. What each night and each stretch moved

What each night moved, from the class matrix in the image before it and the image after it, by group, the mean over the three paths, in M.
The odd places' excitatory units are classes 4, 6 and 7, the even places' 5, 8 and 9; "onto their own" counts each group's excitatory
units onto the same group's; the inhibitory rows are the odd places' class 11 and the even places' class 12 onto every class, and
"inhibitory onto" every inhibitory class, 11 to 13, onto a group's excitatory units, a + more inhibition. Beside the form are H-47's plain
night and night without replay on the same cell, from exploration 020's job on its four cells and exploration 021's on the other eight. The
form's night without replay is not listed: each job's lines for it are those of H-47's night without replay in that job, label aside
(section "The runs are laid out as written"). Where A answers readout 0 its group is the odd places, and where it answers readout 1 the
even places:

| Cell | Night after block | Form | This night | The odd places' excitatory onto all, M | the even places' | The odd places' onto their own | the even places' onto their own | The odd places' inhibitory onto all | the even places' | Inhibitory onto the odd places' excitatory | onto the even places' |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | After block 6 alone | replays | −4.04 | −4.28 | −0.47 | −0.47 | +5.08 | +5.03 | +4.66 | +3.94 |
| Seed 22, eighth, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −4.04 | −4.28 | −0.47 | −0.47 | +5.08 | +5.03 | +4.66 | +3.94 |
| Seed 22, eighth, mirrored | 6 | Without replay (H-47, 021's job) | without replay | −0.22 | −0.30 | −0.33 | −0.30 | +0.15 | +0.16 | +0.27 | +0.21 |
| Seed 22, eighth, mirrored | 12 | After block 6 alone | without replay | +0.09 | −0.07 | −0.21 | −0.18 | −0.31 | −0.27 | −0.07 | −0.19 |
| Seed 22, eighth, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −2.75 | −2.89 | −0.25 | −0.22 | +4.02 | +4.05 | +3.74 | +3.18 |
| Seed 22, eighth, mirrored | 12 | Without replay (H-47, 021's job) | without replay | +0.04 | −0.20 | −0.13 | −0.17 | +0.12 | +0.19 | +0.28 | +0.18 |
| Seed 22, eighth, mirrored | 18 | After block 6 alone | without replay | +0.21 | −0.10 | −0.04 | −0.19 | −0.24 | −0.06 | −0.01 | −0.10 |
| Seed 22, eighth, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −1.61 | −1.90 | +0.06 | −0.05 | +3.29 | +3.61 | +3.16 | +2.73 |
| Seed 22, eighth, mirrored | 18 | Without replay (H-47, 021's job) | without replay | +0.13 | −0.08 | −0.06 | −0.14 | +0.08 | +0.22 | +0.27 | +0.14 |
| Seed 22, eighth, mirrored | 24 | After block 6 alone | without replay | +0.33 | +0.02 | +0.05 | −0.13 | −0.13 | +0.02 | +0.03 | −0.02 |
| Seed 22, eighth, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −0.87 | −1.31 | +0.18 | +0.12 | +2.73 | +3.16 | +2.67 | +2.36 |
| Seed 22, eighth, mirrored | 24 | Without replay (H-47, 021's job) | without replay | +0.35 | 0.00 | +0.04 | −0.07 | +0.13 | +0.26 | +0.28 | +0.16 |
| Seed 22, first mapping, assignment | 6 | After block 6 alone | replays | −3.45 | −3.38 | −0.39 | −0.17 | +4.42 | +4.66 | +3.96 | +3.26 |
| Seed 22, first mapping, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −3.45 | −3.38 | −0.39 | −0.17 | +4.42 | +4.66 | +3.96 | +3.26 |
| Seed 22, first mapping, assignment | 6 | Without replay (H-47, 021's job) | without replay | −0.35 | −0.19 | −0.40 | −0.27 | −0.64 | −0.58 | −0.40 | −0.47 |
| Seed 22, first mapping, assignment | 12 | After block 6 alone | without replay | +0.01 | +0.01 | −0.22 | −0.24 | −0.86 | −0.76 | −0.58 | −0.61 |
| Seed 22, first mapping, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −2.60 | −2.73 | −0.30 | −0.23 | +2.93 | +3.09 | +2.59 | +2.28 |
| Seed 22, first mapping, assignment | 12 | Without replay (H-47, 021's job) | without replay | +0.11 | −0.04 | −0.16 | −0.20 | −0.73 | −0.64 | −0.51 | −0.52 |
| Seed 22, first mapping, assignment | 18 | After block 6 alone | without replay | +0.31 | +0.10 | −0.05 | −0.20 | −0.87 | −0.82 | −0.61 | −0.68 |
| Seed 22, first mapping, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −1.53 | −1.89 | +0.05 | −0.10 | +1.91 | +2.07 | +1.64 | +1.48 |
| Seed 22, first mapping, assignment | 18 | Without replay (H-47, 021's job) | without replay | +0.38 | +0.05 | +0.02 | −0.11 | −0.79 | −0.67 | −0.52 | −0.62 |
| Seed 22, first mapping, assignment | 24 | After block 6 alone | without replay | +0.30 | −0.03 | +0.04 | −0.12 | −0.84 | −0.75 | −0.53 | −0.64 |
| Seed 22, first mapping, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −1.13 | −1.47 | +0.14 | −0.03 | +1.25 | +1.35 | +1.09 | +0.92 |
| Seed 22, first mapping, assignment | 24 | Without replay (H-47, 021's job) | without replay | +0.34 | −0.03 | +0.05 | −0.13 | −0.77 | −0.56 | −0.46 | −0.58 |
| Seed 23, eighth, mirrored | 6 | After block 6 alone | replays | −4.48 | −4.16 | −0.35 | −0.46 | +5.57 | +5.00 | +4.71 | +4.17 |
| Seed 23, eighth, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −4.48 | −4.16 | −0.35 | −0.46 | +5.57 | +5.00 | +4.71 | +4.17 |
| Seed 23, eighth, mirrored | 6 | Without replay (H-47, 021's job) | without replay | −0.27 | −0.41 | −0.18 | −0.22 | +0.18 | +0.15 | +0.26 | +0.20 |
| Seed 23, eighth, mirrored | 12 | After block 6 alone | without replay | +0.08 | −0.19 | −0.16 | −0.16 | −0.34 | −0.30 | −0.14 | −0.17 |
| Seed 23, eighth, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −2.90 | −2.73 | −0.11 | −0.27 | +4.52 | +3.97 | +3.80 | +3.46 |
| Seed 23, eighth, mirrored | 12 | Without replay (H-47, 021's job) | without replay | +0.33 | −0.21 | −0.09 | −0.13 | +0.23 | +0.15 | +0.27 | +0.19 |
| Seed 23, eighth, mirrored | 18 | After block 6 alone | without replay | +0.29 | 0.00 | −0.11 | −0.12 | −0.21 | −0.15 | −0.10 | −0.09 |
| Seed 23, eighth, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −2.15 | −1.96 | −0.05 | −0.20 | +3.61 | +3.44 | +3.21 | +2.84 |
| Seed 23, eighth, mirrored | 18 | Without replay (H-47, 021's job) | without replay | +0.60 | +0.24 | +0.11 | −0.01 | +0.24 | +0.22 | +0.22 | +0.26 |
| Seed 23, eighth, mirrored | 24 | After block 6 alone | without replay | +0.75 | +0.26 | +0.18 | 0.00 | −0.05 | −0.16 | +0.02 | −0.07 |
| Seed 23, eighth, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −1.36 | −1.47 | +0.10 | −0.08 | +3.02 | +2.87 | +2.67 | +2.42 |
| Seed 23, eighth, mirrored | 24 | Without replay (H-47, 021's job) | without replay | +0.83 | +0.68 | +0.33 | +0.17 | +0.37 | +0.26 | +0.38 | +0.23 |
| Seed 23, first mapping, assignment | 6 | After block 6 alone | replays | −3.48 | −3.23 | −0.22 | −0.29 | +5.07 | +4.52 | +4.28 | +3.42 |
| Seed 23, first mapping, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −3.48 | −3.23 | −0.22 | −0.29 | +5.07 | +4.52 | +4.28 | +3.42 |
| Seed 23, first mapping, assignment | 6 | Without replay (H-47, 021's job) | without replay | −0.20 | −0.23 | −0.20 | −0.21 | −0.50 | −0.53 | −0.25 | −0.41 |
| Seed 23, first mapping, assignment | 12 | After block 6 alone | without replay | +0.18 | −0.01 | −0.04 | −0.14 | −0.85 | −0.76 | −0.58 | −0.55 |
| Seed 23, first mapping, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −2.71 | −2.55 | −0.14 | −0.19 | +3.26 | +3.06 | +2.77 | +2.41 |
| Seed 23, first mapping, assignment | 12 | Without replay (H-47, 021's job) | without replay | +0.36 | +0.12 | −0.04 | −0.07 | −0.66 | −0.63 | −0.46 | −0.44 |
| Seed 23, first mapping, assignment | 18 | After block 6 alone | without replay | +0.60 | −0.04 | +0.05 | −0.15 | −0.81 | −0.82 | −0.58 | −0.57 |
| Seed 23, first mapping, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −1.53 | −1.61 | +0.31 | −0.05 | +1.96 | +1.76 | +1.64 | +1.40 |
| Seed 23, first mapping, assignment | 18 | Without replay (H-47, 021's job) | without replay | +0.62 | +0.15 | +0.10 | −0.08 | −0.69 | −0.72 | −0.48 | −0.53 |
| Seed 23, first mapping, assignment | 24 | After block 6 alone | without replay | +0.74 | +0.21 | +0.13 | −0.03 | −0.86 | −0.85 | −0.61 | −0.67 |
| Seed 23, first mapping, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −1.10 | −1.22 | +0.24 | −0.09 | +1.33 | +1.27 | +1.19 | +0.84 |
| Seed 23, first mapping, assignment | 24 | Without replay (H-47, 021's job) | without replay | +0.69 | +0.49 | +0.14 | +0.04 | −0.72 | −0.69 | −0.46 | −0.61 |
| Seed 24, eighth, mirrored | 6 | After block 6 alone | replays | −3.90 | −4.11 | −0.56 | −0.45 | +4.88 | +4.60 | +4.62 | +3.60 |
| Seed 24, eighth, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −3.90 | −4.11 | −0.56 | −0.45 | +4.88 | +4.60 | +4.62 | +3.60 |
| Seed 24, eighth, mirrored | 6 | Without replay (H-47, 021's job) | without replay | −0.08 | −0.43 | −0.23 | −0.41 | +0.14 | +0.17 | +0.24 | +0.23 |
| Seed 24, eighth, mirrored | 12 | After block 6 alone | without replay | +0.12 | −0.04 | −0.17 | −0.29 | −0.29 | −0.26 | −0.19 | −0.05 |
| Seed 24, eighth, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −2.52 | −3.05 | −0.15 | −0.49 | +3.91 | +3.90 | +3.78 | +3.02 |
| Seed 24, eighth, mirrored | 12 | Without replay (H-47, 021's job) | without replay | +0.26 | −0.02 | −0.11 | −0.23 | +0.14 | +0.12 | +0.22 | +0.19 |
| Seed 24, eighth, mirrored | 18 | After block 6 alone | without replay | +0.39 | +0.23 | −0.09 | −0.14 | −0.17 | −0.11 | −0.09 | +0.02 |
| Seed 24, eighth, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −1.61 | −2.21 | −0.05 | −0.25 | +3.28 | +3.33 | +3.26 | +2.47 |
| Seed 24, eighth, mirrored | 18 | Without replay (H-47, 021's job) | without replay | +0.28 | +0.06 | −0.10 | −0.13 | +0.13 | +0.22 | +0.23 | +0.23 |
| Seed 24, eighth, mirrored | 24 | After block 6 alone | without replay | +0.23 | +0.30 | −0.08 | 0.00 | +0.01 | 0.00 | 0.00 | +0.15 |
| Seed 24, eighth, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −0.93 | −1.72 | +0.15 | −0.07 | +3.06 | +2.80 | +2.95 | +2.10 |
| Seed 24, eighth, mirrored | 24 | Without replay (H-47, 021's job) | without replay | +0.27 | +0.12 | +0.03 | −0.04 | +0.25 | +0.24 | +0.24 | +0.29 |
| Seed 24, first mapping, assignment | 6 | After block 6 alone | replays | −2.81 | −3.36 | −0.26 | −0.36 | +4.45 | +4.28 | +4.31 | +2.78 |
| Seed 24, first mapping, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −2.81 | −3.36 | −0.26 | −0.36 | +4.45 | +4.28 | +4.31 | +2.78 |
| Seed 24, first mapping, assignment | 6 | Without replay (H-47, 021's job) | without replay | +0.19 | +0.01 | −0.17 | −0.28 | −0.62 | −0.60 | −0.35 | −0.43 |
| Seed 24, first mapping, assignment | 12 | After block 6 alone | without replay | +0.19 | +0.01 | −0.16 | −0.34 | −0.84 | −0.79 | −0.58 | −0.55 |
| Seed 24, first mapping, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −2.41 | −2.58 | −0.23 | −0.45 | +3.05 | +2.88 | +3.04 | +1.94 |
| Seed 24, first mapping, assignment | 12 | Without replay (H-47, 021's job) | without replay | +0.25 | +0.27 | −0.14 | −0.22 | −0.70 | −0.63 | −0.47 | −0.45 |
| Seed 24, first mapping, assignment | 18 | After block 6 alone | without replay | +0.43 | +0.29 | −0.04 | −0.11 | −0.84 | −0.80 | −0.57 | −0.58 |
| Seed 24, first mapping, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −1.55 | −1.92 | +0.09 | −0.13 | +1.89 | +1.96 | +1.88 | +1.26 |
| Seed 24, first mapping, assignment | 18 | Without replay (H-47, 021's job) | without replay | +0.48 | +0.23 | −0.09 | −0.08 | −0.67 | −0.63 | −0.43 | −0.52 |
| Seed 24, first mapping, assignment | 24 | After block 6 alone | without replay | +0.59 | +0.35 | −0.04 | +0.03 | −0.80 | −0.79 | −0.61 | −0.54 |
| Seed 24, first mapping, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −0.77 | −1.66 | +0.14 | −0.14 | +1.14 | +1.42 | +1.25 | +0.80 |
| Seed 24, first mapping, assignment | 24 | Without replay (H-47, 021's job) | without replay | +0.41 | +0.40 | 0.00 | +0.07 | −0.68 | −0.64 | −0.49 | −0.46 |
| Seed 23, eighth, assignment | 6 | After block 6 alone | replays | −4.78 | −4.62 | −1.07 | −1.03 | +5.05 | +4.67 | +4.10 | +4.29 |
| Seed 23, eighth, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −4.78 | −4.62 | −1.07 | −1.03 | +5.05 | +4.67 | +4.10 | +4.29 |
| Seed 23, eighth, assignment | 6 | Without replay (H-47, 021's job) | without replay | −0.28 | −0.53 | −0.06 | −0.26 | +0.20 | +0.16 | +0.26 | +0.25 |
| Seed 23, eighth, assignment | 12 | After block 6 alone | without replay | +0.12 | −0.08 | 0.00 | −0.08 | −0.34 | −0.27 | −0.12 | −0.20 |
| Seed 23, eighth, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −3.49 | −2.92 | −0.70 | −0.58 | +4.10 | +3.74 | +3.22 | +3.53 |
| Seed 23, eighth, assignment | 12 | Without replay (H-47, 021's job) | without replay | +0.34 | −0.25 | +0.03 | −0.10 | +0.17 | +0.17 | +0.25 | +0.21 |
| Seed 23, eighth, assignment | 18 | After block 6 alone | without replay | +0.22 | +0.15 | +0.02 | −0.03 | −0.22 | −0.11 | −0.04 | −0.06 |
| Seed 23, eighth, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −2.25 | −2.07 | −0.26 | −0.35 | +3.38 | +3.10 | +2.56 | +3.04 |
| Seed 23, eighth, assignment | 18 | Without replay (H-47, 021's job) | without replay | +0.37 | +0.29 | +0.13 | 0.00 | +0.21 | +0.24 | +0.27 | +0.27 |
| Seed 23, eighth, assignment | 24 | After block 6 alone | without replay | +0.42 | +0.30 | +0.15 | +0.06 | −0.09 | −0.11 | +0.02 | −0.04 |
| Seed 23, eighth, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −1.50 | −1.40 | −0.16 | −0.15 | +2.99 | +2.56 | +2.19 | +2.61 |
| Seed 23, eighth, assignment | 24 | Without replay (H-47, 021's job) | without replay | +0.95 | +0.58 | +0.36 | +0.14 | +0.33 | +0.35 | +0.39 | +0.29 |
| Seed 24, first mapping, mirrored | 6 | After block 6 alone | replays | −3.58 | −4.21 | −0.67 | −1.11 | +4.12 | +3.81 | +3.25 | +3.14 |
| Seed 24, first mapping, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −3.58 | −4.21 | −0.67 | −1.11 | +4.12 | +3.81 | +3.25 | +3.14 |
| Seed 24, first mapping, mirrored | 6 | Without replay (H-47, 021's job) | without replay | +0.13 | −0.01 | −0.16 | −0.28 | −0.65 | −0.63 | −0.38 | −0.45 |
| Seed 24, first mapping, mirrored | 12 | After block 6 alone | without replay | −0.02 | +0.09 | −0.20 | −0.30 | −0.82 | −0.85 | −0.58 | −0.58 |
| Seed 24, first mapping, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −2.83 | −2.86 | −0.51 | −0.70 | +2.58 | +2.37 | +2.02 | +2.07 |
| Seed 24, first mapping, mirrored | 12 | Without replay (H-47, 021's job) | without replay | +0.26 | +0.25 | −0.15 | −0.23 | −0.70 | −0.66 | −0.48 | −0.46 |
| Seed 24, first mapping, mirrored | 18 | After block 6 alone | without replay | +0.44 | +0.24 | −0.10 | −0.14 | −0.78 | −0.82 | −0.53 | −0.61 |
| Seed 24, first mapping, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −1.89 | −2.14 | −0.16 | −0.44 | +1.81 | +1.74 | +1.38 | +1.52 |
| Seed 24, first mapping, mirrored | 18 | Without replay (H-47, 021's job) | without replay | +0.50 | +0.22 | −0.09 | −0.10 | −0.67 | −0.63 | −0.43 | −0.50 |
| Seed 24, first mapping, mirrored | 24 | After block 6 alone | without replay | +0.36 | +0.20 | −0.06 | −0.07 | −0.77 | −0.81 | −0.56 | −0.59 |
| Seed 24, first mapping, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −1.57 | −1.51 | −0.14 | −0.25 | +1.26 | +1.41 | +1.07 | +1.09 |
| Seed 24, first mapping, mirrored | 24 | Without replay (H-47, 021's job) | without replay | +0.40 | +0.33 | +0.02 | +0.03 | −0.69 | −0.64 | −0.47 | −0.50 |
| Seed 23, first mapping, mirrored | 6 | After block 6 alone | replays | −3.99 | −3.68 | −0.95 | −0.76 | +4.74 | +4.10 | +3.55 | +3.56 |
| Seed 23, first mapping, mirrored | 6 | Plain on A's (H-47, 020's job) | replays | −3.99 | −3.68 | −0.95 | −0.76 | +4.74 | +4.10 | +3.55 | +3.56 |
| Seed 23, first mapping, mirrored | 6 | Without replay (H-47, 020's job) | without replay | −0.16 | −0.29 | −0.17 | −0.23 | −0.61 | −0.58 | −0.37 | −0.46 |
| Seed 23, first mapping, mirrored | 12 | After block 6 alone | without replay | +0.17 | +0.17 | +0.03 | −0.04 | −0.90 | −0.81 | −0.62 | −0.58 |
| Seed 23, first mapping, mirrored | 12 | Plain on A's (H-47, 020's job) | replays | −3.00 | −2.80 | −0.58 | −0.62 | +3.14 | +2.67 | +2.31 | +2.45 |
| Seed 23, first mapping, mirrored | 12 | Without replay (H-47, 020's job) | without replay | +0.35 | +0.03 | +0.03 | −0.08 | −0.74 | −0.66 | −0.51 | −0.48 |
| Seed 23, first mapping, mirrored | 18 | After block 6 alone | without replay | +0.36 | +0.21 | +0.04 | 0.00 | −0.83 | −0.86 | −0.59 | −0.62 |
| Seed 23, first mapping, mirrored | 18 | Plain on A's (H-47, 020's job) | replays | −1.95 | −1.93 | −0.18 | −0.17 | +2.05 | +1.75 | +1.49 | +1.62 |
| Seed 23, first mapping, mirrored | 18 | Without replay (H-47, 020's job) | without replay | +0.60 | +0.04 | +0.15 | −0.07 | −0.68 | −0.74 | −0.47 | −0.53 |
| Seed 23, first mapping, mirrored | 24 | After block 6 alone | without replay | +0.42 | +0.27 | +0.10 | +0.10 | −0.80 | −0.83 | −0.58 | −0.60 |
| Seed 23, first mapping, mirrored | 24 | Plain on A's (H-47, 020's job) | replays | −1.53 | −1.31 | −0.11 | −0.02 | +1.38 | +1.24 | +0.98 | +1.13 |
| Seed 23, first mapping, mirrored | 24 | Without replay (H-47, 020's job) | without replay | +0.68 | +0.36 | +0.16 | +0.11 | −0.67 | −0.66 | −0.44 | −0.57 |
| Seed 24, eighth, assignment | 6 | After block 6 alone | replays | −4.33 | −4.90 | −0.79 | −1.27 | +4.42 | +4.38 | +3.72 | +3.79 |
| Seed 24, eighth, assignment | 6 | Plain on A's (H-47, 020's job) | replays | −4.33 | −4.90 | −0.79 | −1.27 | +4.42 | +4.38 | +3.72 | +3.79 |
| Seed 24, eighth, assignment | 6 | Without replay (H-47, 020's job) | without replay | −0.07 | −0.32 | −0.21 | −0.39 | +0.14 | +0.17 | +0.26 | +0.21 |
| Seed 24, eighth, assignment | 12 | After block 6 alone | without replay | +0.19 | −0.04 | −0.10 | −0.24 | −0.27 | −0.26 | −0.10 | −0.13 |
| Seed 24, eighth, assignment | 12 | Plain on A's (H-47, 020's job) | replays | −3.06 | −3.55 | −0.50 | −0.80 | +3.63 | +3.54 | +3.07 | +3.10 |
| Seed 24, eighth, assignment | 12 | Without replay (H-47, 020's job) | without replay | +0.16 | −0.01 | −0.17 | −0.19 | +0.12 | +0.13 | +0.22 | +0.18 |
| Seed 24, eighth, assignment | 18 | After block 6 alone | without replay | +0.30 | +0.30 | −0.11 | −0.13 | −0.18 | −0.13 | −0.02 | −0.08 |
| Seed 24, eighth, assignment | 18 | Plain on A's (H-47, 020's job) | replays | −2.16 | −2.24 | −0.24 | −0.39 | +2.91 | +3.02 | +2.47 | +2.62 |
| Seed 24, eighth, assignment | 18 | Without replay (H-47, 020's job) | without replay | +0.29 | +0.17 | −0.12 | −0.09 | +0.12 | +0.20 | +0.24 | +0.17 |
| Seed 24, eighth, assignment | 24 | After block 6 alone | without replay | +0.45 | +0.17 | −0.05 | −0.10 | −0.06 | −0.02 | +0.05 | +0.02 |
| Seed 24, eighth, assignment | 24 | Plain on A's (H-47, 020's job) | replays | −1.69 | −1.64 | −0.25 | −0.30 | +2.58 | +2.58 | +2.12 | +2.24 |
| Seed 24, eighth, assignment | 24 | Without replay (H-47, 020's job) | without replay | +0.31 | +0.05 | −0.03 | −0.03 | +0.23 | +0.21 | +0.27 | +0.22 |
| Seed 22, first mapping, mirrored | 6 | After block 6 alone | replays | −3.57 | −3.94 | −1.01 | −0.92 | +3.72 | +3.63 | +2.73 | +3.02 |
| Seed 22, first mapping, mirrored | 6 | Plain on A's (H-47, 020's job) | replays | −3.57 | −3.94 | −1.01 | −0.92 | +3.72 | +3.63 | +2.73 | +3.02 |
| Seed 22, first mapping, mirrored | 6 | Without replay (H-47, 020's job) | without replay | −0.29 | −0.18 | −0.32 | −0.28 | −0.67 | −0.65 | −0.45 | −0.51 |
| Seed 22, first mapping, mirrored | 12 | After block 6 alone | without replay | +0.12 | −0.07 | −0.19 | −0.24 | −0.86 | −0.80 | −0.57 | −0.64 |
| Seed 22, first mapping, mirrored | 12 | Plain on A's (H-47, 020's job) | replays | −2.68 | −3.06 | −0.83 | −0.77 | +2.31 | +2.44 | +1.91 | +1.89 |
| Seed 22, first mapping, mirrored | 12 | Without replay (H-47, 020's job) | without replay | +0.15 | −0.09 | −0.11 | −0.22 | −0.72 | −0.67 | −0.49 | −0.55 |
| Seed 22, first mapping, mirrored | 18 | After block 6 alone | without replay | +0.30 | +0.11 | 0.00 | −0.16 | −0.83 | −0.81 | −0.57 | −0.70 |
| Seed 22, first mapping, mirrored | 18 | Plain on A's (H-47, 020's job) | replays | −1.63 | −2.18 | −0.28 | −0.34 | +1.60 | +1.61 | +1.14 | +1.34 |
| Seed 22, first mapping, mirrored | 18 | Without replay (H-47, 020's job) | without replay | +0.31 | +0.06 | +0.02 | −0.14 | −0.75 | −0.64 | −0.47 | −0.62 |
| Seed 22, first mapping, mirrored | 24 | After block 6 alone | without replay | +0.25 | +0.08 | +0.05 | −0.14 | −0.85 | −0.71 | −0.52 | −0.64 |
| Seed 22, first mapping, mirrored | 24 | Plain on A's (H-47, 020's job) | replays | −1.32 | −1.61 | −0.29 | −0.22 | +1.06 | +1.18 | +0.74 | +0.97 |
| Seed 22, first mapping, mirrored | 24 | Without replay (H-47, 020's job) | without replay | +0.34 | +0.01 | +0.08 | −0.16 | −0.79 | −0.61 | −0.51 | −0.56 |
| Seed 22, eighth, assignment | 6 | After block 6 alone | replays | −4.47 | −4.64 | −1.19 | −1.22 | +4.05 | +4.30 | +3.51 | +3.58 |
| Seed 22, eighth, assignment | 6 | Plain on A's (H-47, 020's job) | replays | −4.47 | −4.64 | −1.19 | −1.22 | +4.05 | +4.30 | +3.51 | +3.58 |
| Seed 22, eighth, assignment | 6 | Without replay (H-47, 020's job) | without replay | −0.37 | −0.26 | −0.28 | −0.31 | +0.12 | +0.16 | +0.23 | +0.21 |
| Seed 22, eighth, assignment | 12 | After block 6 alone | without replay | +0.09 | −0.11 | −0.19 | −0.21 | −0.33 | −0.26 | −0.09 | −0.19 |
| Seed 22, eighth, assignment | 12 | Plain on A's (H-47, 020's job) | replays | −3.00 | −3.27 | −0.79 | −0.79 | +3.22 | +3.46 | +2.80 | +2.92 |
| Seed 22, eighth, assignment | 12 | Without replay (H-47, 020's job) | without replay | +0.02 | −0.15 | −0.14 | −0.20 | +0.07 | +0.16 | +0.23 | +0.14 |
| Seed 22, eighth, assignment | 18 | After block 6 alone | without replay | +0.13 | +0.01 | −0.10 | −0.18 | −0.18 | −0.08 | +0.02 | −0.10 |
| Seed 22, eighth, assignment | 18 | Plain on A's (H-47, 020's job) | replays | −1.96 | −2.30 | −0.36 | −0.38 | +2.73 | +3.00 | +2.33 | +2.58 |
| Seed 22, eighth, assignment | 18 | Without replay (H-47, 020's job) | without replay | +0.02 | −0.10 | −0.10 | −0.19 | +0.02 | +0.23 | +0.19 | +0.13 |
| Seed 22, eighth, assignment | 24 | After block 6 alone | without replay | +0.21 | +0.04 | −0.01 | −0.13 | −0.13 | −0.03 | +0.03 | −0.02 |
| Seed 22, eighth, assignment | 24 | Plain on A's (H-47, 020's job) | replays | −1.26 | −1.92 | −0.21 | −0.33 | +2.47 | +2.45 | +1.97 | +2.22 |
| Seed 22, eighth, assignment | 24 | Without replay (H-47, 020's job) | without replay | +0.21 | +0.02 | +0.01 | −0.11 | +0.06 | +0.21 | +0.23 | +0.12 |

- **The form's night after block 6 is H-47's first plain night on the same image**, and in every cell it moved what that night moved in
  exploration 020's or 021's job, the same lines and the same class matrices: the excitatory units of the group A answers lost 2.81 to
  4.90 M of weight onto all and the other group's 3.23 to 4.78 M, and the inhibition onto the excitatory units of A's group rose by 3.02 to
  4.71 M and onto the other group's by 2.73 to 4.17 M.
- **Its nights after blocks 12, 18 and 24 are nights without replay after a replayed night.** Over them the inhibition onto the excitatory
  units of A's group moved by −0.70 to +0.03 M, where over H-47's night without replay after the same blocks it moved by −0.62 to +0.38 M
  and over H-47's plain night by +0.97 to +3.80 M. In each of those 36 nights, three a cell, each of the table's four inhibitory columns
  came out lower than over H-47's night without replay after the same block on the same cell: the odd places' inhibitory units onto all by
  0.06 to 0.57 M, the even places' by 0.10 to 0.46 M, the inhibition onto the odd places' excitatory units by 0.02 to 0.41 M and onto the
  even places' by 0.03 to 0.41 M.
- **Over the same nights the excitatory units of A's group moved by −0.11 to +0.75 M onto all** under the form and by −0.25 to +0.83 M
  under H-47's night without replay, where H-47's later plain nights took 0.77 to 3.55 M from them.

What each stretch of blocks moved, from the image after the night before it to the image before the next night, or block 36's; the head,
blocks 1 to 6, is the same in every form of a cell, from the start image to the image before the first night:

| Cell | Stretch | Form | The odd places' excitatory onto all | the even places' | The odd places' onto their own | the even places' onto their own | The odd places' inhibitory onto all | the even places' | Inhibitory onto the odd places' excitatory | onto the even places' |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | blocks 1 to 6 | Every form | 0.00 | 0.00 | −0.13 | −0.20 | +0.26 | +0.33 | +0.30 | +0.50 |
| Seed 22, eighth, mirrored | blocks 7 to 12 | After block 6 alone | 0.00 | 0.00 | −0.13 | −0.11 | −0.63 | −0.79 | −0.48 | −0.48 |
| Seed 22, eighth, mirrored | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.13 | −0.11 | −0.63 | −0.79 | −0.48 | −0.48 |
| Seed 22, eighth, mirrored | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.13 | −0.12 | +0.22 | +0.15 | +0.21 | +0.21 |
| Seed 22, eighth, mirrored | blocks 13 to 18 | After block 6 alone | 0.00 | 0.00 | −0.03 | −0.01 | −0.30 | −0.36 | −0.23 | −0.24 |
| Seed 22, eighth, mirrored | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.04 | −0.02 | −0.68 | −0.85 | −0.53 | −0.54 |
| Seed 22, eighth, mirrored | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.03 | −0.01 | +0.17 | +0.16 | +0.19 | +0.21 |
| Seed 22, eighth, mirrored | blocks 19 to 24 | After block 6 alone | 0.00 | 0.00 | 0.00 | −0.02 | −0.17 | −0.19 | −0.17 | −0.11 |
| Seed 22, eighth, mirrored | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.02 | −0.74 | −0.89 | −0.59 | −0.62 |
| Seed 22, eighth, mirrored | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.01 | +0.22 | +0.15 | +0.19 | +0.19 |
| Seed 22, eighth, mirrored | blocks 25 to 36 | After block 6 alone | 0.00 | 0.00 | −0.03 | −0.03 | −0.35 | −0.30 | −0.30 | −0.04 |
| Seed 22, eighth, mirrored | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.06 | −1.37 | −1.72 | −1.14 | −1.05 |
| Seed 22, eighth, mirrored | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.04 | −0.03 | +0.27 | +0.38 | +0.27 | +0.45 |
| Seed 22, first mapping, assignment | blocks 1 to 6 | Every form | 0.00 | 0.00 | −0.18 | −0.15 | −0.81 | −0.86 | −0.73 | −0.58 |
| Seed 22, first mapping, assignment | blocks 7 to 12 | After block 6 alone | 0.00 | 0.00 | −0.11 | −0.17 | −1.15 | −1.32 | −0.98 | −0.99 |
| Seed 22, first mapping, assignment | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.11 | −0.17 | −1.15 | −1.32 | −0.98 | −0.99 |
| Seed 22, first mapping, assignment | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.08 | −0.16 | −0.79 | −0.98 | −0.79 | −0.78 |
| Seed 22, first mapping, assignment | blocks 13 to 18 | After block 6 alone | 0.00 | 0.00 | −0.01 | −0.02 | −1.15 | −1.32 | −1.04 | −1.10 |
| Seed 22, first mapping, assignment | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.03 | −1.41 | −1.59 | −1.21 | −1.22 |
| Seed 22, first mapping, assignment | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.02 | −0.99 | −1.16 | −0.94 | −1.01 |
| Seed 22, first mapping, assignment | blocks 19 to 24 | After block 6 alone | 0.00 | 0.00 | −0.01 | −0.04 | −1.22 | −1.37 | −1.09 | −1.18 |
| Seed 22, first mapping, assignment | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.03 | −0.03 | −1.49 | −1.70 | −1.30 | −1.30 |
| Seed 22, first mapping, assignment | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.01 | −0.05 | −1.08 | −1.09 | −0.98 | −1.01 |
| Seed 22, first mapping, assignment | blocks 25 to 36 | After block 6 alone | 0.00 | 0.00 | −0.01 | −0.03 | −2.59 | −2.69 | −2.17 | −2.31 |
| Seed 22, first mapping, assignment | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.04 | −0.06 | −2.96 | −3.26 | −2.45 | −2.51 |
| Seed 22, first mapping, assignment | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.06 | −2.36 | −2.19 | −1.93 | −2.06 |
| Seed 23, eighth, mirrored | blocks 1 to 6 | Every form | 0.00 | 0.00 | −0.14 | −0.06 | +0.28 | +0.27 | +0.27 | +0.49 |
| Seed 23, eighth, mirrored | blocks 7 to 12 | After block 6 alone | 0.00 | 0.00 | −0.06 | −0.05 | −0.70 | −0.64 | −0.49 | −0.38 |
| Seed 23, eighth, mirrored | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.06 | −0.05 | −0.70 | −0.64 | −0.49 | −0.38 |
| Seed 23, eighth, mirrored | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.05 | −0.07 | +0.23 | +0.09 | +0.18 | +0.26 |
| Seed 23, eighth, mirrored | blocks 13 to 18 | After block 6 alone | 0.00 | 0.00 | −0.01 | −0.01 | −0.32 | −0.41 | −0.31 | −0.13 |
| Seed 23, eighth, mirrored | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.02 | −0.76 | −0.71 | −0.53 | −0.46 |
| Seed 23, eighth, mirrored | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.02 | +0.33 | +0.09 | +0.18 | +0.28 |
| Seed 23, eighth, mirrored | blocks 19 to 24 | After block 6 alone | 0.00 | 0.00 | −0.01 | −0.03 | −0.18 | −0.25 | −0.14 | −0.11 |
| Seed 23, eighth, mirrored | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.01 | −0.85 | −0.72 | −0.56 | −0.52 |
| Seed 23, eighth, mirrored | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.03 | +0.45 | +0.23 | +0.32 | +0.29 |
| Seed 23, eighth, mirrored | blocks 25 to 36 | After block 6 alone | 0.00 | 0.00 | −0.03 | −0.04 | +0.10 | −0.16 | +0.05 | +0.09 |
| Seed 23, eighth, mirrored | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.04 | −0.03 | −1.69 | −1.37 | −1.12 | −0.95 |
| Seed 23, eighth, mirrored | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.03 | −0.02 | +0.99 | +0.72 | +0.82 | +0.77 |
| Seed 23, first mapping, assignment | blocks 1 to 6 | Every form | 0.00 | 0.00 | −0.10 | −0.02 | −0.71 | −0.78 | −0.62 | −0.51 |
| Seed 23, first mapping, assignment | blocks 7 to 12 | After block 6 alone | 0.00 | 0.00 | −0.15 | −0.08 | −1.16 | −1.15 | −0.88 | −0.86 |
| Seed 23, first mapping, assignment | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.15 | −0.08 | −1.16 | −1.15 | −0.88 | −0.86 |
| Seed 23, first mapping, assignment | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | +0.01 | −0.13 | −0.11 | −0.77 | −0.79 | −0.63 | −0.59 |
| Seed 23, first mapping, assignment | blocks 13 to 18 | After block 6 alone | 0.00 | 0.00 | −0.03 | −0.06 | −1.03 | −1.06 | −0.86 | −0.79 |
| Seed 23, first mapping, assignment | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.05 | −0.07 | −1.31 | −1.29 | −1.00 | −0.98 |
| Seed 23, first mapping, assignment | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.05 | −0.86 | −0.89 | −0.76 | −0.66 |
| Seed 23, first mapping, assignment | blocks 19 to 24 | After block 6 alone | 0.00 | 0.00 | −0.01 | −0.01 | −1.06 | −1.10 | −0.91 | −0.79 |
| Seed 23, first mapping, assignment | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.01 | −1.43 | −1.36 | −1.08 | −1.04 |
| Seed 23, first mapping, assignment | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | 0.00 | −0.02 | −0.95 | −0.99 | −0.84 | −0.75 |
| Seed 23, first mapping, assignment | blocks 25 to 36 | After block 6 alone | 0.00 | 0.00 | 0.00 | −0.03 | −2.29 | −2.43 | −1.86 | −1.89 |
| Seed 23, first mapping, assignment | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.04 | −0.04 | −2.88 | −2.69 | −2.14 | −2.12 |
| Seed 23, first mapping, assignment | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.03 | −2.01 | −2.13 | −1.64 | −1.78 |
| Seed 24, eighth, mirrored | blocks 1 to 6 | Every form | 0.00 | 0.00 | −0.24 | −0.13 | +0.24 | +0.33 | +0.39 | +0.35 |
| Seed 24, eighth, mirrored | blocks 7 to 12 | After block 6 alone | 0.00 | 0.00 | −0.05 | −0.02 | −0.87 | −0.86 | −0.76 | −0.47 |
| Seed 24, eighth, mirrored | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.05 | −0.02 | −0.87 | −0.86 | −0.76 | −0.47 |
| Seed 24, eighth, mirrored | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.04 | −0.03 | +0.12 | +0.03 | +0.07 | +0.17 |
| Seed 24, eighth, mirrored | blocks 13 to 18 | After block 6 alone | 0.00 | 0.00 | −0.02 | −0.02 | −0.47 | −0.46 | −0.48 | −0.20 |
| Seed 24, eighth, mirrored | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.02 | −0.92 | −0.94 | −0.78 | −0.56 |
| Seed 24, eighth, mirrored | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.02 | +0.04 | +0.04 | +0.02 | +0.17 |
| Seed 24, eighth, mirrored | blocks 19 to 24 | After block 6 alone | 0.00 | 0.00 | −0.01 | −0.01 | −0.30 | −0.25 | −0.36 | 0.00 |
| Seed 24, eighth, mirrored | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.01 | −0.95 | −0.97 | −0.84 | −0.54 |
| Seed 24, eighth, mirrored | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.01 | +0.05 | +0.13 | +0.03 | +0.22 |
| Seed 24, eighth, mirrored | blocks 25 to 36 | After block 6 alone | 0.00 | 0.00 | −0.03 | −0.04 | −0.29 | −0.28 | −0.42 | +0.20 |
| Seed 24, eighth, mirrored | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.05 | −0.01 | −1.69 | −1.79 | −1.56 | −0.92 |
| Seed 24, eighth, mirrored | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.03 | −0.03 | +0.20 | +0.28 | +0.14 | +0.49 |
| Seed 24, first mapping, assignment | blocks 1 to 6 | Every form | 0.00 | −0.01 | −0.22 | −0.10 | −0.82 | −0.82 | −0.70 | −0.55 |
| Seed 24, first mapping, assignment | blocks 7 to 12 | After block 6 alone | 0.00 | 0.00 | −0.07 | −0.08 | −1.29 | −1.20 | −1.06 | −0.84 |
| Seed 24, first mapping, assignment | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.07 | −0.08 | −1.29 | −1.20 | −1.06 | −0.84 |
| Seed 24, first mapping, assignment | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.05 | −0.08 | −0.93 | −0.85 | −0.80 | −0.64 |
| Seed 24, first mapping, assignment | blocks 13 to 18 | After block 6 alone | 0.00 | 0.00 | −0.03 | −0.06 | −1.19 | −1.12 | −1.01 | −0.81 |
| Seed 24, first mapping, assignment | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.04 | −0.07 | −1.46 | −1.36 | −1.18 | −0.97 |
| Seed 24, first mapping, assignment | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.06 | −1.03 | −0.90 | −0.87 | −0.72 |
| Seed 24, first mapping, assignment | blocks 19 to 24 | After block 6 alone | 0.00 | 0.00 | −0.01 | −0.01 | −1.25 | −1.10 | −1.08 | −0.81 |
| Seed 24, first mapping, assignment | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.05 | 0.00 | −1.62 | −1.40 | −1.26 | −1.05 |
| Seed 24, first mapping, assignment | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.02 | −1.09 | −0.90 | −0.94 | −0.73 |
| Seed 24, first mapping, assignment | blocks 25 to 36 | After block 6 alone | 0.00 | 0.00 | −0.01 | −0.01 | −2.64 | −2.18 | −2.15 | −1.72 |
| Seed 24, first mapping, assignment | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.06 | −0.02 | −3.17 | −2.77 | −2.48 | −2.11 |
| Seed 24, first mapping, assignment | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.02 | −2.17 | −1.83 | −1.84 | −1.46 |
| Seed 23, eighth, assignment | blocks 1 to 6 | Every form | 0.00 | 0.00 | +0.09 | +0.10 | +0.26 | +0.29 | +0.24 | +0.51 |
| Seed 23, eighth, assignment | blocks 7 to 12 | After block 6 alone | 0.00 | 0.00 | 0.00 | 0.00 | −0.79 | −0.84 | −0.64 | −0.51 |
| Seed 23, eighth, assignment | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | 0.00 | 0.00 | −0.79 | −0.84 | −0.64 | −0.51 |
| Seed 23, eighth, assignment | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | 0.00 | 0.00 | 0.00 | +0.07 | +0.05 | +0.11 | +0.14 |
| Seed 23, eighth, assignment | blocks 13 to 18 | After block 6 alone | 0.00 | −0.01 | +0.02 | −0.02 | −0.45 | −0.57 | −0.40 | −0.33 |
| Seed 23, eighth, assignment | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | +0.01 | 0.00 | −0.89 | −0.95 | −0.71 | −0.63 |
| Seed 23, eighth, assignment | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.02 | −0.01 | +0.07 | −0.01 | −0.01 | +0.17 |
| Seed 23, eighth, assignment | blocks 19 to 24 | After block 6 alone | 0.00 | 0.00 | +0.02 | 0.00 | −0.33 | −0.41 | −0.33 | −0.23 |
| Seed 23, eighth, assignment | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | +0.01 | 0.00 | −0.96 | −1.00 | −0.77 | −0.68 |
| Seed 23, eighth, assignment | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.02 | 0.00 | +0.21 | +0.14 | +0.08 | +0.25 |
| Seed 23, eighth, assignment | blocks 25 to 36 | After block 6 alone | 0.00 | 0.00 | +0.04 | −0.01 | −0.39 | −0.49 | −0.34 | −0.19 |
| Seed 23, eighth, assignment | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | +0.04 | 0.00 | −1.79 | −1.91 | −1.45 | −1.23 |
| Seed 23, eighth, assignment | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.04 | −0.01 | +0.52 | +0.23 | +0.35 | +0.43 |
| Seed 24, first mapping, mirrored | blocks 1 to 6 | Every form | 0.00 | 0.00 | +0.14 | +0.07 | −0.90 | −0.89 | −0.77 | −0.67 |
| Seed 24, first mapping, mirrored | blocks 7 to 12 | After block 6 alone | 0.00 | 0.00 | +0.06 | +0.04 | −1.35 | −1.39 | −1.16 | −0.99 |
| Seed 24, first mapping, mirrored | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | +0.06 | +0.04 | −1.35 | −1.39 | −1.16 | −0.99 |
| Seed 24, first mapping, mirrored | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.04 | +0.02 | −0.97 | −0.99 | −0.87 | −0.77 |
| Seed 24, first mapping, mirrored | blocks 13 to 18 | After block 6 alone | 0.00 | 0.00 | +0.04 | +0.01 | −1.30 | −1.36 | −1.13 | −1.02 |
| Seed 24, first mapping, mirrored | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.49 | −1.59 | −1.26 | −1.14 |
| Seed 24, first mapping, mirrored | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.03 | +0.01 | −1.13 | −1.16 | −1.01 | −0.89 |
| Seed 24, first mapping, mirrored | blocks 19 to 24 | After block 6 alone | 0.00 | 0.00 | +0.02 | 0.00 | −1.34 | −1.37 | −1.19 | −1.04 |
| Seed 24, first mapping, mirrored | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | +0.03 | +0.01 | −1.65 | −1.58 | −1.29 | −1.23 |
| Seed 24, first mapping, mirrored | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.17 | −1.14 | −1.02 | −0.94 |
| Seed 24, first mapping, mirrored | blocks 25 to 36 | After block 6 alone | 0.00 | 0.00 | +0.02 | +0.01 | −2.55 | −2.53 | −2.12 | −2.03 |
| Seed 24, first mapping, mirrored | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | +0.03 | +0.05 | −3.08 | −3.02 | −2.39 | −2.33 |
| Seed 24, first mapping, mirrored | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.01 | +0.01 | −2.13 | −2.05 | −1.80 | −1.73 |
| Seed 23, first mapping, mirrored | blocks 1 to 6 | Every form | 0.00 | 0.00 | +0.10 | +0.13 | −0.77 | −0.92 | −0.73 | −0.62 |
| Seed 23, first mapping, mirrored | blocks 7 to 12 | After block 6 alone | 0.00 | −0.01 | +0.04 | 0.00 | −1.35 | −1.34 | −1.07 | −1.01 |
| Seed 23, first mapping, mirrored | blocks 7 to 12 | Plain on A's (H-47, 020's job) | 0.00 | −0.01 | +0.04 | 0.00 | −1.35 | −1.34 | −1.07 | −1.01 |
| Seed 23, first mapping, mirrored | blocks 7 to 12 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | +0.02 | −0.97 | −0.96 | −0.87 | −0.74 |
| Seed 23, first mapping, mirrored | blocks 13 to 18 | After block 6 alone | 0.00 | 0.00 | +0.05 | +0.01 | −1.26 | −1.18 | −1.00 | −0.97 |
| Seed 23, first mapping, mirrored | blocks 13 to 18 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.05 | +0.01 | −1.51 | −1.44 | −1.16 | −1.13 |
| Seed 23, first mapping, mirrored | blocks 13 to 18 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.07 | −1.03 | −0.90 | −0.85 |
| Seed 23, first mapping, mirrored | blocks 19 to 24 | After block 6 alone | 0.00 | 0.00 | +0.03 | −0.01 | −1.31 | −1.25 | −1.09 | −1.00 |
| Seed 23, first mapping, mirrored | blocks 19 to 24 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.64 | −1.54 | −1.24 | −1.20 |
| Seed 23, first mapping, mirrored | blocks 19 to 24 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.09 | −1.06 | −0.92 | −0.91 |
| Seed 23, first mapping, mirrored | blocks 25 to 36 | After block 6 alone | 0.00 | 0.00 | +0.02 | 0.00 | −2.73 | −2.54 | −2.17 | −2.12 |
| Seed 23, first mapping, mirrored | blocks 25 to 36 | Plain on A's (H-47, 020's job) | 0.00 | −0.01 | +0.02 | +0.03 | −3.31 | −3.10 | −2.52 | −2.46 |
| Seed 23, first mapping, mirrored | blocks 25 to 36 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.03 | 0.00 | −2.28 | −2.39 | −1.93 | −1.94 |
| Seed 24, eighth, assignment | blocks 1 to 6 | Every form | 0.00 | 0.00 | +0.17 | +0.10 | +0.31 | +0.30 | +0.43 | +0.39 |
| Seed 24, eighth, assignment | blocks 7 to 12 | After block 6 alone | 0.00 | 0.00 | +0.04 | 0.00 | −0.67 | −0.85 | −0.52 | −0.49 |
| Seed 24, eighth, assignment | blocks 7 to 12 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.04 | 0.00 | −0.67 | −0.85 | −0.52 | −0.49 |
| Seed 24, eighth, assignment | blocks 7 to 12 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.04 | 0.00 | +0.24 | +0.10 | +0.17 | +0.28 |
| Seed 24, eighth, assignment | blocks 13 to 18 | After block 6 alone | 0.00 | 0.00 | +0.02 | 0.00 | −0.36 | −0.47 | −0.37 | −0.22 |
| Seed 24, eighth, assignment | blocks 13 to 18 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.03 | +0.01 | −0.81 | −0.92 | −0.65 | −0.56 |
| Seed 24, eighth, assignment | blocks 13 to 18 | Without replay (H-47, 020's job) | −0.01 | 0.00 | +0.03 | 0.00 | +0.16 | +0.09 | +0.09 | +0.25 |
| Seed 24, eighth, assignment | blocks 19 to 24 | After block 6 alone | 0.00 | 0.00 | +0.05 | −0.01 | −0.20 | −0.20 | −0.18 | −0.04 |
| Seed 24, eighth, assignment | blocks 19 to 24 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −0.83 | −0.97 | −0.65 | −0.63 |
| Seed 24, eighth, assignment | blocks 19 to 24 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.04 | 0.00 | +0.21 | +0.20 | +0.15 | +0.31 |
| Seed 24, eighth, assignment | blocks 25 to 36 | After block 6 alone | 0.00 | 0.00 | +0.06 | +0.01 | −0.19 | −0.27 | −0.13 | +0.01 |
| Seed 24, eighth, assignment | blocks 25 to 36 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.05 | 0.00 | −1.53 | −1.79 | −1.17 | −1.14 |
| Seed 24, eighth, assignment | blocks 25 to 36 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.07 | +0.01 | +0.31 | +0.37 | +0.27 | +0.45 |
| Seed 22, first mapping, mirrored | blocks 1 to 6 | Every form | 0.00 | 0.00 | +0.21 | +0.16 | −0.89 | −0.98 | −0.79 | −0.74 |
| Seed 22, first mapping, mirrored | blocks 7 to 12 | After block 6 alone | 0.00 | 0.00 | +0.12 | 0.00 | −1.20 | −1.39 | −1.02 | −1.03 |
| Seed 22, first mapping, mirrored | blocks 7 to 12 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.12 | 0.00 | −1.20 | −1.39 | −1.02 | −1.03 |
| Seed 22, first mapping, mirrored | blocks 7 to 12 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.09 | −0.01 | −0.83 | −1.13 | −0.86 | −0.86 |
| Seed 22, first mapping, mirrored | blocks 13 to 18 | After block 6 alone | 0.00 | 0.00 | +0.03 | 0.00 | −1.13 | −1.35 | −1.00 | −1.09 |
| Seed 22, first mapping, mirrored | blocks 13 to 18 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.07 | −0.02 | −1.38 | −1.57 | −1.14 | −1.20 |
| Seed 22, first mapping, mirrored | blocks 13 to 18 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −0.95 | −1.23 | −0.94 | −1.03 |
| Seed 22, first mapping, mirrored | blocks 19 to 24 | After block 6 alone | 0.00 | 0.00 | +0.03 | 0.00 | −1.17 | −1.38 | −1.01 | −1.14 |
| Seed 22, first mapping, mirrored | blocks 19 to 24 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.02 | +0.01 | −1.43 | −1.60 | −1.17 | −1.21 |
| Seed 22, first mapping, mirrored | blocks 19 to 24 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.06 | −1.21 | −0.95 | −1.04 |
| Seed 22, first mapping, mirrored | blocks 25 to 36 | After block 6 alone | 0.00 | 0.00 | +0.02 | +0.01 | −2.57 | −2.68 | −2.10 | −2.34 |
| Seed 22, first mapping, mirrored | blocks 25 to 36 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.03 | +0.01 | −2.86 | −3.19 | −2.30 | −2.42 |
| Seed 22, first mapping, mirrored | blocks 25 to 36 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.01 | +0.01 | −2.33 | −2.35 | −1.95 | −2.07 |
| Seed 22, eighth, assignment | blocks 1 to 6 | Every form | 0.00 | 0.00 | +0.23 | +0.16 | +0.22 | +0.33 | +0.30 | +0.46 |
| Seed 22, eighth, assignment | blocks 7 to 12 | After block 6 alone | 0.00 | 0.00 | +0.03 | 0.00 | −0.70 | −0.85 | −0.50 | −0.56 |
| Seed 22, eighth, assignment | blocks 7 to 12 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.03 | 0.00 | −0.70 | −0.85 | −0.50 | −0.56 |
| Seed 22, eighth, assignment | blocks 7 to 12 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | +0.01 | +0.07 | +0.05 | +0.09 | +0.10 |
| Seed 22, eighth, assignment | blocks 13 to 18 | After block 6 alone | 0.00 | 0.00 | +0.01 | −0.01 | −0.35 | −0.42 | −0.29 | −0.28 |
| Seed 22, eighth, assignment | blocks 13 to 18 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −0.77 | −0.87 | −0.55 | −0.59 |
| Seed 22, eighth, assignment | blocks 13 to 18 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.01 | 0.00 | +0.06 | +0.10 | +0.05 | +0.14 |
| Seed 22, eighth, assignment | blocks 19 to 24 | After block 6 alone | 0.00 | 0.00 | +0.01 | 0.00 | −0.21 | −0.26 | −0.21 | −0.17 |
| Seed 22, eighth, assignment | blocks 19 to 24 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.01 | +0.01 | −0.76 | −0.93 | −0.56 | −0.67 |
| Seed 22, eighth, assignment | blocks 19 to 24 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.01 | +0.01 | +0.09 | +0.08 | +0.02 | +0.14 |
| Seed 22, eighth, assignment | blocks 25 to 36 | After block 6 alone | 0.00 | 0.00 | +0.05 | +0.01 | −0.33 | −0.31 | −0.24 | −0.16 |
| Seed 22, eighth, assignment | blocks 25 to 36 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.05 | +0.01 | −1.40 | −1.70 | −1.00 | −1.18 |
| Seed 22, eighth, assignment | blocks 25 to 36 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.03 | +0.01 | +0.10 | +0.17 | +0.10 | +0.27 |

- **The blocks move no excitatory weight of the readouts' units onto all**: at most 0.01 M either way in any stretch of any form in any
  cell, as exploration 030 read on its cell.
- **The form's blocks 7 to 12 are H-47's plain night's** in every cell. Over blocks 13 to 18, 19 to 24 and 25 to 36, each of the table's
  four inhibitory columns came out lower under the form than under H-47's night without replay in every stretch of every cell, by 0.05 to
  0.91 M.
- **The inhibition onto the excitatory units of A's group fell in every stretch after the first night under H-47's plain night, in every
  cell**, and under the form in every stretch but blocks 25 to 36 on seed 23's eighth image from the mirrored assignment, +0.05 M, and on
  seed 24's eighth image from the assignment, +0.01 M. Under H-47's night without replay it rose in each of those four stretches on the six
  eighth images and fell in each on the six first mappings.

C's own units onto A's answer, the margin exploration 020 reads: their weight onto the readout A answers on the cell less onto the other.
Each night and each stretch, the mean over the three paths, in M; "the four nights" sums the nights and "blocks 7 to 36" the four stretches
after the first night, each sum of the unrounded figures, so the rounded figures beside it may add to 0.01 more or less:

| Cell | Form | blocks 1 to 6 | night after 6 | blocks 7 to 12 | night after 12 | blocks 13 to 18 | night after 18 | blocks 19 to 24 | night after 24 | blocks 25 to 36 | The four nights | Blocks 7 to 36 |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | After block 6 alone | −0.38 | +0.10 | −0.37 | −0.06 | −0.09 | +0.01 | −0.01 | 0.00 | −0.08 | +0.05 | −0.55 |
| Seed 22, eighth, mirrored | Plain on A's (H-47, 021's job) | −0.38 | +0.10 | −0.37 | +0.02 | −0.10 | +0.08 | −0.02 | +0.09 | −0.04 | +0.30 | −0.53 |
| Seed 22, eighth, mirrored | Without replay (H-47, 021's job) | −0.38 | −0.02 | −0.37 | +0.01 | −0.08 | +0.05 | −0.06 | +0.01 | −0.12 | +0.04 | −0.64 |
| Seed 22, first mapping, assignment | After block 6 alone | −0.52 | +0.14 | −0.30 | −0.04 | −0.03 | +0.01 | −0.02 | +0.01 | −0.03 | +0.11 | −0.38 |
| Seed 22, first mapping, assignment | Plain on A's (H-47, 021's job) | −0.52 | +0.14 | −0.30 | +0.10 | −0.03 | +0.11 | −0.07 | +0.07 | −0.09 | +0.42 | −0.49 |
| Seed 22, first mapping, assignment | Without replay (H-47, 021's job) | −0.52 | −0.06 | −0.24 | 0.00 | −0.04 | +0.01 | +0.02 | +0.01 | −0.03 | −0.05 | −0.28 |
| Seed 23, eighth, mirrored | After block 6 alone | −0.55 | +0.15 | −0.21 | −0.04 | −0.02 | −0.04 | −0.03 | +0.01 | −0.12 | +0.08 | −0.38 |
| Seed 23, eighth, mirrored | Plain on A's (H-47, 021's job) | −0.55 | +0.15 | −0.21 | +0.11 | −0.06 | +0.05 | −0.08 | +0.08 | −0.15 | +0.39 | −0.49 |
| Seed 23, eighth, mirrored | Without replay (H-47, 021's job) | −0.55 | −0.02 | −0.17 | −0.01 | −0.05 | 0.00 | −0.03 | −0.01 | −0.11 | −0.05 | −0.37 |
| Seed 23, first mapping, assignment | After block 6 alone | −0.48 | +0.12 | −0.50 | −0.06 | −0.11 | −0.04 | −0.03 | −0.02 | −0.02 | +0.01 | −0.66 |
| Seed 23, first mapping, assignment | Plain on A's (H-47, 021's job) | −0.48 | +0.12 | −0.50 | +0.17 | −0.15 | +0.10 | −0.07 | +0.11 | −0.16 | +0.51 | −0.88 |
| Seed 23, first mapping, assignment | Without replay (H-47, 021's job) | −0.48 | −0.05 | −0.44 | −0.03 | −0.05 | −0.03 | 0.00 | −0.02 | −0.05 | −0.13 | −0.53 |
| Seed 24, eighth, mirrored | After block 6 alone | −0.72 | +0.14 | −0.13 | 0.00 | −0.06 | 0.00 | −0.02 | +0.01 | −0.08 | +0.15 | −0.29 |
| Seed 24, eighth, mirrored | Plain on A's (H-47, 021's job) | −0.72 | +0.14 | −0.13 | +0.19 | −0.05 | +0.11 | −0.02 | +0.13 | −0.15 | +0.57 | −0.35 |
| Seed 24, eighth, mirrored | Without replay (H-47, 021's job) | −0.72 | −0.06 | −0.11 | 0.00 | −0.07 | +0.01 | −0.05 | +0.03 | −0.08 | −0.02 | −0.31 |
| Seed 24, first mapping, assignment | After block 6 alone | −0.66 | +0.16 | −0.20 | −0.01 | −0.09 | 0.00 | −0.04 | −0.02 | −0.04 | +0.13 | −0.37 |
| Seed 24, first mapping, assignment | Plain on A's (H-47, 021's job) | −0.66 | +0.16 | −0.20 | +0.17 | −0.13 | +0.18 | −0.13 | +0.18 | −0.15 | +0.68 | −0.61 |
| Seed 24, first mapping, assignment | Without replay (H-47, 021's job) | −0.66 | −0.08 | −0.17 | 0.00 | −0.07 | +0.01 | −0.05 | +0.04 | −0.03 | −0.03 | −0.32 |
| Seed 23, eighth, assignment | After block 6 alone | −0.45 | −0.11 | −0.30 | −0.02 | −0.13 | −0.04 | −0.06 | −0.05 | −0.15 | −0.22 | −0.64 |
| Seed 23, eighth, assignment | Plain on A's (H-47, 021's job) | −0.45 | −0.11 | −0.30 | −0.05 | −0.09 | −0.07 | −0.07 | −0.09 | −0.12 | −0.32 | −0.57 |
| Seed 23, eighth, assignment | Without replay (H-47, 021's job) | −0.45 | −0.09 | −0.37 | −0.03 | −0.10 | −0.03 | −0.05 | −0.03 | −0.14 | −0.17 | −0.65 |
| Seed 24, first mapping, mirrored | After block 6 alone | −0.46 | +0.03 | −0.25 | −0.01 | −0.12 | −0.03 | −0.04 | −0.03 | −0.04 | −0.03 | −0.45 |
| Seed 24, first mapping, mirrored | Plain on A's (H-47, 021's job) | −0.46 | +0.03 | −0.25 | +0.05 | −0.09 | −0.01 | −0.08 | −0.02 | −0.07 | +0.05 | −0.48 |
| Seed 24, first mapping, mirrored | Without replay (H-47, 021's job) | −0.46 | +0.03 | −0.20 | −0.02 | −0.11 | −0.06 | −0.05 | −0.07 | −0.03 | −0.12 | −0.39 |
| Seed 23, first mapping, mirrored | After block 6 alone | −0.54 | −0.14 | −0.39 | −0.05 | −0.19 | −0.06 | −0.09 | −0.04 | −0.08 | −0.28 | −0.75 |
| Seed 23, first mapping, mirrored | Plain on A's (H-47, 020's job) | −0.54 | −0.14 | −0.39 | −0.11 | −0.19 | −0.04 | −0.09 | −0.05 | −0.10 | −0.34 | −0.77 |
| Seed 23, first mapping, mirrored | Without replay (H-47, 020's job) | −0.54 | −0.04 | −0.36 | −0.04 | −0.13 | −0.05 | −0.07 | −0.04 | −0.12 | −0.16 | −0.69 |
| Seed 24, eighth, assignment | After block 6 alone | −0.48 | +0.07 | −0.12 | +0.02 | −0.08 | 0.00 | −0.12 | +0.03 | −0.17 | +0.13 | −0.48 |
| Seed 24, eighth, assignment | Plain on A's (H-47, 020's job) | −0.48 | +0.07 | −0.12 | +0.07 | −0.06 | −0.04 | −0.05 | −0.01 | −0.12 | +0.09 | −0.35 |
| Seed 24, eighth, assignment | Without replay (H-47, 020's job) | −0.48 | +0.01 | −0.13 | −0.02 | −0.08 | −0.02 | −0.10 | +0.03 | −0.16 | 0.00 | −0.47 |
| Seed 22, first mapping, mirrored | After block 6 alone | −0.60 | +0.06 | −0.32 | −0.04 | −0.07 | −0.04 | −0.05 | −0.02 | −0.04 | −0.04 | −0.47 |
| Seed 22, first mapping, mirrored | Plain on A's (H-47, 020's job) | −0.60 | +0.06 | −0.32 | +0.12 | −0.15 | +0.02 | −0.03 | +0.01 | −0.07 | +0.21 | −0.57 |
| Seed 22, first mapping, mirrored | Without replay (H-47, 020's job) | −0.60 | −0.06 | −0.23 | −0.05 | −0.06 | −0.02 | −0.04 | +0.02 | −0.02 | −0.11 | −0.35 |
| Seed 22, eighth, assignment | After block 6 alone | −0.61 | +0.05 | −0.07 | 0.00 | −0.04 | +0.01 | −0.03 | +0.04 | −0.11 | +0.09 | −0.25 |
| Seed 22, eighth, assignment | Plain on A's (H-47, 020's job) | −0.61 | +0.05 | −0.07 | +0.12 | −0.06 | +0.05 | −0.02 | +0.06 | −0.12 | +0.28 | −0.27 |
| Seed 22, eighth, assignment | Without replay (H-47, 020's job) | −0.61 | −0.05 | −0.04 | 0.00 | −0.03 | +0.01 | −0.03 | +0.04 | −0.08 | 0.00 | −0.18 |

D's own units onto B's answer under A less onto the other readout:

| Cell | Form | blocks 1 to 6 | night after 6 | blocks 7 to 12 | night after 12 | blocks 13 to 18 | night after 18 | blocks 19 to 24 | night after 24 | blocks 25 to 36 | The four nights | Blocks 7 to 36 |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | After block 6 alone | −0.61 | +0.16 | −0.32 | −0.01 | −0.03 | +0.03 | −0.06 | +0.02 | −0.06 | +0.20 | −0.47 |
| Seed 22, eighth, mirrored | Plain on A's (H-47, 021's job) | −0.61 | +0.16 | −0.32 | +0.16 | −0.06 | +0.12 | −0.06 | +0.16 | −0.15 | +0.60 | −0.58 |
| Seed 22, eighth, mirrored | Without replay (H-47, 021's job) | −0.61 | 0.00 | −0.32 | +0.03 | −0.02 | +0.02 | −0.03 | +0.01 | −0.09 | +0.05 | −0.46 |
| Seed 22, first mapping, assignment | After block 6 alone | −0.48 | +0.16 | −0.47 | −0.04 | −0.06 | 0.00 | −0.09 | +0.02 | −0.09 | +0.15 | −0.72 |
| Seed 22, first mapping, assignment | Plain on A's (H-47, 021's job) | −0.48 | +0.16 | −0.47 | +0.12 | −0.08 | +0.11 | −0.08 | +0.07 | −0.16 | +0.46 | −0.79 |
| Seed 22, first mapping, assignment | Without replay (H-47, 021's job) | −0.48 | 0.00 | −0.45 | −0.01 | −0.06 | +0.05 | −0.12 | +0.01 | −0.16 | +0.05 | −0.78 |
| Seed 23, eighth, mirrored | After block 6 alone | −0.59 | +0.20 | −0.21 | +0.02 | −0.07 | +0.02 | −0.10 | +0.01 | −0.14 | +0.25 | −0.51 |
| Seed 23, eighth, mirrored | Plain on A's (H-47, 021's job) | −0.59 | +0.20 | −0.21 | +0.18 | −0.10 | +0.09 | −0.06 | +0.12 | −0.10 | +0.60 | −0.46 |
| Seed 23, eighth, mirrored | Without replay (H-47, 021's job) | −0.59 | +0.01 | −0.27 | +0.02 | −0.08 | +0.02 | −0.10 | +0.03 | −0.09 | +0.09 | −0.54 |
| Seed 23, first mapping, assignment | After block 6 alone | −0.49 | +0.21 | −0.37 | 0.00 | −0.21 | −0.03 | −0.03 | −0.01 | −0.10 | +0.17 | −0.71 |
| Seed 23, first mapping, assignment | Plain on A's (H-47, 021's job) | −0.49 | +0.21 | −0.37 | +0.18 | −0.24 | +0.10 | −0.06 | +0.08 | −0.14 | +0.57 | −0.81 |
| Seed 23, first mapping, assignment | Without replay (H-47, 021's job) | −0.49 | +0.02 | −0.43 | +0.01 | −0.15 | +0.01 | −0.07 | +0.02 | −0.09 | +0.06 | −0.74 |
| Seed 24, eighth, mirrored | After block 6 alone | −0.53 | +0.16 | −0.09 | +0.02 | −0.08 | +0.02 | −0.04 | +0.02 | −0.11 | +0.22 | −0.32 |
| Seed 24, eighth, mirrored | Plain on A's (H-47, 021's job) | −0.53 | +0.16 | −0.09 | +0.09 | −0.06 | +0.04 | −0.04 | +0.01 | −0.05 | +0.30 | −0.24 |
| Seed 24, eighth, mirrored | Without replay (H-47, 021's job) | −0.53 | +0.02 | −0.12 | +0.03 | −0.09 | +0.03 | −0.04 | +0.01 | −0.10 | +0.09 | −0.36 |
| Seed 24, first mapping, assignment | After block 6 alone | −0.41 | +0.11 | −0.34 | −0.02 | −0.21 | −0.01 | −0.03 | −0.01 | −0.05 | +0.06 | −0.63 |
| Seed 24, first mapping, assignment | Plain on A's (H-47, 021's job) | −0.41 | +0.11 | −0.34 | +0.05 | −0.23 | +0.04 | −0.02 | +0.03 | −0.06 | +0.23 | −0.65 |
| Seed 24, first mapping, assignment | Without replay (H-47, 021's job) | −0.41 | +0.04 | −0.32 | 0.00 | −0.22 | +0.01 | −0.08 | +0.01 | −0.08 | +0.06 | −0.71 |
| Seed 23, eighth, assignment | After block 6 alone | −0.83 | −0.03 | −0.08 | 0.00 | +0.10 | +0.02 | +0.03 | −0.04 | +0.09 | −0.05 | +0.15 |
| Seed 23, eighth, assignment | Plain on A's (H-47, 021's job) | −0.83 | −0.03 | −0.08 | −0.04 | +0.07 | +0.01 | +0.05 | −0.10 | +0.01 | −0.17 | +0.05 |
| Seed 23, eighth, assignment | Without replay (H-47, 021's job) | −0.83 | +0.01 | −0.02 | +0.01 | +0.09 | +0.04 | +0.08 | −0.05 | +0.09 | 0.00 | +0.24 |
| Seed 24, first mapping, mirrored | After block 6 alone | −0.58 | −0.04 | −0.18 | −0.05 | −0.01 | −0.03 | 0.00 | −0.04 | −0.04 | −0.15 | −0.23 |
| Seed 24, first mapping, mirrored | Plain on A's (H-47, 021's job) | −0.58 | −0.04 | −0.18 | −0.03 | 0.00 | +0.03 | −0.04 | −0.04 | −0.15 | −0.09 | −0.37 |
| Seed 24, first mapping, mirrored | Without replay (H-47, 021's job) | −0.58 | −0.05 | −0.14 | −0.03 | +0.01 | −0.06 | −0.02 | −0.05 | −0.05 | −0.19 | −0.20 |
| Seed 23, first mapping, mirrored | After block 6 alone | −0.82 | −0.02 | −0.08 | +0.02 | −0.05 | +0.01 | +0.05 | −0.02 | −0.01 | −0.01 | −0.09 |
| Seed 23, first mapping, mirrored | Plain on A's (H-47, 020's job) | −0.82 | −0.02 | −0.08 | −0.01 | −0.05 | −0.03 | −0.01 | 0.00 | −0.12 | −0.06 | −0.26 |
| Seed 23, first mapping, mirrored | Without replay (H-47, 020's job) | −0.82 | +0.03 | −0.09 | +0.04 | +0.02 | 0.00 | −0.01 | −0.04 | +0.03 | +0.02 | −0.06 |
| Seed 24, eighth, assignment | After block 6 alone | −0.70 | −0.02 | −0.04 | −0.08 | −0.03 | −0.05 | +0.05 | −0.08 | −0.05 | −0.24 | −0.07 |
| Seed 24, eighth, assignment | Plain on A's (H-47, 020's job) | −0.70 | −0.02 | −0.04 | −0.06 | −0.04 | −0.02 | 0.00 | −0.05 | 0.00 | −0.16 | −0.08 |
| Seed 24, eighth, assignment | Without replay (H-47, 020's job) | −0.70 | −0.05 | −0.07 | −0.08 | −0.03 | −0.05 | +0.02 | −0.06 | −0.03 | −0.25 | −0.12 |
| Seed 22, first mapping, mirrored | After block 6 alone | −0.58 | −0.07 | −0.12 | −0.02 | −0.01 | −0.01 | −0.02 | +0.02 | −0.06 | −0.08 | −0.21 |
| Seed 22, first mapping, mirrored | Plain on A's (H-47, 020's job) | −0.58 | −0.07 | −0.12 | −0.04 | +0.05 | −0.06 | −0.06 | −0.02 | −0.07 | −0.19 | −0.20 |
| Seed 22, first mapping, mirrored | Without replay (H-47, 020's job) | −0.58 | −0.06 | −0.11 | −0.04 | −0.02 | −0.03 | −0.02 | 0.00 | −0.06 | −0.13 | −0.20 |
| Seed 22, eighth, assignment | After block 6 alone | −0.63 | −0.09 | −0.02 | −0.04 | +0.01 | −0.02 | −0.01 | −0.03 | −0.05 | −0.19 | −0.08 |
| Seed 22, eighth, assignment | Plain on A's (H-47, 020's job) | −0.63 | −0.09 | −0.02 | −0.05 | 0.00 | −0.06 | −0.03 | −0.01 | −0.05 | −0.22 | −0.10 |
| Seed 22, eighth, assignment | Without replay (H-47, 020's job) | −0.63 | −0.07 | −0.06 | −0.03 | −0.01 | −0.03 | −0.05 | 0.00 | −0.04 | −0.13 | −0.16 |

A's margin, the four kinds A alone, A and C, B alone and B and D, each onto the readout A asks of its stimulus less onto the other:

| Cell | Form | blocks 1 to 6 | night after 6 | blocks 7 to 12 | night after 12 | blocks 13 to 18 | night after 18 | blocks 19 to 24 | night after 24 | blocks 25 to 36 | The four nights | Blocks 7 to 36 |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | After block 6 alone | −2.16 | +1.12 | −0.84 | 0.00 | −0.09 | −0.13 | −0.05 | −0.10 | −0.15 | +0.89 | −1.14 |
| Seed 22, eighth, mirrored | Plain on A's (H-47, 021's job) | −2.16 | +1.12 | −0.84 | +1.08 | −0.15 | +0.78 | −0.09 | +0.61 | −0.16 | +3.60 | −1.25 |
| Seed 22, eighth, mirrored | Without replay (H-47, 021's job) | −2.16 | −0.12 | −0.75 | −0.01 | −0.10 | −0.03 | −0.09 | −0.07 | −0.24 | −0.23 | −1.17 |
| Seed 22, first mapping, assignment | After block 6 alone | −1.09 | +1.05 | −0.71 | +0.07 | −0.09 | −0.06 | −0.07 | −0.02 | −0.10 | +1.03 | −0.96 |
| Seed 22, first mapping, assignment | Plain on A's (H-47, 021's job) | −1.09 | +1.05 | −0.71 | +0.95 | −0.09 | +0.82 | −0.11 | +0.64 | −0.18 | +3.46 | −1.09 |
| Seed 22, first mapping, assignment | Without replay (H-47, 021's job) | −1.09 | +0.02 | −0.61 | +0.06 | −0.11 | −0.02 | −0.04 | −0.01 | −0.14 | +0.05 | −0.89 |
| Seed 23, eighth, mirrored | After block 6 alone | −2.80 | +1.38 | −0.66 | −0.16 | −0.11 | −0.09 | −0.17 | −0.03 | −0.33 | +1.10 | −1.26 |
| Seed 23, eighth, mirrored | Plain on A's (H-47, 021's job) | −2.80 | +1.38 | −0.66 | +1.03 | −0.24 | +0.79 | −0.19 | +0.66 | −0.29 | +3.86 | −1.38 |
| Seed 23, eighth, mirrored | Without replay (H-47, 021's job) | −2.80 | −0.10 | −0.60 | −0.09 | −0.17 | +0.04 | −0.19 | +0.05 | −0.27 | −0.11 | −1.23 |
| Seed 23, first mapping, assignment | After block 6 alone | −1.11 | +1.13 | −1.09 | −0.15 | −0.40 | −0.13 | −0.12 | −0.03 | −0.16 | +0.81 | −1.76 |
| Seed 23, first mapping, assignment | Plain on A's (H-47, 021's job) | −1.11 | +1.13 | −1.09 | +0.88 | −0.46 | +0.90 | −0.15 | +0.69 | −0.33 | +3.60 | −2.03 |
| Seed 23, first mapping, assignment | Without replay (H-47, 021's job) | −1.11 | 0.00 | −1.19 | −0.12 | −0.31 | −0.03 | −0.13 | 0.00 | −0.24 | −0.15 | −1.87 |
| Seed 24, eighth, mirrored | After block 6 alone | −3.00 | +1.28 | −0.44 | +0.06 | −0.22 | 0.00 | −0.12 | −0.06 | −0.29 | +1.27 | −1.07 |
| Seed 24, eighth, mirrored | Plain on A's (H-47, 021's job) | −3.00 | +1.28 | −0.44 | +1.17 | −0.21 | +1.12 | −0.18 | +0.76 | −0.28 | +4.33 | −1.11 |
| Seed 24, eighth, mirrored | Without replay (H-47, 021's job) | −3.00 | +0.07 | −0.37 | +0.07 | −0.29 | +0.08 | −0.19 | +0.04 | −0.37 | +0.27 | −1.22 |
| Seed 24, first mapping, assignment | After block 6 alone | −1.30 | +1.27 | −0.94 | +0.15 | −0.54 | +0.06 | −0.21 | +0.12 | −0.30 | +1.59 | −2.00 |
| Seed 24, first mapping, assignment | Plain on A's (H-47, 021's job) | −1.30 | +1.27 | −0.94 | +1.09 | −0.51 | +0.97 | −0.22 | +0.72 | −0.41 | +4.04 | −2.08 |
| Seed 24, first mapping, assignment | Without replay (H-47, 021's job) | −1.30 | +0.26 | −0.96 | +0.22 | −0.57 | +0.16 | −0.28 | +0.21 | −0.34 | +0.86 | −2.15 |
| Seed 23, eighth, assignment | After block 6 alone | −2.29 | +1.17 | −0.60 | +0.05 | −0.02 | −0.08 | −0.06 | −0.10 | −0.13 | +1.04 | −0.81 |
| Seed 23, eighth, assignment | Plain on A's (H-47, 021's job) | −2.29 | +1.17 | −0.60 | +1.08 | −0.01 | +0.67 | −0.03 | +0.58 | −0.16 | +3.50 | −0.79 |
| Seed 23, eighth, assignment | Without replay (H-47, 021's job) | −2.29 | +0.02 | −0.49 | 0.00 | −0.04 | −0.09 | −0.03 | −0.08 | −0.09 | −0.14 | −0.65 |
| Seed 24, first mapping, mirrored | After block 6 alone | −1.30 | +1.25 | −0.78 | −0.04 | −0.15 | +0.04 | −0.08 | +0.02 | −0.15 | +1.26 | −1.16 |
| Seed 24, first mapping, mirrored | Plain on A's (H-47, 021's job) | −1.30 | +1.25 | −0.78 | +0.89 | −0.09 | +0.75 | −0.13 | +0.58 | −0.27 | +3.47 | −1.28 |
| Seed 24, first mapping, mirrored | Without replay (H-47, 021's job) | −1.30 | +0.10 | −0.77 | +0.04 | −0.17 | +0.06 | −0.07 | +0.04 | −0.14 | +0.24 | −1.16 |
| Seed 23, first mapping, mirrored | After block 6 alone | −1.14 | +1.21 | −0.51 | +0.04 | −0.26 | +0.05 | −0.06 | +0.06 | −0.12 | +1.37 | −0.95 |
| Seed 23, first mapping, mirrored | Plain on A's (H-47, 020's job) | −1.14 | +1.21 | −0.51 | +1.01 | −0.27 | +0.75 | −0.11 | +0.54 | −0.22 | +3.50 | −1.10 |
| Seed 23, first mapping, mirrored | Without replay (H-47, 020's job) | −1.14 | +0.15 | −0.44 | +0.05 | −0.16 | +0.07 | −0.10 | +0.10 | −0.16 | +0.37 | −0.85 |
| Seed 24, eighth, assignment | After block 6 alone | −3.31 | +1.26 | −0.37 | −0.05 | −0.20 | −0.13 | −0.10 | −0.21 | −0.43 | +0.86 | −1.09 |
| Seed 24, eighth, assignment | Plain on A's (H-47, 020's job) | −3.31 | +1.26 | −0.37 | +0.95 | −0.20 | +0.56 | −0.05 | +0.58 | −0.19 | +3.35 | −0.81 |
| Seed 24, eighth, assignment | Without replay (H-47, 020's job) | −3.31 | −0.05 | −0.36 | −0.01 | −0.22 | −0.08 | −0.13 | −0.08 | −0.40 | −0.22 | −1.11 |
| Seed 22, first mapping, mirrored | After block 6 alone | −0.93 | +1.20 | −0.62 | +0.05 | −0.13 | +0.13 | −0.09 | +0.04 | −0.16 | +1.41 | −1.01 |
| Seed 22, first mapping, mirrored | Plain on A's (H-47, 020's job) | −0.93 | +1.20 | −0.62 | +0.88 | −0.16 | +0.65 | −0.13 | +0.60 | −0.20 | +3.33 | −1.11 |
| Seed 22, first mapping, mirrored | Without replay (H-47, 020's job) | −0.93 | +0.23 | −0.56 | +0.10 | −0.13 | +0.15 | −0.09 | 0.00 | −0.12 | +0.48 | −0.91 |
| Seed 22, eighth, assignment | After block 6 alone | −2.13 | +1.23 | −0.24 | +0.01 | −0.07 | −0.03 | −0.09 | −0.02 | −0.23 | +1.19 | −0.63 |
| Seed 22, eighth, assignment | Plain on A's (H-47, 020's job) | −2.13 | +1.23 | −0.24 | +1.00 | −0.13 | +0.74 | −0.12 | +0.59 | −0.25 | +3.56 | −0.74 |
| Seed 22, eighth, assignment | Without replay (H-47, 020's job) | −2.13 | +0.08 | −0.19 | +0.01 | −0.08 | +0.04 | −0.12 | 0.00 | −0.18 | +0.13 | −0.57 |

- **C's own units' margin onto A's answer** moved over the four nights by −0.28 to +0.15 M under the form, by −0.34 to +0.68 M under H-47's
  plain night and by −0.17 to +0.04 M under its night without replay. It fell over blocks 7 to 36 under all three in every cell, by 0.25 to
  0.75 M under the form. Of the 144 path-nights, it fell in 79 under the form, 35 under H-47's plain night and 99 under its night without
  replay.
- **A's margin** rose by 1.05 to 1.38 M over the form's night after block 6, H-47's first plain night, and moved by −0.21 to +0.15 M over
  each of its later nights, where H-47's later plain nights raised it by 0.54 to 1.17 M and its nights without replay after the same blocks
  moved it by −0.12 to +0.22 M. Over the four nights it rose by 0.81 to 1.59 M under the form and by 3.33 to 4.33 M under H-47's plain
  night, and moved by −0.23 to +0.86 M under its night without replay; the form less the night without replay over the four nights reads
  +0.74 to +1.20 M. Over blocks 7 to 36 A's margin fell under all three in every cell, by 0.63 to 2.00 M under the form.
- **D's own units** moved over the four nights by −0.24 to +0.25 M under the form, by −0.22 to +0.60 M under H-47's plain night and by
  −0.25 to +0.09 M under its night without replay.

## 5. The engine over each night

The form's night after block 6 beside H-47's plain night's night after block 6 on the same cell, exploration 020's or 021's job's: its
lines, its replays on each path, the ripples that replay one of A's patterns, by the pattern's stimulus, over the three paths, and the
pairings of A's and B's 102 stimulus units the watch read over the three paths:

| Cell | Its start, window, watch and end lines that are the lines job's plain night's after block 6, label aside | Replays, paths 0, 5, 6 | H-47's plain night: replays | Ripples on A's patterns | H-47's plain night | Ripples on B's patterns | H-47's plain night | Pairings watched | H-47's plain night | Lines job |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
| Seed 22, eighth, mirrored | 1 156 of 1 156 | 1 329, 1 329, 1 330 | 1 329, 1 329, 1 330 | 2 048 | 2 048 | 1 940 | 1 940 | 669 312 | 669 312 | 021 |
| Seed 22, first mapping, assignment | 1 141 of 1 141 | 1 339, 1 340, 1 339 | 1 339, 1 340, 1 339 | 2 097 | 2 097 | 1 921 | 1 921 | 716 448 | 716 448 | 021 |
| Seed 23, eighth, mirrored | 1 199 of 1 199 | 1 329, 1 329, 1 329 | 1 329, 1 329, 1 329 | 2 046 | 2 046 | 1 941 | 1 941 | 663 744 | 663 744 | 021 |
| Seed 23, first mapping, assignment | 1 260 of 1 260 | 1 339, 1 340, 1 339 | 1 339, 1 340, 1 339 | 2 097 | 2 097 | 1 921 | 1 921 | 714 944 | 714 944 | 021 |
| Seed 24, eighth, mirrored | 1 194 of 1 194 | 1 330, 1 330, 1 330 | 1 330, 1 330, 1 330 | 2 052 | 2 052 | 1 938 | 1 938 | 668 448 | 668 448 | 021 |
| Seed 24, first mapping, assignment | 1 211 of 1 211 | 1 339, 1 339, 1 339 | 1 339, 1 339, 1 339 | 2 097 | 2 097 | 1 920 | 1 920 | 718 400 | 718 400 | 021 |
| Seed 23, eighth, assignment | 1 218 of 1 218 | 1 329, 1 329, 1 329 | 1 329, 1 329, 1 329 | 2 046 | 2 046 | 1 941 | 1 941 | 675 040 | 675 040 | 021 |
| Seed 24, first mapping, mirrored | 1 222 of 1 222 | 1 339, 1 339, 1 339 | 1 339, 1 339, 1 339 | 2 097 | 2 097 | 1 920 | 1 920 | 736 064 | 736 064 | 021 |
| Seed 23, first mapping, mirrored | 1 214 of 1 214 | 1 339, 1 340, 1 339 | 1 339, 1 340, 1 339 | 2 097 | 2 097 | 1 921 | 1 921 | 721 312 | 721 312 | 020 |
| Seed 24, eighth, assignment | 1 195 of 1 195 | 1 329, 1 329, 1 329 | 1 329, 1 329, 1 329 | 2 046 | 2 046 | 1 941 | 1 941 | 684 096 | 684 096 | 020 |
| Seed 22, first mapping, mirrored | 1 130 of 1 130 | 1 339, 1 340, 1 339 | 1 339, 1 340, 1 339 | 2 097 | 2 097 | 1 921 | 1 921 | 732 000 | 732 000 | 020 |
| Seed 22, eighth, assignment | 1 145 of 1 145 | 1 329, 1 329, 1 330 | 1 329, 1 329, 1 330 | 2 048 | 2 048 | 1 940 | 1 940 | 686 560 | 686 560 | 020 |

- **In every cell the form's night after block 6 printed the lines of H-47's first plain night** in exploration 020's or 021's job, label
  and form aside, and read its replays, 1 329 to 1 340 a path, its ripples, 2 046 to 2 097 on A's patterns and 1 920 to 1 941 on B's over
  the three paths, and its pairings, 663 744 to 736 064 over the three paths. What those ripples and pairings held class by class is
  exploration 020's and 021's reading of that night, which exploration 030's section 5 sets out for seed 23's two first mappings; it is not
  read again here.

Each night's change in the engine's excitatory and inhibitory sums and its gain, from the night's start to its end, the mean over the three
paths; its replays and depotentiations on each path; and the watch's wrong answers over the three paths. The form and its night without
replay are this exploration's jobs'; H-47's plain night is exploration 020's or 021's job's:

| Cell | Night after block | Form | This night | Excitatory sum, M | Inhibitory sum, M | Gain | Replays, paths 0, 5, 6 | Depotentiations | Watch: wrong |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | After block 6 alone | replays | −8.37 | +10.11 | −5 067 | 1 329, 1 329, 1 330 | 640, 640, 640 | 0 |
| Seed 22, eighth, mirrored | 6 | Without replay | without replay | −0.49 | +0.31 | +1 989 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −8.37 | +10.11 | −5 067 | 1 329, 1 329, 1 330 | 640, 640, 640 | 0 |
| Seed 22, eighth, mirrored | 12 | After block 6 alone | without replay | +0.04 | −0.58 | +1 982 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, mirrored | 12 | Without replay | without replay | −0.14 | +0.31 | +1 980 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −5.67 | +8.07 | −4 483 | 1 268, 1 268, 1 268 | 601, 601, 601 | 0 |
| Seed 22, eighth, mirrored | 18 | After block 6 alone | without replay | +0.13 | −0.30 | +2 326 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, mirrored | 18 | Without replay | without replay | +0.07 | +0.30 | +2 312 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −3.55 | +6.90 | −3 966 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 22, eighth, mirrored | 24 | After block 6 alone | without replay | +0.38 | −0.12 | +2 187 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, mirrored | 24 | Without replay | without replay | +0.37 | +0.39 | +2 172 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −2.20 | +5.89 | −3 481 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 22, first mapping, assignment | 6 | After block 6 alone | replays | −6.90 | +9.08 | −5 036 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| Seed 22, first mapping, assignment | 6 | Without replay | without replay | −0.51 | −1.22 | +1 918 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −6.90 | +9.08 | −5 036 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| Seed 22, first mapping, assignment | 12 | After block 6 alone | without replay | +0.07 | −1.62 | +1 806 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, assignment | 12 | Without replay | without replay | +0.11 | −1.36 | +1 782 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −5.36 | +6.02 | −4 440 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| Seed 22, first mapping, assignment | 18 | After block 6 alone | without replay | +0.43 | −1.69 | +1 938 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, assignment | 18 | Without replay | without replay | +0.45 | −1.46 | +1 902 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −3.43 | +3.98 | −3 974 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 22, first mapping, assignment | 24 | After block 6 alone | without replay | +0.28 | −1.59 | +2 157 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, assignment | 24 | Without replay | without replay | +0.34 | −1.33 | +2 140 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −2.62 | +2.60 | −3 388 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 23, eighth, mirrored | 6 | After block 6 alone | replays | −8.75 | +10.56 | −4 692 | 1 329, 1 329, 1 329 | 640, 640, 640 | 0 |
| Seed 23, eighth, mirrored | 6 | Without replay | without replay | −0.69 | +0.33 | +2 160 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −8.75 | +10.56 | −4 692 | 1 329, 1 329, 1 329 | 640, 640, 640 | 0 |
| Seed 23, eighth, mirrored | 12 | After block 6 alone | without replay | −0.11 | −0.64 | +2 202 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, mirrored | 12 | Without replay | without replay | +0.12 | +0.37 | +2 237 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −5.70 | +8.49 | −4 188 | 1 268, 1 268, 1 268 | 601, 601, 601 | 0 |
| Seed 23, eighth, mirrored | 18 | After block 6 alone | without replay | +0.27 | −0.36 | +2 254 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, mirrored | 18 | Without replay | without replay | +0.83 | +0.46 | +2 293 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −4.18 | +7.05 | −3 533 | 1 202, 1 202, 1 202 | 570, 571, 571 | 0 |
| Seed 23, eighth, mirrored | 24 | After block 6 alone | without replay | +1.00 | −0.21 | +2 306 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, mirrored | 24 | Without replay | without replay | +1.49 | +0.63 | +2 360 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −2.89 | +5.90 | −3 111 | 1 128, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 23, first mapping, assignment | 6 | After block 6 alone | replays | −6.79 | +9.58 | −4 828 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| Seed 23, first mapping, assignment | 6 | Without replay | without replay | −0.46 | −1.03 | +1 900 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −6.79 | +9.58 | −4 828 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| Seed 23, first mapping, assignment | 12 | After block 6 alone | without replay | +0.16 | −1.62 | +1 924 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, assignment | 12 | Without replay | without replay | +0.47 | −1.29 | +1 938 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −5.31 | +6.32 | −4 260 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| Seed 23, first mapping, assignment | 18 | After block 6 alone | without replay | +0.56 | −1.63 | +2 012 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, assignment | 18 | Without replay | without replay | +0.76 | −1.41 | +2 107 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −3.20 | +3.72 | −3 830 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 23, first mapping, assignment | 24 | After block 6 alone | without replay | +0.93 | −1.71 | +2 035 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, assignment | 24 | Without replay | without replay | +1.16 | −1.42 | +2 118 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −2.37 | +2.60 | −3 224 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 24, eighth, mirrored | 6 | After block 6 alone | replays | −8.12 | +9.47 | −4 775 | 1 330, 1 330, 1 330 | 640, 640, 640 | 0 |
| Seed 24, eighth, mirrored | 6 | Without replay | without replay | −0.58 | +0.32 | +2 047 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −8.12 | +9.47 | −4 775 | 1 330, 1 330, 1 330 | 640, 640, 640 | 0 |
| Seed 24, eighth, mirrored | 12 | After block 6 alone | without replay | +0.05 | −0.55 | +2 080 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, mirrored | 12 | Without replay | without replay | +0.18 | +0.26 | +2 236 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −5.64 | +7.82 | −4 295 | 1 268, 1 268, 1 268 | 601, 601, 601 | 0 |
| Seed 24, eighth, mirrored | 18 | After block 6 alone | without replay | +0.58 | −0.28 | +2 289 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, mirrored | 18 | Without replay | without replay | +0.31 | +0.35 | +2 402 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −3.88 | +6.61 | −3 658 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 24, eighth, mirrored | 24 | After block 6 alone | without replay | +0.51 | +0.01 | +2 274 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, mirrored | 24 | Without replay | without replay | +0.38 | +0.48 | +2 393 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −2.71 | +5.85 | −3 246 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 24, first mapping, assignment | 6 | After block 6 alone | replays | −6.29 | +8.72 | −4 928 | 1 339, 1 339, 1 339 | 640, 640, 640 | 0 |
| Seed 24, first mapping, assignment | 6 | Without replay | without replay | +0.12 | −1.21 | +1 815 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −6.29 | +8.72 | −4 928 | 1 339, 1 339, 1 339 | 640, 640, 640 | 0 |
| Seed 24, first mapping, assignment | 12 | After block 6 alone | without replay | +0.14 | −1.63 | +1 942 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, assignment | 12 | Without replay | without replay | +0.45 | −1.33 | +1 959 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −5.07 | +5.92 | −4 403 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| Seed 24, first mapping, assignment | 18 | After block 6 alone | without replay | +0.68 | −1.64 | +2 015 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, assignment | 18 | Without replay | without replay | +0.67 | −1.30 | +2 053 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −3.53 | +3.84 | −3 896 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 24, first mapping, assignment | 24 | After block 6 alone | without replay | +0.91 | −1.59 | +2 047 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, assignment | 24 | Without replay | without replay | +0.77 | −1.32 | +2 131 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −2.48 | +2.56 | −3 290 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 23, eighth, assignment | 6 | After block 6 alone | replays | −9.49 | +9.72 | −4 179 | 1 329, 1 329, 1 329 | 640, 640, 640 | 0 |
| Seed 23, eighth, assignment | 6 | Without replay | without replay | −0.82 | +0.36 | +2 052 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −9.49 | +9.72 | −4 179 | 1 329, 1 329, 1 329 | 640, 640, 640 | 0 |
| Seed 23, eighth, assignment | 12 | After block 6 alone | without replay | +0.04 | −0.62 | +2 124 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, assignment | 12 | Without replay | without replay | +0.09 | +0.35 | +2 208 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −6.46 | +7.84 | −3 699 | 1 268, 1 268, 1 268 | 601, 601, 601 | 0 |
| Seed 23, eighth, assignment | 18 | After block 6 alone | without replay | +0.36 | −0.33 | +2 342 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, assignment | 18 | Without replay | without replay | +0.65 | +0.45 | +2 334 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −4.38 | +6.47 | −3 160 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 23, eighth, assignment | 24 | After block 6 alone | without replay | +0.72 | −0.21 | +2 352 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, assignment | 24 | Without replay | without replay | +1.52 | +0.68 | +2 338 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −2.94 | +5.54 | −2 788 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 24, first mapping, mirrored | 6 | After block 6 alone | replays | −7.93 | +7.94 | −4 304 | 1 339, 1 339, 1 339 | 640, 640, 640 | 0 |
| Seed 24, first mapping, mirrored | 6 | Without replay | without replay | +0.04 | −1.28 | +1 853 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −7.93 | +7.94 | −4 304 | 1 339, 1 339, 1 339 | 640, 640, 640 | 0 |
| Seed 24, first mapping, mirrored | 12 | After block 6 alone | without replay | 0.00 | −1.67 | +1 909 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, mirrored | 12 | Without replay | without replay | +0.43 | −1.36 | +1 892 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −5.78 | +4.95 | −3 826 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| Seed 24, first mapping, mirrored | 18 | After block 6 alone | without replay | +0.63 | −1.60 | +2 001 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, mirrored | 18 | Without replay | without replay | +0.68 | −1.30 | +1 940 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −4.10 | +3.55 | −3 305 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 24, first mapping, mirrored | 24 | After block 6 alone | without replay | +0.52 | −1.58 | +2 080 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, mirrored | 24 | Without replay | without replay | +0.69 | −1.33 | +2 001 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −3.14 | +2.67 | −2 879 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 23, first mapping, mirrored | 6 | After block 6 alone | replays | −7.72 | +8.85 | −4 327 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| Seed 23, first mapping, mirrored | 6 | Without replay | without replay | −0.48 | −1.19 | +1 991 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, mirrored | 6 | Plain on A's (H-47, 020's job) | replays | −7.72 | +8.85 | −4 327 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| Seed 23, first mapping, mirrored | 12 | After block 6 alone | without replay | +0.33 | −1.71 | +1 957 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, mirrored | 12 | Without replay | without replay | +0.37 | −1.40 | +1 972 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, mirrored | 12 | Plain on A's (H-47, 020's job) | replays | −5.84 | +5.81 | −3 789 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| Seed 23, first mapping, mirrored | 18 | After block 6 alone | without replay | +0.56 | −1.69 | +2 030 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, mirrored | 18 | Without replay | without replay | +0.64 | −1.43 | +1 978 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, mirrored | 18 | Plain on A's (H-47, 020's job) | replays | −3.93 | +3.79 | −3 392 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 23, first mapping, mirrored | 24 | After block 6 alone | without replay | +0.66 | −1.63 | +2 063 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, mirrored | 24 | Without replay | without replay | +1.02 | −1.33 | +1 991 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, mirrored | 24 | Plain on A's (H-47, 020's job) | replays | −2.89 | +2.62 | −2 803 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 24, eighth, assignment | 6 | After block 6 alone | replays | −9.36 | +8.80 | −4 075 | 1 329, 1 329, 1 329 | 640, 640, 640 | 0 |
| Seed 24, eighth, assignment | 6 | Without replay | without replay | −0.47 | +0.32 | +2 134 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, assignment | 6 | Plain on A's (H-47, 020's job) | replays | −9.36 | +8.80 | −4 075 | 1 329, 1 329, 1 329 | 640, 640, 640 | 0 |
| Seed 24, eighth, assignment | 12 | After block 6 alone | without replay | +0.11 | −0.53 | +2 050 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, assignment | 12 | Without replay | without replay | +0.10 | +0.25 | +2 175 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, assignment | 12 | Plain on A's (H-47, 020's job) | replays | −6.68 | +7.16 | −3 721 | 1 268, 1 268, 1 268 | 601, 601, 601 | 0 |
| Seed 24, eighth, assignment | 18 | After block 6 alone | without replay | +0.56 | −0.31 | +2 284 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, assignment | 18 | Without replay | without replay | +0.43 | +0.32 | +2 448 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, assignment | 18 | Plain on A's (H-47, 020's job) | replays | −4.45 | +5.92 | −2 997 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 24, eighth, assignment | 24 | After block 6 alone | without replay | +0.58 | −0.08 | +2 320 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, assignment | 24 | Without replay | without replay | +0.34 | +0.44 | +2 413 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, assignment | 24 | Plain on A's (H-47, 020's job) | replays | −3.38 | +5.16 | −2 768 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 22, first mapping, mirrored | 6 | After block 6 alone | replays | −7.55 | +7.36 | −4 331 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| Seed 22, first mapping, mirrored | 6 | Without replay | without replay | −0.44 | −1.32 | +1 995 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, mirrored | 6 | Plain on A's (H-47, 020's job) | replays | −7.55 | +7.36 | −4 331 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| Seed 22, first mapping, mirrored | 12 | After block 6 alone | without replay | +0.08 | −1.66 | +1 828 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, mirrored | 12 | Without replay | without replay | +0.09 | −1.39 | +1 792 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, mirrored | 12 | Plain on A's (H-47, 020's job) | replays | −5.75 | +4.75 | −3 872 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| Seed 22, first mapping, mirrored | 18 | After block 6 alone | without replay | +0.44 | −1.65 | +1 859 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, mirrored | 18 | Without replay | without replay | +0.40 | −1.39 | +1 884 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, mirrored | 18 | Plain on A's (H-47, 020's job) | replays | −3.85 | +3.21 | −3 515 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 22, first mapping, mirrored | 24 | After block 6 alone | without replay | +0.34 | −1.56 | +2 123 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, mirrored | 24 | Without replay | without replay | +0.37 | −1.40 | +2 141 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, mirrored | 24 | Plain on A's (H-47, 020's job) | replays | −2.96 | +2.24 | −2 864 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 22, eighth, assignment | 6 | After block 6 alone | replays | −9.17 | +8.35 | −4 301 | 1 329, 1 329, 1 330 | 640, 640, 640 | 0 |
| Seed 22, eighth, assignment | 6 | Without replay | without replay | −0.61 | +0.28 | +2 019 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, assignment | 6 | Plain on A's (H-47, 020's job) | replays | −9.17 | +8.35 | −4 301 | 1 329, 1 329, 1 330 | 640, 640, 640 | 0 |
| Seed 22, eighth, assignment | 12 | After block 6 alone | without replay | +0.02 | −0.59 | +2 030 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, assignment | 12 | Without replay | without replay | −0.11 | +0.23 | +2 034 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, assignment | 12 | Plain on A's (H-47, 020's job) | replays | −6.30 | +6.67 | −3 912 | 1 268, 1 268, 1 268 | 601, 601, 601 | 0 |
| Seed 22, eighth, assignment | 18 | After block 6 alone | without replay | +0.16 | −0.26 | +2 280 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, assignment | 18 | Without replay | without replay | −0.06 | +0.24 | +2 419 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, assignment | 18 | Plain on A's (H-47, 020's job) | replays | −4.29 | +5.74 | −3 311 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 22, eighth, assignment | 24 | After block 6 alone | without replay | +0.25 | −0.17 | +2 164 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, assignment | 24 | Without replay | without replay | +0.23 | +0.28 | +2 133 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, assignment | 24 | Plain on A's (H-47, 020's job) | replays | −3.19 | +4.92 | −2 813 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |

- **The form's night after block 6**, H-47's first plain night, lowered the excitatory sum by 6.29 to 9.49 M, raised the inhibitory sum by
  7.36 to 10.56 M and lowered the gain by 4 075 to 5 067, with 1 329 to 1 340 replays and 640 depotentiations on each path.
- **Its nights after blocks 12, 18 and 24** read `replays 0` and `depotentiations 0`, held inside the test, moved the excitatory sum by
  −0.11 to +1.00 M and the inhibitory by −1.71 to +0.01 M, and raised the gain by 1 806 to 2 352. In each of those 36 nights the inhibitory
  sum's change lay 0.17 to 1.01 M below the night without replay's after the same block on the same cell. The night without replay's nights
  after the same blocks moved the excitatory sum by −0.14 to +1.52 M and the inhibitory by −1.46 to +0.68 M, and raised the gain by 1 782 to
  2 448; H-47's later plain nights lowered the excitatory sum by 2.20 to 6.68 M, raised the inhibitory by 2.24 to 8.49 M and lowered the
  gain by 2 768 to 4 483, with 1 128 to 1 268 replays a path.
- **The watch read `wrong 0`** in each of the 288 nights of the twelve jobs and in H-47's plain night's 144 in exploration 020's and 021's
  jobs.

The ties of both stimuli's trials, the form less the night without replay of its own job, ± the paired SE, and H-47's plain night less its
night without replay, H-47's own:

| Cell | Form | Block 6 | 12 | 18 | 24 | 36 |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | After block 6 alone | −1.4 ± 0.25 | −0.3 ± 0.67 | +0.3 ± 0.35 | −0.2 ± 0.36 | +0.1 ± 0.48 |
| Seed 22, eighth, mirrored | Plain on A's (H-47) | −1.4 ± 0.25 (H-47) | −0.3 ± 0.61 (H-47) | +0.7 ± 0.39 (H-47) | +0.5 ± 0.40 (H-47) | +1.5 ± 0.59 (H-47) |
| Seed 22, first mapping, assignment | After block 6 alone | −0.5 ± 0.15 | +0.5 ± 0.74 | −0.7 ± 0.30 | −0.3 ± 0.16 | −0.5 ± 0.70 |
| Seed 22, first mapping, assignment | Plain on A's (H-47) | −0.5 ± 0.15 (H-47) | +1.0 ± 0.29 (H-47) | −0.1 ± 0.37 (H-47) | +0.5 ± 0.65 (H-47) | −0.1 ± 0.44 (H-47) |
| Seed 23, eighth, mirrored | After block 6 alone | −0.1 ± 0.45 | −0.1 ± 0.21 | +0.9 ± 0.30 | +1.5 ± 0.50 | +0.9 ± 0.70 |
| Seed 23, eighth, mirrored | Plain on A's (H-47) | −0.1 ± 0.45 (H-47) | +1.0 ± 0.48 (H-47) | +0.3 ± 0.59 (H-47) | +0.2 ± 0.46 (H-47) | +0.8 ± 0.35 (H-47) |
| Seed 23, first mapping, assignment | After block 6 alone | −0.3 ± 0.64 | −1.5 ± 0.31 | −1.4 ± 0.60 | −0.2 ± 0.48 | −0.8 ± 0.62 |
| Seed 23, first mapping, assignment | Plain on A's (H-47) | −0.3 ± 0.64 (H-47) | −0.7 ± 0.58 (H-47) | −1.9 ± 0.56 (H-47) | −2.1 ± 0.29 (H-47) | −2.1 ± 0.47 (H-47) |
| Seed 24, eighth, mirrored | After block 6 alone | +0.7 ± 0.53 | −0.1 ± 0.11 | +0.1 ± 0.52 | −0.4 ± 0.30 | +0.6 ± 0.45 |
| Seed 24, eighth, mirrored | Plain on A's (H-47) | +0.7 ± 0.53 (H-47) | +0.5 ± 0.62 (H-47) | +1.1 ± 0.28 (H-47) | +1.1 ± 0.52 (H-47) | +1.5 ± 0.80 (H-47) |
| Seed 24, first mapping, assignment | After block 6 alone | −0.9 ± 0.25 | −0.2 ± 0.31 | +0.1 ± 0.52 | −0.6 ± 0.28 | −0.9 ± 0.58 |
| Seed 24, first mapping, assignment | Plain on A's (H-47) | −0.9 ± 0.25 (H-47) | −0.4 ± 0.34 (H-47) | +0.2 ± 0.06 (H-47) | −1.2 ± 1.09 (H-47) | −0.5 ± 0.29 (H-47) |
| Seed 23, eighth, assignment | After block 6 alone | −0.3 ± 0.60 | −0.3 ± 0.42 | −0.7 ± 0.45 | +0.2 ± 0.02 | +0.5 ± 0.21 |
| Seed 23, eighth, assignment | Plain on A's (H-47) | −0.3 ± 0.60 (H-47) | +0.2 ± 0.46 (H-47) | −0.6 ± 0.31 (H-47) | +0.5 ± 0.46 (H-47) | 0.0 ± 0.60 (H-47) |
| Seed 24, first mapping, mirrored | After block 6 alone | −0.4 ± 0.47 | −0.5 ± 0.48 | −1.1 ± 0.29 | +0.4 ± 0.93 | −0.1 ± 0.42 |
| Seed 24, first mapping, mirrored | Plain on A's (H-47) | −0.4 ± 0.47 (H-47) | −0.9 ± 0.47 (H-47) | −1.2 ± 0.46 (H-47) | −1.0 ± 0.47 (H-47) | −0.3 ± 0.68 (H-47) |
| Seed 23, first mapping, mirrored | After block 6 alone | −0.6 ± 0.42 | −0.8 ± 0.37 | +0.2 ± 0.57 | −0.4 ± 0.49 | −0.2 ± 0.17 |
| Seed 23, first mapping, mirrored | Plain on A's (H-47) | −0.6 ± 0.42 (H-47) | −0.6 ± 0.56 (H-47) | +0.4 ± 0.43 (H-47) | +0.6 ± 0.36 (H-47) | +1.7 ± 0.21 (H-47) |
| Seed 24, eighth, assignment | After block 6 alone | +0.5 ± 0.38 | −0.4 ± 0.36 | +0.4 ± 0.56 | +0.9 ± 0.37 | −0.4 ± 0.32 |
| Seed 24, eighth, assignment | Plain on A's (H-47) | +0.5 ± 0.38 (H-47) | +0.3 ± 0.24 (H-47) | +2.0 ± 0.40 (H-47) | +1.2 ± 0.27 (H-47) | +0.4 ± 0.37 (H-47) |
| Seed 22, first mapping, mirrored | After block 6 alone | +0.2 ± 0.37 | −0.2 ± 0.36 | −0.5 ± 1.06 | −0.5 ± 0.50 | −0.3 ± 0.85 |
| Seed 22, first mapping, mirrored | Plain on A's (H-47) | +0.2 ± 0.37 (H-47) | +0.1 ± 0.56 (H-47) | −0.6 ± 0.45 (H-47) | −0.4 ± 0.81 (H-47) | −0.7 ± 0.35 (H-47) |
| Seed 22, eighth, assignment | After block 6 alone | −0.3 ± 0.33 | −0.2 ± 0.42 | +0.2 ± 0.39 | 0.0 ± 0.57 | +0.3 ± 0.28 |
| Seed 22, eighth, assignment | Plain on A's (H-47) | −0.3 ± 0.33 (H-47) | 0.0 ± 0.45 (H-47) | −0.1 ± 0.45 (H-47) | 0.0 ± 0.24 (H-47) | −0.1 ± 0.18 (H-47) |

- **At block 36 the ties under the form** lie within 2 SE of the night without replay's in eleven cells, and above beyond 2 SE on seed 23's
  eighth image from the assignment, +0.5 ± 0.21. H-47's plain night's lie above its night without replay's beyond 2 SE in three cells and
  below in one, seed 23's first mapping from the assignment, −2.1 ± 0.47.

## 6. The twelve cells at block 36

H-47's twelve cells at block 36, the rows ordered by the readout A answers, the six where it answers readout 0 first. The form and its
night without replay are this exploration's jobs, exploration 030's on seed 23's first mapping from the mirrored assignment; H-47's plain
night is H-47's job's in all twelve. "Less without replay" carries the job's paired SE, "less H-47's plain" exploration 018's unpaired SE,
and H-47's clause 1 H-47's paired SE. No figure is pooled across cells:

| Cell | A answers | After block 6 alone less without replay | After block 6 alone less H-47's plain | H-47: plain less without replay (clause 1) |
| :--- | :--- | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | +7.3 ± 0.65 | −12.4 ± 1.05 | +19.6 ± 1.06 |
| Seed 22, first mapping, assignment | readout 0, the odd places | +4.3 ± 1.05 | −3.6 ± 0.87 | +7.9 ± 0.51 |
| Seed 23, eighth, mirrored | readout 0, the odd places | +4.3 ± 0.63 | −13.1 ± 1.09 | +17.4 ± 1.08 |
| Seed 23, first mapping, assignment | readout 0, the odd places | +5.0 ± 2.61 | −8.3 ± 1.52 | +13.4 ± 1.56 |
| Seed 24, eighth, mirrored | readout 0, the odd places | +3.1 ± 1.84 | −8.2 ± 2.09 | +11.3 ± 0.96 |
| Seed 24, first mapping, assignment | readout 0, the odd places | +3.9 ± 0.76 | +0.1 ± 1.97 | +3.8 ± 1.57 |
| Seed 23, eighth, assignment | readout 1, the even places | +1.3 ± 1.04 | −2.8 ± 1.52 | +4.1 ± 1.45 |
| Seed 24, first mapping, mirrored | readout 1, the even places | +3.2 ± 2.06 | −2.8 ± 1.01 | +6.0 ± 1.54 |
| Seed 23, first mapping, mirrored | readout 1, the even places | +1.7 ± 0.78 | +6.6 ± 0.90 | −4.9 ± 1.09 |
| Seed 24, eighth, assignment | readout 1, the even places | −2.0 ± 0.88 | −3.8 ± 2.14 | +1.8 ± 1.79 |
| Seed 22, first mapping, mirrored | readout 1, the even places | +1.3 ± 2.40 | −3.7 ± 1.74 | +5.0 ± 1.24 |
| Seed 22, eighth, assignment | readout 1, the even places | −0.9 ± 1.07 | −3.1 ± 1.56 | +2.2 ± 0.65 |

| Cell | A answers | A's trials: after block 6 alone less without replay | B's trials | A's trials: less H-47's plain | B's trials | A's trials: H-47, plain less without replay | B's trials |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | +0.8 ± 0.55 | +6.5 ± 0.44 | −8.5 ± 1.02 | −3.9 ± 0.69 | +9.2 ± 0.86 | +10.4 ± 0.54 |
| Seed 22, first mapping, assignment | readout 0, the odd places | +4.0 ± 1.93 | +0.3 ± 1.05 | −4.4 ± 1.43 | +0.8 ± 0.71 | +8.4 ± 1.10 | −0.4 ± 0.83 |
| Seed 23, eighth, mirrored | readout 0, the odd places | +3.7 ± 0.55 | +0.6 ± 0.17 | −12.2 ± 1.13 | −0.9 ± 0.37 | +15.9 ± 1.05 | +1.5 ± 0.28 |
| Seed 23, first mapping, assignment | readout 0, the odd places | +6.4 ± 2.30 | −1.4 ± 0.41 | −7.1 ± 1.51 | −1.3 ± 0.56 | +13.5 ± 1.09 | −0.1 ± 0.62 |
| Seed 24, eighth, mirrored | readout 0, the odd places | +0.7 ± 0.86 | +2.4 ± 1.06 | −0.9 ± 1.68 | −7.3 ± 0.99 | +1.5 ± 0.74 | +9.7 ± 0.98 |
| Seed 24, first mapping, assignment | readout 0, the odd places | +0.8 ± 0.49 | +3.1 ± 0.45 | −2.5 ± 1.07 | +2.6 ± 1.71 | +3.3 ± 0.54 | +0.5 ± 1.44 |
| Seed 23, eighth, assignment | readout 1, the even places | +2.4 ± 0.41 | −1.2 ± 1.31 | +0.1 ± 1.48 | −3.0 ± 1.02 | +2.3 ± 0.88 | +1.8 ± 0.84 |
| Seed 24, first mapping, mirrored | readout 1, the even places | +3.4 ± 0.53 | −0.1 ± 1.54 | +0.1 ± 0.61 | −2.9 ± 1.13 | +3.3 ± 0.67 | +2.7 ± 0.99 |
| Seed 23, first mapping, mirrored | readout 1, the even places | 0.0 ± 0.56 | +1.7 ± 0.40 | +5.2 ± 1.42 | +1.4 ± 1.31 | −5.2 ± 1.50 | +0.3 ± 0.88 |
| Seed 24, eighth, assignment | readout 1, the even places | −1.0 ± 0.63 | −1.0 ± 0.71 | −1.1 ± 1.43 | −2.7 ± 0.99 | +0.2 ± 0.76 | +1.6 ± 1.50 |
| Seed 22, first mapping, mirrored | readout 1, the even places | +0.1 ± 1.24 | +1.1 ± 1.27 | +0.5 ± 0.95 | −4.3 ± 1.60 | −0.4 ± 0.83 | +5.4 ± 1.47 |
| Seed 22, eighth, assignment | readout 1, the even places | −2.0 ± 0.58 | +1.1 ± 1.03 | −0.4 ± 1.13 | −2.7 ± 1.27 | −1.6 ± 0.88 | +3.8 ± 0.81 |

| Cell | A answers | After block 6 alone: A's less B's | Side | H-47: A's less B's | H-47's side |
| :--- | :--- | ---: | :--- | ---: | :--- |
| Seed 22, eighth, mirrored | readout 0, the odd places | −5.8 ± 0.77 | B, the even places | −1.2 ± 0.97 | B, the even places (within 2 SE) |
| Seed 22, first mapping, assignment | readout 0, the odd places | +3.7 ± 2.92 | A, the odd places (within 2 SE) | +8.8 ± 1.88 | A, the odd places |
| Seed 23, eighth, mirrored | readout 0, the odd places | +3.1 ± 0.52 | A, the odd places | +14.4 ± 1.09 | A, the odd places |
| Seed 23, first mapping, assignment | readout 0, the odd places | +7.8 ± 2.02 | A, the odd places | +13.6 ± 0.84 | A, the odd places |
| Seed 24, eighth, mirrored | readout 0, the odd places | −1.7 ± 0.53 | B, the even places | −8.2 ± 1.45 | B, the even places |
| Seed 24, first mapping, assignment | readout 0, the odd places | −2.3 ± 0.55 | B, the even places | +2.8 ± 1.52 | A, the odd places (within 2 SE) |
| Seed 23, eighth, assignment | readout 1, the even places | +3.6 ± 1.64 | A, the even places | +0.5 ± 0.92 | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | readout 1, the even places | +3.5 ± 1.03 | A, the even places | +0.5 ± 0.67 | A, the even places (within 2 SE) |
| Seed 23, first mapping, mirrored | readout 1, the even places | −1.6 ± 0.60 | B, the odd places | −5.5 ± 2.21 | B, the odd places |
| Seed 24, eighth, assignment | readout 1, the even places | +0.1 ± 1.01 | A, the even places (within 2 SE) | −1.4 ± 1.64 | B, the odd places (within 2 SE) |
| Seed 22, first mapping, mirrored | readout 1, the even places | −1.0 ± 0.69 | B, the odd places (within 2 SE) | −5.8 ± 2.05 | B, the odd places |
| Seed 22, eighth, assignment | readout 1, the even places | −3.1 ± 1.23 | B, the odd places | −5.4 ± 1.57 | B, the odd places |

- **Where H-47's clause 1 lies above beyond 2 SE**, ten cells, the form lies above the night without replay beyond 2 SE in four, all where A
  answers readout 0: seed 22's eighth image from the mirrored assignment, seed 22's first mapping from the assignment, seed 23's eighth
  image from the mirrored assignment and seed 24's first mapping from the assignment. It lies within 2 SE in the other six, seed 23's first
  mapping from the assignment and seed 24's eighth image from the mirrored assignment, where A answers readout 0, and the four where it
  answers readout 1 but seed 23's first mapping from the mirrored assignment and seed 24's eighth image from the assignment; of the six only
  seed 22's eighth image from the assignment reads below in sign, −0.9 ± 1.07. In the same ten the form lies below H-47's plain night beyond
  2 SE in seven and within in three: seed 24's first mapping from the assignment, +0.1 ± 1.97, seed 23's eighth image from the assignment,
  −2.8 ± 1.52, and seed 22's eighth image from the assignment, −3.1 ± 1.56.
- **Where H-47's clause 1 lies within 2 SE**, seed 24's eighth image from the assignment, +1.8 ± 1.79, the form lies below the night without
  replay beyond 2 SE, −2.0 ± 0.88, and within 2 SE of H-47's plain night, −3.8 ± 2.14.
- **Where H-47's clause 1 lies below beyond 2 SE**, seed 23's first mapping from the mirrored assignment, −4.9 ± 1.09, the form lies above
  both beyond 2 SE, +1.7 ± 0.78 and +6.6 ± 0.90, exploration 030's reading.
- **The side at block 36** under the form is H-47's in ten cells, all but seed 24's first mapping from the assignment and seed 24's eighth
  image from the assignment (section 2).

## What it costs

The trial took 355 s and the eleven jobs 3 863 to 4 236 s, 64.4 to 70.6 minutes, at most 29.4 % of a job's 240 minutes; ADR-0246 put a job
at about 65 to 69 minutes. Nothing was cut: every job ran its two forms on the three paths, and every night was watched. Twelve jobs ran,
the trial's and eleven, every one to the end; seed 23's first mapping from the mirrored assignment ran none, its two forms read from
exploration 030's job.

## What it suggests

**On these twelve cells, H-47's plain night replayed after block 6 alone, every later night without replay, left A's count at block 36
above the night without replay beyond 2 SE in five cells, below in one and within 2 SE in six.** It lies above in four of the six cells
where A answers readout 0, and in one of the six where A answers readout 1, the one where H-47's plain night lies below the night without
replay. Less H-47's plain night it lies below beyond 2 SE in seven cells, within in four, and above in that one.

**Its replayed night is H-47's first plain night, and what that night moved is H-47's in every cell.** Each of its nights after it, without
replay, left the inhibitory columns of the class matrix lower than H-47's night without replay after the same block did, and so did each
stretch of blocks 13 to 36; over those nights A's margin moved by −0.21 to +0.15 M, where H-47's later plain nights raised it by 0.54 to
1.17 M.

Questions this suggests for a later exploration or a round:
- Replay after block 12, 18 or 24 alone on the eleven cells, which exploration 030 read on one cell alone: whether any one replayed night,
  not only the first, leaves A's count where the form does here.
- Seed 24's eighth image from the assignment, the one cell where the form lies below the night without replay beyond 2 SE at block 36, and
  at block 24 too, and where H-47's plain night lies below it at blocks 18 and 24: what the blocks after the replayed night move there on
  A's and B's trials.
- Replay on two of the four nights, on any cell.
- A read unit by unit, to see which readout units fire on A's trials after the replayed night.

## What did not run

- **Replay after block 12, 18 or 24 alone, on every night but one, or on two of the four nights**, on any cell: so where the form lies
  above the night without replay on a cell, these runs do not say whether the night after block 6 does it or any one replayed night would.
  Exploration 030 ran those single nights and the nights left out on seed 23's first mapping from the mirrored assignment alone.
- **Any hold**, on any night.
- **Plain nights on both mappings and the awake gap**, night by night or at all: no reading here stands against them.
- **A read unit by unit**: a read gives each readout's spike count, not which units fired.
- **Any path but H-47's three**, 0, 5 and 6.
- **A night split apart**, such as slow-wave sleep read apart from REM: a night's change is read whole, from the images before and after
  it, and REM's depotentiations and the zero sum's rounds are not split out of it, as in explorations 020 to 030.
- **The next day's reading of the pairings a night leaves** pending.
- **The replayed night read anew**: the form's night after block 6 is H-47's first plain night on the same image, so its ripples and
  pairings by class are exploration 020's and 021's reading, set beside it in section 5 and not read here.
- **Any SE for section 4, or for section 5's tables of the replayed night and the engine**: the class matrix, the split and the watch are
  means or totals over the three paths. Section 5's ties carry the paired SE.
- **H-47's plain night run again**: its rows and reads are H-47's job's and its night lines exploration 020's or 021's, so the form less
  it carries exploration 018's unpaired SE.
- **Any count, difference or SE pooled across cells**, in section 6 or elsewhere; the nights and pairings of "The runs are laid out as
  written" are tallies over the jobs.
- **Any other share, form, network or seed.**

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_the_first_nights_replay_on_twelve_cells_explore` | [42665197682](https://github.com/Menteion/Menteion/actions/runs/42665197682) | `a6819e3` | 355 |
| `the_first_nights_replay_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780) | `a6819e3` | 3935 |
| `the_first_nights_replay_a_first_mapping_on_seed_22_from_the_assignment_explore` | [42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780) | `a6819e3` | 4105 |
| `the_first_nights_replay_the_eighth_image_on_seed_23_from_the_mirrored_assignment_explore` | [42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780) | `a6819e3` | 3873 |
| `the_first_nights_replay_a_first_mapping_on_seed_23_from_the_assignment_explore` | [42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780) | `a6819e3` | 4035 |
| `the_first_nights_replay_the_eighth_image_on_seed_24_from_the_mirrored_assignment_explore` | [42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780) | `a6819e3` | 3863 |
| `the_first_nights_replay_a_first_mapping_on_seed_24_from_the_assignment_explore` | [42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780) | `a6819e3` | 3979 |
| `the_first_nights_replay_the_eighth_image_on_seed_23_from_the_assignment_explore` | [42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780) | `a6819e3` | 3905 |
| `the_first_nights_replay_a_first_mapping_on_seed_24_from_the_mirrored_assignment_explore` | [42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780) | `a6819e3` | 4236 |
| `one_night_alone_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [42621480151](https://github.com/Menteion/Menteion/actions/runs/42621480151) | `8291346` | 9756 |
| `the_first_nights_replay_the_eighth_image_on_seed_24_from_the_assignment_explore` | [42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780) | `a6819e3` | 3911 |
| `the_first_nights_replay_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780) | `a6819e3` | 4102 |
| `the_first_nights_replay_the_eighth_image_on_seed_22_from_the_assignment_explore` | [42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780) | `a6819e3` | 3887 |
