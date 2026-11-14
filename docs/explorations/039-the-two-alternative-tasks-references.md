---
status: exploratory
date: 2026-11-14
---

# 039: The two-alternative task's references

**Branch**: `explore/039-the-two-alternative-tasks-references`
**Commit**: `114ac85`, which adds the harness, on `58a9e52`, `main` at brief 088's merge. The branch changes nothing under `src/` or
`crates/`, so the engine is `main`'s.
**Runs**: [43597619181](https://github.com/Menteion/Menteion/actions/runs/43597619181) (three jobs, one a network) and
[43595937012](https://github.com/Menteion/Menteion/actions/runs/43595937012) (the harness's trial), both at `114ac85` and `scope=arms`

[ADR-0255](../adr/0255-a-search-by-forks-and-the-two-alternative-tasks-references.md) ordered this exploration, the second step of
[ADR-0254](../adr/0254-the-design-review-planned.md)'s first order: a floor and a ceiling for the two-alternative task, set beside the
engine. Brief 088 built the search it uses ([ADR-0256](../adr/0256-a-search-by-forks-built.md)). On H-42's three networks, seeds 22, 23 and
24 at 1 024 units, under the running configuration, on H-46's first schedule from the assignment and from the mirrored assignment, it asks:

1. how fast and how well two floors learn, simple learners that see only the readouts' counts, and whether they reverse;
2. how far the ceiling reaches, the search by forks over the four stimulus–readout couplings within H-25's 1.30, and whether its best is at
   a corner of the box;
3. how each stands beside the engine.

It writes no prediction and decides nothing. **Every figure here is exploratory**: from one run of each test on the repository's AArch64
host. No SE is claimed. As a scale for every count here, [exploration 017](017-what-a-read-spreads.md) read the spread of one block's read
on seed 22 on its own branch: an SD of 0.63 to 1.21 correct trials of 64 near the top and 3.20 to 4.50 on the slope, 3.52 pooled. A score
per 64 trials is a sum over its blocks over their count, worked from the unrounded sum and rounded once, an exact half up by magnitude.

**In brief**
- **Every engine run held its pinned row.** On all three networks and from both starts the engine's row held H-46's pinned first schedule
  inside the test. Its last 128 trials stood at 122 to 128 over the 24 mappings; its first mapping passed 40 of 64 by block 1 to 7 and its
  18 reversals 9 to 13 blocks after their flips.
- **The floor on each readout unit's count learned every mapping and passed every reversal.** Its last 128 stood at 121 to 128; its first
  mapping passed 40 of 64 by block 1 to 4 and its reversals 5 to 13 blocks after their flips. Mapping by mapping it passed 14 of 24 times
  sooner than the engine, 6 at the same block and 4 later, by −6 to +2 blocks, and its last 128 stood −3 to +6 from the engine's. H-25's
  clauses 1 and 3 held on all six starts, as for the engine.
- **The floor on the two summed counts held clause 1 on no start.** Of its 24 mappings 4 reached 80 of their last 128, its last 128 stood at
  54 to 108, and on seed 22 from the assignment it passed 40 of 64 in no block. Clause 3 held by its rule on three starts, seed 22 from the
  mirrored assignment, seed 24 from the assignment and seed 24 from the mirrored assignment. Of the 9 blocks in which those starts'
  reversals first passed 40 of 64, 7 counted exactly 40, each what answering one readout on every trial gives in that block; the other 2,
  mapping 3 on seed 24 from the assignment and mapping 3 on seed 24 from the mirrored assignment, each counted 51. In 62 to 114 of each
  start's 120 blocks its count equalled what one answer on every trial gives; the readouts' own selection's count did so in 0 to 12.
- **The readouts' own selection**, the floor unmoved, reached 80 of the last 128 in 6 of 24 mappings, 14 to 108 in all. Under the
  assignment's mapping its last 128 stood at 44 to 62, 37 to 44 and 93 to 108 on seeds 22, 23 and 24, under the mirrored mapping at 54 to
  74, 67 to 83 and 14 to 26. It tied in 729, 649 and 469 of the 7 680 trials on seeds 22, 23 and 24, each tie an error under either mapping.
- **The ceiling within H-25's bound.** On every network and mapping the best of the sixteen corners on streams 1 to 3 was the one that sets
  each stimulus's answer coupling at 332 and the other at 128: 55.3 to 63.7 correct trials per 64 there and 58.0 to 64.0 on streams 4 to 6.
  The search's best reached 43.7 to 63.3 per 64 on its streams and 44.3 to 62.3 on the unseen ones, below the corners' best on both in all
  six searches, with 0 to 2 of its four genes at a bound. No read of the 2 133 left a coupling past 1.30 of the start image's.
- **Beside the engine**, per 64 trials under each mapping: the engine's last 128 61.0 to 64.0, the floor on each readout unit's count 60.5
  to 64.0, the floor on the two summed counts 27.0 to 54.0 and the readouts' own selection 7.0 to 54.0.
- **With the reward withheld no coupling moved.** In each withheld run the excitatory sum and the four couplings stood at every block's end
  where they started, and no read moved a coupling. The inhibitory sum went from 100.0 % of the start image's, as each job's derivation
  prints it (`PIN quiet`), at the first block's end to 73.8 to 74.1 % at the run's end.

## What ran

**The networks and the engine.** Each job runs one network: H-42's derivation run again and held to its pinned tables (`probe_made`), H-46's
groups read from H-42's image (`zero_groups`), and the start image, H-42's. The engine is H-46's arm on its first schedule, H-20's: the
first mapping for 24 blocks, then three reversals of 32, 120 blocks from the assignment and 120 from the mirrored assignment, run as
`zero_short` runs it, the zero sum's groups set as H-46's arm sets them. Each start's row held H-46's pinned row of its network and start
inside the test (`ZERO_ROWS`), field for field, among them the trace and the reward values by their FNV-1a hashes, the image by its CRC-64,
each mapping's last 128 and the block it passed 40 of 64. Had one not held, the job would have stopped there.

**The floors' features.** One run of 120 blocks a network, on the start image under the running configuration with the reward withheld, the
groups set as for the engine, on H-46's trials and drive, with no flip: with the reward withheld only a trial's correctness reads the
mapping. A check held, trial by trial, that it presented each engine run's stimuli. On each trial it kept the two readouts' summed window
counts and each of the 918 readout units' window count, read by `Readout::count_window`'s rule; a check held that the units' counts sum to
the readouts'. A check held that no stimulus unit is in a readout's set and no unit is in both readouts.

**The floors.** Two learners on each start, each reading every trial's features in order, one the two summed counts and one each readout
unit's count. Each keeps a vector of `i16` weights for each answer, at first 16 384 on its own readout's features and 0 elsewhere. It
answers the larger of `ValueCritic::value_q16` over its two vectors at the critic's constants, a shift of 9 and a scale of 2 on all three
networks, a tie being no answer and an error. After an answer it moves that answer's weights by `ValueCritic::step` on the error of
`REWARD_Q16` for a correct answer or its negation for a wrong one, less that answer's value. A third reading, **the readouts' own
selection**, is a floor that never moves: a check held, trial by trial, that it answers as the withheld run's readouts selected. **A rail**
is either end of the `i16`.

**The ceiling.** The search by forks over four genes, the factors of the four couplings A→R0, A→R1, B→R0 and B→R1 in 1/256, from 128 to 332
and starting at 256, at P 16, G 10, E 4, M 16 and S 3, its seed `0x0039025502560088` fixed before the trial, the same for both mappings.
Each candidate is written into a fork of the start image by brief 088's writer, the zero sum's groups set from the written engine's
`coupling_sum`. Its score on a stream is the correct trials of one block with the reward withheld, that block's first trial 64 × s and its
drive's seed the schedule's plus s (`nights_read_from`). The search's streams are 1 to 3, its unseen ones 4 to 6. One read counts its
correct trials under both mappings, so a read is kept by its genes and stream and serves both searches and the corners. At each read's end
the four couplings are read against the start image's, past 1.30 as `coupling × 100 > image × 130`. The search runs for the assignment's
mapping and apart for the mirrored one's, each on the start image. The box's sixteen corners are read on streams 1 to 6.

**The trial and the jobs.** The trial, on seed 22, made the network ready in 151 s, then ran the engine from the assignment for 120 blocks
in 724 s beside one generation of the assignment's search, 51 reads in 305 s, 5.971 s a read, two engines at a time. It projected a job at
most 4 738 s, 79.0 minutes, against ADR-0179's 144, so G and P stood at ADR-0255's. The projection is its readiness and the engines' 120
blocks, then half the second part's work, the engine's time again for the withheld run and 798 reads (375 a search, less the 48 of
generation 0 that the second shares, and 96 for the corners), and half its longest piece, one search's 375 reads. Its withheld run and
floors over two blocks ran their checks. It took 886 s as `arm.tsv` records. The three jobs ran in one dispatch, one a network, each the
engine from both starts two at a time, then the two searches, the withheld run and the corners two at a time, then the floors. They took
3 289, 3 356 and 3 332 s on seeds 22, 23 and 24, 54.8, 55.9 and 55.5 minutes; each engine's 120 blocks 656 to 720 s, each withheld run 667
to 698 s, each search 1 623 to 1 926 s and the corners 546 to 560 s, as the jobs print. Each job kept 723, 705 and 705 reads, on seed 22,
seed 23 and seed 24.

**Reading the logs.** Every line of a job but the search's own is `DUMP e039 zero<seed> …`, the job's network named by H-46's arm's name;
the trial's are `DUMP e039 trial zero22 …` and `DUMP search e039 trial zero22 …`. An engine's row is `… <start> engine PIN row`, and each
learner's reading `… <start> <learner> curve […] ties […] last 128 […] crossed […] clause 1 … clause 3 …`, `<learner>` being `engine`,
`floor summed`, `floor units` or `own selection` and `crossed` the blocks `crossings` counts to 40 of 64 in each mapping. A floor's rails
are `… floor <summed|units> rail: …`. A read is `… read genes […] stream <s> correct [<assignment>, <mirrored>] … past […]`; the search's
own lines are `DUMP search e039 zero<seed> <mapping> …`, its best `… best g <generation> i <index> genes […] scores […] unseen […] sum
<unseen sum>`; a corner is `… corner <c> genes […] assignment […] mirrored […]`, and each mapping's best corner `… <mapping> corners best
…`.

## 1. The engine and the floors, block by block

Each mapping's last 128 trials and the blocks it took to pass 40 of 64, counted from the run's first block for mapping 1 and from each flip
for mappings 2 to 4 (`crossings`); "none" where no block of the mapping passed. Clause 1 is every mapping's last 128 at 80 or more; clause 3
every reversal passing within 23 blocks of its flip.

| Network | Start | Learner | Last 128, mappings 1 to 4 | Blocks to 40 of 64, mappings 1 to 4 | Clause 1 | Clause 3 |
| :--- | :--- | :--- | :--- | :--- | :---: | :---: |
| seed 22 | from the assignment | the engine, H-46's arm | 124, 124, 123, 126 | 4, 10, 12, 10 | yes | yes |
| seed 22 | from the assignment | the floor on the two summed counts | 67, 67, 67, 71 | none, none, none, none | no | no |
| seed 22 | from the assignment | the floor on each readout unit's count | 123, 123, 126, 126 | 4, 10, 11, 11 | yes | yes |
| seed 22 | from the assignment | the readouts' own selection, unmoved | 53, 74, 62, 54 | none, 24, none, none | no | no |
| seed 22 | from the mirrored assignment | the engine, H-46's arm | 124, 122, 126, 126 | 2, 13, 11, 11 | yes | yes |
| seed 22 | from the mirrored assignment | the floor on the two summed counts | 61, 64, 55, 54 | none, 22, 20, 10 | no | yes |
| seed 22 | from the mirrored assignment | the floor on each readout unit's count | 121, 123, 126, 127 | 2, 12, 13, 12 | yes | yes |
| seed 22 | from the mirrored assignment | the readouts' own selection, unmoved | 64, 44, 54, 60 | none, none, 10, none | no | no |
| seed 23 | from the assignment | the engine, H-46's arm | 125, 123, 127, 125 | 3, 10, 10, 9 | yes | yes |
| seed 23 | from the assignment | the floor on the two summed counts | 75, 64, 75, 74 | 20, none, 6, 31 | no | no |
| seed 23 | from the assignment | the floor on each readout unit's count | 127, 127, 128, 127 | 2, 7, 8, 10 | yes | yes |
| seed 23 | from the assignment | the readouts' own selection, unmoved | 38, 83, 40, 67 | none, 4, none, 6 | no | no |
| seed 23 | from the mirrored assignment | the engine, H-46's arm | 127, 123, 122, 128 | 1, 9, 9, 11 | yes | yes |
| seed 23 | from the mirrored assignment | the floor on the two summed counts | 75, 64, 78, 74 | 1, 22, 6, 31 | no | no |
| seed 23 | from the mirrored assignment | the floor on each readout unit's count | 125, 127, 128, 127 | 1, 7, 9, 9 | yes | yes |
| seed 23 | from the mirrored assignment | the readouts' own selection, unmoved | 80, 37, 75, 44 | 12, none, 1, none | no | no |
| seed 24 | from the assignment | the engine, H-46's arm | 125, 128, 126, 124 | 1, 11, 12, 11 | yes | yes |
| seed 24 | from the assignment | the floor on the two summed counts | 96, 64, 108, 54 | 1, 22, 3, 10 | no | yes |
| seed 24 | from the assignment | the floor on each readout unit's count | 125, 128, 127, 127 | 1, 5, 7, 8 | yes | yes |
| seed 24 | from the assignment | the readouts' own selection, unmoved | 93, 26, 97, 14 | 1, none, 1, none | no | no |
| seed 24 | from the mirrored assignment | the engine, H-46's arm | 125, 125, 127, 127 | 7, 12, 12, 11 | yes | yes |
| seed 24 | from the mirrored assignment | the floor on the two summed counts | 96, 64, 108, 54 | 3, 22, 3, 10 | no | yes |
| seed 24 | from the mirrored assignment | the floor on each readout unit's count | 126, 128, 128, 128 | 4, 6, 7, 8 | yes | yes |
| seed 24 | from the mirrored assignment | the readouts' own selection, unmoved | 25, 96, 23, 108 | none, 1, none, 1 | no | no |

Each learner's correct trials in each block, a mapping between bars: 24 blocks, then three of 32.

```text
seed 22:
  from the assignment           engine        | 32 37 33 48 49 58 59 60 60 59 56 61 59 62 62 62 62 60 62 64 60 62 62 62 |  7  3  5 13 14 20 28 26 34 42 43 46 50 49 54 54 52 58 57 58 58 61 64 61 62 64 63 62 63 64 63 61 |  2  6  6 11 17 11 22 30 26 31 39 49 45 51 47 55 60 60 60 58 58 59 61 58 62 64 60 63 59 63 61 62 |  2  6  6 14 15 23 26 27 38 41 47 40 53 48 58 54 62 56 58 62 64 61 62 63 60 64 64 63 63 63 64 62
  from the assignment           floor summed  | 35 29 27 36 35 28 33 32 31 36 31 32 31 31 27 29 31 26 33 30 34 35 32 35 | 29 33 36 32 34 34 35 29 32 28 29 31 29 30 32 29 30 30 32 31 28 24 34 33 31 38 34 30 36 37 36 31 | 37 39 28 29 32 29 32 30 34 28 37 30 26 34 35 32 28 38 30 24 33 31 27 37 29 37 33 38 34 36 31 36 | 36 32 33 32 26 30 33 30 31 38 31 30 36 29 38 35 33 32 36 35 34 31 33 30 23 36 35 31 38 34 38 33
  from the assignment           floor units   | 37 38 35 48 43 52 48 49 52 59 47 59 57 59 56 58 55 54 59 60 61 61 61 62 |  4  4 10 17 17 23 25 36 34 42 48 45 48 43 50 57 53 60 56 61 55 60 63 62 62 63 63 62 62 64 61 62 |  0  5  3  7 10 16 22 24 32 23 40 46 47 52 53 48 55 58 56 55 55 61 62 59 61 60 61 63 60 64 62 64 |  2  4  2 10  8 10 13 15 27 23 43 36 43 49 50 49 57 57 55 58 58 60 58 58 59 63 64 63 63 63 64 62
  from the assignment           own selection | 31 28 24 28 29 35 28 29 35 22 17 35 30 23 30 28 27 26 24 31 36 34 28 25 | 34 26 27 33 31 30 31 25 28 32 36 28 29 24 21 35 28 36 22 34 30 23 38 40 27 33 35 22 29 30 40 34 | 24 32 31 30 25 24 29 27 26 21 29 33 29 27 26 28 23 29 22 20 34 27 27 26 28 29 30 31 29 33 28 34 | 24 36 32 27 29 33 29 20 35 27 35 28 33 31 31 28 30 33 32 31 31 27 29 34 27 37 34 31 35 33 26 28
  from the mirrored assignment  engine        | 30 40 42 46 47 46 57 51 55 60 61 57 61 63 61 60 60 60 62 61 58 58 62 62 |  1  2  9 10 15 16 17 27 28 31 33 38 46 53 53 54 59 53 55 57 61 58 60 57 57 61 59 62 60 64 60 62 |  1  3  6  9 15 22 21 21 30 39 44 41 53 60 59 59 61 59 63 61 60 59 58 57 62 61 61 62 62 62 63 63 |  7  7  9  7 15 11 19 24 22 39 44 48 50 59 61 64 61 63 60 62 63 64 62 60 60 64 64 64 62 62 62 64
  from the mirrored assignment  floor summed  | 28 34 33 36 35 21 30 32 34 28 33 32 32 33 37 35 33 38 31 34 30 29 32 29 | 35 31 28 32 30 30 29 35 32 36 35 32 32 30 32 35 34 34 32 33 37 40 30 31 33 27 29 34 27 44 30 34 | 25 26 35 35 31 35 32 34 30 36 27 34 34 30 29 31 35 26 34 40 30 33 37 27 35 27 31 26 30 28 33 22 | 36 32 33 32 27 32 33 28 27 40 33 32 36 29 36 36 34 29 27 31 35 31 37 31 30 39 28 35 28 31 24 30
  from the mirrored assignment  floor units   | 33 42 41 45 49 44 52 52 57 61 60 56 60 60 60 56 60 61 58 59 59 59 61 60 |  2  9  6 14 20 21 29 33 36 35 34 46 48 57 53 59 55 54 57 58 62 58 59 58 58 59 59 64 61 63 59 64 |  1  2  3  6  9 18 16 24 27 37 32 34 40 50 54 52 53 53 57 57 54 60 59 61 63 62 61 64 62 64 64 62 |  3  4  3  1  2 11 12 23 19 32 28 42 39 44 48 53 55 52 56 57 55 62 58 61 63 63 60 63 63 63 63 64
  from the mirrored assignment  own selection | 25 29 34 30 28 24 27 32 23 35 36 26 31 33 30 31 30 32 32 28 24 24 33 31 | 27 32 29 27 31 32 32 28 31 25 22 32 27 34 31 22 27 27 32 24 30 36 21 20 29 23 25 33 28 27 20 24 | 32 30 27 24 33 32 30 33 30 40 28 24 26 29 35 31 28 28 35 36 28 29 31 32 33 31 31 26 25 27 29 25 | 33 22 25 27 29 24 27 37 23 31 28 30 26 27 25 30 29 26 28 28 23 31 26 27 32 24 21 30 23 24 34 26
seed 23:
  from the assignment           engine        | 21 38 41 43 45 54 56 56 56 61 59 60 62 64 61 60 63 60 63 64 64 64 62 63 |  1  2  6 16 17 25 27 32 38 46 48 51 57 59 61 61 61 61 59 60 56 62 62 61 63 63 60 63 63 63 64 59 |  0  7  8 19 16 19 26 39 34 48 57 59 55 63 61 59 63 64 63 62 63 62 62 63 64 64 63 64 64 62 64 63 |  0  2  7 19 18 22 31 35 44 55 58 58 62 59 62 62 62 61 62 62 64 64 61 60 63 63 64 64 64 64 64 61
  from the assignment           floor summed  | 18 31 31 28 30 36 30 32 34 28 33 32 32 33 37 35 33 38 37 44 39 37 33 42 | 23 28 34 32 34 34 35 29 32 28 29 32 32 34 32 29 30 30 32 31 27 24 34 33 31 37 35 30 37 20 34 30 | 39 38 29 29 33 44 42 39 45 42 32 30 46 38 48 40 39 45 29 45 37 40 32 45 34 43 40 41 44 42 34 41 | 24 27 21 33 36 32 31 36 37 24 31 32 28 35 28 28 30 35 37 33 29 33 27 33 34 25 36 29 36 33 40 34
  from the assignment           floor units   | 26 40 47 46 46 52 57 56 60 60 61 59 62 61 62 63 64 61 61 63 63 64 63 64 |  2  4 11 23 29 37 44 50 58 57 62 55 59 63 62 59 63 63 63 64 64 64 64 63 64 64 63 64 64 64 64 63 |  0  6  5 12 11 15 39 45 45 60 57 61 62 63 63 64 64 63 64 64 64 61 63 64 64 64 64 63 64 63 64 64 |  0  0  4  9 11 17 26 32 39 50 57 58 60 57 64 63 63 63 63 62 64 64 63 64 64 64 64 63 64 64 64 63
  from the assignment           own selection | 20 31 22 20 21 20 21 24 22 22 22 21 22 18 16 21 19 15 20 17 24 25 22 16 | 38 37 33 46 41 35 37 36 36 36 39 37 42 41 39 34 45 38 34 41 27 33 43 44 36 43 30 43 37 34 40 43 | 16 22 21 29 13 13 17 26 13 18 25 28 16 25 15 18 22 25 30 19 24 19 25 19 26 17 22 18 21 20 21 19 | 36 34 37 39 39 40 37 36 33 42 41 37 41 34 42 35 35 38 35 37 38 39 43 37 37 42 41 37 51 37 32 35
  from the mirrored assignment  engine        | 41 39 52 59 62 59 58 57 62 61 62 60 62 64 62 63 64 61 61 64 63 62 63 64 |  3  4 12 13 23 25 26 33 40 41 48 49 56 57 62 61 58 64 62 63 64 59 61 61 61 63 63 63 64 60 63 60 |  3  6  9  7 24 30 30 28 44 40 45 49 59 55 62 60 63 63 61 63 63 62 63 63 63 63 62 63 63 62 61 61 |  3  3 10  8 12 19 25 34 36 39 47 57 57 60 60 62 62 62 63 62 63 63 63 63 64 64 62 63 63 62 64 64
  from the mirrored assignment  floor summed  | 44 31 33 43 37 39 38 40 36 38 39 39 36 41 43 37 36 41 39 45 36 36 33 42 | 23 29 28 31 28 30 29 35 32 36 35 32 32 30 32 35 34 34 32 33 37 40 30 31 33 27 29 34 27 44 30 34 | 25 26 37 35 37 43 40 33 48 43 31 31 43 36 47 43 35 43 33 45 36 38 33 41 35 44 41 40 43 42 36 42 | 24 27 21 33 36 32 31 36 37 24 31 32 28 35 28 28 30 35 37 33 29 33 27 33 34 25 36 29 36 33 40 34
  from the mirrored assignment  floor units   | 44 40 53 56 52 54 56 56 61 61 59 59 61 64 62 63 61 64 62 64 62 60 61 64 |  3  8 15 19 32 38 47 49 57 54 56 61 58 63 62 64 61 64 64 62 63 63 64 62 64 62 64 64 64 64 63 64 |  0  1  5  5 22 31 31 35 51 55 54 58 62 61 62 62 64 63 63 64 64 64 64 64 64 64 64 63 64 64 64 64 |  2  0  3  3  7 15 21 34 43 49 53 60 56 60 62 62 64 64 64 64 63 64 64 64 64 64 64 64 64 64 64 63
  from the mirrored assignment  own selection | 39 27 34 39 39 36 36 38 36 38 33 41 36 42 46 37 37 42 39 38 37 33 40 40 | 18 25 27 18 20 25 22 24 24 25 20 21 18 18 18 24 19 15 26 16 31 28 15 14 21 16 29 18 20 26 19 18 | 40 36 36 31 45 46 37 33 45 40 34 30 41 34 43 43 31 34 29 42 36 40 33 40 36 39 38 34 40 42 32 43 | 22 26 23 23 21 21 25 22 24 18 17 25 19 23 18 26 19 20 19 23 19 21 17 22 14 16 16 17 10 19 22 22
seed 24:
  from the assignment           engine        | 47 61 60 59 62 62 58 62 63 63 61 64 61 61 64 64 64 57 63 63 63 62 62 63 |  0  1  6 11 10 12 17 21 23 33 45 41 47 51 54 54 55 61 58 61 60 62 62 60 62 60 64 60 64 64 64 64 |  2  3  6 14 10 18 21 30 31 29 38 49 54 61 59 60 61 58 61 60 64 60 62 62 62 61 63 64 62 64 62 64 |  0  2  6 14 17 15 26 24 28 39 45 51 58 52 61 60 58 63 62 63 64 63 62 63 64 61 62 64 63 63 60 64
  from the assignment           floor summed  | 48 57 53 55 50 54 52 52 54 46 46 56 50 53 53 53 57 45 52 53 56 52 51 45 | 19 31 28 32 30 30 29 35 32 36 35 32 32 30 32 35 34 34 32 33 37 40 30 31 33 27 29 34 27 44 30 34 | 25 33 51 52 48 49 53 55 54 49 52 54 56 57 50 55 53 49 52 50 54 50 59 52 47 54 45 54 53 53 50 58 | 17 31 33 32 27 32 33 28 27 40 33 32 36 29 36 36 34 29 27 31 35 31 37 31 30 39 28 35 28 31 24 30
  from the assignment           floor units   | 48 57 60 61 59 60 57 61 60 59 61 63 61 60 63 61 63 61 62 63 62 63 63 62 |  4 11 14 28 40 38 51 54 54 61 63 58 60 63 62 62 60 62 63 64 63 62 63 62 64 62 64 64 64 64 64 64 |  0  2  6 18 20 31 44 52 58 56 57 59 61 62 62 64 64 63 64 63 64 63 64 64 64 63 64 64 64 64 63 64 |  0  0  2 11 23 22 33 41 49 51 59 58 61 61 63 63 64 63 64 64 63 64 64 63 64 64 64 64 63 63 64 63
  from the assignment           own selection | 47 53 49 55 48 52 49 52 53 46 44 55 49 51 47 50 53 45 51 51 57 48 48 45 | 10  7  8  9  7 11 13 10  6  8 14  8 10  6 12  7 10 11 10 14  9  7 14 10  7 13 13 10 10  8 16 10 | 50 49 53 52 48 48 51 54 55 46 50 48 51 54 51 52 53 46 49 47 48 44 54 49 50 51 46 55 52 53 43 54 |  8  8 15 13 14  7 10 10 10 14 12  8 12  7 14  9  7 11  9  9 12 10 13 12  6 13 11  9  9 14  7  7
  from the mirrored assignment  engine        | 16 15 21 21 31 28 41 42 52 56 57 59 57 63 59 60 59 59 64 63 62 63 63 62 |  6  7  6 10 16 21 26 31 34 36 35 43 41 50 55 55 58 57 61 58 61 61 62 61 63 61 58 63 64 62 62 63 |  2  2  7  6 15 18 17 15 18 30 34 40 43 42 50 58 58 62 63 62 62 62 62 62 61 63 64 63 61 62 64 63 |  0  4  4 16 17 22 21 33 38 38 49 50 57 63 57 61 61 63 63 62 61 63 63 63 64 64 62 62 63 62 63 64
  from the mirrored assignment  floor summed  | 23 31 42 54 52 52 52 52 53 47 47 56 50 54 52 53 58 44 52 52 57 52 51 45 | 19 31 28 32 30 30 29 35 32 36 35 32 32 30 32 35 34 34 32 33 37 40 30 31 33 27 29 34 27 44 30 34 | 25 33 51 52 48 49 53 55 54 49 52 54 56 57 50 55 53 49 52 50 54 50 59 52 47 54 45 54 53 53 50 58 | 17 31 33 32 27 32 33 28 27 40 33 32 36 29 36 36 34 29 27 31 35 31 37 31 30 39 28 35 28 31 24 30
  from the mirrored assignment  floor units   | 19 26 36 49 51 51 55 57 59 61 62 62 61 60 61 63 62 62 63 64 63 64 63 63 |  2  4 10 22 28 46 47 54 59 59 58 63 59 62 64 64 64 64 64 62 64 63 64 64 63 62 62 64 64 64 64 64 |  1  1  6 10 24 30 40 41 56 52 59 59 63 61 64 64 61 64 64 63 64 64 64 63 64 64 64 64 64 64 64 64 |  0  0  2  5 14 28 30 45 44 45 53 61 58 63 59 63 62 63 63 63 64 64 64 64 64 64 64 64 64 64 64 64
  from the mirrored assignment  own selection | 16  7  7  6 10  9  8 10  8 14 16  7 14 10 13 11  6 17  9  9  5  9 11 14 | 50 51 51 48 51 51 47 50 55 51 49 52 49 53 49 53 51 50 49 43 53 52 49 49 51 46 46 51 52 50 45 51 |  9 10  7  7 14 11 10  9  6 11 12 13  9  5  9  9  9 11 12 13 10 15  7 10 11 10 17  7  8  9 15  8 | 51 50 45 47 46 54 51 49 51 45 50 53 47 53 47 51 53 48 49 49 51 48 47 45 54 48 52 51 53 46 55 53
```

**How fast.** The engine's first mapping passed 40 of 64 by block 1 to 7, the floor on each readout unit's count by block 1 to 4. After the
flips the engine took 9 to 13 blocks and that floor 5 to 13. The floor on the two summed counts passed in its first mapping on four of six
starts and in 14 of its 18 reversals, 12 of them within 23 blocks.

**How well.** Over the 24 mappings the engine's last 128 stood at 122 to 128, the floor on each readout unit's count at 121 to 128, the
floor on the two summed counts at 54 to 108 and the readouts' own selection at 14 to 108.

## 2. H-25's clauses 1 and 3, and the rails

**Clauses 1 and 3** are the table's last two columns. The engine and the floor on each readout unit's count held both on all six starts. The
floor on the two summed counts held clause 1 on none and clause 3 on 3, and the readouts' own selection held neither on any.

**The floor on the two summed counts at its crossings.** Each block in which one of its reversals first passed 40 of 64, beside the count
answering one readout on every trial gives in that block, from the engine's count of A's trials there:

| Network | Start | Mapping | Blocks after the flip | Its count | One answer on every trial |
| :--- | :--- | ---: | ---: | ---: | :--- |
| seed 22 | from the mirrored assignment | 2 | 22 | 40 | readout 1, 40 |
| seed 22 | from the mirrored assignment | 3 | 20 | 40 | readout 1, 40 |
| seed 22 | from the mirrored assignment | 4 | 10 | 40 | readout 0, 40 |
| seed 23 | from the assignment | 3 | 6 | 44 | neither |
| seed 23 | from the assignment | 4 | 31 | 40 | readout 0, 40 |
| seed 23 | from the mirrored assignment | 2 | 22 | 40 | readout 1, 40 |
| seed 23 | from the mirrored assignment | 3 | 6 | 43 | neither |
| seed 23 | from the mirrored assignment | 4 | 31 | 40 | readout 1, 40 |
| seed 24 | from the assignment | 2 | 22 | 40 | readout 0, 40 |
| seed 24 | from the assignment | 3 | 3 | 51 | neither |
| seed 24 | from the assignment | 4 | 10 | 40 | readout 1, 40 |
| seed 24 | from the mirrored assignment | 2 | 22 | 40 | readout 1, 40 |
| seed 24 | from the mirrored assignment | 3 | 3 | 51 | neither |
| seed 24 | from the mirrored assignment | 4 | 10 | 40 | readout 0, 40 |

**One answer on every trial.** In each start's 120 blocks, the blocks whose count equals what answering one readout on every trial gives in
that block, under the mapping in force, from the engine's count of A's trials there. A count can equal it by chance, as the readouts' own
selection, which answers by the counts, shows:

| Network | Start | The floor on the two summed counts | The floor on each readout unit's count | The readouts' own selection |
| :--- | :--- | ---: | ---: | ---: |
| seed 22 | from the assignment | 73 | 1 | 11 |
| seed 22 | from the mirrored assignment | 114 | 2 | 12 |
| seed 23 | from the assignment | 79 | 0 | 8 |
| seed 23 | from the mirrored assignment | 62 | 0 | 5 |
| seed 24 | from the assignment | 62 | 1 | 0 |
| seed 24 | from the mirrored assignment | 63 | 0 | 0 |

**The rails.** After each step the answered vector's weights on a rail are counted, those already there included: a step with any is counted
once, and the weights are summed over the steps. A block's end counts both vectors. The floor on the two summed counts holds 4 weights and
the floor on each readout unit's count 1 836.

| Network | Start | Floor | Trials with no answer | Steps that left a weight on a rail, of the steps | Weights on a rail over the steps | Blocks that ended with a weight on a rail | Weights on a rail at the run's end |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| seed 22 | from the assignment | floor on the two summed counts | 1 | 0 of 7 679 | 0 | 0 of 120 | 0 of 4 |
| seed 22 | from the assignment | floor on each readout unit's count | 1 | 2 006 of 7 679 | 2 006 | 46 of 120 | 1 of 1 836 |
| seed 22 | from the mirrored assignment | floor on the two summed counts | 1 | 0 of 7 679 | 0 | 0 of 120 | 0 of 4 |
| seed 22 | from the mirrored assignment | floor on each readout unit's count | 1 | 2 064 of 7 679 | 2 285 | 47 of 120 | 0 of 1 836 |
| seed 23 | from the assignment | floor on the two summed counts | 0 | 23 of 7 680 | 23 | 1 of 120 | 0 of 4 |
| seed 23 | from the assignment | floor on each readout unit's count | 0 | 0 of 7 680 | 0 | 0 of 120 | 0 of 1 836 |
| seed 23 | from the mirrored assignment | floor on the two summed counts | 1 | 37 of 7 679 | 37 | 0 of 120 | 0 of 4 |
| seed 23 | from the mirrored assignment | floor on each readout unit's count | 0 | 0 of 7 680 | 0 | 0 of 120 | 0 of 1 836 |
| seed 24 | from the assignment | floor on the two summed counts | 0 | 449 of 7 680 | 449 | 66 of 120 | 1 of 4 |
| seed 24 | from the assignment | floor on each readout unit's count | 0 | 0 of 7 680 | 0 | 0 of 120 | 0 of 1 836 |
| seed 24 | from the mirrored assignment | floor on the two summed counts | 0 | 452 of 7 680 | 452 | 66 of 120 | 1 of 4 |
| seed 24 | from the mirrored assignment | floor on each readout unit's count | 0 | 12 of 7 680 | 12 | 1 of 120 | 0 of 1 836 |

On the floor on the two summed counts, steps left a weight on a rail 23 times on seed 23 from the assignment, 37 times on seed 23 from the
mirrored assignment, 449 times on seed 24 from the assignment and 452 times on seed 24 from the mirrored assignment, and none did on seed 22
from the assignment or seed 22 from the mirrored assignment. On the floor on each readout unit's count, 2 006 times on seed 22 from the
assignment, 2 064 times on seed 22 from the mirrored assignment and 12 times on seed 24 from the mirrored assignment, and none did on seed
23 from the assignment, seed 23 from the mirrored assignment or seed 24 from the assignment.

## 3. The ceiling

Each search's best, each mapping's best corner and the start, every factor at 256, the readouts' own selection on those streams; the start
was read on streams 1 to 3 alone, as a candidate of generation 0. A score per 64 is the sum of its three streams over three.

| Network | Mapping | Reference | Genes | Streams 1 to 3, per 64 | Streams 4 to 6, per 64 |
| :--- | :--- | :--- | :--- | ---: | ---: |
| seed 22 | the assignment's mapping | the search's best, scored in generation 9 | 332, 128, 142, 285 | 59.3 | 58.7 |
| seed 22 | the assignment's mapping | the corners' best on streams 1 to 3 | 332, 128, 128, 332 | 61.7 | 60.7 |
| seed 22 | the assignment's mapping | the start | 256, 256, 256, 256 | 27.3 | not read |
| seed 22 | the mirrored mapping | the search's best, scored in generation 9 | 176, 325, 292, 133 | 58.7 | 58.3 |
| seed 22 | the mirrored mapping | the corners' best on streams 1 to 3 | 128, 332, 332, 128 | 61.3 | 60.7 |
| seed 22 | the mirrored mapping | the start | 256, 256, 256, 256 | 31.3 | not read |
| seed 23 | the assignment's mapping | the search's best, scored in generation 9 | 309, 144, 169, 294 | 56.7 | 56.0 |
| seed 23 | the assignment's mapping | the corners' best on streams 1 to 3 | 332, 128, 128, 332 | 62.3 | 62.3 |
| seed 23 | the assignment's mapping | the start | 256, 256, 256, 256 | 18.7 | not read |
| seed 23 | the mirrored mapping | the search's best, scored in generation 5 | 187, 326, 256, 128 | 62.3 | 62.0 |
| seed 23 | the mirrored mapping | the corners' best on streams 1 to 3 | 128, 332, 332, 128 | 63.7 | 64.0 |
| seed 23 | the mirrored mapping | the start | 256, 256, 256, 256 | 41.7 | not read |
| seed 24 | the assignment's mapping | the search's best, scored in generation 9 | 308, 144, 187, 280 | 63.3 | 62.3 |
| seed 24 | the assignment's mapping | the corners' best on streams 1 to 3 | 332, 128, 128, 332 | 63.7 | 63.3 |
| seed 24 | the assignment's mapping | the start | 256, 256, 256, 256 | 50.0 | not read |
| seed 24 | the mirrored mapping | the search's best, scored in generation 9 | 176, 315, 293, 172 | 43.7 | 44.3 |
| seed 24 | the mirrored mapping | the corners' best on streams 1 to 3 | 128, 332, 332, 128 | 55.3 | 58.0 |
| seed 24 | the mirrored mapping | the start | 256, 256, 256, 256 | 11.3 | not read |

The corners' best stood above the search's best by 1 to 35 correct trials summed over streams 1 to 3, 0.3 to 11.7 per 64 from the unrounded
sums, and by 3 to 41 over streams 4 to 6, 1.0 to 13.7 per 64. Of the search's best's four genes, two on seed 22 under the assignment's
mapping and one on seed 23 under the mirrored mapping stood at 128 or 332, and none on the other four.

**The search, generation by generation.** The best sum over streams 1 to 3 among the candidates scored by each generation's end, of 192 at
most:

| Network | Mapping | Generations 0 to 9 |
| :--- | :--- | :--- |
| seed 22 | the assignment's mapping | 134, 134, 135, 153, 155, 158, 168, 169, 169, 178 |
| seed 22 | the mirrored mapping | 153, 153, 155, 167, 167, 173, 174, 175, 175, 176 |
| seed 23 | the assignment's mapping | 128, 128, 128, 135, 163, 163, 163, 167, 167, 170 |
| seed 23 | the mirrored mapping | 176, 176, 177, 179, 183, 187, 187, 187, 187, 187 |
| seed 24 | the assignment's mapping | 181, 181, 181, 182, 186, 186, 186, 187, 188, 190 |
| seed 24 | the mirrored mapping | 106, 117, 117, 117, 121, 122, 125, 128, 128, 131 |

**The bound.** No read of the 2 133 moved a coupling, so each stood at its read's end as the writer wrote it. The highest of any read was
1.297 of the start image's, just above 332/256, 1.296875, since the writer rounds each weight half away from zero; none passed 1.30.

**Beside the learners.** Each learner's last 128 under each mapping, per 64, by the start and the mapping it stood in:

| Network | Mapping | Learner | Last 128 under the mapping, per 64 |
| :--- | :--- | :--- | :--- |
| seed 22 | the assignment's mapping | the engine, H-46's arm | from the assignment, mappings 1 and 3: 62.0, 61.5; from the mirrored assignment, mappings 2 and 4: 61.0, 63.0 |
| seed 22 | the assignment's mapping | the floor on the two summed counts | from the assignment, mappings 1 and 3: 33.5, 33.5; from the mirrored assignment, mappings 2 and 4: 32.0, 27.0 |
| seed 22 | the assignment's mapping | the floor on each readout unit's count | from the assignment, mappings 1 and 3: 61.5, 63.0; from the mirrored assignment, mappings 2 and 4: 61.5, 63.5 |
| seed 22 | the assignment's mapping | the readouts' own selection, unmoved | from the assignment, mappings 1 and 3: 26.5, 31.0; from the mirrored assignment, mappings 2 and 4: 22.0, 30.0 |
| seed 22 | the mirrored mapping | the engine, H-46's arm | from the assignment, mappings 2 and 4: 62.0, 63.0; from the mirrored assignment, mappings 1 and 3: 62.0, 63.0 |
| seed 22 | the mirrored mapping | the floor on the two summed counts | from the assignment, mappings 2 and 4: 33.5, 35.5; from the mirrored assignment, mappings 1 and 3: 30.5, 27.5 |
| seed 22 | the mirrored mapping | the floor on each readout unit's count | from the assignment, mappings 2 and 4: 61.5, 63.0; from the mirrored assignment, mappings 1 and 3: 60.5, 63.0 |
| seed 22 | the mirrored mapping | the readouts' own selection, unmoved | from the assignment, mappings 2 and 4: 37.0, 27.0; from the mirrored assignment, mappings 1 and 3: 32.0, 27.0 |
| seed 23 | the assignment's mapping | the engine, H-46's arm | from the assignment, mappings 1 and 3: 62.5, 63.5; from the mirrored assignment, mappings 2 and 4: 61.5, 64.0 |
| seed 23 | the assignment's mapping | the floor on the two summed counts | from the assignment, mappings 1 and 3: 37.5, 37.5; from the mirrored assignment, mappings 2 and 4: 32.0, 37.0 |
| seed 23 | the assignment's mapping | the floor on each readout unit's count | from the assignment, mappings 1 and 3: 63.5, 64.0; from the mirrored assignment, mappings 2 and 4: 63.5, 63.5 |
| seed 23 | the assignment's mapping | the readouts' own selection, unmoved | from the assignment, mappings 1 and 3: 19.0, 20.0; from the mirrored assignment, mappings 2 and 4: 18.5, 22.0 |
| seed 23 | the mirrored mapping | the engine, H-46's arm | from the assignment, mappings 2 and 4: 61.5, 62.5; from the mirrored assignment, mappings 1 and 3: 63.5, 61.0 |
| seed 23 | the mirrored mapping | the floor on the two summed counts | from the assignment, mappings 2 and 4: 32.0, 37.0; from the mirrored assignment, mappings 1 and 3: 37.5, 39.0 |
| seed 23 | the mirrored mapping | the floor on each readout unit's count | from the assignment, mappings 2 and 4: 63.5, 63.5; from the mirrored assignment, mappings 1 and 3: 62.5, 64.0 |
| seed 23 | the mirrored mapping | the readouts' own selection, unmoved | from the assignment, mappings 2 and 4: 41.5, 33.5; from the mirrored assignment, mappings 1 and 3: 40.0, 37.5 |
| seed 24 | the assignment's mapping | the engine, H-46's arm | from the assignment, mappings 1 and 3: 62.5, 63.0; from the mirrored assignment, mappings 2 and 4: 62.5, 63.5 |
| seed 24 | the assignment's mapping | the floor on the two summed counts | from the assignment, mappings 1 and 3: 48.0, 54.0; from the mirrored assignment, mappings 2 and 4: 32.0, 27.0 |
| seed 24 | the assignment's mapping | the floor on each readout unit's count | from the assignment, mappings 1 and 3: 62.5, 63.5; from the mirrored assignment, mappings 2 and 4: 64.0, 64.0 |
| seed 24 | the assignment's mapping | the readouts' own selection, unmoved | from the assignment, mappings 1 and 3: 46.5, 48.5; from the mirrored assignment, mappings 2 and 4: 48.0, 54.0 |
| seed 24 | the mirrored mapping | the engine, H-46's arm | from the assignment, mappings 2 and 4: 64.0, 62.0; from the mirrored assignment, mappings 1 and 3: 62.5, 63.5 |
| seed 24 | the mirrored mapping | the floor on the two summed counts | from the assignment, mappings 2 and 4: 32.0, 27.0; from the mirrored assignment, mappings 1 and 3: 48.0, 54.0 |
| seed 24 | the mirrored mapping | the floor on each readout unit's count | from the assignment, mappings 2 and 4: 64.0, 63.5; from the mirrored assignment, mappings 1 and 3: 63.0, 64.0 |
| seed 24 | the mirrored mapping | the readouts' own selection, unmoved | from the assignment, mappings 2 and 4: 13.0, 7.0; from the mirrored assignment, mappings 1 and 3: 12.5, 11.5 |

**The sixteen corners**, each one's correct trials of 64 on streams 1 to 6, under each mapping:

| Network | Genes | Assignment's mapping, streams 1 to 3 | 4 to 6 | Mirrored mapping, streams 1 to 3 | 4 to 6 | Past 1.30 |
| :--- | :--- | :--- | :--- | :--- | :--- | :---: |
| seed 22 | 128, 128, 128, 128 | 26, 25, 27 | 26, 27, 27 | 31, 33, 31 | 30, 28, 28 | no |
| seed 22 | 332, 128, 128, 128 | 47, 42, 45 | 42, 45, 44 | 12, 19, 17 | 15, 15, 13 | no |
| seed 22 | 128, 332, 128, 128 | 18, 12, 19 | 17, 10, 16 | 43, 48, 44 | 42, 49, 41 | no |
| seed 22 | 332, 332, 128, 128 | 22, 22, 25 | 22, 21, 22 | 33, 37, 34 | 31, 38, 33 | no |
| seed 22 | 128, 128, 332, 128 | 13, 13, 11 | 12, 18, 13 | 47, 47, 50 | 47, 41, 44 | no |
| seed 22 | 332, 128, 332, 128 | 32, 31, 27 | 28, 35, 27 | 30, 32, 34 | 31, 28, 32 | no |
| seed 22 | 128, 332, 332, 128 | 4, 0, 2 | 4, 1, 1 | 59, 63, 62 | 59, 62, 61 | no |
| seed 22 | 332, 332, 332, 128 | 12, 9, 10 | 9, 13, 9 | 50, 51, 51 | 50, 49, 53 | no |
| seed 22 | 128, 128, 128, 332 | 42, 46, 43 | 44, 45, 44 | 19, 16, 14 | 17, 16, 16 | no |
| seed 22 | 332, 128, 128, 332 | 63, 62, 60 | 60, 61, 61 | 0, 1, 0 | 3, 2, 3 | no |
| seed 22 | 128, 332, 128, 332 | 33, 32, 34 | 34, 28, 33 | 31, 31, 28 | 30, 36, 30 | no |
| seed 22 | 332, 332, 128, 332 | 40, 41, 42 | 40, 36, 37 | 21, 19, 19 | 24, 23, 25 | no |
| seed 22 | 128, 128, 332, 332 | 29, 27, 31 | 33, 36, 31 | 27, 32, 28 | 28, 24, 25 | no |
| seed 22 | 332, 128, 332, 332 | 49, 45, 50 | 46, 51, 48 | 11, 13, 13 | 15, 10, 15 | no |
| seed 22 | 128, 332, 332, 332 | 18, 17, 25 | 21, 20, 20 | 41, 45, 39 | 40, 43, 41 | no |
| seed 22 | 332, 332, 332, 332 | 27, 25, 32 | 27, 28, 26 | 33, 32, 28 | 32, 30, 34 | no |
| seed 23 | 128, 128, 128, 128 | 22, 20, 24 | 22, 27, 24 | 33, 32, 35 | 38, 28, 34 | no |
| seed 23 | 332, 128, 128, 128 | 40, 37, 39 | 38, 40, 43 | 18, 20, 23 | 22, 20, 20 | no |
| seed 23 | 128, 332, 128, 128 | 13, 7, 13 | 11, 6, 14 | 48, 48, 48 | 49, 55, 47 | no |
| seed 23 | 332, 332, 128, 128 | 17, 15, 18 | 18, 12, 19 | 42, 39, 45 | 42, 47, 42 | no |
| seed 23 | 128, 128, 332, 128 | 8, 15, 13 | 11, 23, 9 | 52, 44, 49 | 52, 37, 50 | no |
| seed 23 | 332, 128, 332, 128 | 29, 31, 27 | 29, 35, 29 | 34, 33, 37 | 35, 28, 35 | no |
| seed 23 | 128, 332, 332, 128 | 0, 0, 0 | 0, 0, 0 | 64, 63, 64 | 64, 64, 64 | no |
| seed 23 | 332, 332, 332, 128 | 4, 11, 4 | 8, 6, 3 | 59, 51, 59 | 56, 55, 59 | no |
| seed 23 | 128, 128, 128, 332 | 40, 47, 48 | 44, 51, 40 | 19, 12, 12 | 19, 9, 18 | no |
| seed 23 | 332, 128, 128, 332 | 62, 63, 62 | 61, 63, 63 | 1, 1, 1 | 2, 1, 1 | no |
| seed 23 | 128, 332, 128, 332 | 33, 33, 35 | 33, 28, 34 | 31, 30, 28 | 31, 36, 30 | no |
| seed 23 | 332, 332, 128, 332 | 38, 40, 40 | 39, 33, 36 | 26, 18, 24 | 24, 27, 27 | no |
| seed 23 | 128, 128, 332, 332 | 21, 27, 24 | 25, 29, 28 | 37, 29, 35 | 34, 27, 30 | no |
| seed 23 | 332, 128, 332, 332 | 43, 40, 38 | 41, 43, 45 | 19, 22, 24 | 18, 18, 16 | no |
| seed 23 | 128, 332, 332, 332 | 17, 10, 11 | 15, 7, 19 | 45, 50, 48 | 45, 55, 44 | no |
| seed 23 | 332, 332, 332, 332 | 21, 20, 14 | 21, 13, 20 | 39, 41, 46 | 39, 46, 42 | no |
| seed 24 | 128, 128, 128, 128 | 35, 39, 37 | 34, 34, 33 | 17, 21, 21 | 24, 24, 23 | no |
| seed 24 | 332, 128, 128, 128 | 53, 51, 49 | 51, 50, 48 | 8, 11, 10 | 11, 11, 12 | no |
| seed 24 | 128, 332, 128, 128 | 23, 21, 23 | 22, 14, 21 | 38, 39, 36 | 39, 45, 39 | no |
| seed 24 | 332, 332, 128, 128 | 43, 37, 40 | 40, 39, 44 | 15, 23, 17 | 18, 20, 16 | no |
| seed 24 | 128, 128, 332, 128 | 16, 21, 18 | 18, 20, 18 | 42, 39, 40 | 39, 38, 42 | no |
| seed 24 | 332, 128, 332, 128 | 32, 34, 31 | 35, 39, 33 | 32, 27, 27 | 28, 25, 30 | no |
| seed 24 | 128, 332, 332, 128 | 1, 4, 7 | 6, 4, 4 | 59, 55, 52 | 55, 59, 60 | no |
| seed 24 | 332, 332, 332, 128 | 23, 21, 25 | 23, 28, 26 | 36, 37, 34 | 36, 32, 37 | no |
| seed 24 | 128, 128, 128, 332 | 47, 49, 50 | 44, 44, 47 | 12, 14, 12 | 12, 13, 15 | no |
| seed 24 | 332, 128, 128, 332 | 63, 64, 64 | 63, 63, 64 | 1, 0, 0 | 0, 1, 0 | no |
| seed 24 | 128, 332, 128, 332 | 33, 34, 39 | 34, 28, 35 | 31, 29, 25 | 28, 36, 29 | no |
| seed 24 | 332, 332, 128, 332 | 55, 51, 54 | 52, 51, 56 | 6, 11, 6 | 7, 9, 7 | no |
| seed 24 | 128, 128, 332, 332 | 45, 46, 48 | 40, 44, 41 | 13, 15, 13 | 15, 16, 18 | no |
| seed 24 | 332, 128, 332, 332 | 61, 61, 63 | 59, 62, 59 | 2, 2, 0 | 4, 1, 3 | no |
| seed 24 | 128, 332, 332, 332 | 31, 33, 37 | 32, 27, 29 | 32, 31, 27 | 32, 36, 31 | no |
| seed 24 | 332, 332, 332, 332 | 52, 52, 53 | 49, 54, 49 | 7, 11, 8 | 13, 8, 9 | no |

## What did not run

- Any other task, H-47's among them; any gene but the four couplings' factors; a floor that sees a stimulus unit or the critic's window; the
  engine under any other configuration.
- `PROBE_FROZEN`'s frozen blocks, 31, 20 and 47 of 64 under the assignment on seeds 22, 23 and 24, are each derivation's, under its
  calibration and not the running configuration. They stood in ADR-0255 as context and were not read again here; the readouts' own selection
  above is this run's.
- A second seed of the search, or a larger one. It ran once a mapping, from one seed, at ADR-0255's sizes; how often it reaches a corner
  from another seed is not read.
- Floors on frozen features: with the reward withheld the inhibitory rule ran at its baseline, and the inhibitory sum fell as the last item
  of In brief says, so the floors' features carry that drift. Each read is one block from the start image and carries one block's.

## What it suggests

Questions, not conclusions; nothing here decides whether the task is kept or how a ladder is ordered.

- The floor on each readout unit's count, a delta rule over 918 counts of a network whose excitatory sum and couplings never moved, stood
  near the engine on every reading here: its last 128 −3 to +6 trials from the engine's, its blocks to 40 of 64 −6 to +2 blocks. What does
  this task ask of the engine that such a readout of the same counts does not?
- The floor on the two summed counts often counted what one answer on every trial gives. Would a floor on two features leave that at another
  step or scale, or are two features too few on these networks?
- The best corner followed one rule on every network and mapping, each stimulus's answer coupling at 332 and the other at 128, and the
  search stayed below it. Would another seed or more generations reach it, and is the ceiling within the bound at that corner on every
  network?
