---
status: exploratory
date: 2026-11-12
---

# 036: What reaches the readout units, on both cells

**Branch**: `explore/036-what-reaches-the-readout-units-on-both-cells`
**Commit**: `816d89e`, which adds the volley, on `b0f3229`, which adds the harness, on `cdbf20d`: `main` at `af901b6`, ADR-0251's
merge, with exploration 035's tag merged in, which changes nothing under `src/`. This branch changes nothing under `src/`, so the engine
is `main`'s.
**Runs**: [43190778334](https://github.com/Menteion/Menteion/actions/runs/43190778334) (the harness's trial) and
[43192758106](https://github.com/Menteion/Menteion/actions/runs/43192758106) (four jobs), both at `816d89e` and `scope=arms`. Before
them, at `b0f3229`, the harness's first trial, run [43187226229](https://github.com/Menteion/Menteion/actions/runs/43187226229), and four
jobs, run [43189318519](https://github.com/Menteion/Menteion/actions/runs/43189318519), cancelled before any finished. The logs set beside
them are, on the image from the assignment, exploration 033's jobs, run
[43011654795](https://github.com/Menteion/Menteion/actions/runs/43011654795), and exploration 034's trial and jobs, runs
[43051132329](https://github.com/Menteion/Menteion/actions/runs/43051132329) and
[43052358536](https://github.com/Menteion/Menteion/actions/runs/43052358536), and on the image from the mirrored assignment exploration
035's trial and jobs, runs [43133489826](https://github.com/Menteion/Menteion/actions/runs/43133489826) and
[43134386489](https://github.com/Menteion/Menteion/actions/runs/43134386489). None of those was run again.

[ADR-0251](../adr/0251-what-reaches-the-readout-units-on-both-cells.md) ordered this exploration. On seed 24's eighth image from the
mirrored assignment each single replayed night moved the inputs onto the readouts with the same signs as on the image from the assignment,
32 of 32 changes, and both readouts fired less on both stimuli's trials. Yet on A's trials the spikes of A's answer's readout less the
other's, less the night without replay, read −0.84 ± 0.21, +0.69 ± 0.18, +2.42 ± 0.23 and +1.31 ± 0.31 after block 6, 12, 18 and 24 alone
there, and +0.48 ± 0.29, −1.46 ± 0.22, −2.96 ± 0.19 and −3.73 ± 0.16 on the image from the assignment ([exploration
035](035-the-mirrored-cells-units-and-inputs.md)). An input is a sum of weights: it counts no spike of the units the weights come from. This
note reads both images under the same six forms, every unit's spikes on A's reads and **the drive**, each synapse's weight onto a readout
unit times its presynaptic unit's spikes, and asks:

1. whether every row and read under each form is the logs' of the same form, path, checkpoint and stream, so that every spike and input
   figure is explorations 033's, 034's and 035's;
2. how each class of the arena's units fired on each stimulus's trials under each form, A's and B's kinds and the other units outside the
   readouts among them, right after each replayed night and at block 36;
3. what each replayed night changed in the drive onto each readout by presynaptic group and class: the answer's readout's drive less the
   other's beside the spikes', each class's drive beside its input and its units' spikes, the units that carried half of each readout's
   change against the rest, and how a unit's drive difference lines up with its spikes' difference beside its input's;
4. each of those on the two cells side by side, each readout named by the stimulus whose answer it is.

The note writes no prediction and decides nothing. **Every figure here is exploratory**: from one run of each test on the repository's
AArch64 host, in H-47's terms. A spike count's or a drive's mean is over the cell's 48 reads, sixteen on each of three paths, and a
difference carries its paired SE unless another is named; one is written "beyond 2 SE" where it lies beyond twice its SE. **A drive** is a
sum of Q1.15 weights times spikes, written in thousands (k) of the class matrix's units a trial: a weight of 1, 32 768, times one spike is
32.768 k. **An input** is the mean over the three paths of one image each, in millions (M) of the class matrix's units; no SE is claimed
for an input. In both an inhibitory unit's weight is its magnitude, so a + is more inhibition. Every sum, difference, ratio and per cent is
worked from the unrounded values and rounded once, an exact half away from zero.

**In brief**
- **Every row and read is the logs', and every `DUMP` line of the logs set beside the jobs is in them.** On each cell 105 of 105 rows, 1 680
  of 1 680 of A's reads and 1 680 of 1 680 of the pair's are the logs' of the same form, path, checkpoint and stream, label aside, and so
  are 1 680 of 1 680 unit reads and 189 of 189 input reads; label aside, every `DUMP` line of explorations 033's and 034's jobs is among the
  jobs' on the image from the assignment, and every one of exploration 035's on the image from the mirrored assignment. Held inside the test
  on every trial of every read of A: the readout units' counts among every unit's are the unit read's.
- **The presented stimulus's own units fire in the volley, before the window ADR-0251 named.** On every form and checkpoint of both cells,
  A's kinds on A's trials and B's kinds on B's trials fired 24.93 to 24.98 spikes a trial over the 25 units the stimulus alone
  presents and 25.92 to 25.98 over the 26 the pair's stimulus shares, in ticks 0 to 99 of the trial, and 0.00 in the
  window, ticks 100 to 599. So the harness reads both spans; on the night without replay the volley's drive onto each readout is
  6 052.5 to 6 793.5 k a trial from the trial's stimulus's kinds and 130.1 to 184.6 k from all the other classes together, their
  excitatory and inhibitory magnitudes added, and the
  window's 3 788.1 to 6 338.6 k excitatory and 1 651.8 to 3 354.0 k inhibitory.
- **Right after each single replayed night the window's drive fell onto both readouts on both cells, and the volley's moved toward the
  answer's readout.** In the window, excitatory less inhibitory fell by 717.6 to 1 626.1 k a trial, beyond 2 SE in 32 of 32: the
  readouts' pattern targets and inhibitory units fired less, beyond 2 SE in each of their 32 differences on each cell, and
  77 to 91 per cent of each place group's fall on the image from the assignment and
  74 to 96 on the image from the mirrored assignment lies in the spikes per weight. In the volley the trial's stimulus's kinds' drive
  rose onto its answer's readout by 254.0 to 412.6 k and fell onto the other by 236.9 to 399.4 k, each beyond 2 SE, 96 to 103 per
  cent of each change by the weights.
- **On A's trials A's answer's readout less the other moved with the same sign on both cells in each span; their sum parts the cells in
  the spikes' order.** Less the night without replay, after block 6, 12, 18 and 24 alone:
  - the window's read −417.2 ± 34.0, −581.1 ± 27.6, −606.8 ± 19.9 and −814.7 ± 22.3 k on the image from the assignment, and −692.2 ± 22.0,
    −606.2 ± 18.8, −596.5 ± 20.1 and −571.6 ± 37.2 on the image from the mirrored assignment, each below beyond 2 SE;
  - the volley's read +688.3 ± 11.4, +614.8 ± 8.9, +603.9 ± 11.2 and +480.8 ± 5.5 on the image from the assignment, and +665.1 ± 10.9,
    +819.9 ± 4.9, +784.7 ± 14.8 and +685.7 ± 25.3 on the image from the mirrored assignment, each above beyond 2 SE;
  - their sum read +271.0 ± 42.6, +33.7 ± 23.5, −2.9 ± 18.9 and −333.9 ± 26.9 on the image from the assignment, and −27.1 ± 24.0,
    +213.7 ± 18.6, +188.2 ± 27.5 and +114.1 ± 53.6 on the image from the mirrored assignment.

  At block 36 the window's and the volley's lie on the same sides on both cells; the sum lies on the same side of 2 SE on both cells in 0 of
  8, these four and block 36's. In 12 of 12 distinct comparisons on A's trials, right after, at block 36 and under H-47's plain night, the
  cell whose spikes difference lies higher has the higher sum; on B's trials, B's answer's readout less A's, in 11 of 12.
- **Where the cells part on A's trials.** The sum on the image from the mirrored assignment less that on the image from the assignment read
  −298.2, +179.9, +191.1 and +448.0 k right after the four single nights, of which the window carried −275.0, −25.2, +10.3 and +243.1 and
  the volley −23.2, +205.1, +180.8 and +204.9. At block 36 it read +376.9, +548.1, +555.3 and +419.1, of which the window carried +319.3,
  +566.3, +379.1 and +364.2. Split, the parts by the weights differ by −141.1, +115.7, +152.8 and +175.9 right after, and the parts by the
  spikes per weight by −157.0, +64.2, +38.3 and +272.1; at block 36 by +131.8, +32.1, +179.1 and +69.2 and by +245.2, +516.0, +376.3 and
  +350.0.
- **The first night stands apart from the later three, the other way on each cell.** On A's trials the volley's rise from A's kinds onto A's
  answer's readout less the other read +685.9, +624.3, +597.0 and +490.9 k after block 6, 12, 18 and 24 alone on the image from the
  assignment, the highest after the first night, and +670.3, +812.0, +792.1 and +687.9 on the image from the mirrored assignment, the lowest
  after the first.
- **Twelve blocks bring back part of the window's fall, little of the volley's change.** From right after each single night to block 36,
  excitatory less inhibitory, the window's fall came back by 30 to 83 per cent on the image from the assignment and
  37 to 71 on the image from the mirrored assignment, and the volley's change by −35 to +28 and
  −8 to +22; A's kinds' drive onto each readout on A's trials by −23 to +15 and 0 to +14.
- **The units that carried half of each readout's change lost far more drive a unit than the rest.** Right after each single night the 13 to
  19 such units of a readout on the image from the assignment and 13 to 22 on the image from the mirrored assignment lost 11.48 to 26.13 and
  9.30 to 32.69 k a trial each, both spans, excitatory less inhibitory, against the rest's 1.44 to 2.86 and 1.40 to 2.69. Over each
  readout's units a unit's drive difference, both spans, lines up with its spikes' difference at a correlation of 0.40 to 0.56 and 0.28 to
  0.67, its input's at 0.12 to 0.28 and 0.00 to +0.35.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 020 to 035 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. Nights follow blocks 6, 12, 18 and 24, then 12 blocks run with
none to block 36, on paths 0, 5 and 6. Each checkpoint reads A and the pair sixteen times each, a read one block of 64 trials on a fork of
the image with the reward withheld.

**Two cells**, seed 24's eighth image from the assignment, explorations 033's and 034's, and seed 24's eighth image from the mirrored
assignment, exploration 035's: H-25's two arms on the same network. On the image from the assignment A asks readout 1, the even places, of
stimulus A and readout 0, the odd places, of B, and A's reads print `mirrored true`; on the image from the mirrored assignment A asks
readout 0 of A and readout 1 of B, and A's reads print `mirrored false`; every one of A's reads in the four jobs prints it so, and every
one of the pair's the other way. Where the cells are set side by side each readout is named by the stimulus whose answer it is: **A's
answer's readout** is readout 1 on the image from the assignment and readout 0 on the image from the mirrored assignment, **B's answer's
readout** the other. A period of the ring is 20 units; stimulus A presents place 0 of each period and B place 11. **The odd places** are
the odd places but 11 and **the even places** the even places but 0: readout 0 and readout 1, nine places each over 51 periods, 459 units
each, the same units by id on both cells, whose rotation is 0.

**Every unit by class**, as exploration 020's class matrix names them, from each cell's start image (`e020_class`), which each job and the
trial print for the arena's 1 024 units. A pattern target is a unit that one of A's patterns, drawn from the start image, reaches
past its first unit. The readouts' units:

| Readout | Class | Places | Units: from the assignment | from the mirrored assignment |
| :--- | :--- | ---: | ---: | ---: |
| readout 0, the odd places | C's own | 5 | 25 | 25 |
| readout 0, the odd places | readout 0's pattern targets | 1, 3, 5, 7, 13, 15, 17 | 245 | 280 |
| readout 0, the odd places | readout 0's other excitatory | 1, 3, 5, 7, 13, 15, 17 | 87 | 52 |
| readout 0, the odd places | inhibitory in readout 0 | 9, 19 | 102 | 102 |
| readout 1, the even places | D's own | 16 | 25 | 25 |
| readout 1, the even places | readout 1's pattern targets | 2, 6, 8, 10, 12, 16, 18 | 252 | 254 |
| readout 1, the even places | readout 1's other excitatory | 2, 6, 8, 10, 12, 16, 18 | 80 | 78 |
| readout 1, the even places | inhibitory in readout 1 | 4, 14 | 102 | 102 |

The units outside the readouts' sets:

| Class | Places | Units: from the assignment | from the mirrored assignment |
| :--- | ---: | ---: | ---: |
| 0: A alone | 0 | 25 | 25 |
| 1: A and C | 0 | 26 | 26 |
| 2: B alone | 11 | 25 | 25 |
| 3: B and D | 11 | 26 | 26 |
| 10: other excitatory | 0, 1, 2, 3 | 4 | 4 |
| 13: inhibitory elsewhere | – | 0 | 0 |

The arena's 1 024 units on each cell are the readouts' 918 and these 106: A's and B's kinds, the 102 units the two stimuli
present, and the four other excitatory units at places 0 to 3 that no readout's set holds. No inhibitory unit lies outside the readouts'
sets on either cell.

**Six forms, in two jobs a cell**, explorations 033's to 035's. A night that replays is H-47's plain night on A's patterns: A's 102
patterns, drawn from the start image, tagged at priority 200. Every other night is H-47's night without replay, the same night with nothing
tagged. No hold is set. Each night is chosen by its form (`E030Form`), and every night of every form is watched. Job one runs the night
without replay, after block 6 alone and H-47's plain night on all four nights, nine chains; job two the night without replay and after
block 12, 18 or 24 alone, twelve chains.

**The unit read** is exploration 033's: in each of A's reads, on each trial, each unit of the two readouts' sets has its spikes counted in
that trial's readout window, ticks 100 to 599 of the trial, from the executor's own train, by `Readout::count_window`'s rule. **The input
read** is exploration 034's: on each image whose class matrix the job writes, the start image, the image before each night and the image
after it, and block 36's, the weights of the synapses onto each readout unit summed by the presynaptic unit's class, an inhibitory unit's
negated, so a + is more inhibition. Both are run as exploration 035 ran them, and both hold inside the test as there.

**Every unit's spikes**: in each of A's reads, on each trial, every unit of the arena has its spikes counted in the same window by the same
rule, and each unit's spikes are summed over the read's trials of each stimulus. Held inside the test on every trial: the readout units'
counts among every unit's are the unit read's.

**The drive**: in each of A's reads, on each trial, for each readout unit and each presynaptic class, the sum over the synapses onto it
from that class of the synapse's weight at the trial's end, Q1.15, times its presynaptic unit's spikes in the trial's window, an inhibitory
unit's negated, so a + is more inhibition. It is summed over each stimulus's trials of the read and dumped summed over each readout's units
by class; each trial's drive times that trial's end gain (`synaptic_gain_q16`), summed the same way, is dumped beside it; and each readout
unit's drive by class is dumped for each image, summed over its sixteen reads of A. The walk starts from each unit's fan-out head on the
start image and reads the executor's blocks between trials, the units that spiked alone; held inside the test: each image's heads are the
start image's, and on each read's first trial the walk over every unit gives the same drive. With every presynaptic count one the drive is
the input read, held by a unit test on synthetic synapses. A readout's drive and a class's drive are written in thousands (k) of the class
matrix's units a trial: a weight of 1, 32 768, times one spike.

**The weights move during a read**, and the drive takes each synapse's weight at each trial's end. After each read the job prints the
readouts' inputs by class at the read's end. Set beside the input read of the image the read forks from, by this note's parser, the
excitatory classes' sums are the image's on every read at blocks 6 to 24, and at block 36 they moved on 176 of 336
reads on the image from the assignment and 192 of 336 on the image from the mirrored assignment, by 0.340 and
0.055 per cent of a class's sum at most. The inhibitory sums moved on every read, by −0.72 to +0.38
per cent of a readout's on the image from the assignment and −0.81 to +0.44 on the image from the mirrored assignment.

**The volley, read beside the window.** ADR-0251 ordered the drive with the presynaptic spikes in the trial's window. The harness's first
trial, run [43187226229](https://github.com/Menteion/Menteion/actions/runs/43187226229), showed that the presented stimulus's own units fire
almost no spike there, at most one in all over a read's 34 trials of A or 30 of B on each of its 12 reads: their volley lies in ticks 0 to
99 of the trial, before the window. So the window's drive holds almost none of the input from the trial's own stimulus's kinds that
explorations 034 and 035 read moving toward the answer's readout, and the four jobs already dispatched, run
[43189318519](https://github.com/Menteion/Menteion/actions/runs/43189318519), were cancelled. The harness, commit `816d89e`, also counts
every unit's spikes in **the volley**, ticks 0 to 99 of each trial, by the same rule, and reads the drive with those spikes the same way,
without and with the gain, by class over each readout's units after each read and unit by unit over each image's sixteen reads; the window's
drive is ADR-0251's and is unchanged. Held inside the test on every trial: the train holds the whole volley, its oldest spike from before
the trial's first tick wherever the ring has let any go, as exploration 033's guard holds the window. This note reads both spans, and
**both** is their sum, ticks 0 to 599. **The drive pairs a weight with its presynaptic unit's spikes in the same span, not after the
synapse's delay**: a spike late in the volley that arrives in the window counts in the volley's drive, and one late in the window that
arrives after it counts in the window's.

On every form and checkpoint of both cells A's kinds on A's trials and B's kinds on B's trials fired 24.93 to 24.98 spikes a trial in
the volley over the 25 units the stimulus alone presents and 25.92 to 25.98 over the 26 the pair's stimulus shares, about one a unit,
and 0.00 in the window; all the other units of the arena together fired 0.65 to 1.43 spikes a trial in the volley. In
the window each stimulus's kinds fired on the other stimulus's trials, 0.08 to 1.16 spikes a trial a class.

**The presynaptic groups** are exploration 034's, by place: A's kinds (classes 0 and 1), B's kinds (2 and 3), the odd places' excitatory
units (4, 6 and 7: C's own and readout 0's pattern targets and other excitatory units), the even places' (5, 8 and 9), the other excitatory
units (10) and the inhibitory units (11 to 13). **A readout's own place group** is the odd places' for readout 0 and the even places' for
readout 1, and **the other place group** the other.

**The SEs**, exploration 033's. A form less the night without replay carries H-47's paired SE over its 48 differences, the night without
replay of its own job. A count's, a spike count's or a drive's mean is over the cell's 48 reads, sixteen on each of three paths, and its
difference is written with its paired SE; one is written "beyond 2 SE" where it lies beyond twice its SE. **An input is the mean over the
three paths** of one image each, in millions (M) of the class matrix's units, as explorations 034 and 035 give them; no SE is claimed for
an input, and none for **a unit's drive**, the sum over its sixteen reads of each checkpoint, the mean over the three paths, divided by the
trials of each stimulus, so a unit's drive a trial. Every sum, difference, ratio and per cent is worked from the unrounded values and
rounded once, an exact half away from zero.

**The split of a drive's difference.** A readout's drive from a group is its input from the group times the drive a unit of weight brings,
**the spikes per weight**: the drive a trial over the input, the input the image's, the mean over the three paths. A form's drive less the
night without replay's is split exactly into **by the weights**, the input's difference times the night without replay's spikes per weight,
and **by the spikes per weight**, the form's input times the difference in spikes per weight. The second carries everything that is not the
summed weight: the presynaptic units' spikes, which synapses carry them, and the inhibitory weights' change during a read. It is arithmetic
on the logged values, not a measured cause. It is made class by class, and a group's or the net's parts are its classes'
summed, the inhibitory classes' subtracted in the net's.

**A checkpoint is read after the night that follows its block**, and block 36's after the twelve blocks that follow the last night. So
right after its replayed night, a single night's form less the night without replay is that night's replay alone: until then the two ran
the same nights on the same images. H-47's plain night less the night without replay at blocks 6, 12, 18 and 24 is its nights so far
together.

## Before a job ran

ADR-0251 asks whether any form must run bit for bit as one already in the logs. **Each of the six does**, on both cells: every image, row
and read of every form is in explorations 033's, 034's and 035's logs. **No log holds every unit's spikes or the drive**, so each form ran
again, and its rows and reads are set beside those logs' here.

## The runs are laid out as written

**The first trial and the jobs cancelled.** The harness's first trial, run
[43187226229](https://github.com/Menteion/Menteion/actions/runs/43187226229) at `b0f3229`, took 638 s and passed; what it showed
of the window is under What ran. The four jobs dispatched at the same commit, run
[43189318519](https://github.com/Menteion/Menteion/actions/runs/43189318519), were cancelled within five minutes of starting; none
finished, and nothing of theirs is read here. Commit `816d89e` adds the volley; label aside, 2 548 of 2 560 `DUMP` lines of the first trial
are among the second's, and the other 12 are its timings.

**The trial**, run [43190778334](https://github.com/Menteion/Menteion/actions/runs/43190778334) at `816d89e`, took 641 s. It ran
the pair's head and each form's first night on path 0 of each cell and read A once at block 6 after each:

| The cell | Form | Its first night | H-47's row at block 6 on path 0 it holds, inside the test | A's read as it was | The pair's | Replays | Depotentiations | Watch: wrong | Trials of A, of B | Every unit's spikes: the readout units' | The drive by readout | Excitatory input at the read's end | Inhibitory input at the read's end less the image's, onto readout 0, readout 1 | The window: A's kinds' spikes on A's trials, B's kinds' on B's trials | The volley: the same | The volley's drive by readout | A's read with the unit read alone, ms | with every unit and the drive, ms | of which the counts and the walks, ms |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | :--- | :--- | :--- | :--- | ---: | ---: | :--- | ---: | ---: | ---: |
| From the assignment | Without replay | without replay | the night without replay's | 43 | 60 | 0 | 0 | 0 | 34, 30 | the unit read's | its units' summed | the image's | +10 602, +22 959 | 0, 0 | 1 728, 1 526 | its units' summed | 5 917 | 5 709 | 6 |
| From the assignment | After block 6 alone | replays | the plain night's | 42 | 63 | 1 329 | 640 | 0 | 34, 30 | the unit read's | its units' summed | the image's | −185 465, −188 984 | 0, 1 | 1 732, 1 526 | its units' summed | 5 808 | 5 769 | 6 |
| From the assignment | Plain on all four nights (H-47) | replays | the plain night's | 42 | 63 | 1 329 | 640 | 0 | 34, 30 | the unit read's | its units' summed | the image's | −185 465, −188 984 | 0, 1 | 1 732, 1 526 | its units' summed | 5 611 | 5 834 | 6 |
| From the assignment | After block 12 alone | without replay | the night without replay's | 43 | 60 | 0 | 0 | 0 | 34, 30 | the unit read's | its units' summed | the image's | +10 602, +22 959 | 0, 0 | 1 728, 1 526 | its units' summed | 5 716 | 6 034 | 6 |
| From the assignment | After block 18 alone | without replay | the night without replay's | 43 | 60 | 0 | 0 | 0 | 34, 30 | the unit read's | its units' summed | the image's | +10 602, +22 959 | 0, 0 | 1 728, 1 526 | its units' summed | 5 670 | 5 947 | 6 |
| From the assignment | After block 24 alone | without replay | the night without replay's | 43 | 60 | 0 | 0 | 0 | 34, 30 | the unit read's | its units' summed | the image's | +10 602, +22 959 | 0, 0 | 1 728, 1 526 | its units' summed | 5 947 | 5 997 | 6 |
| From the mirrored assignment | Without replay | without replay | the night without replay's | 33 | 60 | 0 | 0 | 0 | 34, 30 | the unit read's | its units' summed | the image's | +10 902, +3 892 | 0, 0 | 1 729, 1 526 | its units' summed | 6 109 | 5 891 | 6 |
| From the mirrored assignment | After block 6 alone | replays | the plain night's | 28 | 62 | 1 330 | 640 | 0 | 34, 30 | the unit read's | its units' summed | the image's | −198 837, −194 729 | 0, 0 | 1 733, 1 528 | its units' summed | 5 737 | 5 974 | 6 |
| From the mirrored assignment | Plain on all four nights (H-47) | replays | the plain night's | 28 | 62 | 1 330 | 640 | 0 | 34, 30 | the unit read's | its units' summed | the image's | −198 837, −194 729 | 0, 0 | 1 733, 1 528 | its units' summed | 5 634 | 5 729 | 6 |
| From the mirrored assignment | After block 12 alone | without replay | the night without replay's | 33 | 60 | 0 | 0 | 0 | 34, 30 | the unit read's | its units' summed | the image's | +10 902, +3 892 | 0, 0 | 1 729, 1 526 | its units' summed | 6 132 | 5 913 | 6 |
| From the mirrored assignment | After block 18 alone | without replay | the night without replay's | 33 | 60 | 0 | 0 | 0 | 34, 30 | the unit read's | its units' summed | the image's | +10 902, +3 892 | 0, 0 | 1 729, 1 526 | its units' summed | 5 984 | 5 901 | 6 |
| From the mirrored assignment | After block 24 alone | without replay | the night without replay's | 33 | 60 | 0 | 0 | 0 | 34, 30 | the unit read's | its units' summed | the image's | +10 902, +3 892 | 0, 0 | 1 729, 1 526 | its units' summed | 6 021 | 5 950 | 6 |

Each first night held H-47's row at block 6 on path 0 of its form, inside the test. On each read, by this note's parser, the readout units'
slice of every unit's spikes is the unit read's, and in each span the drive by readout is its units' drive summed; the excitatory input at
the read's end is the image's, and the inhibitory input moved over the read, up by 3 892 to 22 959 of the class matrix's units onto a
readout on the reads after a night without replay and down by 185 465 to 198 837 after a replayed one. Label aside, 1 240 of 1 240 `DUMP`
lines of exploration 034's trial and 1 246 of 1 246 of exploration 035's are among its `DUMP` lines.

A read of A with every unit counted and the drive walked took −219 to +318 ms more than with the unit read alone, +30 ms
on the mean of the twelve; of that the counts and the walks took 6 ms. Each job's projection adds that mean over its 720 or 960
reads of A to exploration 034's or 035's job time: 5 844 and 7 954 s on the image from the assignment, 5 993 and
7 514 s on the image from the mirrored assignment, the longest 132.6 minutes, under an arms job's 240. No form was dropped.

**The jobs**, run [43192758106](https://github.com/Menteion/Menteion/actions/runs/43192758106) at `816d89e`, two a cell:

| The cell | Job | Form | Rows held inside the test to H-47's, on every path | Of its 15 rows, H-47's night without replay's and its plain night's, as the job prints | Replayed nights: replays and depotentiations | Other nights reading replays 0 and depotentiations 0 | Watch: wrong |
| :--- | :--- | :--- | :--- | ---: | :--- | :--- | ---: |
| From the assignment | Job one | Without replay | the night without replay's, blocks 6 to 36 | 15, 0 | 0: –, – | 12 of 12 | 0 |
| From the assignment | Job one | After block 6 alone | the plain night's, block 6 | 0, 3 | 3: 1 329, 640 | 9 of 9 | 0 |
| From the assignment | Job one | Plain on all four nights (H-47) | the plain night's, blocks 6 to 36 | 0, 15 | 12: 1 129 to 1 329, 545 to 640 | 0 of 0 | 0 |
| From the assignment | Job two | Without replay | the night without replay's, blocks 6 to 36 | 15, 0 | 0: –, – | 12 of 12 | 0 |
| From the assignment | Job two | After block 12 alone | the night without replay's, block 6 | 3, 0 | 3: 1 341, 640 | 9 of 9 | 0 |
| From the assignment | Job two | After block 18 alone | the night without replay's, blocks 6 and 12 | 6, 0 | 3: 1 341, 640 | 9 of 9 | 0 |
| From the assignment | Job two | After block 24 alone | the night without replay's, blocks 6, 12 and 18 | 9, 0 | 3: 1 341, 640 | 9 of 9 | 0 |
| From the mirrored assignment | Job one | Without replay | the night without replay's, blocks 6 to 36 | 15, 0 | 0: –, – | 12 of 12 | 0 |
| From the mirrored assignment | Job one | After block 6 alone | the plain night's, block 6 | 0, 3 | 3: 1 330, 640 | 9 of 9 | 0 |
| From the mirrored assignment | Job one | Plain on all four nights (H-47) | the plain night's, blocks 6 to 36 | 0, 15 | 12: 1 129 to 1 330, 545 to 640 | 0 of 0 | 0 |
| From the mirrored assignment | Job two | Without replay | the night without replay's, blocks 6 to 36 | 15, 0 | 0: –, – | 12 of 12 | 0 |
| From the mirrored assignment | Job two | After block 12 alone | the night without replay's, block 6 | 3, 0 | 3: 1 341, 640 | 9 of 9 | 0 |
| From the mirrored assignment | Job two | After block 18 alone | the night without replay's, blocks 6 and 12 | 6, 0 | 3: 1 341, 640 | 9 of 9 | 0 |
| From the mirrored assignment | Job two | After block 24 alone | the night without replay's, blocks 6, 12 and 18 | 9, 0 | 3: 1 341, 640 | 9 of 9 | 0 |

Each of A's reads in the four jobs prints `mirrored true` on the image from the assignment and `mirrored false` on the image from the
mirrored assignment, and each of the pair's the other way. A read held 28 to 37 trials of A and 27 to 36 of B, the same counts on both
cells. What each job read, by this note's parser:

| The cell | Job | Images read | Their inputs the class matrix's 112 entries, by this note's parser | Entries | Lines saying so, as the job prints | Images whose splits give the same sums by kind onto each readout | Unit reads whose trials and sums by readout are their read's | Every unit's spikes whose readout units are the unit read's | Images whose window drive unit by unit sums to their sixteen reads' drive by readout | Images whose volley drive does |
| :--- | :--- | ---: | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| From the assignment | Job one | 82 | 82 of 82 | 9 184 of 9 184 | 82 of 82 | 82 of 82 | 720 of 720 | 720 of 720 | 45 of 45 | 45 of 45 |
| From the assignment | Job two | 109 | 109 of 109 | 12 208 of 12 208 | 109 of 109 | 109 of 109 | 960 of 960 | 960 of 960 | 60 of 60 | 60 of 60 |
| From the mirrored assignment | Job one | 82 | 82 of 82 | 9 184 of 9 184 | 82 of 82 | 82 of 82 | 720 of 720 | 720 of 720 | 45 of 45 | 45 of 45 |
| From the mirrored assignment | Job two | 109 | 109 of 109 | 12 208 of 12 208 | 109 of 109 | 109 of 109 | 960 of 960 | 960 of 960 | 60 of 60 | 60 of 60 |

The night without replay ran in both jobs of each cell. On each cell its reads of A and of the pair are the same line in both jobs,
480 of 480; by this note's parser, in each span, its reads' drive by readout, their every-unit sums and its images' unit
drives are the same in both jobs too, 240 of 240, 240 of 240 and 15 of 15. On every one of the 1 680 of 1 680
reads of A on each cell the volley's lines name the same trials as the window's.

## 1. The rows and reads against the logs

| The cell | Job | Form | Logs set beside | Rows that are the logs' rows | A's reads with the logs' count and masks | their whole line, label aside | The pair's reads with the logs' count and masks | their whole line | Unit reads that are the logs' | Input reads that are the logs' |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| From the assignment | Job one | Without replay | exploration 033's, run 43011654795 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | – |
| From the assignment | Job one | Without replay | exploration 034's, run 43052358536 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 27 of 27 |
| From the assignment | Job one | After block 6 alone | exploration 033's, run 43011654795 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | – |
| From the assignment | Job one | After block 6 alone | exploration 034's, run 43052358536 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 27 of 27 |
| From the assignment | Job one | Plain on all four nights (H-47) | exploration 033's, run 43011654795 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | – |
| From the assignment | Job one | Plain on all four nights (H-47) | exploration 034's, run 43052358536 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 27 of 27 |
| From the assignment | Job two | Without replay | exploration 033's, run 43011654795 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | – |
| From the assignment | Job two | Without replay | exploration 034's, run 43052358536 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 27 of 27 |
| From the assignment | Job two | After block 12 alone | exploration 033's, run 43011654795 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | – |
| From the assignment | Job two | After block 12 alone | exploration 034's, run 43052358536 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 27 of 27 |
| From the assignment | Job two | After block 18 alone | exploration 033's, run 43011654795 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | – |
| From the assignment | Job two | After block 18 alone | exploration 034's, run 43052358536 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 27 of 27 |
| From the assignment | Job two | After block 24 alone | exploration 033's, run 43011654795 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | – |
| From the assignment | Job two | After block 24 alone | exploration 034's, run 43052358536 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 27 of 27 |
| From the mirrored assignment | Job one | Without replay | exploration 035's, run 43134386489 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 27 of 27 |
| From the mirrored assignment | Job one | After block 6 alone | exploration 035's, run 43134386489 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 27 of 27 |
| From the mirrored assignment | Job one | Plain on all four nights (H-47) | exploration 035's, run 43134386489 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 27 of 27 |
| From the mirrored assignment | Job two | Without replay | exploration 035's, run 43134386489 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 27 of 27 |
| From the mirrored assignment | Job two | After block 12 alone | exploration 035's, run 43134386489 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 27 of 27 |
| From the mirrored assignment | Job two | After block 18 alone | exploration 035's, run 43134386489 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 27 of 27 |
| From the mirrored assignment | Job two | After block 24 alone | exploration 035's, run 43134386489 | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 | 27 of 27 |

Label aside, each `DUMP` line of the jobs set beside them is among the job's `DUMP` lines:

| The cell | Job | Log set beside | Its `DUMP` lines | Among the job's `DUMP` lines, label aside |
| :--- | :--- | :--- | ---: | :--- |
| From the assignment | Job one | exploration 033's job, run 43011654795 | 10 480 | 10 480 of 10 480 |
| From the assignment | Job one | exploration 034's job, run 43052358536 | 10 644 | 10 644 of 10 644 |
| From the assignment | Job two | exploration 033's job, run 43011654795 | 10 589 | 10 589 of 10 589 |
| From the assignment | Job two | exploration 034's job, run 43052358536 | 10 807 | 10 807 of 10 807 |
| From the mirrored assignment | Job one | exploration 035's job, run 43134386489 | 10 645 | 10 645 of 10 645 |
| From the mirrored assignment | Job two | exploration 035's job, run 43134386489 | 10 826 | 10 826 of 10 826 |

Each job's other `DUMP` lines, 2 971 and 3 961 on each cell for job one and job two, are exploration 036's own and of no other kind, by this
note's parser: one line naming every unit's class; after each of A's reads, every unit's spikes and the drive by readout in each span, four
lines; and each image's unit drive in each span, two lines. A's count under each form, less the night without replay, is therefore
explorations 033's and 035's:

| Form | Block | From the assignment: A's count less without replay | on A's trials | on B's trials | From the mirrored assignment: A's count less without replay | on A's trials | on B's trials |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone | 6 | +0.3 ± 0.64 | +2.5 ± 0.77 | −2.3 ± 0.71 | −2.6 ± 1.35 | −2.1 ± 0.99 | −0.5 ± 1.17 |
| After block 12 alone | 12 | −7.3 ± 0.88 | −5.2 ± 0.70 | −2.1 ± 0.59 | +0.8 ± 1.09 | +0.5 ± 0.83 | +0.2 ± 0.60 |
| After block 18 alone | 18 | −12.1 ± 0.63 | −8.1 ± 0.66 | −4.0 ± 0.57 | +9.1 ± 0.92 | +6.0 ± 0.68 | +3.1 ± 1.02 |
| After block 24 alone | 24 | −12.3 ± 0.58 | −10.0 ± 0.33 | −2.3 ± 0.62 | +5.4 ± 2.02 | +1.9 ± 0.99 | +3.5 ± 1.23 |
| After block 6 alone | 36 | −2.0 ± 0.88 | −1.0 ± 0.63 | −1.0 ± 0.71 | +3.1 ± 1.84 | +0.7 ± 0.86 | +2.4 ± 1.06 |
| After block 12 alone | 36 | −2.1 ± 1.78 | −1.2 ± 0.90 | −0.9 ± 1.04 | +3.8 ± 0.78 | +0.7 ± 0.29 | +3.1 ± 1.02 |
| After block 18 alone | 36 | −5.5 ± 0.95 | −4.1 ± 0.51 | −1.4 ± 0.88 | +5.4 ± 1.17 | +1.7 ± 0.54 | +3.7 ± 0.86 |
| After block 24 alone | 36 | −4.6 ± 0.73 | −5.5 ± 0.75 | +0.9 ± 0.13 | +1.9 ± 1.18 | −0.8 ± 0.34 | +2.7 ± 0.89 |
| Plain on all four nights (H-47) | 6 | +0.3 ± 0.64 | +2.5 ± 0.77 | −2.3 ± 0.71 | −2.6 ± 1.35 | −2.1 ± 0.99 | −0.5 ± 1.17 |
| Plain on all four nights (H-47) | 12 | −0.6 ± 1.03 | +0.4 ± 1.31 | −1.0 ± 1.51 | −2.9 ± 1.19 | −3.2 ± 0.72 | +0.3 ± 0.77 |
| Plain on all four nights (H-47) | 18 | −5.9 ± 1.53 | −3.6 ± 0.86 | −2.4 ± 1.34 | +8.3 ± 1.70 | +0.7 ± 0.25 | +7.6 ± 1.52 |
| Plain on all four nights (H-47) | 24 | −5.9 ± 1.85 | −3.9 ± 0.78 | −2.0 ± 1.34 | +9.4 ± 1.58 | +1.6 ± 1.33 | +7.8 ± 1.47 |
| Plain on all four nights (H-47) | 36 | +1.8 ± 1.79 | +0.2 ± 0.76 | +1.6 ± 1.50 | +11.3 ± 0.96 | +1.5 ± 0.74 | +9.7 ± 0.98 |

## 2. Every unit's spikes by class

Each class's spikes a trial, summed over its units, on the night without replay, in the window and in the volley:

| Block | Class | From the assignment: the window, A's trials | B's trials | the volley, A's trials | B's trials | From the mirrored assignment: the window, A's trials | B's trials | the volley, A's trials | B's trials |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | 0: A alone | 0.00 | 0.20 | 24.94 | 0.02 | 0.00 | 0.20 | 24.95 | 0.02 |
| 6 | 1: A and C | 0.00 | 0.22 | 25.93 | 0.03 | 0.00 | 0.24 | 25.93 | 0.03 |
| 6 | 2: B alone | 0.16 | 0.00 | 0.04 | 24.94 | 0.16 | 0.00 | 0.04 | 24.95 |
| 6 | 3: B and D | 0.57 | 0.00 | 0.03 | 25.93 | 0.86 | 0.00 | 0.03 | 25.93 |
| 6 | 4: C's own | 0.36 | 1.55 | 0.03 | 0.05 | 0.64 | 1.18 | 0.03 | 0.05 |
| 6 | 5: D's own | 1.00 | 1.23 | 0.03 | 0.03 | 0.64 | 1.29 | 0.03 | 0.03 |
| 6 | 6: readout 0's pattern targets | 16.64 | 12.15 | 0.33 | 0.34 | 20.88 | 9.96 | 0.39 | 0.39 |
| 6 | 7: readout 0's other excitatory | 3.40 | 1.36 | 0.12 | 0.12 | 1.29 | 2.21 | 0.08 | 0.06 |
| 6 | 8: readout 1's pattern targets | 15.45 | 11.83 | 0.32 | 0.29 | 11.86 | 15.19 | 0.34 | 0.27 |
| 6 | 9: readout 1's other excitatory | 1.06 | 3.66 | 0.10 | 0.08 | 5.74 | 0.80 | 0.10 | 0.10 |
| 6 | 10: other excitatory | 0.17 | 0.04 | 0.01 | 0.01 | 0.23 | 0.10 | 0.01 | 0.01 |
| 6 | 11: inhibitory in readout 0 | 4.44 | 10.71 | 0.21 | 0.18 | 6.47 | 12.34 | 0.20 | 0.20 |
| 6 | 12: inhibitory in readout 1 | 9.84 | 7.77 | 0.17 | 0.16 | 9.43 | 9.34 | 0.17 | 0.16 |
| 12 | 0: A alone | 0.00 | 0.17 | 24.93 | 0.03 | 0.00 | 0.20 | 24.93 | 0.02 |
| 12 | 1: A and C | 0.00 | 0.23 | 25.95 | 0.03 | 0.00 | 0.26 | 25.94 | 0.03 |
| 12 | 2: B alone | 0.19 | 0.00 | 0.03 | 24.94 | 0.18 | 0.00 | 0.03 | 24.94 |
| 12 | 3: B and D | 0.80 | 0.00 | 0.03 | 25.94 | 1.12 | 0.00 | 0.04 | 25.94 |
| 12 | 4: C's own | 0.35 | 1.49 | 0.04 | 0.03 | 0.76 | 1.22 | 0.03 | 0.03 |
| 12 | 5: D's own | 0.97 | 1.43 | 0.03 | 0.02 | 0.72 | 1.26 | 0.03 | 0.03 |
| 12 | 6: readout 0's pattern targets | 18.15 | 12.88 | 0.35 | 0.32 | 21.88 | 11.17 | 0.37 | 0.34 |
| 12 | 7: readout 0's other excitatory | 3.85 | 1.80 | 0.11 | 0.10 | 1.40 | 2.40 | 0.08 | 0.07 |
| 12 | 8: readout 1's pattern targets | 15.76 | 12.84 | 0.32 | 0.28 | 12.68 | 16.01 | 0.32 | 0.29 |
| 12 | 9: readout 1's other excitatory | 1.31 | 3.99 | 0.08 | 0.08 | 6.65 | 0.94 | 0.08 | 0.08 |
| 12 | 10: other excitatory | 0.25 | 0.08 | 0.01 | 0.01 | 0.46 | 0.15 | 0.01 | 0.01 |
| 12 | 11: inhibitory in readout 0 | 5.30 | 11.53 | 0.16 | 0.14 | 7.12 | 12.72 | 0.16 | 0.16 |
| 12 | 12: inhibitory in readout 1 | 10.79 | 8.60 | 0.16 | 0.19 | 11.24 | 10.00 | 0.17 | 0.18 |
| 18 | 0: A alone | 0.00 | 0.18 | 24.94 | 0.04 | 0.00 | 0.20 | 24.94 | 0.04 |
| 18 | 1: A and C | 0.00 | 0.28 | 25.93 | 0.04 | 0.00 | 0.29 | 25.93 | 0.03 |
| 18 | 2: B alone | 0.19 | 0.00 | 0.02 | 24.94 | 0.18 | 0.00 | 0.02 | 24.94 |
| 18 | 3: B and D | 1.00 | 0.00 | 0.04 | 25.94 | 1.11 | 0.00 | 0.03 | 25.94 |
| 18 | 4: C's own | 0.38 | 1.49 | 0.03 | 0.03 | 0.92 | 1.47 | 0.03 | 0.03 |
| 18 | 5: D's own | 0.96 | 1.54 | 0.03 | 0.02 | 0.82 | 1.25 | 0.03 | 0.02 |
| 18 | 6: readout 0's pattern targets | 19.43 | 13.64 | 0.33 | 0.33 | 22.13 | 12.53 | 0.38 | 0.35 |
| 18 | 7: readout 0's other excitatory | 4.04 | 2.35 | 0.12 | 0.09 | 1.33 | 2.50 | 0.05 | 0.08 |
| 18 | 8: readout 1's pattern targets | 16.79 | 13.89 | 0.28 | 0.33 | 13.90 | 16.14 | 0.29 | 0.31 |
| 18 | 9: readout 1's other excitatory | 1.59 | 4.26 | 0.09 | 0.09 | 7.32 | 1.34 | 0.07 | 0.10 |
| 18 | 10: other excitatory | 0.33 | 0.10 | 0.01 | 0.01 | 0.82 | 0.15 | 0.01 | 0.02 |
| 18 | 11: inhibitory in readout 0 | 5.99 | 11.98 | 0.18 | 0.22 | 7.44 | 13.64 | 0.16 | 0.22 |
| 18 | 12: inhibitory in readout 1 | 11.64 | 9.04 | 0.16 | 0.20 | 12.09 | 10.12 | 0.15 | 0.19 |
| 24 | 0: A alone | 0.00 | 0.18 | 24.95 | 0.03 | 0.00 | 0.18 | 24.95 | 0.02 |
| 24 | 1: A and C | 0.00 | 0.31 | 25.94 | 0.03 | 0.00 | 0.35 | 25.94 | 0.03 |
| 24 | 2: B alone | 0.19 | 0.00 | 0.03 | 24.95 | 0.16 | 0.00 | 0.02 | 24.96 |
| 24 | 3: B and D | 1.06 | 0.00 | 0.03 | 25.94 | 1.06 | 0.00 | 0.03 | 25.94 |
| 24 | 4: C's own | 0.33 | 1.42 | 0.02 | 0.03 | 1.15 | 1.57 | 0.03 | 0.03 |
| 24 | 5: D's own | 0.82 | 1.85 | 0.04 | 0.03 | 0.73 | 1.26 | 0.03 | 0.02 |
| 24 | 6: readout 0's pattern targets | 19.88 | 14.26 | 0.30 | 0.32 | 22.41 | 13.87 | 0.29 | 0.35 |
| 24 | 7: readout 0's other excitatory | 4.12 | 2.86 | 0.08 | 0.14 | 1.39 | 2.57 | 0.07 | 0.07 |
| 24 | 8: readout 1's pattern targets | 17.51 | 14.66 | 0.33 | 0.30 | 14.66 | 16.49 | 0.35 | 0.32 |
| 24 | 9: readout 1's other excitatory | 1.87 | 4.62 | 0.09 | 0.08 | 7.89 | 1.88 | 0.09 | 0.09 |
| 24 | 10: other excitatory | 0.19 | 0.10 | 0.01 | 0.01 | 0.80 | 0.22 | 0.01 | 0.01 |
| 24 | 11: inhibitory in readout 0 | 6.89 | 12.53 | 0.19 | 0.20 | 7.87 | 14.28 | 0.19 | 0.20 |
| 24 | 12: inhibitory in readout 1 | 12.77 | 9.51 | 0.17 | 0.17 | 12.89 | 10.39 | 0.17 | 0.16 |
| 36 | 0: A alone | 0.00 | 0.16 | 24.96 | 0.03 | 0.00 | 0.14 | 24.96 | 0.03 |
| 36 | 1: A and C | 0.00 | 0.26 | 25.96 | 0.03 | 0.00 | 0.29 | 25.96 | 0.03 |
| 36 | 2: B alone | 0.12 | 0.00 | 0.02 | 24.97 | 0.10 | 0.00 | 0.02 | 24.97 |
| 36 | 3: B and D | 0.91 | 0.00 | 0.03 | 25.94 | 1.05 | 0.00 | 0.03 | 25.94 |
| 36 | 4: C's own | 0.22 | 1.41 | 0.02 | 0.03 | 1.07 | 1.53 | 0.02 | 0.03 |
| 36 | 5: D's own | 0.79 | 1.81 | 0.02 | 0.02 | 0.71 | 1.22 | 0.02 | 0.02 |
| 36 | 6: readout 0's pattern targets | 19.35 | 13.15 | 0.22 | 0.25 | 20.77 | 13.24 | 0.25 | 0.26 |
| 36 | 7: readout 0's other excitatory | 4.13 | 2.79 | 0.08 | 0.07 | 1.26 | 2.47 | 0.05 | 0.05 |
| 36 | 8: readout 1's pattern targets | 15.38 | 14.10 | 0.29 | 0.23 | 14.07 | 15.82 | 0.29 | 0.26 |
| 36 | 9: readout 1's other excitatory | 1.46 | 4.38 | 0.07 | 0.09 | 7.68 | 1.78 | 0.07 | 0.08 |
| 36 | 10: other excitatory | 0.17 | 0.08 | 0.01 | 0.01 | 0.76 | 0.16 | 0.01 | 0.01 |
| 36 | 11: inhibitory in readout 0 | 6.36 | 11.31 | 0.15 | 0.15 | 6.97 | 13.91 | 0.15 | 0.14 |
| 36 | 12: inhibitory in readout 1 | 11.81 | 9.13 | 0.16 | 0.15 | 12.29 | 9.40 | 0.14 | 0.15 |

In the window each stimulus's kinds fired 0.00 spikes a trial on its own stimulus's trials, to two places, and 0.08 to 1.16 a class on the
other's, on every form and checkpoint. In the volley the trial's stimulus's kinds fired about once a unit, and all the other classes
together fired 0.65 to 1.43 spikes a trial.

**Right after each single replayed night**, less the night without replay:

| Comparison | Class | From the assignment: the window, A's trials | B's trials | the volley, A's trials | B's trials | From the mirrored assignment: the window, A's trials | B's trials | the volley, A's trials | B's trials |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | 0: A alone | 0.00 ± 0.00 | −0.09 ± 0.01 | +0.03 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.08 ± 0.02 | +0.02 ± 0.01 | 0.00 ± 0.01 |
| After block 6 alone less without replay, block 6 | 1: A and C | 0.00 ± 0.00 | −0.07 ± 0.02 | +0.03 ± 0.01 | −0.02 ± 0.01 | 0.00 ± 0.00 | −0.13 ± 0.02 | +0.03 ± 0.01 | −0.03 ± 0.01 |
| After block 6 alone less without replay, block 6 | 2: B alone | −0.05 ± 0.01 | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.01 ± 0.01 | −0.08 ± 0.01 | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.03 ± 0.01 |
| After block 6 alone less without replay, block 6 | 3: B and D | −0.23 ± 0.04 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.01 ± 0.01 | −0.13 ± 0.04 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.03 ± 0.01 |
| After block 6 alone less without replay, block 6 | 4: C's own | −0.17 ± 0.02 | −0.19 ± 0.03 | 0.00 ± 0.01 | −0.02 ± 0.01 | −0.31 ± 0.03 | −0.23 ± 0.03 | −0.01 ± 0.01 | −0.02 ± 0.01 |
| After block 6 alone less without replay, block 6 | 5: D's own | −0.35 ± 0.03 | −0.53 ± 0.03 | −0.01 ± 0.01 | −0.01 ± 0.01 | −0.18 ± 0.03 | −0.06 ± 0.02 | 0.00 ± 0.01 | 0.00 ± 0.01 |
| After block 6 alone less without replay, block 6 | 6: readout 0's pattern targets | −5.47 ± 0.18 | −2.73 ± 0.09 | −0.09 ± 0.03 | −0.11 ± 0.02 | −5.56 ± 0.14 | −3.14 ± 0.13 | −0.21 ± 0.03 | −0.22 ± 0.03 |
| After block 6 alone less without replay, block 6 | 7: readout 0's other excitatory | −0.91 ± 0.05 | −0.73 ± 0.05 | −0.05 ± 0.02 | −0.06 ± 0.02 | −0.35 ± 0.04 | −0.35 ± 0.05 | −0.05 ± 0.01 | −0.02 ± 0.02 |
| After block 6 alone less without replay, block 6 | 8: readout 1's pattern targets | −3.70 ± 0.12 | −2.75 ± 0.10 | −0.11 ± 0.02 | −0.14 ± 0.03 | −3.02 ± 0.09 | −3.31 ± 0.12 | −0.16 ± 0.03 | −0.09 ± 0.03 |
| After block 6 alone less without replay, block 6 | 9: readout 1's other excitatory | −0.54 ± 0.04 | −0.91 ± 0.06 | −0.07 ± 0.01 | −0.04 ± 0.02 | −1.57 ± 0.08 | −0.36 ± 0.04 | −0.05 ± 0.01 | −0.04 ± 0.02 |
| After block 6 alone less without replay, block 6 | 10: other excitatory | −0.13 ± 0.02 | −0.02 ± 0.01 | −0.01 ± 0.00 | −0.01 ± 0.00 | −0.14 ± 0.02 | −0.04 ± 0.02 | −0.01 ± 0.00 | 0.00 ± 0.01 |
| After block 6 alone less without replay, block 6 | 11: inhibitory in readout 0 | −1.36 ± 0.06 | −2.61 ± 0.08 | −0.12 ± 0.02 | −0.05 ± 0.02 | −2.09 ± 0.07 | −1.94 ± 0.16 | −0.10 ± 0.03 | −0.09 ± 0.03 |
| After block 6 alone less without replay, block 6 | 12: inhibitory in readout 1 | −2.86 ± 0.10 | −1.28 ± 0.10 | −0.05 ± 0.02 | −0.05 ± 0.02 | −2.71 ± 0.08 | −2.14 ± 0.06 | −0.10 ± 0.02 | −0.05 ± 0.02 |
| After block 12 alone less without replay, block 12 | 0: A alone | 0.00 ± 0.00 | −0.05 ± 0.02 | +0.03 ± 0.01 | −0.01 ± 0.01 | 0.00 ± 0.00 | −0.07 ± 0.02 | +0.04 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | 1: A and C | 0.00 ± 0.00 | −0.07 ± 0.02 | +0.02 ± 0.01 | −0.02 ± 0.01 | 0.00 ± 0.00 | −0.14 ± 0.02 | +0.02 ± 0.01 | −0.02 ± 0.01 |
| After block 12 alone less without replay, block 12 | 2: B alone | −0.08 ± 0.02 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.04 ± 0.01 | −0.09 ± 0.02 | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.04 ± 0.01 |
| After block 12 alone less without replay, block 12 | 3: B and D | −0.40 ± 0.03 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.02 ± 0.02 | −0.12 ± 0.03 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.02 ± 0.02 |
| After block 12 alone less without replay, block 12 | 4: C's own | −0.14 ± 0.03 | −0.12 ± 0.03 | −0.01 ± 0.01 | −0.01 ± 0.01 | −0.39 ± 0.03 | −0.28 ± 0.04 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | 5: D's own | −0.16 ± 0.03 | −0.38 ± 0.03 | −0.01 ± 0.01 | −0.01 ± 0.01 | −0.20 ± 0.02 | −0.10 ± 0.02 | −0.02 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | 6: readout 0's pattern targets | −4.80 ± 0.18 | −2.74 ± 0.15 | −0.20 ± 0.03 | −0.14 ± 0.04 | −5.33 ± 0.13 | −3.47 ± 0.10 | −0.20 ± 0.03 | −0.13 ± 0.04 |
| After block 12 alone less without replay, block 12 | 7: readout 0's other excitatory | −1.00 ± 0.08 | −0.91 ± 0.05 | −0.03 ± 0.02 | −0.04 ± 0.02 | −0.28 ± 0.03 | −0.33 ± 0.05 | −0.06 ± 0.01 | −0.03 ± 0.02 |
| After block 12 alone less without replay, block 12 | 8: readout 1's pattern targets | −4.63 ± 0.13 | −2.87 ± 0.12 | −0.13 ± 0.03 | −0.09 ± 0.03 | −3.86 ± 0.10 | −3.23 ± 0.11 | −0.14 ± 0.04 | −0.12 ± 0.03 |
| After block 12 alone less without replay, block 12 | 9: readout 1's other excitatory | −0.65 ± 0.05 | −0.81 ± 0.06 | −0.03 ± 0.02 | −0.03 ± 0.01 | −1.71 ± 0.07 | −0.35 ± 0.03 | −0.04 ± 0.01 | −0.03 ± 0.02 |
| After block 12 alone less without replay, block 12 | 10: other excitatory | −0.19 ± 0.05 | −0.05 ± 0.01 | 0.00 ± 0.00 | −0.01 ± 0.00 | −0.33 ± 0.06 | −0.10 ± 0.02 | 0.00 ± 0.00 | −0.01 ± 0.00 |
| After block 12 alone less without replay, block 12 | 11: inhibitory in readout 0 | −1.79 ± 0.09 | −2.71 ± 0.14 | −0.10 ± 0.02 | −0.04 ± 0.02 | −2.48 ± 0.06 | −2.07 ± 0.10 | −0.10 ± 0.02 | −0.06 ± 0.02 |
| After block 12 alone less without replay, block 12 | 12: inhibitory in readout 1 | −3.75 ± 0.09 | −1.52 ± 0.12 | −0.06 ± 0.02 | −0.09 ± 0.02 | −3.40 ± 0.11 | −2.40 ± 0.09 | −0.08 ± 0.02 | −0.09 ± 0.02 |
| After block 18 alone less without replay, block 18 | 0: A alone | 0.00 ± 0.00 | −0.09 ± 0.01 | +0.02 ± 0.01 | −0.01 ± 0.01 | 0.00 ± 0.00 | −0.10 ± 0.02 | +0.02 ± 0.01 | −0.02 ± 0.01 |
| After block 18 alone less without replay, block 18 | 1: A and C | 0.00 ± 0.00 | −0.10 ± 0.01 | +0.01 ± 0.01 | −0.01 ± 0.01 | 0.00 ± 0.00 | −0.15 ± 0.01 | +0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 18 | 2: B alone | −0.07 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.01 | +0.02 ± 0.01 | −0.08 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.01 | +0.03 ± 0.01 |
| After block 18 alone less without replay, block 18 | 3: B and D | −0.31 ± 0.02 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.03 ± 0.01 | −0.09 ± 0.02 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.03 ± 0.01 |
| After block 18 alone less without replay, block 18 | 4: C's own | −0.16 ± 0.02 | −0.22 ± 0.03 | −0.02 ± 0.01 | −0.01 ± 0.01 | −0.53 ± 0.04 | −0.50 ± 0.02 | −0.02 ± 0.00 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 18 | 5: D's own | −0.28 ± 0.03 | −0.32 ± 0.02 | 0.00 ± 0.01 | −0.01 ± 0.01 | −0.23 ± 0.04 | −0.09 ± 0.01 | 0.00 ± 0.01 | −0.02 ± 0.01 |
| After block 18 alone less without replay, block 18 | 6: readout 0's pattern targets | −4.27 ± 0.13 | −3.19 ± 0.09 | −0.12 ± 0.02 | −0.14 ± 0.02 | −4.20 ± 0.13 | −3.57 ± 0.13 | −0.19 ± 0.02 | −0.17 ± 0.02 |
| After block 18 alone less without replay, block 18 | 7: readout 0's other excitatory | −0.83 ± 0.07 | −0.84 ± 0.07 | −0.07 ± 0.01 | −0.02 ± 0.01 | −0.14 ± 0.02 | −0.34 ± 0.03 | −0.02 ± 0.01 | −0.04 ± 0.01 |
| After block 18 alone less without replay, block 18 | 8: readout 1's pattern targets | −5.14 ± 0.11 | −2.73 ± 0.13 | −0.10 ± 0.02 | −0.13 ± 0.02 | −4.07 ± 0.13 | −3.02 ± 0.07 | −0.12 ± 0.03 | −0.12 ± 0.02 |
| After block 18 alone less without replay, block 18 | 9: readout 1's other excitatory | −0.65 ± 0.04 | −0.64 ± 0.04 | −0.03 ± 0.01 | −0.03 ± 0.01 | −1.74 ± 0.06 | −0.45 ± 0.04 | −0.03 ± 0.01 | −0.05 ± 0.01 |
| After block 18 alone less without replay, block 18 | 10: other excitatory | −0.20 ± 0.04 | −0.04 ± 0.01 | 0.00 ± 0.01 | −0.01 ± 0.00 | −0.26 ± 0.03 | −0.08 ± 0.01 | −0.01 ± 0.00 | −0.01 ± 0.00 |
| After block 18 alone less without replay, block 18 | 11: inhibitory in readout 0 | −1.97 ± 0.06 | −2.48 ± 0.09 | −0.08 ± 0.02 | −0.11 ± 0.02 | −2.21 ± 0.07 | −2.39 ± 0.10 | −0.07 ± 0.02 | −0.11 ± 0.02 |
| After block 18 alone less without replay, block 18 | 12: inhibitory in readout 1 | −4.12 ± 0.11 | −1.62 ± 0.10 | −0.06 ± 0.02 | −0.10 ± 0.02 | −3.45 ± 0.09 | −1.92 ± 0.07 | −0.07 ± 0.02 | −0.10 ± 0.03 |
| After block 24 alone less without replay, block 24 | 0: A alone | 0.00 ± 0.00 | −0.08 ± 0.01 | +0.03 ± 0.01 | −0.01 ± 0.01 | 0.00 ± 0.00 | −0.08 ± 0.02 | +0.01 ± 0.01 | −0.02 ± 0.01 |
| After block 24 alone less without replay, block 24 | 1: A and C | 0.00 ± 0.00 | −0.12 ± 0.03 | +0.03 ± 0.01 | −0.01 ± 0.01 | 0.00 ± 0.00 | −0.22 ± 0.02 | +0.04 ± 0.01 | −0.02 ± 0.01 |
| After block 24 alone less without replay, block 24 | 2: B alone | −0.05 ± 0.02 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.02 ± 0.01 | −0.07 ± 0.03 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | 3: B and D | −0.37 ± 0.03 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.03 ± 0.01 | −0.02 ± 0.04 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.03 ± 0.01 |
| After block 24 alone less without replay, block 24 | 4: C's own | −0.14 ± 0.02 | −0.16 ± 0.02 | −0.01 ± 0.01 | −0.02 ± 0.01 | −0.54 ± 0.04 | −0.35 ± 0.04 | −0.02 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | 5: D's own | −0.26 ± 0.02 | −0.49 ± 0.03 | −0.02 ± 0.01 | −0.01 ± 0.01 | −0.04 ± 0.02 | −0.11 ± 0.02 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | 6: readout 0's pattern targets | −3.37 ± 0.13 | −3.46 ± 0.09 | −0.14 ± 0.03 | −0.12 ± 0.02 | −4.43 ± 0.17 | −4.06 ± 0.24 | −0.12 ± 0.04 | −0.17 ± 0.02 |
| After block 24 alone less without replay, block 24 | 7: readout 0's other excitatory | −0.52 ± 0.03 | −0.90 ± 0.06 | −0.02 ± 0.01 | −0.06 ± 0.02 | −0.17 ± 0.03 | −0.35 ± 0.04 | −0.03 ± 0.01 | −0.04 ± 0.02 |
| After block 24 alone less without replay, block 24 | 8: readout 1's pattern targets | −4.90 ± 0.12 | −2.97 ± 0.06 | −0.15 ± 0.03 | −0.14 ± 0.02 | −3.90 ± 0.18 | −3.21 ± 0.13 | −0.21 ± 0.03 | −0.14 ± 0.03 |
| After block 24 alone less without replay, block 24 | 9: readout 1's other excitatory | −0.74 ± 0.06 | −1.09 ± 0.06 | −0.01 ± 0.01 | −0.02 ± 0.01 | −1.90 ± 0.08 | −0.46 ± 0.04 | −0.05 ± 0.02 | −0.04 ± 0.02 |
| After block 24 alone less without replay, block 24 | 10: other excitatory | −0.09 ± 0.02 | −0.04 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.06 ± 0.03 | −0.14 ± 0.02 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | 11: inhibitory in readout 0 | −2.00 ± 0.09 | −2.25 ± 0.09 | −0.11 ± 0.02 | −0.10 ± 0.02 | −2.38 ± 0.08 | −2.69 ± 0.10 | −0.10 ± 0.02 | −0.12 ± 0.02 |
| After block 24 alone less without replay, block 24 | 12: inhibitory in readout 1 | −3.85 ± 0.09 | −1.66 ± 0.09 | −0.09 ± 0.01 | −0.08 ± 0.02 | −3.00 ± 0.07 | −2.01 ± 0.11 | −0.06 ± 0.03 | −0.07 ± 0.02 |

On both cells and both stimuli's trials, in the window, each readout's pattern targets fired fewer spikes by 2.73 to 5.47 a trial
on the image from the assignment and 3.02 to 5.56 on the image from the mirrored assignment, and each readout's inhibitory units by
1.28 to 4.12 and 1.92 to 3.45; each of these 32 differences on each cell lies beyond 2 SE. Each readout's other
excitatory units fired fewer by 0.52 to 1.09 and 0.14 to 1.90. In the volley the trial's stimulus's kinds' spikes rose by
0.01 to 0.04 on the image from the assignment and 0.01 to 0.04 on the image from the mirrored assignment.

**At block 36**, after twelve blocks with no night:

| Comparison | Class | From the assignment: the window, A's trials | B's trials | the volley, A's trials | B's trials | From the mirrored assignment: the window, A's trials | B's trials | the volley, A's trials | B's trials |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 36 | 0: A alone | 0.00 ± 0.00 | 0.00 ± 0.01 | −0.01 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 | +0.03 ± 0.01 | 0.00 ± 0.01 | 0.00 ± 0.00 |
| After block 6 alone less without replay, block 36 | 1: A and C | 0.00 ± 0.00 | +0.04 ± 0.03 | 0.00 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.12 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.00 |
| After block 6 alone less without replay, block 36 | 2: B alone | 0.00 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 | +0.01 ± 0.02 | 0.00 ± 0.00 | +0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 6 alone less without replay, block 36 | 3: B and D | −0.06 ± 0.03 | 0.00 ± 0.00 | +0.01 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.01 ± 0.01 |
| After block 6 alone less without replay, block 36 | 4: C's own | −0.01 ± 0.01 | −0.06 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.53 ± 0.03 | +0.01 ± 0.04 | 0.00 ± 0.01 | −0.01 ± 0.00 |
| After block 6 alone less without replay, block 36 | 5: D's own | +0.05 ± 0.02 | −0.19 ± 0.01 | 0.00 ± 0.00 | +0.01 ± 0.00 | −0.12 ± 0.01 | +0.05 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 |
| After block 6 alone less without replay, block 36 | 6: readout 0's pattern targets | −2.20 ± 0.18 | −0.80 ± 0.20 | +0.02 ± 0.01 | +0.01 ± 0.01 | −0.71 ± 0.15 | −2.23 ± 0.10 | +0.04 ± 0.00 | −0.01 ± 0.01 |
| After block 6 alone less without replay, block 36 | 7: readout 0's other excitatory | −0.05 ± 0.14 | −0.58 ± 0.05 | +0.01 ± 0.01 | +0.01 ± 0.01 | +0.04 ± 0.02 | +0.01 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.01 |
| After block 6 alone less without replay, block 36 | 8: readout 1's pattern targets | −1.83 ± 0.06 | −1.53 ± 0.09 | −0.01 ± 0.02 | 0.00 ± 0.01 | −1.30 ± 0.12 | −0.87 ± 0.14 | 0.00 ± 0.03 | +0.03 ± 0.01 |
| After block 6 alone less without replay, block 36 | 9: readout 1's other excitatory | −0.08 ± 0.06 | −0.59 ± 0.07 | +0.01 ± 0.00 | +0.01 ± 0.00 | −0.32 ± 0.08 | −0.43 ± 0.03 | +0.01 ± 0.01 | −0.01 ± 0.00 |
| After block 6 alone less without replay, block 36 | 10: other excitatory | −0.10 ± 0.01 | −0.02 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.44 ± 0.08 | −0.09 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 |
| After block 6 alone less without replay, block 36 | 11: inhibitory in readout 0 | −0.78 ± 0.04 | −1.15 ± 0.13 | 0.00 ± 0.01 | 0.00 ± 0.01 | −0.55 ± 0.06 | −0.53 ± 0.03 | +0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 6 alone less without replay, block 36 | 12: inhibitory in readout 1 | −1.58 ± 0.09 | +0.10 ± 0.08 | +0.01 ± 0.02 | +0.01 ± 0.00 | −0.63 ± 0.14 | −0.51 ± 0.17 | +0.02 ± 0.01 | 0.00 ± 0.01 |
| After block 12 alone less without replay, block 36 | 0: A alone | 0.00 ± 0.00 | 0.00 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 | +0.02 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 |
| After block 12 alone less without replay, block 36 | 1: A and C | 0.00 ± 0.00 | +0.04 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.12 ± 0.02 | 0.00 ± 0.00 | +0.01 ± 0.00 |
| After block 12 alone less without replay, block 36 | 2: B alone | −0.01 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.01 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.01 ± 0.00 |
| After block 12 alone less without replay, block 36 | 3: B and D | −0.03 ± 0.07 | 0.00 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 | +0.04 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.01 |
| After block 12 alone less without replay, block 36 | 4: C's own | −0.01 ± 0.01 | 0.00 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.46 ± 0.02 | −0.08 ± 0.02 | 0.00 ± 0.01 | 0.00 ± 0.00 |
| After block 12 alone less without replay, block 36 | 5: D's own | +0.04 ± 0.01 | −0.11 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.00 | +0.02 ± 0.02 | +0.02 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 |
| After block 12 alone less without replay, block 36 | 6: readout 0's pattern targets | −1.67 ± 0.15 | −0.49 ± 0.16 | +0.03 ± 0.01 | +0.01 ± 0.01 | −1.59 ± 0.21 | −1.55 ± 0.11 | +0.02 ± 0.01 | +0.01 ± 0.01 |
| After block 12 alone less without replay, block 36 | 7: readout 0's other excitatory | −0.25 ± 0.10 | −0.46 ± 0.08 | +0.01 ± 0.01 | +0.01 ± 0.00 | +0.01 ± 0.01 | −0.06 ± 0.01 | 0.00 ± 0.01 | +0.01 ± 0.01 |
| After block 12 alone less without replay, block 36 | 8: readout 1's pattern targets | −1.47 ± 0.33 | −1.04 ± 0.08 | +0.01 ± 0.01 | +0.01 ± 0.01 | −1.90 ± 0.21 | −0.55 ± 0.08 | +0.02 ± 0.01 | +0.03 ± 0.01 |
| After block 12 alone less without replay, block 36 | 9: readout 1's other excitatory | −0.11 ± 0.08 | −0.25 ± 0.07 | +0.01 ± 0.00 | +0.01 ± 0.00 | −0.47 ± 0.05 | +0.14 ± 0.05 | +0.01 ± 0.00 | 0.00 ± 0.01 |
| After block 12 alone less without replay, block 36 | 10: other excitatory | −0.06 ± 0.00 | −0.01 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.16 ± 0.04 | −0.06 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 |
| After block 12 alone less without replay, block 36 | 11: inhibitory in readout 0 | −0.89 ± 0.08 | −0.87 ± 0.09 | 0.00 ± 0.01 | −0.01 ± 0.01 | −0.72 ± 0.03 | −0.43 ± 0.09 | 0.00 ± 0.01 | +0.01 ± 0.01 |
| After block 12 alone less without replay, block 36 | 12: inhibitory in readout 1 | −1.81 ± 0.24 | +0.11 ± 0.17 | +0.01 ± 0.01 | 0.00 ± 0.00 | −1.37 ± 0.11 | −0.54 ± 0.12 | +0.01 ± 0.00 | 0.00 ± 0.01 |
| After block 18 alone less without replay, block 36 | 0: A alone | 0.00 ± 0.00 | +0.01 ± 0.01 | −0.01 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 | +0.02 ± 0.01 | 0.00 ± 0.01 | 0.00 ± 0.00 |
| After block 18 alone less without replay, block 36 | 1: A and C | 0.00 ± 0.00 | +0.03 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.10 ± 0.02 | −0.01 ± 0.01 | 0.00 ± 0.00 |
| After block 18 alone less without replay, block 36 | 2: B alone | −0.01 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 | +0.02 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.01 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 36 | 3: B and D | −0.08 ± 0.03 | 0.00 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.01 ± 0.02 | 0.00 ± 0.00 | +0.01 ± 0.00 | 0.00 ± 0.01 |
| After block 18 alone less without replay, block 36 | 4: C's own | 0.00 ± 0.01 | −0.01 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.38 ± 0.03 | −0.27 ± 0.04 | 0.00 ± 0.01 | 0.00 ± 0.01 |
| After block 18 alone less without replay, block 36 | 5: D's own | +0.02 ± 0.01 | −0.04 ± 0.02 | 0.00 ± 0.00 | +0.01 ± 0.00 | +0.04 ± 0.02 | +0.05 ± 0.01 | +0.01 ± 0.00 | 0.00 ± 0.00 |
| After block 18 alone less without replay, block 36 | 6: readout 0's pattern targets | −1.53 ± 0.09 | −0.99 ± 0.25 | +0.01 ± 0.01 | 0.00 ± 0.01 | −1.34 ± 0.15 | −1.05 ± 0.09 | +0.03 ± 0.01 | +0.02 ± 0.02 |
| After block 18 alone less without replay, block 36 | 7: readout 0's other excitatory | −0.24 ± 0.13 | −0.53 ± 0.05 | +0.01 ± 0.01 | +0.01 ± 0.00 | +0.03 ± 0.01 | −0.03 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 |
| After block 18 alone less without replay, block 36 | 8: readout 1's pattern targets | −2.26 ± 0.18 | −1.11 ± 0.08 | +0.01 ± 0.01 | 0.00 ± 0.00 | −1.83 ± 0.14 | −0.37 ± 0.14 | −0.01 ± 0.02 | +0.02 ± 0.01 |
| After block 18 alone less without replay, block 36 | 9: readout 1's other excitatory | −0.05 ± 0.05 | −0.13 ± 0.03 | +0.01 ± 0.00 | +0.02 ± 0.01 | −0.82 ± 0.10 | −0.17 ± 0.01 | +0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 36 | 10: other excitatory | −0.09 ± 0.01 | 0.00 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.08 ± 0.04 | −0.06 ± 0.01 | 0.00 ± 0.01 | 0.00 ± 0.00 |
| After block 18 alone less without replay, block 36 | 11: inhibitory in readout 0 | −1.02 ± 0.04 | −0.67 ± 0.18 | 0.00 ± 0.01 | −0.01 ± 0.01 | −0.88 ± 0.11 | −1.06 ± 0.14 | 0.00 ± 0.01 | +0.02 ± 0.03 |
| After block 18 alone less without replay, block 36 | 12: inhibitory in readout 1 | −2.20 ± 0.12 | −0.35 ± 0.05 | +0.01 ± 0.01 | 0.00 ± 0.01 | −1.38 ± 0.07 | −0.33 ± 0.09 | +0.02 ± 0.01 | 0.00 ± 0.01 |
| After block 24 alone less without replay, block 36 | 0: A alone | 0.00 ± 0.00 | −0.01 ± 0.01 | −0.02 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | +0.03 ± 0.01 | −0.01 ± 0.01 | 0.00 ± 0.00 |
| After block 24 alone less without replay, block 36 | 1: A and C | 0.00 ± 0.00 | −0.02 ± 0.02 | 0.00 ± 0.00 | +0.01 ± 0.01 | 0.00 ± 0.00 | −0.07 ± 0.01 | −0.01 ± 0.01 | 0.00 ± 0.00 |
| After block 24 alone less without replay, block 36 | 2: B alone | +0.03 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 | +0.01 ± 0.01 | 0.00 ± 0.00 | +0.01 ± 0.01 | −0.02 ± 0.01 |
| After block 24 alone less without replay, block 36 | 3: B and D | −0.30 ± 0.03 | 0.00 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.01 | +0.04 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.01 |
| After block 24 alone less without replay, block 36 | 4: C's own | +0.02 ± 0.02 | −0.05 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.27 ± 0.01 | −0.09 ± 0.01 | +0.01 ± 0.00 | 0.00 ± 0.00 |
| After block 24 alone less without replay, block 36 | 5: D's own | −0.14 ± 0.02 | −0.29 ± 0.02 | 0.00 ± 0.00 | +0.01 ± 0.00 | +0.04 ± 0.02 | 0.00 ± 0.01 | 0.00 ± 0.00 | +0.01 ± 0.00 |
| After block 24 alone less without replay, block 36 | 6: readout 0's pattern targets | −1.37 ± 0.09 | −1.41 ± 0.14 | +0.03 ± 0.01 | +0.01 ± 0.00 | −2.22 ± 0.13 | −1.93 ± 0.18 | +0.03 ± 0.00 | +0.02 ± 0.02 |
| After block 24 alone less without replay, block 36 | 7: readout 0's other excitatory | −0.06 ± 0.05 | −0.66 ± 0.03 | +0.01 ± 0.00 | 0.00 ± 0.01 | +0.02 ± 0.01 | −0.01 ± 0.01 | +0.01 ± 0.00 | 0.00 ± 0.01 |
| After block 24 alone less without replay, block 36 | 8: readout 1's pattern targets | −2.12 ± 0.28 | −1.54 ± 0.09 | +0.02 ± 0.01 | +0.01 ± 0.01 | −1.42 ± 0.09 | −1.16 ± 0.10 | −0.03 ± 0.03 | −0.01 ± 0.02 |
| After block 24 alone less without replay, block 36 | 9: readout 1's other excitatory | −0.19 ± 0.04 | −0.57 ± 0.02 | +0.01 ± 0.01 | +0.01 ± 0.00 | −0.69 ± 0.05 | −0.18 ± 0.02 | −0.01 ± 0.00 | 0.00 ± 0.01 |
| After block 24 alone less without replay, block 36 | 10: other excitatory | −0.04 ± 0.01 | −0.02 ± 0.00 | 0.00 ± 0.00 | 0.00 ± 0.00 | +0.03 ± 0.05 | −0.07 ± 0.03 | 0.00 ± 0.00 | 0.00 ± 0.00 |
| After block 24 alone less without replay, block 36 | 11: inhibitory in readout 0 | −0.87 ± 0.07 | −0.69 ± 0.05 | −0.01 ± 0.01 | −0.01 ± 0.01 | −0.86 ± 0.13 | −0.99 ± 0.09 | +0.02 ± 0.01 | 0.00 ± 0.01 |
| After block 24 alone less without replay, block 36 | 12: inhibitory in readout 1 | −2.05 ± 0.12 | −0.72 ± 0.11 | +0.02 ± 0.00 | 0.00 ± 0.01 | −0.96 ± 0.10 | −0.53 ± 0.09 | +0.01 ± 0.02 | 0.00 ± 0.00 |

Of the same 32 differences, 30 of 32 on the image from the assignment and 32 of 32 on the image from the mirrored assignment lie below the
night without replay beyond 2 SE.

**H-47's plain night on all four nights**, at each checkpoint; at block 6 it is the night after block 6 alone:

| Comparison | Class | From the assignment: the window, A's trials | B's trials | the volley, A's trials | B's trials | From the mirrored assignment: the window, A's trials | B's trials | the volley, A's trials | B's trials |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Plain on all four nights less without replay, block 6 | 0: A alone | 0.00 ± 0.00 | −0.09 ± 0.01 | +0.03 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.08 ± 0.02 | +0.02 ± 0.01 | 0.00 ± 0.01 |
| Plain on all four nights less without replay, block 6 | 1: A and C | 0.00 ± 0.00 | −0.07 ± 0.02 | +0.03 ± 0.01 | −0.02 ± 0.01 | 0.00 ± 0.00 | −0.13 ± 0.02 | +0.03 ± 0.01 | −0.03 ± 0.01 |
| Plain on all four nights less without replay, block 6 | 2: B alone | −0.05 ± 0.01 | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.01 ± 0.01 | −0.08 ± 0.01 | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.03 ± 0.01 |
| Plain on all four nights less without replay, block 6 | 3: B and D | −0.23 ± 0.04 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.01 ± 0.01 | −0.13 ± 0.04 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.03 ± 0.01 |
| Plain on all four nights less without replay, block 6 | 4: C's own | −0.17 ± 0.02 | −0.19 ± 0.03 | 0.00 ± 0.01 | −0.02 ± 0.01 | −0.31 ± 0.03 | −0.23 ± 0.03 | −0.01 ± 0.01 | −0.02 ± 0.01 |
| Plain on all four nights less without replay, block 6 | 5: D's own | −0.35 ± 0.03 | −0.53 ± 0.03 | −0.01 ± 0.01 | −0.01 ± 0.01 | −0.18 ± 0.03 | −0.06 ± 0.02 | 0.00 ± 0.01 | 0.00 ± 0.01 |
| Plain on all four nights less without replay, block 6 | 6: readout 0's pattern targets | −5.47 ± 0.18 | −2.73 ± 0.09 | −0.09 ± 0.03 | −0.11 ± 0.02 | −5.56 ± 0.14 | −3.14 ± 0.13 | −0.21 ± 0.03 | −0.22 ± 0.03 |
| Plain on all four nights less without replay, block 6 | 7: readout 0's other excitatory | −0.91 ± 0.05 | −0.73 ± 0.05 | −0.05 ± 0.02 | −0.06 ± 0.02 | −0.35 ± 0.04 | −0.35 ± 0.05 | −0.05 ± 0.01 | −0.02 ± 0.02 |
| Plain on all four nights less without replay, block 6 | 8: readout 1's pattern targets | −3.70 ± 0.12 | −2.75 ± 0.10 | −0.11 ± 0.02 | −0.14 ± 0.03 | −3.02 ± 0.09 | −3.31 ± 0.12 | −0.16 ± 0.03 | −0.09 ± 0.03 |
| Plain on all four nights less without replay, block 6 | 9: readout 1's other excitatory | −0.54 ± 0.04 | −0.91 ± 0.06 | −0.07 ± 0.01 | −0.04 ± 0.02 | −1.57 ± 0.08 | −0.36 ± 0.04 | −0.05 ± 0.01 | −0.04 ± 0.02 |
| Plain on all four nights less without replay, block 6 | 10: other excitatory | −0.13 ± 0.02 | −0.02 ± 0.01 | −0.01 ± 0.00 | −0.01 ± 0.00 | −0.14 ± 0.02 | −0.04 ± 0.02 | −0.01 ± 0.00 | 0.00 ± 0.01 |
| Plain on all four nights less without replay, block 6 | 11: inhibitory in readout 0 | −1.36 ± 0.06 | −2.61 ± 0.08 | −0.12 ± 0.02 | −0.05 ± 0.02 | −2.09 ± 0.07 | −1.94 ± 0.16 | −0.10 ± 0.03 | −0.09 ± 0.03 |
| Plain on all four nights less without replay, block 6 | 12: inhibitory in readout 1 | −2.86 ± 0.10 | −1.28 ± 0.10 | −0.05 ± 0.02 | −0.05 ± 0.02 | −2.71 ± 0.08 | −2.14 ± 0.06 | −0.10 ± 0.02 | −0.05 ± 0.02 |
| Plain on all four nights less without replay, block 12 | 0: A alone | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.03 ± 0.01 | 0.00 ± 0.01 | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.03 ± 0.01 | −0.01 ± 0.01 |
| Plain on all four nights less without replay, block 12 | 1: A and C | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.02 ± 0.01 | −0.01 ± 0.01 | 0.00 ± 0.00 | −0.12 ± 0.02 | +0.03 ± 0.01 | 0.00 ± 0.01 |
| Plain on all four nights less without replay, block 12 | 2: B alone | −0.02 ± 0.01 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.02 ± 0.01 | −0.08 ± 0.01 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.02 ± 0.01 |
| Plain on all four nights less without replay, block 12 | 3: B and D | −0.32 ± 0.03 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.01 ± 0.01 | −0.12 ± 0.03 | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.02 ± 0.01 |
| Plain on all four nights less without replay, block 12 | 4: C's own | −0.17 ± 0.02 | −0.19 ± 0.03 | −0.02 ± 0.01 | 0.00 ± 0.01 | −0.44 ± 0.03 | −0.35 ± 0.03 | −0.01 ± 0.00 | 0.00 ± 0.01 |
| Plain on all four nights less without replay, block 12 | 5: D's own | −0.33 ± 0.02 | −0.49 ± 0.04 | 0.00 ± 0.00 | −0.01 ± 0.00 | −0.33 ± 0.04 | −0.02 ± 0.02 | 0.00 ± 0.00 | −0.01 ± 0.00 |
| Plain on all four nights less without replay, block 12 | 6: readout 0's pattern targets | −7.02 ± 0.18 | −3.14 ± 0.12 | −0.12 ± 0.03 | −0.12 ± 0.02 | −6.96 ± 0.19 | −4.36 ± 0.16 | −0.16 ± 0.03 | −0.14 ± 0.02 |
| Plain on all four nights less without replay, block 12 | 7: readout 0's other excitatory | −1.18 ± 0.07 | −1.17 ± 0.05 | −0.04 ± 0.01 | −0.04 ± 0.02 | −0.19 ± 0.03 | −0.32 ± 0.04 | −0.03 ± 0.01 | −0.02 ± 0.01 |
| Plain on all four nights less without replay, block 12 | 8: readout 1's pattern targets | −4.52 ± 0.25 | −3.83 ± 0.14 | −0.11 ± 0.02 | −0.12 ± 0.02 | −3.79 ± 0.10 | −4.01 ± 0.11 | −0.11 ± 0.03 | −0.12 ± 0.02 |
| Plain on all four nights less without replay, block 12 | 9: readout 1's other excitatory | −0.79 ± 0.03 | −1.06 ± 0.07 | −0.02 ± 0.01 | −0.01 ± 0.01 | −2.01 ± 0.09 | −0.36 ± 0.03 | −0.04 ± 0.01 | −0.05 ± 0.01 |
| Plain on all four nights less without replay, block 12 | 10: other excitatory | −0.20 ± 0.05 | −0.03 ± 0.01 | 0.00 ± 0.00 | −0.01 ± 0.00 | −0.31 ± 0.05 | −0.10 ± 0.02 | 0.00 ± 0.00 | −0.01 ± 0.00 |
| Plain on all four nights less without replay, block 12 | 11: inhibitory in readout 0 | −1.77 ± 0.06 | −3.41 ± 0.24 | −0.06 ± 0.02 | −0.04 ± 0.01 | −2.96 ± 0.07 | −2.16 ± 0.08 | −0.07 ± 0.02 | −0.06 ± 0.01 |
| Plain on all four nights less without replay, block 12 | 12: inhibitory in readout 1 | −4.28 ± 0.10 | −2.01 ± 0.09 | −0.05 ± 0.02 | −0.08 ± 0.02 | −3.56 ± 0.10 | −2.77 ± 0.15 | −0.08 ± 0.02 | −0.08 ± 0.02 |
| Plain on all four nights less without replay, block 18 | 0: A alone | 0.00 ± 0.00 | −0.06 ± 0.02 | +0.01 ± 0.01 | −0.01 ± 0.01 | 0.00 ± 0.00 | −0.04 ± 0.02 | +0.02 ± 0.01 | −0.02 ± 0.01 |
| Plain on all four nights less without replay, block 18 | 1: A and C | 0.00 ± 0.00 | −0.11 ± 0.02 | 0.00 ± 0.01 | −0.01 ± 0.01 | 0.00 ± 0.00 | −0.16 ± 0.02 | +0.02 ± 0.01 | −0.01 ± 0.01 |
| Plain on all four nights less without replay, block 18 | 2: B alone | −0.01 ± 0.02 | 0.00 ± 0.00 | +0.01 ± 0.01 | +0.01 ± 0.01 | −0.07 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.01 | +0.03 ± 0.01 |
| Plain on all four nights less without replay, block 18 | 3: B and D | −0.48 ± 0.03 | 0.00 ± 0.00 | −0.02 ± 0.01 | 0.00 ± 0.02 | 0.00 ± 0.02 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.02 ± 0.01 |
| Plain on all four nights less without replay, block 18 | 4: C's own | −0.15 ± 0.02 | −0.20 ± 0.03 | −0.01 ± 0.01 | −0.01 ± 0.01 | −0.63 ± 0.04 | −0.58 ± 0.03 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| Plain on all four nights less without replay, block 18 | 5: D's own | −0.40 ± 0.03 | −0.69 ± 0.03 | −0.01 ± 0.01 | −0.02 ± 0.01 | −0.46 ± 0.03 | +0.07 ± 0.02 | 0.00 ± 0.01 | −0.01 ± 0.01 |
| Plain on all four nights less without replay, block 18 | 6: readout 0's pattern targets | −8.22 ± 0.25 | −3.94 ± 0.26 | −0.10 ± 0.03 | −0.12 ± 0.03 | −7.11 ± 0.23 | −5.87 ± 0.17 | −0.19 ± 0.03 | −0.13 ± 0.03 |
| Plain on all four nights less without replay, block 18 | 7: readout 0's other excitatory | −1.24 ± 0.05 | −1.59 ± 0.08 | −0.05 ± 0.02 | −0.02 ± 0.03 | −0.12 ± 0.02 | −0.29 ± 0.04 | 0.00 ± 0.01 | −0.03 ± 0.01 |
| Plain on all four nights less without replay, block 18 | 8: readout 1's pattern targets | −5.94 ± 0.14 | −4.50 ± 0.13 | −0.06 ± 0.03 | −0.07 ± 0.03 | −4.66 ± 0.21 | −4.24 ± 0.11 | −0.06 ± 0.03 | −0.10 ± 0.02 |
| Plain on all four nights less without replay, block 18 | 9: readout 1's other excitatory | −0.97 ± 0.05 | −1.09 ± 0.05 | −0.01 ± 0.02 | −0.01 ± 0.02 | −2.59 ± 0.06 | −0.57 ± 0.07 | −0.01 ± 0.02 | −0.02 ± 0.01 |
| Plain on all four nights less without replay, block 18 | 10: other excitatory | −0.25 ± 0.06 | −0.06 ± 0.01 | −0.01 ± 0.01 | −0.01 ± 0.00 | −0.57 ± 0.05 | −0.09 ± 0.02 | −0.01 ± 0.00 | −0.01 ± 0.00 |
| Plain on all four nights less without replay, block 18 | 11: inhibitory in readout 0 | −2.33 ± 0.08 | −3.81 ± 0.36 | −0.08 ± 0.03 | −0.08 ± 0.02 | −3.36 ± 0.09 | −3.04 ± 0.20 | −0.05 ± 0.02 | −0.11 ± 0.02 |
| Plain on all four nights less without replay, block 18 | 12: inhibitory in readout 1 | −5.80 ± 0.13 | −2.43 ± 0.08 | −0.07 ± 0.02 | −0.09 ± 0.02 | −4.19 ± 0.08 | −2.32 ± 0.07 | −0.06 ± 0.01 | −0.10 ± 0.02 |
| Plain on all four nights less without replay, block 24 | 0: A alone | 0.00 ± 0.00 | −0.07 ± 0.02 | +0.01 ± 0.01 | −0.02 ± 0.01 | 0.00 ± 0.00 | −0.05 ± 0.03 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| Plain on all four nights less without replay, block 24 | 1: A and C | 0.00 ± 0.00 | −0.08 ± 0.02 | +0.02 ± 0.01 | −0.01 ± 0.01 | 0.00 ± 0.00 | −0.24 ± 0.03 | +0.03 ± 0.01 | −0.02 ± 0.01 |
| Plain on all four nights less without replay, block 24 | 2: B alone | 0.00 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.01 | 0.00 ± 0.01 | −0.06 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.01 | 0.00 ± 0.01 |
| Plain on all four nights less without replay, block 24 | 3: B and D | −0.43 ± 0.03 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.01 ± 0.01 | +0.06 ± 0.04 | 0.00 ± 0.00 | −0.01 ± 0.01 | 0.00 ± 0.01 |
| Plain on all four nights less without replay, block 24 | 4: C's own | −0.12 ± 0.02 | −0.21 ± 0.03 | +0.01 ± 0.01 | −0.01 ± 0.01 | −0.85 ± 0.05 | −0.58 ± 0.04 | −0.02 ± 0.01 | −0.01 ± 0.01 |
| Plain on all four nights less without replay, block 24 | 5: D's own | −0.26 ± 0.03 | −0.75 ± 0.04 | −0.01 ± 0.01 | 0.00 ± 0.01 | −0.32 ± 0.02 | +0.13 ± 0.03 | −0.01 ± 0.01 | 0.00 ± 0.01 |
| Plain on all four nights less without replay, block 24 | 6: readout 0's pattern targets | −8.85 ± 0.12 | −4.44 ± 0.25 | −0.09 ± 0.02 | −0.09 ± 0.03 | −7.88 ± 0.19 | −7.36 ± 0.23 | −0.04 ± 0.04 | −0.11 ± 0.03 |
| Plain on all four nights less without replay, block 24 | 7: readout 0's other excitatory | −1.05 ± 0.04 | −1.87 ± 0.15 | +0.01 ± 0.01 | −0.05 ± 0.02 | −0.11 ± 0.03 | −0.24 ± 0.03 | −0.01 ± 0.01 | 0.00 ± 0.02 |
| Plain on all four nights less without replay, block 24 | 8: readout 1's pattern targets | −6.51 ± 0.20 | −5.22 ± 0.08 | −0.11 ± 0.03 | −0.07 ± 0.02 | −5.93 ± 0.33 | −5.12 ± 0.14 | −0.14 ± 0.03 | −0.08 ± 0.03 |
| Plain on all four nights less without replay, block 24 | 9: readout 1's other excitatory | −1.18 ± 0.08 | −1.34 ± 0.06 | +0.01 ± 0.01 | 0.00 ± 0.01 | −2.82 ± 0.10 | −0.79 ± 0.10 | −0.03 ± 0.02 | −0.03 ± 0.02 |
| Plain on all four nights less without replay, block 24 | 10: other excitatory | −0.14 ± 0.04 | −0.05 ± 0.01 | −0.01 ± 0.00 | −0.01 ± 0.01 | −0.58 ± 0.02 | −0.16 ± 0.02 | −0.01 ± 0.01 | 0.00 ± 0.01 |
| Plain on all four nights less without replay, block 24 | 11: inhibitory in readout 0 | −3.22 ± 0.09 | −4.11 ± 0.25 | −0.07 ± 0.02 | −0.08 ± 0.02 | −3.63 ± 0.09 | −3.28 ± 0.20 | −0.06 ± 0.02 | −0.11 ± 0.02 |
| Plain on all four nights less without replay, block 24 | 12: inhibitory in readout 1 | −6.80 ± 0.10 | −2.89 ± 0.08 | −0.05 ± 0.02 | −0.05 ± 0.02 | −4.90 ± 0.15 | −2.58 ± 0.11 | −0.04 ± 0.02 | −0.05 ± 0.02 |
| Plain on all four nights less without replay, block 36 | 0: A alone | 0.00 ± 0.00 | +0.01 ± 0.02 | −0.01 ± 0.00 | 0.00 ± 0.01 | 0.00 ± 0.00 | +0.04 ± 0.02 | −0.01 ± 0.01 | 0.00 ± 0.00 |
| Plain on all four nights less without replay, block 36 | 1: A and C | 0.00 ± 0.00 | +0.03 ± 0.02 | −0.02 ± 0.01 | +0.01 ± 0.01 | 0.00 ± 0.00 | −0.11 ± 0.02 | −0.03 ± 0.01 | +0.01 ± 0.00 |
| Plain on all four nights less without replay, block 36 | 2: B alone | +0.09 ± 0.01 | 0.00 ± 0.00 | +0.01 ± 0.00 | −0.02 ± 0.01 | +0.05 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.03 ± 0.01 |
| Plain on all four nights less without replay, block 36 | 3: B and D | −0.33 ± 0.07 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.06 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.01 | +0.01 ± 0.01 |
| Plain on all four nights less without replay, block 36 | 4: C's own | +0.03 ± 0.01 | −0.10 ± 0.03 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.68 ± 0.03 | −0.44 ± 0.04 | 0.00 ± 0.01 | 0.00 ± 0.01 |
| Plain on all four nights less without replay, block 36 | 5: D's own | −0.17 ± 0.04 | −0.62 ± 0.04 | 0.00 ± 0.00 | +0.01 ± 0.00 | −0.27 ± 0.04 | +0.23 ± 0.03 | 0.00 ± 0.00 | 0.00 ± 0.00 |
| Plain on all four nights less without replay, block 36 | 6: readout 0's pattern targets | −7.37 ± 0.17 | −2.60 ± 0.22 | +0.04 ± 0.00 | +0.02 ± 0.01 | −5.74 ± 0.17 | −6.03 ± 0.22 | +0.05 ± 0.01 | +0.01 ± 0.02 |
| Plain on all four nights less without replay, block 36 | 7: readout 0's other excitatory | −0.73 ± 0.12 | −1.72 ± 0.18 | +0.02 ± 0.01 | +0.02 ± 0.01 | +0.05 ± 0.02 | 0.00 ± 0.04 | +0.02 ± 0.01 | +0.01 ± 0.00 |
| Plain on all four nights less without replay, block 36 | 8: readout 1's pattern targets | −3.95 ± 0.47 | −4.13 ± 0.06 | 0.00 ± 0.03 | +0.04 ± 0.02 | −4.45 ± 0.26 | −3.62 ± 0.16 | −0.01 ± 0.03 | 0.00 ± 0.01 |
| Plain on all four nights less without replay, block 36 | 9: readout 1's other excitatory | −0.65 ± 0.05 | −0.81 ± 0.09 | +0.03 ± 0.01 | +0.01 ± 0.03 | −1.92 ± 0.07 | −0.54 ± 0.09 | +0.02 ± 0.01 | +0.01 ± 0.00 |
| Plain on all four nights less without replay, block 36 | 10: other excitatory | −0.12 ± 0.02 | −0.02 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 | −0.46 ± 0.02 | −0.10 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 |
| Plain on all four nights less without replay, block 36 | 11: inhibitory in readout 0 | −2.36 ± 0.09 | −2.82 ± 0.18 | −0.01 ± 0.01 | −0.02 ± 0.01 | −2.44 ± 0.10 | −2.65 ± 0.15 | 0.00 ± 0.01 | 0.00 ± 0.02 |
| Plain on all four nights less without replay, block 36 | 12: inhibitory in readout 1 | −5.50 ± 0.11 | −2.32 ± 0.29 | +0.01 ± 0.01 | 0.00 ± 0.02 | −3.56 ± 0.23 | −1.40 ± 0.08 | +0.01 ± 0.01 | 0.00 ± 0.02 |

## 3. The drive onto each readout

**On the night without replay**, by presynaptic group, onto each readout named by the stimulus whose answer it is:

| Block | Onto | From | From the assignment: the window, A's trials, k | B's trials | the volley, A's trials | B's trials | From the mirrored assignment: the window, A's trials | B's trials | the volley, A's trials | B's trials |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | A's answer's readout | A's kinds | 0.0 | 50.8 | 6 557.1 | 6.9 | 0.0 | 57.7 | 6 363.6 | 5.9 |
| 6 | A's answer's readout | B's kinds | 133.3 | 0.1 | 9.1 | 6 605.8 | 135.1 | 0.0 | 8.9 | 6 327.7 |
| 6 | A's answer's readout | The odd places' excitatory | 2 392.5 | 1 755.2 | 53.6 | 58.6 | 2 450.9 | 1 577.9 | 57.9 | 60.1 |
| 6 | A's answer's readout | The even places' excitatory | 2 045.6 | 2 268.2 | 51.8 | 46.1 | 2 487.3 | 2 143.8 | 57.9 | 47.5 |
| 6 | A's answer's readout | Other excitatory | 15.0 | 3.1 | 0.7 | 0.8 | 14.1 | 8.8 | 0.9 | 0.8 |
| 6 | A's answer's readout | Inhibitory | 1 651.8 | 2 099.0 | 48.5 | 45.5 | 2 109.4 | 2 529.6 | 49.8 | 45.9 |
| 6 | A's answer's readout | All excitatory | 4 586.4 | 4 077.4 | 6 672.4 | 6 718.2 | 5 087.4 | 3 788.1 | 6 489.3 | 6 442.0 |
| 6 | A's answer's readout | Excitatory less inhibitory | 2 934.6 | 1 978.4 | 6 623.8 | 6 672.7 | 2 978.0 | 1 258.6 | 6 439.4 | 6 396.1 |
| 6 | B's answer's readout | A's kinds | 0.0 | 52.9 | 6 109.3 | 5.2 | 0.0 | 57.3 | 6 303.0 | 6.0 |
| 6 | B's answer's readout | B's kinds | 98.0 | 0.3 | 8.7 | 6 467.5 | 171.7 | 0.0 | 10.2 | 6 750.1 |
| 6 | B's answer's readout | The odd places' excitatory | 2 187.4 | 1 901.3 | 57.7 | 62.2 | 2 603.2 | 1 816.8 | 55.6 | 55.9 |
| 6 | B's answer's readout | The even places' excitatory | 2 370.4 | 2 050.6 | 54.7 | 48.1 | 2 169.8 | 2 163.3 | 53.4 | 45.7 |
| 6 | B's answer's readout | Other excitatory | 10.4 | 3.5 | 0.8 | 0.6 | 20.4 | 7.7 | 0.8 | 0.9 |
| 6 | B's answer's readout | Inhibitory | 1 925.7 | 2 438.9 | 50.8 | 43.9 | 2 078.5 | 2 593.5 | 46.3 | 48.3 |
| 6 | B's answer's readout | All excitatory | 4 666.1 | 4 008.6 | 6 231.1 | 6 583.7 | 4 965.1 | 4 045.1 | 6 423.1 | 6 858.6 |
| 6 | B's answer's readout | Excitatory less inhibitory | 2 740.5 | 1 569.7 | 6 180.3 | 6 539.8 | 2 886.6 | 1 451.6 | 6 376.8 | 6 810.3 |
| 12 | A's answer's readout | A's kinds | 0.5 | 47.1 | 6 412.4 | 6.1 | 0.6 | 60.1 | 6 251.5 | 5.6 |
| 12 | A's answer's readout | B's kinds | 181.2 | 1.0 | 8.3 | 6 648.2 | 165.3 | 0.3 | 9.7 | 6 362.2 |
| 12 | A's answer's readout | The odd places' excitatory | 2 815.2 | 1 904.2 | 56.8 | 53.9 | 2 584.0 | 1 767.6 | 58.4 | 51.9 |
| 12 | A's answer's readout | The even places' excitatory | 2 148.4 | 2 504.9 | 49.8 | 47.0 | 2 853.3 | 2 409.1 | 51.0 | 45.9 |
| 12 | A's answer's readout | Other excitatory | 19.1 | 6.0 | 0.4 | 0.7 | 20.9 | 10.7 | 0.5 | 0.5 |
| 12 | A's answer's readout | Inhibitory | 2 040.4 | 2 396.6 | 47.0 | 44.4 | 2 460.8 | 2 741.6 | 49.4 | 47.0 |
| 12 | A's answer's readout | All excitatory | 5 164.4 | 4 463.1 | 6 527.7 | 6 755.9 | 5 624.1 | 4 247.7 | 6 371.1 | 6 466.2 |
| 12 | A's answer's readout | Excitatory less inhibitory | 3 124.0 | 2 066.5 | 6 480.7 | 6 711.4 | 3 163.4 | 1 506.1 | 6 321.6 | 6 419.1 |
| 12 | B's answer's readout | A's kinds | 0.3 | 52.3 | 6 257.8 | 5.8 | 0.6 | 60.2 | 6 415.8 | 5.9 |
| 12 | B's answer's readout | B's kinds | 137.7 | 0.2 | 9.3 | 6 427.2 | 245.9 | 0.3 | 9.5 | 6 713.5 |
| 12 | B's answer's readout | The odd places' excitatory | 2 351.8 | 2 153.4 | 59.2 | 55.3 | 2 819.2 | 2 038.6 | 57.9 | 52.8 |
| 12 | B's answer's readout | The even places' excitatory | 2 490.7 | 2 427.7 | 48.8 | 42.5 | 2 419.2 | 2 341.4 | 49.7 | 48.3 |
| 12 | B's answer's readout | Other excitatory | 13.1 | 5.5 | 0.4 | 0.5 | 33.2 | 11.0 | 0.5 | 0.7 |
| 12 | B's answer's readout | Inhibitory | 2 260.9 | 2 802.7 | 51.6 | 46.6 | 2 445.2 | 2 784.0 | 44.6 | 44.7 |
| 12 | B's answer's readout | All excitatory | 4 993.6 | 4 639.1 | 6 375.6 | 6 531.3 | 5 518.2 | 4 451.4 | 6 533.4 | 6 821.1 |
| 12 | B's answer's readout | Excitatory less inhibitory | 2 732.7 | 1 836.4 | 6 324.1 | 6 484.7 | 3 073.0 | 1 667.4 | 6 488.8 | 6 776.4 |
| 18 | A's answer's readout | A's kinds | 0.0 | 52.5 | 6 300.9 | 9.2 | 0.0 | 62.9 | 6 189.0 | 9.0 |
| 18 | A's answer's readout | B's kinds | 234.2 | 0.0 | 10.0 | 6 687.9 | 168.5 | 0.0 | 5.5 | 6 403.7 |
| 18 | A's answer's readout | The odd places' excitatory | 3 120.3 | 2 065.4 | 53.9 | 52.0 | 2 618.4 | 2 019.3 | 55.3 | 49.7 |
| 18 | A's answer's readout | The even places' excitatory | 2 370.0 | 2 778.6 | 46.9 | 52.8 | 3 120.3 | 2 557.2 | 48.9 | 49.1 |
| 18 | A's answer's readout | Other excitatory | 25.9 | 7.6 | 0.6 | 0.9 | 33.4 | 9.4 | 0.3 | 0.8 |
| 18 | A's answer's readout | Inhibitory | 2 500.2 | 2 680.9 | 52.1 | 61.3 | 2 660.0 | 2 968.9 | 46.5 | 62.0 |
| 18 | A's answer's readout | All excitatory | 5 750.4 | 4 904.1 | 6 412.3 | 6 802.8 | 5 940.6 | 4 648.8 | 6 299.1 | 6 512.2 |
| 18 | A's answer's readout | Excitatory less inhibitory | 3 250.1 | 2 223.2 | 6 360.3 | 6 741.6 | 3 280.6 | 1 679.9 | 6 252.5 | 6 450.2 |
| 18 | B's answer's readout | A's kinds | 0.0 | 58.5 | 6 363.0 | 8.9 | 0.0 | 66.8 | 6 474.9 | 8.5 |
| 18 | B's answer's readout | B's kinds | 171.6 | 0.0 | 6.6 | 6 385.4 | 252.8 | 0.0 | 6.8 | 6 671.1 |
| 18 | B's answer's readout | The odd places' excitatory | 2 533.9 | 2 405.2 | 57.3 | 50.2 | 2 937.3 | 2 256.0 | 54.0 | 52.1 |
| 18 | B's answer's readout | The even places' excitatory | 2 660.7 | 2 838.5 | 47.1 | 50.8 | 2 719.2 | 2 476.6 | 45.4 | 49.9 |
| 18 | B's answer's readout | Other excitatory | 16.5 | 6.3 | 0.3 | 0.7 | 59.8 | 11.4 | 0.6 | 1.1 |
| 18 | B's answer's readout | Inhibitory | 2 606.0 | 3 052.2 | 50.8 | 64.6 | 2 778.9 | 3 065.8 | 44.8 | 56.4 |
| 18 | B's answer's readout | All excitatory | 5 382.7 | 5 308.5 | 6 474.4 | 6 495.9 | 5 969.1 | 4 810.9 | 6 581.7 | 6 782.8 |
| 18 | B's answer's readout | Excitatory less inhibitory | 2 776.7 | 2 256.3 | 6 423.6 | 6 431.3 | 3 190.3 | 1 745.1 | 6 536.9 | 6 726.4 |
| 24 | A's answer's readout | A's kinds | 0.0 | 53.1 | 6 222.3 | 6.6 | 0.1 | 62.9 | 6 153.3 | 6.9 |
| 24 | A's answer's readout | B's kinds | 254.9 | 0.4 | 8.7 | 6 715.8 | 158.0 | 0.1 | 7.7 | 6 440.3 |
| 24 | A's answer's readout | The odd places' excitatory | 3 388.0 | 2 190.1 | 46.7 | 59.3 | 2 676.9 | 2 307.4 | 46.3 | 54.7 |
| 24 | A's answer's readout | The even places' excitatory | 2 561.4 | 3 063.2 | 54.2 | 50.3 | 3 260.0 | 2 719.8 | 54.5 | 51.0 |
| 24 | A's answer's readout | Other excitatory | 16.8 | 8.2 | 0.8 | 1.0 | 28.2 | 13.6 | 0.7 | 0.6 |
| 24 | A's answer's readout | Inhibitory | 2 873.6 | 2 954.5 | 53.1 | 56.1 | 2 934.8 | 3 187.3 | 54.0 | 55.9 |
| 24 | A's answer's readout | All excitatory | 6 221.0 | 5 314.9 | 6 332.7 | 6 833.0 | 6 123.3 | 5 103.8 | 6 262.6 | 6 553.5 |
| 24 | A's answer's readout | Excitatory less inhibitory | 3 347.4 | 2 360.5 | 6 279.6 | 6 776.9 | 3 188.4 | 1 916.6 | 6 208.5 | 6 497.6 |
| 24 | B's answer's readout | A's kinds | 0.0 | 59.3 | 6 446.5 | 7.6 | 0.2 | 74.6 | 6 517.7 | 7.2 |
| 24 | B's answer's readout | B's kinds | 181.6 | 0.1 | 7.9 | 6 365.2 | 237.1 | 0.2 | 7.3 | 6 640.9 |
| 24 | B's answer's readout | The odd places' excitatory | 2 581.2 | 2 670.2 | 48.5 | 63.9 | 3 093.8 | 2 440.5 | 43.5 | 55.5 |
| 24 | B's answer's readout | The even places' excitatory | 2 714.4 | 3 151.3 | 49.8 | 49.0 | 2 946.8 | 2 695.7 | 54.6 | 48.1 |
| 24 | B's answer's readout | Other excitatory | 9.3 | 5.4 | 0.5 | 0.7 | 60.7 | 19.1 | 1.0 | 1.0 |
| 24 | B's answer's readout | Inhibitory | 2 988.8 | 3 329.5 | 58.1 | 63.4 | 3 065.3 | 3 354.0 | 53.7 | 52.9 |
| 24 | B's answer's readout | All excitatory | 5 486.5 | 5 886.3 | 6 553.2 | 6 486.4 | 6 338.6 | 5 230.1 | 6 624.0 | 6 752.7 |
| 24 | B's answer's readout | Excitatory less inhibitory | 2 497.7 | 2 556.9 | 6 495.0 | 6 422.9 | 3 273.3 | 1 876.1 | 6 570.3 | 6 699.8 |
| 36 | A's answer's readout | A's kinds | 0.1 | 46.2 | 6 108.3 | 6.2 | 0.1 | 51.1 | 6 052.5 | 6.8 |
| 36 | A's answer's readout | B's kinds | 219.2 | 0.7 | 6.8 | 6 793.5 | 150.2 | 0.2 | 5.6 | 6 521.7 |
| 36 | A's answer's readout | The odd places' excitatory | 3 274.3 | 2 023.8 | 38.8 | 41.4 | 2 445.0 | 2 201.0 | 39.5 | 41.1 |
| 36 | A's answer's readout | The even places' excitatory | 2 233.1 | 2 959.3 | 44.7 | 40.2 | 3 167.2 | 2 619.1 | 46.4 | 42.6 |
| 36 | A's answer's readout | Other excitatory | 14.7 | 6.3 | 0.5 | 0.5 | 25.9 | 10.4 | 0.5 | 0.2 |
| 36 | A's answer's readout | Inhibitory | 2 638.0 | 2 823.5 | 45.8 | 46.0 | 2 738.3 | 3 038.1 | 42.1 | 47.5 |
| 36 | A's answer's readout | All excitatory | 5 741.4 | 5 036.4 | 6 199.3 | 6 881.7 | 5 788.4 | 4 881.8 | 6 144.4 | 6 612.4 |
| 36 | A's answer's readout | Excitatory less inhibitory | 3 103.4 | 2 212.9 | 6 153.5 | 6 835.7 | 3 050.1 | 1 843.7 | 6 102.3 | 6 564.9 |
| 36 | B's answer's readout | A's kinds | 0.1 | 52.7 | 6 569.9 | 7.1 | 0.1 | 62.2 | 6 625.4 | 7.3 |
| 36 | B's answer's readout | B's kinds | 146.1 | 0.8 | 5.8 | 6 286.9 | 216.8 | 0.0 | 5.8 | 6 561.0 |
| 36 | B's answer's readout | The odd places' excitatory | 2 541.0 | 2 527.9 | 42.9 | 44.5 | 2 898.7 | 2 350.6 | 40.2 | 39.1 |
| 36 | B's answer's readout | The even places' excitatory | 2 395.4 | 3 048.0 | 45.1 | 37.9 | 2 864.7 | 2 588.5 | 44.2 | 40.6 |
| 36 | B's answer's readout | Other excitatory | 7.8 | 4.5 | 0.4 | 0.3 | 57.2 | 14.2 | 0.7 | 0.5 |
| 36 | B's answer's readout | Inhibitory | 2 838.4 | 3 126.9 | 46.8 | 50.3 | 2 927.2 | 3 152.8 | 41.2 | 42.7 |
| 36 | B's answer's readout | All excitatory | 5 090.4 | 5 633.8 | 6 664.1 | 6 376.6 | 6 037.5 | 5 015.6 | 6 716.3 | 6 648.4 |
| 36 | B's answer's readout | Excitatory less inhibitory | 2 251.9 | 2 506.9 | 6 617.3 | 6 326.3 | 3 110.4 | 1 862.8 | 6 675.1 | 6 605.7 |

In the window the excitatory drive onto a readout was 3 788.1 to 6 338.6 k a trial and the inhibitory 1 651.8 to 3 354.0 k; the trial's
stimulus's kinds gave 0.0 to 1.0 k, and the other stimulus's kinds 46.2 to 254.9 k. In the volley the trial's stimulus's kinds
gave 6 052.5 to 6 793.5 k and all the other classes together, their excitatory and inhibitory magnitudes added,
130.1 to 184.6 k.

**Right after each single replayed night**, less the night without replay:

| Comparison | Onto | From | From the assignment: the window, A's trials, k | B's trials | the volley, A's trials | B's trials | From the mirrored assignment: the window, A's trials | B's trials | the volley, A's trials | B's trials |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's answer's readout | A's kinds | +0.1 ± 0.1 | −18.3 ± 2.8 | +350.4 ± 6.0 | −2.8 ± 1.3 | 0.0 ± 0.0 | −24.9 ± 4.4 | +342.7 ± 5.7 | −2.9 ± 1.7 |
| After block 6 alone less without replay, block 6 | A's answer's readout | B's kinds | −56.7 ± 7.7 | 0.0 ± 0.2 | −4.9 ± 2.0 | −307.9 ± 6.7 | −25.0 ± 6.7 | 0.0 ± 0.0 | −4.3 ± 1.9 | −261.9 ± 4.5 |
| After block 6 alone less without replay, block 6 | A's answer's readout | The odd places' excitatory | −990.5 ± 22.6 | −464.0 ± 13.1 | −17.4 ± 3.4 | −24.4 ± 2.8 | −822.1 ± 21.4 | −464.2 ± 14.6 | −30.8 ± 4.3 | −32.4 ± 3.8 |
| After block 6 alone less without replay, block 6 | A's answer's readout | The even places' excitatory | −687.2 ± 15.9 | −578.9 ± 17.6 | −24.1 ± 3.0 | −23.0 ± 4.0 | −888.2 ± 24.0 | −622.3 ± 17.0 | −30.1 ± 4.1 | −18.8 ± 4.8 |
| After block 6 alone less without replay, block 6 | A's answer's readout | Other excitatory | −11.5 ± 1.4 | −1.7 ± 0.8 | −0.5 ± 0.3 | −0.7 ± 0.4 | −8.2 ± 1.3 | −4.0 ± 1.3 | −0.9 ± 0.4 | 0.0 ± 0.7 |
| After block 6 alone less without replay, block 6 | A's answer's readout | Inhibitory | −163.2 ± 19.3 | −43.5 ± 22.9 | −14.1 ± 6.2 | −8.4 ± 4.6 | −191.5 ± 17.5 | −139.0 ± 29.0 | −23.4 ± 5.9 | −14.2 ± 5.1 |
| After block 6 alone less without replay, block 6 | A's answer's readout | All excitatory | −1 745.8 ± 34.9 | −1 062.9 ± 22.0 | +303.6 ± 8.0 | −358.8 ± 7.4 | −1 743.6 ± 28.2 | −1 115.4 ± 21.0 | +276.7 ± 6.2 | −316.0 ± 7.0 |
| After block 6 alone less without replay, block 6 | A's answer's readout | Excitatory less inhibitory | −1 582.7 ± 36.4 | −1 019.4 ± 21.9 | +317.7 ± 9.1 | −350.3 ± 8.5 | −1 552.1 ± 36.3 | −976.4 ± 25.0 | +300.0 ± 8.1 | −301.7 ± 9.7 |
| After block 6 alone less without replay, block 6 | B's answer's readout | A's kinds | 0.0 ± 0.0 | −22.5 ± 2.1 | −335.5 ± 5.8 | −2.5 ± 0.9 | 0.0 ± 0.0 | −31.3 ± 5.1 | −327.7 ± 5.1 | −3.6 ± 1.4 |
| After block 6 alone less without replay, block 6 | B's answer's readout | B's kinds | −34.2 ± 6.4 | −0.1 ± 0.4 | −5.1 ± 1.9 | +314.7 ± 6.6 | −32.9 ± 7.8 | 0.0 ± 0.0 | −5.9 ± 1.6 | +275.2 ± 4.4 |
| After block 6 alone less without replay, block 6 | B's answer's readout | The odd places' excitatory | −711.5 ± 21.7 | −537.8 ± 16.7 | −20.8 ± 4.1 | −22.5 ± 3.5 | −735.8 ± 21.7 | −632.5 ± 15.6 | −29.9 ± 3.8 | −29.3 ± 4.9 |
| After block 6 alone less without replay, block 6 | B's answer's readout | The even places' excitatory | −894.4 ± 15.6 | −754.2 ± 19.6 | −26.4 ± 3.3 | −24.6 ± 4.8 | −532.0 ± 16.0 | −495.7 ± 18.7 | −24.5 ± 4.1 | −14.5 ± 3.8 |
| After block 6 alone less without replay, block 6 | B's answer's readout | Other excitatory | −7.9 ± 1.0 | −1.7 ± 1.0 | −0.4 ± 0.3 | −0.5 ± 0.4 | −12.4 ± 1.8 | −3.0 ± 1.1 | −0.8 ± 0.3 | −0.3 ± 0.6 |
| After block 6 alone less without replay, block 6 | B's answer's readout | Inhibitory | −482.5 ± 17.9 | −72.0 ± 26.4 | −17.7 ± 5.2 | −8.3 ± 3.7 | −453.3 ± 24.3 | +9.9 ± 33.3 | −23.7 ± 5.5 | −17.2 ± 4.7 |
| After block 6 alone less without replay, block 6 | B's answer's readout | All excitatory | −1 648.0 ± 29.9 | −1 316.4 ± 25.7 | −388.2 ± 8.2 | +264.6 ± 10.3 | −1 313.2 ± 23.3 | −1 162.5 ± 21.9 | −388.8 ± 7.5 | +227.6 ± 7.2 |
| After block 6 alone less without replay, block 6 | B's answer's readout | Excitatory less inhibitory | −1 165.4 ± 24.9 | −1 244.4 ± 25.8 | −370.6 ± 9.3 | +272.9 ± 9.7 | −859.9 ± 34.7 | −1 172.4 ± 26.8 | −365.0 ± 8.6 | +244.7 ± 10.2 |
| After block 12 alone less without replay, block 12 | A's answer's readout | A's kinds | −0.5 ± 0.4 | −12.3 ± 3.6 | +317.2 ± 4.0 | −3.8 ± 0.9 | −0.5 ± 0.4 | −24.7 ± 3.0 | +412.6 ± 2.8 | −2.8 ± 1.5 |
| After block 12 alone less without replay, block 12 | A's answer's readout | B's kinds | −90.8 ± 6.6 | −1.0 ± 1.0 | −5.0 ± 2.1 | −264.8 ± 4.9 | −19.4 ± 6.1 | −0.3 ± 0.2 | −6.2 ± 2.7 | −253.9 ± 9.4 |
| After block 12 alone less without replay, block 12 | A's answer's readout | The odd places' excitatory | −1 048.0 ± 22.7 | −521.4 ± 18.9 | −30.5 ± 3.7 | −25.6 ± 6.8 | −765.0 ± 15.6 | −524.9 ± 15.9 | −33.5 ± 4.7 | −20.1 ± 5.4 |
| After block 12 alone less without replay, block 12 | A's answer's readout | The even places' excitatory | −796.8 ± 22.7 | −606.3 ± 24.1 | −21.3 ± 3.6 | −17.4 ± 4.1 | −1 012.0 ± 15.2 | −727.4 ± 27.0 | −23.1 ± 4.1 | −19.3 ± 3.7 |
| After block 12 alone less without replay, block 12 | A's answer's readout | Other excitatory | −14.0 ± 3.3 | −3.7 ± 0.9 | 0.0 ± 0.2 | −0.5 ± 0.4 | −13.4 ± 2.2 | −6.7 ± 1.2 | −0.2 ± 0.2 | −0.3 ± 0.2 |
| After block 12 alone less without replay, block 12 | A's answer's readout | Inhibitory | −324.1 ± 27.6 | −106.6 ± 28.1 | −18.0 ± 5.2 | −11.6 ± 6.7 | −241.0 ± 23.1 | −201.1 ± 19.2 | −26.9 ± 3.4 | −16.8 ± 5.5 |
| After block 12 alone less without replay, block 12 | A's answer's readout | All excitatory | −1 950.2 ± 41.5 | −1 144.7 ± 39.1 | +260.3 ± 6.9 | −312.2 ± 8.5 | −1 810.3 ± 26.6 | −1 284.0 ± 36.1 | +349.7 ± 6.4 | −296.4 ± 11.5 |
| After block 12 alone less without replay, block 12 | A's answer's readout | Excitatory less inhibitory | −1 626.1 ± 31.8 | −1 038.1 ± 43.3 | +278.3 ± 9.1 | −300.5 ± 11.3 | −1 569.2 ± 32.3 | −1 082.9 ± 36.5 | +376.6 ± 7.6 | −279.5 ± 12.5 |
| After block 12 alone less without replay, block 12 | B's answer's readout | A's kinds | −0.3 ± 0.2 | −19.4 ± 3.6 | −307.1 ± 4.2 | −2.4 ± 1.8 | −0.4 ± 0.4 | −30.8 ± 3.4 | −399.4 ± 2.6 | −3.9 ± 1.2 |
| After block 12 alone less without replay, block 12 | B's answer's readout | B's kinds | −54.7 ± 6.7 | −0.2 ± 0.2 | −5.4 ± 1.9 | +279.6 ± 4.4 | −34.6 ± 6.7 | −0.3 ± 0.2 | −5.8 ± 2.3 | +271.3 ± 9.6 |
| After block 12 alone less without replay, block 12 | B's answer's readout | The odd places' excitatory | −615.0 ± 23.8 | −577.1 ± 26.3 | −29.0 ± 5.0 | −24.5 ± 6.0 | −818.7 ± 19.1 | −699.5 ± 16.2 | −35.4 ± 3.7 | −22.2 ± 6.3 |
| After block 12 alone less without replay, block 12 | B's answer's readout | The even places' excitatory | −960.2 ± 19.5 | −794.6 ± 24.8 | −19.4 ± 4.0 | −14.2 ± 3.9 | −685.3 ± 17.3 | −538.9 ± 17.8 | −22.1 ± 3.6 | −20.8 ± 4.3 |
| After block 12 alone less without replay, block 12 | B's answer's readout | Other excitatory | −9.2 ± 2.2 | −3.3 ± 0.8 | −0.1 ± 0.2 | −0.3 ± 0.3 | −22.5 ± 3.8 | −7.6 ± 1.3 | −0.2 ± 0.2 | −0.5 ± 0.3 |
| After block 12 alone less without replay, block 12 | B's answer's readout | Inhibitory | −594.3 ± 23.2 | −183.8 ± 31.4 | −24.5 ± 4.3 | −14.9 ± 5.2 | −598.6 ± 18.9 | −89.9 ± 23.3 | −19.6 ± 5.0 | −16.2 ± 6.3 |
| After block 12 alone less without replay, block 12 | B's answer's readout | All excitatory | −1 639.3 ± 40.8 | −1 394.5 ± 48.4 | −361.1 ± 8.7 | +238.3 ± 7.2 | −1 561.6 ± 29.8 | −1 277.0 ± 30.7 | −462.9 ± 5.3 | +223.9 ± 12.1 |
| After block 12 alone less without replay, block 12 | B's answer's readout | Excitatory less inhibitory | −1 045.0 ± 38.7 | −1 210.7 ± 55.5 | −336.6 ± 11.2 | +253.2 ± 9.4 | −963.0 ± 28.3 | −1 187.2 ± 31.0 | −443.3 ± 8.2 | +240.2 ± 14.4 |
| After block 18 alone less without replay, block 18 | A's answer's readout | A's kinds | 0.0 ± 0.0 | −19.8 ± 3.0 | +303.5 ± 4.2 | −3.8 ± 1.6 | 0.0 ± 0.0 | −30.3 ± 4.0 | +401.0 ± 7.3 | −3.4 ± 1.5 |
| After block 18 alone less without replay, block 18 | A's answer's readout | B's kinds | −73.0 ± 5.2 | 0.0 ± 0.0 | −3.4 ± 2.2 | −257.4 ± 7.5 | −25.1 ± 3.6 | 0.0 ± 0.0 | −1.7 ± 1.6 | −256.3 ± 8.0 |
| After block 18 alone less without replay, block 18 | A's answer's readout | The odd places' excitatory | −1 000.3 ± 18.9 | −578.2 ± 15.9 | −25.8 ± 3.9 | −23.7 ± 3.7 | −687.3 ± 19.9 | −579.1 ± 16.1 | −27.8 ± 3.2 | −22.7 ± 3.7 |
| After block 18 alone less without replay, block 18 | A's answer's readout | The even places' excitatory | −886.0 ± 21.5 | −561.2 ± 23.3 | −15.4 ± 4.2 | −22.8 ± 3.6 | −987.5 ± 17.3 | −628.3 ± 14.4 | −22.1 ± 4.0 | −21.9 ± 2.9 |
| After block 18 alone less without replay, block 18 | A's answer's readout | Other excitatory | −16.8 ± 3.0 | −3.3 ± 0.9 | −0.3 ± 0.5 | −0.5 ± 0.3 | −13.4 ± 1.3 | −5.4 ± 0.9 | −0.2 ± 0.1 | −0.6 ± 0.3 |
| After block 18 alone less without replay, block 18 | A's answer's readout | Inhibitory | −511.7 ± 32.3 | −128.4 ± 19.9 | −22.3 ± 4.6 | −25.6 ± 3.8 | −206.1 ± 29.2 | −165.2 ± 15.8 | −16.8 ± 6.5 | −27.6 ± 6.2 |
| After block 18 alone less without replay, block 18 | A's answer's readout | All excitatory | −1 976.2 ± 35.3 | −1 162.5 ± 22.8 | +258.6 ± 6.0 | −308.3 ± 7.0 | −1 713.3 ± 22.1 | −1 243.1 ± 25.7 | +349.2 ± 9.0 | −305.0 ± 9.6 |
| After block 18 alone less without replay, block 18 | A's answer's readout | Excitatory less inhibitory | −1 464.5 ± 31.0 | −1 034.1 ± 29.3 | +280.9 ± 8.6 | −282.7 ± 8.4 | −1 507.2 ± 35.0 | −1 077.9 ± 23.3 | +366.0 ± 12.3 | −277.4 ± 12.9 |
| After block 18 alone less without replay, block 18 | B's answer's readout | A's kinds | 0.0 ± 0.0 | −27.3 ± 2.7 | −293.5 ± 4.4 | −2.6 ± 1.3 | 0.0 ± 0.0 | −38.2 ± 3.4 | −391.1 ± 7.1 | −4.6 ± 1.7 |
| After block 18 alone less without replay, block 18 | B's answer's readout | B's kinds | −42.0 ± 4.4 | 0.0 ± 0.0 | −1.7 ± 1.5 | +270.9 ± 7.7 | −25.8 ± 6.0 | 0.0 ± 0.0 | −1.5 ± 2.1 | +272.1 ± 7.9 |
| After block 18 alone less without replay, block 18 | B's answer's readout | The odd places' excitatory | −597.2 ± 18.6 | −663.9 ± 20.4 | −25.9 ± 4.2 | −16.9 ± 4.0 | −751.4 ± 14.2 | −697.3 ± 12.9 | −28.0 ± 4.1 | −28.0 ± 3.1 |
| After block 18 alone less without replay, block 18 | B's answer's readout | The even places' excitatory | −983.7 ± 22.0 | −803.0 ± 28.1 | −17.6 ± 3.1 | −21.2 ± 2.8 | −722.9 ± 19.9 | −501.0 ± 11.6 | −19.0 ± 4.8 | −21.9 ± 2.7 |
| After block 18 alone less without replay, block 18 | B's answer's readout | Other excitatory | −10.8 ± 1.8 | −2.9 ± 0.8 | −0.1 ± 0.3 | −0.5 ± 0.3 | −22.6 ± 3.1 | −6.7 ± 1.2 | −0.4 ± 0.2 | −0.7 ± 0.3 |
| After block 18 alone less without replay, block 18 | B's answer's readout | Inhibitory | −776.0 ± 18.5 | −203.5 ± 23.8 | −15.9 ± 5.8 | −27.1 ± 5.8 | −612.0 ± 24.9 | −69.7 ± 9.8 | −21.3 ± 5.9 | −24.5 ± 5.0 |
| After block 18 alone less without replay, block 18 | B's answer's readout | All excitatory | −1 633.7 ± 39.6 | −1 497.1 ± 31.7 | −338.8 ± 6.9 | +229.6 ± 8.3 | −1 522.7 ± 25.6 | −1 243.2 ± 18.9 | −439.9 ± 9.9 | +216.8 ± 8.8 |
| After block 18 alone less without replay, block 18 | B's answer's readout | Excitatory less inhibitory | −857.7 ± 28.8 | −1 293.6 ± 41.1 | −322.9 ± 10.3 | +256.8 ± 10.8 | −910.7 ± 34.5 | −1 173.4 ± 17.6 | −418.6 ± 12.4 | +241.3 ± 10.3 |
| After block 24 alone less without replay, block 24 | A's answer's readout | A's kinds | +0.2 ± 0.2 | −19.8 ± 3.0 | +254.0 ± 2.6 | −2.3 ± 1.1 | −0.1 ± 0.1 | −31.0 ± 3.0 | +349.6 ± 12.6 | −4.2 ± 1.6 |
| After block 24 alone less without replay, block 24 | A's answer's readout | B's kinds | −82.9 ± 6.7 | −0.3 ± 0.2 | −4.8 ± 1.7 | −313.6 ± 10.5 | −15.2 ± 6.5 | −0.1 ± 0.0 | −2.9 ± 2.2 | −344.9 ± 12.3 |
| After block 24 alone less without replay, block 24 | A's answer's readout | The odd places' excitatory | −991.2 ± 23.3 | −626.2 ± 11.3 | −22.4 ± 3.7 | −26.4 ± 4.1 | −732.1 ± 24.5 | −645.3 ± 31.7 | −20.8 ± 5.5 | −28.9 ± 4.0 |
| After block 24 alone less without replay, block 24 | A's answer's readout | The even places' excitatory | −877.2 ± 23.2 | −727.4 ± 18.4 | −22.2 ± 4.0 | −19.5 ± 2.9 | −1 009.8 ± 24.4 | −687.5 ± 19.6 | −32.2 ± 3.1 | −22.4 ± 5.6 |
| After block 24 alone less without replay, block 24 | A's answer's readout | Other excitatory | −9.0 ± 2.0 | −4.0 ± 0.7 | −0.3 ± 0.3 | −0.4 ± 0.3 | −4.6 ± 1.7 | −8.8 ± 1.0 | −0.5 ± 0.3 | −0.6 ± 0.3 |
| After block 24 alone less without replay, block 24 | A's answer's readout | Inhibitory | −427.8 ± 33.4 | −142.2 ± 19.6 | −26.7 ± 3.2 | −23.2 ± 3.8 | −225.1 ± 25.2 | −230.9 ± 27.1 | −20.2 ± 5.9 | −27.8 ± 5.0 |
| After block 24 alone less without replay, block 24 | A's answer's readout | All excitatory | −1 960.1 ± 41.3 | −1 377.9 ± 22.2 | +204.2 ± 5.6 | −362.1 ± 11.7 | −1 761.9 ± 34.6 | −1 372.7 ± 35.0 | +293.2 ± 11.1 | −401.0 ± 14.2 |
| After block 24 alone less without replay, block 24 | A's answer's readout | Excitatory less inhibitory | −1 532.3 ± 29.8 | −1 235.6 ± 26.2 | +230.9 ± 6.1 | −338.9 ± 11.4 | −1 536.8 ± 41.6 | −1 141.8 ± 41.3 | +313.4 ± 13.7 | −373.2 ± 12.6 |
| After block 24 alone less without replay, block 24 | B's answer's readout | A's kinds | +0.2 ± 0.2 | −27.0 ± 3.4 | −236.9 ± 2.6 | −3.6 ± 1.8 | −0.2 ± 0.2 | −46.1 ± 3.6 | −338.3 ± 11.9 | −4.8 ± 1.6 |
| After block 24 alone less without replay, block 24 | B's answer's readout | B's kinds | −50.7 ± 5.1 | 0.0 ± 0.1 | −3.9 ± 1.7 | +324.9 ± 10.7 | +16.5 ± 7.3 | −0.2 ± 0.1 | −2.0 ± 2.1 | +354.9 ± 11.0 |
| After block 24 alone less without replay, block 24 | B's answer's readout | The odd places' excitatory | −439.5 ± 12.2 | −741.6 ± 18.5 | −20.1 ± 4.3 | −27.2 ± 5.3 | −802.1 ± 23.2 | −740.5 ± 26.5 | −19.6 ± 6.1 | −32.0 ± 3.5 |
| After block 24 alone less without replay, block 24 | B's answer's readout | The even places' excitatory | −914.3 ± 22.7 | −880.9 ± 14.8 | −20.2 ± 4.1 | −22.8 ± 3.5 | −710.2 ± 35.4 | −618.6 ± 17.5 | −32.9 ± 4.1 | −21.5 ± 5.1 |
| After block 24 alone less without replay, block 24 | B's answer's readout | Other excitatory | −4.4 ± 1.0 | −2.5 ± 0.4 | 0.0 ± 0.3 | −0.4 ± 0.3 | −8.7 ± 2.9 | −13.3 ± 1.4 | −0.5 ± 0.4 | −1.0 ± 0.5 |
| After block 24 alone less without replay, block 24 | B's answer's readout | Inhibitory | −691.2 ± 38.1 | −238.4 ± 42.0 | −31.2 ± 3.0 | −25.8 ± 4.9 | −539.5 ± 19.4 | −177.2 ± 24.9 | −21.0 ± 6.1 | −28.3 ± 4.3 |
| After block 24 alone less without replay, block 24 | B's answer's readout | All excitatory | −1 408.8 ± 34.1 | −1 652.1 ± 24.1 | −281.1 ± 5.5 | +270.9 ± 11.2 | −1 504.7 ± 45.0 | −1 418.6 ± 38.9 | −393.4 ± 17.2 | +295.5 ± 10.6 |
| After block 24 alone less without replay, block 24 | B's answer's readout | Excitatory less inhibitory | −717.6 ± 34.9 | −1 413.7 ± 45.1 | −249.9 ± 5.7 | +296.7 ± 13.8 | −965.2 ± 51.7 | −1 241.5 ± 51.7 | −372.3 ± 16.9 | +323.9 ± 13.3 |

In the window, on both cells, both stimuli's trials and both readouts, the excitatory drive fell, by 1 062.9 to 1 976.2 k on the image from
the assignment and 1 115.4 to 1 810.3 k on the image from the mirrored assignment, and excitatory less inhibitory fell by 717.6 to 1 626.1 k
and 859.9 to 1 569.2 k, each of the 32 beyond 2 SE. The inhibitory drive fell by 43.5 to 776.0 k on the image from the assignment and moved
by −612.0 to +9.9 k on the image from the mirrored assignment, below beyond 2 SE in 30 of 32; the other two lie within 2 SE, both on B's
trials after block 6 alone and both onto readout 1: A's answer's readout on the image from the assignment and B's answer's readout on the
image from the mirrored assignment. In the volley the trial's stimulus's kinds' drive rose onto its answer's readout by 254.0 to 350.4 k on
the image from the assignment and 271.3 to 412.6 k on the image from the mirrored assignment, and fell onto the other by 236.9 to 335.5 k
and 253.9 to 399.4 k, each beyond 2 SE; the largest move of any other group's volley drive was 35.4 k.

**Times the gain.** The gain at the trial's end, read as a readout's window excitatory drive times the gain, a trial, the mean over
the 48 reads at a checkpoint on a stimulus's trials, over the mean of that drive without the gain, read 1.666 to 1.707 on the night
without replay on the image from the assignment and 1.637 to 1.660 right after the single replayed nights, and
1.666 to 1.706 and 1.628 to 1.653 on the image from the mirrored assignment. Excitatory less inhibitory, with and without it:

| Comparison | Onto | Span | From the assignment: excitatory less inhibitory on A's trials, k | times the gain | on B's trials | times the gain | From the mirrored assignment: on A's trials | times the gain | on B's trials | times the gain |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's answer's readout | The window | −1 582.7 ± 36.4 | −2 765.3 ± 61.7 | −1 019.4 ± 21.9 | −1 781.7 ± 37.0 | −1 552.1 ± 36.3 | −2 725.1 ± 61.7 | −976.4 ± 25.0 | −1 677.6 ± 42.3 |
| After block 6 alone less without replay, block 6 | A's answer's readout | The volley | +317.7 ± 9.1 | +205.1 ± 14.9 | −350.3 ± 8.5 | −885.7 ± 13.6 | +300.0 ± 8.1 | +138.3 ± 15.2 | −301.7 ± 9.7 | −832.6 ± 16.2 |
| After block 6 alone less without replay, block 6 | B's answer's readout | The window | −1 165.4 ± 24.9 | −2 064.9 ± 43.5 | −1 244.4 ± 25.8 | −2 135.8 ± 44.3 | −859.9 ± 34.7 | −1 577.7 ± 58.7 | −1 172.4 ± 26.8 | −2 011.7 ± 45.6 |
| After block 6 alone less without replay, block 6 | B's answer's readout | The volley | −370.6 ± 9.3 | −914.4 ± 16.1 | +272.9 ± 9.7 | +154.9 ± 15.9 | −365.0 ± 8.6 | −955.9 ± 15.5 | +244.7 ± 10.2 | +48.6 ± 17.4 |
| After block 12 alone less without replay, block 12 | A's answer's readout | The window | −1 626.1 ± 31.8 | −2 839.4 ± 53.6 | −1 038.1 ± 43.3 | −1 814.8 ± 74.0 | −1 569.2 ± 32.3 | −2 758.6 ± 53.9 | −1 082.9 ± 36.5 | −1 862.4 ± 61.2 |
| After block 12 alone less without replay, block 12 | A's answer's readout | The volley | +278.3 ± 9.1 | +128.6 ± 15.2 | −300.5 ± 11.3 | −818.2 ± 20.8 | +376.6 ± 7.6 | +257.4 ± 13.0 | −279.5 ± 12.5 | −806.2 ± 20.8 |
| After block 12 alone less without replay, block 12 | B's answer's readout | The window | −1 045.0 ± 38.7 | −1 860.8 ± 65.6 | −1 210.7 ± 55.5 | −2 087.4 ± 94.5 | −963.0 ± 28.3 | −1 757.1 ± 46.7 | −1 187.2 ± 31.0 | −2 042.9 ± 53.2 |
| After block 12 alone less without replay, block 12 | B's answer's readout | The volley | −336.6 ± 11.2 | −877.9 ± 19.5 | +253.2 ± 9.4 | +107.4 ± 16.7 | −443.3 ± 8.2 | −1 099.7 ± 13.0 | +240.2 ± 14.4 | +29.7 ± 24.4 |
| After block 18 alone less without replay, block 18 | A's answer's readout | The window | −1 464.5 ± 31.0 | −2 573.0 ± 51.3 | −1 034.1 ± 29.3 | −1 810.5 ± 49.5 | −1 507.2 ± 35.0 | −2 660.7 ± 57.2 | −1 077.9 ± 23.3 | −1 860.3 ± 39.3 |
| After block 18 alone less without replay, block 18 | A's answer's readout | The volley | +280.9 ± 8.6 | +137.5 ± 15.6 | −282.7 ± 8.4 | −788.7 ± 14.8 | +366.0 ± 12.3 | +225.5 ± 22.1 | −277.4 ± 12.9 | −817.8 ± 22.0 |
| After block 18 alone less without replay, block 18 | B's answer's readout | The window | −857.7 ± 28.8 | −1 551.7 ± 50.4 | −1 293.6 ± 41.1 | −2 239.8 ± 69.3 | −910.7 ± 34.5 | −1 678.4 ± 58.2 | −1 173.4 ± 17.6 | −2 018.9 ± 28.7 |
| After block 18 alone less without replay, block 18 | B's answer's readout | The volley | −322.9 ± 10.3 | −859.2 ± 19.0 | +256.8 ± 10.8 | +114.4 ± 20.6 | −418.6 ± 12.4 | −1 075.0 ± 20.7 | +241.3 ± 10.3 | +16.1 ± 16.8 |
| After block 24 alone less without replay, block 24 | A's answer's readout | The window | −1 532.3 ± 29.8 | −2 681.9 ± 50.4 | −1 235.6 ± 26.2 | −2 140.4 ± 42.8 | −1 536.8 ± 41.6 | −2 692.7 ± 69.6 | −1 141.8 ± 41.3 | −1 970.8 ± 69.1 |
| After block 24 alone less without replay, block 24 | A's answer's readout | The volley | +230.9 ± 6.1 | +47.0 ± 12.2 | −338.9 ± 11.4 | −890.6 ± 20.2 | +313.4 ± 13.7 | +135.4 ± 24.0 | −373.2 ± 12.6 | −978.5 ± 20.2 |
| After block 24 alone less without replay, block 24 | B's answer's readout | The window | −717.6 ± 34.9 | −1 305.1 ± 57.5 | −1 413.7 ± 45.1 | −2 442.9 ± 73.9 | −965.2 ± 51.7 | −1 768.1 ± 86.8 | −1 241.5 ± 51.7 | −2 129.9 ± 86.4 |
| After block 24 alone less without replay, block 24 | B's answer's readout | The volley | −249.9 ± 5.7 | −750.9 ± 11.2 | +296.7 ± 13.8 | +167.8 ± 21.9 | −372.3 ± 16.9 | −1 002.4 ± 27.6 | +323.9 ± 13.3 | +145.9 ± 22.7 |
| After block 6 alone less without replay, block 36 | A's answer's readout | The window | −888.6 ± 22.1 | −1 458.6 ± 37.1 | −403.1 ± 52.0 | −653.4 ± 86.3 | −726.7 ± 5.9 | −1 187.7 ± 10.0 | −474.5 ± 42.5 | −776.9 ± 70.8 |
| After block 6 alone less without replay, block 36 | A's answer's readout | The volley | +289.9 ± 21.6 | +548.2 ± 36.2 | −251.1 ± 31.3 | −351.8 ± 53.1 | +322.7 ± 7.7 | +601.0 ± 12.2 | −253.6 ± 27.3 | −359.8 ± 45.6 |
| After block 6 alone less without replay, block 36 | B's answer's readout | The window | −298.4 ± 52.6 | −477.7 ± 88.2 | −874.4 ± 32.7 | −1 440.7 ± 54.1 | −455.8 ± 40.7 | −733.2 ± 67.7 | −626.3 ± 15.9 | −1 031.0 ± 26.5 |
| After block 6 alone less without replay, block 36 | B's answer's readout | The volley | −305.1 ± 24.0 | −444.9 ± 40.1 | +248.4 ± 27.1 | +480.6 ± 45.2 | −330.0 ± 5.8 | −487.3 ± 10.2 | +245.5 ± 28.3 | +477.0 ± 47.5 |
| After block 12 alone less without replay, block 36 | A's answer's readout | The window | −863.0 ± 27.7 | −1 417.1 ± 45.7 | −207.5 ± 72.7 | −326.7 ± 120.7 | −647.7 ± 43.7 | −1 058.2 ± 72.1 | −319.8 ± 36.2 | −519.4 ± 59.9 |
| After block 12 alone less without replay, block 36 | A's answer's readout | The volley | +374.7 ± 12.1 | +686.1 ± 20.9 | −245.7 ± 39.7 | −347.1 ± 67.7 | +366.0 ± 9.3 | +665.9 ± 13.1 | −218.9 ± 24.7 | −309.0 ± 39.0 |
| After block 12 alone less without replay, block 36 | B's answer's readout | The window | −177.8 ± 39.6 | −276.9 ± 66.8 | −759.0 ± 57.0 | −1 248.4 ± 94.8 | −528.8 ± 54.1 | −858.6 ± 89.4 | −381.9 ± 12.2 | −623.2 ± 20.8 |
| After block 12 alone less without replay, block 36 | B's answer's readout | The volley | −379.8 ± 9.8 | −574.2 ± 16.0 | +244.5 ± 37.4 | +469.7 ± 61.4 | −370.3 ± 9.1 | −562.2 ± 17.4 | +210.7 ± 24.9 | +410.6 ± 44.0 |
| After block 18 alone less without replay, block 36 | A's answer's readout | The window | −870.7 ± 31.6 | −1 429.3 ± 52.4 | −358.4 ± 57.7 | −579.0 ± 95.7 | −729.3 ± 29.8 | −1 195.2 ± 49.8 | −309.7 ± 24.3 | −502.7 ± 40.6 |
| After block 18 alone less without replay, block 36 | A's answer's readout | The volley | +297.1 ± 18.4 | +558.3 ± 31.6 | −238.5 ± 13.7 | −332.5 ± 24.2 | +383.3 ± 18.7 | +693.9 ± 30.9 | −257.8 ± 10.4 | −375.2 ± 16.9 |
| After block 18 alone less without replay, block 36 | B's answer's readout | The window | −229.0 ± 23.5 | −361.9 ± 39.8 | −749.7 ± 39.6 | −1 232.1 ± 65.4 | −466.7 ± 42.1 | −755.0 ± 70.2 | −341.7 ± 12.3 | −556.1 ± 20.7 |
| After block 18 alone less without replay, block 36 | B's answer's readout | The volley | −305.5 ± 20.6 | −447.4 ± 33.8 | +237.2 ± 15.8 | +459.9 ± 25.6 | −395.6 ± 18.5 | −605.6 ± 31.7 | +244.0 ± 5.5 | +465.4 ± 9.4 |
| After block 24 alone less without replay, block 36 | A's answer's readout | The window | −1 041.4 ± 41.4 | −1 714.4 ± 68.7 | −684.5 ± 40.1 | −1 124.9 ± 66.0 | −962.5 ± 34.6 | −1 581.5 ± 57.9 | −628.8 ± 21.9 | −1 034.5 ± 36.3 |
| After block 24 alone less without replay, block 36 | A's answer's readout | The volley | +270.0 ± 8.4 | +516.1 ± 12.4 | −355.6 ± 8.3 | −525.5 ± 15.3 | +286.0 ± 4.6 | +544.0 ± 7.7 | −307.2 ± 30.1 | −445.1 ± 51.1 |
| After block 24 alone less without replay, block 36 | B's answer's readout | The window | −306.5 ± 51.2 | −490.9 ± 85.5 | −962.8 ± 22.7 | −1 588.4 ± 37.1 | −591.8 ± 30.7 | −959.4 ± 51.9 | −777.8 ± 10.9 | −1 284.1 ± 18.0 |
| After block 24 alone less without replay, block 36 | B's answer's readout | The volley | −280.8 ± 10.3 | −402.8 ± 18.9 | +354.1 ± 6.6 | +659.3 ± 9.4 | −319.7 ± 11.7 | −465.7 ± 20.8 | +295.9 ± 28.7 | +566.3 ± 47.1 |

Times the gain, each window fall is larger, 64 of 64 right after and at block 36, and each volley change lies lower right after,
32 of 32, the rises smaller and the falls larger; at block 36 in 16 of 32.

**Each class's drive beside its input and its units' spikes**, right after each single replayed night. On the image from the assignment:

| Comparison | Onto | From the class | Its drive's difference, the window, A's trials, k | B's trials | the volley, A's trials | B's trials | Its input difference, M | Its units' spikes difference, the window, A's trials | B's trials | the volley, A's trials | B's trials |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's answer's readout | 0: A alone | 0.0 ± 0.0 | −9.9 ± 2.5 | +158.9 ± 2.8 | −0.1 ± 0.9 | +0.16 | 0.00 ± 0.00 | −0.09 ± 0.01 | +0.03 ± 0.01 | 0.00 ± 0.00 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 1: A and C | +0.1 ± 0.1 | −8.5 ± 2.0 | +191.5 ± 3.7 | −2.7 ± 1.0 | +0.19 | 0.00 ± 0.00 | −0.07 ± 0.02 | +0.03 ± 0.01 | −0.02 ± 0.01 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 2: B alone | −7.6 ± 2.3 | −0.1 ± 0.1 | −2.4 ± 1.2 | −148.4 ± 2.4 | −0.15 | −0.05 ± 0.01 | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.01 ± 0.01 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 3: B and D | −49.1 ± 7.3 | +0.1 ± 0.1 | −2.5 ± 1.3 | −159.5 ± 5.2 | −0.16 | −0.23 ± 0.04 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.01 ± 0.01 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 4: C's own | −15.2 ± 2.4 | −20.1 ± 2.5 | +0.8 ± 1.0 | −1.2 ± 0.7 | +0.03 | −0.17 ± 0.02 | −0.19 ± 0.03 | 0.00 ± 0.01 | −0.02 ± 0.01 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 5: D's own | −51.2 ± 4.7 | −62.5 ± 4.9 | −1.4 ± 1.3 | −2.2 ± 1.0 | −0.01 | −0.35 ± 0.03 | −0.53 ± 0.03 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 6: readout 0's pattern targets | −827.6 ± 21.1 | −349.4 ± 11.3 | −12.8 ± 3.3 | −16.7 ± 2.8 | −1.81 | −5.47 ± 0.18 | −2.73 ± 0.09 | −0.09 ± 0.03 | −0.11 ± 0.02 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 7: readout 0's other excitatory | −147.7 ± 6.1 | −94.6 ± 5.0 | −5.4 ± 1.9 | −6.5 ± 1.6 | −0.78 | −0.91 ± 0.05 | −0.73 ± 0.05 | −0.05 ± 0.02 | −0.06 ± 0.02 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 8: readout 1's pattern targets | −572.9 ± 13.1 | −388.7 ± 16.4 | −14.8 ± 2.5 | −15.5 ± 3.4 | −1.09 | −3.70 ± 0.12 | −2.75 ± 0.10 | −0.11 ± 0.02 | −0.14 ± 0.03 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 9: readout 1's other excitatory | −63.1 ± 4.2 | −127.6 ± 7.7 | −7.9 ± 1.3 | −5.3 ± 2.0 | −0.42 | −0.54 ± 0.04 | −0.91 ± 0.06 | −0.07 ± 0.01 | −0.04 ± 0.02 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 10: other excitatory | −11.5 ± 1.4 | −1.7 ± 0.8 | −0.5 ± 0.3 | −0.7 ± 0.4 | −0.02 | −0.13 ± 0.02 | −0.02 ± 0.01 | −0.01 ± 0.00 | −0.01 ± 0.00 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 11: inhibitory in readout 0 | −94.2 ± 14.2 | −80.6 ± 11.7 | −12.1 ± 4.1 | −3.3 ± 2.7 | +2.10 | −1.36 ± 0.06 | −2.61 ± 0.08 | −0.12 ± 0.02 | −0.05 ± 0.02 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 12: inhibitory in readout 1 | −68.9 ± 12.8 | +37.1 ± 14.6 | −2.1 ± 3.3 | −5.1 ± 4.0 | +2.06 | −2.86 ± 0.10 | −1.28 ± 0.10 | −0.05 ± 0.02 | −0.05 ± 0.02 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 0: A alone | 0.0 ± 0.0 | −10.5 ± 1.2 | −151.1 ± 2.6 | −0.8 ± 0.5 | −0.16 | 0.00 ± 0.00 | −0.09 ± 0.01 | +0.03 ± 0.01 | 0.00 ± 0.00 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 1: A and C | 0.0 ± 0.0 | −12.1 ± 2.1 | −184.4 ± 3.8 | −1.7 ± 0.9 | −0.19 | 0.00 ± 0.00 | −0.07 ± 0.02 | +0.03 ± 0.01 | −0.02 ± 0.01 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 2: B alone | −5.3 ± 1.4 | −0.3 ± 0.3 | −3.0 ± 1.3 | +150.5 ± 2.1 | +0.15 | −0.05 ± 0.01 | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.01 ± 0.01 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 3: B and D | −28.9 ± 5.6 | +0.2 ± 0.2 | −2.1 ± 0.9 | +164.2 ± 5.3 | +0.16 | −0.23 ± 0.04 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.01 ± 0.01 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 4: C's own | −20.3 ± 2.7 | −24.8 ± 2.9 | −0.1 ± 1.3 | −1.4 ± 1.8 | −0.03 | −0.17 ± 0.02 | −0.19 ± 0.03 | 0.00 ± 0.01 | −0.02 ± 0.01 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 5: D's own | −29.4 ± 3.3 | −51.0 ± 3.3 | −1.0 ± 1.1 | −1.1 ± 0.8 | +0.01 | −0.35 ± 0.03 | −0.53 ± 0.03 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 6: readout 0's pattern targets | −565.8 ± 21.5 | −369.4 ± 12.4 | −12.5 ± 3.1 | −14.1 ± 2.9 | −0.78 | −5.47 ± 0.18 | −2.73 ± 0.09 | −0.09 ± 0.03 | −0.11 ± 0.02 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 7: readout 0's other excitatory | −125.4 ± 6.4 | −143.6 ± 10.1 | −8.1 ± 3.3 | −7.0 ± 2.4 | −0.48 | −0.91 ± 0.05 | −0.73 ± 0.05 | −0.05 ± 0.02 | −0.06 ± 0.02 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 8: readout 1's pattern targets | −791.1 ± 13.7 | −564.7 ± 17.9 | −17.3 ± 2.7 | −17.9 ± 4.1 | −2.19 | −3.70 ± 0.12 | −2.75 ± 0.10 | −0.11 ± 0.02 | −0.14 ± 0.03 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 9: readout 1's other excitatory | −74.0 ± 4.6 | −138.6 ± 6.7 | −8.1 ± 1.3 | −5.5 ± 2.2 | −0.67 | −0.54 ± 0.04 | −0.91 ± 0.06 | −0.07 ± 0.01 | −0.04 ± 0.02 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 10: other excitatory | −7.9 ± 1.0 | −1.7 ± 1.0 | −0.4 ± 0.3 | −0.5 ± 0.4 | −0.02 | −0.13 ± 0.02 | −0.02 ± 0.01 | −0.01 ± 0.00 | −0.01 ± 0.00 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 11: inhibitory in readout 0 | −200.6 ± 11.8 | −155.2 ± 15.3 | −13.0 ± 4.2 | −2.6 ± 3.1 | +2.03 | −1.36 ± 0.06 | −2.61 ± 0.08 | −0.12 ± 0.02 | −0.05 ± 0.02 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 12: inhibitory in readout 1 | −282.0 ± 15.1 | +83.1 ± 18.1 | −4.7 ± 3.4 | −5.7 ± 3.3 | +1.98 | −2.86 ± 0.10 | −1.28 ± 0.10 | −0.05 ± 0.02 | −0.05 ± 0.02 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 0: A alone | −0.4 ± 0.4 | −4.5 ± 2.6 | +187.6 ± 2.9 | −1.3 ± 0.9 | +0.18 | 0.00 ± 0.00 | −0.05 ± 0.02 | +0.03 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 1: A and C | −0.1 ± 0.1 | −7.8 ± 2.3 | +129.6 ± 2.5 | −2.5 ± 1.0 | +0.13 | 0.00 ± 0.00 | −0.07 ± 0.02 | +0.02 ± 0.01 | −0.02 ± 0.01 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 2: B alone | −10.3 ± 2.5 | 0.0 ± 0.0 | −2.6 ± 1.1 | −113.2 ± 3.3 | −0.12 | −0.08 ± 0.02 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.04 ± 0.01 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 3: B and D | −80.4 ± 6.2 | −1.0 ± 1.0 | −2.4 ± 1.6 | −151.7 ± 5.2 | −0.15 | −0.40 ± 0.03 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.02 ± 0.02 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 4: C's own | −11.1 ± 2.6 | −10.9 ± 3.5 | −0.4 ± 1.2 | −1.0 ± 1.1 | +0.03 | −0.14 ± 0.03 | −0.12 ± 0.03 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 5: D's own | −24.4 ± 5.2 | −42.2 ± 5.1 | −2.3 ± 1.2 | −1.4 ± 1.3 | −0.02 | −0.16 ± 0.03 | −0.38 ± 0.03 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 6: readout 0's pattern targets | −855.7 ± 23.1 | −374.7 ± 16.7 | −24.8 ± 3.3 | −19.2 ± 6.0 | −1.87 | −4.80 ± 0.18 | −2.74 ± 0.15 | −0.20 ± 0.03 | −0.14 ± 0.04 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 7: readout 0's other excitatory | −181.2 ± 8.5 | −135.7 ± 7.3 | −5.4 ± 2.1 | −5.4 ± 2.0 | −0.77 | −1.00 ± 0.08 | −0.91 ± 0.05 | −0.03 ± 0.02 | −0.04 ± 0.02 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 8: readout 1's pattern targets | −690.9 ± 20.1 | −454.5 ± 23.7 | −15.2 ± 3.5 | −12.3 ± 3.6 | −1.17 | −4.63 ± 0.13 | −2.87 ± 0.12 | −0.13 ± 0.03 | −0.09 ± 0.03 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 9: readout 1's other excitatory | −81.6 ± 3.5 | −109.6 ± 8.0 | −3.8 ± 1.8 | −3.7 ± 1.3 | −0.47 | −0.65 ± 0.05 | −0.81 ± 0.06 | −0.03 ± 0.02 | −0.03 ± 0.01 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 10: other excitatory | −14.0 ± 3.3 | −3.7 ± 0.9 | 0.0 ± 0.2 | −0.5 ± 0.4 | −0.02 | −0.19 ± 0.05 | −0.05 ± 0.01 | 0.00 ± 0.00 | −0.01 ± 0.00 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 11: inhibitory in readout 0 | −125.8 ± 19.4 | −92.9 ± 19.0 | −12.6 ± 3.6 | −1.7 ± 3.7 | +2.12 | −1.79 ± 0.09 | −2.71 ± 0.14 | −0.10 ± 0.02 | −0.04 ± 0.02 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 12: inhibitory in readout 1 | −198.3 ± 15.7 | −13.7 ± 20.2 | −5.3 ± 2.9 | −9.9 ± 4.4 | +2.12 | −3.75 ± 0.09 | −1.52 ± 0.12 | −0.06 ± 0.02 | −0.09 ± 0.02 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 0: A alone | −0.2 ± 0.2 | −7.7 ± 1.9 | −181.2 ± 2.9 | −1.3 ± 1.4 | −0.18 | 0.00 ± 0.00 | −0.05 ± 0.02 | +0.03 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 1: A and C | −0.2 ± 0.2 | −11.7 ± 2.9 | −125.9 ± 2.4 | −1.1 ± 1.3 | −0.13 | 0.00 ± 0.00 | −0.07 ± 0.02 | +0.02 ± 0.01 | −0.02 ± 0.01 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 2: B alone | −8.2 ± 2.0 | 0.0 ± 0.0 | −1.9 ± 0.9 | +122.0 ± 2.9 | +0.12 | −0.08 ± 0.02 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.04 ± 0.01 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 3: B and D | −46.4 ± 6.9 | −0.2 ± 0.2 | −3.5 ± 1.7 | +157.6 ± 4.9 | +0.15 | −0.40 ± 0.03 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.02 ± 0.02 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 4: C's own | −18.8 ± 3.7 | −20.4 ± 5.1 | 0.0 ± 1.3 | −1.0 ± 1.4 | −0.03 | −0.14 ± 0.03 | −0.12 ± 0.03 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 5: D's own | −5.6 ± 2.6 | −39.1 ± 2.8 | −1.3 ± 0.7 | −0.7 ± 0.8 | +0.02 | −0.16 ± 0.03 | −0.38 ± 0.03 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 6: readout 0's pattern targets | −464.7 ± 18.6 | −376.5 ± 20.0 | −24.0 ± 3.5 | −16.5 ± 4.7 | −0.75 | −4.80 ± 0.18 | −2.74 ± 0.15 | −0.20 ± 0.03 | −0.14 ± 0.04 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 7: readout 0's other excitatory | −131.5 ± 9.9 | −180.2 ± 13.0 | −5.0 ± 2.8 | −7.0 ± 3.3 | −0.43 | −1.00 ± 0.08 | −0.91 ± 0.05 | −0.03 ± 0.02 | −0.04 ± 0.02 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 8: readout 1's pattern targets | −863.9 ± 19.2 | −629.7 ± 25.4 | −14.8 ± 3.8 | −9.5 ± 3.6 | −1.95 | −4.63 ± 0.13 | −2.87 ± 0.12 | −0.13 ± 0.03 | −0.09 ± 0.03 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 9: readout 1's other excitatory | −90.6 ± 3.9 | −125.8 ± 6.8 | −3.3 ± 1.8 | −3.9 ± 1.1 | −0.64 | −0.65 ± 0.05 | −0.81 ± 0.06 | −0.03 ± 0.02 | −0.03 ± 0.01 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 10: other excitatory | −9.2 ± 2.2 | −3.3 ± 0.8 | −0.1 ± 0.2 | −0.3 ± 0.3 | −0.03 | −0.19 ± 0.05 | −0.05 ± 0.01 | 0.00 ± 0.00 | −0.01 ± 0.00 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 11: inhibitory in readout 0 | −182.7 ± 16.2 | −210.9 ± 22.6 | −15.6 ± 3.6 | −5.6 ± 2.6 | +2.13 | −1.79 ± 0.09 | −2.71 ± 0.14 | −0.10 ± 0.02 | −0.04 ± 0.02 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 12: inhibitory in readout 1 | −411.6 ± 18.9 | +27.0 ± 24.3 | −9.0 ± 2.6 | −9.4 ± 4.1 | +2.04 | −3.75 ± 0.09 | −1.52 ± 0.12 | −0.06 ± 0.02 | −0.09 ± 0.02 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 0: A alone | 0.0 ± 0.0 | −11.2 ± 2.1 | +152.6 ± 2.9 | −2.3 ± 1.1 | +0.15 | 0.00 ± 0.00 | −0.09 ± 0.01 | +0.02 ± 0.01 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 1: A and C | 0.0 ± 0.0 | −8.6 ± 1.5 | +150.9 ± 3.0 | −1.5 ± 1.1 | +0.15 | 0.00 ± 0.00 | −0.10 ± 0.01 | +0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 2: B alone | −9.8 ± 2.1 | 0.0 ± 0.0 | −0.1 ± 1.0 | −118.6 ± 4.6 | −0.12 | −0.07 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.01 | +0.02 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 3: B and D | −63.3 ± 4.6 | 0.0 ± 0.0 | −3.3 ± 1.7 | −138.7 ± 4.7 | −0.14 | −0.31 ± 0.02 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.03 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 4: C's own | −13.9 ± 1.6 | −19.9 ± 3.1 | −1.6 ± 0.7 | −0.7 ± 0.5 | +0.01 | −0.16 ± 0.02 | −0.22 ± 0.03 | −0.02 ± 0.01 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 5: D's own | −39.9 ± 4.8 | −38.0 ± 4.4 | +0.3 ± 1.2 | −1.5 ± 1.3 | −0.03 | −0.28 ± 0.03 | −0.32 ± 0.02 | 0.00 ± 0.01 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 6: readout 0's pattern targets | −830.8 ± 15.6 | −459.3 ± 9.5 | −16.6 ± 3.2 | −19.2 ± 2.3 | −1.78 | −4.27 ± 0.13 | −3.19 ± 0.09 | −0.12 ± 0.02 | −0.14 ± 0.02 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 7: readout 0's other excitatory | −155.7 ± 8.0 | −99.0 ± 8.8 | −7.5 ± 1.1 | −3.8 ± 1.6 | −0.65 | −0.83 ± 0.07 | −0.84 ± 0.07 | −0.07 ± 0.01 | −0.02 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 8: readout 1's pattern targets | −762.3 ± 18.6 | −417.4 ± 19.5 | −13.7 ± 2.6 | −17.9 ± 2.8 | −1.28 | −5.14 ± 0.11 | −2.73 ± 0.13 | −0.10 ± 0.02 | −0.13 ± 0.02 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 9: readout 1's other excitatory | −83.8 ± 3.7 | −105.9 ± 3.6 | −2.1 ± 2.1 | −3.5 ± 1.3 | −0.37 | −0.65 ± 0.04 | −0.64 ± 0.04 | −0.03 ± 0.01 | −0.03 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 10: other excitatory | −16.8 ± 3.0 | −3.3 ± 0.9 | −0.3 ± 0.5 | −0.5 ± 0.3 | −0.02 | −0.20 ± 0.04 | −0.04 ± 0.01 | 0.00 ± 0.01 | −0.01 ± 0.00 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 11: inhibitory in readout 0 | −205.9 ± 15.6 | −101.0 ± 13.8 | −12.1 ± 3.4 | −12.4 ± 3.4 | +1.99 | −1.97 ± 0.06 | −2.48 ± 0.09 | −0.08 ± 0.02 | −0.11 ± 0.02 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 12: inhibitory in readout 1 | −305.8 ± 20.5 | −27.5 ± 18.0 | −10.2 ± 4.0 | −13.1 ± 2.8 | +2.08 | −4.12 ± 0.11 | −1.62 ± 0.10 | −0.06 ± 0.02 | −0.10 ± 0.02 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 0: A alone | 0.0 ± 0.0 | −11.8 ± 2.0 | −147.0 ± 3.0 | −1.4 ± 1.0 | −0.15 | 0.00 ± 0.00 | −0.09 ± 0.01 | +0.02 ± 0.01 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 1: A and C | 0.0 ± 0.0 | −15.5 ± 2.1 | −146.5 ± 2.9 | −1.2 ± 1.0 | −0.15 | 0.00 ± 0.00 | −0.10 ± 0.01 | +0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 2: B alone | −6.6 ± 1.3 | 0.0 ± 0.0 | +0.2 ± 0.8 | +124.8 ± 4.8 | +0.12 | −0.07 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.01 | +0.02 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 3: B and D | −35.4 ± 4.1 | 0.0 ± 0.0 | −1.9 ± 1.0 | +146.1 ± 4.7 | +0.14 | −0.31 ± 0.02 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.03 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 4: C's own | −19.7 ± 2.8 | −36.2 ± 4.3 | −1.9 ± 0.6 | −1.0 ± 0.8 | −0.01 | −0.16 ± 0.02 | −0.22 ± 0.03 | −0.02 ± 0.01 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 5: D's own | −26.0 ± 2.9 | −33.0 ± 1.9 | −0.4 ± 0.9 | −1.1 ± 0.7 | +0.03 | −0.28 ± 0.03 | −0.32 ± 0.02 | 0.00 ± 0.01 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 6: readout 0's pattern targets | −461.2 ± 15.5 | −447.5 ± 10.2 | −14.9 ± 3.1 | −14.3 ± 2.4 | −0.83 | −4.27 ± 0.13 | −3.19 ± 0.09 | −0.12 ± 0.02 | −0.14 ± 0.02 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 7: readout 0's other excitatory | −116.4 ± 6.5 | −180.2 ± 14.7 | −9.1 ± 1.8 | −1.6 ± 1.9 | −0.44 | −0.83 ± 0.07 | −0.84 ± 0.07 | −0.07 ± 0.01 | −0.02 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 8: readout 1's pattern targets | −860.9 ± 21.2 | −654.9 ± 26.0 | −13.6 ± 2.4 | −16.5 ± 2.4 | −1.96 | −5.14 ± 0.11 | −2.73 ± 0.13 | −0.10 ± 0.02 | −0.13 ± 0.02 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 9: readout 1's other excitatory | −96.8 ± 4.5 | −115.0 ± 5.1 | −3.7 ± 1.4 | −3.7 ± 1.3 | −0.59 | −0.65 ± 0.04 | −0.64 ± 0.04 | −0.03 ± 0.01 | −0.03 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 10: other excitatory | −10.8 ± 1.8 | −2.9 ± 0.8 | −0.1 ± 0.3 | −0.5 ± 0.3 | −0.03 | −0.20 ± 0.04 | −0.04 ± 0.01 | 0.00 ± 0.01 | −0.01 ± 0.00 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 11: inhibitory in readout 0 | −265.6 ± 10.7 | −167.0 ± 15.7 | −9.1 ± 3.4 | −15.0 ± 4.3 | +1.93 | −1.97 ± 0.06 | −2.48 ± 0.09 | −0.08 ± 0.02 | −0.11 ± 0.02 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 12: inhibitory in readout 1 | −510.4 ± 20.5 | −36.5 ± 20.0 | −6.8 ± 4.5 | −12.1 ± 2.9 | +1.94 | −4.12 ± 0.11 | −1.62 ± 0.10 | −0.06 ± 0.02 | −0.10 ± 0.02 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 0: A alone | +0.2 ± 0.2 | −9.9 ± 2.5 | +128.7 ± 2.3 | −0.9 ± 0.6 | +0.12 | 0.00 ± 0.00 | −0.08 ± 0.01 | +0.03 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 1: A and C | 0.0 ± 0.0 | −10.0 ± 2.4 | +125.3 ± 3.9 | −1.4 ± 0.8 | +0.12 | 0.00 ± 0.00 | −0.12 ± 0.03 | +0.03 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 2: B alone | −4.3 ± 2.9 | 0.0 ± 0.0 | −2.2 ± 1.2 | −178.6 ± 4.5 | −0.18 | −0.05 ± 0.02 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.02 ± 0.01 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 3: B and D | −78.6 ± 5.5 | −0.3 ± 0.2 | −2.7 ± 1.2 | −135.0 ± 12.0 | −0.14 | −0.37 ± 0.03 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.03 ± 0.01 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 4: C's own | −11.1 ± 1.3 | −16.0 ± 1.3 | −0.8 ± 0.7 | −1.7 ± 0.8 | 0.00 | −0.14 ± 0.02 | −0.16 ± 0.02 | −0.01 ± 0.01 | −0.02 ± 0.01 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 5: D's own | −41.6 ± 3.9 | −46.1 ± 4.2 | −2.1 ± 1.1 | −1.2 ± 1.5 | −0.01 | −0.26 ± 0.02 | −0.49 ± 0.03 | −0.02 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 6: readout 0's pattern targets | −832.2 ± 27.2 | −524.5 ± 10.3 | −19.0 ± 3.5 | −17.8 ± 3.5 | −1.72 | −3.37 ± 0.13 | −3.46 ± 0.09 | −0.14 ± 0.03 | −0.12 ± 0.02 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 7: readout 0's other excitatory | −147.8 ± 9.1 | −85.7 ± 5.0 | −2.6 ± 1.5 | −6.8 ± 2.0 | −0.69 | −0.52 ± 0.03 | −0.90 ± 0.06 | −0.02 ± 0.01 | −0.06 ± 0.02 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 8: readout 1's pattern targets | −741.6 ± 18.7 | −487.6 ± 13.9 | −18.9 ± 4.3 | −16.7 ± 2.5 | −1.16 | −4.90 ± 0.12 | −2.97 ± 0.06 | −0.15 ± 0.03 | −0.14 ± 0.02 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 9: readout 1's other excitatory | −94.0 ± 5.8 | −193.7 ± 5.5 | −1.3 ± 2.1 | −1.5 ± 1.2 | −0.42 | −0.74 ± 0.06 | −1.09 ± 0.06 | −0.01 ± 0.01 | −0.02 ± 0.01 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 10: other excitatory | −9.0 ± 2.0 | −4.0 ± 0.7 | −0.3 ± 0.3 | −0.4 ± 0.3 | −0.04 | −0.09 ± 0.02 | −0.04 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 11: inhibitory in readout 0 | −140.8 ± 23.9 | −100.2 ± 14.1 | −14.8 ± 2.0 | −12.7 ± 3.4 | +2.06 | −2.00 ± 0.09 | −2.25 ± 0.09 | −0.11 ± 0.02 | −0.10 ± 0.02 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 12: inhibitory in readout 1 | −287.0 ± 20.6 | −42.0 ± 15.0 | −11.9 ± 2.7 | −10.6 ± 3.3 | +1.99 | −3.85 ± 0.09 | −1.66 ± 0.09 | −0.09 ± 0.01 | −0.08 ± 0.02 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 0: A alone | +0.2 ± 0.2 | −10.7 ± 2.0 | −121.8 ± 2.1 | −2.2 ± 1.2 | −0.12 | 0.00 ± 0.00 | −0.08 ± 0.01 | +0.03 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 1: A and C | 0.0 ± 0.0 | −16.3 ± 3.2 | −115.1 ± 4.0 | −1.4 ± 0.9 | −0.12 | 0.00 ± 0.00 | −0.12 ± 0.03 | +0.03 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 2: B alone | −5.5 ± 2.0 | 0.0 ± 0.0 | −2.2 ± 1.2 | +184.1 ± 4.3 | +0.18 | −0.05 ± 0.02 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.02 ± 0.01 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 3: B and D | −45.2 ± 4.9 | 0.0 ± 0.1 | −1.7 ± 1.1 | +140.8 ± 12.0 | +0.14 | −0.37 ± 0.03 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.03 ± 0.01 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 4: C's own | −15.4 ± 2.2 | −20.0 ± 2.7 | −0.6 ± 0.8 | −3.1 ± 1.4 | 0.00 | −0.14 ± 0.02 | −0.16 ± 0.02 | −0.01 ± 0.01 | −0.02 ± 0.01 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 5: D's own | −22.2 ± 2.3 | −53.0 ± 3.1 | −1.6 ± 0.7 | −1.1 ± 0.6 | +0.01 | −0.26 ± 0.02 | −0.49 ± 0.03 | −0.02 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 6: readout 0's pattern targets | −344.2 ± 12.4 | −484.4 ± 10.3 | −16.7 ± 3.4 | −16.1 ± 3.6 | −0.62 | −3.37 ± 0.13 | −3.46 ± 0.09 | −0.14 ± 0.03 | −0.12 ± 0.02 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 7: readout 0's other excitatory | −79.8 ± 4.6 | −237.2 ± 16.6 | −2.8 ± 2.0 | −8.0 ± 3.1 | −0.47 | −0.52 ± 0.03 | −0.90 ± 0.06 | −0.02 ± 0.01 | −0.06 ± 0.02 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 8: readout 1's pattern targets | −788.3 ± 18.8 | −668.9 ± 11.5 | −16.0 ± 3.9 | −19.9 ± 3.4 | −1.76 | −4.90 ± 0.12 | −2.97 ± 0.06 | −0.15 ± 0.03 | −0.14 ± 0.02 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 9: readout 1's other excitatory | −103.8 ± 8.5 | −159.1 ± 6.5 | −2.5 ± 1.5 | −1.8 ± 1.5 | −0.58 | −0.74 ± 0.06 | −1.09 ± 0.06 | −0.01 ± 0.01 | −0.02 ± 0.01 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 10: other excitatory | −4.4 ± 1.0 | −2.5 ± 0.4 | 0.0 ± 0.3 | −0.4 ± 0.3 | −0.02 | −0.09 ± 0.02 | −0.04 ± 0.01 | 0.00 ± 0.00 | 0.00 ± 0.00 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 11: inhibitory in readout 0 | −250.9 ± 17.5 | −176.0 ± 24.0 | −17.2 ± 3.1 | −14.8 ± 3.5 | +1.96 | −2.00 ± 0.09 | −2.25 ± 0.09 | −0.11 ± 0.02 | −0.10 ± 0.02 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 12: inhibitory in readout 1 | −440.3 ± 25.4 | −62.4 ± 21.7 | −14.0 ± 3.0 | −11.0 ± 3.3 | +1.90 | −3.85 ± 0.09 | −1.66 ± 0.09 | −0.09 ± 0.01 | −0.08 ± 0.02 |

On the image from the mirrored assignment:

| Comparison | Onto | From the class | Its drive's difference, the window, A's trials, k | B's trials | the volley, A's trials | B's trials | Its input difference, M | Its units' spikes difference, the window, A's trials | B's trials | the volley, A's trials | B's trials |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's answer's readout | 0: A alone | 0.0 ± 0.0 | −9.6 ± 2.4 | +188.6 ± 2.9 | −0.6 ± 1.3 | +0.18 | 0.00 ± 0.00 | −0.08 ± 0.02 | +0.02 ± 0.01 | 0.00 ± 0.01 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 1: A and C | 0.0 ± 0.0 | −15.3 ± 3.1 | +154.1 ± 4.8 | −2.3 ± 0.8 | +0.15 | 0.00 ± 0.00 | −0.13 ± 0.02 | +0.03 ± 0.01 | −0.03 ± 0.01 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 2: B alone | −8.0 ± 1.8 | 0.0 ± 0.0 | −2.1 ± 0.9 | −97.9 ± 2.0 | −0.10 | −0.08 ± 0.01 | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.03 ± 0.01 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 3: B and D | −17.1 ± 6.2 | 0.0 ± 0.0 | −2.1 ± 1.6 | −163.9 ± 6.1 | −0.17 | −0.13 ± 0.04 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.03 ± 0.01 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 4: C's own | −29.7 ± 3.2 | −11.7 ± 3.5 | −1.2 ± 1.1 | −2.4 ± 0.8 | +0.10 | −0.31 ± 0.03 | −0.23 ± 0.03 | −0.01 ± 0.01 | −0.02 ± 0.01 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 5: D's own | −33.8 ± 5.8 | −7.6 ± 3.0 | −0.6 ± 1.1 | −0.2 ± 1.1 | −0.07 | −0.18 ± 0.03 | −0.06 ± 0.02 | 0.00 ± 0.01 | 0.00 ± 0.01 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 6: readout 0's pattern targets | −754.8 ± 18.5 | −389.9 ± 14.5 | −24.3 ± 3.6 | −25.3 ± 3.2 | −0.85 | −5.56 ± 0.14 | −3.14 ± 0.13 | −0.21 ± 0.03 | −0.22 ± 0.03 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 7: readout 0's other excitatory | −37.6 ± 4.3 | −62.6 ± 5.7 | −5.3 ± 1.8 | −4.6 ± 2.2 | −0.42 | −0.35 ± 0.04 | −0.35 ± 0.05 | −0.05 ± 0.01 | −0.02 ± 0.02 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 8: readout 1's pattern targets | −592.5 ± 16.5 | −565.0 ± 15.7 | −22.8 ± 3.4 | −13.6 ± 3.6 | −2.12 | −3.02 ± 0.09 | −3.31 ± 0.12 | −0.16 ± 0.03 | −0.09 ± 0.03 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 9: readout 1's other excitatory | −261.9 ± 12.5 | −49.7 ± 4.6 | −6.7 ± 1.8 | −5.0 ± 2.0 | −0.71 | −1.57 ± 0.08 | −0.36 ± 0.04 | −0.05 ± 0.01 | −0.04 ± 0.02 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 10: other excitatory | −8.2 ± 1.3 | −4.0 ± 1.3 | −0.9 ± 0.4 | 0.0 ± 0.7 | −0.01 | −0.14 ± 0.02 | −0.04 ± 0.02 | −0.01 ± 0.00 | 0.00 ± 0.01 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 11: inhibitory in readout 0 | −102.8 ± 9.4 | −84.0 ± 26.0 | −11.3 ± 4.7 | −7.9 ± 4.1 | +2.66 | −2.09 ± 0.07 | −1.94 ± 0.16 | −0.10 ± 0.03 | −0.09 ± 0.03 |
| After block 6 alone less without replay, block 6 | A's answer's readout | 12: inhibitory in readout 1 | −88.7 ± 15.2 | −55.0 ± 12.7 | −12.1 ± 3.3 | −6.4 ± 2.8 | +2.30 | −2.71 ± 0.08 | −2.14 ± 0.06 | −0.10 ± 0.02 | −0.05 ± 0.02 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 0: A alone | 0.0 ± 0.0 | −9.4 ± 2.7 | −182.5 ± 1.7 | −0.2 ± 1.0 | −0.18 | 0.00 ± 0.00 | −0.08 ± 0.02 | +0.02 ± 0.01 | 0.00 ± 0.01 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 1: A and C | 0.0 ± 0.0 | −22.0 ± 4.1 | −145.1 ± 4.6 | −3.4 ± 1.0 | −0.15 | 0.00 ± 0.00 | −0.13 ± 0.02 | +0.03 ± 0.01 | −0.03 ± 0.01 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 2: B alone | −17.2 ± 2.6 | 0.0 ± 0.0 | −3.6 ± 1.5 | +104.8 ± 2.7 | +0.10 | −0.08 ± 0.01 | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.03 ± 0.01 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 3: B and D | −15.7 ± 7.2 | 0.0 ± 0.0 | −2.3 ± 1.2 | +170.4 ± 5.7 | +0.17 | −0.13 ± 0.04 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.03 ± 0.01 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 4: C's own | −30.7 ± 4.8 | −33.5 ± 3.7 | −0.5 ± 1.1 | −2.8 ± 1.5 | −0.10 | −0.31 ± 0.03 | −0.23 ± 0.03 | −0.01 ± 0.01 | −0.02 ± 0.01 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 5: D's own | −19.6 ± 3.5 | +4.8 ± 2.7 | +0.1 ± 1.1 | +0.3 ± 0.7 | +0.07 | −0.18 ± 0.03 | −0.06 ± 0.02 | 0.00 ± 0.01 | 0.00 ± 0.01 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 6: readout 0's pattern targets | −658.4 ± 17.4 | −528.8 ± 14.8 | −23.8 ± 3.4 | −24.6 ± 3.8 | −2.22 | −5.56 ± 0.14 | −3.14 ± 0.13 | −0.21 ± 0.03 | −0.22 ± 0.03 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 7: readout 0's other excitatory | −46.8 ± 5.5 | −70.2 ± 4.4 | −5.7 ± 1.4 | −1.9 ± 1.9 | −0.37 | −0.35 ± 0.04 | −0.35 ± 0.05 | −0.05 ± 0.01 | −0.02 ± 0.02 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 8: readout 1's pattern targets | −343.0 ± 13.0 | −458.7 ± 18.9 | −19.1 ± 3.5 | −10.2 ± 2.8 | −0.31 | −3.02 ± 0.09 | −3.31 ± 0.12 | −0.16 ± 0.03 | −0.09 ± 0.03 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 9: readout 1's other excitatory | −169.3 ± 8.7 | −41.8 ± 4.5 | −5.4 ± 1.5 | −4.6 ± 2.0 | −0.29 | −1.57 ± 0.08 | −0.36 ± 0.04 | −0.05 ± 0.01 | −0.04 ± 0.02 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 10: other excitatory | −12.4 ± 1.8 | −3.0 ± 1.1 | −0.8 ± 0.3 | −0.3 ± 0.6 | −0.01 | −0.14 ± 0.02 | −0.04 ± 0.02 | −0.01 ± 0.00 | 0.00 ± 0.01 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 11: inhibitory in readout 0 | −274.0 ± 16.5 | +19.7 ± 28.8 | −10.9 ± 4.1 | −10.0 ± 3.3 | +1.95 | −2.09 ± 0.07 | −1.94 ± 0.16 | −0.10 ± 0.03 | −0.09 ± 0.03 |
| After block 6 alone less without replay, block 6 | B's answer's readout | 12: inhibitory in readout 1 | −179.3 ± 14.9 | −9.8 ± 13.9 | −12.8 ± 2.9 | −7.1 ± 3.2 | +1.98 | −2.71 ± 0.08 | −2.14 ± 0.06 | −0.10 ± 0.02 | −0.05 ± 0.02 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 0: A alone | −0.1 ± 0.2 | −8.6 ± 1.8 | +239.2 ± 2.6 | −0.9 ± 1.2 | +0.24 | 0.00 ± 0.00 | −0.07 ± 0.02 | +0.04 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 1: A and C | −0.4 ± 0.4 | −16.1 ± 1.9 | +173.4 ± 2.1 | −1.9 ± 1.2 | +0.17 | 0.00 ± 0.00 | −0.14 ± 0.02 | +0.02 ± 0.01 | −0.02 ± 0.01 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 2: B alone | −6.6 ± 1.8 | −0.1 ± 0.1 | −2.9 ± 0.7 | −85.2 ± 3.5 | −0.09 | −0.09 ± 0.02 | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.04 ± 0.01 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 3: B and D | −12.8 ± 5.7 | −0.2 ± 0.2 | −3.3 ± 2.3 | −168.6 ± 6.1 | −0.17 | −0.12 ± 0.03 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.02 ± 0.02 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 4: C's own | −35.5 ± 3.2 | −22.1 ± 4.9 | −1.1 ± 1.0 | −0.9 ± 0.9 | +0.11 | −0.39 ± 0.03 | −0.28 ± 0.04 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 5: D's own | −38.0 ± 5.8 | −8.9 ± 2.4 | −2.6 ± 1.0 | −1.4 ± 0.9 | −0.04 | −0.20 ± 0.02 | −0.10 ± 0.02 | −0.02 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 6: readout 0's pattern targets | −699.2 ± 15.4 | −445.7 ± 13.5 | −25.1 ± 3.7 | −13.9 ± 5.0 | −0.72 | −5.33 ± 0.13 | −3.47 ± 0.10 | −0.20 ± 0.03 | −0.13 ± 0.04 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 7: readout 0's other excitatory | −30.3 ± 4.5 | −57.1 ± 6.9 | −7.3 ± 1.9 | −5.3 ± 2.6 | −0.39 | −0.28 ± 0.03 | −0.33 ± 0.05 | −0.06 ± 0.01 | −0.03 ± 0.02 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 8: readout 1's pattern targets | −647.6 ± 10.1 | −668.4 ± 25.0 | −16.3 ± 4.2 | −15.0 ± 3.2 | −1.97 | −3.86 ± 0.10 | −3.23 ± 0.11 | −0.14 ± 0.04 | −0.12 ± 0.03 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 9: readout 1's other excitatory | −326.3 ± 9.6 | −50.1 ± 4.1 | −4.1 ± 1.3 | −2.9 ± 1.9 | −0.72 | −1.71 ± 0.07 | −0.35 ± 0.03 | −0.04 ± 0.01 | −0.03 ± 0.02 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 10: other excitatory | −13.4 ± 2.2 | −6.7 ± 1.2 | −0.2 ± 0.2 | −0.3 ± 0.2 | −0.01 | −0.33 ± 0.06 | −0.10 ± 0.02 | 0.00 ± 0.00 | −0.01 ± 0.00 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 11: inhibitory in readout 0 | −137.7 ± 12.0 | −163.3 ± 16.8 | −13.4 ± 2.2 | −7.3 ± 3.4 | +2.65 | −2.48 ± 0.06 | −2.07 ± 0.10 | −0.10 ± 0.02 | −0.06 ± 0.02 |
| After block 12 alone less without replay, block 12 | A's answer's readout | 12: inhibitory in readout 1 | −103.3 ± 18.6 | −37.8 ± 12.0 | −13.5 ± 2.7 | −9.5 ± 3.7 | +2.45 | −3.40 ± 0.11 | −2.40 ± 0.09 | −0.08 ± 0.02 | −0.09 ± 0.02 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 0: A alone | −0.3 ± 0.4 | −10.4 ± 2.0 | −231.0 ± 2.3 | −1.3 ± 0.7 | −0.24 | 0.00 ± 0.00 | −0.07 ± 0.02 | +0.04 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 1: A and C | −0.1 ± 0.1 | −20.4 ± 2.7 | −168.4 ± 2.3 | −2.6 ± 1.3 | −0.17 | 0.00 ± 0.00 | −0.14 ± 0.02 | +0.02 ± 0.01 | −0.02 ± 0.01 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 2: B alone | −15.4 ± 3.6 | −0.1 ± 0.1 | −3.7 ± 1.3 | +97.5 ± 4.5 | +0.09 | −0.09 ± 0.02 | 0.00 ± 0.00 | −0.03 ± 0.01 | +0.04 ± 0.01 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 3: B and D | −19.2 ± 7.2 | −0.2 ± 0.2 | −2.1 ± 1.4 | +173.8 ± 5.6 | +0.17 | −0.12 ± 0.03 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.02 ± 0.02 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 4: C's own | −39.8 ± 4.0 | −39.9 ± 3.8 | −1.6 ± 1.2 | −1.9 ± 1.4 | −0.11 | −0.39 ± 0.03 | −0.28 ± 0.04 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 5: D's own | −26.8 ± 2.5 | −5.9 ± 2.3 | −1.9 ± 0.9 | −1.2 ± 0.9 | +0.04 | −0.20 ± 0.02 | −0.10 ± 0.02 | −0.02 ± 0.01 | −0.01 ± 0.01 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 6: readout 0's pattern targets | −733.4 ± 18.3 | −596.6 ± 14.7 | −27.1 ± 3.2 | −15.8 ± 5.7 | −2.15 | −5.33 ± 0.13 | −3.47 ± 0.10 | −0.20 ± 0.03 | −0.13 ± 0.04 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 7: readout 0's other excitatory | −45.5 ± 4.9 | −63.1 ± 5.4 | −6.7 ± 1.3 | −4.4 ± 2.1 | −0.39 | −0.28 ± 0.03 | −0.33 ± 0.05 | −0.06 ± 0.01 | −0.03 ± 0.02 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 8: readout 1's pattern targets | −476.8 ± 14.0 | −487.7 ± 17.2 | −15.7 ± 4.3 | −15.2 ± 3.7 | −0.52 | −3.86 ± 0.10 | −3.23 ± 0.11 | −0.14 ± 0.04 | −0.12 ± 0.03 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 9: readout 1's other excitatory | −181.7 ± 8.0 | −45.2 ± 3.5 | −4.5 ± 1.5 | −4.4 ± 1.6 | −0.29 | −1.71 ± 0.07 | −0.35 ± 0.03 | −0.04 ± 0.01 | −0.03 ± 0.02 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 10: other excitatory | −22.5 ± 3.8 | −7.6 ± 1.3 | −0.2 ± 0.2 | −0.5 ± 0.3 | −0.02 | −0.33 ± 0.06 | −0.10 ± 0.02 | 0.00 ± 0.00 | −0.01 ± 0.00 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 11: inhibitory in readout 0 | −346.3 ± 10.2 | −32.8 ± 16.2 | −11.9 ± 3.5 | −5.8 ± 4.1 | +1.94 | −2.48 ± 0.06 | −2.07 ± 0.10 | −0.10 ± 0.02 | −0.06 ± 0.02 |
| After block 12 alone less without replay, block 12 | B's answer's readout | 12: inhibitory in readout 1 | −252.3 ± 16.4 | −57.1 ± 17.1 | −7.7 ± 2.8 | −10.5 ± 3.8 | +1.97 | −3.40 ± 0.11 | −2.40 ± 0.09 | −0.08 ± 0.02 | −0.09 ± 0.02 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 0: A alone | 0.0 ± 0.0 | −11.5 ± 2.9 | +201.4 ± 1.7 | −2.7 ± 1.5 | +0.20 | 0.00 ± 0.00 | −0.10 ± 0.02 | +0.02 ± 0.01 | −0.02 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 1: A and C | 0.0 ± 0.0 | −18.8 ± 2.3 | +199.6 ± 6.8 | −0.7 ± 1.2 | +0.20 | 0.00 ± 0.00 | −0.15 ± 0.01 | +0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 2: B alone | −7.9 ± 1.3 | 0.0 ± 0.0 | 0.0 ± 0.7 | −107.2 ± 2.4 | −0.11 | −0.08 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.01 | +0.03 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 3: B and D | −17.1 ± 3.7 | 0.0 ± 0.0 | −1.7 ± 1.1 | −149.1 ± 9.6 | −0.15 | −0.09 ± 0.02 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.03 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 4: C's own | −43.0 ± 3.1 | −47.1 ± 3.5 | −1.9 ± 0.4 | −1.1 ± 0.5 | +0.11 | −0.53 ± 0.04 | −0.50 ± 0.02 | −0.02 ± 0.00 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 5: D's own | −45.8 ± 8.5 | −14.7 ± 1.2 | −1.6 ± 1.6 | −2.3 ± 0.9 | −0.02 | −0.23 ± 0.04 | −0.09 ± 0.01 | 0.00 ± 0.01 | −0.02 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 6: readout 0's pattern targets | −623.1 ± 18.2 | −479.5 ± 16.2 | −21.8 ± 3.2 | −17.7 ± 2.8 | −0.95 | −4.20 ± 0.13 | −3.57 ± 0.13 | −0.19 ± 0.02 | −0.17 ± 0.02 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 7: readout 0's other excitatory | −21.2 ± 2.1 | −52.5 ± 5.4 | −4.1 ± 1.7 | −3.9 ± 1.5 | −0.30 | −0.14 ± 0.02 | −0.34 ± 0.03 | −0.02 ± 0.01 | −0.04 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 8: readout 1's pattern targets | −620.4 ± 13.7 | −555.1 ± 11.9 | −16.7 ± 2.9 | −14.3 ± 2.9 | −1.97 | −4.07 ± 0.13 | −3.02 ± 0.07 | −0.12 ± 0.03 | −0.12 ± 0.02 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 9: readout 1's other excitatory | −321.3 ± 9.2 | −58.5 ± 5.1 | −3.8 ± 1.2 | −5.4 ± 1.4 | −0.70 | −1.74 ± 0.06 | −0.45 ± 0.04 | −0.03 ± 0.01 | −0.05 ± 0.01 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 10: other excitatory | −13.4 ± 1.3 | −5.4 ± 0.9 | −0.2 ± 0.1 | −0.6 ± 0.3 | 0.00 | −0.26 ± 0.03 | −0.08 ± 0.01 | −0.01 ± 0.00 | −0.01 ± 0.00 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 11: inhibitory in readout 0 | −90.8 ± 16.3 | −185.6 ± 19.0 | −8.8 ± 4.3 | −16.7 ± 4.2 | +2.58 | −2.21 ± 0.07 | −2.39 ± 0.10 | −0.07 ± 0.02 | −0.11 ± 0.02 |
| After block 18 alone less without replay, block 18 | A's answer's readout | 12: inhibitory in readout 1 | −115.2 ± 18.2 | +20.4 ± 13.2 | −8.0 ± 4.2 | −10.9 ± 4.0 | +2.47 | −3.45 ± 0.09 | −1.92 ± 0.07 | −0.07 ± 0.02 | −0.10 ± 0.03 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 0: A alone | 0.0 ± 0.0 | −13.6 ± 2.1 | −196.1 ± 1.9 | −3.2 ± 1.0 | −0.20 | 0.00 ± 0.00 | −0.10 ± 0.02 | +0.02 ± 0.01 | −0.02 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 1: A and C | 0.0 ± 0.0 | −24.6 ± 2.1 | −194.9 ± 6.9 | −1.4 ± 1.4 | −0.20 | 0.00 ± 0.00 | −0.15 ± 0.01 | +0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 2: B alone | −11.4 ± 2.7 | 0.0 ± 0.0 | +0.6 ± 1.2 | +114.5 ± 2.1 | +0.11 | −0.08 ± 0.02 | 0.00 ± 0.00 | 0.00 ± 0.01 | +0.03 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 3: B and D | −14.4 ± 5.2 | 0.0 ± 0.0 | −2.1 ± 1.1 | +157.6 ± 9.3 | +0.15 | −0.09 ± 0.02 | 0.00 ± 0.00 | −0.02 ± 0.01 | +0.03 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 4: C's own | −54.0 ± 5.1 | −53.1 ± 3.0 | −2.9 ± 0.9 | −1.7 ± 0.8 | −0.11 | −0.53 ± 0.04 | −0.50 ± 0.02 | −0.02 ± 0.00 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 5: D's own | −32.8 ± 3.9 | −9.3 ± 1.5 | −0.4 ± 1.2 | −1.5 ± 0.8 | +0.02 | −0.23 ± 0.04 | −0.09 ± 0.01 | 0.00 ± 0.01 | −0.02 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 6: readout 0's pattern targets | −675.1 ± 12.2 | −577.2 ± 11.7 | −22.7 ± 3.4 | −21.6 ± 2.5 | −2.03 | −4.20 ± 0.13 | −3.57 ± 0.13 | −0.19 ± 0.02 | −0.17 ± 0.02 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 7: readout 0's other excitatory | −22.3 ± 3.3 | −67.0 ± 3.5 | −2.4 ± 1.2 | −4.7 ± 1.2 | −0.35 | −0.14 ± 0.02 | −0.34 ± 0.03 | −0.02 ± 0.01 | −0.04 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 8: readout 1's pattern targets | −500.8 ± 18.7 | −427.4 ± 11.3 | −15.3 ± 3.9 | −16.0 ± 2.3 | −0.43 | −4.07 ± 0.13 | −3.02 ± 0.07 | −0.12 ± 0.03 | −0.12 ± 0.02 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 9: readout 1's other excitatory | −189.3 ± 6.9 | −64.3 ± 6.8 | −3.3 ± 1.1 | −4.4 ± 1.2 | −0.32 | −1.74 ± 0.06 | −0.45 ± 0.04 | −0.03 ± 0.01 | −0.05 ± 0.01 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 10: other excitatory | −22.6 ± 3.1 | −6.7 ± 1.2 | −0.4 ± 0.2 | −0.7 ± 0.3 | −0.02 | −0.26 ± 0.03 | −0.08 ± 0.01 | −0.01 ± 0.00 | −0.01 ± 0.00 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 11: inhibitory in readout 0 | −331.7 ± 13.4 | −104.0 ± 16.7 | −11.0 ± 3.5 | −12.6 ± 3.4 | +1.97 | −2.21 ± 0.07 | −2.39 ± 0.10 | −0.07 ± 0.02 | −0.11 ± 0.02 |
| After block 18 alone less without replay, block 18 | B's answer's readout | 12: inhibitory in readout 1 | −280.3 ± 15.5 | +34.3 ± 10.2 | −10.3 ± 3.7 | −11.9 ± 3.8 | +1.99 | −3.45 ± 0.09 | −1.92 ± 0.07 | −0.07 ± 0.02 | −0.10 ± 0.03 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 0: A alone | 0.0 ± 0.0 | −7.7 ± 2.8 | +202.9 ± 3.3 | −1.5 ± 0.9 | +0.20 | 0.00 ± 0.00 | −0.08 ± 0.02 | +0.01 ± 0.01 | −0.02 ± 0.01 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 1: A and C | −0.1 ± 0.1 | −23.3 ± 2.9 | +146.7 ± 9.8 | −2.7 ± 1.3 | +0.14 | 0.00 ± 0.00 | −0.22 ± 0.02 | +0.04 ± 0.01 | −0.02 ± 0.01 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 2: B alone | −7.3 ± 2.6 | 0.0 ± 0.0 | −1.1 ± 0.8 | −126.9 ± 2.3 | −0.13 | −0.07 ± 0.03 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 3: B and D | −7.9 ± 5.9 | −0.1 ± 0.0 | −1.8 ± 2.1 | −218.1 ± 11.2 | −0.22 | −0.02 ± 0.04 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.03 ± 0.01 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 4: C's own | −47.4 ± 3.6 | −34.0 ± 5.2 | −1.5 ± 0.9 | −0.9 ± 0.6 | +0.09 | −0.54 ± 0.04 | −0.35 ± 0.04 | −0.02 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 5: D's own | −19.1 ± 4.6 | −18.1 ± 3.9 | −2.0 ± 1.3 | −1.6 ± 1.1 | −0.05 | −0.04 ± 0.02 | −0.11 ± 0.02 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 6: readout 0's pattern targets | −662.1 ± 21.6 | −547.9 ± 30.0 | −13.3 ± 4.5 | −22.5 ± 2.7 | −0.95 | −4.43 ± 0.17 | −4.06 ± 0.24 | −0.12 ± 0.04 | −0.17 ± 0.02 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 7: readout 0's other excitatory | −22.6 ± 7.6 | −63.4 ± 5.5 | −6.1 ± 2.1 | −5.5 ± 2.5 | −0.37 | −0.17 ± 0.03 | −0.35 ± 0.04 | −0.03 ± 0.01 | −0.04 ± 0.02 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 8: readout 1's pattern targets | −618.2 ± 19.9 | −598.3 ± 18.0 | −25.4 ± 2.5 | −16.4 ± 4.0 | −1.94 | −3.90 ± 0.18 | −3.21 ± 0.13 | −0.21 ± 0.03 | −0.14 ± 0.03 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 9: readout 1's other excitatory | −372.5 ± 11.5 | −71.1 ± 5.5 | −4.8 ± 2.0 | −4.4 ± 2.8 | −0.69 | −1.90 ± 0.08 | −0.46 ± 0.04 | −0.05 ± 0.02 | −0.04 ± 0.02 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 10: other excitatory | −4.6 ± 1.7 | −8.8 ± 1.0 | −0.5 ± 0.3 | −0.6 ± 0.3 | −0.01 | −0.06 ± 0.03 | −0.14 ± 0.02 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 11: inhibitory in readout 0 | −168.4 ± 20.7 | −195.8 ± 16.6 | −13.7 ± 3.4 | −19.5 ± 3.2 | +2.59 | −2.38 ± 0.08 | −2.69 ± 0.10 | −0.10 ± 0.02 | −0.12 ± 0.02 |
| After block 24 alone less without replay, block 24 | A's answer's readout | 12: inhibitory in readout 1 | −56.7 ± 19.7 | −35.1 ± 22.2 | −6.5 ± 3.9 | −8.3 ± 3.3 | +2.41 | −3.00 ± 0.07 | −2.01 ± 0.11 | −0.06 ± 0.03 | −0.07 ± 0.02 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 0: A alone | −0.1 ± 0.1 | −12.1 ± 3.5 | −202.5 ± 3.6 | −1.3 ± 1.0 | −0.20 | 0.00 ± 0.00 | −0.08 ± 0.02 | +0.01 ± 0.01 | −0.02 ± 0.01 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 1: A and C | −0.2 ± 0.2 | −33.9 ± 3.1 | −135.8 ± 9.0 | −3.6 ± 1.2 | −0.14 | 0.00 ± 0.00 | −0.22 ± 0.02 | +0.04 ± 0.01 | −0.02 ± 0.01 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 2: B alone | −10.3 ± 4.3 | 0.0 ± 0.0 | −0.4 ± 1.1 | +128.9 ± 2.4 | +0.13 | −0.07 ± 0.03 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 3: B and D | +26.8 ± 6.4 | −0.2 ± 0.1 | −1.6 ± 1.8 | +226.0 ± 10.3 | +0.22 | −0.02 ± 0.04 | 0.00 ± 0.00 | −0.01 ± 0.01 | +0.03 ± 0.01 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 4: C's own | −59.4 ± 5.1 | −48.9 ± 5.5 | −1.7 ± 0.9 | −1.1 ± 0.8 | −0.09 | −0.54 ± 0.04 | −0.35 ± 0.04 | −0.02 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 5: D's own | −14.0 ± 2.6 | −6.3 ± 2.9 | −1.6 ± 0.9 | −1.6 ± 1.1 | +0.05 | −0.04 ± 0.02 | −0.11 ± 0.02 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 6: readout 0's pattern targets | −724.6 ± 22.5 | −633.3 ± 24.5 | −14.4 ± 5.6 | −26.1 ± 3.4 | −1.98 | −4.43 ± 0.17 | −4.06 ± 0.24 | −0.12 ± 0.04 | −0.17 ± 0.02 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 7: readout 0's other excitatory | −18.1 ± 4.9 | −58.3 ± 4.3 | −3.5 ± 1.5 | −4.8 ± 1.9 | −0.29 | −0.17 ± 0.03 | −0.35 ± 0.04 | −0.03 ± 0.01 | −0.04 ± 0.02 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 8: readout 1's pattern targets | −459.9 ± 26.6 | −524.9 ± 18.4 | −25.9 ± 3.8 | −15.1 ± 3.5 | −0.49 | −3.90 ± 0.18 | −3.21 ± 0.13 | −0.21 ± 0.03 | −0.14 ± 0.03 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 9: readout 1's other excitatory | −236.3 ± 10.5 | −87.4 ± 5.7 | −5.5 ± 1.8 | −4.7 ± 1.9 | −0.35 | −1.90 ± 0.08 | −0.46 ± 0.04 | −0.05 ± 0.02 | −0.04 ± 0.02 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 10: other excitatory | −8.7 ± 2.9 | −13.3 ± 1.4 | −0.5 ± 0.4 | −1.0 ± 0.5 | −0.03 | −0.06 ± 0.03 | −0.14 ± 0.02 | −0.01 ± 0.01 | −0.01 ± 0.01 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 11: inhibitory in readout 0 | −307.0 ± 19.5 | −182.9 ± 14.7 | −13.5 ± 3.6 | −19.7 ± 3.9 | +1.91 | −2.38 ± 0.08 | −2.69 ± 0.10 | −0.10 ± 0.02 | −0.12 ± 0.02 |
| After block 24 alone less without replay, block 24 | B's answer's readout | 12: inhibitory in readout 1 | −232.5 ± 7.4 | +5.8 ± 17.9 | −7.5 ± 4.3 | −8.7 ± 3.0 | +2.00 | −3.00 ± 0.07 | −2.01 ± 0.11 | −0.06 ± 0.03 | −0.07 ± 0.02 |

Each readout's two inhibitory classes' input rose by 1.90 to 2.13 M on the image from the assignment and 1.91 to 2.66 on the image from the
mirrored assignment while their units fired less, and their window drive moved by −510.4 to +83.1 k and −346.3 to +34.3 k. The two
pattern-target classes' input fell by 0.62 to 2.19 M and 0.31 to 2.22, and their window drive by 344.2 to 863.9 k and 343.0 to 754.8 k.

**The split**, by the weights and by the spikes per weight, on A's trials:

| Comparison | Onto | From | Span | From the assignment: the drive's difference, k | by the weights | by the spikes per weight | From the mirrored assignment: the drive's difference | by the weights | by the spikes per weight |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's answer's readout | A's kinds | the window | +0.1 | 0.0 | +0.1 | 0.0 | 0.0 | 0.0 |
| After block 6 alone less without replay, block 6 | A's answer's readout | A's kinds | the volley | +350.4 | +343.7 | +6.7 | +342.7 | +333.8 | +8.9 |
| After block 6 alone less without replay, block 6 | A's answer's readout | B's kinds | the window | −56.7 | −5.9 | −50.9 | −25.0 | −5.7 | −19.3 |
| After block 6 alone less without replay, block 6 | A's answer's readout | B's kinds | the volley | −4.9 | −0.4 | −4.5 | −4.3 | −0.4 | −3.9 |
| After block 6 alone less without replay, block 6 | A's answer's readout | The odd places' excitatory | the window | −990.5 | −157.3 | −833.2 | −822.1 | −63.2 | −758.9 |
| After block 6 alone less without replay, block 6 | A's answer's readout | The odd places' excitatory | the volley | −17.4 | −3.4 | −13.9 | −30.8 | −1.7 | −29.1 |
| After block 6 alone less without replay, block 6 | A's answer's readout | The even places' excitatory | the window | −687.2 | −73.1 | −614.0 | −888.2 | −176.6 | −711.6 |
| After block 6 alone less without replay, block 6 | A's answer's readout | The even places' excitatory | the volley | −24.1 | −1.9 | −22.2 | −30.1 | −4.1 | −26.0 |
| After block 6 alone less without replay, block 6 | A's answer's readout | Inhibitory | the window | −163.2 | +263.7 | −426.9 | −191.5 | +389.4 | −580.9 |
| After block 6 alone less without replay, block 6 | A's answer's readout | Inhibitory | the volley | −14.1 | +7.8 | −21.9 | −23.4 | +9.3 | −32.6 |
| After block 6 alone less without replay, block 6 | B's answer's readout | A's kinds | the window | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| After block 6 alone less without replay, block 6 | B's answer's readout | A's kinds | the volley | −335.5 | −343.5 | +7.9 | −327.7 | −333.9 | +6.3 |
| After block 6 alone less without replay, block 6 | B's answer's readout | B's kinds | the window | −34.2 | +4.5 | −38.7 | −32.9 | +8.5 | −41.5 |
| After block 6 alone less without replay, block 6 | B's answer's readout | B's kinds | the volley | −5.1 | +0.4 | −5.6 | −5.9 | +0.4 | −6.3 |
| After block 6 alone less without replay, block 6 | B's answer's readout | The odd places' excitatory | the window | −711.5 | −66.5 | −645.0 | −735.8 | −177.3 | −558.5 |
| After block 6 alone less without replay, block 6 | B's answer's readout | The odd places' excitatory | the volley | −20.8 | −1.8 | −19.0 | −29.9 | −3.7 | −26.2 |
| After block 6 alone less without replay, block 6 | B's answer's readout | The even places' excitatory | the window | −894.4 | −166.4 | −728.0 | −532.0 | −34.4 | −497.6 |
| After block 6 alone less without replay, block 6 | B's answer's readout | The even places' excitatory | the volley | −26.4 | −3.8 | −22.6 | −24.5 | −0.7 | −23.8 |
| After block 6 alone less without replay, block 6 | B's answer's readout | Inhibitory | the window | −482.5 | +279.7 | −762.2 | −453.3 | +321.0 | −774.3 |
| After block 6 alone less without replay, block 6 | B's answer's readout | Inhibitory | the volley | −17.7 | +7.3 | −25.0 | −23.7 | +7.1 | −30.9 |
| After block 12 alone less without replay, block 12 | A's answer's readout | A's kinds | the window | −0.5 | 0.0 | −0.6 | −0.5 | 0.0 | −0.5 |
| After block 12 alone less without replay, block 12 | A's answer's readout | A's kinds | the volley | +317.2 | +311.0 | +6.2 | +412.6 | +405.1 | +7.5 |
| After block 12 alone less without replay, block 12 | A's answer's readout | B's kinds | the window | −90.8 | −7.2 | −83.5 | −19.4 | −7.1 | −12.3 |
| After block 12 alone less without replay, block 12 | A's answer's readout | B's kinds | the volley | −5.0 | −0.3 | −4.7 | −6.2 | −0.4 | −5.8 |
| After block 12 alone less without replay, block 12 | A's answer's readout | The odd places' excitatory | the window | −1 048.0 | −188.9 | −859.1 | −765.0 | −55.8 | −709.2 |
| After block 12 alone less without replay, block 12 | A's answer's readout | The odd places' excitatory | the volley | −30.5 | −3.6 | −26.9 | −33.5 | −1.5 | −32.0 |
| After block 12 alone less without replay, block 12 | A's answer's readout | The even places' excitatory | the window | −796.8 | −82.8 | −714.1 | −1 012.0 | −192.7 | −819.3 |
| After block 12 alone less without replay, block 12 | A's answer's readout | The even places' excitatory | the volley | −21.3 | −1.9 | −19.4 | −23.1 | −3.3 | −19.8 |
| After block 12 alone less without replay, block 12 | A's answer's readout | Inhibitory | the window | −324.1 | +326.8 | −650.9 | −241.0 | +465.2 | −706.2 |
| After block 12 alone less without replay, block 12 | A's answer's readout | Inhibitory | the volley | −18.0 | +7.5 | −25.5 | −26.9 | +9.3 | −36.2 |
| After block 12 alone less without replay, block 12 | B's answer's readout | A's kinds | the window | −0.3 | 0.0 | −0.3 | −0.4 | 0.0 | −0.4 |
| After block 12 alone less without replay, block 12 | B's answer's readout | A's kinds | the volley | −307.1 | −311.0 | +3.9 | −399.4 | −405.1 | +5.7 |
| After block 12 alone less without replay, block 12 | B's answer's readout | B's kinds | the window | −54.7 | +5.8 | −60.5 | −34.6 | +12.7 | −47.4 |
| After block 12 alone less without replay, block 12 | B's answer's readout | B's kinds | the volley | −5.4 | +0.4 | −5.8 | −5.8 | +0.4 | −6.2 |
| After block 12 alone less without replay, block 12 | B's answer's readout | The odd places' excitatory | the window | −615.0 | −67.2 | −547.8 | −818.7 | −186.0 | −632.7 |
| After block 12 alone less without replay, block 12 | B's answer's readout | The odd places' excitatory | the volley | −29.0 | −1.7 | −27.3 | −35.4 | −3.7 | −31.7 |
| After block 12 alone less without replay, block 12 | B's answer's readout | The even places' excitatory | the window | −960.2 | −156.9 | −803.3 | −685.3 | −52.2 | −633.1 |
| After block 12 alone less without replay, block 12 | B's answer's readout | The even places' excitatory | the volley | −19.4 | −3.1 | −16.4 | −22.1 | −0.9 | −21.2 |
| After block 12 alone less without replay, block 12 | B's answer's readout | Inhibitory | the window | −594.3 | +335.3 | −929.6 | −598.6 | +371.1 | −969.7 |
| After block 12 alone less without replay, block 12 | B's answer's readout | Inhibitory | the volley | −24.5 | +7.6 | −32.2 | −19.6 | +6.8 | −26.4 |
| After block 18 alone less without replay, block 18 | A's answer's readout | A's kinds | the window | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| After block 18 alone less without replay, block 18 | A's answer's readout | A's kinds | the volley | +303.5 | +297.1 | +6.4 | +401.0 | +396.2 | +4.8 |
| After block 18 alone less without replay, block 18 | A's answer's readout | B's kinds | the window | −73.0 | −8.7 | −64.3 | −25.1 | −6.6 | −18.5 |
| After block 18 alone less without replay, block 18 | A's answer's readout | B's kinds | the volley | −3.4 | −0.4 | −3.0 | −1.7 | −0.2 | −1.4 |
| After block 18 alone less without replay, block 18 | A's answer's readout | The odd places' excitatory | the window | −1 000.3 | −194.3 | −806.0 | −687.3 | −70.7 | −616.6 |
| After block 18 alone less without replay, block 18 | A's answer's readout | The odd places' excitatory | the volley | −25.8 | −3.2 | −22.5 | −27.8 | −1.5 | −26.3 |
| After block 18 alone less without replay, block 18 | A's answer's readout | The even places' excitatory | the window | −886.0 | −97.9 | −788.1 | −987.5 | −206.2 | −781.3 |
| After block 18 alone less without replay, block 18 | A's answer's readout | The even places' excitatory | the volley | −15.4 | −1.9 | −13.6 | −22.1 | −3.0 | −19.1 |
| After block 18 alone less without replay, block 18 | A's answer's readout | Inhibitory | the window | −511.7 | +377.9 | −889.6 | −206.1 | +493.6 | −699.6 |
| After block 18 alone less without replay, block 18 | A's answer's readout | Inhibitory | the volley | −22.3 | +7.9 | −30.2 | −16.8 | +8.6 | −25.4 |
| After block 18 alone less without replay, block 18 | B's answer's readout | A's kinds | the window | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| After block 18 alone less without replay, block 18 | B's answer's readout | A's kinds | the volley | −293.5 | −297.2 | +3.7 | −391.1 | −396.2 | +5.1 |
| After block 18 alone less without replay, block 18 | B's answer's readout | B's kinds | the window | −42.0 | +6.8 | −48.8 | −25.8 | +12.1 | −37.8 |
| After block 18 alone less without replay, block 18 | B's answer's readout | B's kinds | the volley | −1.7 | +0.3 | −1.9 | −1.5 | +0.3 | −1.8 |
| After block 18 alone less without replay, block 18 | B's answer's readout | The odd places' excitatory | the window | −597.2 | −78.4 | −518.8 | −751.4 | −181.4 | −570.0 |
| After block 18 alone less without replay, block 18 | B's answer's readout | The odd places' excitatory | the volley | −25.9 | −1.8 | −24.2 | −28.0 | −3.2 | −24.7 |
| After block 18 alone less without replay, block 18 | B's answer's readout | The even places' excitatory | the window | −983.7 | −166.9 | −816.8 | −722.9 | −56.0 | −666.9 |
| After block 18 alone less without replay, block 18 | B's answer's readout | The even places' excitatory | the volley | −17.6 | −2.8 | −14.8 | −19.0 | −0.7 | −18.3 |
| After block 18 alone less without replay, block 18 | B's answer's readout | Inhibitory | the window | −776.0 | +355.7 | −1 131.7 | −612.0 | +419.7 | −1 031.7 |
| After block 18 alone less without replay, block 18 | B's answer's readout | Inhibitory | the volley | −15.9 | +6.9 | −22.8 | −21.3 | +6.8 | −28.0 |
| After block 24 alone less without replay, block 24 | A's answer's readout | A's kinds | the window | +0.2 | 0.0 | +0.2 | −0.1 | 0.0 | −0.2 |
| After block 24 alone less without replay, block 24 | A's answer's readout | A's kinds | the volley | +254.0 | +244.2 | +9.8 | +349.6 | +343.5 | +6.1 |
| After block 24 alone less without replay, block 24 | A's answer's readout | B's kinds | the window | −82.9 | −9.7 | −73.2 | −15.2 | −8.7 | −6.5 |
| After block 24 alone less without replay, block 24 | A's answer's readout | B's kinds | the volley | −4.8 | −0.4 | −4.4 | −2.9 | −0.4 | −2.5 |
| After block 24 alone less without replay, block 24 | A's answer's readout | The odd places' excitatory | the window | −991.2 | −206.9 | −784.3 | −732.1 | −73.8 | −658.3 |
| After block 24 alone less without replay, block 24 | A's answer's readout | The odd places' excitatory | the volley | −22.4 | −2.8 | −19.7 | −20.8 | −1.5 | −19.3 |
| After block 24 alone less without replay, block 24 | A's answer's readout | The even places' excitatory | the window | −877.2 | −97.8 | −779.4 | −1 009.8 | −216.0 | −793.8 |
| After block 24 alone less without replay, block 24 | A's answer's readout | The even places' excitatory | the volley | −22.2 | −2.0 | −20.2 | −32.2 | −3.5 | −28.7 |
| After block 24 alone less without replay, block 24 | A's answer's readout | Inhibitory | the window | −427.8 | +419.1 | −846.8 | −225.1 | +529.8 | −754.9 |
| After block 24 alone less without replay, block 24 | A's answer's readout | Inhibitory | the volley | −26.7 | +7.8 | −34.5 | −20.2 | +9.7 | −29.9 |
| After block 24 alone less without replay, block 24 | B's answer's readout | A's kinds | the window | +0.2 | 0.0 | +0.2 | −0.2 | 0.0 | −0.2 |
| After block 24 alone less without replay, block 24 | B's answer's readout | A's kinds | the volley | −236.9 | −244.4 | +7.5 | −338.3 | −343.5 | +5.3 |
| After block 24 alone less without replay, block 24 | B's answer's readout | B's kinds | the window | −50.7 | +7.4 | −58.1 | +16.5 | +16.2 | +0.2 |
| After block 24 alone less without replay, block 24 | B's answer's readout | B's kinds | the volley | −3.9 | +0.4 | −4.3 | −2.0 | +0.4 | −2.4 |
| After block 24 alone less without replay, block 24 | B's answer's readout | The odd places' excitatory | the window | −439.5 | −65.5 | −374.0 | −802.1 | −183.1 | −619.0 |
| After block 24 alone less without replay, block 24 | B's answer's readout | The odd places' excitatory | the volley | −20.1 | −1.3 | −18.8 | −19.6 | −2.5 | −17.2 |
| After block 24 alone less without replay, block 24 | B's answer's readout | The even places' excitatory | the window | −914.3 | −157.2 | −757.1 | −710.2 | −68.5 | −641.7 |
| After block 24 alone less without replay, block 24 | B's answer's readout | The even places' excitatory | the volley | −20.2 | −2.8 | −17.4 | −32.9 | −1.0 | −31.9 |
| After block 24 alone less without replay, block 24 | B's answer's readout | Inhibitory | the window | −691.2 | +397.9 | −1 089.1 | −539.5 | +447.1 | −986.6 |
| After block 24 alone less without replay, block 24 | B's answer's readout | Inhibitory | the volley | −31.2 | +7.7 | −38.9 | −21.0 | +7.8 | −28.8 |

On B's trials:

| Comparison | Onto | From | Span | From the assignment: the drive's difference, k | by the weights | by the spikes per weight | From the mirrored assignment: the drive's difference | by the weights | by the spikes per weight |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's answer's readout | A's kinds | the window | −18.3 | +2.7 | −21.0 | −24.9 | +3.0 | −27.9 |
| After block 6 alone less without replay, block 6 | A's answer's readout | A's kinds | the volley | −2.8 | +0.4 | −3.2 | −2.9 | +0.3 | −3.2 |
| After block 6 alone less without replay, block 6 | A's answer's readout | B's kinds | the window | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| After block 6 alone less without replay, block 6 | A's answer's readout | B's kinds | the volley | −307.9 | −311.9 | +4.0 | −261.9 | −268.3 | +6.4 |
| After block 6 alone less without replay, block 6 | A's answer's readout | The odd places' excitatory | the window | −464.0 | −104.7 | −359.3 | −464.2 | −45.3 | −418.9 |
| After block 6 alone less without replay, block 6 | A's answer's readout | The odd places' excitatory | the volley | −24.4 | −3.6 | −20.8 | −32.4 | −1.6 | −30.7 |
| After block 6 alone less without replay, block 6 | A's answer's readout | The even places' excitatory | the window | −578.9 | −83.0 | −495.9 | −622.3 | −147.7 | −474.6 |
| After block 6 alone less without replay, block 6 | A's answer's readout | The even places' excitatory | the volley | −23.0 | −1.7 | −21.3 | −18.8 | −3.3 | −15.5 |
| After block 6 alone less without replay, block 6 | A's answer's readout | Inhibitory | the window | −43.5 | +336.4 | −379.9 | −139.0 | +468.4 | −607.4 |
| After block 6 alone less without replay, block 6 | A's answer's readout | Inhibitory | the volley | −8.4 | +7.3 | −15.7 | −14.2 | +8.5 | −22.8 |
| After block 6 alone less without replay, block 6 | B's answer's readout | A's kinds | the window | −22.5 | −3.0 | −19.6 | −31.3 | −2.9 | −28.4 |
| After block 6 alone less without replay, block 6 | B's answer's readout | A's kinds | the volley | −2.5 | −0.3 | −2.2 | −3.6 | −0.3 | −3.3 |
| After block 6 alone less without replay, block 6 | B's answer's readout | B's kinds | the window | −0.1 | 0.0 | −0.1 | 0.0 | 0.0 | 0.0 |
| After block 6 alone less without replay, block 6 | B's answer's readout | B's kinds | the volley | +314.7 | +312.0 | +2.7 | +275.2 | +268.3 | +6.9 |
| After block 6 alone less without replay, block 6 | B's answer's readout | The odd places' excitatory | the window | −537.8 | −53.5 | −484.3 | −632.5 | −119.7 | −512.8 |
| After block 6 alone less without replay, block 6 | B's answer's readout | The odd places' excitatory | the volley | −22.5 | −1.9 | −20.6 | −29.3 | −3.6 | −25.7 |
| After block 6 alone less without replay, block 6 | B's answer's readout | The even places' excitatory | the window | −754.2 | −142.1 | −612.1 | −495.7 | −19.0 | −476.7 |
| After block 6 alone less without replay, block 6 | B's answer's readout | The even places' excitatory | the volley | −24.6 | −3.3 | −21.3 | −14.5 | −0.6 | −13.9 |
| After block 6 alone less without replay, block 6 | B's answer's readout | Inhibitory | the window | −72.0 | +352.6 | −424.7 | +9.9 | +399.3 | −389.5 |
| After block 6 alone less without replay, block 6 | B's answer's readout | Inhibitory | the volley | −8.3 | +6.4 | −14.7 | −17.2 | +7.4 | −24.6 |
| After block 12 alone less without replay, block 12 | A's answer's readout | A's kinds | the window | −12.3 | +2.2 | −14.5 | −24.7 | +3.8 | −28.6 |
| After block 12 alone less without replay, block 12 | A's answer's readout | A's kinds | the volley | −3.8 | +0.3 | −4.1 | −2.8 | +0.4 | −3.2 |
| After block 12 alone less without replay, block 12 | A's answer's readout | B's kinds | the window | −1.0 | 0.0 | −0.9 | −0.3 | 0.0 | −0.3 |
| After block 12 alone less without replay, block 12 | A's answer's readout | B's kinds | the volley | −264.8 | −272.0 | +7.1 | −253.9 | −261.4 | +7.6 |
| After block 12 alone less without replay, block 12 | A's answer's readout | The odd places' excitatory | the window | −521.4 | −118.8 | −402.7 | −524.9 | −43.3 | −481.7 |
| After block 12 alone less without replay, block 12 | A's answer's readout | The odd places' excitatory | the volley | −25.6 | −3.5 | −22.1 | −20.1 | −1.4 | −18.7 |
| After block 12 alone less without replay, block 12 | A's answer's readout | The even places' excitatory | the window | −606.3 | −99.1 | −507.2 | −727.4 | −154.6 | −572.8 |
| After block 12 alone less without replay, block 12 | A's answer's readout | The even places' excitatory | the volley | −17.4 | −1.9 | −15.6 | −19.3 | −3.0 | −16.3 |
| After block 12 alone less without replay, block 12 | A's answer's readout | Inhibitory | the window | −106.6 | +384.3 | −490.9 | −201.1 | +517.8 | −718.9 |
| After block 12 alone less without replay, block 12 | A's answer's readout | Inhibitory | the volley | −11.6 | +7.1 | −18.8 | −16.8 | +8.9 | −25.7 |
| After block 12 alone less without replay, block 12 | B's answer's readout | A's kinds | the window | −19.4 | −2.6 | −16.8 | −30.8 | −3.7 | −27.1 |
| After block 12 alone less without replay, block 12 | B's answer's readout | A's kinds | the volley | −2.4 | −0.3 | −2.1 | −3.9 | −0.4 | −3.5 |
| After block 12 alone less without replay, block 12 | B's answer's readout | B's kinds | the window | −0.2 | 0.0 | −0.2 | −0.3 | 0.0 | −0.3 |
| After block 12 alone less without replay, block 12 | B's answer's readout | B's kinds | the volley | +279.6 | +272.0 | +7.6 | +271.3 | +261.4 | +9.9 |
| After block 12 alone less without replay, block 12 | B's answer's readout | The odd places' excitatory | the window | −577.1 | −58.0 | −519.1 | −699.5 | −131.5 | −568.0 |
| After block 12 alone less without replay, block 12 | B's answer's readout | The odd places' excitatory | the volley | −24.5 | −1.6 | −22.9 | −22.2 | −3.4 | −18.8 |
| After block 12 alone less without replay, block 12 | B's answer's readout | The even places' excitatory | the window | −794.6 | −151.7 | −642.9 | −538.9 | −38.4 | −500.5 |
| After block 12 alone less without replay, block 12 | B's answer's readout | The even places' excitatory | the volley | −14.2 | −2.7 | −11.5 | −20.8 | −0.9 | −19.9 |
| After block 12 alone less without replay, block 12 | B's answer's readout | Inhibitory | the window | −183.8 | +414.7 | −598.5 | −89.9 | +421.1 | −510.9 |
| After block 12 alone less without replay, block 12 | B's answer's readout | Inhibitory | the volley | −14.9 | +6.9 | −21.8 | −16.2 | +6.8 | −23.0 |
| After block 18 alone less without replay, block 18 | A's answer's readout | A's kinds | the window | −19.8 | +2.5 | −22.3 | −30.3 | +4.0 | −34.3 |
| After block 18 alone less without replay, block 18 | A's answer's readout | A's kinds | the volley | −3.8 | +0.4 | −4.3 | −3.4 | +0.6 | −4.0 |
| After block 18 alone less without replay, block 18 | A's answer's readout | B's kinds | the window | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| After block 18 alone less without replay, block 18 | A's answer's readout | B's kinds | the volley | −257.4 | −264.3 | +6.9 | −256.3 | −264.1 | +7.8 |
| After block 18 alone less without replay, block 18 | A's answer's readout | The odd places' excitatory | the window | −578.2 | −122.4 | −455.7 | −579.1 | −51.8 | −527.3 |
| After block 18 alone less without replay, block 18 | A's answer's readout | The odd places' excitatory | the volley | −23.7 | −3.1 | −20.6 | −22.7 | −1.4 | −21.3 |
| After block 18 alone less without replay, block 18 | A's answer's readout | The even places' excitatory | the window | −561.2 | −113.4 | −447.8 | −628.3 | −163.2 | −465.0 |
| After block 18 alone less without replay, block 18 | A's answer's readout | The even places' excitatory | the volley | −22.8 | −2.2 | −20.7 | −21.9 | −3.2 | −18.7 |
| After block 18 alone less without replay, block 18 | A's answer's readout | Inhibitory | the window | −128.4 | +404.6 | −533.1 | −165.2 | +548.6 | −713.8 |
| After block 18 alone less without replay, block 18 | A's answer's readout | Inhibitory | the volley | −25.6 | +9.2 | −34.8 | −27.6 | +11.4 | −39.0 |
| After block 18 alone less without replay, block 18 | B's answer's readout | A's kinds | the window | −27.3 | −2.7 | −24.6 | −38.2 | −4.0 | −34.2 |
| After block 18 alone less without replay, block 18 | B's answer's readout | A's kinds | the volley | −2.6 | −0.4 | −2.2 | −4.6 | −0.5 | −4.1 |
| After block 18 alone less without replay, block 18 | B's answer's readout | B's kinds | the window | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| After block 18 alone less without replay, block 18 | B's answer's readout | B's kinds | the volley | +270.9 | +264.2 | +6.7 | +272.1 | +264.1 | +8.0 |
| After block 18 alone less without replay, block 18 | B's answer's readout | The odd places' excitatory | the window | −663.9 | −70.4 | −593.6 | −697.3 | −136.0 | −561.2 |
| After block 18 alone less without replay, block 18 | B's answer's readout | The odd places' excitatory | the volley | −16.9 | −1.5 | −15.4 | −28.0 | −3.1 | −24.9 |
| After block 18 alone less without replay, block 18 | B's answer's readout | The even places' excitatory | the window | −803.0 | −174.7 | −628.3 | −501.0 | −36.3 | −464.7 |
| After block 18 alone less without replay, block 18 | B's answer's readout | The even places' excitatory | the volley | −21.2 | −3.1 | −18.1 | −21.9 | −0.9 | −21.0 |
| After block 18 alone less without replay, block 18 | B's answer's readout | Inhibitory | the window | −203.5 | +411.6 | −615.1 | −69.7 | +461.4 | −531.1 |
| After block 18 alone less without replay, block 18 | B's answer's readout | Inhibitory | the volley | −27.1 | +8.7 | −35.9 | −24.5 | +8.5 | −32.9 |
| After block 24 alone less without replay, block 24 | A's answer's readout | A's kinds | the window | −19.8 | +2.1 | −21.9 | −31.0 | +3.3 | −34.3 |
| After block 24 alone less without replay, block 24 | A's answer's readout | A's kinds | the volley | −2.3 | +0.3 | −2.5 | −4.2 | +0.4 | −4.6 |
| After block 24 alone less without replay, block 24 | A's answer's readout | B's kinds | the window | −0.3 | 0.0 | −0.3 | −0.1 | 0.0 | −0.1 |
| After block 24 alone less without replay, block 24 | A's answer's readout | B's kinds | the volley | −313.6 | −318.7 | +5.1 | −344.9 | −349.2 | +4.3 |
| After block 24 alone less without replay, block 24 | A's answer's readout | The odd places' excitatory | the window | −626.2 | −128.4 | −497.8 | −645.3 | −66.3 | −579.0 |
| After block 24 alone less without replay, block 24 | A's answer's readout | The odd places' excitatory | the volley | −26.4 | −3.6 | −22.8 | −28.9 | −1.6 | −27.3 |
| After block 24 alone less without replay, block 24 | A's answer's readout | The even places' excitatory | the window | −727.4 | −116.7 | −610.7 | −687.5 | −173.3 | −514.2 |
| After block 24 alone less without replay, block 24 | A's answer's readout | The even places' excitatory | the volley | −19.5 | −1.9 | −17.6 | −22.4 | −3.3 | −19.1 |
| After block 24 alone less without replay, block 24 | A's answer's readout | Inhibitory | the window | −142.2 | +433.7 | −575.9 | −230.9 | +574.1 | −805.0 |
| After block 24 alone less without replay, block 24 | A's answer's readout | Inhibitory | the volley | −23.2 | +8.3 | −31.5 | −27.8 | +10.0 | −37.9 |
| After block 24 alone less without replay, block 24 | B's answer's readout | A's kinds | the window | −27.0 | −2.2 | −24.8 | −46.1 | −3.6 | −42.4 |
| After block 24 alone less without replay, block 24 | B's answer's readout | A's kinds | the volley | −3.6 | −0.3 | −3.3 | −4.8 | −0.4 | −4.5 |
| After block 24 alone less without replay, block 24 | B's answer's readout | B's kinds | the window | 0.0 | 0.0 | 0.0 | −0.2 | 0.0 | −0.2 |
| After block 24 alone less without replay, block 24 | B's answer's readout | B's kinds | the volley | +324.9 | +318.7 | +6.2 | +354.9 | +349.1 | +5.7 |
| After block 24 alone less without replay, block 24 | B's answer's readout | The odd places' excitatory | the window | −741.6 | −65.6 | −676.0 | −740.5 | −139.4 | −601.1 |
| After block 24 alone less without replay, block 24 | B's answer's readout | The odd places' excitatory | the volley | −27.2 | −1.7 | −25.5 | −32.0 | −3.2 | −28.8 |
| After block 24 alone less without replay, block 24 | B's answer's readout | The even places' excitatory | the window | −880.9 | −179.1 | −701.9 | −618.6 | −45.9 | −572.7 |
| After block 24 alone less without replay, block 24 | B's answer's readout | The even places' excitatory | the volley | −22.8 | −2.8 | −20.0 | −21.5 | −0.9 | −20.6 |
| After block 24 alone less without replay, block 24 | B's answer's readout | Inhibitory | the window | −238.4 | +441.1 | −679.5 | −177.2 | +484.6 | −661.8 |
| After block 24 alone less without replay, block 24 | B's answer's readout | Inhibitory | the volley | −25.8 | +8.4 | −34.2 | −28.3 | +7.6 | −36.0 |

Of each place group's window fall, 77 to 91 per cent on the image from the assignment and
74 to 96 on the image from the mirrored assignment lies in the spikes per weight. The inhibitory window drive's part by the weights is
more inhibition, 263.7 to 441.1 k on the image from the assignment and 321.0 to 574.1 k on the image from the mirrored assignment, and its
part by the spikes per weight less, 379.9 to 1 131.7 k and 389.5 to 1 031.7 k. Of the trial's stimulus's kinds' volley
change, 96 to 103 per cent on both cells is by the weights.

**The units that carried half of each readout's change**, exploration 033's fewest units of each readout's spike difference, against the
rest of the readout's 459, each a unit's drive difference a trial, both spans. On the image from the assignment:

| Comparison | Trials | The readout | Units carrying half its change | Their classes | Their drive's difference a unit from the trial's stimulus's kinds, k | the rest's | From the readout's own place group | the rest's | From the other place group | the rest's | Inhibitory | the rest's | Excitatory less inhibitory | the rest's | Their input's difference a unit from the trial's stimulus's kinds | the rest's |
| :--- | :--- | :--- | ---: | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | A's answer's readout | 16 | 1 D's own, 7 pattern targets, 1 other excitatory, 7 inhibitory | −7.88 | +1.08 | −5.21 | −1.41 | −10.94 | −1.87 | −5.59 | −0.20 | −19.82 | −2.13 | −7 967 | +1 065 |
| After block 6 alone less without replay, block 6 | A's trials | B's answer's readout | 15 | 12 pattern targets, 2 other excitatory, 1 inhibitory | −9.96 | −0.42 | −6.30 | −1.43 | −16.66 | −1.51 | −9.96 | −0.79 | −23.11 | −2.68 | −10 144 | −433 |
| After block 6 alone less without replay, block 6 | B's trials | A's answer's readout | 15 | 2 D's own, 6 pattern targets, 4 other excitatory, 3 inhibitory | −11.15 | −0.32 | −2.37 | −1.27 | −2.42 | −1.02 | −4.34 | +0.03 | −11.61 | −2.69 | −11 194 | −326 |
| After block 6 alone less without replay, block 6 | B's trials | B's answer's readout | 18 | 8 pattern targets, 1 other excitatory, 9 inhibitory | −8.24 | +1.05 | −2.67 | −1.16 | −8.32 | −1.42 | −5.13 | +0.02 | −14.16 | −1.62 | −8 272 | +1 047 |
| After block 12 alone less without replay, block 12 | A's trials | A's answer's readout | 18 | 8 pattern targets, 1 other excitatory, 9 inhibitory | −7.19 | +1.01 | −7.85 | −1.53 | −15.39 | −1.81 | −10.36 | −0.35 | −22.42 | −2.14 | −7 226 | +1 002 |
| After block 12 alone less without replay, block 12 | A's trials | B's answer's readout | 15 | 10 pattern targets, 2 other excitatory, 3 inhibitory | −8.66 | −0.40 | −4.61 | −1.30 | −21.57 | −1.48 | −11.88 | −0.99 | −23.07 | −2.34 | −8 739 | −407 |
| After block 12 alone less without replay, block 12 | B's trials | A's answer's readout | 19 | 2 D's own, 9 pattern targets, 3 other excitatory, 5 inhibitory | −7.64 | −0.27 | −5.71 | −1.17 | −2.82 | −1.12 | −4.76 | −0.06 | −11.48 | −2.55 | −7 863 | −280 |
| After block 12 alone less without replay, block 12 | B's trials | B's answer's readout | 17 | 7 pattern targets, 2 other excitatory, 8 inhibitory | −7.68 | +0.93 | −4.79 | −1.18 | −10.37 | −1.43 | −7.07 | −0.18 | −15.82 | −1.55 | −7 788 | +916 |
| After block 18 alone less without replay, block 18 | A's trials | A's answer's readout | 19 | 9 pattern targets, 1 other excitatory, 9 inhibitory | −7.49 | +1.01 | −8.90 | −1.67 | −11.56 | −1.83 | −12.07 | −0.69 | −17.33 | −1.94 | −7 584 | +1 005 |
| After block 18 alone less without replay, block 18 | A's trials | B's answer's readout | 17 | 11 pattern targets, 2 other excitatory, 4 inhibitory | −10.28 | −0.27 | −3.21 | −1.28 | −15.36 | −1.68 | −12.16 | −1.33 | −16.83 | −2.02 | −10 330 | −277 |
| After block 18 alone less without replay, block 18 | B's trials | A's answer's readout | 15 | 1 D's own, 8 pattern targets, 2 other excitatory, 4 inhibitory | −6.93 | −0.35 | −8.13 | −1.04 | −6.48 | −1.14 | −4.77 | −0.19 | −16.96 | −2.39 | −7 057 | −358 |
| After block 18 alone less without replay, block 18 | B's trials | B's answer's readout | 18 | 9 pattern targets, 3 other excitatory, 6 inhibitory | −6.33 | +0.87 | −6.26 | −1.29 | −15.18 | −1.25 | −5.61 | −0.30 | −22.36 | −1.44 | −6 365 | +861 |
| After block 24 alone less without replay, block 24 | A's trials | A's answer's readout | 19 | 1 D's own, 10 pattern targets, 8 inhibitory | −8.22 | +0.93 | −4.80 | −1.84 | −14.52 | −1.68 | −7.26 | −0.73 | −22.12 | −2.00 | −8 344 | +917 |
| After block 24 alone less without replay, block 24 | A's trials | B's answer's readout | 17 | 11 pattern targets, 1 other excitatory, 5 inhibitory | −8.30 | −0.22 | −4.32 | −0.88 | −13.68 | −1.59 | −10.64 | −1.24 | −15.68 | −1.58 | −8 438 | −230 |
| After block 24 alone less without replay, block 24 | B's trials | A's answer's readout | 13 | 1 D's own, 6 pattern targets, 3 other excitatory, 3 inhibitory | −9.64 | −0.42 | −8.17 | −1.44 | −8.89 | −1.20 | −4.04 | −0.26 | −22.68 | −2.86 | −9 682 | −434 |
| After block 24 alone less without replay, block 24 | B's trials | B's answer's readout | 14 | 9 pattern targets, 2 other excitatory, 3 inhibitory | −8.33 | +0.99 | −6.85 | −1.52 | −18.96 | −1.43 | −8.36 | −0.34 | −26.13 | −1.68 | −8 422 | +983 |
| After block 6 alone less without replay, block 36 | A's trials | A's answer's readout | 7 | 4 pattern targets, 3 inhibitory | −4.38 | +0.72 | −11.44 | −0.78 | −24.67 | −1.08 | −4.92 | −0.46 | −35.59 | −0.77 | −4 378 | +730 |
| After block 6 alone less without replay, block 36 | A's trials | B's answer's readout | 6 | 4 pattern targets, 1 other excitatory, 1 inhibitory | −24.51 | −0.34 | −1.66 | −0.57 | −0.36 | −0.99 | −12.61 | −0.77 | −13.91 | −1.15 | −24 614 | −334 |
| After block 6 alone less without replay, block 36 | B's trials | A's answer's readout | 4 | 1 D's own, 3 pattern targets | −11.85 | −0.45 | −11.52 | −0.74 | −12.49 | −0.46 | −10.50 | −0.43 | −25.31 | −1.22 | −11 874 | −446 |
| After block 6 alone less without replay, block 36 | B's trials | B's answer's readout | 6 | 2 pattern targets, 1 other excitatory, 3 inhibitory | −7.58 | +0.65 | −4.77 | −0.90 | −22.18 | −0.95 | −4.37 | −0.22 | −30.20 | −0.97 | −7 603 | +653 |
| After block 12 alone less without replay, block 36 | A's trials | A's answer's readout | 7 | 3 pattern targets, 4 inhibitory | −6.61 | +0.94 | −14.71 | −0.54 | −19.17 | −1.13 | −6.06 | −0.24 | −34.43 | −0.54 | −6 632 | +941 |
| After block 12 alone less without replay, block 36 | A's trials | B's answer's readout | 5 | 3 pattern targets, 1 other excitatory, 1 inhibitory | −25.19 | −0.56 | +0.01 | −0.56 | −1.87 | −0.77 | −13.18 | −0.78 | −13.87 | −1.08 | −25 234 | −557 |
| After block 12 alone less without replay, block 36 | B's trials | A's answer's readout | 3 | 2 pattern targets, 1 other excitatory | −12.37 | −0.46 | −5.68 | −0.54 | −9.68 | −0.27 | −21.59 | −0.31 | −6.13 | −0.95 | −12 366 | −457 |
| After block 12 alone less without replay, block 36 | B's trials | B's answer's readout | 3 | 1 other excitatory, 2 inhibitory | +0.51 | +0.53 | −10.90 | −0.79 | −44.86 | −0.59 | −4.79 | −0.07 | −50.45 | −0.79 | +500 | +535 |
| After block 18 alone less without replay, block 36 | A's trials | A's answer's readout | 8 | 4 pattern targets, 4 inhibitory | −4.92 | +0.76 | −15.90 | −0.70 | −17.88 | −1.05 | −8.50 | −0.35 | −30.20 | −0.73 | −4 922 | +759 |
| After block 18 alone less without replay, block 36 | A's trials | B's answer's readout | 6 | 3 pattern targets, 1 other excitatory, 2 inhibitory | −18.97 | −0.42 | −2.12 | −0.59 | −5.99 | −0.97 | −10.96 | −1.03 | −16.12 | −0.97 | −18 991 | −418 |
| After block 18 alone less without replay, block 36 | B's trials | A's answer's readout | 4 | 3 pattern targets, 1 inhibitory | −12.23 | −0.42 | −3.86 | −0.52 | −7.93 | −0.50 | −3.58 | −0.30 | −20.44 | −1.13 | −12 262 | −415 |
| After block 18 alone less without replay, block 36 | B's trials | B's answer's readout | 6 | 2 pattern targets, 2 other excitatory, 2 inhibitory | −1.68 | +0.55 | −6.36 | −0.89 | −23.10 | −0.68 | −2.40 | −0.27 | −28.73 | −0.75 | −1 676 | +547 |
| After block 24 alone less without replay, block 36 | A's trials | A's answer's readout | 10 | 5 pattern targets, 5 inhibitory | −10.73 | +0.85 | −5.83 | −0.88 | −10.46 | −1.13 | −3.07 | −0.16 | −27.95 | −1.09 | −10 727 | +854 |
| After block 24 alone less without replay, block 36 | A's trials | B's answer's readout | 6 | 4 pattern targets, 1 other excitatory, 1 inhibitory | −17.96 | −0.37 | +0.36 | −0.39 | −7.70 | −0.95 | −9.16 | −0.71 | −16.12 | −1.08 | −17 938 | −372 |
| After block 24 alone less without replay, block 36 | B's trials | A's answer's readout | 6 | 1 D's own, 3 pattern targets, 1 other excitatory, 1 inhibitory | −14.55 | −0.59 | −6.78 | −0.92 | −10.38 | −0.64 | −3.41 | −0.24 | −28.29 | −1.92 | −14 560 | −595 |
| After block 24 alone less without replay, block 36 | B's trials | B's answer's readout | 5 | 2 pattern targets, 2 other excitatory, 1 inhibitory | −1.35 | +0.80 | −11.07 | −0.87 | −41.01 | −0.96 | −3.10 | −0.27 | −50.79 | −0.77 | −1 362 | +801 |

On the image from the mirrored assignment:

| Comparison | Trials | The readout | Units carrying half its change | Their classes | Their drive's difference a unit from the trial's stimulus's kinds, k | the rest's | From the readout's own place group | the rest's | From the other place group | the rest's | Inhibitory | the rest's | Excitatory less inhibitory | the rest's | Their input's difference a unit from the trial's stimulus's kinds | the rest's |
| :--- | :--- | :--- | ---: | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | A's answer's readout | 13 | 9 pattern targets, 4 inhibitory | −12.00 | +1.12 | −11.04 | −1.59 | −13.60 | −1.66 | −4.18 | −0.37 | −32.69 | −1.84 | −12 138 | +1 104 |
| After block 6 alone less without replay, block 6 | A's trials | B's answer's readout | 16 | 6 pattern targets, 3 other excitatory, 7 inhibitory | −11.59 | −0.32 | −3.36 | −1.13 | −7.67 | −1.45 | −8.21 | −0.79 | −15.61 | −2.19 | −11 611 | −336 |
| After block 6 alone less without replay, block 6 | B's trials | A's answer's readout | 18 | 1 C's own, 10 pattern targets, 7 inhibitory | −8.50 | −0.25 | −2.83 | −1.01 | −6.90 | −1.17 | −6.46 | −0.08 | −11.99 | −2.41 | −8 578 | −260 |
| After block 6 alone less without replay, block 6 | B's trials | B's answer's readout | 14 | 8 pattern targets, 6 inhibitory | −10.02 | +0.93 | −5.89 | −0.96 | −9.30 | −1.19 | −5.09 | +0.15 | −20.86 | −1.43 | −10 028 | +920 |
| After block 12 alone less without replay, block 12 | A's trials | A's answer's readout | 15 | 1 C's own, 11 pattern targets, 3 inhibitory | −10.77 | +1.29 | −11.07 | −1.42 | −12.73 | −1.90 | −6.73 | −0.38 | −27.93 | −1.74 | −10 895 | +1 283 |
| After block 12 alone less without replay, block 12 | A's trials | B's answer's readout | 19 | 8 pattern targets, 3 other excitatory, 8 inhibitory | −11.64 | −0.41 | −4.55 | −1.42 | −6.31 | −1.67 | −10.86 | −0.94 | −12.98 | −2.64 | −11 767 | −415 |
| After block 12 alone less without replay, block 12 | B's trials | A's answer's readout | 19 | 11 pattern targets, 8 inhibitory | −7.96 | −0.23 | −4.64 | −1.04 | −8.20 | −1.35 | −6.22 | −0.23 | −14.80 | −2.46 | −8 083 | −246 |
| After block 12 alone less without replay, block 12 | B's trials | B's answer's readout | 15 | 6 pattern targets, 1 other excitatory, 8 inhibitory | −6.77 | +0.84 | −5.34 | −1.08 | −7.53 | −1.37 | −2.43 | −0.16 | −17.73 | −1.53 | −6 826 | +821 |
| After block 18 alone less without replay, block 18 | A's trials | A's answer's readout | 17 | 1 C's own, 10 pattern targets, 6 inhibitory | −8.46 | +1.23 | −10.59 | −1.21 | −11.16 | −1.85 | −9.16 | −0.16 | −21.13 | −1.76 | −8 572 | +1 228 |
| After block 18 alone less without replay, block 18 | A's trials | B's answer's readout | 19 | 8 pattern targets, 2 other excitatory, 9 inhibitory | −11.06 | −0.41 | −5.62 | −1.44 | −8.00 | −1.42 | −10.49 | −0.99 | −14.46 | −2.39 | −11 162 | −421 |
| After block 18 alone less without replay, block 18 | B's trials | A's answer's readout | 20 | 1 C's own, 12 pattern targets, 7 inhibitory | −7.13 | −0.26 | −6.37 | −1.08 | −7.19 | −1.15 | −5.66 | −0.18 | −15.29 | −2.39 | −7 222 | −274 |
| After block 18 alone less without replay, block 18 | B's trials | B's answer's readout | 22 | 14 pattern targets, 2 other excitatory, 6 inhibitory | −4.83 | +0.87 | −2.58 | −1.07 | −5.85 | −1.36 | −4.49 | +0.01 | −9.30 | −1.66 | −4 928 | +854 |
| After block 24 alone less without replay, block 24 | A's trials | A's answer's readout | 17 | 1 C's own, 10 pattern targets, 6 inhibitory | −7.74 | +1.09 | −10.75 | −1.29 | −10.53 | −1.95 | −6.53 | −0.31 | −22.56 | −1.89 | −7 823 | +1 080 |
| After block 24 alone less without replay, block 24 | A's trials | B's answer's readout | 19 | 8 pattern targets, 4 other excitatory, 7 inhibitory | −8.59 | −0.40 | −8.65 | −1.32 | −8.57 | −1.49 | −10.28 | −0.83 | −15.58 | −2.37 | −8 664 | −408 |
| After block 24 alone less without replay, block 24 | B's trials | A's answer's readout | 17 | 9 pattern targets, 8 inhibitory | −9.36 | −0.42 | −7.47 | −1.23 | −7.53 | −1.31 | −5.89 | −0.36 | −18.87 | −2.69 | −9 472 | −427 |
| After block 24 alone less without replay, block 24 | B's trials | B's answer's readout | 17 | 12 pattern targets, 1 other excitatory, 4 inhibitory | −7.57 | +1.09 | −5.01 | −1.25 | −8.21 | −1.43 | −4.66 | −0.29 | −17.03 | −1.40 | −7 552 | +1 082 |
| After block 6 alone less without replay, block 36 | A's trials | A's answer's readout | 3 | 3 pattern targets | −28.13 | +0.90 | −27.73 | −0.42 | +0.09 | −1.18 | −1.89 | −0.18 | −53.89 | −0.53 | −28 101 | +905 |
| After block 6 alone less without replay, block 36 | A's trials | B's answer's readout | 5 | 4 pattern targets, 1 inhibitory | −15.76 | −0.55 | −6.38 | −0.33 | −2.52 | −0.74 | +3.37 | −0.23 | −28.02 | −1.42 | −15 730 | −550 |
| After block 6 alone less without replay, block 36 | B's trials | A's answer's readout | 3 | 2 pattern targets, 1 inhibitory | −19.63 | −0.42 | −48.59 | −0.42 | −1.62 | −0.78 | −14.41 | −0.41 | −55.38 | −1.23 | −19 652 | −417 |
| After block 6 alone less without replay, block 36 | B's trials | B's answer's readout | 4 | 3 pattern targets, 1 other excitatory | −6.71 | +0.60 | −10.69 | −0.44 | −6.60 | −0.79 | −0.45 | −0.07 | −27.32 | −0.60 | −6 722 | +606 |
| After block 12 alone less without replay, block 36 | A's trials | A's answer's readout | 6 | 2 C's own, 4 pattern targets | −17.09 | +1.04 | −9.91 | −0.61 | −7.14 | −1.06 | −1.69 | −0.42 | −32.43 | −0.19 | −17 139 | +1 043 |
| After block 12 alone less without replay, block 36 | A's trials | B's answer's readout | 8 | 4 pattern targets, 1 other excitatory, 3 inhibitory | −11.07 | −0.62 | −9.23 | −0.63 | −4.22 | −0.93 | −5.73 | −0.52 | −18.79 | −1.66 | −11 024 | −624 |
| After block 12 alone less without replay, block 36 | B's trials | A's answer's readout | 3 | 2 pattern targets, 1 inhibitory | −7.77 | −0.42 | −33.90 | −0.35 | −7.31 | −0.45 | −11.42 | −0.30 | −37.55 | −0.93 | −7 740 | −418 |
| After block 12 alone less without replay, block 36 | B's trials | B's answer's readout | 3 | 1 pattern target, 2 inhibitory | −10.40 | +0.53 | −1.45 | −0.19 | −3.18 | −0.69 | −1.61 | −0.11 | −13.24 | −0.29 | −10 419 | +537 |
| After block 18 alone less without replay, block 36 | A's trials | A's answer's readout | 8 | 1 C's own, 5 pattern targets, 2 inhibitory | −9.14 | +1.03 | −9.07 | −0.54 | −8.34 | −1.08 | −2.92 | −0.26 | −23.58 | −0.35 | −9 124 | +1 030 |
| After block 18 alone less without replay, block 36 | A's trials | B's answer's readout | 9 | 3 pattern targets, 1 other excitatory, 5 inhibitory | −8.75 | −0.70 | −12.01 | −0.59 | −4.08 | −0.81 | −6.82 | −0.56 | −18.01 | −1.56 | −8 719 | −696 |
| After block 18 alone less without replay, block 36 | B's trials | A's answer's readout | 6 | 3 pattern targets, 3 inhibitory | −10.52 | −0.41 | −14.91 | −0.40 | −0.07 | −0.43 | −6.31 | −0.27 | −19.15 | −1.00 | −10 535 | −411 |
| After block 18 alone less without replay, block 36 | B's trials | B's answer's readout | 4 | 2 pattern targets, 1 other excitatory, 1 inhibitory | −2.61 | +0.57 | −1.27 | −0.11 | −15.40 | −0.56 | −1.20 | −0.10 | −18.15 | −0.06 | −2 565 | +570 |
| After block 24 alone less without replay, block 36 | A's trials | A's answer's readout | 10 | 1 C's own, 8 pattern targets, 1 inhibitory | −12.19 | +0.94 | −6.76 | −0.74 | −3.72 | −1.22 | −2.57 | +0.07 | −20.11 | −1.06 | −12 126 | +950 |
| After block 24 alone less without replay, block 36 | A's trials | B's answer's readout | 8 | 3 pattern targets, 2 other excitatory, 3 inhibitory | −8.16 | −0.54 | −9.87 | −0.44 | −8.23 | −0.92 | −2.42 | −0.25 | −23.83 | −1.60 | −8 127 | −533 |
| After block 24 alone less without replay, block 36 | B's trials | A's answer's readout | 7 | 5 pattern targets, 2 inhibitory | −13.24 | −0.46 | −9.68 | −0.54 | −8.42 | −0.76 | −4.25 | −0.12 | −27.72 | −1.64 | −13 211 | −458 |
| After block 24 alone less without replay, block 36 | B's trials | B's answer's readout | 5 | 4 pattern targets, 1 other excitatory | −5.40 | +0.72 | −12.56 | −0.56 | −3.04 | −0.89 | −3.11 | +0.08 | −20.31 | −0.84 | −5 391 | +719 |

Right after each single night such units numbered 13 to 19 a readout on the image from the assignment and
13 to 22 on the image from the mirrored assignment. From the trial's stimulus's kinds each lost 6.33 to 11.15 k a trial on the image
from the assignment and 4.83 to 12.00 on the image from the mirrored assignment, where each of the rest moved by −0.42 to +1.08 and
−0.42 to +1.29. Excitatory less inhibitory each lost 11.48 to 26.13 and 9.30 to 32.69, each of the rest
1.44 to 2.86 and 1.40 to 2.69. Their inhibitory drive fell by 4.04 to 12.16 and
2.43 to 10.86, the rest's moved by −1.33 to +0.03 and −0.99 to +0.15.

**The correlation over each readout's 459 units** of a unit's drive difference, both spans, with its spikes' difference in the
window, beside its input's. On the
image from the assignment:

| Comparison | Trials | The readout | The drive's difference from the trial's stimulus's kinds | from the readout's own place group | from the other place group | inhibitory | excitatory less inhibitory | The input's difference from the trial's stimulus's kinds | inhibitory | excitatory less inhibitory |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | A's answer's readout | +0.60 | +0.31 | +0.47 | +0.19 | +0.55 | +0.60 | +0.17 | +0.12 |
| After block 6 alone less without replay, block 6 | A's trials | B's answer's readout | +0.63 | +0.26 | +0.55 | +0.38 | +0.56 | +0.63 | +0.15 | +0.28 |
| After block 6 alone less without replay, block 6 | B's trials | A's answer's readout | +0.72 | +0.19 | +0.15 | +0.21 | +0.43 | +0.72 | +0.11 | +0.15 |
| After block 6 alone less without replay, block 6 | B's trials | B's answer's readout | +0.62 | +0.22 | +0.43 | +0.22 | +0.43 | +0.62 | +0.18 | +0.20 |
| After block 12 alone less without replay, block 12 | A's trials | A's answer's readout | +0.61 | +0.40 | +0.45 | +0.35 | +0.51 | +0.61 | +0.20 | +0.15 |
| After block 12 alone less without replay, block 12 | A's trials | B's answer's readout | +0.60 | +0.25 | +0.65 | +0.40 | +0.52 | +0.60 | +0.09 | +0.23 |
| After block 12 alone less without replay, block 12 | B's trials | A's answer's readout | +0.61 | +0.38 | +0.16 | +0.26 | +0.44 | +0.61 | +0.13 | +0.14 |
| After block 12 alone less without replay, block 12 | B's trials | B's answer's readout | +0.58 | +0.25 | +0.39 | +0.19 | +0.40 | +0.58 | +0.17 | +0.19 |
| After block 18 alone less without replay, block 18 | A's trials | A's answer's readout | +0.61 | +0.44 | +0.45 | +0.37 | +0.48 | +0.62 | +0.18 | +0.22 |
| After block 18 alone less without replay, block 18 | A's trials | B's answer's readout | +0.70 | +0.20 | +0.55 | +0.37 | +0.45 | +0.70 | +0.10 | +0.20 |
| After block 18 alone less without replay, block 18 | B's trials | A's answer's readout | +0.50 | +0.32 | +0.33 | +0.19 | +0.45 | +0.51 | +0.08 | +0.19 |
| After block 18 alone less without replay, block 18 | B's trials | B's answer's readout | +0.57 | +0.34 | +0.48 | +0.21 | +0.48 | +0.57 | +0.17 | +0.21 |
| After block 24 alone less without replay, block 24 | A's trials | A's answer's readout | +0.61 | +0.34 | +0.52 | +0.28 | +0.55 | +0.61 | +0.19 | +0.21 |
| After block 24 alone less without replay, block 24 | A's trials | B's answer's readout | +0.60 | +0.27 | +0.47 | +0.35 | +0.40 | +0.60 | +0.09 | +0.16 |
| After block 24 alone less without replay, block 24 | B's trials | A's answer's readout | +0.55 | +0.29 | +0.43 | +0.20 | +0.52 | +0.55 | +0.05 | +0.19 |
| After block 24 alone less without replay, block 24 | B's trials | B's answer's readout | +0.55 | +0.30 | +0.56 | +0.19 | +0.54 | +0.55 | +0.15 | +0.18 |
| After block 6 alone less without replay, block 36 | A's trials | A's answer's readout | +0.39 | +0.38 | +0.74 | +0.23 | +0.72 | +0.39 | +0.09 | +0.18 |
| After block 6 alone less without replay, block 36 | A's trials | B's answer's readout | +0.79 | +0.15 | +0.26 | +0.48 | +0.39 | +0.79 | +0.12 | +0.23 |
| After block 6 alone less without replay, block 36 | B's trials | A's answer's readout | +0.56 | +0.35 | +0.57 | +0.40 | +0.46 | +0.56 | +0.27 | +0.18 |
| After block 6 alone less without replay, block 36 | B's trials | B's answer's readout | +0.48 | +0.21 | +0.56 | +0.17 | +0.55 | +0.48 | +0.18 | +0.05 |
| After block 12 alone less without replay, block 36 | A's trials | A's answer's readout | +0.39 | +0.46 | +0.68 | +0.21 | +0.65 | +0.39 | +0.13 | +0.29 |
| After block 12 alone less without replay, block 36 | A's trials | B's answer's readout | +0.82 | +0.11 | +0.19 | +0.27 | +0.42 | +0.82 | +0.10 | +0.23 |
| After block 12 alone less without replay, block 36 | B's trials | A's answer's readout | +0.46 | +0.36 | +0.43 | +0.41 | +0.33 | +0.45 | +0.12 | +0.19 |
| After block 12 alone less without replay, block 36 | B's trials | B's answer's readout | +0.36 | +0.30 | +0.61 | +0.14 | +0.51 | +0.36 | +0.15 | +0.08 |
| After block 18 alone less without replay, block 36 | A's trials | A's answer's readout | +0.48 | +0.56 | +0.55 | +0.26 | +0.66 | +0.48 | +0.11 | +0.34 |
| After block 18 alone less without replay, block 36 | A's trials | B's answer's readout | +0.78 | +0.18 | +0.32 | +0.29 | +0.39 | +0.78 | +0.11 | +0.21 |
| After block 18 alone less without replay, block 36 | B's trials | A's answer's readout | +0.55 | +0.25 | +0.45 | +0.24 | +0.49 | +0.55 | +0.10 | +0.24 |
| After block 18 alone less without replay, block 36 | B's trials | B's answer's readout | +0.40 | +0.20 | +0.57 | +0.08 | +0.54 | +0.40 | +0.16 | +0.12 |
| After block 24 alone less without replay, block 36 | A's trials | A's answer's readout | +0.60 | +0.38 | +0.53 | +0.16 | +0.66 | +0.60 | +0.14 | +0.29 |
| After block 24 alone less without replay, block 36 | A's trials | B's answer's readout | +0.73 | +0.17 | +0.31 | +0.27 | +0.45 | +0.73 | +0.12 | +0.21 |
| After block 24 alone less without replay, block 36 | B's trials | A's answer's readout | +0.58 | +0.28 | +0.42 | +0.19 | +0.59 | +0.58 | +0.09 | +0.19 |
| After block 24 alone less without replay, block 36 | B's trials | B's answer's readout | +0.46 | +0.31 | +0.60 | +0.16 | +0.61 | +0.46 | +0.11 | +0.21 |

On the image from the mirrored assignment:

| Comparison | Trials | The readout | The drive's difference from the trial's stimulus's kinds | from the readout's own place group | from the other place group | inhibitory | excitatory less inhibitory | The input's difference from the trial's stimulus's kinds | inhibitory | excitatory less inhibitory |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | A's answer's readout | +0.58 | +0.37 | +0.59 | +0.20 | +0.67 | +0.58 | +0.18 | +0.24 |
| After block 6 alone less without replay, block 6 | A's trials | B's answer's readout | +0.83 | +0.20 | +0.30 | +0.36 | +0.38 | +0.83 | +0.06 | +0.33 |
| After block 6 alone less without replay, block 6 | B's trials | A's answer's readout | +0.71 | +0.26 | +0.22 | +0.28 | +0.28 | +0.71 | +0.17 | +0.13 |
| After block 6 alone less without replay, block 6 | B's trials | B's answer's readout | +0.53 | +0.35 | +0.42 | +0.20 | +0.55 | +0.53 | +0.13 | +0.20 |
| After block 12 alone less without replay, block 12 | A's trials | A's answer's readout | +0.59 | +0.43 | +0.49 | +0.23 | +0.59 | +0.59 | +0.17 | +0.19 |
| After block 12 alone less without replay, block 12 | A's trials | B's answer's readout | +0.78 | +0.28 | +0.36 | +0.43 | +0.34 | +0.78 | +0.08 | +0.35 |
| After block 12 alone less without replay, block 12 | B's trials | A's answer's readout | +0.69 | +0.29 | +0.27 | +0.30 | +0.33 | +0.69 | +0.17 | +0.17 |
| After block 12 alone less without replay, block 12 | B's trials | B's answer's readout | +0.45 | +0.30 | +0.37 | +0.13 | +0.49 | +0.45 | +0.13 | +0.16 |
| After block 18 alone less without replay, block 18 | A's trials | A's answer's readout | +0.61 | +0.42 | +0.44 | +0.27 | +0.48 | +0.61 | +0.17 | +0.04 |
| After block 18 alone less without replay, block 18 | A's trials | B's answer's readout | +0.71 | +0.33 | +0.44 | +0.42 | +0.42 | +0.71 | +0.06 | +0.30 |
| After block 18 alone less without replay, block 18 | B's trials | A's answer's readout | +0.65 | +0.50 | +0.28 | +0.24 | +0.48 | +0.65 | +0.17 | +0.16 |
| After block 18 alone less without replay, block 18 | B's trials | B's answer's readout | +0.42 | +0.22 | +0.40 | +0.22 | +0.35 | +0.43 | +0.16 | +0.17 |
| After block 24 alone less without replay, block 24 | A's trials | A's answer's readout | +0.55 | +0.43 | +0.40 | +0.28 | +0.45 | +0.55 | +0.18 | 0.00 |
| After block 24 alone less without replay, block 24 | A's trials | B's answer's readout | +0.66 | +0.42 | +0.41 | +0.41 | +0.40 | +0.66 | −0.01 | +0.34 |
| After block 24 alone less without replay, block 24 | B's trials | A's answer's readout | +0.68 | +0.39 | +0.35 | +0.27 | +0.53 | +0.68 | +0.17 | +0.14 |
| After block 24 alone less without replay, block 24 | B's trials | B's answer's readout | +0.52 | +0.25 | +0.39 | +0.23 | +0.41 | +0.52 | +0.10 | +0.17 |
| After block 6 alone less without replay, block 36 | A's trials | A's answer's readout | +0.76 | +0.41 | +0.32 | +0.10 | +0.69 | +0.76 | +0.14 | +0.12 |
| After block 6 alone less without replay, block 36 | A's trials | B's answer's readout | +0.69 | +0.48 | +0.25 | +0.04 | +0.62 | +0.68 | +0.18 | +0.26 |
| After block 6 alone less without replay, block 36 | B's trials | A's answer's readout | +0.71 | +0.76 | +0.24 | +0.31 | +0.71 | +0.71 | +0.31 | +0.20 |
| After block 6 alone less without replay, block 36 | B's trials | B's answer's readout | +0.47 | +0.23 | +0.35 | +0.11 | +0.45 | +0.47 | +0.20 | +0.12 |
| After block 12 alone less without replay, block 36 | A's trials | A's answer's readout | +0.64 | +0.41 | +0.38 | +0.13 | +0.64 | +0.64 | +0.14 | +0.10 |
| After block 12 alone less without replay, block 36 | A's trials | B's answer's readout | +0.65 | +0.54 | +0.33 | +0.19 | +0.55 | +0.65 | +0.10 | +0.33 |
| After block 12 alone less without replay, block 36 | B's trials | A's answer's readout | +0.63 | +0.71 | +0.28 | +0.32 | +0.68 | +0.63 | +0.24 | +0.17 |
| After block 12 alone less without replay, block 36 | B's trials | B's answer's readout | +0.52 | +0.26 | +0.28 | +0.05 | +0.45 | +0.52 | −0.01 | +0.22 |
| After block 18 alone less without replay, block 36 | A's trials | A's answer's readout | +0.62 | +0.37 | +0.44 | +0.18 | +0.58 | +0.62 | +0.11 | +0.11 |
| After block 18 alone less without replay, block 36 | A's trials | B's answer's readout | +0.66 | +0.48 | +0.34 | +0.30 | +0.51 | +0.66 | +0.06 | +0.29 |
| After block 18 alone less without replay, block 36 | B's trials | A's answer's readout | +0.59 | +0.66 | +0.25 | +0.31 | +0.58 | +0.58 | +0.21 | +0.18 |
| After block 18 alone less without replay, block 36 | B's trials | B's answer's readout | +0.38 | +0.31 | +0.47 | +0.14 | +0.37 | +0.38 | +0.05 | +0.22 |
| After block 24 alone less without replay, block 36 | A's trials | A's answer's readout | +0.62 | +0.35 | +0.34 | +0.20 | +0.55 | +0.62 | +0.15 | +0.04 |
| After block 24 alone less without replay, block 36 | A's trials | B's answer's readout | +0.61 | +0.50 | +0.40 | +0.25 | +0.56 | +0.61 | +0.01 | +0.31 |
| After block 24 alone less without replay, block 36 | B's trials | A's answer's readout | +0.73 | +0.44 | +0.32 | +0.26 | +0.60 | +0.73 | +0.14 | +0.17 |
| After block 24 alone less without replay, block 36 | B's trials | B's answer's readout | +0.50 | +0.28 | +0.34 | +0.14 | +0.50 | +0.50 | +0.07 | +0.19 |

Right after each single night, excitatory less inhibitory, the drive's correlation read 0.40 to 0.56 on the image from the
assignment and 0.28 to 0.67 on the image from the mirrored assignment, the input's 0.12 to 0.28 and 0.00 to +0.35; the
inhibitory drive's 0.19 to 0.40 and 0.13 to 0.43, the inhibitory input's 0.05 to 0.20 and
−0.01 to +0.18. The trial's stimulus's kinds' drive read 0.50 to 0.72 and 0.42 to 0.83, within
0.009 of their input's in each row.

**At block 36**, less the night without replay:

| Comparison | Onto | From | From the assignment: the window, A's trials, k | B's trials | the volley, A's trials | B's trials | From the mirrored assignment: the window, A's trials | B's trials | the volley, A's trials | B's trials |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 36 | A's answer's readout | A's kinds | −0.1 ± 0.1 | +3.6 ± 3.8 | +296.8 ± 22.1 | +0.8 ± 0.6 | −0.1 ± 0.1 | −6.6 ± 2.9 | +327.2 ± 4.0 | −0.3 ± 0.8 |
| After block 6 alone less without replay, block 36 | A's answer's readout | B's kinds | −31.0 ± 12.7 | +0.1 ± 0.4 | +0.3 ± 0.9 | −250.0 ± 29.5 | +4.5 ± 2.8 | +0.4 ± 0.4 | +1.6 ± 0.9 | −250.5 ± 26.3 |
| After block 6 alone less without replay, block 36 | A's answer's readout | The odd places' excitatory | −660.5 ± 20.2 | −261.6 ± 20.1 | −0.8 ± 1.5 | +0.6 ± 1.1 | −281.0 ± 14.4 | −335.1 ± 15.0 | +3.5 ± 2.2 | −2.5 ± 2.9 |
| After block 6 alone less without replay, block 36 | A's answer's readout | The even places' excitatory | −432.0 ± 20.6 | −382.9 ± 22.7 | −2.4 ± 2.5 | −0.3 ± 2.0 | −535.5 ± 6.2 | −358.8 ± 33.9 | −2.5 ± 3.3 | −1.8 ± 1.7 |
| After block 6 alone less without replay, block 36 | A's answer's readout | Other excitatory | −9.7 ± 1.0 | −2.4 ± 0.7 | −0.2 ± 0.2 | −0.2 ± 0.1 | −13.1 ± 3.9 | −5.6 ± 0.7 | +0.2 ± 0.1 | 0.0 ± 0.1 |
| After block 6 alone less without replay, block 36 | A's answer's readout | Inhibitory | −244.7 ± 16.1 | −240.1 ± 18.0 | +3.8 ± 3.3 | +2.1 ± 1.6 | −98.7 ± 14.7 | −231.1 ± 16.3 | +7.2 ± 3.2 | −1.5 ± 0.7 |
| After block 6 alone less without replay, block 36 | A's answer's readout | All excitatory | −1 133.2 ± 29.8 | −643.2 ± 41.8 | +293.7 ± 19.9 | −249.1 ± 31.0 | −825.3 ± 18.2 | −705.6 ± 33.3 | +329.9 ± 4.9 | −255.1 ± 28.1 |
| After block 6 alone less without replay, block 36 | A's answer's readout | Excitatory less inhibitory | −888.6 ± 22.1 | −403.1 ± 52.0 | +289.9 ± 21.6 | −251.1 ± 31.3 | −726.7 ± 5.9 | −474.5 ± 42.5 | +322.7 ± 7.7 | −253.6 ± 27.3 |
| After block 6 alone less without replay, block 36 | B's answer's readout | A's kinds | −0.1 ± 0.1 | +3.3 ± 4.9 | −300.4 ± 23.0 | 0.0 ± 1.2 | −0.1 ± 0.1 | −22.6 ± 3.8 | −327.7 ± 3.9 | −1.3 ± 1.1 |
| After block 6 alone less without replay, block 36 | B's answer's readout | B's kinds | −2.3 ± 6.6 | −0.2 ± 0.1 | +0.5 ± 0.4 | +249.0 ± 30.7 | +16.3 ± 9.0 | +0.5 ± 0.3 | +2.8 ± 0.7 | +246.3 ± 26.6 |
| After block 6 alone less without replay, block 36 | B's answer's readout | The odd places' excitatory | −270.0 ± 19.5 | −440.1 ± 27.1 | +1.1 ± 1.7 | −0.1 ± 2.1 | −350.3 ± 20.4 | −385.3 ± 13.6 | +1.0 ± 2.0 | −1.5 ± 2.6 |
| After block 6 alone less without replay, block 36 | B's answer's readout | The even places' excitatory | −449.3 ± 23.7 | −563.6 ± 18.3 | −3.8 ± 2.1 | +1.2 ± 2.8 | −184.7 ± 19.8 | −244.0 ± 31.6 | +0.9 ± 2.8 | +2.3 ± 1.7 |
| After block 6 alone less without replay, block 36 | B's answer's readout | Other excitatory | −4.4 ± 0.6 | −1.5 ± 0.5 | −0.1 ± 0.1 | 0.0 ± 0.1 | −34.6 ± 7.2 | −9.1 ± 1.2 | −0.1 ± 0.2 | −0.2 ± 0.2 |
| After block 6 alone less without replay, block 36 | B's answer's readout | Inhibitory | −427.7 ± 40.8 | −127.6 ± 14.3 | +2.4 ± 3.5 | +1.7 ± 2.5 | −97.5 ± 42.1 | −34.2 ± 27.8 | +6.9 ± 1.8 | +0.1 ± 2.4 |
| After block 6 alone less without replay, block 36 | B's answer's readout | All excitatory | −726.1 ± 32.6 | −1 002.1 ± 48.4 | −302.7 ± 25.9 | +250.1 ± 28.7 | −553.3 ± 31.9 | −660.4 ± 39.4 | −323.1 ± 7.5 | +245.6 ± 26.4 |
| After block 6 alone less without replay, block 36 | B's answer's readout | Excitatory less inhibitory | −298.4 ± 52.6 | −874.4 ± 32.7 | −305.1 ± 24.0 | +248.4 ± 27.1 | −455.8 ± 40.7 | −626.3 ± 15.9 | −330.0 ± 5.8 | +245.5 ± 28.3 |
| After block 12 alone less without replay, block 36 | A's answer's readout | A's kinds | +0.1 ± 0.1 | +4.1 ± 2.9 | +377.8 ± 11.4 | +1.1 ± 0.9 | −0.1 ± 0.1 | −5.7 ± 2.8 | +368.5 ± 7.7 | +1.0 ± 1.0 |
| After block 12 alone less without replay, block 36 | A's answer's readout | B's kinds | −12.0 ± 12.7 | −0.2 ± 0.3 | −1.6 ± 0.3 | −246.2 ± 39.5 | +13.6 ± 0.3 | +0.5 ± 0.4 | +1.1 ± 0.7 | −215.4 ± 25.5 |
| After block 12 alone less without replay, block 36 | A's answer's readout | The odd places' excitatory | −647.9 ± 34.2 | −155.0 ± 26.6 | +2.4 ± 1.5 | −0.1 ± 0.7 | −338.4 ± 26.3 | −260.0 ± 14.1 | +0.9 ± 0.1 | 0.0 ± 2.1 |
| After block 12 alone less without replay, block 36 | A's answer's readout | The even places' excitatory | −350.0 ± 24.3 | −264.5 ± 19.2 | +0.5 ± 1.3 | +0.8 ± 1.2 | −521.0 ± 14.6 | −225.0 ± 24.2 | −0.7 ± 1.4 | −1.1 ± 1.6 |
| After block 12 alone less without replay, block 36 | A's answer's readout | Other excitatory | −7.0 ± 0.5 | −1.4 ± 0.5 | +0.1 ± 0.2 | 0.0 ± 0.0 | −4.8 ± 1.6 | −3.8 ± 0.9 | 0.0 ± 0.1 | −0.1 ± 0.1 |
| After block 12 alone less without replay, block 36 | A's answer's readout | Inhibitory | −153.8 ± 43.8 | −209.5 ± 32.3 | +4.5 ± 1.3 | +1.3 ± 1.2 | −203.1 ± 29.1 | −174.3 ± 21.8 | +3.8 ± 2.3 | +3.3 ± 3.2 |
| After block 12 alone less without replay, block 36 | A's answer's readout | All excitatory | −1 016.8 ± 64.0 | −417.0 ± 44.3 | +379.2 ± 12.6 | −244.4 ± 38.9 | −850.8 ± 25.1 | −494.1 ± 39.6 | +369.8 ± 8.3 | −215.6 ± 24.2 |
| After block 12 alone less without replay, block 36 | A's answer's readout | Excitatory less inhibitory | −863.0 ± 27.7 | −207.5 ± 72.7 | +374.7 ± 12.1 | −245.7 ± 39.7 | −647.7 ± 43.7 | −319.8 ± 36.2 | +366.0 ± 9.3 | −218.9 ± 24.7 |
| After block 12 alone less without replay, block 36 | B's answer's readout | A's kinds | +0.1 ± 0.1 | −2.1 ± 3.8 | −378.5 ± 10.8 | +0.7 ± 0.8 | −0.1 ± 0.1 | −21.3 ± 3.7 | −368.9 ± 8.2 | +0.3 ± 1.2 |
| After block 12 alone less without replay, block 36 | B's answer's readout | B's kinds | +22.6 ± 12.9 | −0.2 ± 0.2 | −0.8 ± 0.2 | +243.8 ± 39.1 | +17.6 ± 11.4 | +0.2 ± 0.2 | +1.1 ± 0.2 | +212.3 ± 25.8 |
| After block 12 alone less without replay, block 36 | B's answer's readout | The odd places' excitatory | −258.4 ± 17.2 | −394.4 ± 32.4 | +2.9 ± 1.6 | −0.1 ± 1.7 | −455.0 ± 20.0 | −322.6 ± 8.1 | +0.5 ± 1.4 | +0.3 ± 2.5 |
| After block 12 alone less without replay, block 36 | B's answer's readout | The even places' excitatory | −361.4 ± 23.3 | −407.9 ± 16.8 | +0.2 ± 1.9 | +0.6 ± 1.2 | −360.2 ± 36.6 | −94.6 ± 12.2 | +1.5 ± 1.6 | +2.9 ± 0.8 |
| After block 12 alone less without replay, block 36 | B's answer's readout | Other excitatory | −3.3 ± 0.2 | −0.4 ± 0.3 | +0.1 ± 0.1 | 0.0 ± 0.0 | −16.2 ± 4.9 | −6.7 ± 1.4 | −0.3 ± 0.2 | −0.2 ± 0.2 |
| After block 12 alone less without replay, block 36 | B's answer's readout | Inhibitory | −422.6 ± 71.5 | −45.8 ± 20.9 | +3.7 ± 2.5 | +0.6 ± 1.7 | −285.1 ± 52.0 | −63.0 ± 18.7 | +4.1 ± 2.0 | +4.9 ± 3.5 |
| After block 12 alone less without replay, block 36 | B's answer's readout | All excitatory | −600.4 ± 39.2 | −804.9 ± 44.3 | −376.2 ± 9.3 | +245.1 ± 37.3 | −813.9 ± 60.6 | −444.9 ± 13.8 | −366.2 ± 8.8 | +215.6 ± 26.3 |
| After block 12 alone less without replay, block 36 | B's answer's readout | Excitatory less inhibitory | −177.8 ± 39.6 | −759.0 ± 57.0 | −379.8 ± 9.8 | +244.5 ± 37.4 | −528.8 ± 54.1 | −381.9 ± 12.2 | −370.3 ± 9.1 | +210.7 ± 24.9 |
| After block 18 alone less without replay, block 36 | A's answer's readout | A's kinds | 0.0 ± 0.0 | +2.7 ± 2.2 | +302.2 ± 20.1 | +0.1 ± 0.5 | −0.1 ± 0.1 | −5.4 ± 2.4 | +389.7 ± 19.5 | −0.1 ± 1.3 |
| After block 18 alone less without replay, block 36 | A's answer's readout | B's kinds | −31.2 ± 5.7 | −0.2 ± 0.4 | −0.7 ± 0.3 | −238.0 ± 14.6 | 0.0 ± 4.5 | +0.2 ± 0.2 | +1.1 ± 0.6 | −251.1 ± 8.2 |
| After block 18 alone less without replay, block 36 | A's answer's readout | The odd places' excitatory | −614.3 ± 31.1 | −259.6 ± 40.5 | −1.5 ± 1.3 | −0.7 ± 0.3 | −322.0 ± 20.3 | −271.1 ± 9.1 | +3.1 ± 2.4 | −0.5 ± 2.8 |
| After block 18 alone less without replay, block 36 | A's answer's readout | The even places' excitatory | −444.7 ± 10.8 | −252.7 ± 19.6 | +0.7 ± 0.6 | +0.2 ± 0.7 | −552.6 ± 5.0 | −192.9 ± 4.2 | −3.2 ± 1.7 | −1.8 ± 1.7 |
| After block 18 alone less without replay, block 36 | A's answer's readout | Other excitatory | −9.0 ± 0.9 | −0.9 ± 0.4 | +0.2 ± 0.2 | −0.1 ± 0.1 | −3.2 ± 1.2 | −4.0 ± 0.9 | 0.0 ± 0.3 | 0.0 ± 0.1 |
| After block 18 alone less without replay, block 36 | A's answer's readout | Inhibitory | −228.5 ± 17.1 | −152.3 ± 7.8 | +3.7 ± 1.5 | 0.0 ± 1.9 | −148.7 ± 42.6 | −163.5 ± 16.1 | +7.4 ± 3.4 | +4.3 ± 2.6 |
| After block 18 alone less without replay, block 36 | A's answer's readout | All excitatory | −1 099.2 ± 38.7 | −510.6 ± 59.2 | +300.8 ± 19.5 | −238.5 ± 14.5 | −877.9 ± 27.8 | −473.2 ± 13.8 | +390.7 ± 18.8 | −253.5 ± 7.8 |
| After block 18 alone less without replay, block 36 | A's answer's readout | Excitatory less inhibitory | −870.7 ± 31.6 | −358.4 ± 57.7 | +297.1 ± 18.4 | −238.5 ± 13.7 | −729.3 ± 29.8 | −309.7 ± 24.3 | +383.3 ± 18.7 | −257.8 ± 10.4 |
| After block 18 alone less without replay, block 36 | B's answer's readout | A's kinds | 0.0 ± 0.0 | +0.1 ± 2.7 | −303.9 ± 19.3 | +0.2 ± 0.7 | −0.1 ± 0.1 | −19.7 ± 3.7 | −391.6 ± 18.9 | −0.6 ± 1.2 |
| After block 18 alone less without replay, block 36 | B's answer's readout | B's kinds | −1.9 ± 4.8 | −0.3 ± 0.4 | −0.1 ± 0.5 | +237.1 ± 15.4 | −6.1 ± 8.4 | +0.3 ± 0.3 | +2.1 ± 1.4 | +248.4 ± 9.8 |
| After block 18 alone less without replay, block 36 | B's answer's readout | The odd places' excitatory | −280.7 ± 23.6 | −441.9 ± 35.0 | +1.0 ± 1.4 | −1.1 ± 1.9 | −400.7 ± 25.7 | −318.2 ± 4.7 | −0.4 ± 0.8 | 0.0 ± 2.0 |
| After block 18 alone less without replay, block 36 | B's answer's readout | The even places' excitatory | −477.4 ± 20.7 | −445.8 ± 17.2 | +0.1 ± 1.2 | +0.4 ± 0.9 | −371.4 ± 22.4 | −56.3 ± 21.1 | −0.3 ± 1.1 | +1.1 ± 2.5 |
| After block 18 alone less without replay, block 36 | B's answer's readout | Other excitatory | −4.4 ± 0.5 | −0.6 ± 0.3 | +0.1 ± 0.1 | 0.0 ± 0.0 | −10.6 ± 0.5 | −6.2 ± 1.3 | −0.1 ± 0.5 | −0.1 ± 0.1 |
| After block 18 alone less without replay, block 36 | B's answer's readout | Inhibitory | −535.4 ± 49.9 | −138.7 ± 16.3 | +2.7 ± 1.4 | −0.6 ± 2.4 | −322.2 ± 40.9 | −58.3 ± 16.7 | +5.3 ± 0.9 | +4.9 ± 6.5 |
| After block 18 alone less without replay, block 36 | B's answer's readout | All excitatory | −764.4 ± 34.5 | −888.4 ± 50.6 | −302.8 ± 19.8 | +236.6 ± 15.7 | −788.8 ± 30.2 | −400.1 ± 22.5 | −390.3 ± 18.9 | +248.9 ± 10.3 |
| After block 18 alone less without replay, block 36 | B's answer's readout | Excitatory less inhibitory | −229.0 ± 23.5 | −749.7 ± 39.6 | −305.5 ± 20.6 | +237.2 ± 15.8 | −466.7 ± 42.1 | −341.7 ± 12.3 | −395.6 ± 18.5 | +244.0 ± 5.5 |
| After block 24 alone less without replay, block 36 | A's answer's readout | A's kinds | +0.2 ± 0.1 | −0.7 ± 2.9 | +274.1 ± 9.6 | +0.8 ± 0.2 | −0.1 ± 0.1 | −1.5 ± 2.5 | +300.3 ± 5.2 | −0.4 ± 0.4 |
| After block 24 alone less without replay, block 36 | A's answer's readout | B's kinds | −75.9 ± 9.4 | 0.0 ± 0.4 | +0.6 ± 0.3 | −356.4 ± 7.9 | +9.8 ± 3.8 | +0.1 ± 0.2 | +0.7 ± 0.3 | −301.3 ± 27.8 |
| After block 24 alone less without replay, block 36 | A's answer's readout | The odd places' excitatory | −612.5 ± 32.7 | −352.1 ± 17.5 | +1.2 ± 1.5 | +0.1 ± 0.8 | −401.4 ± 16.9 | −311.7 ± 25.1 | +3.6 ± 1.3 | +0.2 ± 2.5 |
| After block 24 alone less without replay, block 36 | A's answer's readout | The even places' excitatory | −458.5 ± 25.2 | −459.4 ± 28.9 | +1.7 ± 1.9 | +1.1 ± 0.2 | −578.3 ± 18.2 | −400.7 ± 8.3 | −8.0 ± 3.2 | −2.8 ± 0.6 |
| After block 24 alone less without replay, block 36 | A's answer's readout | Other excitatory | −5.0 ± 1.0 | −2.2 ± 0.5 | −0.1 ± 0.3 | +0.1 ± 0.2 | −0.6 ± 2.1 | −4.8 ± 1.7 | −0.1 ± 0.1 | 0.0 ± 0.1 |
| After block 24 alone less without replay, block 36 | A's answer's readout | Inhibitory | −110.5 ± 31.5 | −129.8 ± 28.7 | +7.6 ± 1.6 | +1.2 ± 2.4 | −8.1 ± 30.5 | −89.8 ± 21.3 | +10.5 ± 7.6 | +2.9 ± 3.7 |
| After block 24 alone less without replay, block 36 | A's answer's readout | All excitatory | −1 151.8 ± 52.2 | −814.3 ± 41.5 | +277.5 ± 7.7 | −354.4 ± 6.9 | −970.6 ± 24.6 | −718.6 ± 31.6 | +296.5 ± 4.2 | −304.3 ± 27.3 |
| After block 24 alone less without replay, block 36 | A's answer's readout | Excitatory less inhibitory | −1 041.4 ± 41.4 | −684.5 ± 40.1 | +270.0 ± 8.4 | −355.6 ± 8.3 | −962.5 ± 34.6 | −628.8 ± 21.9 | +286.0 ± 4.6 | −307.2 ± 30.1 |
| After block 24 alone less without replay, block 36 | B's answer's readout | A's kinds | +0.1 ± 0.1 | −6.2 ± 4.0 | −277.6 ± 8.8 | 0.0 ± 0.4 | −0.1 ± 0.1 | −13.5 ± 1.2 | −308.1 ± 7.9 | −1.1 ± 0.6 |
| After block 24 alone less without replay, block 36 | B's answer's readout | B's kinds | −35.1 ± 8.3 | 0.0 ± 0.3 | +1.3 ± 0.8 | +356.7 ± 7.8 | +21.7 ± 9.3 | +0.2 ± 0.1 | +1.1 ± 1.0 | +297.5 ± 27.8 |
| After block 24 alone less without replay, block 36 | B's answer's readout | The odd places' excitatory | −174.9 ± 16.1 | −450.4 ± 26.3 | +2.1 ± 0.4 | −1.4 ± 1.9 | −485.0 ± 24.6 | −422.2 ± 16.1 | +1.3 ± 1.8 | +1.8 ± 2.2 |
| After block 24 alone less without replay, block 36 | B's answer's readout | The even places' excitatory | −478.4 ± 45.4 | −641.7 ± 12.4 | +0.6 ± 1.1 | +1.1 ± 1.7 | −271.2 ± 14.4 | −315.0 ± 7.2 | −5.1 ± 2.4 | −0.1 ± 0.5 |
| After block 24 alone less without replay, block 36 | B's answer's readout | Other excitatory | −1.9 ± 0.6 | −1.5 ± 0.3 | 0.0 ± 0.1 | +0.1 ± 0.2 | −0.8 ± 4.8 | −7.5 ± 2.3 | −0.3 ± 0.2 | −0.1 ± 0.2 |
| After block 24 alone less without replay, block 36 | B's answer's readout | Inhibitory | −383.6 ± 44.0 | −137.0 ± 13.8 | +7.2 ± 2.2 | +2.4 ± 1.0 | −143.6 ± 56.7 | +19.8 ± 18.2 | +8.7 ± 4.0 | +2.2 ± 2.6 |
| After block 24 alone less without replay, block 36 | B's answer's readout | All excitatory | −690.1 ± 58.6 | −1 099.8 ± 36.1 | −273.6 ± 10.1 | +356.5 ± 6.9 | −735.4 ± 38.5 | −758.0 ± 13.8 | −311.0 ± 10.0 | +298.1 ± 30.0 |
| After block 24 alone less without replay, block 36 | B's answer's readout | Excitatory less inhibitory | −306.5 ± 51.2 | −962.8 ± 22.7 | −280.8 ± 10.3 | +354.1 ± 6.6 | −591.8 ± 30.7 | −777.8 ± 10.9 | −319.7 ± 11.7 | +295.9 ± 28.7 |

Excitatory less inhibitory in the window lies below beyond 2 SE in 16 of 16 on the image from the assignment and
16 of 16 on the image from the mirrored assignment; the trial's stimulus's kinds' volley drive lies above onto its answer's readout in
8 of 8 and 8 of 8, and below onto the other in 8 of 8 and
8 of 8.

**What the later blocks bring back**, both spans, from right after each single night to block 36, each change's per cent that came back:

| Form | Trials | Onto | From | From the assignment: right after, k | at block 36 | come back, per cent | From the mirrored assignment: right after | at block 36 | come back, per cent |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone | A's trials | A's answer's readout | A's kinds | +350.4 | +296.7 | 15 | +342.7 | +327.1 | 5 |
| After block 6 alone | A's trials | A's answer's readout | B's kinds | −61.6 | −30.7 | 50 | −29.3 | +6.0 | 121 |
| After block 6 alone | A's trials | A's answer's readout | Inhibitory | −177.3 | −240.9 | −36 | −214.8 | −91.5 | 57 |
| After block 6 alone | A's trials | A's answer's readout | All excitatory | −1 442.3 | −839.5 | 42 | −1 466.9 | −495.4 | 66 |
| After block 6 alone | A's trials | A's answer's readout | Excitatory less inhibitory | −1 265.0 | −598.6 | 53 | −1 252.1 | −404.0 | 68 |
| After block 6 alone | A's trials | B's answer's readout | A's kinds | −335.5 | −300.5 | 10 | −327.7 | −327.8 | 0 |
| After block 6 alone | A's trials | B's answer's readout | B's kinds | −39.3 | −1.8 | 95 | −38.8 | +19.2 | 149 |
| After block 6 alone | A's trials | B's answer's readout | Inhibitory | −500.2 | −425.3 | 15 | −477.0 | −90.6 | 81 |
| After block 6 alone | A's trials | B's answer's readout | All excitatory | −2 036.2 | −1 028.8 | 49 | −1 702.0 | −876.4 | 49 |
| After block 6 alone | A's trials | B's answer's readout | Excitatory less inhibitory | −1 536.0 | −603.6 | 61 | −1 224.9 | −785.8 | 36 |
| After block 6 alone | B's trials | A's answer's readout | A's kinds | −21.2 | +4.4 | 121 | −27.8 | −6.8 | 75 |
| After block 6 alone | B's trials | A's answer's readout | B's kinds | −307.9 | −249.9 | 19 | −261.9 | −250.1 | 5 |
| After block 6 alone | B's trials | A's answer's readout | Inhibitory | −51.9 | −238.1 | −358 | −153.3 | −232.6 | −52 |
| After block 6 alone | B's trials | A's answer's readout | All excitatory | −1 421.7 | −892.3 | 37 | −1 431.4 | −960.7 | 33 |
| After block 6 alone | B's trials | A's answer's readout | Excitatory less inhibitory | −1 369.7 | −654.2 | 52 | −1 278.1 | −728.1 | 43 |
| After block 6 alone | B's trials | B's answer's readout | A's kinds | −25.1 | +3.2 | 113 | −34.9 | −23.9 | 31 |
| After block 6 alone | B's trials | B's answer's readout | B's kinds | +314.6 | +248.9 | 21 | +275.2 | +246.9 | 10 |
| After block 6 alone | B's trials | B's answer's readout | Inhibitory | −80.3 | −125.9 | −57 | −7.3 | −34.0 | −367 |
| After block 6 alone | B's trials | B's answer's readout | All excitatory | −1 051.8 | −752.0 | 29 | −935.0 | −414.8 | 56 |
| After block 6 alone | B's trials | B's answer's readout | Excitatory less inhibitory | −971.5 | −626.1 | 36 | −927.7 | −380.8 | 59 |
| After block 12 alone | A's trials | A's answer's readout | A's kinds | +316.6 | +377.9 | −19 | +412.1 | +368.5 | 11 |
| After block 12 alone | A's trials | A's answer's readout | B's kinds | −95.8 | −13.5 | 86 | −25.6 | +14.7 | 157 |
| After block 12 alone | A's trials | A's answer's readout | Inhibitory | −342.1 | −149.3 | 56 | −267.9 | −199.2 | 26 |
| After block 12 alone | A's trials | A's answer's readout | All excitatory | −1 689.9 | −637.6 | 62 | −1 460.5 | −480.9 | 67 |
| After block 12 alone | A's trials | A's answer's readout | Excitatory less inhibitory | −1 347.8 | −488.3 | 64 | −1 192.6 | −281.7 | 76 |
| After block 12 alone | A's trials | B's answer's readout | A's kinds | −307.4 | −378.4 | −23 | −399.8 | −369.0 | 8 |
| After block 12 alone | A's trials | B's answer's readout | B's kinds | −60.1 | +21.8 | 136 | −40.5 | +18.6 | 146 |
| After block 12 alone | A's trials | B's answer's readout | Inhibitory | −618.8 | −418.9 | 32 | −618.2 | −281.0 | 55 |
| After block 12 alone | A's trials | B's answer's readout | All excitatory | −2 000.4 | −976.6 | 51 | −2 024.5 | −1 180.1 | 42 |
| After block 12 alone | A's trials | B's answer's readout | Excitatory less inhibitory | −1 381.6 | −557.7 | 60 | −1 406.3 | −899.1 | 36 |
| After block 12 alone | B's trials | A's answer's readout | A's kinds | −16.1 | +5.3 | 133 | −27.5 | −4.7 | 83 |
| After block 12 alone | B's trials | A's answer's readout | B's kinds | −265.8 | −246.4 | 7 | −254.2 | −214.9 | 15 |
| After block 12 alone | B's trials | A's answer's readout | Inhibitory | −118.2 | −208.2 | −76 | −217.9 | −171.0 | 22 |
| After block 12 alone | B's trials | A's answer's readout | All excitatory | −1 456.8 | −661.4 | 55 | −1 580.3 | −709.7 | 55 |
| After block 12 alone | B's trials | A's answer's readout | Excitatory less inhibitory | −1 338.6 | −453.2 | 66 | −1 362.4 | −538.7 | 60 |
| After block 12 alone | B's trials | B's answer's readout | A's kinds | −21.8 | −1.3 | 94 | −34.7 | −21.0 | 39 |
| After block 12 alone | B's trials | B's answer's readout | B's kinds | +279.5 | +243.6 | 13 | +271.1 | +212.5 | 22 |
| After block 12 alone | B's trials | B's answer's readout | Inhibitory | −198.7 | −45.2 | 77 | −106.1 | −58.1 | 45 |
| After block 12 alone | B's trials | B's answer's readout | All excitatory | −1 156.2 | −559.7 | 52 | −1 053.1 | −229.3 | 78 |
| After block 12 alone | B's trials | B's answer's readout | Excitatory less inhibitory | −957.5 | −514.5 | 46 | −947.0 | −171.3 | 82 |
| After block 18 alone | A's trials | A's answer's readout | A's kinds | +303.5 | +302.2 | 0 | +401.0 | +389.6 | 3 |
| After block 18 alone | A's trials | A's answer's readout | B's kinds | −76.4 | −31.9 | 58 | −26.8 | +1.0 | 104 |
| After block 18 alone | A's trials | A's answer's readout | Inhibitory | −534.0 | −224.8 | 58 | −222.9 | −141.3 | 37 |
| After block 18 alone | A's trials | A's answer's readout | All excitatory | −1 717.5 | −798.3 | 54 | −1 364.1 | −487.3 | 64 |
| After block 18 alone | A's trials | A's answer's readout | Excitatory less inhibitory | −1 183.5 | −573.6 | 52 | −1 141.2 | −345.9 | 70 |
| After block 18 alone | A's trials | B's answer's readout | A's kinds | −293.5 | −303.9 | −4 | −391.1 | −391.7 | 0 |
| After block 18 alone | A's trials | B's answer's readout | B's kinds | −43.7 | −2.0 | 95 | −27.3 | −4.0 | 85 |
| After block 18 alone | A's trials | B's answer's readout | Inhibitory | −791.9 | −532.7 | 33 | −633.3 | −316.9 | 50 |
| After block 18 alone | A's trials | B's answer's readout | All excitatory | −1 972.6 | −1 067.2 | 46 | −1 962.6 | −1 179.1 | 40 |
| After block 18 alone | A's trials | B's answer's readout | Excitatory less inhibitory | −1 180.6 | −534.5 | 55 | −1 329.3 | −862.2 | 35 |
| After block 18 alone | B's trials | A's answer's readout | A's kinds | −23.7 | +2.9 | 112 | −33.7 | −5.5 | 84 |
| After block 18 alone | B's trials | A's answer's readout | B's kinds | −257.4 | −238.1 | 7 | −256.3 | −250.9 | 2 |
| After block 18 alone | B's trials | A's answer's readout | Inhibitory | −154.0 | −152.3 | 1 | −192.8 | −159.2 | 17 |
| After block 18 alone | B's trials | A's answer's readout | All excitatory | −1 470.8 | −749.1 | 49 | −1 548.1 | −726.7 | 53 |
| After block 18 alone | B's trials | A's answer's readout | Excitatory less inhibitory | −1 316.7 | −596.9 | 55 | −1 355.3 | −567.5 | 58 |
| After block 18 alone | B's trials | B's answer's readout | A's kinds | −29.9 | +0.3 | 101 | −42.9 | −20.3 | 53 |
| After block 18 alone | B's trials | B's answer's readout | B's kinds | +270.9 | +236.8 | 13 | +272.1 | +248.7 | 9 |
| After block 18 alone | B's trials | B's answer's readout | Inhibitory | −230.6 | −139.3 | 40 | −94.2 | −53.4 | 43 |
| After block 18 alone | B's trials | B's answer's readout | All excitatory | −1 267.5 | −651.9 | 49 | −1 026.3 | −151.2 | 85 |
| After block 18 alone | B's trials | B's answer's readout | Excitatory less inhibitory | −1 036.9 | −512.5 | 51 | −932.2 | −97.7 | 90 |
| After block 24 alone | A's trials | A's answer's readout | A's kinds | +254.3 | +274.2 | −8 | +349.5 | +300.2 | 14 |
| After block 24 alone | A's trials | A's answer's readout | B's kinds | −87.7 | −75.3 | 14 | −18.1 | +10.4 | 157 |
| After block 24 alone | A's trials | A's answer's readout | Inhibitory | −454.4 | −102.9 | 77 | −245.3 | +2.3 | 101 |
| After block 24 alone | A's trials | A's answer's readout | All excitatory | −1 755.8 | −874.3 | 50 | −1 468.7 | −674.1 | 54 |
| After block 24 alone | A's trials | A's answer's readout | Excitatory less inhibitory | −1 301.4 | −771.4 | 41 | −1 223.4 | −676.4 | 45 |
| After block 24 alone | A's trials | B's answer's readout | A's kinds | −236.7 | −277.5 | −17 | −338.5 | −308.2 | 9 |
| After block 24 alone | A's trials | B's answer's readout | B's kinds | −54.6 | −33.8 | 38 | +14.4 | +22.8 | −58 |
| After block 24 alone | A's trials | B's answer's readout | Inhibitory | −722.3 | −376.4 | 48 | −560.6 | −134.9 | 76 |
| After block 24 alone | A's trials | B's answer's readout | All excitatory | −1 689.8 | −963.8 | 43 | −1 898.1 | −1 046.5 | 45 |
| After block 24 alone | A's trials | B's answer's readout | Excitatory less inhibitory | −967.5 | −587.4 | 39 | −1 337.5 | −911.5 | 32 |
| After block 24 alone | B's trials | A's answer's readout | A's kinds | −22.1 | +0.1 | 101 | −35.2 | −1.8 | 95 |
| After block 24 alone | B's trials | A's answer's readout | B's kinds | −313.9 | −356.3 | −14 | −345.0 | −301.3 | 13 |
| After block 24 alone | B's trials | A's answer's readout | Inhibitory | −165.5 | −128.6 | 22 | −258.7 | −86.9 | 66 |
| After block 24 alone | B's trials | A's answer's readout | All excitatory | −1 740.0 | −1 168.7 | 33 | −1 773.7 | −1 022.9 | 42 |
| After block 24 alone | B's trials | A's answer's readout | Excitatory less inhibitory | −1 574.5 | −1 040.2 | 34 | −1 515.1 | −936.0 | 38 |
| After block 24 alone | B's trials | B's answer's readout | A's kinds | −30.6 | −6.1 | 80 | −50.9 | −14.6 | 71 |
| After block 24 alone | B's trials | B's answer's readout | B's kinds | +324.9 | +356.7 | −10 | +354.7 | +297.7 | 16 |
| After block 24 alone | B's trials | B's answer's readout | Inhibitory | −264.2 | −134.6 | 49 | −205.5 | +22.0 | 111 |
| After block 24 alone | B's trials | B's answer's readout | All excitatory | −1 381.2 | −743.3 | 46 | −1 123.1 | −459.9 | 59 |
| After block 24 alone | B's trials | B's answer's readout | Excitatory less inhibitory | −1 117.0 | −608.7 | 46 | −917.6 | −481.9 | 47 |

Excitatory less inhibitory came back by 34 to 66 per cent on the image from the assignment and 32 to 90
on the image from the mirrored assignment, the excitatory drive by 29 to 62 and 33 to 85; the inhibitory drive's moved by
−358 to +77 and −367 to +111 per cent, a negative per cent a change that grew; A's kinds' drive onto each readout
on A's trials by −23 to +15 and 0 to +14. Span by span, excitatory less inhibitory, the window's fall came back
by 30 to 83 per cent on the image from the assignment and 37 to 71 on the image from the mirrored
assignment, and the volley's change by −35 to +28 and −8 to +22.

**H-47's plain night on all four nights**:

| Comparison | Onto | From | From the assignment: the window, A's trials, k | B's trials | the volley, A's trials | B's trials | From the mirrored assignment: the window, A's trials | B's trials | the volley, A's trials | B's trials |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Plain on all four nights less without replay, block 6 | A's answer's readout | A's kinds | +0.1 ± 0.1 | −18.3 ± 2.8 | +350.4 ± 6.0 | −2.8 ± 1.3 | 0.0 ± 0.0 | −24.9 ± 4.4 | +342.7 ± 5.7 | −2.9 ± 1.7 |
| Plain on all four nights less without replay, block 6 | A's answer's readout | B's kinds | −56.7 ± 7.7 | 0.0 ± 0.2 | −4.9 ± 2.0 | −307.9 ± 6.7 | −25.0 ± 6.7 | 0.0 ± 0.0 | −4.3 ± 1.9 | −261.9 ± 4.5 |
| Plain on all four nights less without replay, block 6 | A's answer's readout | The odd places' excitatory | −990.5 ± 22.6 | −464.0 ± 13.1 | −17.4 ± 3.4 | −24.4 ± 2.8 | −822.1 ± 21.4 | −464.2 ± 14.6 | −30.8 ± 4.3 | −32.4 ± 3.8 |
| Plain on all four nights less without replay, block 6 | A's answer's readout | The even places' excitatory | −687.2 ± 15.9 | −578.9 ± 17.6 | −24.1 ± 3.0 | −23.0 ± 4.0 | −888.2 ± 24.0 | −622.3 ± 17.0 | −30.1 ± 4.1 | −18.8 ± 4.8 |
| Plain on all four nights less without replay, block 6 | A's answer's readout | Other excitatory | −11.5 ± 1.4 | −1.7 ± 0.8 | −0.5 ± 0.3 | −0.7 ± 0.4 | −8.2 ± 1.3 | −4.0 ± 1.3 | −0.9 ± 0.4 | 0.0 ± 0.7 |
| Plain on all four nights less without replay, block 6 | A's answer's readout | Inhibitory | −163.2 ± 19.3 | −43.5 ± 22.9 | −14.1 ± 6.2 | −8.4 ± 4.6 | −191.5 ± 17.5 | −139.0 ± 29.0 | −23.4 ± 5.9 | −14.2 ± 5.1 |
| Plain on all four nights less without replay, block 6 | A's answer's readout | All excitatory | −1 745.8 ± 34.9 | −1 062.9 ± 22.0 | +303.6 ± 8.0 | −358.8 ± 7.4 | −1 743.6 ± 28.2 | −1 115.4 ± 21.0 | +276.7 ± 6.2 | −316.0 ± 7.0 |
| Plain on all four nights less without replay, block 6 | A's answer's readout | Excitatory less inhibitory | −1 582.7 ± 36.4 | −1 019.4 ± 21.9 | +317.7 ± 9.1 | −350.3 ± 8.5 | −1 552.1 ± 36.3 | −976.4 ± 25.0 | +300.0 ± 8.1 | −301.7 ± 9.7 |
| Plain on all four nights less without replay, block 6 | B's answer's readout | A's kinds | 0.0 ± 0.0 | −22.5 ± 2.1 | −335.5 ± 5.8 | −2.5 ± 0.9 | 0.0 ± 0.0 | −31.3 ± 5.1 | −327.7 ± 5.1 | −3.6 ± 1.4 |
| Plain on all four nights less without replay, block 6 | B's answer's readout | B's kinds | −34.2 ± 6.4 | −0.1 ± 0.4 | −5.1 ± 1.9 | +314.7 ± 6.6 | −32.9 ± 7.8 | 0.0 ± 0.0 | −5.9 ± 1.6 | +275.2 ± 4.4 |
| Plain on all four nights less without replay, block 6 | B's answer's readout | The odd places' excitatory | −711.5 ± 21.7 | −537.8 ± 16.7 | −20.8 ± 4.1 | −22.5 ± 3.5 | −735.8 ± 21.7 | −632.5 ± 15.6 | −29.9 ± 3.8 | −29.3 ± 4.9 |
| Plain on all four nights less without replay, block 6 | B's answer's readout | The even places' excitatory | −894.4 ± 15.6 | −754.2 ± 19.6 | −26.4 ± 3.3 | −24.6 ± 4.8 | −532.0 ± 16.0 | −495.7 ± 18.7 | −24.5 ± 4.1 | −14.5 ± 3.8 |
| Plain on all four nights less without replay, block 6 | B's answer's readout | Other excitatory | −7.9 ± 1.0 | −1.7 ± 1.0 | −0.4 ± 0.3 | −0.5 ± 0.4 | −12.4 ± 1.8 | −3.0 ± 1.1 | −0.8 ± 0.3 | −0.3 ± 0.6 |
| Plain on all four nights less without replay, block 6 | B's answer's readout | Inhibitory | −482.5 ± 17.9 | −72.0 ± 26.4 | −17.7 ± 5.2 | −8.3 ± 3.7 | −453.3 ± 24.3 | +9.9 ± 33.3 | −23.7 ± 5.5 | −17.2 ± 4.7 |
| Plain on all four nights less without replay, block 6 | B's answer's readout | All excitatory | −1 648.0 ± 29.9 | −1 316.4 ± 25.7 | −388.2 ± 8.2 | +264.6 ± 10.3 | −1 313.2 ± 23.3 | −1 162.5 ± 21.9 | −388.8 ± 7.5 | +227.6 ± 7.2 |
| Plain on all four nights less without replay, block 6 | B's answer's readout | Excitatory less inhibitory | −1 165.4 ± 24.9 | −1 244.4 ± 25.8 | −370.6 ± 9.3 | +272.9 ± 9.7 | −859.9 ± 34.7 | −1 172.4 ± 26.8 | −365.0 ± 8.6 | +244.7 ± 10.2 |
| Plain on all four nights less without replay, block 12 | A's answer's readout | A's kinds | −0.3 ± 0.4 | −4.1 ± 1.9 | +569.4 ± 24.1 | −0.4 ± 1.2 | −0.6 ± 0.4 | −14.6 ± 3.7 | +653.3 ± 5.5 | +0.6 ± 1.4 |
| Plain on all four nights less without replay, block 12 | A's answer's readout | B's kinds | −69.0 ± 5.7 | −1.0 ± 1.0 | −4.5 ± 1.3 | −563.7 ± 28.6 | −19.1 ± 5.6 | −0.3 ± 0.2 | −6.3 ± 2.1 | −464.3 ± 14.4 |
| Plain on all four nights less without replay, block 12 | A's answer's readout | The odd places' excitatory | −1 507.2 ± 23.6 | −677.9 ± 21.3 | −25.1 ± 3.5 | −24.2 ± 4.0 | −1 041.6 ± 26.5 | −625.8 ± 20.4 | −24.8 ± 3.3 | −19.5 ± 2.9 |
| Plain on all four nights less without replay, block 12 | A's answer's readout | The even places' excitatory | −896.1 ± 31.6 | −793.4 ± 30.4 | −17.2 ± 3.7 | −20.2 ± 2.5 | −1 334.8 ± 20.8 | −923.0 ± 17.5 | −20.9 ± 2.8 | −23.4 ± 2.5 |
| Plain on all four nights less without replay, block 12 | A's answer's readout | Other excitatory | −15.0 ± 3.1 | −2.8 ± 0.9 | −0.1 ± 0.1 | −0.5 ± 0.3 | −12.9 ± 1.9 | −7.1 ± 1.4 | −0.1 ± 0.1 | −0.3 ± 0.1 |
| Plain on all four nights less without replay, block 12 | A's answer's readout | Inhibitory | −193.2 ± 19.3 | −121.4 ± 16.4 | −9.4 ± 3.8 | −12.0 ± 4.4 | −129.0 ± 29.2 | −55.7 ± 24.9 | −17.0 ± 3.8 | −12.6 ± 5.1 |
| Plain on all four nights less without replay, block 12 | A's answer's readout | All excitatory | −2 487.6 ± 37.8 | −1 479.2 ± 25.9 | +522.4 ± 24.8 | −609.0 ± 29.0 | −2 409.0 ± 34.6 | −1 570.8 ± 30.8 | +601.0 ± 7.4 | −507.0 ± 15.3 |
| Plain on all four nights less without replay, block 12 | A's answer's readout | Excitatory less inhibitory | −2 294.4 ± 37.6 | −1 357.8 ± 31.3 | +531.9 ± 24.8 | −597.0 ± 29.1 | −2 280.0 ± 40.9 | −1 515.1 ± 34.0 | +618.0 ± 7.3 | −494.4 ± 16.1 |
| Plain on all four nights less without replay, block 12 | B's answer's readout | A's kinds | −0.1 ± 0.2 | −15.0 ± 2.6 | −558.9 ± 23.9 | −1.4 ± 1.2 | −0.6 ± 0.3 | −27.5 ± 3.7 | −638.8 ± 5.5 | −0.9 ± 1.0 |
| Plain on all four nights less without replay, block 12 | B's answer's readout | B's kinds | −37.2 ± 4.6 | −0.2 ± 0.2 | −4.3 ± 1.3 | +570.8 ± 28.8 | −34.1 ± 4.0 | −0.3 ± 0.2 | −4.9 ± 1.8 | +474.8 ± 14.8 |
| Plain on all four nights less without replay, block 12 | B's answer's readout | The odd places' excitatory | −884.1 ± 20.7 | −778.2 ± 21.5 | −23.2 ± 4.0 | −20.0 ± 2.9 | −1 033.3 ± 20.5 | −924.1 ± 19.2 | −28.0 ± 3.1 | −22.8 ± 3.3 |
| Plain on all four nights less without replay, block 12 | B's answer's readout | The even places' excitatory | −1 223.0 ± 32.4 | −1 186.1 ± 30.9 | −19.7 ± 2.8 | −18.6 ± 2.4 | −663.8 ± 13.7 | −674.0 ± 19.7 | −17.5 ± 3.4 | −22.0 ± 2.5 |
| Plain on all four nights less without replay, block 12 | B's answer's readout | Other excitatory | −9.8 ± 2.0 | −2.5 ± 1.0 | −0.1 ± 0.0 | −0.4 ± 0.2 | −20.3 ± 3.3 | −7.9 ± 1.4 | −0.3 ± 0.1 | −0.4 ± 0.2 |
| Plain on all four nights less without replay, block 12 | B's answer's readout | Inhibitory | −709.2 ± 26.1 | −104.5 ± 26.6 | −12.1 ± 4.3 | −11.3 ± 5.2 | −605.9 ± 25.4 | +193.2 ± 34.2 | −12.7 ± 3.1 | −13.1 ± 5.4 |
| Plain on all four nights less without replay, block 12 | B's answer's readout | All excitatory | −2 154.2 ± 38.6 | −1 982.0 ± 29.9 | −606.3 ± 24.3 | +530.4 ± 29.0 | −1 752.1 ± 26.2 | −1 633.9 ± 31.2 | −689.5 ± 6.9 | +428.6 ± 15.4 |
| Plain on all four nights less without replay, block 12 | B's answer's readout | Excitatory less inhibitory | −1 445.0 ± 35.3 | −1 877.5 ± 49.0 | −594.1 ± 24.5 | +541.7 ± 30.0 | −1 146.2 ± 28.4 | −1 827.1 ± 40.5 | −676.8 ± 7.6 | +441.7 ± 16.4 |
| Plain on all four nights less without replay, block 18 | A's answer's readout | A's kinds | +0.2 ± 0.2 | −13.0 ± 3.4 | +746.4 ± 14.0 | −3.0 ± 2.1 | 0.0 ± 0.0 | −17.2 ± 4.4 | +943.5 ± 10.3 | −1.8 ± 2.0 |
| Plain on all four nights less without replay, block 18 | A's answer's readout | B's kinds | −113.8 ± 7.2 | 0.0 ± 0.0 | −5.3 ± 3.0 | −713.2 ± 21.7 | +1.3 ± 4.1 | 0.0 ± 0.0 | −1.1 ± 1.7 | −732.2 ± 30.6 |
| Plain on all four nights less without replay, block 18 | A's answer's readout | The odd places' excitatory | −1 869.0 ± 24.6 | −886.5 ± 45.0 | −25.7 ± 4.5 | −26.0 ± 4.5 | −1 086.1 ± 29.2 | −880.8 ± 32.2 | −26.9 ± 3.9 | −18.7 ± 2.6 |
| Plain on all four nights less without replay, block 18 | A's answer's readout | The even places' excitatory | −1 197.9 ± 14.0 | −994.3 ± 32.4 | −10.2 ± 5.3 | −17.1 ± 4.7 | −1 624.6 ± 22.7 | −1 102.8 ± 16.7 | −18.0 ± 3.8 | −20.9 ± 4.2 |
| Plain on all four nights less without replay, block 18 | A's answer's readout | Other excitatory | −20.0 ± 4.2 | −4.5 ± 0.7 | −0.6 ± 0.4 | −0.9 ± 0.3 | −22.7 ± 2.0 | −5.7 ± 1.2 | −0.2 ± 0.1 | −0.8 ± 0.3 |
| Plain on all four nights less without replay, block 18 | A's answer's readout | Inhibitory | −493.5 ± 37.9 | −214.4 ± 42.7 | −17.5 ± 6.9 | −17.1 ± 5.2 | −85.4 ± 23.3 | +12.8 ± 35.1 | −7.7 ± 3.6 | −23.0 ± 5.8 |
| Plain on all four nights less without replay, block 18 | A's answer's readout | All excitatory | −3 200.6 ± 27.8 | −1 898.3 ± 58.8 | +704.6 ± 13.5 | −760.2 ± 21.6 | −2 732.1 ± 46.6 | −2 006.4 ± 38.9 | +897.3 ± 9.8 | −774.4 ± 30.8 |
| Plain on all four nights less without replay, block 18 | A's answer's readout | Excitatory less inhibitory | −2 707.1 ± 38.0 | −1 683.9 ± 40.4 | +722.1 ± 14.2 | −743.0 ± 23.4 | −2 646.7 ± 50.5 | −2 019.2 ± 34.1 | +905.0 ± 10.7 | −751.4 ± 30.7 |
| Plain on all four nights less without replay, block 18 | B's answer's readout | A's kinds | +0.1 ± 0.1 | −26.1 ± 3.9 | −742.1 ± 14.5 | −2.4 ± 1.8 | 0.0 ± 0.0 | −36.9 ± 3.2 | −932.4 ± 9.0 | −4.2 ± 2.0 |
| Plain on all four nights less without replay, block 18 | B's answer's readout | B's kinds | −55.7 ± 6.9 | 0.0 ± 0.0 | −1.4 ± 1.9 | +717.5 ± 21.5 | +19.7 ± 5.3 | 0.0 ± 0.0 | −1.1 ± 2.5 | +742.6 ± 30.1 |
| Plain on all four nights less without replay, block 18 | B's answer's readout | The odd places' excitatory | −1 041.7 ± 24.7 | −1 027.1 ± 25.6 | −21.9 ± 4.4 | −14.5 ± 5.1 | −1 210.2 ± 21.8 | −1 158.8 ± 23.5 | −28.5 ± 3.2 | −24.5 ± 3.3 |
| Plain on all four nights less without replay, block 18 | B's answer's readout | The even places' excitatory | −1 492.7 ± 25.5 | −1 589.8 ± 35.2 | −14.4 ± 4.3 | −18.7 ± 3.9 | −836.0 ± 32.1 | −833.0 ± 19.5 | −11.8 ± 4.2 | −16.7 ± 3.8 |
| Plain on all four nights less without replay, block 18 | B's answer's readout | Other excitatory | −12.1 ± 2.3 | −3.7 ± 0.6 | −0.3 ± 0.2 | −0.7 ± 0.3 | −40.4 ± 4.4 | −7.9 ± 1.6 | −0.5 ± 0.3 | −1.0 ± 0.3 |
| Plain on all four nights less without replay, block 18 | B's answer's readout | Inhibitory | −1 076.8 ± 20.8 | −166.9 ± 64.5 | −12.8 ± 7.3 | −16.7 ± 6.0 | −782.6 ± 17.6 | +351.2 ± 36.9 | −9.5 ± 2.8 | −19.4 ± 3.8 |
| Plain on all four nights less without replay, block 18 | B's answer's readout | All excitatory | −2 602.2 ± 43.4 | −2 646.7 ± 35.9 | −780.2 ± 17.5 | +681.3 ± 22.4 | −2 067.0 ± 37.4 | −2 036.5 ± 35.1 | −974.3 ± 11.8 | +696.2 ± 31.4 |
| Plain on all four nights less without replay, block 18 | B's answer's readout | Excitatory less inhibitory | −1 525.4 ± 43.9 | −2 479.8 ± 57.5 | −767.3 ± 18.7 | +698.0 ± 23.2 | −1 284.3 ± 29.0 | −2 387.7 ± 29.6 | −964.9 ± 12.3 | +715.6 ± 32.4 |
| Plain on all four nights less without replay, block 24 | A's answer's readout | A's kinds | 0.0 ± 0.0 | −9.3 ± 3.4 | +970.5 ± 24.3 | −1.6 ± 1.5 | 0.0 ± 0.2 | −22.8 ± 4.3 | +1 182.6 ± 20.3 | −3.6 ± 1.3 |
| Plain on all four nights less without replay, block 24 | A's answer's readout | B's kinds | −102.7 ± 6.2 | +0.1 ± 0.2 | −4.4 ± 1.6 | −863.0 ± 15.8 | +13.4 ± 6.7 | 0.0 ± 0.0 | −3.1 ± 2.0 | −856.4 ± 33.9 |
| Plain on all four nights less without replay, block 24 | A's answer's readout | The odd places' excitatory | −2 184.1 ± 24.2 | −995.4 ± 50.1 | −16.6 ± 3.0 | −28.1 ± 5.0 | −1 203.7 ± 25.7 | −1 151.1 ± 44.7 | −14.4 ± 5.3 | −20.3 ± 4.3 |
| Plain on all four nights less without replay, block 24 | A's answer's readout | The even places' excitatory | −1 371.5 ± 21.2 | −1 208.7 ± 15.8 | −18.7 ± 3.3 | −12.5 ± 3.2 | −1 834.6 ± 33.7 | −1 321.3 ± 21.9 | −27.2 ± 4.3 | −20.0 ± 5.8 |
| Plain on all four nights less without replay, block 24 | A's answer's readout | Other excitatory | −13.2 ± 3.0 | −4.8 ± 0.9 | −0.6 ± 0.3 | −0.7 ± 0.5 | −20.6 ± 1.4 | −9.8 ± 1.5 | −0.3 ± 0.4 | −0.3 ± 0.3 |
| Plain on all four nights less without replay, block 24 | A's answer's readout | Inhibitory | −700.8 ± 32.8 | −391.6 ± 44.7 | −6.7 ± 5.3 | −11.3 ± 4.3 | −104.7 ± 37.3 | −32.1 ± 42.7 | −0.2 ± 6.6 | −16.5 ± 5.4 |
| Plain on all four nights less without replay, block 24 | A's answer's readout | All excitatory | −3 671.5 ± 32.9 | −2 218.3 ± 49.6 | +930.3 ± 25.1 | −905.9 ± 19.0 | −3 045.6 ± 43.3 | −2 505.0 ± 56.2 | +1 137.6 ± 20.9 | −900.5 ± 35.5 |
| Plain on all four nights less without replay, block 24 | A's answer's readout | Excitatory less inhibitory | −2 970.7 ± 45.5 | −1 826.6 ± 27.9 | +937.0 ± 26.0 | −894.6 ± 15.7 | −2 940.8 ± 62.0 | −2 472.9 ± 52.8 | +1 137.7 ± 22.4 | −884.1 ± 33.9 |
| Plain on all four nights less without replay, block 24 | B's answer's readout | A's kinds | 0.0 ± 0.0 | −24.5 ± 3.6 | −961.4 ± 24.7 | −3.6 ± 1.7 | −0.1 ± 0.3 | −51.4 ± 3.8 | −1 177.6 ± 20.0 | −4.8 ± 1.4 |
| Plain on all four nights less without replay, block 24 | B's answer's readout | B's kinds | −37.2 ± 7.0 | +0.4 ± 0.2 | −1.3 ± 1.4 | +864.0 ± 16.2 | +38.8 ± 9.8 | +0.1 ± 0.1 | −1.5 ± 2.0 | +858.2 ± 33.8 |
| Plain on all four nights less without replay, block 24 | B's answer's readout | The odd places' excitatory | −1 090.3 ± 17.1 | −1 283.0 ± 30.7 | −13.7 ± 3.6 | −27.2 ± 6.1 | −1 420.4 ± 24.6 | −1 344.6 ± 31.3 | −12.2 ± 5.3 | −24.4 ± 4.4 |
| Plain on all four nights less without replay, block 24 | B's answer's readout | The even places' excitatory | −1 568.6 ± 40.5 | −1 896.0 ± 22.6 | −17.9 ± 4.2 | −18.4 ± 3.1 | −1 065.5 ± 58.0 | −1 076.5 ± 27.4 | −21.1 ± 5.2 | −11.4 ± 5.1 |
| Plain on all four nights less without replay, block 24 | B's answer's readout | Other excitatory | −6.4 ± 1.4 | −2.5 ± 0.7 | −0.3 ± 0.3 | −0.5 ± 0.3 | −45.0 ± 2.7 | −15.6 ± 1.8 | −0.6 ± 0.4 | −0.5 ± 0.6 |
| Plain on all four nights less without replay, block 24 | B's answer's readout | Inhibitory | −1 409.7 ± 14.2 | −334.1 ± 74.5 | −11.4 ± 3.2 | −14.9 ± 5.0 | −956.1 ± 37.3 | +354.9 ± 43.3 | −9.5 ± 6.1 | −16.0 ± 4.7 |
| Plain on all four nights less without replay, block 24 | B's answer's readout | All excitatory | −2 702.5 ± 51.1 | −3 205.6 ± 45.7 | −994.5 ± 26.2 | +814.3 ± 18.1 | −2 492.2 ± 61.0 | −2 488.0 ± 55.8 | −1 213.0 ± 22.6 | +817.1 ± 33.7 |
| Plain on all four nights less without replay, block 24 | B's answer's readout | Excitatory less inhibitory | −1 292.8 ± 47.6 | −2 871.5 ± 59.1 | −983.1 ± 25.9 | +829.3 ± 20.4 | −1 536.1 ± 52.4 | −2 842.9 ± 58.8 | −1 203.6 ± 22.0 | +833.1 ± 34.0 |
| Plain on all four nights less without replay, block 36 | A's answer's readout | A's kinds | +0.2 ± 0.1 | +13.0 ± 2.7 | +1 009.2 ± 31.2 | +2.3 ± 0.4 | −0.1 ± 0.1 | +0.7 ± 5.1 | +1 187.7 ± 16.5 | +2.3 ± 1.5 |
| Plain on all four nights less without replay, block 36 | A's answer's readout | B's kinds | −81.5 ± 15.4 | −0.1 ± 0.2 | −0.1 ± 0.2 | −920.4 ± 56.0 | +25.4 ± 3.9 | −0.1 ± 0.2 | −0.7 ± 0.7 | −887.7 ± 27.2 |
| Plain on all four nights less without replay, block 36 | A's answer's readout | The odd places' excitatory | −1 932.4 ± 65.7 | −740.0 ± 18.5 | −2.5 ± 2.3 | −3.7 ± 3.7 | −902.3 ± 27.6 | −946.4 ± 31.1 | +5.3 ± 3.1 | −0.1 ± 3.1 |
| Plain on all four nights less without replay, block 36 | A's answer's readout | The even places' excitatory | −984.9 ± 35.9 | −999.7 ± 3.8 | −0.7 ± 2.3 | +2.0 ± 1.9 | −1 605.8 ± 19.4 | −1 123.7 ± 27.6 | −12.1 ± 2.0 | −9.0 ± 1.2 |
| Plain on all four nights less without replay, block 36 | A's answer's readout | Other excitatory | −11.0 ± 1.2 | −2.2 ± 0.7 | −0.1 ± 0.3 | 0.0 ± 0.0 | −15.1 ± 1.2 | −6.2 ± 1.1 | −0.2 ± 0.2 | 0.0 ± 0.1 |
| Plain on all four nights less without replay, block 36 | A's answer's readout | Inhibitory | −430.1 ± 36.5 | −456.7 ± 51.1 | +10.7 ± 1.9 | +3.4 ± 4.7 | +40.8 ± 48.0 | −148.0 ± 31.6 | +15.9 ± 5.2 | +11.5 ± 5.2 |
| Plain on all four nights less without replay, block 36 | A's answer's readout | All excitatory | −3 009.6 ± 109.9 | −1 728.9 ± 19.5 | +1 005.8 ± 30.4 | −919.8 ± 55.2 | −2 497.9 ± 18.9 | −2 075.8 ± 57.8 | +1 180.0 ± 18.4 | −894.6 ± 29.3 |
| Plain on all four nights less without replay, block 36 | A's answer's readout | Excitatory less inhibitory | −2 579.4 ± 89.2 | −1 272.3 ± 69.8 | +995.1 ± 32.1 | −923.3 ± 53.9 | −2 538.6 ± 52.5 | −1 927.7 ± 60.2 | +1 164.1 ± 23.3 | −906.0 ± 33.9 |
| Plain on all four nights less without replay, block 36 | B's answer's readout | A's kinds | 0.0 ± 0.1 | −7.8 ± 4.3 | −1 016.2 ± 32.0 | −0.4 ± 1.8 | −0.1 ± 0.1 | −27.1 ± 5.8 | −1 199.6 ± 17.6 | −0.5 ± 1.3 |
| Plain on all four nights less without replay, block 36 | B's answer's readout | B's kinds | −15.4 ± 15.9 | −0.3 ± 0.1 | +2.3 ± 0.8 | +915.3 ± 56.1 | +48.3 ± 7.6 | +0.2 ± 0.2 | +0.4 ± 1.4 | +883.1 ± 27.4 |
| Plain on all four nights less without replay, block 36 | B's answer's readout | The odd places' excitatory | −906.6 ± 24.6 | −1 044.0 ± 27.8 | −0.9 ± 0.5 | −1.3 ± 1.7 | −1 145.5 ± 21.5 | −1 163.3 ± 24.5 | −1.5 ± 0.4 | −4.1 ± 3.4 |
| Plain on all four nights less without replay, block 36 | B's answer's readout | The even places' excitatory | −1 178.1 ± 57.2 | −1 703.1 ± 11.1 | −7.7 ± 3.1 | −2.4 ± 2.9 | −787.9 ± 38.8 | −855.2 ± 26.3 | −0.3 ± 2.3 | −1.0 ± 1.0 |
| Plain on all four nights less without replay, block 36 | B's answer's readout | Other excitatory | −4.8 ± 0.7 | −0.7 ± 0.6 | 0.0 ± 0.2 | +0.2 ± 0.1 | −35.2 ± 2.8 | −10.0 ± 1.3 | −0.4 ± 0.2 | −0.2 ± 0.1 |
| Plain on all four nights less without replay, block 36 | B's answer's readout | Inhibitory | −1 221.0 ± 53.2 | −336.8 ± 33.1 | +9.8 ± 4.3 | +3.8 ± 4.0 | −702.2 ± 74.7 | +347.5 ± 29.5 | +10.1 ± 4.8 | +9.0 ± 3.2 |
| Plain on all four nights less without replay, block 36 | B's answer's readout | All excitatory | −2 104.8 ± 87.3 | −2 756.0 ± 35.9 | −1 022.5 ± 32.7 | +911.3 ± 51.2 | −1 920.5 ± 31.0 | −2 055.5 ± 51.9 | −1 201.4 ± 18.2 | +877.2 ± 30.7 |
| Plain on all four nights less without replay, block 36 | B's answer's readout | Excitatory less inhibitory | −883.8 ± 55.0 | −2 419.1 ± 61.5 | −1 032.3 ± 34.2 | +907.5 ± 52.3 | −1 218.3 ± 52.4 | −2 403.1 ± 48.7 | −1 211.5 ± 13.6 | +868.3 ± 28.1 |

Excitatory less inhibitory in the window lies below beyond 2 SE in 20 of 20 on the image from the assignment and
20 of 20 on the image from the mirrored assignment, and the trial's stimulus's kinds' volley drive above onto its answer's readout in
10 of 10 and 10 of 10.

## 4. The two cells side by side

**The answer's readout less the other**, on A's trials A's answer's readout less B's, on B's trials B's answer's less A's, less the night
without replay, beside the spikes:

| Comparison | What | Span | From the assignment: on A's trials, A's answer's readout less B's | on B's trials, B's answer's less A's | From the mirrored assignment: on A's trials | on B's trials |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | Spikes, the readouts' counts | – | +0.48 ± 0.29 | −0.80 ± 0.22 | −0.84 ± 0.21 | −0.21 ± 0.30 |
| After block 6 alone less without replay, block 6 | Drive from A's kinds, k | the window | 0.0 ± 0.0 | −4.2 ± 2.8 | 0.0 ± 0.0 | −6.5 ± 2.7 |
| After block 6 alone less without replay, block 6 | Drive from A's kinds, k | the volley | +685.9 ± 11.4 | +0.3 ± 0.8 | +670.3 ± 9.3 | −0.6 ± 0.7 |
| After block 6 alone less without replay, block 6 | Drive from A's kinds, k | both | +685.9 ± 11.4 | −3.9 ± 2.8 | +670.3 ± 9.3 | −7.1 ± 2.8 |
| After block 6 alone less without replay, block 6 | Drive from B's kinds, k | the window | −22.5 ± 3.9 | −0.1 ± 0.2 | +7.9 ± 4.4 | 0.0 ± 0.0 |
| After block 6 alone less without replay, block 6 | Drive from B's kinds, k | the volley | +0.3 ± 0.7 | +622.6 ± 12.8 | +1.6 ± 1.2 | +537.1 ± 8.5 |
| After block 6 alone less without replay, block 6 | Drive from B's kinds, k | both | −22.3 ± 4.1 | +622.5 ± 12.8 | +9.5 ± 4.6 | +537.1 ± 8.5 |
| After block 6 alone less without replay, block 6 | Drive from the odd places' excitatory, k | the window | −279.1 ± 25.2 | −73.7 ± 9.3 | −86.3 ± 12.6 | −168.3 ± 8.8 |
| After block 6 alone less without replay, block 6 | Drive from the odd places' excitatory, k | the volley | +3.4 ± 2.7 | +1.9 ± 2.6 | −0.9 ± 2.2 | +3.0 ± 3.1 |
| After block 6 alone less without replay, block 6 | Drive from the odd places' excitatory, k | both | −275.7 ± 25.2 | −71.9 ± 8.3 | −87.1 ± 12.1 | −165.2 ± 10.0 |
| After block 6 alone less without replay, block 6 | Drive from the even places' excitatory, k | the window | +207.3 ± 7.5 | −175.4 ± 6.7 | −356.2 ± 14.9 | +126.6 ± 11.6 |
| After block 6 alone less without replay, block 6 | Drive from the even places' excitatory, k | the volley | +2.3 ± 2.1 | −1.6 ± 2.8 | −5.6 ± 3.1 | +4.3 ± 2.9 |
| After block 6 alone less without replay, block 6 | Drive from the even places' excitatory, k | both | +209.6 ± 7.5 | −177.0 ± 6.0 | −361.8 ± 14.9 | +130.9 ± 11.0 |
| After block 6 alone less without replay, block 6 | Drive from inhibitory, k | the window | +319.3 ± 12.2 | −28.5 ± 14.6 | +261.8 ± 15.3 | +148.9 ± 14.3 |
| After block 6 alone less without replay, block 6 | Drive from inhibitory, k | the volley | +3.5 ± 3.5 | +0.1 ± 3.4 | +0.4 ± 3.0 | −2.9 ± 3.9 |
| After block 6 alone less without replay, block 6 | Drive from inhibitory, k | both | +322.9 ± 11.8 | −28.4 ± 15.2 | +262.2 ± 15.8 | +146.0 ± 16.5 |
| After block 6 alone less without replay, block 6 | Drive from excitatory less inhibitory, k | the window | −417.2 ± 34.0 | −225.0 ± 23.1 | −692.2 ± 22.0 | −196.0 ± 20.8 |
| After block 6 alone less without replay, block 6 | Drive from excitatory less inhibitory, k | the volley | +688.3 ± 11.4 | +623.3 ± 14.2 | +665.1 ± 10.9 | +546.5 ± 11.6 |
| After block 6 alone less without replay, block 6 | Drive from excitatory less inhibitory, k | both | +271.0 ± 42.6 | +398.3 ± 26.0 | −27.1 ± 24.0 | +350.4 ± 27.3 |
| After block 12 alone less without replay, block 12 | Spikes, the readouts' counts | – | −1.46 ± 0.22 | −0.90 ± 0.21 | +0.69 ± 0.18 | +0.08 ± 0.19 |
| After block 12 alone less without replay, block 12 | Drive from A's kinds, k | the window | −0.2 ± 0.3 | −7.1 ± 2.9 | −0.1 ± 0.3 | −6.1 ± 2.4 |
| After block 12 alone less without replay, block 12 | Drive from A's kinds, k | the volley | +624.3 ± 6.8 | +1.4 ± 1.3 | +812.0 ± 3.2 | −1.0 ± 1.2 |
| After block 12 alone less without replay, block 12 | Drive from A's kinds, k | both | +624.1 ± 6.8 | −5.7 ± 2.6 | +811.9 ± 3.1 | −7.2 ± 2.3 |
| After block 12 alone less without replay, block 12 | Drive from B's kinds, k | the window | −36.1 ± 3.9 | +0.8 ± 0.8 | +15.2 ± 7.0 | 0.0 ± 0.0 |
| After block 12 alone less without replay, block 12 | Drive from B's kinds, k | the volley | +0.4 ± 1.5 | +544.5 ± 3.8 | −0.4 ± 1.6 | +525.2 ± 16.9 |
| After block 12 alone less without replay, block 12 | Drive from B's kinds, k | both | −35.7 ± 4.2 | +545.3 ± 3.6 | +14.9 ± 7.2 | +525.2 ± 16.9 |
| After block 12 alone less without replay, block 12 | Drive from the odd places' excitatory, k | the window | −433.0 ± 24.6 | −55.7 ± 17.8 | +53.7 ± 13.2 | −174.5 ± 11.5 |
| After block 12 alone less without replay, block 12 | Drive from the odd places' excitatory, k | the volley | −1.5 ± 3.0 | +1.1 ± 2.7 | +1.9 ± 2.6 | −2.1 ± 3.0 |
| After block 12 alone less without replay, block 12 | Drive from the odd places' excitatory, k | both | −434.6 ± 24.5 | −54.6 ± 17.4 | +55.6 ± 12.8 | −176.7 ± 11.9 |
| After block 12 alone less without replay, block 12 | Drive from the even places' excitatory, k | the window | +163.3 ± 11.3 | −188.3 ± 11.3 | −326.7 ± 14.3 | +188.5 ± 14.2 |
| After block 12 alone less without replay, block 12 | Drive from the even places' excitatory, k | the volley | −1.9 ± 1.8 | +3.3 ± 2.3 | −0.9 ± 1.8 | −1.5 ± 2.3 |
| After block 12 alone less without replay, block 12 | Drive from the even places' excitatory, k | both | +161.4 ± 10.9 | −185.1 ± 12.1 | −327.6 ± 14.2 | +187.0 ± 13.7 |
| After block 12 alone less without replay, block 12 | Drive from inhibitory, k | the window | +270.2 ± 21.6 | −77.2 ± 13.7 | +357.6 ± 13.4 | +111.2 ± 18.9 |
| After block 12 alone less without replay, block 12 | Drive from inhibitory, k | the volley | +6.5 ± 3.7 | −3.3 ± 5.9 | −7.3 ± 3.4 | +0.6 ± 4.5 |
| After block 12 alone less without replay, block 12 | Drive from inhibitory, k | both | +276.7 ± 21.3 | −80.5 ± 11.7 | +350.3 ± 11.8 | +111.8 ± 18.5 |
| After block 12 alone less without replay, block 12 | Drive from excitatory less inhibitory, k | the window | −581.1 ± 27.6 | −172.7 ± 24.9 | −606.2 ± 18.8 | −104.3 ± 34.3 |
| After block 12 alone less without replay, block 12 | Drive from excitatory less inhibitory, k | the volley | +614.8 ± 8.9 | +553.8 ± 7.5 | +819.9 ± 4.9 | +519.7 ± 17.7 |
| After block 12 alone less without replay, block 12 | Drive from excitatory less inhibitory, k | both | +33.7 ± 23.5 | +381.1 ± 22.3 | +213.7 ± 18.6 | +415.4 ± 29.1 |
| After block 18 alone less without replay, block 18 | Spikes, the readouts' counts | – | −2.96 ± 0.19 | −1.43 ± 0.37 | +2.42 ± 0.23 | +1.32 ± 0.26 |
| After block 18 alone less without replay, block 18 | Drive from A's kinds, k | the window | 0.0 ± 0.0 | −7.4 ± 2.0 | 0.0 ± 0.0 | −7.9 ± 2.1 |
| After block 18 alone less without replay, block 18 | Drive from A's kinds, k | the volley | +597.0 ± 8.1 | +1.2 ± 1.5 | +792.1 ± 14.1 | −1.2 ± 1.3 |
| After block 18 alone less without replay, block 18 | Drive from A's kinds, k | both | +597.0 ± 8.1 | −6.2 ± 2.4 | +792.1 ± 14.1 | −9.1 ± 1.9 |
| After block 18 alone less without replay, block 18 | Drive from B's kinds, k | the window | −31.0 ± 3.2 | 0.0 ± 0.0 | +0.7 ± 5.0 | 0.0 ± 0.0 |
| After block 18 alone less without replay, block 18 | Drive from B's kinds, k | the volley | −1.7 ± 1.3 | +528.3 ± 14.7 | −0.2 ± 1.2 | +528.4 ± 15.6 |
| After block 18 alone less without replay, block 18 | Drive from B's kinds, k | both | −32.7 ± 3.6 | +528.3 ± 14.7 | +0.5 ± 5.0 | +528.4 ± 15.6 |
| After block 18 alone less without replay, block 18 | Drive from the odd places' excitatory, k | the window | −403.0 ± 12.0 | −85.8 ± 14.3 | +64.1 ± 16.2 | −118.1 ± 10.8 |
| After block 18 alone less without replay, block 18 | Drive from the odd places' excitatory, k | the volley | +0.2 ± 2.8 | +6.8 ± 2.4 | +0.1 ± 2.8 | −5.3 ± 1.6 |
| After block 18 alone less without replay, block 18 | Drive from the odd places' excitatory, k | both | −402.9 ± 12.7 | −79.0 ± 14.7 | +64.2 ± 16.3 | −123.4 ± 11.2 |
| After block 18 alone less without replay, block 18 | Drive from the even places' excitatory, k | the window | +97.6 ± 16.9 | −241.7 ± 11.8 | −264.6 ± 16.4 | +127.3 ± 9.9 |
| After block 18 alone less without replay, block 18 | Drive from the even places' excitatory, k | the volley | +2.2 ± 3.1 | +1.6 ± 2.0 | −3.1 ± 2.7 | 0.0 ± 2.4 |
| After block 18 alone less without replay, block 18 | Drive from the even places' excitatory, k | both | +99.8 ± 17.9 | −240.1 ± 12.2 | −267.7 ± 15.2 | +127.3 ± 9.6 |
| After block 18 alone less without replay, block 18 | Drive from inhibitory, k | the window | +264.3 ± 27.3 | −75.0 ± 10.5 | +405.9 ± 14.6 | +95.5 ± 11.3 |
| After block 18 alone less without replay, block 18 | Drive from inhibitory, k | the volley | −6.4 ± 3.6 | −1.5 ± 4.0 | +4.5 ± 2.8 | +3.1 ± 3.8 |
| After block 18 alone less without replay, block 18 | Drive from inhibitory, k | both | +257.9 ± 26.3 | −76.6 ± 12.1 | +410.4 ± 14.8 | +98.6 ± 10.9 |
| After block 18 alone less without replay, block 18 | Drive from excitatory less inhibitory, k | the window | −606.8 ± 19.9 | −259.6 ± 25.3 | −596.5 ± 20.1 | −95.5 ± 18.2 |
| After block 18 alone less without replay, block 18 | Drive from excitatory less inhibitory, k | the volley | +603.9 ± 11.2 | +539.5 ± 14.2 | +784.7 ± 14.8 | +518.7 ± 17.8 |
| After block 18 alone less without replay, block 18 | Drive from excitatory less inhibitory, k | both | −2.9 ± 18.9 | +279.9 ± 23.9 | +188.2 ± 27.5 | +423.1 ± 23.5 |
| After block 24 alone less without replay, block 24 | Spikes, the readouts' counts | – | −3.73 ± 0.16 | −0.56 ± 0.17 | +1.31 ± 0.31 | +1.66 ± 0.35 |
| After block 24 alone less without replay, block 24 | Drive from A's kinds, k | the window | +0.1 ± 0.1 | −7.1 ± 3.3 | +0.1 ± 0.1 | −15.0 ± 3.4 |
| After block 24 alone less without replay, block 24 | Drive from A's kinds, k | the volley | +490.9 ± 3.8 | −1.4 ± 1.2 | +687.9 ± 24.3 | −0.6 ± 1.3 |
| After block 24 alone less without replay, block 24 | Drive from A's kinds, k | both | +491.0 ± 3.8 | −8.5 ± 3.5 | +687.9 ± 24.2 | −15.7 ± 3.0 |
| After block 24 alone less without replay, block 24 | Drive from B's kinds, k | the window | −32.2 ± 4.5 | +0.3 ± 0.1 | −31.7 ± 5.1 | −0.1 ± 0.0 |
| After block 24 alone less without replay, block 24 | Drive from B's kinds, k | the volley | −0.9 ± 0.9 | +638.5 ± 21.1 | −0.9 ± 1.3 | +699.8 ± 22.9 |
| After block 24 alone less without replay, block 24 | Drive from B's kinds, k | both | −33.1 ± 4.5 | +638.8 ± 21.1 | −32.6 ± 5.1 | +699.7 ± 22.9 |
| After block 24 alone less without replay, block 24 | Drive from the odd places' excitatory, k | the window | −551.7 ± 16.3 | −115.4 ± 16.3 | +70.0 ± 14.4 | −95.2 ± 13.6 |
| After block 24 alone less without replay, block 24 | Drive from the odd places' excitatory, k | the volley | −2.4 ± 2.3 | −0.9 ± 3.3 | −1.2 ± 2.3 | −3.1 ± 4.4 |
| After block 24 alone less without replay, block 24 | Drive from the odd places' excitatory, k | both | −554.0 ± 16.8 | −116.3 ± 17.2 | +68.8 ± 14.8 | −98.3 ± 13.8 |
| After block 24 alone less without replay, block 24 | Drive from the even places' excitatory, k | the window | +37.1 ± 24.5 | −153.5 ± 11.3 | −299.6 ± 19.3 | +68.9 ± 14.5 |
| After block 24 alone less without replay, block 24 | Drive from the even places' excitatory, k | the volley | −2.0 ± 1.8 | −3.3 ± 2.5 | +0.7 ± 3.0 | +0.9 ± 2.9 |
| After block 24 alone less without replay, block 24 | Drive from the even places' excitatory, k | both | +35.0 ± 24.8 | −156.8 ± 12.6 | −298.8 ± 19.6 | +69.7 ± 15.7 |
| After block 24 alone less without replay, block 24 | Drive from inhibitory, k | the window | +263.4 ± 26.1 | −96.2 ± 28.9 | +314.5 ± 23.5 | +53.7 ± 20.1 |
| After block 24 alone less without replay, block 24 | Drive from inhibitory, k | the volley | +4.5 ± 3.5 | −2.6 ± 3.2 | +0.8 ± 3.9 | −0.5 ± 3.5 |
| After block 24 alone less without replay, block 24 | Drive from inhibitory, k | both | +267.9 ± 26.8 | −98.8 ± 28.5 | +315.3 ± 22.8 | +53.2 ± 18.9 |
| After block 24 alone less without replay, block 24 | Drive from excitatory less inhibitory, k | the window | −814.7 ± 22.3 | −178.1 ± 34.1 | −571.6 ± 37.2 | −99.6 ± 25.6 |
| After block 24 alone less without replay, block 24 | Drive from excitatory less inhibitory, k | the volley | +480.8 ± 5.5 | +635.6 ± 22.2 | +685.7 ± 25.3 | +697.1 ± 23.3 |
| After block 24 alone less without replay, block 24 | Drive from excitatory less inhibitory, k | both | −333.9 ± 26.9 | +457.6 ± 49.3 | +114.1 ± 53.6 | +597.5 ± 26.5 |
| After block 6 alone less without replay, block 36 | Spikes, the readouts' counts | – | −0.41 ± 0.23 | −0.37 ± 0.30 | +0.63 ± 0.33 | +0.98 ± 0.42 |
| After block 6 alone less without replay, block 36 | Drive from A's kinds, k | the window | 0.0 ± 0.0 | −0.3 ± 1.8 | 0.0 ± 0.0 | −16.0 ± 1.6 |
| After block 6 alone less without replay, block 36 | Drive from A's kinds, k | the volley | +597.2 ± 45.1 | −0.8 ± 0.8 | +654.9 ± 7.8 | −1.0 ± 1.0 |
| After block 6 alone less without replay, block 36 | Drive from A's kinds, k | both | +597.2 ± 45.1 | −1.2 ± 2.5 | +654.9 ± 7.9 | −17.1 ± 1.7 |
| After block 6 alone less without replay, block 36 | Drive from B's kinds, k | the window | −28.7 ± 9.3 | −0.3 ± 0.4 | −11.9 ± 6.8 | +0.1 ± 0.2 |
| After block 6 alone less without replay, block 36 | Drive from B's kinds, k | the volley | −0.2 ± 0.5 | +499.0 ± 60.2 | −1.3 ± 1.1 | +496.8 ± 52.9 |
| After block 6 alone less without replay, block 36 | Drive from B's kinds, k | both | −28.9 ± 9.4 | +498.8 ± 60.5 | −13.1 ± 7.7 | +496.9 ± 52.9 |
| After block 6 alone less without replay, block 36 | Drive from the odd places' excitatory, k | the window | −390.4 ± 17.1 | −178.5 ± 17.2 | +69.2 ± 11.6 | −50.2 ± 16.0 |
| After block 6 alone less without replay, block 36 | Drive from the odd places' excitatory, k | the volley | −1.9 ± 1.7 | −0.7 ± 1.3 | +2.5 ± 1.8 | +1.0 ± 2.1 |
| After block 6 alone less without replay, block 36 | Drive from the odd places' excitatory, k | both | −392.3 ± 16.7 | −179.2 ± 16.4 | +71.7 ± 9.9 | −49.1 ± 16.9 |
| After block 6 alone less without replay, block 36 | Drive from the even places' excitatory, k | the window | +17.3 ± 35.5 | −180.7 ± 10.9 | −350.8 ± 20.6 | +114.8 ± 11.0 |
| After block 6 alone less without replay, block 36 | Drive from the even places' excitatory, k | the volley | +1.4 ± 0.6 | +1.5 ± 0.9 | −3.4 ± 0.7 | +4.1 ± 1.0 |
| After block 6 alone less without replay, block 36 | Drive from the even places' excitatory, k | both | +18.7 ± 35.0 | −179.2 ± 11.4 | −354.2 ± 21.0 | +118.9 ± 11.5 |
| After block 6 alone less without replay, block 36 | Drive from inhibitory, k | the window | +183.0 ± 42.8 | +112.5 ± 27.7 | −1.1 ± 32.4 | +196.9 ± 32.1 |
| After block 6 alone less without replay, block 36 | Drive from inhibitory, k | the volley | +1.4 ± 0.3 | −0.3 ± 1.5 | +0.2 ± 2.4 | +1.6 ± 1.7 |
| After block 6 alone less without replay, block 36 | Drive from inhibitory, k | both | +184.4 ± 42.9 | +112.2 ± 28.9 | −0.9 ± 30.2 | +198.5 ± 30.2 |
| After block 6 alone less without replay, block 36 | Drive from excitatory less inhibitory, k | the window | −590.1 ± 64.1 | −471.4 ± 31.2 | −270.9 ± 47.0 | −151.8 ± 26.6 |
| After block 6 alone less without replay, block 36 | Drive from excitatory less inhibitory, k | the volley | +595.1 ± 45.5 | +499.5 ± 58.0 | +652.7 ± 8.3 | +499.1 ± 55.3 |
| After block 6 alone less without replay, block 36 | Drive from excitatory less inhibitory, k | both | +4.9 ± 50.6 | +28.1 ± 41.8 | +381.8 ± 40.3 | +347.3 ± 55.4 |
| After block 12 alone less without replay, block 36 | Spikes, the readouts' counts | – | −0.53 ± 0.27 | −0.54 ± 0.46 | +0.95 ± 0.21 | +1.18 ± 0.38 |
| After block 12 alone less without replay, block 36 | Drive from A's kinds, k | the window | 0.0 ± 0.0 | −6.2 ± 2.0 | 0.0 ± 0.0 | −15.6 ± 1.5 |
| After block 12 alone less without replay, block 36 | Drive from A's kinds, k | the volley | +756.3 ± 22.1 | −0.4 ± 0.4 | +737.4 ± 15.9 | −0.8 ± 0.5 |
| After block 12 alone less without replay, block 36 | Drive from A's kinds, k | both | +756.3 ± 22.1 | −6.6 ± 2.4 | +737.5 ± 15.9 | −16.3 ± 1.6 |
| After block 12 alone less without replay, block 36 | Drive from B's kinds, k | the window | −34.6 ± 7.0 | 0.0 ± 0.3 | −4.0 ± 11.7 | −0.2 ± 0.2 |
| After block 12 alone less without replay, block 36 | Drive from B's kinds, k | the volley | −0.8 ± 0.3 | +490.0 ± 78.6 | 0.0 ± 0.6 | +427.7 ± 51.3 |
| After block 12 alone less without replay, block 36 | Drive from B's kinds, k | both | −35.3 ± 6.8 | +490.0 ± 78.6 | −4.0 ± 12.3 | +427.4 ± 51.4 |
| After block 12 alone less without replay, block 36 | Drive from the odd places' excitatory, k | the window | −389.5 ± 17.9 | −239.4 ± 15.4 | +116.6 ± 14.5 | −62.6 ± 8.5 |
| After block 12 alone less without replay, block 36 | Drive from the odd places' excitatory, k | the volley | −0.5 ± 1.3 | +0.1 ± 2.2 | +0.5 ± 1.7 | +0.4 ± 0.5 |
| After block 12 alone less without replay, block 36 | Drive from the odd places' excitatory, k | both | −390.0 ± 18.3 | −239.3 ± 15.3 | +117.0 ± 14.9 | −62.2 ± 8.8 |
| After block 12 alone less without replay, block 36 | Drive from the even places' excitatory, k | the window | +11.4 ± 24.8 | −143.3 ± 25.4 | −160.8 ± 28.2 | +130.4 ± 32.7 |
| After block 12 alone less without replay, block 36 | Drive from the even places' excitatory, k | the volley | +0.3 ± 1.0 | −0.2 ± 0.9 | −2.2 ± 1.5 | +4.0 ± 1.6 |
| After block 12 alone less without replay, block 36 | Drive from the even places' excitatory, k | both | +11.7 ± 25.4 | −143.5 ± 24.7 | −163.0 ± 28.9 | +134.4 ± 32.2 |
| After block 12 alone less without replay, block 36 | Drive from inhibitory, k | the window | +268.8 ± 34.9 | +163.7 ± 16.4 | +82.0 ± 27.7 | +111.3 ± 9.7 |
| After block 12 alone less without replay, block 36 | Drive from inhibitory, k | the volley | +0.9 ± 1.3 | −0.7 ± 1.6 | −0.3 ± 2.3 | +1.6 ± 0.8 |
| After block 12 alone less without replay, block 36 | Drive from inhibitory, k | both | +269.7 ± 35.9 | +163.0 ± 15.5 | +81.7 ± 25.4 | +112.9 ± 9.2 |
| After block 12 alone less without replay, block 36 | Drive from excitatory less inhibitory, k | the window | −685.1 ± 59.3 | −551.6 ± 36.7 | −118.8 ± 22.4 | −62.1 ± 42.9 |
| After block 12 alone less without replay, block 36 | Drive from excitatory less inhibitory, k | the volley | +754.5 ± 20.9 | +490.2 ± 76.9 | +736.3 ± 18.0 | +429.5 ± 49.4 |
| After block 12 alone less without replay, block 36 | Drive from excitatory less inhibitory, k | both | +69.4 ± 40.2 | −61.3 ± 63.2 | +617.4 ± 24.0 | +367.4 ± 91.3 |
| After block 18 alone less without replay, block 36 | Spikes, the readouts' counts | – | −1.69 ± 0.12 | −0.56 ± 0.35 | +1.42 ± 0.35 | +1.59 ± 0.23 |
| After block 18 alone less without replay, block 36 | Drive from A's kinds, k | the window | 0.0 ± 0.0 | −2.6 ± 1.7 | 0.0 ± 0.0 | −14.3 ± 1.4 |
| After block 18 alone less without replay, block 36 | Drive from A's kinds, k | the volley | +606.1 ± 39.4 | +0.1 ± 0.4 | +781.3 ± 38.4 | −0.5 ± 1.0 |
| After block 18 alone less without replay, block 36 | Drive from A's kinds, k | both | +606.1 ± 39.4 | −2.6 ± 1.8 | +781.3 ± 38.4 | −14.8 ± 1.9 |
| After block 18 alone less without replay, block 36 | Drive from B's kinds, k | the window | −29.3 ± 2.6 | −0.1 ± 0.2 | +6.0 ± 3.9 | +0.1 ± 0.1 |
| After block 18 alone less without replay, block 36 | Drive from B's kinds, k | the volley | −0.6 ± 0.7 | +475.0 ± 30.0 | −1.0 ± 1.0 | +499.5 ± 17.9 |
| After block 18 alone less without replay, block 36 | Drive from B's kinds, k | both | −29.9 ± 2.3 | +475.0 ± 30.0 | +5.0 ± 4.8 | +499.6 ± 17.8 |
| After block 18 alone less without replay, block 36 | Drive from the odd places' excitatory, k | the window | −333.6 ± 11.1 | −182.3 ± 14.6 | +78.6 ± 7.7 | −47.1 ± 10.2 |
| After block 18 alone less without replay, block 36 | Drive from the odd places' excitatory, k | the volley | −2.5 ± 1.6 | −0.3 ± 1.5 | +3.6 ± 1.4 | +0.6 ± 1.7 |
| After block 18 alone less without replay, block 36 | Drive from the odd places' excitatory, k | both | −336.1 ± 12.3 | −182.7 ± 14.8 | +82.2 ± 6.9 | −46.6 ± 11.4 |
| After block 18 alone less without replay, block 36 | Drive from the even places' excitatory, k | the window | +32.7 ± 25.7 | −193.1 ± 7.0 | −181.2 ± 22.3 | +136.6 ± 22.6 |
| After block 18 alone less without replay, block 36 | Drive from the even places' excitatory, k | the volley | +0.6 ± 0.5 | +0.2 ± 0.3 | −3.0 ± 0.8 | +2.9 ± 0.5 |
| After block 18 alone less without replay, block 36 | Drive from the even places' excitatory, k | both | +33.3 ± 25.5 | −192.8 ± 7.3 | −184.2 ± 22.3 | +139.5 ± 22.1 |
| After block 18 alone less without replay, block 36 | Drive from inhibitory, k | the window | +306.9 ± 38.0 | +13.5 ± 12.8 | +173.5 ± 9.1 | +105.2 ± 12.9 |
| After block 18 alone less without replay, block 36 | Drive from inhibitory, k | the volley | +1.0 ± 0.5 | −0.6 ± 0.9 | +2.1 ± 2.9 | +0.7 ± 4.7 |
| After block 18 alone less without replay, block 36 | Drive from inhibitory, k | both | +307.9 ± 38.5 | +12.9 ± 13.2 | +175.6 ± 9.9 | +105.8 ± 13.9 |
| After block 18 alone less without replay, block 36 | Drive from excitatory less inhibitory, k | the window | −641.7 ± 46.7 | −391.3 ± 28.4 | −262.6 ± 13.2 | −32.0 ± 23.7 |
| After block 18 alone less without replay, block 36 | Drive from excitatory less inhibitory, k | the volley | +602.7 ± 38.8 | +475.7 ± 29.3 | +778.9 ± 36.8 | +501.8 ± 15.5 |
| After block 18 alone less without replay, block 36 | Drive from excitatory less inhibitory, k | both | −39.0 ± 14.6 | +84.3 ± 16.5 | +516.3 ± 36.9 | +469.7 ± 29.9 |
| After block 24 alone less without replay, block 36 | Spikes, the readouts' counts | – | −2.22 ± 0.31 | +0.32 ± 0.13 | −0.28 ± 0.15 | +1.15 ± 0.41 |
| After block 24 alone less without replay, block 36 | Drive from A's kinds, k | the window | 0.0 ± 0.0 | −5.5 ± 1.3 | 0.0 ± 0.0 | −12.0 ± 1.6 |
| After block 24 alone less without replay, block 36 | Drive from A's kinds, k | the volley | +551.6 ± 18.3 | −0.8 ± 0.5 | +608.4 ± 13.1 | −0.7 ± 0.4 |
| After block 24 alone less without replay, block 36 | Drive from A's kinds, k | both | +551.7 ± 18.3 | −6.3 ± 1.8 | +608.5 ± 13.1 | −12.8 ± 2.0 |
| After block 24 alone less without replay, block 36 | Drive from B's kinds, k | the window | −40.9 ± 1.7 | 0.0 ± 0.2 | −11.9 ± 6.0 | +0.1 ± 0.2 |
| After block 24 alone less without replay, block 36 | Drive from B's kinds, k | the volley | −0.7 ± 0.3 | +713.1 ± 15.5 | −0.4 ± 0.7 | +598.9 ± 55.5 |
| After block 24 alone less without replay, block 36 | Drive from B's kinds, k | both | −41.6 ± 1.4 | +713.1 ± 15.5 | −12.3 ± 6.1 | +599.0 ± 55.5 |
| After block 24 alone less without replay, block 36 | Drive from the odd places' excitatory, k | the window | −437.6 ± 19.4 | −98.3 ± 12.7 | +83.6 ± 15.1 | −110.5 ± 12.7 |
| After block 24 alone less without replay, block 36 | Drive from the odd places' excitatory, k | the volley | −0.9 ± 1.7 | −1.5 ± 2.6 | +2.3 ± 2.0 | +1.6 ± 2.2 |
| After block 24 alone less without replay, block 36 | Drive from the odd places' excitatory, k | both | −438.5 ± 20.4 | −99.9 ± 12.4 | +85.9 ± 13.4 | −108.9 ± 14.2 |
| After block 24 alone less without replay, block 36 | Drive from the even places' excitatory, k | the window | +19.9 ± 24.4 | −182.3 ± 17.9 | −307.1 ± 11.2 | +85.7 ± 9.2 |
| After block 24 alone less without replay, block 36 | Drive from the even places' excitatory, k | the volley | +1.2 ± 1.7 | 0.0 ± 1.6 | −2.9 ± 0.9 | +2.7 ± 0.1 |
| After block 24 alone less without replay, block 36 | Drive from the even places' excitatory, k | both | +21.1 ± 24.2 | −182.3 ± 16.8 | −310.0 ± 11.2 | +88.4 ± 9.0 |
| After block 24 alone less without replay, block 36 | Drive from inhibitory, k | the window | +273.1 ± 15.1 | −7.2 ± 18.4 | +135.5 ± 36.4 | +109.6 ± 7.3 |
| After block 24 alone less without replay, block 36 | Drive from inhibitory, k | the volley | +0.3 ± 1.3 | +1.2 ± 1.4 | +1.8 ± 3.7 | −0.7 ± 1.8 |
| After block 24 alone less without replay, block 36 | Drive from inhibitory, k | both | +273.5 ± 15.5 | −6.0 ± 17.7 | +137.3 ± 38.2 | +108.9 ± 8.0 |
| After block 24 alone less without replay, block 36 | Drive from excitatory less inhibitory, k | the window | −734.8 ± 18.8 | −278.3 ± 31.3 | −370.7 ± 21.9 | −149.0 ± 21.4 |
| After block 24 alone less without replay, block 36 | Drive from excitatory less inhibitory, k | the volley | +550.8 ± 18.6 | +709.7 ± 14.4 | +605.8 ± 11.6 | +603.0 ± 58.6 |
| After block 24 alone less without replay, block 36 | Drive from excitatory less inhibitory, k | both | −184.0 ± 23.4 | +431.5 ± 24.7 | +235.1 ± 30.7 | +454.1 ± 70.2 |

**On A's trials**, right after the nights after block 6, 12, 18 and 24 alone, on the image from the assignment and then on the image from
the mirrored assignment:
- **the spikes** read +0.48 ± 0.29, −1.46 ± 0.22, −2.96 ± 0.19 and −3.73 ± 0.16; −0.84 ± 0.21, +0.69 ± 0.18, +2.42 ± 0.23 and +1.31 ± 0.31;
- **the window's drive**, excitatory less inhibitory, −417.2 ± 34.0, −581.1 ± 27.6, −606.8 ± 19.9 and −814.7 ± 22.3 k; −692.2 ± 22.0,
  −606.2 ± 18.8, −596.5 ± 20.1 and −571.6 ± 37.2;
- **the volley's** +688.3 ± 11.4, +614.8 ± 8.9, +603.9 ± 11.2 and +480.8 ± 5.5; +665.1 ± 10.9, +819.9 ± 4.9, +784.7 ± 14.8 and
  +685.7 ± 25.3;
- **their sum** +271.0 ± 42.6, +33.7 ± 23.5, −2.9 ± 18.9 and −333.9 ± 26.9; −27.1 ± 24.0, +213.7 ± 18.6, +188.2 ± 27.5 and +114.1 ± 53.6.

At block 36 after the same nights, the same way:
- **the spikes** −0.41 ± 0.23, −0.53 ± 0.27, −1.69 ± 0.12 and −2.22 ± 0.31; +0.63 ± 0.33, +0.95 ± 0.21, +1.42 ± 0.35 and −0.28 ± 0.15;
- **the window's** −590.1 ± 64.1, −685.1 ± 59.3, −641.7 ± 46.7 and −734.8 ± 18.8; −270.9 ± 47.0, −118.8 ± 22.4, −262.6 ± 13.2 and
  −370.7 ± 21.9;
- **the volley's** +595.1 ± 45.5, +754.5 ± 20.9, +602.7 ± 38.8 and +550.8 ± 18.6; +652.7 ± 8.3, +736.3 ± 18.0, +778.9 ± 36.8 and
  +605.8 ± 11.6;
- **their sum** +4.9 ± 50.6, +69.4 ± 40.2, −39.0 ± 14.6 and −184.0 ± 23.4; +381.8 ± 40.3, +617.4 ± 24.0, +516.3 ± 36.9 and +235.1 ± 30.7.

So, on A's trials, the window's lies below the night without replay beyond 2 SE on both cells in 8 of 8 comparisons, and the
volley's above on both in 8 of 8; their sum lies on the same side of 2 SE on both cells in 0 of 8. In
12 of 12 distinct comparisons, these eight and H-47's plain night at block 12, 18, 24 and 36, the cell whose spikes difference lies
higher has the higher sum. On B's trials the window's lies on the same side of 2 SE on both cells in 6 of 8, the volley's in
8 of 8 and the sum in 6 of 8, and the cells' order of the sum, B's answer's readout less A's, is the spikes' in 11 of 12, all but right
after the night after block 6 alone.

**Where the cells part**, on each stimulus's trials, both spans, the image from the mirrored assignment's less the image from the
assignment's; the six columns from the trial's stimulus's kinds to the inhibitory sum to it:

| Comparison | Trials | Spikes, the answer's readout less the other: from the assignment | from the mirrored assignment | Drive, both spans, excitatory less inhibitory: from the assignment, k | from the mirrored assignment | The image from the mirrored assignment's less the image from the assignment's | of which the window | the volley | of which the trial's stimulus's kinds | the other stimulus's kinds | the answer's readout's own place group | the other place group | other excitatory | inhibitory, negated |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | +0.48 ± 0.29 | −0.84 ± 0.21 | +271.0 | −27.1 | −298.2 | −275.0 | −23.2 | −15.6 | +31.8 | −296.7 | −86.1 | +7.8 | +60.7 |
| After block 12 alone less without replay, block 12 | A's trials | −1.46 ± 0.22 | +0.69 ± 0.18 | +33.7 | +213.7 | +179.9 | −25.2 | +205.1 | +187.9 | +50.6 | −105.8 | +106.9 | +13.9 | −73.6 |
| After block 18 alone less without replay, block 18 | A's trials | −2.96 ± 0.19 | +2.42 ± 0.23 | −2.9 | +188.2 | +191.1 | +10.3 | +180.8 | +195.1 | +33.2 | −35.6 | +135.2 | +15.6 | −152.5 |
| After block 24 alone less without replay, block 24 | A's trials | −3.73 ± 0.16 | +1.31 ± 0.31 | −333.9 | +114.1 | +448.0 | +243.1 | +204.9 | +197.0 | +0.5 | +33.7 | +255.2 | +9.0 | −47.4 |
| After block 6 alone less without replay, block 36 | A's trials | −0.41 ± 0.23 | +0.63 ± 0.33 | +4.9 | +381.8 | +376.9 | +319.3 | +57.7 | +57.7 | +15.8 | +53.0 | +38.1 | +27.1 | +185.2 |
| After block 12 alone less without replay, block 36 | A's trials | −0.53 ± 0.27 | +0.95 ± 0.21 | +69.4 | +617.4 | +548.1 | +566.3 | −18.2 | −18.8 | +31.3 | +105.3 | +227.0 | +15.4 | +187.9 |
| After block 18 alone less without replay, block 36 | A's trials | −1.69 ± 0.12 | +1.42 ± 0.35 | −39.0 | +516.3 | +555.3 | +379.1 | +176.2 | +175.3 | +34.9 | +48.9 | +151.9 | +12.0 | +132.3 |
| After block 24 alone less without replay, block 36 | A's trials | −2.22 ± 0.31 | −0.28 ± 0.15 | −184.0 | +235.1 | +419.1 | +364.2 | +55.0 | +56.8 | +29.3 | +64.8 | +128.5 | +3.6 | +136.2 |
| Plain on all four nights less without replay, block 6 | A's trials | +0.48 ± 0.29 | −0.84 ± 0.21 | +271.0 | −27.1 | −298.2 | −275.0 | −23.2 | −15.6 | +31.8 | −296.7 | −86.1 | +7.8 | +60.7 |
| Plain on all four nights less without replay, block 12 | A's trials | +0.22 ± 0.43 | −0.87 ± 0.25 | +276.6 | +161.0 | −115.6 | −284.4 | +168.8 | +163.9 | +45.5 | −334.6 | −49.4 | +12.8 | +46.2 |
| Plain on all four nights less without replay, block 18 | A's trials | −1.18 ± 0.30 | +0.68 ± 0.20 | +307.7 | +507.6 | +199.8 | −180.6 | +380.4 | +387.3 | +43.6 | −173.2 | +36.3 | +26.2 | −120.3 |
| Plain on all four nights less without replay, block 24 | A's trials | −1.52 ± 0.27 | +1.50 ± 0.45 | +242.3 | +936.5 | +694.2 | +273.1 | +421.2 | +428.4 | +41.6 | +18.0 | +321.5 | +31.8 | −147.2 |
| Plain on all four nights less without replay, block 36 | A's trials | +0.17 ± 0.31 | +1.39 ± 0.49 | +331.8 | +1 055.3 | +723.5 | +375.3 | +348.2 | +361.7 | +44.5 | +49.8 | +197.8 | +26.7 | +43.0 |
| After block 6 alone less without replay, block 6 | B's trials | −0.80 ± 0.22 | −0.21 ± 0.30 | +398.3 | +350.4 | −47.8 | +29.0 | −76.8 | −85.4 | −3.2 | +202.8 | +11.8 | +0.6 | −174.4 |
| After block 12 alone less without replay, block 12 | B's trials | −0.90 ± 0.21 | +0.08 ± 0.19 | +381.1 | +415.4 | +34.3 | +68.4 | −34.1 | −20.0 | −1.5 | +241.7 | +8.4 | −1.9 | −192.3 |
| After block 18 alone less without replay, block 18 | B's trials | −1.43 ± 0.37 | +1.32 ± 0.26 | +279.9 | +423.1 | +143.2 | +164.0 | −20.8 | +0.1 | −2.9 | +206.3 | +116.7 | −1.8 | −175.2 |
| After block 24 alone less without replay, block 24 | B's trials | −0.56 ± 0.17 | +1.66 ± 0.35 | +457.6 | +597.5 | +139.9 | +78.4 | +61.5 | +60.9 | −7.2 | +186.0 | +58.6 | −6.5 | −151.9 |
| After block 6 alone less without replay, block 36 | B's trials | −0.37 ± 0.30 | +0.98 ± 0.42 | +28.1 | +347.3 | +319.2 | +319.6 | −0.4 | −1.9 | −15.9 | +298.1 | +130.0 | −4.8 | −86.4 |
| After block 12 alone less without replay, block 36 | B's trials | −0.54 ± 0.46 | +1.18 ± 0.38 | −61.3 | +367.4 | +428.8 | +489.5 | −60.7 | −62.6 | −9.7 | +373.7 | +81.3 | −4.0 | +50.0 |
| After block 18 alone less without replay, block 36 | B's trials | −0.56 ± 0.35 | +1.59 ± 0.23 | +84.3 | +469.7 | +385.4 | +359.3 | +26.1 | +24.6 | −12.3 | +322.2 | +146.2 | −2.5 | −92.9 |
| After block 24 alone less without replay, block 36 | B's trials | +0.32 ± 0.13 | +1.15 ± 0.41 | +431.5 | +454.1 | +22.6 | +129.3 | −106.7 | −114.1 | −6.5 | +188.2 | +73.4 | −3.5 | −114.9 |
| Plain on all four nights less without replay, block 6 | B's trials | −0.80 ± 0.22 | −0.21 ± 0.30 | +398.3 | +350.4 | −47.8 | +29.0 | −76.8 | −85.4 | −3.2 | +202.8 | +11.8 | +0.6 | −174.4 |
| Plain on all four nights less without replay, block 12 | B's trials | −0.51 ± 0.52 | +0.03 ± 0.19 | +619.0 | +624.1 | +5.1 | +207.7 | −202.6 | −196.1 | −2.6 | +346.4 | +89.6 | −1.3 | −230.9 |
| Plain on all four nights less without replay, block 18 | B's trials | −0.83 ± 0.65 | +2.72 ± 0.39 | +645.2 | +1 098.6 | +453.4 | +427.4 | +26.0 | +44.1 | −9.5 | +403.1 | +313.3 | −3.5 | −294.1 |
| Plain on all four nights less without replay, block 24 | B's trials | −0.43 ± 0.59 | +3.10 ± 0.40 | +679.0 | +1 347.2 | +668.1 | +674.9 | −6.8 | −12.7 | −12.7 | +540.2 | +495.6 | −8.6 | −333.6 |
| Plain on all four nights less without replay, block 36 | B's trials | +0.65 ± 0.57 | +3.80 ± 0.35 | +683.9 | +1 299.0 | +615.0 | +671.5 | −56.5 | −64.5 | −7.0 | +578.1 | +487.1 | −5.7 | −372.9 |

On A's trials, right after the four single nights, the sum's difference read −298.2, +179.9, +191.1 and +448.0 k: the window carried −275.0,
−25.2, +10.3 and +243.1 and the volley −23.2, +205.1, +180.8 and +204.9; by group, A's kinds −15.6, +187.9, +195.1 and +197.0, B's kinds
+31.8, +50.6, +33.2 and +0.5, A's answer's readout's own place group −296.7, −105.8, −35.6 and +33.7, the other place group −86.1, +106.9,
+135.2 and +255.2, other excitatory +7.8, +13.9, +15.6 and +9.0 and the inhibitory, negated, +60.7, −73.6, −152.5 and −47.4. At block 36 it
read +376.9, +548.1, +555.3 and +419.1: the window +319.3, +566.3, +379.1 and +364.2, the volley +57.7, −18.2, +176.2 and +55.0; A's kinds
+57.7, −18.8, +175.3 and +56.8, B's kinds +15.8, +31.3, +34.9 and +29.3, the own place group +53.0, +105.3, +48.9 and +64.8, the other
+38.1, +227.0, +151.9 and +128.5, other excitatory +27.1, +15.4, +12.0 and +3.6 and the inhibitory, negated, +185.2, +187.9, +132.3 and
+136.2.

**Weights or spikes.** The sum's split, class by class:

| Comparison | Trials | From the assignment: the answer's readout less the other, both spans, excitatory less inhibitory, k | by the weights | by the spikes per weight | From the mirrored assignment | by the weights | by the spikes per weight | The image from the mirrored assignment's less the image from the assignment's: by the weights | by the spikes per weight |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | +271.0 | +694.1 | −423.0 | −27.1 | +552.9 | −580.1 | −141.1 | −157.0 |
| After block 12 alone less without replay, block 12 | A's trials | +33.7 | +568.3 | −534.5 | +213.7 | +684.0 | −470.3 | +115.7 | +64.2 |
| After block 18 alone less without replay, block 18 | A's trials | −2.9 | +507.2 | −510.1 | +188.2 | +660.0 | −471.8 | +152.8 | +38.3 |
| After block 24 alone less without replay, block 24 | A's trials | −333.9 | +365.4 | −699.3 | +114.1 | +541.3 | −427.2 | +175.9 | +272.1 |
| After block 6 alone less without replay, block 36 | A's trials | +4.9 | +503.1 | −498.2 | +381.8 | +634.9 | −253.0 | +131.8 | +245.2 |
| After block 12 alone less without replay, block 36 | A's trials | +69.4 | +660.8 | −591.5 | +617.4 | +692.9 | −75.5 | +32.1 | +516.0 |
| After block 18 alone less without replay, block 36 | A's trials | −39.0 | +518.0 | −557.0 | +516.3 | +697.1 | −180.7 | +179.1 | +376.3 |
| After block 24 alone less without replay, block 36 | A's trials | −184.0 | +444.5 | −628.6 | +235.1 | +513.7 | −278.6 | +69.2 | +350.0 |
| Plain on all four nights less without replay, block 12 | A's trials | +276.6 | +1 062.8 | −786.2 | +161.0 | +1 089.0 | −928.0 | +26.2 | −141.9 |
| Plain on all four nights less without replay, block 18 | A's trials | +307.7 | +1 298.0 | −990.2 | +507.6 | +1 599.1 | −1 091.5 | +301.1 | −101.3 |
| Plain on all four nights less without replay, block 24 | A's trials | +242.3 | +1 637.5 | −1 395.2 | +936.5 | +1 983.1 | −1 046.5 | +345.6 | +348.6 |
| Plain on all four nights less without replay, block 36 | A's trials | +331.8 | +1 765.0 | −1 433.3 | +1 055.3 | +2 068.6 | −1 013.3 | +303.5 | +420.0 |
| After block 6 alone less without replay, block 6 | B's trials | +398.3 | +594.5 | −196.2 | +350.4 | +655.2 | −304.7 | +60.7 | −108.5 |
| After block 12 alone less without replay, block 12 | B's trials | +381.1 | +517.8 | −136.7 | +415.4 | +641.3 | −225.9 | +123.5 | −89.2 |
| After block 18 alone less without replay, block 18 | B's trials | +279.9 | +507.5 | −227.6 | +423.1 | +652.0 | −228.9 | +144.5 | −1.3 |
| After block 24 alone less without replay, block 24 | B's trials | +457.6 | +627.0 | −169.5 | +597.5 | +836.5 | −239.1 | +209.5 | −69.6 |
| After block 6 alone less without replay, block 36 | B's trials | +28.1 | +472.8 | −444.6 | +347.3 | +518.5 | −171.2 | +45.8 | +273.4 |
| After block 12 alone less without replay, block 36 | B's trials | −61.3 | +459.3 | −520.7 | +367.4 | +474.2 | −106.8 | +14.9 | +413.8 |
| After block 18 alone less without replay, block 36 | B's trials | +84.3 | +448.0 | −363.7 | +469.7 | +575.8 | −106.0 | +127.8 | +257.6 |
| After block 24 alone less without replay, block 36 | B's trials | +431.5 | +696.6 | −265.1 | +454.1 | +709.5 | −255.5 | +12.9 | +9.7 |
| Plain on all four nights less without replay, block 12 | B's trials | +619.0 | +1 048.7 | −429.6 | +624.1 | +1 134.7 | −510.5 | +86.0 | −80.9 |
| Plain on all four nights less without replay, block 18 | B's trials | +645.2 | +1 319.3 | −674.2 | +1 098.6 | +1 761.6 | −663.0 | +442.2 | +11.2 |
| Plain on all four nights less without replay, block 24 | B's trials | +679.0 | +1 573.2 | −894.2 | +1 347.2 | +2 100.6 | −753.4 | +527.3 | +140.8 |
| Plain on all four nights less without replay, block 36 | B's trials | +683.9 | +1 680.0 | −996.1 | +1 299.0 | +2 143.3 | −844.3 | +463.3 | +151.8 |

On A's trials right after the four single nights:
- by the weights, +694.1, +568.3, +507.2 and +365.4 k on the image from the assignment and +552.9, +684.0, +660.0 and +541.3 on the image
  from the mirrored assignment, the image from the mirrored assignment's less the image from the assignment's −141.1, +115.7, +152.8 and
  +175.9;
- by the spikes per weight, −423.0, −534.5, −510.1 and −699.3 on the image from the assignment and −580.1, −470.3, −471.8 and −427.2 on the
  image from the mirrored assignment, the image from the mirrored assignment's less the image from the assignment's −157.0, +64.2, +38.3 and
  +272.1.

At block 36 the image from the mirrored assignment's less the image from the assignment's read +131.8, +32.1, +179.1 and +69.2 by the
weights and +245.2, +516.0, +376.3 and +350.0 by the spikes per weight. By group and span:

| Comparison | Trials | From | Span | From the assignment: the answer's readout less the other, its drive's difference, k | by the weights | by the spikes per weight | From the mirrored assignment: the drive's difference | by the weights | by the spikes per weight |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | A's kinds | the window | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| After block 6 alone less without replay, block 6 | A's trials | A's kinds | the volley | +685.9 | +687.1 | −1.2 | +670.3 | +667.7 | +2.6 |
| After block 6 alone less without replay, block 6 | A's trials | B's kinds | the window | −22.5 | −10.3 | −12.2 | +7.9 | −14.3 | +22.2 |
| After block 6 alone less without replay, block 6 | A's trials | B's kinds | the volley | +0.3 | −0.8 | +1.1 | +1.6 | −0.8 | +2.4 |
| After block 6 alone less without replay, block 6 | A's trials | The odd places' excitatory | the window | −279.1 | −90.9 | −188.2 | −86.3 | +114.2 | −200.4 |
| After block 6 alone less without replay, block 6 | A's trials | The odd places' excitatory | the volley | +3.4 | −1.6 | +5.0 | −0.9 | +2.0 | −2.9 |
| After block 6 alone less without replay, block 6 | A's trials | The even places' excitatory | the window | +207.3 | +93.3 | +114.0 | −356.2 | −142.2 | −214.1 |
| After block 6 alone less without replay, block 6 | A's trials | The even places' excitatory | the volley | +2.3 | +1.9 | +0.4 | −5.6 | −3.4 | −2.2 |
| After block 6 alone less without replay, block 6 | A's trials | Inhibitory | the window | +319.3 | −15.9 | +335.3 | +261.8 | +68.4 | +193.4 |
| After block 6 alone less without replay, block 6 | A's trials | Inhibitory | the volley | +3.5 | +0.4 | +3.1 | +0.4 | +2.1 | −1.8 |
| After block 6 alone less without replay, block 6 | A's trials | Excitatory less inhibitory | the window | −417.2 | +7.9 | −425.2 | −692.2 | −110.5 | −581.7 |
| After block 6 alone less without replay, block 6 | A's trials | Excitatory less inhibitory | the volley | +688.3 | +686.1 | +2.2 | +665.1 | +663.5 | +1.6 |
| After block 6 alone less without replay, block 6 | B's trials | A's kinds | the window | −4.2 | −5.7 | +1.4 | −6.5 | −6.0 | −0.5 |
| After block 6 alone less without replay, block 6 | B's trials | A's kinds | the volley | +0.3 | −0.7 | +1.0 | −0.6 | −0.6 | 0.0 |
| After block 6 alone less without replay, block 6 | B's trials | B's kinds | the window | −0.1 | 0.0 | −0.1 | 0.0 | 0.0 | 0.0 |
| After block 6 alone less without replay, block 6 | B's trials | B's kinds | the volley | +622.6 | +624.0 | −1.3 | +537.1 | +536.6 | +0.5 |
| After block 6 alone less without replay, block 6 | B's trials | The odd places' excitatory | the window | −73.7 | +51.3 | −125.0 | −168.3 | −74.4 | −93.9 |
| After block 6 alone less without replay, block 6 | B's trials | The odd places' excitatory | the volley | +1.9 | +1.7 | +0.1 | +3.0 | −2.0 | +5.0 |
| After block 6 alone less without replay, block 6 | B's trials | The even places' excitatory | the window | −175.4 | −59.2 | −116.2 | +126.6 | +128.6 | −2.1 |
| After block 6 alone less without replay, block 6 | B's trials | The even places' excitatory | the volley | −1.6 | −1.6 | 0.0 | +4.3 | +2.7 | +1.6 |
| After block 6 alone less without replay, block 6 | B's trials | Inhibitory | the window | −28.5 | +16.3 | −44.8 | +148.9 | −69.0 | +217.9 |
| After block 6 alone less without replay, block 6 | B's trials | Inhibitory | the volley | +0.1 | −0.9 | +1.1 | −2.9 | −1.1 | −1.8 |
| After block 6 alone less without replay, block 6 | B's trials | Excitatory less inhibitory | the window | −225.0 | −29.9 | −195.2 | −196.0 | +117.4 | −313.4 |
| After block 6 alone less without replay, block 6 | B's trials | Excitatory less inhibitory | the volley | +623.3 | +624.3 | −1.1 | +546.5 | +537.8 | +8.7 |
| After block 12 alone less without replay, block 12 | A's trials | A's kinds | the window | −0.2 | 0.0 | −0.3 | −0.1 | +0.1 | −0.2 |
| After block 12 alone less without replay, block 12 | A's trials | A's kinds | the volley | +624.3 | +622.0 | +2.3 | +812.0 | +810.2 | +1.8 |
| After block 12 alone less without replay, block 12 | A's trials | B's kinds | the window | −36.1 | −13.0 | −23.1 | +15.2 | −19.8 | +35.1 |
| After block 12 alone less without replay, block 12 | A's trials | B's kinds | the volley | +0.4 | −0.7 | +1.1 | −0.4 | −0.8 | +0.4 |
| After block 12 alone less without replay, block 12 | A's trials | The odd places' excitatory | the window | −433.0 | −121.7 | −311.3 | +53.7 | +130.2 | −76.5 |
| After block 12 alone less without replay, block 12 | A's trials | The odd places' excitatory | the volley | −1.5 | −1.9 | +0.4 | +1.9 | +2.3 | −0.3 |
| After block 12 alone less without replay, block 12 | A's trials | The even places' excitatory | the window | +163.3 | +74.1 | +89.2 | −326.7 | −140.5 | −186.2 |
| After block 12 alone less without replay, block 12 | A's trials | The even places' excitatory | the volley | −1.9 | +1.1 | −3.0 | −0.9 | −2.4 | +1.4 |
| After block 12 alone less without replay, block 12 | A's trials | Inhibitory | the window | +270.2 | −8.5 | +278.7 | +357.6 | +94.1 | +263.5 |
| After block 12 alone less without replay, block 12 | A's trials | Inhibitory | the volley | +6.5 | −0.1 | +6.6 | −7.3 | +2.6 | −9.8 |
| After block 12 alone less without replay, block 12 | A's trials | Excitatory less inhibitory | the window | −581.1 | −52.2 | −528.8 | −606.2 | −122.7 | −483.5 |
| After block 12 alone less without replay, block 12 | A's trials | Excitatory less inhibitory | the volley | +614.8 | +620.5 | −5.7 | +819.9 | +806.7 | +13.2 |
| After block 12 alone less without replay, block 12 | B's trials | A's kinds | the window | −7.1 | −4.8 | −2.3 | −6.1 | −7.5 | +1.4 |
| After block 12 alone less without replay, block 12 | B's trials | A's kinds | the volley | +1.4 | −0.6 | +2.0 | −1.0 | −0.7 | −0.3 |
| After block 12 alone less without replay, block 12 | B's trials | B's kinds | the window | +0.8 | 0.0 | +0.7 | 0.0 | 0.0 | 0.0 |
| After block 12 alone less without replay, block 12 | B's trials | B's kinds | the volley | +544.5 | +544.0 | +0.5 | +525.2 | +522.9 | +2.3 |
| After block 12 alone less without replay, block 12 | B's trials | The odd places' excitatory | the window | −55.7 | +60.8 | −116.5 | −174.5 | −88.2 | −86.4 |
| After block 12 alone less without replay, block 12 | B's trials | The odd places' excitatory | the volley | +1.1 | +1.9 | −0.8 | −2.1 | −2.0 | −0.1 |
| After block 12 alone less without replay, block 12 | B's trials | The even places' excitatory | the window | −188.3 | −52.5 | −135.8 | +188.5 | +116.3 | +72.3 |
| After block 12 alone less without replay, block 12 | B's trials | The even places' excitatory | the volley | +3.3 | −0.8 | +4.1 | −1.5 | +2.1 | −3.6 |
| After block 12 alone less without replay, block 12 | B's trials | Inhibitory | the window | −77.2 | +30.4 | −107.6 | +111.2 | −96.8 | +208.0 |
| After block 12 alone less without replay, block 12 | B's trials | Inhibitory | the volley | −3.3 | −0.2 | −3.1 | +0.6 | −2.1 | +2.7 |
| After block 12 alone less without replay, block 12 | B's trials | Excitatory less inhibitory | the window | −172.7 | −27.0 | −145.7 | −104.3 | +117.0 | −221.3 |
| After block 12 alone less without replay, block 12 | B's trials | Excitatory less inhibitory | the volley | +553.8 | +544.7 | +9.0 | +519.7 | +524.3 | −4.6 |
| After block 18 alone less without replay, block 18 | A's trials | A's kinds | the window | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| After block 18 alone less without replay, block 18 | A's trials | A's kinds | the volley | +597.0 | +594.2 | +2.7 | +792.1 | +792.4 | −0.3 |
| After block 18 alone less without replay, block 18 | A's trials | B's kinds | the window | −31.0 | −15.5 | −15.5 | +0.7 | −18.7 | +19.4 |
| After block 18 alone less without replay, block 18 | A's trials | B's kinds | the volley | −1.7 | −0.7 | −1.1 | −0.2 | −0.5 | +0.3 |
| After block 18 alone less without replay, block 18 | A's trials | The odd places' excitatory | the window | −403.0 | −115.8 | −287.2 | +64.1 | +110.8 | −46.6 |
| After block 18 alone less without replay, block 18 | A's trials | The odd places' excitatory | the volley | +0.2 | −1.5 | +1.6 | +0.1 | +1.7 | −1.6 |
| After block 18 alone less without replay, block 18 | A's trials | The even places' excitatory | the window | +97.6 | +68.9 | +28.7 | −264.6 | −150.2 | −114.5 |
| After block 18 alone less without replay, block 18 | A's trials | The even places' excitatory | the volley | +2.2 | +0.9 | +1.3 | −3.1 | −2.3 | −0.8 |
| After block 18 alone less without replay, block 18 | A's trials | Inhibitory | the window | +264.3 | +22.2 | +242.1 | +405.9 | +73.9 | +332.1 |
| After block 18 alone less without replay, block 18 | A's trials | Inhibitory | the volley | −6.4 | +1.0 | −7.4 | +4.5 | +1.8 | +2.7 |
| After block 18 alone less without replay, block 18 | A's trials | Excitatory less inhibitory | the window | −606.8 | −84.9 | −521.9 | −596.5 | −129.5 | −467.0 |
| After block 18 alone less without replay, block 18 | A's trials | Excitatory less inhibitory | the volley | +603.9 | +592.1 | +11.8 | +784.7 | +789.5 | −4.8 |
| After block 18 alone less without replay, block 18 | B's trials | A's kinds | the window | −7.4 | −5.2 | −2.3 | −7.9 | −8.0 | +0.1 |
| After block 18 alone less without replay, block 18 | B's trials | A's kinds | the volley | +1.2 | −0.9 | +2.1 | −1.2 | −1.1 | −0.1 |
| After block 18 alone less without replay, block 18 | B's trials | B's kinds | the window | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| After block 18 alone less without replay, block 18 | B's trials | B's kinds | the volley | +528.3 | +528.6 | −0.2 | +528.4 | +528.2 | +0.3 |
| After block 18 alone less without replay, block 18 | B's trials | The odd places' excitatory | the window | −85.8 | +52.1 | −137.8 | −118.1 | −84.2 | −34.0 |
| After block 18 alone less without replay, block 18 | B's trials | The odd places' excitatory | the volley | +6.8 | +1.6 | +5.2 | −5.3 | −1.7 | −3.6 |
| After block 18 alone less without replay, block 18 | B's trials | The even places' excitatory | the window | −241.7 | −61.3 | −180.5 | +127.3 | +126.9 | +0.4 |
| After block 18 alone less without replay, block 18 | B's trials | The even places' excitatory | the volley | +1.6 | −1.0 | +2.5 | 0.0 | +2.3 | −2.3 |
| After block 18 alone less without replay, block 18 | B's trials | Inhibitory | the window | −75.0 | +6.9 | −82.0 | +95.5 | −87.3 | +182.7 |
| After block 18 alone less without replay, block 18 | B's trials | Inhibitory | the volley | −1.5 | −0.5 | −1.0 | +3.1 | −2.9 | +6.1 |
| After block 18 alone less without replay, block 18 | B's trials | Excitatory less inhibitory | the window | −259.6 | −21.4 | −238.2 | −95.5 | +121.5 | −217.1 |
| After block 18 alone less without replay, block 18 | B's trials | Excitatory less inhibitory | the volley | +539.5 | +528.9 | +10.6 | +518.7 | +530.5 | −11.8 |
| After block 24 alone less without replay, block 24 | A's trials | A's kinds | the window | +0.1 | 0.0 | +0.1 | +0.1 | 0.0 | 0.0 |
| After block 24 alone less without replay, block 24 | A's trials | A's kinds | the volley | +490.9 | +488.6 | +2.3 | +687.9 | +687.0 | +0.9 |
| After block 24 alone less without replay, block 24 | A's trials | B's kinds | the window | −32.2 | −17.1 | −15.1 | −31.7 | −25.0 | −6.7 |
| After block 24 alone less without replay, block 24 | A's trials | B's kinds | the volley | −0.9 | −0.8 | −0.1 | −0.9 | −0.8 | 0.0 |
| After block 24 alone less without replay, block 24 | A's trials | The odd places' excitatory | the window | −551.7 | −141.4 | −410.3 | +70.0 | +109.3 | −39.4 |
| After block 24 alone less without replay, block 24 | A's trials | The odd places' excitatory | the volley | −2.4 | −1.5 | −0.8 | −1.2 | +1.0 | −2.2 |
| After block 24 alone less without replay, block 24 | A's trials | The even places' excitatory | the window | +37.1 | +59.5 | −22.4 | −299.6 | −147.4 | −152.1 |
| After block 24 alone less without replay, block 24 | A's trials | The even places' excitatory | the volley | −2.0 | +0.8 | −2.8 | +0.7 | −2.4 | +3.2 |
| After block 24 alone less without replay, block 24 | A's trials | Inhibitory | the window | +263.4 | +21.2 | +242.2 | +314.5 | +82.8 | +231.7 |
| After block 24 alone less without replay, block 24 | A's trials | Inhibitory | the volley | +4.5 | +0.1 | +4.4 | +0.8 | +1.9 | −1.1 |
| After block 24 alone less without replay, block 24 | A's trials | Excitatory less inhibitory | the window | −814.7 | −121.5 | −693.3 | −571.6 | −141.5 | −430.1 |
| After block 24 alone less without replay, block 24 | A's trials | Excitatory less inhibitory | the volley | +480.8 | +486.9 | −6.1 | +685.7 | +682.8 | +2.9 |
| After block 24 alone less without replay, block 24 | B's trials | A's kinds | the window | −7.1 | −4.3 | −2.8 | −15.0 | −6.9 | −8.1 |
| After block 24 alone less without replay, block 24 | B's trials | A's kinds | the volley | −1.4 | −0.6 | −0.8 | −0.6 | −0.7 | +0.1 |
| After block 24 alone less without replay, block 24 | B's trials | B's kinds | the window | +0.3 | 0.0 | +0.3 | −0.1 | 0.0 | −0.1 |
| After block 24 alone less without replay, block 24 | B's trials | B's kinds | the volley | +638.5 | +637.4 | +1.1 | +699.8 | +698.3 | +1.5 |
| After block 24 alone less without replay, block 24 | B's trials | The odd places' excitatory | the window | −115.4 | +62.8 | −178.2 | −95.2 | −73.1 | −22.1 |
| After block 24 alone less without replay, block 24 | B's trials | The odd places' excitatory | the volley | −0.9 | +1.9 | −2.8 | −3.1 | −1.6 | −1.5 |
| After block 24 alone less without replay, block 24 | B's trials | The even places' excitatory | the window | −153.5 | −62.4 | −91.1 | +68.9 | +127.4 | −58.5 |
| After block 24 alone less without replay, block 24 | B's trials | The even places' excitatory | the volley | −3.3 | −0.9 | −2.4 | +0.9 | +2.4 | −1.5 |
| After block 24 alone less without replay, block 24 | B's trials | Inhibitory | the window | −96.2 | +7.4 | −103.6 | +53.7 | −89.5 | +143.2 |
| After block 24 alone less without replay, block 24 | B's trials | Inhibitory | the volley | −2.6 | +0.1 | −2.7 | −0.5 | −2.4 | +1.9 |
| After block 24 alone less without replay, block 24 | B's trials | Excitatory less inhibitory | the window | −178.1 | −10.8 | −167.3 | −99.6 | +135.7 | −235.4 |
| After block 24 alone less without replay, block 24 | B's trials | Excitatory less inhibitory | the volley | +635.6 | +637.8 | −2.2 | +697.1 | +700.8 | −3.7 |

On A's trials, right after the four single nights, on the image from the assignment and then, after the semicolon, on the image from the
mirrored assignment:
- the volley's change from A's kinds, +685.9, +624.3, +597.0 and +490.9; +670.3, +812.0, +792.1 and +687.9; of which by the weights
  +687.1, +622.0, +594.2 and +488.6; +667.7, +810.2, +792.4 and +687.0;
- the window's from the odd places, −279.1, −433.0, −403.0 and −551.7; −86.3, +53.7, +64.1 and +70.0; of which by the weights
  −90.9, −121.7, −115.8 and −141.4; +114.2, +130.2, +110.8 and +109.3;
- the window's from the even places, +207.3, +163.3, +97.6 and +37.1; −356.2, −326.7, −264.6 and −299.6; of which by the weights
  +93.3, +74.1, +68.9 and +59.5; −142.2, −140.5, −150.2 and −147.4;
- the window's from the inhibitory units, +319.3, +270.2, +264.3 and +263.4; +261.8, +357.6, +405.9 and +314.5; of which by the weights
  −15.9, −8.5, +22.2 and +21.2; +68.4, +94.1, +73.9 and +82.8.

A's answer's readout is the even places' readout on the image from the assignment and the odd places' on the image from the mirrored
assignment.

**The first night against the later three**, on A's trials, A's answer's readout less the other, right after the nights after block 6,
12, 18 and 24 alone, on the image from the assignment and then, after the semicolon, on the image from the mirrored assignment:
- the window's drive, excitatory less inhibitory, −417.2, −581.1, −606.8 and −814.7 k; −692.2, −606.2, −596.5 and −571.6: after the first
  night the smallest fall of the four on the image from the assignment and the largest on the image from the mirrored assignment;
- the volley's from A's kinds, +685.9, +624.3, +597.0 and +490.9; +670.3, +812.0, +792.1 and +687.9: after the first night the largest rise
  on the image from the assignment and the smallest on the image from the mirrored assignment;
- the window's from A's answer's readout's own place group, the even places on the image from the assignment and the odd on the image from
  the mirrored assignment, +207.3, +163.3, +97.6 and +37.1; −86.3, +53.7, +64.1 and +70.0;
- the two spans' sum, +271.0, +33.7, −2.9 and −333.9; −27.1, +213.7, +188.2 and +114.1.

**H-47's plain night on all four nights**, the answer's readout less the other, excitatory less inhibitory:

| Comparison | What | Span | From the assignment: on A's trials, A's answer's readout less B's | on B's trials, B's answer's less A's | From the mirrored assignment: on A's trials | on B's trials |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: |
| Plain on all four nights less without replay, block 6 | Spikes, the readouts' counts | – | +0.48 ± 0.29 | −0.80 ± 0.22 | −0.84 ± 0.21 | −0.21 ± 0.30 |
| Plain on all four nights less without replay, block 6 | Drive from excitatory less inhibitory, k | the window | −417.2 ± 34.0 | −225.0 ± 23.1 | −692.2 ± 22.0 | −196.0 ± 20.8 |
| Plain on all four nights less without replay, block 6 | Drive from excitatory less inhibitory, k | the volley | +688.3 ± 11.4 | +623.3 ± 14.2 | +665.1 ± 10.9 | +546.5 ± 11.6 |
| Plain on all four nights less without replay, block 6 | Drive from excitatory less inhibitory, k | both | +271.0 ± 42.6 | +398.3 ± 26.0 | −27.1 ± 24.0 | +350.4 ± 27.3 |
| Plain on all four nights less without replay, block 12 | Spikes, the readouts' counts | – | +0.22 ± 0.43 | −0.51 ± 0.52 | −0.87 ± 0.25 | +0.03 ± 0.19 |
| Plain on all four nights less without replay, block 12 | Drive from excitatory less inhibitory, k | the window | −849.4 ± 34.0 | −519.7 ± 38.7 | −1 133.8 ± 27.9 | −312.0 ± 32.0 |
| Plain on all four nights less without replay, block 12 | Drive from excitatory less inhibitory, k | the volley | +1 126.0 ± 47.0 | +1 138.7 ± 57.5 | +1 294.8 ± 10.1 | +936.1 ± 28.8 |
| Plain on all four nights less without replay, block 12 | Drive from excitatory less inhibitory, k | both | +276.6 ± 67.3 | +619.0 ± 36.4 | +161.0 ± 36.1 | +624.1 ± 38.1 |
| Plain on all four nights less without replay, block 18 | Spikes, the readouts' counts | – | −1.18 ± 0.30 | −0.83 ± 0.65 | +0.68 ± 0.20 | +2.72 ± 0.39 |
| Plain on all four nights less without replay, block 18 | Drive from excitatory less inhibitory, k | the window | −1 181.7 ± 36.7 | −795.9 ± 48.5 | −1 362.3 ± 28.6 | −368.5 ± 20.3 |
| Plain on all four nights less without replay, block 18 | Drive from excitatory less inhibitory, k | the volley | +1 489.5 ± 28.9 | +1 441.1 ± 42.7 | +1 869.9 ± 19.4 | +1 467.1 ± 61.7 |
| Plain on all four nights less without replay, block 18 | Drive from excitatory less inhibitory, k | both | +307.7 ± 61.6 | +645.2 ± 27.6 | +507.6 ± 42.8 | +1 098.6 ± 67.9 |
| Plain on all four nights less without replay, block 24 | Spikes, the readouts' counts | – | −1.52 ± 0.27 | −0.43 ± 0.59 | +1.50 ± 0.45 | +3.10 ± 0.40 |
| Plain on all four nights less without replay, block 24 | Drive from excitatory less inhibitory, k | the window | −1 677.8 ± 32.4 | −1 044.9 ± 56.6 | −1 404.8 ± 53.7 | −370.0 ± 36.9 |
| Plain on all four nights less without replay, block 24 | Drive from excitatory less inhibitory, k | the volley | +1 920.1 ± 50.1 | +1 723.9 ± 33.9 | +2 341.3 ± 40.8 | +1 717.1 ± 66.7 |
| Plain on all four nights less without replay, block 24 | Drive from excitatory less inhibitory, k | both | +242.3 ± 41.2 | +679.0 ± 66.4 | +936.5 ± 80.7 | +1 347.2 ± 81.8 |
| Plain on all four nights less without replay, block 36 | Spikes, the readouts' counts | – | +0.17 ± 0.31 | +0.65 ± 0.57 | +1.39 ± 0.49 | +3.80 ± 0.35 |
| Plain on all four nights less without replay, block 36 | Drive from excitatory less inhibitory, k | the window | −1 695.6 ± 61.1 | −1 146.8 ± 35.7 | −1 320.4 ± 26.8 | −475.3 ± 36.2 |
| Plain on all four nights less without replay, block 36 | Drive from excitatory less inhibitory, k | the volley | +2 027.4 ± 65.3 | +1 830.8 ± 105.5 | +2 375.6 ± 36.8 | +1 774.3 ± 61.6 |
| Plain on all four nights less without replay, block 36 | Drive from excitatory less inhibitory, k | both | +331.8 ± 38.8 | +683.9 ± 95.5 | +1 055.3 ± 45.3 | +1 299.0 ± 90.2 |

On A's trials, at block 6, 12, 18, 24 and 36, on the image from the assignment and then, after the semicolon, on the image from the mirrored
assignment:
- the volley's, +688.3 ± 11.4, +1 126.0 ± 47.0, +1 489.5 ± 28.9, +1 920.1 ± 50.1 and +2 027.4 ± 65.3 k; +665.1 ± 10.9, +1 294.8 ± 10.1,
  +1 869.9 ± 19.4, +2 341.3 ± 40.8 and +2 375.6 ± 36.8;
- the window's, −417.2 ± 34.0, −849.4 ± 34.0, −1 181.7 ± 36.7, −1 677.8 ± 32.4 and −1 695.6 ± 61.1; −692.2 ± 22.0, −1 133.8 ± 27.9,
  −1 362.3 ± 28.6, −1 404.8 ± 53.7 and −1 320.4 ± 26.8;
- their sum, +271.0 ± 42.6, +276.6 ± 67.3, +307.7 ± 61.6, +242.3 ± 41.2 and +331.8 ± 38.8; −27.1 ± 24.0, +161.0 ± 36.1, +507.6 ± 42.8,
  +936.5 ± 80.7 and +1 055.3 ± 45.3;
- the spikes, +0.48 ± 0.29, +0.22 ± 0.43, −1.18 ± 0.30, −1.52 ± 0.27 and +0.17 ± 0.31; −0.84 ± 0.21, −0.87 ± 0.25, +0.68 ± 0.20,
  +1.50 ± 0.45 and +1.39 ± 0.49.

## What it costs

The first trial took 638 s and the second 641 s. The four jobs took 6 013 and 7 512 s on the image from the
assignment and 5 816 and 7 695 s on the image from the mirrored assignment, 100.2, 125.2, 96.9 and 128.3 minutes, the
longest 53.4 per cent of a job's 240 minutes; the trial projected 5 844, 7 954, 5 993 and 7 514 s.
Explorations 034's and 035's jobs took 5 822, 7 925, 5 971 and 7 485 s. Nothing was cut: each job ran its forms on the three paths, every
night watched, every read of A counted for every unit in both spans and every image's inputs read.

The four logs are 28.2, 36.6, 28.0 and 36.3 MB, 17.2, 23.0, 17.1 and 22.9 MB
more than explorations 034's and 035's, 11.0, 13.5, 10.9 and 13.4 MB. ADR-0251 put the addition at
about 9 to 13 MB a job, for the window alone. Before the jobs ran, the second trial's lines gave about 16.7 MB for job one and
22.3 MB for job two, each added kind of line's mean size times the job's count of it, an image's unit drive taken one character a
number longer for its sixteen reads.

## What it suggests

**On both cells each single replayed night moved the two spans' drive the same way**: in the window both readouts lost drive on both
stimuli's trials, each place group's fall mostly in the spikes per weight, which carries the presynaptic units' spikes, and those units
fired less; the gain at the trial's end was lower too, and the drive split here leaves it out. In the volley the trial's stimulus's kinds'
drive moved toward its answer's readout, by the weights. On A's trials A's answer's readout less the other lost in the window and gained in
the volley on both cells. What differs between the cells is the size of the two, and their sum lies higher on the cell whose spikes
difference lies higher in each of the twelve distinct comparisons on A's trials.

Questions this suggests for a later exploration or a round:
- Why the presynaptic units of both readouts fire less in the window after a replayed night: the gain at the trial's end was lower, and
  each readout's inhibitory input higher, but neither is read here against a unit's spikes trial by trial.
- What sets the volley's rise from A's kinds onto A's answer's readout less the other higher on the image from the mirrored assignment after
  the three later single nights, +670.3, +812.0, +792.1 and +687.9 k against +685.9, +624.3, +597.0 and +490.9: it lies in the weights,
  which exploration 035 read before each night.
- Whether the drive with each spike's arrival after its synapse's delay moves drive from the volley into the window, or from the window
  past the trial's end: the drive pairs a weight with its presynaptic unit's spikes in the same span.
- Why at block 36 the cells part mostly by the spikes per weight, +245.2, +516.0, +376.3 and +350.0 k, where right after two of the later
  three nights the parts by the weights differed more.
- Whether the order of the two cells' sums and spikes holds on the other ten cells, and from read to read within a cell.

## What did not run

- **Any other cell**: the ten cells but the two eighth images of seed 24.
- **Replay on two of the four nights, or on every night but one.**
- **Any hold**, on any night.
- **Plain nights on both mappings and the awake gap.**
- **The pair's reads unit by unit, or the pair's drive**: they are read as before, their counts and masks set beside the logs'.
- **The inputs of the units outside the readouts' sets**: their spikes are counted, the weights onto them are not read.
- **A spike's arrival after its synapse's delay**: the drive pairs a weight with its presynaptic unit's spikes in the same span, the window
  or the volley.
- **A unit's membrane or threshold.**
- **Any path but H-47's three**, 0, 5 and 6.
- **A night split apart**, such as slow-wave sleep read apart from REM.
- **The next day's reading of the pairings a night leaves** pending.
- **Any SE for a unit's drive or for an input**: a unit's drive is summed over its sixteen reads and an input is the mean of three images;
  the correlations and the split carry none either.
- **A table of each of the 918 readout units' drives**: the tables sum the readouts, the classes and the named sets; every unit's drive is
  in the jobs' logs.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_what_reaches_the_readout_units_on_both_cells_explore` | [43187226229](https://github.com/Menteion/Menteion/actions/runs/43187226229) | `b0f3229` | 638 |
| the four jobs below, cancelled | [43189318519](https://github.com/Menteion/Menteion/actions/runs/43189318519) | `b0f3229` | – |
| `the_harness_of_what_reaches_the_readout_units_on_both_cells_explore` | [43190778334](https://github.com/Menteion/Menteion/actions/runs/43190778334) | `816d89e` | 641 |
| `what_reaches_the_readout_units_from_the_assignment_after_block_6_alone_and_on_every_night_explore` | [43192758106](https://github.com/Menteion/Menteion/actions/runs/43192758106) | `816d89e` | 6 013 |
| `what_reaches_the_readout_units_from_the_assignment_after_each_later_night_alone_explore` | [43192758106](https://github.com/Menteion/Menteion/actions/runs/43192758106) | `816d89e` | 7 512 |
| `what_reaches_the_readout_units_from_the_mirrored_assignment_after_block_6_alone_and_on_every_night_explore` | [43192758106](https://github.com/Menteion/Menteion/actions/runs/43192758106) | `816d89e` | 5 816 |
| `what_reaches_the_readout_units_from_the_mirrored_assignment_after_each_later_night_alone_explore` | [43192758106](https://github.com/Menteion/Menteion/actions/runs/43192758106) | `816d89e` | 7 695 |
