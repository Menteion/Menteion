---
status: exploratory
date: 2026-11-09
---

# 030: One cell, night by night

**Branch**: `explore/030-one-cell-night-by-night`
**Commit**: `8291346`, which adds the harness, on `23071d9`: `main` at `d7bc5ec`, ADR-0245's merge, with exploration 023's tag merged
in, which changes nothing under `src/`. This branch changes nothing under `src/`, so the engine is `main`'s.
**Runs**: [42619303153](https://github.com/Menteion/Menteion/actions/runs/42619303153) (the harness's trial) and
[42621480151](https://github.com/Menteion/Menteion/actions/runs/42621480151) (two jobs, one night alone and one night left out), both at
`scope=arms`. H-47's plain night on A's patterns is read from H-47's run
[41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613), and its class matrices, night lines and watch on this cell
from exploration 020's run [41500459562](https://github.com/Menteion/Menteion/actions/runs/41500459562); the twin cell's, seed 23's first
mapping from the assignment, from exploration 021's run [41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514).
None of those was run again.

[ADR-0245](../adr/0245-one-cell-night-by-night.md) ordered this exploration. On one cell of [H-47](../research/H-47.md)'s twelve, seed 23's
first mapping from the mirrored assignment, A's count under H-47's plain night on A's patterns lies below the night without replay at block
36 beyond twice the paired SE, −4.9 ± 1.09, and every held night run on it since in H-47's layout ends below the night without replay there
([exploration 029](029-the-part-held-within-two-parts-at-a-time.md)). No run had replayed on some of its nights and not on others. This note
replays H-47's plain night on one night alone, and on every night but one, each form beside the night without replay, on that cell alone. It
asks, at blocks 6, 12, 18, 24 and 36:

1. A's count and each stimulus's correct trials under each form: less the night without replay, with H-47's paired SE, the forms run in
   one job; and less H-47's plain night, with [exploration 018](018-what-nights-keep.md)'s unpaired SE, where a form's rows are H-47's own
   read as H-47's. At block 36, the four single nights' differences from the night without replay summed, and the plain night's four
   differences from the forms leaving a night out summed, beside the plain night less the night without replay;
2. the side under each form, by stimulus and by group of units, against H-47's side;
3. the pair under each form and A's highest coupling, as H-47's clauses 2 and 3 read them;
4. what each night and each stretch of blocks after it moved, from the class matrix by group, beside H-47's plain night and night
   without replay on this cell and on the twin cell, with C's own and D's own units onto A's answer night by night and stretch by stretch;
5. what each replayed night held, from exploration 020's `ReplayWatch`: its ripples' spikes by class and its pairings by class, beside
   H-47's plain night's same night on this cell and on the twin cell;
6. the engine's two sums over each night, its replays and depotentiations, the watch's wrong answers, and the ties, under each form.

**A checkpoint is read after the night that follows its block**, and block 36's after the twelve blocks that follow the last night. So
at the checkpoint right after a form's one replayed night, that form less the night without replay is that night's replay alone: until
then the two ran the same nights on the same images. On A's count a stretch of blocks is read together with the night that ends it, and
only the class matrix, read in the image before each night and in the image after it, reads the two apart.

**The side** is [ADR-0238](../adr/0238-what-keeps-a-mapping-where-the-replay-did-not.md)'s: at a checkpoint, the stimulus whose
difference from the night without replay is the larger of the two, and neither where they are equal. The note writes no prediction and
decides nothing. **Every figure here is exploratory**: from one run of each test on the repository's AArch64 host, in H-47's terms. A
count is a mean over the cell's 48 reads, sixteen on each of three paths, and a difference carries its paired SE unless another is named.

**In brief**
- **Each form ran H-47's rows until it parted from H-47's forms, and none after.** Inside the test, every row a form read while its nights
  so far were all H-47's night without replay, or all its plain night, held H-47's row of that form, the night without replay's fifteen
  among them, and every night that replays read replays and depotentiations while every other read none. The trial held each form's first
  night to H-47's row at block 6 on path 0. The forms that share their first two nights read the same rows and reads in the two jobs.
- **One night's replay alone, right after it**, less the night without replay: after block 6, +1.9 ± 0.68, and after block 12,
  +1.6 ± 0.55, above beyond twice the paired SE; after block 18, +0.6 ± 0.88, and after block 24, −0.3 ± 0.81, within 2 SE.
- **At block 36, less the night without replay**: after block 6 alone +1.7 ± 0.78, above beyond 2 SE; after block 24 alone −2.4 ± 0.86,
  below beyond 2 SE; after block 12 or 18 alone −0.5 ± 1.11 and +0.3 ± 0.83, within. All but after block 6 −1.7 ± 0.54 and all but after
  block 18 −2.6 ± 1.10, below beyond 2 SE; all but after block 12 −2.5 ± 1.66 and all but after block 24 −1.3 ± 1.40, within. H-47's plain
  night read −4.9 ± 1.09.
- **Less H-47's plain night at block 36**, every form but after block 24 alone lies above beyond twice exploration 018's unpaired SE,
  +2.3 ± 0.64 to +6.6 ± 0.90; after block 24 alone reads +2.5 ± 1.31, within 2 SE.
- **Summed at block 36**, the four single nights' differences from the night without replay read −0.9 and the plain night's four
  differences from the forms leaving a night out −11.5, beside the plain night less the night without replay, −4.9 ± 1.09; no SE is
  claimed for either sum.
- **On stimulus A's trials at block 36** each form leaving a night out lies below the night without replay beyond 2 SE, −2.7 ± 0.90 to
  −4.0 ± 1.39, where H-47's plain night read −5.2 ± 1.50; of the single nights only after block 24 alone does, −1.1 ± 0.24.
- **The side at block 36** lies on B, the odd places, under seven of the eight forms, as under H-47's plain night; under after block 24
  alone it lies on A, the even places, within 2 SE, both stimuli's trials reading below the night without replay.
- **Each form's first night that replays moved the readouts' synapses about as H-47's first plain night did, whichever night it was**:
  the odd places' excitatory units lost 3.35 to 3.99 M, the inhibition onto the even places' excitatory units, the group A answers, rose
  3.27 to 3.56 M, A's margin rose 1.03 to 1.21 M, with 1 339 to 1 341 replays a path. A replayed night after an earlier one moved
  less: −1.53 to −3.00 M, +1.13 to +2.51 M and +0.54 to +1.01 M, with 1 129 to 1 268 replays.
- **C's own units lost margin onto A's answer over every night and every stretch of blocks after the first night, under every form, on
  every path**; over the four nights 0.16 M under the night without replay, 0.19 to 0.28 M under the single nights, 0.29 to 0.36 M under
  the forms leaving a night out and 0.34 M under H-47's plain night. On the twin each of H-47's plain night's nights raised it.
- **The pair** stands at 62.96 to 63.54 at block 36 under every form, and A's highest coupling at 1.082 to 1.147 of H-42's.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 020 to 029 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. Nights follow blocks 6, 12, 18 and 24, then 12 blocks run
with none to block 36, on paths 0, 5 and 6. Each checkpoint reads A and the pair sixteen times each.

**Places.** A period of the ring is 20 units. Stimulus A presents place 0 of each period and B place 11. **The odd places** are the odd
places but 11 and **the even places** the even places but 0: H-47's readout 0 and readout 1, each with its excitatory and its inhibitory
units. C's own units lie at place 5, among the odd places, and D's at place 16, among the even
([exploration 021](021-where-the-gain-falls.md), section 4).

**The cell** is seed 23's first mapping from the mirrored assignment, `zero23 MirroredFirst FirstMapping` in the logs. Its mapping in force
is the mirrored one: stimulus A answers readout 1, the even places, and B readout 0, the odd places; C answers readout 0 and D readout 1.
**Its twin** is seed 23's first mapping from the assignment, where A answers readout 0, the odd places
([exploration 023](023-the-readouts-index-or-its-group.md)); it is read from exploration 021's job and run here not at all.

**A night that replays** is H-47's plain night on A's patterns: A's 102 patterns, drawn from the start image, tagged at priority 200.
**Every other night** is H-47's night without replay, the same night with nothing tagged. No hold is set. Each night is chosen by its form
(`E030Form`), and every night of every form is watched. **Ten forms are read**: the night without replay and eight forms
run here, and H-47's plain night read from its logs, each with its name in the tables and in the logs:

| Form | In the logs | Night after block 6 | after 12 | after 18 | after 24 | Job | Rows held to H-47's, on every path |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Without replay | `no-replay` | without replay | without replay | without replay | without replay | both | all fifteen, the night without replay's |
| After block 6 alone | `alone-6` | replays | without replay | without replay | without replay | one night alone | block 6, the plain night's |
| After block 12 alone | `alone-12` | without replay | replays | without replay | without replay | one night alone | block 6, the night without replay's |
| After block 18 alone | `alone-18` | without replay | without replay | replays | without replay | one night alone | blocks 6 and 12, the night without replay's |
| After block 24 alone | `alone-24` | without replay | without replay | without replay | replays | one night alone | blocks 6, 12 and 18, the night without replay's |
| All but after block 6 | `but-6` | without replay | replays | replays | replays | one night left out | block 6, the night without replay's |
| All but after block 12 | `but-12` | replays | without replay | replays | replays | one night left out | block 6, the plain night's |
| All but after block 18 | `but-18` | replays | replays | without replay | replays | one night left out | blocks 6 and 12, the plain night's |
| All but after block 24 | `but-24` | replays | replays | replays | without replay | one night left out | blocks 6, 12 and 18, the plain night's |
| Plain on A's (H-47) | `plain-first` | replays | replays | replays | replays | H-47's | H-47's own |

**Two jobs** (`e030_job`), each exploration 020's job (`e020_job`) with its forms' nights chosen night by night: the night without replay
and four forms, on the three paths. The job writes A's patterns and their targets, the split and the class matrix of the start image, of
the image before each night and of the image after it, and of block 36's image; it reads each night with `ReplayWatch`, every pairing of
A's and B's 102 stimulus units tick by tick and, in a ripple that replays one of A's patterns, every spike classed against it. Inside the
test (`e030_night`), each night that replays reads replays and depotentiations that are not zero, and every other night reads `replays 0`
and `depotentiations 0`. Every row a form reads after nights that so far are all H-47's night without replay, or all its plain night, is
held inside the test to H-47's row of that form on the same path and block, as the table above lists; so all fifteen of the night without
replay's are. A row is the image's CRC-64, A's and the pair's sixteen counts, A's couplings, their highest over the stretch before the
checkpoint, and the gain at the night's start and end. Beside each form the job prints how many of its fifteen rows are each of H-47's two
forms' rows.

**Read from the logs, not run again**: H-47's plain night on A's patterns, its rows and reads from H-47's job, and its class matrices, night
lines and watch from exploration 020's job on this cell, which holds all 30 of H-47's rows of its two forms; the twin's plain night and
night without replay from exploration 021's job. Each job here ran its own head, the pair's first six blocks on each path, and the head is
read once.

**The class matrix** is exploration 020's, read by group as exploration 023 reads it. **The odd places' excitatory units** are classes 4, 6
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
form's rows at a checkpoint are H-47's own, that difference is read as H-47 reads it: H-47's night without replay less its plain night with
H-47's paired SE, marked (H-47), or nothing where the rows are the plain night's own (= plain). A count of checkpoints within or beyond
twice an SE is taken on the unrounded mean and SE.

## Before a job ran

ADR-0245 asks whether any form but the night without replay must run bit for bit as one already in the logs. **None must.** On this cell
the logs hold H-47's four forms and the nights of explorations 020 to 029, each of which ran here, in H-47's layout or another, H-47's
night without replay or its plain night, with or without a hold, alike on all four nights: every one of them replays on all four nights or
on none. Each form here replays on one night or on three, so at some night it runs a night of one kind on an image that only a chain of the
other kind wrote:

| Form | Its rows are H-47's up to block | The first night where it parts from H-47's forms | That night runs on |
| :--- | ---: | :--- | :--- |
| After block 6 alone | 6, the plain night's | the night after block 12, without replay | the plain night's image after blocks 7 to 12 |
| After block 12 alone | 6, the night without replay's | the night after block 12, which replays | the night without replay's image after blocks 7 to 12 |
| After block 18 alone | 12, the night without replay's | the night after block 18, which replays | the night without replay's image after blocks 13 to 18 |
| After block 24 alone | 18, the night without replay's | the night after block 24, which replays | the night without replay's image after blocks 19 to 24 |
| All but after block 6 | 6, the night without replay's | the night after block 12, which replays | the night without replay's image after blocks 7 to 12 |
| All but after block 12 | 6, the plain night's | the night after block 12, without replay | the plain night's image after blocks 7 to 12 |
| All but after block 18 | 12, the plain night's | the night after block 18, without replay | the plain night's image after blocks 13 to 18 |
| All but after block 24 | 18, the plain night's | the night after block 24, without replay | the plain night's image after blocks 19 to 24 |

At block 6 the two kinds of night, run on the same head, read different rows on every path in H-47's job: the image's CRC-64 differs on
each of the three. No log holds a night of either kind on an image the other kind's chain wrote, so no form's rows after its first parting
are in the logs, and all eight forms ran.

Two pairs of forms share their first two nights across the two jobs here: after block 6 alone and all but after block 12 both replay on
the night after block 6 and not on the night after block 12, and after block 12 alone and all but after block 6 the other way. Their rows
at blocks 6 and 12 must be the same in the two jobs; neither job holds the other's, and section "The runs are laid out as written" sets
them side by side.

## The runs are laid out as written

- **The trial** ran on path 0 the pair's head and each form's first night, all nine watched, each image split and read once as it was, of
  A and of the pair. Each image, those two reads, A's couplings, their highest over the head and the gain held, inside the test, H-47's
  row at block 6 on path 0 of the form whose first night it is:

| Form | Its first night | H-47's row at block 6 on path 0 it holds | Image's CRC-64 | A's read as it was | The pair's | A's couplings | Gain over the night | Replays | Depotentiations | Watch: wrong |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Without replay | without replay | the night without replay's | 16452926217147532021 | 57 | 46 | 6 057 644, 6 705 699, 6 936 603, 6 309 843 | 112 166 to 114 098 | 0 | 0 | 0 |
| After block 6 alone | replays | the plain night's | 3409025272862731850 | 57 | 46 | 5 820 629, 6 942 733, 7 247 189, 5 999 268 | 112 166 to 107 803 | 1 339 | 640 | 0 |
| After block 12 alone | without replay | the night without replay's | 16452926217147532021 | 57 | 46 | 6 057 644, 6 705 699, 6 936 603, 6 309 843 | 112 166 to 114 098 | 0 | 0 | 0 |
| After block 18 alone | without replay | the night without replay's | 16452926217147532021 | 57 | 46 | 6 057 644, 6 705 699, 6 936 603, 6 309 843 | 112 166 to 114 098 | 0 | 0 | 0 |
| After block 24 alone | without replay | the night without replay's | 16452926217147532021 | 57 | 46 | 6 057 644, 6 705 699, 6 936 603, 6 309 843 | 112 166 to 114 098 | 0 | 0 | 0 |
| All but after block 6 | without replay | the night without replay's | 16452926217147532021 | 57 | 46 | 6 057 644, 6 705 699, 6 936 603, 6 309 843 | 112 166 to 114 098 | 0 | 0 | 0 |
| All but after block 12 | replays | the plain night's | 3409025272862731850 | 57 | 46 | 5 820 629, 6 942 733, 7 247 189, 5 999 268 | 112 166 to 107 803 | 1 339 | 640 | 0 |
| All but after block 18 | replays | the plain night's | 3409025272862731850 | 57 | 46 | 5 820 629, 6 942 733, 7 247 189, 5 999 268 | 112 166 to 107 803 | 1 339 | 640 | 0 |
| All but after block 24 | replays | the plain night's | 3409025272862731850 | 57 | 46 | 5 820 629, 6 942 733, 7 247 189, 5 999 268 | 112 166 to 107 803 | 1 339 | 640 | 0 |

- Each of the nine nights' lines, its start, its windows, its watch and its end, are the lines of exploration 020's job's night of the
  same kind after block 6 on path 0, the label and the form's name aside: 91 of 91 for each of the five nights without replay and 401 of
  401 for each of the four that replay. So is the head's line. Each night that replays read 1 339 replays and 640 depotentiations and
  each other night none, held inside the test, and the watch read `wrong 0` in all nine.
- **Each of the two jobs** ran to the end, its five forms on the three paths. Inside the test, every row a form read while its nights so
  far were all H-47's night without replay, or all its plain night, held H-47's row of that form on the same path and block, as the table
  of What ran lists: all fifteen of the night without replay's in each job; the three at block 6 of after block 6 or 12 alone and of all
  but after block 6 or 12; the six at blocks 6 and 12 of after block 18 alone and all but after block 18; and the nine at blocks 6, 12
  and 18 of after block 24 alone and all but after block 24. Beside each form the job prints how many of its fifteen rows are
  H-47's rows of either form: those rows and no other, as the job prints. No row after a form's first parting is either of H-47's forms'
  rows.
- **The night without replay's lines** are the same 1 707 lines in the two jobs. Of them 1 695 are the lines of exploration 020's job's
  night without replay on this cell, label aside, all of that job's; the other 12 are the job's notices that each night read what its
  form asks. The pair's heads, three lines a job, are exploration 020's.
- **The nights held to H-47's.** Each night of a form before its first parting, seven nights without replay and seven plain nights in
  all, prints on its three paths the lines of exploration 020's job's night of the same kind after the same block, the label and the form's
  name aside: 273 of 273 for each night without replay, and 1 214 of 1 214, 1 188 of 1 188 and 1 194 of 1 194 for the plain nights after
  blocks 6, 12 and 18. Of the 18 nights after a form's first parting, none ends, on any path, on the end line of exploration 020's night
  without replay or plain night after the same block.
- **Across the two jobs**, the forms that share their first two nights read the same rows at blocks 6 and 12, 6 of 6, and the same 192
  reads, each read's count, masks and trace: after block 6 alone and all but after block 12, and after block 12 alone and all but after
  block 6. At block 6, after block 6 alone and all but after block 18, and after block 6 alone and all but after block 24, read the same
  3 rows and 96 reads.
- **Every read's `stimuli` mask** is that of H-47's plain night's read of the same path, checkpoint, mapping and stream, in all 4 800 reads
  of the two jobs, so each trial presented the stimulus it presented in H-47.
- **The nights.** The 48 nights that replay read 1 194 to 1 341 replays and 571 to 640 depotentiations, and the 72 others `replays 0` and
  `depotentiations 0`, held inside the test. The watch read `wrong 0` in all 120, over 25 827 424 pairings.

## 1. A's count and each stimulus's trials under each form

A's count at each checkpoint, the mean of 48 reads. The night without replay is this exploration's, its rows H-47's; H-47's plain night is
H-47's job's. R marks a night that replays:

| Form | Nights after 6, 12, 18, 24 | Block 6 | 12 | 18 | 24 | 36 |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| Without replay | – – – – | 55.0 | 52.4 | 52.0 | 53.5 | 53.1 |
| After block 6 alone | R – – – | 56.9 | 53.1 | 51.2 | 54.5 | 54.8 |
| After block 12 alone | – R – – | 55.0 | 54.0 | 52.9 | 54.0 | 52.7 |
| After block 18 alone | – – R – | 55.0 | 52.4 | 52.6 | 53.8 | 53.4 |
| After block 24 alone | – – – R | 55.0 | 52.4 | 52.0 | 53.2 | 50.8 |
| All but after block 6 | – R R R | 55.0 | 54.0 | 53.7 | 52.8 | 51.4 |
| All but after block 12 | R – R R | 56.9 | 53.1 | 51.9 | 52.0 | 50.7 |
| All but after block 18 | R R – R | 56.9 | 54.4 | 50.4 | 53.4 | 50.5 |
| All but after block 24 | R R R – | 56.9 | 54.4 | 52.4 | 52.3 | 51.9 |
| Plain on A's (H-47) | R R R R | 56.9 | 54.4 | 52.4 | 50.9 | 48.3 |

- At block 36 the eight forms read 50.5, all but after block 18, to 54.8, after block 6 alone; the night without replay 53.1 and H-47's
  plain night 48.3.

Less the night without replay of the form's own job, ± the paired SE; "= without replay" marks a checkpoint where the form's rows are the
night without replay's own. H-47's plain night is H-47's job's, less its own night without replay:

| Form | Block 6 | 12 | 18 | 24 | 36 |
| :--- | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone | +1.9 ± 0.68 | +0.8 ± 0.44 | −0.8 ± 1.51 | +1.0 ± 1.08 | +1.7 ± 0.78 |
| After block 12 alone | = without replay | +1.6 ± 0.55 | +0.9 ± 0.76 | +0.5 ± 0.42 | −0.5 ± 1.11 |
| After block 18 alone | = without replay | = without replay | +0.6 ± 0.88 | +0.4 ± 0.45 | +0.3 ± 0.83 |
| After block 24 alone | = without replay | = without replay | = without replay | −0.3 ± 0.81 | −2.4 ± 0.86 |
| All but after block 6 | = without replay | +1.6 ± 0.55 | +1.7 ± 1.18 | −0.7 ± 0.91 | −1.7 ± 0.54 |
| All but after block 12 | +1.9 ± 0.68 | +0.8 ± 0.44 | −0.1 ± 1.64 | −1.4 ± 1.32 | −2.5 ± 1.66 |
| All but after block 18 | +1.9 ± 0.68 | +2.0 ± 1.26 | −1.6 ± 1.49 | 0.0 ± 1.40 | −2.6 ± 1.10 |
| All but after block 24 | +1.9 ± 0.68 | +2.0 ± 1.26 | +0.3 ± 1.17 | −1.2 ± 0.64 | −1.3 ± 1.40 |
| Plain on A's (H-47) | +1.9 ± 0.68 (H-47) | +2.0 ± 1.26 (H-47) | +0.3 ± 1.17 (H-47) | −2.6 ± 1.30 (H-47) | −4.9 ± 1.09 (H-47) |

- **Right after a form's one replayed night**, where that form less the night without replay is that night's replay alone: after block 6
  alone at block 6, +1.9 ± 0.68, H-47's plain night's own row, and after block 12 alone at block 12, +1.6 ± 0.55, lie above beyond twice
  the SE; after block 18 alone at block 18, +0.6 ± 0.88, and after block 24 alone at block 24, −0.3 ± 0.81, lie within 2 SE.
- **At block 36** after block 6 alone lies above beyond 2 SE, +1.7 ± 0.78, and after block 24 alone below, −2.4 ± 0.86; after block 12
  alone, −0.5 ± 1.11, and after block 18 alone, +0.3 ± 0.83, lie within. Of the forms leaving a night out, all but after block 6,
  −1.7 ± 0.54, and all but after block 18, −2.6 ± 1.10, lie below beyond 2 SE; all but after block 12, −2.5 ± 1.66, and all but after
  block 24, −1.3 ± 1.40, within. H-47's plain night read −4.9 ± 1.09.
- **At blocks 12, 18 and 24 no form lies below beyond 2 SE.** Above beyond 2 SE lie, at block 6, the four forms whose first night
  replays, on H-47's plain night's row, and at block 12 after block 12 alone and all but after block 6, +1.6 ± 0.55 each, on the same
  rows.

Less H-47's plain night, ± exploration 018's unpaired SE; (H-47) marks a checkpoint where the form's rows are H-47's night without
replay's, read with H-47's paired SE, and "= plain" one where they are the plain night's own:

| Form | Block 6 | 12 | 18 | 24 | 36 |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Without replay | −1.9 ± 0.68 (H-47) | −2.0 ± 1.26 (H-47) | −0.3 ± 1.17 (H-47) | +2.6 ± 1.30 (H-47) | +4.9 ± 1.09 (H-47) |
| After block 6 alone | = plain | −1.2 ± 1.03 | −1.1 ± 0.84 | +3.6 ± 0.97 | +6.6 ± 0.90 |
| After block 12 alone | −1.9 ± 0.68 (H-47) | −0.4 ± 1.08 | +0.6 ± 0.90 | +3.1 ± 1.35 | +4.4 ± 0.59 |
| After block 18 alone | −1.9 ± 0.68 (H-47) | −2.0 ± 1.26 (H-47) | +0.3 ± 1.01 | +2.9 ± 1.51 | +5.2 ± 1.22 |
| After block 24 alone | −1.9 ± 0.68 (H-47) | −2.0 ± 1.26 (H-47) | −0.3 ± 1.17 (H-47) | +2.3 ± 1.81 | +2.5 ± 1.31 |
| All but after block 6 | −1.9 ± 0.68 (H-47) | −0.4 ± 1.08 | +1.4 ± 1.07 | +1.8 ± 1.24 | +3.2 ± 1.05 |
| All but after block 12 | = plain | −1.2 ± 1.03 | −0.4 ± 0.93 | +1.1 ± 0.94 | +2.4 ± 1.02 |
| All but after block 18 | = plain | = plain | −1.9 ± 0.98 | +2.5 ± 1.17 | +2.3 ± 0.64 |
| All but after block 24 | = plain | = plain | = plain | +1.3 ± 1.12 | +3.6 ± 0.71 |

- **At block 36 every form but after block 24 alone lies above H-47's plain night beyond twice the unpaired SE**: after block 6, 12 or 18
  alone +6.6 ± 0.90, +4.4 ± 0.59 and +5.2 ± 1.22, and all but after block 6, 12, 18 or 24 +3.2 ± 1.05, +2.4 ± 1.02, +2.3 ± 0.64 and
  +3.6 ± 0.71. After block 24 alone reads +2.5 ± 1.31, within 2 SE.
- **Right after the one night a form leaves out**, where that form less the plain night is that night's replay left out, with the
  unpaired SE: all but after block 12 at block 12, −1.2 ± 1.03, all but after block 18 at block 18, −1.9 ± 0.98, and all but after
  block 24 at block 24, +1.3 ± 1.12, lie within 2 SE; all but after block 6 at block 6 is H-47's night without replay, −1.9 ± 0.68 with
  H-47's paired SE.
- **At block 24** after block 6 alone, +3.6 ± 0.97, after block 12 alone, +3.1 ± 1.35, and all but after block 18, +2.5 ± 1.17, lie above
  beyond 2 SE, and the other five within. At blocks 12 and 18 every difference that is not H-47's own lies within 2 SE.

Each stimulus's correct trials, of the trials a read presents of it, and the ties of both stimuli's trials, the mean of 48 reads; then
each stimulus's trials less the night without replay, ± the paired SE, and less H-47's plain night, ± the unpaired SE, marked as above:

| Form | Block | Correct on A's trials | on B's trials | Ties | A's trials less without replay | B's trials less without replay | A's trials less plain | B's trials less plain |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Without replay | 6 | 30.4 | 24.6 | 3.0 | – | – | −0.6 ± 0.31 (H-47) | −1.3 ± 0.56 (H-47) |
| Without replay | 12 | 28.5 | 23.8 | 4.1 | – | – | −1.1 ± 0.70 (H-47) | −1.0 ± 1.17 (H-47) |
| Without replay | 18 | 28.5 | 23.5 | 3.4 | – | – | +1.0 ± 0.92 (H-47) | −1.3 ± 0.69 (H-47) |
| Without replay | 24 | 30.0 | 23.5 | 3.0 | – | – | +5.0 ± 1.25 (H-47) | −2.4 ± 0.38 (H-47) |
| Without replay | 36 | 29.3 | 23.9 | 3.0 | – | – | +5.2 ± 1.50 (H-47) | −0.3 ± 0.88 (H-47) |
| After block 6 alone | 6 | 31.0 | 25.9 | 2.4 | +0.6 ± 0.31 | +1.3 ± 0.56 | = plain | = plain |
| After block 6 alone | 12 | 28.5 | 24.7 | 3.3 | 0.0 ± 0.56 | +0.8 ± 0.61 | −1.1 ± 0.96 | −0.1 ± 1.00 |
| After block 6 alone | 18 | 27.3 | 23.9 | 3.6 | −1.3 ± 0.74 | +0.5 ± 0.89 | −0.3 ± 1.17 | −0.8 ± 1.20 |
| After block 6 alone | 24 | 29.5 | 25.0 | 2.5 | −0.5 ± 0.42 | +1.5 ± 0.99 | +4.5 ± 1.25 | −0.9 ± 1.37 |
| After block 6 alone | 36 | 29.3 | 25.6 | 2.9 | 0.0 ± 0.56 | +1.7 ± 0.40 | +5.2 ± 1.42 | +1.4 ± 1.31 |
| After block 12 alone | 6 | 30.4 | 24.6 | 3.0 | = without replay | = without replay | −0.6 ± 0.31 (H-47) | −1.3 ± 0.56 (H-47) |
| After block 12 alone | 12 | 29.8 | 24.2 | 3.8 | +1.3 ± 0.44 | +0.3 ± 0.43 | +0.3 ± 1.02 | −0.6 ± 1.24 |
| After block 12 alone | 18 | 28.1 | 24.8 | 3.2 | −0.5 ± 0.45 | +1.4 ± 0.60 | +0.5 ± 1.22 | +0.1 ± 0.96 |
| After block 12 alone | 24 | 29.3 | 24.7 | 2.5 | −0.8 ± 0.53 | +1.3 ± 0.38 | +4.2 ± 1.06 | −1.2 ± 1.15 |
| After block 12 alone | 36 | 27.5 | 25.2 | 3.9 | −1.8 ± 1.04 | +1.3 ± 0.58 | +3.4 ± 0.96 | +1.0 ± 0.94 |
| After block 18 alone | 6 | 30.4 | 24.6 | 3.0 | = without replay | = without replay | −0.6 ± 0.31 (H-47) | −1.3 ± 0.56 (H-47) |
| After block 18 alone | 12 | 28.5 | 23.8 | 4.1 | = without replay | = without replay | −1.1 ± 0.70 (H-47) | −1.0 ± 1.17 (H-47) |
| After block 18 alone | 18 | 28.5 | 24.2 | 3.9 | −0.1 ± 0.42 | +0.7 ± 0.78 | +0.9 ± 1.26 | −0.6 ± 1.12 |
| After block 18 alone | 24 | 29.1 | 24.8 | 3.0 | −1.0 ± 0.34 | +1.3 ± 0.28 | +4.0 ± 1.34 | −1.1 ± 1.22 |
| After block 18 alone | 36 | 28.0 | 25.4 | 3.8 | −1.3 ± 0.67 | +1.5 ± 0.76 | +3.9 ± 1.35 | +1.2 ± 0.92 |
| After block 24 alone | 6 | 30.4 | 24.6 | 3.0 | = without replay | = without replay | −0.6 ± 0.31 (H-47) | −1.3 ± 0.56 (H-47) |
| After block 24 alone | 12 | 28.5 | 23.8 | 4.1 | = without replay | = without replay | −1.1 ± 0.70 (H-47) | −1.0 ± 1.17 (H-47) |
| After block 24 alone | 18 | 28.5 | 23.5 | 3.4 | = without replay | = without replay | +1.0 ± 0.92 (H-47) | −1.3 ± 0.69 (H-47) |
| After block 24 alone | 24 | 29.1 | 24.0 | 3.5 | −0.9 ± 0.39 | +0.6 ± 0.73 | +4.1 ± 1.55 | −1.9 ± 1.18 |
| After block 24 alone | 36 | 28.2 | 22.6 | 3.8 | −1.1 ± 0.24 | −1.3 ± 0.96 | +4.1 ± 1.69 | −1.6 ± 1.16 |
| All but after block 6 | 6 | 30.4 | 24.6 | 3.0 | = without replay | = without replay | −0.6 ± 0.31 (H-47) | −1.3 ± 0.56 (H-47) |
| All but after block 6 | 12 | 29.8 | 24.2 | 3.8 | +1.3 ± 0.44 | +0.3 ± 0.43 | +0.3 ± 1.02 | −0.6 ± 1.24 |
| All but after block 6 | 18 | 28.8 | 24.9 | 4.1 | +0.3 ± 0.69 | +1.4 ± 0.78 | +1.2 ± 1.19 | +0.1 ± 0.99 |
| All but after block 6 | 24 | 27.2 | 25.5 | 3.4 | −2.8 ± 0.85 | +2.1 ± 0.54 | +2.2 ± 1.11 | −0.4 ± 1.14 |
| All but after block 6 | 36 | 26.6 | 24.9 | 3.6 | −2.7 ± 0.90 | +1.0 ± 1.19 | +2.5 ± 1.22 | +0.7 ± 0.92 |
| All but after block 12 | 6 | 31.0 | 25.9 | 2.4 | +0.6 ± 0.31 | +1.3 ± 0.56 | = plain | = plain |
| All but after block 12 | 12 | 28.5 | 24.7 | 3.3 | 0.0 ± 0.56 | +0.8 ± 0.61 | −1.1 ± 0.96 | −0.1 ± 1.00 |
| All but after block 12 | 18 | 27.8 | 24.1 | 4.1 | −0.7 ± 0.76 | +0.6 ± 1.10 | +0.2 ± 1.11 | −0.7 ± 1.32 |
| All but after block 12 | 24 | 27.1 | 24.9 | 3.9 | −2.9 ± 1.12 | +1.4 ± 0.50 | +2.1 ± 0.87 | −1.0 ± 1.09 |
| All but after block 12 | 36 | 25.5 | 25.1 | 3.9 | −3.7 ± 1.55 | +1.3 ± 1.13 | +1.5 ± 0.84 | +1.0 ± 1.12 |
| All but after block 18 | 6 | 31.0 | 25.9 | 2.4 | +0.6 ± 0.31 | +1.3 ± 0.56 | = plain | = plain |
| All but after block 18 | 12 | 29.6 | 24.8 | 3.5 | +1.1 ± 0.70 | +1.0 ± 1.17 | = plain | = plain |
| All but after block 18 | 18 | 26.3 | 24.1 | 3.3 | −2.3 ± 1.09 | +0.7 ± 0.78 | −1.3 ± 1.12 | −0.6 ± 1.02 |
| All but after block 18 | 24 | 27.0 | 26.4 | 3.3 | −3.0 ± 1.21 | +3.0 ± 0.55 | +2.0 ± 0.99 | +0.5 ± 1.18 |
| All but after block 18 | 36 | 25.7 | 24.8 | 4.3 | −3.5 ± 1.38 | +0.9 ± 0.60 | +1.6 ± 0.87 | +0.6 ± 0.85 |
| All but after block 24 | 6 | 31.0 | 25.9 | 2.4 | +0.6 ± 0.31 | +1.3 ± 0.56 | = plain | = plain |
| All but after block 24 | 12 | 29.6 | 24.8 | 3.5 | +1.1 ± 0.70 | +1.0 ± 1.17 | = plain | = plain |
| All but after block 24 | 18 | 27.6 | 24.8 | 3.8 | −1.0 ± 0.92 | +1.3 ± 0.69 | = plain | = plain |
| All but after block 24 | 24 | 26.2 | 26.1 | 3.1 | −3.9 ± 0.90 | +2.6 ± 0.60 | +1.1 ± 0.99 | +0.2 ± 1.13 |
| All but after block 24 | 36 | 25.3 | 26.6 | 3.7 | −4.0 ± 1.39 | +2.7 ± 0.95 | +1.2 ± 0.81 | +2.4 ± 0.85 |
| Plain on A's (H-47) | 6 | 31.0 | 25.9 | 2.4 | +0.6 ± 0.31 (H-47) | +1.3 ± 0.56 (H-47) | – | – |
| Plain on A's (H-47) | 12 | 29.6 | 24.8 | 3.5 | +1.1 ± 0.70 (H-47) | +1.0 ± 1.17 (H-47) | – | – |
| Plain on A's (H-47) | 18 | 27.6 | 24.8 | 3.8 | −1.0 ± 0.92 (H-47) | +1.3 ± 0.69 (H-47) | – | – |
| Plain on A's (H-47) | 24 | 25.0 | 25.9 | 3.5 | −5.0 ± 1.25 (H-47) | +2.4 ± 0.38 (H-47) | – | – |
| Plain on A's (H-47) | 36 | 24.1 | 24.2 | 4.7 | −5.2 ± 1.50 (H-47) | +0.3 ± 0.88 (H-47) | – | – |

- **On stimulus A's trials at block 36 each form leaving a night out lies below the night without replay beyond 2 SE**: all but after
  block 6, 12, 18 or 24 −2.7 ± 0.90, −3.7 ± 1.55, −3.5 ± 1.38 and −4.0 ± 1.39, where H-47's plain night read −5.2 ± 1.50. Of the single
  nights only after block 24 alone does, −1.1 ± 0.24; after block 6 alone reads 0.0 ± 0.56, after block 12 alone −1.8 ± 1.04 and after
  block 18 alone −1.3 ± 0.67, within 2 SE.
- **On A's trials at block 24** the four forms leaving a night out lie below beyond 2 SE, −2.8 ± 0.85 to −3.9 ± 0.90, and so do after
  block 18 alone, −1.0 ± 0.34, and after block 24 alone, −0.9 ± 0.39; H-47's plain night read −5.0 ± 1.25.
- **On B's trials at block 36** after block 6 alone, +1.7 ± 0.40, after block 12 alone, +1.3 ± 0.58, and all but after block 24,
  +2.7 ± 0.95, lie above beyond 2 SE. The other five lie within, after block 18 alone at +1.5 ± 0.76 by less than 0.01 of twice its SE,
  and after block 24 alone at −1.3 ± 0.96. H-47's plain night read +0.3 ± 0.88.
- **Less H-47's plain night at block 36**, on A's trials the four single nights lie above beyond 2 SE, +3.4 ± 0.96 to +5.2 ± 1.42, and
  all but after block 6, +2.5 ± 1.22; the other three forms leaving a night out lie within, +1.2 ± 0.81 to +1.6 ± 0.87. On B's trials
  only all but after block 24 lies beyond 2 SE, above, +2.4 ± 0.85.

At block 36, each single night's difference from the night without replay, and the plain night's difference from each form leaving a
night out, each the difference of the two means of 48, with their sums, signed, beside H-47's plain night less the night without replay
± its paired SE. No SE is claimed for either sum:

| Of | After block 6 alone less without replay | after 12 alone | after 18 alone | after 24 alone | Their sum | Plain less all but after block 6 | less all but after 12 | less all but after 18 | less all but after 24 | Their sum | H-47: plain less without replay |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| A's count | +1.7 | −0.5 | +0.3 | −2.4 | −0.9 | −3.2 | −2.4 | −2.3 | −3.6 | −11.5 | −4.9 ± 1.09 |
| A's trials | 0.0 | −1.8 | −1.3 | −1.1 | −4.1 | −2.5 | −1.5 | −1.6 | −1.2 | −6.9 | −5.2 ± 1.50 |
| B's trials | +1.7 | +1.3 | +1.5 | −1.3 | +3.2 | −0.7 | −1.0 | −0.6 | −2.4 | −4.6 | +0.3 ± 0.88 |

- **On A's count** the four single nights' differences, +1.7, −0.5, +0.3 and −2.4, sum to −0.9, and the plain night's differences from
  the four forms leaving a night out, −3.2, −2.4, −2.3 and −3.6, to −11.5, beside H-47's −4.9 ± 1.09.
- **On A's trials** they sum to −4.1 and −6.9, beside −5.2 ± 1.50; **on B's** to +3.2 and −4.6, beside +0.3 ± 0.88.

## 2. The side under each form

Each stimulus's trials less the night without replay of the form's own job, and A's less B's, ± their paired SEs, with the side: the
stimulus whose difference is the larger, and its group of units, A's trials answering the even places and B's the odd places. H-47's
plain night is H-47's job's:

| Form | Block | A's trials less without replay | B's trials less without replay | A's less B's | Side |
| :--- | ---: | ---: | ---: | ---: | :--- |
| After block 6 alone | 6 | +0.6 ± 0.31 | +1.3 ± 0.56 | −0.7 ± 0.61 | B, the odd places (within 2 SE) |
| After block 6 alone | 12 | 0.0 ± 0.56 | +0.8 ± 0.61 | −0.9 ± 1.08 | B, the odd places (within 2 SE) |
| After block 6 alone | 18 | −1.3 ± 0.74 | +0.5 ± 0.89 | −1.7 ± 0.63 | B, the odd places |
| After block 6 alone | 24 | −0.5 ± 0.42 | +1.5 ± 0.99 | −2.0 ± 1.07 | B, the odd places (within 2 SE) |
| After block 6 alone | 36 | 0.0 ± 0.56 | +1.7 ± 0.40 | −1.6 ± 0.60 | B, the odd places |
| After block 12 alone | 6 | = without replay |  |  | – |
| After block 12 alone | 12 | +1.3 ± 0.44 | +0.3 ± 0.43 | +1.0 ± 0.69 | A, the even places (within 2 SE) |
| After block 12 alone | 18 | −0.5 ± 0.45 | +1.4 ± 0.60 | −1.8 ± 0.74 | B, the odd places |
| After block 12 alone | 24 | −0.8 ± 0.53 | +1.3 ± 0.38 | −2.0 ± 0.82 | B, the odd places |
| After block 12 alone | 36 | −1.8 ± 1.04 | +1.3 ± 0.58 | −3.1 ± 1.27 | B, the odd places |
| After block 18 alone | 6 | = without replay |  |  | – |
| After block 18 alone | 12 | = without replay |  |  | – |
| After block 18 alone | 18 | −0.1 ± 0.42 | +0.7 ± 0.78 | −0.8 ± 0.89 | B, the odd places (within 2 SE) |
| After block 18 alone | 24 | −1.0 ± 0.34 | +1.3 ± 0.28 | −2.3 ± 0.43 | B, the odd places |
| After block 18 alone | 36 | −1.3 ± 0.67 | +1.5 ± 0.76 | −2.8 ± 1.14 | B, the odd places |
| After block 24 alone | 6 | = without replay |  |  | – |
| After block 24 alone | 12 | = without replay |  |  | – |
| After block 24 alone | 18 | = without replay |  |  | – |
| After block 24 alone | 24 | −0.9 ± 0.39 | +0.6 ± 0.73 | −1.5 ± 0.85 | B, the odd places (within 2 SE) |
| After block 24 alone | 36 | −1.1 ± 0.24 | −1.3 ± 0.96 | +0.3 ± 1.11 | A, the even places (within 2 SE) |
| All but after block 6 | 6 | = without replay |  |  | – |
| All but after block 6 | 12 | +1.3 ± 0.44 | +0.3 ± 0.43 | +1.0 ± 0.69 | A, the even places (within 2 SE) |
| All but after block 6 | 18 | +0.3 ± 0.69 | +1.4 ± 0.78 | −1.1 ± 0.89 | B, the odd places (within 2 SE) |
| All but after block 6 | 24 | −2.8 ± 0.85 | +2.1 ± 0.54 | −4.9 ± 1.09 | B, the odd places |
| All but after block 6 | 36 | −2.7 ± 0.90 | +1.0 ± 1.19 | −3.6 ± 2.04 | B, the odd places (within 2 SE) |
| All but after block 12 | 6 | +0.6 ± 0.31 | +1.3 ± 0.56 | −0.7 ± 0.61 | B, the odd places (within 2 SE) |
| All but after block 12 | 12 | 0.0 ± 0.56 | +0.8 ± 0.61 | −0.9 ± 1.08 | B, the odd places (within 2 SE) |
| All but after block 12 | 18 | −0.7 ± 0.76 | +0.6 ± 1.10 | −1.4 ± 0.95 | B, the odd places (within 2 SE) |
| All but after block 12 | 24 | −2.9 ± 1.12 | +1.4 ± 0.50 | −4.3 ± 1.12 | B, the odd places |
| All but after block 12 | 36 | −3.7 ± 1.55 | +1.3 ± 1.13 | −5.0 ± 2.15 | B, the odd places |
| All but after block 18 | 6 | +0.6 ± 0.31 | +1.3 ± 0.56 | −0.7 ± 0.61 | B, the odd places (within 2 SE) |
| All but after block 18 | 12 | +1.1 ± 0.70 | +1.0 ± 1.17 | +0.1 ± 1.46 | A, the even places (within 2 SE) |
| All but after block 18 | 18 | −2.3 ± 1.09 | +0.7 ± 0.78 | −2.9 ± 1.18 | B, the odd places |
| All but after block 18 | 24 | −3.0 ± 1.21 | +3.0 ± 0.55 | −6.0 ± 1.24 | B, the odd places |
| All but after block 18 | 36 | −3.5 ± 1.38 | +0.9 ± 0.60 | −4.4 ± 1.83 | B, the odd places |
| All but after block 24 | 6 | +0.6 ± 0.31 | +1.3 ± 0.56 | −0.7 ± 0.61 | B, the odd places (within 2 SE) |
| All but after block 24 | 12 | +1.1 ± 0.70 | +1.0 ± 1.17 | +0.1 ± 1.46 | A, the even places (within 2 SE) |
| All but after block 24 | 18 | −1.0 ± 0.92 | +1.3 ± 0.69 | −2.3 ± 1.12 | B, the odd places |
| All but after block 24 | 24 | −3.9 ± 0.90 | +2.6 ± 0.60 | −6.5 ± 1.40 | B, the odd places |
| All but after block 24 | 36 | −4.0 ± 1.39 | +2.7 ± 0.95 | −6.6 ± 1.92 | B, the odd places |
| Plain on A's (H-47) | 6 | +0.6 ± 0.31 | +1.3 ± 0.56 | −0.7 ± 0.61 | B, the odd places (within 2 SE) |
| Plain on A's (H-47) | 12 | +1.1 ± 0.70 | +1.0 ± 1.17 | +0.1 ± 1.46 | A, the even places (within 2 SE) |
| Plain on A's (H-47) | 18 | −1.0 ± 0.92 | +1.3 ± 0.69 | −2.3 ± 1.12 | B, the odd places |
| Plain on A's (H-47) | 24 | −5.0 ± 1.25 | +2.4 ± 0.38 | −7.4 ± 1.33 | B, the odd places |
| Plain on A's (H-47) | 36 | −5.2 ± 1.50 | +0.3 ± 0.88 | −5.5 ± 2.21 | B, the odd places |

The side, form by form:

| Form | Block 6 | 12 | 18 | 24 | 36 |
| :--- | :--- | :--- | :--- | :--- | :--- |
| After block 6 alone | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places | B, the odd places (within 2 SE) | B, the odd places |
| After block 12 alone | = without replay | A, the even places (within 2 SE) | B, the odd places | B, the odd places | B, the odd places |
| After block 18 alone | = without replay | = without replay | B, the odd places (within 2 SE) | B, the odd places | B, the odd places |
| After block 24 alone | = without replay | = without replay | = without replay | B, the odd places (within 2 SE) | A, the even places (within 2 SE) |
| All but after block 6 | = without replay | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places | B, the odd places (within 2 SE) |
| All but after block 12 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places | B, the odd places |
| All but after block 18 | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places | B, the odd places | B, the odd places |
| All but after block 24 | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places | B, the odd places | B, the odd places |
| Plain on A's (H-47) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places | B, the odd places | B, the odd places |

- **At block 36 the side lies on B, the odd places, under seven of the eight forms**, as under H-47's plain night, beyond twice its SE
  under six: after block 6, 12 or 18 alone and all but after block 12, 18 or 24. Under all but after block 6 it lies there within 2 SE,
  A's less B's −3.6 ± 2.04. Under after block 24 alone it lies on A, the even places, within 2 SE: both stimuli's trials read below the
  night without replay, A's −1.1 ± 0.24 and B's −1.3 ± 0.96, A's less B's +0.3 ± 1.11.
- **At blocks 18 and 24** every form whose rows are not the night without replay's puts the side on B, the odd places, as H-47's plain
  night does: at block 24 beyond 2 SE under each form but two, after block 6 alone and after block 24 alone, where it lies within 2 SE;
  at block 18 beyond 2 SE under after block 6 or 12 alone and all but after block 18 or 24.
- **At block 12**, where H-47's plain night puts it on A, the even places, within 2 SE, so do after block 12 alone and all but after block
  6, 18 or 24, within 2 SE; after block 6 alone and all but after block 12 put it on B, the odd places, within 2 SE. At block 6 each
  form whose first night replays puts it on B within 2 SE, H-47's own reading.

## 3. The pair and A's couplings

The pair's count at each checkpoint, the mean of 48 reads, and A's highest coupling over the same coupling in H-42's image on seed 23, in
any image read and at any block's end of the form's 36, as H-47's clauses 2 and 3 read them:

| Form | The pair at block 6 | 12 | 18 | 24 | 36 | A's highest coupling over H-42's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Without replay | 50.4 | 60.9 | 62.1 | 62.6 | 63.5 | 1.082 |
| After block 6 alone | 49.3 | 59.5 | 62.4 | 63.0 | 63.5 | 1.102 |
| After block 12 alone | 50.4 | 60.8 | 61.0 | 62.3 | 63.3 | 1.087 |
| After block 18 alone | 50.4 | 60.9 | 62.9 | 62.2 | 63.4 | 1.082 |
| After block 24 alone | 50.4 | 60.9 | 62.1 | 63.2 | 63.4 | 1.082 |
| All but after block 6 | 50.4 | 60.8 | 61.1 | 62.6 | 63.0 | 1.140 |
| All but after block 12 | 49.3 | 59.5 | 62.8 | 63.1 | 63.1 | 1.121 |
| All but after block 18 | 49.3 | 59.3 | 62.0 | 62.7 | 63.1 | 1.147 |
| All but after block 24 | 49.3 | 59.3 | 61.9 | 62.7 | 63.1 | 1.137 |
| Plain on A's (H-47) | 49.3 | 59.3 | 61.9 | 62.9 | 63.2 | 1.158 |

- **The pair stands at 62.96 to 63.54 at block 36** under the ten forms read here, the lowest under all but after block 6 and the highest
  under the night without replay, above clause 2's floor of 56 under each.
- **A's highest coupling** reads 1.082 of H-42's under the night without replay and after block 18 or 24 alone, up to 1.147 under all but
  after block 18; H-47's plain night read 1.158. None passes clause 3's 1.30.

## 4. What each night and each stretch moved

What each night moved, from the class matrix in the image before it and the image after it, by group, the mean over the three paths, in M.
The odd places' excitatory units are classes 4, 6 and 7, the even places' 5, 8 and 9; "onto their own" counts each group's excitatory
units onto the same group's; the inhibitory rows are the odd places' class 11 and the even places' class 12 onto every class, and
"inhibitory onto" every inhibitory class, 11 to 13, onto a group's excitatory units, a + more inhibition. Beside this exploration's forms
are H-47's plain night and night without replay on this cell from exploration 020's job, and the twin's from exploration 021's job, where
A answers the odd places:

| Night after block | Form | This night | The odd places' excitatory onto all, M | the even places' | The odd places' onto their own | the even places' onto their own | The odd places' inhibitory onto all | the even places' | Inhibitory onto the odd places' excitatory | onto the even places' |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | Without replay | without replay | −0.16 | −0.29 | −0.17 | −0.23 | −0.61 | −0.58 | −0.37 | −0.46 |
| 6 | After block 6 alone | replays | −3.99 | −3.68 | −0.95 | −0.76 | +4.74 | +4.10 | +3.55 | +3.56 |
| 6 | After block 12 alone | without replay | −0.16 | −0.29 | −0.17 | −0.23 | −0.61 | −0.58 | −0.37 | −0.46 |
| 6 | After block 18 alone | without replay | −0.16 | −0.29 | −0.17 | −0.23 | −0.61 | −0.58 | −0.37 | −0.46 |
| 6 | After block 24 alone | without replay | −0.16 | −0.29 | −0.17 | −0.23 | −0.61 | −0.58 | −0.37 | −0.46 |
| 6 | All but after block 6 | without replay | −0.16 | −0.29 | −0.17 | −0.23 | −0.61 | −0.58 | −0.37 | −0.46 |
| 6 | All but after block 12 | replays | −3.99 | −3.68 | −0.95 | −0.76 | +4.74 | +4.10 | +3.55 | +3.56 |
| 6 | All but after block 18 | replays | −3.99 | −3.68 | −0.95 | −0.76 | +4.74 | +4.10 | +3.55 | +3.56 |
| 6 | All but after block 24 | replays | −3.99 | −3.68 | −0.95 | −0.76 | +4.74 | +4.10 | +3.55 | +3.56 |
| 6 | Plain on A's (H-47, 020's job) | replays | −3.99 | −3.68 | −0.95 | −0.76 | +4.74 | +4.10 | +3.55 | +3.56 |
| 6 | Without replay (H-47, 020's job) | without replay | −0.16 | −0.29 | −0.17 | −0.23 | −0.61 | −0.58 | −0.37 | −0.46 |
| 6 | Twin: plain on A's (021's job) | replays | −3.48 | −3.23 | −0.22 | −0.29 | +5.07 | +4.52 | +4.28 | +3.42 |
| 6 | Twin: without replay (021's job) | without replay | −0.20 | −0.23 | −0.20 | −0.21 | −0.50 | −0.53 | −0.25 | −0.41 |
| 12 | Without replay | without replay | +0.35 | +0.03 | +0.03 | −0.08 | −0.74 | −0.66 | −0.51 | −0.48 |
| 12 | After block 6 alone | without replay | +0.17 | +0.17 | +0.03 | −0.04 | −0.90 | −0.81 | −0.62 | −0.58 |
| 12 | After block 12 alone | replays | −3.65 | −3.43 | −0.84 | −0.72 | +4.61 | +3.88 | +3.33 | +3.44 |
| 12 | After block 18 alone | without replay | +0.35 | +0.03 | +0.03 | −0.08 | −0.74 | −0.66 | −0.51 | −0.48 |
| 12 | After block 24 alone | without replay | +0.35 | +0.03 | +0.03 | −0.08 | −0.74 | −0.66 | −0.51 | −0.48 |
| 12 | All but after block 6 | replays | −3.65 | −3.43 | −0.84 | −0.72 | +4.61 | +3.88 | +3.33 | +3.44 |
| 12 | All but after block 12 | without replay | +0.17 | +0.17 | +0.03 | −0.04 | −0.90 | −0.81 | −0.62 | −0.58 |
| 12 | All but after block 18 | replays | −3.00 | −2.80 | −0.58 | −0.62 | +3.14 | +2.67 | +2.31 | +2.45 |
| 12 | All but after block 24 | replays | −3.00 | −2.80 | −0.58 | −0.62 | +3.14 | +2.67 | +2.31 | +2.45 |
| 12 | Plain on A's (H-47, 020's job) | replays | −3.00 | −2.80 | −0.58 | −0.62 | +3.14 | +2.67 | +2.31 | +2.45 |
| 12 | Without replay (H-47, 020's job) | without replay | +0.35 | +0.03 | +0.03 | −0.08 | −0.74 | −0.66 | −0.51 | −0.48 |
| 12 | Twin: plain on A's (021's job) | replays | −2.71 | −2.55 | −0.14 | −0.19 | +3.26 | +3.06 | +2.77 | +2.41 |
| 12 | Twin: without replay (021's job) | without replay | +0.36 | +0.12 | −0.04 | −0.07 | −0.66 | −0.63 | −0.46 | −0.44 |
| 18 | Without replay | without replay | +0.60 | +0.04 | +0.15 | −0.07 | −0.68 | −0.74 | −0.47 | −0.53 |
| 18 | After block 6 alone | without replay | +0.36 | +0.21 | +0.04 | 0.00 | −0.83 | −0.86 | −0.59 | −0.62 |
| 18 | After block 12 alone | without replay | +0.30 | +0.10 | +0.01 | −0.08 | −0.86 | −0.88 | −0.61 | −0.64 |
| 18 | After block 18 alone | replays | −3.50 | −3.04 | −0.69 | −0.57 | +4.56 | +3.93 | +3.32 | +3.42 |
| 18 | After block 24 alone | without replay | +0.60 | +0.04 | +0.15 | −0.07 | −0.68 | −0.74 | −0.47 | −0.53 |
| 18 | All but after block 6 | replays | −2.87 | −2.61 | −0.67 | −0.48 | +3.21 | +2.64 | +2.38 | +2.46 |
| 18 | All but after block 12 | replays | −2.59 | −2.77 | −0.51 | −0.52 | +3.22 | +2.76 | +2.34 | +2.51 |
| 18 | All but after block 18 | without replay | +0.41 | +0.21 | +0.07 | −0.06 | −0.92 | −0.92 | −0.65 | −0.65 |
| 18 | All but after block 24 | replays | −1.95 | −1.93 | −0.18 | −0.17 | +2.05 | +1.75 | +1.49 | +1.62 |
| 18 | Plain on A's (H-47, 020's job) | replays | −1.95 | −1.93 | −0.18 | −0.17 | +2.05 | +1.75 | +1.49 | +1.62 |
| 18 | Without replay (H-47, 020's job) | without replay | +0.60 | +0.04 | +0.15 | −0.07 | −0.68 | −0.74 | −0.47 | −0.53 |
| 18 | Twin: plain on A's (021's job) | replays | −1.53 | −1.61 | +0.31 | −0.05 | +1.96 | +1.76 | +1.64 | +1.40 |
| 18 | Twin: without replay (021's job) | without replay | +0.62 | +0.15 | +0.10 | −0.08 | −0.69 | −0.72 | −0.48 | −0.53 |
| 24 | Without replay | without replay | +0.68 | +0.36 | +0.16 | +0.11 | −0.67 | −0.66 | −0.44 | −0.57 |
| 24 | After block 6 alone | without replay | +0.42 | +0.27 | +0.10 | +0.10 | −0.80 | −0.83 | −0.58 | −0.60 |
| 24 | After block 12 alone | without replay | +0.29 | +0.12 | +0.02 | +0.03 | −0.87 | −0.89 | −0.63 | −0.65 |
| 24 | After block 18 alone | without replay | +0.39 | +0.30 | +0.07 | +0.11 | −0.85 | −0.87 | −0.60 | −0.67 |
| 24 | After block 24 alone | replays | −3.35 | −2.80 | −0.67 | −0.35 | +4.40 | +3.82 | +3.24 | +3.27 |
| 24 | All but after block 6 | replays | −1.90 | −1.72 | −0.20 | −0.14 | +2.14 | +1.73 | +1.47 | +1.68 |
| 24 | All but after block 12 | replays | −1.77 | −1.80 | −0.17 | −0.15 | +2.21 | +1.79 | +1.51 | +1.75 |
| 24 | All but after block 18 | replays | −1.95 | −1.89 | −0.29 | −0.12 | +2.31 | +1.96 | +1.65 | +1.77 |
| 24 | All but after block 24 | without replay | +0.30 | +0.33 | +0.02 | +0.02 | −0.96 | −0.97 | −0.67 | −0.73 |
| 24 | Plain on A's (H-47, 020's job) | replays | −1.53 | −1.31 | −0.11 | −0.02 | +1.38 | +1.24 | +0.98 | +1.13 |
| 24 | Without replay (H-47, 020's job) | without replay | +0.68 | +0.36 | +0.16 | +0.11 | −0.67 | −0.66 | −0.44 | −0.57 |
| 24 | Twin: plain on A's (021's job) | replays | −1.10 | −1.22 | +0.24 | −0.09 | +1.33 | +1.27 | +1.19 | +0.84 |
| 24 | Twin: without replay (021's job) | without replay | +0.69 | +0.49 | +0.14 | +0.04 | −0.72 | −0.69 | −0.46 | −0.61 |

- **Each form's first night that replays** moved the readouts' synapses about as H-47's first plain night did, whichever night it was:
  over those nights, after block 6 under after block 6 alone, all but after block 12, 18 or 24 and H-47's plain night, after block 12
  under after block 12 alone and all but after block 6, and after block 18 or 24 alone, the odd places' excitatory units lost 3.35 to
  3.99 M of weight onto all and the even places' 2.80 to 3.68 M, and the inhibition onto the even places' excitatory units, the group A
  answers, rose by 3.27 to 3.56 M and onto the odd places' by 3.24 to 3.55 M. Each moved least after block 24 alone and most at the night
  after block 6.
- **A night that replays after an earlier replayed night moved less**: over the eleven such nights, H-47's plain night's last three among
  them, the odd places' excitatory units lost 1.53 to 3.00 M and the inhibition onto the even places' excitatory units rose by 1.13 to
  2.51 M. H-47's plain night's night after block 24 moved least, −1.53 M and +1.13 M.
- **A night without replay after a replayed night** raised the odd places' excitatory weight onto all by 0.17 to 0.42 M and lowered the
  inhibition onto the even places' excitatory units by 0.58 to 0.73 M, where the nights without replay before any replay moved them by
  −0.16 to +0.68 M and −0.46 to −0.57 M.
- **On the twin**, where A answers the odd places, H-47's plain night's night after block 6 took 3.48 M of the odd places' excitatory
  weight and raised the inhibition onto the odd places' excitatory units by 4.28 M and onto the even places' by 3.42 M.

What each stretch of blocks moved, from the image after the night before it to the image before the next night, or block 36's; the head,
blocks 1 to 6, is the same in every form of a cell, from the start image to the image before the first night:

| Stretch | Form | The odd places' excitatory onto all | the even places' | The odd places' onto their own | the even places' onto their own | The odd places' inhibitory onto all | the even places' | Inhibitory onto the odd places' excitatory | onto the even places' |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| blocks 1 to 6 | This cell, every form | 0.00 | 0.00 | +0.10 | +0.13 | −0.77 | −0.92 | −0.73 | −0.62 |
| blocks 1 to 6 | The twin, every form | 0.00 | 0.00 | −0.10 | −0.02 | −0.71 | −0.78 | −0.62 | −0.51 |
| blocks 7 to 12 | Without replay | 0.00 | 0.00 | +0.02 | +0.02 | −0.97 | −0.96 | −0.87 | −0.74 |
| blocks 7 to 12 | After block 6 alone | 0.00 | −0.01 | +0.04 | 0.00 | −1.35 | −1.34 | −1.07 | −1.01 |
| blocks 7 to 12 | After block 12 alone | 0.00 | 0.00 | +0.02 | +0.02 | −0.97 | −0.96 | −0.87 | −0.74 |
| blocks 7 to 12 | After block 18 alone | 0.00 | 0.00 | +0.02 | +0.02 | −0.97 | −0.96 | −0.87 | −0.74 |
| blocks 7 to 12 | After block 24 alone | 0.00 | 0.00 | +0.02 | +0.02 | −0.97 | −0.96 | −0.87 | −0.74 |
| blocks 7 to 12 | All but after block 6 | 0.00 | 0.00 | +0.02 | +0.02 | −0.97 | −0.96 | −0.87 | −0.74 |
| blocks 7 to 12 | All but after block 12 | 0.00 | −0.01 | +0.04 | 0.00 | −1.35 | −1.34 | −1.07 | −1.01 |
| blocks 7 to 12 | All but after block 18 | 0.00 | −0.01 | +0.04 | 0.00 | −1.35 | −1.34 | −1.07 | −1.01 |
| blocks 7 to 12 | All but after block 24 | 0.00 | −0.01 | +0.04 | 0.00 | −1.35 | −1.34 | −1.07 | −1.01 |
| blocks 7 to 12 | Plain on A's (H-47, 020's job) | 0.00 | −0.01 | +0.04 | 0.00 | −1.35 | −1.34 | −1.07 | −1.01 |
| blocks 7 to 12 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | +0.02 | −0.97 | −0.96 | −0.87 | −0.74 |
| blocks 7 to 12 | Twin: plain on A's (021's job) | 0.00 | 0.00 | −0.15 | −0.08 | −1.16 | −1.15 | −0.88 | −0.86 |
| blocks 7 to 12 | Twin: without replay (021's job) | 0.00 | +0.01 | −0.13 | −0.11 | −0.77 | −0.79 | −0.63 | −0.59 |
| blocks 13 to 18 | Without replay | 0.00 | 0.00 | +0.02 | 0.00 | −1.07 | −1.03 | −0.90 | −0.85 |
| blocks 13 to 18 | After block 6 alone | 0.00 | 0.00 | +0.05 | +0.01 | −1.26 | −1.18 | −1.00 | −0.97 |
| blocks 13 to 18 | After block 12 alone | 0.00 | 0.00 | +0.02 | −0.01 | −1.48 | −1.37 | −1.16 | −1.08 |
| blocks 13 to 18 | After block 18 alone | 0.00 | 0.00 | +0.02 | 0.00 | −1.07 | −1.03 | −0.90 | −0.85 |
| blocks 13 to 18 | After block 24 alone | 0.00 | 0.00 | +0.02 | 0.00 | −1.07 | −1.03 | −0.90 | −0.85 |
| blocks 13 to 18 | All but after block 6 | 0.00 | 0.00 | +0.02 | −0.01 | −1.48 | −1.37 | −1.16 | −1.08 |
| blocks 13 to 18 | All but after block 12 | 0.00 | 0.00 | +0.05 | +0.01 | −1.26 | −1.18 | −1.00 | −0.97 |
| blocks 13 to 18 | All but after block 18 | 0.00 | 0.00 | +0.05 | +0.01 | −1.51 | −1.44 | −1.16 | −1.13 |
| blocks 13 to 18 | All but after block 24 | 0.00 | 0.00 | +0.05 | +0.01 | −1.51 | −1.44 | −1.16 | −1.13 |
| blocks 13 to 18 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.05 | +0.01 | −1.51 | −1.44 | −1.16 | −1.13 |
| blocks 13 to 18 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.07 | −1.03 | −0.90 | −0.85 |
| blocks 13 to 18 | Twin: plain on A's (021's job) | 0.00 | 0.00 | −0.05 | −0.07 | −1.31 | −1.29 | −1.00 | −0.98 |
| blocks 13 to 18 | Twin: without replay (021's job) | 0.00 | 0.00 | −0.01 | −0.05 | −0.86 | −0.89 | −0.76 | −0.66 |
| blocks 19 to 24 | Without replay | 0.00 | 0.00 | +0.02 | 0.00 | −1.09 | −1.06 | −0.92 | −0.91 |
| blocks 19 to 24 | After block 6 alone | 0.00 | 0.00 | +0.03 | −0.01 | −1.31 | −1.25 | −1.09 | −1.00 |
| blocks 19 to 24 | After block 12 alone | 0.00 | 0.00 | +0.01 | 0.00 | −1.40 | −1.27 | −1.15 | −1.02 |
| blocks 19 to 24 | After block 18 alone | 0.00 | 0.00 | +0.01 | −0.01 | −1.56 | −1.47 | −1.24 | −1.15 |
| blocks 19 to 24 | After block 24 alone | 0.00 | 0.00 | +0.02 | 0.00 | −1.09 | −1.06 | −0.92 | −0.91 |
| blocks 19 to 24 | All but after block 6 | 0.00 | 0.00 | +0.01 | +0.01 | −1.64 | −1.52 | −1.29 | −1.19 |
| blocks 19 to 24 | All but after block 12 | 0.00 | 0.00 | +0.02 | 0.00 | −1.61 | −1.51 | −1.26 | −1.17 |
| blocks 19 to 24 | All but after block 18 | 0.00 | 0.00 | +0.02 | 0.00 | −1.43 | −1.31 | −1.11 | −1.07 |
| blocks 19 to 24 | All but after block 24 | 0.00 | 0.00 | +0.02 | 0.00 | −1.64 | −1.54 | −1.24 | −1.20 |
| blocks 19 to 24 | Plain on A's (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.64 | −1.54 | −1.24 | −1.20 |
| blocks 19 to 24 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.02 | 0.00 | −1.09 | −1.06 | −0.92 | −0.91 |
| blocks 19 to 24 | Twin: plain on A's (021's job) | 0.00 | 0.00 | −0.02 | −0.01 | −1.43 | −1.36 | −1.08 | −1.04 |
| blocks 19 to 24 | Twin: without replay (021's job) | 0.00 | 0.00 | 0.00 | −0.02 | −0.95 | −0.99 | −0.84 | −0.75 |
| blocks 25 to 36 | Without replay | 0.00 | 0.00 | +0.03 | 0.00 | −2.28 | −2.39 | −1.93 | −1.94 |
| blocks 25 to 36 | After block 6 alone | 0.00 | 0.00 | +0.02 | 0.00 | −2.73 | −2.54 | −2.17 | −2.12 |
| blocks 25 to 36 | After block 12 alone | 0.00 | 0.00 | +0.03 | 0.00 | −2.75 | −2.63 | −2.20 | −2.17 |
| blocks 25 to 36 | After block 18 alone | 0.00 | 0.00 | +0.02 | 0.00 | −2.81 | −2.63 | −2.19 | −2.19 |
| blocks 25 to 36 | After block 24 alone | 0.00 | −0.01 | +0.02 | +0.01 | −2.98 | −2.84 | −2.33 | −2.28 |
| blocks 25 to 36 | All but after block 6 | 0.00 | −0.01 | +0.02 | +0.02 | −3.27 | −3.10 | −2.54 | −2.44 |
| blocks 25 to 36 | All but after block 12 | 0.00 | −0.01 | +0.02 | +0.02 | −3.26 | −3.06 | −2.53 | −2.41 |
| blocks 25 to 36 | All but after block 18 | 0.00 | 0.00 | +0.02 | +0.04 | −3.26 | −3.01 | −2.50 | −2.41 |
| blocks 25 to 36 | All but after block 24 | 0.00 | −0.01 | +0.02 | +0.01 | −3.15 | −2.84 | −2.41 | −2.33 |
| blocks 25 to 36 | Plain on A's (H-47, 020's job) | 0.00 | −0.01 | +0.02 | +0.03 | −3.31 | −3.10 | −2.52 | −2.46 |
| blocks 25 to 36 | Without replay (H-47, 020's job) | 0.00 | 0.00 | +0.03 | 0.00 | −2.28 | −2.39 | −1.93 | −1.94 |
| blocks 25 to 36 | Twin: plain on A's (021's job) | 0.00 | 0.00 | −0.04 | −0.04 | −2.88 | −2.69 | −2.14 | −2.12 |
| blocks 25 to 36 | Twin: without replay (021's job) | 0.00 | 0.00 | −0.01 | −0.03 | −2.01 | −2.13 | −1.64 | −1.78 |

- **The blocks move no excitatory weight of the readouts' units onto all**: at most 0.01 M either way in any stretch of any form, as
  exploration 020 read over the pair's blocks.
- **The inhibition falls in every stretch of every form, and falls more in each stretch after a replayed night.** The odd places'
  inhibitory units' weight onto all fell over blocks 7 to 12 by 1.35 M after a replayed night and 0.97 M without one; over blocks 13 to
  18 by 1.26 to 1.51 M and 1.07 M; over blocks 19 to 24 by 1.31 to 1.64 M and 1.09 M; and over blocks 25 to 36 by 2.73 to 3.31 M and
  2.28 M, the most under H-47's plain night. A stretch before which no night replayed fell as the night without replay's.

C's own units onto A's answer, the margin exploration 020 reads: their weight onto readout 1, the even places, which A answers on this
cell, less onto readout 0, where they lie. Each night and each stretch, the mean over the three paths, in M; "the four nights" sums the
nights and "blocks 7 to 36" the four stretches after the first night. On the twin A answers readout 0, so there it is the other way:

| Form | blocks 1 to 6 | night after 6 | blocks 7 to 12 | night after 12 | blocks 13 to 18 | night after 18 | blocks 19 to 24 | night after 24 | blocks 25 to 36 | The four nights | Blocks 7 to 36 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Without replay | −0.54 | −0.04 | −0.36 | −0.04 | −0.13 | −0.05 | −0.07 | −0.04 | −0.12 | −0.16 | −0.69 |
| After block 6 alone | −0.54 | −0.14 | −0.39 | −0.05 | −0.19 | −0.06 | −0.09 | −0.04 | −0.08 | −0.28 | −0.75 |
| After block 12 alone | −0.54 | −0.04 | −0.36 | −0.09 | −0.15 | −0.05 | −0.06 | −0.03 | −0.12 | −0.22 | −0.69 |
| After block 18 alone | −0.54 | −0.04 | −0.36 | −0.04 | −0.13 | −0.08 | −0.09 | −0.04 | −0.10 | −0.21 | −0.68 |
| After block 24 alone | −0.54 | −0.04 | −0.36 | −0.04 | −0.13 | −0.05 | −0.07 | −0.07 | −0.11 | −0.19 | −0.68 |
| All but after block 6 | −0.54 | −0.04 | −0.36 | −0.09 | −0.15 | −0.06 | −0.11 | −0.09 | −0.12 | −0.29 | −0.74 |
| All but after block 12 | −0.54 | −0.14 | −0.39 | −0.05 | −0.19 | −0.02 | −0.09 | −0.08 | −0.10 | −0.29 | −0.77 |
| All but after block 18 | −0.54 | −0.14 | −0.39 | −0.11 | −0.19 | −0.05 | −0.06 | −0.04 | −0.08 | −0.34 | −0.72 |
| All but after block 24 | −0.54 | −0.14 | −0.39 | −0.11 | −0.19 | −0.04 | −0.09 | −0.07 | −0.08 | −0.36 | −0.75 |
| Plain on A's (H-47, 020's job) | −0.54 | −0.14 | −0.39 | −0.11 | −0.19 | −0.04 | −0.09 | −0.05 | −0.10 | −0.34 | −0.77 |
| Without replay (H-47, 020's job) | −0.54 | −0.04 | −0.36 | −0.04 | −0.13 | −0.05 | −0.07 | −0.04 | −0.12 | −0.16 | −0.69 |
| Twin: plain on A's (021's job) | −0.48 | +0.12 | −0.50 | +0.17 | −0.15 | +0.10 | −0.07 | +0.11 | −0.16 | +0.51 | −0.88 |
| Twin: without replay (021's job) | −0.48 | −0.05 | −0.44 | −0.03 | −0.05 | −0.03 | 0.00 | −0.02 | −0.05 | −0.13 | −0.53 |

D's own units onto B's answer under A, readout 0 on this cell, less onto readout 1, where they lie:

| Form | blocks 1 to 6 | night after 6 | blocks 7 to 12 | night after 12 | blocks 13 to 18 | night after 18 | blocks 19 to 24 | night after 24 | blocks 25 to 36 | The four nights | Blocks 7 to 36 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Without replay | −0.82 | +0.03 | −0.09 | +0.04 | +0.02 | 0.00 | −0.01 | −0.04 | +0.03 | +0.02 | −0.06 |
| After block 6 alone | −0.82 | −0.02 | −0.08 | +0.02 | −0.05 | +0.01 | +0.05 | −0.02 | −0.01 | −0.01 | −0.09 |
| After block 12 alone | −0.82 | +0.03 | −0.09 | −0.02 | +0.05 | −0.02 | −0.02 | +0.01 | 0.00 | 0.00 | −0.07 |
| After block 18 alone | −0.82 | +0.03 | −0.09 | +0.04 | +0.02 | −0.02 | +0.02 | −0.03 | 0.00 | +0.01 | −0.06 |
| After block 24 alone | −0.82 | +0.03 | −0.09 | +0.04 | +0.02 | 0.00 | −0.01 | −0.04 | −0.06 | +0.02 | −0.14 |
| All but after block 6 | −0.82 | +0.03 | −0.09 | −0.02 | +0.05 | 0.00 | −0.03 | −0.03 | −0.07 | −0.02 | −0.14 |
| All but after block 12 | −0.82 | −0.02 | −0.08 | +0.02 | −0.05 | +0.01 | +0.02 | −0.02 | −0.06 | −0.01 | −0.18 |
| All but after block 18 | −0.82 | −0.02 | −0.08 | −0.01 | −0.05 | +0.01 | 0.00 | 0.00 | −0.15 | −0.02 | −0.28 |
| All but after block 24 | −0.82 | −0.02 | −0.08 | −0.01 | −0.05 | −0.03 | −0.01 | +0.01 | −0.03 | −0.04 | −0.17 |
| Plain on A's (H-47, 020's job) | −0.82 | −0.02 | −0.08 | −0.01 | −0.05 | −0.03 | −0.01 | 0.00 | −0.12 | −0.06 | −0.26 |
| Without replay (H-47, 020's job) | −0.82 | +0.03 | −0.09 | +0.04 | +0.02 | 0.00 | −0.01 | −0.04 | +0.03 | +0.02 | −0.06 |
| Twin: plain on A's (021's job) | −0.49 | +0.21 | −0.37 | +0.18 | −0.24 | +0.10 | −0.06 | +0.08 | −0.14 | +0.57 | −0.81 |
| Twin: without replay (021's job) | −0.49 | +0.02 | −0.43 | +0.01 | −0.15 | +0.01 | −0.07 | +0.02 | −0.09 | +0.06 | −0.74 |

A's margin, the four kinds A alone, A and C, B alone and B and D, each onto the readout A asks of its stimulus less onto the other:

| Form | blocks 1 to 6 | night after 6 | blocks 7 to 12 | night after 12 | blocks 13 to 18 | night after 18 | blocks 19 to 24 | night after 24 | blocks 25 to 36 | The four nights | Blocks 7 to 36 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Without replay | −1.14 | +0.15 | −0.44 | +0.05 | −0.16 | +0.07 | −0.10 | +0.10 | −0.16 | +0.37 | −0.85 |
| After block 6 alone | −1.14 | +1.21 | −0.51 | +0.04 | −0.26 | +0.05 | −0.06 | +0.06 | −0.12 | +1.37 | −0.95 |
| After block 12 alone | −1.14 | +0.15 | −0.44 | +1.17 | −0.11 | +0.09 | −0.12 | +0.08 | −0.19 | +1.49 | −0.87 |
| After block 18 alone | −1.14 | +0.15 | −0.44 | +0.05 | −0.16 | +1.08 | −0.08 | +0.06 | −0.19 | +1.35 | −0.87 |
| After block 24 alone | −1.14 | +0.15 | −0.44 | +0.05 | −0.16 | +0.07 | −0.10 | +1.03 | −0.18 | +1.30 | −0.88 |
| All but after block 6 | −1.14 | +0.15 | −0.44 | +1.17 | −0.11 | +0.95 | −0.16 | +0.63 | −0.20 | +2.90 | −0.91 |
| All but after block 12 | −1.14 | +1.21 | −0.51 | +0.04 | −0.26 | +0.82 | −0.09 | +0.68 | −0.20 | +2.75 | −1.05 |
| All but after block 18 | −1.14 | +1.21 | −0.51 | +1.01 | −0.27 | +0.05 | −0.09 | +0.85 | −0.25 | +3.12 | −1.12 |
| All but after block 24 | −1.14 | +1.21 | −0.51 | +1.01 | −0.27 | +0.75 | −0.11 | +0.05 | −0.15 | +3.01 | −1.03 |
| Plain on A's (H-47, 020's job) | −1.14 | +1.21 | −0.51 | +1.01 | −0.27 | +0.75 | −0.11 | +0.54 | −0.22 | +3.50 | −1.10 |
| Without replay (H-47, 020's job) | −1.14 | +0.15 | −0.44 | +0.05 | −0.16 | +0.07 | −0.10 | +0.10 | −0.16 | +0.37 | −0.85 |
| Twin: plain on A's (021's job) | −1.11 | +1.13 | −1.09 | +0.88 | −0.46 | +0.90 | −0.15 | +0.69 | −0.33 | +3.60 | −2.03 |
| Twin: without replay (021's job) | −1.11 | 0.00 | −1.19 | −0.12 | −0.31 | −0.03 | −0.13 | 0.00 | −0.24 | −0.15 | −1.87 |

- **C's own units lose margin onto A's answer over every night and every stretch, under every form**: in each of the twelve path-nights
  and each of the twelve path-stretches after the first night under each of the eight forms, under the night without replay and under
  H-47's plain night. Each form's first night that replays took 0.07 to 0.14 M of it, and a night without replay before any replay 0.04
  to 0.05 M. Over the four nights C's own lost 0.16 M under the night without replay, 0.19 to 0.28 M under the single nights, 0.29 to
  0.36 M under the forms leaving a night out and 0.34 M under H-47's plain night; over blocks 7 to 36 they lost 0.68 to 0.77 M under
  every one of the ten.
- **On the twin** each of H-47's plain night's nights raised C's own margin onto A's answer, +0.10 to +0.17 M, in each of its twelve
  path-nights, and each of its night without replay's nights lowered it, in each of its twelve.
- **D's own units** moved by −0.04 to +0.04 M over any night of any form here, by −0.06 to +0.02 M over the four nights and by −0.06 to
  −0.28 M over blocks 7 to 36.
- **A's margin** rose by 1.03 to 1.21 M over each form's first night that replays, by 0.54 to 1.01 M over a later one, and by 0.04 to
  0.15 M over a night without replay. Over the four nights it rose 1.30 to 1.49 M under the single nights, 2.75 to 3.12 M under the forms
  leaving a night out, 3.50 M under H-47's plain night and 0.37 M under the night without replay; over blocks 7 to 36 it fell 0.85 to
  1.12 M under every one of the ten.

## 5. What each replayed night held

Each night that replays, as `ReplayWatch` reads it, beside H-47's plain night's same night on this cell (exploration 020's job) and the
twin's (exploration 021's job). Spikes in a ripple that replays one of A's patterns, by the pattern's stimulus: the ripples are the total
over the three paths, and each other column a ripple's mean:

| Night after block | Form | Stimulus | Ripples | The first unit | The rest of the pattern | Pair's own in the answer's readout | Pair's own in the other readout | The answer's readout | The other readout | Another stimulus unit | Elsewhere |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | After block 6 alone | A | 2 097 | 3.1 | 21.8 | 0.4 | 0.7 | 10.7 | 12.9 | 1.9 | 0.1 |
| 6 | After block 6 alone | B | 1 921 | 3.1 | 22.2 | 0.4 | 0.6 | 10.9 | 11.8 | 2.1 | 0.1 |
| 6 | All but after block 12 | A | 2 097 | 3.1 | 21.8 | 0.4 | 0.7 | 10.7 | 12.9 | 1.9 | 0.1 |
| 6 | All but after block 12 | B | 1 921 | 3.1 | 22.2 | 0.4 | 0.6 | 10.9 | 11.8 | 2.1 | 0.1 |
| 6 | All but after block 18 | A | 2 097 | 3.1 | 21.8 | 0.4 | 0.7 | 10.7 | 12.9 | 1.9 | 0.1 |
| 6 | All but after block 18 | B | 1 921 | 3.1 | 22.2 | 0.4 | 0.6 | 10.9 | 11.8 | 2.1 | 0.1 |
| 6 | All but after block 24 | A | 2 097 | 3.1 | 21.8 | 0.4 | 0.7 | 10.7 | 12.9 | 1.9 | 0.1 |
| 6 | All but after block 24 | B | 1 921 | 3.1 | 22.2 | 0.4 | 0.6 | 10.9 | 11.8 | 2.1 | 0.1 |
| 6 | Plain on A's (H-47, 020's job) | A | 2 097 | 3.1 | 21.8 | 0.4 | 0.7 | 10.7 | 12.9 | 1.9 | 0.1 |
| 6 | Plain on A's (H-47, 020's job) | B | 1 921 | 3.1 | 22.2 | 0.4 | 0.6 | 10.9 | 11.8 | 2.1 | 0.1 |
| 6 | Twin: plain on A's (021's job) | A | 2 097 | 3.0 | 25.3 | 0.5 | 0.6 | 10.6 | 12.2 | 2.0 | 0.1 |
| 6 | Twin: plain on A's (021's job) | B | 1 921 | 3.0 | 21.8 | 0.4 | 0.5 | 9.7 | 12.1 | 2.0 | 0.1 |
| 12 | After block 12 alone | A | 2 100 | 3.0 | 21.9 | 0.4 | 0.6 | 10.6 | 12.7 | 1.9 | 0.1 |
| 12 | After block 12 alone | B | 1 923 | 3.0 | 22.2 | 0.4 | 0.6 | 11.1 | 12.0 | 2.0 | 0.1 |
| 12 | All but after block 6 | A | 2 100 | 3.0 | 21.9 | 0.4 | 0.6 | 10.6 | 12.7 | 1.9 | 0.1 |
| 12 | All but after block 6 | B | 1 923 | 3.0 | 22.2 | 0.4 | 0.6 | 11.1 | 12.0 | 2.0 | 0.1 |
| 12 | All but after block 18 | A | 1 917 | 3.1 | 21.9 | 0.4 | 0.6 | 11.1 | 13.2 | 2.1 | 0.1 |
| 12 | All but after block 18 | B | 1 851 | 3.0 | 22.2 | 0.5 | 0.6 | 11.1 | 12.1 | 2.1 | 0.1 |
| 12 | All but after block 24 | A | 1 917 | 3.1 | 21.9 | 0.4 | 0.6 | 11.1 | 13.2 | 2.1 | 0.1 |
| 12 | All but after block 24 | B | 1 851 | 3.0 | 22.2 | 0.5 | 0.6 | 11.1 | 12.1 | 2.1 | 0.1 |
| 12 | Plain on A's (H-47, 020's job) | A | 1 917 | 3.1 | 21.9 | 0.4 | 0.6 | 11.1 | 13.2 | 2.1 | 0.1 |
| 12 | Plain on A's (H-47, 020's job) | B | 1 851 | 3.0 | 22.2 | 0.5 | 0.6 | 11.1 | 12.1 | 2.1 | 0.1 |
| 12 | Twin: plain on A's (021's job) | A | 1 917 | 3.0 | 25.4 | 0.5 | 0.6 | 11.0 | 12.4 | 2.2 | 0.1 |
| 12 | Twin: plain on A's (021's job) | B | 1 851 | 3.0 | 21.8 | 0.4 | 0.5 | 10.2 | 12.6 | 2.0 | 0.1 |
| 18 | After block 18 alone | A | 2 100 | 3.1 | 21.7 | 0.3 | 0.6 | 10.9 | 13.2 | 1.9 | 0.1 |
| 18 | After block 18 alone | B | 1 923 | 3.0 | 22.1 | 0.5 | 0.5 | 10.9 | 11.9 | 2.0 | 0.1 |
| 18 | All but after block 6 | A | 1 917 | 3.0 | 21.9 | 0.4 | 0.6 | 11.1 | 13.1 | 2.0 | 0.1 |
| 18 | All but after block 6 | B | 1 851 | 3.0 | 22.2 | 0.5 | 0.6 | 11.3 | 12.3 | 2.1 | 0.1 |
| 18 | All but after block 12 | A | 1 911 | 3.1 | 21.8 | 0.4 | 0.6 | 11.2 | 13.2 | 2.0 | 0.1 |
| 18 | All but after block 12 | B | 1 857 | 3.0 | 22.2 | 0.5 | 0.6 | 11.2 | 12.4 | 2.0 | 0.1 |
| 18 | All but after block 24 | A | 1 818 | 3.1 | 21.9 | 0.4 | 0.6 | 11.6 | 13.6 | 2.1 | 0.1 |
| 18 | All but after block 24 | B | 1 752 | 3.1 | 22.2 | 0.5 | 0.6 | 11.7 | 12.3 | 2.1 | 0.1 |
| 18 | Plain on A's (H-47, 020's job) | A | 1 818 | 3.1 | 21.9 | 0.4 | 0.6 | 11.6 | 13.6 | 2.1 | 0.1 |
| 18 | Plain on A's (H-47, 020's job) | B | 1 752 | 3.1 | 22.2 | 0.5 | 0.6 | 11.7 | 12.3 | 2.1 | 0.1 |
| 18 | Twin: plain on A's (021's job) | A | 1 818 | 3.0 | 25.3 | 0.5 | 0.5 | 11.6 | 12.6 | 2.2 | 0.1 |
| 18 | Twin: plain on A's (021's job) | B | 1 752 | 3.0 | 22.0 | 0.4 | 0.6 | 10.5 | 13.1 | 2.2 | 0.1 |
| 24 | After block 24 alone | A | 2 100 | 3.0 | 21.8 | 0.3 | 0.5 | 10.9 | 13.1 | 1.9 | 0.1 |
| 24 | After block 24 alone | B | 1 923 | 3.0 | 22.1 | 0.4 | 0.5 | 11.0 | 11.8 | 2.0 | 0.1 |
| 24 | All but after block 6 | A | 1 818 | 3.1 | 22.0 | 0.4 | 0.6 | 11.5 | 13.4 | 2.1 | 0.1 |
| 24 | All but after block 6 | B | 1 752 | 3.0 | 22.1 | 0.5 | 0.5 | 11.7 | 12.5 | 2.2 | 0.1 |
| 24 | All but after block 12 | A | 1 818 | 3.1 | 21.9 | 0.4 | 0.6 | 11.6 | 13.5 | 2.2 | 0.1 |
| 24 | All but after block 12 | B | 1 752 | 3.0 | 22.2 | 0.5 | 0.5 | 11.5 | 12.5 | 2.1 | 0.1 |
| 24 | All but after block 18 | A | 1 806 | 3.1 | 21.9 | 0.4 | 0.6 | 11.6 | 13.7 | 2.2 | 0.1 |
| 24 | All but after block 18 | B | 1 737 | 3.1 | 22.2 | 0.5 | 0.6 | 11.7 | 12.5 | 2.1 | 0.1 |
| 24 | Plain on A's (H-47, 020's job) | A | 1 728 | 3.1 | 22.0 | 0.4 | 0.6 | 11.8 | 14.0 | 2.4 | 0.1 |
| 24 | Plain on A's (H-47, 020's job) | B | 1 623 | 3.1 | 22.3 | 0.5 | 0.6 | 12.1 | 12.7 | 2.1 | 0.1 |
| 24 | Twin: plain on A's (021's job) | A | 1 728 | 3.0 | 25.3 | 0.6 | 0.6 | 11.9 | 12.7 | 2.5 | 0.1 |
| 24 | Twin: plain on A's (021's job) | B | 1 623 | 3.0 | 21.9 | 0.5 | 0.6 | 11.0 | 13.6 | 2.2 | 0.1 |

