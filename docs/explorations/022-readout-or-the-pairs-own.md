---
status: exploratory
date: 2026-11-04
---

# 022: The readout or the pair's own units

**Branch**: `explore/022-readout-or-the-pairs-own`
**Commit**: `b4d4daa`, which adds tests and nothing under `src/`, on `c6a398a`, `main` at exploration 021's merge, with exploration
021's tag merged in (`3084a28`).
**Runs**: [41680929737](https://github.com/Menteion/Menteion/actions/runs/41680929737) (the harness's trial) and
[41684468588](https://github.com/Menteion/Menteion/actions/runs/41684468588) (the two layouts, four cells each), both at `scope=arms`. H-47's
layout is read from H-47's run [41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613) and, for its split and class
matrix, from exploration 020's runs [41500459562](https://github.com/Menteion/Menteion/actions/runs/41500459562) and
[41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777) and exploration 021's
[41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514); 021's exchange from its run
[41583848509](https://github.com/Menteion/Menteion/actions/runs/41583848509). None of those was run again.

[Exploration 021](021-where-the-gain-falls.md) read the side lying on readout 0's stimulus from block 18 in 9, 9 and 8 of H-47's twelve
cells, and, with A's and B's roles exchanged within a start, staying on the same units and readout from block 18 in all four cells it
exchanged. In H-47's layout and in that exchange, C's own units lie at place 5, inside readout 0, and D's at place 16, inside readout 1. So
021 could not tell a side that goes with readout 0 from one that goes with the readout that holds C's own units. This note moves the pair's
own units, C's to place 16 and D's to place 5, in two layouts on 021's four exchanged cells, and asks, at blocks 6, 12, 18, 24 and 36:

1. in each new layout, each stimulus's correct trials under the night without replay and the plain night on A's patterns, plain less
   without replay on each stimulus's trials, the side, the readout it lies on, and whether that readout holds C's own units;
2. against H-47's and 021's sides on the same cells, whether the side goes with the readout, with C's own units or with the role;
3. where the pair's own units lie, and what plain nights move of their weight onto each readout;
4. what plain nights take from each readout's own synapses, and onto which readout they add inhibition.

**The side** is ADR-0238's: at a checkpoint, the stimulus whose plain less without replay difference is the larger of the two. The note
writes no prediction and decides nothing. **Every figure here is exploratory**: from one run of each test on the repository's AArch64
host, in H-47's terms. A count is a mean over a cell's 48 reads, sixteen on each of three paths, and a difference carries its paired SE
unless another is named.

**In brief**
- **The runs are laid out as written.** In H-47's layout the trial held H-47's row at block 6 on path 0 for both forms. In layouts 1 and 2 it held the zero sum's groups,
  A's patterns, the pair's kinds and its readouts to H-47's, each in the order its layout reads them. Every split of the eight jobs held the
  pair's own units where its layout puts them, and every read's stimulus mask is H-47's.
- **From block 18 the side lies on the same readout in all four layouts, in all four cells**: readout 0 in three cells, readout 1 on seed
  24's eighth image from the mirrored assignment. A's less B's stands beyond twice its SE in all four layouts in 3, 2 and 3 of the cells at
  blocks 18, 24 and 36.
- **So from block 18 it goes with the readout in 4 cells of 4, with C's own units in none and with the role in none.** In layouts 1 and 2
  the side's readout holds D's own units in three cells and C's in one; in H-47 and 021 it held C's in three and D's in one.
- **At blocks 6 and 12 it goes with the readout in 2 and 1 cells**, and in 2 and 2 with the arrangement of the units. There H-47 and layout
  2, which lay every unit alike, put it on one readout, and 021 and layout 1, which also lay every unit alike, on the other. None of those
  four readings stands beyond twice its SE in all four layouts. It goes with C's own units alone, or the role alone, in no cell.
- **Layout 2 put the side on H-47's readout at every checkpoint in all four cells**, and layout 1 on 021's in 4, 3, 4, 4 and 4.
- **A's count without replay lies by arrangement from block 18**: at block 36 at 53.1, 19.7, 12.5 and 44.2 in H-47 and 52.5, 21.3, 12.8
  and 42.1 in layout 2, against 43.0, 34.3, 23.6 and 32.6 in 021 and 46.7, 34.3, 24.1 and 35.5 in layout 1.
- **Plain nights move the pair's own units' weight toward the readout they lie in** in 7 of each layout's 8 readings, two kinds in four
  cells. The exception, in each layout, is the units at place 5 on seed 22's eighth image from the assignment.
- **Plain nights add more inhibition onto stimulus A's readout's excitatory units in all four cells in H-47's layout and in layout 1.** In
  layout 2 they add more onto readout 0's in all four, A's readout in two. No figure of section 4's table moves by more than 0.82 M from
  H-47's.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
exploration 021 restates them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C sharing
26 of A's 51 source units and D 26 of B's, its answers the other way. Each of C and D presents its 26 shared units and 25 units of its own,
one in each of the other periods. A cell is named, as in H-47, by its seed, its image and the arm it starts from.

**Places.** A period of the ring is 20 units. H-47's stimulus A presents place 0 of each period and B place 11; readout 0 holds the odd
places but 11 and readout 1 the even places but 0. H-47 puts C's own units at place 5, inside readout 0, and D's at place 16, inside readout
1. The pair's readouts are A's less those two places, in every layout here.

**The four layouts.** Two are H-47's and exploration 021's, read from their logs; two ran here.

| Layout | A's and B's roles | Stimulus A presents | C's own units | D's own units | Its runs |
| :--- | :--- | :--- | :--- | :--- | :--- |
| H-47 | as H-47 has them | place 0 | place 5, readout 0 | place 16, readout 1 | H-47's |
| 021 exchanged | exchanged | place 11 | place 5, readout 0 | place 16, readout 1 | exploration 021's |
| Layout 1 | as H-47 has them | place 0 | place 16, readout 1 | place 5, readout 0 | here |
| Layout 2 | exchanged, as 021 exchanges them | place 11 | place 16, readout 1 | place 5, readout 0 | here |

In every layout each set of units answers the readout it answers in H-47 and in the start image: with the roles exchanged the mapping in
force is named the other way (021's `with_roles`). So layout 1 against H-47 moves the pair's own units alone, and layout 2 against 021 the
same; layout 2 against H-47 moves the roles and the pair's own units together, and so does layout 1 against 021.

`with_layout`, on the branch, lays a start out as 021's `with_roles` does, with the pair's own units at any two places, each taken out of
both of the pair's readouts. Against H-47:
- **What stays, in both layouts**: the derivation, the start image and its couplings, the zero sum's targets, the pair's readouts, the share,
  the forms, the paths and the reads' streams. Each trial's stimulus is drawn by its index, as in H-47.
- **What moves in layout 1**:
  - **Which own units go with which shared units.** C presents A's 26 shared units and the 25 units at place 16 that H-47's D presented as
    its own; D presents B's 26 shared units and the 25 at place 5 that were H-47's C's own.
  - **The zero sum's six groups** are H-47's six sets of units at H-47's values, C alone's and D alone's exchanged. A's patterns are H-47's,
    in H-47's order. Those among the patterns' targets that are the pair's own units are the same units, now of the other kind.
  - **The pairs of a unit C presents and one D presents within a local synapse's reach** (`PRIOR_WINDOW`) fall from H-47's 25 to none, as in 021's exchange.
- **What moves in layout 2**: every set of units plays H-47's part under the other name. Stimulus A presents H-47's B's units and B H-47's
  A's; C presents what H-47's D presented, shared and own, and D what H-47's C presented. The zero sum's six groups are H-47's at H-47's
  values in the exchanged order; A's patterns are H-47's, the new A's first; the pairs of C and D within a local synapse are H-47's 25. What
  moves is which units each trial presents, in A's blocks, A's reads and the pair's blocks: a trial that presented H-47's A's units presents
  its B's, and one that presented H-47's C's units presents what H-47's D presented.
- **Layout 1 against 021's exchange** is the same: every set of units plays 021's part under the other name, the pairs of C and D within a
  local synapse none in both. So the four layouts are two arrangements of the units, H-47's and 021's, each run under both namings.

**Two forms ran**, the night without replay and the plain night on A's patterns, which the side needs, on 021's four exchanged cells, in
each new layout: eight jobs. Each is exploration 020's job (`e020_job`) in its layout, with A's patterns' composition, each image's split and
class matrix, the image before each gap with its kinds, and `ReplayWatch` at every night. Its rows are held to nothing: H-47 ran none of
them.

**The split's assert.** `e020_split` asserts that C's own units lie in readout 0 and D's in readout 1. `e022_split` asserts instead that each
readout holds the pair's own kind the layout puts there: in layouts 1 and 2, D's own inside readout 0 and C's own inside readout 1. Its six
parts are each readout less its pair's own units and A's patterns' targets, those targets, and its pair's own units. The class matrix is
exploration 020's (`e020_classes`): a readout's excitatory units are its pattern targets, its other excitatory units, and the pair's own
kind inside it, which in layouts 1 and 2 is D's own in readout 0 and C's own in readout 1.

**A stimulus's trials and the SEs** are 021's: a read's `stimuli` mask splits its 64 trials, and a stimulus's plain less without replay
difference carries H-47's paired SE over its 48 differences; A's less B's carries the paired SE of its 48 differences of differences. **The
side** is ADR-0238's: at a checkpoint, the stimulus whose plain less without replay difference is the larger of the two. Counts of cells
within or beyond twice an SE are taken on the unrounded figures.

**Nothing under `src/` changed.**

## The runs are laid out as written

- **The trial** ran seed 23's first mapping from the mirrored assignment on path 0. It ran the pair's head, the first night without
  replay and the first plain night on A's patterns, both watched, and each image's split and read as it was of A and of the pair. It ran
  them in H-47's layout, then layout 1, then layout 2.
  - **In H-47's layout** each image's CRC-64, the two reads, A's couplings and the gain held H-47's row at block 6 for both forms, inside
    the test. Of its 545 lines, the 110 that H-47's job also prints are all lines of H-47's job, label aside. The other 435 are the watch's
    426, the split's three, the class matrix's three, the layout's line and the two notices that it held H-47's rows.
  - **In layouts 1 and 2** the zero sum's six values, A's patterns, the pair's six kinds and its two readouts held H-47's, inside the test,
    in the orders written in What ran. Each of the pair's stimuli presented its shared units and its own. In layout 2 the units C and D
    present, in their order, held H-47's D's and C's, and its pairs of C and D within a local synapse held H-47's 25. Layout 1's read 0.
  - **Every split** held the pair's own units where its layout puts them, three in each layout.
- **The eight jobs** share with H-47's job of the same cell 38 lines of 1 910 from a first mapping and 37 of 1 909 from the eighth image,
  in both layouts. These are the derivation and its calibration, the zero sum's two groups of 51 units and their values, and a first
  mapping's line. They share the same lines, and only those, with 021's exchanged job of the same cell. From the pair's groups on, every
  line differs.
- **Every read's `stimuli` mask is H-47's** read of the same cell, form, path, checkpoint, mapping and stream: all 3 840 reads of each
  layout, 1 920 of A and 1 920 of the pair. Each trial presents, by name, the stimulus H-47's trial of the same index presented. In layout 2 that
  is the other set of units on every trial.
- **Every split of the jobs** held the pair's own units where the layout puts them, 55 a job. The watch read `wrong 0` in all 192 nights.
  Every night without replay read `replays 0` and `depotentiations 0`, and each plain night 1 129 to 1 340 replays and 545 to 640
  depotentiations.

## 1. The side in layouts 1 and 2

Each stimulus's correct trials, of the trials a read presents of it, under the night without replay and the plain night on A's patterns;
plain less without replay on each stimulus's trials, and A's less B's, ± their paired SEs; the side, and which of the pair's own kinds lies
in the side's readout. In both layouts D's own units lie in readout 0 and C's in readout 1. Layout 1, A's and B's roles as H-47 has them:

| Cell | Block | Stimulus A presents, answers | A's trials: without replay, plain on A's | B's trials: the same | A's: plain less without replay | B's: plain less without replay | A's less B's | Side | The side's readout holds |
| :--- | ---: | :--- | ---: | ---: | ---: | ---: | ---: | :--- | :--- |
| Seed 23, first mapping, mirrored | 6 | H-47's A's, readout 1 | 27.2, 28.7 | 24.5, 25.4 | +1.5 ± 0.79 | +0.9 ± 0.79 | +0.6 ± 1.16 | A, readout 1 (within 2 SE) | C's own |
| Seed 23, first mapping, mirrored | 12 | H-47's A's, readout 1 | 21.7, 19.6 | 20.8, 24.4 | −2.1 ± 0.65 | +3.7 ± 1.06 | −5.8 ± 1.31 | B, readout 0 | D's own |
| Seed 23, first mapping, mirrored | 18 | H-47's A's, readout 1 | 24.8, 20.4 | 21.4, 23.1 | −4.4 ± 0.95 | +1.7 ± 0.76 | −6.1 ± 1.12 | B, readout 0 | D's own |
| Seed 23, first mapping, mirrored | 24 | H-47's A's, readout 1 | 26.2, 17.8 | 21.8, 22.8 | −8.4 ± 0.73 | +1.0 ± 0.90 | −9.4 ± 1.24 | B, readout 0 | D's own |
| Seed 23, first mapping, mirrored | 36 | H-47's A's, readout 1 | 24.3, 16.3 | 22.4, 21.4 | −8.0 ± 0.74 | −1.0 ± 1.47 | −7.0 ± 1.72 | B, readout 0 | D's own |
| Seed 23, first mapping, assignment | 6 | H-47's A's, readout 0 | 27.7, 30.5 | 27.1, 26.2 | +2.8 ± 0.69 | −0.9 ± 0.39 | +3.7 ± 0.87 | A, readout 0 | D's own |
| Seed 23, first mapping, assignment | 12 | H-47's A's, readout 0 | 25.6, 30.0 | 17.7, 10.5 | +4.4 ± 0.50 | −7.2 ± 0.92 | +11.6 ± 0.79 | A, readout 0 | D's own |
| Seed 23, first mapping, assignment | 18 | H-47's A's, readout 0 | 24.0, 30.2 | 14.7, 10.0 | +6.2 ± 0.66 | −4.7 ± 1.12 | +10.9 ± 1.36 | A, readout 0 | D's own |
| Seed 23, first mapping, assignment | 24 | H-47's A's, readout 0 | 22.0, 31.3 | 16.1, 12.6 | +9.3 ± 0.86 | −3.6 ± 1.11 | +12.9 ± 1.59 | A, readout 0 | D's own |
| Seed 23, first mapping, assignment | 36 | H-47's A's, readout 0 | 22.0, 30.6 | 12.4, 10.8 | +8.6 ± 1.03 | −1.6 ± 0.80 | +10.2 ± 1.45 | A, readout 0 | D's own |
| Seed 24, eighth, mirrored | 6 | H-47's A's, readout 0 | 23.1, 21.3 | 20.2, 20.8 | −1.8 ± 0.54 | +0.5 ± 0.56 | −2.3 ± 0.71 | B, readout 1 | C's own |
| Seed 24, eighth, mirrored | 12 | H-47's A's, readout 0 | 19.8, 17.0 | 19.3, 17.4 | −2.8 ± 1.12 | −1.9 ± 1.12 | −0.9 ± 1.37 | B, readout 1 (within 2 SE) | C's own |
| Seed 24, eighth, mirrored | 18 | H-47's A's, readout 0 | 15.3, 16.9 | 14.0, 20.2 | +1.7 ± 0.90 | +6.2 ± 1.04 | −4.5 ± 1.14 | B, readout 1 | C's own |
| Seed 24, eighth, mirrored | 24 | H-47's A's, readout 0 | 12.3, 15.6 | 12.3, 18.4 | +3.3 ± 1.51 | +6.0 ± 1.18 | −2.7 ± 1.01 | B, readout 1 | C's own |
| Seed 24, eighth, mirrored | 36 | H-47's A's, readout 0 | 13.7, 15.6 | 10.4, 17.5 | +1.9 ± 1.51 | +7.2 ± 1.24 | −5.3 ± 2.55 | B, readout 1 | C's own |
| Seed 22, eighth, assignment | 6 | H-47's A's, readout 1 | 21.0, 20.9 | 24.2, 24.3 | −0.1 ± 0.72 | +0.1 ± 0.72 | −0.2 ± 1.11 | B, readout 0 (within 2 SE) | D's own |
| Seed 22, eighth, assignment | 12 | H-47's A's, readout 1 | 20.3, 21.8 | 20.9, 22.7 | +1.5 ± 0.98 | +1.8 ± 0.95 | −0.3 ± 1.51 | B, readout 0 (within 2 SE) | D's own |
| Seed 22, eighth, assignment | 18 | H-47's A's, readout 1 | 22.4, 22.3 | 18.6, 23.9 | −0.1 ± 1.61 | +5.3 ± 0.85 | −5.4 ± 1.43 | B, readout 0 | D's own |
| Seed 22, eighth, assignment | 24 | H-47's A's, readout 1 | 24.4, 24.7 | 13.9, 21.4 | +0.4 ± 0.67 | +7.5 ± 1.05 | −7.1 ± 0.80 | B, readout 0 | D's own |
| Seed 22, eighth, assignment | 36 | H-47's A's, readout 1 | 22.4, 23.1 | 13.1, 19.4 | +0.7 ± 0.54 | +6.3 ± 0.45 | −5.6 ± 0.49 | B, readout 0 | D's own |

Layout 2, the roles exchanged:

| Cell | Block | Stimulus A presents, answers | A's trials: without replay, plain on A's | B's trials: the same | A's: plain less without replay | B's: plain less without replay | A's less B's | Side | The side's readout holds |
| :--- | ---: | :--- | ---: | ---: | ---: | ---: | ---: | :--- | :--- |
| Seed 23, first mapping, mirrored | 6 | H-47's B's, readout 0 | 25.5, 28.4 | 29.9, 30.9 | +2.9 ± 0.55 | +1.0 ± 0.36 | +1.9 ± 0.63 | A, readout 0 | D's own |
| Seed 23, first mapping, mirrored | 12 | H-47's B's, readout 0 | 24.7, 25.8 | 27.4, 29.2 | +1.1 ± 0.74 | +1.8 ± 0.60 | −0.6 ± 0.71 | B, readout 1 (within 2 SE) | C's own |
| Seed 23, first mapping, mirrored | 18 | H-47's B's, readout 0 | 24.5, 26.0 | 27.7, 27.3 | +1.5 ± 0.74 | −0.4 ± 0.80 | +1.9 ± 0.90 | A, readout 0 | D's own |
| Seed 23, first mapping, mirrored | 24 | H-47's B's, readout 0 | 24.2, 27.3 | 29.1, 26.4 | +3.1 ± 0.94 | −2.7 ± 0.88 | +5.9 ± 1.12 | A, readout 0 | D's own |
| Seed 23, first mapping, mirrored | 36 | H-47's B's, readout 0 | 23.9, 26.0 | 28.5, 23.6 | +2.1 ± 0.47 | −4.9 ± 0.52 | +7.0 ± 0.43 | A, readout 0 | D's own |
| Seed 23, first mapping, assignment | 6 | H-47's B's, readout 1 | 26.1, 26.2 | 28.3, 30.6 | +0.1 ± 0.78 | +2.3 ± 0.50 | −2.2 ± 1.01 | B, readout 0 | D's own |
| Seed 23, first mapping, assignment | 12 | H-47's B's, readout 1 | 13.1, 11.2 | 21.8, 28.9 | −1.9 ± 0.98 | +7.0 ± 1.32 | −9.0 ± 1.92 | B, readout 0 | D's own |
| Seed 23, first mapping, assignment | 18 | H-47's B's, readout 1 | 7.9, 5.6 | 17.4, 29.4 | −2.3 ± 0.71 | +11.9 ± 2.20 | −14.2 ± 1.76 | B, readout 0 | D's own |
| Seed 23, first mapping, assignment | 24 | H-47's B's, readout 1 | 7.1, 5.5 | 16.6, 29.6 | −1.5 ± 1.53 | +13.0 ± 1.86 | −14.5 ± 0.95 | B, readout 0 | D's own |
| Seed 23, first mapping, assignment | 36 | H-47's B's, readout 1 | 4.5, 4.5 | 16.8, 29.3 | 0.0 ± 1.03 | +12.5 ± 2.04 | −12.5 ± 1.20 | B, readout 0 | D's own |
| Seed 24, eighth, mirrored | 6 | H-47's B's, readout 1 | 18.4, 19.0 | 17.6, 16.2 | +0.6 ± 1.02 | −1.4 ± 0.93 | +2.1 ± 1.78 | A, readout 1 (within 2 SE) | C's own |
| Seed 24, eighth, mirrored | 12 | H-47's B's, readout 1 | 15.2, 16.1 | 13.7, 12.6 | +0.9 ± 0.68 | −1.0 ± 1.35 | +2.0 ± 1.57 | A, readout 1 (within 2 SE) | C's own |
| Seed 24, eighth, mirrored | 18 | H-47's B's, readout 1 | 11.4, 18.5 | 10.7, 11.0 | +7.1 ± 1.63 | +0.3 ± 1.76 | +6.8 ± 3.26 | A, readout 1 | C's own |
| Seed 24, eighth, mirrored | 24 | H-47's B's, readout 1 | 9.5, 18.5 | 5.7, 8.8 | +9.0 ± 1.88 | +3.1 ± 0.87 | +5.9 ± 2.58 | A, readout 1 | C's own |
| Seed 24, eighth, mirrored | 36 | H-47's B's, readout 1 | 8.1, 19.1 | 4.7, 6.8 | +11.0 ± 0.94 | +2.0 ± 0.80 | +9.0 ± 1.60 | A, readout 1 | C's own |
| Seed 22, eighth, assignment | 6 | H-47's B's, readout 0 | 25.4, 26.1 | 23.3, 24.6 | +0.6 ± 0.49 | +1.3 ± 0.74 | −0.6 ± 0.66 | B, readout 1 (within 2 SE) | C's own |
| Seed 22, eighth, assignment | 12 | H-47's B's, readout 0 | 25.9, 26.3 | 23.1, 25.5 | +0.4 ± 0.48 | +2.4 ± 0.62 | −2.0 ± 0.49 | B, readout 1 | C's own |
| Seed 22, eighth, assignment | 18 | H-47's B's, readout 0 | 22.7, 25.3 | 24.1, 24.9 | +2.6 ± 0.82 | +0.9 ± 0.77 | +1.7 ± 1.06 | A, readout 0 (within 2 SE) | D's own |
| Seed 22, eighth, assignment | 24 | H-47's B's, readout 0 | 20.7, 23.0 | 23.8, 21.8 | +2.3 ± 0.58 | −2.0 ± 1.38 | +4.3 ± 1.54 | A, readout 0 | D's own |
| Seed 22, eighth, assignment | 36 | H-47's B's, readout 0 | 18.9, 21.1 | 23.2, 20.8 | +2.2 ± 0.91 | −2.4 ± 0.97 | +4.6 ± 0.70 | A, readout 0 | D's own |

Counted, with H-47's and 021's on the same four cells beside them:

| Layout | Block | Side on A / on B (beyond 2 SE) | On readout 0 / readout 1 (beyond 2 SE) | On the readout that holds C's own / D's own (beyond 2 SE) | Cells |
| :--- | ---: | ---: | ---: | ---: | ---: |
| H-47 | 6 | 2 / 2 (1 / 0) | 2 / 2 (0 / 1) | 2 / 2 (0 / 1) | 4 |
| H-47 | 12 | 3 / 1 (2 / 1) | 1 / 3 (1 / 2) | 1 / 3 (1 / 2) | 4 |
| H-47 | 18 | 1 / 3 (1 / 3) | 3 / 1 (3 / 1) | 3 / 1 (3 / 1) | 4 |
| H-47 | 24 | 1 / 3 (1 / 2) | 3 / 1 (2 / 1) | 3 / 1 (2 / 1) | 4 |
| H-47 | 36 | 1 / 3 (1 / 3) | 3 / 1 (3 / 1) | 3 / 1 (3 / 1) | 4 |
| 021 | 6 | 2 / 2 (1 / 1) | 2 / 2 (1 / 1) | 2 / 2 (1 / 1) | 4 |
| 021 | 12 | 2 / 2 (0 / 1) | 4 / 0 (1 / 0) | 4 / 0 (1 / 0) | 4 |
| 021 | 18 | 3 / 1 (2 / 1) | 3 / 1 (2 / 1) | 3 / 1 (2 / 1) | 4 |
| 021 | 24 | 3 / 1 (2 / 1) | 3 / 1 (3 / 0) | 3 / 1 (3 / 0) | 4 |
| 021 | 36 | 3 / 1 (2 / 1) | 3 / 1 (3 / 0) | 3 / 1 (3 / 0) | 4 |
| Layout 1 | 6 | 2 / 2 (1 / 1) | 2 / 2 (1 / 1) | 2 / 2 (1 / 1) | 4 |
| Layout 1 | 12 | 1 / 3 (1 / 1) | 3 / 1 (2 / 0) | 1 / 3 (0 / 2) | 4 |
| Layout 1 | 18 | 1 / 3 (1 / 3) | 3 / 1 (3 / 1) | 1 / 3 (1 / 3) | 4 |
| Layout 1 | 24 | 1 / 3 (1 / 3) | 3 / 1 (3 / 1) | 1 / 3 (1 / 3) | 4 |
| Layout 1 | 36 | 1 / 3 (1 / 3) | 3 / 1 (3 / 1) | 1 / 3 (1 / 3) | 4 |
| Layout 2 | 6 | 2 / 2 (1 / 1) | 2 / 2 (2 / 0) | 2 / 2 (0 / 2) | 4 |
| Layout 2 | 12 | 1 / 3 (0 / 2) | 1 / 3 (1 / 1) | 3 / 1 (1 / 1) | 4 |
| Layout 2 | 18 | 3 / 1 (2 / 1) | 3 / 1 (2 / 1) | 1 / 3 (1 / 2) | 4 |
| Layout 2 | 24 | 3 / 1 (3 / 1) | 3 / 1 (3 / 1) | 1 / 3 (1 / 3) | 4 |
| Layout 2 | 36 | 3 / 1 (3 / 1) | 3 / 1 (3 / 1) | 1 / 3 (1 / 3) | 4 |

- **From block 18, in both layouts, the side lies on readout 0 in three cells and on readout 1 in one**, seed 24's eighth image from the
  mirrored assignment. A's less B's stands beyond twice its SE in all four cells at blocks 18, 24 and 36 in layout 1, and in 3, 4 and 4 in
  layout 2.
- **So from block 18 the side's readout holds D's own units in three cells and C's own in one**, in both layouts. In H-47 and 021, on the
  same four cells, it held C's own in three and D's own in one.
- **By stimulus**, from block 18 the side lies on B in three cells and A in one in layout 1, and on A in three and B in one in layout 2.
- **At blocks 6 and 12** it lies on readout 0 in 2 and 3 cells in layout 1 and in 2 and 1 in layout 2.
- **On seed 23's first mapping from the mirrored assignment, in layout 1**, plain nights on A's patterns less nights without replay read
  −2.1, −4.4, −8.4 and −8.0 on A's trials at blocks 12 to 36, against H-47's +1.1, −1.0, −5.0 and −5.2.

## 2. Against H-47 and 021

The side in each of the four layouts, by the readout it lies on and the stimulus there, and what it goes with. **The readout**: the same
readout in all four. **The role**: one readout in H-47 and layout 1, the other in 021 and layout 2, so the same stimulus in all four. **C's
own**: one readout in H-47 and 021, the other in layouts 1 and 2, so the readout that holds C's own units in all four, or D's in all four.
**The arrangement**: one readout in H-47 and layout 2, the other in 021 and layout 1. Every readout is the units' too: in every layout each
set of units answers the readout it answers in H-47.

| Cell | Block | H-47 (C's own in readout 0) | 021 exchanged (C's own in readout 0) | Layout 1 (C's own in readout 1) | Layout 2, exchanged (C's own in readout 1) | The side goes with |
| :--- | ---: | :--- | :--- | :--- | :--- | :--- |
| Seed 23, first mapping, mirrored | 6 | readout 0, B (within 2 SE) | readout 1, B (within 2 SE) | readout 1, A (within 2 SE) | readout 0, A | the arrangement |
| Seed 23, first mapping, mirrored | 12 | readout 1, A (within 2 SE) | readout 0, A (within 2 SE) | readout 0, B | readout 1, B (within 2 SE) | the arrangement |
| Seed 23, first mapping, mirrored | 18 | readout 0, B | readout 0, A | readout 0, B | readout 0, A | the readout |
| Seed 23, first mapping, mirrored | 24 | readout 0, B | readout 0, A | readout 0, B | readout 0, A | the readout |
| Seed 23, first mapping, mirrored | 36 | readout 0, B | readout 0, A | readout 0, B | readout 0, A | the readout |
| Seed 23, first mapping, assignment | 6 | readout 0, A (within 2 SE) | readout 0, B | readout 0, A | readout 0, B | the readout |
| Seed 23, first mapping, assignment | 12 | readout 0, A | readout 0, B | readout 0, A | readout 0, B | the readout |
| Seed 23, first mapping, assignment | 18 | readout 0, A | readout 0, B | readout 0, A | readout 0, B | the readout |
| Seed 23, first mapping, assignment | 24 | readout 0, A | readout 0, B | readout 0, A | readout 0, B | the readout |
| Seed 23, first mapping, assignment | 36 | readout 0, A | readout 0, B | readout 0, A | readout 0, B | the readout |
| Seed 24, eighth, mirrored | 6 | readout 1, B (within 2 SE) | readout 1, A | readout 1, B | readout 1, A (within 2 SE) | the readout |
| Seed 24, eighth, mirrored | 12 | readout 1, B | readout 0, B (within 2 SE) | readout 1, B (within 2 SE) | readout 1, A (within 2 SE) | one layout apart |
| Seed 24, eighth, mirrored | 18 | readout 1, B | readout 1, A | readout 1, B | readout 1, A | the readout |
| Seed 24, eighth, mirrored | 24 | readout 1, B | readout 1, A (within 2 SE) | readout 1, B | readout 1, A | the readout |
| Seed 24, eighth, mirrored | 36 | readout 1, B | readout 1, A (within 2 SE) | readout 1, B | readout 1, A | the readout |
| Seed 22, eighth, assignment | 6 | readout 1, A | readout 0, A (within 2 SE) | readout 0, B (within 2 SE) | readout 1, B (within 2 SE) | the arrangement |
| Seed 22, eighth, assignment | 12 | readout 1, A | readout 0, A (within 2 SE) | readout 0, B (within 2 SE) | readout 1, B | the arrangement |
| Seed 22, eighth, assignment | 18 | readout 0, B | readout 0, A (within 2 SE) | readout 0, B | readout 0, A (within 2 SE) | the readout |
| Seed 22, eighth, assignment | 24 | readout 0, B (within 2 SE) | readout 0, A | readout 0, B | readout 0, A | the readout |
| Seed 22, eighth, assignment | 36 | readout 0, B | readout 0, A | readout 0, B | readout 0, A | the readout |

Counted:

| Block | The same readout in all four (beyond 2 SE in all four) | With the role: H-47 and layout 1 alike, 021 and layout 2 alike | With C's own: H-47 and 021 alike, layouts 1 and 2 alike | With the arrangement: H-47 and layout 2 alike, 021 and layout 1 alike | One layout apart from the other three | Cells |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | 2 (0) | 0 (0) | 0 (0) | 2 (0) | 0 (0) | 4 |
| 12 | 1 (1) | 0 (0) | 0 (0) | 2 (0) | 1 (0) | 4 |
| 18 | 4 (3) | 0 (0) | 0 (0) | 0 (0) | 0 (0) | 4 |
| 24 | 4 (2) | 0 (0) | 0 (0) | 0 (0) | 0 (0) | 4 |
| 36 | 4 (3) | 0 (0) | 0 (0) | 0 (0) | 0 (0) | 4 |

Each new layout against each old one, the same readout or the other, on the same four cells:

| Layouts | Block | The same readout (beyond 2 SE in both) | The other readout (beyond 2 SE in both) | Cells |
| :--- | ---: | ---: | ---: | ---: |
| Layout 1 against H-47 | 6 | 2 (0) | 2 (0) | 4 |
| Layout 1 against H-47 | 12 | 2 (1) | 2 (0) | 4 |
| Layout 1 against H-47 | 18 | 4 (4) | 0 (0) | 4 |
| Layout 1 against H-47 | 24 | 4 (3) | 0 (0) | 4 |
| Layout 1 against H-47 | 36 | 4 (4) | 0 (0) | 4 |
| Layout 2 against 021 | 6 | 2 (1) | 2 (0) | 4 |
| Layout 2 against 021 | 12 | 1 (1) | 3 (0) | 4 |
| Layout 2 against 021 | 18 | 4 (3) | 0 (0) | 4 |
| Layout 2 against 021 | 24 | 4 (3) | 0 (0) | 4 |
| Layout 2 against 021 | 36 | 4 (3) | 0 (0) | 4 |
| Layout 2 against H-47 | 6 | 4 (0) | 0 (0) | 4 |
| Layout 2 against H-47 | 12 | 4 (2) | 0 (0) | 4 |
| Layout 2 against H-47 | 18 | 4 (3) | 0 (0) | 4 |
| Layout 2 against H-47 | 24 | 4 (3) | 0 (0) | 4 |
| Layout 2 against H-47 | 36 | 4 (4) | 0 (0) | 4 |
| Layout 1 against 021 | 6 | 4 (2) | 0 (0) | 4 |
| Layout 1 against 021 | 12 | 3 (1) | 1 (0) | 4 |
| Layout 1 against 021 | 18 | 4 (3) | 0 (0) | 4 |
| Layout 1 against 021 | 24 | 4 (3) | 0 (0) | 4 |
| Layout 1 against 021 | 36 | 4 (3) | 0 (0) | 4 |
| Layout 1 against Layout 2 | 6 | 2 (1) | 2 (0) | 4 |
| Layout 1 against Layout 2 | 12 | 2 (1) | 2 (0) | 4 |
| Layout 1 against Layout 2 | 18 | 4 (3) | 0 (0) | 4 |
| Layout 1 against Layout 2 | 24 | 4 (4) | 0 (0) | 4 |
| Layout 1 against Layout 2 | 36 | 4 (4) | 0 (0) | 4 |

- **From block 18 the side goes with the readout in all four cells at every checkpoint.** With C's own units, the role or the arrangement
  it goes in none. Beyond twice its SE in all four layouts: 3, 2 and 3 cells at blocks 18, 24 and 36. Seed 24's eighth image from the
  mirrored assignment is within at blocks 24 and 36 in 021 alone. Seed 22's eighth image from the assignment is within at block 18 in 021
  and layout 2, and at block 24 in H-47.
- **At block 6 the side goes with the readout in two cells and with the arrangement in two**, seed 23's first mapping from the mirrored
  assignment and seed 22's eighth image from the assignment. At block 12 it goes with the readout in one cell, with the arrangement in the
  same two and one layout apart in one: seed 24's eighth image from the mirrored assignment, on readout 0 in 021 alone. No cell that goes with
  the arrangement stands beyond twice its SE in all four layouts.
- **Pair by pair, the layouts that moved only the pair's own units** left the side on the same readout in 2 and 2 cells at blocks 6 and 12
  (layout 1 against H-47) and in 2 and 1 (layout 2 against 021); from block 18 in 4 of 4. **Layout 2 against H-47** left it on the same
  readout in all four cells at every checkpoint, beyond twice the SE in both in 0, 2, 3, 3 and 4; **layout 1 against 021** in 4, 3, 4, 4 and
  4.

A's count, both stimuli, under each of the two forms and plain less without replay, in each layout:

| Cell | Block | H-47: A's count without replay, plain on A's; plain less without replay | 021 exchanged: the same | Layout 1: the same | Layout 2: the same |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | 55.0, 56.9; +1.9 ± 0.68 | 51.1, 55.8; +4.7 ± 1.11 | 51.7, 54.1; +2.4 ± 1.08 | 55.4, 59.3; +4.0 ± 0.67 |
| Seed 23, first mapping, mirrored | 12 | 52.4, 54.4; +2.0 ± 1.26 | 42.8, 46.1; +3.4 ± 2.48 | 42.4, 44.0; +1.6 ± 1.18 | 52.1, 55.0; +2.9 ± 1.15 |
| Seed 23, first mapping, mirrored | 18 | 52.0, 52.4; +0.3 ± 1.17 | 44.3, 44.1; −0.2 ± 2.74 | 46.2, 43.5; −2.8 ± 1.30 | 52.2, 53.3; +1.1 ± 1.25 |
| Seed 23, first mapping, mirrored | 24 | 53.5, 50.9; −2.6 ± 1.30 | 46.4, 42.9; −3.5 ± 2.74 | 48.0, 40.6; −7.3 ± 1.07 | 53.3, 53.7; +0.4 ± 1.44 |
| Seed 23, first mapping, mirrored | 36 | 53.1, 48.3; −4.9 ± 1.09 | 43.0, 38.3; −4.7 ± 2.26 | 46.7, 37.7; −9.0 ± 1.56 | 52.5, 49.6; −2.8 ± 0.89 |
| Seed 23, first mapping, assignment | 6 | 54.2, 56.3; +2.0 ± 0.79 | 52.5, 55.3; +2.8 ± 0.82 | 54.8, 56.7; +1.9 ± 0.71 | 54.4, 56.7; +2.3 ± 0.84 |
| Seed 23, first mapping, assignment | 12 | 33.4, 37.6; +4.1 ± 1.18 | 39.7, 41.0; +1.3 ± 1.22 | 43.3, 40.5; −2.8 ± 1.25 | 35.0, 40.0; +5.1 ± 1.32 |
| Seed 23, first mapping, assignment | 18 | 27.2, 35.0; +7.9 ± 2.54 | 38.7, 43.4; +4.7 ± 1.04 | 38.7, 40.2; +1.5 ± 1.24 | 25.3, 35.0; +9.7 ± 2.75 |
| Seed 23, first mapping, assignment | 24 | 24.8, 34.8; +10.0 ± 1.90 | 36.3, 43.2; +6.8 ± 0.77 | 38.1, 43.8; +5.7 ± 1.20 | 23.7, 35.1; +11.4 ± 3.27 |
| Seed 23, first mapping, assignment | 36 | 19.7, 33.0; +13.4 ± 1.56 | 34.3, 40.9; +6.6 ± 1.25 | 34.3, 41.4; +7.1 ± 1.12 | 21.3, 33.7; +12.4 ± 3.01 |
| Seed 24, eighth, mirrored | 6 | 36.4, 33.8; −2.6 ± 1.35 | 43.6, 42.1; −1.5 ± 0.93 | 43.3, 42.1; −1.2 ± 0.83 | 36.0, 35.2; −0.8 ± 0.81 |
| Seed 24, eighth, mirrored | 12 | 31.5, 28.6; −2.9 ± 1.19 | 39.2, 41.1; +1.9 ± 1.14 | 39.1, 34.4; −4.7 ± 1.77 | 28.9, 28.8; −0.1 ± 1.44 |
| Seed 24, eighth, mirrored | 18 | 20.2, 28.6; +8.3 ± 1.70 | 32.0, 38.6; +6.6 ± 1.04 | 29.3, 37.1; +7.8 ± 1.57 | 22.1, 29.5; +7.4 ± 0.93 |
| Seed 24, eighth, mirrored | 24 | 16.4, 25.8; +9.4 ± 1.58 | 24.4, 36.4; +12.0 ± 1.16 | 24.6, 34.0; +9.3 ± 2.51 | 15.2, 27.3; +12.0 ± 1.39 |
| Seed 24, eighth, mirrored | 36 | 12.5, 23.8; +11.3 ± 0.96 | 23.6, 34.4; +10.8 ± 1.47 | 24.1, 33.1; +9.1 ± 1.11 | 12.8, 25.9; +13.1 ± 0.68 |
| Seed 22, eighth, assignment | 6 | 50.3, 52.3; +1.9 ± 0.65 | 43.0, 43.4; +0.4 ± 0.49 | 45.2, 45.2; 0.0 ± 0.92 | 48.7, 50.6; +1.9 ± 1.06 |
| Seed 22, eighth, assignment | 12 | 47.6, 50.6; +3.0 ± 0.93 | 43.9, 45.0; +1.1 ± 0.90 | 41.1, 44.5; +3.3 ± 1.20 | 49.0, 51.8; +2.8 ± 0.99 |
| Seed 22, eighth, assignment | 18 | 47.2, 50.6; +3.4 ± 1.05 | 41.5, 45.1; +3.5 ± 1.18 | 41.0, 46.2; +5.2 ± 2.13 | 46.8, 50.2; +3.4 ± 1.19 |
| Seed 22, eighth, assignment | 24 | 47.3, 50.2; +2.8 ± 0.89 | 36.3, 39.2; +3.0 ± 1.69 | 38.2, 46.1; +7.9 ± 1.57 | 44.6, 44.8; +0.2 ± 1.45 |
| Seed 22, eighth, assignment | 36 | 44.2, 46.4; +2.2 ± 0.65 | 32.6, 38.8; +6.2 ± 0.23 | 35.5, 42.5; +7.0 ± 0.88 | 42.1, 41.9; −0.2 ± 1.71 |

- **A's count without replay lies by arrangement from block 18.** In every cell H-47 and layout 2 lie within 2.7, 2.8 and 3.7 of each
  other at blocks 18, 24 and 36, and so do 021 and layout 1; each lies at least 5.2, 5.3 and 5.8 from the other arrangement's two. At block
  12 the two figures are 3.6 and 3.7, and at block 6 2.3 and 0.4. Under plain nights on A's patterns they are 3.2 and 4.0 at block 18 and
  overlap at the other four checkpoints.
- **Plain less without replay of A's count at block 36 keeps its sign in all four layouts on three cells of four.** On seed 22's eighth
  image from the assignment it reads +2.2 ± 0.65, +6.2 ± 0.23 and +7.0 ± 0.88 in H-47, 021 and layout 1, and −0.2 ± 1.71 in layout 2.

## 3. Where the pair's own units lie

Each cell's start split, the readout A answers in the layout, where each of the pair's own kinds lies, the side at block 36, and what four
plain nights on A's patterns moved of each own kind's weight onto each readout's excitatory units, the mean over the three paths. H-47's
rows are exploration 020's job on the same cells:

| Cell | Layout | A answers | C's own lie in | D's own lie in | Splits asserted | Side at block 36 | Plain nights: C's own onto readout 0's excitatory, M | onto readout 1's | D's own onto readout 0's | onto readout 1's |
| :--- | :--- | :--- | :--- | :--- | ---: | :--- | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | H-47 | readout 1 | readout 0 (25 units) | readout 1 (25 units) | 55 | B, readout 0 | +0.20 | −0.16 | −0.04 | +0.08 |
| Seed 23, first mapping, mirrored | Layout 1 | readout 1 | readout 1 (25 units) | readout 0 (25 units) | 55 | B, readout 0 | −0.04 | +0.09 | +0.18 | −0.19 |
| Seed 23, first mapping, mirrored | Layout 2 | readout 0 | readout 1 (25 units) | readout 0 (25 units) | 55 | A, readout 0 | −0.06 | +0.09 | +0.19 | −0.13 |
| Seed 23, first mapping, assignment | H-47 | readout 0 | readout 0 (25 units) | readout 1 (25 units) | 55 | A, readout 0 | +0.20 | −0.26 | −0.25 | +0.18 |
| Seed 23, first mapping, assignment | Layout 1 | readout 0 | readout 1 (25 units) | readout 0 (25 units) | 55 | A, readout 0 | −0.22 | +0.18 | +0.24 | −0.22 |
| Seed 23, first mapping, assignment | Layout 2 | readout 1 | readout 1 (25 units) | readout 0 (25 units) | 55 | B, readout 0 | −0.21 | +0.21 | +0.20 | −0.26 |
| Seed 24, eighth, mirrored | H-47 | readout 0 | readout 0 (25 units) | readout 1 (25 units) | 55 | B, readout 1 | +0.29 | −0.30 | −0.15 | +0.13 |
| Seed 24, eighth, mirrored | Layout 1 | readout 0 | readout 1 (25 units) | readout 0 (25 units) | 55 | B, readout 1 | −0.08 | +0.09 | +0.20 | −0.15 |
| Seed 24, eighth, mirrored | Layout 2 | readout 1 | readout 1 (25 units) | readout 0 (25 units) | 55 | A, readout 1 | −0.06 | +0.10 | +0.34 | −0.36 |
| Seed 22, eighth, assignment | H-47 | readout 1 | readout 0 (25 units) | readout 1 (25 units) | 55 | B, readout 0 | −0.06 | +0.09 | 0.00 | +0.07 |
| Seed 22, eighth, assignment | Layout 1 | readout 1 | readout 1 (25 units) | readout 0 (25 units) | 55 | B, readout 0 | +0.01 | +0.10 | +0.01 | +0.02 |
| Seed 22, eighth, assignment | Layout 2 | readout 0 | readout 1 (25 units) | readout 0 (25 units) | 55 | A, readout 0 | −0.01 | +0.07 | −0.08 | +0.08 |

- **C's own units lie inside readout 1 and D's inside readout 0 in all eight jobs**, 25 units each, and every one of the 55 splits of each
  job asserted it. In H-47's layout they lie the other way, as exploration 021 read.
- **The parts of each readout are the same in all three layouts** on a cell: in layouts 1 and 2 readout 0's own units are D's, the units
  at place 5 that were H-47's C's own.
- **Plain nights move each own kind's weight toward the readout it lies in** in 7 of 8 readings in each layout: onto that readout less
  onto the other, +0.07 to +0.59 M in H-47's layout, +0.09 to +0.46 M in layout 1 and +0.07 to +0.70 M in layout 2. The eighth, in each
  layout, is the units at place 5 on seed 22's eighth image from the assignment: C's own in H-47's layout at −0.14 M, D's own in layouts 1
  and 2 at −0.01 and −0.16 M.

## 4. What plain nights take from each readout's own synapses

What each night moved, summed over the four nights, the mean over the three paths, from the class matrix in the images before and after
each gap, as exploration 021 reads it. A readout's excitatory units are its pattern targets, its other excitatory units, and the pair's own
kind inside it: C's own in readout 0 in H-47's layout, D's own in layouts 1 and 2. Inhibitory entries are magnitudes, so a + is more
inhibition. H-47's rows are exploration 020's job on the same cells; 021's exchange ran no class matrix.

| Cell | Layout | Form | Side at block 36 | Readout 0's excitatory units onto all, M | Readout 1's | Readout 0's onto its own | Readout 1's onto its own | Readout 0's inhibitory onto all | Readout 1's | Inhibitory onto readout 0's excitatory | onto readout 1's |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | H-47 | no-replay | B, readout 0 | +1.48 | +0.14 | +0.18 | −0.28 | −2.70 | −2.65 | −1.79 | −2.04 |
| Seed 23, first mapping, mirrored | H-47 | plain-first | B, readout 0 | −10.47 | −9.72 | −1.82 | −1.58 | +11.31 | +9.76 | +8.33 | +8.77 |
| Seed 23, first mapping, mirrored | Layout 1 | no-replay | B, readout 0 | +1.56 | +0.48 | +0.11 | −0.25 | −2.68 | −2.61 | −1.69 | −2.01 |
| Seed 23, first mapping, mirrored | Layout 1 | plain-first | B, readout 0 | −10.53 | −9.73 | −1.89 | −1.58 | +11.19 | +9.48 | +8.20 | +8.57 |
| Seed 23, first mapping, mirrored | Layout 2 | no-replay | A, readout 0 | +1.46 | +0.15 | +0.17 | −0.28 | −2.73 | −2.67 | −1.81 | −2.07 |
| Seed 23, first mapping, mirrored | Layout 2 | plain-first | A, readout 0 | −10.31 | −9.86 | −1.98 | −1.54 | +11.37 | +9.62 | +8.53 | +8.52 |
| Seed 23, first mapping, assignment | H-47 | no-replay | A, readout 0 | +1.47 | +0.53 | +0.01 | −0.32 | −2.58 | −2.56 | −1.65 | −1.99 |
| Seed 23, first mapping, assignment | H-47 | plain-first | A, readout 0 | −8.82 | −8.61 | +0.18 | −0.62 | +11.61 | +10.61 | +9.88 | +8.08 |
| Seed 23, first mapping, assignment | Layout 1 | no-replay | A, readout 0 | +1.46 | +0.24 | +0.17 | −0.30 | −2.70 | −2.68 | −1.77 | −2.10 |
| Seed 23, first mapping, assignment | Layout 1 | plain-first | A, readout 0 | −8.89 | −8.91 | +0.30 | −0.66 | +11.62 | +11.00 | +10.14 | +8.21 |
| Seed 23, first mapping, assignment | Layout 2 | no-replay | B, readout 0 | +1.43 | +0.56 | +0.02 | −0.27 | −2.58 | −2.59 | −1.69 | −1.98 |
| Seed 23, first mapping, assignment | Layout 2 | plain-first | B, readout 0 | −9.64 | −8.60 | −0.17 | −0.62 | +11.49 | +10.49 | +9.47 | +8.34 |
| Seed 24, eighth, mirrored | H-47 | no-replay | B, readout 1 | +0.73 | −0.27 | −0.41 | −0.80 | +0.66 | +0.75 | +0.93 | +0.94 |
| Seed 24, eighth, mirrored | H-47 | plain-first | B, readout 1 | −8.96 | −11.10 | −0.61 | −1.25 | +15.12 | +14.63 | +14.61 | +11.20 |
| Seed 24, eighth, mirrored | Layout 1 | no-replay | B, readout 1 | +0.64 | −0.42 | −0.53 | −0.84 | +0.65 | +0.75 | +0.99 | +0.90 |
| Seed 24, eighth, mirrored | Layout 1 | plain-first | B, readout 1 | −9.00 | −11.08 | −0.72 | −1.24 | +14.97 | +14.48 | +14.59 | +11.04 |
| Seed 24, eighth, mirrored | Layout 2 | no-replay | A, readout 1 | +0.69 | −0.18 | −0.45 | −0.77 | +0.71 | +0.77 | +0.97 | +0.96 |
| Seed 24, eighth, mirrored | Layout 2 | plain-first | A, readout 1 | −8.77 | −10.85 | −0.46 | −1.52 | +15.05 | +14.70 | +14.09 | +11.64 |
| Seed 22, eighth, assignment | H-47 | no-replay | B, readout 0 | −0.13 | −0.50 | −0.51 | −0.81 | +0.27 | +0.76 | +0.89 | +0.60 |
| Seed 22, eighth, assignment | H-47 | plain-first | B, readout 0 | −10.68 | −12.13 | −2.55 | −2.72 | +12.47 | +13.21 | +10.61 | +11.30 |
| Seed 22, eighth, assignment | Layout 1 | no-replay | B, readout 0 | −0.15 | −0.48 | −0.56 | −0.77 | +0.34 | +0.77 | +0.95 | +0.62 |
| Seed 22, eighth, assignment | Layout 1 | plain-first | B, readout 0 | −10.72 | −12.24 | −2.44 | −2.72 | +12.44 | +13.04 | +10.52 | +11.18 |
| Seed 22, eighth, assignment | Layout 2 | no-replay | A, readout 0 | −0.12 | −0.54 | −0.53 | −0.83 | +0.29 | +0.76 | +0.88 | +0.61 |
| Seed 22, eighth, assignment | Layout 2 | plain-first | A, readout 0 | −11.07 | −12.20 | −2.78 | −2.60 | +12.46 | +13.20 | +11.08 | +10.83 |

- **Plain nights add more inhibition onto stimulus A's readout's excitatory units in all four cells in H-47's layout and in layout 1**,
  by 0.37 to 3.55 M in layout 1, the same readouts as H-47's. **In layout 2 they add more onto readout 0's in all four cells**, by 0.01 to
  2.45 M. That is stimulus A's readout in two cells, by 0.01 M on seed 23's first mapping from the mirrored assignment and by 0.25 M on seed
  22's eighth image from the assignment. In the other two it is the readout of H-47's A's units, by 1.13 and 2.45 M.
- **Against the side at block 36**, the readout that gains more inhibition onto its excitatory units is the side's in 1, 1 and 3 cells of 4
  in H-47's layout, layout 1 and layout 2.
- **Each readout's excitatory units onto its own** lost, the two readouts together, 0.44 to 5.27 M in H-47's layout, 0.35 to 5.15 M in
  layout 1 and 0.80 to 5.38 M in layout 2, in the same order of cells.
- **No figure of the table moves by more than 0.82 M from H-47's** on the same cell and form: at most 0.34 and 0.40 M in layout 1, under
  nights without replay and plain nights, and 0.09 and 0.82 M in layout 2.

## What it costs

The trial took 1 060 s and the eight jobs 3 820 to 4 089 s, at most 28.4 % of a job's 240 minutes and each under 1.5 hours. H-47's jobs of
the same two forms took 3 819 to 4 049 s. Nothing was cut: both layouts ran on all four of 021's exchanged cells. Nine jobs ran, every one
to the end.

## What it suggests

**From block 18, on these four cells, the side stays with its units and their readout in every layout.** It stays whether the pair's own
units move (layout 1 against H-47, layout 2 against 021), the roles are exchanged (021 against H-47, layout 2 against layout 1), or both
(layout 2 against H-47, layout 1 against 021). In layouts 1 and 2 that readout holds D's own units in three cells of four. No layout here
parts a set of units from the readout it answers, so these runs cannot tell the units from their readout.

**At blocks 6 and 12, where the side moved, it moved with the arrangement.** In two cells at each it lay on one readout in H-47 and layout 2
and on the other in 021 and layout 1, within twice the SE in at least one layout. Layout 2 differs from H-47, and layout 1 from 021, in
which units each trial presents, the zero sum's groups' order and A's patterns' order.

**A's count without replay lies by arrangement from block 18.** In every cell each arrangement's two layouts lie nearer each other than
either lies to the other arrangement's, at blocks 18, 24 and 36. Under plain nights that holds at block 18 alone.

Questions this suggests for a later exploration or a round:
- H-47's layout with only the stimulus sequence complemented, the groups and the patterns in H-47's order, to see which of layout 2's three
  differences from H-47 moves the early side and A's count.
- The two layouts on H-47's other eight cells.
- A read unit by unit, to see which of the side's readout's units fire on its trials under each form.

## What did not run

- **The new layouts' other forms**, plain nights on both and the awake gap, so no reading here stands against the awake gap.
- **The new layouts on H-47's other eight cells.**
- **The split and the class matrix on 021's exchange**: 021 ran none, so sections 3 and 4 hold layouts 1 and 2 against H-47's layout alone.
- **A layout that parts a set of units from the readout it answers.**
- **A read unit by unit**: a read gives each readout's spike count, not which units fired.
- **REM's depotentiations and the zero sum's rounds** are not split out of a night's change, as in explorations 020 and 021.
- **Any SE for sections 3 and 4**: the class matrix and the split are means over the three paths.
- **Any other place for the pair's own units, share, form, network or seed.**

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_the_readout_or_the_pairs_own_explore` | [41680929737](https://github.com/Menteion/Menteion/actions/runs/41680929737) | `b4d4daa` | 1060 |
| `the_pairs_own_moved_a_first_mapping_on_seed_23_from_the_assignment_explore` | [41684468588](https://github.com/Menteion/Menteion/actions/runs/41684468588) | `b4d4daa` | 3918 |
| `the_pairs_own_moved_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [41684468588](https://github.com/Menteion/Menteion/actions/runs/41684468588) | `b4d4daa` | 4089 |
| `the_pairs_own_moved_the_eighth_image_on_seed_22_from_the_assignment_explore` | [41684468588](https://github.com/Menteion/Menteion/actions/runs/41684468588) | `b4d4daa` | 3971 |
| `the_pairs_own_moved_the_eighth_image_on_seed_24_from_the_mirrored_assignment_explore` | [41684468588](https://github.com/Menteion/Menteion/actions/runs/41684468588) | `b4d4daa` | 3976 |
| `the_roles_exchanged_and_the_pairs_own_moved_a_first_mapping_on_seed_23_from_the_assignment_explore` | [41684468588](https://github.com/Menteion/Menteion/actions/runs/41684468588) | `b4d4daa` | 4054 |
| `the_roles_exchanged_and_the_pairs_own_moved_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [41684468588](https://github.com/Menteion/Menteion/actions/runs/41684468588) | `b4d4daa` | 3912 |
| `the_roles_exchanged_and_the_pairs_own_moved_the_eighth_image_on_seed_22_from_the_assignment_explore` | [41684468588](https://github.com/Menteion/Menteion/actions/runs/41684468588) | `b4d4daa` | 3824 |
| `the_roles_exchanged_and_the_pairs_own_moved_the_eighth_image_on_seed_24_from_the_mirrored_assignment_explore` | [41684468588](https://github.com/Menteion/Menteion/actions/runs/41684468588) | `b4d4daa` | 3820 |
