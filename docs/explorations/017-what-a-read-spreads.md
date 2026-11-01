---
status: exploratory
date: 2026-11-02
---

# 017: What a read spreads

**Branch**: `explore/017-what-a-read-spreads`
**Commit**: `d261f6a` for the harness's trial, `de34138` for the rest, which adds tests. Both carry `deab29a`, `main` at
exploration 016's note, and the tag `explore/016-what-a-shared-source-takes` with its four changes under `src/`. Neither adds any.
**Runs**: [41325148027](https://github.com/Menteion/Menteion/actions/runs/41325148027) (the harness's trial, question 1 from the
eighth image and the assignment), [41328059162](https://github.com/Menteion/Menteion/actions/runs/41328059162) (questions 2 and 3
from that start) and [41332466567](https://github.com/Menteion/Menteion/actions/runs/41332466567) (the other three starts), all at
`scope=arms`

[Exploration 016](016-what-a-shared-source-takes.md) found a pair sharing 26 of 51 of A's sources that leaves A at 42, 27, 33 and 50
of 64 after 24 blocks. A plain night there raised the margin about 1.1 M and the count had no steady direction, while four rows
with no night read 8 to 12 apart at one margin. Its counts were one block each, so it could not part a night from a read. This
note reads the spread itself. It asks, under 016's second section:

1. how one image's reads spread, and whether reading moves the image;
2. whether a night shows in the mean of many reads of an image;
3. how far a path spreads apart from the reads, and from that, how many reads and paths it takes to see a gap of 8 of 64.

It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the repository's
AArch64 host, on seed 22 alone.

**In brief**
- **One read on this slope is one draw from a spread of about 3.5.** Sixteen reads of A on one image, at 12 and 24 blocks, spread by
  an SD of 3.20 to 4.50 and by as much as 37 to 55. A binomial of 64 at the same means gives 3.26 to 4.00. At the ceiling
  the SD is 0.63 to 1.21.
- **A read's count goes with its stream, near one tick.** Two images' sixteen counts correlate at +0.66 at one tick, +0.49 at 16 to 127
  ticks apart, +0.15 at 520 to 729, and −0.03 at 2 261 or more. All of 016's counts were one stream.
- **Most of 016's one start that stood apart was that stream.** Seven nights read 49 to 53 there on stream 0, within 257 ticks of each
  other. The other fifteen streams average 35.0 to 42.9 on the same images.
- **A read moves the margin little.** A fork moved it in 364 of 1 972 reads from the eighth image and 710 of 1 972 from a first mapping, by at
  most 31 414, against an SD over paths of 0.05 to 0.10 M. From a first mapping 16 blocks on one engine lower the inhibitory sum by 3.5
  to 4.4 %.
- **Means narrow the rows with no night to 1.6 to 5.3 at block 24**, where one read gave 8 to 12. Change in count and in margin
  agree in sign in 24 of the 32 night cells there, where one read gave 18.
- **Plain on A's patterns less the awake row, nine paths each:** +2.4, +3.5, +6.7, +2.1 right after the gap, and +0.0, +0.9, +4.7,
  +0.0 twelve blocks on, each with an SE of 0.75 to 1.34, the margin up 1.05 to 1.30 M at both.
- **A path spreads less than a read.** Pooled, a read's SD is 3.52 and a path's past the reads 1.20. The margin's SD over paths is
  0.05 to 0.10 M.
- **Worked out from those figures, not run: a gap of 8 of 64 stands at 1.5 SE on one read against one read.** It stands at 3 SE or more on
  eight reads on one path, four on each of two paths, or one on each of four, every read on a stream of its own. All of it is seed 22.

## What ran

**The network, the rules and the starts** are 016's second section, unchanged: H-42's network of seed 22 at 1 024 units, its
derivation run again and held to H-42's pinned tables (`probe_made`), H-46's arm, H-25's configuration, the slow-wave baseline at 1/2
on this branch alone, the assignment and the mirrored assignment, the eighth image and a first mapping. The pair shares 26 of 51
units, conflicting, each kind of unit held apart. **A**, **a read** and **the margin** are 016's.

**What fixes a read.** Its stimuli and its drive are drawn, each from a hash. The stimulus of trial `t` is bit 0 of
`mix64(27 ^ t)` (`Task::stimulus_at`), from `t` of 0 in 016. The drive's message reaches the unit `mix64` makes of its seed, 3, the
tick and the message's index (`Drive::unit_at`), and the tick runs on from the image's. So a read is a function of the image, the
first trial's index and the drive's seed, and an image read twice the same way gives one trace. A stream here is that index and that
seed.

**Three ways to vary it.** Each way's sixteen are stream 0, the read as it was, and fifteen more, `i` from 1 to 15:

| Way | First trial | Drive's seed |
| :--- | ---: | ---: |
| As it was, stream 0, 016's read | 0 | 3 |
| `Trials` | 64 × `i` | 3 |
| `Drive` | 0 | 3 + `i` |
| `Both` | 64 × `i` | 3 + `i` |

**Questions 2 and 3 read by `Both`.** The harness's trial showed why, and section 1 holds it for all four starts: on one drive a read
mostly repeats what that drive gives at a position and stimulus.

**Each read is a fork.** It loads the image afresh, so reads of one image do not touch each other. Reading on one engine is section
1's last table. Forks run two at a time, an engine being two threads. In the harness's trial four reads took 23 837 ms alone and
12 124 ms two at a time, to equal traces.

**Reading the logs.** Each read is a line `DUMP <label> read mirrored … correct [n] … spread <way> <i> ReadFrom { trial, drive } stimuli
right tied gain`, the three words one bit a trial. A label is `shared zero22 <arm> <from> share 0b0101 Apart conflict true <image>
<who>`, `<arm>` being `AssignmentFirst` or `MirroredFirst`, `<from>` `Eighth` or `FirstMapping`, and `<who>` `first` for A and
`second` for the pair. Every mean here is of `correct` over the sixteen lines of one label whose `spread` is `as-it-was 0` and, unless
a row names another way, `Both 1` to `Both 15`. Each table names its `<image>`. An **SD** is the sample's, over `n − 1`, and an **SE**
that SD over √16 unless the table says otherwise.

**Bit for bit.** 108 of the 236 reads as they were have a line in 016's logs, and the other 128 are the new paths. 106 of the 108 match
theirs from `read mirrored` through the block. Two do not: a first mapping's start, each way round, whose count, ties, trace, state and
spike counts match, and whose fields after the block differ, a sum by at most 1 564 and a coupling by at most 350. 016 read that image
under the start's two groups and this note under the cut's six, and the groups are the only argument of the read that differs. That
comes from reading the code, and no run tested it. From the eighth image the same lines match. Of what follows only the paragraph on
forks and the table of blocks on one engine read those fields, and they read this note's.

**Under `src/`** nothing changes. `run_on_presenting_from` in the harness and `ReadFrom` in the tests take the first trial and the
drive's seed, and at `AS_IT_WAS` they are 016's calls.

## 1. One image's reads

`<image>` is `blocks 0` for the start, `half none blocks 12` for 12 blocks of the pair, and `blocks 24` for its 24. Each cell is the
mean of sixteen, least to greatest, and the SD. The first column is the harness's trial, at `d261f6a`.

A's count of 64:

| Image, varied | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| The start, `Trials` | 64.0, 64 to 64, 0.00 | 63.7, 63 to 64, 0.48 | 60.4, 58 to 63, 1.59 | 62.0, 58 to 64, 1.75 |
| The start, `Drive` | 63.6, 63 to 64, 0.50 | 63.8, 63 to 64, 0.40 | 62.1, 61 to 63, 0.89 | 61.9, 60 to 63, 0.77 |
| The start, `Both` | 63.6, 61 to 64, 0.89 | 63.5, 62 to 64, 0.63 | 62.0, 59 to 64, 1.21 | 61.9, 59 to 64, 1.15 |
| 12 blocks, `Trials` | 47.4, 44 to 53, 2.63 | 36.5, 31 to 42, 3.12 | 43.8, 40 to 50, 2.88 | 53.9, 50 to 60, 2.50 |
| 12 blocks, `Drive` | 47.9, 43 to 51, 2.53 | 36.1, 28 to 44, 3.77 | 42.4, 36 to 50, 4.10 | 51.5, 47 to 57, 2.99 |
| 12 blocks, `Both` | 46.9, 37 to 55, 4.09 | 34.9, 29 to 44, 3.96 | 43.6, 37 to 55, 4.50 | 50.6, 43 to 55, 3.20 |
| 24 blocks, `Trials` | 42.5, 37 to 47, 3.06 | 28.1, 24 to 31, 2.06 | 38.7, 30 to 48, 4.99 | 46.7, 42 to 56, 3.55 |
| 24 blocks, `Drive` | 43.4, 34 to 51, 4.05 | 32.4, 23 to 38, 4.16 | 40.3, 33 to 49, 4.57 | 47.3, 41 to 53, 3.44 |
| 24 blocks, `Both` | 43.0, 36 to 52, 3.63 | 32.6, 27 to 39, 3.24 | 38.2, 33 to 44, 3.54 | 48.9, 43 to 58, 4.40 |

The pair's count of 64:

| Image, varied | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| The start, `Trials` | 7.4, 3 to 13, 2.22 | 1.2, 0 to 3, 0.98 | 5.8, 3 to 8, 1.64 | 10.4, 4 to 16, 3.03 |
| The start, `Drive` | 10.1, 5 to 15, 3.24 | 0.7, 0 to 3, 1.01 | 7.3, 4 to 10, 2.02 | 10.6, 6 to 16, 2.68 |
| The start, `Both` | 9.1, 4 to 14, 2.55 | 0.9, 0 to 3, 0.96 | 6.9, 3 to 11, 2.42 | 12.2, 7 to 25, 4.75 |
| 12 blocks, `Trials` | 63.1, 62 to 64, 0.81 | 61.1, 58 to 63, 1.24 | 59.0, 55 to 61, 1.79 | 60.1, 58 to 63, 1.31 |
| 12 blocks, `Drive` | 63.3, 62 to 64, 0.79 | 61.0, 57 to 64, 1.86 | 61.2, 60 to 63, 0.98 | 60.1, 57 to 63, 1.93 |
| 12 blocks, `Both` | 63.2, 61 to 64, 0.91 | 60.8, 58 to 64, 1.61 | 60.5, 56 to 63, 1.83 | 60.1, 56 to 64, 2.43 |
| 24 blocks, `Trials` | 63.2, 62 to 64, 0.75 | 63.1, 62 to 64, 0.85 | 62.4, 60 to 64, 0.96 | 62.8, 62 to 64, 0.58 |
| 24 blocks, `Drive` | 63.4, 62 to 64, 0.81 | 61.9, 59 to 64, 1.53 | 63.4, 62 to 64, 0.81 | 62.1, 60 to 64, 1.06 |
| 24 blocks, `Both` | 63.6, 61 to 64, 0.89 | 62.3, 59 to 64, 1.54 | 63.0, 61 to 64, 0.97 | 62.1, 60 to 64, 1.09 |

- **On the slope the spread is about a binomial's in size.** Under `Both` at 12 and 24 blocks A's SD is 3.20 to 4.50, against 3.26 to 4.00
  for a binomial of 64 at each mean, and the mean tie is 3.9 to 6.1 trials. It is not a binomial's in kind, a read's trials not being
  independent draws: see the next table and section 3's split. Over all 220 images read under `Both` below a mean of 58, questions 2
  and 3 included, the SD is 1.41 to 5.86 with a root mean square of 3.57, and least to greatest is 5 to 24 wide, 12.7 on average.
- **At the ceiling it nearly closes.** A at the start reads an SD of 0.63 to 1.21 and at most 5 wide. The pair once learned reads
  0.89 to 2.43 at 12 and 24 blocks.
- **The pair before it has learned** reads an SD of 0.96 to 4.75 at the start, at means of 0.9 to 12.2. From a first mapping and the
  mirrored assignment it reads 7 to 25.
- **`Trials` alone does not sample the read.** On the eight images at 12 and 24 blocks, a position and stimulus under one drive took its
  majority outcome in 0.896 to 0.956 of its reads, the mean over the 128 cells, against 0.598 to 0.805 at a position across sixteen
  drives on one stimulus stream. So `Trials`' mean can stand off: 28.1 against `Both`'s 32.6 from the eighth image and the mirrored
  assignment at 24 blocks, and 53.9 against 50.6 from a first mapping and the mirrored assignment at 12.
- **The read as it was is one of the sixteen, not the mean.** Over question 2's 92 images it less the mean of the other fifteen is
  +1.18 on average with an SD of 4.56, from −8.3 to +15.0.

**A stream carries across images near one tick.** Each read's sixteen streams are the same on every image, so two images of one start
and block can be held against each other stream by stream. Over question 2's forms and question 3's paths, by the correlation of the two
images' sixteen counts:

| Ticks apart | Pairs of images | Mean correlation | Least | Greatest |
| :--- | ---: | ---: | ---: | ---: |
| 0 | 264 | +0.66 | −0.09 | +0.95 |
| 1 to 14 | 361 | +0.58 | −0.40 | +0.94 |
| 16 to 127 | 264 | +0.49 | +0.08 | +0.90 |
| 136 to 255 | 137 | +0.30 | −0.31 | +0.71 |
| 256 to 511 | 188 | +0.25 | −0.30 | +0.74 |
| 520 to 729 | 30 | +0.15 | −0.37 | +0.67 |
| 2 261 to 3 499 | 720 | −0.01 | −0.73 | +0.61 |
| 16 929 to 4 060 621 | 952 | −0.04 | −0.71 | +0.59 |

The correlation falls from one tick, and no pair stands 730 to 2 260 ticks apart or 3 500 to 16 928, so where it reaches nothing was not
read. The eight nights of a start at block 24 stand within 262 to 718 ticks of each other, so they share part of their streams, though
their margins lie up to 2.33 M apart. Nothing halfway and No gap stand about 4 M ticks from them.

**016's one start.** It read seven of eight nights at 49 to 53 from a first mapping and the assignment at block 24, and asked why. Those
seven stand within 257 ticks of each other, on stream 0. `<image>` is the night's label and ` blocks 24`:

| Night | Tick | Stream 0 | The other fifteen, mean | All sixteen, mean | Margin, M |
| :--- | ---: | ---: | ---: | ---: | ---: |
| `before plain` | 77468689 | 51 | 38.3 | 39.1 | 1.31 |
| `before twin` | 77468999 | 42 | 29.8 | 30.6 | −0.85 |
| `half plain-first` | 77468945 | 49 | 42.1 | 42.5 | 1.48 |
| `half twin-first` | 77468944 | 50 | 35.0 | 35.9 | −0.65 |
| `half plain-second` | 77468946 | 51 | 40.3 | 40.9 | −0.06 |
| `half twin-second` | 77468944 | 53 | 41.4 | 42.1 | 0.67 |
| `half plain-both` | 77468945 | 51 | 42.9 | 43.4 | 0.69 |
| `half twin-both` | 77468944 | 50 | 38.1 | 38.9 | 0.10 |

Stream 0 takes most of it, not all. The sixteen's means of those seven read 35.9 to 43.4 against the rows with no night's 38.2 to 39.8,
and four stay above those rows. This note did not part the stream from the tick.

Each stream's mean over question 2's 11 or 12 images of a start at a block, less the mean of the sixteen. In that start at block 24,
stream 0 is the greatest of them:

| Start | Block | Stream 0 | Least of the sixteen | Greatest | SD of the sixteen |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Eighth, assignment | 12 | −1.4 | −3.9 | +2.4 | 1.60 |
| Eighth, assignment | 24 | +2.1 | −3.5 | +3.8 | 1.88 |
| Eighth, mirrored | 12 | +2.6 | −3.6 | +2.6 | 1.58 |
| Eighth, mirrored | 24 | −2.3 | −3.5 | +3.2 | 1.88 |
| First mapping, assignment | 12 | +1.2 | −3.3 | +2.7 | 1.85 |
| First mapping, assignment | 24 | +6.5 | −4.1 | +6.5 | 2.76 |
| First mapping, mirrored | 12 | +0.6 | −2.8 | +2.0 | 1.24 |
| First mapping, mirrored | 24 | −0.6 | −2.6 | +3.4 | 1.46 |

**Neither the stimuli's balance nor their order accounts for a stream.** A stream's stimulus 1 trials run 27 to 36. Over each start's 55
images a stream's mean count correlates with them at +0.53, +0.03, −0.21 and −0.46, in the columns' order, and a count weighing both
stimuli equally leaves the SD of the sixteen streams' means at 0.92, 1.23, 1.58 and 1.00 against 1.01, 1.16, 1.62 and 1.11. Over the 220
images a repeated stimulus reads 0.659 right and an alternation 0.663, and a read's alternations correlate with its count at +0.02, the
mean of each image's correlation. What a stream carries was not read, and this note did not part the drive's series near a tick from
the stimuli.

**A fork moves the margin a little.** Against the block's own end, a fork of A moved its image's margin in 364 of 1 972 reads from the
eighth image and 710 of 1 972 from a first mapping. From the eighth image none of the 92 at the start moved it, and the greatest is
31 414, one of 11 past 10 000, all 11 of them on one image, `half path 6 plain-first blocks 24` from the assignment. From a first
mapping all 92 at the start moved it, the greatest being 22 192. The margin's SD over paths is 0.05 to 0.10 M. Why a withheld read
moves a coupling at all was not read.

**Blocks on one engine.** 16 blocks with the reward withheld, no fork, stream 0 running on, A's and the pair's on each of the three
images above, 24 runs. The first block is the read as it was in all 24. **Margin moved** is the greatest distance of a block's end from the
image's. **Inhibitory sum moved** is the sixteenth block's end against the image's:

| Start | Margin moved, greatest | Inhibitory sum moved, of the sum | Gain, least to greatest | First 8 blocks less last 8, greatest |
| :--- | ---: | :--- | :--- | ---: |
| Eighth, assignment | 0 | −0.24 to +1.12 % | 111019 to 111680 | 2.5 |
| Eighth, mirrored | 27 | −0.12 to +1.05 % | 111010 to 111659 | 0.9 |
| First mapping, assignment | 13732 | −4.35 to −3.53 % | 112097 to 112539 | 1.9 |
| First mapping, mirrored | 673 | −4.37 to −3.75 % | 112134 to 112639 | 1.8 |

- **The margin stands in these runs.** It moved by 0 in 10 of the 12 from the eighth image, and by at most 1 740 from a first mapping but for
  one run of 13 732, A's at the start. These are stream 0 on three images, and the forks above go further.
- **From a first mapping the inhibitory sum falls in every run**, by 5.0 to 6.0 M of 134 to 142 M. Why was not read. Forks do not
  carry it, so no mean here does.
- **The count's first 8 blocks and last 8 differ by at most 2.5.** These blocks run on past the image's tick on trials 0 to 1 023, and
  are not the forks' reads, so this table says what consecutive blocks do and says nothing of a fork's spread.

## 2. Is a night visible in the means?

016's forms, A read sixteen times by `Both` on each image, `<image>` being the form's label and ` blocks <b>`, and at block 24 No gap being
`blocks 24` alone. A's patterns, the pair's and both ran, and nothing was cut. No gap and Nothing halfway are one image at block 12. The
margins are 016's, image for image.

A's mean count of 64 at blocks 12 and 24:

| Form (its label) | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | ---: | ---: | ---: | ---: |
| No gap (`half none` at 12, none at 24) | 46.9, 43.0 | 34.9, 32.6 | 43.6, 38.2 | 50.6, 48.9 |
| Awake before (`before awake`) | 45.3, 44.3 | 31.8, 29.4 | 43.6, 39.8 | 48.2, 43.6 |
| Nothing halfway (`half none`) | 46.9, 45.9 | 34.9, 32.8 | 43.6, 39.4 | 50.6, 47.0 |
| Awake halfway (`half awake`) | 46.4, 46.9 | 36.6, 33.0 | 43.3, 38.9 | 50.0, 45.7 |
| Plain before, A's patterns (`before plain`) | 47.5, 44.4 | 34.6, 32.7 | 47.8, 39.1 | 49.6, 48.1 |
| Twin before, A's patterns (`before twin`) | 41.8, 40.3 | 24.6, 22.9 | 34.1, 30.6 | 46.6, 43.6 |
| Plain halfway, A's patterns (`half plain-first`) | 48.9, 46.9 | 39.6, 30.5 | 49.6, 42.5 | 52.1, 46.6 |
| Twin halfway, A's patterns (`half twin-first`) | 43.2, 41.8 | 28.6, 23.4 | 40.8, 35.9 | 49.6, 43.9 |
| Plain halfway, the pair's (`half plain-second`) | 48.5, 46.8 | 31.7, 26.4 | 45.1, 40.9 | 52.5, 46.5 |
| Twin halfway, the pair's (`half twin-second`) | 52.8, 48.0 | 38.6, 29.8 | 48.4, 42.1 | 54.1, 49.6 |
| Plain halfway, both (`half plain-both`) | 49.5, 46.6 | 35.7, 30.7 | 48.1, 43.4 | 52.3, 46.5 |
| Twin halfway, both (`half twin-both`) | 49.1, 45.1 | 32.1, 27.0 | 44.7, 38.9 | 53.3, 46.1 |

A's SD on these images is 1.41 to 5.86. Each night's mean less the mean of the images with no night, ± an SE, then its margin less theirs,
in M. Three images have no night at block 12 and four at block 24. The ± is the SE of that difference, `√(SD²/16 + ΣSDᵢ²/16/k²)`, the sum
over the `k` images with no night. It is the reads' alone: a path adds to it (section 3), and two nights near one tick share part of a
stream, so their difference is firmer than the two ± say.

At block 12:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| The rows with no night, mean; margin | 46.2; 1.24 | 34.4; 0.33 | 43.5; 0.51 | 49.6; 1.60 |
| Plain before, A's patterns | +1.3 ± 1.1; +0.98 | +0.2 ± 1.1; +1.19 | +4.3 ± 1.1; +1.11 | +0.0 ± 1.0; +0.80 |
| Twin before, A's patterns | −4.4 ± 1.0; −0.65 | −9.8 ± 1.1; −0.76 | −9.4 ± 1.6; −1.21 | −3.0 ± 0.9; −0.92 |
| Plain halfway, A's patterns | +2.7 ± 1.0; +1.21 | +5.2 ± 0.8; +1.32 | +6.1 ± 1.0; +1.24 | +2.5 ± 0.7; +1.09 |
| Twin halfway, A's patterns | −3.0 ± 1.2; −0.50 | −5.8 ± 0.9; −0.70 | −2.7 ± 1.0; −0.98 | +0.0 ± 1.0; −0.82 |
| Plain halfway, the pair's | +2.3 ± 0.9; −0.36 | −2.7 ± 0.9; −0.65 | +1.6 ± 1.0; −0.41 | +2.9 ± 0.7; −0.37 |
| Twin halfway, the pair's | +6.5 ± 0.9; +0.52 | +4.2 ± 1.0; +0.28 | +4.9 ± 1.0; +0.42 | +4.5 ± 0.6; +0.65 |
| Plain halfway, both | +3.3 ± 0.9; +0.48 | +1.3 ± 0.8; +0.31 | +4.6 ± 1.0; +0.45 | +2.7 ± 0.7; +0.31 |
| Twin halfway, both | +2.9 ± 1.2; +0.03 | −2.3 ± 1.2; −0.23 | +1.2 ± 1.0; −0.18 | +3.7 ± 0.8; −0.02 |

At block 24:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| The rows with no night, mean; margin | 45.0; 1.15 | 31.9; 0.22 | 39.1; 0.35 | 46.3; 1.31 |
| Plain before, A's patterns | −0.6 ± 1.0; +0.94 | +0.8 ± 1.0; +1.25 | +0.0 ± 1.5; +0.97 | +1.8 ± 0.8; +0.88 |
| Twin before, A's patterns | −4.8 ± 0.9; −0.67 | −9.0 ± 1.2; −0.71 | −8.5 ± 1.3; −1.19 | −2.7 ± 1.0; −0.85 |
| Plain halfway, A's patterns | +1.9 ± 0.9; +1.18 | −1.4 ± 1.1; +1.10 | +3.4 ± 1.0; +1.13 | +0.3 ± 1.0; +1.12 |
| Twin halfway, A's patterns | −3.2 ± 0.9; −0.46 | −8.5 ± 1.1; −0.77 | −3.1 ± 1.5; −1.00 | −2.4 ± 0.9; −0.78 |
| Plain halfway, the pair's | +1.8 ± 0.9; −0.43 | −5.5 ± 0.9; −0.70 | +1.9 ± 1.1; −0.41 | +0.2 ± 0.9; −0.37 |
| Twin halfway, the pair's | +3.0 ± 0.8; +0.25 | −2.1 ± 1.2; +0.05 | +3.0 ± 1.4; +0.32 | +3.3 ± 0.8; +0.58 |
| Plain halfway, both | +1.6 ± 1.0; +0.44 | −1.2 ± 0.9; +0.26 | +4.4 ± 1.2; +0.34 | +0.2 ± 1.0; +0.29 |
| Twin halfway, both | +0.1 ± 1.0; −0.05 | −4.9 ± 0.8; −0.30 | −0.2 ± 1.3; −0.25 | −0.2 ± 0.9; −0.09 |

One read of each image, stream 0, against the mean of sixteen, each list in its columns' order. The block 24 tallies in the first column
are 016's. Its block 12 readings set other rows against each other, so those here are worked out from its counts, over the three images:

| Reading | One read, stream 0 | Mean of sixteen |
| :--- | :--- | :--- |
| The rows with no night, greatest less least, block 12 | 3, 2, 6, 6 | 1.7, 4.8, 0.3, 2.4 |
| The rows with no night, greatest less least, block 24 | 8, 12, 9, 9 | 3.9, 3.6, 1.6, 5.3 |
| The 32 night cells inside / above / below those rows, block 12 | 14 / 12 / 6 | 5 / 19 / 8 |
| The 32 night cells inside / above / below those rows, block 24 | 18 / 11 / 3 | 16 / 7 / 9 |
| Change in count and in margin agree in sign, block 12 | 22 of 32 | 26 of 32 |
| Change in count and in margin agree in sign, block 24 | 18 of 32 | 24 of 32 |
| Plain halfway on A's patterns less Awake halfway, block 12 | +3, +9, +11, +3 | +2.5, +3.0, +6.3, +2.1 |
| Plain halfway on A's patterns less Awake halfway, block 24 | +5, −11, +10, −6 | +0.1, −2.5, +3.6, +0.9 |
| Twin halfway on the pair's less Awake halfway, block 12 | +5, +6, +8, +6 | +6.3, +2.0, +5.1, +4.1 |
| Twin halfway on the pair's less Awake halfway, block 24 | +9, −12, +14, +1 | +1.1, −3.2, +3.2, +3.9 |

- **The rows with no night close up.** Within 0.13 M of each other, their means stand 1.6 to 5.3 apart at block 24.
- **At block 24 the cells leave the rows on both sides**, seven above and nine below, where one read gave eleven and three. Four of the
  seven are from a first mapping and the assignment, where one read gave seven.
- **Right after the gap most cells leave the rows**, 19 above and 8 below, those rows being three images. At block 24, 16 are inside.
- **Change in count and in margin agree in sign more often**, in 26 and 24 of 32 against 22 and 18.

**The count against the margin**, over question 2's 92 images, 016's bands:

| Margin, M | Images | One read, stream 0 | Mean of sixteen |
| :--- | ---: | ---: | ---: |
| −1 to 0 | 12 | 20 to 51 | 22.9 to 40.9 |
| 0 to 0.5 | 21 | 25 to 50 | 29.4 to 45.1 |
| 0.5 to 1 | 16 | 36 to 53 | 35.7 to 49.6 |
| 1 to 1.5 | 20 | 26 to 53 | 30.5 to 52.5 |
| 1.5 to 2 | 14 | 36 to 54 | 34.6 to 53.3 |
| 2 to 2.5 | 8 | 42 to 54 | 44.4 to 54.1 |
| 2.5 to 3 | 1 | 51 to 51 | 52.1 to 52.1 |

A line through each start's images, of stream 0's read or the mean against the margin: its slope a million, its residual SD over `n − 2`,
and r:

| Start | Images | One read: a M, residual SD, r | Mean of sixteen: a M, residual SD, r |
| :--- | ---: | :--- | :--- |
| Eighth, assignment | 23 | 3.2, 4.14, 0.40 | 3.0, 2.42, 0.58 |
| Eighth, mirrored | 23 | 5.0, 5.93, 0.49 | 4.8, 3.23, 0.70 |
| First mapping, assignment | 23 | 3.7, 5.11, 0.47 | 4.9, 3.16, 0.75 |
| First mapping, mirrored | 23 | 1.3, 4.11, 0.20 | 2.8, 2.65, 0.55 |
| All four | 92 | 5.9, 7.14, 0.56 | 6.9, 5.15, 0.74 |

- **Within a start the mean lies closer to a line than one read does**: a residual SD of 2.42 to 3.23 against 4.11 to 5.93, and r of 0.55
  to 0.75 against 0.20 to 0.49.
- **Pooled over the starts the bands stay wide**, 9.7 to 22.0 of 64 against one read's 12 to 31, and neighbouring bands overlap by up to
  17.9. Each start's line sits at its own height.
- **More is left than the reads explain.** A residual of 2.42 to 3.23 stands against an SE of about 0.9 for a mean of sixteen. A row's
  residual from its start's line, averaged over the starts, is +4.3 on plain with the pair's patterns and +4.8 on the twin with them at
  block 12, and −4.3 on plain before, −3.7 on the twin before and −4.1 on plain halfway on A's patterns at block 24.

## 3. What a path spreads

**Eight more paths** for the pair's blocks, on Awake halfway and Plain halfway on A's patterns. Path `j` runs the pair's 12 blocks from
trial 4 096 × `j`, makes its gap, and runs 12 more from that trial again, as 016's second stretch began at 0 again. Paths 1 to 4 change the
stimuli alone. Paths 5 to 8 also run the pair on the drive's seed 3 + `j`. The gaps run under the drive as it was, and a night takes
A's patterns from the start image, as 016's did. Path 0 is 016's. A's sixteen reads at blocks 12 and 24, `<image>` `half path <j> awake
blocks <b>` or `half path <j> plain-first blocks <b>`, and for path 0 `half awake` or `half plain-first`.

**Every path reads the same sixteen streams**, so each cell below splits nine paths by sixteen streams: a count is the mean, a path, a
stream and the rest. **A read** is the SD within a path, the variance averaged over the nine. Paths whose images stand near one tick share
part of each stream, so a path's SD taken from the spread of its means alone, past `read²/16`, would read too low. It comes from this
split, and no cell's estimate of a path or a stream came out negative.

| Start | Row, block | Mean | Paths' means, least to greatest | A read | A path | A stream | The rest | Margin, M, mean and SD |
| :--- | :--- | ---: | :--- | ---: | ---: | ---: | ---: | :--- |
| Eighth, assignment | Awake halfway, 12 | 45.5 | 44.4 to 47.1 | 3.84 | 0.71 | 3.01 | 2.39 | 1.23, 0.06 |
| Eighth, mirrored | Awake halfway, 12 | 34.9 | 32.4 to 36.6 | 3.54 | 1.31 | 2.60 | 2.41 | 0.36, 0.06 |
| First mapping, assignment | Awake halfway, 12 | 42.7 | 41.5 to 45.3 | 4.41 | 1.04 | 3.65 | 2.49 | 0.48, 0.08 |
| First mapping, mirrored | Awake halfway, 12 | 48.5 | 46.3 to 50.8 | 3.38 | 1.28 | 2.25 | 2.53 | 1.53, 0.09 |
| Eighth, assignment | Plain halfway, A's, 12 | 47.9 | 46.4 to 50.0 | 3.43 | 1.16 | 2.81 | 1.97 | 2.44, 0.06 |
| Eighth, mirrored | Plain halfway, A's, 12 | 38.4 | 36.3 to 39.9 | 2.49 | 1.30 | 1.62 | 1.89 | 1.66, 0.07 |
| First mapping, assignment | Plain halfway, A's, 12 | 49.4 | 48.3 to 51.7 | 2.80 | 0.94 | 2.24 | 1.67 | 1.73, 0.10 |
| First mapping, mirrored | Plain halfway, A's, 12 | 50.6 | 48.4 to 52.7 | 2.50 | 1.18 | 1.81 | 1.72 | 2.59, 0.07 |
| Eighth, assignment | Awake halfway, 24 | 45.1 | 43.1 to 47.8 | 3.37 | 1.46 | 2.62 | 2.12 | 1.13, 0.07 |
| Eighth, mirrored | Awake halfway, 24 | 32.0 | 30.4 to 35.6 | 3.72 | 1.44 | 2.42 | 2.82 | 0.23, 0.07 |
| First mapping, assignment | Awake halfway, 24 | 37.9 | 35.6 to 40.4 | 4.11 | 1.57 | 3.55 | 2.07 | 0.32, 0.10 |
| First mapping, mirrored | Awake halfway, 24 | 45.6 | 44.4 to 46.8 | 3.29 | 0.58 | 2.62 | 1.98 | 1.27, 0.06 |
| Eighth, assignment | Plain halfway, A's, 24 | 45.2 | 42.9 to 46.9 | 3.37 | 1.13 | 2.18 | 2.57 | 2.28, 0.05 |
| Eighth, mirrored | Plain halfway, A's, 24 | 32.9 | 30.5 to 35.4 | 4.05 | 1.47 | 2.83 | 2.90 | 1.44, 0.07 |
| First mapping, assignment | Plain halfway, A's, 24 | 42.7 | 41.0 to 44.6 | 3.92 | 1.09 | 2.79 | 2.75 | 1.47, 0.10 |
| First mapping, mirrored | Plain halfway, A's, 24 | 45.7 | 43.6 to 47.2 | 3.54 | 1.02 | 2.38 | 2.63 | 2.32, 0.06 |

Pooled, variances averaged. The two smaller rows have five paths each:

| Paths, the 16 cells pooled | A read | A path | A stream | The rest |
| :--- | ---: | ---: | ---: | ---: |
| All nine paths | 3.52 | 1.20 | 2.64 | 2.34 |
| Paths 0 to 4, the stimuli alone | 3.62 | 1.11 | 3.17 | 1.76 |
| Paths 0 and 5 to 8, stimuli and drive | 3.41 | 1.21 | 2.23 | 2.58 |

- **A path's SD is about a third of a read's**, at 1.20 against 3.52, by cell 0.58 to 1.57.
- **That figure has a range.** The 16 cells are not sixteen independent estimates, Awake and Plain at block 12 sitting on one path. Taken
  at 32 degrees of freedom, the four starts as independent, a rough 95 % range on a path's SD is 0.90 to 1.66.
- **A path's own drive moves the path's SD little and the split's other two more**: 1.11 against 1.21 for a path, on five paths each, 3.17
  against 2.23 for a stream, and 1.76 against 2.58 for the rest. Why was not read.
- **A stream's SD, 2.64, is larger than the rest's, 2.34.** So sixteen streams on nine paths are not 144 independent reads. The rest also
  holds whatever a stream does on one path and not on another.
- **The margin's SD over paths is 0.05 to 0.10 M.**

Plain halfway on A's patterns less Awake halfway on each path, a path's mean of sixteen against the same path's. The SE keeps the streams,
whose shared part the SD over paths does not see: `√((Awake's stream² + Plain's stream²)/16 + (SD over the paths)²/9)`, each stream the
cell's in the table above.

| Block | Start | Mean of the nine differences | SD over the paths | SE, streams kept | Margin, M, mean | Least to greatest |
| ---: | :--- | ---: | ---: | ---: | ---: | :--- |
| 12 | Eighth, assignment | +2.4 | 0.69 | 1.05 | +1.21 | 1.19 to 1.23 |
| 12 | Eighth, mirrored | +3.5 | 0.69 | 0.80 | +1.30 | 1.27 to 1.33 |
| 12 | First mapping, assignment | +6.7 | 0.84 | 1.11 | +1.25 | 1.01 to 1.31 |
| 12 | First mapping, mirrored | +2.1 | 0.64 | 0.75 | +1.06 | 1.03 to 1.12 |
| 24 | Eighth, assignment | +0.0 | 1.50 | 0.99 | +1.14 | 1.03 to 1.25 |
| 24 | Eighth, mirrored | +0.9 | 2.88 | 1.34 | +1.21 | 1.03 to 1.30 |
| 24 | First mapping, assignment | +4.7 | 1.62 | 1.25 | +1.15 | 0.84 to 1.25 |
| 24 | First mapping, mirrored | +0.0 | 1.30 | 0.98 | +1.05 | 0.92 to 1.13 |

- **Right after the gap the two rows stand on one path.** The SD over paths is 0.64 to 0.84, about what the rest alone gives, 2.16 × √2 / 4 =
  0.76, 2.16 being block 12's rest pooled. So the path cancels there.
- **Twelve blocks on it no longer does.** The SD over paths is 1.30 to 2.88. What parts them was not read.
- **A read does not pair.** The two rows stand 2 262 to 3 268 ticks apart from the eighth image and 17 174 to 17 964 from a first mapping.
  Read by read on one path their counts correlate, by start and block, at −0.15 to +0.18, so the SD of a read's difference is 3.97 to
  6.09, about √2 of a read's.

## What it takes to see 8 of 64

Two arms, `R` reads on each of `P` paths, from the pooled split, unrounded: a read 3.525, a path 1.197, a stream 2.638, the rest 2.338. **This is worked out from the split, not run.** With
every read on a stream of its own, the SE of the difference is `√(2 × (path²/P + read²/(P × R)))`. With one set of `R` streams on all of an
arm's paths, as here, it is `√(2 × (path²/P + stream²/R + rest²/(P × R)))`. Both take the two arms' reads to share no stream near one tick,
and on one path they are one figure.

| Reads on a path | Paths an arm | SE, every read its own stream | 8 of 64 in SE | SE, one set of streams | 8 of 64 in SE |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1 | 5.26 | 1.5 | 5.26 | 1.5 |
| 1 | 2 | 3.72 | 2.1 | 4.56 | 1.8 |
| 1 | 4 | 2.63 | 3.0 | 4.17 | 1.9 |
| 2 | 1 | 3.91 | 2.0 | 3.91 | 2.0 |
| 2 | 2 | 2.77 | 2.9 | 3.34 | 2.4 |
| 2 | 4 | 1.96 | 4.1 | 3.01 | 2.7 |
| 4 | 1 | 3.01 | 2.7 | 3.01 | 2.7 |
| 4 | 2 | 2.13 | 3.8 | 2.51 | 3.2 |
| 4 | 4 | 1.51 | 5.3 | 2.21 | 3.6 |
| 8 | 1 | 2.44 | 3.3 | 2.44 | 3.3 |
| 8 | 2 | 1.73 | 4.6 | 1.96 | 4.1 |
| 8 | 4 | 1.22 | 6.5 | 1.67 | 4.8 |
| 16 | 1 | 2.10 | 3.8 | 2.10 | 3.8 |
| 16 | 2 | 1.49 | 5.4 | 1.63 | 4.9 |
| 16 | 4 | 1.05 | 7.6 | 1.33 | 6.0 |

- **One read against one read, 016's reading, leaves 8 of 64 at 1.5 SE.** Its rows with no night stood up to 12 apart at one margin.
- **Three SE takes eight reads on one path, four on each of two paths, or one on each of four.** On one set of streams four on each of two
  stand at 3.2 and one on each of four at 1.9.
- **Two SE takes two reads on one path, or one on each of two**, the last at 1.8 on one set of streams.
- **Sixteen reads on one path, this note's form, leaves 8 of 64 at 3.8 SE.** At the widest cell's read, 4.41, it is 3.5, and over a path's
  range above 3.0 to 4.5.
- **Reads alone stop at the path.** On one path no number of reads takes the SE under 1.197 × √2 = 1.69, and 8 of 64 clears that at 4.7.
- **Right after a gap on a path both arms share, the path's term goes.** Sixteen reads a side at block 12's pooled read, 3.36, leave an SE
  of 1.19, and 8 of 64 at 6.7.
- **A read is one block on a fork, and a path is 24 blocks and a gap.**
- **All of it is seed 22.** A difference between seeds is a term this table does not hold.

## What it costs

The harness's trial took 1 441 s, the four tests of questions 1 and 2 1 972 to 3 286 s, and the four of question 3 2 670 to 2 922 s. That is
nine jobs, one past the 8 this note was scoped to, the trial being the ninth, and none near a job's 240 minutes. Nothing was cut: sixteen
reads held, and the pair's and both's nights and the first mapping ran. That is 4 504 reads on forks, 8 of them the trial's timing.

## What it suggests

**A count on this slope is partly its stream.** Two counts of one block on two streams differ by an SD of about 5 with nothing changed. Near one
tick two counts on one stream share part of that, and 2 261 ticks or more apart they do not. So whether a comparison carries that part
depends on where its two arms' ticks lie.

**Reads and paths do different work.** Reads narrow a mean as far as the path's SE, on streams not shared. Right after a gap a shared path
cancels. Twelve blocks on it does not.

Questions this suggests for a later exploration or a round:
- what a stream carries: reads on one drive at shifted ticks, and on one stimulus stream, so that a tick's part and the stimuli's come
  apart, and where between 729 and 2 261 ticks it stops;
- what a seed adds past a path, the same split on seeds 23 and 24;
- what the count follows past the margin, its residual being about three times a mean's SE;
- why a withheld read moves a coupling at all, and why a first mapping's inhibitory sum falls 4 % through 16 withheld blocks where the
  eighth image's stands.

## What did not run

- **Seeds 23 and 24**, so nothing here says what a seed adds.
- **Any share but 26 of 51**, an agreeing pair, or the `Kept` cut.
- **Reads at a shifted tick.** Every read starts at its image's tick, so a stream's tick and its drive's seed were not parted.
- **A pair of images 730 to 2 260 ticks apart, or 3 500 to 16 928.**
- **A stream of its own on each path.** Nine paths read one set of sixteen, so the table's column of streams of their own is worked out alone.
- **Question 3 on any row but Awake halfway and Plain halfway on A's patterns**, or a path that changes the gap's own drive.
- **Blocks on one engine on any image but section 1's three, or on any stream but 0.**
- **More than 16 reads, or a read longer than a block's 64 trials.**
- **The pair's count through question 2's forms or on question 3's paths.** Question 1 read it on the start, 12 and 24 blocks, and 016's
  reads stand beside question 2's forms.
- **A rerun of the two start reads under 016's groups**, so their cause comes from the code alone.
- **Question 1 from the eighth image and the assignment at `de34138`.** Its reads are the trial's, at `d261f6a`, and nothing in them changed.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_what_a_read_spreads_explore` | [41325148027](https://github.com/Menteion/Menteion/actions/runs/41325148027) | `d261f6a` | 1441 |
| `what_a_read_spreads_of_the_eighth_image_on_seed_22_from_the_assignment_explore` | [41328059162](https://github.com/Menteion/Menteion/actions/runs/41328059162) | `de34138` | 1972 |
| `what_a_read_spreads_of_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [41332466567](https://github.com/Menteion/Menteion/actions/runs/41332466567) | `de34138` | 3020 |
| `what_a_read_spreads_of_a_first_mapping_on_seed_22_from_the_assignment_explore` | [41332466567](https://github.com/Menteion/Menteion/actions/runs/41332466567) | `de34138` | 3286 |
| `what_a_read_spreads_of_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [41332466567](https://github.com/Menteion/Menteion/actions/runs/41332466567) | `de34138` | 3156 |
| `what_a_path_spreads_of_the_eighth_image_on_seed_22_from_the_assignment_explore` | [41328059162](https://github.com/Menteion/Menteion/actions/runs/41328059162) | `de34138` | 2670 |
| `what_a_path_spreads_of_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [41332466567](https://github.com/Menteion/Menteion/actions/runs/41332466567) | `de34138` | 2705 |
| `what_a_path_spreads_of_a_first_mapping_on_seed_22_from_the_assignment_explore` | [41332466567](https://github.com/Menteion/Menteion/actions/runs/41332466567) | `de34138` | 2811 |
| `what_a_path_spreads_of_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [41332466567](https://github.com/Menteion/Menteion/actions/runs/41332466567) | `de34138` | 2922 |