- **Each form's first night that replays ran as many ripples as H-47's first plain night**: 2 097 to 2 100 on A's patterns and 1 921 to
  1 923 on B's over the three paths, after block 6, 12, 18 or 24 alike. H-47's plain night's later nights ran 1 917, 1 818 and 1 728 on
  A's patterns and 1 851, 1 752 and 1 623 on B's, and the later replayed nights of the forms leaving a night out 1 806 to 1 917 and 1 737
  to 1 857.
- **Beyond the pattern, a replay drives the other readout more than the answer's** in every row here, by 0.57 to 2.31 spikes a ripple, and
  on the twin by 0.85 to 2.64, as exploration 020 read over the four nights. The pattern's first unit fires 3.00 to 3.08 times a ripple and
  the rest of the pattern 21.7 to 22.3 times on this cell; on the twin the rest of A's patterns fire 25.3 to 25.4 times.

What the pairings of A's and B's 102 stimulus units put into the weights over the night, the mean over the three paths: by the target's
class against the unit's stimulus, the pair's own in both readouts together and elsewhere the other stimulus units and the units outside
the readouts; then by the unit's class, onto every target:

| Night after block | Form | Onto its own pattern | Onto the answer's readout | Onto the other readout | Onto the pair's own | Onto elsewhere | From A alone | From A and C | From B alone | From B and D |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | After block 6 alone | +698 873 | −95 417 | −536 744 | +8 123 | −17 340 | +51 443 | −72 828 | +6 496 | +72 384 |
| 6 | All but after block 12 | +698 873 | −95 417 | −536 744 | +8 123 | −17 340 | +51 443 | −72 828 | +6 496 | +72 384 |
| 6 | All but after block 18 | +698 873 | −95 417 | −536 744 | +8 123 | −17 340 | +51 443 | −72 828 | +6 496 | +72 384 |
| 6 | All but after block 24 | +698 873 | −95 417 | −536 744 | +8 123 | −17 340 | +51 443 | −72 828 | +6 496 | +72 384 |
| 6 | Plain on A's (H-47, 020's job) | +698 873 | −95 417 | −536 744 | +8 123 | −17 340 | +51 443 | −72 828 | +6 496 | +72 384 |
| 6 | Twin: plain on A's (021's job) | +704 567 | −131 706 | −469 947 | −13 645 | −27 158 | +29 546 | −10 925 | +27 454 | +16 037 |
| 12 | After block 12 alone | +754 157 | −98 521 | −466 439 | −901 | −21 069 | +70 754 | −59 206 | +52 903 | +102 776 |
| 12 | All but after block 6 | +754 157 | −98 521 | −466 439 | −901 | −21 069 | +70 754 | −59 206 | +52 903 | +102 776 |
| 12 | All but after block 18 | +610 725 | −82 086 | −435 931 | +15 879 | −16 922 | −2 954 | −25 499 | +51 259 | +68 858 |
| 12 | All but after block 24 | +610 725 | −82 086 | −435 931 | +15 879 | −16 922 | −2 954 | −25 499 | +51 259 | +68 858 |
| 12 | Plain on A's (H-47, 020's job) | +610 725 | −82 086 | −435 931 | +15 879 | −16 922 | −2 954 | −25 499 | +51 259 | +68 858 |
| 12 | Twin: plain on A's (021's job) | +494 279 | −91 133 | −412 709 | −7 909 | −19 539 | −25 515 | +45 730 | +42 055 | −99 282 |
| 18 | After block 18 alone | +622 269 | −123 566 | −532 907 | +26 006 | −21 346 | −16 005 | −102 145 | +73 847 | +14 759 |
| 18 | All but after block 6 | +539 858 | −50 706 | −405 413 | +26 798 | −19 117 | +55 146 | +31 373 | −28 263 | +33 163 |
| 18 | All but after block 12 | +438 200 | −94 133 | −389 698 | +52 520 | −20 674 | −13 648 | −31 589 | −41 883 | +73 335 |
| 18 | All but after block 24 | +449 133 | −63 539 | −319 477 | +38 671 | −17 603 | +37 099 | −782 | +14 207 | +36 660 |
| 18 | Plain on A's (H-47, 020's job) | +449 133 | −63 539 | −319 477 | +38 671 | −17 603 | +37 099 | −782 | +14 207 | +36 660 |
| 18 | Twin: plain on A's (021's job) | +595 985 | −53 376 | −289 141 | +14 267 | −21 269 | +23 730 | +110 710 | +70 223 | +41 803 |
| 24 | After block 24 alone | +701 853 | −137 676 | −388 023 | +12 017 | −17 785 | +58 176 | +26 492 | +8 160 | +77 557 |
| 24 | All but after block 6 | +346 286 | −48 957 | −277 977 | +28 348 | −17 476 | −828 | +30 962 | −10 892 | +10 982 |
| 24 | All but after block 12 | +433 291 | −37 348 | −258 786 | +7 384 | −15 792 | +17 079 | +42 411 | +16 908 | +52 352 |
| 24 | All but after block 18 | +584 398 | −60 018 | −292 914 | +3 071 | −19 795 | +18 465 | +78 983 | +2 597 | +114 697 |
| 24 | Plain on A's (H-47, 020's job) | +355 966 | −27 384 | −190 224 | +9 999 | −21 525 | +25 106 | +16 466 | +56 478 | +28 783 |
| 24 | Twin: plain on A's (021's job) | +488 391 | −84 176 | −220 581 | +19 171 | −22 399 | +20 053 | +94 716 | +55 100 | +10 537 |

