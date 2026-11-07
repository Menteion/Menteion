---
status: exploratory
date: 2026-11-08
---

# 023: The readout's index or its group

**Branch**: `explore/023-the-readouts-index-or-its-group`
**Commit**: `9f5d5aa`, which adds tests and nothing under `src/`, on `7701a4f`, `main` at exploration 022's merge, with exploration
022's tag merged in (`6759578`).
**Runs**: [42387230906](https://github.com/Menteion/Menteion/actions/runs/42387230906) (the harness's trial) and
[42389139656](https://github.com/Menteion/Menteion/actions/runs/42389139656) (six jobs: layout 1 on four cells, layout 2 on two), both at
`scope=arms`. H-47's layout is read from H-47's run [41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613) and, for its
split and class matrix, from exploration 020's runs [41500459562](https://github.com/Menteion/Menteion/actions/runs/41500459562) and
[41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777) and exploration 021's
[41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514). None of those was run again.

[Exploration 022](022-readout-or-the-pairs-own.md) read the side lying on the same readout from block 18 in all four of its layouts, in all
four cells, and could not tell the units from their readout: in every layout there, readout 0 was the odd places and readout 1 the even
places. This note parts a readout's index from its group of units in two layouts on 022's four cells. Layout 1 exchanges the index alone:
the task is given the even places as readout 0 and the odd places as readout 1, the mapping in force is named the other way, and the start
image is H-47's. Layout 2 exchanges the places: the same readouts, the mapping in force named as in H-47, and the first mapping learned
again under them. It asks, at blocks 6, 12, 18, 24 and 36:

1. in each new layout, each stimulus's correct trials under the night without replay and the plain night on A's patterns, plain less
   without replay on each stimulus's trials, the side, its readout's index and its group of units;
2. against H-47's side on the same cell, whether the side goes with the index, with the group of units, or with the stimulus and so its
   source units;
3. the ties, and each read's stimulus mask, against H-47's;
4. where the pair's own units lie in the split, and what plain nights take from each group's own synapses, from the class matrix.

**The side** is ADR-0238's: at a checkpoint, the stimulus whose plain less without replay difference is the larger of the two. The note
writes no prediction and decides nothing. **Every figure here is exploratory**: from one run of each test on the repository's AArch64
host, in H-47's terms. A count is a mean over a cell's 48 reads, sixteen on each of three paths, and a difference carries its paired SE
unless another is named.

**In brief**
- **Layout 1 ran H-47's runs, bit for bit, with the readouts' index exchanged.** In all four cells, under both forms, each of the 15 rows a
  form, its image's CRC-64, its sixteen reads' counts of A and of the pair and its gain, is H-47's row with A's couplings exchanged by the
  index, as the job prints after each row; nothing in the job asserts it. Every one of its 3 840 reads carries H-47's read's stimuli,
  right and tied masks, so each trial presented the same stimulus and was right, tied or wrong as in H-47, and its spike counts by readout
  are H-47's exchanged. Each read's trace, which hashes the index selected, differs from H-47's in every read. Every split and class
  matrix is H-47's with the readouts' halves and classes exchanged.
- **Layout 2 ran H-47's other arm, bit for bit.** Its first mapping, learned again under its readouts, is H-47's first mapping from the other
  arm: the same 24 blocks of correct trials and the same image. From there each of its 30 rows a job, its 1 920 reads, its splits and its
  class matrices are H-47's of the other arm's cell with the index exchanged. Its two jobs print the lines of layout 1's two first-mapping
  jobs from the other arm, label aside, but for 33 lines each: its first mapping's two, its layout's line and the rows' 30 notices.
- **So in layout 1 the side goes with the group of units and the stimulus in all four cells at every checkpoint, and with the index in
  none.** From block 18 it lies on the odd places in three cells and on the even places on seed 24's eighth image from the mirrored
  assignment, beyond twice its SE in 4, 3 and 4 of the cells at blocks 18, 24 and 36.
- **In layout 2 it goes with the group alone in both cells at blocks 6, 18, 24 and 36, and with the index and the stimulus at block 12.**
  That is H-47's other arm's side: on seed 23's first mapping H-47 puts it on the odd places from block 18 from either arm, on B from the
  mirrored assignment and on A from the assignment.
- **The ties are H-47's, read by read**: 2 316, 2 929, 2 968 and 2 152 tied trials in each cell's 960 reads in H-47 and in layout 1, and in
  layout 2 those of H-47's other arm's cell.
- **Plain nights add more inhibition onto the excitatory units of the group stimulus A answers** in every cell of every layout here, at
  H-47's figures of the same cell or, in layout 2, of the other arm's.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 021 and 022 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. A cell is named, as in H-47, by its seed, its image and the arm
it starts from. Each layout ran the night without replay and the plain night on A's patterns, on paths 0, 5 and 6, as H-47's job of those two
forms does.

**Places.** A period of the ring is 20 units. Stimulus A presents place 0 of each period and B place 11, in every layout here. **The odd
places** are the odd places but 11, nine units a period; **the even places** the even places but 0. C's own units lie at place 5, among the
odd places, and D's at place 16, among the even, in every layout here, as H-47 lays them. The pair's readouts are A's less those two places.

**The task's readout** is two sets of units, by index: readout 0 and readout 1. A trial counts each set's spikes in its window, and the
selection (`Readout::select`, in `runtime/menteion-runtime/src/task.rs`) releases the readout whose count is above the other's, and none at a
tie. The mapping in force says which readout index each stimulus answers.

**The three layouts.**

| Layout | Readout 0 | Readout 1 | The mapping in force | Stimulus A answers, under the assignment | The start image | Its runs |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| H-47 | the odd places | the even places | H-47's | readout 0, the odd places | H-47's | H-47's |
| Layout 1, the index exchanged | the even places | the odd places | named the other way | readout 1, the odd places | H-47's | here, on four cells |
| Layout 2, the places exchanged | the even places | the odd places | H-47's | readout 0, the even places | learned again under its readouts | here, on two cells |

Both new layouts give the task the same four sets, `[A, B, the even places, the odd places]`, with the roles and the pair's own units as
H-47 has them. They differ in the name of the mapping in force and in the start image. `with_readouts`, on the branch, lays a start out in a
layout. Against H-47, item by item:

| Item | Layout 1, the index exchanged | Layout 2, the places exchanged |
| :--- | :--- | :--- |
| The derivation and H-42's image | H-47's | H-47's |
| The start image and its couplings | H-47's, held bit for bit in the trial; A's four couplings, in a row's order `[A→R0, A→R1, B→R0, B→R1]`, read by the other index | Learned again from H-42's image, H-25's arm over its first 24 blocks under the zero sum's two groups as H-47's is (`e023_first_mapping`): H-47's first mapping from the other arm, bit for bit |
| The zero sum's two groups of H-42 | H-47's units | H-47's units |
| The zero sum's six groups of the pair, and their values | H-47's units at H-47's values, in H-47's order | H-47's units at their sums in its start image: H-47's other arm's values |
| A's patterns and their targets | H-47's, in H-47's order; the targets in each stimulus's answer readout, the same units under the other index | The same first units, in H-47's order; the targets in each stimulus's answer readout in its start image: under the assignment, A's among the even places |
| The pair's kinds and the units C and D present | H-47's | H-47's |
| The pair's readouts | The even places less place 16 as readout 0, the odd places less place 5 as readout 1: H-47's units under the other index | The same |
| The group of units each stimulus answers | H-47's: under the assignment A answers the odd places, now readout 1 | The other: under the assignment A answers the even places, readout 0 |
| Each read's stimulus mask | H-47's, a trial's stimulus drawn by its index | H-47's |
| Where the split holds the pair's own units | D's own inside readout 0 and C's inside readout 1 | The same |
| The class matrix's readout classes | By index: readout 0's (6, 7 and 11) are the even places, readout 1's (8, 9 and 12) the odd | The same |

**The split's assert.** In H-47's layout exploration 020's split holds C's own units inside readout 0 and D's inside readout 1, and its six
parts are readout 0's three, then readout 1's. In both new layouts `e023_own_in` names the readout that holds place 5, and the split
(`e022_split`) holds D's own units inside readout 0 and C's inside readout 1. So its first three parts are the even places' and its last
three the odd places'. Every split held that, and every split's parts added up to each kind's sum onto each readout.

**The class matrix** names its readout classes by the task's index, as exploration 020 wrote it. Sections 3 and 4 sort them by group: a
group's excitatory units are its readout's two classes and the pair's own kind inside it, C's own in the odd places and D's in the even.

**Layout 2 runs from a first mapping alone.** An eighth image under its readouts would need H-46's eight schedules learned again from H-42's
image. H-46's first halves, which made its eighth images, took 101.9 to 110.8 minutes, before a job of about 64 to 68 minutes, past this
exploration's 1.5 hours a job.

## The runs are laid out as written

- **The trial** ran seed 23's first mapping from the mirrored assignment on path 0. It ran the pair's head, the first night without
  replay and the first plain night on A's patterns, both watched. It split the start image and each night's image. It ran them in H-47's
  layout, then layout 1, then layout 2.
  - **In H-47's layout** the first mapping, learned again by the harness's run under H-47's readouts, was H-47's, bit for bit, and each
    form held H-47's row at block 6 on path 0, inside the test.
  - **In layout 1** the start image, the zero sum's groups, A's patterns and the pair held H-47's, inside the test, with each set of units
    answering H-47's units under the other index. Its rows at block 6 on path 0 are H-47's with A's couplings exchanged, for both forms.
  - **In layout 2** the readouts' places, the groups' units and targets, the pair, and A's patterns' first units and targets held, inside the
    test. Its first mapping is H-47's from the other arm, and its rows at block 6 on path 0 are H-47's of the other arm's cell with A's
    couplings exchanged, for both forms.
  - The watch read `wrong 0` in its six nights, and each plain night 1 339 replays and 640 depotentiations.
- **Each of the six jobs** printed, after each row, whether it is H-47's row with A's couplings and their highest exchanged by the index: of
  the same cell in layout 1, of the other arm's cell in layout 2. **Every one of the 180 rows is**, 15 a form in each job.
- **Layout 2's first mappings** are H-47's from the other arm: from the assignment the image `0x1cfc64de212c37f1`, H-47's first mapping from
  the mirrored assignment, and from the mirrored assignment `0x20f09d2e9b41ebfa`, H-47's from the assignment. Each ran the 24 blocks of
  correct trials that H-47's job prints for its first mapping from the other arm.
- **Every read's `stimuli` mask is H-47's** read of the same cell, form, path, checkpoint, mapping and stream: all 3 840 reads of layout 1
  and all 1 920 of layout 2. In layout 1 every read's `right` and `tied` masks are H-47's too, and in layout 2 every read's are those of
  H-47's read of the other arm's cell. In both, every read's spike counts by readout are those reads' with the two readouts exchanged, its
  gain after the read theirs, and its trace not theirs in any read.
- **Every split of the jobs** held the pair's own units where the layouts put them, 55 a job. Of layout 1's, all 55 a job are H-47's split
  of the same cell with its two halves exchanged, and every class matrix H-47's with classes 6 and 8, 7 and 9, and 11 and 12 exchanged. Of
  layout 2's, the same against H-47's of the other arm's cell.
- **Layout 2's two jobs** print 7 305 and 7 294 `DUMP` lines, of which 7 272 and 7 261 are lines of layout 1's job of the other arm's cell, label
  aside. The other 33 of each are its first mapping's line, the line that holds it against H-47's, its layout's line, which names the
  layout, and the 30 rows' notices, which name what each row is held against.
- **The watch read `wrong 0` in all 144 nights.** Every night without replay read `replays 0` and `depotentiations 0`, and each plain night
  1 129 to 1 340 replays and 545 to 640 depotentiations.

## 1. The side in each layout

Each stimulus's correct trials, of the trials a read presents of it, under the night without replay and the plain night on A's patterns;
plain less without replay on each stimulus's trials, and A's less B's, ± their paired SEs; the side, the readout it lies on and that
readout's group of units. H-47's figures, which layout 1's are on every row:

| Cell | Block | A's trials: without replay, plain on A's | B's trials: the same | A's: plain less without replay | B's: plain less without replay | A's less B's | Side in H-47 | Side in layout 1 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | :--- | :--- |
| Seed 23, first mapping, mirrored | 6 | 30.4, 31.0 | 24.6, 25.9 | +0.6 ± 0.31 | +1.3 ± 0.56 | −0.7 ± 0.61 | B, readout 0, the odd places (within 2 SE) | B, readout 1, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 12 | 28.5, 29.6 | 23.8, 24.8 | +1.1 ± 0.70 | +1.0 ± 1.17 | +0.1 ± 1.46 | A, readout 1, the even places (within 2 SE) | A, readout 0, the even places (within 2 SE) |
| Seed 23, first mapping, mirrored | 18 | 28.5, 27.6 | 23.5, 24.8 | −1.0 ± 0.92 | +1.3 ± 0.69 | −2.3 ± 1.12 | B, readout 0, the odd places | B, readout 1, the odd places |
| Seed 23, first mapping, mirrored | 24 | 30.0, 25.0 | 23.5, 25.9 | −5.0 ± 1.25 | +2.4 ± 0.38 | −7.4 ± 1.33 | B, readout 0, the odd places | B, readout 1, the odd places |
| Seed 23, first mapping, mirrored | 36 | 29.3, 24.1 | 23.9, 24.2 | −5.2 ± 1.50 | +0.3 ± 0.88 | −5.5 ± 2.21 | B, readout 0, the odd places | B, readout 1, the odd places |
| Seed 23, first mapping, assignment | 6 | 29.2, 30.9 | 25.0, 25.4 | +1.6 ± 0.46 | +0.4 ± 0.57 | +1.3 ± 0.67 | A, readout 0, the odd places (within 2 SE) | A, readout 1, the odd places (within 2 SE) |
| Seed 23, first mapping, assignment | 12 | 24.2, 30.1 | 9.3, 7.4 | +6.0 ± 0.87 | −1.8 ± 0.85 | +7.8 ± 1.24 | A, readout 0, the odd places | A, readout 1, the odd places |
| Seed 23, first mapping, assignment | 18 | 19.5, 30.1 | 7.6, 4.9 | +10.6 ± 1.65 | −2.8 ± 1.24 | +13.4 ± 1.44 | A, readout 0, the odd places | A, readout 1, the odd places |
| Seed 23, first mapping, assignment | 24 | 17.3, 30.4 | 7.5, 4.3 | +13.2 ± 1.45 | −3.2 ± 0.67 | +16.4 ± 1.23 | A, readout 0, the odd places | A, readout 1, the odd places |
| Seed 23, first mapping, assignment | 36 | 15.7, 29.2 | 4.0, 3.9 | +13.5 ± 1.09 | −0.1 ± 0.62 | +13.6 ± 0.84 | A, readout 0, the odd places | A, readout 1, the odd places |
| Seed 24, eighth, mirrored | 6 | 19.1, 17.0 | 17.3, 16.8 | −2.1 ± 0.99 | −0.5 ± 1.17 | −1.6 ± 1.69 | B, readout 1, the even places (within 2 SE) | B, readout 0, the even places (within 2 SE) |
| Seed 24, eighth, mirrored | 12 | 15.3, 12.1 | 16.3, 16.6 | −3.2 ± 0.72 | +0.3 ± 0.77 | −3.5 ± 0.91 | B, readout 1, the even places | B, readout 0, the even places |
| Seed 24, eighth, mirrored | 18 | 9.2, 9.9 | 11.0, 18.6 | +0.7 ± 0.25 | +7.6 ± 1.52 | −6.9 ± 1.32 | B, readout 1, the even places | B, readout 0, the even places |
| Seed 24, eighth, mirrored | 24 | 7.4, 9.0 | 9.0, 16.8 | +1.6 ± 1.33 | +7.8 ± 1.47 | −6.2 ± 2.32 | B, readout 1, the even places | B, readout 0, the even places |
| Seed 24, eighth, mirrored | 36 | 5.3, 6.9 | 7.2, 16.9 | +1.5 ± 0.74 | +9.7 ± 0.98 | −8.2 ± 1.45 | B, readout 1, the even places | B, readout 0, the even places |
| Seed 22, eighth, assignment | 6 | 24.4, 26.4 | 25.9, 25.8 | +2.0 ± 0.46 | −0.1 ± 0.58 | +2.1 ± 0.83 | A, readout 1, the even places | A, readout 0, the even places |
| Seed 22, eighth, assignment | 12 | 22.9, 25.4 | 24.8, 25.3 | +2.5 ± 0.45 | +0.5 ± 0.81 | +2.0 ± 0.92 | A, readout 1, the even places | A, readout 0, the even places |
| Seed 22, eighth, assignment | 18 | 24.4, 23.8 | 22.8, 26.8 | −0.6 ± 0.88 | +4.0 ± 0.58 | −4.6 ± 1.07 | B, readout 0, the odd places | B, readout 1, the odd places |
| Seed 22, eighth, assignment | 24 | 26.0, 25.8 | 21.3, 24.4 | −0.3 ± 0.99 | +3.1 ± 1.17 | −3.4 ± 1.97 | B, readout 0, the odd places (within 2 SE) | B, readout 1, the odd places (within 2 SE) |
| Seed 22, eighth, assignment | 36 | 24.7, 23.1 | 19.5, 23.2 | −1.6 ± 0.88 | +3.8 ± 0.81 | −5.4 ± 1.57 | B, readout 0, the odd places | B, readout 1, the odd places |

Layout 2, whose figures on each cell are H-47's on the other arm's cell:

| Cell | Block | A's trials: without replay, plain on A's | B's trials: the same | A's: plain less without replay | B's: plain less without replay | A's less B's | Side in layout 2 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
| Seed 23, first mapping, mirrored | 6 | 29.2, 30.9 | 25.0, 25.4 | +1.6 ± 0.46 | +0.4 ± 0.57 | +1.3 ± 0.67 | A, readout 1, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 12 | 24.2, 30.1 | 9.3, 7.4 | +6.0 ± 0.87 | −1.8 ± 0.85 | +7.8 ± 1.24 | A, readout 1, the odd places |
| Seed 23, first mapping, mirrored | 18 | 19.5, 30.1 | 7.6, 4.9 | +10.6 ± 1.65 | −2.8 ± 1.24 | +13.4 ± 1.44 | A, readout 1, the odd places |
| Seed 23, first mapping, mirrored | 24 | 17.3, 30.4 | 7.5, 4.3 | +13.2 ± 1.45 | −3.2 ± 0.67 | +16.4 ± 1.23 | A, readout 1, the odd places |
| Seed 23, first mapping, mirrored | 36 | 15.7, 29.2 | 4.0, 3.9 | +13.5 ± 1.09 | −0.1 ± 0.62 | +13.6 ± 0.84 | A, readout 1, the odd places |
| Seed 23, first mapping, assignment | 6 | 30.4, 31.0 | 24.6, 25.9 | +0.6 ± 0.31 | +1.3 ± 0.56 | −0.7 ± 0.61 | B, readout 1, the odd places (within 2 SE) |
| Seed 23, first mapping, assignment | 12 | 28.5, 29.6 | 23.8, 24.8 | +1.1 ± 0.70 | +1.0 ± 1.17 | +0.1 ± 1.46 | A, readout 0, the even places (within 2 SE) |
| Seed 23, first mapping, assignment | 18 | 28.5, 27.6 | 23.5, 24.8 | −1.0 ± 0.92 | +1.3 ± 0.69 | −2.3 ± 1.12 | B, readout 1, the odd places |
| Seed 23, first mapping, assignment | 24 | 30.0, 25.0 | 23.5, 25.9 | −5.0 ± 1.25 | +2.4 ± 0.38 | −7.4 ± 1.33 | B, readout 1, the odd places |
| Seed 23, first mapping, assignment | 36 | 29.3, 24.1 | 23.9, 24.2 | −5.2 ± 1.50 | +0.3 ± 0.88 | −5.5 ± 2.21 | B, readout 1, the odd places |

Counted, by layout and checkpoint: the side on each stimulus, on each readout and on each group, and of each how many with A's less B's
beyond twice its SE.

| Layout | Block | Side on A / on B (beyond 2 SE) | On readout 0 / readout 1 (beyond 2 SE) | On the odd / the even places (beyond 2 SE) | Cells |
| :--- | ---: | ---: | ---: | ---: | ---: |
| H-47 | 6 | 2 / 2 (1 / 0) | 2 / 2 (0 / 1) | 2 / 2 (0 / 1) | 4 |
| H-47 | 12 | 3 / 1 (2 / 1) | 1 / 3 (1 / 2) | 1 / 3 (1 / 2) | 4 |
| H-47 | 18 | 1 / 3 (1 / 3) | 3 / 1 (3 / 1) | 3 / 1 (3 / 1) | 4 |
| H-47 | 24 | 1 / 3 (1 / 2) | 3 / 1 (2 / 1) | 3 / 1 (2 / 1) | 4 |
| H-47 | 36 | 1 / 3 (1 / 3) | 3 / 1 (3 / 1) | 3 / 1 (3 / 1) | 4 |
| Layout 1 | 6 | 2 / 2 (1 / 0) | 2 / 2 (1 / 0) | 2 / 2 (0 / 1) | 4 |
| Layout 1 | 12 | 3 / 1 (2 / 1) | 3 / 1 (2 / 1) | 1 / 3 (1 / 2) | 4 |
| Layout 1 | 18 | 1 / 3 (1 / 3) | 1 / 3 (1 / 3) | 3 / 1 (3 / 1) | 4 |
| Layout 1 | 24 | 1 / 3 (1 / 2) | 1 / 3 (1 / 2) | 3 / 1 (2 / 1) | 4 |
| Layout 1 | 36 | 1 / 3 (1 / 3) | 1 / 3 (1 / 3) | 3 / 1 (3 / 1) | 4 |
| Layout 2 | 6 | 1 / 1 (0 / 0) | 0 / 2 (0 / 0) | 2 / 0 (0 / 0) | 2 |
| Layout 2 | 12 | 2 / 0 (1 / 0) | 1 / 1 (0 / 1) | 1 / 1 (1 / 0) | 2 |
| Layout 2 | 18 | 1 / 1 (1 / 1) | 0 / 2 (0 / 2) | 2 / 0 (2 / 0) | 2 |
| Layout 2 | 24 | 1 / 1 (1 / 1) | 0 / 2 (0 / 2) | 2 / 0 (2 / 0) | 2 |
| Layout 2 | 36 | 1 / 1 (1 / 1) | 0 / 2 (0 / 2) | 2 / 0 (2 / 0) | 2 |

- **H-47's and layout 1's counts by stimulus and by group are the same at every checkpoint**, and their counts by readout the other way
  round: from block 18 H-47 puts the side on readout 0 in 3 cells and layout 1 on readout 1 in 3.
- **From block 18 the side lies on the odd places** in 3 cells of 4 in H-47 and layout 1, and in both of layout 2's.

## 2. Against H-47

The side in each layout beside H-47's on the same cell, and what it shares with it: **the index**, the same readout index; **the group**,
the same group of units; **the stimulus**, the same stimulus, and so the same source units, since every layout here presents each stimulus
at H-47's place.

| Cell | Block | H-47 | Layout 1 | Layout 1 against H-47: the same | Layout 2 | Layout 2 against H-47: the same |
| :--- | ---: | :--- | :--- | :--- | :--- | :--- |
| Seed 23, first mapping, mirrored | 6 | B, readout 0, odd (within 2 SE) | B, readout 1, odd (within 2 SE) | group, stimulus | A, readout 1, odd (within 2 SE) | group |
| Seed 23, first mapping, mirrored | 12 | A, readout 1, even (within 2 SE) | A, readout 0, even (within 2 SE) | group, stimulus | A, readout 1, odd | index, stimulus |
| Seed 23, first mapping, mirrored | 18 | B, readout 0, odd | B, readout 1, odd | group, stimulus | A, readout 1, odd | group |
| Seed 23, first mapping, mirrored | 24 | B, readout 0, odd | B, readout 1, odd | group, stimulus | A, readout 1, odd | group |
| Seed 23, first mapping, mirrored | 36 | B, readout 0, odd | B, readout 1, odd | group, stimulus | A, readout 1, odd | group |
| Seed 23, first mapping, assignment | 6 | A, readout 0, odd (within 2 SE) | A, readout 1, odd (within 2 SE) | group, stimulus | B, readout 1, odd (within 2 SE) | group |
| Seed 23, first mapping, assignment | 12 | A, readout 0, odd | A, readout 1, odd | group, stimulus | A, readout 0, even (within 2 SE) | index, stimulus |
| Seed 23, first mapping, assignment | 18 | A, readout 0, odd | A, readout 1, odd | group, stimulus | B, readout 1, odd | group |
| Seed 23, first mapping, assignment | 24 | A, readout 0, odd | A, readout 1, odd | group, stimulus | B, readout 1, odd | group |
| Seed 23, first mapping, assignment | 36 | A, readout 0, odd | A, readout 1, odd | group, stimulus | B, readout 1, odd | group |
| Seed 24, eighth, mirrored | 6 | B, readout 1, even (within 2 SE) | B, readout 0, even (within 2 SE) | group, stimulus |  |  |
| Seed 24, eighth, mirrored | 12 | B, readout 1, even | B, readout 0, even | group, stimulus |  |  |
| Seed 24, eighth, mirrored | 18 | B, readout 1, even | B, readout 0, even | group, stimulus |  |  |
| Seed 24, eighth, mirrored | 24 | B, readout 1, even | B, readout 0, even | group, stimulus |  |  |
| Seed 24, eighth, mirrored | 36 | B, readout 1, even | B, readout 0, even | group, stimulus |  |  |
| Seed 22, eighth, assignment | 6 | A, readout 1, even | A, readout 0, even | group, stimulus |  |  |
| Seed 22, eighth, assignment | 12 | A, readout 1, even | A, readout 0, even | group, stimulus |  |  |
| Seed 22, eighth, assignment | 18 | B, readout 0, odd | B, readout 1, odd | group, stimulus |  |  |
| Seed 22, eighth, assignment | 24 | B, readout 0, odd (within 2 SE) | B, readout 1, odd (within 2 SE) | group, stimulus |  |  |
| Seed 22, eighth, assignment | 36 | B, readout 0, odd | B, readout 1, odd | group, stimulus |  |  |

| Layout | Block | The same readout index as H-47 (beyond 2 SE in both) | The same group of units (beyond 2 SE in both) | The same stimulus, so the same source units (beyond 2 SE in both) | Cells |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Layout 1 | 6 | 0 (0) | 4 (1) | 4 (1) | 4 |
| Layout 1 | 12 | 0 (0) | 4 (3) | 4 (3) | 4 |
| Layout 1 | 18 | 0 (0) | 4 (4) | 4 (4) | 4 |
| Layout 1 | 24 | 0 (0) | 4 (3) | 4 (3) | 4 |
| Layout 1 | 36 | 0 (0) | 4 (4) | 4 (4) | 4 |
| Layout 2 | 6 | 0 (0) | 2 (0) | 0 (0) | 2 |
| Layout 2 | 12 | 2 (0) | 0 (0) | 2 (0) | 2 |
| Layout 2 | 18 | 0 (0) | 2 (2) | 0 (0) | 2 |
| Layout 2 | 24 | 0 (0) | 2 (2) | 0 (0) | 2 |
| Layout 2 | 36 | 0 (0) | 2 (2) | 0 (0) | 2 |

- **Layout 1 shares the group and the stimulus with H-47 in every cell at every checkpoint, and the index in none.** It is H-47's side
  under the other name, at H-47's figures.
- **Layout 2 shares the group alone in both cells at blocks 6, 18, 24 and 36, and the index and the stimulus at block 12.** Its side is
  H-47's on the other arm's cell with the index exchanged. From block 18 that side lies on the odd places in both cells, as H-47's does,
  on the other stimulus and the other index.

A's count, both stimuli, under each form, beside H-47's on the same cell:

| Cell | Block | H-47: A's count without replay, plain on A's; plain less without replay | Layout 1: the same | Layout 2: the same |
| :--- | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | 55.0, 56.9; +1.9 ± 0.68 | 55.0, 56.9; +1.9 ± 0.68 | 54.2, 56.3; +2.0 ± 0.79 |
| Seed 23, first mapping, mirrored | 12 | 52.4, 54.4; +2.0 ± 1.26 | 52.4, 54.4; +2.0 ± 1.26 | 33.4, 37.6; +4.1 ± 1.18 |
| Seed 23, first mapping, mirrored | 18 | 52.0, 52.4; +0.3 ± 1.17 | 52.0, 52.4; +0.3 ± 1.17 | 27.2, 35.0; +7.9 ± 2.54 |
| Seed 23, first mapping, mirrored | 24 | 53.5, 50.9; −2.6 ± 1.30 | 53.5, 50.9; −2.6 ± 1.30 | 24.8, 34.8; +10.0 ± 1.90 |
| Seed 23, first mapping, mirrored | 36 | 53.1, 48.3; −4.9 ± 1.09 | 53.1, 48.3; −4.9 ± 1.09 | 19.7, 33.0; +13.4 ± 1.56 |
| Seed 23, first mapping, assignment | 6 | 54.2, 56.3; +2.0 ± 0.79 | 54.2, 56.3; +2.0 ± 0.79 | 55.0, 56.9; +1.9 ± 0.68 |
| Seed 23, first mapping, assignment | 12 | 33.4, 37.6; +4.1 ± 1.18 | 33.4, 37.6; +4.1 ± 1.18 | 52.4, 54.4; +2.0 ± 1.26 |
| Seed 23, first mapping, assignment | 18 | 27.2, 35.0; +7.9 ± 2.54 | 27.2, 35.0; +7.9 ± 2.54 | 52.0, 52.4; +0.3 ± 1.17 |
| Seed 23, first mapping, assignment | 24 | 24.8, 34.8; +10.0 ± 1.90 | 24.8, 34.8; +10.0 ± 1.90 | 53.5, 50.9; −2.6 ± 1.30 |
| Seed 23, first mapping, assignment | 36 | 19.7, 33.0; +13.4 ± 1.56 | 19.7, 33.0; +13.4 ± 1.56 | 53.1, 48.3; −4.9 ± 1.09 |
| Seed 24, eighth, mirrored | 6 | 36.4, 33.8; −2.6 ± 1.35 | 36.4, 33.8; −2.6 ± 1.35 |  |
| Seed 24, eighth, mirrored | 12 | 31.5, 28.6; −2.9 ± 1.19 | 31.5, 28.6; −2.9 ± 1.19 |  |
| Seed 24, eighth, mirrored | 18 | 20.2, 28.6; +8.3 ± 1.70 | 20.2, 28.6; +8.3 ± 1.70 |  |
| Seed 24, eighth, mirrored | 24 | 16.4, 25.8; +9.4 ± 1.58 | 16.4, 25.8; +9.4 ± 1.58 |  |
| Seed 24, eighth, mirrored | 36 | 12.5, 23.8; +11.3 ± 0.96 | 12.5, 23.8; +11.3 ± 0.96 |  |
| Seed 22, eighth, assignment | 6 | 50.3, 52.3; +1.9 ± 0.65 | 50.3, 52.3; +1.9 ± 0.65 |  |
| Seed 22, eighth, assignment | 12 | 47.6, 50.6; +3.0 ± 0.93 | 47.6, 50.6; +3.0 ± 0.93 |  |
| Seed 22, eighth, assignment | 18 | 47.2, 50.6; +3.4 ± 1.05 | 47.2, 50.6; +3.4 ± 1.05 |  |
| Seed 22, eighth, assignment | 24 | 47.3, 50.2; +2.8 ± 0.89 | 47.3, 50.2; +2.8 ± 0.89 |  |
| Seed 22, eighth, assignment | 36 | 44.2, 46.4; +2.2 ± 0.65 | 44.2, 46.4; +2.2 ± 0.65 |  |

Layout 1's A's count is H-47's on every row, and layout 2's on each cell H-47's on the other arm's cell.

## 3. The ties, the masks and where the pair's own units lie

Each layout's reads beside H-47's of the same cell, and layout 2's beside H-47's of the other arm's cell too: the tied trials in all 960
reads of a job, and how many reads carry H-47's masks. A read's three masks give each of its 64 trials' stimulus, whether it was right and
whether no readout was selected.

| Cell | Ties in H-47's 960 reads | In layout 1's | Layout 1's reads with H-47's stimuli mask; with all three masks H-47's | In layout 2's | Layout 2's reads with the stimuli mask of H-47's same cell; with all three masks of its same cell; of its other arm's cell |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 2316 | 2316 | 960 and 960 of 960 | 2929 | 960; 11; 960 of 960 |
| Seed 23, first mapping, assignment | 2929 | 2929 | 960 and 960 of 960 | 2316 | 960; 11; 960 of 960 |
| Seed 24, eighth, mirrored | 2968 | 2968 | 960 and 960 of 960 |  |  |
| Seed 22, eighth, assignment | 2152 | 2152 | 960 and 960 of 960 |  |  |

- **Layout 1's ties are H-47's on every trial of every read.**
- **Layout 2 presents H-47's stimuli on every trial and answers as H-47's other arm**: against its own cell 11 reads in each carry all
  three of H-47's masks, against the other arm's cell all 960.

Each cell's start split, the readout A answers, where each of the pair's own kinds lies, the side at block 36, and what four plain nights
on A's patterns moved of each own kind's weight onto each group's excitatory units, from the class matrix as section 4 reads it, the mean
over the three paths. H-47's rows are exploration 020's and 021's jobs on the same cells:

| Cell | Layout | A answers | C's own lie in | D's own lie in | Splits asserted | Side at block 36 | Plain nights: C's own onto the odd places' excitatory, M | onto the even places' | D's own onto the odd places' | onto the even places' |
| :--- | :--- | :--- | :--- | :--- | ---: | :--- | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | H-47 | readout 1, even | readout 0, odd (25 units) | readout 1, even (25 units) | 55 | B, readout 0, odd | +0.20 | −0.16 | −0.04 | +0.08 |
| Seed 23, first mapping, mirrored | Layout 1 | readout 0, even | readout 1, odd (25 units) | readout 0, even (25 units) | 55 | B, readout 1, odd | +0.20 | −0.16 | −0.04 | +0.08 |
| Seed 23, first mapping, mirrored | Layout 2 | readout 1, odd | readout 1, odd (25 units) | readout 0, even (25 units) | 55 | A, readout 1, odd | +0.20 | −0.26 | −0.25 | +0.18 |
| Seed 23, first mapping, assignment | H-47 | readout 0, odd | readout 0, odd (25 units) | readout 1, even (25 units) | 55 | A, readout 0, odd | +0.20 | −0.26 | −0.25 | +0.18 |
| Seed 23, first mapping, assignment | Layout 1 | readout 1, odd | readout 1, odd (25 units) | readout 0, even (25 units) | 55 | A, readout 1, odd | +0.20 | −0.26 | −0.25 | +0.18 |
| Seed 23, first mapping, assignment | Layout 2 | readout 0, even | readout 1, odd (25 units) | readout 0, even (25 units) | 55 | B, readout 1, odd | +0.20 | −0.16 | −0.04 | +0.08 |
| Seed 24, eighth, mirrored | H-47 | readout 0, odd | readout 0, odd (25 units) | readout 1, even (25 units) | 55 | B, readout 1, even | +0.29 | −0.30 | −0.15 | +0.13 |
| Seed 24, eighth, mirrored | Layout 1 | readout 1, odd | readout 1, odd (25 units) | readout 0, even (25 units) | 55 | B, readout 0, even | +0.29 | −0.30 | −0.15 | +0.13 |
| Seed 22, eighth, assignment | H-47 | readout 1, even | readout 0, odd (25 units) | readout 1, even (25 units) | 55 | B, readout 0, odd | −0.06 | +0.09 | 0.00 | +0.07 |
| Seed 22, eighth, assignment | Layout 1 | readout 0, even | readout 1, odd (25 units) | readout 0, even (25 units) | 55 | B, readout 1, odd | −0.06 | +0.09 | 0.00 | +0.07 |

- **C's own units lie inside readout 1 and D's inside readout 0 in all six jobs**, 25 units each, and every one of the 55 splits of each
  job asserted it. By group they lie where H-47 lays them: C's in the odd places, D's in the even.
- **Plain nights move more of each own kind's weight onto the group it lies in than onto the other** in 7 of 8 readings in H-47 and
  layout 1, at the same figures, and in all 4 of layout 2's. The eighth, in H-47 and layout 1, is C's own units on seed 22's eighth image
  from the assignment, −0.06 M onto the odd places and +0.09 M onto the even. A kind gains onto its own group and loses onto the other in 6
  of 8 there: on that cell D's own units gain onto both, 0.00 M onto the odd places and +0.07 M onto the even. In layout 2 all 4 do.

## 4. What plain nights take from each group's own synapses

What each night moved, summed over the four nights, the mean over the three paths, from the class matrix in the images before and after
each gap, as exploration 021 reads it, sorted by group. Inhibitory entries are magnitudes, so a + is more inhibition. H-47's rows are
exploration 020's and 021's jobs on the same cells.

| Cell | Layout | Form | Side at block 36 | The odd places' excitatory units onto all, M | The even places' | The odd places' onto their own | The even places' onto their own | The odd places' inhibitory onto all | The even places' | Inhibitory onto the odd places' excitatory | onto the even places' |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | H-47 | no-replay | B, readout 0, odd | +1.48 | +0.14 | +0.18 | −0.28 | −2.70 | −2.65 | −1.79 | −2.04 |
| Seed 23, first mapping, mirrored | H-47 | plain-first | B, readout 0, odd | −10.47 | −9.72 | −1.82 | −1.58 | +11.31 | +9.76 | +8.33 | +8.77 |
| Seed 23, first mapping, mirrored | Layout 1 | no-replay | B, readout 1, odd | +1.48 | +0.14 | +0.18 | −0.28 | −2.70 | −2.65 | −1.79 | −2.04 |
| Seed 23, first mapping, mirrored | Layout 1 | plain-first | B, readout 1, odd | −10.47 | −9.72 | −1.82 | −1.58 | +11.31 | +9.76 | +8.33 | +8.77 |
| Seed 23, first mapping, mirrored | Layout 2 | no-replay | A, readout 1, odd | +1.47 | +0.53 | +0.01 | −0.32 | −2.58 | −2.56 | −1.65 | −1.99 |
| Seed 23, first mapping, mirrored | Layout 2 | plain-first | A, readout 1, odd | −8.82 | −8.61 | +0.18 | −0.62 | +11.61 | +10.61 | +9.88 | +8.08 |
| Seed 23, first mapping, assignment | H-47 | no-replay | A, readout 0, odd | +1.47 | +0.53 | +0.01 | −0.32 | −2.58 | −2.56 | −1.65 | −1.99 |
| Seed 23, first mapping, assignment | H-47 | plain-first | A, readout 0, odd | −8.82 | −8.61 | +0.18 | −0.62 | +11.61 | +10.61 | +9.88 | +8.08 |
| Seed 23, first mapping, assignment | Layout 1 | no-replay | A, readout 1, odd | +1.47 | +0.53 | +0.01 | −0.32 | −2.58 | −2.56 | −1.65 | −1.99 |
| Seed 23, first mapping, assignment | Layout 1 | plain-first | A, readout 1, odd | −8.82 | −8.61 | +0.18 | −0.62 | +11.61 | +10.61 | +9.88 | +8.08 |
| Seed 23, first mapping, assignment | Layout 2 | no-replay | B, readout 1, odd | +1.48 | +0.14 | +0.18 | −0.28 | −2.70 | −2.65 | −1.79 | −2.04 |
| Seed 23, first mapping, assignment | Layout 2 | plain-first | B, readout 1, odd | −10.47 | −9.72 | −1.82 | −1.58 | +11.31 | +9.76 | +8.33 | +8.77 |
| Seed 24, eighth, mirrored | H-47 | no-replay | B, readout 1, even | +0.73 | −0.27 | −0.41 | −0.80 | +0.66 | +0.75 | +0.93 | +0.94 |
| Seed 24, eighth, mirrored | H-47 | plain-first | B, readout 1, even | −8.96 | −11.10 | −0.61 | −1.25 | +15.12 | +14.63 | +14.61 | +11.20 |
| Seed 24, eighth, mirrored | Layout 1 | no-replay | B, readout 0, even | +0.73 | −0.27 | −0.41 | −0.80 | +0.66 | +0.75 | +0.93 | +0.94 |
| Seed 24, eighth, mirrored | Layout 1 | plain-first | B, readout 0, even | −8.96 | −11.10 | −0.61 | −1.25 | +15.12 | +14.63 | +14.61 | +11.20 |
| Seed 22, eighth, assignment | H-47 | no-replay | B, readout 0, odd | −0.13 | −0.50 | −0.51 | −0.81 | +0.27 | +0.76 | +0.89 | +0.60 |
| Seed 22, eighth, assignment | H-47 | plain-first | B, readout 0, odd | −10.68 | −12.13 | −2.55 | −2.72 | +12.47 | +13.21 | +10.61 | +11.30 |
| Seed 22, eighth, assignment | Layout 1 | no-replay | B, readout 1, odd | −0.13 | −0.50 | −0.51 | −0.81 | +0.27 | +0.76 | +0.89 | +0.60 |
| Seed 22, eighth, assignment | Layout 1 | plain-first | B, readout 1, odd | −10.68 | −12.13 | −2.55 | −2.72 | +12.47 | +13.21 | +10.61 | +11.30 |

- **Layout 1's rows are H-47's on every figure, and layout 2's H-47's of the other arm's cell.**
- **Plain nights add more inhibition onto the excitatory units of the group stimulus A answers** in all four cells in H-47 and layout 1:
  onto the even places on seed 23's first mapping from the mirrored assignment, +8.77 M against +8.33 M, and on seed 22's eighth image from
  the assignment, +11.30 against +10.61 M; onto the odd places on seed 23's first mapping from the assignment, +9.88 against +8.08 M, and on
  seed 24's eighth image from the mirrored assignment, +14.61 against +11.20 M. In layout 2, where A answers the other group, so do they in
  both cells.
- **Against the side at block 36**, the group that gains more inhibition onto its excitatory units under plain nights is the side's in 1
  cell of 4 in H-47 and layout 1, seed 23's first mapping from the assignment, and in 1 of 2 in layout 2, its first mapping from the
  mirrored assignment.

## What it costs

The trial took 1 240 s and the six jobs 3 801 to 4 086 s, at most 28.4 % of a job's 240 minutes and each under 1.5 hours. H-47's jobs of
the same two forms on the same four cells took 3 819 to 4 049 s. Layout 2's two eighth-image cells were cut, as written in What ran. Seven
jobs ran, every one to the end.

## What it suggests

**In these runs a readout's index is a name.** Exchanging it, with the mapping in force named the other way, left every image, read, gain,
split and class matrix H-47's, the index exchanged. The selection releases a readout by comparing its count with the other's, the same rule
for either index. So the readout 0 that explorations 021 and 022 read the side lying on from block 18 is the odd places: there in three of
these four cells, and on the even places in the fourth.

**Exchanging the readouts' places, with the first mapping learned again, started H-47's other arm on both cells it ran.** Layout 2's first
mapping and every row after it are H-47's from the other arm under the other index. There, with the index a name, a stimulus answering the
even places from the assignment ran as the mirrored assignment, so layout 2 parted nothing from H-47 that H-47's two arms did not already.
Whether its eighth images would do the same is not run. On seed 23's first mapping those two arms put the side on the odd places from
block 18 from either arm, on B from the mirrored assignment and on A from the assignment: there the side stays with the group of units, not
with a stimulus.

**What the odd places hold.** They hold C's own units at place 5 and places 1 and 19, next to stimulus A's place 0; the even places hold
D's own units at place 16 and places 10 and 12, next to B's place 11. These runs do not part the side from any of those. On seed 24's eighth
image from the mirrored assignment it lies on the even places, so where it lies is not the parity alone in every cell.

Questions this suggests for a later exploration or a round:
- Readouts that are not the two parities, two other halves of the eighteen places, to part a group of units from its places, from the
  pair's own units and from the units next to each stimulus.
- Layout 2 on the two eighth images, which would say whether H-46's eight schedules run H-47's other arm as the first mapping's 24 blocks do.
- A read unit by unit, to see which of the side's group's units fire on its trials under each form.

## What did not run

- **Layout 2 on the two eighth-image cells**: its start image would need H-46's eight schedules learned again under its readouts.
- **The new layouts' other forms**, plain nights on both and the awake gap, so no reading here stands against the awake gap.
- **The new layouts on H-47's other eight cells.**
- **A layout whose readouts are not the odd and the even places.**
- **A read unit by unit**: a read gives each readout's spike count, not which units fired.
- **REM's depotentiations and the zero sum's rounds** are not split out of a night's change, as in explorations 020 to 022.
- **Any SE for sections 3 and 4**: the class matrix and the split are means over the three paths.
- **Any other share, form, network or seed.**

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_the_readouts_index_or_its_group_explore` | [42387230906](https://github.com/Menteion/Menteion/actions/runs/42387230906) | `9f5d5aa` | 1240 |
| `the_readouts_index_exchanged_a_first_mapping_on_seed_23_from_the_assignment_explore` | [42389139656](https://github.com/Menteion/Menteion/actions/runs/42389139656) | `9f5d5aa` | 4007 |
| `the_readouts_index_exchanged_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [42389139656](https://github.com/Menteion/Menteion/actions/runs/42389139656) | `9f5d5aa` | 3985 |
| `the_readouts_index_exchanged_the_eighth_image_on_seed_22_from_the_assignment_explore` | [42389139656](https://github.com/Menteion/Menteion/actions/runs/42389139656) | `9f5d5aa` | 4034 |
| `the_readouts_index_exchanged_the_eighth_image_on_seed_24_from_the_mirrored_assignment_explore` | [42389139656](https://github.com/Menteion/Menteion/actions/runs/42389139656) | `9f5d5aa` | 3801 |
| `the_readouts_places_exchanged_a_first_mapping_on_seed_23_from_the_assignment_explore` | [42389139656](https://github.com/Menteion/Menteion/actions/runs/42389139656) | `9f5d5aa` | 3997 |
| `the_readouts_places_exchanged_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [42389139656](https://github.com/Menteion/Menteion/actions/runs/42389139656) | `9f5d5aa` | 4086 |
