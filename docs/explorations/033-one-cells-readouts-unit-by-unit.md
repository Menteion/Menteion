---
status: exploratory
date: 2026-11-11
---

# 033: One cell's readouts, unit by unit

**Branch**: `explore/033-one-cells-readouts-unit-by-unit`
**Commit**: `397b900`, which adds the harness, on `b733e79`: `main` at `99bcc67`, ADR-0248's merge, with exploration 032's tag merged
in, which changes nothing under `src/`. This branch changes nothing under `src/`, so the engine is `main`'s.
**Runs**: [43010140120](https://github.com/Menteion/Menteion/actions/runs/43010140120) (the harness's trial) and
[43011654795](https://github.com/Menteion/Menteion/actions/runs/43011654795) (two jobs), both at `scope=arms`. The logs set beside them are
H-47's run [41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613) for the night without replay and H-47's plain night
on A's patterns, exploration 031's run [42666579780](https://github.com/Menteion/Menteion/actions/runs/42666579780) for replay after block 6
alone, exploration 032's run [42971972683](https://github.com/Menteion/Menteion/actions/runs/42971972683) for replay after block 12, 18 or
24 alone, and exploration 020's run [41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777) for H-47's two forms' class
matrices and night lines on this cell. None of those was run again.

[ADR-0248](../adr/0248-one-cells-readouts-unit-by-unit.md) ordered this exploration. On seed 24's eighth image from the assignment, H-47's
plain night on A's patterns replayed after block 12, 18 or 24 alone, every other night without replay, left A's count below the night
without replay right after its replayed night, −7.3 ± 0.88, −12.1 ± 0.63 and −12.3 ± 0.58, on A's trials by −5.2 ± 0.70, −8.1 ± 0.66
and −10.0 ± 0.33 and on B's by −2.1 ± 0.59, −4.0 ± 0.57 and −2.3 ± 0.62, each beyond 2 SE ([exploration
032](032-each-later-nights-replay-alone-on-twelve-cells.md)). Replayed after block 6 alone it read +0.3 ± 0.64 there ([exploration
031](031-the-first-nights-replay-on-twelve-cells.md)). A read gives each readout's spike count on each trial, not which of its units fired.
This note counts each readout unit's spikes on each of A's trials on that one cell, under six forms, and asks, at blocks 6, 12, 18, 24 and
36:

1. whether every read under each form is the read of the same form, path, checkpoint and stream in the logs, so that A's count and its
   differences are those notes' and are not read anew;
2. each readout unit's spikes a trial, on A's trials and on B's, under each form, and each form less the night without replay;
3. right after each replayed night, which units moved, on A's trials and on B's, by readout and by class, how each readout's spikes moved,
   and how few units carry half of each readout's change;
4. the same at block 36, each unit beside its difference right after the replayed night, and each single night against H-47's plain night;
5. beside each, the class matrix by group over the replayed night, from the logs.

The note writes no prediction and decides nothing. **Every figure here is exploratory**: from one run of each test on the repository's
AArch64 host, in H-47's terms. A mean is over the cell's 48 reads, sixteen on each of three paths, and a difference carries its paired SE
unless another is named. A difference is written "beyond 2 SE" where it lies beyond twice its SE; one written without it carries its sign
alone.

**In brief**
- **Every row and read of the six forms is the logs'.** Inside the jobs every row the test holds to H-47's held, and each form's fifteen
  rows are the rows of the same form in the logs it is set beside, 105 of 105; every read of A and of the pair, 3 360 in all, carries the
  logs' count and masks and its whole line, label aside. The night without replay ran the same in both jobs, its unit reads too, and every
  unit read's sums are its read's own spike counts by stimulus and readout.
- **Right after each single replayed night both readouts fired less on both stimuli's trials**, each beyond 2 SE: on A's trials readout 1 by
  7.45 to 10.19 spikes a trial and readout 0 by 6.03 to 7.92, on B's trials readout 0 by 6.27 to 6.77 and readout 1 by 5.31 to 6.21, 18 to
  33 per cent of the night without replay's spikes at the same checkpoint. Every class of both readouts fired less, each beyond 2 SE.
- **The answer's readout less the other moved with each stimulus's correct trials.** On A's trials readout 1 less readout 0 read
  +0.48 ± 0.29 against the night without replay after block 6 alone, within 2 SE, and −1.46 ± 0.22, −2.96 ± 0.19 and −3.73 ± 0.16 after
  block 12, 18 and 24 alone, beyond 2 SE; on B's trials readout 0 less readout 1 lay below by 0.56 to 1.43 under each of the four, beyond 2
  SE. In every comparison of sections 3 and 4, on each stimulus's trials, its sign is the sign of that stimulus's correct trials as the
  notes give them.
- **Many units moved, and few carry half the change.** Right after each single replayed night 95 to 136 of a readout's 459 units moved down
  beyond twice their own SE and 0 to 5 up, where about 23 would by chance in the two directions together, 11 or 12 each way; 13 to 19 units
  carry half of each readout's drop, about as many as carry half of the night without replay's spikes, 14 to 19, and 3 to 9 of them are
  among those. Six units of readout 1 and four of readout 0 on A's trials, and six of readout 1 and one of readout 0 on B's, are among the
  fewest under all four single nights.
- **By class, on A's trials**, after block 12, 18 or 24 alone readout 1's pattern targets fired 4.63 to 5.14 less a trial and its inhibitory
  units 3.75 to 4.12 less, where after block 6 alone 3.70 and 2.86 less; readout 0's pattern targets fired 4.80, 4.27 and 3.37 less, where
  after block 6 alone 5.47 less.
- **At block 36 each single night still lay below the night without replay on both readouts and both stimuli's trials**, by 1.28 to 4.49
  spikes a trial, each beyond 2 SE, having come back by 3.10 to 5.84 since right after its replayed night. Readout 1 less readout 0 on A's
  trials lay below beyond 2 SE after block 18 and 24 alone, −1.69 ± 0.12 and −2.22 ± 0.31, and within 2 SE after block 6 and 12 alone. The
  units' differences at block 36 correlate with theirs right after by 0.34 to 0.98.
- **H-47's plain night on all four nights** lowered both readouts further with each night: on A's trials readout 1 by 7.45, 9.92, 13.12 and
  14.75 at blocks 6, 12, 18 and 24, and by 10.26 at block 36, the other three readouts' by 7.23 to 10.43 there, each beyond 2 SE. Its
  readout 1 less readout 0 on A's trials lay within 2 SE of the night without replay's at blocks 6, 12 and 36 and below beyond 2 SE at 18
  and 24. Each single night lay above it at block 36 on both readouts and both stimuli's trials, by 4.43 to 8.16, each beyond 2 SE.
- **A's count** and each stimulus's correct trials are the notes' figures, since every read is the logs'. After block 6 alone less H-47's
  plain night, now both in job one, reads +2.0 ± 0.73, +6.4 ± 1.19, +3.3 ± 2.01 and −3.8 ± 2.01 at blocks 12, 18, 24 and 36 with the
  paired SE, where exploration 031 gave ± 0.88, 1.70, 2.22 and 2.14 with exploration 018's unpaired one.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 020 to 032 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. Nights follow blocks 6, 12, 18 and 24, then 12 blocks run with
none to block 36, on paths 0, 5 and 6. Each checkpoint reads A and the pair sixteen times each, a read one block of 64 trials on a fork of
the image with the reward withheld.

**One cell**, seed 24's eighth image from the assignment, the one cell where each later night's replay alone lay below the night without
replay right after it (exploration 032). There A asks readout 1, the even places, of stimulus A and readout 0, the odd places, of B. A
period of the ring is 20 units; stimulus A presents place 0 of each period and B place 11. **The odd places** are the odd places but 11 and
**the even places** the even places but 0: H-47's readout 0 and readout 1, nine places each over 51 periods, 459 units each.

**The readouts' units by class**, as exploration 020's class matrix names them, from the start image (`e020_class`, which the class
matrix itself now calls): a pattern target is a unit that one of A's patterns, drawn from the start image, reaches past its first unit.

| Readout | Class | Places | How many units |
| :--- | :--- | :--- | ---: |
| readout 1, the even places | D's own | 16 | 25 |
| readout 1, the even places | readout 1's pattern targets | 2, 6, 8, 10, 12, 16, 18 | 252 |
| readout 1, the even places | readout 1's other excitatory | 2, 6, 8, 10, 12, 16, 18 | 80 |
| readout 1, the even places | inhibitory in readout 1 | 4, 14 | 102 |
| readout 0, the odd places | C's own | 5 | 25 |
| readout 0, the odd places | readout 0's pattern targets | 1, 3, 5, 7, 13, 15, 17 | 245 |
| readout 0, the odd places | readout 0's other excitatory | 1, 3, 5, 7, 13, 15, 17 | 87 |
| readout 0, the odd places | inhibitory in readout 0 | 9, 19 | 102 |

**Six forms, in two jobs.** A night that replays is H-47's plain night on A's patterns: A's 102 patterns, drawn from the start image, tagged
at priority 200. Every other night is H-47's night without replay, the same night with nothing tagged. No hold is set. Each night is chosen
by its form (`E030Form`, exploration 030's), and every night of every form is watched.

| Form | In the logs | Night after block 6 | after 12 | after 18 | after 24 | Job | Rows held inside its job to H-47's, on every path | Its rows and reads in the logs |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Without replay | `no-replay` | without replay | without replay | without replay | without replay | both | the night without replay's, blocks 6 to 36 | H-47's |
| After block 6 alone | `alone-6` | replays | without replay | without replay | without replay | one | the plain night's, block 6 | exploration 031's |
| Plain on all four nights (H-47) | `plain-first` | replays | replays | replays | replays | one | the plain night's, blocks 6 to 36 | H-47's |
| After block 12 alone | `alone-12` | without replay | replays | without replay | without replay | two | the night without replay's, block 6 | exploration 032's |
| After block 18 alone | `alone-18` | without replay | without replay | replays | without replay | two | the night without replay's, blocks 6 and 12 | exploration 032's |
| After block 24 alone | `alone-24` | without replay | without replay | without replay | replays | two | the night without replay's, blocks 6, 12 and 18 | exploration 032's |

**The unit read.** In each of A's reads, on each trial, each unit of the two readouts' sets has its spikes counted in that trial's readout
window, ticks 100 to 599 of the trial, from the executor's own train (`Executor::train`), by `Readout::count_window`'s rule; the harness's
per-trial observer receives the executor after each trial. A unit's spikes are summed over the read's trials of each stimulus and divided by
that stimulus's trials in the read, for **a unit's spikes a trial**. A read's trials of each stimulus are the same in every form, since a
read's stimuli follow from its index alone: 28 to 37 trials of stimulus A and 27 to 36 of B in a read. **A readout's spikes a trial** is the
sum over its units, the readout's count in the trial's window, which the engine selects on; it is not A's count, which counts correct
trials. The pair's reads are read as before. Inside the test, as a guard, on each trial of each unit read: the train holds the whole window
(it has let go no spike, or its oldest spike is from a tick before the window opens), and each readout's units' spikes sum to that readout's
count of the trial (`Outcome::counts`). The read's own line carries each readout's spikes summed over each stimulus's trials, and the unit
read's sums are those, read by read.

**The SEs.** A form less the night without replay carries H-47's paired SE over its 48 differences, the night without replay of its own job;
so does after block 6 alone less H-47's plain night, both in the first job. After block 12, 18 or 24 alone less H-47's plain night, across
the two jobs, carries exploration 018's unpaired SE: the mean over the paths of each path's difference of means, ± √((the two forms' stream
variances)/16 + (the paths' differences' SD)²/3), a negative variance as zero. A unit's own difference carries its own SE, worked the same
way from that unit's 48 values.

**What moved.** A unit **moved** where its difference lies beyond twice its own SE, up or down. About one unit in twenty would lie there by
chance in each comparison, some 23 of a readout's 459 in the two directions together, 11 or 12 each way, and a count of units
moved is read against that. Where a readout's own
difference lies beyond 2 SE: **the fewest units** carrying half of it, the units ranked by their difference in the direction of the
readout's net change, taken until their differences reach half the net change; **the gross change**, the sum of the units' differences'
magnitudes; and **the units moved against** the net change, with their differences' sum. A readout's difference is the sum of its units'
differences, so the net change is the readout's difference.

**A checkpoint is read after the night that follows its block**, and block 36's after the twelve blocks that follow the last night. So right
after its replayed night, a single night's form less the night without replay is that night's replay alone: until then the two ran the same
nights on the same images. H-47's plain night less the night without replay at blocks 6, 12, 18 and 24 is its nights so far together.

## Before a job ran

ADR-0248 asks whether any form must run bit for bit as one already in the logs. **Each of the six does**: the night without replay and
H-47's plain night are H-47's job's, after block 6 alone exploration 031's job's, and after block 12, 18 or 24 alone exploration 032's,
every image and read of them in those logs. **No log holds a unit's spikes**, so each form ran again, and its rows and reads are set beside
the logs' here.

## The runs are laid out as written

- **The trial** ran on path 0 of the cell: the pair's head and the first night of each of the six forms, each watched, each image split and
  read once as it was, of A and of the pair, A's read counted unit by unit. Inside the test each image, those two reads, A's couplings,
  their highest over the head and the gain held H-47's row at block 6 on path 0 of the form whose first night it ran:

| Form | Its first night | H-47's row at block 6 on path 0 it holds, inside the test | A's read as it was | The pair's | Replays | Depotentiations | Watch: wrong | Its night's lines that are exploration 020's job's same night on path 0, label and its notice aside | Trials of A, of B | A's trials: readout 1's units' spikes summed over them, readout 0's | B's trials: readout 1's, readout 0's | Those sums are |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | :--- | :--- | ---: | ---: | :--- |
| Without replay | without replay | the night without replay's | 43 | 60 | 0 | 0 | 0 | 89 of 89 | 34, 30 | 953, 846 | 735, 798 | the read's |
| After block 6 alone | replays | the plain night's | 42 | 63 | 1 329 | 640 | 0 | 395 of 395 | 34, 30 | 656, 564 | 572, 640 | the read's |
| Plain on all four nights (H-47) | replays | the plain night's | 42 | 63 | 1 329 | 640 | 0 | 395 of 395 | 34, 30 | 656, 564 | 572, 640 | the read's |
| After block 12 alone | without replay | the night without replay's | 43 | 60 | 0 | 0 | 0 | 89 of 89 | 34, 30 | 953, 846 | 735, 798 | the read's |
| After block 18 alone | without replay | the night without replay's | 43 | 60 | 0 | 0 | 0 | 89 of 89 | 34, 30 | 953, 846 | 735, 798 | the read's |
| After block 24 alone | without replay | the night without replay's | 43 | 60 | 0 | 0 | 0 | 89 of 89 | 34, 30 | 953, 846 | 735, 798 | the read's |

- Each night's lines, its start, its windows, its watch and its end, are the lines of exploration 020's job's night after block 6 on path 0
  of the same cell, the night without replay's for the four forms whose first night is without replay and the plain night's for the two
  whose first night replays, label and form aside, each night's notice that it read what its form asks left out, which exploration 020's job
  did not print: 89 of 89 for each of the four and 395 of 395 for each of the two. So is the head's line, 1 of 1. Each night without replay
  read 0 replays and 0 depotentiations and each replayed night 1 329 and 640, as the trial prints, and the watch read `wrong 0` in all six.
  Each unit read's trials of each stimulus are its read's, and its units' spikes summed by readout are the read's own spikes on each
  stimulus's trials, the sums in the table.

- **The two jobs** ran to the end, each form on the three paths. Inside each test every row of the night without replay and of H-47's plain
  night held H-47's row of that form, fifteen each; after block 6 alone's row at block 6 held the plain night's; and after block 12, 18 or
  24 alone's rows before its replayed night's block held the night without replay's, three, six and nine. Every form's replayed nights read
  replays and depotentiations that are not zero and its other nights `replays 0` and `depotentiations 0`, held inside the test:

| Job | Form | Rows held inside the test to H-47's, on every path | Of its 15 rows, H-47's night without replay's and its plain night's, as the job prints | Rows that are the logs' rows of the same form | Replayed nights: replays and depotentiations | Other nights reading replays 0 and depotentiations 0 | Watch: wrong |
| :--- | :--- | :--- | ---: | :--- | :--- | :--- | ---: |
| Job one | Without replay | the night without replay's, blocks 6, 12, 18, 24, 36 | 15, 0 | 15 of 15 | 0: –, – | 12 of 12 | 0 |
| Job one | After block 6 alone | the plain night's, blocks 6 | 0, 3 | 15 of 15 | 3: 1 329, 640 | 9 of 9 | 0 |
| Job one | Plain on all four nights (H-47) | the plain night's, blocks 6, 12, 18, 24, 36 | 0, 15 | 15 of 15 | 12: 1 129 to 1 329, 545 to 640 | 0 of 0 | 0 |
| Job two | Without replay | the night without replay's, blocks 6, 12, 18, 24, 36 | 15, 0 | 15 of 15 | 0: –, – | 12 of 12 | 0 |
| Job two | After block 12 alone | the night without replay's, blocks 6 | 3, 0 | 15 of 15 | 3: 1 341, 640 | 9 of 9 | 0 |
| Job two | After block 18 alone | the night without replay's, blocks 6, 12 | 6, 0 | 15 of 15 | 3: 1 341, 640 | 9 of 9 | 0 |
| Job two | After block 24 alone | the night without replay's, blocks 6, 12, 18 | 9, 0 | 15 of 15 | 3: 1 341, 640 | 9 of 9 | 0 |

- **The rows.** Each form's fifteen rows are the rows of the same form in the logs it is set beside, H-47's for the night without replay and
  its plain night, exploration 031's for after block 6 alone and exploration 032's for the other three: 105 of 105. As each job prints, of
  after block 12, 18 and 24 alone's fifteen rows three, six and nine are H-47's night without replay's and none its plain night's; of after
  block 6 alone's, none the night without replay's and three the plain night's; and the night without replay's and the plain night's fifteen
  are each H-47's of that form.
- **The lines.** Of job one's 9 759 lines, its unit reads and its line of the readouts' units aside, 9 746 are lines of exploration 031's
  job or of exploration 020's job's plain night, label aside; the other 13 are the plain night's notices, twelve that a night read what its
  form asks and one of the rows held, which exploration 020's job did not print. Of job two's 9 628, all are lines of exploration 032's job.
  So the class matrices, splits and night lines of the forms here are those jobs', and section 5 reads them there.
- **The night without replay in both jobs**: its 480 reads and its fifteen rows are the same in job one and job two, and so are its 240 unit
  reads.
- **The unit reads**: 720 in job one and 960 in job two, one after each of A's reads. In each, each stimulus's trials are its read's, 28 to
  37 of A and 27 to 36 of B, the same for a stream on a path in every form, and its units' spikes summed by readout are the read's own
  spikes on that stimulus's trials. Inside the test, on every trial, the train held the whole window and each readout's units summed to its
  count.

## 1. The reads against the logs

Each of A's and the pair's reads under each form is set beside the read of the same form, path, checkpoint and stream in the logs, by the
stream (`spread`'s index), not by the order the job printed them:

| Job | Form | Logs set beside | A's reads with the logs' count and masks | their whole line, label aside | The pair's reads with the logs' count and masks | their whole line |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Job one | Without replay | H-47's, run 41441234613 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job one | After block 6 alone | exploration 031's, run 42666579780 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job one | Plain on all four nights (H-47) | H-47's, run 41441234613 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job two | Without replay | H-47's, run 41441234613 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job two | After block 12 alone | exploration 032's, run 42971972683 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job two | After block 18 alone | exploration 032's, run 42971972683 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job two | After block 24 alone | exploration 032's, run 42971972683 | 240 of 240 | 240 of 240 | 240 of 240 | 240 of 240 |

Every read is the logs' read, its count, its `stimuli`, `right` and `tied` masks and its whole line, label aside, under each form and in
both jobs: 3 360 reads, 1 680 of them A's. So A's count and each stimulus's correct trials under each form are those the notes give, read
here from the same reads; the difference less the night without replay carries H-47's paired SE in the form's own job:

| Form | Block | A's count | Without replay's | Less without replay | On A's trials | On B's trials | The note that gives it |
| :--- | ---: | ---: | ---: | :--- | :--- | :--- | :--- |
| After block 6 alone | 6 | 39.1 | 38.8 | +0.3 ± 0.64 | +2.5 ± 0.77 | −2.3 ± 0.71 | exploration 031 |
| After block 6 alone | 12 | 36.2 | 34.8 | +1.4 ± 0.72 | +1.4 ± 1.16 | 0.0 ± 1.27 | exploration 031 |
| After block 6 alone | 18 | 34.9 | 34.4 | +0.4 ± 1.21 | +1.6 ± 0.86 | −1.2 ± 1.69 | exploration 031 |
| After block 6 alone | 24 | 33.0 | 35.6 | −2.6 ± 0.81 | −2.2 ± 0.37 | −0.4 ± 1.03 | exploration 031 |
| After block 6 alone | 36 | 23.6 | 25.6 | −2.0 ± 0.88 | −1.0 ± 0.63 | −1.0 ± 0.71 | exploration 031 |
| Plain on all four nights (H-47) | 6 | 39.1 | 38.8 | +0.3 ± 0.64 | +2.5 ± 0.77 | −2.3 ± 0.71 | H-47 |
| Plain on all four nights (H-47) | 12 | 34.2 | 34.8 | −0.6 ± 1.03 | +0.4 ± 1.31 | −1.0 ± 1.51 | H-47 |
| Plain on all four nights (H-47) | 18 | 28.5 | 34.4 | −5.9 ± 1.53 | −3.6 ± 0.86 | −2.4 ± 1.34 | H-47 |
| Plain on all four nights (H-47) | 24 | 29.6 | 35.6 | −5.9 ± 1.85 | −3.9 ± 0.78 | −2.0 ± 1.34 | H-47 |
| Plain on all four nights (H-47) | 36 | 27.4 | 25.6 | +1.8 ± 1.79 | +0.2 ± 0.76 | +1.6 ± 1.50 | H-47 |
| After block 12 alone | 6 | 38.8 | 38.8 | = without replay | = without replay | = without replay | exploration 032 |
| After block 12 alone | 12 | 27.5 | 34.8 | −7.3 ± 0.88 | −5.2 ± 0.70 | −2.1 ± 0.59 | exploration 032 |
| After block 12 alone | 18 | 29.9 | 34.4 | −4.6 ± 1.83 | −2.6 ± 0.46 | −1.9 ± 1.61 | exploration 032 |
| After block 12 alone | 24 | 29.0 | 35.6 | −6.5 ± 0.42 | −3.6 ± 0.95 | −3.0 ± 0.89 | exploration 032 |
| After block 12 alone | 36 | 23.5 | 25.6 | −2.1 ± 1.78 | −1.2 ± 0.90 | −0.9 ± 1.04 | exploration 032 |
| After block 18 alone | 6 | 38.8 | 38.8 | = without replay | = without replay | = without replay | exploration 032 |
| After block 18 alone | 12 | 34.8 | 34.8 | = without replay | = without replay | = without replay | exploration 032 |
| After block 18 alone | 18 | 22.3 | 34.4 | −12.1 ± 0.63 | −8.1 ± 0.66 | −4.0 ± 0.57 | exploration 032 |
| After block 18 alone | 24 | 26.7 | 35.6 | −8.8 ± 0.82 | −6.5 ± 0.94 | −2.4 ± 0.83 | exploration 032 |
| After block 18 alone | 36 | 20.1 | 25.6 | −5.5 ± 0.95 | −4.1 ± 0.51 | −1.4 ± 0.88 | exploration 032 |
| After block 24 alone | 6 | 38.8 | 38.8 | = without replay | = without replay | = without replay | exploration 032 |
| After block 24 alone | 12 | 34.8 | 34.8 | = without replay | = without replay | = without replay | exploration 032 |
| After block 24 alone | 18 | 34.4 | 34.4 | = without replay | = without replay | = without replay | exploration 032 |
| After block 24 alone | 24 | 23.3 | 35.6 | −12.3 ± 0.58 | −10.0 ± 0.33 | −2.3 ± 0.62 | exploration 032 |
| After block 24 alone | 36 | 20.9 | 25.6 | −4.6 ± 0.73 | −5.5 ± 0.75 | +0.9 ± 0.13 | exploration 032 |

After block 6 alone and H-47's plain night now share job one, so their difference in A's count carries the paired SE beside exploration
031's unpaired one; the means are the same:

| Block | Exploration 018's unpaired SE, from these two forms | As exploration 031 gives it | Paired SE, both in job one |
| :--- | :--- | :--- | :--- |
| 6 | = plain | = plain | = plain |
| 12 | +2.0 ± 0.88 | +2.0 ± 0.88 | +2.0 ± 0.73 |
| 18 | +6.4 ± 1.70 | +6.4 ± 1.70 | +6.4 ± 1.19 |
| 24 | +3.3 ± 2.22 | +3.3 ± 2.22 | +3.3 ± 2.01 |
| 36 | −3.8 ± 2.14 | −3.8 ± 2.14 | −3.8 ± 2.01 |

## 2. Each readout's units under each form

**Each readout's spikes a trial**, the sum over its 459 units, on A's trials and on B's, the mean over the 48 reads. On this cell
A asks readout 1 of stimulus A and readout 0 of B:

| Form | Block | A's trials: readout 1, the even places | readout 0, the odd places | B's trials: readout 1 | readout 0 |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Without replay | 6 | 27.34 | 24.84 | 24.49 | 25.77 |
| Without replay | 12 | 28.83 | 27.66 | 26.86 | 27.70 |
| Without replay | 18 | 30.98 | 29.84 | 28.73 | 29.47 |
| Without replay | 24 | 32.96 | 31.22 | 30.64 | 31.07 |
| Without replay | 36 | 29.44 | 30.05 | 29.41 | 28.66 |
| After block 6 alone | 6 | 19.90 | 16.92 | 19.02 | 19.50 |
| After block 6 alone | 12 | 25.38 | 23.53 | 24.33 | 24.89 |
| After block 6 alone | 18 | 27.47 | 25.82 | 26.34 | 26.69 |
| After block 6 alone | 24 | 29.02 | 28.25 | 28.02 | 28.59 |
| After block 6 alone | 36 | 25.99 | 27.02 | 27.20 | 26.07 |
| Plain on all four nights (H-47) | 6 | 19.90 | 16.92 | 19.02 | 19.50 |
| Plain on all four nights (H-47) | 12 | 18.91 | 17.52 | 19.47 | 19.79 |
| Plain on all four nights (H-47) | 18 | 17.87 | 17.90 | 20.02 | 19.93 |
| Plain on all four nights (H-47) | 24 | 18.21 | 17.99 | 20.44 | 20.44 |
| Plain on all four nights (H-47) | 36 | 19.17 | 19.62 | 21.53 | 21.43 |
| After block 12 alone | 6 | 27.34 | 24.84 | 24.49 | 25.77 |
| After block 12 alone | 12 | 19.64 | 19.93 | 21.27 | 21.21 |
| After block 12 alone | 18 | 26.29 | 26.17 | 27.10 | 27.06 |
| After block 12 alone | 24 | 28.96 | 28.64 | 29.21 | 28.60 |
| After block 12 alone | 36 | 26.09 | 27.24 | 28.13 | 26.84 |
| After block 18 alone | 6 | 27.34 | 24.84 | 24.49 | 25.77 |
| After block 18 alone | 12 | 28.83 | 27.66 | 26.86 | 27.70 |
| After block 18 alone | 18 | 20.79 | 22.61 | 23.42 | 22.73 |
| After block 18 alone | 24 | 27.75 | 28.53 | 28.84 | 28.33 |
| After block 18 alone | 36 | 24.95 | 27.26 | 27.77 | 26.46 |
| After block 24 alone | 6 | 27.34 | 24.84 | 24.49 | 25.77 |
| After block 24 alone | 12 | 28.83 | 27.66 | 26.86 | 27.70 |
| After block 24 alone | 18 | 30.98 | 29.84 | 28.73 | 29.47 |
| After block 24 alone | 24 | 23.21 | 25.19 | 24.42 | 24.30 |
| After block 24 alone | 36 | 24.94 | 27.78 | 26.30 | 25.86 |

Each form less the night without replay, with H-47's paired SE; "= without replay" where the form's nights so far are the night without
replay's:

| Form | Block | A's trials: readout 1, less without replay | readout 0 | B's trials: readout 1 | readout 0 |
| :--- | ---: | :--- | :--- | :--- | :--- |
| After block 6 alone | 6 | −7.45 ± 0.20 | −7.92 ± 0.21 | −5.47 ± 0.19 | −6.27 ± 0.16 |
| After block 6 alone | 12 | −3.45 ± 0.37 | −4.13 ± 0.14 | −2.53 ± 0.19 | −2.81 ± 0.29 |
| After block 6 alone | 18 | −3.51 ± 0.26 | −4.02 ± 0.10 | −2.39 ± 0.15 | −2.78 ± 0.68 |
| After block 6 alone | 24 | −3.94 ± 0.17 | −2.97 ± 0.08 | −2.62 ± 0.09 | −2.49 ± 0.51 |
| After block 6 alone | 36 | −3.45 ± 0.07 | −3.04 ± 0.17 | −2.21 ± 0.08 | −2.58 ± 0.30 |
| Plain on all four nights (H-47) | 6 | −7.45 ± 0.20 | −7.92 ± 0.21 | −5.47 ± 0.19 | −6.27 ± 0.16 |
| Plain on all four nights (H-47) | 12 | −9.92 ± 0.31 | −10.14 ± 0.22 | −7.39 ± 0.23 | −7.91 ± 0.33 |
| Plain on all four nights (H-47) | 18 | −13.12 ± 0.15 | −11.94 ± 0.26 | −8.71 ± 0.21 | −9.54 ± 0.58 |
| Plain on all four nights (H-47) | 24 | −14.75 ± 0.25 | −13.23 ± 0.16 | −10.20 ± 0.17 | −10.63 ± 0.58 |
| Plain on all four nights (H-47) | 36 | −10.26 ± 0.48 | −10.43 ± 0.36 | −7.88 ± 0.33 | −7.23 ± 0.26 |
| After block 12 alone | 6 | = without replay | = without replay | = without replay | = without replay |
| After block 12 alone | 12 | −9.19 ± 0.20 | −7.73 ± 0.22 | −5.59 ± 0.14 | −6.49 ± 0.24 |
| After block 12 alone | 18 | −4.69 ± 0.16 | −3.67 ± 0.08 | −1.62 ± 0.17 | −2.41 ± 0.56 |
| After block 12 alone | 24 | −4.01 ± 0.24 | −2.58 ± 0.11 | −1.42 ± 0.13 | −2.47 ± 0.38 |
| After block 12 alone | 36 | −3.35 ± 0.47 | −2.82 ± 0.24 | −1.28 ± 0.20 | −1.82 ± 0.29 |
| After block 18 alone | 6 | = without replay | = without replay | = without replay | = without replay |
| After block 18 alone | 12 | = without replay | = without replay | = without replay | = without replay |
| After block 18 alone | 18 | −10.19 ± 0.24 | −7.23 ± 0.14 | −5.31 ± 0.21 | −6.74 ± 0.20 |
| After block 18 alone | 24 | −5.21 ± 0.13 | −2.69 ± 0.13 | −1.79 ± 0.10 | −2.74 ± 0.13 |
| After block 18 alone | 36 | −4.49 ± 0.22 | −2.79 ± 0.20 | −1.64 ± 0.09 | −2.20 ± 0.40 |
| After block 24 alone | 6 | = without replay | = without replay | = without replay | = without replay |
| After block 24 alone | 12 | = without replay | = without replay | = without replay | = without replay |
| After block 24 alone | 18 | = without replay | = without replay | = without replay | = without replay |
| After block 24 alone | 24 | −9.75 ± 0.22 | −6.03 ± 0.18 | −6.21 ± 0.15 | −6.77 ± 0.17 |
| After block 24 alone | 36 | −4.49 ± 0.37 | −2.28 ± 0.15 | −3.12 ± 0.15 | −2.80 ± 0.19 |

Each replayed night lowered both readouts on both stimuli's trials at the checkpoint after it, and every later checkpoint of every form lies
below the night without replay too, each of the 76 differences from a form's replayed night on beyond 2 SE and none above.

**How the night without replay's spikes lie over each readout's units**: how many of its units fire at all over the 48 reads, and the
fewest of its units, the most spikes first, that carry half its spikes:

| Block | Trials | Spikes a trial, readout 1 | of its units, any spike | fewest of its units carrying half | Spikes a trial, readout 0 | of its units, any spike | fewest carrying half |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | A's trials | 27.34 | 457 | 17 | 24.84 | 453 | 16 |
| 6 | B's trials | 24.49 | 444 | 14 | 25.77 | 445 | 18 |
| 12 | A's trials | 28.83 | 455 | 18 | 27.66 | 455 | 17 |
| 12 | B's trials | 26.86 | 443 | 15 | 27.70 | 451 | 18 |
| 18 | A's trials | 30.98 | 449 | 19 | 29.84 | 450 | 17 |
| 18 | B's trials | 28.73 | 449 | 16 | 29.47 | 448 | 19 |
| 24 | A's trials | 32.96 | 449 | 19 | 31.22 | 446 | 17 |
| 24 | B's trials | 30.64 | 443 | 17 | 31.07 | 448 | 19 |
| 36 | A's trials | 29.44 | 447 | 17 | 30.05 | 451 | 16 |
| 36 | B's trials | 29.41 | 447 | 16 | 28.66 | 449 | 18 |

Each readout fires on almost all of its units, 443 to 457 of its 459 over the 48 reads, yet 14 to 19 of them carry half of its spikes, under
the night without replay at every checkpoint.

## 3. Right after each replayed night

Each single night's form less the night without replay at its replayed night's block, and H-47's plain night less the night without
replay at blocks 6, 12, 18 and 24, its nights so far together. At block 6 H-47's plain night and after block 6 alone ran the same night on
the same image, so their rows here are the same. **Each readout's difference**, the units of the readout moved up and down beyond twice
their own SE, the fewest units carrying half the difference, the gross change, and the units moved against it:

| Comparison | Trials | Readout | The first form's spikes a trial | The second's | Difference | Moved up, beyond 2 SE | down | Fewest units to half the difference | Gross change | Moved against it, their sum |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | 19.90 | 27.34 | −7.45 ± 0.20 | 4 | 130 | 16 | 7.89 | 4, +0.14 |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | 16.92 | 24.84 | −7.92 ± 0.21 | 0 | 126 | 15 | 8.09 | 0, 0.00 |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | 19.02 | 24.49 | −5.47 ± 0.19 | 4 | 99 | 15 | 6.00 | 4, +0.15 |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | 19.50 | 25.77 | −6.27 ± 0.16 | 3 | 109 | 18 | 6.74 | 3, +0.13 |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | 19.64 | 28.83 | −9.19 ± 0.20 | 2 | 117 | 18 | 9.55 | 2, +0.01 |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | 19.93 | 27.66 | −7.73 ± 0.22 | 0 | 125 | 15 | 7.95 | 0, 0.00 |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | 21.27 | 26.86 | −5.59 ± 0.14 | 1 | 108 | 19 | 6.15 | 1, +0.10 |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | 21.21 | 27.70 | −6.49 ± 0.24 | 3 | 102 | 17 | 7.29 | 3, +0.20 |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | 20.79 | 30.98 | −10.19 ± 0.24 | 2 | 136 | 19 | 10.41 | 2, +0.05 |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | 22.61 | 29.84 | −7.23 ± 0.14 | 0 | 134 | 17 | 7.29 | 0, 0.00 |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | 23.42 | 28.73 | −5.31 ± 0.21 | 3 | 107 | 15 | 5.84 | 3, +0.23 |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | 22.73 | 29.47 | −6.74 ± 0.20 | 2 | 116 | 18 | 7.28 | 2, +0.14 |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | 23.21 | 32.96 | −9.75 ± 0.22 | 1 | 133 | 19 | 9.93 | 1, +0.01 |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | 25.19 | 31.22 | −6.03 ± 0.18 | 1 | 124 | 17 | 6.23 | 1, +0.02 |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | 24.42 | 30.64 | −6.21 ± 0.15 | 1 | 109 | 13 | 6.51 | 1, +0.10 |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | 24.30 | 31.07 | −6.77 ± 0.17 | 5 | 95 | 14 | 7.35 | 5, +0.23 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | 19.90 | 27.34 | −7.45 ± 0.20 | 4 | 130 | 16 | 7.89 | 4, +0.14 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | 16.92 | 24.84 | −7.92 ± 0.21 | 0 | 126 | 15 | 8.09 | 0, 0.00 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | 19.02 | 24.49 | −5.47 ± 0.19 | 4 | 99 | 15 | 6.00 | 4, +0.15 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | 19.50 | 25.77 | −6.27 ± 0.16 | 3 | 109 | 18 | 6.74 | 3, +0.13 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | 18.91 | 28.83 | −9.92 ± 0.31 | 8 | 109 | 14 | 10.57 | 8, +0.11 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | 17.52 | 27.66 | −10.14 ± 0.22 | 3 | 113 | 13 | 10.55 | 3, +0.14 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | 19.47 | 26.86 | −7.39 ± 0.23 | 5 | 101 | 13 | 8.01 | 5, +0.21 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | 19.79 | 27.70 | −7.91 ± 0.33 | 5 | 85 | 13 | 8.61 | 5, +0.11 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | 17.87 | 30.98 | −13.12 ± 0.15 | 13 | 115 | 15 | 14.22 | 13, +0.26 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | 17.90 | 29.84 | −11.94 ± 0.26 | 5 | 114 | 12 | 12.91 | 5, +0.27 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | 20.02 | 28.73 | −8.71 ± 0.21 | 6 | 95 | 11 | 9.73 | 6, +0.29 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | 19.93 | 29.47 | −9.54 ± 0.58 | 18 | 89 | 12 | 10.96 | 18, +0.30 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | 18.21 | 32.96 | −14.75 ± 0.25 | 37 | 105 | 14 | 16.61 | 37, +0.63 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | 17.99 | 31.22 | −13.23 ± 0.16 | 21 | 121 | 11 | 15.01 | 21, +0.69 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | 20.44 | 30.64 | −10.20 ± 0.17 | 23 | 94 | 10 | 11.57 | 23, +0.53 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | 20.44 | 31.07 | −10.63 ± 0.58 | 47 | 75 | 11 | 13.07 | 47, +0.85 |

Right after each single night's replayed night both readouts lay below the night without replay on both stimuli's trials, each beyond 2 SE:
95 to 136 of a readout's 459 units moved down beyond twice their own SE and 0 to 5 up, against about 23 by chance in the two directions
together. The gross change exceeds each readout's drop by 0.06 to 0.81, and the 0 to 5 units moved against it sum to 0.23 or less. Under
H-47's plain night the drop grew with each night, to 10.20 to 14.75 at block 24, with 75 to 121 units moved down and 3 to 47 up at blocks 12
to 24.

**The answer's readout less the other**: on A's trials readout 1 less readout 0, on B's trials readout 0 less readout 1, under each form,
and the difference with its SE:

| Comparison | A's trials: readout 1 less readout 0, the first form | the second | Difference | B's trials: readout 0 less readout 1, the first form | the second | Difference |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | 2.98 | 2.50 | +0.48 ± 0.29 | 0.48 | 1.28 | −0.80 ± 0.22 |
| After block 12 alone less without replay, block 12 | −0.29 | 1.17 | −1.46 ± 0.22 | −0.06 | 0.84 | −0.90 ± 0.21 |
| After block 18 alone less without replay, block 18 | −1.82 | 1.14 | −2.96 ± 0.19 | −0.69 | 0.74 | −1.43 ± 0.37 |
| After block 24 alone less without replay, block 24 | −1.99 | 1.74 | −3.73 ± 0.16 | −0.13 | 0.44 | −0.56 ± 0.17 |
| Plain on all four nights less without replay, block 6 | 2.98 | 2.50 | +0.48 ± 0.29 | 0.48 | 1.28 | −0.80 ± 0.22 |
| Plain on all four nights less without replay, block 12 | 1.39 | 1.17 | +0.22 ± 0.43 | 0.33 | 0.84 | −0.51 ± 0.52 |
| Plain on all four nights less without replay, block 18 | −0.03 | 1.14 | −1.18 ± 0.30 | −0.10 | 0.74 | −0.83 ± 0.65 |
| Plain on all four nights less without replay, block 24 | 0.22 | 1.74 | −1.52 ± 0.27 | 0.00 | 0.44 | −0.43 ± 0.59 |

On A's trials readout 1 fell less than readout 0 after block 6 alone, within 2 SE, and more after each later single night, by more the later
the block; on B's trials readout 0, B's answer, fell more than readout 1 under all four, beyond 2 SE. Under H-47's plain night readout 1
less readout 0 on A's trials lay within 2 SE at blocks 6 and 12 and below beyond 2 SE at 18 and 24; on B's trials readout 0 less readout 1
lay below beyond 2 SE at block 6 and within 2 SE at 12, 18 and 24.

**By class**, each class's spikes a trial summed over its units, its difference with the SE of that sum, and its units moved:

| Comparison | Trials | Readout | Class | How many units | The first form's spikes a trial | The second's | Difference | Moved up | Moved down |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | D's own | 25 | 0.65 | 1.00 | −0.35 ± 0.03 | 0 | 7 |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 11.75 | 15.45 | −3.70 ± 0.12 | 3 | 70 |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 0.51 | 1.06 | −0.54 ± 0.04 | 0 | 7 |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 6.98 | 9.84 | −2.86 ± 0.10 | 1 | 46 |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | C's own | 25 | 0.19 | 0.36 | −0.17 ± 0.02 | 0 | 7 |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 11.16 | 16.64 | −5.47 ± 0.18 | 0 | 71 |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 2.49 | 3.40 | −0.91 ± 0.05 | 0 | 13 |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 3.08 | 4.44 | −1.36 ± 0.06 | 0 | 35 |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | D's own | 25 | 0.70 | 1.23 | −0.53 ± 0.03 | 0 | 7 |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 9.08 | 11.83 | −2.75 ± 0.10 | 1 | 48 |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 2.74 | 3.66 | −0.91 ± 0.06 | 0 | 11 |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 6.50 | 7.77 | −1.28 ± 0.10 | 3 | 33 |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | C's own | 25 | 1.36 | 1.55 | −0.19 ± 0.03 | 1 | 6 |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 9.41 | 12.15 | −2.73 ± 0.09 | 1 | 53 |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 0.63 | 1.36 | −0.73 ± 0.05 | 0 | 7 |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 8.10 | 10.71 | −2.61 ± 0.08 | 1 | 43 |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | D's own | 25 | 0.81 | 0.97 | −0.16 ± 0.03 | 0 | 5 |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 11.13 | 15.76 | −4.63 ± 0.13 | 2 | 56 |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 0.67 | 1.31 | −0.65 ± 0.05 | 0 | 13 |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.04 | 10.79 | −3.75 ± 0.09 | 0 | 43 |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | C's own | 25 | 0.21 | 0.35 | −0.14 ± 0.03 | 0 | 3 |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 13.35 | 18.15 | −4.80 ± 0.18 | 0 | 61 |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 2.85 | 3.85 | −1.00 ± 0.08 | 0 | 18 |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 3.52 | 5.30 | −1.79 ± 0.09 | 0 | 43 |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | D's own | 25 | 1.05 | 1.43 | −0.38 ± 0.03 | 0 | 4 |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 9.97 | 12.84 | −2.87 ± 0.12 | 0 | 51 |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 3.18 | 3.99 | −0.81 ± 0.06 | 0 | 17 |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.08 | 8.60 | −1.52 ± 0.12 | 1 | 36 |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | C's own | 25 | 1.36 | 1.49 | −0.12 ± 0.03 | 2 | 5 |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 10.14 | 12.88 | −2.74 ± 0.15 | 0 | 49 |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 0.90 | 1.80 | −0.91 ± 0.05 | 0 | 11 |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 8.82 | 11.53 | −2.71 ± 0.14 | 1 | 37 |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | D's own | 25 | 0.69 | 0.96 | −0.28 ± 0.03 | 0 | 7 |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 11.66 | 16.79 | −5.14 ± 0.11 | 2 | 70 |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 0.93 | 1.59 | −0.65 ± 0.04 | 0 | 12 |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.52 | 11.64 | −4.12 ± 0.11 | 0 | 47 |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | C's own | 25 | 0.21 | 0.38 | −0.16 ± 0.02 | 0 | 6 |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 15.16 | 19.43 | −4.27 ± 0.13 | 0 | 70 |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 3.21 | 4.04 | −0.83 ± 0.07 | 0 | 17 |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 4.03 | 5.99 | −1.97 ± 0.06 | 0 | 41 |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | D's own | 25 | 1.21 | 1.54 | −0.32 ± 0.02 | 0 | 4 |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 11.16 | 13.89 | −2.73 ± 0.13 | 0 | 54 |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 3.61 | 4.26 | −0.64 ± 0.04 | 1 | 14 |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.43 | 9.04 | −1.62 ± 0.10 | 2 | 35 |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | C's own | 25 | 1.26 | 1.49 | −0.22 ± 0.03 | 0 | 9 |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 10.45 | 13.64 | −3.19 ± 0.09 | 2 | 52 |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 1.51 | 2.35 | −0.84 ± 0.07 | 0 | 14 |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 9.51 | 11.98 | −2.48 ± 0.09 | 0 | 41 |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | D's own | 25 | 0.56 | 0.82 | −0.26 ± 0.02 | 1 | 4 |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 12.61 | 17.51 | −4.90 ± 0.12 | 0 | 67 |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 1.12 | 1.87 | −0.74 ± 0.06 | 0 | 14 |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.92 | 12.77 | −3.85 ± 0.09 | 0 | 48 |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | C's own | 25 | 0.19 | 0.33 | −0.14 ± 0.02 | 0 | 6 |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 16.51 | 19.88 | −3.37 ± 0.13 | 1 | 63 |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 3.60 | 4.12 | −0.52 ± 0.03 | 0 | 14 |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 4.88 | 6.89 | −2.00 ± 0.09 | 0 | 41 |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | D's own | 25 | 1.35 | 1.85 | −0.49 ± 0.03 | 0 | 5 |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 11.69 | 14.66 | −2.97 ± 0.06 | 0 | 48 |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 3.53 | 4.62 | −1.09 ± 0.06 | 0 | 12 |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 7.85 | 9.51 | −1.66 ± 0.09 | 1 | 44 |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | C's own | 25 | 1.26 | 1.42 | −0.16 ± 0.02 | 1 | 3 |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 10.81 | 14.26 | −3.46 ± 0.09 | 2 | 46 |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 1.96 | 2.86 | −0.90 ± 0.06 | 0 | 7 |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 10.27 | 12.53 | −2.25 ± 0.09 | 2 | 39 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | D's own | 25 | 0.65 | 1.00 | −0.35 ± 0.03 | 0 | 7 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 11.75 | 15.45 | −3.70 ± 0.12 | 3 | 70 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 0.51 | 1.06 | −0.54 ± 0.04 | 0 | 7 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 6.98 | 9.84 | −2.86 ± 0.10 | 1 | 46 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | C's own | 25 | 0.19 | 0.36 | −0.17 ± 0.02 | 0 | 7 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 11.16 | 16.64 | −5.47 ± 0.18 | 0 | 71 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 2.49 | 3.40 | −0.91 ± 0.05 | 0 | 13 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 3.08 | 4.44 | −1.36 ± 0.06 | 0 | 35 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | D's own | 25 | 0.70 | 1.23 | −0.53 ± 0.03 | 0 | 7 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 9.08 | 11.83 | −2.75 ± 0.10 | 1 | 48 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 2.74 | 3.66 | −0.91 ± 0.06 | 0 | 11 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 6.50 | 7.77 | −1.28 ± 0.10 | 3 | 33 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | C's own | 25 | 1.36 | 1.55 | −0.19 ± 0.03 | 1 | 6 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 9.41 | 12.15 | −2.73 ± 0.09 | 1 | 53 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 0.63 | 1.36 | −0.73 ± 0.05 | 0 | 7 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 8.10 | 10.71 | −2.61 ± 0.08 | 1 | 43 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | D's own | 25 | 0.64 | 0.97 | −0.33 ± 0.02 | 0 | 6 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 11.24 | 15.76 | −4.52 ± 0.25 | 7 | 53 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 0.52 | 1.31 | −0.79 ± 0.03 | 0 | 10 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 6.51 | 10.79 | −4.28 ± 0.10 | 1 | 40 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | C's own | 25 | 0.18 | 0.35 | −0.17 ± 0.02 | 0 | 6 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 11.14 | 18.15 | −7.02 ± 0.18 | 1 | 67 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 2.67 | 3.85 | −1.18 ± 0.07 | 1 | 11 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 3.53 | 5.30 | −1.77 ± 0.06 | 1 | 29 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | D's own | 25 | 0.93 | 1.43 | −0.49 ± 0.04 | 0 | 5 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 9.01 | 12.84 | −3.83 ± 0.14 | 2 | 48 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 2.93 | 3.99 | −1.06 ± 0.07 | 0 | 11 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 6.59 | 8.60 | −2.01 ± 0.09 | 3 | 37 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | C's own | 25 | 1.30 | 1.49 | −0.19 ± 0.03 | 1 | 5 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 9.73 | 12.88 | −3.14 ± 0.12 | 4 | 35 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 0.64 | 1.80 | −1.17 ± 0.05 | 0 | 7 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 8.12 | 11.53 | −3.41 ± 0.24 | 0 | 38 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | D's own | 25 | 0.57 | 0.96 | −0.40 ± 0.03 | 0 | 5 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 10.85 | 16.79 | −5.94 ± 0.14 | 11 | 54 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 0.61 | 1.59 | −0.97 ± 0.05 | 0 | 11 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 5.84 | 11.64 | −5.80 ± 0.13 | 2 | 45 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | C's own | 25 | 0.23 | 0.38 | −0.15 ± 0.02 | 0 | 6 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 11.21 | 19.43 | −8.22 ± 0.25 | 1 | 68 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 2.80 | 4.04 | −1.24 ± 0.05 | 4 | 8 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 3.66 | 5.99 | −2.33 ± 0.08 | 0 | 32 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | D's own | 25 | 0.85 | 1.54 | −0.69 ± 0.03 | 0 | 4 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 9.39 | 13.89 | −4.50 ± 0.13 | 3 | 44 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 3.16 | 4.26 | −1.09 ± 0.05 | 1 | 13 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 6.62 | 9.04 | −2.43 ± 0.08 | 2 | 34 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | C's own | 25 | 1.29 | 1.49 | −0.20 ± 0.03 | 2 | 4 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 9.69 | 13.64 | −3.94 ± 0.26 | 16 | 37 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 0.76 | 2.35 | −1.59 ± 0.08 | 0 | 11 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 8.18 | 11.98 | −3.81 ± 0.36 | 0 | 37 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | D's own | 25 | 0.56 | 0.82 | −0.26 ± 0.03 | 4 | 4 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 11.00 | 17.51 | −6.51 ± 0.20 | 32 | 48 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 0.69 | 1.87 | −1.18 ± 0.08 | 0 | 10 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 5.96 | 12.77 | −6.80 ± 0.10 | 1 | 43 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | C's own | 25 | 0.22 | 0.33 | −0.12 ± 0.02 | 0 | 5 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 11.03 | 19.88 | −8.85 ± 0.12 | 7 | 68 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 3.08 | 4.12 | −1.05 ± 0.04 | 12 | 13 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 3.67 | 6.89 | −3.22 ± 0.09 | 2 | 35 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | D's own | 25 | 1.09 | 1.85 | −0.75 ± 0.04 | 1 | 6 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 9.45 | 14.66 | −5.22 ± 0.08 | 10 | 46 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 3.28 | 4.62 | −1.34 ± 0.06 | 7 | 11 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 6.62 | 9.51 | −2.89 ± 0.08 | 5 | 31 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | C's own | 25 | 1.22 | 1.42 | −0.21 ± 0.03 | 5 | 4 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 9.82 | 14.26 | −4.44 ± 0.25 | 29 | 32 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 0.99 | 2.86 | −1.87 ± 0.15 | 4 | 12 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 8.42 | 12.53 | −4.11 ± 0.25 | 9 | 27 |

Every class of both readouts fired less on both stimuli's trials right after each replayed night, each of the 64 class sums of the four
single nights and the 64 of H-47's plain night at blocks 6 to 24 beyond 2 SE. On A's trials readout 1's pattern targets and inhibitory units
lost more after each later single night than after block 6 alone, 4.63 to 5.14 and 3.75 to 4.12 a trial against 3.70 and 2.86, and readout
0's pattern targets less, 3.37 to 4.80 against 5.47. On B's trials each class lost about as much under all four: readout 0's pattern targets
2.73 to 3.46 and its inhibitory units 2.25 to 2.71, readout 1's pattern targets 2.73 to 2.97 and its inhibitory units 1.28 to 1.66. C's own
and D's own lost 0.12 to 0.53.

**The fewest units carrying half of each readout's difference**, all of them beyond 2 SE here, named by their id and class: T a pattern
target, E another excitatory unit, I an inhibitory unit, C C's own, D D's own. A unit's place is its id's remainder by 20 on this cell,
whose rotation is 0. Beside each, how many of them are among the night without replay's units carrying half its spikes at the same
checkpoint:

| Comparison | Trials | Readout | Fewest units | Their differences' sum | Their classes | Among the night without replay's units carrying half its spikes | The units, by id and class, the largest difference first |
| :--- | :--- | :--- | ---: | ---: | :--- | ---: | :--- |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | 16 | −3.73 | T 7, E 1, I 7, D 1 | 7 | 338 T, 774 I, 144 I, 832 T, 830 E, 784 I, 974 I, 756 D, 322 T, 438 T, 642 T, 604 I, 4 I, 804 I, 648 T, 6 T |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | 15 | −4.11 | T 12, E 2, I 1 | 7 | 7 T, 461 T, 5 T, 801 E, 143 T, 685 T, 1005 T, 655 T, 507 T, 495 T, 861 E, 337 T, 339 I, 3 T, 343 T |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | 15 | −2.78 | T 6, E 4, I 3, D 2 | 4 | 230 T, 164 I, 376 T, 276 D, 156 D, 346 T, 56 E, 270 E, 304 I, 326 T, 918 T, 16 E, 68 E, 314 I, 108 T |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | 18 | −3.17 | T 8, E 1, I 9 | 9 | 801 E, 179 I, 849 I, 619 I, 325 T, 779 I, 935 T, 913 T, 937 T, 115 T, 853 T, 1009 I, 939 I, 1017 T, 229 I, 689 I, 29 I, 43 T |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | 18 | −4.73 | T 8, E 1, I 9 | 9 | 774 I, 974 I, 678 T, 830 E, 832 T, 6 T, 144 I, 826 T, 794 I, 448 T, 966 T, 784 I, 864 I, 804 I, 834 I, 4 I, 642 T, 338 T |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | 15 | −3.87 | T 10, E 2, I 3 | 6 | 685 T, 833 T, 819 I, 861 E, 461 T, 801 E, 143 T, 5 T, 7 T, 655 T, 3 T, 285 T, 1005 T, 829 I, 649 I |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | 19 | −2.83 | T 9, E 3, I 5, D 2 | 5 | 230 T, 6 T, 56 E, 276 D, 164 I, 314 I, 376 T, 326 T, 324 I, 346 T, 22 T, 304 I, 16 E, 686 E, 156 D, 246 T, 108 T, 334 I, 658 T |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | 17 | −3.27 | T 7, E 2, I 8 | 7 | 801 E, 453 T, 179 I, 329 I, 849 I, 779 I, 243 T, 619 I, 317 E, 325 T, 29 I, 853 T, 809 I, 99 I, 935 T, 5 T, 75 T |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | 19 | −5.14 | T 9, E 1, I 9 | 8 | 774 I, 974 I, 278 T, 648 T, 292 T, 384 I, 864 I, 642 T, 286 T, 830 E, 832 T, 144 I, 338 T, 604 I, 784 I, 834 I, 804 I, 862 T, 438 T |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | 17 | −3.73 | T 11, E 2, I 4 | 6 | 819 I, 285 T, 829 I, 801 E, 387 T, 649 I, 143 T, 461 T, 833 T, 647 T, 861 E, 655 T, 5 T, 337 T, 495 T, 7 T, 339 I |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | 15 | −2.74 | T 8, E 2, I 4, D 1 | 5 | 346 T, 164 I, 314 I, 658 T, 324 I, 108 T, 16 E, 6 T, 22 T, 276 D, 918 T, 56 E, 376 T, 804 I, 52 T |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | 18 | −3.38 | T 9, E 3, I 6 | 6 | 5 T, 779 I, 7 T, 75 T, 849 I, 453 T, 179 I, 1017 T, 937 T, 915 E, 329 I, 659 I, 15 T, 913 T, 853 T, 49 I, 801 E, 101 E |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | 19 | −4.94 | T 10, I 8, D 1 | 9 | 774 I, 974 I, 338 T, 286 T, 966 T, 784 I, 226 T, 834 I, 604 I, 448 T, 438 T, 826 T, 648 T, 678 T, 804 I, 342 T, 144 I, 756 D, 794 I |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | 17 | −3.09 | T 11, E 1, I 5 | 7 | 819 I, 343 T, 801 E, 833 T, 559 I, 565 T, 143 T, 655 T, 337 T, 461 T, 339 I, 649 I, 227 T, 685 T, 829 I, 285 T, 835 T |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | 13 | −3.12 | T 6, E 3, I 3, D 1 | 3 | 346 T, 108 T, 22 T, 276 D, 314 I, 16 E, 658 T, 270 E, 52 T, 164 I, 68 E, 326 T, 324 I |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | 14 | −3.49 | T 9, E 2, I 3 | 7 | 5 T, 915 E, 329 I, 15 T, 7 T, 101 E, 453 T, 653 T, 179 I, 937 T, 659 I, 75 T, 1017 T, 325 T |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | 16 | −3.73 | T 7, E 1, I 7, D 1 | 7 | 338 T, 774 I, 144 I, 832 T, 830 E, 784 I, 974 I, 756 D, 322 T, 438 T, 642 T, 604 I, 4 I, 804 I, 648 T, 6 T |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | 15 | −4.11 | T 12, E 2, I 1 | 7 | 7 T, 461 T, 5 T, 801 E, 143 T, 685 T, 1005 T, 655 T, 507 T, 495 T, 861 E, 337 T, 339 I, 3 T, 343 T |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | 15 | −2.78 | T 6, E 4, I 3, D 2 | 4 | 230 T, 164 I, 376 T, 276 D, 156 D, 346 T, 56 E, 270 E, 304 I, 326 T, 918 T, 16 E, 68 E, 314 I, 108 T |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | 18 | −3.17 | T 8, E 1, I 9 | 9 | 801 E, 179 I, 849 I, 619 I, 325 T, 779 I, 935 T, 913 T, 937 T, 115 T, 853 T, 1009 I, 939 I, 1017 T, 229 I, 689 I, 29 I, 43 T |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | 14 | −5.13 | T 5, E 1, I 7, D 1 | 7 | 774 I, 338 T, 830 E, 144 I, 974 I, 784 I, 832 T, 804 I, 826 T, 864 I, 642 T, 604 I, 756 D, 278 T |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | 13 | −5.38 | T 11, E 2 | 8 | 861 E, 495 T, 461 T, 1005 T, 801 E, 833 T, 143 T, 7 T, 655 T, 5 T, 685 T, 343 T, 507 T |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | 13 | −3.80 | T 7, E 2, I 3, D 1 | 5 | 230 T, 346 T, 164 I, 16 E, 376 T, 304 I, 276 D, 324 I, 6 T, 56 E, 326 T, 918 T, 22 T |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | 13 | −4.15 | T 6, E 2, I 5 | 7 | 801 E, 325 T, 779 I, 179 I, 619 I, 913 T, 849 I, 329 I, 115 T, 317 E, 5 T, 15 T, 853 T |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | 15 | −6.71 | T 4, E 1, I 9, D 1 | 8 | 774 I, 974 I, 338 T, 144 I, 864 I, 830 E, 784 I, 438 T, 834 I, 278 T, 604 I, 648 T, 804 I, 384 I, 756 D |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | 12 | −6.31 | T 9, E 2, I 1 | 7 | 861 E, 495 T, 655 T, 1005 T, 461 T, 801 E, 285 T, 143 T, 829 I, 565 T, 833 T, 507 T |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | 11 | −4.55 | T 5, E 1, I 4, D 1 | 4 | 230 T, 346 T, 16 E, 276 D, 164 I, 324 I, 376 T, 658 T, 314 I, 304 I, 22 T |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | 12 | −4.87 | T 6, E 1, I 5 | 8 | 801 E, 325 T, 779 I, 5 T, 179 I, 849 I, 115 T, 619 I, 7 T, 329 I, 913 T, 15 T |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | 14 | −7.65 | T 5, E 1, I 8 | 6 | 774 I, 338 T, 974 I, 144 I, 834 I, 830 E, 384 I, 784 I, 278 T, 604 I, 864 I, 438 T, 832 T, 862 T |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | 11 | −7.08 | T 7, E 2, I 2 | 8 | 655 T, 285 T, 495 T, 461 T, 1005 T, 861 E, 801 E, 143 T, 819 I, 829 I, 565 T |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | 10 | −5.30 | T 5, E 1, I 3, D 1 | 5 | 230 T, 346 T, 108 T, 22 T, 314 I, 376 T, 276 D, 16 E, 524 I, 164 I |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | 11 | −5.62 | T 4, E 2, I 5 | 7 | 5 T, 325 T, 915 E, 801 E, 179 I, 619 I, 329 I, 49 I, 779 I, 7 T, 1017 T |

**The four single nights' fewest units, set side by side**:

| Trials | Readout | After block 6 alone, at 6 | after 12 alone, at 12 | after 18 alone, at 18 | after 24 alone, at 24 | In all four | In any | The units in all four |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
| A's trials | readout 1, the even places | 16 | 18 | 19 | 19 | 6 | 30 | 338 T, 774 I, 144 I, 784 I, 974 I, 804 I |
| A's trials | readout 0, the odd places | 15 | 15 | 17 | 17 | 4 | 26 | 461 T, 801 E, 143 T, 655 T |
| B's trials | readout 1, the even places | 15 | 19 | 15 | 13 | 6 | 24 | 164 I, 276 D, 346 T, 16 E, 314 I, 108 T |
| B's trials | readout 0, the odd places | 18 | 17 | 18 | 14 | 1 | 33 | 179 I |

The fewest units carrying half of each single night's readout drop are 13 to 19, most of them pattern targets and inhibitory units: on A's
trials readout 1's are 7 to 10 pattern targets and 7 to 9 inhibitory units, readout 0's 10 to 12 pattern targets. They recur from night to
night: of the 24 to 33 units among any of the four single nights' fewest of a readout on a stimulus's trials, 1 to 6 are among all four's.
Under H-47's plain night the fewest are 10 to 18 at blocks 6 to 24, on A's trials readout 1's 7 to 9 of them inhibitory units.

## 4. At block 36

The same at block 36: each form less the night without replay, paired in its job; after block 6 alone less H-47's plain night, paired, both
in job one; after block 12, 18 or 24 alone less H-47's plain night, across the two jobs, with exploration 018's unpaired SE:

| Comparison | Trials | Readout | The first form's spikes a trial | The second's | Difference | Moved up, beyond 2 SE | down | Fewest units to half the difference | Gross change | Moved against it, their sum |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | 25.99 | 29.44 | −3.45 ± 0.07 | 42 | 48 | 7 | 5.38 | 42, +0.48 |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | 27.02 | 30.05 | −3.04 ± 0.17 | 32 | 45 | 6 | 4.56 | 32, +0.41 |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | 27.20 | 29.41 | −2.21 ± 0.08 | 26 | 36 | 4 | 4.04 | 26, +0.41 |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | 26.07 | 28.66 | −2.58 ± 0.30 | 42 | 36 | 6 | 4.67 | 42, +0.72 |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | 26.09 | 29.44 | −3.35 ± 0.47 | 39 | 42 | 7 | 5.31 | 39, +0.59 |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | 27.24 | 30.05 | −2.82 ± 0.24 | 20 | 46 | 5 | 4.33 | 20, +0.48 |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | 28.13 | 29.41 | −1.28 ± 0.20 | 23 | 34 | 3 | 3.06 | 23, +0.50 |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | 26.84 | 28.66 | −1.82 ± 0.29 | 20 | 35 | 3 | 3.89 | 20, +0.54 |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | 24.95 | 29.44 | −4.49 ± 0.22 | 35 | 55 | 8 | 6.00 | 35, +0.41 |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | 27.26 | 30.05 | −2.79 ± 0.20 | 18 | 46 | 6 | 4.05 | 18, +0.31 |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | 27.77 | 29.41 | −1.64 ± 0.09 | 24 | 32 | 4 | 3.09 | 24, +0.41 |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | 26.46 | 28.66 | −2.20 ± 0.40 | 30 | 43 | 6 | 4.22 | 30, +0.68 |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | 24.94 | 29.44 | −4.49 ± 0.37 | 44 | 55 | 10 | 5.81 | 44, +0.30 |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | 27.78 | 30.05 | −2.28 ± 0.15 | 33 | 39 | 6 | 3.69 | 33, +0.24 |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | 26.30 | 29.41 | −3.12 ± 0.15 | 20 | 38 | 6 | 4.22 | 20, +0.18 |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | 25.86 | 28.66 | −2.80 ± 0.19 | 39 | 45 | 5 | 5.20 | 39, +0.71 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | 19.17 | 29.44 | −10.26 ± 0.48 | 104 | 75 | 10 | 13.62 | 104, +1.14 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | 19.62 | 30.05 | −10.43 ± 0.36 | 67 | 63 | 8 | 13.71 | 67, +1.15 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | 21.53 | 29.41 | −7.88 ± 0.33 | 67 | 61 | 8 | 10.41 | 67, +0.86 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | 21.43 | 28.66 | −7.23 ± 0.26 | 104 | 55 | 7 | 11.49 | 104, +1.57 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 25.99 | 19.17 | +6.82 ± 0.50 | 58 | 77 | 8 | 9.28 | 77, −0.73 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 27.02 | 19.62 | +7.40 ± 0.30 | 60 | 52 | 8 | 9.84 | 52, −0.81 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 27.20 | 21.53 | +5.67 ± 0.33 | 60 | 52 | 6 | 7.78 | 52, −0.58 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 26.07 | 21.43 | +4.64 ± 0.05 | 53 | 83 | 5 | 8.37 | 83, −1.20 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 26.09 | 19.17 | +6.91 ± 0.28 | 44 | 33 | 9 | 9.67 | 33, −0.50 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 27.24 | 19.62 | +7.62 ± 0.39 | 34 | 19 | 8 | 10.26 | 19, −0.70 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 28.13 | 21.53 | +6.60 ± 0.36 | 39 | 20 | 7 | 8.92 | 20, −0.50 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 26.84 | 21.43 | +5.41 ± 0.37 | 39 | 41 | 6 | 9.63 | 41, −1.21 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 24.95 | 19.17 | +5.78 ± 0.37 | 45 | 37 | 8 | 9.06 | 37, −0.76 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 27.26 | 19.62 | +7.64 ± 0.32 | 38 | 20 | 7 | 10.50 | 20, −0.81 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 27.77 | 21.53 | +6.24 ± 0.45 | 41 | 20 | 8 | 8.41 | 20, −0.37 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 26.46 | 21.43 | +5.03 ± 0.31 | 33 | 36 | 6 | 8.81 | 36, −0.84 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 24.94 | 19.17 | +5.77 ± 0.33 | 51 | 42 | 7 | 8.78 | 42, −0.70 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 27.78 | 19.62 | +8.16 ± 0.36 | 40 | 22 | 7 | 10.73 | 22, −0.77 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 26.30 | 21.53 | +4.77 ± 0.49 | 43 | 21 | 6 | 7.05 | 21, −0.54 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 25.86 | 21.43 | +4.43 ± 0.30 | 42 | 41 | 6 | 8.27 | 41, −0.93 |

At block 36 each single night still lay below the night without replay on both readouts and both stimuli's trials, each beyond 2 SE: by 2.28
to 4.49 a trial on A's trials and 1.28 to 3.12 on B's. Fewer units moved down than right after, 32 to 55, and more up, 18 to 44; 3 to 10
units carry half of each readout's difference, and the gross change exceeds it by 1.11 to 2.40, more than right after in each. H-47's plain
night lay below by 7.23 to 10.43, with 55 to 75 units moved down and 67 to 104 up, and each single night lay above it by 4.43 to 8.16, each
beyond 2 SE.

| Comparison | A's trials: readout 1 less readout 0, the first form | the second | Difference | B's trials: readout 0 less readout 1, the first form | the second | Difference |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 36 | −1.03 | −0.62 | −0.41 ± 0.23 | −1.12 | −0.75 | −0.37 ± 0.30 |
| After block 12 alone less without replay, block 36 | −1.15 | −0.62 | −0.53 ± 0.27 | −1.29 | −0.75 | −0.54 ± 0.46 |
| After block 18 alone less without replay, block 36 | −2.31 | −0.62 | −1.69 ± 0.12 | −1.32 | −0.75 | −0.56 ± 0.35 |
| After block 24 alone less without replay, block 36 | −2.84 | −0.62 | −2.22 ± 0.31 | −0.44 | −0.75 | +0.32 ± 0.13 |
| Plain on all four nights less without replay, block 36 | −0.45 | −0.62 | +0.17 ± 0.31 | −0.10 | −0.75 | +0.65 ± 0.57 |
| After block 6 alone less plain on all four nights, block 36 | −1.03 | −0.45 | −0.58 ± 0.46 | −1.12 | −0.10 | −1.02 ± 0.30 |
| After block 12 alone less plain on all four nights, block 36 | −1.15 | −0.45 | −0.70 ± 0.37 | −1.29 | −0.10 | −1.20 ± 0.47 |
| After block 18 alone less plain on all four nights, block 36 | −2.31 | −0.45 | −1.87 ± 0.31 | −1.32 | −0.10 | −1.22 ± 0.26 |
| After block 24 alone less plain on all four nights, block 36 | −2.84 | −0.45 | −2.39 ± 0.18 | −0.44 | −0.10 | −0.34 ± 0.51 |

At block 36 readout 1 less readout 0 on A's trials lay below the night without replay's under each single night, beyond 2 SE after block 18
and 24 alone and within 2 SE after block 6 and 12 alone; readout 0 less readout 1 on B's trials lay within 2 SE after block 6, 12 and 18
alone and above it after block 24 alone, +0.32 ± 0.13, beyond 2 SE, where B's correct trials lay above too, +0.9 ± 0.13 (exploration 032).
Under H-47's plain night both lay within 2 SE. Less H-47's plain night, each single night lay below on A's trials, beyond 2 SE after block
18 and 24 alone, and below on B's trials, beyond 2 SE after block 6, 12 and 18 alone.

**By class at block 36**, beside each class's difference right after its replayed night (for H-47's plain night, at block 24):

| Comparison | Trials | Readout | Class | How many units | The first form's spikes a trial | The second's | Difference | Right after its replayed night | Moved up | Moved down |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.83 | 0.79 | +0.05 ± 0.02 | −0.35 ± 0.03 | 3 | 1 |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 13.55 | 15.38 | −1.83 ± 0.06 | −3.70 ± 0.12 | 26 | 25 |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 1.38 | 1.46 | −0.08 ± 0.06 | −0.54 ± 0.04 | 5 | 2 |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 10.23 | 11.81 | −1.58 ± 0.09 | −2.86 ± 0.10 | 8 | 20 |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.22 | 0.22 | −0.01 ± 0.01 | −0.17 ± 0.02 | 0 | 1 |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 17.15 | 19.35 | −2.20 ± 0.18 | −5.47 ± 0.18 | 16 | 30 |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 4.08 | 4.13 | −0.05 ± 0.14 | −0.91 ± 0.05 | 7 | 2 |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 5.58 | 6.36 | −0.78 ± 0.04 | −1.36 ± 0.06 | 9 | 12 |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.61 | 1.81 | −0.19 ± 0.01 | −0.53 ± 0.03 | 0 | 3 |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 12.56 | 14.10 | −1.53 ± 0.09 | −2.75 ± 0.10 | 14 | 17 |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 3.79 | 4.38 | −0.59 ± 0.07 | −0.91 ± 0.06 | 5 | 6 |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 9.23 | 9.13 | +0.10 ± 0.08 | −1.28 ± 0.10 | 7 | 10 |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.35 | 1.41 | −0.06 ± 0.01 | −0.19 ± 0.03 | 6 | 3 |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 12.35 | 13.15 | −0.80 ± 0.20 | −2.73 ± 0.09 | 25 | 14 |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 2.21 | 2.79 | −0.58 ± 0.05 | −0.73 ± 0.05 | 2 | 4 |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 10.16 | 11.31 | −1.15 ± 0.13 | −2.61 ± 0.08 | 9 | 15 |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.83 | 0.79 | +0.04 ± 0.01 | −0.16 ± 0.03 | 2 | 0 |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 13.91 | 15.38 | −1.47 ± 0.33 | −4.63 ± 0.13 | 27 | 20 |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 1.35 | 1.46 | −0.11 ± 0.08 | −0.65 ± 0.05 | 1 | 6 |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 10.00 | 11.81 | −1.81 ± 0.24 | −3.75 ± 0.09 | 9 | 16 |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.22 | 0.22 | −0.01 ± 0.01 | −0.14 ± 0.03 | 0 | 1 |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 17.68 | 19.35 | −1.67 ± 0.15 | −4.80 ± 0.18 | 11 | 28 |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 3.87 | 4.13 | −0.25 ± 0.10 | −1.00 ± 0.08 | 4 | 3 |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 5.47 | 6.36 | −0.89 ± 0.08 | −1.79 ± 0.09 | 5 | 14 |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.70 | 1.81 | −0.11 ± 0.02 | −0.38 ± 0.03 | 2 | 1 |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 13.06 | 14.10 | −1.04 ± 0.08 | −2.87 ± 0.12 | 9 | 20 |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 4.13 | 4.38 | −0.25 ± 0.07 | −0.81 ± 0.06 | 6 | 4 |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 9.24 | 9.13 | +0.11 ± 0.17 | −1.52 ± 0.12 | 6 | 9 |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.41 | 1.41 | 0.00 ± 0.02 | −0.12 ± 0.03 | 1 | 2 |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 12.66 | 13.15 | −0.49 ± 0.16 | −2.74 ± 0.15 | 9 | 17 |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 2.33 | 2.79 | −0.46 ± 0.08 | −0.91 ± 0.05 | 2 | 3 |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 10.44 | 11.31 | −0.87 ± 0.09 | −2.71 ± 0.14 | 8 | 13 |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.81 | 0.79 | +0.02 ± 0.01 | −0.28 ± 0.03 | 3 | 0 |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 13.12 | 15.38 | −2.26 ± 0.18 | −5.14 ± 0.11 | 22 | 31 |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 1.41 | 1.46 | −0.05 ± 0.05 | −0.65 ± 0.04 | 2 | 2 |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 9.61 | 11.81 | −2.20 ± 0.12 | −4.12 ± 0.11 | 8 | 22 |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.23 | 0.22 | 0.00 ± 0.01 | −0.16 ± 0.02 | 1 | 1 |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 17.81 | 19.35 | −1.53 ± 0.09 | −4.27 ± 0.13 | 8 | 27 |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 3.89 | 4.13 | −0.24 ± 0.13 | −0.83 ± 0.07 | 4 | 3 |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 5.34 | 6.36 | −1.02 ± 0.04 | −1.97 ± 0.06 | 5 | 15 |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.76 | 1.81 | −0.04 ± 0.02 | −0.32 ± 0.02 | 2 | 1 |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 12.98 | 14.10 | −1.11 ± 0.08 | −2.73 ± 0.13 | 10 | 14 |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 4.25 | 4.38 | −0.13 ± 0.03 | −0.64 ± 0.04 | 4 | 5 |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.78 | 9.13 | −0.35 ± 0.05 | −1.62 ± 0.10 | 8 | 12 |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.39 | 1.41 | −0.01 ± 0.01 | −0.22 ± 0.03 | 1 | 4 |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 12.16 | 13.15 | −0.99 ± 0.25 | −3.19 ± 0.09 | 17 | 18 |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 2.26 | 2.79 | −0.53 ± 0.05 | −0.84 ± 0.07 | 0 | 5 |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 10.64 | 11.31 | −0.67 ± 0.18 | −2.48 ± 0.09 | 12 | 16 |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.65 | 0.79 | −0.14 ± 0.02 | −0.26 ± 0.02 | 2 | 2 |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 13.26 | 15.38 | −2.12 ± 0.28 | −4.90 ± 0.12 | 29 | 32 |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 1.27 | 1.46 | −0.19 ± 0.04 | −0.74 ± 0.06 | 4 | 4 |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 9.77 | 11.81 | −2.05 ± 0.12 | −3.85 ± 0.09 | 9 | 17 |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.25 | 0.22 | +0.02 ± 0.02 | −0.14 ± 0.02 | 1 | 0 |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 17.98 | 19.35 | −1.37 ± 0.09 | −3.37 ± 0.13 | 15 | 24 |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 4.06 | 4.13 | −0.06 ± 0.05 | −0.52 ± 0.03 | 14 | 2 |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 5.48 | 6.36 | −0.87 ± 0.07 | −2.00 ± 0.09 | 3 | 13 |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.52 | 1.81 | −0.29 ± 0.02 | −0.49 ± 0.03 | 1 | 3 |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 12.56 | 14.10 | −1.54 ± 0.09 | −2.97 ± 0.06 | 15 | 16 |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 3.81 | 4.38 | −0.57 ± 0.02 | −1.09 ± 0.06 | 1 | 8 |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.41 | 9.13 | −0.72 ± 0.11 | −1.66 ± 0.09 | 3 | 11 |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.36 | 1.41 | −0.05 ± 0.02 | −0.16 ± 0.02 | 5 | 1 |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 11.74 | 13.15 | −1.41 ± 0.14 | −3.46 ± 0.09 | 24 | 22 |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 2.13 | 2.79 | −0.66 ± 0.03 | −0.90 ± 0.06 | 3 | 4 |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 10.62 | 11.31 | −0.69 ± 0.05 | −2.25 ± 0.09 | 7 | 18 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.62 | 0.79 | −0.17 ± 0.04 | −0.26 ± 0.03 | 6 | 3 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 11.43 | 15.38 | −3.95 ± 0.47 | −6.51 ± 0.20 | 82 | 36 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 0.81 | 1.46 | −0.65 ± 0.05 | −1.18 ± 0.08 | 6 | 7 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 6.31 | 11.81 | −5.50 ± 0.11 | −6.80 ± 0.10 | 10 | 29 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.25 | 0.22 | +0.03 ± 0.01 | −0.12 ± 0.02 | 2 | 2 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 11.98 | 19.35 | −7.37 ± 0.17 | −8.85 ± 0.12 | 33 | 38 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 3.39 | 4.13 | −0.73 ± 0.12 | −1.05 ± 0.04 | 24 | 3 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 4.00 | 6.36 | −2.36 ± 0.09 | −3.22 ± 0.09 | 8 | 20 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.19 | 1.81 | −0.62 ± 0.04 | −0.75 ± 0.04 | 3 | 4 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 9.96 | 14.10 | −4.13 ± 0.06 | −5.22 ± 0.08 | 36 | 30 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 3.57 | 4.38 | −0.81 ± 0.09 | −1.34 ± 0.06 | 14 | 6 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 6.81 | 9.13 | −2.32 ± 0.29 | −2.89 ± 0.08 | 14 | 21 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.31 | 1.41 | −0.10 ± 0.03 | −0.21 ± 0.03 | 7 | 2 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 10.55 | 13.15 | −2.60 ± 0.22 | −4.44 ± 0.25 | 68 | 21 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 1.08 | 2.79 | −1.72 ± 0.18 | −1.87 ± 0.15 | 10 | 6 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 8.49 | 11.31 | −2.82 ± 0.18 | −4.11 ± 0.25 | 19 | 26 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.83 | 0.62 | +0.22 ± 0.02 | – | 2 | 5 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 13.55 | 11.43 | +2.11 ± 0.44 | – | 26 | 62 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 1.38 | 0.81 | +0.57 ± 0.05 | – | 7 | 5 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 10.23 | 6.31 | +3.92 ± 0.10 | – | 23 | 5 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.22 | 0.25 | −0.03 ± 0.01 | – | 1 | 3 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 17.15 | 11.98 | +5.17 ± 0.27 | – | 37 | 26 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 4.08 | 3.39 | +0.69 ± 0.07 | – | 5 | 16 |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 5.58 | 4.00 | +1.58 ± 0.07 | – | 17 | 7 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.61 | 1.19 | +0.43 ± 0.03 | – | 3 | 3 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 12.56 | 9.96 | +2.60 ± 0.11 | – | 27 | 23 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 3.79 | 3.57 | +0.22 ± 0.07 | – | 6 | 17 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 9.23 | 6.81 | +2.42 ± 0.23 | – | 24 | 9 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.35 | 1.31 | +0.04 ± 0.02 | – | 2 | 5 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 12.35 | 10.55 | +1.80 ± 0.07 | – | 22 | 58 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 2.21 | 1.08 | +1.13 ± 0.15 | – | 6 | 8 |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 10.16 | 8.49 | +1.67 ± 0.05 | – | 23 | 12 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.83 | 0.62 | +0.21 ± 0.05 | – | 2 | 2 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 13.91 | 11.43 | +2.48 ± 0.23 | – | 21 | 27 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 1.35 | 0.81 | +0.54 ± 0.07 | – | 4 | 2 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 10.00 | 6.31 | +3.68 ± 0.23 | – | 17 | 2 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.22 | 0.25 | −0.04 ± 0.01 | – | 0 | 0 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 17.68 | 11.98 | +5.71 ± 0.21 | – | 22 | 6 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 3.87 | 3.39 | +0.48 ± 0.08 | – | 2 | 9 |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 5.47 | 4.00 | +1.47 ± 0.14 | – | 10 | 4 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.70 | 1.19 | +0.51 ± 0.04 | – | 3 | 0 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 13.06 | 9.96 | +3.10 ± 0.15 | – | 18 | 9 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 4.13 | 3.57 | +0.56 ± 0.16 | – | 3 | 5 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 9.24 | 6.81 | +2.43 ± 0.30 | – | 15 | 6 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.41 | 1.31 | +0.10 ± 0.04 | – | 3 | 3 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 12.66 | 10.55 | +2.11 ± 0.26 | – | 15 | 30 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 2.33 | 1.08 | +1.25 ± 0.12 | – | 5 | 4 |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 10.44 | 8.49 | +1.95 ± 0.21 | – | 16 | 4 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.81 | 0.62 | +0.19 ± 0.05 | – | 2 | 1 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 13.12 | 11.43 | +1.69 ± 0.33 | – | 20 | 30 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 1.41 | 0.81 | +0.60 ± 0.05 | – | 4 | 3 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 9.61 | 6.31 | +3.30 ± 0.14 | – | 19 | 3 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.23 | 0.25 | −0.03 ± 0.02 | – | 0 | 1 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 17.81 | 11.98 | +5.84 ± 0.17 | – | 23 | 7 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 3.89 | 3.39 | +0.50 ± 0.08 | – | 3 | 10 |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 5.34 | 4.00 | +1.34 ± 0.12 | – | 12 | 2 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.76 | 1.19 | +0.58 ± 0.05 | – | 3 | 0 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 12.98 | 9.96 | +3.02 ± 0.15 | – | 19 | 10 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 4.25 | 3.57 | +0.68 ± 0.13 | – | 6 | 5 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.78 | 6.81 | +1.97 ± 0.29 | – | 13 | 5 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.39 | 1.31 | +0.08 ± 0.04 | – | 1 | 4 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 12.16 | 10.55 | +1.61 ± 0.21 | – | 13 | 25 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 2.26 | 1.08 | +1.18 ± 0.15 | – | 4 | 4 |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 10.64 | 8.49 | +2.15 ± 0.12 | – | 15 | 3 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | D's own | 25 | 0.65 | 0.62 | +0.03 ± 0.03 | – | 1 | 2 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's pattern targets | 252 | 13.26 | 11.43 | +1.82 ± 0.30 | – | 24 | 37 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | readout 1's other excitatory | 80 | 1.27 | 0.81 | +0.47 ± 0.03 | – | 5 | 2 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 9.77 | 6.31 | +3.45 ± 0.12 | – | 21 | 1 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | C's own | 25 | 0.25 | 0.25 | 0.00 ± 0.02 | – | 0 | 2 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 17.98 | 11.98 | +6.01 ± 0.21 | – | 25 | 8 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 4.06 | 3.39 | +0.67 ± 0.11 | – | 2 | 11 |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 5.48 | 4.00 | +1.48 ± 0.11 | – | 13 | 1 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | D's own | 25 | 1.52 | 1.19 | +0.34 ± 0.03 | – | 4 | 0 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's pattern targets | 252 | 12.56 | 9.96 | +2.60 ± 0.16 | – | 21 | 8 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | readout 1's other excitatory | 80 | 3.81 | 3.57 | +0.23 ± 0.10 | – | 3 | 7 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | inhibitory in readout 1 | 102 | 8.41 | 6.81 | +1.60 ± 0.34 | – | 15 | 6 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | C's own | 25 | 1.36 | 1.31 | +0.05 ± 0.03 | – | 1 | 2 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's pattern targets | 245 | 11.74 | 10.55 | +1.19 ± 0.19 | – | 15 | 32 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | readout 0's other excitatory | 87 | 2.13 | 1.08 | +1.06 ± 0.20 | – | 7 | 1 |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | inhibitory in readout 0 | 102 | 10.62 | 8.49 | +2.13 ± 0.20 | – | 19 | 6 |

At block 36 every class sum of each single night lies nearer the night without replay than right after its replayed night, all 64, and so do
H-47's plain night's 16 against block 24. Of the single nights' 64, 46 lie below beyond 2 SE, 15 within 2 SE and three above beyond 2 SE:
D's own on A's trials after block 6, 12 and 18 alone. On A's trials readout 1's pattern targets lie 1.47 to 2.26 below and its inhibitory
units 1.58 to 2.20, readout 0's pattern targets 1.37 to 2.20. Under H-47's plain night readout 1's inhibitory units on A's trials lie 5.50
below and readout 0's pattern targets 7.37.

**The fewest units carrying half of each readout's difference at block 36**:

| Comparison | Trials | Readout | Fewest units | Their differences' sum | Their classes | Among the night without replay's units carrying half its spikes | The units, by id and class, the largest difference first |
| :--- | :--- | :--- | ---: | ---: | :--- | ---: | :--- |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | 7 | −1.84 | T 4, I 3 | 2 | 144 I, 862 T, 286 T, 338 T, 854 I, 342 T, 784 I |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | 6 | −1.60 | T 4, E 1, I 1 | 4 | 143 T, 495 T, 1005 T, 801 E, 655 T, 499 I |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | 4 | −1.28 | T 3, D 1 | 0 | 22 T, 108 T, 376 T, 276 D |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | 6 | −1.41 | T 2, E 1, I 3 | 1 | 49 I, 101 E, 43 T, 329 I, 179 I, 1017 T |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | 7 | −1.75 | T 3, I 4 | 3 | 774 I, 862 T, 286 T, 144 I, 854 I, 278 T, 384 I |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | 5 | −1.44 | T 3, E 1, I 1 | 4 | 285 T, 819 I, 801 E, 143 T, 461 T |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | 3 | −0.85 | T 2, E 1 | 1 | 108 T, 22 T, 56 E |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | 3 | −0.91 | E 1, I 2 | 0 | 329 I, 49 I, 101 E |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | 8 | −2.27 | T 4, I 4 | 4 | 774 I, 862 T, 286 T, 144 I, 278 T, 854 I, 648 T, 804 I |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | 6 | −1.44 | T 3, E 1, I 2 | 4 | 285 T, 819 I, 801 E, 143 T, 461 T, 649 I |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | 4 | −0.94 | T 3, I 1 | 1 | 108 T, 22 T, 314 I, 346 T |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | 6 | −1.22 | T 2, E 2, I 2 | 2 | 49 I, 101 E, 915 E, 329 I, 5 T, 75 T |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | 10 | −2.40 | T 5, I 5 | 5 | 974 I, 774 I, 966 T, 338 T, 286 T, 804 I, 784 I, 226 T, 648 T, 384 I |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | 6 | −1.18 | T 4, E 1, I 1 | 4 | 819 I, 801 E, 343 T, 655 T, 143 T, 461 T |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | 6 | −1.63 | T 3, E 1, I 1, D 1 | 2 | 346 T, 108 T, 22 T, 276 D, 314 I, 16 E |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | 5 | −1.41 | T 2, E 2, I 1 | 2 | 5 T, 915 E, 329 I, 7 T, 101 E |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | 10 | −5.37 | T 1, I 9 | 6 | 774 I, 338 T, 144 I, 384 I, 974 I, 834 I, 784 I, 804 I, 864 I, 604 I |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | 8 | −5.39 | T 6, E 2 | 6 | 655 T, 285 T, 495 T, 1005 T, 801 E, 861 E, 461 T, 143 T |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | 8 | −4.26 | T 5, I 2, D 1 | 3 | 230 T, 346 T, 376 T, 108 T, 524 I, 276 D, 314 I, 22 T |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | 7 | −3.69 | T 3, E 2, I 2 | 5 | 5 T, 325 T, 915 E, 801 E, 329 I, 7 T, 179 I |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 8 | +3.59 | T 2, I 6 | 5 | 774 I, 384 I, 834 I, 974 I, 338 T, 864 I, 226 T, 804 I |
| After block 6 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 8 | +4.03 | T 5, E 2, I 1 | 5 | 861 E, 655 T, 285 T, 819 I, 461 T, 801 E, 387 T, 565 T |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 6 | +3.08 | T 2, I 4 | 3 | 230 T, 346 T, 314 I, 524 I, 164 I, 324 I |
| After block 6 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 5 | +2.46 | T 3, E 2 | 4 | 5 T, 801 E, 325 T, 7 T, 915 E |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 9 | +3.73 | T 2, E 1, I 6 | 5 | 774 I, 338 T, 974 I, 226 T, 834 I, 384 I, 864 I, 144 I, 830 E |
| After block 12 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 8 | +4.16 | T 6, E 2 | 5 | 655 T, 495 T, 861 E, 1005 T, 461 T, 565 T, 801 E, 337 T |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 7 | +3.57 | T 3, E 1, I 3 | 4 | 230 T, 346 T, 524 I, 314 I, 16 E, 164 I, 376 T |
| After block 12 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 6 | +2.92 | T 3, E 2, I 1 | 4 | 5 T, 915 E, 801 E, 7 T, 325 T, 659 I |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 8 | +3.06 | T 1, E 1, I 6 | 4 | 338 T, 774 I, 834 I, 384 I, 974 I, 144 I, 784 I, 830 E |
| After block 18 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 7 | +4.04 | T 5, E 1, I 1 | 4 | 655 T, 495 T, 1005 T, 861 E, 565 T, 461 T, 499 I |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 8 | +3.35 | T 3, E 1, I 3, D 1 | 4 | 230 T, 346 T, 524 I, 276 D, 376 T, 324 I, 16 E, 164 I |
| After block 18 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 6 | +2.64 | T 4, E 2 | 4 | 801 E, 325 T, 5 T, 915 E, 7 T, 43 T |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 1, the even places | 7 | +2.95 | T 2, E 1, I 4 | 4 | 774 I, 144 I, 834 I, 338 T, 384 I, 830 E, 278 T |
| After block 24 alone less plain on all four nights, block 36 | A's trials | readout 0, the odd places | 7 | +4.19 | T 6, E 1 | 5 | 861 E, 495 T, 655 T, 1005 T, 285 T, 461 T, 143 T |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 1, the even places | 6 | +2.52 | T 3, E 1, I 2 | 5 | 230 T, 524 I, 376 T, 346 T, 304 I, 56 E |
| After block 24 alone less plain on all four nights, block 36 | B's trials | readout 0, the odd places | 6 | +2.41 | T 2, E 1, I 3 | 3 | 801 E, 325 T, 49 I, 779 I, 43 T, 849 I |

At block 36, 3 to 10 units carry half of each single night's readout difference, and 7 to 10 of H-47's plain night's, nine of the ten of
readout 1 on A's trials inhibitory units.

**Each unit at block 36 beside right after its replayed night**, over all of a readout's units: the readout's difference right after and at
block 36 and the change between; how many units moved right after, and of those how many moved the same way at block 36; how many moved at
block 36 and not right after; and the correlation of the units' differences right after with theirs at block 36, over the readout's
459 units:

| Form | Blocks | Trials | Readout | Less without replay right after | At block 36 | Change | Moved right after, down and up | Of those, moved the same way at 36, down and up | Moved at 36 and not right after | Correlation of the units' differences, right after with 36 |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone | 6 → 36 | A's trials | readout 1, the even places | −7.45 ± 0.20 | −3.45 ± 0.07 | +4.00 | 130, 4 | 40, 3 | 36 | 0.68 |
| After block 6 alone | 6 → 36 | A's trials | readout 0, the odd places | −7.92 ± 0.21 | −3.04 ± 0.17 | +4.88 | 126, 0 | 32, 0 | 36 | 0.63 |
| After block 6 alone | 6 → 36 | B's trials | readout 1, the even places | −5.47 ± 0.19 | −2.21 ± 0.08 | +3.26 | 99, 4 | 26, 0 | 30 | 0.42 |
| After block 6 alone | 6 → 36 | B's trials | readout 0, the odd places | −6.27 ± 0.16 | −2.58 ± 0.30 | +3.68 | 109, 3 | 31, 2 | 41 | 0.55 |
| After block 12 alone | 12 → 36 | A's trials | readout 1, the even places | −9.19 ± 0.20 | −3.35 ± 0.47 | +5.84 | 117, 2 | 35, 1 | 37 | 0.64 |
| After block 12 alone | 12 → 36 | A's trials | readout 0, the odd places | −7.73 ± 0.22 | −2.82 ± 0.24 | +4.92 | 125, 0 | 37, 0 | 22 | 0.59 |
| After block 12 alone | 12 → 36 | B's trials | readout 1, the even places | −5.59 ± 0.14 | −1.28 ± 0.20 | +4.31 | 108, 1 | 26, 0 | 27 | 0.34 |
| After block 12 alone | 12 → 36 | B's trials | readout 0, the odd places | −6.49 ± 0.24 | −1.82 ± 0.29 | +4.67 | 102, 3 | 28, 2 | 22 | 0.57 |
| After block 18 alone | 18 → 36 | A's trials | readout 1, the even places | −10.19 ± 0.24 | −4.49 ± 0.22 | +5.70 | 136, 2 | 51, 1 | 30 | 0.87 |
| After block 18 alone | 18 → 36 | A's trials | readout 0, the odd places | −7.23 ± 0.14 | −2.79 ± 0.20 | +4.43 | 134, 0 | 40, 0 | 15 | 0.79 |
| After block 18 alone | 18 → 36 | B's trials | readout 1, the even places | −5.31 ± 0.21 | −1.64 ± 0.09 | +3.67 | 107, 3 | 28, 1 | 20 | 0.77 |
| After block 18 alone | 18 → 36 | B's trials | readout 0, the odd places | −6.74 ± 0.20 | −2.20 ± 0.40 | +4.54 | 116, 2 | 37, 2 | 29 | 0.80 |
| After block 24 alone | 24 → 36 | A's trials | readout 1, the even places | −9.75 ± 0.22 | −4.49 ± 0.37 | +5.26 | 133, 1 | 47, 0 | 41 | 0.92 |
| After block 24 alone | 24 → 36 | A's trials | readout 0, the odd places | −6.03 ± 0.18 | −2.28 ± 0.15 | +3.75 | 124, 1 | 34, 0 | 33 | 0.89 |
| After block 24 alone | 24 → 36 | B's trials | readout 1, the even places | −6.21 ± 0.15 | −3.12 ± 0.15 | +3.10 | 109, 1 | 34, 1 | 21 | 0.98 |
| After block 24 alone | 24 → 36 | B's trials | readout 0, the odd places | −6.77 ± 0.17 | −2.80 ± 0.19 | +3.98 | 95, 5 | 38, 3 | 36 | 0.93 |
| Plain on all four nights (H-47) | 24 → 36 | A's trials | readout 1, the even places | −14.75 ± 0.25 | −10.26 ± 0.48 | +4.49 | 105, 37 | 66, 28 | 81 | 0.98 |
| Plain on all four nights (H-47) | 24 → 36 | A's trials | readout 0, the odd places | −13.23 ± 0.16 | −10.43 ± 0.36 | +2.80 | 121, 21 | 52, 12 | 63 | 0.99 |
| Plain on all four nights (H-47) | 24 → 36 | B's trials | readout 1, the even places | −10.20 ± 0.17 | −7.88 ± 0.33 | +2.32 | 94, 23 | 51, 14 | 60 | 0.99 |
| Plain on all four nights (H-47) | 24 → 36 | B's trials | readout 0, the odd places | −10.63 ± 0.58 | −7.23 ± 0.26 | +3.40 | 75, 47 | 45, 28 | 83 | 0.98 |

From right after its replayed night to block 36 each single night's readouts came back by 3.10 to 5.84 spikes a trial, and H-47's plain
night's by 2.32 to 4.49 from block 24. Under each single night, of the units moved down right after, 26 to 51 still lay below beyond 2 SE at
block 36, and 15 to 41 units moved at block 36 that had not moved either way right after. The units' differences at block 36 correlate with
theirs right after by 0.34 to 0.98, the least on B's trials' readout 1 after block 12 alone and the most there after block 24 alone; under
H-47's plain night by 0.98 to 0.99.

**Each single night's fewest units right after its replayed night**, unit by unit, beside the same unit at block 36:

| Form | Trials | Readout | Unit | Place | Class | Less without replay right after its replayed night | At block 36 | Change |
| :--- | :--- | :--- | :--- | :--- | :--- | ---: | ---: | ---: |
| After block 6 alone | A's trials | readout 1, the even places | 338 | 18 | readout 1's pattern targets | −0.39 ± 0.02 | −0.25 ± 0.02 | +0.14 |
| After block 6 alone | A's trials | readout 1, the even places | 774 | 14 | inhibitory in readout 1 | −0.36 ± 0.02 | −0.12 ± 0.03 | +0.24 |
| After block 6 alone | A's trials | readout 1, the even places | 144 | 4 | inhibitory in readout 1 | −0.31 ± 0.03 | −0.48 ± 0.02 | −0.17 |
| After block 6 alone | A's trials | readout 1, the even places | 832 | 12 | readout 1's pattern targets | −0.29 ± 0.02 | −0.15 ± 0.04 | +0.14 |
| After block 6 alone | A's trials | readout 1, the even places | 830 | 10 | readout 1's other excitatory | −0.28 ± 0.02 | −0.15 ± 0.04 | +0.14 |
| After block 6 alone | A's trials | readout 1, the even places | 784 | 4 | inhibitory in readout 1 | −0.25 ± 0.02 | −0.16 ± 0.01 | +0.10 |
| After block 6 alone | A's trials | readout 1, the even places | 974 | 14 | inhibitory in readout 1 | −0.24 ± 0.03 | −0.10 ± 0.03 | +0.15 |
| After block 6 alone | A's trials | readout 1, the even places | 756 | 16 | D's own | −0.22 ± 0.02 | +0.05 ± 0.02 | +0.27 |
| After block 6 alone | A's trials | readout 1, the even places | 322 | 2 | readout 1's pattern targets | −0.22 ± 0.02 | −0.08 ± 0.03 | +0.13 |
| After block 6 alone | A's trials | readout 1, the even places | 438 | 18 | readout 1's pattern targets | −0.21 ± 0.02 | −0.14 ± 0.02 | +0.06 |
| After block 6 alone | A's trials | readout 1, the even places | 642 | 2 | readout 1's pattern targets | −0.18 ± 0.03 | −0.01 ± 0.03 | +0.17 |
| After block 6 alone | A's trials | readout 1, the even places | 604 | 4 | inhibitory in readout 1 | −0.16 ± 0.02 | −0.15 ± 0.03 | +0.01 |
| After block 6 alone | A's trials | readout 1, the even places | 4 | 4 | inhibitory in readout 1 | −0.16 ± 0.03 | 0.00 ± 0.03 | +0.16 |
| After block 6 alone | A's trials | readout 1, the even places | 804 | 4 | inhibitory in readout 1 | −0.16 ± 0.02 | −0.12 ± 0.09 | +0.04 |
| After block 6 alone | A's trials | readout 1, the even places | 648 | 8 | readout 1's pattern targets | −0.15 ± 0.02 | −0.15 ± 0.01 | 0.00 |
| After block 6 alone | A's trials | readout 1, the even places | 6 | 6 | readout 1's pattern targets | −0.15 ± 0.02 | 0.00 ± 0.03 | +0.15 |
| After block 6 alone | A's trials | readout 0, the odd places | 7 | 7 | readout 0's pattern targets | −0.40 ± 0.04 | −0.01 ± 0.03 | +0.40 |
| After block 6 alone | A's trials | readout 0, the odd places | 461 | 1 | readout 0's pattern targets | −0.37 ± 0.02 | −0.14 ± 0.01 | +0.23 |
| After block 6 alone | A's trials | readout 0, the odd places | 5 | 5 | readout 0's pattern targets | −0.30 ± 0.04 | −0.01 ± 0.02 | +0.29 |
| After block 6 alone | A's trials | readout 0, the odd places | 801 | 1 | readout 0's other excitatory | −0.30 ± 0.02 | −0.21 ± 0.11 | +0.08 |
| After block 6 alone | A's trials | readout 0, the odd places | 143 | 3 | readout 0's pattern targets | −0.29 ± 0.03 | −0.47 ± 0.02 | −0.17 |
| After block 6 alone | A's trials | readout 0, the odd places | 685 | 5 | readout 0's pattern targets | −0.29 ± 0.03 | +0.03 ± 0.01 | +0.32 |
| After block 6 alone | A's trials | readout 0, the odd places | 1005 | 5 | readout 0's pattern targets | −0.28 ± 0.02 | −0.28 ± 0.06 | 0.00 |
| After block 6 alone | A's trials | readout 0, the odd places | 655 | 15 | readout 0's pattern targets | −0.27 ± 0.02 | −0.20 ± 0.03 | +0.07 |
| After block 6 alone | A's trials | readout 0, the odd places | 507 | 7 | readout 0's pattern targets | −0.27 ± 0.02 | 0.00 ± 0.01 | +0.27 |
| After block 6 alone | A's trials | readout 0, the odd places | 495 | 15 | readout 0's pattern targets | −0.24 ± 0.02 | −0.29 ± 0.02 | −0.05 |
| After block 6 alone | A's trials | readout 0, the odd places | 861 | 1 | readout 0's other excitatory | −0.23 ± 0.02 | −0.06 ± 0.01 | +0.17 |
| After block 6 alone | A's trials | readout 0, the odd places | 337 | 17 | readout 0's pattern targets | −0.23 ± 0.01 | −0.15 ± 0.01 | +0.07 |
| After block 6 alone | A's trials | readout 0, the odd places | 339 | 19 | inhibitory in readout 0 | −0.22 ± 0.02 | −0.15 ± 0.02 | +0.07 |
| After block 6 alone | A's trials | readout 0, the odd places | 3 | 3 | readout 0's pattern targets | −0.21 ± 0.03 | −0.01 ± 0.03 | +0.21 |
| After block 6 alone | A's trials | readout 0, the odd places | 343 | 3 | readout 0's pattern targets | −0.21 ± 0.02 | −0.12 ± 0.01 | +0.09 |
| After block 6 alone | B's trials | readout 1, the even places | 230 | 10 | readout 1's pattern targets | −0.46 ± 0.03 | +0.04 ± 0.01 | +0.49 |
| After block 6 alone | B's trials | readout 1, the even places | 164 | 4 | inhibitory in readout 1 | −0.21 ± 0.02 | +0.04 ± 0.06 | +0.26 |
| After block 6 alone | B's trials | readout 1, the even places | 376 | 16 | readout 1's pattern targets | −0.21 ± 0.02 | −0.30 ± 0.02 | −0.09 |
| After block 6 alone | B's trials | readout 1, the even places | 276 | 16 | D's own | −0.20 ± 0.02 | −0.19 ± 0.02 | +0.02 |
| After block 6 alone | B's trials | readout 1, the even places | 156 | 16 | D's own | −0.20 ± 0.02 | 0.00 ± 0.01 | +0.20 |
| After block 6 alone | B's trials | readout 1, the even places | 346 | 6 | readout 1's pattern targets | −0.18 ± 0.02 | −0.12 ± 0.05 | +0.06 |
| After block 6 alone | B's trials | readout 1, the even places | 56 | 16 | readout 1's other excitatory | −0.17 ± 0.02 | −0.16 ± 0.02 | +0.01 |
| After block 6 alone | B's trials | readout 1, the even places | 270 | 10 | readout 1's other excitatory | −0.16 ± 0.02 | −0.11 ± 0.02 | +0.05 |
| After block 6 alone | B's trials | readout 1, the even places | 304 | 4 | inhibitory in readout 1 | −0.16 ± 0.03 | +0.01 ± 0.00 | +0.17 |
| After block 6 alone | B's trials | readout 1, the even places | 326 | 6 | readout 1's pattern targets | −0.15 ± 0.03 | +0.01 ± 0.02 | +0.17 |
| After block 6 alone | B's trials | readout 1, the even places | 918 | 18 | readout 1's pattern targets | −0.15 ± 0.02 | −0.11 ± 0.00 | +0.04 |
| After block 6 alone | B's trials | readout 1, the even places | 16 | 16 | readout 1's other excitatory | −0.14 ± 0.02 | −0.08 ± 0.03 | +0.06 |
| After block 6 alone | B's trials | readout 1, the even places | 68 | 8 | readout 1's other excitatory | −0.14 ± 0.02 | −0.11 ± 0.01 | +0.03 |
| After block 6 alone | B's trials | readout 1, the even places | 314 | 14 | inhibitory in readout 1 | −0.13 ± 0.02 | +0.03 ± 0.02 | +0.16 |
| After block 6 alone | B's trials | readout 1, the even places | 108 | 8 | readout 1's pattern targets | −0.11 ± 0.01 | −0.38 ± 0.02 | −0.28 |
| After block 6 alone | B's trials | readout 0, the odd places | 801 | 1 | readout 0's other excitatory | −0.34 ± 0.02 | −0.04 ± 0.02 | +0.29 |
| After block 6 alone | B's trials | readout 0, the odd places | 179 | 19 | inhibitory in readout 0 | −0.25 ± 0.03 | −0.21 ± 0.03 | +0.04 |
| After block 6 alone | B's trials | readout 0, the odd places | 849 | 9 | inhibitory in readout 0 | −0.22 ± 0.02 | −0.04 ± 0.04 | +0.18 |
| After block 6 alone | B's trials | readout 0, the odd places | 619 | 19 | inhibitory in readout 0 | −0.22 ± 0.03 | −0.12 ± 0.01 | +0.10 |
| After block 6 alone | B's trials | readout 0, the odd places | 325 | 5 | readout 0's pattern targets | −0.22 ± 0.03 | −0.13 ± 0.02 | +0.09 |
| After block 6 alone | B's trials | readout 0, the odd places | 779 | 19 | inhibitory in readout 0 | −0.20 ± 0.02 | −0.09 ± 0.04 | +0.11 |
| After block 6 alone | B's trials | readout 0, the odd places | 935 | 15 | readout 0's pattern targets | −0.18 ± 0.02 | −0.10 ± 0.00 | +0.08 |
| After block 6 alone | B's trials | readout 0, the odd places | 913 | 13 | readout 0's pattern targets | −0.17 ± 0.01 | −0.09 ± 0.01 | +0.08 |
| After block 6 alone | B's trials | readout 0, the odd places | 937 | 17 | readout 0's pattern targets | −0.17 ± 0.02 | −0.05 ± 0.03 | +0.12 |
| After block 6 alone | B's trials | readout 0, the odd places | 115 | 15 | readout 0's pattern targets | −0.16 ± 0.02 | +0.01 ± 0.01 | +0.16 |
| After block 6 alone | B's trials | readout 0, the odd places | 853 | 13 | readout 0's pattern targets | −0.15 ± 0.02 | −0.02 ± 0.04 | +0.13 |
| After block 6 alone | B's trials | readout 0, the odd places | 1009 | 9 | inhibitory in readout 0 | −0.15 ± 0.02 | −0.11 ± 0.01 | +0.04 |
| After block 6 alone | B's trials | readout 0, the odd places | 939 | 19 | inhibitory in readout 0 | −0.15 ± 0.01 | −0.04 ± 0.03 | +0.11 |
| After block 6 alone | B's trials | readout 0, the odd places | 1017 | 17 | readout 0's pattern targets | −0.13 ± 0.01 | −0.16 ± 0.04 | −0.03 |
| After block 6 alone | B's trials | readout 0, the odd places | 229 | 9 | inhibitory in readout 0 | −0.13 ± 0.02 | +0.08 ± 0.02 | +0.20 |
| After block 6 alone | B's trials | readout 0, the odd places | 689 | 9 | inhibitory in readout 0 | −0.11 ± 0.02 | −0.02 ± 0.02 | +0.09 |
| After block 6 alone | B's trials | readout 0, the odd places | 29 | 9 | inhibitory in readout 0 | −0.11 ± 0.01 | −0.04 ± 0.01 | +0.07 |
| After block 6 alone | B's trials | readout 0, the odd places | 43 | 3 | readout 0's pattern targets | −0.11 ± 0.01 | −0.22 ± 0.04 | −0.11 |
| After block 12 alone | A's trials | readout 1, the even places | 774 | 14 | inhibitory in readout 1 | −0.43 ± 0.03 | −0.32 ± 0.06 | +0.11 |
| After block 12 alone | A's trials | readout 1, the even places | 974 | 14 | inhibitory in readout 1 | −0.40 ± 0.03 | −0.09 ± 0.03 | +0.31 |
| After block 12 alone | A's trials | readout 1, the even places | 678 | 18 | readout 1's pattern targets | −0.33 ± 0.03 | 0.00 ± 0.04 | +0.34 |
| After block 12 alone | A's trials | readout 1, the even places | 830 | 10 | readout 1's other excitatory | −0.30 ± 0.03 | −0.04 ± 0.06 | +0.26 |
| After block 12 alone | A's trials | readout 1, the even places | 832 | 12 | readout 1's pattern targets | −0.27 ± 0.03 | −0.04 ± 0.06 | +0.23 |
| After block 12 alone | A's trials | readout 1, the even places | 6 | 6 | readout 1's pattern targets | −0.25 ± 0.05 | −0.05 ± 0.01 | +0.20 |
| After block 12 alone | A's trials | readout 1, the even places | 144 | 4 | inhibitory in readout 1 | −0.24 ± 0.02 | −0.25 ± 0.01 | 0.00 |
| After block 12 alone | A's trials | readout 1, the even places | 826 | 6 | readout 1's pattern targets | −0.24 ± 0.03 | −0.09 ± 0.09 | +0.16 |
| After block 12 alone | A's trials | readout 1, the even places | 794 | 14 | inhibitory in readout 1 | −0.24 ± 0.02 | −0.10 ± 0.05 | +0.14 |
| After block 12 alone | A's trials | readout 1, the even places | 448 | 8 | readout 1's pattern targets | −0.24 ± 0.03 | −0.04 ± 0.04 | +0.19 |
| After block 12 alone | A's trials | readout 1, the even places | 966 | 6 | readout 1's pattern targets | −0.24 ± 0.03 | 0.00 ± 0.06 | +0.23 |
| After block 12 alone | A's trials | readout 1, the even places | 784 | 4 | inhibitory in readout 1 | −0.23 ± 0.02 | −0.17 ± 0.02 | +0.06 |
| After block 12 alone | A's trials | readout 1, the even places | 864 | 4 | inhibitory in readout 1 | −0.23 ± 0.03 | −0.04 ± 0.01 | +0.19 |
| After block 12 alone | A's trials | readout 1, the even places | 804 | 4 | inhibitory in readout 1 | −0.23 ± 0.03 | −0.17 ± 0.05 | +0.06 |
| After block 12 alone | A's trials | readout 1, the even places | 834 | 14 | inhibitory in readout 1 | −0.22 ± 0.03 | −0.13 ± 0.07 | +0.09 |
| After block 12 alone | A's trials | readout 1, the even places | 4 | 4 | inhibitory in readout 1 | −0.22 ± 0.05 | −0.04 ± 0.01 | +0.17 |
| After block 12 alone | A's trials | readout 1, the even places | 642 | 2 | readout 1's pattern targets | −0.22 ± 0.02 | −0.01 ± 0.02 | +0.20 |
| After block 12 alone | A's trials | readout 1, the even places | 338 | 18 | readout 1's pattern targets | −0.21 ± 0.03 | −0.10 ± 0.01 | +0.11 |
| After block 12 alone | A's trials | readout 0, the odd places | 685 | 5 | readout 0's pattern targets | −0.36 ± 0.03 | +0.15 ± 0.04 | +0.50 |
| After block 12 alone | A's trials | readout 0, the odd places | 833 | 13 | readout 0's pattern targets | −0.31 ± 0.03 | −0.06 ± 0.04 | +0.25 |
| After block 12 alone | A's trials | readout 0, the odd places | 819 | 19 | inhibitory in readout 0 | −0.31 ± 0.02 | −0.31 ± 0.03 | 0.00 |
| After block 12 alone | A's trials | readout 0, the odd places | 861 | 1 | readout 0's other excitatory | −0.30 ± 0.02 | −0.04 ± 0.02 | +0.26 |
| After block 12 alone | A's trials | readout 0, the odd places | 461 | 1 | readout 0's pattern targets | −0.30 ± 0.02 | −0.19 ± 0.01 | +0.11 |
| After block 12 alone | A's trials | readout 0, the odd places | 801 | 1 | readout 0's other excitatory | −0.28 ± 0.03 | −0.29 ± 0.07 | −0.01 |
| After block 12 alone | A's trials | readout 0, the odd places | 143 | 3 | readout 0's pattern targets | −0.25 ± 0.02 | −0.25 ± 0.02 | 0.00 |
| After block 12 alone | A's trials | readout 0, the odd places | 5 | 5 | readout 0's pattern targets | −0.25 ± 0.04 | −0.05 ± 0.01 | +0.20 |
| After block 12 alone | A's trials | readout 0, the odd places | 7 | 7 | readout 0's pattern targets | −0.24 ± 0.04 | −0.05 ± 0.01 | +0.20 |
| After block 12 alone | A's trials | readout 0, the odd places | 655 | 15 | readout 0's pattern targets | −0.23 ± 0.02 | −0.09 ± 0.01 | +0.13 |
| After block 12 alone | A's trials | readout 0, the odd places | 3 | 3 | readout 0's pattern targets | −0.23 ± 0.05 | −0.04 ± 0.01 | +0.18 |
| After block 12 alone | A's trials | readout 0, the odd places | 285 | 5 | readout 0's pattern targets | −0.23 ± 0.02 | −0.40 ± 0.03 | −0.18 |
| After block 12 alone | A's trials | readout 0, the odd places | 1005 | 5 | readout 0's pattern targets | −0.21 ± 0.02 | −0.07 ± 0.01 | +0.14 |
| After block 12 alone | A's trials | readout 0, the odd places | 829 | 9 | inhibitory in readout 0 | −0.21 ± 0.03 | −0.05 ± 0.05 | +0.16 |
| After block 12 alone | A's trials | readout 0, the odd places | 649 | 9 | inhibitory in readout 0 | −0.18 ± 0.02 | −0.07 ± 0.02 | +0.11 |
| After block 12 alone | B's trials | readout 1, the even places | 230 | 10 | readout 1's pattern targets | −0.35 ± 0.03 | +0.03 ± 0.01 | +0.39 |
| After block 12 alone | B's trials | readout 1, the even places | 6 | 6 | readout 1's pattern targets | −0.26 ± 0.04 | +0.01 ± 0.01 | +0.27 |
| After block 12 alone | B's trials | readout 1, the even places | 56 | 16 | readout 1's other excitatory | −0.19 ± 0.02 | −0.22 ± 0.06 | −0.03 |
| After block 12 alone | B's trials | readout 1, the even places | 276 | 16 | D's own | −0.18 ± 0.03 | −0.14 ± 0.02 | +0.04 |
| After block 12 alone | B's trials | readout 1, the even places | 164 | 4 | inhibitory in readout 1 | −0.18 ± 0.04 | +0.07 ± 0.04 | +0.25 |
| After block 12 alone | B's trials | readout 1, the even places | 314 | 14 | inhibitory in readout 1 | −0.17 ± 0.03 | −0.02 ± 0.02 | +0.15 |
| After block 12 alone | B's trials | readout 1, the even places | 376 | 16 | readout 1's pattern targets | −0.16 ± 0.02 | −0.15 ± 0.01 | +0.01 |
| After block 12 alone | B's trials | readout 1, the even places | 326 | 6 | readout 1's pattern targets | −0.16 ± 0.02 | −0.03 ± 0.02 | +0.13 |
| After block 12 alone | B's trials | readout 1, the even places | 324 | 4 | inhibitory in readout 1 | −0.15 ± 0.02 | −0.02 ± 0.02 | +0.13 |
| After block 12 alone | B's trials | readout 1, the even places | 346 | 6 | readout 1's pattern targets | −0.15 ± 0.03 | +0.01 ± 0.01 | +0.16 |
| After block 12 alone | B's trials | readout 1, the even places | 22 | 2 | readout 1's pattern targets | −0.13 ± 0.03 | −0.29 ± 0.05 | −0.15 |
| After block 12 alone | B's trials | readout 1, the even places | 304 | 4 | inhibitory in readout 1 | −0.11 ± 0.01 | −0.03 ± 0.01 | +0.08 |
| After block 12 alone | B's trials | readout 1, the even places | 16 | 16 | readout 1's other excitatory | −0.11 ± 0.02 | +0.03 ± 0.02 | +0.14 |
| After block 12 alone | B's trials | readout 1, the even places | 686 | 6 | readout 1's other excitatory | −0.11 ± 0.03 | 0.00 ± 0.01 | +0.11 |
| After block 12 alone | B's trials | readout 1, the even places | 156 | 16 | D's own | −0.09 ± 0.02 | +0.02 ± 0.02 | +0.11 |
| After block 12 alone | B's trials | readout 1, the even places | 246 | 6 | readout 1's pattern targets | −0.09 ± 0.02 | −0.02 ± 0.01 | +0.07 |
| After block 12 alone | B's trials | readout 1, the even places | 108 | 8 | readout 1's pattern targets | −0.08 ± 0.01 | −0.35 ± 0.02 | −0.27 |
| After block 12 alone | B's trials | readout 1, the even places | 334 | 14 | inhibitory in readout 1 | −0.08 ± 0.01 | −0.02 ± 0.02 | +0.06 |
| After block 12 alone | B's trials | readout 1, the even places | 658 | 18 | readout 1's pattern targets | −0.07 ± 0.02 | −0.06 ± 0.06 | +0.01 |
| After block 12 alone | B's trials | readout 0, the odd places | 801 | 1 | readout 0's other excitatory | −0.41 ± 0.03 | −0.03 ± 0.01 | +0.37 |
| After block 12 alone | B's trials | readout 0, the odd places | 453 | 13 | readout 0's pattern targets | −0.25 ± 0.02 | −0.17 ± 0.05 | +0.08 |
| After block 12 alone | B's trials | readout 0, the odd places | 179 | 19 | inhibitory in readout 0 | −0.24 ± 0.02 | −0.15 ± 0.07 | +0.10 |
| After block 12 alone | B's trials | readout 0, the odd places | 329 | 9 | inhibitory in readout 0 | −0.24 ± 0.02 | −0.36 ± 0.04 | −0.11 |
| After block 12 alone | B's trials | readout 0, the odd places | 849 | 9 | inhibitory in readout 0 | −0.22 ± 0.02 | −0.04 ± 0.04 | +0.18 |
| After block 12 alone | B's trials | readout 0, the odd places | 779 | 19 | inhibitory in readout 0 | −0.22 ± 0.02 | −0.10 ± 0.02 | +0.12 |
| After block 12 alone | B's trials | readout 0, the odd places | 243 | 3 | readout 0's pattern targets | −0.21 ± 0.02 | −0.05 ± 0.02 | +0.16 |
| After block 12 alone | B's trials | readout 0, the odd places | 619 | 19 | inhibitory in readout 0 | −0.18 ± 0.01 | 0.00 ± 0.02 | +0.18 |
| After block 12 alone | B's trials | readout 0, the odd places | 317 | 17 | readout 0's other excitatory | −0.17 ± 0.03 | −0.18 ± 0.03 | −0.02 |
| After block 12 alone | B's trials | readout 0, the odd places | 325 | 5 | readout 0's pattern targets | −0.16 ± 0.02 | −0.16 ± 0.02 | 0.00 |
| After block 12 alone | B's trials | readout 0, the odd places | 29 | 9 | inhibitory in readout 0 | −0.16 ± 0.02 | −0.06 ± 0.02 | +0.10 |
| After block 12 alone | B's trials | readout 0, the odd places | 853 | 13 | readout 0's pattern targets | −0.16 ± 0.02 | −0.02 ± 0.04 | +0.13 |
| After block 12 alone | B's trials | readout 0, the odd places | 809 | 9 | inhibitory in readout 0 | −0.15 ± 0.03 | −0.02 ± 0.03 | +0.13 |
| After block 12 alone | B's trials | readout 0, the odd places | 99 | 19 | inhibitory in readout 0 | −0.14 ± 0.02 | −0.01 ± 0.01 | +0.12 |
| After block 12 alone | B's trials | readout 0, the odd places | 935 | 15 | readout 0's pattern targets | −0.12 ± 0.02 | −0.07 ± 0.01 | +0.05 |
| After block 12 alone | B's trials | readout 0, the odd places | 5 | 5 | readout 0's pattern targets | −0.12 ± 0.03 | +0.01 ± 0.03 | +0.13 |
| After block 12 alone | B's trials | readout 0, the odd places | 75 | 15 | readout 0's pattern targets | −0.11 ± 0.02 | −0.06 ± 0.01 | +0.05 |
| After block 18 alone | A's trials | readout 1, the even places | 774 | 14 | inhibitory in readout 1 | −0.56 ± 0.03 | −0.53 ± 0.02 | +0.03 |
| After block 18 alone | A's trials | readout 1, the even places | 974 | 14 | inhibitory in readout 1 | −0.40 ± 0.03 | −0.14 ± 0.01 | +0.26 |
| After block 18 alone | A's trials | readout 1, the even places | 278 | 18 | readout 1's pattern targets | −0.33 ± 0.03 | −0.25 ± 0.10 | +0.08 |
| After block 18 alone | A's trials | readout 1, the even places | 648 | 8 | readout 1's pattern targets | −0.31 ± 0.02 | −0.22 ± 0.02 | +0.09 |
| After block 18 alone | A's trials | readout 1, the even places | 292 | 12 | readout 1's pattern targets | −0.29 ± 0.02 | −0.15 ± 0.02 | +0.14 |
| After block 18 alone | A's trials | readout 1, the even places | 384 | 4 | inhibitory in readout 1 | −0.29 ± 0.03 | −0.15 ± 0.03 | +0.14 |
| After block 18 alone | A's trials | readout 1, the even places | 864 | 4 | inhibitory in readout 1 | −0.28 ± 0.02 | −0.15 ± 0.02 | +0.14 |
| After block 18 alone | A's trials | readout 1, the even places | 642 | 2 | readout 1's pattern targets | −0.25 ± 0.02 | −0.04 ± 0.05 | +0.22 |
| After block 18 alone | A's trials | readout 1, the even places | 286 | 6 | readout 1's pattern targets | −0.25 ± 0.02 | −0.28 ± 0.02 | −0.02 |
| After block 18 alone | A's trials | readout 1, the even places | 830 | 10 | readout 1's other excitatory | −0.25 ± 0.03 | −0.11 ± 0.05 | +0.14 |
| After block 18 alone | A's trials | readout 1, the even places | 832 | 12 | readout 1's pattern targets | −0.24 ± 0.02 | −0.10 ± 0.04 | +0.14 |
| After block 18 alone | A's trials | readout 1, the even places | 144 | 4 | inhibitory in readout 1 | −0.23 ± 0.02 | −0.25 ± 0.02 | −0.02 |
| After block 18 alone | A's trials | readout 1, the even places | 338 | 18 | readout 1's pattern targets | −0.23 ± 0.02 | −0.15 ± 0.02 | +0.08 |
| After block 18 alone | A's trials | readout 1, the even places | 604 | 4 | inhibitory in readout 1 | −0.22 ± 0.02 | −0.15 ± 0.03 | +0.07 |
| After block 18 alone | A's trials | readout 1, the even places | 784 | 4 | inhibitory in readout 1 | −0.22 ± 0.02 | −0.12 ± 0.01 | +0.10 |
| After block 18 alone | A's trials | readout 1, the even places | 834 | 14 | inhibitory in readout 1 | −0.21 ± 0.04 | −0.08 ± 0.04 | +0.13 |
| After block 18 alone | A's trials | readout 1, the even places | 804 | 4 | inhibitory in readout 1 | −0.20 ± 0.04 | −0.19 ± 0.08 | +0.01 |
| After block 18 alone | A's trials | readout 1, the even places | 862 | 2 | readout 1's pattern targets | −0.20 ± 0.03 | −0.32 ± 0.02 | −0.12 |
| After block 18 alone | A's trials | readout 1, the even places | 438 | 18 | readout 1's pattern targets | −0.19 ± 0.01 | −0.07 ± 0.02 | +0.12 |
| After block 18 alone | A's trials | readout 0, the odd places | 819 | 19 | inhibitory in readout 0 | −0.35 ± 0.04 | −0.31 ± 0.03 | +0.05 |
| After block 18 alone | A's trials | readout 0, the odd places | 285 | 5 | readout 0's pattern targets | −0.35 ± 0.02 | −0.31 ± 0.03 | +0.04 |
| After block 18 alone | A's trials | readout 0, the odd places | 829 | 9 | inhibitory in readout 0 | −0.27 ± 0.02 | −0.12 ± 0.05 | +0.15 |
| After block 18 alone | A's trials | readout 0, the odd places | 801 | 1 | readout 0's other excitatory | −0.27 ± 0.03 | −0.29 ± 0.10 | −0.02 |
| After block 18 alone | A's trials | readout 0, the odd places | 387 | 7 | readout 0's pattern targets | −0.26 ± 0.02 | −0.03 ± 0.02 | +0.23 |
| After block 18 alone | A's trials | readout 0, the odd places | 649 | 9 | inhibitory in readout 0 | −0.25 ± 0.02 | −0.13 ± 0.02 | +0.13 |
| After block 18 alone | A's trials | readout 0, the odd places | 143 | 3 | readout 0's pattern targets | −0.22 ± 0.02 | −0.25 ± 0.02 | −0.03 |
| After block 18 alone | A's trials | readout 0, the odd places | 461 | 1 | readout 0's pattern targets | −0.21 ± 0.02 | −0.15 ± 0.01 | +0.06 |
| After block 18 alone | A's trials | readout 0, the odd places | 833 | 13 | readout 0's pattern targets | −0.20 ± 0.01 | −0.05 ± 0.03 | +0.15 |
| After block 18 alone | A's trials | readout 0, the odd places | 647 | 7 | readout 0's pattern targets | −0.20 ± 0.02 | +0.01 ± 0.02 | +0.21 |
| After block 18 alone | A's trials | readout 0, the odd places | 861 | 1 | readout 0's other excitatory | −0.20 ± 0.02 | −0.02 ± 0.00 | +0.18 |
| After block 18 alone | A's trials | readout 0, the odd places | 655 | 15 | readout 0's pattern targets | −0.18 ± 0.02 | −0.06 ± 0.01 | +0.12 |
| After block 18 alone | A's trials | readout 0, the odd places | 5 | 5 | readout 0's pattern targets | −0.16 ± 0.03 | −0.06 ± 0.01 | +0.09 |
| After block 18 alone | A's trials | readout 0, the odd places | 337 | 17 | readout 0's pattern targets | −0.16 ± 0.02 | −0.13 ± 0.02 | +0.03 |
| After block 18 alone | A's trials | readout 0, the odd places | 495 | 15 | readout 0's pattern targets | −0.16 ± 0.02 | −0.02 ± 0.01 | +0.13 |
| After block 18 alone | A's trials | readout 0, the odd places | 7 | 7 | readout 0's pattern targets | −0.16 ± 0.04 | −0.07 ± 0.02 | +0.08 |
| After block 18 alone | A's trials | readout 0, the odd places | 339 | 19 | inhibitory in readout 0 | −0.14 ± 0.02 | −0.12 ± 0.01 | +0.02 |
| After block 18 alone | B's trials | readout 1, the even places | 346 | 6 | readout 1's pattern targets | −0.33 ± 0.03 | −0.19 ± 0.02 | +0.14 |
| After block 18 alone | B's trials | readout 1, the even places | 164 | 4 | inhibitory in readout 1 | −0.27 ± 0.03 | −0.07 ± 0.02 | +0.20 |
| After block 18 alone | B's trials | readout 1, the even places | 314 | 14 | inhibitory in readout 1 | −0.26 ± 0.02 | −0.20 ± 0.03 | +0.06 |
| After block 18 alone | B's trials | readout 1, the even places | 658 | 18 | readout 1's pattern targets | −0.22 ± 0.02 | −0.08 ± 0.04 | +0.14 |
| After block 18 alone | B's trials | readout 1, the even places | 324 | 4 | inhibitory in readout 1 | −0.21 ± 0.03 | −0.06 ± 0.02 | +0.15 |
| After block 18 alone | B's trials | readout 1, the even places | 108 | 8 | readout 1's pattern targets | −0.19 ± 0.02 | −0.34 ± 0.02 | −0.15 |
| After block 18 alone | B's trials | readout 1, the even places | 16 | 16 | readout 1's other excitatory | −0.19 ± 0.01 | −0.10 ± 0.01 | +0.09 |
| After block 18 alone | B's trials | readout 1, the even places | 6 | 6 | readout 1's pattern targets | −0.17 ± 0.04 | −0.01 ± 0.01 | +0.16 |
| After block 18 alone | B's trials | readout 1, the even places | 22 | 2 | readout 1's pattern targets | −0.17 ± 0.01 | −0.21 ± 0.02 | −0.04 |
| After block 18 alone | B's trials | readout 1, the even places | 276 | 16 | D's own | −0.15 ± 0.02 | −0.07 ± 0.01 | +0.08 |
| After block 18 alone | B's trials | readout 1, the even places | 918 | 18 | readout 1's pattern targets | −0.14 ± 0.02 | −0.06 ± 0.00 | +0.08 |
| After block 18 alone | B's trials | readout 1, the even places | 56 | 16 | readout 1's other excitatory | −0.11 ± 0.02 | −0.06 ± 0.01 | +0.05 |
| After block 18 alone | B's trials | readout 1, the even places | 376 | 16 | readout 1's pattern targets | −0.11 ± 0.01 | −0.11 ± 0.02 | 0.00 |
| After block 18 alone | B's trials | readout 1, the even places | 804 | 4 | inhibitory in readout 1 | −0.11 ± 0.02 | −0.09 ± 0.01 | +0.02 |
| After block 18 alone | B's trials | readout 1, the even places | 52 | 12 | readout 1's pattern targets | −0.10 ± 0.01 | −0.14 ± 0.01 | −0.04 |
| After block 18 alone | B's trials | readout 0, the odd places | 5 | 5 | readout 0's pattern targets | −0.28 ± 0.04 | −0.16 ± 0.02 | +0.11 |
| After block 18 alone | B's trials | readout 0, the odd places | 779 | 19 | inhibitory in readout 0 | −0.22 ± 0.02 | −0.08 ± 0.03 | +0.14 |
| After block 18 alone | B's trials | readout 0, the odd places | 7 | 7 | readout 0's pattern targets | −0.21 ± 0.02 | −0.07 ± 0.01 | +0.14 |
| After block 18 alone | B's trials | readout 0, the odd places | 75 | 15 | readout 0's pattern targets | −0.21 ± 0.02 | −0.15 ± 0.01 | +0.07 |
| After block 18 alone | B's trials | readout 0, the odd places | 849 | 9 | inhibitory in readout 0 | −0.20 ± 0.03 | −0.10 ± 0.04 | +0.11 |
| After block 18 alone | B's trials | readout 0, the odd places | 453 | 13 | readout 0's pattern targets | −0.20 ± 0.04 | −0.10 ± 0.02 | +0.10 |
| After block 18 alone | B's trials | readout 0, the odd places | 179 | 19 | inhibitory in readout 0 | −0.20 ± 0.02 | −0.09 ± 0.04 | +0.11 |
| After block 18 alone | B's trials | readout 0, the odd places | 1017 | 17 | readout 0's pattern targets | −0.19 ± 0.02 | −0.14 ± 0.04 | +0.05 |
| After block 18 alone | B's trials | readout 0, the odd places | 937 | 17 | readout 0's pattern targets | −0.19 ± 0.02 | −0.12 ± 0.02 | +0.07 |
| After block 18 alone | B's trials | readout 0, the odd places | 915 | 15 | readout 0's other excitatory | −0.18 ± 0.04 | −0.19 ± 0.02 | −0.01 |
| After block 18 alone | B's trials | readout 0, the odd places | 329 | 9 | inhibitory in readout 0 | −0.17 ± 0.02 | −0.18 ± 0.02 | −0.01 |
| After block 18 alone | B's trials | readout 0, the odd places | 659 | 19 | inhibitory in readout 0 | −0.17 ± 0.02 | −0.11 ± 0.01 | +0.06 |
| After block 18 alone | B's trials | readout 0, the odd places | 15 | 15 | readout 0's pattern targets | −0.17 ± 0.01 | −0.08 ± 0.03 | +0.09 |
| After block 18 alone | B's trials | readout 0, the odd places | 913 | 13 | readout 0's pattern targets | −0.16 ± 0.02 | −0.07 ± 0.01 | +0.10 |
| After block 18 alone | B's trials | readout 0, the odd places | 853 | 13 | readout 0's pattern targets | −0.16 ± 0.03 | −0.06 ± 0.04 | +0.10 |
| After block 18 alone | B's trials | readout 0, the odd places | 49 | 9 | inhibitory in readout 0 | −0.16 ± 0.01 | −0.27 ± 0.04 | −0.11 |
| After block 18 alone | B's trials | readout 0, the odd places | 801 | 1 | readout 0's other excitatory | −0.14 ± 0.04 | −0.02 ± 0.01 | +0.13 |
| After block 18 alone | B's trials | readout 0, the odd places | 101 | 1 | readout 0's other excitatory | −0.14 ± 0.02 | −0.26 ± 0.02 | −0.12 |
| After block 24 alone | A's trials | readout 1, the even places | 774 | 14 | inhibitory in readout 1 | −0.42 ± 0.04 | −0.33 ± 0.02 | +0.09 |
| After block 24 alone | A's trials | readout 1, the even places | 974 | 14 | inhibitory in readout 1 | −0.40 ± 0.03 | −0.35 ± 0.03 | +0.05 |
| After block 24 alone | A's trials | readout 1, the even places | 338 | 18 | readout 1's pattern targets | −0.36 ± 0.03 | −0.27 ± 0.01 | +0.10 |
| After block 24 alone | A's trials | readout 1, the even places | 286 | 6 | readout 1's pattern targets | −0.35 ± 0.03 | −0.26 ± 0.02 | +0.09 |
| After block 24 alone | A's trials | readout 1, the even places | 966 | 6 | readout 1's pattern targets | −0.28 ± 0.02 | −0.32 ± 0.04 | −0.03 |
| After block 24 alone | A's trials | readout 1, the even places | 784 | 4 | inhibitory in readout 1 | −0.26 ± 0.02 | −0.18 ± 0.02 | +0.08 |
| After block 24 alone | A's trials | readout 1, the even places | 226 | 6 | readout 1's pattern targets | −0.25 ± 0.02 | −0.18 ± 0.02 | +0.07 |
| After block 24 alone | A's trials | readout 1, the even places | 834 | 14 | inhibitory in readout 1 | −0.25 ± 0.02 | −0.07 ± 0.05 | +0.18 |
| After block 24 alone | A's trials | readout 1, the even places | 604 | 4 | inhibitory in readout 1 | −0.25 ± 0.04 | −0.11 ± 0.02 | +0.14 |
| After block 24 alone | A's trials | readout 1, the even places | 448 | 8 | readout 1's pattern targets | −0.24 ± 0.05 | −0.04 ± 0.02 | +0.20 |
| After block 24 alone | A's trials | readout 1, the even places | 438 | 18 | readout 1's pattern targets | −0.23 ± 0.02 | −0.09 ± 0.02 | +0.14 |
| After block 24 alone | A's trials | readout 1, the even places | 826 | 6 | readout 1's pattern targets | −0.22 ± 0.03 | −0.03 ± 0.07 | +0.19 |
| After block 24 alone | A's trials | readout 1, the even places | 648 | 8 | readout 1's pattern targets | −0.22 ± 0.02 | −0.16 ± 0.01 | +0.05 |
| After block 24 alone | A's trials | readout 1, the even places | 678 | 18 | readout 1's pattern targets | −0.22 ± 0.04 | −0.16 ± 0.07 | +0.06 |
| After block 24 alone | A's trials | readout 1, the even places | 804 | 4 | inhibitory in readout 1 | −0.21 ± 0.03 | −0.19 ± 0.05 | +0.02 |
| After block 24 alone | A's trials | readout 1, the even places | 342 | 2 | readout 1's pattern targets | −0.21 ± 0.02 | −0.16 ± 0.03 | +0.05 |
| After block 24 alone | A's trials | readout 1, the even places | 144 | 4 | inhibitory in readout 1 | −0.19 ± 0.02 | −0.12 ± 0.01 | +0.07 |
| After block 24 alone | A's trials | readout 1, the even places | 756 | 16 | D's own | −0.19 ± 0.02 | −0.12 ± 0.01 | +0.07 |
| After block 24 alone | A's trials | readout 1, the even places | 794 | 14 | inhibitory in readout 1 | −0.19 ± 0.03 | −0.11 ± 0.03 | +0.07 |
| After block 24 alone | A's trials | readout 0, the odd places | 819 | 19 | inhibitory in readout 0 | −0.40 ± 0.04 | −0.32 ± 0.03 | +0.08 |
| After block 24 alone | A's trials | readout 0, the odd places | 343 | 3 | readout 0's pattern targets | −0.27 ± 0.02 | −0.19 ± 0.01 | +0.08 |
| After block 24 alone | A's trials | readout 0, the odd places | 801 | 1 | readout 0's other excitatory | −0.25 ± 0.04 | −0.27 ± 0.04 | −0.03 |
| After block 24 alone | A's trials | readout 0, the odd places | 833 | 13 | readout 0's pattern targets | −0.23 ± 0.02 | −0.02 ± 0.05 | +0.21 |
| After block 24 alone | A's trials | readout 0, the odd places | 559 | 19 | inhibitory in readout 0 | −0.19 ± 0.02 | −0.07 ± 0.02 | +0.13 |
| After block 24 alone | A's trials | readout 0, the odd places | 565 | 5 | readout 0's pattern targets | −0.18 ± 0.01 | −0.11 ± 0.01 | +0.07 |
| After block 24 alone | A's trials | readout 0, the odd places | 143 | 3 | readout 0's pattern targets | −0.18 ± 0.02 | −0.12 ± 0.02 | +0.06 |
| After block 24 alone | A's trials | readout 0, the odd places | 655 | 15 | readout 0's pattern targets | −0.16 ± 0.05 | −0.16 ± 0.04 | +0.01 |
| After block 24 alone | A's trials | readout 0, the odd places | 337 | 17 | readout 0's pattern targets | −0.16 ± 0.02 | −0.09 ± 0.00 | +0.07 |
| After block 24 alone | A's trials | readout 0, the odd places | 461 | 1 | readout 0's pattern targets | −0.16 ± 0.02 | −0.11 ± 0.02 | +0.04 |
| After block 24 alone | A's trials | readout 0, the odd places | 339 | 19 | inhibitory in readout 0 | −0.15 ± 0.02 | −0.09 ± 0.01 | +0.06 |
| After block 24 alone | A's trials | readout 0, the odd places | 649 | 9 | inhibitory in readout 0 | −0.14 ± 0.02 | −0.09 ± 0.01 | +0.05 |
| After block 24 alone | A's trials | readout 0, the odd places | 227 | 7 | readout 0's pattern targets | −0.13 ± 0.02 | −0.09 ± 0.01 | +0.04 |
| After block 24 alone | A's trials | readout 0, the odd places | 685 | 5 | readout 0's pattern targets | −0.13 ± 0.03 | −0.01 ± 0.06 | +0.12 |
| After block 24 alone | A's trials | readout 0, the odd places | 829 | 9 | inhibitory in readout 0 | −0.13 ± 0.02 | −0.03 ± 0.02 | +0.10 |
| After block 24 alone | A's trials | readout 0, the odd places | 285 | 5 | readout 0's pattern targets | −0.13 ± 0.02 | −0.11 ± 0.01 | +0.02 |
| After block 24 alone | A's trials | readout 0, the odd places | 835 | 15 | readout 0's pattern targets | −0.10 ± 0.02 | −0.07 ± 0.02 | +0.03 |
| After block 24 alone | B's trials | readout 1, the even places | 346 | 6 | readout 1's pattern targets | −0.37 ± 0.02 | −0.34 ± 0.03 | +0.02 |
| After block 24 alone | B's trials | readout 1, the even places | 108 | 8 | readout 1's pattern targets | −0.35 ± 0.02 | −0.30 ± 0.02 | +0.05 |
| After block 24 alone | B's trials | readout 1, the even places | 22 | 2 | readout 1's pattern targets | −0.34 ± 0.03 | −0.27 ± 0.03 | +0.07 |
| After block 24 alone | B's trials | readout 1, the even places | 276 | 16 | D's own | −0.31 ± 0.02 | −0.25 ± 0.01 | +0.06 |
| After block 24 alone | B's trials | readout 1, the even places | 314 | 14 | inhibitory in readout 1 | −0.28 ± 0.04 | −0.23 ± 0.03 | +0.05 |
| After block 24 alone | B's trials | readout 1, the even places | 16 | 16 | readout 1's other excitatory | −0.27 ± 0.03 | −0.23 ± 0.01 | +0.04 |
| After block 24 alone | B's trials | readout 1, the even places | 658 | 18 | readout 1's pattern targets | −0.22 ± 0.02 | −0.16 ± 0.06 | +0.06 |
| After block 24 alone | B's trials | readout 1, the even places | 270 | 10 | readout 1's other excitatory | −0.19 ± 0.02 | −0.14 ± 0.02 | +0.06 |
| After block 24 alone | B's trials | readout 1, the even places | 52 | 12 | readout 1's pattern targets | −0.19 ± 0.01 | −0.13 ± 0.01 | +0.06 |
| After block 24 alone | B's trials | readout 1, the even places | 164 | 4 | inhibitory in readout 1 | −0.19 ± 0.03 | −0.10 ± 0.06 | +0.09 |
| After block 24 alone | B's trials | readout 1, the even places | 68 | 8 | readout 1's other excitatory | −0.15 ± 0.02 | −0.09 ± 0.00 | +0.06 |
| After block 24 alone | B's trials | readout 1, the even places | 326 | 6 | readout 1's pattern targets | −0.13 ± 0.01 | −0.09 ± 0.03 | +0.04 |
| After block 24 alone | B's trials | readout 1, the even places | 324 | 4 | inhibitory in readout 1 | −0.13 ± 0.01 | −0.12 ± 0.02 | +0.01 |
| After block 24 alone | B's trials | readout 0, the odd places | 5 | 5 | readout 0's pattern targets | −0.44 ± 0.02 | −0.38 ± 0.02 | +0.06 |
| After block 24 alone | B's trials | readout 0, the odd places | 915 | 15 | readout 0's other excitatory | −0.35 ± 0.02 | −0.30 ± 0.02 | +0.05 |
| After block 24 alone | B's trials | readout 0, the odd places | 329 | 9 | inhibitory in readout 0 | −0.29 ± 0.02 | −0.26 ± 0.01 | +0.02 |
| After block 24 alone | B's trials | readout 0, the odd places | 15 | 15 | readout 0's pattern targets | −0.28 ± 0.02 | −0.21 ± 0.02 | +0.07 |
| After block 24 alone | B's trials | readout 0, the odd places | 7 | 7 | readout 0's pattern targets | −0.28 ± 0.02 | −0.24 ± 0.04 | +0.04 |
| After block 24 alone | B's trials | readout 0, the odd places | 101 | 1 | readout 0's other excitatory | −0.27 ± 0.03 | −0.24 ± 0.02 | +0.03 |
| After block 24 alone | B's trials | readout 0, the odd places | 453 | 13 | readout 0's pattern targets | −0.23 ± 0.02 | −0.13 ± 0.02 | +0.10 |
| After block 24 alone | B's trials | readout 0, the odd places | 653 | 13 | readout 0's pattern targets | −0.22 ± 0.02 | −0.17 ± 0.05 | +0.06 |
| After block 24 alone | B's trials | readout 0, the odd places | 179 | 19 | inhibitory in readout 0 | −0.22 ± 0.03 | −0.14 ± 0.04 | +0.09 |
| After block 24 alone | B's trials | readout 0, the odd places | 937 | 17 | readout 0's pattern targets | −0.21 ± 0.02 | −0.12 ± 0.02 | +0.09 |
| After block 24 alone | B's trials | readout 0, the odd places | 659 | 19 | inhibitory in readout 0 | −0.20 ± 0.02 | −0.18 ± 0.03 | +0.02 |
| After block 24 alone | B's trials | readout 0, the odd places | 75 | 15 | readout 0's pattern targets | −0.18 ± 0.02 | −0.13 ± 0.01 | +0.06 |
| After block 24 alone | B's trials | readout 0, the odd places | 1017 | 17 | readout 0's pattern targets | −0.16 ± 0.02 | −0.08 ± 0.04 | +0.08 |
| After block 24 alone | B's trials | readout 0, the odd places | 325 | 5 | readout 0's pattern targets | −0.16 ± 0.01 | −0.11 ± 0.02 | +0.05 |

On the unrounded values, 232 of these 265 rows lie nearer the night without replay at block 36 than right after and 33 further from it, and
seven rows read alike at two decimals; 24 changed sign, and 196 still lie below beyond 2 SE.

## 5. Beside each: the class matrix over the replayed night

The class matrix by group and the margins over each replayed night on this cell, as explorations 031 and 032 read them, from their logs:
replay after block 6 alone from exploration 031's job, after block 12, 18 or 24 alone from exploration 032's, and H-47's two forms from
exploration 020's job. A night's change is the image after it less the image before it, the mean over the three paths, in millions (M); an
inhibitory entry is a magnitude, so a + is more inhibition. **The odd places' excitatory units** are classes 4, 6 and 7, C's own and readout
0's two classes, and **the even places'** classes 5, 8 and 9; the inhibitory units are class 11 in the odd places, 12 in the even places and
13 elsewhere. A kind's margin onto A's answer is its weight onto the readout A asks of its stimulus less onto the other, as exploration 020
reads it; A's margin is the sum of four kinds', A alone, A and C, B alone, and B and D.

| Night after block | Form | Its job's log | This night | The odd places' excitatory onto all, M | the even places' | The odd places' onto their own | the even places' onto their own | The odd places' inhibitory onto all | the even places' | Inhibitory onto the odd places' excitatory | onto the even places' | C's own onto A's answer, M | D's own | A's margin |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | After block 6 alone | exploration 031's | replays | −4.33 | −4.90 | −0.79 | −1.27 | +4.42 | +4.38 | +3.72 | +3.79 | +0.07 | −0.02 | +1.26 |
| 6 | Plain on all four nights (H-47) | exploration 020's | replays | −4.33 | −4.90 | −0.79 | −1.27 | +4.42 | +4.38 | +3.72 | +3.79 | +0.07 | −0.02 | +1.26 |
| 6 | Without replay | exploration 020's | without replay | −0.07 | −0.32 | −0.21 | −0.39 | +0.14 | +0.17 | +0.26 | +0.21 | +0.01 | −0.05 | −0.05 |
| 12 | After block 12 alone | exploration 032's | replays | −4.06 | −4.53 | −0.66 | −1.16 | +4.60 | +4.52 | +3.82 | +3.84 | +0.04 | −0.04 | +1.16 |
| 12 | Plain on all four nights (H-47) | exploration 020's | replays | −3.06 | −3.55 | −0.50 | −0.80 | +3.63 | +3.54 | +3.07 | +3.10 | +0.07 | −0.06 | +0.95 |
| 12 | Without replay | exploration 020's | without replay | +0.16 | −0.01 | −0.17 | −0.19 | +0.12 | +0.13 | +0.22 | +0.18 | −0.02 | −0.08 | −0.01 |
| 18 | After block 18 alone | exploration 032's | replays | −3.75 | −4.18 | −0.64 | −1.07 | +4.30 | +4.50 | +3.55 | +3.70 | −0.01 | +0.02 | +1.05 |
| 18 | Plain on all four nights (H-47) | exploration 020's | replays | −2.16 | −2.24 | −0.24 | −0.39 | +2.91 | +3.02 | +2.47 | +2.62 | −0.04 | −0.02 | +0.56 |
| 18 | Without replay | exploration 020's | without replay | +0.29 | +0.17 | −0.12 | −0.09 | +0.12 | +0.20 | +0.24 | +0.17 | −0.02 | −0.05 | −0.08 |
| 24 | After block 24 alone | exploration 032's | replays | −3.49 | −4.13 | −0.41 | −0.94 | +4.56 | +4.42 | +3.61 | +3.69 | +0.03 | −0.05 | +1.05 |
| 24 | Plain on all four nights (H-47) | exploration 020's | replays | −1.69 | −1.64 | −0.25 | −0.30 | +2.58 | +2.58 | +2.12 | +2.24 | −0.01 | −0.05 | +0.58 |
| 24 | Without replay | exploration 020's | without replay | +0.31 | +0.05 | −0.03 | −0.03 | +0.23 | +0.21 | +0.27 | +0.22 | +0.03 | −0.06 | −0.08 |

Each single replayed night moved the class matrix by group about as H-47's first plain night did on this cell: over the nights after block
12, 18 and 24 alone the odd places' excitatory units lost 3.49 to 4.06 M onto all and the even places' 4.13 to 4.53 M, and the inhibition
onto the odd places' and the even places' excitatory units rose by 3.55 to 3.82 M and 3.69 to 3.84 M, where over H-47's first plain night,
after block 6 alone's too, they lost 4.33 and 4.90 M and the inhibition rose by 3.72 and 3.79 M. H-47's later plain nights moved less, 1.64
to 3.55 M lost onto all and 2.12 to 3.10 M more inhibition onto the excitatory units, and its nights without replay 0.39 M or less in each
group. A's margin rose by 1.05 to 1.26 M over each single replayed night.

## What it costs

The trial took 304 s and the two jobs 5 850 and 7 429 s, 97.5 and 123.8 minutes, at most 51.6 % of a job's 240 minutes; ADR-0248 put them at
about 110 and 131 minutes, not counting the unit reads' dumps. Nothing was cut: each job ran its forms on the three paths, every night
watched, and every read of A counted unit by unit. The two logs are 6.6 and 7.8 MB.

## What it suggests

**On this cell each replayed night lowered both readouts' spikes on both stimuli's trials at once**, by 18 to 33 per cent right after a
single night, every class of unit in both readouts with them, and 13 to 19 units carried half of each drop, about as many as carry half of a
readout's spikes. Whether a stimulus's correct trials rose or fell went with which readout fell more on its trials: on A's trials readout 0
fell more after block 6 alone and readout 1 more after each later single night. By block 36 both readouts had come back by 50 to 77 per cent
of their drop under each single night, and by 21 to 32 per cent of theirs at block 24 under H-47's plain night on all four nights.

Questions this suggests for a later exploration or a round:
- What a replayed night changes in the inputs of the units that carry each readout's drop, which this note does not read: the weights onto
  them, by class of the presynaptic unit.
- Whether the same few units carry the drop on other cells, seed 24's eighth image from the mirrored assignment among them, where after
  block 18 and 24 alone lay above the night without replay right after their nights.
- Why, on this cell, readout 1 loses more than readout 0 on A's trials after a later single night and less after the first: what differs in
  the image each replayed night starts from.
- The units outside the readouts' sets, the stimulus units and the rest, whose spikes drive the readouts.

## What did not run

- **Any other cell**, seed 24's eighth image from the mirrored assignment among them, where after block 18 and after block 24 alone lay
  above the night without replay right after their nights; and seed 23's eighth image from the assignment, where after block 12 alone also
  lay below it right after its night, −1.9 ± 0.88.
- **Replay on two of the four nights, or on every night but one.**
- **Any hold**, on any night.
- **Plain nights on both mappings and the awake gap.**
- **The pair's reads unit by unit**: they are read as before, their counts and masks set beside the logs'.
- **The units outside the readouts' sets**: the stimulus units, the other excitatory units and the inhibitory units elsewhere.
- **Any path but H-47's three**, 0, 5 and 6.
- **A night split apart**, such as slow-wave sleep read apart from REM: a night's change is read whole, from the images before and after
  it.
- **The next day's reading of the pairings a night leaves** pending.
- **What a unit receives**: a unit's spikes are counted, not its inputs, its membrane or the weights onto it, so this note does not say why
  a unit fired less.
- **Any SE for section 5**: the class matrix and the margins are means over the three paths, and sections 2 to 4's correlations and counts
  of units carry none.
- **A unit-by-unit table of all 918 units**: the tables name the fewest units and count the rest; every unit's sums are in the jobs' logs.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_one_cells_readouts_unit_by_unit_explore` | [43010140120](https://github.com/Menteion/Menteion/actions/runs/43010140120) | `397b900` | 304 |
| `one_cells_readouts_after_block_6_alone_and_on_every_night_explore` | [43011654795](https://github.com/Menteion/Menteion/actions/runs/43011654795) | `397b900` | 5850 |
| `one_cells_readouts_after_each_later_night_alone_explore` | [43011654795](https://github.com/Menteion/Menteion/actions/runs/43011654795) | `397b900` | 7429 |