- **Every replayed night's pairings put weight onto the units' own patterns and took it from the other readout and from the answer's
  readout off the pattern**: onto the patterns +346 286 to +754 157, from the other readout −190 224 to −536 744 and from the answer's
  readout −27 384 to −137 676 a night on this cell, and on the twin +488 391 to +704 567, −220 581 to −469 947 and −53 376 to −131 706.
- **Each form's first night that replays** put +622 269 to +754 157 onto the patterns and took −388 023 to −536 744 from the other
  readout; a later one +346 286 to +610 725 and −190 224 to −435 931.

## 6. The engine over each night

Each night's change in the engine's excitatory and inhibitory sums and its gain, from the night's start to its end, the mean over the three
paths; its replays and depotentiations on each path; and the watch's wrong answers over the three paths. H-47's plain night is exploration
020's job's:

| Form | Night after block | This night | Excitatory sum, M | Inhibitory sum, M | Gain | Replays, paths 0, 5, 6 | Depotentiations | Watch: wrong |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Without replay | 6 | without replay | −0.48 | −1.19 | +1 991 | 0, 0, 0 | 0, 0, 0 | 0 |
| Without replay | 12 | without replay | +0.37 | −1.40 | +1 972 | 0, 0, 0 | 0, 0, 0 | 0 |
| Without replay | 18 | without replay | +0.64 | −1.43 | +1 978 | 0, 0, 0 | 0, 0, 0 | 0 |
| Without replay | 24 | without replay | +1.02 | −1.33 | +1 991 | 0, 0, 0 | 0, 0, 0 | 0 |
| After block 6 alone | 6 | replays | −7.72 | +8.85 | −4 327 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| After block 6 alone | 12 | without replay | +0.33 | −1.71 | +1 957 | 0, 0, 0 | 0, 0, 0 | 0 |
| After block 6 alone | 18 | without replay | +0.56 | −1.69 | +2 030 | 0, 0, 0 | 0, 0, 0 | 0 |
| After block 6 alone | 24 | without replay | +0.66 | −1.63 | +2 063 | 0, 0, 0 | 0, 0, 0 | 0 |
| After block 12 alone | 6 | without replay | −0.48 | −1.19 | +1 991 | 0, 0, 0 | 0, 0, 0 | 0 |
| After block 12 alone | 12 | replays | −7.14 | +8.49 | −4 333 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| After block 12 alone | 18 | without replay | +0.40 | −1.74 | +2 031 | 0, 0, 0 | 0, 0, 0 | 0 |
| After block 12 alone | 24 | without replay | +0.38 | −1.75 | +2 097 | 0, 0, 0 | 0, 0, 0 | 0 |
| After block 18 alone | 6 | without replay | −0.48 | −1.19 | +1 991 | 0, 0, 0 | 0, 0, 0 | 0 |
| After block 18 alone | 12 | without replay | +0.37 | −1.40 | +1 972 | 0, 0, 0 | 0, 0, 0 | 0 |
| After block 18 alone | 18 | replays | −6.58 | +8.49 | −4 385 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| After block 18 alone | 24 | without replay | +0.67 | −1.72 | +2 051 | 0, 0, 0 | 0, 0, 0 | 0 |
| After block 24 alone | 6 | without replay | −0.48 | −1.19 | +1 991 | 0, 0, 0 | 0, 0, 0 | 0 |
| After block 24 alone | 12 | without replay | +0.37 | −1.40 | +1 972 | 0, 0, 0 | 0, 0, 0 | 0 |
| After block 24 alone | 18 | without replay | +0.64 | −1.43 | +1 978 | 0, 0, 0 | 0, 0, 0 | 0 |
| After block 24 alone | 24 | replays | −6.21 | +8.22 | −4 391 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| All but after block 6 | 6 | without replay | −0.48 | −1.19 | +1 991 | 0, 0, 0 | 0, 0, 0 | 0 |
| All but after block 6 | 12 | replays | −7.14 | +8.49 | −4 333 | 1 341, 1 341, 1 341 | 640, 640, 640 | 0 |
| All but after block 6 | 18 | replays | −5.53 | +5.86 | −3 787 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| All but after block 6 | 24 | replays | −3.68 | +3.86 | −3 287 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| All but after block 12 | 6 | replays | −7.72 | +8.85 | −4 327 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| All but after block 12 | 12 | without replay | +0.33 | −1.71 | +1 957 | 0, 0, 0 | 0, 0, 0 | 0 |
| All but after block 12 | 18 | replays | −5.43 | +5.98 | −3 767 | 1 268, 1 268, 1 268 | 600, 600, 600 | 0 |
| All but after block 12 | 24 | replays | −3.62 | +4.00 | −3 302 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| All but after block 18 | 6 | replays | −7.72 | +8.85 | −4 327 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| All but after block 18 | 12 | replays | −5.84 | +5.81 | −3 789 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| All but after block 18 | 18 | without replay | +0.62 | −1.84 | +1 981 | 0, 0, 0 | 0, 0, 0 | 0 |
| All but after block 18 | 24 | replays | −3.89 | +4.27 | −3 264 | 1 194, 1 194, 1 194 | 571, 571, 571 | 0 |
| All but after block 24 | 6 | replays | −7.72 | +8.85 | −4 327 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| All but after block 24 | 12 | replays | −5.84 | +5.81 | −3 789 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| All but after block 24 | 18 | replays | −3.93 | +3.79 | −3 392 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| All but after block 24 | 24 | without replay | +0.62 | −1.93 | +2 074 | 0, 0, 0 | 0, 0, 0 | 0 |
| Plain on A's (H-47, 020's job) | 6 | replays | −7.72 | +8.85 | −4 327 | 1 339, 1 340, 1 339 | 640, 640, 640 | 0 |
| Plain on A's (H-47, 020's job) | 12 | replays | −5.84 | +5.81 | −3 789 | 1 268, 1 268, 1 268 | 602, 602, 602 | 0 |
| Plain on A's (H-47, 020's job) | 18 | replays | −3.93 | +3.79 | −3 392 | 1 202, 1 202, 1 202 | 571, 571, 571 | 0 |
| Plain on A's (H-47, 020's job) | 24 | replays | −2.89 | +2.62 | −2 803 | 1 129, 1 129, 1 129 | 545, 545, 545 | 0 |

