---
status: exploratory
date: 2026-11-11
---

# 032: Each later night's replay alone, on twelve cells

**Branch**: `explore/032-each-later-nights-replay-alone-on-twelve-cells`
**Commit**: `b93a130`, which adds the harness, on `b4d299a`: `main` at `093cb5e`, ADR-0247's merge, with exploration 031's tag merged
in, which changes nothing under `src/`. This branch changes nothing under `src/`, so the engine is `main`'s.
**Runs**: [42970775904](https://github.com/Menteion/Menteion/actions/runs/42970775904) (the harness's trial) and
[42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683) (eleven jobs, one a cell), both at `scope=arms`. On seed 23's
first mapping from the mirrored assignment the four forms and replay after block 6 alone are read from exploration 030's one-night-alone
job, run [42621480151](https://github.com/Menteion/Menteion/actions/runs/42621480151); on the other eleven, replay after block 6 alone is
read from exploration 031's jobs, run [42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780). H-47's plain night on
A's patterns is read from H-47's run [41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613), and its class matrices,
night lines and watch from exploration 020's runs [41500459562](https://github.com/Menteion/Menteion/actions/runs/41500459562) and
[41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777) for exploration 020's four cells and from exploration 021's run
[41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) for the other eight. None of those was run again.

[ADR-0247](../adr/0247-each-later-nights-replay-alone-on-twelve-cells.md) ordered this exploration. On all twelve of
[H-47](../research/H-47.md)'s cells, [exploration 031](031-the-first-nights-replay-on-twelve-cells.md) read H-47's plain night on A's
patterns replayed after block 6 alone, every later night without replay: at block 36 it lay above the night without replay beyond twice the
paired SE in five cells, below in one and within 2 SE in six. Replay after block 12, 18 or 24 alone had been read only on seed 23's first
mapping from the mirrored assignment ([exploration 030](030-one-cell-night-by-night.md)). This note reads those three forms beside the night
without replay on all twelve cells, with replay after block 6 alone beside them. It asks, at blocks 6, 12, 18, 24 and 36:

1. A's count and each stimulus's correct trials under each form: less the night without replay, with H-47's paired SE, the forms run in
   one job; and less H-47's plain night, with [exploration 018](018-what-nights-keep.md)'s unpaired SE, H-47's plain night run in another
   job;
2. the four single nights at block 36, replay after block 6 alone among them: each less the night without replay, and their sum, beside
   H-47's plain night less the night without replay, by stimulus;
3. the side under each form, by stimulus and by group of units, against H-47's side on the same cell;
4. the pair under each form and A's highest coupling, as H-47's clauses 2 and 3 read them;
5. what each night and each stretch of blocks moved, from the class matrix by group, each replayed night beside H-47's night without
   replay after the same block, which ran on the same image, and H-47's plain night after block 6 and after the same block, with C's own
   and D's own units and A's margin onto A's answer;
6. the engine's two sums over each night, its replays and depotentiations, the watch's wrong answers and the ties; and what each replayed
   night held, its ripples' spikes and its pairings by class, beside H-47's first plain night on the same cell;
7. the twelve cells side by side at block 36: each form less the night without replay with the job's paired SE, and less H-47's plain
   night with exploration 018's unpaired SE, each with its stimulus split, beside replay after block 6 alone and H-47's clause 1.

**A checkpoint is read after the night that follows its block**, and block 36's after the twelve blocks that follow the last night. So a
form's rows are H-47's night without replay's at every checkpoint before its replayed night's block: there the form less the night without
replay is written "= without replay", and the form less H-47's plain night is H-47's own night without replay less its plain night, with
H-47's paired SE, marked (H-47). From its replayed night's block on, its rows part from both of H-47's forms. At the checkpoint right after
its replayed night, the form less the night without replay is that night's replay alone, as exploration 030 reads it: until then the two
ran the same nights on the same images. At block 36 the blocks since the one replayed night differ by form: 30 after block 6 alone, 24
after block 12 alone, 18 after block 18 alone and 12 after block 24 alone.

**The side** is [ADR-0238](../adr/0238-what-keeps-a-mapping-where-the-replay-did-not.md)'s: at a checkpoint, the stimulus whose
difference from the night without replay is the larger of the two, and neither where they are equal. The note writes no prediction and
decides nothing. **Every figure here is exploratory**: from one run of each test on the repository's AArch64 host, in H-47's terms. A count
is a mean over the cell's 48 reads, sixteen on each of three paths, and a difference carries its paired SE unless another is named. A
difference is written "beyond 2 SE" where it lies beyond twice its SE; one written without it carries its sign alone.

**In brief**
- **Each job ran H-47's rows where its nights were H-47's, and each form parts from H-47's forms at its replayed night.** Inside the test
  every row of the night without replay held H-47's, fifteen a cell, and each form's rows before its replayed night's block held H-47's
  night without replay's, eighteen a cell. The trial held all four forms' first nights to H-47's row at block 6 on path 0. Each replayed
  night read 1 340 to 1 341 replays a path and every other night none.
- **Right after its replayed night**, less the night without replay, after block 12 alone lies above beyond twice the paired SE in seven
  cells, after block 18 alone in seven and after block 24 alone in six; each lies below on seed 24's eighth image from the assignment,
  −7.3 ± 0.88, −12.1 ± 0.63 and −12.3 ± 0.58, and after block 12 alone on seed 23's eighth image from the assignment too, −1.9 ± 0.88.
- **At block 36, less the night without replay**, after block 12 alone lies above beyond 2 SE in eight cells, within in three and below
  in one; after block 18 alone above in seven, within in four and below in one; after block 24 alone above in three, within in seven and
  below in two. After block 6 alone lay above in five, within in six and below in one (exploration 031).
- **Where A answers readout 0**, after block 12 alone lies above the night without replay at block 36 beyond 2 SE in all six cells, after
  block 18 alone in five and after block 24 alone in three; **where A answers readout 1**, in two, two and none.
- **Less H-47's plain night at block 36**, beyond twice exploration 018's unpaired SE, after block 12 alone and after block 18 alone lie
  below in eight cells each and above in one, seed 23's first mapping from the mirrored assignment; after block 24 alone lies below in
  eleven and within in that one.
- **The four single nights' differences at block 36 sum** to −14.3 to +26.8 on A's count, above H-47's plain night less the night without
  replay in eight cells and below in four; no SE is claimed for a sum.
- **Each form's replayed night moved the readouts' synapses about as H-47's first plain night did**, whichever block it followed, and more
  than H-47's plain night after the same block in every cell. Each night after a form's replayed night, without replay, left each
  inhibitory column of the class matrix lower than H-47's night without replay after the same block, all 144, and so did each stretch of
  blocks after it, all 288.
- **The side at block 36** falls on H-47's stimulus in ten cells under after block 12 alone and after block 18 alone and in nine under
  after block 24 alone. **The pair** stands at 61.8 to 63.5 at block 36 under the three forms, and A's highest coupling at 1.078 to 1.179 of
  H-42's, below H-47's plain night's in all twelve cells.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 020 to 031 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. A cell is named, as in H-47, by its seed, its image and the
arm it starts from, not by the mapping in force. Nights follow blocks 6, 12, 18 and 24, then 12 blocks run with none to block 36, on paths
0, 5 and 6. Each checkpoint reads A and the pair sixteen times each.

**Places.** A period of the ring is 20 units. Stimulus A presents place 0 of each period and B place 11. **The odd places** are the odd
places but 11 and **the even places** the even places but 0: H-47's readout 0 and readout 1, each with its excitatory and its inhibitory
units. C's own units lie at place 5, among the odd places, and D's at place 16, among the even, in every cell
([exploration 021](021-where-the-gain-falls.md), section 4).

**Twelve cells**, all of H-47's, in exploration 031's order: the six where A answers readout 0 first, then the six where it answers the
other readout. A answers readout 0 on the eighth images from the mirrored assignment and on the first mappings from the assignment, as each
cell's reads of A print it:

| Cell | A answers | B answers | H-47: plain less without replay at block 36 (clause 1) | After block 6 alone less without replay at block 36 (031) | Its four forms | After block 6 alone | H-47's class matrices and night lines |
| :--- | :--- | :--- | ---: | ---: | :--- | :--- | :--- |
| Seed 22, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +19.6 ± 1.06 | +7.3 ± 0.65 | this exploration's | 031's | 021's |
| Seed 22, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +7.9 ± 0.51 | +4.3 ± 1.05 | this exploration's | 031's | 021's |
| Seed 23, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +17.4 ± 1.08 | +4.3 ± 0.63 | this exploration's | 031's | 021's |
| Seed 23, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +13.4 ± 1.56 | +5.0 ± 2.61 | this exploration's | 031's | 021's |
| Seed 24, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +11.3 ± 0.96 | +3.1 ± 1.84 | this exploration's | 031's | 021's |
| Seed 24, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +3.8 ± 1.57 | +3.9 ± 0.76 | this exploration's | 031's | 021's |
| Seed 23, eighth, assignment | readout 1, the even places | readout 0, the odd places | +4.1 ± 1.45 | +1.3 ± 1.04 | this exploration's | 031's | 021's |
| Seed 24, first mapping, mirrored | readout 1, the even places | readout 0, the odd places | +6.0 ± 1.54 | +3.2 ± 2.06 | this exploration's | 031's | 021's |
| Seed 23, first mapping, mirrored | readout 1, the even places | readout 0, the odd places | −4.9 ± 1.09 | +1.7 ± 0.78 | 030's one-night-alone job | 030's | 020's |
| Seed 24, eighth, assignment | readout 1, the even places | readout 0, the odd places | +1.8 ± 1.79 | −2.0 ± 0.88 | this exploration's | 031's | 020's |
| Seed 22, first mapping, mirrored | readout 1, the even places | readout 0, the odd places | +5.0 ± 1.24 | +1.3 ± 2.40 | this exploration's | 031's | 020's |
| Seed 22, eighth, assignment | readout 1, the even places | readout 0, the odd places | +2.2 ± 0.65 | −0.9 ± 1.07 | this exploration's | 031's | 020's |

**A night that replays** is H-47's plain night on A's patterns: A's 102 patterns, drawn from the start image, tagged at priority 200.
**Every other night** is H-47's night without replay, the same night with nothing tagged. No hold is set. Each night is chosen by its form
(`E030Form`, exploration 030's), and every night of every form is watched. **Four forms a cell** (`E032_FORMS`), with replay after block 6
alone and H-47's plain night read beside them, each with its name in the tables and in the logs:

| Form | In the logs | Night after block 6 | after 12 | after 18 | after 24 | Job | Rows held inside its job to H-47's, on every path |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Without replay | `no-replay` | without replay | without replay | without replay | without replay | this exploration's | the night without replay's, blocks 6 to 36 |
| After block 12 alone | `alone-12` | without replay | replays | without replay | without replay | this exploration's | the night without replay's, block 6 |
| After block 18 alone | `alone-18` | without replay | without replay | replays | without replay | this exploration's | the night without replay's, blocks 6 and 12 |
| After block 24 alone | `alone-24` | without replay | without replay | without replay | replays | this exploration's | the night without replay's, blocks 6, 12 and 18 |
| After block 6 alone | `alone-6` | replays | without replay | without replay | without replay | exploration 031's | the plain night's, block 6 |
| Plain on A's (H-47) | `plain-first` | replays | replays | replays | replays | H-47's | H-47's own |

- **The night without replay** is the job's control: inside the job it holds that the harness runs H-47's rows on the cell, and it pairs
  the forms for their SEs.
- **After block 6 alone** is read from exploration 031's job on the cell, and from exploration 030's on seed 23's first mapping from the
  mirrored assignment; it is not run here. Exploration 031's night without replay carried H-47's count and masks in every read, and this
  job's is held to H-47's rows, so after block 6 alone less the night without replay, with its paired SE from exploration 031's job, is the
  same difference against this job's (section "The runs are laid out as written" sets the reads side by side).
- **H-47's plain night on A's patterns** (`plain-first`, plain on A's in the tables) is read from H-47's job, its rows and reads, and from
  exploration 020's or 021's job on the same cell, its class matrices, night lines and watch; it is not run here.

**One job a cell** (`e030_job`), exploration 030's job with the four forms, on the eleven cells but seed 23's first mapping from the
mirrored assignment, where the four forms are exploration 030's and are read from its one-night-alone job. The job writes A's patterns and
their targets, the split and the class matrix of the start image, of the image before each night and of the image after it, and of block
36's image; it reads each night with `ReplayWatch`. Inside the test (`e030_night`), each form's night after its block reads replays and
depotentiations that are not zero, and every other night of every form reads `replays 0` and `depotentiations 0`. Every row of the night
without replay, and each form's rows before its replayed night's block on each path, are held inside the test to H-47's night without
replay's row on the same path and block (`held_to`): fifteen and eighteen a cell. A row is the image's CRC-64, A's and the pair's sixteen
counts, A's couplings, their highest over the stretch before the checkpoint, and the gain at the night's start and end. Beside each form the
job prints how many of its fifteen rows are each of H-47's two forms' rows. Each job ran its own head, the pair's first six blocks on each
path, and the head is read once.

**The class matrix** is exploration 020's, read by group as exploration 030 reads it. **The odd places' excitatory units** are classes 4, 6
and 7, C's own and readout 0's two classes, and **the even places'** classes 5, 8 and 9, D's own and readout 1's two; the inhibitory units
are class 11 in the odd places, 12 in the even places and 13 elsewhere. An inhibitory entry is a magnitude, the weight negated, so a + is
more inhibition. **A night's change** is the image after it less the image before it; **a stretch's** the image before the next night, or
block 36's, less the image after the night before it; **the head's** the image before the first night less the start image. Each is the
mean over the three paths.

**A kind's margin onto A's answer** is exploration 020's, from the split: the kind's weight onto the readout mapping A asks of its stimulus
less onto the other, A's kinds and C's own against stimulus A's answer, B's kinds and D's own against B's. **A's margin** is the sum of
four kinds', A alone, A and C, B alone, and B and D.

**The SEs.** A form less the night without replay carries H-47's paired SE over its 48 differences, the night without replay of its own
job. A form less H-47's plain night, run in another job, carries exploration 018's unpaired SE: the mean over the paths of each path's
difference of means, ± √((the two forms' stream variances)/16 + (the paths' differences' SD)²/3), a negative variance as zero. Where a
form's rows at a checkpoint are H-47's night without replay's, the first is written "= without replay" and the second is H-47's night
without replay less its plain night with H-47's paired SE, marked (H-47). H-47's plain night less its night without replay is H-47's own,
with H-47's paired SE, marked (H-47). A count of cells or checkpoints within or beyond twice an SE is taken on the unrounded mean and SE.

## Before a job ran

ADR-0247 asks whether any form but the night without replay must run bit for bit as one already in the logs. **On seed 23's first mapping
from the mirrored assignment all three must, and it ran no job there**: exploration 030's one-night-alone job ran the night without replay
and the three forms on that cell, on the same engine, harness and paths, and they are read from it. **On the other eleven cells none
must.** There the logs hold H-47's four forms, the nights of explorations 020 to 029, in H-47's layout or another, with or without a hold,
and exploration 031's two forms; each of H-47's forms and of explorations 020 to 029's replays on all four nights or on none, and
exploration 031's form replays after block 6 alone. Each form here replays on one later night alone, so at that night it runs H-47's plain
night on an image that only the night without replay's chain wrote:

| Form | Its rows are H-47's up to block | The first night where it parts from H-47's forms | That night runs on |
| :--- | ---: | :--- | :--- |
| After block 12 alone | 6, the night without replay's | the night after block 12, which replays | the night without replay's image after blocks 7 to 12 |
| After block 18 alone | 12, the night without replay's | the night after block 18, which replays | the night without replay's image after blocks 13 to 18 |
| After block 24 alone | 18, the night without replay's | the night after block 24, which replays | the night without replay's image after blocks 19 to 24 |

No log holds H-47's plain night on an image the night without replay's chain wrote after block 6 on any of the eleven cells: H-47's plain
night's later nights run on images its own earlier nights wrote, and exploration 031's form replays on the head's image alone. So each
form's rows from its replayed night's block are in no log there, and the eleven jobs ran.

## The runs are laid out as written

- **The trial** ran on path 0 of seed 23's first mapping from the assignment, a cell that runs a job, the pair's head and each form's first
  night, all four watched, each image split and read once as it was, of A and of the pair. Each form's first night is without replay, so
  each image, those two reads, A's couplings, their highest over the head and the gain held, inside the test, H-47's night without replay's
  row at block 6 on path 0:

| Form | Its first night | H-47's row at block 6 on path 0 it holds | Image's CRC-64, H-47's row held inside the test | A's read as it was | The pair's | A's couplings | Gain over the night | Replays | Depotentiations | Watch: wrong | Its lines that are 021's job's night without replay after block 6 on path 0, label aside |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | :--- | ---: | ---: | ---: | :--- |
| Without replay | without replay | the night without replay's | 1049472092185774887 | 50 | 31 | 7 096 340, 5 666 987, 6 345 105, 6 901 365 | 112 276 to 114 144 | 0 | 0 | 0 | 91 of 91 |
| After block 12 alone | without replay | the night without replay's | 1049472092185774887 | 50 | 31 | 7 096 340, 5 666 987, 6 345 105, 6 901 365 | 112 276 to 114 144 | 0 | 0 | 0 | 91 of 91 |
| After block 18 alone | without replay | the night without replay's | 1049472092185774887 | 50 | 31 | 7 096 340, 5 666 987, 6 345 105, 6 901 365 | 112 276 to 114 144 | 0 | 0 | 0 | 91 of 91 |
| After block 24 alone | without replay | the night without replay's | 1049472092185774887 | 50 | 31 | 7 096 340, 5 666 987, 6 345 105, 6 901 365 | 112 276 to 114 144 | 0 | 0 | 0 | 91 of 91 |

- Each of the four nights' lines, its start, its windows, its watch and its end, are the lines of exploration 021's job's night without
  replay after block 6 on path 0 of the same cell, the label and the form's name aside: 91 of 91 for each. So is the head's line. Each
  night read 0 replays and 0 depotentiations, as the trial prints, and inside the test each was held to read none of either; the watch
  read `wrong 0` in all four. The trial runs no night that replays: each form's replayed night first ran in the jobs, where `e030_night`
  held its replays and depotentiations not zero.

- **Each of the eleven jobs** ran to the end, its four forms on the three paths. Inside the test, every row of the night without replay held
  H-47's row of its form on the same path and block, fifteen a cell, and each form's rows before its replayed night's block held H-47's
  night without replay's, eighteen a cell: three of after block 12 alone, six of after block 18 alone and nine of after block 24 alone.
  Beside each form the job prints how many of its fifteen rows are H-47's rows of either form. Set beside H-47's rows and lines here, cell
  by cell, with seed 23's first mapping from the mirrored assignment read from exploration 030's job:

| Cell | Without replay: rows H-47's | After block 12 alone: rows H-47's night without replay's, its plain night's | After block 18 alone | After block 24 alone | Without replay: lines in H-47's lines job, label aside | Heads: lines in that job | The forms' nights before their replayed night: start, window, watch and end lines that are that job's night without replay's | Their replayed and later nights: end lines among that job's, either form, this job's other forms' and after block 6 alone's | Lines job |
| :--- | :--- | ---: | ---: | ---: | :--- | :--- | :--- | :--- | :--- |
| Seed 22, eighth, mirrored | 15 of 15 | 3, 0 | 6, 0 | 9, 0 | 1 647 of 1 647 | 3 of 3 | 1 566 of 1 566 | 0 of 18 | 021 |
| Seed 22, first mapping, assignment | 15 of 15 | 3, 0 | 6, 0 | 9, 0 | 1 647 of 1 647 | 3 of 3 | 1 566 of 1 566 | 0 of 18 | 021 |
| Seed 23, eighth, mirrored | 15 of 15 | 3, 0 | 6, 0 | 9, 0 | 1 695 of 1 695 | 3 of 3 | 1 638 of 1 638 | 0 of 18 | 021 |
| Seed 23, first mapping, assignment | 15 of 15 | 3, 0 | 6, 0 | 9, 0 | 1 695 of 1 695 | 3 of 3 | 1 638 of 1 638 | 0 of 18 | 021 |
| Seed 24, eighth, mirrored | 15 of 15 | 3, 0 | 6, 0 | 9, 0 | 1 671 of 1 671 | 3 of 3 | 1 602 of 1 602 | 0 of 18 | 021 |
| Seed 24, first mapping, assignment | 15 of 15 | 3, 0 | 6, 0 | 9, 0 | 1 671 of 1 671 | 3 of 3 | 1 602 of 1 602 | 0 of 18 | 021 |
| Seed 23, eighth, assignment | 15 of 15 | 3, 0 | 6, 0 | 9, 0 | 1 695 of 1 695 | 3 of 3 | 1 638 of 1 638 | 0 of 18 | 021 |
| Seed 24, first mapping, mirrored | 15 of 15 | 3, 0 | 6, 0 | 9, 0 | 1 671 of 1 671 | 3 of 3 | 1 602 of 1 602 | 0 of 18 | 021 |
| Seed 23, first mapping, mirrored | 15 of 15 | 3, 0 | 6, 0 | 9, 0 | 1 695 of 1 695 | 3 of 3 | 1 638 of 1 638 | 0 of 18 | 020 |
| Seed 24, eighth, assignment | 15 of 15 | 3, 0 | 6, 0 | 9, 0 | 1 671 of 1 671 | 3 of 3 | 1 602 of 1 602 | 0 of 18 | 020 |
| Seed 22, first mapping, mirrored | 15 of 15 | 3, 0 | 6, 0 | 9, 0 | 1 647 of 1 647 | 3 of 3 | 1 566 of 1 566 | 0 of 18 | 020 |
| Seed 22, eighth, assignment | 15 of 15 | 3, 0 | 6, 0 | 9, 0 | 1 647 of 1 647 | 3 of 3 | 1 566 of 1 566 | 0 of 18 | 020 |

- **The night without replay's lines** are 1 659 to 1 707 a job. Of them 1 647 to 1 695 are the lines of H-47's night without replay in
  exploration 020's or 021's job on the same cell, label aside, all of that job's; the other 12 are the job's notices that each night read
  what its form asks. The pair's heads, three lines a job, are that job's. So the night without replay's class matrices, splits and night
  lines are those jobs': section 5 lists H-47's night without replay from them, and section 6 this job's, line for line the same.
- **Each form's nights before its replayed night**, six a cell on each path, three of after block 24 alone, two of after block 18 alone and
  one of after block 12 alone, print on the three paths the lines of that job's night without replay after the same block, label and form
  aside, its start, windows, watch and end: 1 566 to 1 638 lines a cell, all of them. Each form's images before its replayed night, 45 a
  cell, read in the class matrix what the night without replay's read, and the night without replay's 27 images a cell what H-47's night
  without replay's read in that job.
- **Each form's replayed night and the nights after it**, 18 a cell on the three paths, none ends on the end line of that job's night
  without replay or plain night after the same block, of another form of its own job, or of after block 6 alone.
- **The rows.** Of each form's fifteen rows in each cell, those before its replayed night's block, three, six and nine, are H-47's night
  without replay's, and none of its rows is H-47's plain night's; the night without replay's fifteen are H-47's night without replay's and
  none its plain night's, as each job prints and as the rows set beside H-47's read.
- **The nights.** The forms' 108 replayed nights, nine a cell, read 1 340 to 1 341 replays and 640 depotentiations each, as the jobs print,
  held inside the test to be not zero; the other 468 nights of the four forms read `replays 0` and `depotentiations 0`, held inside the
  test, exploration 030's job's among them. The watch read `wrong 0` in all 576 nights, over 116 315 040 pairings, a tally over the twelve
  jobs. Each job's log prints H-42's couplings on its seed as H-47's, the lines job's and after block 6 alone's job's logs print them.

The reads, set beside H-47's and beside the job that after block 6 alone is read from, exploration 031's (on seed 23's first mapping from
the mirrored assignment exploration 030's, the same job as the forms'):

| Cell | Without replay: reads with H-47's count and `stimuli`, `right` and `tied` masks | Reads that are after block 6 alone's job's night without replay's, the same way | The forms' reads before their replayed night that are the night without replay's, trace too | Every read with H-47's plain night's `stimuli` mask | After block 6 alone's job |
| :--- | :--- | :--- | :--- | :--- | :--- |
| Seed 22, eighth, mirrored | 480 of 480 | 480 of 480 | 576 of 576 | 1 920 of 1 920 | 031's |
| Seed 22, first mapping, assignment | 480 of 480 | 480 of 480 | 576 of 576 | 1 920 of 1 920 | 031's |
| Seed 23, eighth, mirrored | 480 of 480 | 480 of 480 | 576 of 576 | 1 920 of 1 920 | 031's |
| Seed 23, first mapping, assignment | 480 of 480 | 480 of 480 | 576 of 576 | 1 920 of 1 920 | 031's |
| Seed 24, eighth, mirrored | 480 of 480 | 480 of 480 | 576 of 576 | 1 920 of 1 920 | 031's |
| Seed 24, first mapping, assignment | 480 of 480 | 480 of 480 | 576 of 576 | 1 920 of 1 920 | 031's |
| Seed 23, eighth, assignment | 480 of 480 | 480 of 480 | 576 of 576 | 1 920 of 1 920 | 031's |
| Seed 24, first mapping, mirrored | 480 of 480 | 480 of 480 | 576 of 576 | 1 920 of 1 920 | 031's |
| Seed 23, first mapping, mirrored | 480 of 480 | 480 of 480 | 576 of 576 | 1 920 of 1 920 | 030's |
| Seed 24, eighth, assignment | 480 of 480 | 480 of 480 | 576 of 576 | 1 920 of 1 920 | 031's |
| Seed 22, first mapping, mirrored | 480 of 480 | 480 of 480 | 576 of 576 | 1 920 of 1 920 | 031's |
| Seed 22, eighth, assignment | 480 of 480 | 480 of 480 | 576 of 576 | 1 920 of 1 920 | 031's |

- **Each read of the night without replay** carries H-47's read's count and its `stimuli`, `right` and `tied` masks, 480 a cell, and so
  does each read of exploration 031's night without replay: its 480 reads a cell are this job's, read for read. So after block 6 alone less
  the night without replay, with its paired SE from exploration 031's job, is the same figure against this job's night without replay, on
  A's count, each stimulus's trials and the ties, at every checkpoint of every cell.
- **Each form's reads before its replayed night** are the night without replay's, their count, masks and trace, 576 a cell, so the form
  less the night without replay there is zero read for read, written "= without replay".
- **Every read of the four forms** carries the `stimuli` mask of H-47's plain night's read of the same path, checkpoint, mapping and stream,
  1 920 a cell, so each trial presented the stimulus it presented in H-47.

## 1. A's count and each stimulus's trials under each form

A's count at each checkpoint, the mean of 48 reads, under the night without replay and the three forms, this exploration's jobs, and under
H-47's plain night, H-47's job's:

| Cell | Block | Without replay | After block 12 alone | After block 18 alone | After block 24 alone | Plain on A's (H-47) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 46.3 | 46.3 | 46.3 | 46.3 | 53.3 |
| Seed 22, eighth, mirrored | 12 | 30.2 | 39.2 | 30.2 | 30.2 | 42.3 |
| Seed 22, eighth, mirrored | 18 | 26.3 | 33.1 | 31.8 | 26.3 | 41.8 |
| Seed 22, eighth, mirrored | 24 | 22.6 | 29.5 | 27.2 | 27.2 | 40.2 |
| Seed 22, eighth, mirrored | 36 | 16.6 | 24.7 | 22.9 | 21.6 | 36.2 |
| Seed 22, first mapping, assignment | 6 | 52.4 | 52.4 | 52.4 | 52.4 | 56.5 |
| Seed 22, first mapping, assignment | 12 | 44.8 | 49.3 | 44.8 | 44.8 | 47.6 |
| Seed 22, first mapping, assignment | 18 | 40.2 | 45.5 | 46.7 | 40.2 | 46.9 |
| Seed 22, first mapping, assignment | 24 | 38.9 | 42.8 | 40.3 | 42.5 | 48.0 |
| Seed 22, first mapping, assignment | 36 | 36.3 | 39.6 | 38.0 | 37.1 | 44.3 |
| Seed 23, eighth, mirrored | 6 | 34.4 | 34.4 | 34.4 | 34.4 | 36.1 |
| Seed 23, eighth, mirrored | 12 | 19.5 | 20.5 | 19.5 | 19.5 | 28.5 |
| Seed 23, eighth, mirrored | 18 | 14.9 | 20.1 | 16.8 | 14.9 | 27.6 |
| Seed 23, eighth, mirrored | 24 | 10.6 | 14.5 | 13.5 | 11.4 | 29.3 |
| Seed 23, eighth, mirrored | 36 | 8.2 | 12.0 | 11.2 | 10.6 | 25.6 |
| Seed 23, first mapping, assignment | 6 | 54.2 | 54.2 | 54.2 | 54.2 | 56.3 |
| Seed 23, first mapping, assignment | 12 | 33.4 | 37.9 | 33.4 | 33.4 | 37.6 |
| Seed 23, first mapping, assignment | 18 | 27.2 | 33.1 | 31.5 | 27.2 | 35.0 |
| Seed 23, first mapping, assignment | 24 | 24.8 | 30.9 | 30.3 | 26.9 | 34.8 |
| Seed 23, first mapping, assignment | 36 | 19.7 | 26.3 | 25.3 | 25.5 | 33.0 |
| Seed 24, eighth, mirrored | 6 | 36.4 | 36.4 | 36.4 | 36.4 | 33.8 |
| Seed 24, eighth, mirrored | 12 | 31.5 | 32.3 | 31.5 | 31.5 | 28.6 |
| Seed 24, eighth, mirrored | 18 | 20.2 | 26.3 | 29.3 | 20.2 | 28.6 |
| Seed 24, eighth, mirrored | 24 | 16.4 | 21.4 | 20.9 | 21.8 | 25.8 |
| Seed 24, eighth, mirrored | 36 | 12.5 | 16.3 | 18.0 | 14.4 | 23.8 |
| Seed 24, first mapping, assignment | 6 | 58.3 | 58.3 | 58.3 | 58.3 | 61.2 |
| Seed 24, first mapping, assignment | 12 | 52.0 | 54.6 | 52.0 | 52.0 | 54.8 |
| Seed 24, first mapping, assignment | 18 | 47.5 | 48.8 | 51.5 | 47.5 | 51.3 |
| Seed 24, first mapping, assignment | 24 | 43.7 | 47.9 | 48.1 | 52.1 | 52.4 |
| Seed 24, first mapping, assignment | 36 | 40.1 | 43.1 | 42.7 | 41.7 | 43.8 |
| Seed 23, eighth, assignment | 6 | 47.4 | 47.4 | 47.4 | 47.4 | 50.4 |
| Seed 23, eighth, assignment | 12 | 40.8 | 38.9 | 40.8 | 40.8 | 43.4 |
| Seed 23, eighth, assignment | 18 | 37.3 | 35.0 | 40.2 | 37.3 | 45.2 |
| Seed 23, eighth, assignment | 24 | 38.0 | 32.5 | 37.3 | 39.4 | 43.7 |
| Seed 23, eighth, assignment | 36 | 36.2 | 30.6 | 35.0 | 34.6 | 40.3 |
| Seed 24, first mapping, mirrored | 6 | 54.3 | 54.3 | 54.3 | 54.3 | 57.7 |
| Seed 24, first mapping, mirrored | 12 | 46.3 | 48.4 | 46.3 | 46.3 | 51.0 |
| Seed 24, first mapping, mirrored | 18 | 44.0 | 47.9 | 43.0 | 44.0 | 51.8 |
| Seed 24, first mapping, mirrored | 24 | 44.4 | 48.7 | 45.4 | 46.4 | 52.0 |
| Seed 24, first mapping, mirrored | 36 | 42.7 | 47.4 | 43.0 | 44.0 | 48.7 |
| Seed 23, first mapping, mirrored | 6 | 55.0 | 55.0 | 55.0 | 55.0 | 56.9 |
| Seed 23, first mapping, mirrored | 12 | 52.4 | 54.0 | 52.4 | 52.4 | 54.4 |
| Seed 23, first mapping, mirrored | 18 | 52.0 | 52.9 | 52.6 | 52.0 | 52.4 |
| Seed 23, first mapping, mirrored | 24 | 53.5 | 54.0 | 53.8 | 53.2 | 50.9 |
| Seed 23, first mapping, mirrored | 36 | 53.1 | 52.7 | 53.4 | 50.8 | 48.3 |
| Seed 24, eighth, assignment | 6 | 38.8 | 38.8 | 38.8 | 38.8 | 39.1 |
| Seed 24, eighth, assignment | 12 | 34.8 | 27.5 | 34.8 | 34.8 | 34.2 |
| Seed 24, eighth, assignment | 18 | 34.4 | 29.9 | 22.3 | 34.4 | 28.5 |
| Seed 24, eighth, assignment | 24 | 35.6 | 29.0 | 26.7 | 23.3 | 29.6 |
| Seed 24, eighth, assignment | 36 | 25.6 | 23.5 | 20.1 | 20.9 | 27.4 |
| Seed 22, first mapping, mirrored | 6 | 56.6 | 56.6 | 56.6 | 56.6 | 57.7 |
| Seed 22, first mapping, mirrored | 12 | 49.5 | 51.0 | 49.5 | 49.5 | 52.5 |
| Seed 22, first mapping, mirrored | 18 | 49.0 | 50.3 | 50.8 | 49.0 | 52.7 |
| Seed 22, first mapping, mirrored | 24 | 45.9 | 47.2 | 48.2 | 48.9 | 50.6 |
| Seed 22, first mapping, mirrored | 36 | 44.1 | 44.7 | 47.3 | 45.9 | 49.1 |
| Seed 22, eighth, assignment | 6 | 50.3 | 50.3 | 50.3 | 50.3 | 52.3 |
| Seed 22, eighth, assignment | 12 | 47.6 | 51.4 | 47.6 | 47.6 | 50.6 |
| Seed 22, eighth, assignment | 18 | 47.2 | 49.2 | 51.4 | 47.2 | 50.6 |
| Seed 22, eighth, assignment | 24 | 47.3 | 48.0 | 49.8 | 48.3 | 50.2 |
| Seed 22, eighth, assignment | 36 | 44.2 | 45.3 | 46.8 | 43.1 | 46.4 |

Each form less the night without replay, ± the paired SE, "= without replay" where the form's rows are the night without replay's; each
form less H-47's plain night, ± the unpaired SE, and where the form's rows are the night without replay's H-47's night without replay less
its plain night, marked (H-47); and H-47's plain night less its night without replay, H-47's own:

| Cell | Block | After block 12 alone less without replay | after 18 alone | after 24 alone | After block 12 alone less plain | after 18 alone | after 24 alone | H-47: plain less without replay |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | = without replay | = without replay | = without replay | −7.1 ± 0.91 (H-47) | −7.1 ± 0.91 (H-47) | −7.1 ± 0.91 (H-47) | +7.1 ± 0.91 (H-47) |
| Seed 22, eighth, mirrored | 12 | +9.0 ± 0.47 | = without replay | = without replay | −3.1 ± 1.14 | −12.1 ± 0.85 (H-47) | −12.1 ± 0.85 (H-47) | +12.1 ± 0.85 (H-47) |
| Seed 22, eighth, mirrored | 18 | +6.9 ± 1.15 | +5.5 ± 1.13 | = without replay | −8.7 ± 0.77 | −10.0 ± 2.25 | −15.5 ± 1.67 (H-47) | +15.5 ± 1.67 (H-47) |
| Seed 22, eighth, mirrored | 24 | +6.9 ± 0.93 | +4.6 ± 0.91 | +4.6 ± 0.53 | −10.8 ± 1.38 | −13.1 ± 1.68 | −13.0 ± 1.78 | +17.6 ± 1.49 (H-47) |
| Seed 22, eighth, mirrored | 36 | +8.1 ± 0.74 | +6.3 ± 0.74 | +5.1 ± 1.31 | −11.5 ± 1.59 | −13.4 ± 0.74 | −14.6 ± 1.71 | +19.6 ± 1.06 (H-47) |
| Seed 22, first mapping, assignment | 6 | = without replay | = without replay | = without replay | −4.1 ± 0.82 (H-47) | −4.1 ± 0.82 (H-47) | −4.1 ± 0.82 (H-47) | +4.1 ± 0.82 (H-47) |
| Seed 22, first mapping, assignment | 12 | +4.6 ± 0.82 | = without replay | = without replay | +1.7 ± 1.05 | −2.9 ± 0.56 (H-47) | −2.9 ± 0.56 (H-47) | +2.9 ± 0.56 (H-47) |
| Seed 22, first mapping, assignment | 18 | +5.3 ± 1.12 | +6.5 ± 1.17 | = without replay | −1.4 ± 1.78 | −0.3 ± 1.35 | −6.8 ± 1.12 (H-47) | +6.8 ± 1.12 (H-47) |
| Seed 22, first mapping, assignment | 24 | +3.9 ± 1.31 | +1.4 ± 0.80 | +3.6 ± 1.11 | −5.2 ± 1.40 | −7.8 ± 1.07 | −5.5 ± 1.47 | +9.1 ± 1.37 (H-47) |
| Seed 22, first mapping, assignment | 36 | +3.2 ± 0.58 | +1.7 ± 0.52 | +0.8 ± 1.29 | −4.7 ± 0.55 | −6.3 ± 0.46 | −7.1 ± 0.83 | +7.9 ± 0.51 (H-47) |
| Seed 23, eighth, mirrored | 6 | = without replay | = without replay | = without replay | −1.7 ± 1.20 (H-47) | −1.7 ± 1.20 (H-47) | −1.7 ± 1.20 (H-47) | +1.7 ± 1.20 (H-47) |
| Seed 23, eighth, mirrored | 12 | +1.0 ± 0.76 | = without replay | = without replay | −7.9 ± 1.26 | −9.0 ± 1.07 (H-47) | −9.0 ± 1.07 (H-47) | +9.0 ± 1.07 (H-47) |
| Seed 23, eighth, mirrored | 18 | +5.3 ± 0.69 | +2.0 ± 0.87 | = without replay | −7.5 ± 1.11 | −10.7 ± 1.36 | −12.7 ± 1.02 (H-47) | +12.7 ± 1.02 (H-47) |
| Seed 23, eighth, mirrored | 24 | +3.9 ± 0.75 | +2.9 ± 0.65 | +0.8 ± 1.09 | −14.8 ± 1.22 | −15.8 ± 1.28 | −17.8 ± 1.87 | +18.7 ± 1.62 (H-47) |
| Seed 23, eighth, mirrored | 36 | +3.8 ± 0.82 | +3.0 ± 0.64 | +2.4 ± 0.35 | −13.6 ± 1.05 | −14.4 ± 0.93 | −15.0 ± 1.16 | +17.4 ± 1.08 (H-47) |
| Seed 23, first mapping, assignment | 6 | = without replay | = without replay | = without replay | −2.0 ± 0.79 (H-47) | −2.0 ± 0.79 (H-47) | −2.0 ± 0.79 (H-47) | +2.0 ± 0.79 (H-47) |
| Seed 23, first mapping, assignment | 12 | +4.4 ± 1.52 | = without replay | = without replay | +0.3 ± 1.46 | −4.1 ± 1.18 (H-47) | −4.1 ± 1.18 (H-47) | +4.1 ± 1.18 (H-47) |
| Seed 23, first mapping, assignment | 18 | +5.9 ± 2.54 | +4.3 ± 1.43 | = without replay | −1.9 ± 1.67 | −3.5 ± 2.21 | −7.9 ± 2.54 (H-47) | +7.9 ± 2.54 (H-47) |
| Seed 23, first mapping, assignment | 24 | +6.1 ± 1.72 | +5.5 ± 1.08 | +2.1 ± 0.89 | −3.9 ± 1.39 | −4.4 ± 1.56 | −7.9 ± 1.76 | +10.0 ± 1.90 (H-47) |
| Seed 23, first mapping, assignment | 36 | +6.6 ± 1.68 | +5.6 ± 1.11 | +5.8 ± 0.54 | −6.8 ± 1.88 | −7.7 ± 1.97 | −7.5 ± 1.59 | +13.4 ± 1.56 (H-47) |
| Seed 24, eighth, mirrored | 6 | = without replay | = without replay | = without replay | +2.6 ± 1.35 (H-47) | +2.6 ± 1.35 (H-47) | +2.6 ± 1.35 (H-47) | −2.6 ± 1.35 (H-47) |
| Seed 24, eighth, mirrored | 12 | +0.8 ± 1.09 | = without replay | = without replay | +3.6 ± 1.01 | +2.9 ± 1.19 (H-47) | +2.9 ± 1.19 (H-47) | −2.9 ± 1.19 (H-47) |
| Seed 24, eighth, mirrored | 18 | +6.1 ± 0.77 | +9.1 ± 0.92 | = without replay | −2.3 ± 1.78 | +0.8 ± 1.68 | −8.3 ± 1.70 (H-47) | +8.3 ± 1.70 (H-47) |
| Seed 24, eighth, mirrored | 24 | +4.9 ± 1.23 | +4.4 ± 1.23 | +5.4 ± 2.02 | −4.4 ± 1.15 | −4.9 ± 1.24 | −4.0 ± 1.40 | +9.4 ± 1.58 (H-47) |
| Seed 24, eighth, mirrored | 36 | +3.8 ± 0.78 | +5.4 ± 1.17 | +1.9 ± 1.18 | −7.5 ± 0.98 | −5.8 ± 1.20 | −9.4 ± 1.39 | +11.3 ± 0.96 (H-47) |
| Seed 24, first mapping, assignment | 6 | = without replay | = without replay | = without replay | −2.9 ± 0.72 (H-47) | −2.9 ± 0.72 (H-47) | −2.9 ± 0.72 (H-47) | +2.9 ± 0.72 (H-47) |
| Seed 24, first mapping, assignment | 12 | +2.7 ± 0.67 | = without replay | = without replay | −0.1 ± 1.09 | −2.8 ± 0.93 (H-47) | −2.8 ± 0.93 (H-47) | +2.8 ± 0.93 (H-47) |
| Seed 24, first mapping, assignment | 18 | +1.4 ± 0.68 | +4.1 ± 1.17 | = without replay | −2.5 ± 1.50 | +0.2 ± 2.10 | −3.9 ± 1.41 (H-47) | +3.9 ± 1.41 (H-47) |
| Seed 24, first mapping, assignment | 24 | +4.2 ± 1.90 | +4.4 ± 0.81 | +8.4 ± 1.47 | −4.5 ± 1.73 | −4.3 ± 2.62 | −0.3 ± 2.37 | +8.7 ± 3.17 (H-47) |
| Seed 24, first mapping, assignment | 36 | +3.1 ± 0.40 | +2.6 ± 1.40 | +1.6 ± 1.34 | −0.7 ± 1.89 | −1.1 ± 1.85 | −2.1 ± 1.02 | +3.8 ± 1.57 (H-47) |
| Seed 23, eighth, assignment | 6 | = without replay | = without replay | = without replay | −3.0 ± 0.96 (H-47) | −3.0 ± 0.96 (H-47) | −3.0 ± 0.96 (H-47) | +3.0 ± 0.96 (H-47) |
| Seed 23, eighth, assignment | 12 | −1.9 ± 0.88 | = without replay | = without replay | −4.5 ± 1.19 | −2.6 ± 0.94 (H-47) | −2.6 ± 0.94 (H-47) | +2.6 ± 0.94 (H-47) |
| Seed 23, eighth, assignment | 18 | −2.3 ± 0.76 | +2.8 ± 1.71 | = without replay | −10.2 ± 1.13 | −5.1 ± 1.42 | −7.9 ± 0.90 (H-47) | +7.9 ± 0.90 (H-47) |
| Seed 23, eighth, assignment | 24 | −5.5 ± 1.49 | −0.7 ± 1.26 | +1.5 ± 1.11 | −11.2 ± 1.72 | −6.4 ± 1.24 | −4.3 ± 1.06 | +5.7 ± 1.34 (H-47) |
| Seed 23, eighth, assignment | 36 | −5.6 ± 1.92 | −1.1 ± 1.78 | −1.5 ± 1.44 | −9.6 ± 1.06 | −5.2 ± 1.12 | −5.6 ± 1.20 | +4.1 ± 1.45 (H-47) |
| Seed 24, first mapping, mirrored | 6 | = without replay | = without replay | = without replay | −3.4 ± 0.79 (H-47) | −3.4 ± 0.79 (H-47) | −3.4 ± 0.79 (H-47) | +3.4 ± 0.79 (H-47) |
| Seed 24, first mapping, mirrored | 12 | +2.1 ± 1.15 | = without replay | = without replay | −2.6 ± 1.75 | −4.7 ± 1.50 (H-47) | −4.7 ± 1.50 (H-47) | +4.7 ± 1.50 (H-47) |
| Seed 24, first mapping, mirrored | 18 | +3.9 ± 0.62 | −1.0 ± 1.41 | = without replay | −3.9 ± 1.43 | −8.8 ± 1.72 | −7.8 ± 0.89 (H-47) | +7.8 ± 0.89 (H-47) |
| Seed 24, first mapping, mirrored | 24 | +4.4 ± 1.50 | +1.1 ± 0.90 | +2.0 ± 1.56 | −3.3 ± 1.14 | −6.6 ± 0.96 | −5.6 ± 1.08 | +7.6 ± 1.32 (H-47) |
| Seed 24, first mapping, mirrored | 36 | +4.6 ± 1.37 | +0.3 ± 0.70 | +1.3 ± 0.66 | −1.3 ± 0.73 | −5.7 ± 1.86 | −4.7 ± 1.15 | +6.0 ± 1.54 (H-47) |
| Seed 23, first mapping, mirrored | 6 | = without replay | = without replay | = without replay | −1.9 ± 0.68 (H-47) | −1.9 ± 0.68 (H-47) | −1.9 ± 0.68 (H-47) | +1.9 ± 0.68 (H-47) |
| Seed 23, first mapping, mirrored | 12 | +1.6 ± 0.55 | = without replay | = without replay | −0.4 ± 1.08 | −2.0 ± 1.26 (H-47) | −2.0 ± 1.26 (H-47) | +2.0 ± 1.26 (H-47) |
| Seed 23, first mapping, mirrored | 18 | +0.9 ± 0.76 | +0.6 ± 0.88 | = without replay | +0.6 ± 0.90 | +0.3 ± 1.01 | −0.3 ± 1.17 (H-47) | +0.3 ± 1.17 (H-47) |
| Seed 23, first mapping, mirrored | 24 | +0.5 ± 0.42 | +0.4 ± 0.45 | −0.3 ± 0.81 | +3.1 ± 1.35 | +2.9 ± 1.51 | +2.3 ± 1.81 | −2.6 ± 1.30 (H-47) |
| Seed 23, first mapping, mirrored | 36 | −0.5 ± 1.11 | +0.3 ± 0.83 | −2.4 ± 0.86 | +4.4 ± 0.59 | +5.2 ± 1.22 | +2.5 ± 1.31 | −4.9 ± 1.09 (H-47) |
| Seed 24, eighth, assignment | 6 | = without replay | = without replay | = without replay | −0.3 ± 0.64 (H-47) | −0.3 ± 0.64 (H-47) | −0.3 ± 0.64 (H-47) | +0.3 ± 0.64 (H-47) |
| Seed 24, eighth, assignment | 12 | −7.3 ± 0.88 | = without replay | = without replay | −6.7 ± 1.41 | +0.6 ± 1.03 (H-47) | +0.6 ± 1.03 (H-47) | −0.6 ± 1.03 (H-47) |
| Seed 24, eighth, assignment | 18 | −4.6 ± 1.83 | −12.1 ± 0.63 | = without replay | +1.4 ± 1.53 | −6.2 ± 1.48 | +5.9 ± 1.53 (H-47) | −5.9 ± 1.53 (H-47) |
| Seed 24, eighth, assignment | 24 | −6.5 ± 0.42 | −8.8 ± 0.82 | −12.3 ± 0.58 | −0.6 ± 2.07 | −2.9 ± 1.70 | −6.3 ± 2.02 | −5.9 ± 1.85 (H-47) |
| Seed 24, eighth, assignment | 36 | −2.1 ± 1.78 | −5.5 ± 0.95 | −4.6 ± 0.73 | −3.9 ± 0.91 | −7.3 ± 1.29 | −6.5 ± 1.63 | +1.8 ± 1.79 (H-47) |
| Seed 22, first mapping, mirrored | 6 | = without replay | = without replay | = without replay | −1.1 ± 0.63 (H-47) | −1.1 ± 0.63 (H-47) | −1.1 ± 0.63 (H-47) | +1.1 ± 0.63 (H-47) |
| Seed 22, first mapping, mirrored | 12 | +1.5 ± 0.62 | = without replay | = without replay | −1.5 ± 2.01 | −2.9 ± 1.93 (H-47) | −2.9 ± 1.93 (H-47) | +2.9 ± 1.93 (H-47) |
| Seed 22, first mapping, mirrored | 18 | +1.3 ± 0.49 | +1.8 ± 0.93 | = without replay | −2.4 ± 2.14 | −1.9 ± 1.93 | −3.7 ± 2.22 (H-47) | +3.7 ± 2.22 (H-47) |
| Seed 22, first mapping, mirrored | 24 | +1.3 ± 0.21 | +2.3 ± 0.60 | +3.0 ± 0.69 | −3.4 ± 2.33 | −2.4 ± 2.30 | −1.7 ± 2.13 | +4.7 ± 2.24 (H-47) |
| Seed 22, first mapping, mirrored | 36 | +0.5 ± 0.15 | +3.1 ± 0.49 | +1.7 ± 1.36 | −4.5 ± 1.52 | −1.8 ± 1.39 | −3.3 ± 0.92 | +5.0 ± 1.24 (H-47) |
| Seed 22, eighth, assignment | 6 | = without replay | = without replay | = without replay | −1.9 ± 0.65 (H-47) | −1.9 ± 0.65 (H-47) | −1.9 ± 0.65 (H-47) | +1.9 ± 0.65 (H-47) |
| Seed 22, eighth, assignment | 12 | +3.8 ± 0.77 | = without replay | = without replay | +0.8 ± 1.12 | −3.0 ± 0.93 (H-47) | −3.0 ± 0.93 (H-47) | +3.0 ± 0.93 (H-47) |
| Seed 22, eighth, assignment | 18 | +2.0 ± 0.74 | +4.2 ± 1.19 | = without replay | −1.4 ± 1.39 | +0.8 ± 1.15 | −3.4 ± 1.05 (H-47) | +3.4 ± 1.05 (H-47) |
| Seed 22, eighth, assignment | 24 | +0.7 ± 0.83 | +2.4 ± 0.48 | +1.0 ± 0.86 | −2.1 ± 1.12 | −0.4 ± 1.04 | −1.9 ± 0.91 | +2.8 ± 0.89 (H-47) |
| Seed 22, eighth, assignment | 36 | +1.1 ± 0.81 | +2.6 ± 0.38 | −1.1 ± 0.65 | −1.1 ± 1.44 | +0.4 ± 1.16 | −3.3 ± 1.26 | +2.2 ± 0.65 (H-47) |

- **Right after its replayed night**, each form less the night without replay is that night's replay alone. After block 12 alone at
  block 12 lies above beyond twice the paired SE in seven cells, +1.5 ± 0.62 on seed 22's first mapping from the mirrored assignment to
  +9.0 ± 0.47 on seed 22's eighth image from the mirrored assignment, within 2 SE in three, and below in two: seed 23's eighth image from
  the assignment, −1.9 ± 0.88, and seed 24's eighth image from the assignment, −7.3 ± 0.88. After block 18 alone at block 18 lies above in
  seven, +2.0 ± 0.87 on seed 23's eighth image from the mirrored assignment to +9.1 ± 0.92 on seed 24's eighth image from the mirrored
  assignment, within in four, and below on seed 24's eighth image from the assignment, −12.1 ± 0.63. After block 24 alone at block 24 lies
  above in six, +2.1 ± 0.89 on seed 23's first mapping from the assignment to +8.4 ± 1.47 on seed 24's first mapping from the assignment,
  within in five, and below on seed 24's eighth image from the assignment, −12.3 ± 0.58. H-47's first plain night, after block 6 alone at
  block 6, read above in eight and within in four (exploration 031).
- **At block 36, less the night without replay**, after block 12 alone lies above beyond 2 SE in eight cells, +0.5 ± 0.15 on seed 22's
  first mapping from the mirrored assignment to +8.1 ± 0.74 on seed 22's eighth image from the mirrored assignment; within 2 SE in three,
  seed 23's first mapping from the mirrored assignment, −0.5 ± 1.11, seed 24's eighth image from the assignment, −2.1 ± 1.78, and seed
  22's eighth image from the assignment, +1.1 ± 0.81; and below on seed 23's eighth image from the assignment, −5.6 ± 1.92.
- **After block 18 alone at block 36** lies above in seven cells, +1.7 ± 0.52 on seed 22's first mapping from the assignment to
  +6.3 ± 0.74 on seed 22's eighth image from the mirrored assignment; within in four, seed 24's first mapping from the assignment,
  +2.6 ± 1.40, seed 23's eighth image from the assignment, −1.1 ± 1.78, seed 24's first mapping from the mirrored assignment, +0.3 ± 0.70,
  and seed 23's first mapping from the mirrored assignment, +0.3 ± 0.83; and below on seed 24's eighth image from the assignment,
  −5.5 ± 0.95.
- **After block 24 alone at block 36** lies above in three cells, seed 22's eighth image from the mirrored assignment, +5.1 ± 1.31, seed
  23's eighth image from the mirrored assignment, +2.4 ± 0.35, and seed 23's first mapping from the assignment, +5.8 ± 0.54; below in two,
  seed 23's first mapping from the mirrored assignment, −2.4 ± 0.86, and seed 24's eighth image from the assignment, −4.6 ± 0.73; and within
  in seven, −1.5 ± 1.44 on seed 23's eighth image from the assignment to +1.9 ± 1.18 on seed 24's eighth image from the mirrored
  assignment. After block 6 alone lay above in five, within in six and below in one (exploration 031; section 7).
- **By the readout A answers, at block 36**: where A answers readout 0, after block 12 alone lies above the night without replay beyond 2
  SE in all six cells, after block 18 alone in five, within on seed 24's first mapping from the assignment, and after block 24 alone in
  three, within on seed 22's first mapping from the assignment, seed 24's eighth image from the mirrored assignment and seed 24's first
  mapping from the assignment; none of the three lies below there. Where A answers readout 1, after block 12 alone lies above in two,
  seed 24's first mapping from the mirrored assignment and seed 22's first mapping from the mirrored assignment, after block 18 alone in
  two, seed 22's first mapping from the mirrored assignment and seed 22's eighth image from the assignment, and after block 24 alone in
  none.
- **From each form's replayed night's block to block 36**, after block 12 alone lies above the night without replay beyond 2 SE at 32 of
  its 48 checkpoints and below at 7, after block 18 alone above at 21 of 36 and below at 3, and after block 24 alone above at 9 of 24 and
  below at 3. Every one of those thirteen below lies on seed 23's eighth image from the assignment or seed 24's eighth image from the
  assignment, but after block 24 alone at block 36 on seed 23's first mapping from the mirrored assignment.
- **Less H-47's plain night at block 36**, beyond twice exploration 018's unpaired SE, after block 12 alone lies below in eight cells,
  −3.9 ± 0.91 on seed 24's eighth image from the assignment to −13.6 ± 1.05 on seed 23's eighth image from the mirrored assignment; within
  in three, seed 24's first mapping from the assignment, −0.7 ± 1.89, seed 24's first mapping from the mirrored assignment, −1.3 ± 0.73,
  and seed 22's eighth image from the assignment, −1.1 ± 1.44; and above on seed 23's first mapping from the mirrored assignment,
  +4.4 ± 0.59. After block 18 alone lies below in eight, −5.2 ± 1.12 on seed 23's eighth image from the assignment to −14.4 ± 0.93 on seed
  23's eighth image from the mirrored assignment; within in three, seed 24's first mapping from the assignment, −1.1 ± 1.85, seed 22's
  first mapping from the mirrored assignment, −1.8 ± 1.39, and seed 22's eighth image from the assignment, +0.4 ± 1.16; and above on seed
  23's first mapping from the mirrored assignment, +5.2 ± 1.22. After block 24 alone lies below in eleven, −2.1 ± 1.02 on seed 24's first
  mapping from the assignment to −15.0 ± 1.16 on seed 23's eighth image from the mirrored assignment, and within on seed 23's first mapping
  from the mirrored assignment, +2.5 ± 1.31.

Each stimulus's correct trials under each form, of the trials a read presents of it, and the ties of both stimuli's trials, the mean of 48
reads; then each stimulus's trials less the night without replay, ± the paired SE, and less H-47's plain night, ± the unpaired SE, marked
as above. H-47's plain night's rows give its own trials and, in the columns less the night without replay, H-47's plain night less its
night without replay, H-47's own:

| Cell | Form | Block | Correct on A's trials | on B's trials | Ties | A's trials less without replay | B's trials less without replay | A's trials less plain | B's trials less plain |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | After block 12 alone | 6 | 24.8 | 21.5 | 4.8 | = without replay | = without replay | −4.2 ± 0.89 (H-47) | −2.9 ± 1.07 (H-47) |
| Seed 22, eighth, mirrored | After block 12 alone | 12 | 15.6 | 23.6 | 4.8 | +4.3 ± 0.60 | +4.7 ± 0.58 | −3.5 ± 1.25 | +0.4 ± 0.92 |
| Seed 22, eighth, mirrored | After block 12 alone | 18 | 11.0 | 22.1 | 4.6 | +1.6 ± 1.40 | +5.3 ± 0.47 | −7.0 ± 1.61 | −1.6 ± 1.48 |
| Seed 22, eighth, mirrored | After block 12 alone | 24 | 11.1 | 18.4 | 4.8 | +2.5 ± 1.27 | +4.4 ± 0.64 | −3.6 ± 0.84 | −7.1 ± 1.59 |
| Seed 22, eighth, mirrored | After block 12 alone | 36 | 7.2 | 17.5 | 5.1 | +2.5 ± 1.08 | +5.6 ± 0.96 | −6.8 ± 1.01 | −4.8 ± 1.41 |
| Seed 22, eighth, mirrored | After block 18 alone | 6 | 24.8 | 21.5 | 4.8 | = without replay | = without replay | −4.2 ± 0.89 (H-47) | −2.9 ± 1.07 (H-47) |
| Seed 22, eighth, mirrored | After block 18 alone | 12 | 11.3 | 18.9 | 5.4 | = without replay | = without replay | −7.8 ± 1.13 (H-47) | −4.3 ± 0.84 (H-47) |
| Seed 22, eighth, mirrored | After block 18 alone | 18 | 8.4 | 23.4 | 4.5 | −1.1 ± 1.10 | +6.6 ± 0.86 | −9.7 ± 2.74 | −0.3 ± 1.31 |
| Seed 22, eighth, mirrored | After block 18 alone | 24 | 6.9 | 20.3 | 4.4 | −1.8 ± 0.34 | +6.3 ± 0.80 | −7.8 ± 1.00 | −5.2 ± 1.37 |
| Seed 22, eighth, mirrored | After block 18 alone | 36 | 4.6 | 18.3 | 4.3 | −0.1 ± 0.39 | +6.4 ± 0.55 | −9.4 ± 0.89 | −4.0 ± 0.66 |
| Seed 22, eighth, mirrored | After block 24 alone | 6 | 24.8 | 21.5 | 4.8 | = without replay | = without replay | −4.2 ± 0.89 (H-47) | −2.9 ± 1.07 (H-47) |
| Seed 22, eighth, mirrored | After block 24 alone | 12 | 11.3 | 18.9 | 5.4 | = without replay | = without replay | −7.8 ± 1.13 (H-47) | −4.3 ± 0.84 (H-47) |
| Seed 22, eighth, mirrored | After block 24 alone | 18 | 9.5 | 16.8 | 4.9 | = without replay | = without replay | −8.6 ± 1.79 (H-47) | −6.9 ± 1.22 (H-47) |
| Seed 22, eighth, mirrored | After block 24 alone | 24 | 6.6 | 20.6 | 4.5 | −2.0 ± 0.39 | +6.6 ± 0.95 | −8.1 ± 0.91 | −4.9 ± 1.64 |
| Seed 22, eighth, mirrored | After block 24 alone | 36 | 5.4 | 16.2 | 4.2 | +0.8 ± 0.89 | +4.3 ± 1.03 | −8.5 ± 0.99 | −6.1 ± 1.29 |
| Seed 22, eighth, mirrored | Plain on A's (H-47) | 6 | 29.0 | 24.4 | 3.4 | +4.2 ± 0.89 (H-47) | +2.9 ± 1.07 (H-47) | – | – |
| Seed 22, eighth, mirrored | Plain on A's (H-47) | 12 | 19.1 | 23.2 | 5.0 | +7.8 ± 1.13 (H-47) | +4.3 ± 0.84 (H-47) | – | – |
| Seed 22, eighth, mirrored | Plain on A's (H-47) | 18 | 18.1 | 23.7 | 5.5 | +8.6 ± 1.79 (H-47) | +6.9 ± 1.22 (H-47) | – | – |
| Seed 22, eighth, mirrored | Plain on A's (H-47) | 24 | 14.7 | 25.5 | 5.3 | +6.1 ± 1.14 (H-47) | +11.6 ± 1.20 (H-47) | – | – |
| Seed 22, eighth, mirrored | Plain on A's (H-47) | 36 | 13.9 | 22.3 | 5.8 | +9.2 ± 0.86 (H-47) | +10.4 ± 0.54 (H-47) | – | – |
| Seed 22, first mapping, assignment | After block 12 alone | 6 | 25.5 | 26.9 | 3.3 | = without replay | = without replay | −4.1 ± 0.45 (H-47) | 0.0 ± 0.50 (H-47) |
| Seed 22, first mapping, assignment | After block 12 alone | 12 | 25.0 | 24.3 | 4.7 | +3.9 ± 0.88 | +0.7 ± 0.93 | +0.1 ± 0.82 | +1.6 ± 1.31 |
| Seed 22, first mapping, assignment | After block 12 alone | 18 | 22.0 | 23.5 | 4.8 | +5.6 ± 1.15 | −0.3 ± 0.92 | −2.7 ± 1.17 | +1.3 ± 1.34 |
| Seed 22, first mapping, assignment | After block 12 alone | 24 | 19.3 | 23.5 | 4.8 | +3.9 ± 1.46 | +0.1 ± 0.52 | −4.4 ± 0.72 | −0.8 ± 1.05 |
| Seed 22, first mapping, assignment | After block 12 alone | 36 | 17.0 | 22.6 | 5.8 | +3.8 ± 0.74 | −0.6 ± 0.22 | −4.5 ± 1.20 | −0.1 ± 0.90 |
| Seed 22, first mapping, assignment | After block 18 alone | 6 | 25.5 | 26.9 | 3.3 | = without replay | = without replay | −4.1 ± 0.45 (H-47) | 0.0 ± 0.50 (H-47) |
| Seed 22, first mapping, assignment | After block 18 alone | 12 | 21.1 | 23.6 | 4.4 | = without replay | = without replay | −3.8 ± 0.48 (H-47) | +0.9 ± 0.40 (H-47) |
| Seed 22, first mapping, assignment | After block 18 alone | 18 | 21.0 | 25.6 | 5.6 | +4.6 ± 1.32 | +1.9 ± 0.73 | −3.8 ± 1.23 | +3.5 ± 0.96 |
| Seed 22, first mapping, assignment | After block 18 alone | 24 | 15.9 | 24.3 | 4.7 | +0.5 ± 1.03 | +0.9 ± 0.50 | −7.8 ± 0.77 | +0.1 ± 0.96 |
| Seed 22, first mapping, assignment | After block 18 alone | 36 | 15.0 | 23.0 | 5.4 | +1.8 ± 0.38 | −0.1 ± 0.23 | −6.5 ± 1.19 | +0.3 ± 0.82 |
| Seed 22, first mapping, assignment | After block 24 alone | 6 | 25.5 | 26.9 | 3.3 | = without replay | = without replay | −4.1 ± 0.45 (H-47) | 0.0 ± 0.50 (H-47) |
| Seed 22, first mapping, assignment | After block 24 alone | 12 | 21.1 | 23.6 | 4.4 | = without replay | = without replay | −3.8 ± 0.48 (H-47) | +0.9 ± 0.40 (H-47) |
| Seed 22, first mapping, assignment | After block 24 alone | 18 | 16.4 | 23.7 | 5.8 | = without replay | = without replay | −8.3 ± 0.81 (H-47) | +1.6 ± 0.69 (H-47) |
| Seed 22, first mapping, assignment | After block 24 alone | 24 | 16.4 | 26.1 | 5.8 | +1.0 ± 1.29 | +2.6 ± 0.41 | −7.3 ± 1.49 | +1.8 ± 1.05 |
| Seed 22, first mapping, assignment | After block 24 alone | 36 | 14.0 | 23.1 | 5.8 | +0.8 ± 1.04 | 0.0 ± 1.26 | −7.5 ± 1.20 | +0.4 ± 1.40 |
| Seed 22, first mapping, assignment | Plain on A's (H-47) | 6 | 29.6 | 26.9 | 2.8 | +4.1 ± 0.45 (H-47) | 0.0 ± 0.50 (H-47) | – | – |
| Seed 22, first mapping, assignment | Plain on A's (H-47) | 12 | 24.9 | 22.7 | 5.4 | +3.8 ± 0.48 (H-47) | −0.9 ± 0.40 (H-47) | – | – |
| Seed 22, first mapping, assignment | Plain on A's (H-47) | 18 | 24.8 | 22.2 | 5.8 | +8.3 ± 0.81 (H-47) | −1.6 ± 0.69 (H-47) | – | – |
| Seed 22, first mapping, assignment | Plain on A's (H-47) | 24 | 23.8 | 24.3 | 5.2 | +8.3 ± 1.55 (H-47) | +0.8 ± 0.85 (H-47) | – | – |
| Seed 22, first mapping, assignment | Plain on A's (H-47) | 36 | 21.5 | 22.7 | 5.4 | +8.4 ± 1.10 (H-47) | −0.4 ± 0.83 (H-47) | – | – |
| Seed 23, eighth, mirrored | After block 12 alone | 6 | 20.0 | 14.4 | 5.4 | = without replay | = without replay | −2.5 ± 0.98 (H-47) | +0.8 ± 0.90 (H-47) |
| Seed 23, eighth, mirrored | After block 12 alone | 12 | 17.0 | 3.5 | 4.8 | +1.7 ± 0.48 | −0.7 ± 0.55 | −4.4 ± 1.24 | −3.5 ± 0.66 |
| Seed 23, eighth, mirrored | After block 12 alone | 18 | 16.2 | 3.9 | 4.5 | +4.0 ± 0.68 | +1.3 ± 0.61 | −8.1 ± 1.25 | +0.7 ± 0.65 |
| Seed 23, eighth, mirrored | After block 12 alone | 24 | 11.9 | 2.5 | 3.7 | +3.0 ± 0.57 | +0.8 ± 0.32 | −13.9 ± 1.19 | −0.9 ± 0.39 |
| Seed 23, eighth, mirrored | After block 12 alone | 36 | 11.0 | 1.0 | 2.8 | +3.6 ± 0.55 | +0.3 ± 0.28 | −12.3 ± 1.00 | −1.3 ± 0.47 |
| Seed 23, eighth, mirrored | After block 18 alone | 6 | 20.0 | 14.4 | 5.4 | = without replay | = without replay | −2.5 ± 0.98 (H-47) | +0.8 ± 0.90 (H-47) |
| Seed 23, eighth, mirrored | After block 18 alone | 12 | 15.3 | 4.2 | 4.5 | = without replay | = without replay | −6.1 ± 0.90 (H-47) | −2.8 ± 0.44 (H-47) |
| Seed 23, eighth, mirrored | After block 18 alone | 18 | 14.6 | 2.2 | 3.6 | +2.4 ± 0.92 | −0.4 ± 0.34 | −9.7 ± 1.30 | −1.0 ± 0.45 |
| Seed 23, eighth, mirrored | After block 18 alone | 24 | 12.0 | 1.5 | 3.5 | +3.1 ± 0.43 | −0.2 ± 0.29 | −13.8 ± 1.27 | −2.0 ± 0.32 |
| Seed 23, eighth, mirrored | After block 18 alone | 36 | 10.3 | 0.9 | 3.1 | +2.9 ± 0.47 | +0.1 ± 0.34 | −13.0 ± 1.09 | −1.4 ± 0.53 |
| Seed 23, eighth, mirrored | After block 24 alone | 6 | 20.0 | 14.4 | 5.4 | = without replay | = without replay | −2.5 ± 0.98 (H-47) | +0.8 ± 0.90 (H-47) |
| Seed 23, eighth, mirrored | After block 24 alone | 12 | 15.3 | 4.2 | 4.5 | = without replay | = without replay | −6.1 ± 0.90 (H-47) | −2.8 ± 0.44 (H-47) |
| Seed 23, eighth, mirrored | After block 24 alone | 18 | 12.3 | 2.6 | 3.6 | = without replay | = without replay | −12.1 ± 1.05 (H-47) | −0.6 ± 0.57 (H-47) |
| Seed 23, eighth, mirrored | After block 24 alone | 24 | 10.0 | 1.5 | 3.6 | +1.1 ± 1.24 | −0.2 ± 0.39 | −15.8 ± 1.91 | −2.0 ± 0.53 |
| Seed 23, eighth, mirrored | After block 24 alone | 36 | 9.6 | 1.0 | 3.3 | +2.3 ± 0.28 | +0.2 ± 0.26 | −13.6 ± 1.25 | −1.3 ± 0.49 |
| Seed 23, eighth, mirrored | Plain on A's (H-47) | 6 | 22.5 | 13.6 | 5.3 | +2.5 ± 0.98 (H-47) | −0.8 ± 0.90 (H-47) | – | – |
| Seed 23, eighth, mirrored | Plain on A's (H-47) | 12 | 21.5 | 7.0 | 5.4 | +6.1 ± 0.90 (H-47) | +2.8 ± 0.44 (H-47) | – | – |
| Seed 23, eighth, mirrored | Plain on A's (H-47) | 18 | 24.4 | 3.2 | 3.9 | +12.1 ± 1.05 (H-47) | +0.6 ± 0.57 (H-47) | – | – |
| Seed 23, eighth, mirrored | Plain on A's (H-47) | 24 | 25.8 | 3.5 | 3.6 | +16.9 ± 1.43 (H-47) | +1.8 ± 0.26 (H-47) | – | – |
| Seed 23, eighth, mirrored | Plain on A's (H-47) | 36 | 23.3 | 2.3 | 3.5 | +15.9 ± 1.05 (H-47) | +1.5 ± 0.28 (H-47) | – | – |
| Seed 23, first mapping, assignment | After block 12 alone | 6 | 29.2 | 25.0 | 3.1 | = without replay | = without replay | −1.6 ± 0.46 (H-47) | −0.4 ± 0.57 (H-47) |
| Seed 23, first mapping, assignment | After block 12 alone | 12 | 29.3 | 8.6 | 4.2 | +5.1 ± 0.79 | −0.6 ± 0.85 | −0.9 ± 0.86 | +1.2 ± 1.22 |
| Seed 23, first mapping, assignment | After block 12 alone | 18 | 25.7 | 7.4 | 4.3 | +6.2 ± 1.58 | −0.3 ± 1.03 | −4.4 ± 1.34 | +2.5 ± 0.86 |
| Seed 23, first mapping, assignment | After block 12 alone | 24 | 21.5 | 9.4 | 4.9 | +4.3 ± 0.67 | +1.8 ± 1.17 | −8.9 ± 1.38 | +5.0 ± 0.93 |
| Seed 23, first mapping, assignment | After block 12 alone | 36 | 20.8 | 5.5 | 5.1 | +5.0 ± 1.27 | +1.5 ± 0.82 | −8.4 ± 1.64 | +1.7 ± 0.77 |
| Seed 23, first mapping, assignment | After block 18 alone | 6 | 29.2 | 25.0 | 3.1 | = without replay | = without replay | −1.6 ± 0.46 (H-47) | −0.4 ± 0.57 (H-47) |
| Seed 23, first mapping, assignment | After block 18 alone | 12 | 24.2 | 9.3 | 4.8 | = without replay | = without replay | −6.0 ± 0.87 (H-47) | +1.8 ± 0.85 (H-47) |
| Seed 23, first mapping, assignment | After block 18 alone | 18 | 25.1 | 6.4 | 4.4 | +5.6 ± 0.93 | −1.2 ± 0.72 | −5.0 ± 1.62 | +1.5 ± 0.97 |
| Seed 23, first mapping, assignment | After block 18 alone | 24 | 22.5 | 7.9 | 4.8 | +5.2 ± 1.18 | +0.3 ± 0.56 | −8.0 ± 1.07 | +3.5 ± 0.91 |
| Seed 23, first mapping, assignment | After block 18 alone | 36 | 20.7 | 4.6 | 5.2 | +5.0 ± 1.15 | +0.6 ± 0.36 | −8.5 ± 1.45 | +0.8 ± 0.84 |
| Seed 23, first mapping, assignment | After block 24 alone | 6 | 29.2 | 25.0 | 3.1 | = without replay | = without replay | −1.6 ± 0.46 (H-47) | −0.4 ± 0.57 (H-47) |
| Seed 23, first mapping, assignment | After block 24 alone | 12 | 24.2 | 9.3 | 4.8 | = without replay | = without replay | −6.0 ± 0.87 (H-47) | +1.8 ± 0.85 (H-47) |
| Seed 23, first mapping, assignment | After block 24 alone | 18 | 19.5 | 7.6 | 4.8 | = without replay | = without replay | −10.6 ± 1.65 (H-47) | +2.8 ± 1.24 (H-47) |
| Seed 23, first mapping, assignment | After block 24 alone | 24 | 21.2 | 5.6 | 5.2 | +4.0 ± 0.73 | −1.9 ± 0.40 | −9.2 ± 1.44 | +1.3 ± 0.70 |
| Seed 23, first mapping, assignment | After block 24 alone | 36 | 20.9 | 4.6 | 4.6 | +5.2 ± 0.21 | +0.6 ± 0.40 | −8.3 ± 1.34 | +0.8 ± 0.68 |
| Seed 23, first mapping, assignment | Plain on A's (H-47) | 6 | 30.9 | 25.4 | 2.8 | +1.6 ± 0.46 (H-47) | +0.4 ± 0.57 (H-47) | – | – |
| Seed 23, first mapping, assignment | Plain on A's (H-47) | 12 | 30.1 | 7.4 | 4.1 | +6.0 ± 0.87 (H-47) | −1.8 ± 0.85 (H-47) | – | – |
| Seed 23, first mapping, assignment | Plain on A's (H-47) | 18 | 30.1 | 4.9 | 2.9 | +10.6 ± 1.65 (H-47) | −2.8 ± 1.24 (H-47) | – | – |
| Seed 23, first mapping, assignment | Plain on A's (H-47) | 24 | 30.4 | 4.3 | 2.8 | +13.2 ± 1.45 (H-47) | −3.2 ± 0.67 (H-47) | – | – |
| Seed 23, first mapping, assignment | Plain on A's (H-47) | 36 | 29.2 | 3.9 | 3.0 | +13.5 ± 1.09 (H-47) | −0.1 ± 0.62 (H-47) | – | – |
| Seed 24, eighth, mirrored | After block 12 alone | 6 | 19.1 | 17.3 | 5.1 | = without replay | = without replay | +2.1 ± 0.99 (H-47) | +0.5 ± 1.17 (H-47) |
| Seed 24, eighth, mirrored | After block 12 alone | 12 | 15.8 | 16.5 | 6.3 | +0.5 ± 0.83 | +0.2 ± 0.60 | +3.7 ± 0.96 | −0.1 ± 0.91 |
| Seed 24, eighth, mirrored | After block 12 alone | 18 | 12.9 | 13.4 | 5.4 | +3.7 ± 0.49 | +2.4 ± 0.73 | +3.0 ± 0.75 | −5.2 ± 1.52 |
| Seed 24, eighth, mirrored | After block 12 alone | 24 | 9.7 | 11.6 | 5.0 | +2.3 ± 0.97 | +2.6 ± 0.58 | +0.7 ± 0.91 | −5.1 ± 1.18 |
| Seed 24, eighth, mirrored | After block 12 alone | 36 | 6.0 | 10.3 | 4.6 | +0.7 ± 0.29 | +3.1 ± 1.02 | −0.8 ± 0.86 | −6.7 ± 0.70 |
| Seed 24, eighth, mirrored | After block 18 alone | 6 | 19.1 | 17.3 | 5.1 | = without replay | = without replay | +2.1 ± 0.99 (H-47) | +0.5 ± 1.17 (H-47) |
| Seed 24, eighth, mirrored | After block 18 alone | 12 | 15.3 | 16.3 | 5.2 | = without replay | = without replay | +3.2 ± 0.72 (H-47) | −0.3 ± 0.77 (H-47) |
| Seed 24, eighth, mirrored | After block 18 alone | 18 | 15.2 | 14.1 | 5.8 | +6.0 ± 0.68 | +3.1 ± 1.02 | +5.3 ± 0.98 | −4.5 ± 1.16 |
| Seed 24, eighth, mirrored | After block 18 alone | 24 | 8.5 | 12.3 | 5.3 | +1.1 ± 0.59 | +3.3 ± 0.82 | −0.5 ± 1.26 | −4.5 ± 1.15 |
| Seed 24, eighth, mirrored | After block 18 alone | 36 | 7.1 | 10.9 | 5.9 | +1.7 ± 0.54 | +3.7 ± 0.86 | +0.2 ± 1.42 | −6.0 ± 0.83 |
| Seed 24, eighth, mirrored | After block 24 alone | 6 | 19.1 | 17.3 | 5.1 | = without replay | = without replay | +2.1 ± 0.99 (H-47) | +0.5 ± 1.17 (H-47) |
| Seed 24, eighth, mirrored | After block 24 alone | 12 | 15.3 | 16.3 | 5.2 | = without replay | = without replay | +3.2 ± 0.72 (H-47) | −0.3 ± 0.77 (H-47) |
| Seed 24, eighth, mirrored | After block 24 alone | 18 | 9.2 | 11.0 | 4.7 | = without replay | = without replay | −0.7 ± 0.25 (H-47) | −7.6 ± 1.52 (H-47) |
| Seed 24, eighth, mirrored | After block 24 alone | 24 | 9.3 | 12.5 | 5.2 | +1.9 ± 0.99 | +3.5 ± 1.23 | +0.3 ± 1.54 | −4.3 ± 1.30 |
| Seed 24, eighth, mirrored | After block 24 alone | 36 | 4.5 | 9.9 | 4.5 | −0.8 ± 0.34 | +2.7 ± 0.89 | −2.4 ± 1.13 | −7.0 ± 0.74 |
| Seed 24, eighth, mirrored | Plain on A's (H-47) | 6 | 17.0 | 16.8 | 5.8 | −2.1 ± 0.99 (H-47) | −0.5 ± 1.17 (H-47) | – | – |
| Seed 24, eighth, mirrored | Plain on A's (H-47) | 12 | 12.1 | 16.6 | 5.7 | −3.2 ± 0.72 (H-47) | +0.3 ± 0.77 (H-47) | – | – |
| Seed 24, eighth, mirrored | Plain on A's (H-47) | 18 | 9.9 | 18.6 | 5.9 | +0.7 ± 0.25 (H-47) | +7.6 ± 1.52 (H-47) | – | – |
| Seed 24, eighth, mirrored | Plain on A's (H-47) | 24 | 9.0 | 16.8 | 6.0 | +1.6 ± 1.33 (H-47) | +7.8 ± 1.47 (H-47) | – | – |
| Seed 24, eighth, mirrored | Plain on A's (H-47) | 36 | 6.9 | 16.9 | 5.3 | +1.5 ± 0.74 (H-47) | +9.7 ± 0.98 (H-47) | – | – |
| Seed 24, first mapping, assignment | After block 12 alone | 6 | 28.8 | 29.5 | 2.3 | = without replay | = without replay | −1.9 ± 0.57 (H-47) | −1.1 ± 0.38 (H-47) |
| Seed 24, first mapping, assignment | After block 12 alone | 12 | 28.9 | 25.8 | 3.0 | +2.5 ± 0.59 | +0.1 ± 0.45 | +0.3 ± 0.98 | −0.5 ± 1.24 |
| Seed 24, first mapping, assignment | After block 12 alone | 18 | 25.3 | 23.6 | 4.5 | +0.4 ± 0.43 | +1.0 ± 0.55 | −3.6 ± 0.93 | +1.1 ± 1.55 |
| Seed 24, first mapping, assignment | After block 12 alone | 24 | 23.3 | 24.6 | 4.5 | +1.1 ± 0.91 | +3.1 ± 1.23 | −5.0 ± 1.48 | +0.5 ± 1.36 |
| Seed 24, first mapping, assignment | After block 12 alone | 36 | 22.0 | 21.1 | 5.1 | +0.8 ± 1.08 | +2.3 ± 0.75 | −2.5 ± 1.60 | +1.8 ± 1.52 |
| Seed 24, first mapping, assignment | After block 18 alone | 6 | 28.8 | 29.5 | 2.3 | = without replay | = without replay | −1.9 ± 0.57 (H-47) | −1.1 ± 0.38 (H-47) |
| Seed 24, first mapping, assignment | After block 18 alone | 12 | 26.4 | 25.6 | 3.5 | = without replay | = without replay | −2.2 ± 0.43 (H-47) | −0.6 ± 0.99 (H-47) |
| Seed 24, first mapping, assignment | After block 18 alone | 18 | 27.6 | 23.9 | 4.3 | +2.8 ± 0.72 | +1.3 ± 0.95 | −1.2 ± 1.00 | +1.4 ± 1.82 |
| Seed 24, first mapping, assignment | After block 18 alone | 24 | 23.9 | 24.2 | 4.3 | +1.6 ± 0.60 | +2.8 ± 0.60 | −4.5 ± 1.53 | +0.2 ± 1.98 |
| Seed 24, first mapping, assignment | After block 18 alone | 36 | 21.1 | 21.6 | 5.3 | −0.1 ± 0.68 | +2.8 ± 0.92 | −3.4 ± 1.26 | +2.3 ± 1.26 |
| Seed 24, first mapping, assignment | After block 24 alone | 6 | 28.8 | 29.5 | 2.3 | = without replay | = without replay | −1.9 ± 0.57 (H-47) | −1.1 ± 0.38 (H-47) |
| Seed 24, first mapping, assignment | After block 24 alone | 12 | 26.4 | 25.6 | 3.5 | = without replay | = without replay | −2.2 ± 0.43 (H-47) | −0.6 ± 0.99 (H-47) |
| Seed 24, first mapping, assignment | After block 24 alone | 18 | 24.9 | 22.6 | 3.8 | = without replay | = without replay | −4.0 ± 0.73 (H-47) | +0.1 ± 1.55 (H-47) |
| Seed 24, first mapping, assignment | After block 24 alone | 24 | 27.2 | 24.9 | 4.1 | +4.9 ± 0.93 | +3.4 ± 0.95 | −1.1 ± 1.29 | +0.8 ± 1.98 |
| Seed 24, first mapping, assignment | After block 24 alone | 36 | 22.3 | 19.4 | 5.6 | +1.0 ± 0.57 | +0.6 ± 1.66 | −2.2 ± 0.87 | +0.1 ± 0.96 |
| Seed 24, first mapping, assignment | Plain on A's (H-47) | 6 | 30.7 | 30.6 | 1.4 | +1.9 ± 0.57 (H-47) | +1.1 ± 0.38 (H-47) | – | – |
| Seed 24, first mapping, assignment | Plain on A's (H-47) | 12 | 28.6 | 26.2 | 3.1 | +2.2 ± 0.43 (H-47) | +0.6 ± 0.99 (H-47) | – | – |
| Seed 24, first mapping, assignment | Plain on A's (H-47) | 18 | 28.9 | 22.5 | 3.9 | +4.0 ± 0.73 (H-47) | −0.1 ± 1.55 (H-47) | – | – |
| Seed 24, first mapping, assignment | Plain on A's (H-47) | 24 | 28.4 | 24.0 | 3.6 | +6.1 ± 1.25 (H-47) | +2.6 ± 2.15 (H-47) | – | – |
| Seed 24, first mapping, assignment | Plain on A's (H-47) | 36 | 24.5 | 19.3 | 5.3 | +3.3 ± 0.54 (H-47) | +0.5 ± 1.44 (H-47) | – | – |
| Seed 23, eighth, assignment | After block 12 alone | 6 | 24.3 | 23.1 | 4.2 | = without replay | = without replay | −2.6 ± 0.75 (H-47) | −0.4 ± 0.66 (H-47) |
| Seed 23, eighth, assignment | After block 12 alone | 12 | 18.8 | 20.1 | 4.8 | +0.6 ± 0.64 | −2.5 ± 0.66 | −2.4 ± 1.38 | −2.2 ± 1.13 |
| Seed 23, eighth, assignment | After block 12 alone | 18 | 16.7 | 18.3 | 5.7 | +1.1 ± 0.64 | −3.5 ± 0.52 | −4.1 ± 0.99 | −6.1 ± 1.11 |
| Seed 23, eighth, assignment | After block 12 alone | 24 | 15.7 | 16.8 | 5.0 | −0.6 ± 0.85 | −4.9 ± 0.88 | −3.9 ± 1.29 | −7.3 ± 1.61 |
| Seed 23, eighth, assignment | After block 12 alone | 36 | 12.1 | 18.5 | 4.9 | −0.9 ± 0.76 | −4.7 ± 1.43 | −3.2 ± 1.18 | −6.5 ± 1.50 |
| Seed 23, eighth, assignment | After block 18 alone | 6 | 24.3 | 23.1 | 4.2 | = without replay | = without replay | −2.6 ± 0.75 (H-47) | −0.4 ± 0.66 (H-47) |
| Seed 23, eighth, assignment | After block 18 alone | 12 | 18.3 | 22.6 | 4.4 | = without replay | = without replay | −2.9 ± 0.87 (H-47) | +0.4 ± 0.64 (H-47) |
| Seed 23, eighth, assignment | After block 18 alone | 18 | 16.3 | 23.9 | 5.1 | +0.7 ± 1.21 | +2.1 ± 0.55 | −4.5 ± 1.14 | −0.5 ± 1.08 |
| Seed 23, eighth, assignment | After block 18 alone | 24 | 16.3 | 21.1 | 4.4 | −0.1 ± 0.68 | −0.6 ± 0.72 | −3.3 ± 1.10 | −3.1 ± 1.45 |
| Seed 23, eighth, assignment | After block 18 alone | 36 | 12.5 | 22.5 | 4.8 | −0.5 ± 0.93 | −0.7 ± 0.93 | −2.8 ± 0.88 | −2.5 ± 1.11 |
| Seed 23, eighth, assignment | After block 24 alone | 6 | 24.3 | 23.1 | 4.2 | = without replay | = without replay | −2.6 ± 0.75 (H-47) | −0.4 ± 0.66 (H-47) |
| Seed 23, eighth, assignment | After block 24 alone | 12 | 18.3 | 22.6 | 4.4 | = without replay | = without replay | −2.9 ± 0.87 (H-47) | +0.4 ± 0.64 (H-47) |
| Seed 23, eighth, assignment | After block 24 alone | 18 | 15.6 | 21.7 | 4.7 | = without replay | = without replay | −5.2 ± 0.90 (H-47) | −2.7 ± 0.71 (H-47) |
| Seed 23, eighth, assignment | After block 24 alone | 24 | 16.0 | 23.4 | 5.5 | −0.3 ± 0.58 | +1.7 ± 0.86 | −3.5 ± 1.39 | −0.7 ± 1.31 |
| Seed 23, eighth, assignment | After block 24 alone | 36 | 11.1 | 23.5 | 4.8 | −1.9 ± 0.06 | +0.3 ± 1.46 | −4.1 ± 1.12 | −1.5 ± 1.03 |
| Seed 23, eighth, assignment | Plain on A's (H-47) | 6 | 26.9 | 23.4 | 3.9 | +2.6 ± 0.75 (H-47) | +0.4 ± 0.66 (H-47) | – | – |
| Seed 23, eighth, assignment | Plain on A's (H-47) | 12 | 21.2 | 22.2 | 4.6 | +2.9 ± 0.87 (H-47) | −0.4 ± 0.64 (H-47) | – | – |
| Seed 23, eighth, assignment | Plain on A's (H-47) | 18 | 20.8 | 24.4 | 4.2 | +5.2 ± 0.90 (H-47) | +2.7 ± 0.71 (H-47) | – | – |
| Seed 23, eighth, assignment | Plain on A's (H-47) | 24 | 19.6 | 24.1 | 4.7 | +3.3 ± 1.29 (H-47) | +2.5 ± 0.79 (H-47) | – | – |
| Seed 23, eighth, assignment | Plain on A's (H-47) | 36 | 15.3 | 25.0 | 4.2 | +2.3 ± 0.88 (H-47) | +1.8 ± 0.84 (H-47) | – | – |
| Seed 24, first mapping, mirrored | After block 12 alone | 6 | 28.1 | 26.2 | 3.0 | = without replay | = without replay | −2.0 ± 0.50 (H-47) | −1.4 ± 0.59 (H-47) |
| Seed 24, first mapping, mirrored | After block 12 alone | 12 | 26.5 | 21.9 | 5.1 | +1.9 ± 0.68 | +0.2 ± 0.82 | −0.8 ± 1.03 | −1.9 ± 1.50 |
| Seed 24, first mapping, mirrored | After block 12 alone | 18 | 25.1 | 22.9 | 4.3 | +3.4 ± 0.61 | +0.6 ± 0.68 | −2.0 ± 1.24 | −1.9 ± 1.20 |
| Seed 24, first mapping, mirrored | After block 12 alone | 24 | 25.7 | 23.0 | 4.5 | +3.6 ± 0.58 | +0.8 ± 0.95 | −0.1 ± 1.04 | −3.2 ± 1.20 |
| Seed 24, first mapping, mirrored | After block 12 alone | 36 | 24.9 | 22.4 | 4.6 | +3.0 ± 0.74 | +1.7 ± 0.76 | −0.3 ± 0.53 | −1.1 ± 0.83 |
| Seed 24, first mapping, mirrored | After block 18 alone | 6 | 28.1 | 26.2 | 3.0 | = without replay | = without replay | −2.0 ± 0.50 (H-47) | −1.4 ± 0.59 (H-47) |
| Seed 24, first mapping, mirrored | After block 18 alone | 12 | 24.6 | 21.7 | 4.8 | = without replay | = without replay | −2.6 ± 0.61 (H-47) | −2.1 ± 1.01 (H-47) |
| Seed 24, first mapping, mirrored | After block 18 alone | 18 | 22.1 | 21.0 | 5.3 | +0.4 ± 0.94 | −1.3 ± 0.72 | −5.0 ± 1.65 | −3.8 ± 1.09 |
| Seed 24, first mapping, mirrored | After block 18 alone | 24 | 21.6 | 23.9 | 4.4 | −0.5 ± 0.59 | +1.6 ± 0.67 | −4.2 ± 1.10 | −2.4 ± 1.24 |
| Seed 24, first mapping, mirrored | After block 18 alone | 36 | 20.8 | 22.3 | 5.4 | −1.2 ± 0.40 | +1.5 ± 0.64 | −4.4 ± 1.11 | −1.3 ± 1.16 |
| Seed 24, first mapping, mirrored | After block 24 alone | 6 | 28.1 | 26.2 | 3.0 | = without replay | = without replay | −2.0 ± 0.50 (H-47) | −1.4 ± 0.59 (H-47) |
| Seed 24, first mapping, mirrored | After block 24 alone | 12 | 24.6 | 21.7 | 4.8 | = without replay | = without replay | −2.6 ± 0.61 (H-47) | −2.1 ± 1.01 (H-47) |
| Seed 24, first mapping, mirrored | After block 24 alone | 18 | 21.7 | 22.3 | 5.2 | = without replay | = without replay | −5.3 ± 0.46 (H-47) | −2.5 ± 0.95 (H-47) |
| Seed 24, first mapping, mirrored | After block 24 alone | 24 | 24.4 | 22.0 | 4.9 | +2.3 ± 1.18 | −0.3 ± 0.68 | −1.4 ± 1.06 | −4.3 ± 0.96 |
| Seed 24, first mapping, mirrored | After block 24 alone | 36 | 22.2 | 21.8 | 5.2 | +0.3 ± 0.70 | +1.0 ± 0.53 | −3.0 ± 0.88 | −1.7 ± 1.08 |
| Seed 24, first mapping, mirrored | Plain on A's (H-47) | 6 | 30.1 | 27.6 | 2.6 | +2.0 ± 0.50 (H-47) | +1.4 ± 0.59 (H-47) | – | – |
| Seed 24, first mapping, mirrored | Plain on A's (H-47) | 12 | 27.3 | 23.8 | 3.9 | +2.6 ± 0.61 (H-47) | +2.1 ± 1.01 (H-47) | – | – |
| Seed 24, first mapping, mirrored | Plain on A's (H-47) | 18 | 27.0 | 24.8 | 4.0 | +5.3 ± 0.46 (H-47) | +2.5 ± 0.95 (H-47) | – | – |
| Seed 24, first mapping, mirrored | Plain on A's (H-47) | 24 | 25.8 | 26.2 | 3.6 | +3.7 ± 0.86 (H-47) | +4.0 ± 0.60 (H-47) | – | – |
| Seed 24, first mapping, mirrored | Plain on A's (H-47) | 36 | 25.2 | 23.5 | 4.4 | +3.3 ± 0.67 (H-47) | +2.7 ± 0.99 (H-47) | – | – |
| Seed 23, first mapping, mirrored | After block 12 alone | 6 | 30.4 | 24.6 | 3.0 | = without replay | = without replay | −0.6 ± 0.31 (H-47) | −1.3 ± 0.56 (H-47) |
| Seed 23, first mapping, mirrored | After block 12 alone | 12 | 29.8 | 24.2 | 3.8 | +1.3 ± 0.44 | +0.3 ± 0.43 | +0.3 ± 1.02 | −0.6 ± 1.24 |
| Seed 23, first mapping, mirrored | After block 12 alone | 18 | 28.1 | 24.8 | 3.2 | −0.5 ± 0.45 | +1.4 ± 0.60 | +0.5 ± 1.22 | +0.1 ± 0.96 |
| Seed 23, first mapping, mirrored | After block 12 alone | 24 | 29.3 | 24.7 | 2.5 | −0.8 ± 0.53 | +1.3 ± 0.38 | +4.2 ± 1.06 | −1.2 ± 1.15 |
| Seed 23, first mapping, mirrored | After block 12 alone | 36 | 27.5 | 25.2 | 3.9 | −1.8 ± 1.04 | +1.3 ± 0.58 | +3.4 ± 0.96 | +1.0 ± 0.94 |
| Seed 23, first mapping, mirrored | After block 18 alone | 6 | 30.4 | 24.6 | 3.0 | = without replay | = without replay | −0.6 ± 0.31 (H-47) | −1.3 ± 0.56 (H-47) |
| Seed 23, first mapping, mirrored | After block 18 alone | 12 | 28.5 | 23.8 | 4.1 | = without replay | = without replay | −1.1 ± 0.70 (H-47) | −1.0 ± 1.17 (H-47) |
| Seed 23, first mapping, mirrored | After block 18 alone | 18 | 28.5 | 24.2 | 3.9 | −0.1 ± 0.42 | +0.7 ± 0.78 | +0.9 ± 1.26 | −0.6 ± 1.12 |
| Seed 23, first mapping, mirrored | After block 18 alone | 24 | 29.1 | 24.8 | 3.0 | −1.0 ± 0.34 | +1.3 ± 0.28 | +4.0 ± 1.34 | −1.1 ± 1.22 |
| Seed 23, first mapping, mirrored | After block 18 alone | 36 | 28.0 | 25.4 | 3.8 | −1.3 ± 0.67 | +1.5 ± 0.76 | +3.9 ± 1.35 | +1.2 ± 0.92 |
| Seed 23, first mapping, mirrored | After block 24 alone | 6 | 30.4 | 24.6 | 3.0 | = without replay | = without replay | −0.6 ± 0.31 (H-47) | −1.3 ± 0.56 (H-47) |
| Seed 23, first mapping, mirrored | After block 24 alone | 12 | 28.5 | 23.8 | 4.1 | = without replay | = without replay | −1.1 ± 0.70 (H-47) | −1.0 ± 1.17 (H-47) |
| Seed 23, first mapping, mirrored | After block 24 alone | 18 | 28.5 | 23.5 | 3.4 | = without replay | = without replay | +1.0 ± 0.92 (H-47) | −1.3 ± 0.69 (H-47) |
| Seed 23, first mapping, mirrored | After block 24 alone | 24 | 29.1 | 24.0 | 3.5 | −0.9 ± 0.39 | +0.6 ± 0.73 | +4.1 ± 1.55 | −1.9 ± 1.18 |
| Seed 23, first mapping, mirrored | After block 24 alone | 36 | 28.2 | 22.6 | 3.8 | −1.1 ± 0.24 | −1.3 ± 0.96 | +4.1 ± 1.69 | −1.6 ± 1.16 |
| Seed 23, first mapping, mirrored | Plain on A's (H-47) | 6 | 31.0 | 25.9 | 2.4 | +0.6 ± 0.31 (H-47) | +1.3 ± 0.56 (H-47) | – | – |
| Seed 23, first mapping, mirrored | Plain on A's (H-47) | 12 | 29.6 | 24.8 | 3.5 | +1.1 ± 0.70 (H-47) | +1.0 ± 1.17 (H-47) | – | – |
| Seed 23, first mapping, mirrored | Plain on A's (H-47) | 18 | 27.6 | 24.8 | 3.8 | −1.0 ± 0.92 (H-47) | +1.3 ± 0.69 (H-47) | – | – |
| Seed 23, first mapping, mirrored | Plain on A's (H-47) | 24 | 25.0 | 25.9 | 3.5 | −5.0 ± 1.25 (H-47) | +2.4 ± 0.38 (H-47) | – | – |
| Seed 23, first mapping, mirrored | Plain on A's (H-47) | 36 | 24.1 | 24.2 | 4.7 | −5.2 ± 1.50 (H-47) | +0.3 ± 0.88 (H-47) | – | – |
| Seed 24, eighth, assignment | After block 12 alone | 6 | 21.3 | 17.5 | 4.6 | = without replay | = without replay | −2.5 ± 0.77 (H-47) | +2.3 ± 0.71 (H-47) |
| Seed 24, eighth, assignment | After block 12 alone | 12 | 13.3 | 14.3 | 6.0 | −5.2 ± 0.70 | −2.1 ± 0.59 | −5.5 ± 1.67 | −1.1 ± 1.83 |
| Seed 24, eighth, assignment | After block 12 alone | 18 | 15.3 | 14.5 | 4.6 | −2.6 ± 0.46 | −1.9 ± 1.61 | +0.9 ± 1.16 | +0.4 ± 1.74 |
| Seed 24, eighth, assignment | After block 12 alone | 24 | 15.7 | 13.4 | 5.4 | −3.6 ± 0.95 | −3.0 ± 0.89 | +0.4 ± 1.54 | −1.0 ± 1.08 |
| Seed 24, eighth, assignment | After block 12 alone | 36 | 11.7 | 11.8 | 4.5 | −1.2 ± 0.90 | −0.9 ± 1.04 | −1.4 ± 1.06 | −2.5 ± 1.28 |
| Seed 24, eighth, assignment | After block 18 alone | 6 | 21.3 | 17.5 | 4.6 | = without replay | = without replay | −2.5 ± 0.77 (H-47) | +2.3 ± 0.71 (H-47) |
| Seed 24, eighth, assignment | After block 18 alone | 12 | 18.4 | 16.4 | 4.9 | = without replay | = without replay | −0.4 ± 1.31 (H-47) | +1.0 ± 1.51 (H-47) |
| Seed 24, eighth, assignment | After block 18 alone | 18 | 9.9 | 12.4 | 5.6 | −8.1 ± 0.66 | −4.0 ± 0.57 | −4.5 ± 1.33 | −1.7 ± 1.56 |
| Seed 24, eighth, assignment | After block 18 alone | 24 | 12.8 | 14.0 | 4.9 | −6.5 ± 0.94 | −2.4 ± 0.83 | −2.6 ± 1.04 | −0.4 ± 1.56 |
| Seed 24, eighth, assignment | After block 18 alone | 36 | 8.8 | 11.3 | 4.9 | −4.1 ± 0.51 | −1.4 ± 0.88 | −4.3 ± 0.97 | −3.0 ± 0.80 |
| Seed 24, eighth, assignment | After block 24 alone | 6 | 21.3 | 17.5 | 4.6 | = without replay | = without replay | −2.5 ± 0.77 (H-47) | +2.3 ± 0.71 (H-47) |
| Seed 24, eighth, assignment | After block 24 alone | 12 | 18.4 | 16.4 | 4.9 | = without replay | = without replay | −0.4 ± 1.31 (H-47) | +1.0 ± 1.51 (H-47) |
| Seed 24, eighth, assignment | After block 24 alone | 18 | 18.0 | 16.5 | 4.0 | = without replay | = without replay | +3.6 ± 0.86 (H-47) | +2.4 ± 1.34 (H-47) |
| Seed 24, eighth, assignment | After block 24 alone | 24 | 9.3 | 14.1 | 5.5 | −10.0 ± 0.33 | −2.3 ± 0.62 | −6.1 ± 0.86 | −0.3 ± 1.46 |
| Seed 24, eighth, assignment | After block 24 alone | 36 | 7.4 | 13.5 | 4.5 | −5.5 ± 0.75 | +0.9 ± 0.13 | −5.7 ± 0.53 | −0.8 ± 1.39 |
| Seed 24, eighth, assignment | Plain on A's (H-47) | 6 | 23.8 | 15.3 | 5.1 | +2.5 ± 0.77 (H-47) | −2.3 ± 0.71 (H-47) | – | – |
| Seed 24, eighth, assignment | Plain on A's (H-47) | 12 | 18.8 | 15.4 | 5.2 | +0.4 ± 1.31 (H-47) | −1.0 ± 1.51 (H-47) | – | – |
| Seed 24, eighth, assignment | Plain on A's (H-47) | 18 | 14.4 | 14.1 | 6.0 | −3.6 ± 0.86 (H-47) | −2.4 ± 1.34 (H-47) | – | – |
| Seed 24, eighth, assignment | Plain on A's (H-47) | 24 | 15.3 | 14.3 | 5.6 | −3.9 ± 0.78 (H-47) | −2.0 ± 1.34 (H-47) | – | – |
| Seed 24, eighth, assignment | Plain on A's (H-47) | 36 | 13.1 | 14.3 | 5.7 | +0.2 ± 0.76 (H-47) | +1.6 ± 1.50 (H-47) | – | – |
| Seed 22, first mapping, mirrored | After block 12 alone | 6 | 30.6 | 26.0 | 2.3 | = without replay | = without replay | −0.7 ± 0.36 (H-47) | −0.4 ± 0.54 (H-47) |
| Seed 22, first mapping, mirrored | After block 12 alone | 12 | 28.3 | 22.7 | 4.4 | +0.1 ± 0.34 | +1.4 ± 0.49 | −1.0 ± 1.26 | −0.4 ± 1.68 |
| Seed 22, first mapping, mirrored | After block 12 alone | 18 | 27.8 | 22.5 | 3.9 | −0.1 ± 0.21 | +1.4 ± 0.44 | +0.3 ± 1.32 | −2.6 ± 1.49 |
| Seed 22, first mapping, mirrored | After block 12 alone | 24 | 27.4 | 19.8 | 4.0 | −0.1 ± 0.21 | +1.4 ± 0.37 | −0.2 ± 1.43 | −3.2 ± 1.82 |
| Seed 22, first mapping, mirrored | After block 12 alone | 36 | 28.0 | 16.6 | 4.8 | +0.3 ± 0.11 | +0.3 ± 0.19 | +0.7 ± 1.12 | −5.1 ± 1.64 |
| Seed 22, first mapping, mirrored | After block 18 alone | 6 | 30.6 | 26.0 | 2.3 | = without replay | = without replay | −0.7 ± 0.36 (H-47) | −0.4 ± 0.54 (H-47) |
| Seed 22, first mapping, mirrored | After block 18 alone | 12 | 28.2 | 21.3 | 3.9 | = without replay | = without replay | −1.1 ± 0.68 (H-47) | −1.8 ± 1.42 (H-47) |
| Seed 22, first mapping, mirrored | After block 18 alone | 18 | 28.5 | 22.4 | 4.4 | +0.5 ± 0.40 | +1.3 ± 0.69 | +0.9 ± 1.25 | −2.8 ± 1.57 |
| Seed 22, first mapping, mirrored | After block 18 alone | 24 | 27.5 | 20.8 | 3.6 | −0.1 ± 0.25 | +2.4 ± 0.41 | −0.2 ± 1.18 | −2.2 ± 1.94 |
| Seed 22, first mapping, mirrored | After block 18 alone | 36 | 28.5 | 18.8 | 4.4 | +0.7 ± 0.32 | +2.4 ± 0.49 | +1.1 ± 0.87 | −2.9 ± 1.93 |
| Seed 22, first mapping, mirrored | After block 24 alone | 6 | 30.6 | 26.0 | 2.3 | = without replay | = without replay | −0.7 ± 0.36 (H-47) | −0.4 ± 0.54 (H-47) |
| Seed 22, first mapping, mirrored | After block 24 alone | 12 | 28.2 | 21.3 | 3.9 | = without replay | = without replay | −1.1 ± 0.68 (H-47) | −1.8 ± 1.42 (H-47) |
| Seed 22, first mapping, mirrored | After block 24 alone | 18 | 28.0 | 21.0 | 4.3 | = without replay | = without replay | +0.4 ± 1.04 (H-47) | −4.1 ± 1.42 (H-47) |
| Seed 22, first mapping, mirrored | After block 24 alone | 24 | 29.0 | 19.8 | 3.8 | +1.5 ± 0.68 | +1.5 ± 0.42 | +1.4 ± 1.08 | −3.1 ± 1.82 |
| Seed 22, first mapping, mirrored | After block 24 alone | 36 | 27.9 | 18.0 | 4.3 | +0.1 ± 0.49 | +1.6 ± 1.17 | +0.5 ± 0.95 | −3.8 ± 1.32 |
| Seed 22, first mapping, mirrored | Plain on A's (H-47) | 6 | 31.4 | 26.4 | 2.5 | +0.7 ± 0.36 (H-47) | +0.4 ± 0.54 (H-47) | – | – |
| Seed 22, first mapping, mirrored | Plain on A's (H-47) | 12 | 29.3 | 23.1 | 4.0 | +1.1 ± 0.68 (H-47) | +1.8 ± 1.42 (H-47) | – | – |
| Seed 22, first mapping, mirrored | Plain on A's (H-47) | 18 | 27.6 | 25.1 | 3.7 | −0.4 ± 1.04 (H-47) | +4.1 ± 1.42 (H-47) | – | – |
| Seed 22, first mapping, mirrored | Plain on A's (H-47) | 24 | 27.6 | 23.0 | 3.8 | +0.1 ± 1.09 (H-47) | +4.6 ± 1.47 (H-47) | – | – |
| Seed 22, first mapping, mirrored | Plain on A's (H-47) | 36 | 27.4 | 21.8 | 4.3 | −0.4 ± 0.83 (H-47) | +5.4 ± 1.47 (H-47) | – | – |
| Seed 22, eighth, assignment | After block 12 alone | 6 | 24.4 | 25.9 | 3.7 | = without replay | = without replay | −2.0 ± 0.46 (H-47) | +0.1 ± 0.58 (H-47) |
| Seed 22, eighth, assignment | After block 12 alone | 12 | 25.8 | 25.6 | 4.3 | +2.9 ± 0.69 | +0.9 ± 0.39 | +0.4 ± 0.79 | +0.4 ± 1.21 |
| Seed 22, eighth, assignment | After block 12 alone | 18 | 24.6 | 24.6 | 4.2 | +0.2 ± 0.50 | +1.8 ± 0.54 | +0.9 ± 1.27 | −2.2 ± 0.76 |
| Seed 22, eighth, assignment | After block 12 alone | 24 | 26.0 | 22.0 | 3.8 | 0.0 ± 0.58 | +0.8 ± 0.49 | +0.3 ± 1.62 | −2.4 ± 1.36 |
| Seed 22, eighth, assignment | After block 12 alone | 36 | 24.6 | 20.6 | 4.5 | −0.1 ± 0.40 | +1.2 ± 0.57 | +1.5 ± 1.16 | −2.6 ± 1.49 |
| Seed 22, eighth, assignment | After block 18 alone | 6 | 24.4 | 25.9 | 3.7 | = without replay | = without replay | −2.0 ± 0.46 (H-47) | +0.1 ± 0.58 (H-47) |
| Seed 22, eighth, assignment | After block 18 alone | 12 | 22.9 | 24.8 | 3.8 | = without replay | = without replay | −2.5 ± 0.45 (H-47) | −0.5 ± 0.81 (H-47) |
| Seed 22, eighth, assignment | After block 18 alone | 18 | 25.9 | 25.5 | 3.7 | +1.5 ± 0.83 | +2.7 ± 0.58 | +2.1 ± 1.11 | −1.3 ± 0.78 |
| Seed 22, eighth, assignment | After block 18 alone | 24 | 26.8 | 22.9 | 3.3 | +0.8 ± 0.31 | +1.6 ± 0.46 | +1.1 ± 1.44 | −1.5 ± 1.51 |
| Seed 22, eighth, assignment | After block 18 alone | 36 | 25.8 | 21.0 | 3.7 | +1.1 ± 0.56 | +1.5 ± 0.71 | +2.7 ± 1.40 | −2.3 ± 1.55 |
| Seed 22, eighth, assignment | After block 24 alone | 6 | 24.4 | 25.9 | 3.7 | = without replay | = without replay | −2.0 ± 0.46 (H-47) | +0.1 ± 0.58 (H-47) |
| Seed 22, eighth, assignment | After block 24 alone | 12 | 22.9 | 24.8 | 3.8 | = without replay | = without replay | −2.5 ± 0.45 (H-47) | −0.5 ± 0.81 (H-47) |
| Seed 22, eighth, assignment | After block 24 alone | 18 | 24.4 | 22.8 | 3.8 | = without replay | = without replay | +0.6 ± 0.88 (H-47) | −4.0 ± 0.58 (H-47) |
| Seed 22, eighth, assignment | After block 24 alone | 24 | 26.9 | 21.4 | 4.1 | +0.8 ± 0.56 | +0.1 ± 0.57 | +1.1 ± 1.41 | −3.0 ± 1.40 |
| Seed 22, eighth, assignment | After block 24 alone | 36 | 23.4 | 19.7 | 4.3 | −1.3 ± 0.44 | +0.2 ± 0.62 | +0.3 ± 1.14 | −3.5 ± 1.31 |
| Seed 22, eighth, assignment | Plain on A's (H-47) | 6 | 26.4 | 25.8 | 3.4 | +2.0 ± 0.46 (H-47) | −0.1 ± 0.58 (H-47) | – | – |
| Seed 22, eighth, assignment | Plain on A's (H-47) | 12 | 25.4 | 25.3 | 3.8 | +2.5 ± 0.45 (H-47) | +0.5 ± 0.81 (H-47) | – | – |
| Seed 22, eighth, assignment | Plain on A's (H-47) | 18 | 23.8 | 26.8 | 3.7 | −0.6 ± 0.88 (H-47) | +4.0 ± 0.58 (H-47) | – | – |
| Seed 22, eighth, assignment | Plain on A's (H-47) | 24 | 25.8 | 24.4 | 3.7 | −0.3 ± 0.99 (H-47) | +3.1 ± 1.17 (H-47) | – | – |
| Seed 22, eighth, assignment | Plain on A's (H-47) | 36 | 23.1 | 23.2 | 4.0 | −1.6 ± 0.88 (H-47) | +3.8 ± 0.81 (H-47) | – | – |

- **On A's trials at block 36, less the night without replay**, after block 12 alone lies above beyond 2 SE in seven cells and within in
  five; after block 18 alone above in five, below in two, seed 24's first mapping from the mirrored assignment, −1.2 ± 0.40, and seed 24's
  eighth image from the assignment, −4.1 ± 0.51, and within in five; after block 24 alone above in two, seed 23's eighth image from the
  mirrored assignment, +2.3 ± 0.28, and seed 23's first mapping from the assignment, +5.2 ± 0.21, below in five, −0.8 ± 0.34 on seed 24's
  eighth image from the mirrored assignment to −5.5 ± 0.75 on seed 24's eighth image from the assignment, and within in five.
- **On B's trials at block 36**, after block 12 alone lies above beyond 2 SE in six cells, below in two, seed 22's first mapping from the
  assignment, −0.6 ± 0.22, and seed 23's eighth image from the assignment, −4.7 ± 1.43, and within in four; after block 18 alone above in
  six and within in six; after block 24 alone above in three, seed 22's eighth image from the mirrored assignment, +4.3 ± 1.03, seed 24's
  eighth image from the mirrored assignment, +2.7 ± 0.89, and seed 24's eighth image from the assignment, +0.9 ± 0.13, and within in nine.
- **Of the cells where A's count lies above the night without replay beyond 2 SE at block 36**, under after block 12 alone, eight, the
  difference lies beyond 2 SE on A's trials alone in three, seed 23's eighth image from the mirrored assignment, seed 23's first mapping
  from the assignment and seed 22's first mapping from the mirrored assignment; on B's alone in one, seed 24's first mapping from the
  assignment; and on both in four, above on both on seed 22's eighth image from the mirrored assignment, seed 24's eighth image from the
  mirrored assignment and seed 24's first mapping from the mirrored assignment, and on seed 22's first mapping from the assignment above on
  A's, +3.8 ± 0.74, and below on B's, −0.6 ± 0.22. Under after block 18 alone, seven, on A's alone in three, seed 22's first mapping from
  the assignment, seed 23's eighth image from the mirrored assignment and seed 23's first mapping from the assignment; on B's alone in two,
  seed 22's eighth image from the mirrored assignment and seed 22's eighth image from the assignment; and above on both in two, seed 24's
  eighth image from the mirrored assignment and seed 22's first mapping from the mirrored assignment. Under after block 24 alone, three, on
  A's alone on seed 23's eighth image from the mirrored assignment and seed 23's first mapping from the assignment, and on B's alone on seed
  22's eighth image from the mirrored assignment.
- **Where A's count lies below beyond 2 SE at block 36**: under after block 12 alone on seed 23's eighth image from the assignment, B's
  trials below, −4.7 ± 1.43, and A's within, −0.9 ± 0.76; under after block 18 alone on seed 24's eighth image from the assignment, A's
  below, −4.1 ± 0.51, and B's within, −1.4 ± 0.88; under after block 24 alone on seed 23's first mapping from the mirrored assignment, A's
  below, −1.1 ± 0.24, and B's within, −1.3 ± 0.96, and on seed 24's eighth image from the assignment A's below, −5.5 ± 0.75, and B's
  above, +0.9 ± 0.13.
- **On seed 24's eighth image from the assignment, right after its replayed night**, each form lies below the night without replay beyond
  2 SE, −7.3 ± 0.88, −12.1 ± 0.63 and −12.3 ± 0.58 after block 12, 18 and 24 alone, on A's trials by −5.2 ± 0.70, −8.1 ± 0.66 and
  −10.0 ± 0.33 and on B's by −2.1 ± 0.59, −4.0 ± 0.57 and −2.3 ± 0.62, each beyond 2 SE. There H-47's plain night less its night without
  replay read −0.6 ± 1.03 at block 12 and −5.9 ± 1.53 and −5.9 ± 1.85 at blocks 18 and 24.

## 2. The four single nights at block 36

At block 36, each single night's difference from the night without replay, after block 6 alone from exploration 031's job (exploration
030's on seed 23's first mapping from the mirrored assignment) and the other three from this exploration's, each the difference of the two
means of 48, with their sum, signed, beside H-47's plain night less the night without replay ± its paired SE. Each sum is of the unrounded
differences, so the rounded figures beside it may add to 0.1 more or less. No SE is claimed for the sum; each single night's SE is in
section 7:

| Cell | Of | After block 6 alone less without replay | after 12 alone | after 18 alone | after 24 alone | Their sum | H-47: plain less without replay |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | A's count | +7.3 | +8.1 | +6.3 | +5.1 | +26.8 | +19.6 ± 1.06 |
| Seed 22, eighth, mirrored | A's trials | +0.8 | +2.5 | −0.1 | +0.8 | +3.9 | +9.2 ± 0.86 |
| Seed 22, eighth, mirrored | B's trials | +6.5 | +5.6 | +6.4 | +4.3 | +22.9 | +10.4 ± 0.54 |
| Seed 22, first mapping, assignment | A's count | +4.3 | +3.2 | +1.7 | +0.8 | +10.0 | +7.9 ± 0.51 |
| Seed 22, first mapping, assignment | A's trials | +4.0 | +3.8 | +1.8 | +0.8 | +10.4 | +8.4 ± 1.10 |
| Seed 22, first mapping, assignment | B's trials | +0.3 | −0.6 | −0.1 | 0.0 | −0.5 | −0.4 ± 0.83 |
| Seed 23, eighth, mirrored | A's count | +4.3 | +3.8 | +3.0 | +2.4 | +13.5 | +17.4 ± 1.08 |
| Seed 23, eighth, mirrored | A's trials | +3.7 | +3.6 | +2.9 | +2.3 | +12.4 | +15.9 ± 1.05 |
| Seed 23, eighth, mirrored | B's trials | +0.6 | +0.3 | +0.1 | +0.2 | +1.1 | +1.5 ± 0.28 |
| Seed 23, first mapping, assignment | A's count | +5.0 | +6.6 | +5.6 | +5.8 | +23.1 | +13.4 ± 1.56 |
| Seed 23, first mapping, assignment | A's trials | +6.4 | +5.0 | +5.0 | +5.2 | +21.6 | +13.5 ± 1.09 |
| Seed 23, first mapping, assignment | B's trials | −1.4 | +1.5 | +0.6 | +0.6 | +1.4 | −0.1 ± 0.62 |
| Seed 24, eighth, mirrored | A's count | +3.1 | +3.8 | +5.4 | +1.9 | +14.2 | +11.3 ± 0.96 |
| Seed 24, eighth, mirrored | A's trials | +0.7 | +0.7 | +1.7 | −0.8 | +2.3 | +1.5 ± 0.74 |
| Seed 24, eighth, mirrored | B's trials | +2.4 | +3.1 | +3.7 | +2.7 | +11.9 | +9.7 ± 0.98 |
| Seed 24, first mapping, assignment | A's count | +3.9 | +3.1 | +2.6 | +1.6 | +11.2 | +3.8 ± 1.57 |
| Seed 24, first mapping, assignment | A's trials | +0.8 | +0.8 | −0.1 | +1.0 | +2.4 | +3.3 ± 0.54 |
| Seed 24, first mapping, assignment | B's trials | +3.1 | +2.3 | +2.8 | +0.6 | +8.8 | +0.5 ± 1.44 |
| Seed 23, eighth, assignment | A's count | +1.3 | −5.6 | −1.1 | −1.5 | −7.0 | +4.1 ± 1.45 |
| Seed 23, eighth, assignment | A's trials | +2.4 | −0.9 | −0.5 | −1.9 | −0.8 | +2.3 ± 0.88 |
| Seed 23, eighth, assignment | B's trials | −1.2 | −4.7 | −0.7 | +0.3 | −6.2 | +1.8 ± 0.84 |
| Seed 24, first mapping, mirrored | A's count | +3.2 | +4.6 | +0.3 | +1.3 | +9.5 | +6.0 ± 1.54 |
| Seed 24, first mapping, mirrored | A's trials | +3.4 | +3.0 | −1.2 | +0.3 | +5.4 | +3.3 ± 0.67 |
| Seed 24, first mapping, mirrored | B's trials | −0.1 | +1.7 | +1.5 | +1.0 | +4.0 | +2.7 ± 0.99 |
| Seed 23, first mapping, mirrored | A's count | +1.7 | −0.5 | +0.3 | −2.4 | −0.9 | −4.9 ± 1.09 |
| Seed 23, first mapping, mirrored | A's trials | 0.0 | −1.8 | −1.3 | −1.1 | −4.1 | −5.2 ± 1.50 |
| Seed 23, first mapping, mirrored | B's trials | +1.7 | +1.3 | +1.5 | −1.3 | +3.2 | +0.3 ± 0.88 |
| Seed 24, eighth, assignment | A's count | −2.0 | −2.1 | −5.5 | −4.6 | −14.3 | +1.8 ± 1.79 |
| Seed 24, eighth, assignment | A's trials | −1.0 | −1.2 | −4.1 | −5.5 | −11.8 | +0.2 ± 0.76 |
| Seed 24, eighth, assignment | B's trials | −1.0 | −0.9 | −1.4 | +0.9 | −2.5 | +1.6 ± 1.50 |
| Seed 22, first mapping, mirrored | A's count | +1.3 | +0.5 | +3.1 | +1.7 | +6.6 | +5.0 ± 1.24 |
| Seed 22, first mapping, mirrored | A's trials | +0.1 | +0.3 | +0.7 | +0.1 | +1.2 | −0.4 ± 0.83 |
| Seed 22, first mapping, mirrored | B's trials | +1.1 | +0.3 | +2.4 | +1.6 | +5.4 | +5.4 ± 1.47 |
| Seed 22, eighth, assignment | A's count | −0.9 | +1.1 | +2.6 | −1.1 | +1.7 | +2.2 ± 0.65 |
| Seed 22, eighth, assignment | A's trials | −2.0 | −0.1 | +1.1 | −1.3 | −2.4 | −1.6 ± 0.88 |
| Seed 22, eighth, assignment | B's trials | +1.1 | +1.2 | +1.5 | +0.2 | +4.0 | +3.8 ± 0.81 |

- **On A's count** the four single nights' differences sum to −14.3 on seed 24's eighth image from the assignment to +26.8 on seed 22's
  eighth image from the mirrored assignment. The sum lies above H-47's plain night less the night without replay in eight cells and below
  it in four: seed 23's eighth image from the mirrored assignment, +13.5 beside +17.4 ± 1.08, seed 23's eighth image from the assignment,
  −7.0 beside +4.1 ± 1.45, seed 24's eighth image from the assignment, −14.3 beside +1.8 ± 1.79, and seed 22's eighth image from the
  assignment, +1.7 beside +2.2 ± 0.65. On seed 23's first mapping from the mirrored assignment it is exploration 030's, −0.9 beside
  −4.9 ± 1.09.
- **On A's trials** the sum lies above H-47's in six cells and below in six; **on B's** above in eight and below in four, by less than 0.1
  on two of the twelve: seed 22's first mapping from the mirrored assignment, +5.4 beside +5.4 ± 1.47, above, and seed 22's first mapping
  from the assignment, −0.5 beside −0.4 ± 0.83, below.
- **Each single night's difference is positive** in all four on seed 22's eighth image from the mirrored assignment, seed 22's first
  mapping from the assignment, seed 23's eighth image from the mirrored assignment, seed 23's first mapping from the assignment, seed 24's
  eighth image from the mirrored assignment, seed 24's first mapping from the assignment, seed 24's first mapping from the mirrored
  assignment and seed 22's first mapping from the mirrored assignment, and negative in all four on seed 24's eighth image from the
  assignment.

## 3. The side under each form

Each stimulus's trials less the night without replay of the form's own job, and A's less B's, ± their paired SEs, with the side: the
stimulus whose difference is the larger, and its group of units, the places the stimulus answers on that cell under A. A form's rows before
its replayed night's block are the night without replay's, so its side is read from that block's checkpoint on. H-47's side is its plain
night's against its night without replay, H-47's job's:

| Cell | Form | Block | A's trials less without replay | B's trials less without replay | A's less B's | Side | H-47's side |
| :--- | :--- | ---: | ---: | ---: | ---: | :--- | :--- |
| Seed 22, eighth, mirrored | After block 12 alone | 12 | +4.3 ± 0.60 | +4.7 ± 0.58 | −0.4 ± 1.08 | B, the even places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | After block 12 alone | 18 | +1.6 ± 1.40 | +5.3 ± 0.47 | −3.7 ± 1.74 | B, the even places | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | After block 12 alone | 24 | +2.5 ± 1.27 | +4.4 ± 0.64 | −2.0 ± 1.79 | B, the even places (within 2 SE) | B, the even places |
| Seed 22, eighth, mirrored | After block 12 alone | 36 | +2.5 ± 1.08 | +5.6 ± 0.96 | −3.1 ± 1.90 | B, the even places (within 2 SE) | B, the even places (within 2 SE) |
| Seed 22, eighth, mirrored | After block 18 alone | 18 | −1.1 ± 1.10 | +6.6 ± 0.86 | −7.7 ± 1.63 | B, the even places | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | After block 18 alone | 24 | −1.8 ± 0.34 | +6.3 ± 0.80 | −8.1 ± 0.83 | B, the even places | B, the even places |
| Seed 22, eighth, mirrored | After block 18 alone | 36 | −0.1 ± 0.39 | +6.4 ± 0.55 | −6.5 ± 0.61 | B, the even places | B, the even places (within 2 SE) |
| Seed 22, eighth, mirrored | After block 24 alone | 24 | −2.0 ± 0.39 | +6.6 ± 0.95 | −8.6 ± 1.36 | B, the even places | B, the even places |
| Seed 22, eighth, mirrored | After block 24 alone | 36 | +0.8 ± 0.89 | +4.3 ± 1.03 | −3.6 ± 1.40 | B, the even places | B, the even places (within 2 SE) |
| Seed 22, first mapping, assignment | After block 12 alone | 12 | +3.9 ± 0.88 | +0.7 ± 0.93 | +3.2 ± 1.62 | A, the odd places (within 2 SE) | A, the odd places |
| Seed 22, first mapping, assignment | After block 12 alone | 18 | +5.6 ± 1.15 | −0.3 ± 0.92 | +5.9 ± 1.76 | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | After block 12 alone | 24 | +3.9 ± 1.46 | +0.1 ± 0.52 | +3.8 ± 1.75 | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | After block 12 alone | 36 | +3.8 ± 0.74 | −0.6 ± 0.22 | +4.4 ± 0.91 | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | After block 18 alone | 18 | +4.6 ± 1.32 | +1.9 ± 0.73 | +2.7 ± 1.78 | A, the odd places (within 2 SE) | A, the odd places |
| Seed 22, first mapping, assignment | After block 18 alone | 24 | +0.5 ± 1.03 | +0.9 ± 0.50 | −0.4 ± 1.38 | B, the even places (within 2 SE) | A, the odd places |
| Seed 22, first mapping, assignment | After block 18 alone | 36 | +1.8 ± 0.38 | −0.1 ± 0.23 | +2.0 ± 0.34 | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | After block 24 alone | 24 | +1.0 ± 1.29 | +2.6 ± 0.41 | −1.7 ± 1.56 | B, the even places (within 2 SE) | A, the odd places |
| Seed 22, first mapping, assignment | After block 24 alone | 36 | +0.8 ± 1.04 | 0.0 ± 1.26 | +0.9 ± 1.92 | A, the odd places (within 2 SE) | A, the odd places |
| Seed 23, eighth, mirrored | After block 12 alone | 12 | +1.7 ± 0.48 | −0.7 ± 0.55 | +2.4 ± 0.69 | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | After block 12 alone | 18 | +4.0 ± 0.68 | +1.3 ± 0.61 | +2.7 ± 1.10 | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | After block 12 alone | 24 | +3.0 ± 0.57 | +0.8 ± 0.32 | +2.2 ± 0.54 | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | After block 12 alone | 36 | +3.6 ± 0.55 | +0.3 ± 0.28 | +3.3 ± 0.30 | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | After block 18 alone | 18 | +2.4 ± 0.92 | −0.4 ± 0.34 | +2.8 ± 1.08 | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | After block 18 alone | 24 | +3.1 ± 0.43 | −0.2 ± 0.29 | +3.3 ± 0.35 | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | After block 18 alone | 36 | +2.9 ± 0.47 | +0.1 ± 0.34 | +2.9 ± 0.56 | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | After block 24 alone | 24 | +1.1 ± 1.24 | −0.2 ± 0.39 | +1.3 ± 1.49 | A, the odd places (within 2 SE) | A, the odd places |
| Seed 23, eighth, mirrored | After block 24 alone | 36 | +2.3 ± 0.28 | +0.2 ± 0.26 | +2.1 ± 0.41 | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | After block 12 alone | 12 | +5.1 ± 0.79 | −0.6 ± 0.85 | +5.7 ± 0.62 | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | After block 12 alone | 18 | +6.2 ± 1.58 | −0.3 ± 1.03 | +6.5 ± 0.79 | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | After block 12 alone | 24 | +4.3 ± 0.67 | +1.8 ± 1.17 | +2.5 ± 0.84 | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | After block 12 alone | 36 | +5.0 ± 1.27 | +1.5 ± 0.82 | +3.5 ± 1.31 | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | After block 18 alone | 18 | +5.6 ± 0.93 | −1.2 ± 0.72 | +6.8 ± 0.84 | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | After block 18 alone | 24 | +5.2 ± 1.18 | +0.3 ± 0.56 | +4.9 ± 1.49 | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | After block 18 alone | 36 | +5.0 ± 1.15 | +0.6 ± 0.36 | +4.4 ± 1.30 | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | After block 24 alone | 24 | +4.0 ± 0.73 | −1.9 ± 0.40 | +5.9 ± 0.77 | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | After block 24 alone | 36 | +5.2 ± 0.21 | +0.6 ± 0.40 | +4.5 ± 0.33 | A, the odd places | A, the odd places |
| Seed 24, eighth, mirrored | After block 12 alone | 12 | +0.5 ± 0.83 | +0.2 ± 0.60 | +0.3 ± 0.96 | A, the odd places (within 2 SE) | B, the even places |
| Seed 24, eighth, mirrored | After block 12 alone | 18 | +3.7 ± 0.49 | +2.4 ± 0.73 | +1.3 ± 0.97 | A, the odd places (within 2 SE) | B, the even places |
| Seed 24, eighth, mirrored | After block 12 alone | 24 | +2.3 ± 0.97 | +2.6 ± 0.58 | −0.4 ± 1.02 | B, the even places (within 2 SE) | B, the even places |
| Seed 24, eighth, mirrored | After block 12 alone | 36 | +0.7 ± 0.29 | +3.1 ± 1.02 | −2.4 ± 1.27 | B, the even places (within 2 SE) | B, the even places |
| Seed 24, eighth, mirrored | After block 18 alone | 18 | +6.0 ± 0.68 | +3.1 ± 1.02 | +2.8 ± 1.48 | A, the odd places (within 2 SE) | B, the even places |
| Seed 24, eighth, mirrored | After block 18 alone | 24 | +1.1 ± 0.59 | +3.3 ± 0.82 | −2.2 ± 0.72 | B, the even places | B, the even places |
| Seed 24, eighth, mirrored | After block 18 alone | 36 | +1.7 ± 0.54 | +3.7 ± 0.86 | −2.0 ± 0.90 | B, the even places | B, the even places |
| Seed 24, eighth, mirrored | After block 24 alone | 24 | +1.9 ± 0.99 | +3.5 ± 1.23 | −1.6 ± 0.93 | B, the even places (within 2 SE) | B, the even places |
| Seed 24, eighth, mirrored | After block 24 alone | 36 | −0.8 ± 0.34 | +2.7 ± 0.89 | −3.5 ± 0.64 | B, the even places | B, the even places |
| Seed 24, first mapping, assignment | After block 12 alone | 12 | +2.5 ± 0.59 | +0.1 ± 0.45 | +2.4 ± 0.81 | A, the odd places | A, the odd places (within 2 SE) |
| Seed 24, first mapping, assignment | After block 12 alone | 18 | +0.4 ± 0.43 | +1.0 ± 0.55 | −0.6 ± 0.72 | B, the even places (within 2 SE) | A, the odd places |
| Seed 24, first mapping, assignment | After block 12 alone | 24 | +1.1 ± 0.91 | +3.1 ± 1.23 | −2.1 ± 1.02 | B, the even places | A, the odd places |
| Seed 24, first mapping, assignment | After block 12 alone | 36 | +0.8 ± 1.08 | +2.3 ± 0.75 | −1.5 ± 1.82 | B, the even places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 24, first mapping, assignment | After block 18 alone | 18 | +2.8 ± 0.72 | +1.3 ± 0.95 | +1.5 ± 1.20 | A, the odd places (within 2 SE) | A, the odd places |
| Seed 24, first mapping, assignment | After block 18 alone | 24 | +1.6 ± 0.60 | +2.8 ± 0.60 | −1.2 ± 0.89 | B, the even places (within 2 SE) | A, the odd places |
| Seed 24, first mapping, assignment | After block 18 alone | 36 | −0.1 ± 0.68 | +2.8 ± 0.92 | −2.9 ± 0.82 | B, the even places | A, the odd places (within 2 SE) |
| Seed 24, first mapping, assignment | After block 24 alone | 24 | +4.9 ± 0.93 | +3.4 ± 0.95 | +1.5 ± 1.16 | A, the odd places (within 2 SE) | A, the odd places |
| Seed 24, first mapping, assignment | After block 24 alone | 36 | +1.0 ± 0.57 | +0.6 ± 1.66 | +0.4 ± 2.09 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 23, eighth, assignment | After block 12 alone | 12 | +0.6 ± 0.64 | −2.5 ± 0.66 | +3.1 ± 0.95 | A, the even places | A, the even places |
| Seed 23, eighth, assignment | After block 12 alone | 18 | +1.1 ± 0.64 | −3.5 ± 0.52 | +4.6 ± 0.88 | A, the even places | A, the even places (within 2 SE) |
| Seed 23, eighth, assignment | After block 12 alone | 24 | −0.6 ± 0.85 | −4.9 ± 0.88 | +4.3 ± 0.89 | A, the even places | A, the even places (within 2 SE) |
| Seed 23, eighth, assignment | After block 12 alone | 36 | −0.9 ± 0.76 | −4.7 ± 1.43 | +3.8 ± 1.27 | A, the even places | A, the even places (within 2 SE) |
| Seed 23, eighth, assignment | After block 18 alone | 18 | +0.7 ± 1.21 | +2.1 ± 0.55 | −1.5 ± 0.77 | B, the odd places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 23, eighth, assignment | After block 18 alone | 24 | −0.1 ± 0.68 | −0.6 ± 0.72 | +0.5 ± 0.62 | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 23, eighth, assignment | After block 18 alone | 36 | −0.5 ± 0.93 | −0.7 ± 0.93 | +0.2 ± 0.55 | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 23, eighth, assignment | After block 24 alone | 24 | −0.3 ± 0.58 | +1.7 ± 0.86 | −2.0 ± 0.97 | B, the odd places | A, the even places (within 2 SE) |
| Seed 23, eighth, assignment | After block 24 alone | 36 | −1.9 ± 0.06 | +0.3 ± 1.46 | −2.2 ± 1.49 | B, the odd places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | After block 12 alone | 12 | +1.9 ± 0.68 | +0.2 ± 0.82 | +1.7 ± 0.98 | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | After block 12 alone | 18 | +3.4 ± 0.61 | +0.6 ± 0.68 | +2.8 ± 1.14 | A, the even places | A, the even places |
| Seed 24, first mapping, mirrored | After block 12 alone | 24 | +3.6 ± 0.58 | +0.8 ± 0.95 | +2.8 ± 0.49 | A, the even places | B, the odd places (within 2 SE) |
| Seed 24, first mapping, mirrored | After block 12 alone | 36 | +3.0 ± 0.74 | +1.7 ± 0.76 | +1.3 ± 0.60 | A, the even places | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | After block 18 alone | 18 | +0.4 ± 0.94 | −1.3 ± 0.72 | +1.7 ± 0.89 | A, the even places (within 2 SE) | A, the even places |
| Seed 24, first mapping, mirrored | After block 18 alone | 24 | −0.5 ± 0.59 | +1.6 ± 0.67 | −2.1 ± 0.89 | B, the odd places | B, the odd places (within 2 SE) |
| Seed 24, first mapping, mirrored | After block 18 alone | 36 | −1.2 ± 0.40 | +1.5 ± 0.64 | −2.6 ± 0.83 | B, the odd places | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | After block 24 alone | 24 | +2.3 ± 1.18 | −0.3 ± 0.68 | +2.6 ± 1.12 | A, the even places | B, the odd places (within 2 SE) |
| Seed 24, first mapping, mirrored | After block 24 alone | 36 | +0.3 ± 0.70 | +1.0 ± 0.53 | −0.8 ± 1.04 | B, the odd places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 23, first mapping, mirrored | After block 12 alone | 12 | +1.3 ± 0.44 | +0.3 ± 0.43 | +1.0 ± 0.69 | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 23, first mapping, mirrored | After block 12 alone | 18 | −0.5 ± 0.45 | +1.4 ± 0.60 | −1.8 ± 0.74 | B, the odd places | B, the odd places |
| Seed 23, first mapping, mirrored | After block 12 alone | 24 | −0.8 ± 0.53 | +1.3 ± 0.38 | −2.0 ± 0.82 | B, the odd places | B, the odd places |
| Seed 23, first mapping, mirrored | After block 12 alone | 36 | −1.8 ± 1.04 | +1.3 ± 0.58 | −3.1 ± 1.27 | B, the odd places | B, the odd places |
| Seed 23, first mapping, mirrored | After block 18 alone | 18 | −0.1 ± 0.42 | +0.7 ± 0.78 | −0.8 ± 0.89 | B, the odd places (within 2 SE) | B, the odd places |
| Seed 23, first mapping, mirrored | After block 18 alone | 24 | −1.0 ± 0.34 | +1.3 ± 0.28 | −2.3 ± 0.43 | B, the odd places | B, the odd places |
| Seed 23, first mapping, mirrored | After block 18 alone | 36 | −1.3 ± 0.67 | +1.5 ± 0.76 | −2.8 ± 1.14 | B, the odd places | B, the odd places |
| Seed 23, first mapping, mirrored | After block 24 alone | 24 | −0.9 ± 0.39 | +0.6 ± 0.73 | −1.5 ± 0.85 | B, the odd places (within 2 SE) | B, the odd places |
| Seed 23, first mapping, mirrored | After block 24 alone | 36 | −1.1 ± 0.24 | −1.3 ± 0.96 | +0.3 ± 1.11 | A, the even places (within 2 SE) | B, the odd places |
| Seed 24, eighth, assignment | After block 12 alone | 12 | −5.2 ± 0.70 | −2.1 ± 0.59 | −3.0 ± 0.95 | B, the odd places | A, the even places (within 2 SE) |
| Seed 24, eighth, assignment | After block 12 alone | 18 | −2.6 ± 0.46 | −1.9 ± 1.61 | −0.7 ± 1.51 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | After block 12 alone | 24 | −3.6 ± 0.95 | −3.0 ± 0.89 | −0.6 ± 1.79 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | After block 12 alone | 36 | −1.2 ± 0.90 | −0.9 ± 1.04 | −0.4 ± 0.80 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | After block 18 alone | 18 | −8.1 ± 0.66 | −4.0 ± 0.57 | −4.1 ± 1.06 | B, the odd places | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | After block 18 alone | 24 | −6.5 ± 0.94 | −2.4 ± 0.83 | −4.1 ± 1.57 | B, the odd places | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | After block 18 alone | 36 | −4.1 ± 0.51 | −1.4 ± 0.88 | −2.7 ± 1.09 | B, the odd places | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | After block 24 alone | 24 | −10.0 ± 0.33 | −2.3 ± 0.62 | −7.7 ± 0.80 | B, the odd places | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | After block 24 alone | 36 | −5.5 ± 0.75 | +0.9 ± 0.13 | −6.4 ± 0.77 | B, the odd places | B, the odd places (within 2 SE) |
| Seed 22, first mapping, mirrored | After block 12 alone | 12 | +0.1 ± 0.34 | +1.4 ± 0.49 | −1.3 ± 0.56 | B, the odd places | B, the odd places (within 2 SE) |
| Seed 22, first mapping, mirrored | After block 12 alone | 18 | −0.1 ± 0.21 | +1.4 ± 0.44 | −1.6 ± 0.48 | B, the odd places | B, the odd places |
| Seed 22, first mapping, mirrored | After block 12 alone | 24 | −0.1 ± 0.21 | +1.4 ± 0.37 | −1.5 ± 0.56 | B, the odd places | B, the odd places |
| Seed 22, first mapping, mirrored | After block 12 alone | 36 | +0.3 ± 0.11 | +0.3 ± 0.19 | 0.0 ± 0.31 | A, the even places (within 2 SE) | B, the odd places |
| Seed 22, first mapping, mirrored | After block 18 alone | 18 | +0.5 ± 0.40 | +1.3 ± 0.69 | −0.8 ± 0.64 | B, the odd places (within 2 SE) | B, the odd places |
| Seed 22, first mapping, mirrored | After block 18 alone | 24 | −0.1 ± 0.25 | +2.4 ± 0.41 | −2.5 ± 0.33 | B, the odd places | B, the odd places |
| Seed 22, first mapping, mirrored | After block 18 alone | 36 | +0.7 ± 0.32 | +2.4 ± 0.49 | −1.7 ± 0.66 | B, the odd places | B, the odd places |
| Seed 22, first mapping, mirrored | After block 24 alone | 24 | +1.5 ± 0.68 | +1.5 ± 0.42 | +0.1 ± 0.89 | A, the even places (within 2 SE) | B, the odd places |
| Seed 22, first mapping, mirrored | After block 24 alone | 36 | +0.1 ± 0.49 | +1.6 ± 1.17 | −1.5 ± 1.17 | B, the odd places (within 2 SE) | B, the odd places |
| Seed 22, eighth, assignment | After block 12 alone | 12 | +2.9 ± 0.69 | +0.9 ± 0.39 | +2.1 ± 0.81 | A, the even places | A, the even places |
| Seed 22, eighth, assignment | After block 12 alone | 18 | +0.2 ± 0.50 | +1.8 ± 0.54 | −1.6 ± 0.73 | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | After block 12 alone | 24 | 0.0 ± 0.58 | +0.8 ± 0.49 | −0.8 ± 0.68 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 22, eighth, assignment | After block 12 alone | 36 | −0.1 ± 0.40 | +1.2 ± 0.57 | −1.3 ± 0.57 | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | After block 18 alone | 18 | +1.5 ± 0.83 | +2.7 ± 0.58 | −1.2 ± 0.79 | B, the odd places (within 2 SE) | B, the odd places |
| Seed 22, eighth, assignment | After block 18 alone | 24 | +0.8 ± 0.31 | +1.6 ± 0.46 | −0.9 ± 0.62 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 22, eighth, assignment | After block 18 alone | 36 | +1.1 ± 0.56 | +1.5 ± 0.71 | −0.4 ± 1.25 | B, the odd places (within 2 SE) | B, the odd places |
| Seed 22, eighth, assignment | After block 24 alone | 24 | +0.8 ± 0.56 | +0.1 ± 0.57 | +0.7 ± 0.74 | A, the even places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 22, eighth, assignment | After block 24 alone | 36 | −1.3 ± 0.44 | +0.2 ± 0.62 | −1.6 ± 0.86 | B, the odd places (within 2 SE) | B, the odd places |

- **At block 36** the side falls on the stimulus of H-47's side under after block 12 alone in ten cells: not on seed 24's first mapping from
  the assignment, where it lies on B, the even places, within 2 SE, A's less B's −1.5 ± 1.82, and H-47's on A, the odd places, within 2 SE,
  nor on seed 22's first mapping from the mirrored assignment, where it lies on A, the even places, within 2 SE, 0.0 ± 0.31, and H-47's on
  B, the odd places, beyond 2 SE. Under after block 18 alone in ten: not on seed 24's first mapping from the assignment, B, the even places,
  −2.9 ± 0.82, nor on seed 24's first mapping from the mirrored assignment, B, the odd places, −2.6 ± 0.83, both beyond 2 SE where H-47's
  lies on A within 2 SE. Under after block 24 alone in nine: not on seed 23's eighth image from the assignment, B, the odd places,
  −2.2 ± 1.49, seed 24's first mapping from the mirrored assignment, B, the odd places, −0.8 ± 1.04, both within 2 SE where H-47's lies on
  A within 2 SE, nor seed 23's first mapping from the mirrored assignment, A, the even places, within 2 SE, +0.3 ± 1.11, where H-47's lies
  on B beyond 2 SE. After block 6 alone's falls on H-47's stimulus in ten (exploration 031; section 7).
- **At block 36** the side lies on A in six cells under after block 12 alone, four under after block 18 alone and five under after block 24
  alone, and beyond 2 SE in seven, ten and five. No checkpoint of any form in any cell reads neither.
- **Right after its replayed night** the side falls on H-47's stimulus under after block 12 alone at block 12 in nine cells, under after
  block 18 alone at block 18 in nine and under after block 24 alone at block 24 in seven.

## 4. The pair and A's couplings

The pair's count under each form at each checkpoint, the mean of 48 reads, and A's highest coupling over the same coupling in H-42's image
on the cell's seed, in any image read and at any block's end of the form's 36, as H-47's clauses 2 and 3 read them:

| Cell | Form | The pair at block 6 | 12 | 18 | 24 | 36 | A's highest coupling over H-42's |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | Without replay | 43.3 | 60.5 | 60.7 | 61.1 | 63.0 | 1.144 |
| Seed 22, eighth, mirrored | After block 12 alone | 43.3 | 62.4 | 61.1 | 62.1 | 63.3 | 1.144 |
| Seed 22, eighth, mirrored | After block 18 alone | 43.3 | 60.5 | 62.9 | 61.8 | 63.1 | 1.144 |
| Seed 22, eighth, mirrored | After block 24 alone | 43.3 | 60.5 | 60.7 | 63.3 | 63.2 | 1.144 |
| Seed 22, eighth, mirrored | Plain on A's (H-47) | 45.8 | 63.1 | 62.8 | 62.6 | 62.8 | 1.197 |
| Seed 22, first mapping, assignment | Without replay | 45.7 | 60.9 | 61.6 | 61.5 | 63.3 | 1.112 |
| Seed 22, first mapping, assignment | After block 12 alone | 45.7 | 62.3 | 61.2 | 61.1 | 63.2 | 1.115 |
| Seed 22, first mapping, assignment | After block 18 alone | 45.7 | 60.9 | 62.5 | 61.7 | 63.1 | 1.112 |
| Seed 22, first mapping, assignment | After block 24 alone | 45.7 | 60.9 | 61.6 | 63.0 | 63.3 | 1.112 |
| Seed 22, first mapping, assignment | Plain on A's (H-47) | 42.1 | 61.7 | 61.4 | 61.7 | 63.1 | 1.198 |
| Seed 23, eighth, mirrored | Without replay | 54.0 | 60.6 | 60.3 | 60.9 | 62.6 | 1.147 |
| Seed 23, eighth, mirrored | After block 12 alone | 54.0 | 62.7 | 59.9 | 61.4 | 63.0 | 1.147 |
| Seed 23, eighth, mirrored | After block 18 alone | 54.0 | 60.6 | 62.7 | 61.9 | 63.0 | 1.147 |
| Seed 23, eighth, mirrored | After block 24 alone | 54.0 | 60.6 | 60.3 | 62.6 | 63.2 | 1.147 |
| Seed 23, eighth, mirrored | Plain on A's (H-47) | 57.8 | 61.6 | 62.0 | 61.9 | 63.1 | 1.211 |
| Seed 23, first mapping, assignment | Without replay | 28.0 | 58.7 | 61.4 | 61.2 | 63.1 | 1.125 |
| Seed 23, first mapping, assignment | After block 12 alone | 28.0 | 59.0 | 61.1 | 60.3 | 63.0 | 1.125 |
| Seed 23, first mapping, assignment | After block 18 alone | 28.0 | 58.7 | 61.9 | 60.6 | 62.9 | 1.125 |
| Seed 23, first mapping, assignment | After block 24 alone | 28.0 | 58.7 | 61.4 | 62.1 | 62.7 | 1.125 |
| Seed 23, first mapping, assignment | Plain on A's (H-47) | 27.7 | 56.2 | 61.4 | 60.4 | 61.9 | 1.212 |
| Seed 24, eighth, mirrored | Without replay | 58.0 | 59.1 | 60.7 | 60.0 | 62.7 | 1.078 |
| Seed 24, eighth, mirrored | After block 12 alone | 58.0 | 62.4 | 60.1 | 60.0 | 62.8 | 1.078 |
| Seed 24, eighth, mirrored | After block 18 alone | 58.0 | 59.1 | 62.0 | 61.1 | 62.5 | 1.078 |
| Seed 24, eighth, mirrored | After block 24 alone | 58.0 | 59.1 | 60.7 | 61.8 | 63.1 | 1.078 |
| Seed 24, eighth, mirrored | Plain on A's (H-47) | 62.0 | 63.0 | 62.5 | 62.5 | 63.0 | 1.141 |
| Seed 24, first mapping, assignment | Without replay | 34.1 | 50.5 | 59.1 | 59.7 | 62.1 | 1.085 |
| Seed 24, first mapping, assignment | After block 12 alone | 34.1 | 53.5 | 59.6 | 59.5 | 62.3 | 1.085 |
| Seed 24, first mapping, assignment | After block 18 alone | 34.1 | 50.5 | 60.5 | 58.5 | 61.8 | 1.085 |
| Seed 24, first mapping, assignment | After block 24 alone | 34.1 | 50.5 | 59.1 | 60.6 | 62.5 | 1.085 |
| Seed 24, first mapping, assignment | Plain on A's (H-47) | 34.8 | 52.4 | 60.0 | 59.2 | 61.8 | 1.162 |
| Seed 23, eighth, assignment | Without replay | 52.4 | 60.5 | 61.4 | 62.4 | 63.4 | 1.118 |
| Seed 23, eighth, assignment | After block 12 alone | 52.4 | 62.5 | 62.4 | 62.4 | 63.5 | 1.118 |
| Seed 23, eighth, assignment | After block 18 alone | 52.4 | 60.5 | 62.8 | 62.5 | 63.4 | 1.118 |
| Seed 23, eighth, assignment | After block 24 alone | 52.4 | 60.5 | 61.4 | 63.1 | 63.4 | 1.118 |
| Seed 23, eighth, assignment | Plain on A's (H-47) | 54.7 | 62.3 | 62.5 | 63.2 | 63.4 | 1.171 |
| Seed 24, first mapping, mirrored | Without replay | 50.1 | 60.5 | 62.4 | 63.1 | 63.4 | 1.177 |
| Seed 24, first mapping, mirrored | After block 12 alone | 50.1 | 61.8 | 61.4 | 62.3 | 63.1 | 1.177 |
| Seed 24, first mapping, mirrored | After block 18 alone | 50.1 | 60.5 | 62.9 | 62.1 | 62.9 | 1.177 |
| Seed 24, first mapping, mirrored | After block 24 alone | 50.1 | 60.5 | 62.4 | 63.4 | 63.1 | 1.177 |
| Seed 24, first mapping, mirrored | Plain on A's (H-47) | 51.6 | 61.6 | 62.1 | 62.0 | 62.4 | 1.223 |
| Seed 23, first mapping, mirrored | Without replay | 50.4 | 60.9 | 62.1 | 62.6 | 63.5 | 1.082 |
| Seed 23, first mapping, mirrored | After block 12 alone | 50.4 | 60.8 | 61.0 | 62.3 | 63.3 | 1.087 |
| Seed 23, first mapping, mirrored | After block 18 alone | 50.4 | 60.9 | 62.9 | 62.2 | 63.4 | 1.082 |
| Seed 23, first mapping, mirrored | After block 24 alone | 50.4 | 60.9 | 62.1 | 63.2 | 63.4 | 1.082 |
| Seed 23, first mapping, mirrored | Plain on A's (H-47) | 49.3 | 59.3 | 61.9 | 62.9 | 63.2 | 1.158 |
| Seed 24, eighth, assignment | Without replay | 58.8 | 59.3 | 58.9 | 58.3 | 62.6 | 1.179 |
| Seed 24, eighth, assignment | After block 12 alone | 58.8 | 63.0 | 61.2 | 60.2 | 62.8 | 1.179 |
| Seed 24, eighth, assignment | After block 18 alone | 58.8 | 59.3 | 62.2 | 60.1 | 62.7 | 1.179 |
| Seed 24, eighth, assignment | After block 24 alone | 58.8 | 59.3 | 58.9 | 62.1 | 62.6 | 1.179 |
| Seed 24, eighth, assignment | Plain on A's (H-47) | 61.0 | 62.2 | 62.6 | 62.1 | 62.9 | 1.200 |
| Seed 22, first mapping, mirrored | Without replay | 53.1 | 61.0 | 62.6 | 62.4 | 63.1 | 1.114 |
| Seed 22, first mapping, mirrored | After block 12 alone | 53.1 | 62.5 | 61.9 | 62.5 | 63.1 | 1.117 |
| Seed 22, first mapping, mirrored | After block 18 alone | 53.1 | 61.0 | 63.3 | 62.5 | 63.1 | 1.117 |
| Seed 22, first mapping, mirrored | After block 24 alone | 53.1 | 61.0 | 62.6 | 62.9 | 63.4 | 1.114 |
| Seed 22, first mapping, mirrored | Plain on A's (H-47) | 54.3 | 61.4 | 62.4 | 61.6 | 62.5 | 1.182 |
| Seed 22, eighth, assignment | Without replay | 62.1 | 61.3 | 61.6 | 60.9 | 62.6 | 1.133 |
| Seed 22, eighth, assignment | After block 12 alone | 62.1 | 62.9 | 61.3 | 61.0 | 62.6 | 1.133 |
| Seed 22, eighth, assignment | After block 18 alone | 62.1 | 61.3 | 63.1 | 60.8 | 62.6 | 1.133 |
| Seed 22, eighth, assignment | After block 24 alone | 62.1 | 61.3 | 61.6 | 62.9 | 62.8 | 1.133 |
| Seed 22, eighth, assignment | Plain on A's (H-47) | 63.1 | 62.6 | 63.3 | 61.9 | 62.9 | 1.184 |

- **The pair at block 36** stands at 62.3 to 63.5 under after block 12 alone, the lowest on seed 24's first mapping from the assignment and
  the highest on seed 23's eighth image from the assignment; at 61.8 to 63.4 under after block 18 alone and at 62.5 to 63.4 under after
  block 24 alone, the lowest on seed 24's first mapping from the assignment under both and the highest on seed 23's first mapping from the
  mirrored assignment under both, shared under after block 18 alone with seed 23's eighth image from the assignment; above clause 2's floor
  of 56 in every cell. It stands at 62.1 to 63.5 under the night without replay and at 61.8 to 63.4 under H-47's plain night.
- **A's highest coupling** reads 1.078 to 1.179 of H-42's under each of the three forms, the lowest on seed 24's eighth image from the
  mirrored assignment and the highest on seed 24's eighth image from the assignment; none passes clause 3's 1.30. Under after block 24 alone
  it is the night without replay's in all twelve cells; under after block 18 alone in eleven, above it on seed 22's first mapping from the
  mirrored assignment, 1.117 beside 1.114; under after block 12 alone in nine, above it on seed 22's first mapping from the assignment,
  1.115 beside 1.112, seed 23's first mapping from the mirrored assignment, 1.087 beside 1.082, and seed 22's first mapping from the
  mirrored assignment, 1.117 beside 1.114. It lies below H-47's plain night's, 1.141 to 1.223, in all twelve under each form.

## 5. What each night and each stretch moved

What each night moved, from the class matrix in the image before it and the image after it, by group, the mean over the three paths, in M.
The odd places' excitatory units are classes 4, 6 and 7, the even places' 5, 8 and 9; "onto their own" counts each group's excitatory
units onto the same group's; the inhibitory rows are the odd places' class 11 and the even places' class 12 onto every class, and
"inhibitory onto" every inhibitory class, 11 to 13, onto a group's excitatory units, a + more inhibition. A form is listed at a night only
from its replayed night on; before it each form's night is the night without replay's. Beside the forms are H-47's night without replay
and plain night on the same cell, from exploration 020's job on its four cells and exploration 021's on the other eight; H-47's plain night
after block 6 is its first replayed night. This job's night without replay is not listed: its lines are those of H-47's night without
replay in that job, label aside (section "The runs are laid out as written"). Where A answers readout 0 its group is the odd places, and
where it answers readout 1 the even places:

| Cell | Night after block | Form | This night | The odd places' excitatory onto all, M | the even places' | The odd places' onto their own | the even places' onto their own | The odd places' inhibitory onto all | the even places' | Inhibitory onto the odd places' excitatory | onto the even places' |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | Without replay (H-47, 021's job) | without replay | −0.22 | −0.30 | −0.33 | −0.30 | +0.15 | +0.16 | +0.27 | +0.21 |
| Seed 22, eighth, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −4.04 | −4.28 | −0.47 | −0.47 | +5.08 | +5.03 | +4.66 | +3.94 |
| Seed 22, eighth, mirrored | 12 | After block 12 alone | replays | −3.70 | −4.07 | −0.35 | −0.31 | +5.20 | +5.27 | +4.80 | +3.98 |
| Seed 22, eighth, mirrored | 12 | Without replay (H-47, 021's job) | without replay | +0.04 | −0.20 | −0.13 | −0.17 | +0.12 | +0.19 | +0.28 | +0.18 |
| Seed 22, eighth, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −2.75 | −2.89 | −0.25 | −0.22 | +4.02 | +4.05 | +3.74 | +3.18 |
| Seed 22, eighth, mirrored | 18 | After block 12 alone | without replay | +0.14 | −0.09 | −0.08 | −0.18 | −0.31 | −0.19 | −0.07 | −0.20 |
| Seed 22, eighth, mirrored | 18 | After block 18 alone | replays | −3.28 | −3.71 | −0.06 | −0.22 | +5.08 | +5.12 | +4.60 | +3.87 |
| Seed 22, eighth, mirrored | 18 | Without replay (H-47, 021's job) | without replay | +0.13 | −0.08 | −0.06 | −0.14 | +0.08 | +0.22 | +0.27 | +0.14 |
| Seed 22, eighth, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −1.61 | −1.90 | +0.06 | −0.05 | +3.29 | +3.61 | +3.16 | +2.73 |
| Seed 22, eighth, mirrored | 24 | After block 12 alone | without replay | +0.41 | −0.01 | +0.09 | −0.13 | −0.23 | −0.12 | −0.06 | −0.12 |
| Seed 22, eighth, mirrored | 24 | After block 18 alone | without replay | +0.27 | +0.08 | −0.01 | −0.13 | −0.33 | −0.20 | −0.12 | −0.21 |
| Seed 22, eighth, mirrored | 24 | After block 24 alone | replays | −3.15 | −3.46 | −0.07 | −0.22 | +5.07 | +5.28 | +4.60 | +3.90 |
| Seed 22, eighth, mirrored | 24 | Without replay (H-47, 021's job) | without replay | +0.35 | 0.00 | +0.04 | −0.07 | +0.13 | +0.26 | +0.28 | +0.16 |
| Seed 22, eighth, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −0.87 | −1.31 | +0.18 | +0.12 | +2.73 | +3.16 | +2.67 | +2.36 |
| Seed 22, first mapping, assignment | 6 | Without replay (H-47, 021's job) | without replay | −0.35 | −0.19 | −0.40 | −0.27 | −0.64 | −0.58 | −0.40 | −0.47 |
| Seed 22, first mapping, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −3.45 | −3.38 | −0.39 | −0.17 | +4.42 | +4.66 | +3.96 | +3.26 |
| Seed 22, first mapping, assignment | 12 | After block 12 alone | replays | −3.11 | −2.97 | −0.11 | −0.21 | +4.25 | +4.40 | +3.74 | +3.11 |
| Seed 22, first mapping, assignment | 12 | Without replay (H-47, 021's job) | without replay | +0.11 | −0.04 | −0.16 | −0.20 | −0.73 | −0.64 | −0.51 | −0.52 |
| Seed 22, first mapping, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −2.60 | −2.73 | −0.30 | −0.23 | +2.93 | +3.09 | +2.59 | +2.28 |
| Seed 22, first mapping, assignment | 18 | After block 12 alone | without replay | +0.37 | +0.11 | −0.12 | −0.16 | −0.86 | −0.84 | −0.61 | −0.68 |
| Seed 22, first mapping, assignment | 18 | After block 18 alone | replays | −2.41 | −3.02 | +0.17 | 0.00 | +4.27 | +4.50 | +3.69 | +3.13 |
| Seed 22, first mapping, assignment | 18 | Without replay (H-47, 021's job) | without replay | +0.38 | +0.05 | +0.02 | −0.11 | −0.79 | −0.67 | −0.52 | −0.62 |
| Seed 22, first mapping, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −1.53 | −1.89 | +0.05 | −0.10 | +1.91 | +2.07 | +1.64 | +1.48 |
| Seed 22, first mapping, assignment | 24 | After block 12 alone | without replay | +0.39 | 0.00 | +0.05 | −0.12 | −0.85 | −0.73 | −0.52 | −0.65 |
| Seed 22, first mapping, assignment | 24 | After block 18 alone | without replay | +0.25 | +0.06 | 0.00 | −0.12 | −0.86 | −0.71 | −0.51 | −0.66 |
| Seed 22, first mapping, assignment | 24 | After block 24 alone | replays | −2.87 | −2.85 | +0.06 | +0.04 | +4.10 | +4.40 | +3.64 | +2.97 |
| Seed 22, first mapping, assignment | 24 | Without replay (H-47, 021's job) | without replay | +0.34 | −0.03 | +0.05 | −0.13 | −0.77 | −0.56 | −0.46 | −0.58 |
| Seed 22, first mapping, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −1.13 | −1.47 | +0.14 | −0.03 | +1.25 | +1.35 | +1.09 | +0.92 |
| Seed 23, eighth, mirrored | 6 | Without replay (H-47, 021's job) | without replay | −0.27 | −0.41 | −0.18 | −0.22 | +0.18 | +0.15 | +0.26 | +0.20 |
| Seed 23, eighth, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −4.48 | −4.16 | −0.35 | −0.46 | +5.57 | +5.00 | +4.71 | +4.17 |
| Seed 23, eighth, mirrored | 12 | After block 12 alone | replays | −4.41 | −3.62 | −0.39 | −0.44 | +5.61 | +5.03 | +4.75 | +4.09 |
| Seed 23, eighth, mirrored | 12 | Without replay (H-47, 021's job) | without replay | +0.33 | −0.21 | −0.09 | −0.13 | +0.23 | +0.15 | +0.27 | +0.19 |
| Seed 23, eighth, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −2.90 | −2.73 | −0.11 | −0.27 | +4.52 | +3.97 | +3.80 | +3.46 |
| Seed 23, eighth, mirrored | 18 | After block 12 alone | without replay | +0.36 | −0.02 | −0.10 | −0.12 | −0.32 | −0.28 | −0.20 | −0.16 |
| Seed 23, eighth, mirrored | 18 | After block 18 alone | replays | −3.86 | −3.27 | −0.35 | −0.25 | +5.79 | +5.04 | +4.69 | +4.14 |
| Seed 23, eighth, mirrored | 18 | Without replay (H-47, 021's job) | without replay | +0.60 | +0.24 | +0.11 | −0.01 | +0.24 | +0.22 | +0.22 | +0.26 |
| Seed 23, eighth, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −2.15 | −1.96 | −0.05 | −0.20 | +3.61 | +3.44 | +3.21 | +2.84 |
| Seed 23, eighth, mirrored | 24 | After block 12 alone | without replay | +0.77 | +0.31 | +0.18 | −0.02 | −0.13 | −0.23 | −0.07 | −0.09 |
| Seed 23, eighth, mirrored | 24 | After block 18 alone | without replay | +0.88 | +0.34 | +0.14 | −0.03 | −0.24 | −0.29 | −0.15 | −0.17 |
| Seed 23, eighth, mirrored | 24 | After block 24 alone | replays | −3.83 | −3.20 | −0.30 | −0.28 | +5.75 | +5.09 | +4.71 | +4.07 |
| Seed 23, eighth, mirrored | 24 | Without replay (H-47, 021's job) | without replay | +0.83 | +0.68 | +0.33 | +0.17 | +0.37 | +0.26 | +0.38 | +0.23 |
| Seed 23, eighth, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −1.36 | −1.47 | +0.10 | −0.08 | +3.02 | +2.87 | +2.67 | +2.42 |
| Seed 23, first mapping, assignment | 6 | Without replay (H-47, 021's job) | without replay | −0.20 | −0.23 | −0.20 | −0.21 | −0.50 | −0.53 | −0.25 | −0.41 |
| Seed 23, first mapping, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −3.48 | −3.23 | −0.22 | −0.29 | +5.07 | +4.52 | +4.28 | +3.42 |
| Seed 23, first mapping, assignment | 12 | After block 12 alone | replays | −3.39 | −2.51 | −0.20 | −0.04 | +4.82 | +4.56 | +4.13 | +3.32 |
| Seed 23, first mapping, assignment | 12 | Without replay (H-47, 021's job) | without replay | +0.36 | +0.12 | −0.04 | −0.07 | −0.66 | −0.63 | −0.46 | −0.44 |
| Seed 23, first mapping, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −2.71 | −2.55 | −0.14 | −0.19 | +3.26 | +3.06 | +2.77 | +2.41 |
| Seed 23, first mapping, assignment | 18 | After block 12 alone | without replay | +0.44 | +0.03 | −0.04 | −0.14 | −0.86 | −0.86 | −0.62 | −0.60 |
| Seed 23, first mapping, assignment | 18 | After block 18 alone | replays | −3.06 | −2.62 | −0.09 | −0.20 | +4.88 | +4.33 | +4.08 | +3.14 |
| Seed 23, first mapping, assignment | 18 | Without replay (H-47, 021's job) | without replay | +0.62 | +0.15 | +0.10 | −0.08 | −0.69 | −0.72 | −0.48 | −0.53 |
| Seed 23, first mapping, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −1.53 | −1.61 | +0.31 | −0.05 | +1.96 | +1.76 | +1.64 | +1.40 |
| Seed 23, first mapping, assignment | 24 | After block 12 alone | without replay | +0.67 | +0.20 | +0.11 | −0.04 | −0.87 | −0.83 | −0.61 | −0.64 |
| Seed 23, first mapping, assignment | 24 | After block 18 alone | without replay | +0.66 | +0.29 | +0.06 | −0.06 | −0.89 | −0.87 | −0.63 | −0.66 |
| Seed 23, first mapping, assignment | 24 | After block 24 alone | replays | −3.30 | −2.34 | −0.23 | −0.09 | +4.84 | +4.36 | +4.06 | +3.10 |
| Seed 23, first mapping, assignment | 24 | Without replay (H-47, 021's job) | without replay | +0.69 | +0.49 | +0.14 | +0.04 | −0.72 | −0.69 | −0.46 | −0.61 |
| Seed 23, first mapping, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −1.10 | −1.22 | +0.24 | −0.09 | +1.33 | +1.27 | +1.19 | +0.84 |
| Seed 24, eighth, mirrored | 6 | Without replay (H-47, 021's job) | without replay | −0.08 | −0.43 | −0.23 | −0.41 | +0.14 | +0.17 | +0.24 | +0.23 |
| Seed 24, eighth, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −3.90 | −4.11 | −0.56 | −0.45 | +4.88 | +4.60 | +4.62 | +3.60 |
| Seed 24, eighth, mirrored | 12 | After block 12 alone | replays | −3.43 | −3.80 | −0.32 | −0.55 | +4.89 | +4.77 | +4.71 | +3.56 |
| Seed 24, eighth, mirrored | 12 | Without replay (H-47, 021's job) | without replay | +0.26 | −0.02 | −0.11 | −0.23 | +0.14 | +0.12 | +0.22 | +0.19 |
| Seed 24, eighth, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −2.52 | −3.05 | −0.15 | −0.49 | +3.91 | +3.90 | +3.78 | +3.02 |
| Seed 24, eighth, mirrored | 18 | After block 12 alone | without replay | +0.31 | +0.18 | −0.17 | −0.17 | −0.31 | −0.23 | −0.20 | −0.06 |
| Seed 24, eighth, mirrored | 18 | After block 18 alone | replays | −3.32 | −3.63 | −0.42 | −0.40 | +4.87 | +4.96 | +4.67 | +3.62 |
| Seed 24, eighth, mirrored | 18 | Without replay (H-47, 021's job) | without replay | +0.28 | +0.06 | −0.10 | −0.13 | +0.13 | +0.22 | +0.23 | +0.23 |
| Seed 24, eighth, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −1.61 | −2.21 | −0.05 | −0.25 | +3.28 | +3.33 | +3.26 | +2.47 |
| Seed 24, eighth, mirrored | 24 | After block 12 alone | without replay | +0.39 | +0.28 | −0.07 | +0.02 | −0.07 | −0.08 | −0.05 | +0.09 |
| Seed 24, eighth, mirrored | 24 | After block 18 alone | without replay | +0.35 | +0.26 | −0.12 | −0.06 | −0.21 | −0.23 | −0.17 | −0.03 |
| Seed 24, eighth, mirrored | 24 | After block 24 alone | replays | −3.31 | −3.61 | −0.40 | −0.32 | +4.98 | +4.96 | +4.67 | +3.63 |
| Seed 24, eighth, mirrored | 24 | Without replay (H-47, 021's job) | without replay | +0.27 | +0.12 | +0.03 | −0.04 | +0.25 | +0.24 | +0.24 | +0.29 |
| Seed 24, eighth, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −0.93 | −1.72 | +0.15 | −0.07 | +3.06 | +2.80 | +2.95 | +2.10 |
| Seed 24, first mapping, assignment | 6 | Without replay (H-47, 021's job) | without replay | +0.19 | +0.01 | −0.17 | −0.28 | −0.62 | −0.60 | −0.35 | −0.43 |
| Seed 24, first mapping, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −2.81 | −3.36 | −0.26 | −0.36 | +4.45 | +4.28 | +4.31 | +2.78 |
| Seed 24, first mapping, assignment | 12 | After block 12 alone | replays | −3.00 | −2.79 | −0.20 | −0.39 | +4.11 | +4.04 | +4.03 | +2.59 |
| Seed 24, first mapping, assignment | 12 | Without replay (H-47, 021's job) | without replay | +0.25 | +0.27 | −0.14 | −0.22 | −0.70 | −0.63 | −0.47 | −0.45 |
| Seed 24, first mapping, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −2.41 | −2.58 | −0.23 | −0.45 | +3.05 | +2.88 | +3.04 | +1.94 |
| Seed 24, first mapping, assignment | 18 | After block 12 alone | without replay | +0.39 | +0.17 | −0.11 | −0.16 | −0.83 | −0.81 | −0.55 | −0.59 |
| Seed 24, first mapping, assignment | 18 | After block 18 alone | replays | −2.67 | −2.85 | −0.17 | −0.17 | +4.21 | +4.14 | +4.08 | +2.65 |
| Seed 24, first mapping, assignment | 18 | Without replay (H-47, 021's job) | without replay | +0.48 | +0.23 | −0.09 | −0.08 | −0.67 | −0.63 | −0.43 | −0.52 |
| Seed 24, first mapping, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −1.55 | −1.92 | +0.09 | −0.13 | +1.89 | +1.96 | +1.88 | +1.26 |
| Seed 24, first mapping, assignment | 24 | After block 12 alone | without replay | +0.41 | +0.30 | −0.08 | +0.04 | −0.81 | −0.78 | −0.60 | −0.55 |
| Seed 24, first mapping, assignment | 24 | After block 18 alone | without replay | +0.56 | +0.32 | −0.05 | 0.00 | −0.84 | −0.83 | −0.62 | −0.56 |
| Seed 24, first mapping, assignment | 24 | After block 24 alone | replays | −2.98 | −2.79 | −0.17 | −0.17 | +4.03 | +4.03 | +3.95 | +2.52 |
| Seed 24, first mapping, assignment | 24 | Without replay (H-47, 021's job) | without replay | +0.41 | +0.40 | 0.00 | +0.07 | −0.68 | −0.64 | −0.49 | −0.46 |
| Seed 24, first mapping, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −0.77 | −1.66 | +0.14 | −0.14 | +1.14 | +1.42 | +1.25 | +0.80 |
| Seed 23, eighth, assignment | 6 | Without replay (H-47, 021's job) | without replay | −0.28 | −0.53 | −0.06 | −0.26 | +0.20 | +0.16 | +0.26 | +0.25 |
| Seed 23, eighth, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −4.78 | −4.62 | −1.07 | −1.03 | +5.05 | +4.67 | +4.10 | +4.29 |
| Seed 23, eighth, assignment | 12 | After block 12 alone | replays | −4.52 | −4.18 | −1.01 | −0.97 | +5.03 | +4.75 | +3.96 | +4.33 |
| Seed 23, eighth, assignment | 12 | Without replay (H-47, 021's job) | without replay | +0.34 | −0.25 | +0.03 | −0.10 | +0.17 | +0.17 | +0.25 | +0.21 |
| Seed 23, eighth, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −3.49 | −2.92 | −0.70 | −0.58 | +4.10 | +3.74 | +3.22 | +3.53 |
| Seed 23, eighth, assignment | 18 | After block 12 alone | without replay | +0.26 | +0.09 | −0.02 | −0.06 | −0.37 | −0.25 | −0.16 | −0.19 |
| Seed 23, eighth, assignment | 18 | After block 18 alone | replays | −4.35 | −3.62 | −0.96 | −0.80 | +5.19 | +4.76 | +3.97 | +4.33 |
| Seed 23, eighth, assignment | 18 | Without replay (H-47, 021's job) | without replay | +0.37 | +0.29 | +0.13 | 0.00 | +0.21 | +0.24 | +0.27 | +0.27 |
| Seed 23, eighth, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −2.25 | −2.07 | −0.26 | −0.35 | +3.38 | +3.10 | +2.56 | +3.04 |
| Seed 23, eighth, assignment | 24 | After block 12 alone | without replay | +0.51 | +0.28 | +0.19 | +0.07 | −0.15 | −0.14 | −0.01 | −0.08 |
| Seed 23, eighth, assignment | 24 | After block 18 alone | without replay | +0.70 | +0.40 | +0.24 | +0.07 | −0.26 | −0.23 | −0.08 | −0.17 |
| Seed 23, eighth, assignment | 24 | After block 24 alone | replays | −3.74 | −3.71 | −0.73 | −0.76 | +5.08 | +4.68 | +3.90 | +4.17 |
| Seed 23, eighth, assignment | 24 | Without replay (H-47, 021's job) | without replay | +0.95 | +0.58 | +0.36 | +0.14 | +0.33 | +0.35 | +0.39 | +0.29 |
| Seed 23, eighth, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −1.50 | −1.40 | −0.16 | −0.15 | +2.99 | +2.56 | +2.19 | +2.61 |
| Seed 24, first mapping, mirrored | 6 | Without replay (H-47, 021's job) | without replay | +0.13 | −0.01 | −0.16 | −0.28 | −0.65 | −0.63 | −0.38 | −0.45 |
| Seed 24, first mapping, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −3.58 | −4.21 | −0.67 | −1.11 | +4.12 | +3.81 | +3.25 | +3.14 |
| Seed 24, first mapping, mirrored | 12 | After block 12 alone | replays | −3.24 | −3.79 | −0.45 | −1.05 | +3.89 | +3.71 | +3.00 | +3.07 |
| Seed 24, first mapping, mirrored | 12 | Without replay (H-47, 021's job) | without replay | +0.26 | +0.25 | −0.15 | −0.23 | −0.70 | −0.66 | −0.48 | −0.46 |
| Seed 24, first mapping, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −2.83 | −2.86 | −0.51 | −0.70 | +2.58 | +2.37 | +2.02 | +2.07 |
| Seed 24, first mapping, mirrored | 18 | After block 12 alone | without replay | +0.38 | +0.25 | −0.13 | −0.15 | −0.84 | −0.84 | −0.57 | −0.63 |
| Seed 24, first mapping, mirrored | 18 | After block 18 alone | replays | −3.04 | −3.44 | −0.50 | −0.80 | +3.88 | +3.71 | +2.91 | +3.08 |
| Seed 24, first mapping, mirrored | 18 | Without replay (H-47, 021's job) | without replay | +0.50 | +0.22 | −0.09 | −0.10 | −0.67 | −0.63 | −0.43 | −0.50 |
| Seed 24, first mapping, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −1.89 | −2.14 | −0.16 | −0.44 | +1.81 | +1.74 | +1.38 | +1.52 |
| Seed 24, first mapping, mirrored | 24 | After block 12 alone | without replay | +0.28 | +0.25 | −0.09 | −0.05 | −0.82 | −0.82 | −0.58 | −0.63 |
| Seed 24, first mapping, mirrored | 24 | After block 18 alone | without replay | +0.39 | +0.31 | −0.08 | −0.08 | −0.85 | −0.83 | −0.61 | −0.61 |
| Seed 24, first mapping, mirrored | 24 | After block 24 alone | replays | −3.03 | −3.49 | −0.35 | −0.82 | +3.78 | +3.63 | +2.81 | +2.95 |
| Seed 24, first mapping, mirrored | 24 | Without replay (H-47, 021's job) | without replay | +0.40 | +0.33 | +0.02 | +0.03 | −0.69 | −0.64 | −0.47 | −0.50 |
| Seed 24, first mapping, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −1.57 | −1.51 | −0.14 | −0.25 | +1.26 | +1.41 | +1.07 | +1.09 |
| Seed 23, first mapping, mirrored | 6 | Without replay (H-47, 020's job) | without replay | −0.16 | −0.29 | −0.17 | −0.23 | −0.61 | −0.58 | −0.37 | −0.46 |
| Seed 23, first mapping, mirrored | 6 | Plain on A's (H-47, 020's job) | replays | −3.99 | −3.68 | −0.95 | −0.76 | +4.74 | +4.10 | +3.55 | +3.56 |
| Seed 23, first mapping, mirrored | 12 | After block 12 alone | replays | −3.65 | −3.43 | −0.84 | −0.72 | +4.61 | +3.88 | +3.33 | +3.44 |
| Seed 23, first mapping, mirrored | 12 | Without replay (H-47, 020's job) | without replay | +0.35 | +0.03 | +0.03 | −0.08 | −0.74 | −0.66 | −0.51 | −0.48 |
| Seed 23, first mapping, mirrored | 12 | Plain on A's (H-47, 020's job) | replays | −3.00 | −2.80 | −0.58 | −0.62 | +3.14 | +2.67 | +2.31 | +2.45 |
| Seed 23, first mapping, mirrored | 18 | After block 12 alone | without replay | +0.30 | +0.10 | +0.01 | −0.08 | −0.86 | −0.88 | −0.61 | −0.64 |
| Seed 23, first mapping, mirrored | 18 | After block 18 alone | replays | −3.50 | −3.04 | −0.69 | −0.57 | +4.56 | +3.93 | +3.32 | +3.42 |
| Seed 23, first mapping, mirrored | 18 | Without replay (H-47, 020's job) | without replay | +0.60 | +0.04 | +0.15 | −0.07 | −0.68 | −0.74 | −0.47 | −0.53 |
| Seed 23, first mapping, mirrored | 18 | Plain on A's (H-47, 020's job) | replays | −1.95 | −1.93 | −0.18 | −0.17 | +2.05 | +1.75 | +1.49 | +1.62 |
| Seed 23, first mapping, mirrored | 24 | After block 12 alone | without replay | +0.29 | +0.12 | +0.02 | +0.03 | −0.87 | −0.89 | −0.63 | −0.65 |
| Seed 23, first mapping, mirrored | 24 | After block 18 alone | without replay | +0.39 | +0.30 | +0.07 | +0.11 | −0.85 | −0.87 | −0.60 | −0.67 |
| Seed 23, first mapping, mirrored | 24 | After block 24 alone | replays | −3.35 | −2.80 | −0.67 | −0.35 | +4.40 | +3.82 | +3.24 | +3.27 |
| Seed 23, first mapping, mirrored | 24 | Without replay (H-47, 020's job) | without replay | +0.68 | +0.36 | +0.16 | +0.11 | −0.67 | −0.66 | −0.44 | −0.57 |
| Seed 23, first mapping, mirrored | 24 | Plain on A's (H-47, 020's job) | replays | −1.53 | −1.31 | −0.11 | −0.02 | +1.38 | +1.24 | +0.98 | +1.13 |
| Seed 24, eighth, assignment | 6 | Without replay (H-47, 020's job) | without replay | −0.07 | −0.32 | −0.21 | −0.39 | +0.14 | +0.17 | +0.26 | +0.21 |
| Seed 24, eighth, assignment | 6 | Plain on A's (H-47, 020's job) | replays | −4.33 | −4.90 | −0.79 | −1.27 | +4.42 | +4.38 | +3.72 | +3.79 |
| Seed 24, eighth, assignment | 12 | After block 12 alone | replays | −4.06 | −4.53 | −0.66 | −1.16 | +4.60 | +4.52 | +3.82 | +3.84 |
| Seed 24, eighth, assignment | 12 | Without replay (H-47, 020's job) | without replay | +0.16 | −0.01 | −0.17 | −0.19 | +0.12 | +0.13 | +0.22 | +0.18 |
| Seed 24, eighth, assignment | 12 | Plain on A's (H-47, 020's job) | replays | −3.06 | −3.55 | −0.50 | −0.80 | +3.63 | +3.54 | +3.07 | +3.10 |
| Seed 24, eighth, assignment | 18 | After block 12 alone | without replay | +0.27 | +0.25 | −0.15 | −0.15 | −0.28 | −0.26 | −0.09 | −0.18 |
| Seed 24, eighth, assignment | 18 | After block 18 alone | replays | −3.75 | −4.18 | −0.64 | −1.07 | +4.30 | +4.50 | +3.55 | +3.70 |
| Seed 24, eighth, assignment | 18 | Without replay (H-47, 020's job) | without replay | +0.29 | +0.17 | −0.12 | −0.09 | +0.12 | +0.20 | +0.24 | +0.17 |
| Seed 24, eighth, assignment | 18 | Plain on A's (H-47, 020's job) | replays | −2.16 | −2.24 | −0.24 | −0.39 | +2.91 | +3.02 | +2.47 | +2.62 |
| Seed 24, eighth, assignment | 24 | After block 12 alone | without replay | +0.40 | +0.22 | −0.08 | −0.07 | −0.13 | −0.09 | −0.01 | −0.02 |
| Seed 24, eighth, assignment | 24 | After block 18 alone | without replay | +0.27 | +0.21 | −0.12 | −0.07 | −0.26 | −0.20 | −0.11 | −0.12 |
| Seed 24, eighth, assignment | 24 | After block 24 alone | replays | −3.49 | −4.13 | −0.41 | −0.94 | +4.56 | +4.42 | +3.61 | +3.69 |
| Seed 24, eighth, assignment | 24 | Without replay (H-47, 020's job) | without replay | +0.31 | +0.05 | −0.03 | −0.03 | +0.23 | +0.21 | +0.27 | +0.22 |
| Seed 24, eighth, assignment | 24 | Plain on A's (H-47, 020's job) | replays | −1.69 | −1.64 | −0.25 | −0.30 | +2.58 | +2.58 | +2.12 | +2.24 |
| Seed 22, first mapping, mirrored | 6 | Without replay (H-47, 020's job) | without replay | −0.29 | −0.18 | −0.32 | −0.28 | −0.67 | −0.65 | −0.45 | −0.51 |
| Seed 22, first mapping, mirrored | 6 | Plain on A's (H-47, 020's job) | replays | −3.57 | −3.94 | −1.01 | −0.92 | +3.72 | +3.63 | +2.73 | +3.02 |
| Seed 22, first mapping, mirrored | 12 | After block 12 alone | replays | −3.38 | −3.62 | −0.96 | −0.88 | +3.53 | +3.62 | +2.66 | +2.87 |
| Seed 22, first mapping, mirrored | 12 | Without replay (H-47, 020's job) | without replay | +0.15 | −0.09 | −0.11 | −0.22 | −0.72 | −0.67 | −0.49 | −0.55 |
| Seed 22, first mapping, mirrored | 12 | Plain on A's (H-47, 020's job) | replays | −2.68 | −3.06 | −0.83 | −0.77 | +2.31 | +2.44 | +1.91 | +1.89 |
| Seed 22, first mapping, mirrored | 18 | After block 12 alone | without replay | +0.40 | +0.07 | −0.02 | −0.18 | −0.87 | −0.82 | −0.57 | −0.71 |
| Seed 22, first mapping, mirrored | 18 | After block 18 alone | replays | −2.86 | −3.59 | −0.73 | −0.78 | +3.50 | +3.56 | +2.61 | +2.87 |
| Seed 22, first mapping, mirrored | 18 | Without replay (H-47, 020's job) | without replay | +0.31 | +0.06 | +0.02 | −0.14 | −0.75 | −0.64 | −0.47 | −0.62 |
| Seed 22, first mapping, mirrored | 18 | Plain on A's (H-47, 020's job) | replays | −1.63 | −2.18 | −0.28 | −0.34 | +1.60 | +1.61 | +1.14 | +1.34 |
| Seed 22, first mapping, mirrored | 24 | After block 12 alone | without replay | +0.23 | +0.14 | +0.02 | −0.13 | −0.88 | −0.76 | −0.55 | −0.66 |
| Seed 22, first mapping, mirrored | 24 | After block 18 alone | without replay | +0.33 | +0.15 | +0.04 | −0.14 | −0.88 | −0.74 | −0.55 | −0.66 |
| Seed 22, first mapping, mirrored | 24 | After block 24 alone | replays | −3.12 | −3.59 | −0.78 | −0.78 | +3.42 | +3.73 | +2.63 | +2.90 |
| Seed 22, first mapping, mirrored | 24 | Without replay (H-47, 020's job) | without replay | +0.34 | +0.01 | +0.08 | −0.16 | −0.79 | −0.61 | −0.51 | −0.56 |
| Seed 22, first mapping, mirrored | 24 | Plain on A's (H-47, 020's job) | replays | −1.32 | −1.61 | −0.29 | −0.22 | +1.06 | +1.18 | +0.74 | +0.97 |
| Seed 22, eighth, assignment | 6 | Without replay (H-47, 020's job) | without replay | −0.37 | −0.26 | −0.28 | −0.31 | +0.12 | +0.16 | +0.23 | +0.21 |
| Seed 22, eighth, assignment | 6 | Plain on A's (H-47, 020's job) | replays | −4.47 | −4.64 | −1.19 | −1.22 | +4.05 | +4.30 | +3.51 | +3.58 |
| Seed 22, eighth, assignment | 12 | After block 12 alone | replays | −4.15 | −4.19 | −1.00 | −1.09 | +4.01 | +4.45 | +3.46 | +3.59 |
| Seed 22, eighth, assignment | 12 | Without replay (H-47, 020's job) | without replay | +0.02 | −0.15 | −0.14 | −0.20 | +0.07 | +0.16 | +0.23 | +0.14 |
| Seed 22, eighth, assignment | 12 | Plain on A's (H-47, 020's job) | replays | −3.00 | −3.27 | −0.79 | −0.79 | +3.22 | +3.46 | +2.80 | +2.92 |
| Seed 22, eighth, assignment | 18 | After block 12 alone | without replay | +0.08 | +0.05 | −0.11 | −0.16 | −0.27 | −0.21 | −0.05 | −0.22 |
| Seed 22, eighth, assignment | 18 | After block 18 alone | replays | −4.09 | −4.19 | −1.00 | −1.01 | +4.01 | +4.40 | +3.37 | +3.64 |
| Seed 22, eighth, assignment | 18 | Without replay (H-47, 020's job) | without replay | +0.02 | −0.10 | −0.10 | −0.19 | +0.02 | +0.23 | +0.19 | +0.13 |
| Seed 22, eighth, assignment | 18 | Plain on A's (H-47, 020's job) | replays | −1.96 | −2.30 | −0.36 | −0.38 | +2.73 | +3.00 | +2.33 | +2.58 |
| Seed 22, eighth, assignment | 24 | After block 12 alone | without replay | +0.21 | +0.01 | −0.01 | −0.16 | −0.18 | −0.11 | +0.01 | −0.09 |
| Seed 22, eighth, assignment | 24 | After block 18 alone | without replay | +0.21 | +0.10 | +0.01 | −0.15 | −0.34 | −0.25 | −0.11 | −0.23 |
| Seed 22, eighth, assignment | 24 | After block 24 alone | replays | −3.88 | −3.91 | −0.99 | −0.86 | +4.04 | +4.35 | +3.37 | +3.58 |
| Seed 22, eighth, assignment | 24 | Without replay (H-47, 020's job) | without replay | +0.21 | +0.02 | +0.01 | −0.11 | +0.06 | +0.21 | +0.23 | +0.12 |
| Seed 22, eighth, assignment | 24 | Plain on A's (H-47, 020's job) | replays | −1.26 | −1.92 | −0.21 | −0.33 | +2.47 | +2.45 | +1.97 | +2.22 |

- **Each form's replayed night moved the readouts' synapses about as H-47's first plain night on the same cell did, whichever block it
  followed.** The excitatory units of the group A answers lost 3.00 to 4.53 M of weight onto all over after block 12 alone's, 2.41 to
  4.19 M over after block 18 alone's and 2.80 to 4.13 M over after block 24 alone's, and 2.81 to 4.90 M over H-47's first plain night; the
  inhibition onto them rose by 2.87 to 4.80 M, 2.87 to 4.69 M and 2.90 to 4.71 M, and by 3.02 to 4.71 M over H-47's first plain night.
  Less H-47's first plain night on the same cell, that rise reads −0.28 to +0.14 M, −0.27 to +0.06 M and −0.37 to +0.06 M.
- **Each form's replayed night moved more than H-47's plain night after the same block**, its second, third or fourth replayed night, in
  every cell: it took more excitatory weight from the group A answers, and raised the inhibition onto each group's excitatory units more,
  onto A's group by 0.67 to 1.36 M more after block 12, 1.05 to 2.45 M after block 18 and 1.36 to 2.87 M after block 24. H-47's plain
  night after those blocks raised the inhibition onto A's group by 1.89 to 3.80 M, 1.34 to 3.26 M and 0.97 to 2.95 M.
- **H-47's night without replay after the same block**, on the image each form's replayed night ran on, moved the excitatory units of A's
  group onto all by −0.25 to +0.36 M after block 12, −0.10 to +0.62 M after block 18 and +0.01 to +0.83 M after block 24, and the
  inhibition onto them by −0.55 to +0.28 M, −0.62 to +0.27 M and −0.57 to +0.38 M.
- **The nights after a form's replayed night are nights without replay after a replayed night**: after block 12 alone's after blocks 18
  and 24 and after block 18 alone's after block 24, 36 nights, three a cell. In each of them each of the table's four inhibitory columns
  came out lower than over H-47's night without replay after the same block on the same cell, all 144, by 0.02 to 0.61 M. Over them the
  inhibition onto the excitatory units of A's group fell by 0.02 to 0.71 M, where over H-47's nights without replay after blocks 18 and 24
  it moved by −0.62 to +0.38 M and over its plain nights it rose by 0.97 to 3.26 M; the excitatory units of A's group moved onto all by
  +0.01 to +0.88 M.

What each stretch of blocks moved, from the image after the night before it to the image before the next night, or block 36's; the head,
blocks 1 to 6, is the same in every form of a cell, from the start image to the image before the first night, and a form is listed at a
stretch only after its replayed night:

| Cell | Stretch | Form | The odd places' excitatory onto all | the even places' | The odd places' onto their own | the even places' onto their own | The odd places' inhibitory onto all | the even places' | Inhibitory onto the odd places' excitatory | onto the even places' |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | blocks 1 to 6 | Every form | 0.00 | 0.00 | −0.13 | −0.20 | +0.26 | +0.33 | +0.30 | +0.50 |
| Seed 22, eighth, mirrored | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.13 | −0.12 | +0.22 | +0.15 | +0.21 | +0.21 |
| Seed 22, eighth, mirrored | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.13 | −0.11 | −0.63 | −0.79 | −0.48 | −0.48 |
| Seed 22, eighth, mirrored | blocks 13 to 18 | After block 12 alone | 0.00 | 0.00 | −0.04 | −0.02 | −0.68 | −0.84 | −0.53 | −0.55 |
| Seed 22, eighth, mirrored | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.03 | −0.01 | +0.17 | +0.16 | +0.19 | +0.21 |
| Seed 22, eighth, mirrored | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.04 | −0.02 | −0.68 | −0.85 | −0.53 | −0.54 |
| Seed 22, eighth, mirrored | blocks 19 to 24 | After block 12 alone | 0.00 | 0.00 | −0.01 | −0.03 | −0.29 | −0.40 | −0.28 | −0.26 |
| Seed 22, eighth, mirrored | blocks 19 to 24 | After block 18 alone | 0.00 | 0.00 | −0.01 | −0.04 | −0.68 | −0.84 | −0.58 | −0.56 |
| Seed 22, eighth, mirrored | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.01 | +0.22 | +0.15 | +0.19 | +0.19 |
| Seed 22, eighth, mirrored | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.02 | −0.74 | −0.89 | −0.59 | −0.62 |
| Seed 22, eighth, mirrored | blocks 25 to 36 | After block 12 alone | 0.00 | 0.00 | −0.03 | −0.03 | −0.53 | −0.46 | −0.40 | −0.25 |
| Seed 22, eighth, mirrored | blocks 25 to 36 | After block 18 alone | 0.00 | 0.00 | −0.02 | −0.04 | −0.73 | −0.69 | −0.64 | −0.41 |
| Seed 22, eighth, mirrored | blocks 25 to 36 | After block 24 alone | 0.00 | 0.00 | −0.03 | −0.04 | −1.26 | −1.43 | −1.07 | −0.97 |
| Seed 22, eighth, mirrored | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.04 | −0.03 | +0.27 | +0.38 | +0.27 | +0.45 |
| Seed 22, eighth, mirrored | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.06 | −1.37 | −1.72 | −1.14 | −1.05 |
| Seed 22, first mapping, assignment | blocks 1 to 6 | Every form | 0.00 | 0.00 | −0.18 | −0.15 | −0.81 | −0.86 | −0.73 | −0.58 |
| Seed 22, first mapping, assignment | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.08 | −0.16 | −0.79 | −0.98 | −0.79 | −0.78 |
| Seed 22, first mapping, assignment | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.11 | −0.17 | −1.15 | −1.32 | −0.98 | −0.99 |
| Seed 22, first mapping, assignment | blocks 13 to 18 | After block 12 alone | 0.00 | 0.00 | 0.00 | −0.03 | −1.34 | −1.52 | −1.15 | −1.19 |
| Seed 22, first mapping, assignment | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.02 | −0.99 | −1.16 | −0.94 | −1.01 |
| Seed 22, first mapping, assignment | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.03 | −1.41 | −1.59 | −1.21 | −1.22 |
| Seed 22, first mapping, assignment | blocks 19 to 24 | After block 12 alone | 0.00 | 0.00 | 0.00 | −0.04 | −1.23 | −1.37 | −1.11 | −1.15 |
| Seed 22, first mapping, assignment | blocks 19 to 24 | After block 18 alone | 0.00 | 0.00 | −0.01 | −0.03 | −1.41 | −1.59 | −1.21 | −1.27 |
| Seed 22, first mapping, assignment | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.01 | −0.05 | −1.08 | −1.09 | −0.98 | −1.01 |
| Seed 22, first mapping, assignment | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.03 | −0.03 | −1.49 | −1.70 | −1.30 | −1.30 |
| Seed 22, first mapping, assignment | blocks 25 to 36 | After block 12 alone | 0.00 | 0.00 | −0.02 | −0.06 | −2.58 | −2.57 | −2.15 | −2.27 |
| Seed 22, first mapping, assignment | blocks 25 to 36 | After block 18 alone | 0.00 | 0.00 | 0.00 | −0.08 | −2.64 | −2.71 | −2.20 | −2.33 |
| Seed 22, first mapping, assignment | blocks 25 to 36 | After block 24 alone | 0.00 | 0.00 | −0.02 | −0.07 | −2.83 | −2.91 | −2.36 | −2.41 |
| Seed 22, first mapping, assignment | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.06 | −2.36 | −2.19 | −1.93 | −2.06 |
| Seed 22, first mapping, assignment | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.04 | −0.06 | −2.96 | −3.26 | −2.45 | −2.51 |
| Seed 23, eighth, mirrored | blocks 1 to 6 | Every form | 0.00 | 0.00 | −0.14 | −0.06 | +0.28 | +0.27 | +0.27 | +0.49 |
| Seed 23, eighth, mirrored | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.05 | −0.07 | +0.23 | +0.09 | +0.18 | +0.26 |
| Seed 23, eighth, mirrored | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.06 | −0.05 | −0.70 | −0.64 | −0.49 | −0.38 |
| Seed 23, eighth, mirrored | blocks 13 to 18 | After block 12 alone | 0.00 | 0.00 | −0.01 | −0.02 | −0.69 | −0.67 | −0.51 | −0.39 |
| Seed 23, eighth, mirrored | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.02 | +0.33 | +0.09 | +0.18 | +0.28 |
| Seed 23, eighth, mirrored | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.02 | −0.76 | −0.71 | −0.53 | −0.46 |
| Seed 23, eighth, mirrored | blocks 19 to 24 | After block 12 alone | 0.00 | 0.00 | −0.02 | −0.03 | −0.35 | −0.37 | −0.26 | −0.18 |
| Seed 23, eighth, mirrored | blocks 19 to 24 | After block 18 alone | 0.00 | 0.00 | 0.00 | −0.02 | −0.74 | −0.63 | −0.49 | −0.45 |
| Seed 23, eighth, mirrored | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.03 | +0.45 | +0.23 | +0.32 | +0.29 |
| Seed 23, eighth, mirrored | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.01 | −0.85 | −0.72 | −0.56 | −0.52 |
| Seed 23, eighth, mirrored | blocks 25 to 36 | After block 12 alone | 0.00 | 0.00 | −0.02 | −0.04 | −0.18 | −0.38 | −0.15 | −0.04 |
| Seed 23, eighth, mirrored | blocks 25 to 36 | After block 18 alone | 0.00 | 0.00 | −0.02 | −0.03 | −0.49 | −0.47 | −0.36 | −0.18 |
| Seed 23, eighth, mirrored | blocks 25 to 36 | After block 24 alone | 0.00 | 0.00 | −0.03 | −0.03 | −1.16 | −1.03 | −0.83 | −0.64 |
| Seed 23, eighth, mirrored | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.03 | −0.02 | +0.99 | +0.72 | +0.82 | +0.77 |
| Seed 23, eighth, mirrored | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.04 | −0.03 | −1.69 | −1.37 | −1.12 | −0.95 |
| Seed 23, first mapping, assignment | blocks 1 to 6 | Every form | 0.00 | 0.00 | −0.10 | −0.02 | −0.71 | −0.78 | −0.62 | −0.51 |
| Seed 23, first mapping, assignment | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | +0.01 | −0.13 | −0.11 | −0.77 | −0.79 | −0.63 | −0.59 |
| Seed 23, first mapping, assignment | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.15 | −0.08 | −1.16 | −1.15 | −0.88 | −0.86 |
| Seed 23, first mapping, assignment | blocks 13 to 18 | After block 12 alone | 0.00 | 0.00 | −0.03 | −0.06 | −1.26 | −1.20 | −0.98 | −0.93 |
| Seed 23, first mapping, assignment | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.05 | −0.86 | −0.89 | −0.76 | −0.66 |
| Seed 23, first mapping, assignment | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.05 | −0.07 | −1.31 | −1.29 | −1.00 | −0.98 |
| Seed 23, first mapping, assignment | blocks 19 to 24 | After block 12 alone | 0.00 | 0.00 | −0.03 | −0.01 | −1.14 | −1.10 | −0.96 | −0.82 |
| Seed 23, first mapping, assignment | blocks 19 to 24 | After block 18 alone | 0.00 | 0.00 | 0.00 | −0.02 | −1.35 | −1.27 | −1.06 | −0.96 |
| Seed 23, first mapping, assignment | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | 0.00 | −0.02 | −0.95 | −0.99 | −0.84 | −0.75 |
| Seed 23, first mapping, assignment | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.01 | −1.43 | −1.36 | −1.08 | −1.04 |
| Seed 23, first mapping, assignment | blocks 25 to 36 | After block 12 alone | 0.00 | 0.00 | 0.00 | −0.03 | −2.34 | −2.42 | −1.92 | −1.86 |
| Seed 23, first mapping, assignment | blocks 25 to 36 | After block 18 alone | 0.00 | 0.00 | −0.02 | −0.03 | −2.46 | −2.42 | −1.97 | −1.90 |
| Seed 23, first mapping, assignment | blocks 25 to 36 | After block 24 alone | 0.00 | 0.00 | 0.00 | −0.04 | −2.72 | −2.59 | −2.14 | −2.02 |
| Seed 23, first mapping, assignment | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.03 | −2.01 | −2.13 | −1.64 | −1.78 |
| Seed 23, first mapping, assignment | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.04 | −0.04 | −2.88 | −2.69 | −2.14 | −2.12 |
| Seed 24, eighth, mirrored | blocks 1 to 6 | Every form | 0.00 | 0.00 | −0.24 | −0.13 | +0.24 | +0.33 | +0.39 | +0.35 |
| Seed 24, eighth, mirrored | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.04 | −0.03 | +0.12 | +0.03 | +0.07 | +0.17 |
| Seed 24, eighth, mirrored | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.05 | −0.02 | −0.87 | −0.86 | −0.76 | −0.47 |
| Seed 24, eighth, mirrored | blocks 13 to 18 | After block 12 alone | 0.00 | 0.00 | −0.04 | −0.01 | −0.87 | −0.85 | −0.76 | −0.46 |
| Seed 24, eighth, mirrored | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.02 | +0.04 | +0.04 | +0.02 | +0.17 |
| Seed 24, eighth, mirrored | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.02 | −0.92 | −0.94 | −0.78 | −0.56 |
| Seed 24, eighth, mirrored | blocks 19 to 24 | After block 12 alone | 0.00 | 0.00 | −0.02 | −0.01 | −0.47 | −0.45 | −0.52 | −0.12 |
| Seed 24, eighth, mirrored | blocks 19 to 24 | After block 18 alone | 0.00 | 0.00 | −0.02 | −0.01 | −0.92 | −0.90 | −0.83 | −0.48 |
| Seed 24, eighth, mirrored | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.01 | +0.05 | +0.13 | +0.03 | +0.22 |
| Seed 24, eighth, mirrored | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.01 | −0.01 | −0.95 | −0.97 | −0.84 | −0.54 |
| Seed 24, eighth, mirrored | blocks 25 to 36 | After block 12 alone | 0.00 | 0.00 | −0.03 | −0.04 | −0.49 | −0.55 | −0.61 | −0.01 |
| Seed 24, eighth, mirrored | blocks 25 to 36 | After block 18 alone | 0.00 | 0.00 | −0.02 | −0.02 | −0.82 | −0.90 | −0.91 | −0.27 |
| Seed 24, eighth, mirrored | blocks 25 to 36 | After block 24 alone | 0.00 | 0.00 | −0.06 | −0.03 | −1.51 | −1.57 | −1.46 | −0.73 |
| Seed 24, eighth, mirrored | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.03 | −0.03 | +0.20 | +0.28 | +0.14 | +0.49 |
| Seed 24, eighth, mirrored | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.05 | −0.01 | −1.69 | −1.79 | −1.56 | −0.92 |
| Seed 24, first mapping, assignment | blocks 1 to 6 | Every form | 0.00 | −0.01 | −0.22 | −0.10 | −0.82 | −0.82 | −0.70 | −0.55 |
| Seed 24, first mapping, assignment | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.05 | −0.08 | −0.93 | −0.85 | −0.80 | −0.64 |
| Seed 24, first mapping, assignment | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.07 | −0.08 | −1.29 | −1.20 | −1.06 | −0.84 |
| Seed 24, first mapping, assignment | blocks 13 to 18 | After block 12 alone | 0.00 | 0.00 | −0.05 | −0.06 | −1.39 | −1.27 | −1.13 | −0.93 |
| Seed 24, first mapping, assignment | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.06 | −1.03 | −0.90 | −0.87 | −0.72 |
| Seed 24, first mapping, assignment | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.04 | −0.07 | −1.46 | −1.36 | −1.18 | −0.97 |
| Seed 24, first mapping, assignment | blocks 19 to 24 | After block 12 alone | 0.00 | 0.00 | −0.02 | −0.01 | −1.30 | −1.14 | −1.12 | −0.84 |
| Seed 24, first mapping, assignment | blocks 19 to 24 | After block 18 alone | 0.00 | 0.00 | −0.03 | −0.01 | −1.53 | −1.27 | −1.19 | −0.97 |
| Seed 24, first mapping, assignment | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.02 | −1.09 | −0.90 | −0.94 | −0.73 |
| Seed 24, first mapping, assignment | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.05 | 0.00 | −1.62 | −1.40 | −1.26 | −1.05 |
| Seed 24, first mapping, assignment | blocks 25 to 36 | After block 12 alone | 0.00 | 0.00 | −0.02 | −0.02 | −2.56 | −2.16 | −2.07 | −1.70 |
| Seed 24, first mapping, assignment | blocks 25 to 36 | After block 18 alone | 0.00 | 0.00 | −0.03 | −0.03 | −2.58 | −2.22 | −2.11 | −1.73 |
| Seed 24, first mapping, assignment | blocks 25 to 36 | After block 24 alone | 0.00 | 0.00 | −0.05 | −0.02 | −2.83 | −2.46 | −2.26 | −1.85 |
| Seed 24, first mapping, assignment | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | −0.02 | −0.02 | −2.17 | −1.83 | −1.84 | −1.46 |
| Seed 24, first mapping, assignment | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | −0.06 | −0.02 | −3.17 | −2.77 | −2.48 | −2.11 |
| Seed 23, eighth, assignment | blocks 1 to 6 | Every form | 0.00 | 0.00 | +0.09 | +0.10 | +0.26 | +0.29 | +0.24 | +0.51 |
| Seed 23, eighth, assignment | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | 0.00 | 0.00 | 0.00 | +0.07 | +0.05 | +0.11 | +0.14 |
| Seed 23, eighth, assignment | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | 0.00 | 0.00 | −0.79 | −0.84 | −0.64 | −0.51 |
| Seed 23, eighth, assignment | blocks 13 to 18 | After block 12 alone | 0.00 | 0.00 | 0.00 | 0.00 | −0.89 | −0.89 | −0.71 | −0.60 |
| Seed 23, eighth, assignment | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.02 | −0.01 | +0.07 | −0.01 | −0.01 | +0.17 |
| Seed 23, eighth, assignment | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | +0.01 | 0.00 | −0.89 | −0.95 | −0.71 | −0.63 |
| Seed 23, eighth, assignment | blocks 19 to 24 | After block 12 alone | 0.00 | 0.00 | +0.01 | 0.00 | −0.50 | −0.53 | −0.42 | −0.36 |
| Seed 23, eighth, assignment | blocks 19 to 24 | After block 18 alone | 0.00 | 0.00 | 0.00 | 0.00 | −0.91 | −0.90 | −0.74 | −0.63 |
| Seed 23, eighth, assignment | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.02 | 0.00 | +0.21 | +0.14 | +0.08 | +0.25 |
| Seed 23, eighth, assignment | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | +0.01 | 0.00 | −0.96 | −1.00 | −0.77 | −0.68 |
| Seed 23, eighth, assignment | blocks 25 to 36 | After block 12 alone | 0.00 | 0.00 | +0.04 | −0.01 | −0.57 | −0.65 | −0.46 | −0.35 |
| Seed 23, eighth, assignment | blocks 25 to 36 | After block 18 alone | 0.00 | 0.00 | +0.04 | −0.01 | −0.85 | −0.91 | −0.60 | −0.63 |
| Seed 23, eighth, assignment | blocks 25 to 36 | After block 24 alone | 0.00 | 0.00 | +0.03 | 0.00 | −1.49 | −1.53 | −1.15 | −1.04 |
| Seed 23, eighth, assignment | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.04 | −0.01 | +0.52 | +0.23 | +0.35 | +0.43 |
| Seed 23, eighth, assignment | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | +0.04 | 0.00 | −1.79 | −1.91 | −1.45 | −1.23 |
| Seed 24, first mapping, mirrored | blocks 1 to 6 | Every form | 0.00 | 0.00 | +0.14 | +0.07 | −0.90 | −0.89 | −0.77 | −0.67 |
| Seed 24, first mapping, mirrored | blocks 7 to 12 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.04 | +0.02 | −0.97 | −0.99 | −0.87 | −0.77 |
| Seed 24, first mapping, mirrored | blocks 7 to 12 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | +0.06 | +0.04 | −1.35 | −1.39 | −1.16 | −0.99 |
| Seed 24, first mapping, mirrored | blocks 13 to 18 | After block 12 alone | 0.00 | 0.00 | +0.03 | +0.01 | −1.43 | −1.59 | −1.25 | −1.13 |
| Seed 24, first mapping, mirrored | blocks 13 to 18 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.03 | +0.01 | −1.13 | −1.16 | −1.01 | −0.89 |
| Seed 24, first mapping, mirrored | blocks 13 to 18 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.49 | −1.59 | −1.26 | −1.14 |
| Seed 24, first mapping, mirrored | blocks 19 to 24 | After block 12 alone | 0.00 | 0.00 | +0.03 | +0.01 | −1.39 | −1.35 | −1.21 | −1.08 |
| Seed 24, first mapping, mirrored | blocks 19 to 24 | After block 18 alone | 0.00 | 0.00 | +0.01 | +0.01 | −1.52 | −1.54 | −1.25 | −1.17 |
| Seed 24, first mapping, mirrored | blocks 19 to 24 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.17 | −1.14 | −1.02 | −0.94 |
| Seed 24, first mapping, mirrored | blocks 19 to 24 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | +0.03 | +0.01 | −1.65 | −1.58 | −1.29 | −1.23 |
| Seed 24, first mapping, mirrored | blocks 25 to 36 | After block 12 alone | 0.00 | 0.00 | +0.03 | +0.01 | −2.65 | −2.41 | −2.13 | −2.05 |
| Seed 24, first mapping, mirrored | blocks 25 to 36 | After block 18 alone | 0.00 | 0.00 | +0.02 | +0.01 | −2.66 | −2.50 | −2.20 | −2.02 |
| Seed 24, first mapping, mirrored | blocks 25 to 36 | After block 24 alone | 0.00 | 0.00 | +0.02 | +0.01 | −2.75 | −2.76 | −2.22 | −2.17 |
| Seed 24, first mapping, mirrored | blocks 25 to 36 | Without replay (H-47, 021's job) | 0.00 | 0.00 | +0.01 | +0.01 | −2.13 | −2.05 | −1.80 | −1.73 |
| Seed 24, first mapping, mirrored | blocks 25 to 36 | Plain on A's (H-47, 021's job) | 0.00 | 0.00 | +0.03 | +0.05 | −3.08 | −3.02 | −2.39 | −2.33 |
| Seed 23, first mapping, mirrored | blocks 1 to 6 | Every form | 0.00 | 0.00 | +0.10 | +0.13 | −0.77 | −0.92 | −0.73 | −0.62 |
| Seed 23, first mapping, mirrored | blocks 7 to 12 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | +0.02 | −0.97 | −0.96 | −0.87 | −0.74 |
| Seed 23, first mapping, mirrored | blocks 7 to 12 | Plain on A's (H-47, 020's job) | 0.00 | −0.01 | +0.04 | 0.00 | −1.35 | −1.34 | −1.07 | −1.01 |
| Seed 23, first mapping, mirrored | blocks 13 to 18 | After block 12 alone | 0.00 | 0.00 | +0.02 | −0.01 | −1.48 | −1.37 | −1.16 | −1.08 |
| Seed 23, first mapping, mirrored | blocks 13 to 18 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.07 | −1.03 | −0.90 | −0.85 |
| Seed 23, first mapping, mirrored | blocks 13 to 18 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.05 | +0.01 | −1.51 | −1.44 | −1.16 | −1.13 |
| Seed 23, first mapping, mirrored | blocks 19 to 24 | After block 12 alone | 0.00 | 0.00 | +0.01 | 0.00 | −1.40 | −1.27 | −1.15 | −1.02 |
| Seed 23, first mapping, mirrored | blocks 19 to 24 | After block 18 alone | 0.00 | 0.00 | +0.01 | −0.01 | −1.56 | −1.47 | −1.24 | −1.15 |
| Seed 23, first mapping, mirrored | blocks 19 to 24 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.09 | −1.06 | −0.92 | −0.91 |
| Seed 23, first mapping, mirrored | blocks 19 to 24 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.64 | −1.54 | −1.24 | −1.20 |
| Seed 23, first mapping, mirrored | blocks 25 to 36 | After block 12 alone | 0.00 | 0.00 | +0.03 | 0.00 | −2.75 | −2.63 | −2.20 | −2.17 |
| Seed 23, first mapping, mirrored | blocks 25 to 36 | After block 18 alone | 0.00 | 0.00 | +0.02 | 0.00 | −2.81 | −2.63 | −2.19 | −2.19 |
| Seed 23, first mapping, mirrored | blocks 25 to 36 | After block 24 alone | 0.00 | −0.01 | +0.02 | +0.01 | −2.98 | −2.84 | −2.33 | −2.28 |
| Seed 23, first mapping, mirrored | blocks 25 to 36 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.03 | 0.00 | −2.28 | −2.39 | −1.93 | −1.94 |
| Seed 23, first mapping, mirrored | blocks 25 to 36 | Plain on A's (H-47, 020's job) | 0.00 | −0.01 | +0.02 | +0.03 | −3.31 | −3.10 | −2.52 | −2.46 |
| Seed 24, eighth, assignment | blocks 1 to 6 | Every form | 0.00 | 0.00 | +0.17 | +0.10 | +0.31 | +0.30 | +0.43 | +0.39 |
| Seed 24, eighth, assignment | blocks 7 to 12 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.04 | 0.00 | +0.24 | +0.10 | +0.17 | +0.28 |
| Seed 24, eighth, assignment | blocks 7 to 12 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.04 | 0.00 | −0.67 | −0.85 | −0.52 | −0.49 |
| Seed 24, eighth, assignment | blocks 13 to 18 | After block 12 alone | 0.00 | 0.00 | +0.02 | 0.00 | −0.75 | −0.83 | −0.60 | −0.51 |
| Seed 24, eighth, assignment | blocks 13 to 18 | Without replay (H-47, 020's job) | −0.01 | 0.00 | +0.03 | 0.00 | +0.16 | +0.09 | +0.09 | +0.25 |
| Seed 24, eighth, assignment | blocks 13 to 18 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.03 | +0.01 | −0.81 | −0.92 | −0.65 | −0.56 |
| Seed 24, eighth, assignment | blocks 19 to 24 | After block 12 alone | 0.00 | 0.00 | +0.02 | 0.00 | −0.36 | −0.42 | −0.33 | −0.18 |
| Seed 24, eighth, assignment | blocks 19 to 24 | After block 18 alone | 0.00 | 0.00 | +0.02 | 0.00 | −0.78 | −0.83 | −0.64 | −0.50 |
| Seed 24, eighth, assignment | blocks 19 to 24 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.04 | 0.00 | +0.21 | +0.20 | +0.15 | +0.31 |
| Seed 24, eighth, assignment | blocks 19 to 24 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −0.83 | −0.97 | −0.65 | −0.63 |
| Seed 24, eighth, assignment | blocks 25 to 36 | After block 12 alone | 0.00 | 0.00 | +0.05 | 0.00 | −0.43 | −0.47 | −0.35 | −0.17 |
| Seed 24, eighth, assignment | blocks 25 to 36 | After block 18 alone | 0.00 | 0.00 | +0.04 | +0.01 | −0.65 | −0.76 | −0.54 | −0.37 |
| Seed 24, eighth, assignment | blocks 25 to 36 | After block 24 alone | 0.00 | 0.00 | +0.04 | +0.01 | −1.35 | −1.43 | −1.05 | −0.93 |
| Seed 24, eighth, assignment | blocks 25 to 36 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.07 | +0.01 | +0.31 | +0.37 | +0.27 | +0.45 |
| Seed 24, eighth, assignment | blocks 25 to 36 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.05 | 0.00 | −1.53 | −1.79 | −1.17 | −1.14 |
| Seed 22, first mapping, mirrored | blocks 1 to 6 | Every form | 0.00 | 0.00 | +0.21 | +0.16 | −0.89 | −0.98 | −0.79 | −0.74 |
| Seed 22, first mapping, mirrored | blocks 7 to 12 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.09 | −0.01 | −0.83 | −1.13 | −0.86 | −0.86 |
| Seed 22, first mapping, mirrored | blocks 7 to 12 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.12 | 0.00 | −1.20 | −1.39 | −1.02 | −1.03 |
| Seed 22, first mapping, mirrored | blocks 13 to 18 | After block 12 alone | 0.00 | 0.00 | +0.05 | 0.00 | −1.32 | −1.53 | −1.14 | −1.19 |
| Seed 22, first mapping, mirrored | blocks 13 to 18 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −0.95 | −1.23 | −0.94 | −1.03 |
| Seed 22, first mapping, mirrored | blocks 13 to 18 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.07 | −0.02 | −1.38 | −1.57 | −1.14 | −1.20 |
| Seed 22, first mapping, mirrored | blocks 19 to 24 | After block 12 alone | 0.00 | 0.00 | +0.01 | 0.00 | −1.21 | −1.39 | −1.03 | −1.15 |
| Seed 22, first mapping, mirrored | blocks 19 to 24 | After block 18 alone | 0.00 | 0.00 | +0.03 | 0.00 | −1.39 | −1.58 | −1.15 | −1.25 |
| Seed 22, first mapping, mirrored | blocks 19 to 24 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.06 | −1.21 | −0.95 | −1.04 |
| Seed 22, first mapping, mirrored | blocks 19 to 24 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.02 | +0.01 | −1.43 | −1.60 | −1.17 | −1.21 |
| Seed 22, first mapping, mirrored | blocks 25 to 36 | After block 12 alone | 0.00 | 0.00 | +0.02 | +0.02 | −2.62 | −2.84 | −2.17 | −2.36 |
| Seed 22, first mapping, mirrored | blocks 25 to 36 | After block 18 alone | 0.00 | 0.00 | +0.01 | +0.01 | −2.63 | −2.75 | −2.14 | −2.33 |
| Seed 22, first mapping, mirrored | blocks 25 to 36 | After block 24 alone | 0.00 | 0.00 | +0.03 | +0.01 | −2.81 | −3.01 | −2.28 | −2.41 |
| Seed 22, first mapping, mirrored | blocks 25 to 36 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.01 | +0.01 | −2.33 | −2.35 | −1.95 | −2.07 |
| Seed 22, first mapping, mirrored | blocks 25 to 36 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.03 | +0.01 | −2.86 | −3.19 | −2.30 | −2.42 |
| Seed 22, eighth, assignment | blocks 1 to 6 | Every form | 0.00 | 0.00 | +0.23 | +0.16 | +0.22 | +0.33 | +0.30 | +0.46 |
| Seed 22, eighth, assignment | blocks 7 to 12 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | +0.01 | +0.07 | +0.05 | +0.09 | +0.10 |
| Seed 22, eighth, assignment | blocks 7 to 12 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.03 | 0.00 | −0.70 | −0.85 | −0.50 | −0.56 |
| Seed 22, eighth, assignment | blocks 13 to 18 | After block 12 alone | 0.00 | 0.00 | +0.01 | 0.00 | −0.74 | −0.86 | −0.57 | −0.59 |
| Seed 22, eighth, assignment | blocks 13 to 18 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.01 | 0.00 | +0.06 | +0.10 | +0.05 | +0.14 |
| Seed 22, eighth, assignment | blocks 13 to 18 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −0.77 | −0.87 | −0.55 | −0.59 |
| Seed 22, eighth, assignment | blocks 19 to 24 | After block 12 alone | 0.00 | 0.00 | +0.03 | 0.00 | −0.33 | −0.43 | −0.30 | −0.29 |
| Seed 22, eighth, assignment | blocks 19 to 24 | After block 18 alone | 0.00 | 0.00 | +0.02 | +0.01 | −0.75 | −0.86 | −0.58 | −0.64 |
| Seed 22, eighth, assignment | blocks 19 to 24 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.01 | +0.01 | +0.09 | +0.08 | +0.02 | +0.14 |
| Seed 22, eighth, assignment | blocks 19 to 24 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.01 | +0.01 | −0.76 | −0.93 | −0.56 | −0.67 |
| Seed 22, eighth, assignment | blocks 25 to 36 | After block 12 alone | 0.00 | 0.00 | +0.03 | +0.01 | −0.43 | −0.45 | −0.29 | −0.34 |
| Seed 22, eighth, assignment | blocks 25 to 36 | After block 18 alone | 0.00 | 0.00 | +0.04 | +0.01 | −0.68 | −0.80 | −0.52 | −0.58 |
| Seed 22, eighth, assignment | blocks 25 to 36 | After block 24 alone | 0.00 | 0.00 | +0.05 | +0.01 | −1.24 | −1.46 | −0.97 | −1.01 |
| Seed 22, eighth, assignment | blocks 25 to 36 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.03 | +0.01 | +0.10 | +0.17 | +0.10 | +0.27 |
| Seed 22, eighth, assignment | blocks 25 to 36 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.05 | +0.01 | −1.40 | −1.70 | −1.00 | −1.18 |

- **The blocks move no excitatory weight of the readouts' units onto all**: at most 0.01 M either way in any stretch listed, as
  explorations 030 and 031 read.
- **Over each stretch after a form's replayed night**, six a cell, each of the table's four inhibitory columns came out lower under the
  form than under H-47's night without replay in every stretch of every cell, all 288, by 0.07 to 2.15 M. The inhibition onto the
  excitatory units of A's group fell over each of those stretches under each form, by 0.15 to 2.41 M.

C's own units onto A's answer, the margin exploration 020 reads: their weight onto the readout A answers on the cell less onto the other.
Each night and each stretch, the mean over the three paths, in M; "the four nights" sums the nights and "blocks 7 to 36" the four stretches
after the first night, each sum of the unrounded figures, so the rounded figures beside it may add to 0.01 more or less:

| Cell | Form | blocks 1 to 6 | night after 6 | blocks 7 to 12 | night after 12 | blocks 13 to 18 | night after 18 | blocks 19 to 24 | night after 24 | blocks 25 to 36 | The four nights | Blocks 7 to 36 |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | After block 12 alone | −0.38 | −0.02 | −0.37 | +0.06 | −0.11 | +0.02 | −0.03 | +0.02 | −0.08 | +0.07 | −0.59 |
| Seed 22, eighth, mirrored | After block 18 alone | −0.38 | −0.02 | −0.37 | +0.01 | −0.08 | +0.13 | −0.04 | +0.01 | −0.07 | +0.13 | −0.56 |
| Seed 22, eighth, mirrored | After block 24 alone | −0.38 | −0.02 | −0.37 | +0.01 | −0.08 | +0.05 | −0.06 | +0.07 | −0.07 | +0.11 | −0.59 |
| Seed 22, eighth, mirrored | Without replay (H-47, 021's job) | −0.38 | −0.02 | −0.37 | +0.01 | −0.08 | +0.05 | −0.06 | +0.01 | −0.12 | +0.04 | −0.64 |
| Seed 22, eighth, mirrored | Plain on A's (H-47, 021's job) | −0.38 | +0.10 | −0.37 | +0.02 | −0.10 | +0.08 | −0.02 | +0.09 | −0.04 | +0.30 | −0.53 |
| Seed 22, first mapping, assignment | After block 12 alone | −0.52 | −0.06 | −0.24 | +0.10 | 0.00 | −0.02 | −0.01 | 0.00 | −0.07 | +0.02 | −0.31 |
| Seed 22, first mapping, assignment | After block 18 alone | −0.52 | −0.06 | −0.24 | 0.00 | −0.04 | +0.16 | −0.04 | +0.02 | 0.00 | +0.11 | −0.31 |
| Seed 22, first mapping, assignment | After block 24 alone | −0.52 | −0.06 | −0.24 | 0.00 | −0.04 | +0.01 | +0.02 | +0.16 | −0.05 | +0.11 | −0.31 |
| Seed 22, first mapping, assignment | Without replay (H-47, 021's job) | −0.52 | −0.06 | −0.24 | 0.00 | −0.04 | +0.01 | +0.02 | +0.01 | −0.03 | −0.05 | −0.28 |
| Seed 22, first mapping, assignment | Plain on A's (H-47, 021's job) | −0.52 | +0.14 | −0.30 | +0.10 | −0.03 | +0.11 | −0.07 | +0.07 | −0.09 | +0.42 | −0.49 |
| Seed 23, eighth, mirrored | After block 12 alone | −0.55 | −0.02 | −0.17 | +0.18 | −0.04 | −0.08 | −0.05 | −0.01 | −0.08 | +0.07 | −0.33 |
| Seed 23, eighth, mirrored | After block 18 alone | −0.55 | −0.02 | −0.17 | −0.01 | −0.05 | +0.13 | −0.01 | −0.06 | −0.09 | +0.03 | −0.32 |
| Seed 23, eighth, mirrored | After block 24 alone | −0.55 | −0.02 | −0.17 | −0.01 | −0.05 | 0.00 | −0.03 | +0.19 | −0.12 | +0.15 | −0.37 |
| Seed 23, eighth, mirrored | Without replay (H-47, 021's job) | −0.55 | −0.02 | −0.17 | −0.01 | −0.05 | 0.00 | −0.03 | −0.01 | −0.11 | −0.05 | −0.37 |
| Seed 23, eighth, mirrored | Plain on A's (H-47, 021's job) | −0.55 | +0.15 | −0.21 | +0.11 | −0.06 | +0.05 | −0.08 | +0.08 | −0.15 | +0.39 | −0.49 |
| Seed 23, first mapping, assignment | After block 12 alone | −0.48 | −0.05 | −0.44 | +0.14 | −0.11 | −0.05 | −0.09 | −0.02 | −0.01 | +0.02 | −0.65 |
| Seed 23, first mapping, assignment | After block 18 alone | −0.48 | −0.05 | −0.44 | −0.03 | −0.05 | +0.14 | −0.02 | −0.07 | −0.08 | −0.01 | −0.58 |
| Seed 23, first mapping, assignment | After block 24 alone | −0.48 | −0.05 | −0.44 | −0.03 | −0.05 | −0.03 | 0.00 | +0.17 | −0.01 | +0.06 | −0.50 |
| Seed 23, first mapping, assignment | Without replay (H-47, 021's job) | −0.48 | −0.05 | −0.44 | −0.03 | −0.05 | −0.03 | 0.00 | −0.02 | −0.05 | −0.13 | −0.53 |
| Seed 23, first mapping, assignment | Plain on A's (H-47, 021's job) | −0.48 | +0.12 | −0.50 | +0.17 | −0.15 | +0.10 | −0.07 | +0.11 | −0.16 | +0.51 | −0.88 |
| Seed 24, eighth, mirrored | After block 12 alone | −0.72 | −0.06 | −0.11 | +0.21 | −0.11 | −0.03 | −0.05 | +0.01 | −0.08 | +0.14 | −0.35 |
| Seed 24, eighth, mirrored | After block 18 alone | −0.72 | −0.06 | −0.11 | 0.00 | −0.07 | +0.23 | −0.07 | −0.03 | −0.05 | +0.15 | −0.30 |
| Seed 24, eighth, mirrored | After block 24 alone | −0.72 | −0.06 | −0.11 | 0.00 | −0.07 | +0.01 | −0.05 | +0.21 | −0.16 | +0.15 | −0.38 |
| Seed 24, eighth, mirrored | Without replay (H-47, 021's job) | −0.72 | −0.06 | −0.11 | 0.00 | −0.07 | +0.01 | −0.05 | +0.03 | −0.08 | −0.02 | −0.31 |
| Seed 24, eighth, mirrored | Plain on A's (H-47, 021's job) | −0.72 | +0.14 | −0.13 | +0.19 | −0.05 | +0.11 | −0.02 | +0.13 | −0.15 | +0.57 | −0.35 |
| Seed 24, first mapping, assignment | After block 12 alone | −0.66 | −0.08 | −0.17 | +0.21 | −0.15 | −0.02 | −0.06 | +0.01 | −0.06 | +0.12 | −0.44 |
| Seed 24, first mapping, assignment | After block 18 alone | −0.66 | −0.08 | −0.17 | 0.00 | −0.07 | +0.21 | −0.10 | 0.00 | −0.08 | +0.13 | −0.42 |
| Seed 24, first mapping, assignment | After block 24 alone | −0.66 | −0.08 | −0.17 | 0.00 | −0.07 | +0.01 | −0.05 | +0.29 | −0.14 | +0.22 | −0.42 |
| Seed 24, first mapping, assignment | Without replay (H-47, 021's job) | −0.66 | −0.08 | −0.17 | 0.00 | −0.07 | +0.01 | −0.05 | +0.04 | −0.03 | −0.03 | −0.32 |
| Seed 24, first mapping, assignment | Plain on A's (H-47, 021's job) | −0.66 | +0.16 | −0.20 | +0.17 | −0.13 | +0.18 | −0.13 | +0.18 | −0.15 | +0.68 | −0.61 |
| Seed 23, eighth, assignment | After block 12 alone | −0.45 | −0.09 | −0.37 | −0.09 | −0.08 | −0.05 | −0.06 | −0.05 | −0.12 | −0.27 | −0.63 |
| Seed 23, eighth, assignment | After block 18 alone | −0.45 | −0.09 | −0.37 | −0.03 | −0.10 | −0.05 | −0.05 | −0.04 | −0.13 | −0.21 | −0.65 |
| Seed 23, eighth, assignment | After block 24 alone | −0.45 | −0.09 | −0.37 | −0.03 | −0.10 | −0.03 | −0.05 | −0.11 | −0.15 | −0.26 | −0.66 |
| Seed 23, eighth, assignment | Without replay (H-47, 021's job) | −0.45 | −0.09 | −0.37 | −0.03 | −0.10 | −0.03 | −0.05 | −0.03 | −0.14 | −0.17 | −0.65 |
| Seed 23, eighth, assignment | Plain on A's (H-47, 021's job) | −0.45 | −0.11 | −0.30 | −0.05 | −0.09 | −0.07 | −0.07 | −0.09 | −0.12 | −0.32 | −0.57 |
| Seed 24, first mapping, mirrored | After block 12 alone | −0.46 | +0.03 | −0.20 | +0.03 | −0.11 | −0.02 | −0.06 | −0.02 | −0.09 | +0.03 | −0.46 |
| Seed 24, first mapping, mirrored | After block 18 alone | −0.46 | +0.03 | −0.20 | −0.02 | −0.11 | +0.02 | −0.03 | −0.02 | −0.06 | +0.02 | −0.41 |
| Seed 24, first mapping, mirrored | After block 24 alone | −0.46 | +0.03 | −0.20 | −0.02 | −0.11 | −0.06 | −0.05 | +0.04 | −0.06 | −0.01 | −0.42 |
| Seed 24, first mapping, mirrored | Without replay (H-47, 021's job) | −0.46 | +0.03 | −0.20 | −0.02 | −0.11 | −0.06 | −0.05 | −0.07 | −0.03 | −0.12 | −0.39 |
| Seed 24, first mapping, mirrored | Plain on A's (H-47, 021's job) | −0.46 | +0.03 | −0.25 | +0.05 | −0.09 | −0.01 | −0.08 | −0.02 | −0.07 | +0.05 | −0.48 |
| Seed 23, first mapping, mirrored | After block 12 alone | −0.54 | −0.04 | −0.36 | −0.09 | −0.15 | −0.05 | −0.06 | −0.03 | −0.12 | −0.22 | −0.69 |
| Seed 23, first mapping, mirrored | After block 18 alone | −0.54 | −0.04 | −0.36 | −0.04 | −0.13 | −0.08 | −0.09 | −0.04 | −0.10 | −0.21 | −0.68 |
| Seed 23, first mapping, mirrored | After block 24 alone | −0.54 | −0.04 | −0.36 | −0.04 | −0.13 | −0.05 | −0.07 | −0.07 | −0.11 | −0.19 | −0.68 |
| Seed 23, first mapping, mirrored | Without replay (H-47, 020's job) | −0.54 | −0.04 | −0.36 | −0.04 | −0.13 | −0.05 | −0.07 | −0.04 | −0.12 | −0.16 | −0.69 |
| Seed 23, first mapping, mirrored | Plain on A's (H-47, 020's job) | −0.54 | −0.14 | −0.39 | −0.11 | −0.19 | −0.04 | −0.09 | −0.05 | −0.10 | −0.34 | −0.77 |
| Seed 24, eighth, assignment | After block 12 alone | −0.48 | +0.01 | −0.13 | +0.04 | −0.04 | −0.01 | −0.06 | +0.02 | −0.12 | +0.06 | −0.36 |
| Seed 24, eighth, assignment | After block 18 alone | −0.48 | +0.01 | −0.13 | −0.02 | −0.08 | −0.01 | −0.06 | +0.01 | −0.11 | −0.01 | −0.38 |
| Seed 24, eighth, assignment | After block 24 alone | −0.48 | +0.01 | −0.13 | −0.02 | −0.08 | −0.02 | −0.10 | +0.03 | −0.10 | −0.01 | −0.41 |
| Seed 24, eighth, assignment | Without replay (H-47, 020's job) | −0.48 | +0.01 | −0.13 | −0.02 | −0.08 | −0.02 | −0.10 | +0.03 | −0.16 | 0.00 | −0.47 |
| Seed 24, eighth, assignment | Plain on A's (H-47, 020's job) | −0.48 | +0.07 | −0.12 | +0.07 | −0.06 | −0.04 | −0.05 | −0.01 | −0.12 | +0.09 | −0.35 |
| Seed 22, first mapping, mirrored | After block 12 alone | −0.60 | −0.06 | −0.23 | +0.09 | −0.11 | −0.06 | −0.02 | +0.01 | −0.04 | −0.01 | −0.39 |
| Seed 22, first mapping, mirrored | After block 18 alone | −0.60 | −0.06 | −0.23 | −0.05 | −0.06 | +0.08 | −0.05 | −0.01 | −0.02 | −0.03 | −0.36 |
| Seed 22, first mapping, mirrored | After block 24 alone | −0.60 | −0.06 | −0.23 | −0.05 | −0.06 | −0.02 | −0.04 | +0.15 | −0.06 | +0.03 | −0.38 |
| Seed 22, first mapping, mirrored | Without replay (H-47, 020's job) | −0.60 | −0.06 | −0.23 | −0.05 | −0.06 | −0.02 | −0.04 | +0.02 | −0.02 | −0.11 | −0.35 |
| Seed 22, first mapping, mirrored | Plain on A's (H-47, 020's job) | −0.60 | +0.06 | −0.32 | +0.12 | −0.15 | +0.02 | −0.03 | +0.01 | −0.07 | +0.21 | −0.57 |
| Seed 22, eighth, assignment | After block 12 alone | −0.61 | −0.05 | −0.04 | +0.08 | −0.02 | +0.01 | −0.07 | +0.03 | −0.08 | +0.07 | −0.21 |
| Seed 22, eighth, assignment | After block 18 alone | −0.61 | −0.05 | −0.04 | 0.00 | −0.03 | +0.07 | −0.05 | +0.02 | −0.10 | +0.04 | −0.22 |
| Seed 22, eighth, assignment | After block 24 alone | −0.61 | −0.05 | −0.04 | 0.00 | −0.03 | +0.01 | −0.03 | +0.10 | −0.10 | +0.06 | −0.20 |
| Seed 22, eighth, assignment | Without replay (H-47, 020's job) | −0.61 | −0.05 | −0.04 | 0.00 | −0.03 | +0.01 | −0.03 | +0.04 | −0.08 | 0.00 | −0.18 |
| Seed 22, eighth, assignment | Plain on A's (H-47, 020's job) | −0.61 | +0.05 | −0.07 | +0.12 | −0.06 | +0.05 | −0.02 | +0.06 | −0.12 | +0.28 | −0.27 |

D's own units onto B's answer under A less onto the other readout:

| Cell | Form | blocks 1 to 6 | night after 6 | blocks 7 to 12 | night after 12 | blocks 13 to 18 | night after 18 | blocks 19 to 24 | night after 24 | blocks 25 to 36 | The four nights | Blocks 7 to 36 |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | After block 12 alone | −0.61 | 0.00 | −0.32 | +0.23 | −0.04 | +0.03 | −0.09 | +0.02 | −0.08 | +0.28 | −0.53 |
| Seed 22, eighth, mirrored | After block 18 alone | −0.61 | 0.00 | −0.32 | +0.03 | −0.02 | +0.26 | −0.10 | +0.02 | −0.11 | +0.30 | −0.55 |
| Seed 22, eighth, mirrored | After block 24 alone | −0.61 | 0.00 | −0.32 | +0.03 | −0.02 | +0.02 | −0.03 | +0.20 | −0.12 | +0.24 | −0.48 |
| Seed 22, eighth, mirrored | Without replay (H-47, 021's job) | −0.61 | 0.00 | −0.32 | +0.03 | −0.02 | +0.02 | −0.03 | +0.01 | −0.09 | +0.05 | −0.46 |
| Seed 22, eighth, mirrored | Plain on A's (H-47, 021's job) | −0.61 | +0.16 | −0.32 | +0.16 | −0.06 | +0.12 | −0.06 | +0.16 | −0.15 | +0.60 | −0.58 |
| Seed 22, first mapping, assignment | After block 12 alone | −0.48 | 0.00 | −0.45 | +0.14 | −0.09 | 0.00 | −0.11 | +0.04 | −0.14 | +0.18 | −0.79 |
| Seed 22, first mapping, assignment | After block 18 alone | −0.48 | 0.00 | −0.45 | −0.01 | −0.06 | +0.23 | −0.08 | +0.04 | −0.19 | +0.25 | −0.77 |
| Seed 22, first mapping, assignment | After block 24 alone | −0.48 | 0.00 | −0.45 | −0.01 | −0.06 | +0.05 | −0.12 | +0.20 | −0.18 | +0.24 | −0.81 |
| Seed 22, first mapping, assignment | Without replay (H-47, 021's job) | −0.48 | 0.00 | −0.45 | −0.01 | −0.06 | +0.05 | −0.12 | +0.01 | −0.16 | +0.05 | −0.78 |
| Seed 22, first mapping, assignment | Plain on A's (H-47, 021's job) | −0.48 | +0.16 | −0.47 | +0.12 | −0.08 | +0.11 | −0.08 | +0.07 | −0.16 | +0.46 | −0.79 |
| Seed 23, eighth, mirrored | After block 12 alone | −0.59 | +0.01 | −0.27 | +0.18 | −0.06 | +0.02 | −0.10 | 0.00 | −0.15 | +0.21 | −0.58 |
| Seed 23, eighth, mirrored | After block 18 alone | −0.59 | +0.01 | −0.27 | +0.02 | −0.08 | +0.22 | −0.09 | +0.01 | −0.12 | +0.27 | −0.55 |
| Seed 23, eighth, mirrored | After block 24 alone | −0.59 | +0.01 | −0.27 | +0.02 | −0.08 | +0.02 | −0.10 | +0.18 | −0.11 | +0.24 | −0.56 |
| Seed 23, eighth, mirrored | Without replay (H-47, 021's job) | −0.59 | +0.01 | −0.27 | +0.02 | −0.08 | +0.02 | −0.10 | +0.03 | −0.09 | +0.09 | −0.54 |
| Seed 23, eighth, mirrored | Plain on A's (H-47, 021's job) | −0.59 | +0.20 | −0.21 | +0.18 | −0.10 | +0.09 | −0.06 | +0.12 | −0.10 | +0.60 | −0.46 |
| Seed 23, first mapping, assignment | After block 12 alone | −0.49 | +0.02 | −0.43 | +0.16 | −0.21 | −0.02 | −0.03 | 0.00 | −0.12 | +0.17 | −0.78 |
| Seed 23, first mapping, assignment | After block 18 alone | −0.49 | +0.02 | −0.43 | +0.01 | −0.15 | +0.13 | −0.08 | +0.01 | −0.11 | +0.17 | −0.77 |
| Seed 23, first mapping, assignment | After block 24 alone | −0.49 | +0.02 | −0.43 | +0.01 | −0.15 | +0.01 | −0.07 | +0.16 | −0.14 | +0.20 | −0.79 |
| Seed 23, first mapping, assignment | Without replay (H-47, 021's job) | −0.49 | +0.02 | −0.43 | +0.01 | −0.15 | +0.01 | −0.07 | +0.02 | −0.09 | +0.06 | −0.74 |
| Seed 23, first mapping, assignment | Plain on A's (H-47, 021's job) | −0.49 | +0.21 | −0.37 | +0.18 | −0.24 | +0.10 | −0.06 | +0.08 | −0.14 | +0.57 | −0.81 |
| Seed 24, eighth, mirrored | After block 12 alone | −0.53 | +0.02 | −0.12 | +0.11 | −0.04 | +0.01 | −0.03 | +0.02 | −0.13 | +0.16 | −0.33 |
| Seed 24, eighth, mirrored | After block 18 alone | −0.53 | +0.02 | −0.12 | +0.03 | −0.09 | +0.07 | −0.05 | +0.01 | −0.09 | +0.13 | −0.35 |
| Seed 24, eighth, mirrored | After block 24 alone | −0.53 | +0.02 | −0.12 | +0.03 | −0.09 | +0.03 | −0.04 | +0.11 | −0.11 | +0.19 | −0.36 |
| Seed 24, eighth, mirrored | Without replay (H-47, 021's job) | −0.53 | +0.02 | −0.12 | +0.03 | −0.09 | +0.03 | −0.04 | +0.01 | −0.10 | +0.09 | −0.36 |
| Seed 24, eighth, mirrored | Plain on A's (H-47, 021's job) | −0.53 | +0.16 | −0.09 | +0.09 | −0.06 | +0.04 | −0.04 | +0.01 | −0.05 | +0.30 | −0.24 |
| Seed 24, first mapping, assignment | After block 12 alone | −0.41 | +0.04 | −0.32 | +0.03 | −0.20 | −0.01 | −0.06 | −0.01 | −0.10 | +0.06 | −0.69 |
| Seed 24, first mapping, assignment | After block 18 alone | −0.41 | +0.04 | −0.32 | 0.00 | −0.22 | +0.11 | −0.05 | −0.01 | −0.11 | +0.14 | −0.71 |
| Seed 24, first mapping, assignment | After block 24 alone | −0.41 | +0.04 | −0.32 | 0.00 | −0.22 | +0.01 | −0.08 | +0.09 | −0.08 | +0.15 | −0.71 |
| Seed 24, first mapping, assignment | Without replay (H-47, 021's job) | −0.41 | +0.04 | −0.32 | 0.00 | −0.22 | +0.01 | −0.08 | +0.01 | −0.08 | +0.06 | −0.71 |
| Seed 24, first mapping, assignment | Plain on A's (H-47, 021's job) | −0.41 | +0.11 | −0.34 | +0.05 | −0.23 | +0.04 | −0.02 | +0.03 | −0.06 | +0.23 | −0.65 |
| Seed 23, eighth, assignment | After block 12 alone | −0.83 | +0.01 | −0.02 | +0.01 | +0.03 | +0.04 | +0.04 | −0.03 | +0.08 | +0.02 | +0.14 |
| Seed 23, eighth, assignment | After block 18 alone | −0.83 | +0.01 | −0.02 | +0.01 | +0.09 | +0.03 | +0.04 | −0.03 | +0.08 | +0.01 | +0.19 |
| Seed 23, eighth, assignment | After block 24 alone | −0.83 | +0.01 | −0.02 | +0.01 | +0.09 | +0.04 | +0.08 | −0.04 | +0.02 | +0.02 | +0.16 |
| Seed 23, eighth, assignment | Without replay (H-47, 021's job) | −0.83 | +0.01 | −0.02 | +0.01 | +0.09 | +0.04 | +0.08 | −0.05 | +0.09 | 0.00 | +0.24 |
| Seed 23, eighth, assignment | Plain on A's (H-47, 021's job) | −0.83 | −0.03 | −0.08 | −0.04 | +0.07 | +0.01 | +0.05 | −0.10 | +0.01 | −0.17 | +0.05 |
| Seed 24, first mapping, mirrored | After block 12 alone | −0.58 | −0.05 | −0.14 | −0.03 | −0.02 | −0.06 | −0.02 | −0.03 | −0.05 | −0.17 | −0.22 |
| Seed 24, first mapping, mirrored | After block 18 alone | −0.58 | −0.05 | −0.14 | −0.03 | +0.01 | −0.02 | −0.04 | −0.04 | −0.03 | −0.14 | −0.21 |
| Seed 24, first mapping, mirrored | After block 24 alone | −0.58 | −0.05 | −0.14 | −0.03 | +0.01 | −0.06 | −0.02 | −0.03 | −0.01 | −0.17 | −0.17 |
| Seed 24, first mapping, mirrored | Without replay (H-47, 021's job) | −0.58 | −0.05 | −0.14 | −0.03 | +0.01 | −0.06 | −0.02 | −0.05 | −0.05 | −0.19 | −0.20 |
| Seed 24, first mapping, mirrored | Plain on A's (H-47, 021's job) | −0.58 | −0.04 | −0.18 | −0.03 | 0.00 | +0.03 | −0.04 | −0.04 | −0.15 | −0.09 | −0.37 |
| Seed 23, first mapping, mirrored | After block 12 alone | −0.82 | +0.03 | −0.09 | −0.02 | +0.05 | −0.02 | −0.02 | +0.01 | 0.00 | 0.00 | −0.07 |
| Seed 23, first mapping, mirrored | After block 18 alone | −0.82 | +0.03 | −0.09 | +0.04 | +0.02 | −0.02 | +0.02 | −0.03 | 0.00 | +0.01 | −0.06 |
| Seed 23, first mapping, mirrored | After block 24 alone | −0.82 | +0.03 | −0.09 | +0.04 | +0.02 | 0.00 | −0.01 | −0.04 | −0.06 | +0.02 | −0.14 |
| Seed 23, first mapping, mirrored | Without replay (H-47, 020's job) | −0.82 | +0.03 | −0.09 | +0.04 | +0.02 | 0.00 | −0.01 | −0.04 | +0.03 | +0.02 | −0.06 |
| Seed 23, first mapping, mirrored | Plain on A's (H-47, 020's job) | −0.82 | −0.02 | −0.08 | −0.01 | −0.05 | −0.03 | −0.01 | 0.00 | −0.12 | −0.06 | −0.26 |
| Seed 24, eighth, assignment | After block 12 alone | −0.70 | −0.05 | −0.07 | −0.04 | −0.04 | −0.06 | 0.00 | −0.08 | −0.02 | −0.24 | −0.13 |
| Seed 24, eighth, assignment | After block 18 alone | −0.70 | −0.05 | −0.07 | −0.08 | −0.03 | +0.02 | −0.02 | −0.07 | −0.03 | −0.19 | −0.16 |
| Seed 24, eighth, assignment | After block 24 alone | −0.70 | −0.05 | −0.07 | −0.08 | −0.03 | −0.05 | +0.02 | −0.05 | −0.03 | −0.23 | −0.12 |
| Seed 24, eighth, assignment | Without replay (H-47, 020's job) | −0.70 | −0.05 | −0.07 | −0.08 | −0.03 | −0.05 | +0.02 | −0.06 | −0.03 | −0.25 | −0.12 |
| Seed 24, eighth, assignment | Plain on A's (H-47, 020's job) | −0.70 | −0.02 | −0.04 | −0.06 | −0.04 | −0.02 | 0.00 | −0.05 | 0.00 | −0.16 | −0.08 |
| Seed 22, first mapping, mirrored | After block 12 alone | −0.58 | −0.06 | −0.11 | −0.08 | −0.01 | −0.03 | −0.03 | +0.01 | −0.08 | −0.16 | −0.22 |
| Seed 22, first mapping, mirrored | After block 18 alone | −0.58 | −0.06 | −0.11 | −0.04 | −0.02 | −0.08 | −0.01 | −0.02 | −0.06 | −0.20 | −0.20 |
| Seed 22, first mapping, mirrored | After block 24 alone | −0.58 | −0.06 | −0.11 | −0.04 | −0.02 | −0.03 | −0.02 | −0.07 | −0.04 | −0.21 | −0.19 |
| Seed 22, first mapping, mirrored | Without replay (H-47, 020's job) | −0.58 | −0.06 | −0.11 | −0.04 | −0.02 | −0.03 | −0.02 | 0.00 | −0.06 | −0.13 | −0.20 |
| Seed 22, first mapping, mirrored | Plain on A's (H-47, 020's job) | −0.58 | −0.07 | −0.12 | −0.04 | +0.05 | −0.06 | −0.06 | −0.02 | −0.07 | −0.19 | −0.20 |
| Seed 22, eighth, assignment | After block 12 alone | −0.63 | −0.07 | −0.06 | −0.02 | −0.01 | −0.02 | −0.03 | −0.01 | −0.05 | −0.12 | −0.14 |
| Seed 22, eighth, assignment | After block 18 alone | −0.63 | −0.07 | −0.06 | −0.03 | −0.01 | −0.05 | −0.03 | −0.01 | −0.04 | −0.15 | −0.14 |
| Seed 22, eighth, assignment | After block 24 alone | −0.63 | −0.07 | −0.06 | −0.03 | −0.01 | −0.03 | −0.05 | −0.03 | −0.04 | −0.16 | −0.16 |
| Seed 22, eighth, assignment | Without replay (H-47, 020's job) | −0.63 | −0.07 | −0.06 | −0.03 | −0.01 | −0.03 | −0.05 | 0.00 | −0.04 | −0.13 | −0.16 |
| Seed 22, eighth, assignment | Plain on A's (H-47, 020's job) | −0.63 | −0.09 | −0.02 | −0.05 | 0.00 | −0.06 | −0.03 | −0.01 | −0.05 | −0.22 | −0.10 |

A's margin, the four kinds A alone, A and C, B alone and B and D, each onto the readout A asks of its stimulus less onto the other:

| Cell | Form | blocks 1 to 6 | night after 6 | blocks 7 to 12 | night after 12 | blocks 13 to 18 | night after 18 | blocks 19 to 24 | night after 24 | blocks 25 to 36 | The four nights | Blocks 7 to 36 |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | After block 12 alone | −2.16 | −0.12 | −0.75 | +1.34 | −0.17 | −0.11 | −0.10 | −0.10 | −0.16 | +1.00 | −1.18 |
| Seed 22, eighth, mirrored | After block 18 alone | −2.16 | −0.12 | −0.75 | −0.01 | −0.10 | +1.15 | −0.18 | −0.11 | −0.17 | +0.91 | −1.20 |
| Seed 22, eighth, mirrored | After block 24 alone | −2.16 | −0.12 | −0.75 | −0.01 | −0.10 | −0.03 | −0.09 | +1.32 | −0.21 | +1.16 | −1.14 |
| Seed 22, eighth, mirrored | Without replay (H-47, 021's job) | −2.16 | −0.12 | −0.75 | −0.01 | −0.10 | −0.03 | −0.09 | −0.07 | −0.24 | −0.23 | −1.17 |
| Seed 22, eighth, mirrored | Plain on A's (H-47, 021's job) | −2.16 | +1.12 | −0.84 | +1.08 | −0.15 | +0.78 | −0.09 | +0.61 | −0.16 | +3.60 | −1.25 |
| Seed 22, first mapping, assignment | After block 12 alone | −1.09 | +0.02 | −0.61 | +1.22 | −0.09 | −0.08 | −0.07 | −0.01 | −0.19 | +1.15 | −0.97 |
| Seed 22, first mapping, assignment | After block 18 alone | −1.09 | +0.02 | −0.61 | +0.06 | −0.11 | +1.03 | −0.08 | −0.06 | −0.14 | +1.05 | −0.94 |
| Seed 22, first mapping, assignment | After block 24 alone | −1.09 | +0.02 | −0.61 | +0.06 | −0.11 | −0.02 | −0.04 | +1.04 | −0.19 | +1.10 | −0.95 |
| Seed 22, first mapping, assignment | Without replay (H-47, 021's job) | −1.09 | +0.02 | −0.61 | +0.06 | −0.11 | −0.02 | −0.04 | −0.01 | −0.14 | +0.05 | −0.89 |
| Seed 22, first mapping, assignment | Plain on A's (H-47, 021's job) | −1.09 | +1.05 | −0.71 | +0.95 | −0.09 | +0.82 | −0.11 | +0.64 | −0.18 | +3.46 | −1.09 |
| Seed 23, eighth, mirrored | After block 12 alone | −2.80 | −0.10 | −0.60 | +1.16 | −0.14 | −0.16 | −0.17 | −0.05 | −0.24 | +0.85 | −1.14 |
| Seed 23, eighth, mirrored | After block 18 alone | −2.80 | −0.10 | −0.60 | −0.09 | −0.17 | +1.16 | −0.16 | −0.08 | −0.25 | +0.89 | −1.18 |
| Seed 23, eighth, mirrored | After block 24 alone | −2.80 | −0.10 | −0.60 | −0.09 | −0.17 | +0.04 | −0.19 | +1.29 | −0.31 | +1.14 | −1.27 |
| Seed 23, eighth, mirrored | Without replay (H-47, 021's job) | −2.80 | −0.10 | −0.60 | −0.09 | −0.17 | +0.04 | −0.19 | +0.05 | −0.27 | −0.11 | −1.23 |
| Seed 23, eighth, mirrored | Plain on A's (H-47, 021's job) | −2.80 | +1.38 | −0.66 | +1.03 | −0.24 | +0.79 | −0.19 | +0.66 | −0.29 | +3.86 | −1.38 |
| Seed 23, first mapping, assignment | After block 12 alone | −1.11 | 0.00 | −1.19 | +1.24 | −0.37 | −0.11 | −0.18 | −0.01 | −0.25 | +1.11 | −1.98 |
| Seed 23, first mapping, assignment | After block 18 alone | −1.11 | 0.00 | −1.19 | −0.12 | −0.31 | +1.25 | −0.16 | −0.07 | −0.30 | +1.06 | −1.96 |
| Seed 23, first mapping, assignment | After block 24 alone | −1.11 | 0.00 | −1.19 | −0.12 | −0.31 | −0.03 | −0.13 | +1.27 | −0.21 | +1.12 | −1.84 |
| Seed 23, first mapping, assignment | Without replay (H-47, 021's job) | −1.11 | 0.00 | −1.19 | −0.12 | −0.31 | −0.03 | −0.13 | 0.00 | −0.24 | −0.15 | −1.87 |
| Seed 23, first mapping, assignment | Plain on A's (H-47, 021's job) | −1.11 | +1.13 | −1.09 | +0.88 | −0.46 | +0.90 | −0.15 | +0.69 | −0.33 | +3.60 | −2.03 |
| Seed 24, eighth, mirrored | After block 12 alone | −3.00 | +0.07 | −0.37 | +1.41 | −0.27 | −0.02 | −0.18 | −0.06 | −0.35 | +1.39 | −1.18 |
| Seed 24, eighth, mirrored | After block 18 alone | −3.00 | +0.07 | −0.37 | +0.07 | −0.29 | +1.40 | −0.28 | −0.01 | −0.27 | +1.54 | −1.21 |
| Seed 24, eighth, mirrored | After block 24 alone | −3.00 | +0.07 | −0.37 | +0.07 | −0.29 | +0.08 | −0.19 | +1.43 | −0.55 | +1.66 | −1.40 |
| Seed 24, eighth, mirrored | Without replay (H-47, 021's job) | −3.00 | +0.07 | −0.37 | +0.07 | −0.29 | +0.08 | −0.19 | +0.04 | −0.37 | +0.27 | −1.22 |
| Seed 24, eighth, mirrored | Plain on A's (H-47, 021's job) | −3.00 | +1.28 | −0.44 | +1.17 | −0.21 | +1.12 | −0.18 | +0.76 | −0.28 | +4.33 | −1.11 |
| Seed 24, first mapping, assignment | After block 12 alone | −1.30 | +0.26 | −0.96 | +1.33 | −0.57 | +0.11 | −0.25 | +0.10 | −0.43 | +1.81 | −2.21 |
| Seed 24, first mapping, assignment | After block 18 alone | −1.30 | +0.26 | −0.96 | +0.22 | −0.57 | +1.18 | −0.25 | +0.15 | −0.46 | +1.82 | −2.23 |
| Seed 24, first mapping, assignment | After block 24 alone | −1.30 | +0.26 | −0.96 | +0.22 | −0.57 | +0.16 | −0.28 | +1.38 | −0.44 | +2.02 | −2.25 |
| Seed 24, first mapping, assignment | Without replay (H-47, 021's job) | −1.30 | +0.26 | −0.96 | +0.22 | −0.57 | +0.16 | −0.28 | +0.21 | −0.34 | +0.86 | −2.15 |
| Seed 24, first mapping, assignment | Plain on A's (H-47, 021's job) | −1.30 | +1.27 | −0.94 | +1.09 | −0.51 | +0.97 | −0.22 | +0.72 | −0.41 | +4.04 | −2.08 |
| Seed 23, eighth, assignment | After block 12 alone | −2.29 | +0.02 | −0.49 | +1.09 | −0.06 | −0.03 | −0.05 | −0.07 | −0.11 | +1.01 | −0.71 |
| Seed 23, eighth, assignment | After block 18 alone | −2.29 | +0.02 | −0.49 | 0.00 | −0.04 | +1.04 | −0.05 | −0.12 | −0.10 | +0.95 | −0.69 |
| Seed 23, eighth, assignment | After block 24 alone | −2.29 | +0.02 | −0.49 | 0.00 | −0.04 | −0.09 | −0.03 | +1.21 | −0.21 | +1.15 | −0.77 |
| Seed 23, eighth, assignment | Without replay (H-47, 021's job) | −2.29 | +0.02 | −0.49 | 0.00 | −0.04 | −0.09 | −0.03 | −0.08 | −0.09 | −0.14 | −0.65 |
| Seed 23, eighth, assignment | Plain on A's (H-47, 021's job) | −2.29 | +1.17 | −0.60 | +1.08 | −0.01 | +0.67 | −0.03 | +0.58 | −0.16 | +3.50 | −0.79 |
| Seed 24, first mapping, mirrored | After block 12 alone | −1.30 | +0.10 | −0.77 | +1.19 | −0.15 | +0.05 | −0.08 | +0.04 | −0.18 | +1.37 | −1.19 |
| Seed 24, first mapping, mirrored | After block 18 alone | −1.30 | +0.10 | −0.77 | +0.04 | −0.17 | +0.91 | −0.09 | +0.05 | −0.10 | +1.10 | −1.14 |
| Seed 24, first mapping, mirrored | After block 24 alone | −1.30 | +0.10 | −0.77 | +0.04 | −0.17 | +0.06 | −0.07 | +1.10 | −0.11 | +1.30 | −1.13 |
| Seed 24, first mapping, mirrored | Without replay (H-47, 021's job) | −1.30 | +0.10 | −0.77 | +0.04 | −0.17 | +0.06 | −0.07 | +0.04 | −0.14 | +0.24 | −1.16 |
| Seed 24, first mapping, mirrored | Plain on A's (H-47, 021's job) | −1.30 | +1.25 | −0.78 | +0.89 | −0.09 | +0.75 | −0.13 | +0.58 | −0.27 | +3.47 | −1.28 |
| Seed 23, first mapping, mirrored | After block 12 alone | −1.14 | +0.15 | −0.44 | +1.17 | −0.11 | +0.09 | −0.12 | +0.08 | −0.19 | +1.49 | −0.87 |
| Seed 23, first mapping, mirrored | After block 18 alone | −1.14 | +0.15 | −0.44 | +0.05 | −0.16 | +1.08 | −0.08 | +0.06 | −0.19 | +1.35 | −0.87 |
| Seed 23, first mapping, mirrored | After block 24 alone | −1.14 | +0.15 | −0.44 | +0.05 | −0.16 | +0.07 | −0.10 | +1.03 | −0.18 | +1.30 | −0.88 |
| Seed 23, first mapping, mirrored | Without replay (H-47, 020's job) | −1.14 | +0.15 | −0.44 | +0.05 | −0.16 | +0.07 | −0.10 | +0.10 | −0.16 | +0.37 | −0.85 |
| Seed 23, first mapping, mirrored | Plain on A's (H-47, 020's job) | −1.14 | +1.21 | −0.51 | +1.01 | −0.27 | +0.75 | −0.11 | +0.54 | −0.22 | +3.50 | −1.10 |
| Seed 24, eighth, assignment | After block 12 alone | −3.31 | −0.05 | −0.36 | +1.16 | −0.15 | −0.18 | −0.08 | −0.19 | −0.22 | +0.74 | −0.82 |
| Seed 24, eighth, assignment | After block 18 alone | −3.31 | −0.05 | −0.36 | −0.01 | −0.22 | +1.05 | −0.17 | −0.22 | −0.26 | +0.77 | −1.01 |
| Seed 24, eighth, assignment | After block 24 alone | −3.31 | −0.05 | −0.36 | −0.01 | −0.22 | −0.08 | −0.13 | +1.05 | −0.26 | +0.91 | −0.97 |
| Seed 24, eighth, assignment | Without replay (H-47, 020's job) | −3.31 | −0.05 | −0.36 | −0.01 | −0.22 | −0.08 | −0.13 | −0.08 | −0.40 | −0.22 | −1.11 |
| Seed 24, eighth, assignment | Plain on A's (H-47, 020's job) | −3.31 | +1.26 | −0.37 | +0.95 | −0.20 | +0.56 | −0.05 | +0.58 | −0.19 | +3.35 | −0.81 |
| Seed 22, first mapping, mirrored | After block 12 alone | −0.93 | +0.23 | −0.56 | +1.19 | −0.17 | +0.09 | −0.09 | +0.02 | −0.18 | +1.52 | −0.99 |
| Seed 22, first mapping, mirrored | After block 18 alone | −0.93 | +0.23 | −0.56 | +0.10 | −0.13 | +1.07 | −0.08 | −0.05 | −0.12 | +1.35 | −0.89 |
| Seed 22, first mapping, mirrored | After block 24 alone | −0.93 | +0.23 | −0.56 | +0.10 | −0.13 | +0.15 | −0.09 | +1.00 | −0.15 | +1.48 | −0.94 |
| Seed 22, first mapping, mirrored | Without replay (H-47, 020's job) | −0.93 | +0.23 | −0.56 | +0.10 | −0.13 | +0.15 | −0.09 | 0.00 | −0.12 | +0.48 | −0.91 |
| Seed 22, first mapping, mirrored | Plain on A's (H-47, 020's job) | −0.93 | +1.20 | −0.62 | +0.88 | −0.16 | +0.65 | −0.13 | +0.60 | −0.20 | +3.33 | −1.11 |
| Seed 22, eighth, assignment | After block 12 alone | −2.13 | +0.08 | −0.19 | +1.21 | −0.09 | −0.03 | −0.16 | +0.01 | −0.16 | +1.27 | −0.60 |
| Seed 22, eighth, assignment | After block 18 alone | −2.13 | +0.08 | −0.19 | +0.01 | −0.08 | +1.20 | −0.16 | +0.02 | −0.19 | +1.32 | −0.62 |
| Seed 22, eighth, assignment | After block 24 alone | −2.13 | +0.08 | −0.19 | +0.01 | −0.08 | +0.04 | −0.12 | +1.19 | −0.25 | +1.33 | −0.64 |
| Seed 22, eighth, assignment | Without replay (H-47, 020's job) | −2.13 | +0.08 | −0.19 | +0.01 | −0.08 | +0.04 | −0.12 | 0.00 | −0.18 | +0.13 | −0.57 |
| Seed 22, eighth, assignment | Plain on A's (H-47, 020's job) | −2.13 | +1.23 | −0.24 | +1.00 | −0.13 | +0.74 | −0.12 | +0.59 | −0.25 | +3.56 | −0.74 |

- **A's margin** rose by 1.09 to 1.41 M over after block 12 alone's replayed night, 0.91 to 1.40 M over after block 18 alone's and 1.00
  to 1.43 M over after block 24 alone's, where H-47's first plain night raised it by 1.05 to 1.38 M and its plain night after block 12, 18
  and 24 by 0.88 to 1.17 M, 0.56 to 1.12 M and 0.54 to 0.76 M. Over each night after a form's replayed night it moved by −0.22 to
  +0.15 M.
- **Over the four nights** A's margin rose by 0.74 to 1.81 M under after block 12 alone, 0.77 to 1.82 M under after block 18 alone and
  0.91 to 2.02 M under after block 24 alone, by 0.81 to 1.59 M under after block 6 alone (exploration 031's job) and 3.33 to 4.33 M under
  H-47's plain night, and moved by −0.23 to +0.86 M under its night without replay. Over blocks 7 to 36 it fell under the three forms in
  every cell, by 0.60 to 2.25 M.
- **C's own units' margin onto A's answer** moved over the four nights by −0.27 to +0.22 M under the three forms, by −0.34 to +0.68 M
  under H-47's plain night and by −0.17 to +0.04 M under its night without replay, and over each form's replayed night by −0.11 to
  +0.29 M. It fell over blocks 7 to 36 under the three forms in every cell, by 0.20 to 0.69 M.
- **D's own units** moved over the four nights by −0.24 to +0.30 M under the three forms, by −0.22 to +0.60 M under H-47's plain night and
  by −0.25 to +0.09 M under its night without replay.

## 6. The engine over each night, and what each replayed night held

Each night's change in the engine's excitatory and inhibitory sums and its gain, from the night's start to its end, the mean over the three
paths; its replays and depotentiations on each path; and the watch's wrong answers over the three paths. A form is listed at a night only
from its replayed night on. The forms and the night without replay are this exploration's jobs'; H-47's plain night is exploration 020's or
021's job's:

| Cell | Night after block | Form | This night | Excitatory sum, M | Inhibitory sum, M | Gain | Replays, paths 0, 5, 6 | Depotentiations | Watch: wrong |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | Without replay | without replay | −0.49 | +0.31 | +1 989 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −8.37 | +10.11 | −5 067 | 1 329, 1 329, 1 330 | 640, 640, 640 | 0 |
| Seed 22, eighth, mirrored | 12 | After block 12 alone | replays | −7.83 | +10.48 | −5 234 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 22, eighth, mirrored | 12 | Without replay | without replay | −0.14 | +0.31 | +1 980 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −5.67 | +8.07 | −4 483 | 1 268, 1 268, 1 268 | 601, 601, 601 | 0 |
| Seed 22, eighth, mirrored | 18 | After block 12 alone | without replay | +0.07 | −0.50 | +2 300 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, mirrored | 18 | After block 18 alone | replays | −7.04 | +10.20 | −5 168 | 1 341, 1 341, 1 340 | 640, 640, 640 | 0 |
| Seed 22, eighth, mirrored | 18 | Without replay | without replay | +0.07 | +0.30 | +2 312 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −3.55 | +6.90 | −3 966 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 22, eighth, mirrored | 24 | After block 12 alone | without replay | +0.43 | −0.35 | +2 174 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, mirrored | 24 | After block 18 alone | without replay | +0.37 | −0.53 | +2 120 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, mirrored | 24 | After block 24 alone | replays | −6.65 | +10.35 | −5 155 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 22, eighth, mirrored | 24 | Without replay | without replay | +0.37 | +0.39 | +2 172 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −2.20 | +5.89 | −3 481 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 22, first mapping, assignment | 6 | Without replay | without replay | −0.51 | −1.22 | +1 918 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −6.90 | +9.08 | −5 036 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| Seed 22, first mapping, assignment | 12 | After block 12 alone | replays | −6.14 | +8.65 | −5 167 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 22, first mapping, assignment | 12 | Without replay | without replay | +0.11 | −1.36 | +1 782 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −5.36 | +6.02 | −4 440 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| Seed 22, first mapping, assignment | 18 | After block 12 alone | without replay | +0.51 | −1.70 | +1 870 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, assignment | 18 | After block 18 alone | replays | −5.49 | +8.77 | −5 190 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 22, first mapping, assignment | 18 | Without replay | without replay | +0.45 | −1.46 | +1 902 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −3.43 | +3.98 | −3 974 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 22, first mapping, assignment | 24 | After block 12 alone | without replay | +0.41 | −1.57 | +2 060 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, assignment | 24 | After block 18 alone | without replay | +0.32 | −1.58 | +2 077 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, assignment | 24 | After block 24 alone | replays | −5.75 | +8.50 | −5 162 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 22, first mapping, assignment | 24 | Without replay | without replay | +0.34 | −1.33 | +2 140 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −2.62 | +2.60 | −3 388 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 23, eighth, mirrored | 6 | Without replay | without replay | −0.69 | +0.33 | +2 160 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −8.75 | +10.56 | −4 692 | 1 329, 1 329, 1 329 | 640, 640, 640 | 0 |
| Seed 23, eighth, mirrored | 12 | After block 12 alone | replays | −8.12 | +10.64 | −4 774 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 23, eighth, mirrored | 12 | Without replay | without replay | +0.12 | +0.37 | +2 237 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −5.70 | +8.49 | −4 188 | 1 268, 1 268, 1 268 | 601, 601, 601 | 0 |
| Seed 23, eighth, mirrored | 18 | After block 12 alone | without replay | +0.33 | −0.60 | +2 302 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, mirrored | 18 | After block 18 alone | replays | −7.21 | +10.83 | −4 843 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 23, eighth, mirrored | 18 | Without replay | without replay | +0.83 | +0.46 | +2 293 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −4.18 | +7.05 | −3 533 | 1 202, 1 202, 1 202 | 570, 571, 571 | 0 |
| Seed 23, eighth, mirrored | 24 | After block 12 alone | without replay | +1.06 | −0.37 | +2 280 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, mirrored | 24 | After block 18 alone | without replay | +1.19 | −0.53 | +2 256 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, mirrored | 24 | After block 24 alone | replays | −7.13 | +10.84 | −4 876 | 1 340, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 23, eighth, mirrored | 24 | Without replay | without replay | +1.49 | +0.63 | +2 360 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −2.89 | +5.90 | −3 111 | 1 128, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 23, first mapping, assignment | 6 | Without replay | without replay | −0.46 | −1.03 | +1 900 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −6.79 | +9.58 | −4 828 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| Seed 23, first mapping, assignment | 12 | After block 12 alone | replays | −5.99 | +9.38 | −4 824 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 23, first mapping, assignment | 12 | Without replay | without replay | +0.47 | −1.29 | +1 938 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −5.31 | +6.32 | −4 260 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| Seed 23, first mapping, assignment | 18 | After block 12 alone | without replay | +0.47 | −1.72 | +2 051 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, assignment | 18 | After block 18 alone | replays | −5.77 | +9.20 | −4 835 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 23, first mapping, assignment | 18 | Without replay | without replay | +0.76 | −1.41 | +2 107 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −3.20 | +3.72 | −3 830 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 23, first mapping, assignment | 24 | After block 12 alone | without replay | +0.84 | −1.70 | +2 023 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, assignment | 24 | After block 18 alone | without replay | +0.93 | −1.76 | +2 012 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, assignment | 24 | After block 24 alone | replays | −5.70 | +9.20 | −4 971 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 23, first mapping, assignment | 24 | Without replay | without replay | +1.16 | −1.42 | +2 118 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −2.37 | +2.60 | −3 224 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 24, eighth, mirrored | 6 | Without replay | without replay | −0.58 | +0.32 | +2 047 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −8.12 | +9.47 | −4 775 | 1 330, 1 330, 1 330 | 640, 640, 640 | 0 |
| Seed 24, eighth, mirrored | 12 | After block 12 alone | replays | −7.33 | +9.66 | −4 811 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 24, eighth, mirrored | 12 | Without replay | without replay | +0.18 | +0.26 | +2 236 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −5.64 | +7.82 | −4 295 | 1 268, 1 268, 1 268 | 601, 601, 601 | 0 |
| Seed 24, eighth, mirrored | 18 | After block 12 alone | without replay | +0.45 | −0.54 | +2 270 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, mirrored | 18 | After block 18 alone | replays | −7.03 | +9.83 | −4 837 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 24, eighth, mirrored | 18 | Without replay | without replay | +0.31 | +0.35 | +2 402 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −3.88 | +6.61 | −3 658 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 24, eighth, mirrored | 24 | After block 12 alone | without replay | +0.65 | −0.15 | +2 305 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, mirrored | 24 | After block 18 alone | without replay | +0.59 | −0.44 | +2 277 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, mirrored | 24 | After block 24 alone | replays | −7.00 | +9.94 | −4 866 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 24, eighth, mirrored | 24 | Without replay | without replay | +0.38 | +0.48 | +2 393 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −2.71 | +5.85 | −3 246 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 24, first mapping, assignment | 6 | Without replay | without replay | +0.12 | −1.21 | +1 815 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −6.29 | +8.72 | −4 928 | 1 339, 1 339, 1 339 | 640, 640, 640 | 0 |
| Seed 24, first mapping, assignment | 12 | After block 12 alone | replays | −5.90 | +8.15 | −5 012 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 24, first mapping, assignment | 12 | Without replay | without replay | +0.45 | −1.33 | +1 959 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −5.07 | +5.92 | −4 403 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| Seed 24, first mapping, assignment | 18 | After block 12 alone | without replay | +0.53 | −1.63 | +2 024 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, assignment | 18 | After block 18 alone | replays | −5.61 | +8.36 | −5 117 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 24, first mapping, assignment | 18 | Without replay | without replay | +0.67 | −1.30 | +2 053 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −3.53 | +3.84 | −3 896 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 24, first mapping, assignment | 24 | After block 12 alone | without replay | +0.67 | −1.59 | +2 086 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, assignment | 24 | After block 18 alone | without replay | +0.84 | −1.67 | +2 064 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, assignment | 24 | After block 24 alone | replays | −5.87 | +8.06 | −5 011 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 24, first mapping, assignment | 24 | Without replay | without replay | +0.77 | −1.32 | +2 131 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −2.48 | +2.56 | −3 290 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 23, eighth, assignment | 6 | Without replay | without replay | −0.82 | +0.36 | +2 052 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, assignment | 6 | Plain on A's (H-47, 021's job) | replays | −9.49 | +9.72 | −4 179 | 1 329, 1 329, 1 329 | 640, 640, 640 | 0 |
| Seed 23, eighth, assignment | 12 | After block 12 alone | replays | −8.77 | +9.78 | −4 251 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 23, eighth, assignment | 12 | Without replay | without replay | +0.09 | +0.35 | +2 208 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, assignment | 12 | Plain on A's (H-47, 021's job) | replays | −6.46 | +7.84 | −3 699 | 1 268, 1 268, 1 268 | 601, 601, 601 | 0 |
| Seed 23, eighth, assignment | 18 | After block 12 alone | without replay | +0.35 | −0.63 | +2 329 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, assignment | 18 | After block 18 alone | replays | −8.04 | +9.95 | −4 275 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 23, eighth, assignment | 18 | Without replay | without replay | +0.65 | +0.45 | +2 334 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, assignment | 18 | Plain on A's (H-47, 021's job) | replays | −4.38 | +6.47 | −3 160 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 23, eighth, assignment | 24 | After block 12 alone | without replay | +0.78 | −0.29 | +2 323 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, assignment | 24 | After block 18 alone | without replay | +1.10 | −0.49 | +2 277 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, assignment | 24 | After block 24 alone | replays | −7.51 | +9.76 | −4 285 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 23, eighth, assignment | 24 | Without replay | without replay | +1.52 | +0.68 | +2 338 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, eighth, assignment | 24 | Plain on A's (H-47, 021's job) | replays | −2.94 | +5.54 | −2 788 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 24, first mapping, mirrored | 6 | Without replay | without replay | +0.04 | −1.28 | +1 853 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, mirrored | 6 | Plain on A's (H-47, 021's job) | replays | −7.93 | +7.94 | −4 304 | 1 339, 1 339, 1 339 | 640, 640, 640 | 0 |
| Seed 24, first mapping, mirrored | 12 | After block 12 alone | replays | −7.18 | +7.61 | −4 451 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 24, first mapping, mirrored | 12 | Without replay | without replay | +0.43 | −1.36 | +1 892 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, mirrored | 12 | Plain on A's (H-47, 021's job) | replays | −5.78 | +4.95 | −3 826 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| Seed 24, first mapping, mirrored | 18 | After block 12 alone | without replay | +0.59 | −1.68 | +1 951 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, mirrored | 18 | After block 18 alone | replays | −6.62 | +7.60 | −4 388 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 24, first mapping, mirrored | 18 | Without replay | without replay | +0.68 | −1.30 | +1 940 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, mirrored | 18 | Plain on A's (H-47, 021's job) | replays | −4.10 | +3.55 | −3 305 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 24, first mapping, mirrored | 24 | After block 12 alone | without replay | +0.49 | −1.65 | +2 042 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, mirrored | 24 | After block 18 alone | without replay | +0.65 | −1.68 | +2 056 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, mirrored | 24 | After block 24 alone | replays | −6.65 | +7.40 | −4 407 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 24, first mapping, mirrored | 24 | Without replay | without replay | +0.69 | −1.33 | +2 001 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, first mapping, mirrored | 24 | Plain on A's (H-47, 021's job) | replays | −3.14 | +2.67 | −2 879 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 23, first mapping, mirrored | 6 | Without replay | without replay | −0.48 | −1.19 | +1 991 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, mirrored | 6 | Plain on A's (H-47, 020's job) | replays | −7.72 | +8.85 | −4 327 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| Seed 23, first mapping, mirrored | 12 | After block 12 alone | replays | −7.14 | +8.49 | −4 333 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 23, first mapping, mirrored | 12 | Without replay | without replay | +0.37 | −1.40 | +1 972 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, mirrored | 12 | Plain on A's (H-47, 020's job) | replays | −5.84 | +5.81 | −3 789 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| Seed 23, first mapping, mirrored | 18 | After block 12 alone | without replay | +0.40 | −1.74 | +2 031 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, mirrored | 18 | After block 18 alone | replays | −6.58 | +8.49 | −4 385 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 23, first mapping, mirrored | 18 | Without replay | without replay | +0.64 | −1.43 | +1 978 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, mirrored | 18 | Plain on A's (H-47, 020's job) | replays | −3.93 | +3.79 | −3 392 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 23, first mapping, mirrored | 24 | After block 12 alone | without replay | +0.38 | −1.75 | +2 097 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, mirrored | 24 | After block 18 alone | without replay | +0.67 | −1.72 | +2 051 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, mirrored | 24 | After block 24 alone | replays | −6.21 | +8.22 | −4 391 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 23, first mapping, mirrored | 24 | Without replay | without replay | +1.02 | −1.33 | +1 991 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 23, first mapping, mirrored | 24 | Plain on A's (H-47, 020's job) | replays | −2.89 | +2.62 | −2 803 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 24, eighth, assignment | 6 | Without replay | without replay | −0.47 | +0.32 | +2 134 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, assignment | 6 | Plain on A's (H-47, 020's job) | replays | −9.36 | +8.80 | −4 075 | 1 329, 1 329, 1 329 | 640, 640, 640 | 0 |
| Seed 24, eighth, assignment | 12 | After block 12 alone | replays | −8.72 | +9.12 | −4 270 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 24, eighth, assignment | 12 | Without replay | without replay | +0.10 | +0.25 | +2 175 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, assignment | 12 | Plain on A's (H-47, 020's job) | replays | −6.68 | +7.16 | −3 721 | 1 268, 1 268, 1 268 | 601, 601, 601 | 0 |
| Seed 24, eighth, assignment | 18 | After block 12 alone | without replay | +0.49 | −0.54 | +2 290 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, assignment | 18 | After block 18 alone | replays | −8.03 | +8.80 | −4 016 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 24, eighth, assignment | 18 | Without replay | without replay | +0.43 | +0.32 | +2 448 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, assignment | 18 | Plain on A's (H-47, 020's job) | replays | −4.45 | +5.92 | −2 997 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 24, eighth, assignment | 24 | After block 12 alone | without replay | +0.60 | −0.22 | +2 328 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, assignment | 24 | After block 18 alone | without replay | +0.46 | −0.46 | +2 246 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, assignment | 24 | After block 24 alone | replays | −7.72 | +8.98 | −4 119 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 24, eighth, assignment | 24 | Without replay | without replay | +0.34 | +0.44 | +2 413 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 24, eighth, assignment | 24 | Plain on A's (H-47, 020's job) | replays | −3.38 | +5.16 | −2 768 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 22, first mapping, mirrored | 6 | Without replay | without replay | −0.44 | −1.32 | +1 995 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, mirrored | 6 | Plain on A's (H-47, 020's job) | replays | −7.55 | +7.36 | −4 331 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| Seed 22, first mapping, mirrored | 12 | After block 12 alone | replays | −7.04 | +7.15 | −4 452 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 22, first mapping, mirrored | 12 | Without replay | without replay | +0.09 | −1.39 | +1 792 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, mirrored | 12 | Plain on A's (H-47, 020's job) | replays | −5.75 | +4.75 | −3 872 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| Seed 22, first mapping, mirrored | 18 | After block 12 alone | without replay | +0.49 | −1.70 | +1 861 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, mirrored | 18 | After block 18 alone | replays | −6.50 | +7.05 | −4 450 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 22, first mapping, mirrored | 18 | Without replay | without replay | +0.40 | −1.39 | +1 884 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, mirrored | 18 | Plain on A's (H-47, 020's job) | replays | −3.85 | +3.21 | −3 515 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 22, first mapping, mirrored | 24 | After block 12 alone | without replay | +0.37 | −1.63 | +2 139 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, mirrored | 24 | After block 18 alone | without replay | +0.51 | −1.62 | +2 089 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, mirrored | 24 | After block 24 alone | replays | −6.76 | +7.15 | −4 469 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 22, first mapping, mirrored | 24 | Without replay | without replay | +0.37 | −1.40 | +2 141 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, first mapping, mirrored | 24 | Plain on A's (H-47, 020's job) | replays | −2.96 | +2.24 | −2 864 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |
| Seed 22, eighth, assignment | 6 | Without replay | without replay | −0.61 | +0.28 | +2 019 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, assignment | 6 | Plain on A's (H-47, 020's job) | replays | −9.17 | +8.35 | −4 301 | 1 329, 1 329, 1 330 | 640, 640, 640 | 0 |
| Seed 22, eighth, assignment | 12 | After block 12 alone | replays | −8.39 | +8.46 | −4 381 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 22, eighth, assignment | 12 | Without replay | without replay | −0.11 | +0.23 | +2 034 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, assignment | 12 | Plain on A's (H-47, 020's job) | replays | −6.30 | +6.67 | −3 912 | 1 268, 1 268, 1 268 | 601, 601, 601 | 0 |
| Seed 22, eighth, assignment | 18 | After block 12 alone | without replay | +0.16 | −0.48 | +2 281 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, assignment | 18 | After block 18 alone | replays | −8.31 | +8.41 | −4 309 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 22, eighth, assignment | 18 | Without replay | without replay | −0.06 | +0.24 | +2 419 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, assignment | 18 | Plain on A's (H-47, 020's job) | replays | −4.29 | +5.74 | −3 311 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Seed 22, eighth, assignment | 24 | After block 12 alone | without replay | +0.20 | −0.29 | +2 122 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, assignment | 24 | After block 18 alone | without replay | +0.32 | −0.59 | +2 110 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, assignment | 24 | After block 24 alone | replays | −7.84 | +8.39 | −4 462 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| Seed 22, eighth, assignment | 24 | Without replay | without replay | +0.23 | +0.28 | +2 133 | 0, 0, 0 | 0, 0, 0 | 0 |
| Seed 22, eighth, assignment | 24 | Plain on A's (H-47, 020's job) | replays | −3.19 | +4.92 | −2 813 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |

- **Each form's replayed night** lowered the excitatory sum by 5.49 to 8.77 M, raised the inhibitory sum by 7.05 to 10.84 M and lowered
  the gain by 4 016 to 5 234, with 1 340 to 1 341 replays and 640 depotentiations on each path. H-47's first plain night lowered the
  excitatory sum by 6.29 to 9.49 M, raised the inhibitory by 7.36 to 10.56 M and lowered the gain by 4 075 to 5 067, with 1 329 to 1 340
  replays a path; its plain nights after blocks 12, 18 and 24 read 1 268, 1 202 and 1 128 to 1 129 replays a path, lowered the excitatory
  sum by 2.20 to 6.68 M, raised the inhibitory by 2.24 to 8.49 M and lowered the gain by 2 768 to 4 483.
- **The nights after a form's replayed night** read `replays 0` and `depotentiations 0`, held inside the test, moved the excitatory sum by
  +0.07 to +1.19 M and the inhibitory by −1.76 to −0.15 M, and raised the gain by 1 861 to 2 329. In each of those 36 nights the
  inhibitory sum's change lay 0.23 to 1.17 M below the night without replay's after the same block on the same cell. The night without
  replay's nights moved the excitatory sum by −0.82 to +1.52 M and the inhibitory by −1.46 to +0.68 M, and raised the gain by 1 782 to
  2 448.
- **The watch read `wrong 0`** in each of the 576 nights of the twelve jobs, and in each of H-47's plain night's 144 nights and its night
  without replay's 144 in exploration 020's and 021's jobs.

Each replayed night as `ReplayWatch` reads it, beside H-47's first plain night on the same cell, its night after block 6 (exploration 020's
or 021's job, and on seed 23's first mapping from the mirrored assignment exploration 030's, where after block 6 alone's night after block 6
printed it). Spikes in a ripple that replays one of A's patterns, by the pattern's stimulus: the ripples are the total over the three paths,
and each other column a ripple's mean:

| Cell | Night after block | Form | Stimulus | Ripples | The first unit | The rest of the pattern | Pair's own in the answer's readout | Pair's own in the other readout | The answer's readout | The other readout | Another stimulus unit | Elsewhere |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 12 | After block 12 alone | A | 2 100 | 3.1 | 25.0 | 0.5 | 0.8 | 10.6 | 13.0 | 1.7 | 0.1 |
| Seed 22, eighth, mirrored | 12 | After block 12 alone | B | 1 923 | 3.0 | 23.4 | 0.4 | 0.7 | 9.9 | 12.9 | 1.9 | 0.1 |
| Seed 22, eighth, mirrored | 18 | After block 18 alone | A | 2 099 | 3.1 | 24.9 | 0.4 | 0.8 | 10.6 | 12.8 | 1.8 | 0.1 |
| Seed 22, eighth, mirrored | 18 | After block 18 alone | B | 1 923 | 3.0 | 23.4 | 0.4 | 0.7 | 10.0 | 12.8 | 1.9 | 0.1 |
| Seed 22, eighth, mirrored | 24 | After block 24 alone | A | 2 100 | 3.0 | 24.9 | 0.5 | 0.8 | 11.0 | 13.1 | 1.8 | 0.1 |
| Seed 22, eighth, mirrored | 24 | After block 24 alone | B | 1 923 | 3.0 | 23.3 | 0.4 | 0.8 | 9.7 | 12.8 | 2.0 | 0.1 |
| Seed 22, eighth, mirrored | 6 | H-47's first plain night (021's job) | A | 2 048 | 3.1 | 25.2 | 0.5 | 0.7 | 10.7 | 12.8 | 1.7 | 0.1 |
| Seed 22, eighth, mirrored | 6 | H-47's first plain night (021's job) | B | 1 940 | 3.0 | 23.6 | 0.4 | 0.8 | 9.7 | 12.6 | 1.8 | 0.1 |
| Seed 22, first mapping, assignment | 12 | After block 12 alone | A | 2 100 | 3.1 | 25.0 | 0.4 | 0.7 | 10.5 | 12.0 | 1.9 | 0.1 |
| Seed 22, first mapping, assignment | 12 | After block 12 alone | B | 1 923 | 3.0 | 23.3 | 0.4 | 0.8 | 9.9 | 12.2 | 2.1 | 0.1 |
| Seed 22, first mapping, assignment | 18 | After block 18 alone | A | 2 100 | 3.1 | 24.8 | 0.4 | 0.8 | 10.8 | 12.3 | 1.9 | 0.1 |
| Seed 22, first mapping, assignment | 18 | After block 18 alone | B | 1 923 | 3.0 | 23.3 | 0.4 | 0.8 | 9.8 | 12.2 | 2.0 | 0.1 |
| Seed 22, first mapping, assignment | 24 | After block 24 alone | A | 2 100 | 3.1 | 24.8 | 0.5 | 0.8 | 10.7 | 12.2 | 1.9 | 0.1 |
| Seed 22, first mapping, assignment | 24 | After block 24 alone | B | 1 923 | 3.0 | 23.2 | 0.4 | 0.8 | 10.0 | 12.3 | 2.1 | 0.1 |
| Seed 22, first mapping, assignment | 6 | H-47's first plain night (021's job) | A | 2 097 | 3.1 | 25.0 | 0.5 | 0.7 | 10.5 | 12.2 | 2.0 | 0.1 |
| Seed 22, first mapping, assignment | 6 | H-47's first plain night (021's job) | B | 1 921 | 3.0 | 23.4 | 0.4 | 0.8 | 9.8 | 12.0 | 2.1 | 0.1 |
| Seed 23, eighth, mirrored | 12 | After block 12 alone | A | 2 100 | 3.0 | 25.4 | 0.5 | 0.6 | 10.5 | 12.9 | 1.8 | 0.1 |
| Seed 23, eighth, mirrored | 12 | After block 12 alone | B | 1 923 | 3.0 | 21.8 | 0.3 | 0.5 | 9.9 | 13.4 | 1.7 | 0.1 |
| Seed 23, eighth, mirrored | 18 | After block 18 alone | A | 2 100 | 3.0 | 25.2 | 0.4 | 0.6 | 10.7 | 13.0 | 1.8 | 0.1 |
| Seed 23, eighth, mirrored | 18 | After block 18 alone | B | 1 923 | 3.0 | 21.6 | 0.3 | 0.5 | 10.2 | 13.3 | 1.7 | 0.1 |
| Seed 23, eighth, mirrored | 24 | After block 24 alone | A | 2 099 | 3.0 | 25.1 | 0.5 | 0.5 | 10.9 | 13.3 | 1.8 | 0.1 |
| Seed 23, eighth, mirrored | 24 | After block 24 alone | B | 1 923 | 3.0 | 21.5 | 0.3 | 0.5 | 10.0 | 13.4 | 1.8 | 0.1 |
| Seed 23, eighth, mirrored | 6 | H-47's first plain night (021's job) | A | 2 046 | 3.0 | 25.5 | 0.5 | 0.6 | 10.4 | 12.8 | 1.7 | 0.1 |
| Seed 23, eighth, mirrored | 6 | H-47's first plain night (021's job) | B | 1 941 | 3.0 | 21.9 | 0.4 | 0.5 | 10.0 | 12.9 | 1.6 | 0.1 |
| Seed 23, first mapping, assignment | 12 | After block 12 alone | A | 2 100 | 3.0 | 25.3 | 0.5 | 0.6 | 10.4 | 12.3 | 1.9 | 0.1 |
| Seed 23, first mapping, assignment | 12 | After block 12 alone | B | 1 923 | 3.0 | 21.7 | 0.4 | 0.5 | 10.0 | 12.4 | 1.9 | 0.1 |
| Seed 23, first mapping, assignment | 18 | After block 18 alone | A | 2 100 | 3.0 | 25.2 | 0.5 | 0.5 | 10.7 | 12.6 | 1.9 | 0.1 |
| Seed 23, first mapping, assignment | 18 | After block 18 alone | B | 1 923 | 3.0 | 21.7 | 0.3 | 0.5 | 9.9 | 12.5 | 1.9 | 0.1 |
| Seed 23, first mapping, assignment | 24 | After block 24 alone | A | 2 100 | 3.0 | 25.1 | 0.5 | 0.6 | 10.7 | 12.6 | 1.8 | 0.1 |
| Seed 23, first mapping, assignment | 24 | After block 24 alone | B | 1 923 | 2.9 | 21.5 | 0.3 | 0.4 | 10.2 | 13.0 | 1.9 | 0.1 |
| Seed 23, first mapping, assignment | 6 | H-47's first plain night (021's job) | A | 2 097 | 3.0 | 25.3 | 0.5 | 0.6 | 10.6 | 12.2 | 2.0 | 0.1 |
| Seed 23, first mapping, assignment | 6 | H-47's first plain night (021's job) | B | 1 921 | 3.0 | 21.8 | 0.4 | 0.5 | 9.7 | 12.1 | 2.0 | 0.1 |
| Seed 24, eighth, mirrored | 12 | After block 12 alone | A | 2 100 | 3.0 | 24.8 | 0.5 | 0.5 | 10.5 | 13.3 | 1.7 | 0.1 |
| Seed 24, eighth, mirrored | 12 | After block 12 alone | B | 1 923 | 3.0 | 23.1 | 0.3 | 0.7 | 10.4 | 12.3 | 1.7 | 0.2 |
| Seed 24, eighth, mirrored | 18 | After block 18 alone | A | 2 100 | 3.0 | 24.7 | 0.4 | 0.5 | 10.5 | 13.4 | 1.9 | 0.2 |
| Seed 24, eighth, mirrored | 18 | After block 18 alone | B | 1 923 | 3.0 | 22.9 | 0.3 | 0.7 | 10.6 | 12.5 | 1.7 | 0.2 |
| Seed 24, eighth, mirrored | 24 | After block 24 alone | A | 2 100 | 3.0 | 24.6 | 0.5 | 0.5 | 10.6 | 13.6 | 1.8 | 0.2 |
| Seed 24, eighth, mirrored | 24 | After block 24 alone | B | 1 923 | 3.0 | 22.9 | 0.3 | 0.7 | 10.4 | 12.5 | 1.7 | 0.2 |
| Seed 24, eighth, mirrored | 6 | H-47's first plain night (021's job) | A | 2 052 | 3.0 | 24.9 | 0.4 | 0.5 | 10.4 | 13.4 | 1.8 | 0.1 |
| Seed 24, eighth, mirrored | 6 | H-47's first plain night (021's job) | B | 1 938 | 3.0 | 23.2 | 0.4 | 0.6 | 10.3 | 12.3 | 1.7 | 0.2 |
| Seed 24, first mapping, assignment | 12 | After block 12 alone | A | 2 100 | 3.0 | 24.7 | 0.4 | 0.4 | 10.4 | 12.6 | 2.0 | 0.1 |
| Seed 24, first mapping, assignment | 12 | After block 12 alone | B | 1 923 | 3.0 | 23.1 | 0.4 | 0.7 | 10.4 | 11.9 | 1.9 | 0.2 |
| Seed 24, first mapping, assignment | 18 | After block 18 alone | A | 2 100 | 3.0 | 24.6 | 0.5 | 0.4 | 10.5 | 13.0 | 2.0 | 0.2 |
| Seed 24, first mapping, assignment | 18 | After block 18 alone | B | 1 923 | 3.0 | 23.0 | 0.3 | 0.7 | 10.5 | 12.2 | 1.9 | 0.2 |
| Seed 24, first mapping, assignment | 24 | After block 24 alone | A | 2 100 | 3.0 | 24.5 | 0.5 | 0.4 | 10.6 | 13.2 | 2.1 | 0.2 |
| Seed 24, first mapping, assignment | 24 | After block 24 alone | B | 1 923 | 3.0 | 22.9 | 0.4 | 0.6 | 10.5 | 12.3 | 1.9 | 0.2 |
| Seed 24, first mapping, assignment | 6 | H-47's first plain night (021's job) | A | 2 097 | 3.1 | 24.7 | 0.5 | 0.5 | 10.6 | 12.7 | 2.0 | 0.1 |
| Seed 24, first mapping, assignment | 6 | H-47's first plain night (021's job) | B | 1 920 | 3.0 | 23.1 | 0.4 | 0.7 | 10.2 | 11.6 | 2.0 | 0.2 |
| Seed 23, eighth, assignment | 12 | After block 12 alone | A | 2 100 | 3.0 | 22.0 | 0.4 | 0.6 | 10.9 | 13.7 | 1.8 | 0.1 |
| Seed 23, eighth, assignment | 12 | After block 12 alone | B | 1 923 | 3.0 | 22.3 | 0.4 | 0.6 | 10.8 | 12.4 | 1.8 | 0.1 |
| Seed 23, eighth, assignment | 18 | After block 18 alone | A | 2 100 | 3.0 | 21.8 | 0.4 | 0.5 | 10.9 | 13.8 | 1.8 | 0.1 |
| Seed 23, eighth, assignment | 18 | After block 18 alone | B | 1 923 | 3.0 | 22.2 | 0.4 | 0.6 | 11.0 | 12.5 | 1.8 | 0.1 |
| Seed 23, eighth, assignment | 24 | After block 24 alone | A | 2 100 | 3.0 | 21.8 | 0.4 | 0.6 | 11.0 | 14.2 | 1.8 | 0.1 |
| Seed 23, eighth, assignment | 24 | After block 24 alone | B | 1 923 | 3.0 | 22.1 | 0.4 | 0.6 | 11.0 | 12.1 | 1.8 | 0.1 |
| Seed 23, eighth, assignment | 6 | H-47's first plain night (021's job) | A | 2 046 | 3.1 | 22.0 | 0.4 | 0.6 | 10.8 | 13.5 | 1.8 | 0.1 |
| Seed 23, eighth, assignment | 6 | H-47's first plain night (021's job) | B | 1 941 | 3.0 | 22.4 | 0.5 | 0.6 | 10.7 | 12.5 | 1.7 | 0.1 |
| Seed 24, first mapping, mirrored | 12 | After block 12 alone | A | 2 100 | 3.0 | 22.1 | 0.4 | 0.6 | 11.1 | 12.8 | 1.9 | 0.2 |
| Seed 24, first mapping, mirrored | 12 | After block 12 alone | B | 1 923 | 3.0 | 21.8 | 0.4 | 0.4 | 10.8 | 12.3 | 2.1 | 0.1 |
| Seed 24, first mapping, mirrored | 18 | After block 18 alone | A | 2 100 | 3.0 | 22.0 | 0.3 | 0.7 | 11.4 | 12.9 | 2.0 | 0.2 |
| Seed 24, first mapping, mirrored | 18 | After block 18 alone | B | 1 923 | 3.0 | 21.7 | 0.4 | 0.4 | 10.9 | 12.2 | 2.1 | 0.2 |
| Seed 24, first mapping, mirrored | 24 | After block 24 alone | A | 2 100 | 3.0 | 22.0 | 0.4 | 0.6 | 11.5 | 12.9 | 1.9 | 0.2 |
| Seed 24, first mapping, mirrored | 24 | After block 24 alone | B | 1 923 | 3.0 | 21.7 | 0.4 | 0.4 | 11.0 | 12.5 | 2.1 | 0.2 |
| Seed 24, first mapping, mirrored | 6 | H-47's first plain night (021's job) | A | 2 097 | 3.1 | 22.2 | 0.4 | 0.7 | 11.0 | 12.9 | 2.0 | 0.1 |
| Seed 24, first mapping, mirrored | 6 | H-47's first plain night (021's job) | B | 1 920 | 3.1 | 21.8 | 0.5 | 0.5 | 10.7 | 12.0 | 2.1 | 0.1 |
| Seed 23, first mapping, mirrored | 12 | After block 12 alone | A | 2 100 | 3.0 | 21.9 | 0.4 | 0.6 | 10.6 | 12.7 | 1.9 | 0.1 |
| Seed 23, first mapping, mirrored | 12 | After block 12 alone | B | 1 923 | 3.0 | 22.2 | 0.4 | 0.6 | 11.1 | 12.0 | 2.0 | 0.1 |
| Seed 23, first mapping, mirrored | 18 | After block 18 alone | A | 2 100 | 3.1 | 21.7 | 0.3 | 0.6 | 10.9 | 13.2 | 1.9 | 0.1 |
| Seed 23, first mapping, mirrored | 18 | After block 18 alone | B | 1 923 | 3.0 | 22.1 | 0.5 | 0.5 | 10.9 | 11.9 | 2.0 | 0.1 |
| Seed 23, first mapping, mirrored | 24 | After block 24 alone | A | 2 100 | 3.0 | 21.8 | 0.3 | 0.5 | 10.9 | 13.1 | 1.9 | 0.1 |
| Seed 23, first mapping, mirrored | 24 | After block 24 alone | B | 1 923 | 3.0 | 22.1 | 0.4 | 0.5 | 11.0 | 11.8 | 2.0 | 0.1 |
| Seed 23, first mapping, mirrored | 6 | H-47's first plain night (030's job) | A | 2 097 | 3.1 | 21.8 | 0.4 | 0.7 | 10.7 | 12.9 | 1.9 | 0.1 |
| Seed 23, first mapping, mirrored | 6 | H-47's first plain night (030's job) | B | 1 921 | 3.1 | 22.2 | 0.4 | 0.6 | 10.9 | 11.8 | 2.1 | 0.1 |
| Seed 24, eighth, assignment | 12 | After block 12 alone | A | 2 100 | 3.1 | 22.2 | 0.4 | 0.6 | 11.2 | 13.5 | 1.7 | 0.1 |
| Seed 24, eighth, assignment | 12 | After block 12 alone | B | 1 923 | 3.0 | 21.9 | 0.4 | 0.5 | 10.8 | 12.7 | 1.9 | 0.1 |
| Seed 24, eighth, assignment | 18 | After block 18 alone | A | 2 100 | 3.1 | 22.1 | 0.4 | 0.6 | 10.9 | 13.4 | 1.8 | 0.1 |
| Seed 24, eighth, assignment | 18 | After block 18 alone | B | 1 923 | 3.0 | 21.8 | 0.3 | 0.5 | 10.9 | 12.9 | 1.8 | 0.2 |
| Seed 24, eighth, assignment | 24 | After block 24 alone | A | 2 100 | 3.0 | 22.0 | 0.4 | 0.6 | 11.2 | 13.7 | 1.8 | 0.2 |
| Seed 24, eighth, assignment | 24 | After block 24 alone | B | 1 923 | 3.0 | 21.8 | 0.4 | 0.4 | 10.8 | 12.7 | 1.9 | 0.2 |
| Seed 24, eighth, assignment | 6 | H-47's first plain night (020's job) | A | 2 046 | 3.1 | 22.3 | 0.4 | 0.7 | 11.0 | 13.2 | 1.8 | 0.1 |
| Seed 24, eighth, assignment | 6 | H-47's first plain night (020's job) | B | 1 941 | 3.0 | 22.0 | 0.4 | 0.5 | 10.7 | 12.8 | 1.8 | 0.1 |
| Seed 22, first mapping, mirrored | 12 | After block 12 alone | A | 2 100 | 3.1 | 21.3 | 0.5 | 0.7 | 10.9 | 11.9 | 2.0 | 0.2 |
| Seed 22, first mapping, mirrored | 12 | After block 12 alone | B | 1 923 | 3.0 | 23.1 | 0.4 | 0.6 | 11.1 | 12.0 | 2.1 | 0.2 |
| Seed 22, first mapping, mirrored | 18 | After block 18 alone | A | 2 100 | 3.0 | 21.3 | 0.5 | 0.7 | 11.1 | 12.0 | 1.9 | 0.2 |
| Seed 22, first mapping, mirrored | 18 | After block 18 alone | B | 1 923 | 3.0 | 23.0 | 0.4 | 0.6 | 11.5 | 12.0 | 2.1 | 0.2 |
| Seed 22, first mapping, mirrored | 24 | After block 24 alone | A | 2 100 | 3.0 | 21.3 | 0.5 | 0.7 | 11.1 | 12.0 | 1.9 | 0.2 |
| Seed 22, first mapping, mirrored | 24 | After block 24 alone | B | 1 923 | 3.0 | 23.0 | 0.4 | 0.6 | 11.6 | 12.3 | 2.1 | 0.2 |
| Seed 22, first mapping, mirrored | 6 | H-47's first plain night (020's job) | A | 2 097 | 3.1 | 21.4 | 0.5 | 0.7 | 10.9 | 12.0 | 2.0 | 0.1 |
| Seed 22, first mapping, mirrored | 6 | H-47's first plain night (020's job) | B | 1 921 | 3.0 | 23.1 | 0.4 | 0.6 | 11.2 | 12.2 | 2.2 | 0.1 |
| Seed 22, eighth, assignment | 12 | After block 12 alone | A | 2 100 | 3.0 | 21.3 | 0.5 | 0.7 | 11.1 | 12.7 | 1.8 | 0.2 |
| Seed 22, eighth, assignment | 12 | After block 12 alone | B | 1 923 | 3.0 | 23.2 | 0.4 | 0.7 | 11.3 | 12.6 | 2.0 | 0.2 |
| Seed 22, eighth, assignment | 18 | After block 18 alone | A | 2 100 | 3.0 | 21.3 | 0.5 | 0.7 | 11.1 | 12.5 | 1.8 | 0.2 |
| Seed 22, eighth, assignment | 18 | After block 18 alone | B | 1 923 | 3.0 | 23.0 | 0.4 | 0.6 | 11.4 | 12.8 | 1.8 | 0.2 |
| Seed 22, eighth, assignment | 24 | After block 24 alone | A | 2 100 | 3.0 | 21.3 | 0.5 | 0.8 | 11.2 | 13.0 | 1.9 | 0.2 |
| Seed 22, eighth, assignment | 24 | After block 24 alone | B | 1 923 | 3.0 | 23.0 | 0.4 | 0.6 | 11.4 | 12.4 | 2.0 | 0.2 |
| Seed 22, eighth, assignment | 6 | H-47's first plain night (020's job) | A | 2 048 | 3.1 | 21.5 | 0.5 | 0.7 | 11.0 | 12.8 | 1.8 | 0.2 |
| Seed 22, eighth, assignment | 6 | H-47's first plain night (020's job) | B | 1 940 | 3.0 | 23.2 | 0.4 | 0.6 | 11.1 | 12.6 | 1.8 | 0.1 |

- **Each form's replayed night ran about as many ripples as H-47's first plain night**: 2 099 to 2 100 on A's patterns and 1 923 on B's
  over the three paths, after block 12, 18 or 24 alike, where H-47's first plain night ran 2 046 to 2 097 and 1 920 to 1 941.
- **Beyond the pattern, a replay drives the other readout more than the answer's** in every row of the three forms, by 0.54 to 3.46
  spikes a ripple, and over H-47's first plain night by 0.96 to 2.97. The pattern's first unit fires 2.95 to 3.07 times a ripple and the
  rest of the pattern 21.25 to 25.38 times under the three forms.

What the pairings of A's and B's 102 stimulus units put into the weights over the night, the mean over the three paths: by the target's
class against the unit's stimulus, the pair's own in both readouts together and elsewhere the other stimulus units and the units outside
the readouts; then by the unit's class, onto every target:

| Cell | Night after block | Form | Onto its own pattern | Onto the answer's readout | Onto the other readout | Onto the pair's own | Onto elsewhere | From A alone | From A and C | From B alone | From B and D |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 12 | After block 12 alone | +1 029 774 | −63 881 | −313 342 | +40 810 | −23 283 | +118 848 | +303 945 | +78 553 | +168 732 |
| Seed 22, eighth, mirrored | 18 | After block 18 alone | +867 915 | −75 227 | −314 705 | +31 315 | −22 957 | +21 265 | +279 695 | +26 441 | +158 939 |
| Seed 22, eighth, mirrored | 24 | After block 24 alone | +1 139 638 | −68 315 | −194 724 | +33 785 | −23 921 | +202 035 | +215 618 | +190 357 | +278 452 |
| Seed 22, eighth, mirrored | 6 | H-47's first plain night (021's job) | +913 227 | −82 786 | −262 475 | +65 834 | −18 562 | +81 061 | +187 891 | +129 207 | +217 079 |
| Seed 22, first mapping, assignment | 12 | After block 12 alone | +804 052 | −113 696 | −469 338 | −8 526 | −27 362 | +33 742 | +60 931 | −11 839 | +102 296 |
| Seed 22, first mapping, assignment | 18 | After block 18 alone | +690 855 | −100 434 | −410 732 | +1 284 | −27 963 | −62 078 | +103 589 | +86 125 | +25 374 |
| Seed 22, first mapping, assignment | 24 | After block 24 alone | +702 993 | −112 207 | −419 660 | −13 004 | −30 253 | −244 | +28 482 | +65 798 | +33 832 |
| Seed 22, first mapping, assignment | 6 | H-47's first plain night (021's job) | +643 589 | −96 020 | −459 054 | −4 716 | −30 275 | −66 014 | +22 848 | +29 474 | +67 216 |
| Seed 23, eighth, mirrored | 12 | After block 12 alone | +879 388 | −123 526 | −303 696 | +33 412 | −25 311 | +181 063 | +103 679 | +119 178 | +56 347 |
| Seed 23, eighth, mirrored | 18 | After block 18 alone | +865 200 | −112 637 | −346 698 | +1 938 | −24 798 | +103 115 | +132 014 | +187 987 | −40 113 |
| Seed 23, eighth, mirrored | 24 | After block 24 alone | +934 438 | −101 723 | −358 846 | +32 470 | −21 984 | +201 914 | +58 955 | +166 793 | +56 694 |
| Seed 23, eighth, mirrored | 6 | H-47's first plain night (021's job) | +1 063 355 | −110 057 | −325 818 | +74 741 | −25 976 | +225 122 | +168 948 | +124 433 | +157 743 |
| Seed 23, first mapping, assignment | 12 | After block 12 alone | +849 255 | −89 014 | −399 578 | +9 452 | −25 643 | +106 160 | +122 017 | +49 291 | +67 003 |
| Seed 23, first mapping, assignment | 18 | After block 18 alone | +758 851 | −118 371 | −496 661 | +9 235 | −26 515 | +52 414 | +64 921 | +60 949 | −51 746 |
| Seed 23, first mapping, assignment | 24 | After block 24 alone | +769 422 | −116 955 | −513 419 | −5 067 | −24 333 | +82 159 | +111 403 | +18 746 | −102 660 |
| Seed 23, first mapping, assignment | 6 | H-47's first plain night (021's job) | +704 567 | −131 706 | −469 947 | −13 645 | −27 158 | +29 546 | −10 925 | +27 454 | +16 037 |
| Seed 24, eighth, mirrored | 12 | After block 12 alone | +1 064 649 | −63 283 | −312 872 | +72 074 | −32 246 | +177 627 | +237 925 | +57 296 | +255 474 |
| Seed 24, eighth, mirrored | 18 | After block 18 alone | +1 034 442 | −37 498 | −334 171 | +61 282 | −39 078 | +114 735 | +302 301 | +95 939 | +172 001 |
| Seed 24, eighth, mirrored | 24 | After block 24 alone | +1 091 153 | +3 211 | −238 743 | +71 496 | −27 389 | +203 658 | +169 286 | +182 695 | +344 088 |
| Seed 24, eighth, mirrored | 6 | H-47's first plain night (021's job) | +977 434 | −80 552 | −290 626 | +54 581 | −29 760 | +83 317 | +192 057 | +100 840 | +254 863 |
| Seed 24, first mapping, assignment | 12 | After block 12 alone | +888 942 | −33 758 | −420 978 | +23 655 | −22 026 | +140 761 | +43 575 | +75 088 | +176 412 |
| Seed 24, first mapping, assignment | 18 | After block 18 alone | +678 916 | −16 873 | −445 650 | +9 290 | −34 784 | +58 782 | +12 227 | +46 550 | +73 340 |
| Seed 24, first mapping, assignment | 24 | After block 24 alone | +871 565 | −27 671 | −458 047 | +2 692 | −30 302 | +105 226 | +67 188 | +28 690 | +157 135 |
| Seed 24, first mapping, assignment | 6 | H-47's first plain night (021's job) | +710 891 | −36 493 | −516 336 | −6 288 | −28 011 | −7 063 | −9 014 | +6 336 | +133 505 |
| Seed 23, eighth, assignment | 12 | After block 12 alone | +902 357 | −104 045 | −252 849 | +13 456 | −15 435 | +223 789 | +130 096 | +8 144 | +181 454 |
| Seed 23, eighth, assignment | 18 | After block 18 alone | +832 489 | −106 562 | −250 214 | +16 208 | −17 950 | +104 235 | +186 209 | +49 547 | +133 980 |
| Seed 23, eighth, assignment | 24 | After block 24 alone | +1 047 032 | −73 243 | −201 663 | +30 441 | −14 097 | +227 357 | +131 442 | +159 403 | +270 268 |
| Seed 23, eighth, assignment | 6 | H-47's first plain night (021's job) | +920 222 | −62 874 | −262 661 | +45 168 | −17 425 | +204 399 | +122 604 | +73 107 | +222 320 |
| Seed 24, first mapping, mirrored | 12 | After block 12 alone | +744 124 | −89 338 | −461 850 | +16 896 | −37 019 | −17 144 | +4 707 | +1 050 | +184 200 |
| Seed 24, first mapping, mirrored | 18 | After block 18 alone | +548 243 | −102 836 | −375 097 | +14 447 | −46 532 | −110 329 | +42 671 | −12 624 | +118 507 |
| Seed 24, first mapping, mirrored | 24 | After block 24 alone | +753 400 | −143 483 | −410 715 | +6 868 | −42 072 | −82 087 | +101 761 | +20 006 | +124 318 |
| Seed 24, first mapping, mirrored | 6 | H-47's first plain night (021's job) | +728 969 | −96 957 | −516 099 | +8 374 | −35 834 | −101 083 | +14 212 | +81 236 | +94 087 |
| Seed 23, first mapping, mirrored | 12 | After block 12 alone | +754 157 | −98 521 | −466 439 | −901 | −21 069 | +70 754 | −59 206 | +52 903 | +102 776 |
| Seed 23, first mapping, mirrored | 18 | After block 18 alone | +622 269 | −123 566 | −532 907 | +26 006 | −21 346 | −16 005 | −102 145 | +73 847 | +14 759 |
| Seed 23, first mapping, mirrored | 24 | After block 24 alone | +701 853 | −137 676 | −388 023 | +12 017 | −17 785 | +58 176 | +26 492 | +8 160 | +77 557 |
| Seed 23, first mapping, mirrored | 6 | H-47's first plain night (030's job) | +698 873 | −95 417 | −536 744 | +8 123 | −17 340 | +51 443 | −72 828 | +6 496 | +72 384 |
| Seed 24, eighth, assignment | 12 | After block 12 alone | +857 457 | −67 796 | −282 142 | +83 176 | −34 163 | +120 407 | +122 185 | +121 303 | +192 638 |
| Seed 24, eighth, assignment | 18 | After block 18 alone | +797 105 | −75 327 | −240 715 | +52 168 | −43 113 | +30 279 | +189 918 | +134 103 | +135 819 |
| Seed 24, eighth, assignment | 24 | After block 24 alone | +896 396 | −48 672 | −147 485 | +57 835 | −35 982 | +172 850 | +92 682 | +283 112 | +173 448 |
| Seed 24, eighth, assignment | 6 | H-47's first plain night (020's job) | +851 354 | −61 053 | −362 890 | +75 559 | −41 081 | +56 257 | +111 730 | +174 524 | +119 378 |
| Seed 22, first mapping, mirrored | 12 | After block 12 alone | +748 323 | −143 554 | −478 790 | +50 592 | −12 697 | −55 263 | +96 895 | −2 630 | +124 872 |
| Seed 22, first mapping, mirrored | 18 | After block 18 alone | +612 060 | −123 091 | −488 006 | +31 059 | −23 112 | −113 682 | −10 698 | +95 670 | +37 620 |
| Seed 22, first mapping, mirrored | 24 | After block 24 alone | +697 420 | −166 429 | −412 974 | +5 004 | −23 829 | −3 346 | +69 685 | −8 616 | +41 469 |
| Seed 22, first mapping, mirrored | 6 | H-47's first plain night (020's job) | +734 606 | −149 583 | −543 627 | +38 253 | −18 461 | +1 169 | −49 758 | +15 507 | +94 270 |
| Seed 22, eighth, assignment | 12 | After block 12 alone | +918 114 | −20 549 | −277 131 | +46 455 | −21 494 | +185 868 | +222 477 | +72 679 | +164 371 |
| Seed 22, eighth, assignment | 18 | After block 18 alone | +891 826 | −47 848 | −302 696 | +27 219 | −20 709 | +34 548 | +257 346 | +71 987 | +183 911 |
| Seed 22, eighth, assignment | 24 | After block 24 alone | +973 506 | −36 456 | −206 348 | +55 719 | −12 684 | +122 373 | +196 236 | +230 399 | +224 730 |
| Seed 22, eighth, assignment | 6 | H-47's first plain night (020's job) | +880 484 | −49 242 | −328 775 | +56 963 | −13 241 | +10 427 | +176 791 | +119 128 | +239 843 |

- **Every replayed night's pairings put weight onto the units' own patterns and took it from the other readout**: onto the patterns
  +548 243 to +1 139 638 a night under the three forms, from the other readout −147 485 to −532 907. From the answer's readout off the
  pattern they took −16 873 to −166 429 in every replayed night but after block 24 alone's on seed 24's eighth image from the mirrored
  assignment, which put +3 211 onto it. H-47's first plain night put +643 589 to +1 063 355 onto the patterns and took −262 475 to
  −543 627 from the other readout and −36 493 to −149 583 from the answer's readout.

The ties of both stimuli's trials, each form less the night without replay of its own job, ± the paired SE, and H-47's plain night less its
night without replay, H-47's own:

| Cell | Form | Block 6 | 12 | 18 | 24 | 36 |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | After block 12 alone | = without replay | −0.6 ± 0.50 | −0.3 ± 0.48 | −0.1 ± 0.85 | +0.8 ± 0.46 |
| Seed 22, eighth, mirrored | After block 18 alone | = without replay | = without replay | −0.4 ± 0.44 | −0.4 ± 0.17 | 0.0 ± 0.53 |
| Seed 22, eighth, mirrored | After block 24 alone | = without replay | = without replay | = without replay | −0.3 ± 0.46 | −0.1 ± 0.25 |
| Seed 22, eighth, mirrored | Plain on A's (H-47) | −1.4 ± 0.25 (H-47) | −0.3 ± 0.61 (H-47) | +0.7 ± 0.39 (H-47) | +0.5 ± 0.40 (H-47) | +1.5 ± 0.59 (H-47) |
| Seed 22, first mapping, assignment | After block 12 alone | = without replay | +0.3 ± 0.50 | −1.0 ± 0.41 | 0.0 ± 0.57 | +0.3 ± 0.12 |
| Seed 22, first mapping, assignment | After block 18 alone | = without replay | = without replay | −0.3 ± 0.62 | −0.1 ± 0.10 | −0.1 ± 0.28 |
| Seed 22, first mapping, assignment | After block 24 alone | = without replay | = without replay | = without replay | +1.0 ± 0.38 | +0.3 ± 0.92 |
| Seed 22, first mapping, assignment | Plain on A's (H-47) | −0.5 ± 0.15 (H-47) | +1.0 ± 0.29 (H-47) | −0.1 ± 0.37 (H-47) | +0.5 ± 0.65 (H-47) | −0.1 ± 0.44 (H-47) |
| Seed 23, eighth, mirrored | After block 12 alone | = without replay | +0.3 ± 0.34 | +0.9 ± 0.46 | +0.4 ± 0.26 | +0.2 ± 0.36 |
| Seed 23, eighth, mirrored | After block 18 alone | = without replay | = without replay | 0.0 ± 0.56 | +0.2 ± 0.56 | +0.5 ± 0.39 |
| Seed 23, eighth, mirrored | After block 24 alone | = without replay | = without replay | = without replay | +0.3 ± 0.40 | +0.7 ± 0.38 |
| Seed 23, eighth, mirrored | Plain on A's (H-47) | −0.1 ± 0.45 (H-47) | +1.0 ± 0.48 (H-47) | +0.3 ± 0.59 (H-47) | +0.2 ± 0.46 (H-47) | +0.8 ± 0.35 (H-47) |
| Seed 23, first mapping, assignment | After block 12 alone | = without replay | −0.6 ± 0.49 | −0.5 ± 0.40 | −0.1 ± 0.38 | 0.0 ± 0.42 |
| Seed 23, first mapping, assignment | After block 18 alone | = without replay | = without replay | −0.4 ± 0.92 | −0.1 ± 0.32 | +0.1 ± 0.65 |
| Seed 23, first mapping, assignment | After block 24 alone | = without replay | = without replay | = without replay | +0.3 ± 0.57 | −0.4 ± 0.66 |
| Seed 23, first mapping, assignment | Plain on A's (H-47) | −0.3 ± 0.64 (H-47) | −0.7 ± 0.58 (H-47) | −1.9 ± 0.56 (H-47) | −2.1 ± 0.29 (H-47) | −2.1 ± 0.47 (H-47) |
| Seed 24, eighth, mirrored | After block 12 alone | = without replay | +1.1 ± 0.63 | +0.7 ± 0.30 | +0.1 ± 0.31 | +0.8 ± 0.47 |
| Seed 24, eighth, mirrored | After block 18 alone | = without replay | = without replay | +1.1 ± 0.52 | +0.4 ± 0.48 | +2.1 ± 0.60 |
| Seed 24, eighth, mirrored | After block 24 alone | = without replay | = without replay | = without replay | +0.3 ± 0.27 | +0.7 ± 0.52 |
| Seed 24, eighth, mirrored | Plain on A's (H-47) | +0.7 ± 0.53 (H-47) | +0.5 ± 0.62 (H-47) | +1.1 ± 0.28 (H-47) | +1.1 ± 0.52 (H-47) | +1.5 ± 0.80 (H-47) |
| Seed 24, first mapping, assignment | After block 12 alone | = without replay | −0.5 ± 0.32 | +0.7 ± 0.77 | −0.4 ± 0.75 | −0.6 ± 0.47 |
| Seed 24, first mapping, assignment | After block 18 alone | = without replay | = without replay | +0.6 ± 0.44 | −0.5 ± 0.64 | −0.5 ± 0.32 |
| Seed 24, first mapping, assignment | After block 24 alone | = without replay | = without replay | = without replay | −0.8 ± 0.59 | −0.1 ± 0.38 |
| Seed 24, first mapping, assignment | Plain on A's (H-47) | −0.9 ± 0.25 (H-47) | −0.4 ± 0.34 (H-47) | +0.2 ± 0.06 (H-47) | −1.2 ± 1.09 (H-47) | −0.5 ± 0.29 (H-47) |
| Seed 23, eighth, assignment | After block 12 alone | = without replay | +0.4 ± 0.58 | +0.9 ± 0.39 | +0.8 ± 0.49 | +0.7 ± 0.52 |
| Seed 23, eighth, assignment | After block 18 alone | = without replay | = without replay | +0.4 ± 0.54 | +0.1 ± 0.51 | +0.5 ± 0.58 |
| Seed 23, eighth, assignment | After block 24 alone | = without replay | = without replay | = without replay | +1.2 ± 0.15 | +0.5 ± 0.18 |
| Seed 23, eighth, assignment | Plain on A's (H-47) | −0.3 ± 0.60 (H-47) | +0.2 ± 0.46 (H-47) | −0.6 ± 0.31 (H-47) | +0.5 ± 0.46 (H-47) | 0.0 ± 0.60 (H-47) |
| Seed 24, first mapping, mirrored | After block 12 alone | = without replay | +0.3 ± 0.84 | −1.0 ± 0.45 | −0.1 ± 0.74 | 0.0 ± 0.31 |
| Seed 24, first mapping, mirrored | After block 18 alone | = without replay | = without replay | +0.1 ± 0.14 | −0.1 ± 0.46 | +0.8 ± 0.41 |
| Seed 24, first mapping, mirrored | After block 24 alone | = without replay | = without replay | = without replay | +0.3 ± 0.59 | +0.6 ± 0.30 |
| Seed 24, first mapping, mirrored | Plain on A's (H-47) | −0.4 ± 0.47 (H-47) | −0.9 ± 0.47 (H-47) | −1.2 ± 0.46 (H-47) | −1.0 ± 0.47 (H-47) | −0.3 ± 0.68 (H-47) |
| Seed 23, first mapping, mirrored | After block 12 alone | = without replay | −0.3 ± 0.60 | −0.2 ± 0.13 | −0.4 ± 0.25 | +0.9 ± 0.32 |
| Seed 23, first mapping, mirrored | After block 18 alone | = without replay | = without replay | +0.5 ± 0.35 | 0.0 ± 0.25 | +0.7 ± 0.47 |
| Seed 23, first mapping, mirrored | After block 24 alone | = without replay | = without replay | = without replay | +0.6 ± 0.59 | +0.7 ± 0.36 |
| Seed 23, first mapping, mirrored | Plain on A's (H-47) | −0.6 ± 0.42 (H-47) | −0.6 ± 0.56 (H-47) | +0.4 ± 0.43 (H-47) | +0.6 ± 0.36 (H-47) | +1.7 ± 0.21 (H-47) |
| Seed 24, eighth, assignment | After block 12 alone | = without replay | +1.0 ± 0.24 | +0.6 ± 0.33 | +1.0 ± 0.26 | −0.8 ± 0.51 |
| Seed 24, eighth, assignment | After block 18 alone | = without replay | = without replay | +1.5 ± 0.44 | +0.6 ± 0.18 | −0.4 ± 0.37 |
| Seed 24, eighth, assignment | After block 24 alone | = without replay | = without replay | = without replay | +1.2 ± 0.04 | −0.7 ± 0.51 |
| Seed 24, eighth, assignment | Plain on A's (H-47) | +0.5 ± 0.38 (H-47) | +0.3 ± 0.24 (H-47) | +2.0 ± 0.40 (H-47) | +1.2 ± 0.27 (H-47) | +0.4 ± 0.37 (H-47) |
| Seed 22, first mapping, mirrored | After block 12 alone | = without replay | +0.6 ± 0.43 | −0.4 ± 0.15 | −0.2 ± 0.52 | −0.2 ± 0.14 |
| Seed 22, first mapping, mirrored | After block 18 alone | = without replay | = without replay | +0.1 ± 0.35 | −0.6 ± 0.45 | −0.6 ± 0.22 |
| Seed 22, first mapping, mirrored | After block 24 alone | = without replay | = without replay | = without replay | −0.4 ± 0.54 | −0.7 ± 0.43 |
| Seed 22, first mapping, mirrored | Plain on A's (H-47) | +0.2 ± 0.37 (H-47) | +0.1 ± 0.56 (H-47) | −0.6 ± 0.45 (H-47) | −0.4 ± 0.81 (H-47) | −0.7 ± 0.35 (H-47) |
| Seed 22, eighth, assignment | After block 12 alone | = without replay | +0.5 ± 0.55 | +0.5 ± 0.39 | +0.1 ± 0.28 | +0.4 ± 0.48 |
| Seed 22, eighth, assignment | After block 18 alone | = without replay | = without replay | 0.0 ± 0.15 | −0.4 ± 0.28 | −0.4 ± 0.72 |
| Seed 22, eighth, assignment | After block 24 alone | = without replay | = without replay | = without replay | +0.4 ± 0.20 | +0.1 ± 0.50 |
| Seed 22, eighth, assignment | Plain on A's (H-47) | −0.3 ± 0.33 (H-47) | 0.0 ± 0.45 (H-47) | −0.1 ± 0.45 (H-47) | 0.0 ± 0.24 (H-47) | −0.1 ± 0.18 (H-47) |

- **At block 36 the ties** under after block 12 alone lie above the night without replay's beyond 2 SE on seed 22's first mapping from
  the assignment, +0.3 ± 0.12, and seed 23's first mapping from the mirrored assignment, +0.9 ± 0.32, and within in ten; under after block
  18 alone above on seed 24's eighth image from the mirrored assignment, +2.1 ± 0.60, and seed 24's first mapping from the mirrored
  assignment, +0.8 ± 0.41, below on seed 22's first mapping from the mirrored assignment, −0.6 ± 0.22, and within in nine; under after
  block 24 alone above on seed 23's eighth image from the assignment, +0.5 ± 0.18, and within in eleven. H-47's plain night's lie above
  its night without replay's beyond 2 SE in three cells and below on seed 23's first mapping from the assignment, −2.1 ± 0.47.

## 7. The twelve cells at block 36

H-47's twelve cells at block 36, the rows ordered by the readout A answers, the six where it answers readout 0 first. The three forms and
the night without replay are this exploration's jobs, exploration 030's on seed 23's first mapping from the mirrored assignment; after block
6 alone is exploration 031's job's, exploration 030's on that cell; H-47's plain night is H-47's job's in all twelve. "Less without replay"
carries the job's paired SE, "less H-47's plain" exploration 018's unpaired SE, and H-47's clause 1 H-47's paired SE. No figure is pooled
across cells.

A's count, each form less the night without replay, beside H-47's clause 1:

| Cell | A answers | After block 12 alone less without replay | after 18 alone | after 24 alone | after 6 alone (031) | H-47: plain less without replay (clause 1) |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | +8.1 ± 0.74 | +6.3 ± 0.74 | +5.1 ± 1.31 | +7.3 ± 0.65 | +19.6 ± 1.06 |
| Seed 22, first mapping, assignment | readout 0, the odd places | +3.2 ± 0.58 | +1.7 ± 0.52 | +0.8 ± 1.29 | +4.3 ± 1.05 | +7.9 ± 0.51 |
| Seed 23, eighth, mirrored | readout 0, the odd places | +3.8 ± 0.82 | +3.0 ± 0.64 | +2.4 ± 0.35 | +4.3 ± 0.63 | +17.4 ± 1.08 |
| Seed 23, first mapping, assignment | readout 0, the odd places | +6.6 ± 1.68 | +5.6 ± 1.11 | +5.8 ± 0.54 | +5.0 ± 2.61 | +13.4 ± 1.56 |
| Seed 24, eighth, mirrored | readout 0, the odd places | +3.8 ± 0.78 | +5.4 ± 1.17 | +1.9 ± 1.18 | +3.1 ± 1.84 | +11.3 ± 0.96 |
| Seed 24, first mapping, assignment | readout 0, the odd places | +3.1 ± 0.40 | +2.6 ± 1.40 | +1.6 ± 1.34 | +3.9 ± 0.76 | +3.8 ± 1.57 |
| Seed 23, eighth, assignment | readout 1, the even places | −5.6 ± 1.92 | −1.1 ± 1.78 | −1.5 ± 1.44 | +1.3 ± 1.04 | +4.1 ± 1.45 |
| Seed 24, first mapping, mirrored | readout 1, the even places | +4.6 ± 1.37 | +0.3 ± 0.70 | +1.3 ± 0.66 | +3.2 ± 2.06 | +6.0 ± 1.54 |
| Seed 23, first mapping, mirrored | readout 1, the even places | −0.5 ± 1.11 | +0.3 ± 0.83 | −2.4 ± 0.86 | +1.7 ± 0.78 | −4.9 ± 1.09 |
| Seed 24, eighth, assignment | readout 1, the even places | −2.1 ± 1.78 | −5.5 ± 0.95 | −4.6 ± 0.73 | −2.0 ± 0.88 | +1.8 ± 1.79 |
| Seed 22, first mapping, mirrored | readout 1, the even places | +0.5 ± 0.15 | +3.1 ± 0.49 | +1.7 ± 1.36 | +1.3 ± 2.40 | +5.0 ± 1.24 |
| Seed 22, eighth, assignment | readout 1, the even places | +1.1 ± 0.81 | +2.6 ± 0.38 | −1.1 ± 0.65 | −0.9 ± 1.07 | +2.2 ± 0.65 |

A's count, each form less H-47's plain night:

| Cell | A answers | After block 12 alone less H-47's plain | after 18 alone | after 24 alone | after 6 alone (031) |
| :--- | :--- | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | −11.5 ± 1.59 | −13.4 ± 0.74 | −14.6 ± 1.71 | −12.4 ± 1.05 |
| Seed 22, first mapping, assignment | readout 0, the odd places | −4.7 ± 0.55 | −6.3 ± 0.46 | −7.1 ± 0.83 | −3.6 ± 0.87 |
| Seed 23, eighth, mirrored | readout 0, the odd places | −13.6 ± 1.05 | −14.4 ± 0.93 | −15.0 ± 1.16 | −13.1 ± 1.09 |
| Seed 23, first mapping, assignment | readout 0, the odd places | −6.8 ± 1.88 | −7.7 ± 1.97 | −7.5 ± 1.59 | −8.3 ± 1.52 |
| Seed 24, eighth, mirrored | readout 0, the odd places | −7.5 ± 0.98 | −5.8 ± 1.20 | −9.4 ± 1.39 | −8.2 ± 2.09 |
| Seed 24, first mapping, assignment | readout 0, the odd places | −0.7 ± 1.89 | −1.1 ± 1.85 | −2.1 ± 1.02 | +0.1 ± 1.97 |
| Seed 23, eighth, assignment | readout 1, the even places | −9.6 ± 1.06 | −5.2 ± 1.12 | −5.6 ± 1.20 | −2.8 ± 1.52 |
| Seed 24, first mapping, mirrored | readout 1, the even places | −1.3 ± 0.73 | −5.7 ± 1.86 | −4.7 ± 1.15 | −2.8 ± 1.01 |
| Seed 23, first mapping, mirrored | readout 1, the even places | +4.4 ± 0.59 | +5.2 ± 1.22 | +2.5 ± 1.31 | +6.6 ± 0.90 |
| Seed 24, eighth, assignment | readout 1, the even places | −3.9 ± 0.91 | −7.3 ± 1.29 | −6.5 ± 1.63 | −3.8 ± 2.14 |
| Seed 22, first mapping, mirrored | readout 1, the even places | −4.5 ± 1.52 | −1.8 ± 1.39 | −3.3 ± 0.92 | −3.7 ± 1.74 |
| Seed 22, eighth, assignment | readout 1, the even places | −1.1 ± 1.44 | +0.4 ± 1.16 | −3.3 ± 1.26 | −3.1 ± 1.56 |

A's trials and B's trials, each form less the night without replay:

| Cell | A answers | A's trials: after block 12 alone less without replay | after 18 alone | after 24 alone | after 6 alone (031) | H-47: plain less without replay |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | +2.5 ± 1.08 | −0.1 ± 0.39 | +0.8 ± 0.89 | +0.8 ± 0.55 | +9.2 ± 0.86 |
| Seed 22, first mapping, assignment | readout 0, the odd places | +3.8 ± 0.74 | +1.8 ± 0.38 | +0.8 ± 1.04 | +4.0 ± 1.93 | +8.4 ± 1.10 |
| Seed 23, eighth, mirrored | readout 0, the odd places | +3.6 ± 0.55 | +2.9 ± 0.47 | +2.3 ± 0.28 | +3.7 ± 0.55 | +15.9 ± 1.05 |
| Seed 23, first mapping, assignment | readout 0, the odd places | +5.0 ± 1.27 | +5.0 ± 1.15 | +5.2 ± 0.21 | +6.4 ± 2.30 | +13.5 ± 1.09 |
| Seed 24, eighth, mirrored | readout 0, the odd places | +0.7 ± 0.29 | +1.7 ± 0.54 | −0.8 ± 0.34 | +0.7 ± 0.86 | +1.5 ± 0.74 |
| Seed 24, first mapping, assignment | readout 0, the odd places | +0.8 ± 1.08 | −0.1 ± 0.68 | +1.0 ± 0.57 | +0.8 ± 0.49 | +3.3 ± 0.54 |
| Seed 23, eighth, assignment | readout 1, the even places | −0.9 ± 0.76 | −0.5 ± 0.93 | −1.9 ± 0.06 | +2.4 ± 0.41 | +2.3 ± 0.88 |
| Seed 24, first mapping, mirrored | readout 1, the even places | +3.0 ± 0.74 | −1.2 ± 0.40 | +0.3 ± 0.70 | +3.4 ± 0.53 | +3.3 ± 0.67 |
| Seed 23, first mapping, mirrored | readout 1, the even places | −1.8 ± 1.04 | −1.3 ± 0.67 | −1.1 ± 0.24 | 0.0 ± 0.56 | −5.2 ± 1.50 |
| Seed 24, eighth, assignment | readout 1, the even places | −1.2 ± 0.90 | −4.1 ± 0.51 | −5.5 ± 0.75 | −1.0 ± 0.63 | +0.2 ± 0.76 |
| Seed 22, first mapping, mirrored | readout 1, the even places | +0.3 ± 0.11 | +0.7 ± 0.32 | +0.1 ± 0.49 | +0.1 ± 1.24 | −0.4 ± 0.83 |
| Seed 22, eighth, assignment | readout 1, the even places | −0.1 ± 0.40 | +1.1 ± 0.56 | −1.3 ± 0.44 | −2.0 ± 0.58 | −1.6 ± 0.88 |

| Cell | A answers | B's trials: after block 12 alone less without replay | after 18 alone | after 24 alone | after 6 alone (031) | H-47: plain less without replay |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | +5.6 ± 0.96 | +6.4 ± 0.55 | +4.3 ± 1.03 | +6.5 ± 0.44 | +10.4 ± 0.54 |
| Seed 22, first mapping, assignment | readout 0, the odd places | −0.6 ± 0.22 | −0.1 ± 0.23 | 0.0 ± 1.26 | +0.3 ± 1.05 | −0.4 ± 0.83 |
| Seed 23, eighth, mirrored | readout 0, the odd places | +0.3 ± 0.28 | +0.1 ± 0.34 | +0.2 ± 0.26 | +0.6 ± 0.17 | +1.5 ± 0.28 |
| Seed 23, first mapping, assignment | readout 0, the odd places | +1.5 ± 0.82 | +0.6 ± 0.36 | +0.6 ± 0.40 | −1.4 ± 0.41 | −0.1 ± 0.62 |
| Seed 24, eighth, mirrored | readout 0, the odd places | +3.1 ± 1.02 | +3.7 ± 0.86 | +2.7 ± 0.89 | +2.4 ± 1.06 | +9.7 ± 0.98 |
| Seed 24, first mapping, assignment | readout 0, the odd places | +2.3 ± 0.75 | +2.8 ± 0.92 | +0.6 ± 1.66 | +3.1 ± 0.45 | +0.5 ± 1.44 |
| Seed 23, eighth, assignment | readout 1, the even places | −4.7 ± 1.43 | −0.7 ± 0.93 | +0.3 ± 1.46 | −1.2 ± 1.31 | +1.8 ± 0.84 |
| Seed 24, first mapping, mirrored | readout 1, the even places | +1.7 ± 0.76 | +1.5 ± 0.64 | +1.0 ± 0.53 | −0.1 ± 1.54 | +2.7 ± 0.99 |
| Seed 23, first mapping, mirrored | readout 1, the even places | +1.3 ± 0.58 | +1.5 ± 0.76 | −1.3 ± 0.96 | +1.7 ± 0.40 | +0.3 ± 0.88 |
| Seed 24, eighth, assignment | readout 1, the even places | −0.9 ± 1.04 | −1.4 ± 0.88 | +0.9 ± 0.13 | −1.0 ± 0.71 | +1.6 ± 1.50 |
| Seed 22, first mapping, mirrored | readout 1, the even places | +0.3 ± 0.19 | +2.4 ± 0.49 | +1.6 ± 1.17 | +1.1 ± 1.27 | +5.4 ± 1.47 |
| Seed 22, eighth, assignment | readout 1, the even places | +1.2 ± 0.57 | +1.5 ± 0.71 | +0.2 ± 0.62 | +1.1 ± 1.03 | +3.8 ± 0.81 |

A's trials and B's trials, each form less H-47's plain night:

| Cell | A answers | A's trials: after block 12 alone less H-47's plain | after 18 alone | after 24 alone | after 6 alone (031) |
| :--- | :--- | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | −6.8 ± 1.01 | −9.4 ± 0.89 | −8.5 ± 0.99 | −8.5 ± 1.02 |
| Seed 22, first mapping, assignment | readout 0, the odd places | −4.5 ± 1.20 | −6.5 ± 1.19 | −7.5 ± 1.20 | −4.4 ± 1.43 |
| Seed 23, eighth, mirrored | readout 0, the odd places | −12.3 ± 1.00 | −13.0 ± 1.09 | −13.6 ± 1.25 | −12.2 ± 1.13 |
| Seed 23, first mapping, assignment | readout 0, the odd places | −8.4 ± 1.64 | −8.5 ± 1.45 | −8.3 ± 1.34 | −7.1 ± 1.51 |
| Seed 24, eighth, mirrored | readout 0, the odd places | −0.8 ± 0.86 | +0.2 ± 1.42 | −2.4 ± 1.13 | −0.9 ± 1.68 |
| Seed 24, first mapping, assignment | readout 0, the odd places | −2.5 ± 1.60 | −3.4 ± 1.26 | −2.2 ± 0.87 | −2.5 ± 1.07 |
| Seed 23, eighth, assignment | readout 1, the even places | −3.2 ± 1.18 | −2.8 ± 0.88 | −4.1 ± 1.12 | +0.1 ± 1.48 |
| Seed 24, first mapping, mirrored | readout 1, the even places | −0.3 ± 0.53 | −4.4 ± 1.11 | −3.0 ± 0.88 | +0.1 ± 0.61 |
| Seed 23, first mapping, mirrored | readout 1, the even places | +3.4 ± 0.96 | +3.9 ± 1.35 | +4.1 ± 1.69 | +5.2 ± 1.42 |
| Seed 24, eighth, assignment | readout 1, the even places | −1.4 ± 1.06 | −4.3 ± 0.97 | −5.7 ± 0.53 | −1.1 ± 1.43 |
| Seed 22, first mapping, mirrored | readout 1, the even places | +0.7 ± 1.12 | +1.1 ± 0.87 | +0.5 ± 0.95 | +0.5 ± 0.95 |
| Seed 22, eighth, assignment | readout 1, the even places | +1.5 ± 1.16 | +2.7 ± 1.40 | +0.3 ± 1.14 | −0.4 ± 1.13 |

| Cell | A answers | B's trials: after block 12 alone less H-47's plain | after 18 alone | after 24 alone | after 6 alone (031) |
| :--- | :--- | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | −4.8 ± 1.41 | −4.0 ± 0.66 | −6.1 ± 1.29 | −3.9 ± 0.69 |
| Seed 22, first mapping, assignment | readout 0, the odd places | −0.1 ± 0.90 | +0.3 ± 0.82 | +0.4 ± 1.40 | +0.8 ± 0.71 |
| Seed 23, eighth, mirrored | readout 0, the odd places | −1.3 ± 0.47 | −1.4 ± 0.53 | −1.3 ± 0.49 | −0.9 ± 0.37 |
| Seed 23, first mapping, assignment | readout 0, the odd places | +1.7 ± 0.77 | +0.8 ± 0.84 | +0.8 ± 0.68 | −1.3 ± 0.56 |
| Seed 24, eighth, mirrored | readout 0, the odd places | −6.7 ± 0.70 | −6.0 ± 0.83 | −7.0 ± 0.74 | −7.3 ± 0.99 |
| Seed 24, first mapping, assignment | readout 0, the odd places | +1.8 ± 1.52 | +2.3 ± 1.26 | +0.1 ± 0.96 | +2.6 ± 1.71 |
| Seed 23, eighth, assignment | readout 1, the even places | −6.5 ± 1.50 | −2.5 ± 1.11 | −1.5 ± 1.03 | −3.0 ± 1.02 |
| Seed 24, first mapping, mirrored | readout 1, the even places | −1.1 ± 0.83 | −1.3 ± 1.16 | −1.7 ± 1.08 | −2.9 ± 1.13 |
| Seed 23, first mapping, mirrored | readout 1, the even places | +1.0 ± 0.94 | +1.2 ± 0.92 | −1.6 ± 1.16 | +1.4 ± 1.31 |
| Seed 24, eighth, assignment | readout 1, the even places | −2.5 ± 1.28 | −3.0 ± 0.80 | −0.8 ± 1.39 | −2.7 ± 0.99 |
| Seed 22, first mapping, mirrored | readout 1, the even places | −5.1 ± 1.64 | −2.9 ± 1.93 | −3.8 ± 1.32 | −4.3 ± 1.60 |
| Seed 22, eighth, assignment | readout 1, the even places | −2.6 ± 1.49 | −2.3 ± 1.55 | −3.5 ± 1.31 | −2.7 ± 1.27 |

The side at block 36 under each form, and H-47's:

| Cell | A answers | After block 12 alone: side | After block 18 alone: side | After block 24 alone: side | After block 6 alone (031): side | H-47's side |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Seed 22, eighth, mirrored | readout 0, the odd places | B, the even places (within 2 SE) | B, the even places | B, the even places | B, the even places | B, the even places (within 2 SE) |
| Seed 22, first mapping, assignment | readout 0, the odd places | A, the odd places | A, the odd places | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places |
| Seed 23, eighth, mirrored | readout 0, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | readout 0, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 24, eighth, mirrored | readout 0, the odd places | B, the even places (within 2 SE) | B, the even places | B, the even places | B, the even places | B, the even places |
| Seed 24, first mapping, assignment | readout 0, the odd places | B, the even places (within 2 SE) | B, the even places | A, the odd places (within 2 SE) | B, the even places | A, the odd places (within 2 SE) |
| Seed 23, eighth, assignment | readout 1, the even places | A, the even places | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | readout 1, the even places | A, the even places | B, the odd places | B, the odd places (within 2 SE) | A, the even places | A, the even places (within 2 SE) |
| Seed 23, first mapping, mirrored | readout 1, the even places | B, the odd places | B, the odd places | A, the even places (within 2 SE) | B, the odd places | B, the odd places |
| Seed 24, eighth, assignment | readout 1, the even places | B, the odd places (within 2 SE) | B, the odd places | B, the odd places | A, the even places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 22, first mapping, mirrored | readout 1, the even places | A, the even places (within 2 SE) | B, the odd places | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places |
| Seed 22, eighth, assignment | readout 1, the even places | B, the odd places | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places | B, the odd places |

- **Where H-47's clause 1 lies above beyond 2 SE**, ten cells, after block 12 alone lies above the night without replay beyond 2 SE in
  eight, within on seed 22's eighth image from the assignment and below on seed 23's eighth image from the assignment; after block 18
  alone above in seven and within in three, seed 24's first mapping from the assignment, seed 23's eighth image from the assignment and
  seed 24's first mapping from the mirrored assignment; after block 24 alone above in three and within in seven; after block 6 alone above
  in four and within in six.
- **Where H-47's clause 1 lies within 2 SE**, seed 24's eighth image from the assignment, +1.8 ± 1.79, after block 18 alone, −5.5 ± 0.95,
  and after block 24 alone, −4.6 ± 0.73, lie below the night without replay beyond 2 SE, as after block 6 alone did, −2.0 ± 0.88, and after
  block 12 alone within 2 SE, −2.1 ± 1.78. Less H-47's plain night all three lie below beyond 2 SE, −3.9 ± 0.91, −7.3 ± 1.29 and
  −6.5 ± 1.63, where after block 6 alone lay within, −3.8 ± 2.14.
- **Where H-47's clause 1 lies below beyond 2 SE**, seed 23's first mapping from the mirrored assignment, −4.9 ± 1.09, the readings are
  exploration 030's: after block 12 alone −0.5 ± 1.11 and after block 18 alone +0.3 ± 0.83 within 2 SE, after block 24 alone −2.4 ± 0.86
  below, and after block 6 alone +1.7 ± 0.78 above; less H-47's plain night after block 12 alone, +4.4 ± 0.59, and after block 18 alone,
  +5.2 ± 1.22, lie above beyond 2 SE, and after block 24 alone within, +2.5 ± 1.31.
- **Cell by cell over the four single nights at block 36**, all four lie above the night without replay beyond 2 SE on seed 22's eighth
  image from the mirrored assignment and seed 23's eighth image from the mirrored assignment, and three of four on seed 22's first mapping
  from the assignment and seed 23's first mapping from the assignment; none lies above on seed 23's eighth image from the assignment, where
  after block 12 alone lies below, or on seed 24's eighth image from the assignment, where all but after block 12 alone lie below. Of the
  four single nights, after block 6 alone reads the highest of the four in six cells, after block 12 alone in three and after block 18 alone
  in three, and after block 24 alone the lowest in seven.
- **The side at block 36** falls on H-47's stimulus under after block 12 alone and after block 18 alone in ten cells and under after block
  24 alone in nine (section 3).

## What it costs

The trial took 410 s and the eleven jobs 7 472 to 8 110 s, 124.5 to 135.2 minutes, at most 56.3 % of a job's 240 minutes; ADR-0247 put
a job at about 129 to 141 minutes. Nothing was cut: every job ran its four forms on the three paths, and every night was watched. Twelve
jobs ran, the trial's and eleven, every one to the end; seed 23's first mapping from the mirrored assignment ran none, its four forms read
from exploration 030's job.

## What it suggests

**On these twelve cells, H-47's plain night replayed after one later block alone, every other night without replay, left A's count at
block 36 above the night without replay beyond 2 SE in eight cells after block 12, seven after block 18 and three after block 24**, where
after block 6 alone it lay above in five. After block 12 and after block 18 alone lie above in all six and five of the six cells where A
answers readout 0, and in two of the six where it answers readout 1. Less H-47's plain night at block 36 each lies below beyond 2 SE in
eight to eleven cells.

**Each replayed night moved the readouts' synapses about as H-47's first plain night did, whichever block it followed**, and each night
and stretch after it left the inhibitory columns of the class matrix lower than H-47's night without replay did, as exploration 031 read
after block 6 alone. On seed 24's eighth image from the assignment A's count lay below the night without replay right after each later
replayed night, −7.3 ± 0.88 to −12.3 ± 0.58, more on A's trials than on B's, and −2.1 ± 1.78 to −5.5 ± 0.95 at block 36.

Questions this suggests for a later exploration or a round:
- Replay on two of the four nights, on any cell: where the four single nights' differences sum above or below H-47's plain night less the
  night without replay, which pairs of nights together read as H-47's four do.
- Seed 24's eighth image from the assignment, where A's count lay below the night without replay right after each later replayed night,
  by more on A's trials than on B's: what that night moves on A's trials there.
- Seed 23's eighth image from the assignment, where after block 12 alone lies below the night without replay at block 36 beyond 2 SE on
  B's trials, −4.7 ± 1.43, and the other single nights within 2 SE.
- A read unit by unit, to see which readout units fire on A's trials after each replayed night.

## What did not run

- **Replay after block 6 alone run again**: it is read from exploration 031's jobs, and on seed 23's first mapping from the mirrored
  assignment from exploration 030's, so the four single nights' sum carries no SE.
- **Replay on two of the four nights**, on any cell: so where single nights do not add up to H-47's plain night, these runs do not say
  which nights together do.
- **Replay on every night but one** on the eleven cells: exploration 030 ran it on seed 23's first mapping from the mirrored assignment
  alone.
- **Any hold**, on any night.
- **Plain nights on both mappings and the awake gap**, night by night or at all: no reading here stands against them.
- **A read unit by unit**: a read gives each readout's spike count, not which units fired.
- **Any path but H-47's three**, 0, 5 and 6.
- **A night split apart**, such as slow-wave sleep read apart from REM: a night's change is read whole, from the images before and after
  it, and REM's depotentiations and the zero sum's rounds are not split out of it, as in explorations 020 to 031.
- **The next day's reading of the pairings a night leaves** pending.
- **Any SE for section 5, or for section 6's tables of the engine and the replayed nights**: the class matrix, the split and the watch are
  means or totals over the three paths. Section 6's ties carry the paired SE.
- **H-47's plain night run again**: its rows and reads are H-47's job's and its night lines exploration 020's or 021's, so a form less it
  carries exploration 018's unpaired SE.
- **Any mean, difference or SE pooled across cells**, in section 7 or elsewhere. The tallies of cells, checkpoints, nights and pairings
  are counts of where each cell's own reading falls, not pooled figures.
- **Any other share, form, network or seed.**

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_each_later_nights_replay_alone_on_twelve_cells_explore` | [42970775904](https://github.com/Menteion/Menteion/actions/runs/42970775904) | `b93a130` | 410 |
| `each_later_night_alone_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683) | `b93a130` | 7663 |
| `each_later_night_alone_a_first_mapping_on_seed_22_from_the_assignment_explore` | [42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683) | `b93a130` | 8104 |
| `each_later_night_alone_the_eighth_image_on_seed_23_from_the_mirrored_assignment_explore` | [42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683) | `b93a130` | 7652 |
| `each_later_night_alone_a_first_mapping_on_seed_23_from_the_assignment_explore` | [42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683) | `b93a130` | 7690 |
| `each_later_night_alone_the_eighth_image_on_seed_24_from_the_mirrored_assignment_explore` | [42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683) | `b93a130` | 7603 |
| `each_later_night_alone_a_first_mapping_on_seed_24_from_the_assignment_explore` | [42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683) | `b93a130` | 7559 |
| `each_later_night_alone_the_eighth_image_on_seed_23_from_the_assignment_explore` | [42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683) | `b93a130` | 7687 |
| `each_later_night_alone_a_first_mapping_on_seed_24_from_the_mirrored_assignment_explore` | [42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683) | `b93a130` | 8110 |
| `one_night_alone_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [42621480151](https://github.com/Menteion/Menteion/actions/runs/42621480151) | `8291346` | 9756 |
| `each_later_night_alone_the_eighth_image_on_seed_24_from_the_assignment_explore` | [42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683) | `b93a130` | 7872 |
| `each_later_night_alone_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683) | `b93a130` | 7802 |
| `each_later_night_alone_the_eighth_image_on_seed_22_from_the_assignment_explore` | [42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683) | `b93a130` | 7472 |
