---
status: exploratory
date: 2026-11-12
---

# 035: The mirrored cell's units and inputs

**Branch**: `explore/035-the-mirrored-cells-units-and-inputs`
**Commit**: `6a44ad3`, which adds the harness, on `73a1dc8`: `main` at `64db9c1`, ADR-0250's merge, with exploration 034's tag merged
in, which changes nothing under `src/`. This branch changes nothing under `src/`, so the engine is `main`'s.
**Runs**: [43133489826](https://github.com/Menteion/Menteion/actions/runs/43133489826) (the harness's trial) and
[43134386489](https://github.com/Menteion/Menteion/actions/runs/43134386489) (two jobs), both at `scope=arms`. The logs set beside them
are H-47's run [41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613) for the night without replay and H-47's
plain night, exploration 021's run [41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) for H-47's two forms'
class matrices and night lines, exploration 031's run [42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780) for
replay after block 6 alone and exploration 032's run [42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683) for
replay after block 12, 18 or 24 alone, all on this cell; and for the image from the assignment, exploration 034's jobs, run
[43052358536](https://github.com/Menteion/Menteion/actions/runs/43052358536), whose rows, reads and unit reads are exploration 033's.
None of those was run again.

[ADR-0250](../adr/0250-the-mirrored-cells-units-and-inputs.md) ordered this exploration. On seed 24's eighth image from the assignment,
right after each single replayed night both readouts fired less on both stimuli's trials, and the inputs moved alike over each: more
inhibition onto both readouts, each place group's excitatory units' weights off the other readout, A's kinds toward A's answer's readout
([exploration 033](033-one-cells-readouts-unit-by-unit.md), [exploration 034](034-one-cells-readout-units-inputs.md)). Yet on A's trials the
spikes of A's answer's readout less the other's, less the night without replay, read +0.48 ± 0.29 after block 6 alone and −1.46 ± 0.22,
−2.96 ± 0.19 and −3.73 ± 0.16 after block 12, 18 and 24 alone. On seed 24's eighth image from the mirrored assignment, H-25's other arm on
the same seed, A's count lay above the night without replay right after the nights after block 18 and 24 alone, +9.1 ± 0.92 and +5.4 ± 2.02
([exploration 032](032-each-later-nights-replay-alone-on-twelve-cells.md)). This note reads that cell under the same six forms, each readout
unit's spikes on A's reads as exploration 033 read them and the weights onto each readout unit by presynaptic class as exploration 034 read
them, and asks:

1. whether every row and read under each form is the logs' of the same form, path, checkpoint and stream, so that A's count is those notes';
2. what each replayed night did to the two readouts' spikes, unit by unit and by class, right after it and at block 36;
3. what each replayed night changed in the inputs of each readout's units, by presynaptic group and class, whether a unit's input difference
   lines up with its spikes' difference, and what the later nights and blocks changed in it to block 36;
4. each of those beside exploration 033's and 034's readings of the image from the assignment, each readout named by the stimulus whose
   answer it is.

The note writes no prediction and decides nothing. **Every figure here is exploratory**: from one run of each test on the repository's
AArch64 host, in H-47's terms. A spike count's mean is over the cell's 48 reads, sixteen on each of three paths, and a difference carries
its paired SE unless another is named; one is written "beyond 2 SE" where it lies beyond twice its SE. **An input is the mean over the three
paths** of one image each, in the class matrix's units: a sum of Q1.15 weights, 32 768 to a weight of 1, written in millions (M) where it is
a readout's or a class's; an inhibitory unit's weight is its magnitude, so a + is more inhibition. No SE is claimed for an input. Every sum,
difference, ratio and per cent is worked from the unrounded values and rounded once, an exact half away from zero.

**In brief**
- **Every row and read is the logs', and every line of the logs set beside the jobs is in them.** 105 of 105 rows, 1 680 of 1 680 of A's
  reads and 1 680 of 1 680 of the pair's are the logs' of the same form, path, checkpoint and stream, label aside, so A's count under each
  form is H-47's, exploration 031's or 032's. Label aside, 4 441 of 4 441 lines of exploration 031's job and 5 306 of 5 306 of exploration
  021's plain night are among job one's lines and 9 647 of 9 647 of exploration 032's job among job two's. On each of the 191 images the
  jobs read, each presynaptic class's sums over the readouts' units of each class are the class matrix's entries, held inside the test, and
  1 680 unit reads were taken, one after each of A's reads.
- **Right after each single replayed night both readouts fired less on both stimuli's trials, as on the image from the assignment**: by 5.48
  to 9.49 spikes a trial, 19 to 29 per cent of the night without replay's spikes at the same checkpoint, each beyond 2 SE, with 80 to 145 of
  a readout's 459 units moved down beyond twice their own SE. Each of the 16 differences lies the same side of 2 SE on both cells.
- **Yet A's answer's readout less the other moved the other way from the image from the assignment.** On A's trials, less the night without
  replay, it read −0.84 ± 0.21 after block 6 alone and +0.69 ± 0.18, +2.42 ± 0.23 and +1.31 ± 0.31 after block 12, 18 and 24 alone here,
  below beyond 2 SE after the first and above beyond 2 SE after the three later single nights, where on the image from the assignment it
  read +0.48 ± 0.29 and −1.46 ± 0.22, −2.96 ± 0.19 and −3.73 ± 0.16; on B's trials B's answer's readout less the other read −0.21 ± 0.30,
  +0.08 ± 0.19, +1.32 ± 0.26 and +1.66 ± 0.35 here and −0.80 ± 0.22, −0.90 ± 0.21, −1.43 ± 0.37 and −0.56 ± 0.17 there. Of the eight on each
  cell, 0 of 8 lie the same side of 2 SE on both. A's count lay above the night without replay beyond 2 SE right after the nights after
  block 18 and 24 alone, +9.1 ± 0.92 and +5.4 ± 2.02, and at block 36 after block 12 and 18 alone, +3.8 ± 0.78 and +5.4 ± 1.17, and below it
  beyond 2 SE after no single night, right after or at block 36.
- **Over each single replayed night the inputs moved as on the image from the assignment.** Of the inhibitory, all excitatory and A's and
  B's kinds' changes onto each readout, 32 of 32 carry the same sign on both cells: the inhibitory input onto the readouts rose by 4.16 to
  5.36 M here and 4.18 to 4.48 there and all excitatory input fell by 3.02 to 4.18 here and 3.37 to 4.29 there, A's kinds' input onto A's
  answer's readout rose by 0.35 to 0.42 M here and 0.23 to 0.31 there and B's onto B's by 0.28 to 0.34 here and 0.28 to 0.32 there, and each
  place group's excitatory units lost more onto the other readout than onto their own, 8 of 8 here and 8 of 8 there. The gain fell by 0.073
  to 0.074 here and 0.061 to 0.065 there, and A's margin rose by 1.28 to 1.43 M here and 1.05 to 1.26 there. Here the inhibitory rise onto
  A's answer's readout, readout 0, was 5.25 to 5.36 M and onto B's 4.16 to 4.31; on the image from the assignment 4.32 to 4.48 and 4.18 to
  4.43.
- **On both cells A's answer's readout lost more of its excitatory input less inhibitory input than the other over each single replayed
  night, more here.** Right after the nights after block 6, 12, 18 and 24 alone its difference less the other's read −1.76, −1.21, −1.42 and
  −1.86 M here and −0.01, −0.48, −0.46 and −0.95 there, while A's kinds' input difference onto A's answer's readout less onto the other read
  +0.67 to +0.81 M here and +0.49 to +0.69 there. On the image each replayed night starts from, A's answer's readout's excitatory input less
  inhibitory input less the other's read −0.33, −0.38, −0.44 and −0.50 M here and +1.22, +0.72, +0.67 and +0.40 there, and A's kinds' input
  onto A's answer's readout less onto the other +0.03, −0.16, −0.33 and −0.43 here and +0.52, +0.17, +0.04 and −0.19 there.
- **The units that carried half of each readout's drop lost input from the trial's stimulus's kinds, as on the image from the assignment.**
  Right after each single replayed night their input difference from the trial's stimulus's kinds lay below the rest of their readout's in
  16 of 16 sets here and 16 of 16 there; on A's trials here, from A's kinds, −12 138 to −7 823 a unit against the rest's −421 to +1 283, in
  the class matrix's units. Their inhibitory input difference lay below the rest's in 15 of 16 sets here, all but readout 1's on A's trials
  after block 24 alone, +11 621 a unit against the rest's +8 381, and in 16 of 16 there. Over each readout's 459 units a unit's input
  difference from the trial's stimulus's kinds correlated with its spikes' difference by +0.43 to +0.83 here and +0.51 to +0.72 there.
- **By block 36 the inhibitory difference had come back by 39 to 78 per cent here and 42 to 81 on the image from the assignment.** Of all
  excitatory input's difference, here it had come back by 1 to 13 per cent in 7 of 8 cases, and it lay further from the night without replay
  after block 24 alone onto A's answer's readout, by 1 per cent; on the image from the assignment it had come back by 1 to 13 per cent in 6
  of 8 cases, and it lay further from the night without replay after block 18 alone onto B's answer's readout and after block 24 alone onto
  B's answer's readout, by 2 and 1 per cent. Here each readout's spikes' difference lay nearer the night without replay at block 36 than
  right after the replayed night in 16 of 16 and below it beyond 2 SE in 16 of 16.
- **Under H-47's plain night on all four nights each input's difference grew with each night**: of the inhibitory input's, each place
  group's and A's and B's kinds' differences onto the two readouts, 10 of 10 grew in magnitude from block 6 to block 24. The inhibitory
  input onto A's answer's readout read +4.96, +7.99, +10.44 and +12.38 M less the night without replay's at blocks 6, 12, 18 and 24 here and
  +4.15, +6.56, +8.32 and +9.49 on the image from the assignment, and +10.24 and +7.48 at block 36. Here A's count under it lay above the
  night without replay at block 36 by +11.3 ± 0.96.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 020 to 034 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. Nights follow blocks 6, 12, 18 and 24, then 12 blocks run with
none to block 36, on paths 0, 5 and 6. Each checkpoint reads A and the pair sixteen times each, a read one block of 64 trials on a fork of
the image with the reward withheld.

**One cell**, seed 24's eighth image from the mirrored assignment. There A asks readout 0, the odd places, of stimulus A and readout 1, the
even places, of B: A's reads print `mirrored false`, and the pair's `mirrored true`, as every read of the two jobs and the trial prints it,
the same way round as exploration 032's job on this cell. On the image from the assignment, the same network, A asks readout 1 of A and
readout 0 of B. A period of the ring is 20 units; stimulus A presents place 0 of each period and B place 11. **The odd places** are the odd
places but 11 and **the even places** the even places but 0: H-47's readout 0 and readout 1, nine places each over 51 periods, 459 units
each, the same units by id as on the image from the assignment, whose rotation is also 0, so a unit's place is its id's remainder by 20.

**The readouts' units by class**, as exploration 020's class matrix names them, from the start image (`e020_class`): a pattern target is a
unit that one of A's patterns, drawn from the start image, reaches past its first unit. A's answer's readout's rows come first:

| Readout | Class | Places | How many units |
| :--- | :--- | :--- | ---: |
| readout 0, the odd places | C's own | 5 | 25 |
| readout 0, the odd places | readout 0's pattern targets | 1, 3, 5, 7, 13, 15, 17 | 280 |
| readout 0, the odd places | readout 0's other excitatory | 1, 3, 5, 7, 13, 15, 17 | 52 |
| readout 0, the odd places | inhibitory in readout 0 | 9, 19 | 102 |
| readout 1, the even places | D's own | 16 | 25 |
| readout 1, the even places | readout 1's pattern targets | 2, 6, 8, 10, 12, 16, 18 | 254 |
| readout 1, the even places | readout 1's other excitatory | 2, 6, 8, 10, 12, 16, 18 | 78 |
| readout 1, the even places | inhibitory in readout 1 | 4, 14 | 102 |

C's own units lie at place 5 and D's own at place 16, as on the image from the assignment, and each readout's inhibitory units at the same
places. A's patterns, drawn from each cell's own start image, reach 280 of readout 0's excitatory units here and 245 there, and 254 of
readout 1's here and 252 there: of the 918 units, 239 are of another class on the image from the assignment, each a pattern target on one
cell and another excitatory unit of the same readout on the other.

**Six forms, in two jobs**, exploration 033's and 034's. A night that replays is H-47's plain night on A's patterns: A's 102 patterns, drawn
from the start image, tagged at priority 200. Every other night is H-47's night without replay, the same night with nothing tagged. No
hold is set. Each night is chosen by its form (`E030Form`), and every night of every form is watched.

| Form | In the logs | Night after block 6 | after 12 | after 18 | after 24 | Job | Rows held inside its job to H-47's, on every path | Its rows and reads in the logs |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Without replay | `no-replay` | without replay | without replay | without replay | without replay | both | the night without replay's, blocks 6 to 36 | H-47's |
| After block 6 alone | `alone-6` | replays | without replay | without replay | without replay | one | the plain night's, block 6 | exploration 031's |
| Plain on all four nights (H-47) | `plain-first` | replays | replays | replays | replays | one | the plain night's, blocks 6 to 36 | H-47's |
| After block 12 alone | `alone-12` | without replay | replays | without replay | without replay | two | the night without replay's, block 6 | exploration 032's |
| After block 18 alone | `alone-18` | without replay | without replay | replays | without replay | two | the night without replay's, blocks 6 and 12 | exploration 032's |
| After block 24 alone | `alone-24` | without replay | without replay | without replay | replays | two | the night without replay's, blocks 6, 12 and 18 | exploration 032's |

**The unit read** is exploration 033's. In each of A's reads, on each trial, each unit of the two readouts' sets has its spikes counted in
that trial's readout window, ticks 100 to 599 of the trial, from the executor's own train, by `Readout::count_window`'s rule. A unit's
spikes are summed over the read's trials of each stimulus and divided by that stimulus's trials in the read, for **a unit's spikes a
trial**; a read's trials of each stimulus are the same in every form, since a read's stimuli follow from its index alone: 28 to 37 trials of
stimulus A and 27 to 36 of B in a read. **A readout's spikes a trial** is the sum over its units, the readout's count in the trial's window,
which the engine selects on; it is not A's count, which counts correct trials. Inside the test, on each trial of each unit read: the train
holds the whole window, and each readout's units' spikes sum to that readout's count of the trial (`Outcome::counts`).

**The input read** is exploration 034's. On each image whose class matrix the job writes, the start image, the image before each night and
the image after it, and block 36's, the harness walks every unit's fan-out, as exploration 020's class matrix does, and for each synapse
onto a unit of the two readouts' sets adds its weight to that unit's sum for the class of the presynaptic unit, an inhibitory unit's weight
negated: fourteen sums a unit, 12 852 an image. Held inside the test on each image: each readout unit keeps the class it had on the start
image, and summed over the units of each of the eight classes the readouts' units make up, each presynaptic class's sums are that class
matrix's entry, 112 entries an image. Nothing under `src/` changed: the walk is `exec.units()` and `unit.fan_out(exec.blocks())`.

**The presynaptic groups** are exploration 034's, by place: A's kinds (classes 0 and 1, A alone and A and C), B's kinds (2 and 3, B alone
and B and D), the odd places' excitatory units (4, 6 and 7: C's own and readout 0's pattern targets and other excitatory units), the even
places' (5, 8 and 9: D's own and readout 1's two classes), the other excitatory units (10) and the inhibitory units (11 to 13). On this
cell, as on the image from the assignment, every unit of the 51 whole periods, units 0 to 1 019, is a stimulus unit or a readout unit, so
class 10 and class 13 can hold only units 1 020 to 1 023, past the last whole period; ADR-0249 gives class 10 as those four units and class
13 as none on the same network. Class 13's row and column are 0 in every one of the 191 class matrices the jobs wrote, and its sum is 0 on
every readout unit on each of the 191 images; class 10's row is not 0 in 191 of 191 and its column in 191 of 191, and its sum is not 0 on
9 198 of the 175 338 unit images.

**The SEs**, exploration 033's. A form less the night without replay carries H-47's paired SE over its 48 differences, the night without
replay of its own job; so does after block 6 alone less H-47's plain night, both in the first job. After block 12, 18 or 24 alone less
H-47's plain night, across the two jobs, carries exploration 018's unpaired SE: the mean over the paths of each path's difference of
means, ± √((the two forms' stream variances)/16 + (the paths' differences' SD)²/3), a negative variance as zero. A unit's own difference
carries its own SE, worked the same way from that unit's 48 values.

**What moved.** A unit **moved** where its difference lies beyond twice its own SE, up or down. About one unit in twenty would lie there by
chance in each comparison, some 23 of a readout's 459 in the two directions together, 11 or 12 each way. Where a readout's own
difference lies beyond 2 SE: **the fewest units** carrying half of it, the units ranked by their difference in the direction of the
readout's net change, taken until their differences reach half the net change, worked here by exploration 033's rule on this cell; **the
gross change**, the sum of the units' differences' magnitudes; and **the units moved against** the net change, with their differences'
sum. A readout's difference is the sum of its units' differences, so the net change is the readout's difference. Where a readout's change
is a rise, the fewest units carry half of the rise.

**What is read of the inputs**, on the mean over the three paths of each image: **a night's change** is the image after the night less the
image before it; **a stretch's** the image before the next night, or block 36's, less the image after the night before it; **an input's
difference** is the form's image less the night without replay's on the same path at the same point, in the form's own job. A readout's
input is the sum over its 459 units; **a unit's input difference** in a set of units is the mean over the set. **The gain** scales every
unit's summed input: an image's gain before a night is the night's start state's and after it the end state's, the two the rows carry as the
night's gain, held equal here; block 36's is the run line's of the twelve blocks; a gain is written as a multiple of 1, its Q16.16 value
over 65 536. **The correlation** of a readout's units' input difference from a group with their spikes' difference, on one stimulus's
trials, is Pearson's over the readout's 459 units. **A's margin** from the input read is each of A's and B's kinds' input onto its
stimulus's answer readout less onto the other, summed over the four kinds; on this cell A's kinds answer on readout 0 and B's on readout 1.

**A checkpoint is read after the night that follows its block**, and block 36's after the twelve blocks that follow the last night. So right
after its replayed night, a single night's form less the night without replay is that night's replay alone: until then the two ran the same
nights on the same images. H-47's plain night less the night without replay at blocks 6, 12, 18 and 24 is its nights so far together.

**The two cells side by side** (section 9): each readout is named by the stimulus whose answer it is, **A's answer's readout** (readout 0
here, readout 1 on the image from the assignment) and **B's answer's readout** (readout 1 here, readout 0 there); classes and place groups
keep their names, so C's own units, at place 5, lie in A's answer's readout here and in B's there. The image from the assignment's figures
are worked by this note's parser from exploration 034's jobs, the same reads and images as exploration 033's and 034's notes.

## Before a job ran

ADR-0250 asks whether any form must run bit for bit as one already in the logs. **Each of the six does**: the night without replay and
H-47's plain night are H-47's job's, after block 6 alone exploration 031's job's, and after block 12, 18 or 24 alone exploration 032's,
every image, row and read of them in those logs, the class matrices and night lines of H-47's two forms in exploration 021's. **No log holds
this cell's units one by one or their inputs**, so each form ran again, and its rows and reads are set beside the logs' here.

## The runs are laid out as written

- **The trial** ran on path 0 of the cell: the pair's head and the first night of each of the six forms, each watched, each image split and
  read once as it was, of A and of the pair, A's read counted unit by unit and the image's inputs read. Inside the test each image, those
  two reads, A's couplings, their highest over the head and the gain held H-47's row at block 6 on path 0 of the form whose first night it
  ran:

| Form | Its first night | H-47's row at block 6 on path 0 it holds, inside the test | A's read as it was | The pair's | Replays | Depotentiations | Watch: wrong | Its night's lines that are exploration 021's job's same night on path 0, label and its notice aside | Trials of A, of B | A's trials: readout 0's units' spikes summed over them, readout 1's | B's trials: readout 0's, readout 1's | Those sums are |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | :--- | :--- | ---: | ---: | :--- |
| Without replay | without replay | the night without replay's | 33 | 60 | 0 | 0 | 0 | 89 of 89 | 34, 30 | 979, 961 | 788, 804 | the read's |
| After block 6 alone | replays | the plain night's | 28 | 62 | 1 330 | 640 | 0 | 398 of 398 | 34, 30 | 683, 710 | 616, 624 | the read's |
| Plain on all four nights (H-47) | replays | the plain night's | 28 | 62 | 1 330 | 640 | 0 | 398 of 398 | 34, 30 | 683, 710 | 616, 624 | the read's |
| After block 12 alone | without replay | the night without replay's | 33 | 60 | 0 | 0 | 0 | 89 of 89 | 34, 30 | 979, 961 | 788, 804 | the read's |
| After block 18 alone | without replay | the night without replay's | 33 | 60 | 0 | 0 | 0 | 89 of 89 | 34, 30 | 979, 961 | 788, 804 | the read's |
| After block 24 alone | without replay | the night without replay's | 33 | 60 | 0 | 0 | 0 | 89 of 89 | 34, 30 | 979, 961 | 788, 804 | the read's |

- Each night's lines, its start, its windows, its watch and its end, are looked for among exploration 021's job's night after block 6 on
  path 0 of this cell, the night without replay's for the four forms whose first night is without replay and the plain night's for the two
  whose first night replays, label and form aside, each night's notice that it read what its form asks left out: 89 of 89 for each of the
  four, against exploration 021's 89, and 398 of 398 for each of the two, against its 398. The head's line is exploration 021's, 1 of 1.
  Each night without replay read 0 replays and 0 depotentiations and each replayed night 1 330 and 640, as the trial prints, and the watch
  read `wrong 0` in all six. Each unit read's trials of each stimulus are its read's, and its units' spikes summed by readout are the read's
  own spikes on each stimulus's trials, the sums in the table. Each of the six images' input reads made its class matrix's 112 entries, 672
  of 672 in all by this note's parser, and the trial prints it after each, 6 of 6; each image's split gives each kind's sum onto each
  readout as the input read does, 6 of 6.

- **The two jobs** ran to the end, each form on the three paths. Inside each test every row of the night without replay and of H-47's plain
  night held H-47's row of that form, fifteen each; after block 6 alone's row at block 6 held the plain night's; and after block 12, 18 or
  24 alone's rows before its replayed night's block held the night without replay's, three, six and nine. Every form's replayed nights read
  replays and depotentiations that are not zero and its other nights `replays 0` and `depotentiations 0`, held inside the test:

| Job | Form | Rows held inside the test to H-47's, on every path | Of its 15 rows, H-47's night without replay's and its plain night's, as the job prints | Replayed nights: replays and depotentiations | Other nights reading replays 0 and depotentiations 0 | Watch: wrong |
| :--- | :--- | :--- | ---: | :--- | :--- | ---: |
| Job one | Without replay | the night without replay's, blocks 6, 12, 18, 24 and 36 | 15, 0 | 0: –, – | 12 of 12 | 0 |
| Job one | After block 6 alone | the plain night's, block 6 | 0, 3 | 3: 1 330, 640 | 9 of 9 | 0 |
| Job one | Plain on all four nights (H-47) | the plain night's, blocks 6, 12, 18, 24 and 36 | 0, 15 | 12: 1 129 to 1 330, 545 to 640 | 0 of 0 | 0 |
| Job two | Without replay | the night without replay's, blocks 6, 12, 18, 24 and 36 | 15, 0 | 0: –, – | 12 of 12 | 0 |
| Job two | After block 12 alone | the night without replay's, block 6 | 3, 0 | 3: 1 341, 640 | 9 of 9 | 0 |
| Job two | After block 18 alone | the night without replay's, blocks 6 and 12 | 6, 0 | 3: 1 341, 640 | 9 of 9 | 0 |
| Job two | After block 24 alone | the night without replay's, blocks 6, 12 and 18 | 9, 0 | 3: 1 341, 640 | 9 of 9 | 0 |

- **The rows.** Each form's fifteen rows are set beside the rows of the same form in the logs, H-47's for the night without replay and its
  plain night, exploration 031's for after block 6 alone and exploration 032's for the other three: 105 of 105 are those rows (section 1).
  As each job prints, of after block 12, 18 and 24 alone's fifteen rows 3, 6 and 9 are H-47's night without replay's and 0, 0 and 0 its
  plain night's; of after block 6 alone's, 0 the night without replay's and 3 the plain night's; and the night without replay's fifteen in
  each job and the plain night's are each H-47's of that form, 15, 15 and 15. Every night's notice that it read what its form asks gives the
  replays and depotentiations its end line gives, 84 of 84 nights.

| Job | Images read: the start image and each chain's nine | Their inputs the class matrix's 112 entries, by this note's parser | Entries | Lines saying so, as the job prints | Images whose splits give the same sums by kind onto each readout | Unit reads whose trials and sums by readout are their read's |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Job one | 82 | 82 of 82 | 9 184 of 9 184 | 82 of 82 | 82 of 82 | 720 of 720 |
| Job two | 109 | 109 of 109 | 12 208 of 12 208 | 109 of 109 | 109 of 109 | 960 of 960 |

- **The input reads**: each image's inputs make its class matrix's 112 entries, 82 images in job one and 109 in job two, and each job prints
  it after each. A split prints each kind's sum onto each readout in three parts, the kinds in the order A alone, A and C, C's own, B alone,
  B and D, D's own, so classes 0, 1, 4, 2, 3 and 5; on every image the input read's sum over each readout's units from each of those six
  classes is the split's sum of that kind onto that readout, 191 of 191 images, 2 292 sums. The night without replay's input reads on the
  three paths' nine images are the same in job one and job two, 27 of 27, and so is the start image's. **The unit reads**: 720 in job one
  and 960 in job two, one after each of A's reads; the night without replay's 240 in each job are the same in both. Inside the test, on
  every trial, the train held the whole window and each readout's units summed to its count.
- **The lines.** Each log's lines, label aside, looked for among the job's lines:

| Job | Log set beside | Its lines looked for | Among the job's lines, label aside |
| :--- | :--- | :--- | :--- |
| Job one | exploration 031's job, run 42666579780 | every line | 4 441 of 4 441 |
| Job one | exploration 021's job, run 41591077514 | its plain night's lines | 5 306 of 5 306 |
| Job two | exploration 032's job, run 42971972683 | every line | 9 647 of 9 647 |

- Of job one's 10 645 lines, 898 are lines of neither exploration 031's job nor exploration 021's plain night, label aside: 720 unit reads
  and the readouts' line, 721, 164 input read lines, 12 notices that a night read what its form asks and 1 of the rows held. Of job two's
  10 826, 1 179 are not lines of exploration 032's job: 960 unit reads and the readouts' line, 961, and 218 input read lines.

## 1. The rows and reads against the logs

Each form's rows and each of A's and the pair's reads are set beside the same form's, path's, checkpoint's and stream's in the logs, by the
stream (`spread`'s index), not by the order the job printed them:

| Job | Form | Logs set beside | Rows that are the logs' rows of the same form | A's reads with the logs' count and masks | their whole line, label aside | The pair's reads with the logs' count and masks | their whole line |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Job one | Without replay | H-47's, run 41441234613 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job one | After block 6 alone | exploration 031's, run 42666579780 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job one | Plain on all four nights (H-47) | H-47's, run 41441234613 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job two | Without replay | H-47's, run 41441234613 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job two | After block 12 alone | exploration 032's, run 42971972683 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job two | After block 18 alone | exploration 032's, run 42971972683 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job two | After block 24 alone | exploration 032's, run 42971972683 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 |

Every row and read is the logs': 105 of 105 rows, 1 680 of 1 680 of A's reads and 1 680 of 1 680 of the pair's with the logs' count,
`stimuli`, `right` and `tied` masks, 1 680 of 1 680 and 1 680 of 1 680 with the whole line, label aside. Of them H-47's: the night without
replay's rows in job one and job two and the plain night's, 15 of 15, 15 of 15 and 15 of 15, and their reads of A, 240 of 240, 240 of 240
and 240 of 240. The night without replay's 480 reads are the same in job one and job two, 480 of 480, and so are its rows, 15 of 15. So A's
count and each stimulus's correct trials under each form are those the notes give, read here from the same reads; the difference less the
night without replay carries H-47's paired SE in the form's own job:

| Form | Block | A's count | Without replay's | Less without replay | On A's trials | On B's trials | The note that gives it |
| :--- | :--- | ---: | ---: | :--- | :--- | :--- | :--- |
| After block 6 alone | 6 | 33.8 | 36.4 | −2.6 ± 1.35 | −2.1 ± 0.99 | −0.5 ± 1.17 | exploration 031 |
| After block 6 alone | 12 | 28.3 | 31.5 | −3.2 ± 1.07 | −3.4 ± 0.55 | +0.2 ± 0.67 | exploration 031 |
| After block 6 alone | 18 | 22.4 | 20.2 | +2.2 ± 1.15 | +0.1 ± 0.70 | +2.1 ± 1.06 | exploration 031 |
| After block 6 alone | 24 | 19.0 | 16.4 | +2.6 ± 1.22 | 0.0 ± 1.35 | +2.6 ± 0.82 | exploration 031 |
| After block 6 alone | 36 | 15.6 | 12.5 | +3.1 ± 1.84 | +0.7 ± 0.86 | +2.4 ± 1.06 | exploration 031 |
| Plain on all four nights (H-47) | 6 | 33.8 | 36.4 | −2.6 ± 1.35 | −2.1 ± 0.99 | −0.5 ± 1.17 | H-47 |
| Plain on all four nights (H-47) | 12 | 28.6 | 31.5 | −2.9 ± 1.19 | −3.2 ± 0.72 | +0.3 ± 0.77 | H-47 |
| Plain on all four nights (H-47) | 18 | 28.6 | 20.2 | +8.3 ± 1.70 | +0.7 ± 0.25 | +7.6 ± 1.52 | H-47 |
| Plain on all four nights (H-47) | 24 | 25.8 | 16.4 | +9.4 ± 1.58 | +1.6 ± 1.33 | +7.8 ± 1.47 | H-47 |
| Plain on all four nights (H-47) | 36 | 23.8 | 12.5 | +11.3 ± 0.96 | +1.5 ± 0.74 | +9.7 ± 0.98 | H-47 |
| After block 12 alone | 6 | 36.4 | 36.4 | = without replay | = without replay | = without replay | exploration 032 |
| After block 12 alone | 12 | 32.3 | 31.5 | +0.8 ± 1.09 | +0.5 ± 0.83 | +0.2 ± 0.60 | exploration 032 |
| After block 12 alone | 18 | 26.3 | 20.2 | +6.1 ± 0.77 | +3.7 ± 0.49 | +2.4 ± 0.73 | exploration 032 |
| After block 12 alone | 24 | 21.4 | 16.4 | +4.9 ± 1.23 | +2.3 ± 0.97 | +2.6 ± 0.58 | exploration 032 |
| After block 12 alone | 36 | 16.3 | 12.5 | +3.8 ± 0.78 | +0.7 ± 0.29 | +3.1 ± 1.02 | exploration 032 |
| After block 18 alone | 6 | 36.4 | 36.4 | = without replay | = without replay | = without replay | exploration 032 |
| After block 18 alone | 12 | 31.5 | 31.5 | = without replay | = without replay | = without replay | exploration 032 |
| After block 18 alone | 18 | 29.3 | 20.2 | +9.1 ± 0.92 | +6.0 ± 0.68 | +3.1 ± 1.02 | exploration 032 |
| After block 18 alone | 24 | 20.9 | 16.4 | +4.4 ± 1.23 | +1.1 ± 0.59 | +3.3 ± 0.82 | exploration 032 |
| After block 18 alone | 36 | 18.0 | 12.5 | +5.4 ± 1.17 | +1.7 ± 0.54 | +3.7 ± 0.86 | exploration 032 |
| After block 24 alone | 6 | 36.4 | 36.4 | = without replay | = without replay | = without replay | exploration 032 |
| After block 24 alone | 12 | 31.5 | 31.5 | = without replay | = without replay | = without replay | exploration 032 |
| After block 24 alone | 18 | 20.2 | 20.2 | = without replay | = without replay | = without replay | exploration 032 |
| After block 24 alone | 24 | 21.8 | 16.4 | +5.4 ± 2.02 | +1.9 ± 0.99 | +3.5 ± 1.23 | exploration 032 |
| After block 24 alone | 36 | 14.4 | 12.5 | +1.9 ± 1.18 | −0.8 ± 0.34 | +2.7 ± 0.89 | exploration 032 |

Right after its replayed night A's count lay above the night without replay beyond 2 SE after block 18 and 24 alone, +9.1 ± 0.92 and
+5.4 ± 2.02; within 2 SE of it after block 6 and 12 alone, −2.6 ± 1.35 and +0.8 ± 1.09. At block 36 it lay above the night without replay
beyond 2 SE after block 12 and 18 alone, +3.8 ± 0.78 and +5.4 ± 1.17; within 2 SE of it after block 6 and 24 alone, +3.1 ± 1.84 and
+1.9 ± 1.18; H-47's plain night on all four nights lay above the night without replay beyond 2 SE, +11.3 ± 0.96. After block 6 alone and
H-47's plain night share job one, so their difference in A's count carries the paired SE beside exploration 031's unpaired one; the means
are the same:

| Block | Exploration 018's unpaired SE, from these two forms | As exploration 031 gives it | Paired SE, both in job one |
| :--- | :--- | :--- | :--- |
| 6 | = plain | = plain | = plain |
| 12 | −0.3 ± 1.20 | −0.3 ± 1.20 | −0.3 ± 0.78 |
| 18 | −6.2 ± 1.69 | −6.2 ± 1.69 | −6.2 ± 1.51 |
| 24 | −6.8 ± 1.22 | −6.8 ± 1.22 | −6.8 ± 1.18 |
| 36 | −8.2 ± 2.09 | −8.2 ± 2.09 | −8.2 ± 2.14 |

## 2. Each readout's spikes under each form

**Each readout's spikes a trial**, the sum over its 459 units, on A's trials and on B's, the mean over the 48 reads. On this
cell A asks readout 0 of stimulus A and readout 1 of B:

| Form | Block | A's trials: readout 0, the odd places | readout 1, the even places | B's trials: readout 0 | readout 1 |
| :--- | :--- | ---: | ---: | ---: | ---: |
| Without replay | 6 | 29.27 | 27.67 | 25.68 | 26.62 |
| Without replay | 12 | 31.15 | 31.29 | 27.51 | 28.21 |
| Without replay | 18 | 31.83 | 34.13 | 30.14 | 28.84 |
| Without replay | 24 | 32.82 | 36.17 | 32.29 | 30.02 |
| Without replay | 36 | 30.07 | 34.75 | 31.15 | 28.23 |
| After block 6 alone | 6 | 20.96 | 20.19 | 20.03 | 20.75 |
| After block 6 alone | 12 | 27.13 | 28.35 | 24.72 | 25.41 |
| After block 6 alone | 18 | 29.15 | 31.48 | 26.83 | 26.26 |
| After block 6 alone | 24 | 30.43 | 33.94 | 29.23 | 28.15 |
| After block 6 alone | 36 | 28.32 | 32.37 | 28.40 | 26.46 |
| Plain on all four nights (H-47) | 6 | 20.96 | 20.19 | 20.03 | 20.75 |
| Plain on all four nights (H-47) | 12 | 20.59 | 21.60 | 20.32 | 21.05 |
| Plain on all four nights (H-47) | 18 | 20.61 | 22.23 | 20.36 | 21.78 |
| Plain on all four nights (H-47) | 24 | 20.34 | 22.19 | 20.83 | 21.66 |
| Plain on all four nights (H-47) | 36 | 21.26 | 24.55 | 22.02 | 22.90 |
| After block 12 alone | 6 | 29.27 | 27.67 | 25.68 | 26.62 |
| After block 12 alone | 12 | 22.66 | 22.11 | 21.35 | 22.13 |
| After block 12 alone | 18 | 28.85 | 29.74 | 27.36 | 27.07 |
| After block 12 alone | 24 | 30.07 | 32.33 | 29.86 | 28.88 |
| After block 12 alone | 36 | 27.31 | 31.03 | 29.03 | 27.29 |
| After block 18 alone | 6 | 29.27 | 27.67 | 25.68 | 26.62 |
| After block 18 alone | 12 | 31.15 | 31.29 | 27.51 | 28.21 |
| After block 18 alone | 18 | 24.76 | 24.64 | 23.35 | 23.36 |
| After block 18 alone | 24 | 29.71 | 32.16 | 29.77 | 28.84 |
| After block 18 alone | 36 | 27.50 | 30.76 | 28.73 | 27.40 |
| After block 24 alone | 6 | 29.27 | 27.67 | 25.68 | 26.62 |
| After block 24 alone | 12 | 31.15 | 31.29 | 27.51 | 28.21 |
| After block 24 alone | 18 | 31.83 | 34.13 | 30.14 | 28.84 |
| After block 24 alone | 24 | 25.30 | 27.34 | 24.84 | 24.23 |
| After block 24 alone | 36 | 26.75 | 31.71 | 28.13 | 26.36 |

Each form less the night without replay, with H-47's paired SE; "= without replay" where the form's nights so far are the night without
replay's:

| Form | Block | A's trials: readout 0, less without replay | readout 1 | B's trials: readout 0 | readout 1 |
| :--- | :--- | :--- | :--- | :--- | :--- |
| After block 6 alone | 6 | −8.32 ± 0.19 | −7.48 ± 0.10 | −5.66 ± 0.27 | −5.87 ± 0.15 |
| After block 6 alone | 12 | −4.02 ± 0.17 | −2.94 ± 0.14 | −2.80 ± 0.09 | −2.80 ± 0.14 |
| After block 6 alone | 18 | −2.68 ± 0.18 | −2.65 ± 0.28 | −3.31 ± 0.27 | −2.58 ± 0.16 |
| After block 6 alone | 24 | −2.38 ± 0.15 | −2.23 ± 0.34 | −3.06 ± 0.15 | −1.87 ± 0.16 |
| After block 6 alone | 36 | −1.75 ± 0.22 | −2.38 ± 0.25 | −2.74 ± 0.15 | −1.76 ± 0.33 |
| Plain on all four nights (H-47) | 6 | −8.32 ± 0.19 | −7.48 ± 0.10 | −5.66 ± 0.27 | −5.87 ± 0.15 |
| Plain on all four nights (H-47) | 12 | −10.56 ± 0.20 | −9.69 ± 0.22 | −7.19 ± 0.16 | −7.16 ± 0.19 |
| Plain on all four nights (H-47) | 18 | −11.22 ± 0.24 | −11.90 ± 0.23 | −9.78 ± 0.34 | −7.06 ± 0.13 |
| Plain on all four nights (H-47) | 24 | −12.48 ± 0.20 | −13.98 ± 0.41 | −11.46 ± 0.41 | −8.36 ± 0.16 |
| Plain on all four nights (H-47) | 36 | −8.81 ± 0.13 | −10.20 ± 0.39 | −9.13 ± 0.31 | −5.33 ± 0.15 |
| After block 12 alone | 6 | = without replay | = without replay | = without replay | = without replay |
| After block 12 alone | 12 | −8.48 ± 0.16 | −9.17 ± 0.20 | −6.16 ± 0.15 | −6.08 ± 0.19 |
| After block 12 alone | 18 | −2.98 ± 0.13 | −4.39 ± 0.26 | −2.79 ± 0.30 | −1.77 ± 0.15 |
| After block 12 alone | 24 | −2.75 ± 0.11 | −3.85 ± 0.33 | −2.43 ± 0.16 | −1.14 ± 0.11 |
| After block 12 alone | 36 | −2.76 ± 0.20 | −3.71 ± 0.27 | −2.11 ± 0.17 | −0.93 ± 0.22 |
| After block 18 alone | 6 | = without replay | = without replay | = without replay | = without replay |
| After block 18 alone | 12 | = without replay | = without replay | = without replay | = without replay |
| After block 18 alone | 18 | −7.07 ± 0.15 | −9.49 ± 0.19 | −6.79 ± 0.22 | −5.48 ± 0.08 |
| After block 18 alone | 24 | −3.11 ± 0.18 | −4.01 ± 0.30 | −2.52 ± 0.19 | −1.18 ± 0.15 |
| After block 18 alone | 36 | −2.57 ± 0.26 | −3.99 ± 0.16 | −2.42 ± 0.12 | −0.83 ± 0.21 |
| After block 24 alone | 6 | = without replay | = without replay | = without replay | = without replay |
| After block 24 alone | 12 | = without replay | = without replay | = without replay | = without replay |
| After block 24 alone | 18 | = without replay | = without replay | = without replay | = without replay |
| After block 24 alone | 24 | −7.52 ± 0.18 | −8.83 ± 0.24 | −7.46 ± 0.28 | −5.79 ± 0.18 |
| After block 24 alone | 36 | −3.32 ± 0.24 | −3.04 ± 0.12 | −3.02 ± 0.27 | −1.87 ± 0.17 |

Of the 76 differences from each form's replayed night on, 76 lie below the night without replay beyond 2 SE, 0 above beyond 2 SE and 0
within 2 SE; at the checkpoint right after each form's first replayed night, 20 of 20 lie below beyond 2 SE.

**How the night without replay's spikes lie over each readout's units**: how many of its units fire at all over the 48 reads, and the
fewest of its units, the most spikes first, that carry half its spikes:

| Block | Trials | Spikes a trial, readout 0 | of its units, any spike | fewest of its units carrying half | Spikes a trial, readout 1 | of its units, any spike | fewest carrying half |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | A's trials | 29.27 | 452 | 17 | 27.67 | 451 | 17 |
| 6 | B's trials | 25.68 | 449 | 15 | 26.62 | 443 | 14 |
| 12 | A's trials | 31.15 | 455 | 17 | 31.29 | 452 | 18 |
| 12 | B's trials | 27.51 | 447 | 16 | 28.21 | 445 | 15 |
| 18 | A's trials | 31.83 | 452 | 18 | 34.13 | 452 | 19 |
| 18 | B's trials | 30.14 | 446 | 17 | 28.84 | 446 | 16 |
| 24 | A's trials | 32.82 | 451 | 18 | 36.17 | 452 | 20 |
| 24 | B's trials | 32.29 | 452 | 18 | 30.02 | 444 | 16 |
| 36 | A's trials | 30.07 | 450 | 17 | 34.75 | 446 | 20 |
| 36 | B's trials | 31.15 | 446 | 17 | 28.23 | 447 | 15 |

Each readout fires on almost all of its units, 443 to 455 of its 459 over the 48 reads, yet 14 to 20 of them carry half of its spikes, under
the night without replay at every checkpoint.

## 3. Right after each replayed night

Each single night's form less the night without replay at its replayed night's block, and H-47's plain night less the night without
replay at blocks 6, 12, 18 and 24, its nights so far together. At block 6 H-47's plain night and after block 6 alone ran the same night on
the same image, so their rows here are the same. **Each readout's difference**, the units moved up and down beyond twice their own SE, the
fewest units carrying half the difference, the gross change, and the units moved against it:

| Comparison | Trials | Readout | The first form's spikes a trial | The second's | Difference | Moved up, beyond 2 SE | down | Fewest units to half the difference | Gross change | Moved against it, their sum |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | 20.96 | 29.27 | −8.32 ± 0.19 | 0 | 84 | 13 | 8.74 | 0, 0.00 |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | 20.19 | 27.67 | −7.48 ± 0.10 | 1 | 105 | 16 | 7.95 | 1, +0.01 |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | 20.03 | 25.68 | −5.66 ± 0.27 | 3 | 81 | 18 | 6.23 | 3, +0.05 |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | 20.75 | 26.62 | −5.87 ± 0.15 | 3 | 80 | 14 | 6.48 | 3, +0.10 |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | 22.66 | 31.15 | −8.48 ± 0.16 | 3 | 118 | 15 | 8.92 | 3, +0.07 |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | 22.11 | 31.29 | −9.17 ± 0.20 | 2 | 118 | 19 | 9.48 | 2, +0.03 |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | 21.35 | 27.51 | −6.16 ± 0.15 | 0 | 97 | 19 | 6.49 | 0, 0.00 |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | 22.13 | 28.21 | −6.08 ± 0.19 | 2 | 100 | 15 | 6.50 | 2, +0.03 |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | 24.76 | 31.83 | −7.07 ± 0.15 | 0 | 126 | 17 | 7.19 | 0, 0.00 |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | 24.64 | 34.13 | −9.49 ± 0.19 | 0 | 145 | 19 | 9.61 | 0, 0.00 |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | 23.35 | 30.14 | −6.79 ± 0.22 | 0 | 129 | 20 | 6.84 | 0, 0.00 |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | 23.36 | 28.84 | −5.48 ± 0.08 | 1 | 115 | 22 | 5.69 | 1, +0.04 |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | 25.30 | 32.82 | −7.52 ± 0.18 | 0 | 102 | 17 | 7.86 | 0, 0.00 |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | 27.34 | 36.17 | −8.83 ± 0.24 | 3 | 135 | 19 | 9.38 | 3, +0.11 |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | 24.84 | 32.29 | −7.46 ± 0.28 | 2 | 118 | 17 | 7.90 | 2, +0.07 |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | 24.23 | 30.02 | −5.79 ± 0.18 | 4 | 114 | 17 | 6.59 | 4, +0.17 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | 20.96 | 29.27 | −8.32 ± 0.19 | 0 | 84 | 13 | 8.74 | 0, 0.00 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | 20.19 | 27.67 | −7.48 ± 0.10 | 1 | 105 | 16 | 7.95 | 1, +0.01 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | 20.03 | 25.68 | −5.66 ± 0.27 | 3 | 81 | 18 | 6.23 | 3, +0.05 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | 20.75 | 26.62 | −5.87 ± 0.15 | 3 | 80 | 14 | 6.48 | 3, +0.10 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | 20.59 | 31.15 | −10.56 ± 0.20 | 3 | 101 | 10 | 11.19 | 3, +0.05 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | 21.60 | 31.29 | −9.69 ± 0.22 | 4 | 112 | 14 | 10.13 | 4, +0.13 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | 20.32 | 27.51 | −7.19 ± 0.16 | 3 | 101 | 12 | 7.50 | 3, +0.06 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | 21.05 | 28.21 | −7.16 ± 0.19 | 7 | 92 | 10 | 7.93 | 7, +0.20 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | 20.61 | 31.83 | −11.22 ± 0.24 | 16 | 81 | 9 | 13.12 | 16, +0.44 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | 22.23 | 34.13 | −11.90 ± 0.23 | 6 | 94 | 13 | 13.52 | 6, +0.50 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | 20.36 | 30.14 | −9.78 ± 0.34 | 6 | 96 | 10 | 10.53 | 6, +0.08 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | 21.78 | 28.84 | −7.06 ± 0.13 | 21 | 76 | 8 | 9.04 | 21, +0.52 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | 20.34 | 32.82 | −12.48 ± 0.20 | 16 | 86 | 10 | 14.94 | 16, +0.43 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | 22.19 | 36.17 | −13.98 ± 0.41 | 9 | 100 | 12 | 16.05 | 9, +0.54 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | 20.83 | 32.29 | −11.46 ± 0.41 | 8 | 81 | 9 | 12.82 | 8, +0.22 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | 21.66 | 30.02 | −8.36 ± 0.16 | 20 | 73 | 8 | 11.27 | 20, +0.85 |

Right after each single night's replayed night both readouts lay below the night without replay on both stimuli's trials, each beyond 2 SE:
on A's trials readout 0 by 7.07 to 8.48 spikes a trial and readout 1 by 7.48 to 9.49, on B's trials readout 1 by 5.48 to 6.08 and readout 0
by 5.66 to 7.46, 19 to 29 per cent of the night without replay's spikes at the same checkpoint. 80 to 145 of a readout's 459 units moved
down beyond twice their own SE and 0 to 4 up, against about 23 by chance in the two directions together. 13 to 22 units carry half of each
readout's difference beyond 2 SE; the gross change exceeds each readout's difference by 0.05 to 0.80, and the 0 to 4 units of a readout
moved against it sum to 0.00 to +0.17. Under H-47's plain night at block 24 the readouts' differences lay below by 8.36 to 13.98, with 73 to
112 units moved down and 3 to 21 up at blocks 12 to 24.

**The answer's readout less the other**: on A's trials readout 0 less readout 1, on B's trials readout 1 less readout 0, under each form,
and the difference with its SE:

| Comparison | A's trials: readout 0 less readout 1, the first form | the second | Difference | B's trials: readout 1 less readout 0, the first form | the second | Difference |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | 0.77 | 1.60 | −0.84 ± 0.21 | 0.72 | 0.93 | −0.21 ± 0.30 |
| After block 12 alone less without replay, block 12 | 0.55 | −0.14 | +0.69 ± 0.18 | 0.78 | 0.70 | +0.08 ± 0.19 |
| After block 18 alone less without replay, block 18 | 0.11 | −2.30 | +2.42 ± 0.23 | 0.01 | −1.30 | +1.32 ± 0.26 |
| After block 24 alone less without replay, block 24 | −2.04 | −3.35 | +1.31 ± 0.31 | −0.61 | −2.27 | +1.66 ± 0.35 |
| Plain on all four nights less without replay, block 6 | 0.77 | 1.60 | −0.84 ± 0.21 | 0.72 | 0.93 | −0.21 ± 0.30 |
| Plain on all four nights less without replay, block 12 | −1.01 | −0.14 | −0.87 ± 0.25 | 0.73 | 0.70 | +0.03 ± 0.19 |
| Plain on all four nights less without replay, block 18 | −1.63 | −2.30 | +0.68 ± 0.20 | 1.41 | −1.30 | +2.72 ± 0.39 |
| Plain on all four nights less without replay, block 24 | −1.85 | −3.35 | +1.50 ± 0.45 | 0.83 | −2.27 | +3.10 ± 0.40 |

On A's trials readout 0, A's answer, less readout 1 lay above the night without replay's beyond 2 SE after block 12, 18 and 24 alone,
+0.69 ± 0.18, +2.42 ± 0.23 and +1.31 ± 0.31; below it beyond 2 SE after block 6 alone, −0.84 ± 0.21. On B's trials readout 1, B's answer,
less readout 0 lay above the night without replay's beyond 2 SE after block 18 and 24 alone, +1.32 ± 0.26 and +1.66 ± 0.35; within 2 SE of
it after block 6 and 12 alone, −0.21 ± 0.30 and +0.08 ± 0.19. Under H-47's plain night readout 0 less readout 1 on A's trials lay above
beyond 2 SE at blocks 18 and 24 and below beyond 2 SE at blocks 6 and 12 and readout 1 less readout 0 on B's trials above beyond 2 SE at
blocks 18 and 24 and within 2 SE at blocks 6 and 12. In 16 of 16 of these comparisons on each stimulus's trials its sign is the sign of that
stimulus's correct trials less the night without replay's (section 1).

**By class**, each class's spikes a trial summed over its units, its difference with the SE of that sum, and its units moved:

| Comparison | Trials | Readout | Class | How many units | The first form's spikes a trial | The second's | Difference | Moved up | Moved down |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | C's own | 25 | 0.33 | 0.64 | −0.31 ± 0.03 | 0 | 3 |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 15.32 | 20.88 | −5.56 ± 0.14 | 0 | 52 |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 0.94 | 1.29 | −0.35 ± 0.04 | 0 | 7 |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 4.38 | 6.47 | −2.09 ± 0.07 | 0 | 22 |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | D's own | 25 | 0.46 | 0.64 | −0.18 ± 0.03 | 0 | 8 |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 8.84 | 11.86 | −3.02 ± 0.09 | 1 | 46 |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 4.17 | 5.74 | −1.57 ± 0.08 | 0 | 18 |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 6.72 | 9.43 | −2.71 ± 0.08 | 0 | 33 |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | C's own | 25 | 0.95 | 1.18 | −0.23 ± 0.03 | 0 | 5 |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 6.83 | 9.96 | −3.14 ± 0.13 | 2 | 40 |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.86 | 2.21 | −0.35 ± 0.05 | 0 | 7 |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 10.39 | 12.34 | −1.94 ± 0.16 | 1 | 29 |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | D's own | 25 | 1.23 | 1.29 | −0.06 ± 0.02 | 1 | 3 |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 11.88 | 15.19 | −3.31 ± 0.12 | 2 | 40 |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 0.43 | 0.80 | −0.36 ± 0.04 | 0 | 5 |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.20 | 9.34 | −2.14 ± 0.06 | 0 | 32 |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | C's own | 25 | 0.37 | 0.76 | −0.39 ± 0.03 | 0 | 4 |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 16.54 | 21.88 | −5.33 ± 0.13 | 3 | 64 |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.12 | 1.40 | −0.28 ± 0.03 | 0 | 9 |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 4.63 | 7.12 | −2.48 ± 0.06 | 0 | 41 |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | D's own | 25 | 0.52 | 0.72 | −0.20 ± 0.02 | 0 | 6 |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 8.82 | 12.68 | −3.86 ± 0.10 | 0 | 54 |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 4.94 | 6.65 | −1.71 ± 0.07 | 1 | 19 |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.84 | 11.24 | −3.40 ± 0.11 | 1 | 39 |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | C's own | 25 | 0.94 | 1.22 | −0.28 ± 0.04 | 0 | 6 |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 7.70 | 11.17 | −3.47 ± 0.10 | 0 | 50 |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.07 | 2.40 | −0.33 ± 0.05 | 0 | 5 |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 10.65 | 12.72 | −2.07 ± 0.10 | 0 | 36 |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | D's own | 25 | 1.16 | 1.26 | −0.10 ± 0.02 | 0 | 5 |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 12.78 | 16.01 | −3.23 ± 0.11 | 1 | 49 |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 0.59 | 0.94 | −0.35 ± 0.03 | 0 | 8 |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.60 | 10.00 | −2.40 ± 0.09 | 1 | 38 |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | C's own | 25 | 0.39 | 0.92 | −0.53 ± 0.04 | 0 | 7 |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 17.94 | 22.13 | −4.20 ± 0.13 | 0 | 73 |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.19 | 1.33 | −0.14 ± 0.02 | 0 | 5 |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 5.23 | 7.44 | −2.21 ± 0.07 | 0 | 41 |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | D's own | 25 | 0.59 | 0.82 | −0.23 ± 0.04 | 0 | 6 |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 9.83 | 13.90 | −4.07 ± 0.13 | 0 | 64 |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 5.58 | 7.32 | −1.74 ± 0.06 | 0 | 26 |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.64 | 12.09 | −3.45 ± 0.09 | 0 | 49 |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | C's own | 25 | 0.98 | 1.47 | −0.50 ± 0.02 | 0 | 11 |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 8.95 | 12.53 | −3.57 ± 0.13 | 0 | 63 |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.17 | 2.50 | −0.34 ± 0.03 | 0 | 9 |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 11.25 | 13.64 | −2.39 ± 0.10 | 0 | 46 |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | D's own | 25 | 1.16 | 1.25 | −0.09 ± 0.01 | 0 | 6 |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 13.12 | 16.14 | −3.02 ± 0.07 | 1 | 53 |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 0.89 | 1.34 | −0.45 ± 0.04 | 0 | 9 |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.20 | 10.12 | −1.92 ± 0.07 | 0 | 47 |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | C's own | 25 | 0.62 | 1.15 | −0.54 ± 0.04 | 0 | 7 |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 17.97 | 22.41 | −4.43 ± 0.17 | 0 | 55 |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.23 | 1.39 | −0.17 ± 0.03 | 0 | 5 |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 5.48 | 7.87 | −2.38 ± 0.08 | 0 | 35 |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | D's own | 25 | 0.69 | 0.73 | −0.04 ± 0.02 | 0 | 3 |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 10.75 | 14.66 | −3.90 ± 0.18 | 2 | 63 |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 6.00 | 7.89 | −1.90 ± 0.08 | 1 | 21 |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 9.89 | 12.89 | −3.00 ± 0.07 | 0 | 48 |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | C's own | 25 | 1.22 | 1.57 | −0.35 ± 0.04 | 0 | 10 |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 9.81 | 13.87 | −4.06 ± 0.24 | 1 | 57 |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.23 | 2.57 | −0.35 ± 0.04 | 1 | 8 |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 11.59 | 14.28 | −2.69 ± 0.10 | 0 | 43 |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | D's own | 25 | 1.15 | 1.26 | −0.11 ± 0.02 | 0 | 6 |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 13.28 | 16.49 | −3.21 ± 0.13 | 3 | 53 |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 1.42 | 1.88 | −0.46 ± 0.04 | 0 | 12 |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.38 | 10.39 | −2.01 ± 0.11 | 1 | 43 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | C's own | 25 | 0.33 | 0.64 | −0.31 ± 0.03 | 0 | 3 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 15.32 | 20.88 | −5.56 ± 0.14 | 0 | 52 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 0.94 | 1.29 | −0.35 ± 0.04 | 0 | 7 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 4.38 | 6.47 | −2.09 ± 0.07 | 0 | 22 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | D's own | 25 | 0.46 | 0.64 | −0.18 ± 0.03 | 0 | 8 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 8.84 | 11.86 | −3.02 ± 0.09 | 1 | 46 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 4.17 | 5.74 | −1.57 ± 0.08 | 0 | 18 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 6.72 | 9.43 | −2.71 ± 0.08 | 0 | 33 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | C's own | 25 | 0.95 | 1.18 | −0.23 ± 0.03 | 0 | 5 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 6.83 | 9.96 | −3.14 ± 0.13 | 2 | 40 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.86 | 2.21 | −0.35 ± 0.05 | 0 | 7 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 10.39 | 12.34 | −1.94 ± 0.16 | 1 | 29 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | D's own | 25 | 1.23 | 1.29 | −0.06 ± 0.02 | 1 | 3 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 11.88 | 15.19 | −3.31 ± 0.12 | 2 | 40 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 0.43 | 0.80 | −0.36 ± 0.04 | 0 | 5 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.20 | 9.34 | −2.14 ± 0.06 | 0 | 32 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | C's own | 25 | 0.32 | 0.76 | −0.44 ± 0.03 | 0 | 5 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 14.92 | 21.88 | −6.96 ± 0.19 | 3 | 56 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.20 | 1.40 | −0.19 ± 0.03 | 0 | 6 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 4.15 | 7.12 | −2.96 ± 0.07 | 0 | 34 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | D's own | 25 | 0.39 | 0.72 | −0.33 ± 0.04 | 0 | 5 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 8.89 | 12.68 | −3.79 ± 0.10 | 2 | 53 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 4.64 | 6.65 | −2.01 ± 0.09 | 1 | 17 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.68 | 11.24 | −3.56 ± 0.10 | 1 | 37 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | C's own | 25 | 0.87 | 1.22 | −0.35 ± 0.03 | 0 | 8 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 6.81 | 11.17 | −4.36 ± 0.16 | 0 | 51 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.08 | 2.40 | −0.32 ± 0.04 | 1 | 7 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 10.56 | 12.72 | −2.16 ± 0.08 | 2 | 35 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | D's own | 25 | 1.24 | 1.26 | −0.02 ± 0.02 | 2 | 3 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 12.00 | 16.01 | −4.01 ± 0.11 | 5 | 45 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 0.57 | 0.94 | −0.36 ± 0.03 | 0 | 8 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.23 | 10.00 | −2.77 ± 0.15 | 0 | 36 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | C's own | 25 | 0.29 | 0.92 | −0.63 ± 0.04 | 0 | 5 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 15.02 | 22.13 | −7.11 ± 0.23 | 16 | 42 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.22 | 1.33 | −0.12 ± 0.02 | 0 | 2 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 4.08 | 7.44 | −3.36 ± 0.09 | 0 | 32 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | D's own | 25 | 0.36 | 0.82 | −0.46 ± 0.03 | 0 | 3 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 9.25 | 13.90 | −4.66 ± 0.21 | 2 | 46 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 4.73 | 7.32 | −2.59 ± 0.06 | 1 | 15 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.90 | 12.09 | −4.19 ± 0.08 | 3 | 30 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | C's own | 25 | 0.90 | 1.47 | −0.58 ± 0.03 | 0 | 6 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 6.66 | 12.53 | −5.87 ± 0.17 | 4 | 49 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.21 | 2.50 | −0.29 ± 0.04 | 2 | 8 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 10.59 | 13.64 | −3.04 ± 0.20 | 0 | 33 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | D's own | 25 | 1.32 | 1.25 | +0.07 ± 0.02 | 5 | 3 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 11.89 | 16.14 | −4.24 ± 0.11 | 13 | 37 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 0.76 | 1.34 | −0.57 ± 0.07 | 2 | 5 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.80 | 10.12 | −2.32 ± 0.07 | 1 | 31 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | C's own | 25 | 0.30 | 1.15 | −0.85 ± 0.05 | 1 | 4 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 14.53 | 22.41 | −7.88 ± 0.19 | 13 | 43 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.28 | 1.39 | −0.11 ± 0.03 | 1 | 4 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 4.23 | 7.87 | −3.63 ± 0.09 | 1 | 35 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | D's own | 25 | 0.41 | 0.73 | −0.32 ± 0.02 | 1 | 5 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 8.72 | 14.66 | −5.93 ± 0.33 | 6 | 44 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 5.07 | 7.89 | −2.82 ± 0.10 | 1 | 14 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.99 | 12.89 | −4.90 ± 0.15 | 1 | 37 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | C's own | 25 | 0.99 | 1.57 | −0.58 ± 0.04 | 1 | 8 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 6.51 | 13.87 | −7.36 ± 0.23 | 1 | 38 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.33 | 2.57 | −0.24 ± 0.03 | 2 | 7 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 11.00 | 14.28 | −3.28 ± 0.20 | 4 | 28 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | D's own | 25 | 1.39 | 1.26 | +0.13 ± 0.03 | 4 | 3 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 11.37 | 16.49 | −5.12 ± 0.14 | 12 | 31 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 1.09 | 1.88 | −0.79 ± 0.10 | 1 | 6 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.82 | 10.39 | −2.58 ± 0.11 | 3 | 33 |

Of the 64 class sums of the four single nights right after their replayed nights, 63 lie below the night without replay beyond 2 SE, 0 above
beyond 2 SE and 1 within 2 SE; of H-47's plain night's 64 at blocks 6 to 24, 61 below, 2 above and 1 within. On A's trials, a trial: readout
0's pattern targets fired less by 4.20 to 5.56 (after block 6 alone −5.56, after block 12, 18 and 24 alone −5.33, −4.20 and −4.43); readout
0's inhibitory units fired less by 2.09 to 2.48 (after block 6 alone −2.09, after block 12, 18 and 24 alone −2.48, −2.21 and −2.38); readout
1's pattern targets fired less by 3.02 to 4.07 (after block 6 alone −3.02, after block 12, 18 and 24 alone −3.86, −4.07 and −3.90); readout
1's inhibitory units fired less by 2.71 to 3.45 (after block 6 alone −2.71, after block 12, 18 and 24 alone −3.40, −3.45 and −3.00).

**The fewest units carrying half of each readout's difference**, where that difference lies beyond 2 SE, as each readout's here does, a unit
among them within twice its own SE or beyond it, named by their id and class: T a pattern
target, E another excitatory unit, I an inhibitory unit, C C's own, D D's own. Beside each, how many of them are among the night without
replay's units carrying half its spikes at the same checkpoint:

| Comparison | Trials | Readout | Fewest units | Their differences' sum | Their classes | Among the night without replay's units carrying half its spikes | The units, by id and class, the largest difference first |
| :--- | :--- | :--- | ---: | ---: | :--- | ---: | :--- |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | 13 | −4.17 | T 9, I 4 | 4 | 7 T, 5 T, 41 T, 35 T, 33 T, 315 T, 647 T, 419 I, 649 I, 399 I, 455 T, 415 T, 759 I |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | 16 | −3.77 | T 6, E 3, I 7 | 7 | 424 I, 72 E, 648 T, 466 T, 92 T, 974 I, 702 E, 772 T, 448 T, 204 I, 254 I, 864 I, 862 E, 4 I, 858 T, 904 I |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | 18 | −2.87 | T 10, I 7, C 1 | 5 | 45 T, 115 T, 801 T, 907 T, 229 I, 43 T, 137 T, 75 T, 653 T, 569 I, 939 I, 659 I, 289 I, 649 I, 145 C, 639 I, 913 T, 5 T |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | 14 | −2.98 | T 8, I 6 | 2 | 566 T, 564 I, 6 T, 794 I, 658 T, 16 T, 134 I, 494 I, 118 T, 192 T, 268 T, 974 I, 918 T, 574 I |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | 15 | −4.37 | T 11, I 3, C 1 | 5 | 5 T, 683 T, 649 I, 419 I, 33 T, 35 T, 647 T, 41 T, 465 C, 315 T, 455 T, 399 I, 415 T, 273 T, 861 T |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | 19 | −4.65 | T 8, E 3, I 8 | 8 | 424 I, 254 I, 772 T, 904 I, 648 T, 466 T, 892 T, 774 I, 974 I, 702 E, 862 E, 902 T, 864 I, 72 E, 204 I, 448 T, 854 I, 92 T, 858 T |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | 19 | −3.12 | T 11, I 8 | 6 | 229 I, 801 T, 907 T, 45 T, 115 T, 75 T, 137 T, 659 I, 47 T, 5 T, 289 I, 43 T, 913 T, 649 I, 49 I, 179 I, 1009 I, 653 T, 639 I |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | 15 | −3.12 | T 6, E 1, I 8 | 2 | 6 T, 658 T, 494 I, 974 I, 268 T, 134 I, 794 I, 566 T, 574 I, 118 T, 564 I, 22 E, 918 T, 164 I, 454 I |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | 17 | −3.68 | T 10, I 6, C 1 | 5 | 465 C, 557 T, 649 I, 33 T, 35 T, 897 T, 759 I, 419 I, 415 T, 273 T, 889 I, 41 T, 861 T, 455 T, 339 I, 559 I, 337 T |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | 19 | −4.90 | T 8, E 2, I 9 | 6 | 72 E, 774 I, 466 T, 424 I, 648 T, 892 T, 904 I, 448 T, 894 I, 772 T, 974 I, 862 E, 552 T, 854 I, 254 I, 598 T, 204 I, 864 I, 690 T |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | 20 | −3.48 | T 12, I 7, C 1 | 6 | 659 I, 47 T, 907 T, 45 T, 5 T, 115 T, 229 I, 505 C, 49 I, 43 T, 801 T, 75 T, 639 I, 193 T, 137 T, 179 I, 1009 I, 649 I, 913 T, 273 T |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | 22 | −2.76 | T 14, E 2, I 6 | 4 | 658 T, 566 T, 794 I, 6 T, 192 T, 494 I, 134 I, 118 T, 790 T, 792 T, 1018 T, 268 T, 164 I, 22 E, 498 E, 918 T, 574 I, 204 I, 758 T, 108 T, 550 T, 766 T |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | 17 | −3.76 | T 10, I 6, C 1 | 3 | 465 C, 649 I, 33 T, 339 I, 337 T, 41 T, 35 T, 861 T, 315 T, 557 T, 459 I, 419 I, 393 T, 565 T, 415 T, 279 I, 889 I |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | 19 | −4.51 | T 8, E 4, I 7 | 5 | 774 I, 904 I, 72 E, 338 E, 648 T, 466 T, 772 T, 70 T, 142 E, 858 T, 892 T, 862 E, 424 I, 894 I, 854 I, 92 T, 864 I, 598 T, 204 I |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | 17 | −3.83 | T 9, I 8 | 4 | 5 T, 115 T, 47 T, 49 I, 907 T, 43 T, 639 I, 193 T, 137 T, 75 T, 289 I, 273 T, 659 I, 179 I, 1009 I, 969 I, 9 I |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | 17 | −2.98 | T 12, E 1, I 4 | 4 | 118 T, 6 T, 22 E, 134 I, 192 T, 566 T, 794 I, 126 T, 494 I, 1018 T, 878 T, 268 T, 686 T, 108 T, 918 T, 658 T, 164 I |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | 13 | −4.17 | T 9, I 4 | 4 | 7 T, 5 T, 41 T, 35 T, 33 T, 315 T, 647 T, 419 I, 649 I, 399 I, 455 T, 415 T, 759 I |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | 16 | −3.77 | T 6, E 3, I 7 | 7 | 424 I, 72 E, 648 T, 466 T, 92 T, 974 I, 702 E, 772 T, 448 T, 204 I, 254 I, 864 I, 862 E, 4 I, 858 T, 904 I |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | 18 | −2.87 | T 10, I 7, C 1 | 5 | 45 T, 115 T, 801 T, 907 T, 229 I, 43 T, 137 T, 75 T, 653 T, 569 I, 939 I, 659 I, 289 I, 649 I, 145 C, 639 I, 913 T, 5 T |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | 14 | −2.98 | T 8, I 6 | 2 | 566 T, 564 I, 6 T, 794 I, 658 T, 16 T, 134 I, 494 I, 118 T, 192 T, 268 T, 974 I, 918 T, 574 I |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | 10 | −5.48 | T 6, I 3, C 1 | 5 | 7 T, 5 T, 41 T, 647 T, 419 I, 759 I, 33 T, 35 T, 399 I, 465 C |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | 14 | −5.00 | T 6, E 3, I 5 | 7 | 424 I, 254 I, 702 E, 772 T, 974 I, 466 T, 92 T, 904 I, 862 E, 72 E, 648 T, 864 I, 598 T, 858 T |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | 12 | −3.65 | T 8, I 4 | 3 | 801 T, 45 T, 907 T, 115 T, 229 I, 659 I, 75 T, 137 T, 569 I, 639 I, 653 T, 5 T |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | 10 | −3.77 | T 5, I 5 | 3 | 6 T, 566 T, 658 T, 794 I, 494 I, 118 T, 564 I, 16 T, 134 I, 974 I |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | 9 | −5.81 | T 5, I 3, C 1 | 5 | 7 T, 5 T, 647 T, 759 I, 41 T, 419 I, 33 T, 465 C, 399 I |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | 13 | −6.12 | T 5, E 3, I 4, D 1 | 6 | 254 I, 424 I, 772 T, 72 E, 904 I, 466 T, 92 T, 774 I, 598 T, 648 T, 278 E, 756 D, 702 E |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | 10 | −5.05 | T 6, I 3, C 1 | 4 | 45 T, 801 T, 907 T, 47 T, 659 I, 229 I, 115 T, 569 I, 505 C, 5 T |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | 8 | −3.65 | T 6, E 1, I 1 | 2 | 6 T, 658 T, 566 T, 22 E, 16 T, 794 I, 118 T, 192 T |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | 10 | −6.39 | T 7, I 2, C 1 | 5 | 7 T, 5 T, 647 T, 41 T, 465 C, 759 I, 33 T, 419 I, 35 T, 393 T |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | 12 | −7.15 | T 5, E 3, I 4 | 6 | 254 I, 72 E, 424 I, 772 T, 774 I, 92 T, 904 I, 466 T, 598 T, 142 E, 448 T, 862 E |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | 9 | −6.17 | T 6, I 3 | 4 | 45 T, 47 T, 907 T, 801 T, 659 I, 115 T, 49 I, 5 T, 229 I |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | 8 | −4.46 | T 6, E 1, I 1 | 3 | 6 T, 22 E, 658 T, 566 T, 16 T, 118 T, 192 T, 794 I |

**The four single nights' fewest units, set side by side**:

| Trials | Readout | After block 6 alone, at 6 | after 12 alone, at 12 | after 18 alone, at 18 | after 24 alone, at 24 | In all four | In any | The units in all four |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
| A's trials | readout 0, the odd places | 13 | 15 | 17 | 17 | 6 | 27 | 41 T, 35 T, 33 T, 419 I, 649 I, 415 T |
| A's trials | readout 1, the even places | 16 | 19 | 19 | 19 | 9 | 27 | 424 I, 72 E, 648 T, 466 T, 772 T, 204 I, 864 I, 862 E, 904 I |
| B's trials | readout 0, the odd places | 18 | 19 | 20 | 17 | 8 | 27 | 115 T, 907 T, 43 T, 137 T, 75 T, 659 I, 639 I, 5 T |
| B's trials | readout 1, the even places | 14 | 15 | 22 | 17 | 9 | 29 | 566 T, 6 T, 794 I, 658 T, 134 I, 494 I, 118 T, 268 T, 918 T |

The fewest units carrying half of each single night's readout difference beyond 2 SE are 13 to 22, in 16 of its 16 sets. On A's trials
readout 0's are 9 to 11 pattern targets, 3 to 6 inhibitory units and 0 or 1 of C's own, readout 1's 6 to 8 pattern targets, 7 to 9
inhibitory units and 2 to 4 other excitatory units. Of the 27 to 29 units among any of the four single nights' fewest of a readout on a
stimulus's trials, 6 to 9 are among all four's. Under H-47's plain night the fewest are 8 to 18 at blocks 6 to 24.

## 4. At block 36

The same at block 36: each form less the night without replay, paired in its job; after block 6 alone less H-47's plain night, paired, both
in job one; after block 12, 18 or 24 alone less H-47's plain night, across the two jobs, with exploration 018's unpaired SE:

| Comparison | Trials | Readout | The first form's spikes a trial | The second's | Difference | Moved up, beyond 2 SE | down | Fewest units to half the difference | Gross change | Moved against it, their sum |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | 28.32 | 30.07 | −1.75 ± 0.22 | 52 | 46 | 3 | 5.14 | 52, +0.88 |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | 32.37 | 34.75 | −2.38 ± 0.25 | 44 | 55 | 5 | 6.19 | 44, +1.44 |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | 28.40 | 31.15 | −2.74 ± 0.15 | 32 | 37 | 3 | 5.07 | 32, +0.69 |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | 26.46 | 28.23 | −1.76 ± 0.33 | 38 | 32 | 4 | 3.63 | 38, +0.41 |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | 27.31 | 30.07 | −2.76 ± 0.20 | 36 | 50 | 6 | 4.29 | 36, +0.28 |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | 31.03 | 34.75 | −3.71 ± 0.27 | 24 | 55 | 8 | 5.55 | 24, +0.55 |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | 29.03 | 31.15 | −2.11 ± 0.17 | 27 | 42 | 3 | 3.65 | 27, +0.40 |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | 27.29 | 28.23 | −0.93 ± 0.22 | 30 | 25 | 3 | 2.82 | 30, +0.58 |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | 27.50 | 30.07 | −2.57 ± 0.26 | 36 | 48 | 8 | 4.33 | 36, +0.25 |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | 30.76 | 34.75 | −3.99 ± 0.16 | 31 | 57 | 9 | 5.71 | 31, +0.47 |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | 28.73 | 31.15 | −2.42 ± 0.12 | 23 | 35 | 6 | 3.65 | 23, +0.20 |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | 27.40 | 28.23 | −0.83 ± 0.21 | 39 | 29 | 4 | 2.72 | 39, +0.47 |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | 26.75 | 30.07 | −3.32 ± 0.24 | 36 | 47 | 10 | 4.89 | 36, +0.28 |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | 31.71 | 34.75 | −3.04 ± 0.12 | 26 | 47 | 8 | 4.98 | 26, +0.40 |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | 28.13 | 31.15 | −3.02 ± 0.27 | 21 | 37 | 7 | 4.31 | 21, +0.17 |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | 26.36 | 28.23 | −1.87 ± 0.17 | 36 | 31 | 5 | 3.68 | 36, +0.37 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | 21.26 | 30.07 | −8.81 ± 0.13 | 66 | 57 | 7 | 13.41 | 66, +1.30 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | 24.55 | 34.75 | −10.20 ± 0.39 | 34 | 71 | 9 | 14.22 | 34, +1.08 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | 22.02 | 31.15 | −9.13 ± 0.31 | 35 | 50 | 6 | 11.87 | 35, +0.62 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | 22.90 | 28.23 | −5.33 ± 0.15 | 62 | 40 | 5 | 10.15 | 62, +1.51 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 28.32 | 21.26 | +7.06 ± 0.31 | 61 | 46 | 6 | 10.43 | 46, −0.80 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 32.37 | 24.55 | +7.83 ± 0.49 | 67 | 26 | 8 | 10.15 | 26, −0.33 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 28.40 | 22.02 | +6.38 ± 0.19 | 57 | 27 | 6 | 8.46 | 27, −0.38 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 26.46 | 22.90 | +3.56 ± 0.32 | 54 | 42 | 4 | 7.69 | 42, −1.13 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 27.31 | 21.26 | +6.04 ± 0.39 | 34 | 38 | 4 | 10.39 | 38, −1.06 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 31.03 | 24.55 | +6.49 ± 0.43 | 47 | 25 | 7 | 10.30 | 25, −1.06 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 29.03 | 22.02 | +7.01 ± 0.28 | 35 | 19 | 6 | 9.34 | 19, −0.40 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 27.29 | 22.90 | +4.39 ± 0.32 | 36 | 40 | 4 | 8.68 | 40, −1.18 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 27.50 | 21.26 | +6.24 ± 0.43 | 54 | 42 | 5 | 10.49 | 42, −0.93 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 30.76 | 24.55 | +6.21 ± 0.60 | 60 | 28 | 6 | 10.50 | 28, −1.22 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 28.73 | 22.02 | +6.71 ± 0.38 | 50 | 22 | 5 | 9.31 | 22, −0.47 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 27.40 | 22.90 | +4.50 ± 0.28 | 51 | 45 | 4 | 9.19 | 45, −1.48 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 26.75 | 21.26 | +5.48 ± 0.42 | 38 | 35 | 4 | 9.58 | 35, −0.94 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 31.71 | 24.55 | +7.16 ± 0.53 | 50 | 19 | 7 | 10.49 | 19, −0.96 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 28.13 | 22.02 | +6.11 ± 0.25 | 40 | 9 | 5 | 8.21 | 9, −0.23 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 26.36 | 22.90 | +3.45 ± 0.25 | 32 | 30 | 4 | 7.06 | 30, −0.91 |

At block 36, of the single nights' 16 readout differences 16 lie below the night without replay beyond 2 SE, 0 above beyond 2 SE and 0
within 2 SE, by 0.83 to 3.99 below; 16 of 16 lie nearer the night without replay than right after the replayed night. 25 to 57 units moved
down and 21 to 52 up, and 3 to 10 units carry half of each difference beyond 2 SE. H-47's plain night's four lay below by 5.33 to 10.20, 4
below beyond 2 SE, 0 above; each single night less H-47's plain night lay above it by 3.45 to 7.83, 0 of 16 below beyond 2 SE and 16 above.

| Comparison | A's trials: readout 0 less readout 1, the first form | the second | Difference | B's trials: readout 1 less readout 0, the first form | the second | Difference |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 36 | −4.05 | −4.68 | +0.63 ± 0.33 | −1.94 | −2.92 | +0.98 ± 0.42 |
| After block 12 alone less without replay, block 36 | −3.73 | −4.68 | +0.95 ± 0.21 | −1.74 | −2.92 | +1.18 ± 0.38 |
| After block 18 alone less without replay, block 36 | −3.26 | −4.68 | +1.42 ± 0.35 | −1.33 | −2.92 | +1.59 ± 0.23 |
| After block 24 alone less without replay, block 36 | −4.96 | −4.68 | −0.28 ± 0.15 | −1.77 | −2.92 | +1.15 ± 0.41 |
| Plain on all four nights less without replay, block 36 | −3.28 | −4.68 | +1.39 ± 0.49 | 0.88 | −2.92 | +3.80 ± 0.35 |
| After block 6 alone less plain on all four nights, block 36 | −4.05 | −3.28 | −0.77 ± 0.73 | −1.94 | 0.88 | −2.82 ± 0.36 |
| After block 12 alone less plain on all four nights, block 36 | −3.73 | −3.28 | −0.44 ± 0.39 | −1.74 | 0.88 | −2.62 ± 0.24 |
| After block 18 alone less plain on all four nights, block 36 | −3.26 | −3.28 | +0.02 ± 0.81 | −1.33 | 0.88 | −2.21 ± 0.24 |
| After block 24 alone less plain on all four nights, block 36 | −4.96 | −3.28 | −1.68 ± 0.62 | −1.77 | 0.88 | −2.65 ± 0.28 |

At block 36 readout 0 less readout 1 on A's trials lay, less the night without replay's, above beyond 2 SE after block 12 and 18 alone,
+0.95 ± 0.21 and +1.42 ± 0.35; within 2 SE after block 6 and 24 alone, +0.63 ± 0.33 and −0.28 ± 0.15; readout 1 less readout 0 on B's trials
above beyond 2 SE after block 6, 12, 18 and 24 alone, +0.98 ± 0.42, +1.18 ± 0.38, +1.59 ± 0.23 and +1.15 ± 0.41. Under H-47's plain night
they read +1.39 ± 0.49 and +3.80 ± 0.35. Less H-47's plain night, readout 0 less readout 1 on A's trials lay below beyond 2 SE after block
24 alone, −1.68 ± 0.62; within 2 SE after block 6, 12 and 18 alone, −0.77 ± 0.73, −0.44 ± 0.39 and +0.02 ± 0.81; readout 1 less readout 0 on
B's trials below beyond 2 SE after block 6, 12, 18 and 24 alone, −2.82 ± 0.36, −2.62 ± 0.24, −2.21 ± 0.24 and −2.65 ± 0.28.

**By class at block 36**, beside each class's difference right after its replayed night (for H-47's plain night, at block 24):

| Comparison | Trials | Readout | Class | How many units | The first form's spikes a trial | The second's | Difference | Right after its replayed night | Moved up | Moved down |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.54 | 1.07 | −0.53 ± 0.03 | −0.31 ± 0.03 | 1 | 4 |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 20.06 | 20.77 | −0.71 ± 0.15 | −5.56 ± 0.14 | 41 | 24 |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.30 | 1.26 | +0.04 ± 0.02 | −0.35 ± 0.04 | 4 | 1 |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 6.41 | 6.97 | −0.55 ± 0.06 | −2.09 ± 0.07 | 6 | 17 |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.59 | 0.71 | −0.12 ± 0.01 | −0.18 ± 0.03 | 0 | 2 |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 12.76 | 14.07 | −1.30 ± 0.12 | −3.02 ± 0.09 | 24 | 26 |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 7.36 | 7.68 | −0.32 ± 0.08 | −1.57 ± 0.08 | 8 | 10 |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 11.66 | 12.29 | −0.63 ± 0.14 | −2.71 ± 0.08 | 12 | 17 |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.54 | 1.53 | +0.01 ± 0.04 | −0.23 ± 0.03 | 1 | 2 |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 11.01 | 13.24 | −2.23 ± 0.10 | −3.14 ± 0.13 | 12 | 22 |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.48 | 2.47 | +0.01 ± 0.02 | −0.35 ± 0.05 | 4 | 1 |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 13.38 | 13.91 | −0.53 ± 0.03 | −1.94 ± 0.16 | 15 | 12 |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.27 | 1.22 | +0.05 ± 0.01 | −0.06 ± 0.02 | 3 | 0 |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 14.95 | 15.82 | −0.87 ± 0.14 | −3.31 ± 0.12 | 27 | 17 |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 1.35 | 1.78 | −0.43 ± 0.03 | −0.36 ± 0.04 | 1 | 2 |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.89 | 9.40 | −0.51 ± 0.17 | −2.14 ± 0.06 | 7 | 13 |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.61 | 1.07 | −0.46 ± 0.02 | −0.39 ± 0.03 | 1 | 3 |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 19.18 | 20.77 | −1.59 ± 0.21 | −5.33 ± 0.13 | 34 | 31 |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.27 | 1.26 | +0.01 ± 0.01 | −0.28 ± 0.03 | 0 | 2 |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 6.24 | 6.97 | −0.72 ± 0.03 | −2.48 ± 0.06 | 1 | 14 |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.72 | 0.71 | +0.02 ± 0.02 | −0.20 ± 0.02 | 0 | 1 |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 12.17 | 14.07 | −1.90 ± 0.21 | −3.86 ± 0.10 | 11 | 26 |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 7.21 | 7.68 | −0.47 ± 0.05 | −1.71 ± 0.07 | 4 | 8 |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 10.93 | 12.29 | −1.37 ± 0.11 | −3.40 ± 0.11 | 9 | 20 |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.45 | 1.53 | −0.08 ± 0.02 | −0.28 ± 0.04 | 1 | 5 |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 11.69 | 13.24 | −1.55 ± 0.11 | −3.47 ± 0.10 | 12 | 18 |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.41 | 2.47 | −0.06 ± 0.01 | −0.33 ± 0.05 | 1 | 5 |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 13.48 | 13.91 | −0.43 ± 0.09 | −2.07 ± 0.10 | 13 | 14 |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.24 | 1.22 | +0.02 ± 0.00 | −0.10 ± 0.02 | 2 | 0 |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 15.27 | 15.82 | −0.55 ± 0.08 | −3.23 ± 0.11 | 20 | 12 |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 1.92 | 1.78 | +0.14 ± 0.05 | −0.35 ± 0.03 | 1 | 1 |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.86 | 9.40 | −0.54 ± 0.12 | −2.40 ± 0.09 | 7 | 12 |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.69 | 1.07 | −0.38 ± 0.03 | −0.53 ± 0.04 | 1 | 3 |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 19.43 | 20.77 | −1.34 ± 0.15 | −4.20 ± 0.13 | 31 | 25 |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.30 | 1.26 | +0.03 ± 0.01 | −0.14 ± 0.02 | 0 | 1 |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 6.08 | 6.97 | −0.88 ± 0.11 | −2.21 ± 0.07 | 4 | 19 |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.75 | 0.71 | +0.04 ± 0.02 | −0.23 ± 0.04 | 2 | 1 |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 12.24 | 14.07 | −1.83 ± 0.14 | −4.07 ± 0.13 | 15 | 25 |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 6.86 | 7.68 | −0.82 ± 0.10 | −1.74 ± 0.06 | 4 | 12 |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 10.91 | 12.29 | −1.38 ± 0.07 | −3.45 ± 0.09 | 10 | 19 |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.26 | 1.53 | −0.27 ± 0.04 | −0.50 ± 0.02 | 0 | 6 |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 12.19 | 13.24 | −1.05 ± 0.09 | −3.57 ± 0.13 | 11 | 13 |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.44 | 2.47 | −0.03 ± 0.01 | −0.34 ± 0.03 | 4 | 2 |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 12.84 | 13.91 | −1.06 ± 0.14 | −2.39 ± 0.10 | 8 | 14 |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.27 | 1.22 | +0.05 ± 0.01 | −0.09 ± 0.01 | 3 | 0 |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 15.45 | 15.82 | −0.37 ± 0.14 | −3.02 ± 0.07 | 25 | 18 |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 1.62 | 1.78 | −0.17 ± 0.01 | −0.45 ± 0.04 | 3 | 1 |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 9.07 | 9.40 | −0.33 ± 0.09 | −1.92 ± 0.07 | 8 | 10 |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.79 | 1.07 | −0.27 ± 0.01 | −0.54 ± 0.04 | 2 | 4 |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 18.56 | 20.77 | −2.22 ± 0.13 | −4.43 ± 0.17 | 25 | 28 |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.29 | 1.26 | +0.02 ± 0.01 | −0.17 ± 0.03 | 3 | 0 |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 6.11 | 6.97 | −0.86 ± 0.13 | −2.38 ± 0.08 | 6 | 15 |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.74 | 0.71 | +0.04 ± 0.02 | −0.04 ± 0.02 | 2 | 1 |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 12.64 | 14.07 | −1.42 ± 0.09 | −3.90 ± 0.18 | 13 | 22 |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 6.99 | 7.68 | −0.69 ± 0.05 | −1.90 ± 0.08 | 4 | 11 |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 11.34 | 12.29 | −0.96 ± 0.10 | −3.00 ± 0.07 | 7 | 13 |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.44 | 1.53 | −0.09 ± 0.01 | −0.35 ± 0.04 | 1 | 4 |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 11.31 | 13.24 | −1.93 ± 0.18 | −4.06 ± 0.24 | 8 | 16 |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.46 | 2.47 | −0.01 ± 0.01 | −0.35 ± 0.04 | 4 | 2 |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 12.92 | 13.91 | −0.99 ± 0.09 | −2.69 ± 0.10 | 8 | 15 |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.22 | 1.22 | 0.00 ± 0.01 | −0.11 ± 0.02 | 2 | 0 |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 14.66 | 15.82 | −1.16 ± 0.10 | −3.21 ± 0.13 | 23 | 14 |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 1.60 | 1.78 | −0.18 ± 0.02 | −0.46 ± 0.04 | 4 | 2 |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.88 | 9.40 | −0.53 ± 0.09 | −2.01 ± 0.11 | 7 | 15 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.39 | 1.07 | −0.68 ± 0.03 | −0.85 ± 0.05 | 3 | 3 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 15.04 | 20.77 | −5.74 ± 0.17 | −7.88 ± 0.19 | 52 | 29 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.31 | 1.26 | +0.05 ± 0.02 | −0.11 ± 0.03 | 5 | 3 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 4.52 | 6.97 | −2.44 ± 0.10 | −3.63 ± 0.09 | 6 | 22 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.43 | 0.71 | −0.27 ± 0.04 | −0.32 ± 0.02 | 2 | 3 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 9.62 | 14.07 | −4.45 ± 0.26 | −5.93 ± 0.33 | 15 | 34 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 5.76 | 7.68 | −1.92 ± 0.07 | −2.82 ± 0.10 | 6 | 15 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.73 | 12.29 | −3.56 ± 0.23 | −4.90 ± 0.15 | 11 | 19 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.09 | 1.53 | −0.44 ± 0.04 | −0.58 ± 0.04 | 3 | 6 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 7.21 | 13.24 | −6.03 ± 0.22 | −7.36 ± 0.23 | 14 | 25 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.47 | 2.47 | 0.00 ± 0.04 | −0.24 ± 0.03 | 8 | 5 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 11.26 | 13.91 | −2.65 ± 0.15 | −3.28 ± 0.20 | 10 | 14 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.46 | 1.22 | +0.23 ± 0.03 | +0.13 ± 0.03 | 7 | 0 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 12.20 | 15.82 | −3.62 ± 0.16 | −5.12 ± 0.14 | 46 | 22 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 1.24 | 1.78 | −0.54 ± 0.09 | −0.79 ± 0.10 | 3 | 2 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.00 | 9.40 | −1.40 ± 0.08 | −2.58 ± 0.11 | 6 | 16 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.54 | 0.39 | +0.15 ± 0.03 | – | 2 | 1 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 20.06 | 15.04 | +5.03 ± 0.30 | – | 33 | 39 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.30 | 1.31 | −0.01 ± 0.02 | – | 6 | 3 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 6.41 | 4.52 | +1.89 ± 0.07 | – | 20 | 3 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.59 | 0.43 | +0.15 ± 0.03 | – | 4 | 1 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 12.76 | 9.62 | +3.15 ± 0.19 | – | 29 | 14 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 7.36 | 5.76 | +1.60 ± 0.06 | – | 17 | 4 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 11.66 | 8.73 | +2.93 ± 0.29 | – | 17 | 7 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.54 | 1.09 | +0.45 ± 0.02 | – | 7 | 2 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 11.01 | 7.21 | +3.80 ± 0.17 | – | 31 | 10 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.48 | 2.47 | +0.01 ± 0.03 | – | 6 | 8 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 13.38 | 11.26 | +2.12 ± 0.12 | – | 13 | 7 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.27 | 1.46 | −0.19 ± 0.02 | – | 0 | 7 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 14.95 | 12.20 | +2.75 ± 0.13 | – | 26 | 31 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 1.35 | 1.24 | +0.11 ± 0.11 | – | 7 | 2 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.89 | 8.00 | +0.89 ± 0.16 | – | 21 | 2 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.61 | 0.39 | +0.22 ± 0.04 | – | 2 | 0 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 19.18 | 15.04 | +4.14 ± 0.36 | – | 24 | 32 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.27 | 1.31 | −0.04 ± 0.02 | – | 0 | 2 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 6.24 | 4.52 | +1.72 ± 0.12 | – | 8 | 4 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.72 | 0.43 | +0.29 ± 0.05 | – | 2 | 3 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 12.17 | 9.62 | +2.55 ± 0.23 | – | 21 | 10 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 7.21 | 5.76 | +1.45 ± 0.09 | – | 10 | 4 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 10.93 | 8.73 | +2.19 ± 0.20 | – | 14 | 8 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.45 | 1.09 | +0.36 ± 0.04 | – | 3 | 2 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 11.69 | 7.21 | +4.49 ± 0.19 | – | 19 | 7 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.41 | 2.47 | −0.06 ± 0.04 | – | 3 | 6 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 13.48 | 11.26 | +2.22 ± 0.17 | – | 10 | 4 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.24 | 1.46 | −0.21 ± 0.03 | – | 0 | 7 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 15.27 | 12.20 | +3.07 ± 0.18 | – | 20 | 28 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 1.92 | 1.24 | +0.68 ± 0.07 | – | 3 | 0 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.86 | 8.00 | +0.86 ± 0.14 | – | 13 | 5 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.69 | 0.39 | +0.29 ± 0.03 | – | 3 | 2 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 19.43 | 15.04 | +4.39 ± 0.34 | – | 30 | 33 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.30 | 1.31 | −0.01 ± 0.02 | – | 3 | 2 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 6.08 | 4.52 | +1.56 ± 0.10 | – | 18 | 5 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.75 | 0.43 | +0.31 ± 0.04 | – | 2 | 1 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 12.24 | 9.62 | +2.63 ± 0.34 | – | 30 | 13 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 6.86 | 5.76 | +1.10 ± 0.10 | – | 13 | 7 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 10.91 | 8.73 | +2.18 ± 0.24 | – | 15 | 7 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.26 | 1.09 | +0.18 ± 0.04 | – | 4 | 2 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 12.19 | 7.21 | +4.98 ± 0.22 | – | 26 | 9 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.44 | 2.47 | −0.03 ± 0.03 | – | 5 | 5 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 12.84 | 11.26 | +1.58 ± 0.29 | – | 15 | 6 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.27 | 1.46 | −0.18 ± 0.03 | – | 0 | 7 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 15.45 | 12.20 | +3.24 ± 0.20 | – | 29 | 34 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 1.62 | 1.24 | +0.37 ± 0.07 | – | 7 | 1 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 9.07 | 8.00 | +1.07 ± 0.11 | – | 15 | 3 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.79 | 0.39 | +0.40 ± 0.03 | – | 2 | 1 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 18.56 | 15.04 | +3.52 ± 0.32 | – | 22 | 33 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 1.29 | 1.31 | −0.02 ± 0.02 | – | 0 | 0 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 6.11 | 4.52 | +1.58 ± 0.13 | – | 14 | 1 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.74 | 0.43 | +0.31 ± 0.04 | – | 2 | 1 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 254 | 12.64 | 9.62 | +3.03 ± 0.30 | – | 24 | 6 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 78 | 6.99 | 5.76 | +1.22 ± 0.08 | – | 10 | 6 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 11.34 | 8.73 | +2.60 ± 0.20 | – | 14 | 6 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.44 | 1.09 | +0.36 ± 0.04 | – | 4 | 0 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 280 | 11.31 | 7.21 | +4.10 ± 0.19 | – | 21 | 4 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 52 | 2.46 | 2.47 | −0.01 ± 0.03 | – | 3 | 2 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 12.92 | 11.26 | +1.66 ± 0.14 | – | 12 | 3 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.22 | 1.46 | −0.23 ± 0.02 | – | 0 | 6 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 254 | 14.66 | 12.20 | +2.45 ± 0.13 | – | 19 | 21 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 78 | 1.60 | 1.24 | +0.36 ± 0.07 | – | 1 | 0 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.88 | 8.00 | +0.88 ± 0.11 | – | 12 | 3 |

At block 36 61 of 64 class sums of the single nights lie nearer the night without replay than right after their replayed nights, and 15 of
16 of H-47's plain night's against block 24. Of the single nights' 64, 49 lie below beyond 2 SE, 8 within 2 SE and 7 above beyond 2 SE.

**The fewest units carrying half of each readout's difference at block 36**:

| Comparison | Trials | Readout | Fewest units | Their differences' sum | Their classes | Among the night without replay's units carrying half its spikes | The units, by id and class, the largest difference first |
| :--- | :--- | :--- | ---: | ---: | :--- | ---: | :--- |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | 3 | −1.13 | T 3 | 1 | 41 T, 33 T, 35 T |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | 5 | −1.41 | T 4, I 1 | 1 | 424 I, 466 T, 70 T, 552 T, 598 T |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | 3 | −1.59 | T 2, I 1 | 1 | 47 T, 49 I, 115 T |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | 4 | −0.96 | T 3, E 1 | 1 | 22 E, 566 T, 118 T, 6 T |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | 6 | −1.41 | T 4, C 2 | 2 | 33 T, 465 C, 35 T, 41 T, 683 T, 705 C |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | 8 | −1.93 | T 4, E 1, I 3 | 2 | 904 I, 552 T, 466 T, 772 T, 774 I, 424 I, 70 T, 862 E |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | 3 | −1.09 | T 2, I 1 | 1 | 47 T, 49 I, 193 T |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | 3 | −0.49 | T 1, I 2 | 0 | 118 T, 494 I, 794 I |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | 8 | −1.42 | T 5, I 2, C 1 | 2 | 465 C, 557 T, 273 T, 565 T, 649 I, 279 I, 41 T, 33 T |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | 9 | −2.08 | T 3, E 1, I 5 | 1 | 552 T, 466 T, 774 I, 424 I, 70 T, 894 I, 854 I, 862 E, 904 I |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | 6 | −1.37 | T 3, I 3 | 2 | 49 I, 47 T, 659 I, 639 I, 115 T, 907 T |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | 4 | −0.47 | T 2, E 1, I 1 | 0 | 498 E, 1018 T, 494 I, 108 T |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | 10 | −1.78 | T 8, I 1, C 1 | 2 | 41 T, 337 T, 393 T, 339 I, 557 T, 33 T, 315 T, 465 C, 35 T, 683 T |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | 8 | −1.60 | T 3, E 2, I 3 | 1 | 774 I, 70 T, 338 E, 772 T, 142 E, 904 I, 466 T, 894 I |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | 7 | −1.71 | T 5, I 2 | 2 | 115 T, 47 T, 5 T, 49 I, 639 I, 907 T, 75 T |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | 5 | −0.96 | T 4, E 1 | 1 | 118 T, 22 E, 6 T, 126 T, 192 T |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | 7 | −4.67 | T 4, I 2, C 1 | 5 | 7 T, 5 T, 647 T, 41 T, 465 C, 419 I, 759 I |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | 9 | −5.46 | T 4, E 1, I 4 | 5 | 254 I, 772 T, 424 I, 904 I, 92 T, 774 I, 466 T, 72 E, 552 T |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | 6 | −4.72 | T 5, I 1 | 4 | 45 T, 47 T, 907 T, 801 T, 659 I, 115 T |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | 5 | −3.00 | T 4, E 1 | 2 | 6 T, 22 E, 658 T, 16 T, 118 T |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 6 | +3.87 | T 5, I 1 | 4 | 7 T, 5 T, 647 T, 419 I, 565 T, 683 T |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 8 | +4.01 | T 3, E 1, I 4 | 5 | 254 I, 774 I, 772 T, 72 E, 904 I, 92 T, 690 T, 424 I |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 6 | +3.48 | T 4, I 2 | 3 | 45 T, 801 T, 907 T, 229 I, 659 I, 193 T |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 4 | +2.00 | T 4 | 1 | 658 T, 6 T, 16 T, 192 T |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 4 | +3.10 | T 3, I 1 | 3 | 7 T, 5 T, 647 T, 419 I |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 7 | +3.35 | T 3, E 2, I 2 | 5 | 254 I, 72 E, 424 I, 772 T, 92 T, 142 E, 598 T |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 6 | +3.90 | T 4, I 2 | 4 | 45 T, 801 T, 907 T, 659 I, 229 I, 47 T |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 4 | +2.50 | T 3, E 1 | 2 | 6 T, 658 T, 22 E, 16 T |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 5 | +3.38 | T 4, I 1 | 5 | 7 T, 5 T, 647 T, 41 T, 759 I |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 6 | +3.14 | T 2, E 1, I 3 | 5 | 254 I, 772 T, 72 E, 92 T, 424 I, 904 I |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 5 | +3.53 | T 4, I 1 | 4 | 45 T, 801 T, 47 T, 907 T, 229 I |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 4 | +2.57 | T 3, E 1 | 2 | 6 T, 658 T, 22 E, 16 T |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 4 | +3.01 | T 3, I 1 | 4 | 7 T, 5 T, 647 T, 759 I |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 7 | +3.66 | T 3, E 1, I 3 | 5 | 254 I, 424 I, 72 E, 772 T, 552 T, 92 T, 904 I |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 5 | +3.50 | T 4, I 1 | 4 | 45 T, 801 T, 47 T, 659 I, 907 T |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 4 | +2.07 | T 3, E 1 | 2 | 658 T, 6 T, 22 E, 16 T |

At block 36, 3 to 10 units carry half of each single night's readout difference beyond 2 SE, in 16 of 16 sets, and 5 to 9 of H-47's plain
night's, in 4 of 4.

**Each unit at block 36 beside right after its replayed night**, over all of a readout's units: the readout's difference right after and at
block 36 and the change between; how many units moved right after, and of those how many moved the same way at block 36; how many moved at
block 36 and not right after; and the correlation of the units' differences right after with theirs at block 36, over the readout's
459 units:

| Form | Blocks | Trials | Readout | Less without replay right after | At block 36 | Change | Moved right after, down and up | Of those, moved the same way at 36, down and up | Moved at 36 and not right after | Correlation of the units' differences, right after with 36 |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone | 6 → 36 | A's trials | readout 0, the odd places | −8.32 ± 0.19 | −1.75 ± 0.22 | +6.57 | 84, 0 | 25, 0 | 65 | 0.43 |
| After block 6 alone | 6 → 36 | A's trials | readout 1, the even places | −7.48 ± 0.10 | −2.38 ± 0.25 | +5.10 | 105, 1 | 37, 1 | 53 | 0.54 |
| After block 6 alone | 6 → 36 | B's trials | readout 0, the odd places | −5.66 ± 0.27 | −2.74 ± 0.15 | +2.91 | 81, 3 | 23, 1 | 39 | 0.41 |
| After block 6 alone | 6 → 36 | B's trials | readout 1, the even places | −5.87 ± 0.15 | −1.76 ± 0.33 | +4.11 | 80, 3 | 24, 2 | 39 | 0.65 |
| After block 12 alone | 12 → 36 | A's trials | readout 0, the odd places | −8.48 ± 0.16 | −2.76 ± 0.20 | +5.72 | 118, 3 | 39, 2 | 40 | 0.68 |
| After block 12 alone | 12 → 36 | A's trials | readout 1, the even places | −9.17 ± 0.20 | −3.71 ± 0.27 | +5.46 | 118, 2 | 44, 1 | 25 | 0.72 |
| After block 12 alone | 12 → 36 | B's trials | readout 0, the odd places | −6.16 ± 0.15 | −2.11 ± 0.17 | +4.05 | 97, 0 | 28, 0 | 36 | 0.49 |
| After block 12 alone | 12 → 36 | B's trials | readout 1, the even places | −6.08 ± 0.19 | −0.93 ± 0.22 | +5.14 | 100, 2 | 21, 1 | 26 | 0.50 |
| After block 18 alone | 18 → 36 | A's trials | readout 0, the odd places | −7.07 ± 0.15 | −2.57 ± 0.26 | +4.50 | 126, 0 | 40, 0 | 40 | 0.85 |
| After block 18 alone | 18 → 36 | A's trials | readout 1, the even places | −9.49 ± 0.19 | −3.99 ± 0.16 | +5.50 | 145, 0 | 51, 0 | 27 | 0.77 |
| After block 18 alone | 18 → 36 | B's trials | readout 0, the odd places | −6.79 ± 0.22 | −2.42 ± 0.12 | +4.38 | 129, 0 | 29, 0 | 20 | 0.79 |
| After block 18 alone | 18 → 36 | B's trials | readout 1, the even places | −5.48 ± 0.08 | −0.83 ± 0.21 | +4.65 | 115, 1 | 27, 1 | 31 | 0.51 |
| After block 24 alone | 24 → 36 | A's trials | readout 0, the odd places | −7.52 ± 0.18 | −3.32 ± 0.24 | +4.20 | 102, 0 | 39, 0 | 40 | 0.93 |
| After block 24 alone | 24 → 36 | A's trials | readout 1, the even places | −8.83 ± 0.24 | −3.04 ± 0.12 | +5.79 | 135, 3 | 38, 1 | 24 | 0.86 |
| After block 24 alone | 24 → 36 | B's trials | readout 0, the odd places | −7.46 ± 0.28 | −3.02 ± 0.27 | +4.44 | 118, 2 | 31, 1 | 22 | 0.90 |
| After block 24 alone | 24 → 36 | B's trials | readout 1, the even places | −5.79 ± 0.18 | −1.87 ± 0.17 | +3.92 | 114, 4 | 28, 3 | 30 | 0.93 |
| Plain on all four nights (H-47) | 24 → 36 | A's trials | readout 0, the odd places | −12.48 ± 0.20 | −8.81 ± 0.13 | +3.67 | 86, 16 | 49, 8 | 66 | 0.99 |
| Plain on all four nights (H-47) | 24 → 36 | A's trials | readout 1, the even places | −13.98 ± 0.41 | −10.20 ± 0.39 | +3.78 | 100, 9 | 57, 6 | 40 | 0.98 |
| Plain on all four nights (H-47) | 24 → 36 | B's trials | readout 0, the odd places | −11.46 ± 0.41 | −9.13 ± 0.31 | +2.33 | 81, 8 | 39, 5 | 38 | 0.99 |
| Plain on all four nights (H-47) | 24 → 36 | B's trials | readout 1, the even places | −8.36 ± 0.16 | −5.33 ± 0.15 | +3.03 | 73, 20 | 36, 14 | 51 | 0.98 |

From right after its replayed night to block 36 each single night's readouts' differences came back toward the night without replay by 2.91
to 6.57 spikes a trial, and H-47's plain night's came back by 2.33 to 3.78 from block 24. Under each single night, of the units moved in the
readout's direction right after, 21 to 51 still lay beyond 2 SE the same way at block 36, and 20 to 65 units moved at block 36 that had not
moved either way right after. The units' differences at block 36 correlate with theirs right after by +0.41 to +0.93, the least on B's
trials' readout 0 after block 6 alone and the most on B's trials' readout 1 after block 24 alone; under H-47's plain night by +0.98 to
+0.99.

**Each single night's fewest units right after its replayed night**, unit by unit, beside the same unit at block 36:

| Form | Trials | Readout | Unit | Place | Class | Less without replay right after its replayed night | At block 36 | Change |
| :--- | :--- | :--- | :--- | :--- | :--- | ---: | ---: | ---: |
| After block 6 alone | A's trials | readout 0, the odd places | 7 | 7 | readout 0's pattern targets | −0.55 ± 0.03 | +0.05 ± 0.01 | +0.60 |
| After block 6 alone | A's trials | readout 0, the odd places | 5 | 5 | readout 0's pattern targets | −0.48 ± 0.03 | −0.02 ± 0.02 | +0.46 |
| After block 6 alone | A's trials | readout 0, the odd places | 41 | 1 | readout 0's pattern targets | −0.44 ± 0.03 | −0.45 ± 0.04 | −0.01 |
| After block 6 alone | A's trials | readout 0, the odd places | 35 | 15 | readout 0's pattern targets | −0.34 ± 0.02 | −0.31 ± 0.02 | +0.03 |
| After block 6 alone | A's trials | readout 0, the odd places | 33 | 13 | readout 0's pattern targets | −0.34 ± 0.03 | −0.37 ± 0.02 | −0.03 |
| After block 6 alone | A's trials | readout 0, the odd places | 315 | 15 | readout 0's pattern targets | −0.29 ± 0.02 | −0.05 ± 0.07 | +0.24 |
| After block 6 alone | A's trials | readout 0, the odd places | 647 | 7 | readout 0's pattern targets | −0.29 ± 0.04 | 0.00 ± 0.01 | +0.28 |
| After block 6 alone | A's trials | readout 0, the odd places | 419 | 19 | inhibitory in readout 0 | −0.28 ± 0.02 | 0.00 ± 0.01 | +0.28 |
| After block 6 alone | A's trials | readout 0, the odd places | 649 | 9 | inhibitory in readout 0 | −0.27 ± 0.03 | −0.12 ± 0.03 | +0.15 |
| After block 6 alone | A's trials | readout 0, the odd places | 399 | 19 | inhibitory in readout 0 | −0.24 ± 0.03 | +0.03 ± 0.04 | +0.27 |
| After block 6 alone | A's trials | readout 0, the odd places | 455 | 15 | readout 0's pattern targets | −0.23 ± 0.02 | 0.00 ± 0.01 | +0.23 |
| After block 6 alone | A's trials | readout 0, the odd places | 415 | 15 | readout 0's pattern targets | −0.22 ± 0.02 | +0.01 ± 0.01 | +0.23 |
| After block 6 alone | A's trials | readout 0, the odd places | 759 | 19 | inhibitory in readout 0 | −0.20 ± 0.03 | −0.25 ± 0.05 | −0.05 |
| After block 6 alone | A's trials | readout 1, the even places | 424 | 4 | inhibitory in readout 1 | −0.40 ± 0.03 | −0.37 ± 0.02 | +0.03 |
| After block 6 alone | A's trials | readout 1, the even places | 72 | 12 | readout 1's other excitatory | −0.39 ± 0.03 | 0.00 ± 0.01 | +0.39 |
| After block 6 alone | A's trials | readout 1, the even places | 648 | 8 | readout 1's pattern targets | −0.30 ± 0.03 | −0.10 ± 0.02 | +0.20 |
| After block 6 alone | A's trials | readout 1, the even places | 466 | 6 | readout 1's pattern targets | −0.29 ± 0.02 | −0.32 ± 0.02 | −0.03 |
| After block 6 alone | A's trials | readout 1, the even places | 92 | 12 | readout 1's pattern targets | −0.25 ± 0.03 | −0.14 ± 0.02 | +0.11 |
| After block 6 alone | A's trials | readout 1, the even places | 974 | 14 | inhibitory in readout 1 | −0.24 ± 0.02 | −0.11 ± 0.01 | +0.13 |
| After block 6 alone | A's trials | readout 1, the even places | 702 | 2 | readout 1's other excitatory | −0.23 ± 0.03 | +0.07 ± 0.02 | +0.30 |
| After block 6 alone | A's trials | readout 1, the even places | 772 | 12 | readout 1's pattern targets | −0.22 ± 0.03 | −0.14 ± 0.01 | +0.08 |
| After block 6 alone | A's trials | readout 1, the even places | 448 | 8 | readout 1's pattern targets | −0.22 ± 0.02 | +0.06 ± 0.02 | +0.28 |
| After block 6 alone | A's trials | readout 1, the even places | 204 | 4 | inhibitory in readout 1 | −0.19 ± 0.03 | 0.00 ± 0.01 | +0.19 |
| After block 6 alone | A's trials | readout 1, the even places | 254 | 14 | inhibitory in readout 1 | −0.18 ± 0.02 | −0.02 ± 0.01 | +0.17 |
| After block 6 alone | A's trials | readout 1, the even places | 864 | 4 | inhibitory in readout 1 | −0.18 ± 0.02 | −0.12 ± 0.01 | +0.07 |
| After block 6 alone | A's trials | readout 1, the even places | 862 | 2 | readout 1's other excitatory | −0.18 ± 0.02 | −0.17 ± 0.02 | +0.01 |
| After block 6 alone | A's trials | readout 1, the even places | 4 | 4 | inhibitory in readout 1 | −0.17 ± 0.02 | +0.04 ± 0.01 | +0.21 |
| After block 6 alone | A's trials | readout 1, the even places | 858 | 18 | readout 1's pattern targets | −0.16 ± 0.02 | −0.04 ± 0.01 | +0.12 |
| After block 6 alone | A's trials | readout 1, the even places | 904 | 4 | inhibitory in readout 1 | −0.14 ± 0.02 | −0.12 ± 0.01 | +0.02 |
| After block 6 alone | B's trials | readout 0, the odd places | 45 | 5 | readout 0's pattern targets | −0.27 ± 0.03 | −0.14 ± 0.02 | +0.13 |
| After block 6 alone | B's trials | readout 0, the odd places | 115 | 15 | readout 0's pattern targets | −0.26 ± 0.02 | −0.36 ± 0.03 | −0.10 |
| After block 6 alone | B's trials | readout 0, the odd places | 801 | 1 | readout 0's pattern targets | −0.22 ± 0.02 | +0.03 ± 0.01 | +0.25 |
| After block 6 alone | B's trials | readout 0, the odd places | 907 | 7 | readout 0's pattern targets | −0.22 ± 0.02 | −0.22 ± 0.01 | 0.00 |
| After block 6 alone | B's trials | readout 0, the odd places | 229 | 9 | inhibitory in readout 0 | −0.20 ± 0.02 | +0.01 ± 0.01 | +0.21 |
| After block 6 alone | B's trials | readout 0, the odd places | 43 | 3 | readout 0's pattern targets | −0.19 ± 0.02 | −0.01 ± 0.01 | +0.17 |
| After block 6 alone | B's trials | readout 0, the odd places | 137 | 17 | readout 0's pattern targets | −0.17 ± 0.03 | −0.01 ± 0.00 | +0.17 |
| After block 6 alone | B's trials | readout 0, the odd places | 75 | 15 | readout 0's pattern targets | −0.16 ± 0.02 | −0.19 ± 0.01 | −0.02 |
| After block 6 alone | B's trials | readout 0, the odd places | 653 | 13 | readout 0's pattern targets | −0.13 ± 0.11 | +0.01 ± 0.00 | +0.14 |
| After block 6 alone | B's trials | readout 0, the odd places | 569 | 9 | inhibitory in readout 0 | −0.13 ± 0.02 | +0.06 ± 0.01 | +0.19 |
| After block 6 alone | B's trials | readout 0, the odd places | 939 | 19 | inhibitory in readout 0 | −0.13 ± 0.04 | 0.00 ± 0.02 | +0.13 |
| After block 6 alone | B's trials | readout 0, the odd places | 659 | 19 | inhibitory in readout 0 | −0.12 ± 0.02 | −0.27 ± 0.02 | −0.15 |
| After block 6 alone | B's trials | readout 0, the odd places | 289 | 9 | inhibitory in readout 0 | −0.12 ± 0.04 | +0.03 ± 0.02 | +0.14 |
| After block 6 alone | B's trials | readout 0, the odd places | 649 | 9 | inhibitory in readout 0 | −0.12 ± 0.02 | +0.01 ± 0.02 | +0.13 |
| After block 6 alone | B's trials | readout 0, the odd places | 145 | 5 | C's own | −0.11 ± 0.02 | −0.08 ± 0.01 | +0.04 |
| After block 6 alone | B's trials | readout 0, the odd places | 639 | 19 | inhibitory in readout 0 | −0.11 ± 0.02 | −0.01 ± 0.03 | +0.10 |
| After block 6 alone | B's trials | readout 0, the odd places | 913 | 13 | readout 0's pattern targets | −0.11 ± 0.02 | −0.09 ± 0.01 | +0.02 |
| After block 6 alone | B's trials | readout 0, the odd places | 5 | 5 | readout 0's pattern targets | −0.10 ± 0.01 | −0.31 ± 0.04 | −0.21 |
| After block 6 alone | B's trials | readout 1, the even places | 566 | 6 | readout 1's pattern targets | −0.45 ± 0.04 | −0.22 ± 0.07 | +0.23 |
| After block 6 alone | B's trials | readout 1, the even places | 564 | 4 | inhibitory in readout 1 | −0.35 ± 0.03 | −0.02 ± 0.01 | +0.33 |
| After block 6 alone | B's trials | readout 1, the even places | 6 | 6 | readout 1's pattern targets | −0.34 ± 0.02 | −0.18 ± 0.03 | +0.16 |
| After block 6 alone | B's trials | readout 1, the even places | 794 | 14 | inhibitory in readout 1 | −0.27 ± 0.03 | −0.16 ± 0.01 | +0.11 |
| After block 6 alone | B's trials | readout 1, the even places | 658 | 18 | readout 1's pattern targets | −0.23 ± 0.02 | −0.08 ± 0.03 | +0.15 |
| After block 6 alone | B's trials | readout 1, the even places | 16 | 16 | readout 1's pattern targets | −0.19 ± 0.02 | 0.00 ± 0.01 | +0.19 |
| After block 6 alone | B's trials | readout 1, the even places | 134 | 14 | inhibitory in readout 1 | −0.18 ± 0.02 | −0.01 ± 0.01 | +0.17 |
| After block 6 alone | B's trials | readout 1, the even places | 494 | 14 | inhibitory in readout 1 | −0.18 ± 0.02 | −0.10 ± 0.03 | +0.08 |
| After block 6 alone | B's trials | readout 1, the even places | 118 | 18 | readout 1's pattern targets | −0.15 ± 0.02 | −0.18 ± 0.01 | −0.03 |
| After block 6 alone | B's trials | readout 1, the even places | 192 | 12 | readout 1's pattern targets | −0.15 ± 0.03 | +0.01 ± 0.01 | +0.16 |
| After block 6 alone | B's trials | readout 1, the even places | 268 | 8 | readout 1's pattern targets | −0.15 ± 0.03 | −0.04 ± 0.01 | +0.10 |
| After block 6 alone | B's trials | readout 1, the even places | 974 | 14 | inhibitory in readout 1 | −0.13 ± 0.03 | −0.02 ± 0.07 | +0.10 |
| After block 6 alone | B's trials | readout 1, the even places | 918 | 18 | readout 1's pattern targets | −0.11 ± 0.01 | −0.09 ± 0.01 | +0.02 |
| After block 6 alone | B's trials | readout 1, the even places | 574 | 14 | inhibitory in readout 1 | −0.10 ± 0.02 | −0.08 ± 0.01 | +0.02 |
| After block 12 alone | A's trials | readout 0, the odd places | 5 | 5 | readout 0's pattern targets | −0.47 ± 0.03 | +0.03 ± 0.01 | +0.50 |
| After block 12 alone | A's trials | readout 0, the odd places | 683 | 3 | readout 0's pattern targets | −0.41 ± 0.04 | −0.21 ± 0.05 | +0.21 |
| After block 12 alone | A's trials | readout 0, the odd places | 649 | 9 | inhibitory in readout 0 | −0.38 ± 0.02 | −0.18 ± 0.02 | +0.19 |
| After block 12 alone | A's trials | readout 0, the odd places | 419 | 19 | inhibitory in readout 0 | −0.36 ± 0.03 | 0.00 ± 0.00 | +0.36 |
| After block 12 alone | A's trials | readout 0, the odd places | 33 | 13 | readout 0's pattern targets | −0.34 ± 0.02 | −0.29 ± 0.03 | +0.05 |
| After block 12 alone | A's trials | readout 0, the odd places | 35 | 15 | readout 0's pattern targets | −0.34 ± 0.02 | −0.24 ± 0.04 | +0.10 |
| After block 12 alone | A's trials | readout 0, the odd places | 647 | 7 | readout 0's pattern targets | −0.30 ± 0.03 | +0.01 ± 0.01 | +0.31 |
| After block 12 alone | A's trials | readout 0, the odd places | 41 | 1 | readout 0's pattern targets | −0.29 ± 0.02 | −0.22 ± 0.03 | +0.07 |
| After block 12 alone | A's trials | readout 0, the odd places | 465 | 5 | C's own | −0.26 ± 0.02 | −0.27 ± 0.02 | −0.02 |
| After block 12 alone | A's trials | readout 0, the odd places | 315 | 15 | readout 0's pattern targets | −0.22 ± 0.03 | −0.09 ± 0.02 | +0.13 |
| After block 12 alone | A's trials | readout 0, the odd places | 455 | 15 | readout 0's pattern targets | −0.22 ± 0.02 | −0.02 ± 0.01 | +0.20 |
| After block 12 alone | A's trials | readout 0, the odd places | 399 | 19 | inhibitory in readout 0 | −0.21 ± 0.02 | +0.01 ± 0.03 | +0.21 |
| After block 12 alone | A's trials | readout 0, the odd places | 415 | 15 | readout 0's pattern targets | −0.20 ± 0.02 | +0.01 ± 0.00 | +0.21 |
| After block 12 alone | A's trials | readout 0, the odd places | 273 | 13 | readout 0's pattern targets | −0.19 ± 0.02 | −0.18 ± 0.05 | +0.01 |
| After block 12 alone | A's trials | readout 0, the odd places | 861 | 1 | readout 0's pattern targets | −0.19 ± 0.02 | −0.08 ± 0.01 | +0.11 |
| After block 12 alone | A's trials | readout 1, the even places | 424 | 4 | inhibitory in readout 1 | −0.34 ± 0.02 | −0.23 ± 0.01 | +0.11 |
| After block 12 alone | A's trials | readout 1, the even places | 254 | 14 | inhibitory in readout 1 | −0.34 ± 0.03 | −0.02 ± 0.00 | +0.32 |
| After block 12 alone | A's trials | readout 1, the even places | 772 | 12 | readout 1's pattern targets | −0.32 ± 0.02 | −0.24 ± 0.01 | +0.08 |
| After block 12 alone | A's trials | readout 1, the even places | 904 | 4 | inhibitory in readout 1 | −0.31 ± 0.01 | −0.33 ± 0.02 | −0.02 |
| After block 12 alone | A's trials | readout 1, the even places | 648 | 8 | readout 1's pattern targets | −0.31 ± 0.02 | −0.12 ± 0.02 | +0.18 |
| After block 12 alone | A's trials | readout 1, the even places | 466 | 6 | readout 1's pattern targets | −0.27 ± 0.02 | −0.24 ± 0.02 | +0.03 |
| After block 12 alone | A's trials | readout 1, the even places | 892 | 12 | readout 1's pattern targets | −0.26 ± 0.02 | −0.09 ± 0.02 | +0.17 |
| After block 12 alone | A's trials | readout 1, the even places | 774 | 14 | inhibitory in readout 1 | −0.26 ± 0.03 | −0.24 ± 0.02 | +0.02 |
| After block 12 alone | A's trials | readout 1, the even places | 974 | 14 | inhibitory in readout 1 | −0.25 ± 0.04 | −0.08 ± 0.03 | +0.17 |
| After block 12 alone | A's trials | readout 1, the even places | 702 | 2 | readout 1's other excitatory | −0.25 ± 0.03 | +0.04 ± 0.01 | +0.29 |
| After block 12 alone | A's trials | readout 1, the even places | 862 | 2 | readout 1's other excitatory | −0.24 ± 0.02 | −0.20 ± 0.02 | +0.05 |
| After block 12 alone | A's trials | readout 1, the even places | 902 | 2 | readout 1's pattern targets | −0.23 ± 0.03 | −0.16 ± 0.02 | +0.06 |
| After block 12 alone | A's trials | readout 1, the even places | 864 | 4 | inhibitory in readout 1 | −0.21 ± 0.02 | −0.11 ± 0.02 | +0.10 |
| After block 12 alone | A's trials | readout 1, the even places | 72 | 12 | readout 1's other excitatory | −0.21 ± 0.03 | +0.01 ± 0.00 | +0.21 |
| After block 12 alone | A's trials | readout 1, the even places | 204 | 4 | inhibitory in readout 1 | −0.20 ± 0.02 | 0.00 ± 0.01 | +0.20 |
| After block 12 alone | A's trials | readout 1, the even places | 448 | 8 | readout 1's pattern targets | −0.17 ± 0.03 | +0.07 ± 0.02 | +0.25 |
| After block 12 alone | A's trials | readout 1, the even places | 854 | 14 | inhibitory in readout 1 | −0.17 ± 0.02 | −0.18 ± 0.02 | −0.02 |
| After block 12 alone | A's trials | readout 1, the even places | 92 | 12 | readout 1's pattern targets | −0.16 ± 0.02 | −0.12 ± 0.01 | +0.04 |
| After block 12 alone | A's trials | readout 1, the even places | 858 | 18 | readout 1's pattern targets | −0.16 ± 0.02 | −0.08 ± 0.02 | +0.08 |
| After block 12 alone | B's trials | readout 0, the odd places | 229 | 9 | inhibitory in readout 0 | −0.31 ± 0.03 | 0.00 ± 0.02 | +0.31 |
| After block 12 alone | B's trials | readout 0, the odd places | 801 | 1 | readout 0's pattern targets | −0.30 ± 0.02 | +0.01 ± 0.01 | +0.31 |
| After block 12 alone | B's trials | readout 0, the odd places | 907 | 7 | readout 0's pattern targets | −0.28 ± 0.03 | −0.05 ± 0.01 | +0.24 |
| After block 12 alone | B's trials | readout 0, the odd places | 45 | 5 | readout 0's pattern targets | −0.24 ± 0.03 | −0.08 ± 0.03 | +0.16 |
| After block 12 alone | B's trials | readout 0, the odd places | 115 | 15 | readout 0's pattern targets | −0.23 ± 0.01 | −0.23 ± 0.02 | 0.00 |
| After block 12 alone | B's trials | readout 0, the odd places | 75 | 15 | readout 0's pattern targets | −0.21 ± 0.02 | −0.23 ± 0.01 | −0.02 |
| After block 12 alone | B's trials | readout 0, the odd places | 137 | 17 | readout 0's pattern targets | −0.20 ± 0.02 | −0.01 ± 0.00 | +0.19 |
| After block 12 alone | B's trials | readout 0, the odd places | 659 | 19 | inhibitory in readout 0 | −0.18 ± 0.02 | −0.11 ± 0.02 | +0.06 |
| After block 12 alone | B's trials | readout 0, the odd places | 47 | 7 | readout 0's pattern targets | −0.15 ± 0.02 | −0.52 ± 0.03 | −0.37 |
| After block 12 alone | B's trials | readout 0, the odd places | 5 | 5 | readout 0's pattern targets | −0.14 ± 0.01 | −0.17 ± 0.02 | −0.03 |
| After block 12 alone | B's trials | readout 0, the odd places | 289 | 9 | inhibitory in readout 0 | −0.11 ± 0.02 | +0.02 ± 0.01 | +0.13 |
| After block 12 alone | B's trials | readout 0, the odd places | 43 | 3 | readout 0's pattern targets | −0.11 ± 0.02 | 0.00 ± 0.00 | +0.11 |
| After block 12 alone | B's trials | readout 0, the odd places | 913 | 13 | readout 0's pattern targets | −0.10 ± 0.02 | −0.05 ± 0.01 | +0.05 |
| After block 12 alone | B's trials | readout 0, the odd places | 649 | 9 | inhibitory in readout 0 | −0.10 ± 0.01 | +0.01 ± 0.00 | +0.11 |
| After block 12 alone | B's trials | readout 0, the odd places | 49 | 9 | inhibitory in readout 0 | −0.10 ± 0.02 | −0.31 ± 0.05 | −0.21 |
| After block 12 alone | B's trials | readout 0, the odd places | 179 | 19 | inhibitory in readout 0 | −0.10 ± 0.02 | +0.01 ± 0.01 | +0.11 |
| After block 12 alone | B's trials | readout 0, the odd places | 1009 | 9 | inhibitory in readout 0 | −0.09 ± 0.01 | −0.03 ± 0.00 | +0.06 |
| After block 12 alone | B's trials | readout 0, the odd places | 653 | 13 | readout 0's pattern targets | −0.09 ± 0.02 | 0.00 ± 0.01 | +0.08 |
| After block 12 alone | B's trials | readout 0, the odd places | 639 | 19 | inhibitory in readout 0 | −0.08 ± 0.02 | −0.02 ± 0.01 | +0.06 |
| After block 12 alone | B's trials | readout 1, the even places | 6 | 6 | readout 1's pattern targets | −0.45 ± 0.04 | −0.03 ± 0.03 | +0.42 |
| After block 12 alone | B's trials | readout 1, the even places | 658 | 18 | readout 1's pattern targets | −0.34 ± 0.02 | +0.01 ± 0.02 | +0.35 |
| After block 12 alone | B's trials | readout 1, the even places | 494 | 14 | inhibitory in readout 1 | −0.30 ± 0.03 | −0.16 ± 0.02 | +0.14 |
| After block 12 alone | B's trials | readout 1, the even places | 974 | 14 | inhibitory in readout 1 | −0.27 ± 0.07 | −0.03 ± 0.05 | +0.23 |
| After block 12 alone | B's trials | readout 1, the even places | 268 | 8 | readout 1's pattern targets | −0.26 ± 0.02 | −0.13 ± 0.01 | +0.14 |
| After block 12 alone | B's trials | readout 1, the even places | 134 | 14 | inhibitory in readout 1 | −0.21 ± 0.02 | −0.02 ± 0.02 | +0.20 |
| After block 12 alone | B's trials | readout 1, the even places | 794 | 14 | inhibitory in readout 1 | −0.21 ± 0.02 | −0.14 ± 0.01 | +0.07 |
| After block 12 alone | B's trials | readout 1, the even places | 566 | 6 | readout 1's pattern targets | −0.19 ± 0.03 | +0.02 ± 0.10 | +0.21 |
| After block 12 alone | B's trials | readout 1, the even places | 574 | 14 | inhibitory in readout 1 | −0.17 ± 0.03 | −0.10 ± 0.02 | +0.08 |
| After block 12 alone | B's trials | readout 1, the even places | 118 | 18 | readout 1's pattern targets | −0.16 ± 0.02 | −0.19 ± 0.01 | −0.03 |
| After block 12 alone | B's trials | readout 1, the even places | 564 | 4 | inhibitory in readout 1 | −0.15 ± 0.02 | +0.01 ± 0.00 | +0.17 |
| After block 12 alone | B's trials | readout 1, the even places | 22 | 2 | readout 1's other excitatory | −0.13 ± 0.02 | −0.04 ± 0.02 | +0.09 |
| After block 12 alone | B's trials | readout 1, the even places | 918 | 18 | readout 1's pattern targets | −0.10 ± 0.01 | −0.06 ± 0.01 | +0.04 |
| After block 12 alone | B's trials | readout 1, the even places | 164 | 4 | inhibitory in readout 1 | −0.09 ± 0.02 | 0.00 ± 0.01 | +0.09 |
| After block 12 alone | B's trials | readout 1, the even places | 454 | 14 | inhibitory in readout 1 | −0.08 ± 0.02 | −0.02 ± 0.00 | +0.07 |
| After block 18 alone | A's trials | readout 0, the odd places | 465 | 5 | C's own | −0.35 ± 0.02 | −0.27 ± 0.02 | +0.09 |
| After block 18 alone | A's trials | readout 0, the odd places | 557 | 17 | readout 0's pattern targets | −0.29 ± 0.03 | −0.19 ± 0.06 | +0.11 |
| After block 18 alone | A's trials | readout 0, the odd places | 649 | 9 | inhibitory in readout 0 | −0.29 ± 0.02 | −0.17 ± 0.02 | +0.12 |
| After block 18 alone | A's trials | readout 0, the odd places | 33 | 13 | readout 0's pattern targets | −0.29 ± 0.04 | −0.14 ± 0.01 | +0.15 |
| After block 18 alone | A's trials | readout 0, the odd places | 35 | 15 | readout 0's pattern targets | −0.25 ± 0.04 | −0.10 ± 0.02 | +0.15 |
| After block 18 alone | A's trials | readout 0, the odd places | 897 | 17 | readout 0's pattern targets | −0.24 ± 0.02 | −0.11 ± 0.01 | +0.13 |
| After block 18 alone | A's trials | readout 0, the odd places | 759 | 19 | inhibitory in readout 0 | −0.23 ± 0.03 | −0.03 ± 0.03 | +0.20 |
| After block 18 alone | A's trials | readout 0, the odd places | 419 | 19 | inhibitory in readout 0 | −0.22 ± 0.02 | −0.12 ± 0.01 | +0.11 |
| After block 18 alone | A's trials | readout 0, the odd places | 415 | 15 | readout 0's pattern targets | −0.22 ± 0.02 | −0.11 ± 0.01 | +0.10 |
| After block 18 alone | A's trials | readout 0, the odd places | 273 | 13 | readout 0's pattern targets | −0.22 ± 0.02 | −0.18 ± 0.03 | +0.03 |
| After block 18 alone | A's trials | readout 0, the odd places | 889 | 9 | inhibitory in readout 0 | −0.20 ± 0.02 | −0.10 ± 0.01 | +0.10 |
| After block 18 alone | A's trials | readout 0, the odd places | 41 | 1 | readout 0's pattern targets | −0.15 ± 0.02 | −0.15 ± 0.05 | 0.00 |
| After block 18 alone | A's trials | readout 0, the odd places | 861 | 1 | readout 0's pattern targets | −0.15 ± 0.02 | −0.04 ± 0.02 | +0.11 |
| After block 18 alone | A's trials | readout 0, the odd places | 455 | 15 | readout 0's pattern targets | −0.15 ± 0.02 | −0.02 ± 0.01 | +0.13 |
| After block 18 alone | A's trials | readout 0, the odd places | 339 | 19 | inhibitory in readout 0 | −0.14 ± 0.02 | −0.10 ± 0.01 | +0.05 |
| After block 18 alone | A's trials | readout 0, the odd places | 559 | 19 | inhibitory in readout 0 | −0.14 ± 0.02 | −0.02 ± 0.04 | +0.12 |
| After block 18 alone | A's trials | readout 0, the odd places | 337 | 17 | readout 0's pattern targets | −0.14 ± 0.02 | −0.09 ± 0.01 | +0.05 |
| After block 18 alone | A's trials | readout 1, the even places | 72 | 12 | readout 1's other excitatory | −0.34 ± 0.02 | −0.02 ± 0.00 | +0.32 |
| After block 18 alone | A's trials | readout 1, the even places | 774 | 14 | inhibitory in readout 1 | −0.34 ± 0.03 | −0.25 ± 0.02 | +0.10 |
| After block 18 alone | A's trials | readout 1, the even places | 466 | 6 | readout 1's pattern targets | −0.34 ± 0.03 | −0.28 ± 0.02 | +0.06 |
| After block 18 alone | A's trials | readout 1, the even places | 424 | 4 | inhibitory in readout 1 | −0.34 ± 0.02 | −0.24 ± 0.02 | +0.10 |
| After block 18 alone | A's trials | readout 1, the even places | 648 | 8 | readout 1's pattern targets | −0.32 ± 0.02 | −0.14 ± 0.02 | +0.18 |
| After block 18 alone | A's trials | readout 1, the even places | 892 | 12 | readout 1's pattern targets | −0.31 ± 0.02 | −0.14 ± 0.02 | +0.17 |
| After block 18 alone | A's trials | readout 1, the even places | 904 | 4 | inhibitory in readout 1 | −0.31 ± 0.02 | −0.17 ± 0.01 | +0.14 |
| After block 18 alone | A's trials | readout 1, the even places | 448 | 8 | readout 1's pattern targets | −0.26 ± 0.02 | +0.06 ± 0.04 | +0.32 |
| After block 18 alone | A's trials | readout 1, the even places | 894 | 14 | inhibitory in readout 1 | −0.25 ± 0.03 | −0.21 ± 0.03 | +0.04 |
| After block 18 alone | A's trials | readout 1, the even places | 772 | 12 | readout 1's pattern targets | −0.24 ± 0.02 | −0.17 ± 0.01 | +0.08 |
| After block 18 alone | A's trials | readout 1, the even places | 974 | 14 | inhibitory in readout 1 | −0.23 ± 0.02 | −0.11 ± 0.02 | +0.12 |
| After block 18 alone | A's trials | readout 1, the even places | 862 | 2 | readout 1's other excitatory | −0.22 ± 0.02 | −0.19 ± 0.02 | +0.03 |
| After block 18 alone | A's trials | readout 1, the even places | 552 | 12 | readout 1's pattern targets | −0.21 ± 0.04 | −0.33 ± 0.03 | −0.11 |
| After block 18 alone | A's trials | readout 1, the even places | 854 | 14 | inhibitory in readout 1 | −0.21 ± 0.02 | −0.19 ± 0.02 | +0.01 |
| After block 18 alone | A's trials | readout 1, the even places | 254 | 14 | inhibitory in readout 1 | −0.20 ± 0.02 | −0.02 ± 0.01 | +0.18 |
| After block 18 alone | A's trials | readout 1, the even places | 598 | 18 | readout 1's pattern targets | −0.20 ± 0.05 | −0.07 ± 0.09 | +0.12 |
| After block 18 alone | A's trials | readout 1, the even places | 204 | 4 | inhibitory in readout 1 | −0.20 ± 0.02 | −0.01 ± 0.01 | +0.18 |
| After block 18 alone | A's trials | readout 1, the even places | 864 | 4 | inhibitory in readout 1 | −0.19 ± 0.02 | −0.11 ± 0.02 | +0.08 |
| After block 18 alone | A's trials | readout 1, the even places | 690 | 10 | readout 1's pattern targets | −0.18 ± 0.02 | −0.02 ± 0.03 | +0.16 |
| After block 18 alone | B's trials | readout 0, the odd places | 659 | 19 | inhibitory in readout 0 | −0.35 ± 0.02 | −0.22 ± 0.02 | +0.13 |
| After block 18 alone | B's trials | readout 0, the odd places | 47 | 7 | readout 0's pattern targets | −0.32 ± 0.05 | −0.22 ± 0.05 | +0.09 |
| After block 18 alone | B's trials | readout 0, the odd places | 907 | 7 | readout 0's pattern targets | −0.31 ± 0.02 | −0.17 ± 0.01 | +0.14 |
| After block 18 alone | B's trials | readout 0, the odd places | 45 | 5 | readout 0's pattern targets | −0.22 ± 0.02 | −0.11 ± 0.02 | +0.11 |
| After block 18 alone | B's trials | readout 0, the odd places | 5 | 5 | readout 0's pattern targets | −0.21 ± 0.02 | −0.09 ± 0.01 | +0.12 |
| After block 18 alone | B's trials | readout 0, the odd places | 115 | 15 | readout 0's pattern targets | −0.21 ± 0.02 | −0.19 ± 0.01 | +0.02 |
| After block 18 alone | B's trials | readout 0, the odd places | 229 | 9 | inhibitory in readout 0 | −0.20 ± 0.02 | −0.01 ± 0.01 | +0.19 |
| After block 18 alone | B's trials | readout 0, the odd places | 505 | 5 | C's own | −0.18 ± 0.02 | −0.13 ± 0.01 | +0.05 |
| After block 18 alone | B's trials | readout 0, the odd places | 49 | 9 | inhibitory in readout 0 | −0.17 ± 0.03 | −0.36 ± 0.05 | −0.19 |
| After block 18 alone | B's trials | readout 0, the odd places | 43 | 3 | readout 0's pattern targets | −0.14 ± 0.01 | −0.02 ± 0.01 | +0.12 |
| After block 18 alone | B's trials | readout 0, the odd places | 801 | 1 | readout 0's pattern targets | −0.14 ± 0.03 | +0.03 ± 0.00 | +0.17 |
| After block 18 alone | B's trials | readout 0, the odd places | 75 | 15 | readout 0's pattern targets | −0.14 ± 0.01 | −0.09 ± 0.01 | +0.04 |
| After block 18 alone | B's trials | readout 0, the odd places | 639 | 19 | inhibitory in readout 0 | −0.13 ± 0.01 | −0.20 ± 0.02 | −0.07 |
| After block 18 alone | B's trials | readout 0, the odd places | 193 | 13 | readout 0's pattern targets | −0.13 ± 0.02 | −0.12 ± 0.03 | 0.00 |
| After block 18 alone | B's trials | readout 0, the odd places | 137 | 17 | readout 0's pattern targets | −0.12 ± 0.02 | 0.00 ± 0.01 | +0.12 |
| After block 18 alone | B's trials | readout 0, the odd places | 179 | 19 | inhibitory in readout 0 | −0.12 ± 0.02 | −0.01 ± 0.02 | +0.11 |
| After block 18 alone | B's trials | readout 0, the odd places | 1009 | 9 | inhibitory in readout 0 | −0.11 ± 0.02 | −0.08 ± 0.02 | +0.03 |
| After block 18 alone | B's trials | readout 0, the odd places | 649 | 9 | inhibitory in readout 0 | −0.10 ± 0.01 | −0.01 ± 0.02 | +0.09 |
| After block 18 alone | B's trials | readout 0, the odd places | 913 | 13 | readout 0's pattern targets | −0.09 ± 0.02 | −0.05 ± 0.01 | +0.04 |
| After block 18 alone | B's trials | readout 0, the odd places | 273 | 13 | readout 0's pattern targets | −0.09 ± 0.02 | −0.04 ± 0.04 | +0.06 |
| After block 18 alone | B's trials | readout 1, the even places | 658 | 18 | readout 1's pattern targets | −0.24 ± 0.03 | 0.00 ± 0.00 | +0.23 |
| After block 18 alone | B's trials | readout 1, the even places | 566 | 6 | readout 1's pattern targets | −0.22 ± 0.03 | +0.03 ± 0.09 | +0.25 |
| After block 18 alone | B's trials | readout 1, the even places | 794 | 14 | inhibitory in readout 1 | −0.21 ± 0.02 | −0.07 ± 0.02 | +0.13 |
| After block 18 alone | B's trials | readout 1, the even places | 6 | 6 | readout 1's pattern targets | −0.19 ± 0.02 | +0.03 ± 0.01 | +0.23 |
| After block 18 alone | B's trials | readout 1, the even places | 192 | 12 | readout 1's pattern targets | −0.19 ± 0.01 | −0.08 ± 0.02 | +0.10 |
| After block 18 alone | B's trials | readout 1, the even places | 494 | 14 | inhibitory in readout 1 | −0.18 ± 0.02 | −0.09 ± 0.01 | +0.08 |
| After block 18 alone | B's trials | readout 1, the even places | 134 | 14 | inhibitory in readout 1 | −0.14 ± 0.02 | −0.02 ± 0.01 | +0.12 |
| After block 18 alone | B's trials | readout 1, the even places | 118 | 18 | readout 1's pattern targets | −0.12 ± 0.01 | −0.08 ± 0.00 | +0.05 |
| After block 18 alone | B's trials | readout 1, the even places | 790 | 10 | readout 1's pattern targets | −0.12 ± 0.02 | +0.02 ± 0.01 | +0.14 |
| After block 18 alone | B's trials | readout 1, the even places | 792 | 12 | readout 1's pattern targets | −0.12 ± 0.02 | −0.01 ± 0.01 | +0.11 |
| After block 18 alone | B's trials | readout 1, the even places | 1018 | 18 | readout 1's pattern targets | −0.11 ± 0.01 | −0.11 ± 0.01 | 0.00 |
| After block 18 alone | B's trials | readout 1, the even places | 268 | 8 | readout 1's pattern targets | −0.10 ± 0.02 | −0.04 ± 0.02 | +0.06 |
| After block 18 alone | B's trials | readout 1, the even places | 164 | 4 | inhibitory in readout 1 | −0.09 ± 0.01 | −0.01 ± 0.02 | +0.08 |
| After block 18 alone | B's trials | readout 1, the even places | 22 | 2 | readout 1's other excitatory | −0.09 ± 0.03 | −0.02 ± 0.03 | +0.06 |
| After block 18 alone | B's trials | readout 1, the even places | 498 | 18 | readout 1's other excitatory | −0.09 ± 0.01 | −0.17 ± 0.01 | −0.09 |
| After block 18 alone | B's trials | readout 1, the even places | 918 | 18 | readout 1's pattern targets | −0.08 ± 0.02 | −0.04 ± 0.01 | +0.04 |
| After block 18 alone | B's trials | readout 1, the even places | 574 | 14 | inhibitory in readout 1 | −0.08 ± 0.02 | −0.03 ± 0.02 | +0.05 |
| After block 18 alone | B's trials | readout 1, the even places | 204 | 4 | inhibitory in readout 1 | −0.08 ± 0.02 | −0.05 ± 0.01 | +0.04 |
| After block 18 alone | B's trials | readout 1, the even places | 758 | 18 | readout 1's pattern targets | −0.08 ± 0.01 | −0.06 ± 0.01 | +0.02 |
| After block 18 alone | B's trials | readout 1, the even places | 108 | 8 | readout 1's pattern targets | −0.08 ± 0.01 | −0.09 ± 0.01 | −0.01 |
| After block 18 alone | B's trials | readout 1, the even places | 550 | 10 | readout 1's pattern targets | −0.08 ± 0.01 | −0.02 ± 0.01 | +0.06 |
| After block 18 alone | B's trials | readout 1, the even places | 766 | 6 | readout 1's pattern targets | −0.07 ± 0.01 | −0.02 ± 0.01 | +0.05 |
| After block 24 alone | A's trials | readout 0, the odd places | 465 | 5 | C's own | −0.29 ± 0.03 | −0.16 ± 0.01 | +0.14 |
| After block 24 alone | A's trials | readout 0, the odd places | 649 | 9 | inhibitory in readout 0 | −0.29 ± 0.02 | −0.12 ± 0.01 | +0.17 |
| After block 24 alone | A's trials | readout 0, the odd places | 33 | 13 | readout 0's pattern targets | −0.27 ± 0.03 | −0.17 ± 0.02 | +0.10 |
| After block 24 alone | A's trials | readout 0, the odd places | 339 | 19 | inhibitory in readout 0 | −0.26 ± 0.02 | −0.19 ± 0.01 | +0.07 |
| After block 24 alone | A's trials | readout 0, the odd places | 337 | 17 | readout 0's pattern targets | −0.25 ± 0.03 | −0.19 ± 0.01 | +0.06 |
| After block 24 alone | A's trials | readout 0, the odd places | 41 | 1 | readout 0's pattern targets | −0.23 ± 0.04 | −0.22 ± 0.03 | +0.02 |
| After block 24 alone | A's trials | readout 0, the odd places | 35 | 15 | readout 0's pattern targets | −0.23 ± 0.03 | −0.16 ± 0.03 | +0.07 |
| After block 24 alone | A's trials | readout 0, the odd places | 861 | 1 | readout 0's pattern targets | −0.22 ± 0.02 | −0.10 ± 0.01 | +0.12 |
| After block 24 alone | A's trials | readout 0, the odd places | 315 | 15 | readout 0's pattern targets | −0.22 ± 0.03 | −0.17 ± 0.03 | +0.05 |
| After block 24 alone | A's trials | readout 0, the odd places | 557 | 17 | readout 0's pattern targets | −0.21 ± 0.04 | −0.18 ± 0.04 | +0.03 |
| After block 24 alone | A's trials | readout 0, the odd places | 459 | 19 | inhibitory in readout 0 | −0.20 ± 0.02 | −0.06 ± 0.00 | +0.14 |
| After block 24 alone | A's trials | readout 0, the odd places | 419 | 19 | inhibitory in readout 0 | −0.20 ± 0.02 | −0.10 ± 0.01 | +0.10 |
| After block 24 alone | A's trials | readout 0, the odd places | 393 | 13 | readout 0's pattern targets | −0.18 ± 0.03 | −0.19 ± 0.03 | −0.01 |
| After block 24 alone | A's trials | readout 0, the odd places | 565 | 5 | readout 0's pattern targets | −0.18 ± 0.02 | −0.12 ± 0.01 | +0.06 |
| After block 24 alone | A's trials | readout 0, the odd places | 415 | 15 | readout 0's pattern targets | −0.18 ± 0.02 | −0.09 ± 0.00 | +0.09 |
| After block 24 alone | A's trials | readout 0, the odd places | 279 | 19 | inhibitory in readout 0 | −0.17 ± 0.05 | −0.12 ± 0.02 | +0.05 |
| After block 24 alone | A's trials | readout 0, the odd places | 889 | 9 | inhibitory in readout 0 | −0.17 ± 0.02 | −0.08 ± 0.01 | +0.09 |
| After block 24 alone | A's trials | readout 1, the even places | 774 | 14 | inhibitory in readout 1 | −0.36 ± 0.01 | −0.24 ± 0.02 | +0.12 |
| After block 24 alone | A's trials | readout 1, the even places | 904 | 4 | inhibitory in readout 1 | −0.30 ± 0.02 | −0.18 ± 0.01 | +0.12 |
| After block 24 alone | A's trials | readout 1, the even places | 72 | 12 | readout 1's other excitatory | −0.29 ± 0.03 | −0.03 ± 0.02 | +0.26 |
| After block 24 alone | A's trials | readout 1, the even places | 338 | 18 | readout 1's other excitatory | −0.29 ± 0.02 | −0.23 ± 0.02 | +0.06 |
| After block 24 alone | A's trials | readout 1, the even places | 648 | 8 | readout 1's pattern targets | −0.28 ± 0.02 | −0.10 ± 0.02 | +0.18 |
| After block 24 alone | A's trials | readout 1, the even places | 466 | 6 | readout 1's pattern targets | −0.28 ± 0.03 | −0.18 ± 0.01 | +0.10 |
| After block 24 alone | A's trials | readout 1, the even places | 772 | 12 | readout 1's pattern targets | −0.26 ± 0.02 | −0.21 ± 0.01 | +0.05 |
| After block 24 alone | A's trials | readout 1, the even places | 70 | 10 | readout 1's pattern targets | −0.25 ± 0.07 | −0.24 ± 0.08 | +0.01 |
| After block 24 alone | A's trials | readout 1, the even places | 142 | 2 | readout 1's other excitatory | −0.24 ± 0.02 | −0.19 ± 0.01 | +0.05 |
| After block 24 alone | A's trials | readout 1, the even places | 858 | 18 | readout 1's pattern targets | −0.23 ± 0.02 | −0.10 ± 0.01 | +0.13 |
| After block 24 alone | A's trials | readout 1, the even places | 892 | 12 | readout 1's pattern targets | −0.23 ± 0.02 | −0.13 ± 0.02 | +0.09 |
| After block 24 alone | A's trials | readout 1, the even places | 862 | 2 | readout 1's other excitatory | −0.22 ± 0.02 | −0.13 ± 0.02 | +0.09 |
| After block 24 alone | A's trials | readout 1, the even places | 424 | 4 | inhibitory in readout 1 | −0.22 ± 0.02 | −0.09 ± 0.01 | +0.13 |
| After block 24 alone | A's trials | readout 1, the even places | 894 | 14 | inhibitory in readout 1 | −0.20 ± 0.02 | −0.14 ± 0.02 | +0.05 |
| After block 24 alone | A's trials | readout 1, the even places | 854 | 14 | inhibitory in readout 1 | −0.19 ± 0.02 | −0.12 ± 0.01 | +0.06 |
| After block 24 alone | A's trials | readout 1, the even places | 92 | 12 | readout 1's pattern targets | −0.17 ± 0.02 | −0.12 ± 0.01 | +0.05 |
| After block 24 alone | A's trials | readout 1, the even places | 864 | 4 | inhibitory in readout 1 | −0.17 ± 0.01 | −0.09 ± 0.01 | +0.08 |
| After block 24 alone | A's trials | readout 1, the even places | 598 | 18 | readout 1's pattern targets | −0.17 ± 0.01 | −0.06 ± 0.08 | +0.11 |
| After block 24 alone | A's trials | readout 1, the even places | 204 | 4 | inhibitory in readout 1 | −0.17 ± 0.03 | 0.00 ± 0.01 | +0.17 |
| After block 24 alone | B's trials | readout 0, the odd places | 5 | 5 | readout 0's pattern targets | −0.39 ± 0.04 | −0.25 ± 0.03 | +0.14 |
| After block 24 alone | B's trials | readout 0, the odd places | 115 | 15 | readout 0's pattern targets | −0.34 ± 0.02 | −0.28 ± 0.02 | +0.06 |
| After block 24 alone | B's trials | readout 0, the odd places | 47 | 7 | readout 0's pattern targets | −0.33 ± 0.05 | −0.27 ± 0.03 | +0.06 |
| After block 24 alone | B's trials | readout 0, the odd places | 49 | 9 | inhibitory in readout 0 | −0.28 ± 0.03 | −0.24 ± 0.04 | +0.04 |
| After block 24 alone | B's trials | readout 0, the odd places | 907 | 7 | readout 0's pattern targets | −0.27 ± 0.02 | −0.23 ± 0.01 | +0.04 |
| After block 24 alone | B's trials | readout 0, the odd places | 43 | 3 | readout 0's pattern targets | −0.26 ± 0.05 | −0.09 ± 0.05 | +0.17 |
| After block 24 alone | B's trials | readout 0, the odd places | 639 | 19 | inhibitory in readout 0 | −0.26 ± 0.02 | −0.24 ± 0.02 | +0.02 |
| After block 24 alone | B's trials | readout 0, the odd places | 193 | 13 | readout 0's pattern targets | −0.23 ± 0.02 | −0.18 ± 0.02 | +0.05 |
| After block 24 alone | B's trials | readout 0, the odd places | 137 | 17 | readout 0's pattern targets | −0.23 ± 0.02 | −0.11 ± 0.04 | +0.12 |
| After block 24 alone | B's trials | readout 0, the odd places | 75 | 15 | readout 0's pattern targets | −0.23 ± 0.02 | −0.20 ± 0.01 | +0.03 |
| After block 24 alone | B's trials | readout 0, the odd places | 289 | 9 | inhibitory in readout 0 | −0.19 ± 0.02 | 0.00 ± 0.03 | +0.20 |
| After block 24 alone | B's trials | readout 0, the odd places | 273 | 13 | readout 0's pattern targets | −0.15 ± 0.02 | −0.08 ± 0.01 | +0.08 |
| After block 24 alone | B's trials | readout 0, the odd places | 659 | 19 | inhibitory in readout 0 | −0.15 ± 0.04 | −0.08 ± 0.02 | +0.08 |
| After block 24 alone | B's trials | readout 0, the odd places | 179 | 19 | inhibitory in readout 0 | −0.15 ± 0.01 | −0.09 ± 0.02 | +0.06 |
| After block 24 alone | B's trials | readout 0, the odd places | 1009 | 9 | inhibitory in readout 0 | −0.13 ± 0.02 | −0.03 ± 0.01 | +0.10 |
| After block 24 alone | B's trials | readout 0, the odd places | 969 | 9 | inhibitory in readout 0 | −0.12 ± 0.02 | −0.02 ± 0.01 | +0.10 |
| After block 24 alone | B's trials | readout 0, the odd places | 9 | 9 | inhibitory in readout 0 | −0.12 ± 0.01 | −0.10 ± 0.01 | +0.03 |
| After block 24 alone | B's trials | readout 1, the even places | 118 | 18 | readout 1's pattern targets | −0.31 ± 0.02 | −0.26 ± 0.01 | +0.06 |
| After block 24 alone | B's trials | readout 1, the even places | 6 | 6 | readout 1's pattern targets | −0.26 ± 0.03 | −0.19 ± 0.02 | +0.07 |
| After block 24 alone | B's trials | readout 1, the even places | 22 | 2 | readout 1's other excitatory | −0.25 ± 0.01 | −0.23 ± 0.01 | +0.02 |
| After block 24 alone | B's trials | readout 1, the even places | 134 | 14 | inhibitory in readout 1 | −0.24 ± 0.03 | −0.12 ± 0.04 | +0.12 |
| After block 24 alone | B's trials | readout 1, the even places | 192 | 12 | readout 1's pattern targets | −0.22 ± 0.02 | −0.13 ± 0.02 | +0.09 |
| After block 24 alone | B's trials | readout 1, the even places | 566 | 6 | readout 1's pattern targets | −0.22 ± 0.03 | −0.07 ± 0.07 | +0.15 |
| After block 24 alone | B's trials | readout 1, the even places | 794 | 14 | inhibitory in readout 1 | −0.20 ± 0.02 | −0.12 ± 0.02 | +0.08 |
| After block 24 alone | B's trials | readout 1, the even places | 126 | 6 | readout 1's pattern targets | −0.18 ± 0.02 | −0.15 ± 0.01 | +0.03 |
| After block 24 alone | B's trials | readout 1, the even places | 494 | 14 | inhibitory in readout 1 | −0.17 ± 0.02 | −0.12 ± 0.02 | +0.05 |
| After block 24 alone | B's trials | readout 1, the even places | 1018 | 18 | readout 1's pattern targets | −0.15 ± 0.02 | −0.11 ± 0.02 | +0.04 |
| After block 24 alone | B's trials | readout 1, the even places | 878 | 18 | readout 1's pattern targets | −0.12 ± 0.02 | −0.11 ± 0.01 | +0.01 |
| After block 24 alone | B's trials | readout 1, the even places | 268 | 8 | readout 1's pattern targets | −0.12 ± 0.02 | −0.06 ± 0.01 | +0.06 |
| After block 24 alone | B's trials | readout 1, the even places | 686 | 6 | readout 1's pattern targets | −0.11 ± 0.02 | −0.05 ± 0.02 | +0.07 |
| After block 24 alone | B's trials | readout 1, the even places | 108 | 8 | readout 1's pattern targets | −0.11 ± 0.02 | −0.09 ± 0.01 | +0.02 |
| After block 24 alone | B's trials | readout 1, the even places | 918 | 18 | readout 1's pattern targets | −0.11 ± 0.02 | −0.08 ± 0.01 | +0.03 |
| After block 24 alone | B's trials | readout 1, the even places | 658 | 18 | readout 1's pattern targets | −0.10 ± 0.04 | −0.02 ± 0.01 | +0.07 |
| After block 24 alone | B's trials | readout 1, the even places | 164 | 4 | inhibitory in readout 1 | −0.09 ± 0.02 | −0.05 ± 0.01 | +0.04 |

On the unrounded values, 252 of 277 of these rows lie nearer the night without replay at block 36 than right after and 25 further from it,
and 4 read alike at two decimals; 41 changed sign, and 192 still lie beyond 2 SE the same way.

## 5. What each replayed night changed in the readouts' inputs

**By presynaptic group**, each night's change onto each readout's 459 units, each replayed night beside the night without
replay's same night; a single night's replayed night and the night without replay's same night ran on the same image:

| Night after block | Form | Its job | This night | Onto | A's kinds (classes 0, 1), M | B's kinds (2, 3), M | the odd places' excitatory (4, 6, 7), M | the even places' (5, 8, 9), M | other excitatory (10), M | inhibitory (11 to 13), M | All excitatory | Excitatory less inhibitory |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | After block 6 alone | job one | replays | readout 0, the odd places | +0.35 | −0.29 | −1.20 | −3.01 | −0.04 | +5.25 | −4.18 | −9.42 |
| 6 | After block 6 alone | job one | replays | readout 1, the even places | −0.35 | +0.28 | −2.62 | −0.72 | −0.03 | +4.19 | −3.44 | −7.63 |
| 6 | Plain on all four nights (H-47) | job one | replays | readout 0, the odd places | +0.35 | −0.29 | −1.20 | −3.01 | −0.04 | +5.25 | −4.18 | −9.42 |
| 6 | Plain on all four nights (H-47) | job one | replays | readout 1, the even places | −0.35 | +0.28 | −2.62 | −0.72 | −0.03 | +4.19 | −3.44 | −7.63 |
| 6 | Without replay | job two | without replay | readout 0, the odd places | +0.02 | −0.02 | −0.02 | −0.11 | −0.03 | +0.29 | −0.16 | −0.44 |
| 6 | Without replay | job two | without replay | readout 1, the even places | −0.02 | +0.02 | +0.07 | −0.20 | −0.02 | +0.27 | −0.15 | −0.41 |
| 12 | After block 12 alone | job two | replays | readout 0, the odd places | +0.40 | −0.30 | −0.97 | −2.70 | −0.03 | +5.36 | −3.60 | −8.96 |
| 12 | After block 12 alone | job two | replays | readout 1, the even places | −0.40 | +0.30 | −2.44 | −0.83 | −0.04 | +4.16 | −3.40 | −7.57 |
| 12 | Plain on all four nights (H-47) | job one | replays | readout 0, the odd places | +0.31 | −0.28 | −0.61 | −2.17 | −0.02 | +4.32 | −2.77 | −7.09 |
| 12 | Plain on all four nights (H-47) | job one | replays | readout 1, the even places | −0.31 | +0.28 | −1.81 | −0.68 | −0.03 | +3.48 | −2.55 | −6.03 |
| 12 | Without replay | job two | without replay | readout 0, the odd places | 0.00 | −0.04 | +0.04 | +0.03 | −0.02 | +0.26 | 0.00 | −0.26 |
| 12 | Without replay | job two | without replay | readout 1, the even places | 0.00 | +0.04 | +0.21 | −0.06 | −0.02 | +0.25 | +0.18 | −0.07 |
| 18 | After block 18 alone | job two | replays | readout 0, the odd places | +0.42 | −0.28 | −1.07 | −2.68 | −0.02 | +5.34 | −3.63 | −8.98 |
| 18 | After block 18 alone | job two | replays | readout 1, the even places | −0.42 | +0.28 | −2.33 | −0.70 | −0.01 | +4.26 | −3.18 | −7.44 |
| 18 | Plain on all four nights (H-47) | job one | replays | readout 0, the odd places | +0.27 | −0.29 | −0.36 | −1.65 | −0.03 | +3.72 | −2.06 | −5.78 |
| 18 | Plain on all four nights (H-47) | job one | replays | readout 1, the even places | −0.27 | +0.29 | −1.24 | −0.40 | −0.01 | +2.88 | −1.64 | −4.52 |
| 18 | Without replay | job two | without replay | readout 0, the odd places | +0.02 | −0.02 | +0.07 | +0.01 | −0.01 | +0.29 | +0.06 | −0.23 |
| 18 | Without replay | job two | without replay | readout 1, the even places | −0.02 | +0.02 | +0.16 | +0.02 | 0.00 | +0.30 | +0.18 | −0.11 |
| 24 | After block 24 alone | job two | replays | readout 0, the odd places | +0.38 | −0.34 | −1.06 | −2.71 | −0.02 | +5.34 | −3.76 | −9.10 |
| 24 | After block 24 alone | job two | replays | readout 1, the even places | −0.38 | +0.34 | −2.30 | −0.67 | −0.02 | +4.31 | −3.02 | −7.33 |
| 24 | Plain on all four nights (H-47) | job one | replays | readout 0, the odd places | +0.22 | −0.16 | −0.01 | −1.35 | −0.02 | +3.36 | −1.32 | −4.68 |
| 24 | Plain on all four nights (H-47) | job one | replays | readout 1, the even places | −0.22 | +0.16 | −0.87 | −0.18 | −0.01 | +2.50 | −1.13 | −3.63 |
| 24 | Without replay | job two | without replay | readout 0, the odd places | +0.03 | +0.01 | +0.18 | −0.03 | −0.01 | +0.34 | +0.18 | −0.16 |
| 24 | Without replay | job two | without replay | readout 1, the even places | −0.03 | −0.01 | +0.06 | +0.12 | +0.01 | +0.40 | +0.15 | −0.25 |

Over each single replayed night the inhibitory units' input onto the readouts rose by 4.16 to 5.36 M (the least after block 12 alone onto
readout 1, the most after block 12 alone onto readout 0). The odd places' excitatory units' input onto readout 0 fell by 0.97 to 1.20 M and
onto readout 1 fell by 2.30 to 2.62, the even places' onto readout 1 fell by 0.67 to 0.83 and onto readout 0 fell by 2.68 to 3.01, and the
other excitatory units' fell by 0.01 to 0.04: of the eight place groups' changes, 8 lie lower onto the other readout than onto the group's
own. A's kinds' input onto readout 0, A's answer, rose by 0.35 to 0.42 M and onto readout 1 fell by 0.35 to 0.42; B's kinds' input onto
readout 1, B's answer, rose by 0.28 to 0.34 M and onto readout 0 fell by 0.28 to 0.34; of their sums onto the two readouts together, means
over the paths, in the class matrix's units, the largest move was A's kinds' after block 18 alone, −42. All excitatory input onto readout 0
changed by −4.18, −3.60, −3.63 and −3.76 M over the night after block 6, 12, 18 and 24 alone, and onto readout 1 by −3.44, −3.40, −3.18 and
−3.02. Over the night without replay's same nights the largest move of an excitatory group was the odd places' excitatory units' onto
readout 1 over its night after block 12, +0.21 M, and its inhibitory input onto the readouts rose by 0.25 to 0.40. H-47's plain nights after
block 12, 18 and 24 moved the inhibitory input and each place group's onto each readout less than its first did, the inhibitory input onto
readout 0 by +4.32, +3.72 and +3.36 M where its first night moved it by +5.25.

**By presynaptic class**, the same nights, in M:

| Night after block | Form | This night | Onto | 0: A alone | 1: A and C | 2: B alone | 3: B and D | 4: C's own | 5: D's own | 6: readout 0's pattern targets | 7: readout 0's other excitatory | 8: readout 1's pattern targets | 9: readout 1's other excitatory | 10: other excitatory | 11: inhibitory in readout 0 | 12: inhibitory in readout 1 | 13: inhibitory elsewhere |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | After block 6 alone | replays | readout 0, the odd places | +0.156 | +0.198 | −0.086 | −0.199 | +0.069 | −0.078 | −0.969 | −0.295 | −2.079 | −0.852 | −0.041 | +2.795 | +2.450 | 0.000 |
| 6 | After block 6 alone | replays | readout 1, the even places | −0.156 | −0.198 | +0.086 | +0.199 | −0.069 | +0.078 | −2.205 | −0.344 | −0.403 | −0.394 | −0.031 | +2.068 | +2.123 | 0.000 |
| 6 | Plain on all four nights (H-47) | replays | readout 0, the odd places | +0.156 | +0.198 | −0.086 | −0.199 | +0.069 | −0.078 | −0.969 | −0.295 | −2.079 | −0.852 | −0.041 | +2.795 | +2.450 | 0.000 |
| 6 | Plain on all four nights (H-47) | replays | readout 1, the even places | −0.156 | −0.198 | +0.086 | +0.199 | −0.069 | +0.078 | −2.205 | −0.344 | −0.403 | −0.394 | −0.031 | +2.068 | +2.123 | 0.000 |
| 6 | Without replay | without replay | readout 0, the odd places | −0.029 | +0.048 | +0.015 | −0.031 | −0.029 | −0.009 | −0.120 | +0.124 | +0.042 | −0.138 | −0.031 | +0.136 | +0.151 | 0.000 |
| 6 | Without replay | without replay | readout 1, the even places | +0.029 | −0.048 | −0.015 | +0.031 | +0.029 | +0.009 | +0.017 | +0.028 | −0.098 | −0.108 | −0.022 | +0.121 | +0.145 | 0.000 |
| 12 | After block 12 alone | replays | readout 0, the odd places | +0.176 | +0.226 | −0.056 | −0.246 | +0.107 | −0.054 | −0.764 | −0.310 | −1.897 | −0.752 | −0.028 | +2.784 | +2.579 | 0.000 |
| 12 | After block 12 alone | replays | readout 1, the even places | −0.176 | −0.226 | +0.056 | +0.246 | −0.107 | +0.054 | −2.047 | −0.283 | −0.539 | −0.348 | −0.036 | +2.060 | +2.104 | 0.000 |
| 12 | Plain on all four nights (H-47) | replays | readout 0, the odd places | +0.130 | +0.177 | −0.080 | −0.197 | +0.096 | −0.047 | −0.478 | −0.224 | −1.433 | −0.692 | −0.025 | +2.259 | +2.058 | 0.000 |
| 12 | Plain on all four nights (H-47) | replays | readout 1, the even places | −0.130 | −0.177 | +0.080 | +0.197 | −0.096 | +0.047 | −1.494 | −0.221 | −0.362 | −0.361 | −0.028 | +1.651 | +1.833 | 0.000 |
| 12 | Without replay | without replay | readout 0, the odd places | −0.059 | +0.056 | +0.033 | −0.073 | 0.000 | −0.015 | −0.040 | +0.078 | +0.075 | −0.029 | −0.023 | +0.132 | +0.128 | 0.000 |
| 12 | Without replay | without replay | readout 1, the even places | +0.059 | −0.056 | −0.033 | +0.073 | 0.000 | +0.015 | +0.107 | +0.104 | −0.015 | −0.058 | −0.019 | +0.118 | +0.132 | 0.000 |
| 18 | After block 18 alone | replays | readout 0, the odd places | +0.153 | +0.264 | −0.081 | −0.203 | +0.115 | −0.037 | −0.878 | −0.304 | −1.851 | −0.793 | −0.018 | +2.722 | +2.621 | 0.000 |
| 18 | After block 18 alone | replays | readout 1, the even places | −0.154 | −0.264 | +0.081 | +0.203 | −0.115 | +0.037 | −1.984 | −0.235 | −0.374 | −0.360 | −0.014 | +2.074 | +2.184 | 0.000 |
| 18 | Plain on all four nights (H-47) | replays | readout 0, the odd places | +0.122 | +0.151 | −0.090 | −0.199 | +0.053 | −0.019 | −0.198 | −0.219 | −1.095 | −0.541 | −0.025 | +1.917 | +1.803 | 0.000 |
| 18 | Plain on all four nights (H-47) | replays | readout 1, the even places | −0.122 | −0.151 | +0.090 | +0.199 | −0.053 | +0.019 | −1.028 | −0.164 | −0.131 | −0.287 | −0.014 | +1.368 | +1.510 | 0.000 |
| 18 | Without replay | without replay | readout 0, the odd places | −0.046 | +0.066 | +0.029 | −0.049 | +0.003 | −0.016 | +0.067 | −0.004 | +0.117 | −0.093 | −0.014 | +0.144 | +0.150 | 0.000 |
| 18 | Without replay | without replay | readout 1, the even places | +0.046 | −0.066 | −0.029 | +0.049 | −0.003 | +0.016 | +0.047 | +0.112 | +0.053 | −0.044 | +0.002 | +0.105 | +0.190 | 0.000 |
| 24 | After block 24 alone | replays | readout 0, the odd places | +0.178 | +0.197 | −0.098 | −0.243 | +0.103 | −0.057 | −0.857 | −0.308 | −1.884 | −0.767 | −0.023 | +2.753 | +2.582 | 0.000 |
| 24 | After block 24 alone | replays | readout 1, the even places | −0.178 | −0.197 | +0.098 | +0.243 | −0.103 | +0.057 | −1.962 | −0.236 | −0.362 | −0.362 | −0.018 | +2.118 | +2.195 | 0.000 |
| 24 | Plain on all four nights (H-47) | replays | readout 0, the odd places | +0.103 | +0.120 | −0.035 | −0.121 | +0.066 | −0.007 | +0.055 | −0.133 | −0.857 | −0.486 | −0.023 | +1.794 | +1.565 | 0.000 |
| 24 | Plain on all four nights (H-47) | replays | readout 1, the even places | −0.102 | −0.120 | +0.035 | +0.121 | −0.066 | +0.007 | −0.712 | −0.092 | +0.042 | −0.230 | −0.013 | +1.268 | +1.234 | 0.000 |
| 24 | Without replay | without replay | readout 0, the odd places | −0.025 | +0.056 | +0.030 | −0.021 | +0.017 | −0.007 | +0.097 | +0.063 | +0.060 | −0.081 | −0.012 | +0.168 | +0.167 | 0.000 |
| 24 | Without replay | without replay | readout 1, the even places | +0.025 | −0.056 | −0.030 | +0.021 | −0.017 | +0.007 | +0.022 | +0.058 | +0.126 | −0.015 | +0.011 | +0.211 | +0.193 | 0.000 |

Over each single replayed night the class that lost the most of the excitatory classes onto a readout was the other readout's pattern
targets in 8 of the 8 cases. Readout 0's pattern targets' input onto readout 1 fell by 1.96 to 2.21 M, and readout 1's onto readout 0 fell
by 1.85 to 2.08, where readout 1's own pattern targets' input onto readout 1 fell by 0.36 to 0.54 and readout 0's onto readout 0 fell by
0.76 to 0.97. The inhibitory units in readout 0 moved their input onto a readout by +2.06 to +2.80 M and those in readout 1 by +2.10 to
+2.62. Each of A's and B's four kinds, C's own and D's own moved onto one readout about what it moved the other way onto the other: of their
sums onto the two readouts, in the class matrix's units, the largest moves, equal in size, were class 0's, A alone, after block 18 alone,
−29 and class 0's, A alone, after block 12 alone, +29.

**By the readouts' classes**, each single night's replayed night and the night without replay's same night, by group, in M:

| Night after block | Form | This night | Onto the class | A's kinds (classes 0, 1) | B's kinds (2, 3) | the odd places' excitatory (4, 6, 7) | the even places' (5, 8, 9) | other excitatory (10) | inhibitory (11 to 13) |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | After block 6 alone | replays | C's own | +0.04 | −0.02 | −0.20 | −0.19 | 0.00 | +0.18 |
| 6 | After block 6 alone | replays | readout 0's pattern targets | +0.36 | −0.19 | −0.35 | −2.37 | −0.03 | +4.13 |
| 6 | After block 6 alone | replays | readout 0's other excitatory | 0.00 | −0.01 | −0.01 | −0.05 | 0.00 | +0.31 |
| 6 | After block 6 alone | replays | inhibitory in readout 0 | −0.05 | −0.06 | −0.63 | −0.40 | −0.01 | +0.63 |
| 6 | After block 6 alone | replays | D's own | −0.02 | +0.03 | −0.13 | +0.04 | 0.00 | +0.24 |
| 6 | After block 6 alone | replays | readout 1's pattern targets | −0.20 | +0.37 | −2.04 | −0.25 | −0.03 | +2.82 |
| 6 | After block 6 alone | replays | readout 1's other excitatory | −0.03 | 0.00 | −0.20 | −0.24 | 0.00 | +0.55 |
| 6 | After block 6 alone | replays | inhibitory in readout 1 | −0.11 | −0.11 | −0.25 | −0.27 | −0.01 | +0.59 |
| 6 | Without replay | without replay | C's own | −0.01 | 0.00 | −0.01 | +0.01 | 0.00 | −0.04 |
| 6 | Without replay | without replay | readout 0's pattern targets | 0.00 | −0.03 | −0.21 | −0.19 | −0.03 | +0.22 |
| 6 | Without replay | without replay | readout 0's other excitatory | 0.00 | 0.00 | −0.01 | −0.01 | 0.00 | +0.05 |
| 6 | Without replay | without replay | inhibitory in readout 0 | +0.03 | +0.01 | +0.20 | +0.08 | 0.00 | +0.05 |
| 6 | Without replay | without replay | D's own | 0.00 | 0.00 | −0.03 | −0.01 | 0.00 | −0.05 |
| 6 | Without replay | without replay | readout 1's pattern targets | −0.06 | −0.02 | −0.07 | −0.29 | −0.02 | +0.21 |
| 6 | Without replay | without replay | readout 1's other excitatory | +0.02 | 0.00 | +0.01 | −0.12 | 0.00 | +0.07 |
| 6 | Without replay | without replay | inhibitory in readout 1 | +0.02 | +0.04 | +0.16 | +0.21 | −0.01 | +0.03 |
| 12 | After block 12 alone | replays | C's own | +0.06 | −0.01 | −0.15 | −0.22 | 0.00 | +0.19 |
| 12 | After block 12 alone | replays | readout 0's pattern targets | +0.41 | −0.20 | −0.16 | −2.05 | −0.02 | +4.24 |
| 12 | After block 12 alone | replays | readout 0's other excitatory | 0.00 | −0.01 | −0.02 | −0.04 | 0.00 | +0.28 |
| 12 | After block 12 alone | replays | inhibitory in readout 0 | −0.07 | −0.08 | −0.65 | −0.40 | −0.01 | +0.65 |
| 12 | After block 12 alone | replays | D's own | −0.01 | +0.01 | −0.15 | 0.00 | 0.00 | +0.23 |
| 12 | After block 12 alone | replays | readout 1's pattern targets | −0.22 | +0.37 | −1.80 | −0.33 | −0.02 | +2.79 |
| 12 | After block 12 alone | replays | readout 1's other excitatory | −0.06 | 0.00 | −0.21 | −0.23 | 0.00 | +0.53 |
| 12 | After block 12 alone | replays | inhibitory in readout 1 | −0.11 | −0.08 | −0.27 | −0.28 | −0.01 | +0.61 |
| 12 | Without replay | without replay | C's own | 0.00 | 0.00 | +0.02 | +0.04 | 0.00 | −0.03 |
| 12 | Without replay | without replay | readout 0's pattern targets | −0.02 | −0.03 | −0.16 | −0.01 | −0.02 | +0.20 |
| 12 | Without replay | without replay | readout 0's other excitatory | 0.00 | 0.00 | +0.03 | −0.03 | 0.00 | +0.05 |
| 12 | Without replay | without replay | inhibitory in readout 0 | +0.02 | −0.01 | +0.15 | +0.03 | 0.00 | +0.04 |
| 12 | Without replay | without replay | D's own | 0.00 | −0.01 | −0.04 | 0.00 | 0.00 | −0.07 |
| 12 | Without replay | without replay | readout 1's pattern targets | −0.04 | −0.01 | +0.08 | −0.12 | −0.01 | +0.17 |
| 12 | Without replay | without replay | readout 1's other excitatory | +0.03 | 0.00 | 0.00 | −0.11 | 0.00 | +0.09 |
| 12 | Without replay | without replay | inhibitory in readout 1 | +0.02 | +0.06 | +0.16 | +0.17 | −0.01 | +0.06 |
| 18 | After block 18 alone | replays | C's own | +0.05 | −0.01 | −0.15 | −0.24 | 0.00 | +0.18 |
| 18 | After block 18 alone | replays | readout 0's pattern targets | +0.41 | −0.19 | −0.20 | −1.97 | −0.01 | +4.22 |
| 18 | After block 18 alone | replays | readout 0's other excitatory | 0.00 | −0.01 | −0.06 | −0.06 | 0.00 | +0.26 |
| 18 | After block 18 alone | replays | inhibitory in readout 0 | −0.04 | −0.07 | −0.65 | −0.41 | 0.00 | +0.68 |
| 18 | After block 18 alone | replays | D's own | −0.02 | +0.01 | −0.15 | 0.00 | 0.00 | +0.23 |
| 18 | After block 18 alone | replays | readout 1's pattern targets | −0.22 | +0.36 | −1.70 | −0.23 | −0.02 | +2.84 |
| 18 | After block 18 alone | replays | readout 1's other excitatory | −0.05 | 0.00 | −0.21 | −0.17 | 0.00 | +0.54 |
| 18 | After block 18 alone | replays | inhibitory in readout 1 | −0.13 | −0.08 | −0.27 | −0.30 | +0.01 | +0.64 |
| 18 | Without replay | without replay | C's own | 0.00 | +0.01 | +0.01 | +0.01 | 0.00 | −0.04 |
| 18 | Without replay | without replay | readout 0's pattern targets | +0.01 | −0.03 | −0.10 | −0.05 | −0.01 | +0.22 |
| 18 | Without replay | without replay | readout 0's other excitatory | 0.00 | 0.00 | −0.01 | −0.03 | 0.00 | +0.04 |
| 18 | Without replay | without replay | inhibitory in readout 0 | +0.01 | −0.01 | +0.17 | +0.08 | 0.00 | +0.07 |
| 18 | Without replay | without replay | D's own | 0.00 | 0.00 | −0.04 | 0.00 | 0.00 | −0.08 |
| 18 | Without replay | without replay | readout 1's pattern targets | −0.06 | −0.01 | +0.10 | −0.07 | 0.00 | +0.22 |
| 18 | Without replay | without replay | readout 1's other excitatory | +0.02 | 0.00 | +0.02 | −0.06 | 0.00 | +0.09 |
| 18 | Without replay | without replay | inhibitory in readout 1 | +0.01 | +0.03 | +0.08 | +0.15 | 0.00 | +0.07 |
| 24 | After block 24 alone | replays | C's own | +0.05 | −0.01 | −0.19 | −0.24 | 0.00 | +0.19 |
| 24 | After block 24 alone | replays | readout 0's pattern targets | +0.35 | −0.21 | −0.19 | −1.96 | −0.02 | +4.21 |
| 24 | After block 24 alone | replays | readout 0's other excitatory | 0.00 | −0.02 | −0.01 | −0.06 | 0.00 | +0.28 |
| 24 | After block 24 alone | replays | inhibitory in readout 0 | −0.03 | −0.10 | −0.67 | −0.44 | 0.00 | +0.66 |
| 24 | After block 24 alone | replays | D's own | −0.03 | +0.02 | −0.15 | −0.01 | 0.00 | +0.24 |
| 24 | After block 24 alone | replays | readout 1's pattern targets | −0.20 | +0.41 | −1.67 | −0.12 | −0.02 | +2.87 |
| 24 | After block 24 alone | replays | readout 1's other excitatory | −0.04 | 0.00 | −0.18 | −0.19 | 0.00 | +0.52 |
| 24 | After block 24 alone | replays | inhibitory in readout 1 | −0.11 | −0.08 | −0.30 | −0.35 | 0.00 | +0.69 |
| 24 | Without replay | without replay | C's own | +0.01 | 0.00 | +0.04 | +0.01 | 0.00 | −0.01 |
| 24 | Without replay | without replay | readout 0's pattern targets | +0.01 | +0.01 | −0.02 | −0.06 | −0.01 | +0.24 |
| 24 | Without replay | without replay | readout 0's other excitatory | 0.00 | 0.00 | +0.01 | −0.06 | 0.00 | +0.02 |
| 24 | Without replay | without replay | inhibitory in readout 0 | +0.01 | 0.00 | +0.15 | +0.08 | 0.00 | +0.09 |
| 24 | Without replay | without replay | D's own | −0.01 | 0.00 | −0.04 | +0.01 | 0.00 | −0.06 |
| 24 | Without replay | without replay | readout 1's pattern targets | −0.04 | −0.04 | −0.02 | 0.00 | 0.00 | +0.25 |
| 24 | Without replay | without replay | readout 1's other excitatory | +0.02 | 0.00 | +0.04 | −0.05 | 0.00 | +0.10 |
| 24 | Without replay | without replay | inhibitory in readout 1 | 0.00 | +0.03 | +0.09 | +0.16 | +0.01 | +0.12 |

Of each readout's inhibitory change over each single replayed night its pattern targets received 66 to 79 per cent, +10 994 to +15 148 a
unit, and its inhibitory units +5 746 to +6 722 a unit, in the class matrix's units.

**The units that carried half of each readout's change** right after each single replayed night (section 3), against the rest of their
readout: each set's spikes' difference a unit and its input difference a unit from each group, the mean over its units, in the class
matrix's units; other excitatory aside:

| Comparison | Trials | Readout | Units carrying half its change (section 3 or 4) | Their spikes' difference a unit | The rest's | A's kinds: their input difference a unit | the rest's | B's kinds: theirs | the rest's | The odd places' excitatory: theirs | the rest's | The even places': theirs | the rest's | Inhibitory: theirs | the rest's | Excitatory less inhibitory: theirs | the rest's |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | 13 | −0.32 | −0.01 | −12 138 | +1 104 | −866 | −578 | −7 242 | −2 415 | −17 962 | −5 987 | +366 | +11 109 | −38 428 | −19 010 |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | 16 | −0.24 | −0.01 | −11 611 | −336 | −357 | +620 | −15 247 | −5 524 | −8 255 | −881 | +4 118 | +8 712 | −39 173 | −14 870 |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | 18 | −0.16 | −0.01 | +1 111 | +714 | −8 578 | −260 | −4 955 | −2 454 | −14 408 | −5 997 | +3 126 | +11 118 | −29 959 | −19 135 |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | 14 | −0.21 | −0.01 | −1 489 | −705 | −10 028 | +920 | −10 887 | −5 705 | −6 401 | −973 | +2 483 | +8 743 | −31 141 | −15 232 |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | 15 | −0.29 | −0.01 | −10 895 | +1 283 | −763 | −564 | −7 259 | −2 017 | −15 594 | −5 630 | +939 | +11 463 | −35 699 | −18 395 |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | 19 | −0.24 | −0.01 | −11 767 | −415 | −500 | +617 | −13 745 | −5 421 | −8 669 | −1 387 | +4 411 | +8 705 | −39 092 | −15 348 |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | 19 | −0.16 | −0.01 | +1 085 | +876 | −8 083 | −246 | −7 255 | −1 969 | −13 786 | −5 617 | +1 809 | +11 521 | −30 005 | −18 484 |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | 15 | −0.21 | −0.01 | −1 936 | −849 | −6 826 | +821 | −6 823 | −5 730 | −7 657 | −1 487 | +2 280 | +8 738 | −25 465 | −16 023 |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | 17 | −0.22 | −0.01 | −8 572 | +1 228 | −271 | −589 | −3 172 | −2 444 | −5 622 | −5 867 | −162 | +11 428 | −17 475 | −19 108 |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | 19 | −0.26 | −0.01 | −11 162 | −421 | +306 | +588 | −12 921 | −5 102 | −5 488 | −1 402 | +6 513 | +8 726 | −35 778 | −15 098 |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | 20 | −0.17 | −0.01 | +350 | +889 | −7 222 | −274 | −8 364 | −2 202 | −13 111 | −5 528 | +1 311 | +11 440 | −29 518 | −18 570 |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | 22 | −0.13 | −0.01 | −1 619 | −828 | −4 928 | +854 | −8 799 | −5 255 | −5 167 | −1 390 | +3 485 | +8 893 | −24 068 | −15 545 |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | 17 | −0.22 | −0.01 | −7 823 | +1 080 | −853 | −759 | −3 010 | −2 690 | −5 555 | −5 850 | +1 641 | +11 250 | −18 882 | −19 494 |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | 19 | −0.24 | −0.01 | −8 664 | −408 | +1 205 | +743 | −10 878 | −4 903 | −9 116 | −1 393 | +11 621 | +8 381 | −39 074 | −14 409 |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | 17 | −0.23 | −0.01 | +603 | +756 | −9 472 | −427 | −5 380 | −2 599 | −13 264 | −5 554 | +2 864 | +11 203 | −30 609 | −19 043 |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | 17 | −0.18 | −0.01 | −910 | −744 | −7 552 | +1 082 | −7 491 | −5 060 | −5 649 | −1 561 | +4 036 | +8 688 | −25 701 | −15 035 |

Right after each single replayed night, in the 16 sets whose readout difference lies beyond 2 SE: on A's trials the units that carried half
of each readout's change had an input difference from A's kinds of −12 138 to −7 823 a unit, below 0 in 8 of 8 sets, where the rest of their
readout had −421 to +1 283; on B's trials theirs from B's kinds −10 028 to −4 928, below 0 in 8 of 8, and the rest's −427 to +1 082. Theirs
from the trial's stimulus's kinds lay below the rest's in 16 of the 16 sets. Their excitatory input less inhibitory input difference read
−39 173 to −17 475 a unit and the rest's −19 494 to −14 409, theirs below the rest's in 14 of the 16; their excitatory input difference lay
below the rest's in 16, and their inhibitory input difference, −162 to +11 621 a unit against the rest's +8 381 to +11 521, above the rest's
in 1. Their spikes' difference a unit read −0.32 to −0.13 and the rest's −0.01.

**The gain**, before and after each night:

| Night after block | Form | This night | Gain before the night | After it | Its change |
| :--- | :--- | :--- | ---: | ---: | :--- |
| 6 | After block 6 alone | replays | 1.695 | 1.622 | −0.073 |
| 6 | Plain on all four nights (H-47) | replays | 1.695 | 1.622 | −0.073 |
| 6 | Without replay | without replay | 1.695 | 1.726 | +0.031 |
| 12 | After block 12 alone | replays | 1.689 | 1.616 | −0.073 |
| 12 | Plain on all four nights (H-47) | replays | 1.699 | 1.633 | −0.066 |
| 12 | Without replay | without replay | 1.689 | 1.723 | +0.034 |
| 18 | After block 18 alone | replays | 1.683 | 1.609 | −0.074 |
| 18 | Plain on all four nights (H-47) | replays | 1.698 | 1.642 | −0.056 |
| 18 | Without replay | without replay | 1.683 | 1.720 | +0.037 |
| 24 | After block 24 alone | replays | 1.675 | 1.601 | −0.074 |
| 24 | Plain on all four nights (H-47) | replays | 1.696 | 1.647 | −0.050 |
| 24 | Without replay | without replay | 1.675 | 1.712 | +0.037 |

Each single replayed night lowered the gain by 0.073 to 0.074 (the least after block 6 alone, the most after block 24 alone), where the
night without replay's same nights raised it by 0.031 to 0.037; H-47's later plain nights lowered it by 0.050 to 0.066. The gain on the
image before the nights after block 6, 12, 18 and 24, which each single night's replayed night started from, was 1.695, 1.689, 1.683 and
1.675.

**The image each replayed night starts from**: a single night's image before its replayed night is the night without replay's image before
the same night, in M, with its gain:

| Image before the night after block | Onto | A's kinds (classes 0, 1), M | B's kinds (2, 3), M | the odd places' excitatory (4, 6, 7), M | the even places' (5, 8, 9), M | other excitatory (10), M | inhibitory (11 to 13), M | Its gain |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
| 6 | readout 0, the odd places | +6.36 | +6.36 | +41.83 | +41.91 | +0.36 | +26.39 | 1.695 |
| 6 | readout 1, the even places | +6.34 | +6.75 | +41.06 | +41.41 | +0.36 | +25.16 | 1.695 |
| 12 | readout 0, the odd places | +6.27 | +6.42 | +41.75 | +41.86 | +0.33 | +26.79 | 1.689 |
| 12 | readout 1, the even places | +6.43 | +6.69 | +41.19 | +41.16 | +0.34 | +25.58 | 1.689 |
| 18 | readout 0, the odd places | +6.19 | +6.44 | +41.76 | +41.94 | +0.31 | +27.08 | 1.683 |
| 18 | readout 1, the even places | +6.51 | +6.67 | +41.44 | +41.05 | +0.32 | +25.99 | 1.683 |
| 24 | readout 0, the odd places | +6.14 | +6.44 | +41.80 | +41.97 | +0.29 | +27.44 | 1.675 |
| 24 | readout 1, the even places | +6.56 | +6.66 | +41.62 | +41.06 | +0.32 | +26.52 | 1.675 |

| Image before the night after block | A's kinds onto readout 0, M | onto readout 1 | readout 0 less readout 1 | B's kinds onto readout 1 | onto readout 0 | readout 1 less readout 0 | A's margin |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | +6.36 | +6.34 | +0.03 | +6.75 | +6.36 | +0.39 | +0.42 |
| 12 | +6.27 | +6.43 | −0.16 | +6.69 | +6.42 | +0.27 | +0.12 |
| 18 | +6.19 | +6.51 | −0.33 | +6.67 | +6.44 | +0.23 | −0.10 |
| 24 | +6.14 | +6.56 | −0.43 | +6.66 | +6.44 | +0.22 | −0.21 |

On the image each replayed night starts from, A's kinds' input onto readout 0, A's answer, less onto readout 1 read +0.03, −0.16, −0.33 and
−0.43 M before the nights after block 6, 12, 18 and 24, B's kinds' onto readout 1 less onto readout 0 +0.39, +0.27, +0.23 and +0.22, and A's
margin +0.42, +0.12, −0.10 and −0.21. The inhibitory input onto readout 0 was 26.39 M before the night after block 6 and 27.44 before the
night after block 24, onto readout 1 25.16 and 26.52; of A's and B's kinds' and each place group's input onto each readout, between the two
the largest move was the odd places' excitatory units' onto readout 1, +0.55 M, and the next the even places' onto readout 1, −0.35.

## 6. The inputs against the spikes

**The correlation** over each readout's 459 units of a unit's input difference from each group right after the replayed night
with its spikes' difference there (section 3); "–" where a group's input difference is the same on every unit:

| Comparison | Trials | Readout | Its spikes' difference (section 3 or 4) | A's kinds | B's kinds | The odd places' excitatory | The even places' | Other excitatory | Inhibitory | All excitatory | Excitatory less inhibitory |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | −8.32 | +0.58 | +0.02 | +0.09 | +0.34 | −0.12 | +0.18 | +0.43 | +0.24 |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | −7.48 | +0.83 | +0.06 | +0.22 | +0.16 | −0.08 | +0.06 | +0.41 | +0.33 |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | −5.66 | −0.02 | +0.71 | +0.11 | +0.16 | +0.03 | +0.17 | +0.29 | +0.13 |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | −5.87 | +0.04 | +0.53 | +0.12 | +0.14 | −0.06 | +0.13 | +0.32 | +0.20 |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | −8.48 | +0.59 | +0.03 | +0.10 | +0.22 | +0.25 | +0.17 | +0.38 | +0.19 |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | −9.17 | +0.78 | +0.10 | +0.21 | +0.21 | −0.01 | +0.08 | +0.46 | +0.35 |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | −6.16 | +0.02 | +0.69 | +0.14 | +0.21 | +0.10 | +0.17 | +0.35 | +0.17 |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | −6.08 | +0.09 | +0.45 | +0.08 | +0.19 | −0.03 | +0.13 | +0.28 | +0.16 |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | −7.07 | +0.61 | −0.01 | +0.06 | +0.03 | −0.05 | +0.17 | +0.20 | +0.04 |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | −9.49 | +0.71 | +0.01 | +0.22 | +0.14 | −0.01 | +0.06 | +0.40 | +0.30 |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | −6.79 | +0.02 | +0.65 | +0.19 | +0.19 | −0.08 | +0.17 | +0.35 | +0.16 |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | −5.48 | +0.08 | +0.43 | +0.14 | +0.17 | 0.00 | +0.16 | +0.32 | +0.17 |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | −7.52 | +0.55 | 0.00 | 0.00 | +0.01 | −0.03 | +0.18 | +0.15 | 0.00 |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | −8.83 | +0.66 | 0.00 | +0.20 | +0.23 | −0.01 | −0.01 | +0.40 | +0.34 |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | −7.46 | 0.00 | +0.68 | +0.07 | +0.21 | +0.24 | +0.17 | +0.32 | +0.14 |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | −5.79 | +0.01 | +0.52 | +0.07 | +0.16 | +0.01 | +0.10 | +0.28 | +0.17 |

Right after each single replayed night, over the 16 sets the correlation of a unit's input difference from the trial's stimulus's kinds with
its spikes' difference read +0.43 to +0.83, 16 of the 16 above 0: A's kinds' on A's trials +0.55 to +0.83 and B's kinds' on B's trials +0.43
to +0.71; from the other stimulus's kinds −0.02 to +0.10. Of its excitatory input less inhibitory input difference it read 0.00 to +0.35
(after block 24 alone at block 24, readout 0 on A's trials the least, after block 12 alone at block 12, readout 1 on A's trials the most),
15 of the 16 above 0; of its inhibitory input difference −0.01 to +0.18, 15 above 0; of its excitatory input difference +0.15 to +0.46, 16
above 0; of the odd places' excitatory 0.00 to +0.22 and of the even places' +0.01 to +0.34.

**A's kinds onto each readout**, each single night's difference right after its replayed night and H-47's plain night's at blocks 6 to 24,
beside the spikes' readout 0 less readout 1 on A's trials and readout 1 less readout 0 on B's (section 3, where their SEs are):

| Comparison | A's kinds onto readout 0, its difference, M | onto readout 1 | readout 0 less readout 1 | B's kinds onto readout 1 | onto readout 0 | readout 1 less readout 0 | Spikes on A's trials: readout 0 less readout 1 (section 3) | On B's trials: readout 1 less readout 0 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | +0.33 | −0.33 | +0.67 | +0.27 | −0.27 | +0.54 | −0.84 | −0.21 |
| After block 12 alone less without replay, block 12 | +0.41 | −0.41 | +0.81 | +0.26 | −0.26 | +0.52 | +0.69 | +0.08 |
| After block 18 alone less without replay, block 18 | +0.40 | −0.40 | +0.79 | +0.26 | −0.26 | +0.53 | +2.42 | +1.32 |
| After block 24 alone less without replay, block 24 | +0.34 | −0.34 | +0.69 | +0.35 | −0.35 | +0.70 | +1.31 | +1.66 |
| Plain on all four nights less without replay, block 6 | +0.33 | −0.33 | +0.67 | +0.27 | −0.27 | +0.54 | −0.84 | −0.21 |
| Plain on all four nights less without replay, block 12 | +0.65 | −0.65 | +1.29 | +0.47 | −0.47 | +0.94 | −0.87 | +0.03 |
| Plain on all four nights less without replay, block 18 | +0.94 | −0.94 | +1.88 | +0.74 | −0.74 | +1.48 | +0.68 | +2.72 |
| Plain on all four nights less without replay, block 24 | +1.18 | −1.18 | +2.36 | +0.86 | −0.86 | +1.72 | +1.50 | +3.10 |

Right after each single replayed night A's kinds' input onto readout 0, A's answer, lay above the night without replay's by 0.33 to 0.41 M
and onto readout 1 below it by 0.33 to 0.41: readout 0's difference less readout 1's read +0.67 to +0.81 (after block 12 alone the most,
after block 6 alone the least), below 0 after 0 of the four single nights, where the spikes on A's trials, readout 0 less readout 1, read
−0.84, +0.69, +2.42 and +1.31 after block 6, 12, 18 and 24 alone. B's kinds' onto readout 1 less onto readout 0 read +0.52 to +0.70, where
the spikes on B's trials, readout 1 less readout 0, read −0.21, +0.08, +1.32 and +1.66. Under H-47's plain night A's kinds' readout 0 less
readout 1 read +1.29, +1.88 and +2.36 at blocks 12, 18 and 24.

**Readout 0 against readout 1**: each group's difference onto readout 0 less onto readout 1, and each readout's excitatory input less its
inhibitory input, each comparison's, beside the spikes' readout 0 less readout 1 on A's trials and readout 1 less readout 0 on B's:

| Comparison | A's kinds (classes 0, 1): readout 0's difference less readout 1's, M | B's kinds (2, 3): readout 0's difference less readout 1's, M | the odd places' excitatory (4, 6, 7): readout 0's difference less readout 1's, M | the even places' (5, 8, 9): readout 0's difference less readout 1's, M | other excitatory (10): readout 0's difference less readout 1's, M | inhibitory (11 to 13): readout 0's difference less readout 1's, M | Excitatory less inhibitory: readout 0's difference | readout 1's | readout 0's less readout 1's | Spikes on A's trials: readout 0 less readout 1 (sections 3 and 4) | On B's trials: readout 1 less readout 0 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | +0.67 | −0.54 | +1.52 | −2.38 | 0.00 | +1.03 | −8.98 | −7.21 | −1.76 | −0.84 | −0.21 |
| After block 12 alone less without replay, block 12 | +0.81 | −0.52 | +1.64 | −1.96 | +0.01 | +1.19 | −8.70 | −7.50 | −1.21 | +0.69 | +0.08 |
| After block 18 alone less without replay, block 18 | +0.79 | −0.53 | +1.36 | −1.97 | +0.01 | +1.09 | −8.74 | −7.32 | −1.42 | +2.42 | +1.32 |
| After block 24 alone less without replay, block 24 | +0.69 | −0.70 | +1.12 | −1.89 | +0.02 | +1.09 | −8.94 | −7.08 | −1.86 | +1.31 | +1.66 |
| Plain on all four nights less without replay, block 6 | +0.67 | −0.54 | +1.52 | −2.38 | 0.00 | +1.03 | −8.98 | −7.21 | −1.76 | −0.84 | −0.21 |
| Plain on all four nights less without replay, block 12 | +1.29 | −0.94 | +2.88 | −4.00 | +0.01 | +1.61 | −14.78 | −12.41 | −2.37 | −0.87 | +0.03 |
| Plain on all four nights less without replay, block 18 | +1.88 | −1.48 | +3.87 | −5.27 | +0.01 | +2.34 | −19.31 | −15.98 | −3.33 | +0.68 | +2.72 |
| Plain on all four nights less without replay, block 24 | +2.36 | −1.72 | +4.64 | −6.29 | +0.02 | +3.12 | −22.63 | −18.54 | −4.09 | +1.50 | +3.10 |
| Plain on all four nights less without replay, block 36 | +2.39 | −1.77 | +4.57 | −6.34 | +0.02 | +2.71 | −20.57 | −16.73 | −3.83 | +1.39 | +3.80 |
| After block 6 alone less without replay, block 36 | +0.66 | −0.50 | +1.16 | −2.17 | +0.03 | −0.26 | −4.84 | −4.28 | −0.56 | +0.63 | +0.98 |
| After block 12 alone less without replay, block 36 | +0.74 | −0.43 | +1.11 | −1.79 | +0.03 | +0.20 | −5.16 | −4.62 | −0.54 | +0.95 | +1.18 |
| After block 18 alone less without replay, block 36 | +0.78 | −0.50 | +1.02 | −1.86 | +0.02 | +0.40 | −5.84 | −4.90 | −0.94 | +1.42 | +1.59 |
| After block 24 alone less without replay, block 36 | +0.61 | −0.60 | +1.04 | −1.89 | +0.02 | +0.67 | −7.03 | −5.55 | −1.48 | −0.28 | +1.15 |

Right after each single replayed night readout 0's excitatory input less its inhibitory input, less the night without replay's, lay below
readout 1's by 1.76, 1.21, 1.42 and 1.86 M after block 6, 12, 18 and 24 alone, where the spikes' readout 0 less readout 1 on A's trials read
−0.84, +0.69, +2.42 and +1.31 and readout 1 less readout 0 on B's trials −0.21, +0.08, +1.32 and +1.66. Readout 0's difference less readout
1's from the odd places' excitatory units read +1.12 to +1.64 M, from the even places' −2.38 to −1.89, and from the inhibitory units +1.03
to +1.19. At block 36 it read −0.56, −0.54, −0.94 and −1.48 M under after block 6, 12, 18 and 24 alone; under H-47's plain night −1.76,
−2.37, −3.33, −4.09, −3.83 at blocks 6, 12, 18, 24 and 36.

**By the readouts' classes**, each class's input difference from each group and its excitatory input less its inhibitory input, beside the
class's spikes' difference on each stimulus's trials with its SE (sections 3 and 4):

| Comparison | Class | How many units | A's kinds (classes 0, 1), M | B's kinds (2, 3), M | the odd places' excitatory (4, 6, 7), M | the even places' (5, 8, 9), M | inhibitory (11 to 13), M | Excitatory less inhibitory | Spikes on A's trials (sections 3 and 4) | On B's trials |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | C's own | 25 | +0.05 | −0.02 | −0.19 | −0.20 | +0.22 | −0.58 | −0.31 ± 0.03 | −0.23 ± 0.03 |
| After block 6 alone less without replay, block 6 | readout 0's pattern targets | 280 | +0.36 | −0.16 | −0.14 | −2.18 | +3.91 | −6.03 | −5.56 ± 0.14 | −3.14 ± 0.13 |
| After block 6 alone less without replay, block 6 | readout 0's other excitatory | 52 | 0.00 | −0.01 | −0.01 | −0.04 | +0.25 | −0.32 | −0.35 ± 0.04 | −0.35 ± 0.05 |
| After block 6 alone less without replay, block 6 | inhibitory in readout 0 | 102 | −0.07 | −0.07 | −0.84 | −0.48 | +0.58 | −2.05 | −2.09 ± 0.07 | −1.94 ± 0.16 |
| After block 6 alone less without replay, block 6 | D's own | 25 | −0.01 | +0.03 | −0.10 | +0.05 | +0.28 | −0.32 | −0.18 ± 0.03 | −0.06 ± 0.02 |
| After block 6 alone less without replay, block 6 | readout 1's pattern targets | 254 | −0.15 | +0.39 | −1.98 | +0.03 | +2.61 | −4.32 | −3.02 ± 0.09 | −3.31 ± 0.12 |
| After block 6 alone less without replay, block 6 | readout 1's other excitatory | 78 | −0.05 | 0.00 | −0.20 | −0.13 | +0.48 | −0.86 | −1.57 ± 0.08 | −0.36 ± 0.04 |
| After block 6 alone less without replay, block 6 | inhibitory in readout 1 | 102 | −0.12 | −0.15 | −0.41 | −0.48 | +0.55 | −1.72 | −2.71 ± 0.08 | −2.14 ± 0.06 |
| After block 12 alone less without replay, block 12 | C's own | 25 | +0.06 | −0.01 | −0.17 | −0.27 | +0.22 | −0.60 | −0.39 ± 0.03 | −0.28 ± 0.04 |
| After block 12 alone less without replay, block 12 | readout 0's pattern targets | 280 | +0.43 | −0.17 | +0.01 | −2.03 | +4.04 | −5.82 | −5.33 ± 0.13 | −3.47 ± 0.10 |
| After block 12 alone less without replay, block 12 | readout 0's other excitatory | 52 | 0.00 | −0.01 | −0.04 | 0.00 | +0.23 | −0.28 | −0.28 ± 0.03 | −0.33 ± 0.05 |
| After block 12 alone less without replay, block 12 | inhibitory in readout 0 | 102 | −0.08 | −0.07 | −0.80 | −0.43 | +0.61 | −2.00 | −2.48 ± 0.06 | −2.07 ± 0.10 |
| After block 12 alone less without replay, block 12 | D's own | 25 | −0.01 | +0.02 | −0.11 | 0.00 | +0.30 | −0.40 | −0.20 ± 0.02 | −0.10 ± 0.02 |
| After block 12 alone less without replay, block 12 | readout 1's pattern targets | 254 | −0.18 | +0.38 | −1.89 | −0.21 | +2.62 | −4.53 | −3.86 ± 0.10 | −3.23 ± 0.11 |
| After block 12 alone less without replay, block 12 | readout 1's other excitatory | 78 | −0.09 | 0.00 | −0.21 | −0.12 | +0.45 | −0.87 | −1.71 ± 0.07 | −0.35 ± 0.03 |
| After block 12 alone less without replay, block 12 | inhibitory in readout 1 | 102 | −0.13 | −0.14 | −0.44 | −0.45 | +0.55 | −1.70 | −3.40 ± 0.11 | −2.40 ± 0.09 |
| After block 18 alone less without replay, block 18 | C's own | 25 | +0.05 | −0.02 | −0.17 | −0.25 | +0.22 | −0.61 | −0.53 ± 0.04 | −0.50 ± 0.02 |
| After block 18 alone less without replay, block 18 | readout 0's pattern targets | 280 | +0.40 | −0.17 | −0.10 | −1.92 | +4.00 | −5.79 | −4.20 ± 0.13 | −3.57 ± 0.13 |
| After block 18 alone less without replay, block 18 | readout 0's other excitatory | 52 | 0.00 | −0.01 | −0.06 | −0.03 | +0.22 | −0.32 | −0.14 ± 0.02 | −0.34 ± 0.03 |
| After block 18 alone less without replay, block 18 | inhibitory in readout 0 | 102 | −0.05 | −0.06 | −0.81 | −0.49 | +0.61 | −2.02 | −2.21 ± 0.07 | −2.39 ± 0.10 |
| After block 18 alone less without replay, block 18 | D's own | 25 | −0.02 | +0.01 | −0.11 | 0.00 | +0.31 | −0.43 | −0.23 ± 0.04 | −0.09 ± 0.01 |
| After block 18 alone less without replay, block 18 | readout 1's pattern targets | 254 | −0.16 | +0.37 | −1.80 | −0.16 | +2.62 | −4.41 | −4.07 ± 0.13 | −3.02 ± 0.07 |
| After block 18 alone less without replay, block 18 | readout 1's other excitatory | 78 | −0.07 | 0.00 | −0.23 | −0.11 | +0.45 | −0.86 | −1.74 ± 0.06 | −0.45 ± 0.04 |
| After block 18 alone less without replay, block 18 | inhibitory in readout 1 | 102 | −0.15 | −0.11 | −0.35 | −0.45 | +0.58 | −1.62 | −3.45 ± 0.09 | −1.92 ± 0.07 |
| After block 24 alone less without replay, block 24 | C's own | 25 | +0.04 | −0.01 | −0.23 | −0.25 | +0.20 | −0.64 | −0.54 ± 0.04 | −0.35 ± 0.04 |
| After block 24 alone less without replay, block 24 | readout 0's pattern targets | 280 | +0.34 | −0.22 | −0.18 | −1.90 | +3.97 | −5.93 | −4.43 ± 0.17 | −4.06 ± 0.24 |
| After block 24 alone less without replay, block 24 | readout 0's other excitatory | 52 | 0.00 | −0.02 | −0.02 | −0.01 | +0.26 | −0.31 | −0.17 ± 0.03 | −0.35 ± 0.04 |
| After block 24 alone less without replay, block 24 | inhibitory in readout 0 | 102 | −0.04 | −0.10 | −0.82 | −0.52 | +0.57 | −2.06 | −2.38 ± 0.08 | −2.69 ± 0.10 |
| After block 24 alone less without replay, block 24 | D's own | 25 | −0.02 | +0.02 | −0.11 | −0.02 | +0.30 | −0.43 | −0.04 ± 0.02 | −0.11 ± 0.02 |
| After block 24 alone less without replay, block 24 | readout 1's pattern targets | 254 | −0.16 | +0.45 | −1.65 | −0.12 | +2.62 | −4.12 | −3.90 ± 0.18 | −3.21 ± 0.13 |
| After block 24 alone less without replay, block 24 | readout 1's other excitatory | 78 | −0.06 | 0.00 | −0.22 | −0.13 | +0.42 | −0.83 | −1.90 ± 0.08 | −0.46 ± 0.04 |
| After block 24 alone less without replay, block 24 | inhibitory in readout 1 | 102 | −0.11 | −0.12 | −0.39 | −0.51 | +0.57 | −1.70 | −3.00 ± 0.07 | −2.01 ± 0.11 |
| After block 6 alone less without replay, block 36 | C's own | 25 | 0.00 | −0.03 | −0.27 | −0.19 | +0.02 | −0.51 | −0.53 ± 0.03 | +0.01 ± 0.04 |
| After block 6 alone less without replay, block 36 | readout 0's pattern targets | 280 | +0.37 | −0.19 | −0.21 | −2.02 | +1.06 | −3.10 | −0.71 ± 0.15 | −2.23 ± 0.10 |
| After block 6 alone less without replay, block 36 | readout 0's other excitatory | 52 | 0.00 | −0.01 | +0.01 | +0.03 | +0.06 | −0.04 | +0.04 ± 0.02 | +0.01 ± 0.02 |
| After block 6 alone less without replay, block 36 | inhibitory in readout 0 | 102 | −0.04 | −0.02 | −0.77 | −0.39 | −0.04 | −1.18 | −0.55 ± 0.06 | −0.53 ± 0.03 |
| After block 6 alone less without replay, block 36 | D's own | 25 | −0.02 | +0.05 | −0.09 | +0.02 | +0.24 | −0.29 | −0.12 ± 0.01 | +0.05 ± 0.01 |
| After block 6 alone less without replay, block 36 | readout 1's pattern targets | 254 | −0.13 | +0.32 | −1.79 | +0.04 | +0.85 | −2.43 | −1.30 ± 0.12 | −0.87 ± 0.14 |
| After block 6 alone less without replay, block 36 | readout 1's other excitatory | 78 | −0.04 | 0.00 | −0.20 | −0.14 | +0.17 | −0.54 | −0.32 ± 0.08 | −0.43 ± 0.03 |
| After block 6 alone less without replay, block 36 | inhibitory in readout 1 | 102 | −0.13 | −0.12 | −0.33 | −0.33 | +0.09 | −1.01 | −0.63 ± 0.14 | −0.51 ± 0.17 |
| After block 12 alone less without replay, block 36 | C's own | 25 | +0.03 | −0.01 | −0.19 | −0.27 | +0.05 | −0.50 | −0.46 ± 0.02 | −0.08 ± 0.02 |
| After block 12 alone less without replay, block 36 | readout 0's pattern targets | 280 | +0.40 | −0.17 | −0.14 | −1.85 | +1.59 | −3.35 | −1.59 ± 0.21 | −1.55 ± 0.11 |
| After block 12 alone less without replay, block 36 | readout 0's other excitatory | 52 | 0.00 | −0.01 | −0.05 | +0.06 | +0.06 | −0.06 | +0.01 ± 0.01 | −0.06 ± 0.01 |
| After block 12 alone less without replay, block 36 | inhibitory in readout 0 | 102 | −0.06 | −0.02 | −0.76 | −0.40 | 0.00 | −1.25 | −0.72 ± 0.03 | −0.43 ± 0.09 |
| After block 12 alone less without replay, block 36 | D's own | 25 | −0.01 | +0.03 | −0.10 | 0.00 | +0.26 | −0.34 | +0.02 ± 0.02 | +0.02 ± 0.00 |
| After block 12 alone less without replay, block 36 | readout 1's pattern targets | 254 | −0.17 | +0.31 | −1.61 | −0.19 | +1.06 | −2.75 | −1.90 ± 0.21 | −0.55 ± 0.08 |
| After block 12 alone less without replay, block 36 | readout 1's other excitatory | 78 | −0.05 | 0.00 | −0.16 | −0.12 | +0.09 | −0.43 | −0.47 ± 0.05 | +0.14 ± 0.05 |
| After block 12 alone less without replay, block 36 | inhibitory in readout 1 | 102 | −0.13 | −0.12 | −0.38 | −0.37 | +0.09 | −1.11 | −1.37 ± 0.11 | −0.54 ± 0.12 |
| After block 18 alone less without replay, block 36 | C's own | 25 | +0.03 | −0.03 | −0.19 | −0.25 | +0.08 | −0.52 | −0.38 ± 0.03 | −0.27 ± 0.04 |
| After block 18 alone less without replay, block 36 | readout 0's pattern targets | 280 | +0.40 | −0.17 | −0.21 | −1.78 | +1.97 | −3.73 | −1.34 ± 0.15 | −1.05 ± 0.09 |
| After block 18 alone less without replay, block 36 | readout 0's other excitatory | 52 | 0.00 | −0.01 | −0.06 | 0.00 | +0.06 | −0.13 | +0.03 ± 0.01 | −0.03 ± 0.01 |
| After block 18 alone less without replay, block 36 | inhibitory in readout 0 | 102 | −0.04 | −0.04 | −0.77 | −0.51 | +0.10 | −1.46 | −0.88 ± 0.11 | −1.06 ± 0.14 |
| After block 18 alone less without replay, block 36 | D's own | 25 | −0.02 | +0.02 | −0.11 | −0.01 | +0.24 | −0.35 | +0.04 ± 0.02 | +0.05 ± 0.01 |
| After block 18 alone less without replay, block 36 | readout 1's pattern targets | 254 | −0.17 | +0.33 | −1.60 | −0.17 | +1.28 | −2.90 | −1.83 ± 0.14 | −0.37 ± 0.14 |
| After block 18 alone less without replay, block 36 | readout 1's other excitatory | 78 | −0.07 | 0.00 | −0.23 | −0.11 | +0.08 | −0.49 | −0.82 ± 0.10 | −0.17 ± 0.01 |
| After block 18 alone less without replay, block 36 | inhibitory in readout 1 | 102 | −0.14 | −0.10 | −0.32 | −0.39 | +0.20 | −1.15 | −1.38 ± 0.07 | −0.33 ± 0.09 |
| After block 24 alone less without replay, block 36 | C's own | 25 | +0.04 | −0.01 | −0.23 | −0.25 | +0.11 | −0.55 | −0.27 ± 0.01 | −0.09 ± 0.01 |
| After block 24 alone less without replay, block 36 | readout 0's pattern targets | 280 | +0.31 | −0.20 | −0.20 | −1.92 | +2.59 | −4.60 | −2.22 ± 0.13 | −1.93 ± 0.18 |
| After block 24 alone less without replay, block 36 | readout 0's other excitatory | 52 | 0.00 | −0.01 | −0.02 | −0.01 | +0.14 | −0.19 | +0.02 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 36 | inhibitory in readout 0 | 102 | −0.05 | −0.07 | −0.83 | −0.50 | +0.23 | −1.69 | −0.86 ± 0.13 | −0.99 ± 0.09 |
| After block 24 alone less without replay, block 36 | D's own | 25 | −0.02 | +0.02 | −0.11 | −0.02 | +0.24 | −0.37 | +0.04 ± 0.02 | 0.00 ± 0.01 |
| After block 24 alone less without replay, block 36 | readout 1's pattern targets | 254 | −0.14 | +0.41 | −1.62 | −0.12 | +1.70 | −3.19 | −1.42 ± 0.09 | −1.16 ± 0.10 |
| After block 24 alone less without replay, block 36 | readout 1's other excitatory | 78 | −0.03 | 0.00 | −0.22 | −0.13 | +0.19 | −0.57 | −0.69 ± 0.05 | −0.18 ± 0.02 |
| After block 24 alone less without replay, block 36 | inhibitory in readout 1 | 102 | −0.11 | −0.13 | −0.38 | −0.51 | +0.28 | −1.42 | −0.96 ± 0.10 | −0.53 ± 0.09 |

Of the eight classes' excitatory input less inhibitory input right after each of the four single replayed nights, 32 of the 32 lay below the
night without replay's. Readout 0's pattern targets' read −6.03, −5.82, −5.79 and −5.93 M after block 6, 12, 18 and 24 alone, where their
spikes on A's trials moved by −5.56, −5.33, −4.20 and −4.43; readout 0's inhibitory units' −2.05, −2.00, −2.02 and −2.06 M, their spikes
−2.09, −2.48, −2.21 and −2.38; readout 1's pattern targets' −4.32, −4.53, −4.41 and −4.12 M, their spikes on A's trials −3.02, −3.86, −4.07
and −3.90; readout 1's inhibitory units' −1.72, −1.70, −1.62 and −1.70 M, their spikes −2.71, −3.40, −3.45 and −3.00.

**A's margin**, its change over each night, from the input read:

| Night after block | Form | This night | A's margin, its change over the night, M |
| :--- | :--- | :--- | ---: |
| 6 | After block 6 alone | replays | +1.28 |
| 6 | Plain on all four nights (H-47) | replays | +1.28 |
| 6 | Without replay | without replay | +0.07 |
| 12 | After block 12 alone | replays | +1.41 |
| 12 | Plain on all four nights (H-47) | replays | +1.17 |
| 12 | Without replay | without replay | +0.07 |
| 18 | After block 18 alone | replays | +1.40 |
| 18 | Plain on all four nights (H-47) | replays | +1.12 |
| 18 | Without replay | without replay | +0.08 |
| 24 | After block 24 alone | replays | +1.43 |
| 24 | Plain on all four nights (H-47) | replays | +0.76 |
| 24 | Without replay | without replay | +0.04 |

A's margin rose by 1.28, 1.41, 1.40 and 1.43 M over the replayed night after block 6, 12, 18 and 24 alone, and rose by 1.17, 1.12 and 0.76
over H-47's later plain nights; of the night without replay's nights, the largest move was its night after block 18's, +0.08 M.

## 7. The nights and blocks after the replayed night

Each single night's difference from the night without replay right after its replayed night, each later night's and each stretch's change
in it (the form's change less the night without replay's), and the difference at block 36, in M:

| Form | Onto | Its difference from the night without replay, and each stage's change in it | A's kinds (classes 0, 1), M | B's kinds (2, 3), M | the odd places' excitatory (4, 6, 7), M | the even places' (5, 8, 9), M | other excitatory (10), M | inhibitory (11 to 13), M |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone | readout 0, the odd places | right after its night (block 6) | +0.33 | −0.27 | −1.17 | −2.90 | −0.01 | +4.96 |
| After block 6 alone | readout 0, the odd places | blocks 6 to 12, less without replay's | 0.00 | +0.04 | −0.01 | −0.02 | 0.00 | −1.02 |
| After block 6 alone | readout 0, the odd places | the night after block 12, less without replay's | −0.02 | −0.01 | +0.01 | +0.05 | +0.01 | −0.49 |
| After block 6 alone | readout 0, the odd places | blocks 12 to 18, less without replay's | +0.01 | −0.02 | 0.00 | −0.01 | 0.00 | −0.58 |
| After block 6 alone | readout 0, the odd places | the night after block 18, less without replay's | −0.02 | +0.02 | +0.02 | +0.18 | 0.00 | −0.37 |
| After block 6 alone | readout 0, the odd places | blocks 18 to 24, less without replay's | +0.04 | 0.00 | +0.02 | 0.00 | 0.00 | −0.45 |
| After block 6 alone | readout 0, the odd places | the night after block 24, less without replay's | −0.06 | 0.00 | −0.12 | +0.12 | 0.00 | −0.29 |
| After block 6 alone | readout 0, the odd places | blocks 24 to 36, less without replay's | +0.04 | 0.00 | 0.00 | 0.00 | 0.00 | −0.66 |
| After block 6 alone | readout 0, the odd places | at block 36 | +0.33 | −0.25 | −1.25 | −2.58 | 0.00 | +1.09 |
| After block 6 alone | readout 1, the even places | right after its night (block 6) | −0.33 | +0.27 | −2.69 | −0.52 | −0.01 | +3.93 |
| After block 6 alone | readout 1, the even places | blocks 6 to 12, less without replay's | 0.00 | −0.04 | +0.01 | +0.02 | 0.00 | −0.78 |
| After block 6 alone | readout 1, the even places | the night after block 12, less without replay's | +0.02 | +0.01 | +0.02 | −0.02 | 0.00 | −0.31 |
| After block 6 alone | readout 1, the even places | blocks 12 to 18, less without replay's | −0.01 | +0.02 | 0.00 | +0.01 | 0.00 | −0.43 |
| After block 6 alone | readout 1, the even places | the night after block 18, less without replay's | +0.02 | −0.02 | +0.18 | +0.03 | −0.01 | −0.25 |
| After block 6 alone | readout 1, the even places | blocks 18 to 24, less without replay's | −0.04 | 0.00 | −0.01 | 0.00 | 0.00 | −0.27 |
| After block 6 alone | readout 1, the even places | the night after block 24, less without replay's | +0.06 | 0.00 | +0.09 | +0.09 | −0.01 | −0.18 |
| After block 6 alone | readout 1, the even places | blocks 24 to 36, less without replay's | −0.04 | 0.00 | 0.00 | −0.01 | 0.00 | −0.35 |
| After block 6 alone | readout 1, the even places | at block 36 | −0.33 | +0.25 | −2.41 | −0.40 | −0.03 | +1.36 |
| After block 12 alone | readout 0, the odd places | right after its night (block 12) | +0.41 | −0.26 | −1.00 | −2.73 | −0.01 | +5.10 |
| After block 12 alone | readout 0, the odd places | blocks 12 to 18, less without replay's | −0.01 | −0.01 | −0.02 | −0.03 | 0.00 | −0.96 |
| After block 12 alone | readout 0, the odd places | the night after block 18, less without replay's | −0.02 | +0.03 | −0.04 | +0.19 | 0.00 | −0.52 |
| After block 12 alone | readout 0, the odd places | blocks 18 to 24, less without replay's | +0.01 | +0.01 | 0.00 | 0.00 | 0.00 | −0.67 |
| After block 12 alone | readout 0, the odd places | the night after block 24, less without replay's | −0.04 | +0.01 | −0.08 | +0.09 | 0.00 | −0.37 |
| After block 12 alone | readout 0, the odd places | blocks 24 to 36, less without replay's | +0.03 | +0.02 | 0.00 | +0.02 | 0.00 | −0.89 |
| After block 12 alone | readout 0, the odd places | at block 36 | +0.37 | −0.21 | −1.15 | −2.46 | −0.01 | +1.70 |
| After block 12 alone | readout 1, the even places | right after its night (block 12) | −0.41 | +0.26 | −2.65 | −0.77 | −0.02 | +3.91 |
| After block 12 alone | readout 1, the even places | blocks 12 to 18, less without replay's | +0.01 | +0.01 | +0.02 | +0.02 | 0.00 | −0.76 |
| After block 12 alone | readout 1, the even places | the night after block 18, less without replay's | +0.02 | −0.03 | +0.19 | −0.01 | −0.01 | −0.36 |
| After block 12 alone | readout 1, the even places | blocks 18 to 24, less without replay's | −0.01 | −0.01 | 0.00 | 0.00 | 0.00 | −0.42 |
| After block 12 alone | readout 1, the even places | the night after block 24, less without replay's | +0.04 | −0.01 | +0.18 | +0.10 | −0.01 | −0.26 |
| After block 12 alone | readout 1, the even places | blocks 24 to 36, less without replay's | −0.03 | −0.02 | 0.00 | −0.01 | 0.00 | −0.62 |
| After block 12 alone | readout 1, the even places | at block 36 | −0.37 | +0.21 | −2.26 | −0.67 | −0.04 | +1.50 |
| After block 18 alone | readout 0, the odd places | right after its night (block 18) | +0.40 | −0.26 | −1.13 | −2.69 | 0.00 | +5.05 |
| After block 18 alone | readout 0, the odd places | blocks 18 to 24, less without replay's | 0.00 | +0.04 | −0.01 | +0.01 | 0.00 | −1.05 |
| After block 18 alone | readout 0, the odd places | the night after block 24, less without replay's | −0.03 | −0.01 | −0.10 | +0.16 | 0.00 | −0.52 |
| After block 18 alone | readout 0, the odd places | blocks 24 to 36, less without replay's | +0.03 | −0.02 | +0.02 | −0.01 | 0.00 | −1.28 |
| After block 18 alone | readout 0, the odd places | at block 36 | +0.39 | −0.25 | −1.24 | −2.53 | 0.00 | +2.21 |
| After block 18 alone | readout 1, the even places | right after its night (block 18) | −0.40 | +0.26 | −2.49 | −0.72 | −0.02 | +3.96 |
| After block 18 alone | readout 1, the even places | blocks 18 to 24, less without replay's | 0.00 | −0.04 | +0.01 | 0.00 | 0.00 | −0.87 |
| After block 18 alone | readout 1, the even places | the night after block 24, less without replay's | +0.03 | +0.01 | +0.24 | +0.05 | −0.01 | −0.40 |
| After block 18 alone | readout 1, the even places | blocks 24 to 36, less without replay's | −0.03 | +0.02 | −0.02 | +0.01 | 0.00 | −0.90 |
| After block 18 alone | readout 1, the even places | at block 36 | −0.39 | +0.25 | −2.25 | −0.67 | −0.02 | +1.80 |
| After block 24 alone | readout 0, the odd places | right after its night (block 24) | +0.34 | −0.35 | −1.24 | −2.68 | −0.01 | +5.00 |
| After block 24 alone | readout 0, the odd places | blocks 24 to 36, less without replay's | −0.04 | +0.05 | −0.04 | 0.00 | 0.00 | −1.93 |
| After block 24 alone | readout 0, the odd places | at block 36 | +0.31 | −0.30 | −1.28 | −2.68 | −0.01 | +3.07 |
| After block 24 alone | readout 1, the even places | right after its night (block 24) | −0.34 | +0.35 | −2.36 | −0.79 | −0.03 | +3.91 |
| After block 24 alone | readout 1, the even places | blocks 24 to 36, less without replay's | +0.04 | −0.05 | +0.04 | 0.00 | 0.00 | −1.51 |
| After block 24 alone | readout 1, the even places | at block 36 | −0.31 | +0.30 | −2.32 | −0.79 | −0.03 | +2.40 |

Right after each single replayed night the inhibitory input onto each readout lay above the night without replay's by 3.91 to 5.10 M; at
block 36 above it by 1.09 to 3.07, having come back by 39 to 78 per cent of it (the least after block 24 alone onto readout 1, the most
after block 6 alone onto readout 0). Each stretch of blocks after the replayed night lowered it, by 0.27 to 1.93 M, and of the 6 later
nights in the single nights' forms, onto both readouts, 12 of their 12 changes lowered it. All excitatory input onto each readout lay below
the night without replay's by 3.17 to 4.02 M right after and below by 2.92 to 3.96 at block 36: 7 of the eight came back, by 1 to 13 per
cent, and 1 lay further from it, by 1 per cent (after block 24 alone onto readout 0).

The night without replay's own nights and stretches, by group, onto each readout:

| Stage | Onto | A's kinds (classes 0, 1), M | B's kinds (2, 3), M | the odd places' excitatory (4, 6, 7), M | the even places' (5, 8, 9), M | other excitatory (10), M | inhibitory (11 to 13), M |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| the night after block 6 | readout 0, the odd places | +0.02 | −0.02 | −0.02 | −0.11 | −0.03 | +0.29 |
| blocks 6 to 12 | readout 0, the odd places | −0.11 | +0.07 | −0.05 | +0.06 | 0.00 | +0.11 |
| the night after block 6 | readout 1, the even places | −0.02 | +0.02 | +0.07 | −0.20 | −0.02 | +0.27 |
| blocks 6 to 12 | readout 1, the even places | +0.11 | −0.07 | +0.06 | −0.06 | 0.00 | +0.16 |
| the night after block 12 | readout 0, the odd places | 0.00 | −0.04 | +0.04 | +0.03 | −0.02 | +0.26 |
| blocks 12 to 18 | readout 0, the odd places | −0.08 | +0.06 | −0.03 | +0.05 | 0.00 | +0.03 |
| the night after block 12 | readout 1, the even places | 0.00 | +0.04 | +0.21 | −0.06 | −0.02 | +0.25 |
| blocks 12 to 18 | readout 1, the even places | +0.08 | −0.06 | +0.03 | −0.05 | 0.00 | +0.16 |
| the night after block 18 | readout 0, the odd places | +0.02 | −0.02 | +0.07 | +0.01 | −0.01 | +0.29 |
| blocks 18 to 24 | readout 0, the odd places | −0.07 | +0.02 | −0.02 | +0.02 | 0.00 | +0.06 |
| the night after block 18 | readout 1, the even places | −0.02 | +0.02 | +0.16 | +0.02 | 0.00 | +0.30 |
| blocks 18 to 24 | readout 1, the even places | +0.07 | −0.02 | +0.02 | −0.02 | 0.00 | +0.23 |
| the night after block 24 | readout 0, the odd places | +0.03 | +0.01 | +0.18 | −0.03 | −0.01 | +0.34 |
| blocks 24 to 36 | readout 0, the odd places | −0.10 | +0.08 | −0.04 | +0.05 | 0.00 | +0.24 |
| the night after block 24 | readout 1, the even places | −0.03 | −0.01 | +0.06 | +0.12 | +0.01 | +0.40 |
| blocks 24 to 36 | readout 1, the even places | +0.10 | −0.08 | +0.04 | −0.05 | 0.00 | +0.55 |

**Right after and at block 36**, each group's difference:

| Form | Onto | A's kinds: right after, M | at block 36 | B's kinds: right after, M | at block 36 | The odd places' excitatory: right after, M | at block 36 | The even places' excitatory: right after, M | at block 36 | Other excitatory: right after, M | at block 36 | Inhibitory: right after, M | at block 36 |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone | readout 0, the odd places | +0.33 | +0.33 | −0.27 | −0.25 | −1.17 | −1.25 | −2.90 | −2.58 | −0.01 | 0.00 | +4.96 | +1.09 |
| After block 6 alone | readout 1, the even places | −0.33 | −0.33 | +0.27 | +0.25 | −2.69 | −2.41 | −0.52 | −0.40 | −0.01 | −0.03 | +3.93 | +1.36 |
| After block 12 alone | readout 0, the odd places | +0.41 | +0.37 | −0.26 | −0.21 | −1.00 | −1.15 | −2.73 | −2.46 | −0.01 | −0.01 | +5.10 | +1.70 |
| After block 12 alone | readout 1, the even places | −0.41 | −0.37 | +0.26 | +0.21 | −2.65 | −2.26 | −0.77 | −0.67 | −0.02 | −0.04 | +3.91 | +1.50 |
| After block 18 alone | readout 0, the odd places | +0.40 | +0.39 | −0.26 | −0.25 | −1.13 | −1.24 | −2.69 | −2.53 | 0.00 | 0.00 | +5.05 | +2.21 |
| After block 18 alone | readout 1, the even places | −0.40 | −0.39 | +0.26 | +0.25 | −2.49 | −2.25 | −0.72 | −0.67 | −0.02 | −0.02 | +3.96 | +1.80 |
| After block 24 alone | readout 0, the odd places | +0.34 | +0.31 | −0.35 | −0.30 | −1.24 | −1.28 | −2.68 | −2.68 | −0.01 | −0.01 | +5.00 | +3.07 |
| After block 24 alone | readout 1, the even places | −0.34 | −0.31 | +0.35 | +0.30 | −2.36 | −2.32 | −0.79 | −0.79 | −0.03 | −0.03 | +3.91 | +2.40 |

From right after each single replayed night to block 36, A's and B's kinds' and each place group's difference onto each readout made its
largest move in the odd places' excitatory units' onto readout 1 after block 12 alone, +0.39 M, where the inhibitory input's difference
moved by −3.86 to −1.51 M.

**At block 36**, the units that carried half of each readout's change there (section 4), against the rest:

| Comparison | Trials | Readout | Units carrying half its change (section 3 or 4) | Their spikes' difference a unit | The rest's | A's kinds: their input difference a unit | the rest's | B's kinds: theirs | the rest's | The odd places' excitatory: theirs | the rest's | The even places': theirs | the rest's | Inhibitory: theirs | the rest's | Excitatory less inhibitory: theirs | the rest's |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | 3 | −0.38 | 0.00 | −28 101 | +905 | +97 | −547 | +4 462 | −2 772 | +8 130 | −5 708 | −7 824 | +2 452 | −7 588 | −10 563 |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | 5 | −0.28 | 0.00 | −15 730 | −550 | +253 | +546 | −15 198 | −5 142 | −4 789 | −836 | −5 116 | +3 041 | −30 347 | −9 086 |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | 3 | −0.53 | 0.00 | +3 350 | +698 | −19 652 | −417 | −44 953 | −2 447 | −6 124 | −5 614 | −29 027 | +2 591 | −38 352 | −10 361 |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | 4 | −0.24 | 0.00 | +2 339 | −742 | −6 722 | +606 | −7 311 | −5 233 | −6 284 | −831 | −8 759 | +3 055 | −9 268 | −9 318 |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | 6 | −0.24 | 0.00 | −17 139 | +1 043 | +268 | −475 | +2 010 | −2 559 | −6 399 | −5 355 | −3 642 | +3 799 | −17 617 | −11 157 |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | 8 | −0.24 | 0.00 | −11 024 | −624 | +311 | +468 | −16 465 | −4 718 | −12 011 | −1 279 | −2 614 | +3 370 | −36 575 | −9 601 |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | 3 | −0.36 | 0.00 | +142 | +810 | −7 740 | −418 | −29 452 | −2 322 | −12 218 | −5 324 | −17 716 | +3 842 | −31 553 | −11 108 |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | 3 | −0.16 | 0.00 | −166 | −809 | −10 419 | +537 | −2 469 | −4 938 | −7 414 | −1 427 | +2 021 | +3 274 | −22 489 | −9 989 |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | 8 | −0.18 | 0.00 | −9 124 | +1 030 | −664 | −541 | −8 756 | −2 587 | −4 148 | −5 544 | −296 | +4 900 | −22 395 | −12 549 |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | 9 | −0.23 | 0.00 | −8 719 | −696 | +514 | +544 | −13 226 | −4 745 | −11 303 | −1 271 | +1 351 | +3 984 | −34 085 | −10 201 |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | 6 | −0.23 | 0.00 | +2 427 | +832 | −10 535 | −411 | −18 634 | −2 483 | −15 012 | −5 394 | −6 506 | +4 960 | −35 248 | −12 422 |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | 4 | −0.12 | 0.00 | −871 | −853 | −2 565 | +570 | −21 852 | −4 762 | −1 311 | −1 469 | +1 980 | +3 949 | −29 057 | −10 508 |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | 10 | −0.18 | 0.00 | −12 126 | +950 | +90 | −669 | +1 330 | −2 880 | −64 | −5 962 | −519 | +6 844 | −10 253 | −15 429 |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | 8 | −0.20 | 0.00 | −8 127 | −533 | +943 | +647 | −12 927 | −4 924 | −9 080 | −1 591 | +7 240 | +5 192 | −36 432 | −11 657 |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | 7 | −0.24 | 0.00 | +2 191 | +642 | −13 211 | −458 | −4 750 | −2 758 | −18 075 | −5 644 | +346 | +6 782 | −34 836 | −15 014 |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | 5 | −0.19 | 0.00 | −1 819 | −652 | −5 391 | +719 | −5 401 | −5 060 | −11 288 | −1 616 | +141 | +5 284 | −24 078 | −11 957 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | 7 | −0.67 | −0.01 | −19 263 | +2 946 | −2 670 | −1 922 | −7 842 | −5 306 | −74 371 | −16 872 | −11 963 | +22 843 | −92 968 | −44 059 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | 9 | −0.61 | −0.01 | −45 632 | −1 747 | +1 006 | +1 952 | −30 744 | −15 000 | −7 041 | −3 876 | +7 043 | +16 595 | −89 453 | −35 396 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | 6 | −0.79 | −0.01 | +7 692 | +2 540 | −45 540 | −1 356 | −31 820 | −4 994 | −30 216 | −17 583 | −5 786 | +22 684 | −94 096 | −44 152 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | 5 | −0.60 | −0.01 | +2 756 | −2 667 | −14 503 | +2 115 | −36 297 | −15 078 | −26 286 | −3 692 | −13 457 | +16 737 | −60 880 | −36 187 |

At block 36, in the 20 sets whose readout difference lies beyond 2 SE: on A's trials the units that carried half of each readout's change
had an input difference from A's kinds of −45 632 to −8 127 a unit, below 0 in 10 of 10 sets, where the rest of their readout had −1 747 to
+2 946; on B's trials theirs from B's kinds −45 540 to −2 565, below 0 in 10 of 10, and the rest's −1 356 to +2 115. Theirs from the trial's
stimulus's kinds lay below the rest's in 20 of the 20 sets. Their excitatory input less inhibitory input difference read −94 096 to −7 588 a
unit and the rest's −44 152 to −9 086, theirs below the rest's in 17 of the 20; their excitatory input difference lay below the rest's in
20, and their inhibitory input difference, −29 027 to +7 240 a unit against the rest's +2 452 to +22 843, above the rest's in 1. Their
spikes' difference a unit read −0.79 to −0.12 and the rest's −0.01 to 0.00.

| Comparison | Trials | Readout | Its spikes' difference (section 3 or 4) | A's kinds | B's kinds | The odd places' excitatory | The even places' | Other excitatory | Inhibitory | All excitatory | Excitatory less inhibitory |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | −1.75 | +0.76 | −0.05 | −0.01 | +0.03 | −0.01 | +0.14 | +0.20 | +0.12 |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | −2.38 | +0.68 | +0.01 | +0.18 | +0.16 | 0.00 | +0.18 | +0.37 | +0.26 |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | −2.74 | −0.04 | +0.71 | +0.28 | +0.09 | −0.16 | +0.31 | +0.38 | +0.20 |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | −1.76 | −0.06 | +0.47 | +0.05 | +0.16 | +0.01 | +0.20 | +0.24 | +0.12 |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | −2.76 | +0.64 | −0.03 | 0.00 | +0.06 | −0.03 | +0.14 | +0.19 | +0.10 |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | −3.71 | +0.65 | +0.03 | +0.23 | +0.23 | −0.01 | +0.10 | +0.43 | +0.33 |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | −2.11 | −0.01 | +0.63 | +0.21 | +0.12 | +0.11 | +0.24 | +0.33 | +0.17 |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | −0.93 | +0.02 | +0.52 | +0.09 | +0.11 | +0.02 | −0.01 | +0.23 | +0.22 |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | −2.57 | +0.62 | −0.02 | +0.06 | +0.02 | −0.03 | +0.11 | +0.19 | +0.11 |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | −3.99 | +0.66 | −0.04 | +0.22 | +0.15 | −0.01 | +0.06 | +0.37 | +0.29 |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | −2.42 | 0.00 | +0.58 | +0.21 | +0.19 | −0.04 | +0.21 | +0.34 | +0.18 |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | −0.83 | −0.05 | +0.38 | +0.16 | +0.16 | +0.04 | +0.05 | +0.28 | +0.22 |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | −3.32 | +0.62 | 0.00 | −0.02 | +0.01 | −0.01 | +0.15 | +0.15 | +0.04 |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | −3.04 | +0.61 | −0.04 | +0.22 | +0.19 | −0.02 | +0.01 | +0.36 | +0.31 |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | −3.02 | −0.02 | +0.73 | +0.06 | +0.21 | +0.22 | +0.14 | +0.29 | +0.17 |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | −1.87 | 0.00 | +0.50 | +0.05 | +0.17 | +0.01 | +0.07 | +0.27 | +0.19 |

At block 36, under each single night, over the 16 sets the correlation of a unit's input difference from the trial's stimulus's kinds with
its spikes' difference read +0.38 to +0.76, 16 of the 16 above 0: A's kinds' on A's trials +0.61 to +0.76 and B's kinds' on B's trials +0.38
to +0.73; from the other stimulus's kinds −0.06 to +0.03. Of its excitatory input less inhibitory input difference it read +0.04 to +0.33
(after block 24 alone at block 36, readout 0 on A's trials the least, after block 12 alone at block 36, readout 1 on A's trials the most),
16 of the 16 above 0; of its inhibitory input difference −0.01 to +0.31, 15 above 0; of its excitatory input difference +0.15 to +0.43, 16
above 0; of the odd places' excitatory −0.02 to +0.28 and of the even places' +0.01 to +0.23.

The gain after the night after block 24 and at block 36:

| Form | Gain after the night after block 24 | At block 36 |
| :--- | ---: | ---: |
| Without replay | 1.712 | 1.669 |
| After block 6 alone | 1.719 | 1.679 |
| Plain on all four nights (H-47) | 1.647 | 1.695 |
| After block 12 alone | 1.718 | 1.677 |
| After block 18 alone | 1.719 | 1.677 |
| After block 24 alone | 1.601 | 1.679 |

## 8. H-47's plain night on all four nights

Its inputs' difference from the night without replay at blocks 6 to 24, its nights so far together, and at block 36, in M:

| Block | Onto | A's kinds (classes 0, 1), M | B's kinds (2, 3), M | the odd places' excitatory (4, 6, 7), M | the even places' (5, 8, 9), M | other excitatory (10), M | inhibitory (11 to 13), M |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | readout 0, the odd places | +0.33 | −0.27 | −1.17 | −2.90 | −0.01 | +4.96 |
| 6 | readout 1, the even places | −0.33 | +0.27 | −2.69 | −0.52 | −0.01 | +3.93 |
| 12 | readout 0, the odd places | +0.65 | −0.47 | −1.82 | −5.12 | −0.01 | +7.99 |
| 12 | readout 1, the even places | −0.65 | +0.47 | −4.70 | −1.12 | −0.02 | +6.38 |
| 18 | readout 0, the odd places | +0.94 | −0.74 | −2.25 | −6.80 | −0.02 | +10.44 |
| 18 | readout 1, the even places | −0.94 | +0.74 | −6.11 | −1.53 | −0.03 | +8.09 |
| 24 | readout 0, the odd places | +1.18 | −0.86 | −2.42 | −8.12 | −0.03 | +12.38 |
| 24 | readout 1, the even places | −1.18 | +0.86 | −7.06 | −1.83 | −0.06 | +9.26 |
| 36 | readout 0, the odd places | +1.20 | −0.89 | −2.45 | −8.15 | −0.03 | +10.24 |
| 36 | readout 1, the even places | −1.20 | +0.89 | −7.03 | −1.81 | −0.06 | +7.53 |

Under H-47's plain night, of the inhibitory input's, each place group's and A's and B's kinds' differences onto the two readouts, 10 of 10
grew in magnitude with each night from block 6 to block 24. Less the night without replay's, at blocks 6, 12, 18, 24 and 36: the inhibitory
input onto readout 0 read +4.96, +7.99, +10.44, +12.38, +10.24 M and onto readout 1 +3.93, +6.38, +8.09, +9.26, +7.53; the odd places'
excitatory input onto readout 1 −2.69, −4.70, −6.11, −7.06, −7.03, and the even places' onto readout 0 −2.90, −5.12, −6.80, −8.12, −8.15;
A's kinds' input onto readout 0 +0.33, +0.65, +0.94, +1.18, +1.20.

The units that carried half of each readout's change under it at blocks 6 to 24 (section 3), against the rest:

| Comparison | Trials | Readout | Units carrying half its change (section 3 or 4) | Their spikes' difference a unit | The rest's | A's kinds: their input difference a unit | the rest's | B's kinds: theirs | the rest's | The odd places' excitatory: theirs | the rest's | The even places': theirs | the rest's | Inhibitory: theirs | the rest's | Excitatory less inhibitory: theirs | the rest's |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | 13 | −0.32 | −0.01 | −12 138 | +1 104 | −866 | −578 | −7 242 | −2 415 | −17 962 | −5 987 | +366 | +11 109 | −38 428 | −19 010 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | 16 | −0.24 | −0.01 | −11 611 | −336 | −357 | +620 | −15 247 | −5 524 | −8 255 | −881 | +4 118 | +8 712 | −39 173 | −14 870 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | 18 | −0.16 | −0.01 | +1 111 | +714 | −8 578 | −260 | −4 955 | −2 454 | −14 408 | −5 997 | +3 126 | +11 118 | −29 959 | −19 135 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | 14 | −0.21 | −0.01 | −1 489 | −705 | −10 028 | +920 | −10 887 | −5 705 | −6 401 | −973 | +2 483 | +8 743 | −31 141 | −15 232 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | 10 | −0.55 | −0.01 | −21 223 | +1 912 | −905 | −1 024 | −18 442 | −3 653 | −34 990 | −10 629 | +216 | +17 801 | −75 477 | −31 227 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | 14 | −0.36 | −0.01 | −22 956 | −730 | −410 | +1 067 | −21 369 | −9 900 | −14 333 | −2 077 | +12 505 | +13 954 | −71 574 | −25 636 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | 12 | −0.30 | −0.01 | +2 369 | +1 382 | −17 398 | −582 | −13 237 | −3 727 | −31 573 | −10 612 | +2 959 | +17 806 | −62 702 | −31 372 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | 10 | −0.38 | −0.01 | −2 119 | −1 392 | −17 139 | +1 426 | −22 002 | −9 988 | −16 786 | −2 132 | +2 184 | +14 171 | −60 184 | −26 299 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | 9 | −0.65 | −0.01 | −22 936 | +2 548 | −1 279 | −1 617 | −26 431 | −4 464 | −48 474 | −14 146 | −2 329 | +23 245 | −96 805 | −40 974 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | 13 | −0.47 | −0.01 | −34 860 | −1 092 | +1 579 | +1 611 | −22 665 | −13 047 | −6 688 | −3 241 | +16 389 | +17 672 | −79 024 | −33 518 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | 10 | −0.50 | −0.01 | +3 027 | +2 027 | −25 881 | −1 070 | −26 979 | −4 403 | −34 715 | −14 376 | +996 | +23 228 | −85 659 | −41 098 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | 8 | −0.46 | −0.01 | +1 030 | −2 103 | −16 901 | +1 939 | −25 885 | −13 096 | −16 107 | −3 112 | −222 | +17 953 | −57 620 | −34 402 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | 10 | −0.64 | −0.01 | −26 603 | +3 226 | −1 574 | −1 876 | −7 572 | −5 222 | −47 934 | −17 018 | −4 519 | +27 669 | −79 716 | −48 623 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | 12 | −0.60 | −0.02 | −43 531 | −1 477 | +2 029 | +1 865 | −26 066 | −15 097 | −13 983 | −3 729 | +23 065 | +20 096 | −104 616 | −38 664 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | 9 | −0.69 | −0.01 | +3 144 | +2 565 | −34 861 | −1 210 | −29 686 | −4 785 | −49 344 | −17 059 | −3 713 | +27 582 | −107 642 | −48 134 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | 8 | −0.56 | −0.01 | +2 107 | −2 659 | −21 231 | +2 279 | −27 132 | −15 176 | −19 142 | −3 729 | −3 640 | +20 596 | −61 770 | −40 009 |

Under H-47's plain night at blocks 6 to 24, in the 16 sets whose readout difference lies beyond 2 SE: on A's trials the units that carried
half of each readout's change had an input difference from A's kinds of −43 531 to −11 611 a unit, below 0 in 8 of 8 sets, where the rest of
their readout had −1 477 to +3 226; on B's trials theirs from B's kinds −34 861 to −8 578, below 0 in 8 of 8, and the rest's −1 210 to
+2 279. Theirs from the trial's stimulus's kinds lay below the rest's in 16 of the 16 sets. Their excitatory input less inhibitory input
difference read −107 642 to −29 959 a unit and the rest's −48 623 to −14 870, theirs below the rest's in 16 of the 16; their excitatory
input difference lay below the rest's in 16, and their inhibitory input difference, −4 519 to +23 065 a unit against the rest's +8 712 to
+27 669, above the rest's in 1. Their spikes' difference a unit read −0.69 to −0.16 and the rest's −0.02 to −0.01.

The correlation of the units' input difference with their spikes' difference under it at blocks 6 to 36:

| Comparison | Trials | Readout | Its spikes' difference (section 3 or 4) | A's kinds | B's kinds | The odd places' excitatory | The even places' | Other excitatory | Inhibitory | All excitatory | Excitatory less inhibitory |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | −8.32 | +0.58 | +0.02 | +0.09 | +0.34 | −0.12 | +0.18 | +0.43 | +0.24 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | −7.48 | +0.83 | +0.06 | +0.22 | +0.16 | −0.08 | +0.06 | +0.41 | +0.33 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | −5.66 | −0.02 | +0.71 | +0.11 | +0.16 | +0.03 | +0.17 | +0.29 | +0.13 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | −5.87 | +0.04 | +0.53 | +0.12 | +0.14 | −0.06 | +0.13 | +0.32 | +0.20 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | −10.56 | +0.58 | +0.03 | +0.10 | +0.34 | −0.17 | +0.17 | +0.44 | +0.26 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | −9.69 | +0.85 | +0.06 | +0.17 | +0.14 | −0.02 | +0.07 | +0.42 | +0.31 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | −7.19 | −0.03 | +0.76 | +0.12 | +0.18 | 0.00 | +0.15 | +0.31 | +0.16 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | −7.16 | +0.01 | +0.54 | +0.11 | +0.21 | −0.02 | +0.15 | +0.35 | +0.21 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | −11.22 | +0.59 | +0.02 | +0.06 | +0.29 | −0.01 | +0.18 | +0.39 | +0.19 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | −11.90 | +0.85 | +0.04 | +0.14 | +0.14 | −0.01 | +0.05 | +0.43 | +0.33 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | −9.78 | −0.03 | +0.74 | +0.14 | +0.19 | +0.02 | +0.15 | +0.34 | +0.17 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | −7.06 | −0.04 | +0.48 | +0.11 | +0.19 | 0.00 | +0.15 | +0.31 | +0.16 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | −12.48 | +0.61 | +0.02 | 0.00 | +0.26 | +0.10 | +0.20 | +0.35 | +0.13 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | −13.98 | +0.84 | +0.04 | +0.12 | +0.14 | −0.01 | +0.04 | +0.43 | +0.33 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | −11.46 | −0.03 | +0.76 | +0.13 | +0.17 | +0.06 | +0.15 | +0.36 | +0.17 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | −8.36 | −0.07 | +0.54 | +0.08 | +0.16 | 0.00 | +0.17 | +0.28 | +0.12 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | −8.81 | +0.55 | +0.03 | +0.01 | +0.29 | +0.12 | +0.21 | +0.35 | +0.14 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | −10.20 | +0.83 | +0.01 | +0.14 | +0.11 | −0.02 | +0.04 | +0.41 | +0.32 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | −9.13 | −0.03 | +0.77 | +0.14 | +0.15 | +0.03 | +0.18 | +0.35 | +0.16 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | −5.33 | −0.07 | +0.49 | +0.08 | +0.16 | 0.00 | +0.18 | +0.27 | +0.10 |

Under H-47's plain night at blocks 6 to 36, over the 20 sets the correlation of a unit's input difference from the trial's stimulus's kinds
with its spikes' difference read +0.48 to +0.85, 20 of the 20 above 0: A's kinds' on A's trials +0.55 to +0.85 and B's kinds' on B's trials
+0.48 to +0.77; from the other stimulus's kinds −0.07 to +0.06. Of its excitatory input less inhibitory input difference it read +0.10 to
+0.33 (H-47's plain night at block 36, readout 1 on B's trials the least, H-47's plain night at block 24, readout 1 on A's trials the most),
20 of the 20 above 0; of its inhibitory input difference +0.04 to +0.21, 20 above 0; of its excitatory input difference +0.27 to +0.44, 20
above 0; of the odd places' excitatory 0.00 to +0.22 and of the even places' +0.11 to +0.34.

## 9. The two cells side by side

Each reading above beside the same reading of the image from the assignment, the same form, night and checkpoint, each readout named by the
stimulus whose answer it is. **The readouts' units by class** on the two cells, from each start image:

| The readout | Class | This cell: places | units | The image from the assignment: places | units |
| :--- | :--- | ---: | ---: | ---: | ---: |
| A's answer's readout | C's own | 5 | 25 | – | – |
| A's answer's readout | readout 0's pattern targets | 1, 3, 5, 7, 13, 15, 17 | 280 | – | – |
| A's answer's readout | readout 0's other excitatory | 1, 3, 5, 7, 13, 15, 17 | 52 | – | – |
| A's answer's readout | inhibitory in readout 0 | 9, 19 | 102 | – | – |
| A's answer's readout | D's own | – | – | 16 | 25 |
| A's answer's readout | readout 1's pattern targets | – | – | 2, 6, 8, 10, 12, 16, 18 | 252 |
| A's answer's readout | readout 1's other excitatory | – | – | 2, 6, 8, 10, 12, 16, 18 | 80 |
| A's answer's readout | inhibitory in readout 1 | – | – | 4, 14 | 102 |
| B's answer's readout | D's own | 16 | 25 | – | – |
| B's answer's readout | readout 1's pattern targets | 2, 6, 8, 10, 12, 16, 18 | 254 | – | – |
| B's answer's readout | readout 1's other excitatory | 2, 6, 8, 10, 12, 16, 18 | 78 | – | – |
| B's answer's readout | inhibitory in readout 1 | 4, 14 | 102 | – | – |
| B's answer's readout | C's own | – | – | 5 | 25 |
| B's answer's readout | readout 0's pattern targets | – | – | 1, 3, 5, 7, 13, 15, 17 | 245 |
| B's answer's readout | readout 0's other excitatory | – | – | 1, 3, 5, 7, 13, 15, 17 | 87 |
| B's answer's readout | inhibitory in readout 0 | – | – | 9, 19 | 102 |

The two cells' readouts hold the same 918 units by id: C's own, D's own and the inhibitory units are the same units on both, and 239 units
are a pattern target on one cell and another excitatory unit of the same readout on the other. A's answer's readout holds C's own here and
D's own on the image from the assignment.

**A's count less the night without replay**, with H-47's paired SE, on each cell (this cell's from section 1, the image from the
assignment's as exploration 033 gives it):

| Form | Block | This cell: A's count less without replay | on A's trials | on B's trials | The image from the assignment: A's count less without replay | on A's trials | on B's trials |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone | 6 | −2.6 ± 1.35 | −2.1 ± 0.99 | −0.5 ± 1.17 | +0.3 ± 0.64 | +2.5 ± 0.77 | −2.3 ± 0.71 |
| After block 6 alone | 36 | +3.1 ± 1.84 | +0.7 ± 0.86 | +2.4 ± 1.06 | −2.0 ± 0.88 | −1.0 ± 0.63 | −1.0 ± 0.71 |
| After block 12 alone | 12 | +0.8 ± 1.09 | +0.5 ± 0.83 | +0.2 ± 0.60 | −7.3 ± 0.88 | −5.2 ± 0.70 | −2.1 ± 0.59 |
| After block 12 alone | 36 | +3.8 ± 0.78 | +0.7 ± 0.29 | +3.1 ± 1.02 | −2.1 ± 1.78 | −1.2 ± 0.90 | −0.9 ± 1.04 |
| After block 18 alone | 18 | +9.1 ± 0.92 | +6.0 ± 0.68 | +3.1 ± 1.02 | −12.1 ± 0.63 | −8.1 ± 0.66 | −4.0 ± 0.57 |
| After block 18 alone | 36 | +5.4 ± 1.17 | +1.7 ± 0.54 | +3.7 ± 0.86 | −5.5 ± 0.95 | −4.1 ± 0.51 | −1.4 ± 0.88 |
| After block 24 alone | 24 | +5.4 ± 2.02 | +1.9 ± 0.99 | +3.5 ± 1.23 | −12.3 ± 0.58 | −10.0 ± 0.33 | −2.3 ± 0.62 |
| After block 24 alone | 36 | +1.9 ± 1.18 | −0.8 ± 0.34 | +2.7 ± 0.89 | −4.6 ± 0.73 | −5.5 ± 0.75 | +0.9 ± 0.13 |
| Plain on all four nights (H-47) | 6 | −2.6 ± 1.35 | −2.1 ± 0.99 | −0.5 ± 1.17 | +0.3 ± 0.64 | +2.5 ± 0.77 | −2.3 ± 0.71 |
| Plain on all four nights (H-47) | 12 | −2.9 ± 1.19 | −3.2 ± 0.72 | +0.3 ± 0.77 | −0.6 ± 1.03 | +0.4 ± 1.31 | −1.0 ± 1.51 |
| Plain on all four nights (H-47) | 18 | +8.3 ± 1.70 | +0.7 ± 0.25 | +7.6 ± 1.52 | −5.9 ± 1.53 | −3.6 ± 0.86 | −2.4 ± 1.34 |
| Plain on all four nights (H-47) | 24 | +9.4 ± 1.58 | +1.6 ± 1.33 | +7.8 ± 1.47 | −5.9 ± 1.85 | −3.9 ± 0.78 | −2.0 ± 1.34 |
| Plain on all four nights (H-47) | 36 | +11.3 ± 0.96 | +1.5 ± 0.74 | +9.7 ± 0.98 | +1.8 ± 1.79 | +0.2 ± 0.76 | +1.6 ± 1.50 |

Right after the nights after block 6, 12, 18 and 24 alone A's count less the night without replay's read −2.6 ± 1.35, +0.8 ± 1.09,
+9.1 ± 0.92 (above beyond 2 SE) and +5.4 ± 2.02 (above beyond 2 SE) here and +0.3 ± 0.64, −7.3 ± 0.88 (below beyond 2 SE), −12.1 ± 0.63
(below beyond 2 SE) and −12.3 ± 0.58 (below beyond 2 SE) on the image from the assignment; at block 36 +3.1 ± 1.84, +3.8 ± 0.78 (above
beyond 2 SE), +5.4 ± 1.17 (above beyond 2 SE) and +1.9 ± 1.18 here and −2.0 ± 0.88 (below beyond 2 SE), −2.1 ± 1.78, −5.5 ± 0.95 (below
beyond 2 SE) and −4.6 ± 0.73 (below beyond 2 SE) there.

**Each readout's spikes' difference**, the units moved and the fewest to half of it, right after each replayed night and at block 36:

| Comparison | Trials | The readout | This cell: its difference | units moved down, up | fewest to half | The image from the assignment: its difference | units moved down, up | fewest to half |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | A's answer's readout | −8.32 ± 0.19 | 84, 0 | 13 | −7.45 ± 0.20 | 130, 4 | 16 |
| After block 6 alone less without replay, block 6 | A's trials | B's answer's readout | −7.48 ± 0.10 | 105, 1 | 16 | −7.92 ± 0.21 | 126, 0 | 15 |
| After block 6 alone less without replay, block 6 | B's trials | A's answer's readout | −5.66 ± 0.27 | 81, 3 | 18 | −5.47 ± 0.19 | 99, 4 | 15 |
| After block 6 alone less without replay, block 6 | B's trials | B's answer's readout | −5.87 ± 0.15 | 80, 3 | 14 | −6.27 ± 0.16 | 109, 3 | 18 |
| After block 12 alone less without replay, block 12 | A's trials | A's answer's readout | −8.48 ± 0.16 | 118, 3 | 15 | −9.19 ± 0.20 | 117, 2 | 18 |
| After block 12 alone less without replay, block 12 | A's trials | B's answer's readout | −9.17 ± 0.20 | 118, 2 | 19 | −7.73 ± 0.22 | 125, 0 | 15 |
| After block 12 alone less without replay, block 12 | B's trials | A's answer's readout | −6.16 ± 0.15 | 97, 0 | 19 | −5.59 ± 0.14 | 108, 1 | 19 |
| After block 12 alone less without replay, block 12 | B's trials | B's answer's readout | −6.08 ± 0.19 | 100, 2 | 15 | −6.49 ± 0.24 | 102, 3 | 17 |
| After block 18 alone less without replay, block 18 | A's trials | A's answer's readout | −7.07 ± 0.15 | 126, 0 | 17 | −10.19 ± 0.24 | 136, 2 | 19 |
| After block 18 alone less without replay, block 18 | A's trials | B's answer's readout | −9.49 ± 0.19 | 145, 0 | 19 | −7.23 ± 0.14 | 134, 0 | 17 |
| After block 18 alone less without replay, block 18 | B's trials | A's answer's readout | −6.79 ± 0.22 | 129, 0 | 20 | −5.31 ± 0.21 | 107, 3 | 15 |
| After block 18 alone less without replay, block 18 | B's trials | B's answer's readout | −5.48 ± 0.08 | 115, 1 | 22 | −6.74 ± 0.20 | 116, 2 | 18 |
| After block 24 alone less without replay, block 24 | A's trials | A's answer's readout | −7.52 ± 0.18 | 102, 0 | 17 | −9.75 ± 0.22 | 133, 1 | 19 |
| After block 24 alone less without replay, block 24 | A's trials | B's answer's readout | −8.83 ± 0.24 | 135, 3 | 19 | −6.03 ± 0.18 | 124, 1 | 17 |
| After block 24 alone less without replay, block 24 | B's trials | A's answer's readout | −7.46 ± 0.28 | 118, 2 | 17 | −6.21 ± 0.15 | 109, 1 | 13 |
| After block 24 alone less without replay, block 24 | B's trials | B's answer's readout | −5.79 ± 0.18 | 114, 4 | 17 | −6.77 ± 0.17 | 95, 5 | 14 |
| Plain on all four nights less without replay, block 6 | A's trials | A's answer's readout | −8.32 ± 0.19 | 84, 0 | 13 | −7.45 ± 0.20 | 130, 4 | 16 |
| Plain on all four nights less without replay, block 6 | A's trials | B's answer's readout | −7.48 ± 0.10 | 105, 1 | 16 | −7.92 ± 0.21 | 126, 0 | 15 |
| Plain on all four nights less without replay, block 6 | B's trials | A's answer's readout | −5.66 ± 0.27 | 81, 3 | 18 | −5.47 ± 0.19 | 99, 4 | 15 |
| Plain on all four nights less without replay, block 6 | B's trials | B's answer's readout | −5.87 ± 0.15 | 80, 3 | 14 | −6.27 ± 0.16 | 109, 3 | 18 |
| Plain on all four nights less without replay, block 12 | A's trials | A's answer's readout | −10.56 ± 0.20 | 101, 3 | 10 | −9.92 ± 0.31 | 109, 8 | 14 |
| Plain on all four nights less without replay, block 12 | A's trials | B's answer's readout | −9.69 ± 0.22 | 112, 4 | 14 | −10.14 ± 0.22 | 113, 3 | 13 |
| Plain on all four nights less without replay, block 12 | B's trials | A's answer's readout | −7.19 ± 0.16 | 101, 3 | 12 | −7.39 ± 0.23 | 101, 5 | 13 |
| Plain on all four nights less without replay, block 12 | B's trials | B's answer's readout | −7.16 ± 0.19 | 92, 7 | 10 | −7.91 ± 0.33 | 85, 5 | 13 |
| Plain on all four nights less without replay, block 18 | A's trials | A's answer's readout | −11.22 ± 0.24 | 81, 16 | 9 | −13.12 ± 0.15 | 115, 13 | 15 |
| Plain on all four nights less without replay, block 18 | A's trials | B's answer's readout | −11.90 ± 0.23 | 94, 6 | 13 | −11.94 ± 0.26 | 114, 5 | 12 |
| Plain on all four nights less without replay, block 18 | B's trials | A's answer's readout | −9.78 ± 0.34 | 96, 6 | 10 | −8.71 ± 0.21 | 95, 6 | 11 |
| Plain on all four nights less without replay, block 18 | B's trials | B's answer's readout | −7.06 ± 0.13 | 76, 21 | 8 | −9.54 ± 0.58 | 89, 18 | 12 |
| Plain on all four nights less without replay, block 24 | A's trials | A's answer's readout | −12.48 ± 0.20 | 86, 16 | 10 | −14.75 ± 0.25 | 105, 37 | 14 |
| Plain on all four nights less without replay, block 24 | A's trials | B's answer's readout | −13.98 ± 0.41 | 100, 9 | 12 | −13.23 ± 0.16 | 121, 21 | 11 |
| Plain on all four nights less without replay, block 24 | B's trials | A's answer's readout | −11.46 ± 0.41 | 81, 8 | 9 | −10.20 ± 0.17 | 94, 23 | 10 |
| Plain on all four nights less without replay, block 24 | B's trials | B's answer's readout | −8.36 ± 0.16 | 73, 20 | 8 | −10.63 ± 0.58 | 75, 47 | 11 |
| After block 6 alone less without replay, block 36 | A's trials | A's answer's readout | −1.75 ± 0.22 | 46, 52 | 3 | −3.45 ± 0.07 | 48, 42 | 7 |
| After block 6 alone less without replay, block 36 | A's trials | B's answer's readout | −2.38 ± 0.25 | 55, 44 | 5 | −3.04 ± 0.17 | 45, 32 | 6 |
| After block 6 alone less without replay, block 36 | B's trials | A's answer's readout | −2.74 ± 0.15 | 37, 32 | 3 | −2.21 ± 0.08 | 36, 26 | 4 |
| After block 6 alone less without replay, block 36 | B's trials | B's answer's readout | −1.76 ± 0.33 | 32, 38 | 4 | −2.58 ± 0.30 | 36, 42 | 6 |
| After block 12 alone less without replay, block 36 | A's trials | A's answer's readout | −2.76 ± 0.20 | 50, 36 | 6 | −3.35 ± 0.47 | 42, 39 | 7 |
| After block 12 alone less without replay, block 36 | A's trials | B's answer's readout | −3.71 ± 0.27 | 55, 24 | 8 | −2.82 ± 0.24 | 46, 20 | 5 |
| After block 12 alone less without replay, block 36 | B's trials | A's answer's readout | −2.11 ± 0.17 | 42, 27 | 3 | −1.28 ± 0.20 | 34, 23 | 3 |
| After block 12 alone less without replay, block 36 | B's trials | B's answer's readout | −0.93 ± 0.22 | 25, 30 | 3 | −1.82 ± 0.29 | 35, 20 | 3 |
| After block 18 alone less without replay, block 36 | A's trials | A's answer's readout | −2.57 ± 0.26 | 48, 36 | 8 | −4.49 ± 0.22 | 55, 35 | 8 |
| After block 18 alone less without replay, block 36 | A's trials | B's answer's readout | −3.99 ± 0.16 | 57, 31 | 9 | −2.79 ± 0.20 | 46, 18 | 6 |
| After block 18 alone less without replay, block 36 | B's trials | A's answer's readout | −2.42 ± 0.12 | 35, 23 | 6 | −1.64 ± 0.09 | 32, 24 | 4 |
| After block 18 alone less without replay, block 36 | B's trials | B's answer's readout | −0.83 ± 0.21 | 29, 39 | 4 | −2.20 ± 0.40 | 43, 30 | 6 |
| After block 24 alone less without replay, block 36 | A's trials | A's answer's readout | −3.32 ± 0.24 | 47, 36 | 10 | −4.49 ± 0.37 | 55, 44 | 10 |
| After block 24 alone less without replay, block 36 | A's trials | B's answer's readout | −3.04 ± 0.12 | 47, 26 | 8 | −2.28 ± 0.15 | 39, 33 | 6 |
| After block 24 alone less without replay, block 36 | B's trials | A's answer's readout | −3.02 ± 0.27 | 37, 21 | 7 | −3.12 ± 0.15 | 38, 20 | 6 |
| After block 24 alone less without replay, block 36 | B's trials | B's answer's readout | −1.87 ± 0.17 | 31, 36 | 5 | −2.80 ± 0.19 | 45, 39 | 5 |
| Plain on all four nights less without replay, block 36 | A's trials | A's answer's readout | −8.81 ± 0.13 | 57, 66 | 7 | −10.26 ± 0.48 | 75, 104 | 10 |
| Plain on all four nights less without replay, block 36 | A's trials | B's answer's readout | −10.20 ± 0.39 | 71, 34 | 9 | −10.43 ± 0.36 | 63, 67 | 8 |
| Plain on all four nights less without replay, block 36 | B's trials | A's answer's readout | −9.13 ± 0.31 | 50, 35 | 6 | −7.88 ± 0.33 | 61, 67 | 8 |
| Plain on all four nights less without replay, block 36 | B's trials | B's answer's readout | −5.33 ± 0.15 | 40, 62 | 5 | −7.23 ± 0.26 | 55, 104 | 7 |

Right after each single replayed night, less the night without replay, A's answer's readout on A's trials −8.48 to −7.07 here and −10.19 to
−7.45 there, on B's trials −7.46 to −5.66 and −6.21 to −5.31; B's answer's readout on A's trials −9.49 to −7.48 here and −7.92 to −6.03
there, on B's trials −6.08 to −5.48 and −6.77 to −6.27. Of these 16 differences on each cell, 16 of 16 lie the same side of 2 SE on both
cells (below, within or above). At block 36: A's answer's readout on A's trials −3.32 to −1.75 here and −4.49 to −3.35 there, on B's trials
−3.02 to −2.11 and −3.12 to −1.28; B's answer's readout on A's trials −3.99 to −2.38 here and −3.04 to −2.28 there, on B's trials −1.87 to
−0.83 and −2.80 to −1.82; 16 of 16 the same side of 2 SE on both.

**The answer's readout less the other**, less the night without replay:

| Comparison | On A's trials, A's answer's readout less the other: this cell | the image from the assignment | On B's trials, B's answer's readout less the other: this cell | the image from the assignment |
| :--- | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | −0.84 ± 0.21 | +0.48 ± 0.29 | −0.21 ± 0.30 | −0.80 ± 0.22 |
| After block 12 alone less without replay, block 12 | +0.69 ± 0.18 | −1.46 ± 0.22 | +0.08 ± 0.19 | −0.90 ± 0.21 |
| After block 18 alone less without replay, block 18 | +2.42 ± 0.23 | −2.96 ± 0.19 | +1.32 ± 0.26 | −1.43 ± 0.37 |
| After block 24 alone less without replay, block 24 | +1.31 ± 0.31 | −3.73 ± 0.16 | +1.66 ± 0.35 | −0.56 ± 0.17 |
| Plain on all four nights less without replay, block 6 | −0.84 ± 0.21 | +0.48 ± 0.29 | −0.21 ± 0.30 | −0.80 ± 0.22 |
| Plain on all four nights less without replay, block 12 | −0.87 ± 0.25 | +0.22 ± 0.43 | +0.03 ± 0.19 | −0.51 ± 0.52 |
| Plain on all four nights less without replay, block 18 | +0.68 ± 0.20 | −1.18 ± 0.30 | +2.72 ± 0.39 | −0.83 ± 0.65 |
| Plain on all four nights less without replay, block 24 | +1.50 ± 0.45 | −1.52 ± 0.27 | +3.10 ± 0.40 | −0.43 ± 0.59 |
| After block 6 alone less without replay, block 36 | +0.63 ± 0.33 | −0.41 ± 0.23 | +0.98 ± 0.42 | −0.37 ± 0.30 |
| After block 12 alone less without replay, block 36 | +0.95 ± 0.21 | −0.53 ± 0.27 | +1.18 ± 0.38 | −0.54 ± 0.46 |
| After block 18 alone less without replay, block 36 | +1.42 ± 0.35 | −1.69 ± 0.12 | +1.59 ± 0.23 | −0.56 ± 0.35 |
| After block 24 alone less without replay, block 36 | −0.28 ± 0.15 | −2.22 ± 0.31 | +1.15 ± 0.41 | +0.32 ± 0.13 |
| Plain on all four nights less without replay, block 36 | +1.39 ± 0.49 | +0.17 ± 0.31 | +3.80 ± 0.35 | +0.65 ± 0.57 |

Right after the nights after block 6, 12, 18 and 24 alone, on A's trials A's answer's readout less the other, less the night without replay,
read −0.84 ± 0.21, +0.69 ± 0.18, +2.42 ± 0.23 and +1.31 ± 0.31 here and +0.48 ± 0.29, −1.46 ± 0.22, −2.96 ± 0.19 and −3.73 ± 0.16 on the
image from the assignment; on B's trials B's answer's readout less the other −0.21 ± 0.30, +0.08 ± 0.19, +1.32 ± 0.26 and +1.66 ± 0.35 here
and −0.80 ± 0.22, −0.90 ± 0.21, −1.43 ± 0.37 and −0.56 ± 0.17 there. Of these eight on each cell, 0 of 8 lie the same side of 2 SE on both.

**Each night's change in the inputs by presynaptic group**, onto each readout named by its stimulus:

| Night after block | Form | Onto | A's kinds: this cell, M | the image from the assignment | B's kinds: this cell | the image from the assignment | The odd places' excitatory: this cell | the image from the assignment | The even places': this cell | the image from the assignment | Inhibitory: this cell | the image from the assignment | All excitatory: this cell | the image from the assignment |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | After block 6 alone | A's answer's readout | +0.35 | +0.31 | −0.29 | −0.32 | −1.20 | −2.56 | −3.01 | −1.68 | +5.25 | +4.39 | −4.18 | −4.29 |
| 6 | After block 6 alone | B's answer's readout | −0.35 | −0.31 | +0.28 | +0.32 | −2.62 | −1.28 | −0.72 | −2.90 | +4.19 | +4.32 | −3.44 | −4.21 |
| 6 | Plain on all four nights (H-47) | A's answer's readout | +0.35 | +0.31 | −0.29 | −0.32 | −1.20 | −2.56 | −3.01 | −1.68 | +5.25 | +4.39 | −4.18 | −4.29 |
| 6 | Plain on all four nights (H-47) | B's answer's readout | −0.35 | −0.31 | +0.28 | +0.32 | −2.62 | −1.28 | −0.72 | −2.90 | +4.19 | +4.32 | −3.44 | −4.21 |
| 6 | Without replay | A's answer's readout | +0.02 | −0.04 | −0.02 | −0.01 | −0.02 | +0.01 | −0.11 | −0.16 | +0.29 | +0.24 | −0.16 | −0.22 |
| 6 | Without replay | B's answer's readout | −0.02 | +0.04 | +0.02 | +0.01 | +0.07 | +0.02 | −0.20 | −0.05 | +0.27 | +0.31 | −0.15 | −0.01 |
| 12 | After block 12 alone | A's answer's readout | +0.40 | +0.30 | −0.30 | −0.28 | −0.97 | −2.44 | −2.70 | −1.68 | +5.36 | +4.48 | −3.60 | −4.14 |
| 12 | After block 12 alone | B's answer's readout | −0.40 | −0.30 | +0.30 | +0.28 | −2.44 | −1.25 | −0.83 | −2.56 | +4.16 | +4.43 | −3.40 | −3.89 |
| 12 | Plain on all four nights (H-47) | A's answer's readout | +0.31 | +0.23 | −0.28 | −0.24 | −0.61 | −1.92 | −2.17 | −1.16 | +4.32 | +3.57 | −2.77 | −3.10 |
| 12 | Plain on all four nights (H-47) | B's answer's readout | −0.31 | −0.23 | +0.28 | +0.24 | −1.81 | −0.90 | −0.68 | −2.18 | +3.48 | +3.54 | −2.55 | −3.09 |
| 12 | Without replay | A's answer's readout | 0.00 | −0.01 | −0.04 | 0.00 | +0.04 | +0.17 | +0.03 | −0.03 | +0.26 | +0.24 | 0.00 | +0.11 |
| 12 | Without replay | B's answer's readout | 0.00 | +0.01 | +0.04 | 0.00 | +0.21 | −0.05 | −0.06 | +0.01 | +0.25 | +0.26 | +0.18 | −0.05 |
| 18 | After block 18 alone | A's answer's readout | +0.42 | +0.25 | −0.28 | −0.28 | −1.07 | −2.25 | −2.68 | −1.62 | +5.34 | +4.32 | −3.63 | −3.91 |
| 18 | After block 18 alone | B's answer's readout | −0.42 | −0.25 | +0.28 | +0.28 | −2.33 | −1.25 | −0.70 | −2.45 | +4.26 | +4.18 | −3.18 | −3.71 |
| 18 | Plain on all four nights (H-47) | A's answer's readout | +0.27 | +0.12 | −0.29 | −0.17 | −0.36 | −1.56 | −1.65 | −0.58 | +3.72 | +2.98 | −2.06 | −2.21 |
| 18 | Plain on all four nights (H-47) | B's answer's readout | −0.27 | −0.12 | +0.29 | +0.17 | −1.24 | −0.49 | −0.40 | −1.47 | +2.88 | +2.88 | −1.64 | −1.93 |
| 18 | Without replay | A's answer's readout | +0.02 | −0.05 | −0.02 | −0.01 | +0.07 | +0.18 | +0.01 | +0.07 | +0.29 | +0.25 | +0.06 | +0.19 |
| 18 | Without replay | B's answer's readout | −0.02 | +0.05 | +0.02 | +0.01 | +0.16 | +0.03 | +0.02 | +0.06 | +0.30 | +0.31 | +0.18 | +0.14 |
| 24 | After block 24 alone | A's answer's readout | +0.38 | +0.23 | −0.34 | −0.30 | −1.06 | −2.26 | −2.71 | −1.46 | +5.34 | +4.39 | −3.76 | −3.82 |
| 24 | After block 24 alone | B's answer's readout | −0.38 | −0.23 | +0.34 | +0.30 | −2.30 | −0.95 | −0.67 | −2.45 | +4.31 | +4.22 | −3.02 | −3.37 |
| 24 | Plain on all four nights (H-47) | A's answer's readout | +0.22 | +0.18 | −0.16 | −0.11 | −0.01 | −1.14 | −1.35 | −0.43 | +3.36 | +2.63 | −1.32 | −1.52 |
| 24 | Plain on all four nights (H-47) | B's answer's readout | −0.22 | −0.18 | +0.16 | +0.11 | −0.87 | −0.48 | −0.18 | −1.13 | +2.50 | +2.48 | −1.13 | −1.68 |
| 24 | Without replay | A's answer's readout | +0.03 | −0.02 | +0.01 | +0.02 | +0.18 | +0.16 | −0.03 | +0.13 | +0.34 | +0.34 | +0.18 | +0.30 |
| 24 | Without replay | B's answer's readout | −0.03 | +0.02 | −0.01 | −0.02 | +0.06 | +0.13 | +0.12 | −0.12 | +0.40 | +0.36 | +0.15 | −0.01 |

Over each single replayed night: inhibitory onto A's answer's readout +5.25 to +5.36 M here and +4.32 to +4.48 there, onto B's answer's
+4.16 to +4.31 and +4.18 to +4.43; A's kinds' onto A's answer's readout +0.35 to +0.42 M here and +0.23 to +0.31 there, onto B's answer's
−0.42 to −0.35 and −0.31 to −0.23; B's kinds' onto A's answer's readout −0.34 to −0.28 M here and −0.32 to −0.28 there, onto B's answer's
+0.28 to +0.34 and +0.28 to +0.32; all excitatory onto A's answer's readout −4.18 to −3.60 M here and −4.29 to −3.82 there, onto B's
answer's −3.44 to −3.02 and −4.21 to −3.37. Of these 32 changes on each cell, 32 carry the same sign on both. Each place group's excitatory
units lost more onto the other readout than onto their own in 8 of the 8 cases here and 8 on the image from the assignment.

**A's and B's kinds and each readout's net input**, each comparison's difference, A's answer's readout against the other:

| Comparison | A's kinds, their difference onto A's answer's readout less onto the other: this cell, M | the image from the assignment | B's kinds onto B's answer's readout less onto the other: this cell | the image from the assignment | Excitatory less inhibitory, A's answer's readout's difference less the other's: this cell | the image from the assignment | Inhibitory, A's answer's readout's difference less the other's: this cell | the image from the assignment | Spikes on A's trials, A's answer's readout less the other: this cell | the image from the assignment |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | +0.67 | +0.69 | +0.54 | +0.63 | −1.76 | −0.01 | +1.03 | +0.14 | −0.84 | +0.48 |
| After block 12 alone less without replay, block 12 | +0.81 | +0.62 | +0.52 | +0.55 | −1.21 | −0.48 | +1.19 | +0.07 | +0.69 | −1.46 |
| After block 18 alone less without replay, block 18 | +0.79 | +0.60 | +0.53 | +0.53 | −1.42 | −0.46 | +1.09 | +0.21 | +2.42 | −2.96 |
| After block 24 alone less without replay, block 24 | +0.69 | +0.49 | +0.70 | +0.64 | −1.86 | −0.95 | +1.09 | +0.19 | +1.31 | −3.73 |
| Plain on all four nights less without replay, block 6 | +0.67 | +0.69 | +0.54 | +0.63 | −1.76 | −0.01 | +1.03 | +0.14 | −0.84 | +0.48 |
| Plain on all four nights less without replay, block 12 | +1.29 | +1.13 | +0.94 | +1.14 | −2.37 | −0.26 | +1.61 | +0.12 | −0.87 | +0.22 |
| Plain on all four nights less without replay, block 18 | +1.88 | +1.49 | +1.48 | +1.44 | −3.33 | −0.61 | +2.34 | +0.21 | +0.68 | −1.18 |
| Plain on all four nights less without replay, block 24 | +2.36 | +1.93 | +1.72 | +1.73 | −4.09 | −0.73 | +3.12 | +0.26 | +1.50 | −1.52 |
| Plain on all four nights less without replay, block 36 | +2.39 | +2.03 | +1.77 | +1.84 | −3.83 | −0.60 | +2.71 | +0.12 | +1.39 | +0.17 |
| After block 6 alone less without replay, block 36 | +0.66 | +0.60 | +0.50 | +0.50 | −0.56 | −0.32 | −0.26 | +0.03 | +0.63 | −0.41 |
| After block 12 alone less without replay, block 36 | +0.74 | +0.76 | +0.43 | +0.49 | −0.54 | −0.14 | +0.20 | −0.05 | +0.95 | −0.53 |
| After block 18 alone less without replay, block 36 | +0.78 | +0.61 | +0.50 | +0.48 | −0.94 | −0.18 | +0.40 | +0.21 | +1.42 | −1.69 |
| After block 24 alone less without replay, block 36 | +0.61 | +0.55 | +0.60 | +0.71 | −1.48 | −0.80 | +0.67 | +0.09 | −0.28 | −2.22 |

Right after the nights after block 6, 12, 18 and 24 alone, A's kinds' difference onto A's answer's readout less onto the other read +0.67,
+0.81, +0.79 and +0.69 M here and +0.69, +0.62, +0.60 and +0.49 on the image from the assignment; A's answer's readout's excitatory input
less inhibitory input difference less the other's −1.76, −1.21, −1.42 and −1.86 M here and −0.01, −0.48, −0.46 and −0.95 there, where the
spikes on A's trials, A's answer's readout less the other, read −0.84, +0.69, +2.42 and +1.31 here and +0.48, −1.46, −2.96 and −3.73 there.

**The gain and A's margin over each night**:

| Night after block | Form | This night | The gain's change: this cell | the image from the assignment | A's margin's change: this cell, M | the image from the assignment |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: |
| 6 | After block 6 alone | replays | −0.073 | −0.062 | +1.28 | +1.26 |
| 6 | Plain on all four nights (H-47) | replays | −0.073 | −0.062 | +1.28 | +1.26 |
| 6 | Without replay | without replay | +0.031 | +0.033 | +0.07 | −0.05 |
| 12 | After block 12 alone | replays | −0.073 | −0.065 | +1.41 | +1.16 |
| 12 | Plain on all four nights (H-47) | replays | −0.066 | −0.057 | +1.17 | +0.95 |
| 12 | Without replay | without replay | +0.034 | +0.033 | +0.07 | −0.01 |
| 18 | After block 18 alone | replays | −0.074 | −0.061 | +1.40 | +1.05 |
| 18 | Plain on all four nights (H-47) | replays | −0.056 | −0.046 | +1.12 | +0.56 |
| 18 | Without replay | without replay | +0.037 | +0.037 | +0.08 | −0.08 |
| 24 | After block 24 alone | replays | −0.074 | −0.063 | +1.43 | +1.05 |
| 24 | Plain on all four nights (H-47) | replays | −0.050 | −0.042 | +0.76 | +0.58 |
| 24 | Without replay | without replay | +0.037 | +0.037 | +0.04 | −0.08 |

Over the replayed night after block 6, 12, 18 and 24 alone the gain changed by −0.073, −0.073, −0.074 and −0.074 here and −0.062, −0.065,
−0.061 and −0.063 on the image from the assignment, the night without replay's same nights by +0.031 to +0.037 and +0.033 to +0.037; A's
margin changed by +1.28, +1.41, +1.40 and +1.43 M here and +1.26, +1.16, +1.05 and +1.05 there.

**The image each replayed night starts from**, the night without replay's image before each night, on each cell:

| Image before the night after block | A's kinds onto A's answer's readout less onto the other: this cell, M | the image from the assignment | B's kinds onto B's answer's readout less onto the other: this cell | the image from the assignment | A's margin: this cell | the image from the assignment | Inhibitory onto A's answer's readout: this cell | the image from the assignment | onto B's answer's readout: this cell | the image from the assignment | Excitatory less inhibitory, A's answer's readout less the other: this cell | the image from the assignment | Gain: this cell | the image from the assignment |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | +0.03 | +0.52 | +0.39 | −0.16 | +0.42 | +0.36 | +26.39 | +25.70 | +25.16 | +27.44 | −0.33 | +1.22 | 1.695 | 1.694 |
| 12 | −0.16 | +0.17 | +0.27 | −0.23 | +0.12 | −0.06 | +26.79 | +26.22 | +25.58 | +27.92 | −0.38 | +0.72 | 1.689 | 1.690 |
| 18 | −0.33 | +0.04 | +0.23 | −0.32 | −0.10 | −0.28 | +27.08 | +26.72 | +25.99 | +28.27 | −0.44 | +0.67 | 1.683 | 1.683 |
| 24 | −0.43 | −0.19 | +0.22 | −0.31 | −0.21 | −0.50 | +27.44 | +27.31 | +26.52 | +28.77 | −0.50 | +0.40 | 1.675 | 1.675 |

Before the nights after block 6, 12, 18 and 24, A's kinds' input onto A's answer's readout less onto the other read +0.03, −0.16, −0.33 and
−0.43 M here and +0.52, +0.17, +0.04 and −0.19 on the image from the assignment, A's margin +0.42, +0.12, −0.10 and −0.21 and +0.36, −0.06,
−0.28 and −0.50, and A's answer's readout's excitatory input less inhibitory input less the other's −0.33, −0.38, −0.44 and −0.50 and +1.22,
+0.72, +0.67 and +0.40; the gain was 1.695, 1.689, 1.683 and 1.675 here and 1.694, 1.690, 1.683 and 1.675 there.

**The units that carried half of each readout's change**, their input difference from the trial's stimulus's kinds and their inhibitory
input difference against the rest's, and the correlation over each readout's units of a unit's input difference from the trial's stimulus's
kinds with its spikes' difference:

| Comparison | Trials | The readout | This cell: units carrying half its change | their input difference from the trial's stimulus's kinds, a unit | the rest's | their inhibitory input difference | the rest's | Correlation of a unit's input difference from the trial's stimulus's kinds with its spikes' difference | The image from the assignment: units carrying half | their input difference from the trial's stimulus's kinds | the rest's | their inhibitory | the rest's | Correlation |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | A's answer's readout | 13 | −12 138 | +1 104 | +366 | +11 109 | +0.58 | 16 | −7 967 | +1 065 | +749 | +9 350 | +0.60 |
| After block 6 alone less without replay, block 6 | A's trials | B's answer's readout | 16 | −11 611 | −336 | +4 118 | +8 712 | +0.83 | 15 | −10 144 | −433 | +455 | +9 028 | +0.63 |
| After block 6 alone less without replay, block 6 | B's trials | A's answer's readout | 18 | −8 578 | −260 | +3 126 | +11 118 | +0.71 | 15 | −11 194 | −326 | +3 663 | +9 232 | +0.72 |
| After block 6 alone less without replay, block 6 | B's trials | B's answer's readout | 14 | −10 028 | +920 | +2 483 | +8 743 | +0.53 | 18 | −8 272 | +1 047 | +606 | +9 080 | +0.62 |
| After block 12 alone less without replay, block 12 | A's trials | A's answer's readout | 15 | −10 895 | +1 283 | +939 | +11 463 | +0.59 | 18 | −7 226 | +1 002 | +1 129 | +9 573 | +0.61 |
| After block 12 alone less without replay, block 12 | A's trials | B's answer's readout | 19 | −11 767 | −415 | +4 411 | +8 705 | +0.78 | 15 | −8 739 | −407 | +5 211 | +9 219 | +0.60 |
| After block 12 alone less without replay, block 12 | B's trials | A's answer's readout | 19 | −8 083 | −246 | +1 809 | +11 521 | +0.69 | 19 | −7 863 | −280 | +3 955 | +9 470 | +0.61 |
| After block 12 alone less without replay, block 12 | B's trials | B's answer's readout | 15 | −6 826 | +821 | +2 280 | +8 738 | +0.45 | 17 | −7 788 | +916 | +920 | +9 402 | +0.58 |
| After block 18 alone less without replay, block 18 | A's trials | A's answer's readout | 17 | −8 572 | +1 228 | −162 | +11 428 | +0.61 | 19 | −7 584 | +1 005 | +1 901 | +9 171 | +0.62 |
| After block 18 alone less without replay, block 18 | A's trials | B's answer's readout | 19 | −11 162 | −421 | +6 513 | +8 726 | +0.71 | 17 | −10 330 | −277 | +4 460 | +8 568 | +0.70 |
| After block 18 alone less without replay, block 18 | B's trials | A's answer's readout | 20 | −7 222 | −274 | +1 311 | +11 440 | +0.65 | 15 | −7 057 | −358 | +5 535 | +8 983 | +0.51 |
| After block 18 alone less without replay, block 18 | B's trials | B's answer's readout | 22 | −4 928 | +854 | +3 485 | +8 893 | +0.43 | 18 | −6 365 | +861 | +1 029 | +8 717 | +0.57 |
| After block 24 alone less without replay, block 24 | A's trials | A's answer's readout | 17 | −7 823 | +1 080 | +1 641 | +11 250 | +0.55 | 19 | −8 344 | +917 | +789 | +9 177 | +0.61 |
| After block 24 alone less without replay, block 24 | A's trials | B's answer's readout | 19 | −8 664 | −408 | +11 621 | +8 381 | +0.66 | 17 | −8 438 | −230 | +3 390 | +8 609 | +0.60 |
| After block 24 alone less without replay, block 24 | B's trials | A's answer's readout | 17 | −9 472 | −427 | +2 864 | +11 203 | +0.68 | 13 | −9 682 | −434 | +6 269 | +8 904 | +0.55 |
| After block 24 alone less without replay, block 24 | B's trials | B's answer's readout | 17 | −7 552 | +1 082 | +4 036 | +8 688 | +0.52 | 14 | −8 422 | +983 | +797 | +8 655 | +0.55 |
| After block 6 alone less without replay, block 36 | A's trials | A's answer's readout | 3 | −28 101 | +905 | −7 824 | +2 452 | +0.76 | 7 | −4 378 | +730 | +135 | +1 725 | +0.39 |
| After block 6 alone less without replay, block 36 | A's trials | B's answer's readout | 5 | −15 730 | −550 | −5 116 | +3 041 | +0.68 | 6 | −24 614 | −334 | −2 111 | +1 695 | +0.79 |
| After block 6 alone less without replay, block 36 | B's trials | A's answer's readout | 3 | −19 652 | −417 | −29 027 | +2 591 | +0.71 | 4 | −11 874 | −446 | −16 420 | +1 860 | +0.56 |
| After block 6 alone less without replay, block 36 | B's trials | B's answer's readout | 4 | −6 722 | +606 | −8 759 | +3 055 | +0.47 | 6 | −7 603 | +653 | −8 335 | +1 777 | +0.48 |
| After block 12 alone less without replay, block 36 | A's trials | A's answer's readout | 6 | −17 139 | +1 043 | −3 642 | +3 799 | +0.64 | 7 | −6 632 | +941 | −4 475 | +2 712 | +0.39 |
| After block 12 alone less without replay, block 36 | A's trials | B's answer's readout | 8 | −11 024 | −624 | −2 614 | +3 370 | +0.65 | 5 | −25 234 | −557 | −1 271 | +2 762 | +0.82 |
| After block 12 alone less without replay, block 36 | B's trials | A's answer's readout | 3 | −7 740 | −418 | −17 716 | +3 842 | +0.63 | 3 | −12 366 | −457 | −3 314 | +2 641 | +0.45 |
| After block 12 alone less without replay, block 36 | B's trials | B's answer's readout | 3 | −10 419 | +537 | +2 021 | +3 274 | +0.52 | 3 | +500 | +535 | −12 826 | +2 820 | +0.36 |
| After block 18 alone less without replay, block 36 | A's trials | A's answer's readout | 8 | −9 124 | +1 030 | −296 | +4 900 | +0.62 | 8 | −4 922 | +759 | −2 285 | +3 671 | +0.48 |
| After block 18 alone less without replay, block 36 | A's trials | B's answer's readout | 9 | −8 719 | −696 | +1 351 | +3 984 | +0.66 | 6 | −18 991 | −418 | −403 | +3 160 | +0.78 |
| After block 18 alone less without replay, block 36 | B's trials | A's answer's readout | 6 | −10 535 | −411 | −6 506 | +4 960 | +0.58 | 4 | −12 262 | −415 | +775 | +3 592 | +0.55 |
| After block 18 alone less without replay, block 36 | B's trials | B's answer's readout | 4 | −2 565 | +570 | +1 980 | +3 949 | +0.38 | 6 | −1 676 | +547 | −6 036 | +3 234 | +0.40 |
| After block 24 alone less without replay, block 36 | A's trials | A's answer's readout | 10 | −12 126 | +950 | −519 | +6 844 | +0.62 | 10 | −10 727 | +854 | −730 | +5 229 | +0.60 |
| After block 24 alone less without replay, block 36 | A's trials | B's answer's readout | 8 | −8 127 | −533 | +7 240 | +5 192 | +0.61 | 6 | −17 938 | −372 | +413 | +4 961 | +0.73 |
| After block 24 alone less without replay, block 36 | B's trials | A's answer's readout | 7 | −13 211 | −458 | +346 | +6 782 | +0.73 | 6 | −14 560 | −595 | +2 733 | +5 131 | +0.58 |
| After block 24 alone less without replay, block 36 | B's trials | B's answer's readout | 5 | −5 391 | +719 | +141 | +5 284 | +0.50 | 5 | −1 362 | +801 | −2 113 | +4 979 | +0.46 |
| Plain on all four nights less without replay, block 6 | A's trials | A's answer's readout | 13 | −12 138 | +1 104 | +366 | +11 109 | +0.58 | 16 | −7 967 | +1 065 | +749 | +9 350 | +0.60 |
| Plain on all four nights less without replay, block 6 | A's trials | B's answer's readout | 16 | −11 611 | −336 | +4 118 | +8 712 | +0.83 | 15 | −10 144 | −433 | +455 | +9 028 | +0.63 |
| Plain on all four nights less without replay, block 6 | B's trials | A's answer's readout | 18 | −8 578 | −260 | +3 126 | +11 118 | +0.71 | 15 | −11 194 | −326 | +3 663 | +9 232 | +0.72 |
| Plain on all four nights less without replay, block 6 | B's trials | B's answer's readout | 14 | −10 028 | +920 | +2 483 | +8 743 | +0.53 | 18 | −8 272 | +1 047 | +606 | +9 080 | +0.62 |
| Plain on all four nights less without replay, block 12 | A's trials | A's answer's readout | 10 | −21 223 | +1 912 | +216 | +17 801 | +0.58 | 14 | −12 777 | +1 672 | +1 125 | +14 717 | +0.58 |
| Plain on all four nights less without replay, block 12 | A's trials | B's answer's readout | 14 | −22 956 | −730 | +12 505 | +13 954 | +0.85 | 13 | −21 020 | −655 | +6 980 | +14 252 | +0.73 |
| Plain on all four nights less without replay, block 12 | B's trials | A's answer's readout | 12 | −17 398 | −582 | +2 959 | +17 806 | +0.76 | 13 | −20 053 | −690 | +2 690 | +14 641 | +0.79 |
| Plain on all four nights less without replay, block 12 | B's trials | B's answer's readout | 10 | −17 139 | +1 426 | +2 184 | +14 171 | +0.54 | 13 | −12 109 | +1 628 | +903 | +14 429 | +0.57 |
| Plain on all four nights less without replay, block 18 | A's trials | A's answer's readout | 9 | −22 936 | +2 548 | −2 329 | +23 245 | +0.59 | 15 | −17 811 | +2 280 | +1 005 | +18 696 | +0.61 |
| Plain on all four nights less without replay, block 18 | A's trials | B's answer's readout | 13 | −34 860 | −1 092 | +16 389 | +17 672 | +0.85 | 12 | −34 025 | −754 | +9 157 | +17 893 | +0.78 |
| Plain on all four nights less without replay, block 18 | B's trials | A's answer's readout | 10 | −25 881 | −1 070 | +996 | +23 228 | +0.74 | 11 | −30 294 | −858 | +5 392 | +18 430 | +0.79 |
| Plain on all four nights less without replay, block 18 | B's trials | B's answer's readout | 8 | −16 901 | +1 939 | −222 | +17 953 | +0.48 | 12 | −14 448 | +1 993 | +1 478 | +18 099 | +0.54 |
| Plain on all four nights less without replay, block 24 | A's trials | A's answer's readout | 10 | −26 603 | +3 226 | −4 519 | +27 669 | +0.61 | 14 | −23 723 | +2 919 | +176 | +21 317 | +0.58 |
| Plain on all four nights less without replay, block 24 | A's trials | B's answer's readout | 12 | −43 531 | −1 477 | +23 065 | +20 096 | +0.84 | 11 | −46 866 | −1 007 | −3 474 | +20 695 | +0.81 |
| Plain on all four nights less without replay, block 24 | B's trials | A's answer's readout | 9 | −34 861 | −1 210 | −3 713 | +27 582 | +0.76 | 10 | −48 473 | −848 | +8 336 | +20 947 | +0.80 |
| Plain on all four nights less without replay, block 24 | B's trials | B's answer's readout | 8 | −21 231 | +2 279 | −3 640 | +20 596 | +0.54 | 11 | −13 887 | +2 273 | −234 | +20 615 | +0.51 |
| Plain on all four nights less without replay, block 36 | A's trials | A's answer's readout | 7 | −19 263 | +2 946 | −11 963 | +22 843 | +0.55 | 10 | −22 989 | +2 772 | −3 586 | +16 741 | +0.55 |
| Plain on all four nights less without replay, block 36 | A's trials | B's answer's readout | 9 | −45 632 | −1 747 | +7 043 | +16 595 | +0.83 | 8 | −56 374 | −1 250 | −4 582 | +16 409 | +0.84 |
| Plain on all four nights less without replay, block 36 | B's trials | A's answer's readout | 6 | −45 540 | −1 356 | −5 786 | +22 684 | +0.77 | 8 | −51 647 | −1 123 | +2 593 | +16 541 | +0.81 |
| Plain on all four nights less without replay, block 36 | B's trials | B's answer's readout | 5 | −14 503 | +2 115 | −13 457 | +16 737 | +0.49 | 7 | −4 686 | +2 107 | −4 861 | +16 367 | +0.43 |

Right after each single replayed night, the units that carried half of a readout's change had an input difference from the trial's
stimulus's kinds below the rest's in 16 of 16 sets here and 16 of 16 on the image from the assignment, and an inhibitory input difference
below the rest's in 15 of 16 and 16 of 16; over each readout's units a unit's input difference from the trial's stimulus's kinds correlated
with its spikes' difference by +0.43 to +0.83 here and +0.51 to +0.72 there.

**From right after each single replayed night to block 36**, the inhibitory and all excitatory input's difference onto each readout:

| Form | Onto | This cell: inhibitory difference right after, M | at block 36 | come back, per cent | All excitatory difference right after | at block 36 | The image from the assignment: inhibitory right after | at block 36 | come back, per cent | All excitatory right after | at block 36 |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone | A's answer's readout | +4.96 | +1.09 | 78 | −4.02 | −3.75 | +4.15 | +0.78 | 81 | −4.07 | −3.95 |
| After block 6 alone | B's answer's readout | +3.93 | +1.36 | 65 | −3.29 | −2.92 | +4.02 | +0.76 | 81 | −4.20 | −3.65 |
| After block 12 alone | A's answer's readout | +5.10 | +1.70 | 67 | −3.60 | −3.46 | +4.24 | +1.19 | 72 | −4.25 | −3.95 |
| After block 12 alone | B's answer's readout | +3.91 | +1.50 | 62 | −3.58 | −3.12 | +4.17 | +1.25 | 70 | −3.84 | −3.75 |
| After block 18 alone | A's answer's readout | +5.05 | +2.21 | 56 | −3.69 | −3.63 | +4.07 | +1.64 | 60 | −4.10 | −3.89 |
| After block 18 alone | B's answer's readout | +3.96 | +1.80 | 54 | −3.36 | −3.09 | +3.86 | +1.43 | 63 | −3.85 | −3.91 |
| After block 24 alone | A's answer's readout | +5.00 | +3.07 | 39 | −3.94 | −3.96 | +4.05 | +2.34 | 42 | −4.13 | −4.10 |
| After block 24 alone | B's answer's readout | +3.91 | +2.40 | 39 | −3.17 | −3.15 | +3.86 | +2.25 | 42 | −3.36 | −3.39 |

By block 36 the inhibitory input's difference onto each readout had come back by 39 to 78 per cent of its value right after the replayed
night here and by 42 to 81 per cent on the image from the assignment.

**H-47's plain night on all four nights**, its inputs' difference from the night without replay on each cell:

| Block | Onto | This cell: A's kinds, M | B's kinds | Inhibitory | All excitatory | The image from the assignment: A's kinds | B's kinds | Inhibitory | All excitatory |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | A's answer's readout | +0.33 | −0.27 | +4.96 | −4.02 | +0.34 | −0.31 | +4.15 | −4.07 |
| 6 | B's answer's readout | −0.33 | +0.27 | +3.93 | −3.29 | −0.34 | +0.31 | +4.02 | −4.20 |
| 12 | A's answer's readout | +0.65 | −0.47 | +7.99 | −6.78 | +0.57 | −0.57 | +6.56 | −7.33 |
| 12 | B's answer's readout | −0.65 | +0.47 | +6.38 | −6.03 | −0.57 | +0.57 | +6.45 | −7.19 |
| 18 | A's answer's readout | +0.94 | −0.74 | +10.44 | −8.87 | +0.75 | −0.72 | +8.32 | −9.69 |
| 18 | B's answer's readout | −0.94 | +0.74 | +8.09 | −7.88 | −0.75 | +0.72 | +8.11 | −9.29 |
| 24 | A's answer's readout | +1.18 | −0.86 | +12.38 | −10.25 | +0.97 | −0.87 | +9.49 | −11.48 |
| 24 | B's answer's readout | −1.18 | +0.86 | +9.26 | −9.28 | −0.97 | +0.87 | +9.23 | −11.00 |
| 36 | A's answer's readout | +1.20 | −0.89 | +10.24 | −10.32 | +1.01 | −0.92 | +7.48 | −11.47 |
| 36 | B's answer's readout | −1.20 | +0.89 | +7.53 | −9.20 | −1.01 | +0.92 | +7.36 | −10.99 |

Under H-47's plain night, less the night without replay's at blocks 6, 12, 18, 24 and 36, the inhibitory input onto A's answer's readout
read +4.96, +7.99, +10.44, +12.38, +10.24 M here and +4.15, +6.56, +8.32, +9.49, +7.48 on the image from the assignment; A's kinds' onto A's
answer's readout +0.33, +0.65, +0.94, +1.18, +1.20 here and +0.34, +0.57, +0.75, +0.97, +1.01 there.

## What it costs

The trial took 303 s and the two jobs 5 971 and 7 485 s, 99.5 and 124.8 minutes, the longer 52.0 % of a job's 240 minutes; ADR-0250 put them
at about 96 and 128 minutes. Exploration 034's jobs on the image from the assignment took 5 822 and 7 925 s. Nothing was cut: each job ran
its forms on the three paths, every night watched, every read of A counted unit by unit and every image's inputs read. The two logs are 10.9
and 13.4 MB.

## What it suggests

**On this cell each single replayed night moved both readouts' units' inputs, and both readouts' spikes, as on the image from the
assignment**: more inhibition onto both readouts, each place group's excitatory units' weights off the other readout, A's kinds toward A's
answer's readout and B's toward B's, the gain lower and A's margin from the inputs higher; both readouts fired less on both stimuli's
trials, and the units that carried half of each drop had lost input from the trial's stimulus's kinds. What differed is which readout fell
more on A's trials: after the three later single nights A's answer's readout fell less than the other here and more there, though here it
lost more of its excitatory input less inhibitory input than the other, and more than on the image from the assignment. A's count lay above
the night without replay beyond 2 SE right after the nights after block 18 and 24 alone here, and below it beyond 2 SE right after the
nights after block 12, 18 and 24 alone there.

Questions this suggests for a later exploration or a round:
- What makes the same changes in the inputs leave A's answer's readout less the other above the night without replay here and below it
  there: an input here is a sum of weights, and the presynaptic units' spikes on A's trials, which weight it, are not read.
- Whether the image a replayed night starts from decides the direction: before the nights after block 6, 12, 18 and 24, A's answer's
  readout's excitatory input less inhibitory input less the other's read −0.33, −0.38, −0.44 and −0.50 M here and +1.22, +0.72, +0.67 and
  +0.40 on the image from the assignment, and A's kinds' input onto A's answer's readout less onto the other +0.03, −0.16, −0.33 and −0.43
  here and +0.52, +0.17, +0.04 and −0.19 there.
- Why the night after block 6 alone left A's answer's readout less the other below the night without replay on A's trials here,
  −0.84 ± 0.21, where the three later single nights left it above.
- What lands more of a replayed night's inhibition on readout 0 here, 5.25 to 5.36 M against 4.16 to 4.31 onto readout 1, where on the image
  from the assignment the two readouts' rises overlapped, 4.32 to 4.48 onto A's answer's readout and 4.18 to 4.43 onto B's.
- Whether the units that carried half of each drop are the units with the most input from the trial's stimulus's kinds on the image before
  the night: those inputs are in these jobs' logs and are not read here against the units.
- Whether the inputs move the same way on the other ten cells.

## What did not run

- **Any other cell**: the ten cells but the two eighth images of seed 24.
- **Replay on two of the four nights, or on every night but one.**
- **Any hold**, on any night.
- **Plain nights on both mappings and the awake gap.**
- **The pair's reads unit by unit**: they are read as before, their counts and masks set beside the logs'.
- **The units outside the readouts' sets**, their spikes and their inputs: the stimulus units, the other excitatory units and the inhibitory
  units elsewhere.
- **A synapse's weight paired with its presynaptic unit's spikes**: the readout units' own spikes are counted, the spikes of the units
  outside the readouts are not, and no weight is set against the spikes of the unit it comes from.
- **A unit's membrane, threshold or delays**: a unit's inputs are the weights onto it, not its drive.
- **Any path but H-47's three**, 0, 5 and 6.
- **A night split apart**, such as slow-wave sleep read apart from REM: a night's change is read whole, from the images before and after
  it.
- **The next day's reading of the pairings a night leaves** pending.
- **Any SE for an input**: an input is the mean of three images, one a path; the correlations carry none either.
- **A table of each of the 918 units' inputs**: the tables sum the readouts, the classes and the named sets; every unit's sums are in the
  jobs' logs.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_the_mirrored_cells_units_and_inputs_explore` | [43133489826](https://github.com/Menteion/Menteion/actions/runs/43133489826) | `6a44ad3` | 303 |
| `the_mirrored_cells_units_and_inputs_after_block_6_alone_and_on_every_night_explore` | [43134386489](https://github.com/Menteion/Menteion/actions/runs/43134386489) | `6a44ad3` | 5 971 |
| `the_mirrored_cells_units_and_inputs_after_each_later_night_alone_explore` | [43134386489](https://github.com/Menteion/Menteion/actions/runs/43134386489) | `6a44ad3` | 7 485 |