- **Each form's first night that replays** lowered the excitatory sum by 6.21 to 7.72 M, raised the inhibitory sum by 8.22 to 8.85 M and
  lowered the gain by 4 327 to 4 391, with 1 339 to 1 341 replays and 640 depotentiations on each path. **A later replayed night**
  lowered the excitatory sum by 2.89 to 5.84 M, raised the inhibitory by 2.62 to 5.98 M and lowered the gain by 2 803 to 3 789, with
  1 129 to 1 268 replays and 545 to 602 depotentiations.
- **Every night without replay** read `replays 0` and `depotentiations 0`, held inside the test, moved the excitatory sum by −0.48 to
  +1.02 M and the inhibitory by −1.19 to −1.93 M, and raised the gain by 1 957 to 2 097.
- **The watch read `wrong 0`** in each of the 120 nights of the two jobs.

The ties of both stimuli's trials, each form less the night without replay of its own job, ± the paired SE:

| Form | Block 6 | 12 | 18 | 24 | 36 |
| :--- | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone | −0.6 ± 0.42 | −0.8 ± 0.37 | +0.2 ± 0.57 | −0.4 ± 0.49 | −0.2 ± 0.17 |
| After block 12 alone | = without replay | −0.3 ± 0.60 | −0.2 ± 0.13 | −0.4 ± 0.25 | +0.9 ± 0.32 |
| After block 18 alone | = without replay | = without replay | +0.5 ± 0.35 | 0.0 ± 0.25 | +0.7 ± 0.47 |
| After block 24 alone | = without replay | = without replay | = without replay | +0.6 ± 0.59 | +0.7 ± 0.36 |
| All but after block 6 | = without replay | −0.3 ± 0.60 | +0.6 ± 0.27 | +0.4 ± 0.45 | +0.5 ± 0.59 |
| All but after block 12 | −0.6 ± 0.42 | −0.8 ± 0.37 | +0.7 ± 0.60 | +0.9 ± 0.44 | +0.9 ± 0.79 |
| All but after block 18 | −0.6 ± 0.42 | −0.6 ± 0.56 | −0.1 ± 0.30 | +0.4 ± 0.32 | +1.3 ± 0.37 |
| All but after block 24 | −0.6 ± 0.42 | −0.6 ± 0.56 | +0.4 ± 0.43 | +0.2 ± 0.54 | +0.6 ± 0.50 |
| Plain on A's (H-47) | −0.6 ± 0.42 (H-47) | −0.6 ± 0.56 (H-47) | +0.4 ± 0.43 (H-47) | +0.6 ± 0.36 (H-47) | +1.7 ± 0.21 (H-47) |

