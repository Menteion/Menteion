---
status: exploratory
date: 2026-11-09
---

# 026: The excitatory hold on H-47's other eight cells

**Branch**: `explore/026-the-excitatory-hold-on-eight-cells`
**Commit**: `c98207f`, which adds the harness and nothing under `src/`, on `c53d3db`: `main` at `6d2dc51`, ADR-0241's merge, with
exploration 025's tag merged in, which carries exploration 024's and 025's changes under `src/`.
**Runs**: [42488465689](https://github.com/Menteion/Menteion/actions/runs/42488465689) (the harness's trial) and
[42489793510](https://github.com/Menteion/Menteion/actions/runs/42489793510) (eight jobs, one a cell), both at `scope=arms`. H-47's plain
night on A's patterns is read from H-47's run [41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613), its class
matrix and night lines from exploration 021's run [41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514), which ran
exploration 020's job on these eight cells. Section 6 reads exploration 025's run
[42469251250](https://github.com/Menteion/Menteion/actions/runs/42469251250) for its four cells. None of those was run again.

[ADR-0241](../adr/0241-synapses-from-the-readouts-excitatory-units-on-eight-cells.md) ordered this exploration.
[Exploration 025](025-synapses-from-the-readouts-excitatory-units.md) held every synapse from the readouts' excitatory units, the pair's
own units excepted, from a night's first tick to its last, on [exploration 020](020-what-a-replay-costs.md)'s four cells, all where
stimulus A answers readout 1. Under that excitatory night A's count stood within 2 SE of H-47's plain night's or above it at every
checkpoint, above at 8 of 20. This note runs the same hold, unchanged, on H-47's other eight cells, where no hold has run: A answers readout
0 in six of them and readout 1 in two, and H-47's clause 1 held in all eight. It asks, at blocks 6, 12, 18, 24 and 36:

1. A's count and each stimulus's correct trials under each held form: less the night without replay, with H-47's paired SE, the forms run
   in one job; and the excitatory night less H-47's plain night, with [exploration 018](018-what-nights-keep.md)'s unpaired SE, the forms
   run in two jobs;
2. the side under each held form, by stimulus and by group of units, against H-47's side on the same cell;
3. the pair under each held form and A's highest coupling, as H-47's clauses 2 and 3 read them;
4. what each night moved, from the class matrix by group, with the held units' synapses split by target and their change shown nil;
5. what the hold leaves: what the quiet ticks after each night move, and the pairings still pending under the excitatory rule, the gain and
   the loss apart;
6. the twelve cells side by side at block 36: each cell's excitatory night less its night without replay, with the paired SE of the job
   that ran both, and less H-47's plain night, with exploration 018's unpaired SE.

**The side** is [ADR-0238](../adr/0238-what-keeps-a-mapping-where-the-replay-did-not.md)'s: at a checkpoint, the stimulus whose difference
from the night without replay is the larger of the two. The note writes no prediction and decides nothing. **Every figure here is
exploratory**: from one run of each test on the repository's AArch64 host, in H-47's terms. A count is a mean over a cell's 48 reads,
sixteen on each of three paths, and a difference carries its paired SE unless another is named.

**In brief**
- **The hold held, and the night without replay ran H-47's.** In all 192 nights with the excitatory hold every held synapse ended the night
  at the weight it started with and its trace cleared, held inside the test, while 4 919 to 4 989 excitatory synapses from other units
  moved. The quiet ticks after a night moved none of the held synapses (section 5). Every row of the night without replay, 15 a cell, is
  H-47's, held inside the test.
- **Under the excitatory night A's count less H-47's plain night's** lies between −4.0 and +8.1, beyond twice its unpaired SE at 17 of the
  40 checkpoints, 11 above and 6 below. Above: on seed 24's eighth image from the mirrored assignment at every checkpoint, +2.9 ± 0.92 to
  +8.1 ± 1.60; on seed 23's eighth image from the assignment from block 12 on, +4.1 ± 1.53 to +7.3 ± 1.37; on seed 24's first mapping from
  the assignment at block 36, +2.0 ± 0.79, and from the mirrored assignment at block 24, +1.7 ± 0.68. Below: on seed 23's eighth image from
  the mirrored assignment from block 12 on, −2.7 ± 1.06 to −4.0 ± 1.09, and on seed 22's two cells at block 36, −3.4 ± 1.65 and
  −2.7 ± 1.05. On seed 23's first mapping from the assignment it lies within 2 SE at every checkpoint.
- **The excitatory night less the night without replay at block 36**, where A answers readout 0, is +16.3 ± 0.84 and +5.2 ± 1.24 on seed
  22's eighth image from the mirrored assignment and first mapping from the assignment, +14.7 ± 1.54 and +12.6 ± 1.56 on seed 23's, and
  +16.7 ± 1.23 and +5.8 ± 1.60 on seed 24's, where H-47's clause 1 read +19.6 ± 1.06, +7.9 ± 0.51, +17.4 ± 1.08, +13.4 ± 1.56,
  +11.3 ± 0.96 and +3.8 ± 1.57. Where A answers readout 1, on seed 23's eighth image from the assignment and seed 24's first mapping from
  the mirrored assignment, it is +10.9 ± 2.49 and +7.4 ± 1.09, where H-47's read +4.1 ± 1.45 and +6.0 ± 1.54. At every checkpoint it lies
  above the night without replay, beyond 2 SE at 37 of the 40.
- **The excitatory night without replay less the night without replay** lies between −5.4 and +8.1, beyond 2 SE at 15 of the 40
  checkpoints, 7 above and 8 below.
- **The side under the excitatory night is H-47's at 35 of the 40 checkpoints.** It differs at seed 22's eighth image from the mirrored
  assignment at block 36, at seed 23's eighth image from the mirrored assignment at block 12, at seed 24's eighth image from the mirrored
  assignment at block 24, and at seed 24's first mapping from the mirrored assignment at blocks 12 and 36 (section 2).
- **The excitatory night ran H-47's plain night's replays and depotentiations in 94 of its 96 nights.** The two others are on one path of
  seed 23's eighth image from the mirrored assignment, each one replay or one depotentiation more than the plain night's.
- **The held units' synapses moved by exactly nothing in every held night**, where H-47's plain night moved them by −17.05 to −23.04 M a
  path over four nights, −5.10 to −6.98 M of each group's onto the other readout's excitatory units.
- **Under the excitatory night the inhibition onto each group's excitatory units rose by 7.46 to 16.32 M** over four nights, 0.69 to
  1.92 M more than under H-47's plain night. The engine's excitatory sum fell by 0.02 to 0.18 M a night, where the plain night took 2.20 to
  9.49 M.
- **The pairings the hold leaves**: after the quiet ticks of each excitatory night, 10 591 to 11 474 of the 21 248 held synapses carry a
  pending gain, 467 165 to 731 434 LSB together, read at the presynaptic unit's next spike if it comes before the target's next spike. A
  presynaptic spike at the first tick after the image would read a loss of 45 419 to 84 224 LSB on them together, and a later one, before
  the target's next spike, no more.
- **The pair under the excitatory night** stands at 61.7 to 63.0 at block 36, and A's highest coupling at 1.146 to 1.236 of H-42's.
- **The twelve cells at block 36** (section 6): less H-47's plain night, the excitatory night lies beyond 2 SE below in three cells, all
  where A answers readout 0, and above in five, two where A answers readout 0 and three where it answers readout 1. Less the night without
  replay it lies above beyond 2 SE in eleven and below in one, seed 23's first mapping from the mirrored assignment (exploration 025).

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 020 to 025 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. A cell is named, as in H-47, by its seed, its image and the arm
it starts from, not by the mapping in force. Nights follow blocks 6, 12, 18 and 24, then 12 blocks run with none to block 36, on paths 0, 5
and 6. Each checkpoint reads A and the pair sixteen times each, after the night that follows its block, and at block 36 after the block.

**Places.** A period of the ring is 20 units. Stimulus A presents place 0 of each period and B place 11. **The odd places** are the odd
places but 11 and **the even places** the even places but 0: H-47's readout 0 and readout 1, each with its excitatory and its inhibitory
units. C's own units lie at place 5, among the odd places, and D's at place 16, among the even, in every cell
([exploration 021](021-where-the-gain-falls.md), section 4).

**Eight cells**, H-47's twelve less exploration 020's four, where no hold has run. Under the assignment stimulus A answers readout 0, the odd
places, and B readout 1, the even places; under the mirrored mapping the other way. The tables list the six cells where A answers readout 0
first:

| Cell | A answers | B answers | H-47: plain less without replay at block 36 (clause 1) |
| :--- | :--- | :--- | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +19.6 ± 1.06 |
| Seed 22, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +7.9 ± 0.51 |
| Seed 23, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +17.4 ± 1.08 |
| Seed 23, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +13.4 ± 1.56 |
| Seed 24, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +11.3 ± 0.96 |
| Seed 24, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +3.8 ± 1.57 |
| Seed 23, eighth, assignment | readout 1, the even places | readout 0, the odd places | +4.1 ± 1.45 |
| Seed 24, first mapping, mirrored | readout 1, the even places | readout 0, the odd places | +6.0 ± 1.54 |

**The excitatory hold** is exploration 025's, on the branch alone (`72bd3f5`, carried by 025's tag), unchanged: this branch changes nothing
under `src/`. `Executor::set_night_hold_sources` takes a set of excitatory units. While it is set and the engine is in slow-wave sleep or
REM, the fan-out skips the consolidation of every block of a held unit's chain, onto whatever target: no slot of it consolidates, at any
modulation, so every weight stays where it stood while its trace takes the pairings. **The night's last tick** is the tick whose window steps
the engine from sleep to awake; the hold is in force through it, and at its end every slot of a held unit's chain has its trace cleared to
zero. Every other synapse consolidates as in H-47. The hold is in no image, so the next day's engine, decoded from the night's image, holds
nothing. Exploration 024's hold (`set_night_hold`) is on the branch too, and no form here sets it.

**The held units** (`e025_held`) are every excitatory unit of the two readouts but the pair's own: exploration 024's 714, less C's own 25
and D's own 25, so 664, 332 a readout, in each cell, as each job prints. That none of them is one of the zero sum's 152 sources, nor a unit
of the six kinds, is held inside the test, so they are the class matrix's classes 6 to 9: each readout's units that A's patterns reach and
its other excitatory units. In each cell the hold holds the held units' 21 248 synapses, of the cell's 26 240 excitatory synapses. The
synapses from A's, B's and the pair's own units consolidate as in H-47, the zero sum's rounds with them, and so does every inhibitory
synapse.

**Three forms a cell, in one job** (`e026_job`), exploration 025's job with the night under both holds left out
([ADR-0241](../adr/0241-synapses-from-the-readouts-excitatory-units-on-eight-cells.md)):
- **the night without replay**, every row held inside the test to the row H-47 pinned;
- **the excitatory night**, H-47's plain night on A's patterns with the excitatory hold set;
- **the excitatory night without replay**, the night without replay with the excitatory hold set, to read the hold apart from the replay.

The job is exploration 020's and 025's: A's patterns, the split and the class matrix of the start image, of the image before each night and
of the image after it, and the nights after blocks 6, 12, 18 and 24 watched by `ReplayWatch`. H-47's plain night on A's patterns is read
from H-47's logs, its class matrix and night lines from exploration 021's job on these eight cells.

**At each night with the excitatory hold** (`e025_night`) the job reads every excitatory synapse three times: before the night's first
tick, at its last, and after the quiet ticks, the awake ticks without drive that H-47's harness runs until the engine is quiescent, before
it writes the night's image. At the last it holds, inside the test, that every held synapse stands at its first weight with its trace
cleared and that some excitatory synapse from a unit outside the hold moved. After the quiet ticks it prints what they moved, and what the
held synapses would read at their presynaptic unit's next spike from the spikes on record since the night's first tick, if that spike came
before the target's next one: for the gain and for the loss, how many held synapses carry one, their sum and the largest.

**The excitatory rule** (`step_stdp_shaped`, in `crates/menteion-core/src/dynamics/synapse.rs`) reads two pairings at a presynaptic spike
at $t$, with $p$ the presynaptic unit's previous spike and $q$ the target's last spike: if $p < q \le t$, a gain of
$A_+ (1 - 2^{-11})^{q - p}$; and if $q < t$, a loss of $A_- (1 - 2^{-11})^{t - q}$ scaled by the slot's magnitude, by the exponent the
configuration sets ([ADR-0228](../adr/0228-a-zero-sum-at-the-stimulus-built.md)). Both are read only there, so the night's last pairings
escape the clearing. The job finds each with the rule itself, on a copy of the block, the slot's trace set to zero; with $t_0$ the first
tick after the image:
- **the pending gain**: a held synapse whose target's last spike and presynaptic unit's last spike are both on record since the night's
  first tick, $q > p$: $A_+ (1 - 2^{-11})^{q - p}$, read at the presynaptic unit's next spike if it comes before the target's next spike;
- **the pending loss**: a held synapse whose target fired since the night's first tick: the loss a presynaptic spike at $t_0$ would read,
  at the synapse's magnitude after the quiet ticks. It falls as $t$ grows, so it is the most a later presynaptic spike reads, if that spike
  comes before the target's next one.

A target spike before the next presynaptic one takes $q$'s place, and both readings change with it.

**The class matrix** is exploration 020's, as 021 reads it by readout and 023 to 025 by group. Each group's excitatory units are its
readout's two classes and the pair's own kind inside it, C's own in the odd places and D's in the even. An inhibitory entry is a
magnitude, the weight negated, so a rise is more inhibition. The held synapses are the rows of classes 6 and 7, the odd places' held
units, and 8 and 9, the even places', onto every class.

## Before a job ran

ADR-0241 asks whether any form but the night without replay must run bit for bit as one already in the logs. **None must.** No hold had run
on these eight cells, and each held form holds synapses that consolidate in the form it is set beside: in exploration 021's job on these
cells, over four nights, the held units' synapses moved by −7.61 to −11.96 M a group under H-47's plain night and by −0.59 to +1.50 M under
the night without replay, so neither the excitatory night nor the excitatory night without replay starts from a night that leaves them
alone. The trial then printed each held form's row at block 6 on path 0 beside H-47's of the form it is set beside: neither is the same,
as the trial prints. So all three forms ran.

## The runs are laid out as written

- **The trial** ran on path 0. On seed 23's first mapping from the assignment: the pair's head and the first night of H-47's plain night on
  A's patterns, the night without replay, the excitatory night and the excitatory night without replay, all watched. With no hold set, the
  night without replay and the plain night held H-47's row at block 6 on path 0, inside the test. With the excitatory hold set, each night
  held every held synapse at its first weight with its trace cleared, inside the test: 21 248 of the cell's 26 240 excitatory synapses,
  while 4 989 and 4 982 excitatory synapses from other units moved in the excitatory night and the excitatory night without replay. On seed
  23's first mapping from the mirrored assignment: the pair's head and the excitatory night's first night, which held the row exploration
  025's trial printed at block 6 on path 0 (run 42468595193), inside the test: its image's CRC-64, A's first count of 58, its couplings and
  the gain.
- The two held forms' rows at block 6 on path 0 on seed 23's first mapping from the assignment are not H-47's of the form beside them, as
  the trial prints: each image's CRC-64, A's couplings and the gain differ, and A's first count is 56 under the excitatory night, where
  H-47's plain night read 57, and 52 under the excitatory night without replay, where H-47's night without replay read 50. The watch read
  `wrong 0` in the trial's five nights; the plain night and the excitatory night on each cell read 1 339 replays and 640 depotentiations
  each, and the two nights without replay none.
- **Each of the eight jobs** ran to the end. Their reproduction, cell by cell:

| Cell | Without replay: rows H-47's | Excitatory: rows H-47's plain on A's | Excitatory without replay: rows H-47's without replay | Without replay: its paths' lines in 021's job, label aside | Heads: lines in 021's job | Held nights: the excitatory hold held inside the test |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 15 of 15 | 0 of 15 | 0 of 15 | 1 632 of 1 632 | 3 of 3 | 24 of 24 |
| Seed 22, first mapping, assignment | 15 of 15 | 0 of 15 | 0 of 15 | 1 632 of 1 632 | 3 of 3 | 24 of 24 |
| Seed 23, eighth, mirrored | 15 of 15 | 0 of 15 | 0 of 15 | 1 680 of 1 680 | 3 of 3 | 24 of 24 |
| Seed 23, first mapping, assignment | 15 of 15 | 0 of 15 | 0 of 15 | 1 680 of 1 680 | 3 of 3 | 24 of 24 |
| Seed 24, eighth, mirrored | 15 of 15 | 0 of 15 | 0 of 15 | 1 656 of 1 656 | 3 of 3 | 24 of 24 |
| Seed 24, first mapping, assignment | 15 of 15 | 0 of 15 | 0 of 15 | 1 656 of 1 656 | 3 of 3 | 24 of 24 |
| Seed 23, eighth, assignment | 15 of 15 | 0 of 15 | 0 of 15 | 1 680 of 1 680 | 3 of 3 | 24 of 24 |
| Seed 24, first mapping, mirrored | 15 of 15 | 0 of 15 | 0 of 15 | 1 656 of 1 656 | 3 of 3 | 24 of 24 |

- The night without replay's `DUMP` lines, its paths' 1 632 to 1 680 a cell, its 15 rows' and its notice, are the lines of exploration
  021's run of exploration 020's job on the same cell, label aside, all of them in each cell; so are the pair's head's three. Each read of
  the night without replay carries H-47's read's `stimuli`, `right` and `tied` masks, all 480 a cell. The excitatory night's reads carry
  H-47's plain night's `stimuli` masks, and the excitatory night without replay's H-47's night without replay's, so each trial presented
  the stimulus it presented in H-47.
- For each held form the job prints how many of its 15 rows are H-47's of the matching form: **none, in all eight cells**, as the job
  prints; nothing in the job asserts it.
- **The watch read `wrong 0` in all 288 nights.** Every night without replay, held or not, read `replays 0` and `depotentiations 0`. The
  excitatory night read H-47's plain night's replays and depotentiations, as exploration 021's run printed them, in 94 of its 96 nights.
  The two others are on seed 23's eighth image from the mirrored assignment, path 0: the night after block 18 read 1 202 replays and 571
  depotentiations, where the plain night read 1 202 and 570, and the night after block 24 read 1 129 replays and 545 depotentiations, where
  the plain night read 1 128 and 545.
- Every held unit's synapse was read at each night: 664 held units, 332 a readout, out of exploration 024's 714, C's own 25 and D's own 25
  left out, as each job prints; that none of the zero sum's 152 sources is among them each job holds. Each held night held the hold inside
  the test, 24 a cell.

## 1. A's count under each held night

A's count at each checkpoint under each form. H-47's plain night on A's patterns is H-47's job's:

| Cell | Block | Without replay | Excitatory | Excitatory without replay | Plain on A's (H-47) |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 46.3 | 52.4 | 45.8 | 53.3 |
| Seed 22, eighth, mirrored | 12 | 30.2 | 41.0 | 29.7 | 42.3 |
| Seed 22, eighth, mirrored | 18 | 26.3 | 40.9 | 25.8 | 41.8 |
| Seed 22, eighth, mirrored | 24 | 22.6 | 37.1 | 21.1 | 40.2 |
| Seed 22, eighth, mirrored | 36 | 16.6 | 32.8 | 15.5 | 36.2 |
| Seed 22, first mapping, assignment | 6 | 52.4 | 56.8 | 51.8 | 56.5 |
| Seed 22, first mapping, assignment | 12 | 44.8 | 45.8 | 43.7 | 47.6 |
| Seed 22, first mapping, assignment | 18 | 40.2 | 46.2 | 40.4 | 46.9 |
| Seed 22, first mapping, assignment | 24 | 38.9 | 45.6 | 39.8 | 48.0 |
| Seed 22, first mapping, assignment | 36 | 36.3 | 41.5 | 36.5 | 44.3 |
| Seed 23, eighth, mirrored | 6 | 34.4 | 36.8 | 34.6 | 36.1 |
| Seed 23, eighth, mirrored | 12 | 19.5 | 25.5 | 23.5 | 28.5 |
| Seed 23, eighth, mirrored | 18 | 14.9 | 24.0 | 20.6 | 27.6 |
| Seed 23, eighth, mirrored | 24 | 10.6 | 25.2 | 18.7 | 29.3 |
| Seed 23, eighth, mirrored | 36 | 8.2 | 22.9 | 13.4 | 25.6 |
| Seed 23, first mapping, assignment | 6 | 54.2 | 56.1 | 55.0 | 56.3 |
| Seed 23, first mapping, assignment | 12 | 33.4 | 37.5 | 34.6 | 37.6 |
| Seed 23, first mapping, assignment | 18 | 27.2 | 35.1 | 30.2 | 35.0 |
| Seed 23, first mapping, assignment | 24 | 24.8 | 35.0 | 27.6 | 34.8 |
| Seed 23, first mapping, assignment | 36 | 19.7 | 32.3 | 25.5 | 33.0 |
| Seed 24, eighth, mirrored | 6 | 36.4 | 36.6 | 32.5 | 33.8 |
| Seed 24, eighth, mirrored | 12 | 31.5 | 33.6 | 26.2 | 28.6 |
| Seed 24, eighth, mirrored | 18 | 20.2 | 34.4 | 22.8 | 28.6 |
| Seed 24, eighth, mirrored | 24 | 16.4 | 33.9 | 18.0 | 25.8 |
| Seed 24, eighth, mirrored | 36 | 12.5 | 29.2 | 14.1 | 23.8 |
| Seed 24, first mapping, assignment | 6 | 58.3 | 61.1 | 58.6 | 61.2 |
| Seed 24, first mapping, assignment | 12 | 52.0 | 55.7 | 51.3 | 54.8 |
| Seed 24, first mapping, assignment | 18 | 47.5 | 52.5 | 44.1 | 51.3 |
| Seed 24, first mapping, assignment | 24 | 43.7 | 52.7 | 41.9 | 52.4 |
| Seed 24, first mapping, assignment | 36 | 40.1 | 45.8 | 37.8 | 43.8 |
| Seed 23, eighth, assignment | 6 | 47.4 | 50.7 | 48.2 | 50.4 |
| Seed 23, eighth, assignment | 12 | 40.8 | 47.5 | 38.7 | 43.4 |
| Seed 23, eighth, assignment | 18 | 37.3 | 50.1 | 36.3 | 45.2 |
| Seed 23, eighth, assignment | 24 | 38.0 | 51.0 | 37.5 | 43.7 |
| Seed 23, eighth, assignment | 36 | 36.2 | 47.1 | 33.4 | 40.3 |
| Seed 24, first mapping, mirrored | 6 | 54.3 | 57.9 | 54.4 | 57.7 |
| Seed 24, first mapping, mirrored | 12 | 46.3 | 52.3 | 45.7 | 51.0 |
| Seed 24, first mapping, mirrored | 18 | 44.0 | 53.3 | 42.8 | 51.8 |
| Seed 24, first mapping, mirrored | 24 | 44.4 | 53.7 | 42.4 | 52.0 |
| Seed 24, first mapping, mirrored | 36 | 42.7 | 50.1 | 39.6 | 48.7 |

The differences. "Less without replay" carries the paired SE, the forms run in one job; "less plain" carries exploration 018's unpaired
SE. The night without replay is H-47's, read by read, so "H-47: plain less without replay" at block 36 is H-47's clause 1:

| Cell | Block | Excitatory less without replay | H-47: plain less without replay | Excitatory less plain (unpaired) | Excitatory without replay less without replay | Excitatory less excitatory without replay |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +6.1 ± 0.98 | +7.1 ± 0.91 | −0.9 ± 0.75 | −0.5 ± 0.52 | +6.6 ± 0.84 |
| Seed 22, eighth, mirrored | 12 | +10.8 ± 0.98 | +12.1 ± 0.85 | −1.3 ± 0.99 | −0.5 ± 0.67 | +11.2 ± 0.51 |
| Seed 22, eighth, mirrored | 18 | +14.6 ± 1.78 | +15.5 ± 1.67 | −0.9 ± 0.82 | −0.5 ± 1.30 | +15.1 ± 0.83 |
| Seed 22, eighth, mirrored | 24 | +14.5 ± 1.46 | +17.6 ± 1.49 | −3.1 ± 1.88 | −1.5 ± 0.87 | +16.0 ± 1.07 |
| Seed 22, eighth, mirrored | 36 | +16.3 ± 0.84 | +19.6 ± 1.06 | −3.4 ± 1.65 | −1.1 ± 0.45 | +17.3 ± 0.69 |
| Seed 22, first mapping, assignment | 6 | +4.4 ± 1.10 | +4.1 ± 0.82 | +0.3 ± 0.69 | −0.6 ± 0.28 | +4.9 ± 1.12 |
| Seed 22, first mapping, assignment | 12 | +1.0 ± 1.21 | +2.9 ± 0.56 | −1.8 ± 1.28 | −1.1 ± 0.49 | +2.1 ± 1.22 |
| Seed 22, first mapping, assignment | 18 | +6.1 ± 1.11 | +6.8 ± 1.12 | −0.7 ± 1.40 | +0.3 ± 1.21 | +5.8 ± 0.86 |
| Seed 22, first mapping, assignment | 24 | +6.7 ± 1.33 | +9.1 ± 1.37 | −2.4 ± 1.35 | +1.0 ± 1.11 | +5.8 ± 1.25 |
| Seed 22, first mapping, assignment | 36 | +5.2 ± 1.24 | +7.9 ± 0.51 | −2.7 ± 1.05 | +0.2 ± 1.02 | +5.0 ± 1.71 |
| Seed 23, eighth, mirrored | 6 | +2.4 ± 1.08 | +1.7 ± 1.20 | +0.7 ± 1.10 | +0.2 ± 0.82 | +2.2 ± 1.14 |
| Seed 23, eighth, mirrored | 12 | +6.0 ± 0.97 | +9.0 ± 1.07 | −3.0 ± 1.40 | +4.0 ± 0.68 | +2.0 ± 0.46 |
| Seed 23, eighth, mirrored | 18 | +9.1 ± 1.46 | +12.7 ± 1.02 | −3.6 ± 1.24 | +5.8 ± 0.69 | +3.3 ± 1.75 |
| Seed 23, eighth, mirrored | 24 | +14.6 ± 1.96 | +18.7 ± 1.62 | −4.0 ± 1.09 | +8.1 ± 0.86 | +6.6 ± 1.61 |
| Seed 23, eighth, mirrored | 36 | +14.7 ± 1.54 | +17.4 ± 1.08 | −2.7 ± 1.06 | +5.2 ± 0.76 | +9.5 ± 0.93 |
| Seed 23, first mapping, assignment | 6 | +1.9 ± 0.87 | +2.0 ± 0.79 | −0.2 ± 0.84 | +0.8 ± 0.42 | +1.0 ± 0.74 |
| Seed 23, first mapping, assignment | 12 | +4.1 ± 0.95 | +4.1 ± 1.18 | −0.1 ± 1.08 | +1.2 ± 1.09 | +2.9 ± 1.15 |
| Seed 23, first mapping, assignment | 18 | +7.9 ± 2.40 | +7.9 ± 2.54 | 0.0 ± 1.36 | +3.0 ± 2.44 | +4.9 ± 0.63 |
| Seed 23, first mapping, assignment | 24 | +10.3 ± 2.20 | +10.0 ± 1.90 | +0.3 ± 1.28 | +2.8 ± 1.64 | +7.4 ± 0.94 |
| Seed 23, first mapping, assignment | 36 | +12.6 ± 1.56 | +13.4 ± 1.56 | −0.8 ± 0.94 | +5.8 ± 1.10 | +6.8 ± 0.79 |
| Seed 24, eighth, mirrored | 6 | +0.3 ± 1.15 | −2.6 ± 1.35 | +2.9 ± 0.92 | −3.9 ± 0.53 | +4.2 ± 0.97 |
| Seed 24, eighth, mirrored | 12 | +2.1 ± 1.50 | −2.9 ± 1.19 | +5.0 ± 1.22 | −5.4 ± 1.12 | +7.4 ± 0.76 |
| Seed 24, eighth, mirrored | 18 | +14.2 ± 1.34 | +8.3 ± 1.70 | +5.9 ± 1.68 | +2.6 ± 0.79 | +11.6 ± 1.21 |
| Seed 24, eighth, mirrored | 24 | +17.5 ± 1.30 | +9.4 ± 1.58 | +8.1 ± 1.60 | +1.6 ± 0.97 | +15.9 ± 1.24 |
| Seed 24, eighth, mirrored | 36 | +16.7 ± 1.23 | +11.3 ± 0.96 | +5.4 ± 1.53 | +1.6 ± 0.96 | +15.1 ± 1.08 |
| Seed 24, first mapping, assignment | 6 | +2.8 ± 0.68 | +2.9 ± 0.72 | −0.1 ± 0.43 | +0.3 ± 0.31 | +2.6 ± 0.63 |
| Seed 24, first mapping, assignment | 12 | +3.7 ± 1.16 | +2.8 ± 0.93 | +0.9 ± 0.97 | −0.7 ± 0.76 | +4.4 ± 0.85 |
| Seed 24, first mapping, assignment | 18 | +5.0 ± 1.19 | +3.9 ± 1.41 | +1.2 ± 0.98 | −3.3 ± 1.22 | +8.4 ± 0.67 |
| Seed 24, first mapping, assignment | 24 | +9.0 ± 2.46 | +8.7 ± 3.17 | +0.3 ± 1.60 | −1.8 ± 2.05 | +10.8 ± 1.10 |
| Seed 24, first mapping, assignment | 36 | +5.8 ± 1.60 | +3.8 ± 1.57 | +2.0 ± 0.79 | −2.3 ± 0.92 | +8.0 ± 1.04 |
| Seed 23, eighth, assignment | 6 | +3.4 ± 1.07 | +3.0 ± 0.96 | +0.4 ± 1.10 | +0.8 ± 0.40 | +2.6 ± 0.90 |
| Seed 23, eighth, assignment | 12 | +6.7 ± 1.57 | +2.6 ± 0.94 | +4.1 ± 1.53 | −2.1 ± 1.27 | +8.8 ± 1.84 |
| Seed 23, eighth, assignment | 18 | +12.7 ± 1.45 | +7.9 ± 0.90 | +4.8 ± 1.62 | −1.1 ± 1.17 | +13.8 ± 0.61 |
| Seed 23, eighth, assignment | 24 | +13.0 ± 2.14 | +5.7 ± 1.34 | +7.3 ± 1.37 | −0.4 ± 1.71 | +13.4 ± 0.99 |
| Seed 23, eighth, assignment | 36 | +10.9 ± 2.49 | +4.1 ± 1.45 | +6.9 ± 1.57 | −2.8 ± 1.63 | +13.8 ± 1.16 |
| Seed 24, first mapping, mirrored | 6 | +3.6 ± 0.88 | +3.4 ± 0.79 | +0.3 ± 0.86 | +0.1 ± 0.27 | +3.5 ± 0.84 |
| Seed 24, first mapping, mirrored | 12 | +6.0 ± 1.30 | +4.7 ± 1.50 | +1.3 ± 1.05 | −0.6 ± 0.76 | +6.6 ± 0.83 |
| Seed 24, first mapping, mirrored | 18 | +9.3 ± 0.89 | +7.8 ± 0.89 | +1.5 ± 0.86 | −1.3 ± 1.01 | +10.5 ± 0.91 |
| Seed 24, first mapping, mirrored | 24 | +9.3 ± 1.35 | +7.6 ± 1.32 | +1.7 ± 0.68 | −2.0 ± 1.25 | +11.3 ± 1.34 |
| Seed 24, first mapping, mirrored | 36 | +7.4 ± 1.09 | +6.0 ± 1.54 | +1.4 ± 0.86 | −3.2 ± 0.94 | +10.6 ± 0.72 |

- **The excitatory night less H-47's plain night** lies between −4.0 and +8.1, beyond twice its unpaired SE at 17 of the 40 checkpoints,
  11 above and 6 below, and at most 5.30 times it. Where A answers readout 0 it lies beyond at 12 of 30, 6 above and 6 below: above on
  seed 24's eighth image from the mirrored assignment at every checkpoint and on seed 24's first mapping from the assignment at block 36;
  below on seed 23's eighth image from the mirrored assignment from block 12 on and on seed 22's two cells at block 36. Where A answers
  readout 1 it lies beyond at 5 of 10, all above: on seed 23's eighth image from the assignment from block 12 on and on seed 24's first
  mapping from the mirrored assignment at block 24. On seed 23's first mapping from the assignment it lies within 2 SE at every checkpoint.
- **The excitatory night less the night without replay** lies beyond twice its SE at 37 of the 40 checkpoints, all above; it lies within
  2 SE on seed 22's first mapping from the assignment at block 12, +1.0 ± 1.21, and on seed 24's eighth image from the mirrored assignment
  at blocks 6 and 12, +0.3 ± 1.15 and +2.1 ± 1.50. H-47's plain night lies beyond at 38: 37 above and one below, seed 24's eighth image
  from the mirrored assignment at block 12, −2.9 ± 1.19.

The forms without replay, and the forms against each other, each pair run in one job:
- **The excitatory night without replay less the night without replay** lies between −5.4 and +8.1, beyond twice its SE at 15 of the 40
  checkpoints: above at 7, among them seed 23's eighth image from the mirrored assignment from block 12 on, +4.0 ± 0.68 to +8.1 ± 0.86, and
  below at 8, among them seed 24's eighth image from the mirrored assignment at blocks 6 and 12, −3.9 ± 0.53 and −5.4 ± 1.12. Where A
  answers readout 1 it lies beyond at 2 of 10: above on seed 23's eighth image from the assignment at block 6, +0.8 ± 0.40, and below on
  seed 24's first mapping from the mirrored assignment at block 36, −3.2 ± 0.94.
- **The excitatory night less the excitatory night without replay**, the replay with the hold set in both, lies between +1.0 and +17.3,
  beyond twice its SE at 36 of the 40 checkpoints, all above.

Each stimulus's correct trials, the mean of the 48 reads, under each form:

| Cell | Block | A's: without replay | excitatory | excitatory without replay | plain on A's | B's: without replay | excitatory | excitatory without replay | plain on A's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 24.8 | 29.4 | 25.0 | 29.0 | 21.5 | 23.0 | 20.8 | 24.4 |
| Seed 22, eighth, mirrored | 12 | 11.3 | 20.1 | 12.6 | 19.1 | 18.9 | 20.8 | 17.2 | 23.2 |
| Seed 22, eighth, mirrored | 18 | 9.5 | 19.0 | 8.8 | 18.1 | 16.8 | 21.9 | 17.1 | 23.7 |
| Seed 22, eighth, mirrored | 24 | 8.6 | 15.8 | 6.5 | 14.7 | 14.0 | 21.3 | 14.6 | 25.5 |
| Seed 22, eighth, mirrored | 36 | 4.7 | 15.0 | 3.8 | 13.9 | 11.9 | 17.8 | 11.7 | 22.3 |
| Seed 22, first mapping, assignment | 6 | 25.5 | 29.7 | 25.3 | 29.6 | 26.9 | 27.1 | 26.5 | 26.9 |
| Seed 22, first mapping, assignment | 12 | 21.1 | 25.8 | 21.5 | 24.9 | 23.6 | 20.0 | 22.1 | 22.7 |
| Seed 22, first mapping, assignment | 18 | 16.4 | 25.0 | 18.5 | 24.8 | 23.7 | 21.3 | 21.9 | 22.2 |
| Seed 22, first mapping, assignment | 24 | 15.5 | 23.2 | 17.7 | 23.8 | 23.4 | 22.4 | 22.2 | 24.3 |
| Seed 22, first mapping, assignment | 36 | 13.2 | 21.3 | 14.9 | 21.5 | 23.1 | 20.3 | 21.6 | 22.7 |
| Seed 23, eighth, mirrored | 6 | 20.0 | 21.8 | 20.2 | 22.5 | 14.4 | 15.0 | 14.4 | 13.6 |
| Seed 23, eighth, mirrored | 12 | 15.3 | 18.0 | 18.1 | 21.5 | 4.2 | 7.4 | 5.4 | 7.0 |
| Seed 23, eighth, mirrored | 18 | 12.3 | 19.5 | 15.9 | 24.4 | 2.6 | 4.5 | 4.7 | 3.2 |
| Seed 23, eighth, mirrored | 24 | 8.9 | 21.4 | 14.0 | 25.8 | 1.7 | 3.8 | 4.6 | 3.5 |
| Seed 23, eighth, mirrored | 36 | 7.4 | 20.9 | 11.6 | 23.3 | 0.8 | 2.0 | 1.8 | 2.3 |
| Seed 23, first mapping, assignment | 6 | 29.2 | 30.8 | 30.2 | 30.9 | 25.0 | 25.3 | 24.9 | 25.4 |
| Seed 23, first mapping, assignment | 12 | 24.2 | 29.6 | 25.7 | 30.1 | 9.3 | 7.9 | 9.0 | 7.4 |
| Seed 23, first mapping, assignment | 18 | 19.5 | 29.2 | 22.3 | 30.1 | 7.6 | 5.9 | 7.9 | 4.9 |
| Seed 23, first mapping, assignment | 24 | 17.3 | 29.2 | 19.0 | 30.4 | 7.5 | 5.8 | 8.6 | 4.3 |
| Seed 23, first mapping, assignment | 36 | 15.7 | 28.0 | 19.4 | 29.2 | 4.0 | 4.3 | 6.0 | 3.9 |
| Seed 24, eighth, mirrored | 6 | 19.1 | 18.9 | 16.3 | 17.0 | 17.3 | 17.8 | 16.2 | 16.8 |
| Seed 24, eighth, mirrored | 12 | 15.3 | 16.2 | 10.6 | 12.1 | 16.3 | 17.4 | 15.5 | 16.6 |
| Seed 24, eighth, mirrored | 18 | 9.2 | 16.0 | 8.3 | 9.9 | 11.0 | 18.5 | 14.5 | 18.6 |
| Seed 24, eighth, mirrored | 24 | 7.4 | 16.6 | 5.8 | 9.0 | 9.0 | 17.3 | 12.2 | 16.8 |
| Seed 24, eighth, mirrored | 36 | 5.3 | 13.3 | 3.6 | 6.9 | 7.2 | 15.9 | 10.5 | 16.9 |
| Seed 24, first mapping, assignment | 6 | 28.8 | 30.6 | 28.6 | 30.7 | 29.5 | 30.6 | 29.9 | 30.6 |
| Seed 24, first mapping, assignment | 12 | 26.4 | 29.0 | 25.6 | 28.6 | 25.6 | 26.6 | 25.7 | 26.2 |
| Seed 24, first mapping, assignment | 18 | 24.9 | 29.1 | 23.6 | 28.9 | 22.6 | 23.4 | 20.5 | 22.5 |
| Seed 24, first mapping, assignment | 24 | 22.3 | 29.1 | 23.4 | 28.4 | 21.4 | 23.6 | 18.5 | 24.0 |
| Seed 24, first mapping, assignment | 36 | 21.2 | 26.1 | 20.9 | 24.5 | 18.8 | 19.7 | 16.9 | 19.3 |
| Seed 23, eighth, assignment | 6 | 24.3 | 27.0 | 23.9 | 26.9 | 23.1 | 23.7 | 24.3 | 23.4 |
| Seed 23, eighth, assignment | 12 | 18.3 | 23.2 | 15.3 | 21.2 | 22.6 | 24.3 | 23.4 | 22.2 |
| Seed 23, eighth, assignment | 18 | 15.6 | 24.4 | 14.4 | 20.8 | 21.7 | 25.7 | 21.9 | 24.4 |
| Seed 23, eighth, assignment | 24 | 16.3 | 25.2 | 14.5 | 19.6 | 21.7 | 25.8 | 23.0 | 24.1 |
| Seed 23, eighth, assignment | 36 | 13.0 | 21.0 | 8.9 | 15.3 | 23.2 | 26.1 | 24.5 | 25.0 |
| Seed 24, first mapping, mirrored | 6 | 28.1 | 30.2 | 28.2 | 30.1 | 26.2 | 27.8 | 26.2 | 27.6 |
| Seed 24, first mapping, mirrored | 12 | 24.6 | 27.2 | 23.7 | 27.3 | 21.7 | 25.1 | 22.0 | 23.8 |
| Seed 24, first mapping, mirrored | 18 | 21.7 | 26.8 | 18.8 | 27.0 | 22.3 | 26.5 | 24.0 | 24.8 |
| Seed 24, first mapping, mirrored | 24 | 22.1 | 26.1 | 17.4 | 25.8 | 22.3 | 27.6 | 25.0 | 26.2 |
| Seed 24, first mapping, mirrored | 36 | 22.0 | 25.5 | 16.4 | 25.2 | 20.8 | 24.6 | 23.1 | 23.5 |

## 2. The side under each held night

The side under a held night is the stimulus whose difference from the night without replay is the larger. "A's less B's" is the paired
difference of the two, and "within 2 SE" marks it within twice that SE. H-47's side is the same reading of H-47's plain night, from H-47's
job. The group named is the one that stimulus answers in that cell:

| Cell | Block | A's: excitatory less without replay | B's | A's less B's | Side under the excitatory night | A's: H-47's plain less without replay | B's | A's less B's | H-47's side |
| :--- | ---: | ---: | ---: | ---: | :--- | ---: | ---: | ---: | :--- |
| Seed 22, eighth, mirrored | 6 | +4.6 ± 0.91 | +1.5 ± 1.10 | +3.1 ± 1.76 | A, the odd places (within 2 SE) | +4.2 ± 0.89 | +2.9 ± 1.07 | +1.4 ± 1.74 | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | 12 | +8.8 ± 0.97 | +2.0 ± 0.60 | +6.9 ± 1.29 | A, the odd places | +7.8 ± 1.13 | +4.3 ± 0.84 | +3.5 ± 1.80 | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | 18 | +9.5 ± 1.69 | +5.1 ± 1.07 | +4.4 ± 2.20 | A, the odd places (within 2 SE) | +8.6 ± 1.79 | +6.9 ± 1.22 | +1.7 ± 2.57 | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | 24 | +7.1 ± 1.03 | +7.4 ± 0.83 | −0.2 ± 1.16 | B, the even places (within 2 SE) | +6.1 ± 1.14 | +11.6 ± 1.20 | −5.5 ± 1.80 | B, the even places |
| Seed 22, eighth, mirrored | 36 | +10.3 ± 0.50 | +5.9 ± 0.45 | +4.4 ± 0.32 | A, the odd places | +9.2 ± 0.86 | +10.4 ± 0.54 | −1.2 ± 0.97 | B, the even places (within 2 SE) |
| Seed 22, first mapping, assignment | 6 | +4.2 ± 0.43 | +0.2 ± 0.68 | +4.0 ± 0.34 | A, the odd places | +4.1 ± 0.45 | 0.0 ± 0.50 | +4.1 ± 0.49 | A, the odd places |
| Seed 22, first mapping, assignment | 12 | +4.6 ± 0.92 | −3.6 ± 0.97 | +8.2 ± 1.44 | A, the odd places | +3.8 ± 0.48 | −0.9 ± 0.40 | +4.7 ± 0.68 | A, the odd places |
| Seed 22, first mapping, assignment | 18 | +8.5 ± 1.36 | −2.5 ± 1.07 | +11.0 ± 2.18 | A, the odd places | +8.3 ± 0.81 | −1.6 ± 0.69 | +9.9 ± 1.00 | A, the odd places |
| Seed 22, first mapping, assignment | 24 | +7.8 ± 1.24 | −1.0 ± 0.94 | +8.8 ± 1.76 | A, the odd places | +8.3 ± 1.55 | +0.8 ± 0.85 | +7.5 ± 2.09 | A, the odd places |
| Seed 22, first mapping, assignment | 36 | +8.1 ± 1.12 | −2.9 ± 0.36 | +11.0 ± 1.12 | A, the odd places | +8.4 ± 1.10 | −0.4 ± 0.83 | +8.8 ± 1.88 | A, the odd places |
| Seed 23, eighth, mirrored | 6 | +1.8 ± 0.80 | +0.6 ± 0.87 | +1.2 ± 1.27 | A, the odd places (within 2 SE) | +2.5 ± 0.98 | −0.8 ± 0.90 | +3.4 ± 1.46 | A, the odd places |
| Seed 23, eighth, mirrored | 12 | +2.7 ± 0.48 | +3.3 ± 0.91 | −0.6 ± 1.07 | B, the even places (within 2 SE) | +6.1 ± 0.90 | +2.8 ± 0.44 | +3.3 ± 0.93 | A, the odd places |
| Seed 23, eighth, mirrored | 18 | +7.3 ± 0.99 | +1.9 ± 0.96 | +5.4 ± 1.30 | A, the odd places | +12.1 ± 1.05 | +0.6 ± 0.57 | +11.5 ± 1.34 | A, the odd places |
| Seed 23, eighth, mirrored | 24 | +12.5 ± 1.26 | +2.1 ± 0.79 | +10.4 ± 0.75 | A, the odd places | +16.9 ± 1.43 | +1.8 ± 0.26 | +15.1 ± 1.26 | A, the odd places |
| Seed 23, eighth, mirrored | 36 | +13.5 ± 1.05 | +1.2 ± 0.54 | +12.3 ± 0.63 | A, the odd places | +15.9 ± 1.05 | +1.5 ± 0.28 | +14.4 ± 1.09 | A, the odd places |
| Seed 23, first mapping, assignment | 6 | +1.6 ± 0.51 | +0.3 ± 0.70 | +1.3 ± 0.86 | A, the odd places (within 2 SE) | +1.6 ± 0.46 | +0.4 ± 0.57 | +1.3 ± 0.67 | A, the odd places (within 2 SE) |
| Seed 23, first mapping, assignment | 12 | +5.4 ± 0.86 | −1.3 ± 1.11 | +6.7 ± 1.74 | A, the odd places | +6.0 ± 0.87 | −1.8 ± 0.85 | +7.8 ± 1.24 | A, the odd places |
| Seed 23, first mapping, assignment | 18 | +9.7 ± 1.80 | −1.8 ± 0.90 | +11.4 ± 1.54 | A, the odd places | +10.6 ± 1.65 | −2.8 ± 1.24 | +13.4 ± 1.44 | A, the odd places |
| Seed 23, first mapping, assignment | 24 | +12.0 ± 1.66 | −1.7 ± 0.85 | +13.7 ± 1.46 | A, the odd places | +13.2 ± 1.45 | −3.2 ± 0.67 | +16.4 ± 1.23 | A, the odd places |
| Seed 23, first mapping, assignment | 36 | +12.3 ± 1.11 | +0.3 ± 0.73 | +12.0 ± 1.06 | A, the odd places | +13.5 ± 1.09 | −0.1 ± 0.62 | +13.6 ± 0.84 | A, the odd places |
| Seed 24, eighth, mirrored | 6 | −0.3 ± 0.69 | +0.5 ± 0.94 | −0.8 ± 1.17 | B, the even places (within 2 SE) | −2.1 ± 0.99 | −0.5 ± 1.17 | −1.6 ± 1.69 | B, the even places (within 2 SE) |
| Seed 24, eighth, mirrored | 12 | +1.0 ± 0.97 | +1.1 ± 1.17 | −0.1 ± 1.55 | B, the even places (within 2 SE) | −3.2 ± 0.72 | +0.3 ± 0.77 | −3.5 ± 0.91 | B, the even places |
| Seed 24, eighth, mirrored | 18 | +6.7 ± 0.95 | +7.5 ± 0.88 | −0.7 ± 1.25 | B, the even places (within 2 SE) | +0.7 ± 0.25 | +7.6 ± 1.52 | −6.9 ± 1.32 | B, the even places |
| Seed 24, eighth, mirrored | 24 | +9.2 ± 0.71 | +8.3 ± 0.80 | +0.9 ± 0.78 | A, the odd places (within 2 SE) | +1.6 ± 1.33 | +7.8 ± 1.47 | −6.2 ± 2.32 | B, the even places |
| Seed 24, eighth, mirrored | 36 | +7.9 ± 0.60 | +8.8 ± 0.85 | −0.8 ± 0.81 | B, the even places (within 2 SE) | +1.5 ± 0.74 | +9.7 ± 0.98 | −8.2 ± 1.45 | B, the even places |
| Seed 24, first mapping, assignment | 6 | +1.8 ± 0.60 | +1.1 ± 0.31 | +0.7 ± 0.66 | A, the odd places (within 2 SE) | +1.9 ± 0.57 | +1.1 ± 0.38 | +0.8 ± 0.65 | A, the odd places (within 2 SE) |
| Seed 24, first mapping, assignment | 12 | +2.7 ± 0.63 | +1.0 ± 1.22 | +1.7 ± 1.56 | A, the odd places (within 2 SE) | +2.2 ± 0.43 | +0.6 ± 0.99 | +1.6 ± 1.22 | A, the odd places (within 2 SE) |
| Seed 24, first mapping, assignment | 18 | +4.3 ± 0.45 | +0.8 ± 1.19 | +3.5 ± 1.34 | A, the odd places | +4.0 ± 0.73 | −0.1 ± 1.55 | +4.1 ± 1.97 | A, the odd places |
| Seed 24, first mapping, assignment | 24 | +6.8 ± 1.01 | +2.2 ± 1.68 | +4.6 ± 1.28 | A, the odd places | +6.1 ± 1.25 | +2.6 ± 2.15 | +3.5 ± 1.53 | A, the odd places |
| Seed 24, first mapping, assignment | 36 | +4.9 ± 0.68 | +0.8 ± 1.06 | +4.1 ± 0.78 | A, the odd places | +3.3 ± 0.54 | +0.5 ± 1.44 | +2.8 ± 1.52 | A, the odd places (within 2 SE) |
| Seed 23, eighth, assignment | 6 | +2.7 ± 0.67 | +0.6 ± 0.59 | +2.1 ± 0.67 | A, the even places | +2.6 ± 0.75 | +0.4 ± 0.66 | +2.3 ± 1.04 | A, the even places |
| Seed 23, eighth, assignment | 12 | +5.0 ± 0.99 | +1.7 ± 1.02 | +3.3 ± 1.26 | A, the even places | +2.9 ± 0.87 | −0.4 ± 0.64 | +3.3 ± 1.21 | A, the even places |
| Seed 23, eighth, assignment | 18 | +8.8 ± 0.61 | +4.0 ± 1.18 | +4.8 ± 1.19 | A, the even places | +5.2 ± 0.90 | +2.7 ± 0.71 | +2.6 ± 1.35 | A, the even places (within 2 SE) |
| Seed 23, eighth, assignment | 24 | +8.9 ± 1.14 | +4.1 ± 1.31 | +4.8 ± 1.22 | A, the even places | +3.3 ± 1.29 | +2.5 ± 0.79 | +0.8 ± 1.67 | A, the even places (within 2 SE) |
| Seed 23, eighth, assignment | 36 | +8.0 ± 0.58 | +2.9 ± 2.40 | +5.1 ± 2.45 | A, the even places | +2.3 ± 0.88 | +1.8 ± 0.84 | +0.5 ± 0.92 | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | 6 | +2.1 ± 0.54 | +1.5 ± 0.63 | +0.6 ± 0.78 | A, the even places (within 2 SE) | +2.0 ± 0.50 | +1.4 ± 0.59 | +0.7 ± 0.76 | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | 12 | +2.6 ± 0.67 | +3.4 ± 1.04 | −0.9 ± 1.16 | B, the odd places (within 2 SE) | +2.6 ± 0.61 | +2.1 ± 1.01 | +0.6 ± 0.73 | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | 18 | +5.1 ± 0.45 | +4.2 ± 0.94 | +0.9 ± 1.17 | A, the even places (within 2 SE) | +5.3 ± 0.46 | +2.5 ± 0.95 | +2.9 ± 1.20 | A, the even places |
| Seed 24, first mapping, mirrored | 24 | +4.0 ± 0.77 | +5.4 ± 0.67 | −1.4 ± 0.52 | B, the odd places | +3.7 ± 0.86 | +4.0 ± 0.60 | −0.3 ± 0.68 | B, the odd places (within 2 SE) |
| Seed 24, first mapping, mirrored | 36 | +3.5 ± 0.50 | +3.9 ± 0.87 | −0.3 ± 0.91 | B, the odd places (within 2 SE) | +3.3 ± 0.67 | +2.7 ± 0.99 | +0.5 ± 0.67 | A, the even places (within 2 SE) |

- **The side under the excitatory night is H-47's at 35 of the 40 checkpoints.** It lies beyond twice its SE at 23 of the 40, and H-47's
  at 24. It differs at five:
  - seed 22's eighth image from the mirrored assignment at block 36: on A, the odd places, beyond 2 SE, where H-47's lies on B, the even
    places, within 2 SE;
  - seed 23's eighth image from the mirrored assignment at block 12: on B, the even places, within 2 SE, where H-47's lies on A, the odd
    places, beyond;
  - seed 24's eighth image from the mirrored assignment at block 24: on A, the odd places, within 2 SE, where H-47's lies on B, the even
    places, beyond;
  - seed 24's first mapping from the mirrored assignment at blocks 12 and 36: on B, the odd places, within 2 SE, where H-47's lies on A,
    the even places, within 2 SE.
- **On A at every checkpoint, under the excitatory night and H-47's plain night alike**: on seed 22's, seed 23's and seed 24's first
  mappings from the assignment, where A answers the odd places, and on seed 23's eighth image from the assignment, where it answers the
  even places. On seed 24's eighth image from the mirrored assignment the excitatory night's side lies on B, the even places, at four of the
  five checkpoints, within 2 SE at each, and H-47's on B at all five.

Each stimulus's trials less H-47's plain night's, each with exploration 018's unpaired SE:

| Cell | Block | A's: excitatory less plain | B's: excitatory less plain |
| :--- | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +0.4 ± 0.93 | −1.3 ± 0.99 |
| Seed 22, eighth, mirrored | 12 | +1.0 ± 0.95 | −2.4 ± 0.95 |
| Seed 22, eighth, mirrored | 18 | +0.9 ± 0.91 | −1.8 ± 1.13 |
| Seed 22, eighth, mirrored | 24 | +1.1 ± 0.75 | −4.2 ± 1.67 |
| Seed 22, eighth, mirrored | 36 | +1.1 ± 1.07 | −4.5 ± 1.08 |
| Seed 22, first mapping, assignment | 6 | +0.1 ± 0.81 | +0.2 ± 0.76 |
| Seed 22, first mapping, assignment | 12 | +0.8 ± 0.89 | −2.6 ± 1.30 |
| Seed 22, first mapping, assignment | 18 | +0.2 ± 1.16 | −0.9 ± 1.14 |
| Seed 22, first mapping, assignment | 24 | −0.5 ± 0.75 | −1.9 ± 1.29 |
| Seed 22, first mapping, assignment | 36 | −0.3 ± 0.88 | −2.5 ± 1.04 |
| Seed 23, eighth, mirrored | 6 | −0.7 ± 1.15 | +1.4 ± 0.93 |
| Seed 23, eighth, mirrored | 12 | −3.4 ± 1.10 | +0.4 ± 0.91 |
| Seed 23, eighth, mirrored | 18 | −4.8 ± 0.96 | +1.3 ± 0.70 |
| Seed 23, eighth, mirrored | 24 | −4.4 ± 0.90 | +0.4 ± 0.75 |
| Seed 23, eighth, mirrored | 36 | −2.4 ± 0.95 | −0.3 ± 0.61 |
| Seed 23, first mapping, assignment | 6 | −0.1 ± 0.84 | −0.1 ± 1.03 |
| Seed 23, first mapping, assignment | 12 | −0.6 ± 0.96 | +0.5 ± 0.92 |
| Seed 23, first mapping, assignment | 18 | −0.9 ± 1.15 | +1.0 ± 0.92 |
| Seed 23, first mapping, assignment | 24 | −1.2 ± 0.91 | +1.5 ± 0.82 |
| Seed 23, first mapping, assignment | 36 | −1.2 ± 0.97 | +0.4 ± 0.56 |
| Seed 24, eighth, mirrored | 6 | +1.9 ± 1.16 | +1.0 ± 1.02 |
| Seed 24, eighth, mirrored | 12 | +4.2 ± 1.11 | +0.8 ± 1.24 |
| Seed 24, eighth, mirrored | 18 | +6.0 ± 0.85 | −0.2 ± 1.50 |
| Seed 24, eighth, mirrored | 24 | +7.6 ± 1.38 | +0.5 ± 1.39 |
| Seed 24, eighth, mirrored | 36 | +6.4 ± 1.40 | −1.0 ± 0.81 |
| Seed 24, first mapping, assignment | 6 | −0.1 ± 1.02 | 0.0 ± 0.93 |
| Seed 24, first mapping, assignment | 12 | +0.5 ± 0.98 | +0.4 ± 1.06 |
| Seed 24, first mapping, assignment | 18 | +0.3 ± 0.98 | +0.9 ± 1.12 |
| Seed 24, first mapping, assignment | 24 | +0.7 ± 0.92 | −0.4 ± 1.08 |
| Seed 24, first mapping, assignment | 36 | +1.7 ± 1.22 | +0.3 ± 0.83 |
| Seed 23, eighth, assignment | 6 | +0.1 ± 0.82 | +0.3 ± 1.20 |
| Seed 23, eighth, assignment | 12 | +2.0 ± 1.51 | +2.1 ± 1.42 |
| Seed 23, eighth, assignment | 18 | +3.5 ± 0.96 | +1.3 ± 1.96 |
| Seed 23, eighth, assignment | 24 | +5.6 ± 1.05 | +1.6 ± 1.72 |
| Seed 23, eighth, assignment | 36 | +5.8 ± 1.36 | +1.1 ± 1.86 |
| Seed 24, first mapping, mirrored | 6 | +0.1 ± 0.98 | +0.2 ± 0.93 |
| Seed 24, first mapping, mirrored | 12 | −0.1 ± 0.99 | +1.4 ± 1.07 |
| Seed 24, first mapping, mirrored | 18 | −0.2 ± 1.11 | +1.7 ± 0.87 |
| Seed 24, first mapping, mirrored | 24 | +0.3 ± 0.98 | +1.4 ± 1.05 |
| Seed 24, first mapping, mirrored | 36 | +0.3 ± 0.63 | +1.1 ± 0.83 |

- **Under the excitatory night less the plain night**, A's trials lie between −4.8 and +7.6, beyond 2 SE at 11 of the 40 checkpoints:
  below on seed 23's eighth image from the mirrored assignment from block 12 on, −2.4 ± 0.95 to −4.8 ± 0.96; above on seed 24's eighth
  image from the mirrored assignment from block 12 on, +4.2 ± 1.11 to +7.6 ± 1.38, and on seed 23's eighth image from the assignment from
  block 18 on, +3.5 ± 0.96 to +5.8 ± 1.36. B's lie between −4.5 and +2.1, beyond 2 SE at 5 of 40, all below and all on seed 22's two
  cells: its eighth image at blocks 12, 24 and 36, and its first mapping at blocks 12 and 36.

Under the excitatory night without replay, its side, read the same way against the night without replay:

| Cell | Block | A's: excitatory without replay less without replay | B's: the same | A's less B's | Its side |
| :--- | ---: | ---: | ---: | ---: | :--- |
| Seed 22, eighth, mirrored | 6 | +0.3 ± 0.38 | −0.7 ± 0.40 | +1.0 ± 0.58 | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | 12 | +1.2 ± 0.58 | −1.7 ± 0.37 | +2.9 ± 0.71 | A, the odd places |
| Seed 22, eighth, mirrored | 18 | −0.7 ± 0.63 | +0.3 ± 0.84 | −1.0 ± 0.71 | B, the even places (within 2 SE) |
| Seed 22, eighth, mirrored | 24 | −2.1 ± 0.62 | +0.6 ± 0.57 | −2.8 ± 0.81 | B, the even places |
| Seed 22, eighth, mirrored | 36 | −0.9 ± 0.41 | −0.2 ± 0.38 | −0.7 ± 0.60 | B, the even places (within 2 SE) |
| Seed 22, first mapping, assignment | 6 | −0.2 ± 0.20 | −0.4 ± 0.23 | +0.2 ± 0.35 | A, the odd places (within 2 SE) |
| Seed 22, first mapping, assignment | 12 | +0.4 ± 0.41 | −1.5 ± 0.38 | +1.9 ± 0.62 | A, the odd places |
| Seed 22, first mapping, assignment | 18 | +2.0 ± 0.84 | −1.8 ± 0.61 | +3.8 ± 0.83 | A, the odd places |
| Seed 22, first mapping, assignment | 24 | +2.2 ± 0.79 | −1.3 ± 0.52 | +3.5 ± 0.73 | A, the odd places |
| Seed 22, first mapping, assignment | 36 | +1.8 ± 0.98 | −1.6 ± 1.51 | +3.3 ± 2.32 | A, the odd places (within 2 SE) |
| Seed 23, eighth, mirrored | 6 | +0.2 ± 0.50 | 0.0 ± 0.66 | +0.2 ± 0.84 | A, the odd places (within 2 SE) |
| Seed 23, eighth, mirrored | 12 | +2.8 ± 0.38 | +1.3 ± 0.52 | +1.5 ± 0.61 | A, the odd places |
| Seed 23, eighth, mirrored | 18 | +3.7 ± 0.61 | +2.1 ± 0.45 | +1.6 ± 0.83 | A, the odd places (within 2 SE) |
| Seed 23, eighth, mirrored | 24 | +5.1 ± 0.90 | +3.0 ± 0.56 | +2.1 ± 1.23 | A, the odd places (within 2 SE) |
| Seed 23, eighth, mirrored | 36 | +4.2 ± 0.66 | +1.0 ± 0.19 | +3.2 ± 0.61 | A, the odd places |
| Seed 23, first mapping, assignment | 6 | +1.0 ± 0.26 | −0.1 ± 0.36 | +1.1 ± 0.46 | A, the odd places |
| Seed 23, first mapping, assignment | 12 | +1.5 ± 0.82 | −0.3 ± 0.96 | +1.8 ± 1.42 | A, the odd places (within 2 SE) |
| Seed 23, first mapping, assignment | 18 | +2.8 ± 1.59 | +0.2 ± 1.22 | +2.6 ± 1.42 | A, the odd places (within 2 SE) |
| Seed 23, first mapping, assignment | 24 | +1.8 ± 1.50 | +1.0 ± 0.58 | +0.8 ± 1.58 | A, the odd places (within 2 SE) |
| Seed 23, first mapping, assignment | 36 | +3.7 ± 1.52 | +2.1 ± 0.78 | +1.6 ± 2.15 | A, the odd places (within 2 SE) |
| Seed 24, eighth, mirrored | 6 | −2.8 ± 0.39 | −1.1 ± 0.49 | −1.8 ± 0.71 | B, the even places |
| Seed 24, eighth, mirrored | 12 | −4.6 ± 0.49 | −0.7 ± 0.82 | −3.9 ± 0.77 | B, the even places |
| Seed 24, eighth, mirrored | 18 | −0.9 ± 0.64 | +3.5 ± 0.62 | −4.5 ± 0.98 | B, the even places |
| Seed 24, eighth, mirrored | 24 | −1.6 ± 0.75 | +3.2 ± 0.97 | −4.8 ± 1.45 | B, the even places |
| Seed 24, eighth, mirrored | 36 | −1.8 ± 0.56 | +3.3 ± 0.96 | −5.1 ± 1.24 | B, the even places |
| Seed 24, first mapping, assignment | 6 | −0.2 ± 0.25 | +0.4 ± 0.15 | −0.6 ± 0.27 | B, the even places |
| Seed 24, first mapping, assignment | 12 | −0.8 ± 0.32 | 0.0 ± 0.85 | −0.8 ± 1.04 | B, the even places (within 2 SE) |
| Seed 24, first mapping, assignment | 18 | −1.3 ± 0.27 | −2.0 ± 1.12 | +0.8 ± 1.08 | A, the odd places (within 2 SE) |
| Seed 24, first mapping, assignment | 24 | +1.1 ± 1.17 | −2.9 ± 1.35 | +4.1 ± 1.48 | A, the odd places |
| Seed 24, first mapping, assignment | 36 | −0.4 ± 1.02 | −1.9 ± 1.13 | +1.5 ± 1.94 | A, the odd places (within 2 SE) |
| Seed 23, eighth, assignment | 6 | −0.4 ± 0.10 | +1.2 ± 0.31 | −1.6 ± 0.19 | B, the odd places |
| Seed 23, eighth, assignment | 12 | −2.9 ± 0.82 | +0.8 ± 0.52 | −3.8 ± 0.55 | B, the odd places |
| Seed 23, eighth, assignment | 18 | −1.2 ± 0.68 | +0.1 ± 0.76 | −1.4 ± 0.85 | B, the odd places (within 2 SE) |
| Seed 23, eighth, assignment | 24 | −1.8 ± 0.59 | +1.4 ± 1.41 | −3.2 ± 1.32 | B, the odd places |
| Seed 23, eighth, assignment | 36 | −4.1 ± 0.67 | +1.3 ± 1.40 | −5.4 ± 1.46 | B, the odd places |
| Seed 24, first mapping, mirrored | 6 | +0.1 ± 0.34 | 0.0 ± 0.21 | +0.1 ± 0.49 | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | 12 | −0.9 ± 0.31 | +0.3 ± 0.59 | −1.1 ± 0.55 | B, the odd places |
| Seed 24, first mapping, mirrored | 18 | −3.0 ± 0.62 | +1.7 ± 0.61 | −4.7 ± 0.71 | B, the odd places |
| Seed 24, first mapping, mirrored | 24 | −4.7 ± 0.66 | +2.7 ± 0.66 | −7.4 ± 0.45 | B, the odd places |
| Seed 24, first mapping, mirrored | 36 | −5.5 ± 0.48 | +2.4 ± 0.47 | −7.9 ± 0.17 | B, the odd places |

Its side is H-47's at 29 of the 40 checkpoints and lies beyond twice its SE at 23. It lies on A, the odd places, at every checkpoint on
seed 22's first mapping and on seed 23's eighth image from the mirrored assignment and first mapping; on B at every checkpoint on seed 24's
eighth image, the even places, and on seed 23's eighth image from the assignment, the odd places; and on B, the odd places, from block 12 on,
on seed 24's first mapping from the mirrored assignment.

## 3. The pair and A's couplings

The pair's count under the excitatory night at each checkpoint, and at block 36 under the other forms. H-47's floor for clause 2 is 56:

| Cell | Excitatory at 6 | 12 | 18 | 24 | 36 | At 36: without replay | excitatory without replay | plain on A's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 44.6 | 62.8 | 62.3 | 62.7 | 62.9 | 63.0 | 63.1 | 62.8 |
| Seed 22, first mapping, assignment | 41.7 | 61.6 | 61.1 | 61.8 | 62.7 | 63.3 | 62.8 | 63.1 |
| Seed 23, eighth, mirrored | 57.6 | 62.0 | 61.9 | 61.5 | 62.8 | 62.6 | 62.8 | 63.1 |
| Seed 23, first mapping, assignment | 27.3 | 56.2 | 60.9 | 59.9 | 61.9 | 63.1 | 63.0 | 61.9 |
| Seed 24, eighth, mirrored | 61.6 | 62.5 | 62.1 | 61.9 | 63.0 | 62.7 | 62.9 | 63.0 |
| Seed 24, first mapping, assignment | 34.8 | 51.2 | 59.9 | 58.7 | 61.7 | 62.1 | 62.5 | 61.8 |
| Seed 23, eighth, assignment | 55.4 | 62.0 | 61.9 | 62.7 | 62.8 | 63.4 | 63.3 | 63.4 |
| Seed 24, first mapping, mirrored | 51.7 | 60.7 | 61.4 | 61.6 | 62.4 | 63.4 | 63.2 | 62.4 |

A's highest coupling over H-42's under each form, in any image read and at any block's end, as clause 3 reads it; its bound is 1.30:

| Cell | Without replay | Excitatory | Excitatory without replay | Plain on A's (H-47) |
| :--- | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 1.144 | 1.201 | 1.144 | 1.197 |
| Seed 22, first mapping, assignment | 1.112 | 1.202 | 1.112 | 1.198 |
| Seed 23, eighth, mirrored | 1.147 | 1.212 | 1.147 | 1.211 |
| Seed 23, first mapping, assignment | 1.125 | 1.212 | 1.125 | 1.212 |
| Seed 24, eighth, mirrored | 1.078 | 1.146 | 1.078 | 1.141 |
| Seed 24, first mapping, assignment | 1.085 | 1.165 | 1.085 | 1.162 |
| Seed 23, eighth, assignment | 1.118 | 1.162 | 1.118 | 1.171 |
| Seed 24, first mapping, mirrored | 1.177 | 1.236 | 1.177 | 1.223 |

Under the excitatory night the pair stands at 61.7 to 63.0 at block 36, where H-47's plain night gave 61.8 to 63.4, the night without
replay 62.1 to 63.4 and the excitatory night without replay 62.5 to 63.3. A's highest coupling is 1.146 to 1.236 of H-42's under the
excitatory night, where H-47's plain night gave 1.141 to 1.223; under the excitatory night without replay it is the night without replay's
in every cell, 1.078 to 1.177.

## 4. What each night moved

What the four nights moved, summed over the four, the mean over the three paths, from the class matrix in the images before and after each
night, in M. A group's held units are its readout's two classes, the pair's own kind left out; "onto their own group" counts the pair's own
kind among the targets, as exploration 025's did. H-47's plain night is exploration 021's run of exploration 020's job:

| Cell | Form | The odd places' held units onto their own group | onto the even places' excitatory | onto the inhibitory units | onto A's and B's units | onto the other excitatory | The even places' held units onto their own group | onto the odd places' excitatory | onto the inhibitory units | onto A's and B's units | onto the other excitatory |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | without replay | −0.50 | −0.47 | +1.28 | −0.02 | +0.05 | −0.74 | −0.65 | +1.02 | −0.18 | −0.03 |
| Seed 22, eighth, mirrored | excitatory | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, mirrored | excitatory without replay | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, mirrored | plain on A's (H-47) | −0.66 | −5.98 | −2.19 | −0.40 | 0.00 | −0.92 | −6.72 | −1.96 | −0.74 | −0.01 |
| Seed 22, first mapping, assignment | without replay | −0.45 | −0.52 | +1.49 | −0.04 | +0.04 | −0.75 | −0.61 | +1.41 | −0.24 | −0.03 |
| Seed 22, first mapping, assignment | excitatory | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, assignment | excitatory without replay | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, assignment | plain on A's (H-47) | −0.73 | −5.66 | −2.02 | −0.25 | +0.01 | −0.76 | −6.32 | −1.56 | −0.73 | −0.01 |
| Seed 23, eighth, mirrored | without replay | +0.23 | −0.01 | +1.35 | −0.04 | −0.03 | −0.20 | −0.59 | +1.34 | −0.19 | 0.00 |
| Seed 23, eighth, mirrored | excitatory | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, mirrored | excitatory without replay | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, mirrored | plain on A's (H-47) | −0.56 | −6.98 | −2.71 | −0.59 | +0.01 | −1.17 | −6.55 | −2.07 | −0.47 | −0.03 |
| Seed 23, first mapping, assignment | without replay | +0.12 | −0.17 | +1.53 | +0.04 | −0.03 | −0.30 | −0.38 | +1.53 | −0.27 | 0.00 |
| Seed 23, first mapping, assignment | excitatory | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, assignment | excitatory without replay | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, assignment | plain on A's (H-47) | −0.02 | −6.24 | −2.31 | −0.18 | 0.00 | −0.80 | −5.79 | −1.55 | −0.35 | −0.06 |
| Seed 24, eighth, mirrored | without replay | −0.39 | +0.07 | +1.08 | −0.07 | +0.07 | −0.78 | −0.31 | +0.88 | −0.11 | +0.10 |
| Seed 24, eighth, mirrored | excitatory | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, mirrored | excitatory without replay | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, mirrored | plain on A's (H-47) | −0.90 | −5.39 | −2.43 | −0.25 | −0.05 | −1.39 | −6.95 | −1.82 | −0.97 | +0.06 |
| Seed 24, first mapping, assignment | without replay | −0.37 | +0.14 | +1.23 | +0.19 | +0.14 | −0.46 | −0.03 | +1.42 | −0.07 | +0.10 |
| Seed 24, first mapping, assignment | excitatory | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, assignment | excitatory without replay | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, assignment | plain on A's (H-47) | −0.62 | −5.10 | −1.98 | +0.12 | −0.03 | −1.17 | −6.04 | −1.45 | −0.87 | +0.05 |
| Seed 23, eighth, assignment | without replay | +0.49 | −0.17 | +1.19 | −0.03 | −0.03 | −0.15 | −0.57 | +1.10 | −0.23 | −0.01 |
| Seed 23, eighth, assignment | excitatory | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, assignment | excitatory without replay | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, assignment | plain on A's (H-47) | −2.38 | −6.32 | −2.44 | −0.82 | −0.01 | −2.18 | −5.94 | −2.12 | −0.70 | −0.02 |
| Seed 24, first mapping, mirrored | without replay | −0.39 | +0.25 | +1.09 | +0.19 | +0.14 | −0.58 | −0.05 | +1.38 | −0.01 | +0.10 |
| Seed 24, first mapping, mirrored | excitatory | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, mirrored | excitatory without replay | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, mirrored | plain on A's (H-47) | −1.60 | −5.34 | −2.09 | −0.79 | −0.03 | −2.39 | −5.55 | −2.18 | −0.60 | +0.01 |

- **Under both held forms the held units' synapses show 0.00**, and the class matrix's held entries moved by exactly nothing, in LSB, in
  every one of the 192 held nights; no quiet tick after a night moved any of the held units' synapses (below).
- **Under H-47's plain night the held units' synapses lost most onto the other readout's excitatory units**: −5.10 to −6.98 M a group over
  four nights, against −0.02 to −2.39 M onto their own group, −1.45 to −2.71 M onto the inhibitory units, −0.97 to +0.12 M onto A's and
  B's units and −0.06 to +0.06 M onto the other excitatory units. Under the night without replay they moved by −0.59 to +1.50 M a group,
  +0.88 to +1.53 M onto the inhibitory units.

The rest of each night's change: the inhibition onto each group's excitatory units and onto the other targets, and the excitatory units
outside the hold onto every target:

| Cell | Form | Inhibitory onto the odd places' excitatory units | onto the even places' | onto the inhibitory units | onto A's and B's units | onto the other excitatory | C's own onto all | D's own onto all | A's and B's units onto all | The other excitatory onto all |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | without replay | +1.10 | +0.70 | +0.66 | −1.17 | +0.03 | −0.04 | +0.01 | −0.02 | +0.11 |
| Seed 22, eighth, mirrored | excitatory | +15.87 | +13.70 | +4.79 | +0.14 | +0.04 | −0.08 | −0.06 | −0.04 | −0.15 |
| Seed 22, eighth, mirrored | excitatory without replay | +0.98 | +0.66 | +0.38 | −1.07 | +0.01 | −0.05 | +0.02 | −0.03 | +0.05 |
| Seed 22, eighth, mirrored | plain on A's (H-47) | +14.23 | +12.21 | +4.39 | +0.09 | +0.04 | −0.05 | −0.02 | −0.04 | −0.10 |
| Seed 22, first mapping, assignment | without replay | −1.89 | −2.20 | −0.35 | −0.97 | +0.04 | −0.04 | +0.01 | −0.02 | +0.13 |
| Seed 22, first mapping, assignment | excitatory | +10.28 | +8.98 | +3.89 | +1.08 | +0.03 | −0.07 | −0.08 | −0.06 | −0.10 |
| Seed 22, first mapping, assignment | excitatory without replay | −1.70 | −1.95 | −0.70 | −0.88 | +0.01 | −0.05 | −0.01 | −0.04 | +0.05 |
| Seed 22, first mapping, assignment | plain on A's (H-47) | +9.29 | +7.95 | +3.43 | +0.99 | +0.02 | −0.06 | −0.08 | −0.07 | −0.06 |
| Seed 23, eighth, mirrored | without replay | +1.13 | +0.88 | +0.79 | −1.02 | +0.02 | 0.00 | −0.05 | −0.03 | −0.02 |
| Seed 23, eighth, mirrored | excitatory | +16.32 | +14.41 | +5.18 | +0.21 | +0.05 | −0.08 | −0.02 | −0.07 | −0.23 |
| Seed 23, eighth, mirrored | excitatory without replay | +0.85 | +0.65 | +0.36 | −0.96 | +0.02 | −0.03 | −0.05 | −0.03 | −0.03 |
| Seed 23, eighth, mirrored | plain on A's (H-47) | +14.40 | +12.88 | +4.55 | +0.13 | +0.05 | −0.05 | −0.03 | −0.07 | −0.24 |
| Seed 23, first mapping, assignment | without replay | −1.65 | −1.99 | −0.59 | −0.87 | −0.03 | −0.03 | −0.05 | −0.04 | −0.03 |
| Seed 23, first mapping, assignment | excitatory | +10.88 | +9.08 | +3.68 | +1.25 | 0.00 | −0.09 | −0.07 | −0.09 | −0.13 |
| Seed 23, first mapping, assignment | excitatory without replay | −1.57 | −1.87 | −0.85 | −0.81 | −0.03 | −0.04 | −0.05 | −0.04 | −0.04 |
| Seed 23, first mapping, assignment | plain on A's (H-47) | +9.88 | +8.08 | +3.09 | +1.16 | +0.01 | −0.07 | −0.06 | −0.09 | −0.15 |
| Seed 24, eighth, mirrored | without replay | +0.93 | +0.94 | +0.52 | −1.00 | +0.02 | −0.02 | −0.05 | −0.03 | −0.13 |
| Seed 24, eighth, mirrored | excitatory | +15.90 | +12.12 | +4.11 | +0.15 | +0.02 | +0.04 | −0.02 | −0.08 | −0.24 |
| Seed 24, eighth, mirrored | excitatory without replay | +0.86 | +0.81 | +0.28 | −0.93 | +0.01 | −0.02 | −0.05 | −0.03 | −0.14 |
| Seed 24, eighth, mirrored | plain on A's (H-47) | +14.61 | +11.20 | +3.89 | +0.05 | 0.00 | +0.06 | −0.02 | −0.08 | −0.22 |
| Seed 24, first mapping, assignment | without replay | −1.73 | −1.86 | −0.73 | −0.86 | +0.03 | 0.00 | −0.06 | −0.05 | −0.17 |
| Seed 24, first mapping, assignment | excitatory | +11.39 | +7.46 | +3.01 | +1.13 | +0.04 | +0.03 | −0.02 | −0.09 | −0.22 |
| Seed 24, first mapping, assignment | excitatory without replay | −1.64 | −1.85 | −1.00 | −0.76 | −0.01 | +0.01 | −0.06 | −0.05 | −0.12 |
| Seed 24, first mapping, assignment | plain on A's (H-47) | +10.49 | +6.77 | +2.76 | +1.03 | +0.01 | +0.07 | −0.01 | −0.10 | −0.23 |
| Seed 23, eighth, assignment | without replay | +1.17 | +1.02 | +0.67 | −1.03 | +0.02 | −0.06 | −0.05 | −0.03 | −0.02 |
| Seed 23, eighth, assignment | excitatory | +13.82 | +14.93 | +4.43 | +0.15 | +0.02 | −0.05 | −0.08 | −0.07 | −0.18 |
| Seed 23, eighth, assignment | excitatory without replay | +0.85 | +0.75 | +0.29 | −0.97 | +0.02 | −0.06 | −0.05 | −0.03 | −0.03 |
| Seed 23, eighth, assignment | plain on A's (H-47) | +12.07 | +13.48 | +3.92 | +0.10 | +0.02 | −0.05 | −0.06 | −0.06 | −0.18 |
| Seed 24, first mapping, mirrored | without replay | −1.77 | −1.92 | −0.73 | −0.87 | +0.03 | +0.01 | −0.06 | −0.05 | −0.17 |
| Seed 24, first mapping, mirrored | excitatory | +8.60 | +8.80 | +2.81 | +1.24 | +0.07 | −0.04 | −0.02 | −0.10 | −0.24 |
| Seed 24, first mapping, mirrored | excitatory without replay | −1.64 | −1.83 | −0.96 | −0.81 | −0.01 | +0.01 | −0.06 | −0.06 | −0.12 |
| Seed 24, first mapping, mirrored | plain on A's (H-47) | +7.72 | +7.83 | +2.32 | +1.18 | +0.06 | −0.03 | −0.01 | −0.10 | −0.26 |

- **Under the excitatory night the inhibition onto each group's excitatory units rose by 7.46 to 16.32 M** over four nights, where H-47's
  plain night raised it by 6.77 to 14.61 M: by 0.69 to 1.92 M more, group by group. Onto the inhibitory units it rose by 2.81 to 5.18 M,
  where the plain night's rose by 2.32 to 4.55 M. In every cell it rose by more onto the group A answers than onto the other, by 0.20 to
  3.93 M, where the plain night's rose by 0.10 to 3.71 M more.
- **Under the excitatory night without replay** the inhibition onto each group's excitatory units moved by −1.95 to +0.98 M, where the
  night without replay moved it by −2.20 to +1.17 M: by −0.32 to +0.24 M from it, group by group.
- **The excitatory units outside the hold**: C's own onto all moved by −0.09 to +0.04 M and D's own by −0.08 to −0.02 M under the
  excitatory night, A's and B's units onto all by −0.10 to −0.04 M, and the other excitatory units by −0.24 to −0.10 M.

The held units' synapses night by night, against what the quiet ticks after each night moved:

| Cell | Form | Nights the held units' entries moved by nothing | Nights they moved by what the quiet ticks moved | Nights the quiet ticks moved held synapses |
| :--- | :--- | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | excitatory | 12 of 12 | 12 of 12 | 0 |
| Seed 22, eighth, mirrored | excitatory without replay | 12 of 12 | 12 of 12 | 0 |
| Seed 22, first mapping, assignment | excitatory | 12 of 12 | 12 of 12 | 0 |
| Seed 22, first mapping, assignment | excitatory without replay | 12 of 12 | 12 of 12 | 0 |
| Seed 23, eighth, mirrored | excitatory | 12 of 12 | 12 of 12 | 0 |
| Seed 23, eighth, mirrored | excitatory without replay | 12 of 12 | 12 of 12 | 0 |
| Seed 23, first mapping, assignment | excitatory | 12 of 12 | 12 of 12 | 0 |
| Seed 23, first mapping, assignment | excitatory without replay | 12 of 12 | 12 of 12 | 0 |
| Seed 24, eighth, mirrored | excitatory | 12 of 12 | 12 of 12 | 0 |
| Seed 24, eighth, mirrored | excitatory without replay | 12 of 12 | 12 of 12 | 0 |
| Seed 24, first mapping, assignment | excitatory | 12 of 12 | 12 of 12 | 0 |
| Seed 24, first mapping, assignment | excitatory without replay | 12 of 12 | 12 of 12 | 0 |
| Seed 23, eighth, assignment | excitatory | 12 of 12 | 12 of 12 | 0 |
| Seed 23, eighth, assignment | excitatory without replay | 12 of 12 | 12 of 12 | 0 |
| Seed 24, first mapping, mirrored | excitatory | 12 of 12 | 12 of 12 | 0 |
| Seed 24, first mapping, mirrored | excitatory without replay | 12 of 12 | 12 of 12 | 0 |

**The held units' synapses from the start image to each checkpoint's image**, the mean over the three paths, under the forms that do not
hold them. Under the two held forms they stood within −20 264 and +28 944 LSB of the start image at every checkpoint, path by path. That is
the awake blocks' change: summed over a path's awake blocks to block 36 it is −18 830 to +28 475 LSB under the held forms, where H-47's
plain night's awake blocks moved them by −16 272 to +16 938 LSB and its nights by −17 048 479 to −23 039 434 LSB a path:

| Cell | Block | Without replay: odd | even | Plain on A's: odd | even |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | −0.20 | −0.30 | −4.02 | −4.26 |
| Seed 22, eighth, mirrored | 12 | −0.15 | −0.50 | −6.76 | −7.12 |
| Seed 22, eighth, mirrored | 18 | −0.01 | −0.60 | −8.35 | −9.02 |
| Seed 22, eighth, mirrored | 24 | +0.35 | −0.59 | −9.22 | −10.35 |
| Seed 22, eighth, mirrored | 36 | +0.35 | −0.59 | −9.22 | −10.36 |
| Seed 22, first mapping, assignment | 6 | −0.34 | −0.19 | −3.45 | −3.37 |
| Seed 22, first mapping, assignment | 12 | −0.22 | −0.22 | −6.02 | −6.08 |
| Seed 22, first mapping, assignment | 18 | +0.17 | −0.18 | −7.54 | −7.95 |
| Seed 22, first mapping, assignment | 24 | +0.53 | −0.22 | −8.65 | −9.39 |
| Seed 22, first mapping, assignment | 36 | +0.53 | −0.22 | −8.65 | −9.39 |
| Seed 23, eighth, mirrored | 6 | −0.25 | −0.40 | −4.45 | −4.13 |
| Seed 23, eighth, mirrored | 12 | +0.09 | −0.59 | −7.34 | −6.86 |
| Seed 23, eighth, mirrored | 18 | +0.67 | −0.33 | −9.49 | −8.82 |
| Seed 23, eighth, mirrored | 24 | +1.50 | +0.36 | −10.84 | −10.29 |
| Seed 23, eighth, mirrored | 36 | +1.49 | +0.36 | −10.84 | −10.29 |
| Seed 23, first mapping, assignment | 6 | −0.19 | −0.21 | −3.46 | −3.21 |
| Seed 23, first mapping, assignment | 12 | +0.18 | −0.07 | −6.16 | −5.74 |
| Seed 23, first mapping, assignment | 18 | +0.81 | +0.09 | −7.67 | −7.35 |
| Seed 23, first mapping, assignment | 24 | +1.50 | +0.59 | −8.75 | −8.55 |
| Seed 23, first mapping, assignment | 36 | +1.50 | +0.59 | −8.75 | −8.55 |
| Seed 24, eighth, mirrored | 6 | −0.06 | −0.41 | −3.90 | −4.09 |
| Seed 24, eighth, mirrored | 12 | +0.18 | −0.41 | −6.42 | −7.14 |
| Seed 24, eighth, mirrored | 18 | +0.47 | −0.34 | −8.06 | −9.35 |
| Seed 24, eighth, mirrored | 24 | +0.75 | −0.21 | −9.02 | −11.07 |
| Seed 24, eighth, mirrored | 36 | +0.75 | −0.21 | −9.02 | −11.07 |
| Seed 24, first mapping, assignment | 6 | +0.18 | +0.02 | −2.83 | −3.36 |
| Seed 24, first mapping, assignment | 12 | +0.42 | +0.30 | −5.24 | −5.95 |
| Seed 24, first mapping, assignment | 18 | +0.90 | +0.54 | −6.81 | −7.86 |
| Seed 24, first mapping, assignment | 24 | +1.32 | +0.96 | −7.61 | −9.50 |
| Seed 24, first mapping, assignment | 36 | +1.32 | +0.96 | −7.61 | −9.50 |
| Seed 23, eighth, assignment | 6 | −0.25 | −0.51 | −4.75 | −4.59 |
| Seed 23, eighth, assignment | 12 | +0.10 | −0.75 | −8.22 | −7.50 |
| Seed 23, eighth, assignment | 18 | +0.48 | −0.46 | −10.47 | −9.56 |
| Seed 23, eighth, assignment | 24 | +1.45 | +0.14 | −11.96 | −10.95 |
| Seed 23, eighth, assignment | 36 | +1.44 | +0.14 | −11.96 | −10.95 |
| Seed 24, first mapping, mirrored | 6 | +0.13 | 0.00 | −3.56 | −4.20 |
| Seed 24, first mapping, mirrored | 12 | +0.38 | +0.27 | −6.36 | −7.06 |
| Seed 24, first mapping, mirrored | 18 | +0.89 | +0.50 | −8.26 | −9.19 |
| Seed 24, first mapping, mirrored | 24 | +1.29 | +0.84 | −9.84 | −10.71 |
| Seed 24, first mapping, mirrored | 36 | +1.29 | +0.84 | −9.84 | −10.71 |

Each night's change of the engine's two sums, from its `start` line to its `end` line, the mean over the three paths:

| Cell | Gap | Without replay: inhibitory sum, M | excitatory sum | Excitatory: inhibitory | excitatory | Excitatory without replay: inhibitory | excitatory | Plain on A's: inhibitory | excitatory |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +0.31 | −0.49 | +10.75 | −0.12 | +0.32 | 0.00 | +10.11 | −8.37 |
| Seed 22, eighth, mirrored | 12 | +0.31 | −0.14 | +9.15 | −0.09 | +0.21 | 0.00 | +8.07 | −5.67 |
| Seed 22, eighth, mirrored | 18 | +0.30 | +0.07 | +7.82 | −0.10 | +0.22 | +0.01 | +6.90 | −3.55 |
| Seed 22, eighth, mirrored | 24 | +0.39 | +0.37 | +6.82 | −0.03 | +0.22 | −0.01 | +5.89 | −2.20 |
| Seed 22, first mapping, assignment | 6 | −1.22 | −0.51 | +9.57 | −0.10 | −1.26 | +0.01 | +9.08 | −6.90 |
| Seed 22, first mapping, assignment | 12 | −1.36 | +0.11 | +6.74 | −0.07 | −1.33 | −0.01 | +6.02 | −5.36 |
| Seed 22, first mapping, assignment | 18 | −1.46 | +0.45 | +4.71 | −0.07 | −1.39 | −0.02 | +3.98 | −3.43 |
| Seed 22, first mapping, assignment | 24 | −1.33 | +0.34 | +3.24 | −0.06 | −1.24 | −0.03 | +2.60 | −2.62 |
| Seed 23, eighth, mirrored | 6 | +0.33 | −0.69 | +11.35 | −0.16 | +0.31 | −0.05 | +10.56 | −8.75 |
| Seed 23, eighth, mirrored | 12 | +0.37 | +0.12 | +9.43 | −0.12 | +0.23 | −0.03 | +8.49 | −5.70 |
| Seed 23, eighth, mirrored | 18 | +0.46 | +0.83 | +8.38 | −0.06 | +0.24 | −0.02 | +7.05 | −4.18 |
| Seed 23, eighth, mirrored | 24 | +0.63 | +1.49 | +7.01 | −0.06 | +0.16 | −0.04 | +5.90 | −2.89 |
| Seed 23, first mapping, assignment | 6 | −1.03 | −0.46 | +10.13 | −0.11 | −1.05 | −0.06 | +9.58 | −6.79 |
| Seed 23, first mapping, assignment | 12 | −1.29 | +0.47 | +7.07 | −0.10 | −1.35 | −0.04 | +6.32 | −5.31 |
| Seed 23, first mapping, assignment | 18 | −1.41 | +0.76 | +4.48 | −0.06 | −1.32 | −0.02 | +3.72 | −3.20 |
| Seed 23, first mapping, assignment | 24 | −1.42 | +1.16 | +3.21 | −0.10 | −1.41 | −0.05 | +2.60 | −2.37 |
| Seed 24, eighth, mirrored | 6 | +0.32 | −0.58 | +9.95 | −0.14 | +0.32 | −0.11 | +9.47 | −8.12 |
| Seed 24, eighth, mirrored | 12 | +0.26 | +0.18 | +8.56 | −0.09 | +0.27 | −0.04 | +7.82 | −5.64 |
| Seed 24, eighth, mirrored | 18 | +0.35 | +0.31 | +7.37 | −0.05 | +0.24 | −0.05 | +6.61 | −3.88 |
| Seed 24, eighth, mirrored | 24 | +0.48 | +0.38 | +6.41 | −0.02 | +0.20 | −0.05 | +5.85 | −2.71 |
| Seed 24, first mapping, assignment | 6 | −1.21 | +0.12 | +9.08 | −0.11 | −1.28 | −0.07 | +8.72 | −6.29 |
| Seed 24, first mapping, assignment | 12 | −1.33 | +0.45 | +6.38 | −0.07 | −1.34 | −0.06 | +5.92 | −5.07 |
| Seed 24, first mapping, assignment | 18 | −1.30 | +0.67 | +4.43 | −0.06 | −1.34 | −0.03 | +3.84 | −3.53 |
| Seed 24, first mapping, assignment | 24 | −1.32 | +0.77 | +3.13 | −0.06 | −1.30 | −0.07 | +2.56 | −2.48 |
| Seed 23, eighth, assignment | 6 | +0.36 | −0.82 | +10.37 | −0.16 | +0.32 | −0.07 | +9.72 | −9.49 |
| Seed 23, eighth, assignment | 12 | +0.35 | +0.09 | +8.86 | −0.10 | +0.23 | −0.03 | +7.84 | −6.46 |
| Seed 23, eighth, assignment | 18 | +0.45 | +0.65 | +7.63 | −0.06 | +0.22 | −0.03 | +6.47 | −4.38 |
| Seed 23, eighth, assignment | 24 | +0.68 | +1.52 | +6.50 | −0.05 | +0.18 | −0.04 | +5.54 | −2.94 |
| Seed 24, first mapping, mirrored | 6 | −1.28 | +0.04 | +8.49 | −0.18 | −1.32 | −0.07 | +7.94 | −7.93 |
| Seed 24, first mapping, mirrored | 12 | −1.36 | +0.43 | +5.63 | −0.12 | −1.38 | −0.06 | +4.95 | −5.78 |
| Seed 24, first mapping, mirrored | 18 | −1.30 | +0.68 | +4.10 | −0.05 | −1.31 | −0.03 | +3.55 | −4.10 |
| Seed 24, first mapping, mirrored | 24 | −1.33 | +0.69 | +3.30 | −0.05 | −1.25 | −0.06 | +2.67 | −3.14 |

An excitatory night, the mean of its three paths, raised the inhibitory sum by 3.13 to 11.35 M, where H-47's plain night raised it by 2.56
to 10.56 M, and lowered the excitatory sum by 0.02 to 0.18 M, where the plain night lowered it by 2.20 to 9.49 M. An excitatory night
without replay moved the inhibitory sum by −1.41 to +0.32 M and the excitatory sum by −0.11 to +0.01 M.

**The ties**, the tied trials of each cell's 240 reads of A and 240 of the pair, under each form:

| Cell | Without replay, A's and the pair's | H-47's without replay | Excitatory | Excitatory without replay | H-47's plain on A's |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 1160 and 382 | 1160 and 382 | 1324 and 338 | 1151 and 370 | 1202 and 287 |
| Seed 22, first mapping, assignment | 1141 and 408 | 1141 and 408 | 1213 and 472 | 1213 and 411 | 1183 and 491 |
| Seed 23, eighth, mirrored | 935 and 332 | 935 and 332 | 1090 and 242 | 1068 and 359 | 1042 and 228 |
| Seed 23, first mapping, assignment | 1086 and 483 | 1086 and 483 | 883 and 629 | 1048 and 438 | 742 and 618 |
| Seed 24, eighth, mirrored | 1137 and 322 | 1137 and 322 | 1403 and 170 | 1231 and 342 | 1372 and 137 |
| Seed 24, first mapping, assignment | 968 and 520 | 968 and 520 | 857 and 588 | 1015 and 516 | 835 and 584 |
| Seed 23, eighth, assignment | 1049 and 256 | 1049 and 256 | 907 and 251 | 1016 and 276 | 1039 and 213 |
| Seed 24, first mapping, mirrored | 1066 and 315 | 1066 and 315 | 835 and 421 | 1134 and 346 | 887 and 332 |

The night without replay's ties are H-47's, read by read. Under the excitatory night they lie 22 to 141 from H-47's plain night's on A's
reads and 4 to 89 on the pair's.

## 5. What the hold leaves

At each night with the excitatory hold, over its twelve nights a cell and form (ranges are over the twelve): the synapses held; their traces
cleared at the night's last tick, how many were not zero and what they held above and below zero; the excitatory synapses from other units
the night moved; the nights in which a held unit fired at or after the night's last tick, and the nights in which the quiet ticks moved held
synapses:

| Cell | Form | Nights | Held synapses | Traces cleared, not zero | cleared above zero, LSB | below zero | Other excitatory synapses moved | Nights a held synapse's presynaptic unit fired at or after the night's last tick | Nights held synapses moved in the quiet ticks: how many, their change |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | excitatory | 12 | 21 248 | 13 121 to 13 562 | 712 432 to 848 327 | −1 463 458 to −1 212 163 | 4 946 to 4 985 | 0 | none |
| Seed 22, eighth, mirrored | excitatory without replay | 12 | 21 248 | 14 082 to 14 645 | 453 407 to 518 804 | −626 574 to −526 337 | 4 919 to 4 977 | 2 | none |
| Seed 22, first mapping, assignment | excitatory | 12 | 21 248 | 12 616 to 12 874 | 705 874 to 786 157 | −1 263 870 to −1 054 333 | 4 957 to 4 989 | 0 | none |
| Seed 22, first mapping, assignment | excitatory without replay | 12 | 21 248 | 14 026 to 14 666 | 433 025 to 524 856 | −555 096 to −492 287 | 4 950 to 4 985 | 0 | none |
| Seed 23, eighth, mirrored | excitatory | 12 | 21 248 | 13 158 to 13 679 | 682 634 to 812 646 | −1 551 849 to −1 260 706 | 4 961 to 4 984 | 3 | none |
| Seed 23, eighth, mirrored | excitatory without replay | 12 | 21 248 | 14 190 to 14 914 | 431 077 to 541 845 | −611 666 to −531 600 | 4 934 to 4 979 | 6 | none |
| Seed 23, first mapping, assignment | excitatory | 12 | 21 248 | 12 712 to 13 221 | 673 687 to 780 379 | −1 305 231 to −1 139 766 | 4 966 to 4 989 | 0 | none |
| Seed 23, first mapping, assignment | excitatory without replay | 12 | 21 248 | 14 241 to 14 821 | 433 141 to 481 433 | −560 157 to −507 739 | 4 934 to 4 984 | 0 | none |
| Seed 24, eighth, mirrored | excitatory | 12 | 21 248 | 12 842 to 13 255 | 739 127 to 779 935 | −1 398 790 to −1 261 613 | 4 971 to 4 988 | 0 | none |
| Seed 24, eighth, mirrored | excitatory without replay | 12 | 21 248 | 14 341 to 14 771 | 504 040 to 564 469 | −640 254 to −561 471 | 4 934 to 4 980 | 0 | none |
| Seed 24, first mapping, assignment | excitatory | 12 | 21 248 | 12 545 to 13 125 | 648 963 to 850 249 | −1 375 126 to −1 124 156 | 4 975 to 4 989 | 0 | none |
| Seed 24, first mapping, assignment | excitatory without replay | 12 | 21 248 | 14 430 to 14 639 | 482 973 to 529 504 | −565 847 to −498 604 | 4 946 to 4 985 | 0 | none |
| Seed 23, eighth, assignment | excitatory | 12 | 21 248 | 12 735 to 13 341 | 584 015 to 686 022 | −1 411 947 to −1 200 969 | 4 954 to 4 989 | 3 | none |
| Seed 23, eighth, assignment | excitatory without replay | 12 | 21 248 | 14 202 to 14 709 | 447 371 to 520 385 | −628 330 to −533 197 | 4 951 to 4 982 | 6 | none |
| Seed 24, first mapping, mirrored | excitatory | 12 | 21 248 | 12 405 to 12 955 | 597 070 to 706 425 | −1 236 332 to −981 311 | 4 961 to 4 987 | 0 | none |
| Seed 24, first mapping, mirrored | excitatory without replay | 12 | 21 248 | 14 374 to 14 592 | 469 875 to 526 516 | −568 896 to −501 934 | 4 951 to 4 982 | 0 | none |

- **Before the night's first tick** 12 018 to 13 516 of the 21 248 held synapses carried a trace that was not zero.
- **A held unit fired at or after the night's last tick** in 6 of the 96 excitatory nights, 3 on each of seed 23's eighth images, and in
  14 of the 96 excitatory nights without replay, 6 on each of seed 23's eighth images and 2 on seed 22's eighth image from the mirrored
  assignment. The quiet ticks moved none of the held units' synapses in any of the 192 nights; after them 0 to 34 of those synapses carried
  a trace, 0 to 11 after an excitatory night.

The night's pairings still to be read, as What ran defines them, each read at the held synapse's presynaptic unit's next spike if that
spike comes before the target's next spike:

| Cell | Form | Pending gains: held synapses | their sum, LSB | their largest | Pending losses with a presynaptic spike at the next tick: held synapses | their sum, LSB | their largest |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | excitatory | 10 854 to 11 085 | 558 939 to 696 705 | 328 | 21 248 | 54 712 to 71 059 | 106 to 126 |
| Seed 22, eighth, mirrored | excitatory without replay | 11 204 to 11 552 | 270 250 to 378 095 | 326 to 328 | 21 248 | 70 260 to 93 299 | 113 to 132 |
| Seed 22, first mapping, assignment | excitatory | 10 802 to 11 474 | 520 340 to 612 654 | 328 | 21 248 | 48 335 to 71 411 | 102 to 150 |
| Seed 22, first mapping, assignment | excitatory without replay | 11 166 to 11 731 | 268 861 to 341 279 | 326 to 328 | 21 248 | 59 211 to 80 997 | 112 to 146 |
| Seed 23, eighth, mirrored | excitatory | 10 591 to 11 185 | 622 147 to 731 434 | 328 | 21 248 | 66 856 to 83 493 | 105 to 150 |
| Seed 23, eighth, mirrored | excitatory without replay | 11 119 to 11 508 | 303 889 to 319 577 | 327 to 328 | 21 248 | 59 308 to 84 991 | 114 to 126 |
| Seed 23, first mapping, assignment | excitatory | 10 799 to 11 246 | 547 865 to 716 221 | 328 | 21 248 | 61 569 to 74 060 | 109 to 130 |
| Seed 23, first mapping, assignment | excitatory without replay | 11 221 to 11 395 | 288 621 to 311 691 | 325 to 328 | 21 248 | 61 483 to 86 935 | 113 to 131 |
| Seed 24, eighth, mirrored | excitatory | 10 750 to 11 093 | 608 195 to 702 822 | 328 | 21 248 | 61 452 to 84 224 | 107 to 143 |
| Seed 24, eighth, mirrored | excitatory without replay | 11 275 to 11 475 | 295 565 to 338 127 | 327 to 328 | 21 248 | 65 993 to 88 652 | 108 to 135 |
| Seed 24, first mapping, assignment | excitatory | 10 769 to 11 204 | 513 642 to 657 083 | 328 | 21 248 | 47 553 to 69 914 | 107 to 132 |
| Seed 24, first mapping, assignment | excitatory without replay | 11 169 to 11 594 | 276 479 to 315 437 | 327 to 328 | 21 248 | 65 529 to 74 249 | 121 to 151 |
| Seed 23, eighth, assignment | excitatory | 10 953 to 11 167 | 574 949 to 675 021 | 328 | 21 248 | 51 253 to 78 236 | 113 to 149 |
| Seed 23, eighth, assignment | excitatory without replay | 11 189 to 11 584 | 291 561 to 343 191 | 326 to 328 | 21 248 | 57 227 to 98 565 | 114 to 144 |
| Seed 24, first mapping, mirrored | excitatory | 10 991 to 11 192 | 467 165 to 626 503 | 328 | 21 248 | 45 419 to 80 880 | 108 to 132 |
| Seed 24, first mapping, mirrored | excitatory without replay | 11 147 to 11 575 | 267 285 to 334 744 | 327 to 328 | 21 248 | 64 349 to 77 078 | 121 to 150 |

- **The pending gain**, the target's spike after the presynaptic unit's last, is on 10 591 to 11 474 held synapses under the excitatory
  night, 467 165 to 731 434 LSB together, the largest 328 LSB, which is $A_+$. Under the excitatory night without replay it is on 11 119 to
  11 731, 267 285 to 378 095 LSB together, the largest 325 to 328 LSB.
- **The pending loss** with a presynaptic spike at the first tick after the image is on all 21 248 held synapses: 45 419 to 84 224 LSB
  together under the excitatory night, the largest 102 to 150 LSB, and 57 227 to 98 565 under the excitatory night without replay, the
  largest 108 to 151. A later spike, before the target's next one, reads no more.
- **Against what the hold cleared.** These pairings were never in the trace the hold cleared, which held, synapse by synapse, the decayed
  net of what the trace carried into the night and the pairings read at the night's presynaptic spikes. Set beside it as sizes alone: the
  pending gains are 66.9 % to 102.4 % the size of the traces cleared above zero under the excitatory night, and 54.1 % to 74.0 % under the
  excitatory night without replay; the most the pending losses can be is 3.8 % to 6.7 % the size of the traces cleared below zero under
  the excitatory night, and 10.0 % to 18.3 % without replay.
- These are trace, not weight: at the next presynaptic spike, in the next day's engine, each synapse's trace takes its pairings, and its
  consolidation moves the weight under that day's modulation. That consolidation is not read here.

## 6. The twelve cells at block 36

H-47's twelve cells at block 36, the rows ordered by the readout A answers, the six where it answers readout 0 first. The eight cells here
are this exploration's jobs; exploration 020's four are exploration 025's job (run
[42469251250](https://github.com/Menteion/Menteion/actions/runs/42469251250)). H-47's plain night is H-47's job's in all twelve. "Less
without replay" carries the paired SE of the job that ran both forms, and "less H-47's plain" exploration 018's unpaired SE. No figure is
pooled across cells:

| Cell | A answers | Job | Excitatory less without replay (paired) | Excitatory less H-47's plain (unpaired) | H-47: plain less without replay | Side under the excitatory night |
| :--- | :--- | :--- | ---: | ---: | ---: | :--- |
| Seed 22, eighth, mirrored | readout 0, the odd places | 026 | +16.3 ± 0.84 | −3.4 ± 1.65 | +19.6 ± 1.06 | A, the odd places |
| Seed 22, first mapping, assignment | readout 0, the odd places | 026 | +5.2 ± 1.24 | −2.7 ± 1.05 | +7.9 ± 0.51 | A, the odd places |
| Seed 23, eighth, mirrored | readout 0, the odd places | 026 | +14.7 ± 1.54 | −2.7 ± 1.06 | +17.4 ± 1.08 | A, the odd places |
| Seed 23, first mapping, assignment | readout 0, the odd places | 026 | +12.6 ± 1.56 | −0.8 ± 0.94 | +13.4 ± 1.56 | A, the odd places |
| Seed 24, eighth, mirrored | readout 0, the odd places | 026 | +16.7 ± 1.23 | +5.4 ± 1.53 | +11.3 ± 0.96 | B, the even places (within 2 SE) |
| Seed 24, first mapping, assignment | readout 0, the odd places | 026 | +5.8 ± 1.60 | +2.0 ± 0.79 | +3.8 ± 1.57 | A, the odd places |
| Seed 23, eighth, assignment | readout 1, the even places | 026 | +10.9 ± 2.49 | +6.9 ± 1.57 | +4.1 ± 1.45 | A, the even places |
| Seed 24, first mapping, mirrored | readout 1, the even places | 026 | +7.4 ± 1.09 | +1.4 ± 0.86 | +6.0 ± 1.54 | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | readout 1, the even places | 025 | −1.7 ± 0.42 | +3.2 ± 1.10 | −4.9 ± 1.09 | B, the odd places |
| Seed 24, eighth, assignment | readout 1, the even places | 025 | +8.8 ± 1.28 | +7.0 ± 1.21 | +1.8 ± 1.79 | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | readout 1, the even places | 025 | +4.7 ± 1.52 | −0.3 ± 1.21 | +5.0 ± 1.24 | B, the odd places |
| Seed 22, eighth, assignment | readout 1, the even places | 025 | +4.6 ± 0.66 | +2.4 ± 1.49 | +2.2 ± 0.65 | B, the odd places |

- **Less H-47's plain night** the excitatory night lies beyond 2 SE above in five of the twelve cells and below in three. Where A answers
  readout 0 it lies below in three, seed 22's two cells and seed 23's eighth image from the mirrored assignment; above in two, seed 24's
  eighth image from the mirrored assignment and first mapping from the assignment; and within 2 SE on seed 23's first mapping from the
  assignment. Where A answers readout 1 it lies above in three, seed 23's eighth image from the assignment, seed 23's first mapping from the
  mirrored assignment and seed 24's eighth image from the assignment, and within 2 SE in the other three.
- **Less the night without replay** it lies beyond 2 SE above in eleven of the twelve cells and below in one, seed 23's first mapping from
  the mirrored assignment, −1.7 ± 0.42, where H-47's clause 1 read −4.9 ± 1.09.
- **The side under the excitatory night** at block 36 lies on the odd places in nine of the twelve cells: on A in five where A answers
  readout 0 and on B in four where A answers readout 1. It lies on the even places on seed 24's eighth image from the mirrored assignment,
  on B, and on seed 23's eighth image and seed 24's eighth image from the assignment, on A.

## What it costs

The trial took 772 s and the eight jobs 5 702 to 6 129 s, 95.0 to 102.2 minutes, at most 42.6 % of a job's 240 minutes; ADR-0241 put a job
at about 95 to 100 minutes. Nothing was cut: every cell ran its three forms. Nine jobs ran, the trial's and eight, every one to the end.

## What it suggests

**In these eight cells A's count under the excitatory night less H-47's plain night's lies beyond 2 SE at 17 of the 40 checkpoints, 11
above and 6 below.** The six below are all where A answers readout 0: on seed 23's eighth image from the mirrored assignment from block 12
on, and on seed 22's two cells at block 36. Across H-47's twelve cells at block 36 the excitatory night lies within 2 SE of H-47's plain
night or above it in all six where A answers readout 1, and below it beyond 2 SE in three of the six where A answers readout 0. The held
units' synapses, which H-47's plain night moved by −17.05 to −23.04 M a path over four nights, did not move in the held nights. The side is
H-47's at 35 of the 40 checkpoints.

**These runs do not part the hold's share from the rest.** Under the excitatory night the inhibition onto the readouts' excitatory units
rose by 0.69 to 1.92 M more than under the plain night, the excitatory night without replay moved A's count beyond 2 SE of the night
without replay at 15 of the 40 checkpoints, and the hold leaves pending gains on about half the held synapses. The note does not read what
the next day does with them.

Questions this suggests for a later exploration or a round:
- The held units' synapses held by target: onto the other readout's excitatory units, which carried the largest part of the plain night's
  change in each cell here and in exploration 025, apart from the rest.
- What moves A's trials below the plain night's on seed 23's eighth image from the mirrored assignment from block 12 on, where B's do not
  move beyond 2 SE.
- A read of what the next day's first consolidations do with the pending gains and losses.

## What did not run

- **Exploration 024's hold on these eight cells**, alone or with exploration 025's: the inhibitory synapses onto the readouts' excitatory
  units consolidate as in H-47 under every form here.
- **Plain nights on both mappings and the awake gap**, under the hold or not: no reading here stands against them.
- **A hold through the quiet ticks**: the hold ends at the night's last tick, and the image is written after the quiet ticks.
  Here the quiet ticks moved no held excitatory synapse in any of the 192 held nights.
- **The held units' synapses held by target**, those onto the other readout's excitatory units apart from the rest. Section 4 breaks the
  held units' synapses down by target; nothing here holds one target and not another.
- **The excitatory synapses onto the readouts from A's, B's and the pair's own units**, which consolidate as in H-47 under every form, the
  zero sum's rounds with them, and **the inhibitory synapses onto the readouts' inhibitory units**, which the hold does not reach.
- **The night under both holds**, left out by ADR-0241, and **both holds without replay**.
- **The pending pairings' effect on a weight**: the next day's consolidation of the held synapses is not read.
- **REM's depotentiations and the zero sum's rounds** are not split out of a night's change, as in explorations 020 to 025.
- **Any SE for sections 4 and 5**: the class matrix is a mean over the three paths, and the hold's readings are ranges.
- **Any figure pooled across cells**, in section 6 or elsewhere.
- **Any other share, form, network or seed.**

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_the_excitatory_hold_on_eight_cells_explore` | [42488465689](https://github.com/Menteion/Menteion/actions/runs/42488465689) | `c98207f` | 772 |
| `the_excitatory_hold_on_eight_cells_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [42489793510](https://github.com/Menteion/Menteion/actions/runs/42489793510) | `c98207f` | 5843 |
| `the_excitatory_hold_on_eight_cells_a_first_mapping_on_seed_22_from_the_assignment_explore` | [42489793510](https://github.com/Menteion/Menteion/actions/runs/42489793510) | `c98207f` | 5878 |
| `the_excitatory_hold_on_eight_cells_the_eighth_image_on_seed_23_from_the_mirrored_assignment_explore` | [42489793510](https://github.com/Menteion/Menteion/actions/runs/42489793510) | `c98207f` | 5829 |
| `the_excitatory_hold_on_eight_cells_a_first_mapping_on_seed_23_from_the_assignment_explore` | [42489793510](https://github.com/Menteion/Menteion/actions/runs/42489793510) | `c98207f` | 5952 |
| `the_excitatory_hold_on_eight_cells_the_eighth_image_on_seed_24_from_the_mirrored_assignment_explore` | [42489793510](https://github.com/Menteion/Menteion/actions/runs/42489793510) | `c98207f` | 5702 |
| `the_excitatory_hold_on_eight_cells_a_first_mapping_on_seed_24_from_the_assignment_explore` | [42489793510](https://github.com/Menteion/Menteion/actions/runs/42489793510) | `c98207f` | 6129 |
| `the_excitatory_hold_on_eight_cells_the_eighth_image_on_seed_23_from_the_assignment_explore` | [42489793510](https://github.com/Menteion/Menteion/actions/runs/42489793510) | `c98207f` | 5890 |
| `the_excitatory_hold_on_eight_cells_a_first_mapping_on_seed_24_from_the_mirrored_assignment_explore` | [42489793510](https://github.com/Menteion/Menteion/actions/runs/42489793510) | `c98207f` | 6025 |
