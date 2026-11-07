---
status: exploratory
date: 2026-11-08
---

# 024: The readouts' inhibition held through a night

**Branch**: `explore/024-the-readouts-inhibition-held`
**Commit**: `ca36ddc`, which adds the harness, on `7cf2c71`, which adds the night hold under `src/`, on `73576e4`: `main` at `1b93eb3`,
ADR-0239's merge, with exploration 023's tag merged in.
**Runs**: [42405480901](https://github.com/Menteion/Menteion/actions/runs/42405480901) (the harness's trial) and
[42406369084](https://github.com/Menteion/Menteion/actions/runs/42406369084) (four jobs, one a cell), both at `scope=arms`. H-47's plain
night on A's patterns is read from H-47's run [41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613), and its split and
class matrix from exploration 020's runs [41500459562](https://github.com/Menteion/Menteion/actions/runs/41500459562) and
[41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777). None of those was run again.

[ADR-0239](../adr/0239-the-readouts-inhibition-held-through-a-night.md) ordered this exploration. In H-47's layout, plain nights on A's
patterns add more inhibition onto the excitatory units of the readout stimulus A answers than onto the other's, in all twelve cells
([exploration 021](021-where-the-gain-falls.md)). This note runs H-47's plain night on A's patterns with every inhibitory synapse onto an
excitatory unit of either readout held where it stood, from the night's first tick to its last, on
[exploration 020](020-what-a-replay-costs.md)'s four cells. It asks, at blocks 6, 12, 18, 24 and 36:

1. A's count and each stimulus's correct trials under the held night: held less without replay, with H-47's paired SE, the two forms run
   in one job; and held less H-47's plain night, with [exploration 018](018-what-nights-keep.md)'s unpaired SE, a form run in another job;
2. the side under the held night, by stimulus and by group of units, against H-47's side on the same cell;
3. the pair under the held night and A's highest coupling, as H-47's clauses 2 and 3 read them;
4. what each night moved, from the class matrix by group, the held synapses' change shown nil;
5. what the hold leaves: the pairings of the night still read at a held synapse's first presynaptic spike after it.

**The side** is [ADR-0238](../adr/0238-what-keeps-a-mapping-where-the-replay-did-not.md)'s: at a checkpoint, the stimulus whose difference
from the night without replay is the larger of the two. Here it is the held night's difference, against H-47's, which is its plain night's.
The note writes no prediction and decides nothing. **Every figure here is exploratory**: from one run of each test on the repository's
AArch64 host, in H-47's terms. A count is a mean over a cell's 48 reads, sixteen on each of three paths, and a difference carries its paired
SE unless another is named.

**In brief**
- **The hold held, and the night without replay ran H-47's.** In all 96 held nights every held synapse ended the night at the weight it
  started with and its trace cleared, held inside the test, while 1 369 to 1 623 inhibitory synapses onto other units moved. Every row of
  the night without replay, 15 a cell, is H-47's, held inside the test.
- **Under the held night A's count lies within 2 SE of H-47's plain night's at every checkpoint in every cell**: held less plain lies
  between −1.2 and +0.6, at most 1.04 times its unpaired SE. Held less without replay at block 36 is −4.8 ± 1.45, +1.1 ± 1.20, +5.0 ±
  1.56 and +1.5 ± 0.92 in seed 23's, seed 24's and seed 22's two cells, where H-47's plain night gave −4.9 ± 1.09, +1.8 ± 1.79, +5.0 ±
  1.24 and +2.2 ± 0.65.
- **The side under the held night is H-47's at all 20 checkpoints**, by stimulus and by group: from block 18 it lies on B, the odd places,
  in all four cells, as in H-47. Each stimulus's trials under the held night lie within 1.1 trials and 1.01 SE of H-47's plain night's.
- **Each held night ran H-47's plain night's replays and depotentiations, night by night**: 1 129 to 1 340 replays and 545 to 640
  depotentiations.
- **In the held nights themselves the inhibition onto each group's excitatory units moved by nothing**, where H-47's plain night added
  6.52 to 11.76 M onto each group over four nights, image to image. Under the held night the quiet ticks after the hold moved it by −45 LSB
  onto the odd places' and −207 onto the even places', a path's mean over four nights, on seed 24's eighth image alone; under the held night
  without replay, on three cells, by −260 to +402 LSB. The held night's inhibition onto every other target is the plain night's within
  0.08 M, and it took 9.35 to 13.21 M from each group's excitatory units' synapses, where the plain night took 9.20 to 12.33 M.
- **The pairings the hold leaves**: after the quiet ticks of each held night, 1 886 to 2 285 of the 4 863 to 4 904 held synapses carry a
  pairing of the night's spikes that their presynaptic unit's next spike would read, if it came before the target's next spike: 57 906 to
  113 394 LSB of trace together, 10.1 % to 17.7 % the size of the positive trace the hold cleared. A presynaptic spike at the first tick
  after the image would add at most 9 591 to 17 428 LSB more.
- **The pair under the held night** stands at 62.2 to 63.2 at block 36, and A's highest coupling at 1.156 to 1.192 of H-42's.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 020 to 023 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. A cell is named, as in H-47, by its seed, its image and the arm
it starts from. Nights follow blocks 6, 12, 18 and 24, then 12 blocks run with none to block 36, on paths 0, 5 and 6. Each checkpoint reads
A and the pair sixteen times each, after the night that follows its block, and at block 36 after the block.

**Places.** A period of the ring is 20 units. Stimulus A presents place 0 of each period and B place 11. **The odd places** are the odd
places but 11 and **the even places** the even places but 0: H-47's readout 0 and readout 1, each with its excitatory and its inhibitory
units. C's own units lie at place 5, among the odd places, and D's at place 16, among the even. As
[exploration 023](023-the-readouts-index-or-its-group.md) read, the readout's index is a name here: readout 0 is the odd places and readout
1 the even.

**Four cells**, exploration 020's: H-47's two that failed clause 1, seed 23's first mapping from the mirrored assignment and seed 24's eighth
image from the assignment, and seed 22's two from the same starts. In all four the mapping in force is the mirrored one, so stimulus A answers
readout 1, the even places, and B readout 0, the odd places.

**The night hold**, on the branch alone (`7cf2c71`). `Executor::set_night_hold` takes a set of units. While it is set and the engine is in
slow-wave sleep or REM, the fan-out skips the consolidation of every slot of an inhibitory block whose target is one of them, so the weight
stays where it stood. A consolidation at modulation zero still brings a weight on the wrong side of zero to its side, so the hold skips it
rather than giving it modulation zero. **The night's last tick** is the tick whose window steps the engine from sleep to awake; at its end
every held slot's trace is cleared to zero. Every other synapse consolidates as in H-47. Unset, the fan-out takes its old path. The hold is
in no image, so the next day's engine, decoded from the night's image, holds nothing.

**The held units** are every excitatory unit of the two readouts (`e024_held`): 714, 357 a readout, among them C's own 25 and D's own 25.
None of A's or B's stimulus units is among them, held inside the test. The readouts' 204 inhibitory units are not held targets, so the
inhibitory synapses onto them consolidate, and so does every excitatory synapse, the readouts' own among them. That is the part of
ADR-0238's night that ADR-0239 takes. Of each cell's 6 528 inhibitory synapses the hold holds 4 881 on seed 23, 4 904 on seed 24 and 4 863
on seed 22.

**Three forms a cell, in one job** (`e024_job`):
- **the night without replay**, every row held inside the test to the row H-47 pinned;
- **the held night**, H-47's plain night on A's patterns with the hold set on the held units;
- **the held night without replay**, the night without replay with the hold set, to read the hold apart from the replay.

The job is exploration 020's: A's patterns, the split and the class matrix of the start image, of the image before each night and of the
image after it, and every night watched by `ReplayWatch`. H-47's plain night on A's patterns is read from H-47's logs, its split and class
matrix from exploration 020's. The held night is not H-47's plain night bit for bit: its rows differ from H-47's (below), so it had to run.

**At each held night** (`e024_night`) the job reads every inhibitory synapse three times: before the night's first tick, at its last, and after
the quiet ticks, the awake ticks without drive that H-47's harness runs until the engine is quiescent, before it writes the night's image.
At the last it holds, inside the test, that every held synapse stands at its first weight with its trace cleared and that some inhibitory
synapse onto another unit moved. After the quiet ticks it prints what they moved, and what each held synapse would read at its
presynaptic unit's next spike from the spikes on record since the night's first tick, if that spike came before its target's next one.
The inhibitory rule reads its pairings only at a presynaptic spike (`step_stdp_shaped`, in `crates/menteion-core/src/dynamics/synapse.rs`),
so those pairings escape the clearing. With $p$ the presynaptic unit's last spike, $q$ the target's last spike and $t_0$ the first tick
after the image, they are two:
- the target's last spike after the presynaptic unit's last, if $q > p$: $A_+ (1 - 2^{-11})^{q - p}$, its count, sum and largest printed;
- the target's last spike with the next presynaptic spike: $A_+ (1 - 2^{-11})^{t - q}$ for a spike at $t$, at most its value at $t = t_0$,
  printed as a sum over every held synapse whose target fired since the night's first tick.

A target spike before the next presynaptic one takes $q$'s place, and both readings change with it.

**The class matrix** is exploration 020's, as 021 reads it by readout and 023 by group. Each group's excitatory units are its readout's two
classes and the pair's own kind inside it, C's own in the odd places and D's in the even. An inhibitory entry is a magnitude, the weight
negated, so a rise is more inhibition. The held synapses are the three inhibitory classes' entries onto the six excitatory classes of the
two groups.

## The runs are laid out as written

- **The trial** ran seed 23's first mapping from the mirrored assignment on path 0: the pair's head and the first night of four forms, all
  watched. With the hold unset, the night without replay and the plain night on A's patterns held H-47's row at block 6 on path 0, inside the
  test. With it set, each held night held every held synapse at its first weight with its trace cleared, inside the test. In the held night
  1 587 inhibitory synapses onto other units moved, and in the held night without replay 1 597. The held night's row at block 6 on path 0 is
  not H-47's plain night's: its image's CRC-64, A's couplings and its gain differ, and A's count at its first read, 57, is the same. The
  held night without replay's row is not H-47's night without replay's either, as the trial prints. The watch read `wrong 0` in its four
  nights, and the plain and the held night each 1 339 replays and 640 depotentiations.
- **Each of the four jobs** ran to the end. Their reproduction, cell by cell:

| Cell | Without replay: rows H-47's | Held: rows H-47's plain on A's | Held without replay: rows H-47's without replay | Without replay: its paths' lines in 020's job, label aside | Heads: lines in 020's job | Held nights held inside the test |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 15 of 15 | 0 of 15 | 0 of 15 | 1 680 of 1 680 | 3 of 3 | 24 of 24 |
| Seed 24, eighth, assignment | 15 of 15 | 0 of 15 | 0 of 15 | 1 656 of 1 656 | 3 of 3 | 24 of 24 |
| Seed 22, first mapping, mirrored | 15 of 15 | 0 of 15 | 0 of 15 | 1 632 of 1 632 | 3 of 3 | 24 of 24 |
| Seed 22, eighth, assignment | 15 of 15 | 0 of 15 | 0 of 15 | 1 632 of 1 632 | 3 of 3 | 24 of 24 |

- The night without replay's `DUMP` lines, its paths' 1 632 to 1 680 a cell, its 15 rows' and its notice, are exploration 020's job's
  lines of the same cell, label aside, all of them in each cell; so are the pair's head's three, its six blocks before the first night on
  each path. Each read of the night without replay carries H-47's read's
  `stimuli`, `right` and `tied` masks, all 480 a cell. The held night's reads carry H-47's plain night's `stimuli` masks and the held night
  without replay's H-47's night without replay's, so each trial presented the stimulus it presented in H-47.
- For each held form the job prints how many of its 15 rows are H-47's of the matching form, the plain night on A's patterns or the night
  without replay: **none, in all four cells**, as the job prints; nothing in the job asserts it.
- **The watch read `wrong 0` in all 144 nights.** Every night without replay, held or not, read `replays 0` and `depotentiations 0`.

## 1. A's count under the held night

A's count at each checkpoint under each form, and the differences. "Held less without replay" and "H-47: plain less without replay" carry
the paired SE, the two forms of each run in one job; "Held less plain" carries exploration 018's unpaired SE. The night without replay is
H-47's, read by read, so "H-47: plain less without replay" at block 36 is H-47's clause 1:

| Cell | Block | Without replay | Held | Held without replay | Plain on A's (H-47) | Held less without replay | H-47: plain less without replay | Held less plain (unpaired) | Held without replay less without replay | Held less held without replay |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | 55.0 | 56.8 | 55.1 | 56.9 | +1.8 ± 0.71 | +1.9 ± 0.68 | −0.1 ± 0.69 | +0.1 ± 0.12 | +1.7 ± 0.72 |
| Seed 23, first mapping, mirrored | 12 | 52.4 | 54.5 | 52.5 | 54.4 | +2.2 ± 1.15 | +2.0 ± 1.26 | +0.1 ± 1.11 | +0.2 ± 0.14 | +2.0 ± 1.16 |
| Seed 23, first mapping, mirrored | 18 | 52.0 | 51.9 | 52.8 | 52.4 | −0.1 ± 1.08 | +0.3 ± 1.17 | −0.4 ± 0.82 | +0.8 ± 0.45 | −0.9 ± 1.34 |
| Seed 23, first mapping, mirrored | 24 | 53.5 | 50.4 | 53.8 | 50.9 | −3.1 ± 1.57 | −2.6 ± 1.30 | −0.5 ± 1.20 | +0.3 ± 0.43 | −3.4 ± 1.83 |
| Seed 23, first mapping, mirrored | 36 | 53.1 | 48.3 | 53.5 | 48.3 | −4.8 ± 1.45 | −4.9 ± 1.09 | +0.1 ± 0.71 | +0.4 ± 0.54 | −5.2 ± 1.30 |
| Seed 24, eighth, assignment | 6 | 38.8 | 38.5 | 38.7 | 39.1 | −0.3 ± 0.81 | +0.3 ± 0.64 | −0.5 ± 0.91 | −0.1 ± 0.20 | −0.1 ± 0.86 |
| Seed 24, eighth, assignment | 12 | 34.8 | 33.0 | 35.9 | 34.2 | −1.8 ± 1.12 | −0.6 ± 1.03 | −1.2 ± 1.17 | +1.0 ± 0.08 | −2.9 ± 1.15 |
| Seed 24, eighth, assignment | 18 | 34.4 | 29.0 | 34.3 | 28.5 | −5.4 ± 2.42 | −5.9 ± 1.53 | +0.5 ± 1.66 | −0.1 ± 0.31 | −5.3 ± 2.67 |
| Seed 24, eighth, assignment | 24 | 35.6 | 28.6 | 36.5 | 29.6 | −7.0 ± 1.62 | −5.9 ± 1.85 | −1.1 ± 2.31 | +0.9 ± 0.44 | −7.9 ± 1.82 |
| Seed 24, eighth, assignment | 36 | 25.6 | 26.7 | 25.7 | 27.4 | +1.1 ± 1.20 | +1.8 ± 1.79 | −0.7 ± 1.69 | +0.1 ± 1.07 | +1.0 ± 0.86 |
| Seed 22, first mapping, mirrored | 6 | 56.6 | 57.8 | 56.7 | 57.7 | +1.2 ± 0.69 | +1.1 ± 0.63 | +0.1 ± 0.57 | 0.0 ± 0.12 | +1.2 ± 0.76 |
| Seed 22, first mapping, mirrored | 12 | 49.5 | 52.4 | 49.5 | 52.5 | +2.9 ± 1.69 | +2.9 ± 1.93 | −0.1 ± 0.93 | 0.0 ± 0.22 | +2.9 ± 1.52 |
| Seed 22, first mapping, mirrored | 18 | 49.0 | 52.8 | 48.7 | 52.7 | +3.8 ± 2.27 | +3.7 ± 2.22 | +0.1 ± 1.07 | −0.3 ± 0.47 | +4.1 ± 1.99 |
| Seed 22, first mapping, mirrored | 24 | 45.9 | 51.1 | 45.7 | 50.6 | +5.2 ± 2.42 | +4.7 ± 2.24 | +0.5 ± 1.23 | −0.2 ± 0.59 | +5.4 ± 2.08 |
| Seed 22, first mapping, mirrored | 36 | 44.1 | 49.2 | 44.3 | 49.1 | +5.0 ± 1.56 | +5.0 ± 1.24 | 0.0 ± 0.72 | +0.2 ± 0.75 | +4.8 ± 1.24 |
| Seed 22, eighth, assignment | 6 | 50.3 | 52.0 | 50.3 | 52.3 | +1.7 ± 0.57 | +1.9 ± 0.65 | −0.3 ± 0.99 | 0.0 ± 0.04 | +1.7 ± 0.48 |
| Seed 22, eighth, assignment | 12 | 47.6 | 51.2 | 47.9 | 50.6 | +3.6 ± 0.93 | +3.0 ± 0.93 | +0.6 ± 0.94 | +0.2 ± 0.16 | +3.4 ± 0.85 |
| Seed 22, eighth, assignment | 18 | 47.2 | 50.4 | 47.4 | 50.6 | +3.2 ± 0.96 | +3.4 ± 1.05 | −0.2 ± 1.38 | +0.3 ± 0.46 | +2.9 ± 1.27 |
| Seed 22, eighth, assignment | 24 | 47.3 | 49.3 | 47.4 | 50.2 | +1.9 ± 1.06 | +2.8 ± 0.89 | −0.9 ± 0.98 | +0.1 ± 0.17 | +1.9 ± 0.92 |
| Seed 22, eighth, assignment | 36 | 44.2 | 45.7 | 43.6 | 46.4 | +1.5 ± 0.92 | +2.2 ± 0.65 | −0.6 ± 1.58 | −0.6 ± 0.54 | +2.1 ± 0.97 |

- **Held less plain** lies between −1.2 and +0.6 at the 20 checkpoints, at most 1.04 times its unpaired SE.
- **Held less without replay** lies beyond twice its SE at 9 of the 20 checkpoints, and H-47's plain less without replay at 11. The two that
  differ are seed 22's eighth image at blocks 24 and 36, where it is +1.9 ± 1.06 and +1.5 ± 0.92 under the held night, within 2 SE, and +2.8
  ± 0.89 and +2.2 ± 0.65 under H-47's plain night, above.
- **The held night without replay** lies between −0.6 and +1.0 from the night without replay at every checkpoint, beyond twice its SE at 2
  of 20: seed 24's eighth image at block 12, +1.0 ± 0.08, and block 24, +0.9 ± 0.44.
- **Held less held without replay**, the replay with the hold set in both, lies beyond twice its SE at 12 of the 20 checkpoints; at block 36
  it is −5.2 ± 1.30, +1.0 ± 0.86, +4.8 ± 1.24 and +2.1 ± 0.97.

Each stimulus's correct trials, the mean of the 48 reads, under each form:

| Cell | Block | A's trials: without replay | held | held without replay | plain on A's | B's trials: without replay | held | held without replay | plain on A's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | 30.4 | 31.1 | 30.4 | 31.0 | 24.6 | 25.8 | 24.7 | 25.9 |
| Seed 23, first mapping, mirrored | 12 | 28.5 | 29.7 | 28.6 | 29.6 | 23.8 | 24.8 | 23.9 | 24.8 |
| Seed 23, first mapping, mirrored | 18 | 28.5 | 27.3 | 29.1 | 27.6 | 23.5 | 24.6 | 23.8 | 24.8 |
| Seed 23, first mapping, mirrored | 24 | 30.0 | 24.4 | 30.1 | 25.0 | 23.5 | 26.1 | 23.7 | 25.9 |
| Seed 23, first mapping, mirrored | 36 | 29.3 | 23.7 | 29.6 | 24.1 | 23.9 | 24.6 | 23.9 | 24.2 |
| Seed 24, eighth, assignment | 6 | 21.3 | 23.3 | 21.3 | 23.8 | 17.5 | 15.2 | 17.4 | 15.3 |
| Seed 24, eighth, assignment | 12 | 18.4 | 18.6 | 18.9 | 18.8 | 16.4 | 14.4 | 17.0 | 15.4 |
| Seed 24, eighth, assignment | 18 | 18.0 | 13.8 | 17.6 | 14.4 | 16.5 | 15.2 | 16.7 | 14.1 |
| Seed 24, eighth, assignment | 24 | 19.2 | 14.7 | 19.4 | 15.3 | 16.3 | 13.9 | 17.1 | 14.3 |
| Seed 24, eighth, assignment | 36 | 12.9 | 13.0 | 13.3 | 13.1 | 12.7 | 13.6 | 12.4 | 14.3 |
| Seed 22, first mapping, mirrored | 6 | 30.6 | 31.3 | 30.6 | 31.4 | 26.0 | 26.5 | 26.1 | 26.4 |
| Seed 22, first mapping, mirrored | 12 | 28.2 | 29.4 | 28.2 | 29.3 | 21.3 | 23.0 | 21.3 | 23.1 |
| Seed 22, first mapping, mirrored | 18 | 28.0 | 28.0 | 28.1 | 27.6 | 21.0 | 24.8 | 20.6 | 25.1 |
| Seed 22, first mapping, mirrored | 24 | 27.5 | 27.7 | 27.6 | 27.6 | 18.4 | 23.4 | 18.1 | 23.0 |
| Seed 22, first mapping, mirrored | 36 | 27.8 | 27.4 | 27.5 | 27.4 | 16.4 | 21.8 | 16.8 | 21.8 |
| Seed 22, eighth, assignment | 6 | 24.4 | 26.4 | 24.3 | 26.4 | 25.9 | 25.6 | 26.0 | 25.8 |
| Seed 22, eighth, assignment | 12 | 22.9 | 25.9 | 23.1 | 25.4 | 24.8 | 25.4 | 24.8 | 25.3 |
| Seed 22, eighth, assignment | 18 | 24.4 | 23.9 | 24.8 | 23.8 | 22.8 | 26.5 | 22.7 | 26.8 |
| Seed 22, eighth, assignment | 24 | 26.0 | 25.1 | 25.8 | 25.8 | 21.3 | 24.2 | 21.6 | 24.4 |
| Seed 22, eighth, assignment | 36 | 24.7 | 22.9 | 24.5 | 23.1 | 19.5 | 22.9 | 19.1 | 23.2 |

## 2. The side under the held night

The side under the held night is the stimulus whose held less without replay difference is the larger. "A's less B's" is the paired
difference of the two, and "within 2 SE" marks it within twice that SE. H-47's side is the same reading of H-47's plain night, from H-47's
job. Stimulus A answers the even places in all four cells, and B the odd places:

| Cell | Block | A's: held less without replay | B's: held less without replay | A's less B's | Side under the held night | H-47's side | A's: held less plain (unpaired) | B's: held less plain (unpaired) |
| :--- | ---: | ---: | ---: | ---: | :--- | :--- | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | +0.6 ± 0.36 | +1.2 ± 0.61 | −0.5 ± 0.71 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | +0.1 ± 1.00 | −0.1 ± 0.94 |
| Seed 23, first mapping, mirrored | 12 | +1.2 ± 0.59 | +1.0 ± 0.96 | +0.2 ± 1.11 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | +0.1 ± 0.90 | 0.0 ± 1.28 |
| Seed 23, first mapping, mirrored | 18 | −1.2 ± 0.96 | +1.1 ± 0.77 | −2.4 ± 1.36 | B, the odd places (within 2 SE) | B, the odd places | −0.3 ± 1.18 | −0.2 ± 0.96 |
| Seed 23, first mapping, mirrored | 24 | −5.7 ± 1.86 | +2.6 ± 0.57 | −8.3 ± 2.26 | B, the odd places | B, the odd places | −0.7 ± 1.03 | +0.2 ± 1.21 |
| Seed 23, first mapping, mirrored | 36 | −5.5 ± 1.84 | +0.7 ± 0.96 | −6.2 ± 2.55 | B, the odd places | B, the odd places | −0.3 ± 0.96 | +0.4 ± 0.89 |
| Seed 24, eighth, assignment | 6 | +2.0 ± 0.88 | −2.3 ± 0.84 | +4.3 ± 1.52 | A, the even places | A, the even places | −0.5 ± 0.84 | 0.0 ± 0.95 |
| Seed 24, eighth, assignment | 12 | +0.1 ± 1.03 | −2.0 ± 1.50 | +2.1 ± 2.32 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | −0.2 ± 1.05 | −1.0 ± 0.97 |
| Seed 24, eighth, assignment | 18 | −4.2 ± 0.96 | −1.3 ± 2.36 | −2.9 ± 2.67 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | −0.6 ± 0.86 | +1.1 ± 1.44 |
| Seed 24, eighth, assignment | 24 | −4.6 ± 0.69 | −2.4 ± 1.40 | −2.1 ± 1.49 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | −0.6 ± 1.14 | −0.4 ± 1.59 |
| Seed 24, eighth, assignment | 36 | +0.1 ± 1.01 | +1.0 ± 0.74 | −0.9 ± 1.30 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | −0.1 ± 0.86 | −0.6 ± 1.11 |
| Seed 22, first mapping, mirrored | 6 | +0.7 ± 0.39 | +0.5 ± 0.56 | +0.2 ± 0.68 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | 0.0 ± 0.85 | +0.1 ± 0.70 |
| Seed 22, first mapping, mirrored | 12 | +1.2 ± 0.40 | +1.7 ± 1.40 | −0.5 ± 1.18 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | +0.1 ± 1.14 | −0.1 ± 1.07 |
| Seed 22, first mapping, mirrored | 18 | 0.0 ± 1.06 | +3.8 ± 1.40 | −3.7 ± 1.02 | B, the odd places | B, the odd places | +0.4 ± 0.96 | −0.3 ± 0.99 |
| Seed 22, first mapping, mirrored | 24 | +0.2 ± 1.06 | +5.0 ± 1.57 | −4.8 ± 1.15 | B, the odd places | B, the odd places | +0.1 ± 0.67 | +0.4 ± 1.45 |
| Seed 22, first mapping, mirrored | 36 | −0.4 ± 0.65 | +5.4 ± 1.57 | −5.7 ± 1.83 | B, the odd places | B, the odd places | 0.0 ± 0.84 | 0.0 ± 1.02 |
| Seed 22, eighth, assignment | 6 | +2.0 ± 0.31 | −0.3 ± 0.60 | +2.3 ± 0.77 | A, the even places | A, the even places | 0.0 ± 0.59 | −0.2 ± 1.30 |
| Seed 22, eighth, assignment | 12 | +3.0 ± 0.66 | +0.6 ± 0.87 | +2.4 ± 1.23 | A, the even places (within 2 SE) | A, the even places | +0.5 ± 0.67 | +0.1 ± 1.14 |
| Seed 22, eighth, assignment | 18 | −0.5 ± 0.86 | +3.7 ± 0.56 | −4.3 ± 1.10 | B, the odd places | B, the odd places | +0.1 ± 1.52 | −0.3 ± 0.71 |
| Seed 22, eighth, assignment | 24 | −0.9 ± 0.81 | +2.9 ± 0.83 | −3.8 ± 1.25 | B, the odd places | B, the odd places (within 2 SE) | −0.6 ± 1.19 | −0.3 ± 0.98 |
| Seed 22, eighth, assignment | 36 | −1.9 ± 0.89 | +3.4 ± 0.78 | −5.3 ± 1.41 | B, the odd places | B, the odd places | −0.3 ± 1.31 | −0.4 ± 1.28 |

- **The side is H-47's at all 20 checkpoints**, by stimulus and so by group. At block 6 it lies on A, the even places, in three cells and on
  B, the odd places, on seed 23's; at block 12 on A in three and on B on seed 22's first mapping; from block 18 on B, the odd places, in all
  four cells.
- **It lies beyond twice its SE at 10 of the 20 checkpoints**, and H-47's at 11. They differ at three: seed 23's block 18 and seed 22's eighth
  image at block 12, beyond under H-47's plain night alone, and seed 22's eighth image at block 24, beyond under the held night alone.
- **Each stimulus's trials under the held night less H-47's plain night's** lie between −0.7 and +0.5 for A and between −1.0 and +1.1 for B,
  at most 1.01 times their unpaired SE, the two last columns.

Under the held night without replay, each stimulus's trials lie within 0.8 of the night without replay's, and its side, read the same way,
is the stimulus whose difference from the night without replay is the larger:

| Cell | Block | A's: held without replay less without replay | B's: the same | A's less B's | Its side |
| :--- | ---: | ---: | ---: | ---: | :--- |
| Seed 23, first mapping, mirrored | 6 | 0.0 ± 0.04 | +0.1 ± 0.12 | −0.1 ± 0.13 | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 12 | +0.1 ± 0.07 | +0.1 ± 0.12 | 0.0 ± 0.15 | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 18 | +0.5 ± 0.22 | +0.3 ± 0.23 | +0.3 ± 0.04 | A, the even places |
| Seed 23, first mapping, mirrored | 24 | +0.1 ± 0.16 | +0.2 ± 0.27 | −0.1 ± 0.15 | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 36 | +0.4 ± 0.17 | 0.0 ± 0.62 | +0.4 ± 0.72 | A, the even places (within 2 SE) |
| Seed 24, eighth, assignment | 6 | 0.0 ± 0.10 | −0.1 ± 0.13 | +0.1 ± 0.10 | A, the even places (within 2 SE) |
| Seed 24, eighth, assignment | 12 | +0.4 ± 0.32 | +0.6 ± 0.27 | −0.2 ± 0.58 | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | 18 | −0.4 ± 0.29 | +0.2 ± 0.25 | −0.6 ± 0.46 | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | 24 | +0.2 ± 0.24 | +0.8 ± 0.29 | −0.6 ± 0.24 | B, the odd places |
| Seed 24, eighth, assignment | 36 | +0.4 ± 0.70 | −0.3 ± 0.37 | +0.6 ± 0.34 | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | 6 | −0.1 ± 0.04 | +0.1 ± 0.09 | −0.1 ± 0.07 | B, the odd places |
| Seed 22, first mapping, mirrored | 12 | 0.0 ± 0.04 | 0.0 ± 0.19 | 0.0 ± 0.17 | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | 18 | +0.1 ± 0.14 | −0.4 ± 0.46 | +0.5 ± 0.51 | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | 24 | 0.0 ± 0.26 | −0.2 ± 0.40 | +0.3 ± 0.32 | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | 36 | −0.3 ± 0.17 | +0.5 ± 0.62 | −0.7 ± 0.52 | B, the odd places (within 2 SE) |
| Seed 22, eighth, assignment | 6 | −0.1 ± 0.06 | +0.1 ± 0.06 | −0.2 ± 0.10 | B, the odd places (within 2 SE) |
| Seed 22, eighth, assignment | 12 | +0.2 ± 0.19 | 0.0 ± 0.06 | +0.2 ± 0.23 | A, the even places (within 2 SE) |
| Seed 22, eighth, assignment | 18 | +0.4 ± 0.14 | −0.1 ± 0.56 | +0.4 ± 0.68 | A, the even places (within 2 SE) |
| Seed 22, eighth, assignment | 24 | −0.2 ± 0.26 | +0.3 ± 0.25 | −0.5 ± 0.48 | B, the odd places (within 2 SE) |
| Seed 22, eighth, assignment | 36 | −0.3 ± 0.23 | −0.3 ± 0.29 | 0.0 ± 0.08 | A, the even places (within 2 SE) |

## 3. The pair and A's couplings

The pair's count under the held night at each checkpoint, at block 36 under the other forms, and A's highest coupling over H-42's under each
form, in any image read and at any block's end, as clause 3 reads it. H-47's floor for clause 2 is 56, and its bound for clause 3 is 1.30:

| Cell | The pair under the held night at 6 | 12 | 18 | 24 | 36 | At 36: without replay | held without replay | plain on A's | A's highest coupling over H-42's: held | held without replay | without replay | plain on A's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 49.0 | 59.2 | 62.1 | 62.7 | 63.1 | 63.5 | 63.5 | 63.2 | 1.156 | 1.082 | 1.082 | 1.158 |
| Seed 24, eighth, assignment | 61.2 | 62.5 | 62.5 | 62.4 | 63.2 | 62.6 | 62.7 | 62.9 | 1.192 | 1.179 | 1.179 | 1.200 |
| Seed 22, first mapping, mirrored | 54.3 | 61.3 | 62.4 | 61.7 | 62.2 | 63.1 | 63.2 | 62.5 | 1.181 | 1.114 | 1.114 | 1.182 |
| Seed 22, eighth, assignment | 63.2 | 62.7 | 63.0 | 62.2 | 63.0 | 62.6 | 62.7 | 62.9 | 1.174 | 1.133 | 1.133 | 1.184 |

Under the held night the pair stands at 62.2 to 63.2 at block 36, and under H-47's plain night at 62.5 to 63.2. A's highest coupling under
the held night is 1.156 to 1.192 of H-42's, and under H-47's plain night 1.158 to 1.200, as in H-47's own table.

## 4. What each night moved

What the four nights moved, summed over the four, the mean over the three paths, from the class matrix in the images before and after each
night, as exploration 021 reads it and by group as 023 does. "Inhibitory onto a group's excitatory units" counts every inhibitory unit; a
group's inhibitory units are those inside its set; "onto their own" is a group's excitatory units onto the same units. H-47's plain night is
exploration 020's job's:

| Cell | Form | Inhibitory onto the odd places' excitatory units, M | onto the even places' | onto the inhibitory units | onto A's and B's units | onto the other excitatory | The odd places' inhibitory units onto all | the even places' | the other inhibitory units' | The odd places' excitatory units onto all | the even places' | The odd places' onto their own | the even places' onto their own |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | without replay | −1.79 | −2.04 | −0.59 | −0.90 | −0.04 | −2.70 | −2.65 | 0.00 | +1.48 | +0.14 | +0.18 | −0.28 |
| Seed 23, first mapping, mirrored | held | 0.00 | 0.00 | +2.94 | +1.08 | −0.03 | +2.12 | +1.87 | 0.00 | −10.74 | −9.96 | −1.85 | −1.59 |
| Seed 23, first mapping, mirrored | held without replay | 0.00 | 0.00 | −0.60 | −0.90 | −0.03 | −0.79 | −0.74 | 0.00 | +1.42 | +0.10 | +0.15 | −0.31 |
| Seed 23, first mapping, mirrored | plain on A's | +8.33 | +8.77 | +2.94 | +1.07 | −0.03 | +11.31 | +9.76 | 0.00 | −10.47 | −9.72 | −1.82 | −1.58 |
| Seed 24, eighth, assignment | without replay | +0.98 | +0.78 | +0.54 | −0.99 | +0.02 | +0.61 | +0.72 | 0.00 | +0.68 | −0.12 | −0.53 | −0.70 |
| Seed 24, eighth, assignment | held | 0.00 | 0.00 | +3.73 | +0.21 | +0.04 | +1.93 | +2.05 | 0.00 | −12.20 | −13.21 | −2.00 | −2.84 |
| Seed 24, eighth, assignment | held without replay | 0.00 | 0.00 | +0.56 | −0.99 | +0.02 | −0.19 | −0.22 | 0.00 | +0.67 | −0.15 | −0.55 | −0.69 |
| Seed 24, eighth, assignment | plain on A's | +11.38 | +11.76 | +3.66 | +0.22 | +0.04 | +13.53 | +13.52 | 0.00 | −11.23 | −12.33 | −1.78 | −2.75 |
| Seed 22, first mapping, mirrored | without replay | −1.92 | −2.23 | −0.39 | −0.99 | +0.03 | −2.93 | −2.57 | 0.00 | +0.52 | −0.20 | −0.34 | −0.80 |
| Seed 22, first mapping, mirrored | held | 0.00 | 0.00 | +3.07 | +0.71 | +0.06 | +1.76 | +2.07 | 0.00 | −9.35 | −10.90 | −2.46 | −2.25 |
| Seed 22, first mapping, mirrored | held without replay | 0.00 | 0.00 | −0.40 | −0.99 | +0.03 | −0.71 | −0.66 | 0.00 | +0.55 | −0.21 | −0.32 | −0.79 |
| Seed 22, first mapping, mirrored | plain on A's | +6.52 | +7.22 | +3.05 | +0.70 | +0.06 | +8.69 | +8.86 | 0.00 | −9.20 | −10.78 | −2.41 | −2.24 |
| Seed 22, eighth, assignment | without replay | +0.89 | +0.60 | +0.62 | −1.11 | +0.03 | +0.27 | +0.76 | 0.00 | −0.13 | −0.50 | −0.51 | −0.81 |
| Seed 22, eighth, assignment | held | 0.00 | 0.00 | +3.77 | 0.00 | +0.05 | +1.73 | +2.09 | 0.00 | −11.61 | −12.84 | −2.70 | −2.88 |
| Seed 22, eighth, assignment | held without replay | 0.00 | 0.00 | +0.62 | −1.11 | +0.03 | −0.34 | −0.11 | 0.00 | −0.16 | −0.51 | −0.52 | −0.80 |
| Seed 22, eighth, assignment | plain on A's | +10.61 | +11.30 | +3.72 | 0.00 | +0.05 | +12.47 | +13.21 | 0.00 | −10.68 | −12.13 | −2.55 | −2.72 |

- **Under the held night the inhibition onto each group's excitatory units shows 0.00**, the held synapses' change, where H-47's plain night
  added 6.52 to 11.76 M onto each group and the night without replay moved it by −2.23 to +0.98 M.
- **The image after a night is written after its quiet ticks.** In 15 of the 96 held nights a held synapse's presynaptic unit fired from
  the night's last tick on. In 14 of those, 13 to 46 held synapses moved in the quiet ticks, with the hold no longer in force, by −523 to
  +931 LSB together: in 6 held nights, all on seed 24's eighth image, and in 8 held nights without replay. In the fifteenth, seed 23's held
  night without replay on path 6 after block 12, no held synapse moved and none carried a trace after the quiet ticks. Section 5 counts
  them. In each of those 14 the class matrix's held entries moved by exactly what the quiet ticks moved, and in the other 82 by exactly
  nothing.
- **The held night's inhibition onto every other target is H-47's plain night's within 0.08 M**: onto the inhibitory units, onto A's and
  B's units and onto the other excitatory units.
- **The groups' excitatory units' synapses** lost 9.35 to 13.21 M under the held night, and 9.20 to 12.33 M under H-47's plain night.

The inhibition onto each group's excitatory units from the start image to the image each checkpoint reads, the mean over the three paths.
It holds the awake blocks' change as well as the nights':

| Cell | Block | Without replay: onto the odd places | onto the even places | Held: odd | even | Held without replay: odd | even | Plain on A's: odd | even |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | −1.10 | −1.08 | −0.73 | −0.62 | −0.73 | −0.62 | +2.82 | +2.94 |
| Seed 23, first mapping, mirrored | 12 | −2.48 | −2.29 | −1.68 | −1.52 | −1.57 | −1.35 | +4.06 | +4.38 |
| Seed 23, first mapping, mirrored | 18 | −3.84 | −3.67 | −2.68 | −2.50 | −2.46 | −2.17 | +4.38 | +4.88 |
| Seed 23, first mapping, mirrored | 24 | −5.20 | −5.15 | −3.72 | −3.48 | −3.36 | −3.05 | +4.13 | +4.80 |
| Seed 23, first mapping, mirrored | 36 | −7.13 | −7.09 | −5.82 | −5.46 | −5.39 | −5.13 | +1.61 | +2.35 |
| Seed 24, eighth, assignment | 6 | +0.69 | +0.61 | +0.43 | +0.39 | +0.43 | +0.39 | +4.15 | +4.18 |
| Seed 24, eighth, assignment | 12 | +1.07 | +1.06 | +0.56 | +0.57 | +0.72 | +0.78 | +6.70 | +6.80 |
| Seed 24, eighth, assignment | 18 | +1.40 | +1.48 | +0.60 | +0.72 | +0.91 | +1.13 | +8.52 | +8.86 |
| Seed 24, eighth, assignment | 24 | +1.82 | +2.01 | +0.69 | +0.82 | +1.19 | +1.52 | +9.99 | +10.47 |
| Seed 24, eighth, assignment | 36 | +2.09 | +2.46 | +0.90 | +1.13 | +1.67 | +2.22 | +8.81 | +9.34 |
| Seed 22, first mapping, mirrored | 6 | −1.24 | −1.25 | −0.79 | −0.74 | −0.79 | −0.74 | +1.94 | +2.28 |
| Seed 22, first mapping, mirrored | 12 | −2.59 | −2.66 | −1.75 | −1.68 | −1.62 | −1.60 | +2.82 | +3.14 |
| Seed 22, first mapping, mirrored | 18 | −4.00 | −4.30 | −2.76 | −2.73 | −2.52 | −2.64 | +2.83 | +3.28 |
| Seed 22, first mapping, mirrored | 24 | −5.46 | −5.90 | −3.76 | −3.80 | −3.46 | −3.72 | +2.40 | +3.04 |
| Seed 22, first mapping, mirrored | 36 | −7.41 | −7.97 | −5.74 | −5.95 | −5.51 | −5.91 | +0.10 | +0.62 |
| Seed 22, eighth, assignment | 6 | +0.53 | +0.67 | +0.30 | +0.46 | +0.30 | +0.46 | +3.80 | +4.04 |
| Seed 22, eighth, assignment | 12 | +0.85 | +0.91 | +0.36 | +0.54 | +0.51 | +0.68 | +6.10 | +6.40 |
| Seed 22, eighth, assignment | 18 | +1.10 | +1.18 | +0.44 | +0.62 | +0.71 | +0.92 | +7.88 | +8.40 |
| Seed 22, eighth, assignment | 24 | +1.35 | +1.44 | +0.52 | +0.68 | +0.85 | +1.17 | +9.30 | +9.94 |
| Seed 22, eighth, assignment | 36 | +1.45 | +1.71 | +0.75 | +0.87 | +1.13 | +1.64 | +8.30 | +8.77 |

At block 36 the held night's inhibition onto each group lies between −5.95 and +1.13 M from the start image, the held night without
replay's between −5.91 and +2.22 M, the night without replay's between −7.97 and +2.46 M, and H-47's plain night's between +0.10 and +9.34 M.

Each night's change of the engine's two sums, from its `start` line to its `end` line, the mean over the three paths:

| Cell | Gap | Without replay: inhibitory sum, M | excitatory sum | Held: inhibitory | excitatory | Held without replay: inhibitory | excitatory | Plain on A's: inhibitory | excitatory |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | −1.19 | −0.48 | +1.74 | −7.79 | −0.37 | −0.48 | +8.85 | −7.72 |
| Seed 23, first mapping, mirrored | 12 | −1.40 | +0.37 | +1.07 | −6.01 | −0.41 | +0.37 | +5.81 | −5.84 |
| Seed 23, first mapping, mirrored | 18 | −1.43 | +0.64 | +0.67 | −4.09 | −0.43 | +0.61 | +3.79 | −3.93 |
| Seed 23, first mapping, mirrored | 24 | −1.33 | +1.02 | +0.52 | −3.01 | −0.33 | +0.96 | +2.62 | −2.89 |
| Seed 24, eighth, assignment | 6 | +0.32 | −0.47 | +1.31 | −9.85 | −0.15 | −0.48 | +8.80 | −9.36 |
| Seed 24, eighth, assignment | 12 | +0.25 | +0.10 | +0.99 | −7.14 | −0.15 | +0.10 | +7.16 | −6.68 |
| Seed 24, eighth, assignment | 18 | +0.32 | +0.43 | +0.87 | −4.88 | −0.08 | +0.41 | +5.92 | −4.45 |
| Seed 24, eighth, assignment | 24 | +0.44 | +0.34 | +0.81 | −3.85 | −0.03 | +0.34 | +5.16 | −3.38 |
| Seed 22, first mapping, mirrored | 6 | −1.32 | −0.44 | +1.61 | −7.63 | −0.36 | −0.45 | +7.36 | −7.55 |
| Seed 22, first mapping, mirrored | 12 | −1.39 | +0.09 | +0.96 | −5.85 | −0.35 | +0.10 | +4.75 | −5.75 |
| Seed 22, first mapping, mirrored | 18 | −1.39 | +0.40 | +0.74 | −3.85 | −0.31 | +0.42 | +3.21 | −3.85 |
| Seed 22, first mapping, mirrored | 24 | −1.40 | +0.37 | +0.53 | −3.03 | −0.34 | +0.36 | +2.24 | −2.96 |
| Seed 22, eighth, assignment | 6 | +0.28 | −0.61 | +1.29 | −9.55 | −0.16 | −0.62 | +8.35 | −9.17 |
| Seed 22, eighth, assignment | 12 | +0.23 | −0.11 | +0.96 | −6.83 | −0.14 | −0.11 | +6.67 | −6.30 |
| Seed 22, eighth, assignment | 18 | +0.24 | −0.06 | +0.82 | −4.68 | −0.08 | −0.07 | +5.74 | −4.29 |
| Seed 22, eighth, assignment | 24 | +0.28 | +0.23 | +0.75 | −3.54 | −0.07 | +0.22 | +4.92 | −3.19 |

A held night, the mean of its three paths, raised the inhibitory sum by 0.52 to 1.74 M and H-47's plain night by 2.24 to 8.85 M; it
lowered the excitatory sum by 3.01 to 9.85 M and the plain night by 2.89 to 9.36 M.

## 5. What the hold leaves

At each held night, over its twelve nights a cell and form (ranges are over the twelve): the synapses held; their traces cleared at the
night's last tick, how many were not zero and what they held above and below zero; the inhibitory synapses onto other units the night
moved; the nights in which a held synapse's presynaptic unit fired from the night's last tick on, and the nights in which held synapses
moved in the quiet ticks; and the night's pairings still to be read, as What ran defines them:

| Cell | Form | Nights | Held synapses | Traces cleared, not zero | cleared above zero, LSB | below zero | Other inhibitory synapses moved | Nights a held synapse's presynaptic unit fired from the night's last tick on | Nights held synapses moved in the quiet ticks: how many, their change | Pending pairings, the target's spike after the presynaptic unit's | their sum, LSB | their largest | With a spike at the next tick, at most, LSB |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | held | 12 | 4 881 | 4 871 to 4 878 | 536 067 to 672 315 | −128 970 to −113 249 | 1 538 to 1 588 | 0 | none | 1 949 to 2 250 | 69 308 to 100 057 | 326 to 328 | 10 764 to 14 938 |
| Seed 23, first mapping, mirrored | held without replay | 12 | 4 881 | 4 863 to 4 877 | 208 725 to 315 280 | −164 556 to −152 479 | 1 534 to 1 597 | 3 | 2: 26 synapses, 931 LSB | 1 722 to 1 998 | 16 991 to 24 735 | 311 to 328 | 11 789 to 17 930 |
| Seed 24, eighth, assignment | held | 12 | 4 904 | 4 882 to 4 900 | 595 141 to 753 758 | −124 473 to −107 795 | 1 406 to 1 466 | 6 | 6: 14 to 25 synapses, −184 to −45 LSB | 2 106 to 2 285 | 77 127 to 113 394 | 327 to 328 | 9 591 to 16 506 |
| Seed 24, eighth, assignment | held without replay | 12 | 4 904 | 4 878 to 4 894 | 234 135 to 315 966 | −154 683 to −140 992 | 1 369 to 1 444 | 3 | 3: 13 to 23 synapses, −264 to −144 LSB | 1 775 to 2 123 | 23 264 to 31 495 | 324 to 328 | 11 045 to 20 109 |
| Seed 22, first mapping, mirrored | held | 12 | 4 863 | 4 835 to 4 862 | 511 414 to 659 819 | −129 594 to −118 082 | 1 541 to 1 604 | 0 | none | 1 908 to 2 089 | 57 906 to 76 544 | 326 to 328 | 11 622 to 15 862 |
| Seed 22, first mapping, mirrored | held without replay | 12 | 4 863 | 4 845 to 4 858 | 238 929 to 283 057 | −163 136 to −153 086 | 1 564 to 1 623 | 3 | 3: 45 to 46 synapses, −523 to −511 LSB | 1 692 to 1 923 | 20 923 to 31 456 | 326 to 328 | 10 159 to 16 905 |
| Seed 22, eighth, assignment | held | 12 | 4 863 | 4 852 to 4 861 | 593 999 to 742 061 | −128 348 to −109 558 | 1 428 to 1 484 | 0 | none | 1 886 to 2 223 | 66 092 to 104 645 | 326 to 328 | 10 778 to 17 428 |
| Seed 22, eighth, assignment | held without replay | 12 | 4 863 | 4 839 to 4 857 | 235 803 to 344 257 | −154 550 to −144 154 | 1 406 to 1 487 | 0 | none | 1 682 to 1 902 | 27 420 to 35 863 | 325 to 328 | 11 346 to 15 431 |

- **Before the night's first tick** 4 805 to 4 884 of the held synapses carried a trace that was not zero.
- **The pairing after the presynaptic unit's last spike**, as a presynaptic spike before the target's next would read it, is pending on
  1 886 to 2 285 held synapses under the held night, 57 906 to 113 394 LSB together, the largest 326 to 328 LSB, against $A_+$ of 328. That
  sum is 10.1 % to 17.7 % the size of the positive trace cleared at the night's last tick. Under the held night without replay it is pending
  on 1 682 to 2 123, 16 991 to 35 863 LSB together, 7.3 % to 12.5 % the size of it.
- **The pairing with the next presynaptic spike** is at most 9 591 to 17 428 LSB together under the held night, and 10 159 to 20 109 under
  the held night without replay, every held synapse's target having fired since the night's first tick.
- These are trace, not weight: at the next presynaptic spike, in the next day's engine, each synapse's trace takes its pairings and loses
  the presynaptic depression, and its consolidation moves the weight under that day's modulation. That consolidation is not read here.

## 6. The ties

The tied trials of each cell's 240 reads of A and 240 of the pair, under each form:

| Cell | Without replay, A's and the pair's | H-47's without replay | Held | H-47's plain on A's | Held without replay |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 792 and 293 | 792 and 293 | 838 and 386 | 866 and 365 | 780 and 319 |
| Seed 24, eighth, assignment | 1114 and 362 | 1114 and 362 | 1389 and 165 | 1324 and 174 | 1142 and 356 |
| Seed 22, first mapping, mirrored | 943 and 302 | 943 and 302 | 848 and 344 | 878 and 357 | 899 and 304 |
| Seed 22, eighth, assignment | 917 and 207 | 917 and 207 | 890 and 133 | 898 and 130 | 883 and 200 |

The night without replay's ties are H-47's, read by read. Under the held night they lie 8 to 65 from H-47's plain night's on A's reads and 3
to 21 on the pair's.

## What it costs

The trial took 391 s and the four jobs 5 796 to 6 057 s, at most 42.1 % of a job's 240 minutes. Exploration 020's jobs of two forms on the
same four cells took 3 874 to 4 106 s. The held night without replay joined each job; nothing was cut. Five jobs ran, every one to the end.

## What it suggests

**In these four cells A's count under the held night lies within 2 SE of H-47's plain night's at every checkpoint**, held less plain −1.2
to +0.6, and each stimulus's trials within 1.1 of it, while the inhibition onto the readouts' excitatory units, which the plain night raised
by 6.52 to 11.76 M onto each group over four nights, did not move in the held nights themselves. The side is H-47's at every checkpoint.
Held less without replay at block 36 is −4.8 ± 1.45, +1.1 ± 1.20, +5.0 ± 1.56 and +1.5 ± 0.92, where H-47's clause 1 read −4.9 ± 1.09,
+1.8 ± 1.79, +5.0 ± 1.24 and +2.2 ± 0.65. The held night ran the plain night's counts of replays and depotentiations, night by night. The
groups' excitatory units' synapses lost 9.35 to 13.21 M under it and 9.20 to 12.33 M under the plain night, and moved by −0.50 to +1.48 M
without replay. These runs do not part the inhibition's share from the rest: the hold leaves pairings pending, the quiet ticks moved some
held synapses, and under the held night each group's excitatory units' synapses lost up to 0.97 M more than under the plain night, and the
engine's excitatory sum up to 1.85 M more, over four nights.

**The hold leaves part of the night's pairings.** Under the held night, pairings 10.1 % to 17.7 % the size of the positive trace it
cleared are pending, as a presynaptic spike before the target's next would read them, at most 113 394 LSB of trace a night; under the held
night without replay, 7.3 % to 12.5 %. In 14 of 96 held nights the quiet ticks moved held synapses by up to 931 LSB together. This note
does not read how much of the pending trace reaches a weight the next day.

Questions this suggests for a later exploration or a round:
- The rest of ADR-0238's night: the readouts' excitatory synapses held, alone or with their inhibition.
- The held night on H-47's other eight cells, among them every cell where A answers readout 0.
- A read of what the next day's first consolidations do with the pending pairings, and whether a hold through the quiet ticks changes them.

## What did not run

- **H-47's other eight cells**, among them every cell where stimulus A answers readout 0, where exploration 021 read the lean onto A's
  readout at 1.35 to 3.71 M.
- **Plain nights on both mappings and the awake gap**, under the hold or not: no reading here stands against them.
- **The rest of ADR-0238's night**: the readouts' excitatory synapses and the inhibitory synapses onto their inhibitory units consolidated
  as in H-47 throughout.
- **A hold through the quiet ticks**: the hold ends at the night's last tick, and the image is written after the quiet ticks.
- **The pending pairings' effect on a weight**: the next day's consolidation of the held synapses is not read.
- **REM's depotentiations and the zero sum's rounds** are not split out of a night's change, as in explorations 020 to 023.
- **Any SE for sections 4 and 5**: the class matrix is a mean over the three paths, and the hold's readings are ranges.
- **Any other share, form, network or seed.**

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_the_readouts_inhibition_held_explore` | [42405480901](https://github.com/Menteion/Menteion/actions/runs/42405480901) | `ca36ddc` | 391 |
| `the_readouts_inhibition_held_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [42406369084](https://github.com/Menteion/Menteion/actions/runs/42406369084) | `ca36ddc` | 6057 |
| `the_readouts_inhibition_held_the_eighth_image_on_seed_24_from_the_assignment_explore` | [42406369084](https://github.com/Menteion/Menteion/actions/runs/42406369084) | `ca36ddc` | 5796 |
| `the_readouts_inhibition_held_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [42406369084](https://github.com/Menteion/Menteion/actions/runs/42406369084) | `ca36ddc` | 5900 |
| `the_readouts_inhibition_held_the_eighth_image_on_seed_22_from_the_assignment_explore` | [42406369084](https://github.com/Menteion/Menteion/actions/runs/42406369084) | `ca36ddc` | 5908 |