- **Beyond twice the SE** the ties read below the night without replay at block 12 under after block 6 alone and all but after block 12,
  −0.8 ± 0.37 each, on the same rows, and above at block 18 under all but after block 6, +0.6 ± 0.27, at block 24 under all but after
  block 12, +0.9 ± 0.44, and at block 36 under after block 12 alone, +0.9 ± 0.32, and all but after block 18, +1.3 ± 0.37. H-47's plain
  night read +1.7 ± 0.21 at block 36. The others lie within 2 SE, after block 24 alone at block 36, +0.7 ± 0.36, by less than 0.01 of
  twice its SE.

## What it costs

The trial took 540 s and the two jobs 9 756 s and 9 433 s, 162.6 and 157.2 minutes, at most 67.8 % of a job's 240 minutes; ADR-0245 put a
job at about 150 to 170 minutes. Nothing was cut: every form ran on its three paths, and every night was watched. Three jobs ran, the
trial's and two, every one to the end.

## What it suggests

**On this cell no single night's replay, after nights without replay, left A's count at block 36 as far below the night without replay as
H-47's four did.** The four single nights read +1.7, −0.5, +0.3 and −2.4 there, against H-47's −4.9, and their differences sum to −0.9.
Only the night after block 24, the last, lies below beyond 2 SE alone; there both stimuli's trials read below the night without replay,
A's beyond 2 SE. Right after it, at block 24, it read −0.3 ± 0.81.

**Each form leaving one night out lies at block 36 below the night without replay, −1.3 to −2.6, beyond 2 SE in two, and above H-47's
plain night beyond 2 SE in all four, +2.3 to +3.6.** On stimulus A's trials all four lie below the night without replay beyond 2 SE, as
H-47's plain night does, by less than it.

**What a replayed night moved on the readouts' synapses ran with how many replayed nights came before it.** A form's first replayed night
took 3.35 to 3.99 M of the odd places' excitatory weight after any block; a second took 2.59 to 3.00 M, a third 1.77 to 1.95 M, and
H-47's fourth 1.53 M. C's own units lost margin onto A's answer in every night and every stretch under every form, as under the night
without replay: over the four nights 0.16 M without replay and 0.19 to 0.36 M under the forms that replay. On the twin each of H-47's
plain nights raised it.

Questions this suggests for a later exploration or a round:
- Replay on two of the four nights, which no form here reads: whether two nights' replay without the other two leaves A's count where
  H-47's four did.
- What the twelve blocks after the night after block 24 move on A's trials, where after block 24 alone read −0.9 ± 0.39 there at block 24
  and −1.1 ± 0.24 at block 36.
- The same forms on the twin, where H-47's plain night less its night without replay read +13.4 ± 1.56 at block 36.
- A read unit by unit, to see which readout units fire on A's trials after each form's nights.

## What did not run

- **The twin cell, or any other cell**: the twin's plain night and night without replay are read from exploration 021's job, beside this
  cell's forms, with no SE.
- **Replay on two of the four nights**: so where no night alone is enough and two nights are each needed, these runs do not say whether
  those two nights' replay is enough without the others.
- **Plain nights on both mappings and the awake gap**, night by night or at all: no reading here stands against them.
- **Any hold**, on any night.
- **A read unit by unit**: a read gives each readout's spike count, not which units fired.
- **Any path but H-47's three**, 0, 5 and 6.
- **A night split apart**, such as slow-wave sleep read apart from REM: a night's change is read whole, from the images before and after
  it, and REM's depotentiations and the zero sum's rounds are not split out of it, as in explorations 020 to 029.
- **The next day's reading of the pairings a night leaves** pending.
- **Any SE for sections 4 and 5**, or for the block-36 sums of section 1: the class matrix, the split and the watch are means over the three
  paths.
- **H-47's plain night run again**: its rows and reads are H-47's job's and its night lines exploration 020's, so a form less it carries
  exploration 018's unpaired SE.
- **Any other share, form, network or seed.**

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_one_cell_night_by_night_explore` | [42619303153](https://github.com/Menteion/Menteion/actions/runs/42619303153) | `8291346` | 540 |
| `one_night_alone_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [42621480151](https://github.com/Menteion/Menteion/actions/runs/42621480151) | `8291346` | 9756 |
| `one_night_left_out_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [42621480151](https://github.com/Menteion/Menteion/actions/runs/42621480151) | `8291346` | 9433 |
