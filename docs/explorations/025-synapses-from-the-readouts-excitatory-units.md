---
status: exploratory
date: 2026-11-08
---

# 025: The synapses from the readouts' excitatory units held through a night

**Branch**: `explore/025-synapses-from-the-readouts-excitatory-units`
**Commit**: `2862685`, which adds the harness, on `72bd3f5`, which adds the excitatory hold under `src/`, on `739d7e1`: `main` at
`f7a6808`, ADR-0240's merge, with exploration 024's tag merged in.
**Runs**: [42468595193](https://github.com/Menteion/Menteion/actions/runs/42468595193) (the harness's trial) and
[42469251250](https://github.com/Menteion/Menteion/actions/runs/42469251250) (four jobs, one a cell), both at `scope=arms`. H-47's plain
night on A's patterns is read from H-47's run [41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613), its class
matrix and night lines from exploration 020's runs [41500459562](https://github.com/Menteion/Menteion/actions/runs/41500459562) and
[41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777), and exploration 024's held night from 024's run
[42406369084](https://github.com/Menteion/Menteion/actions/runs/42406369084). None of those was run again.

[ADR-0240](../adr/0240-synapses-from-the-readouts-excitatory-units-held.md) ordered this exploration. Under
[exploration 024](024-the-readouts-inhibition-held.md)'s held night, the inhibition onto the readouts' excitatory units held, A's count
stood within 2 SE of H-47's plain night's at every checkpoint of its four cells, while the same nights took 9.35 to 13.21 M from each
group's excitatory units' synapses. This note runs H-47's plain night on A's patterns with every synapse from the readouts' excitatory units held
where it stood, the pair's own units excepted, from the night's first tick to its last, alone and with exploration 024's hold, on
[exploration 020](020-what-a-replay-costs.md)'s four cells. It asks, at blocks 6, 12, 18, 24 and 36:

1. A's count and each stimulus's correct trials under each held night: less the night without replay, with H-47's paired SE, the forms run
   in one job; and less H-47's plain night and less exploration 024's held night, each with
   [exploration 018](018-what-nights-keep.md)'s unpaired SE, forms run in other jobs;
2. the side under each held night, by stimulus and by group of units, against H-47's side and 024's on the same cell;
3. the pair under each held night and A's highest coupling, as H-47's clauses 2 and 3 read them;
4. what each night moved, from the class matrix by group, with the held units' synapses split by target and their change shown nil;
5. what the hold leaves: what the quiet ticks after each night move, and the pairings still pending under the excitatory rule, the gain and
   the loss apart.

**The side** is [ADR-0238](../adr/0238-what-keeps-a-mapping-where-the-replay-did-not.md)'s: at a checkpoint, the stimulus whose difference
from the night without replay is the larger of the two. The note writes no prediction and decides nothing. **Every figure here is
exploratory**: from one run of each test on the repository's AArch64 host, in H-47's terms. A count is a mean over a cell's 48 reads,
sixteen on each of three paths, and a difference carries its paired SE unless another is named.

**In brief**
- **The hold held, and the night without replay ran H-47's.** In all 144 nights with the excitatory hold every held synapse ended the night
  at the weight it started with and its trace cleared, held inside the test, while 4 934 to 4 989 excitatory synapses from other units
  moved; under both holds exploration 024's held too, in all 48 nights. The quiet ticks after a night moved no synapse the excitatory hold
  held; under both holds they moved exploration 024's held synapses in 5 of the 48 nights (section 5). Every row of the night without
  replay, 15 a cell, is H-47's, held inside the test.
- **Under the excitatory night A's count less H-47's plain night's** lies between −0.3 and +7.1, beyond twice its unpaired SE at 8 of the
  20 checkpoints, all above: on seed 24's eighth image from block 12 on, +2.6 ± 1.04 to +7.1 ± 1.50; on seed 23's at blocks 18 and 36,
  +1.8 ± 0.89 and +3.2 ± 1.10; on seed 22's eighth image at blocks 12 and 24, +2.2 ± 0.91 and +2.0 ± 0.92. On seed 22's first mapping it
  lies within 2 SE at every checkpoint. Less 024's held night it lies between −0.4 and +8.1, beyond 2 SE at 9 of 20, all above.
- **The excitatory night less the night without replay at block 36** is −1.7 ± 0.42, +8.8 ± 1.28, +4.7 ± 1.52 and +4.6 ± 0.66 in seed
  23's, seed 24's and seed 22's two cells, where H-47's plain night gave −4.9 ± 1.09, +1.8 ± 1.79, +5.0 ± 1.24 and +2.2 ± 0.65. Under both
  holds it is −1.7 ± 0.50, +8.3 ± 1.53, +5.3 ± 1.72 and +3.8 ± 1.24. The excitatory night without replay less the night without replay is
  −0.8 ± 1.35, −4.9 ± 1.12, +4.4 ± 0.94 and +3.2 ± 0.37 there.
- **The side under the excitatory night is H-47's at 19 of the 20 checkpoints**, from block 18 to 24 on B, the odd places, in all four
  cells. It differs at seed 24's block 36, where it lies on A, the even places, within 2 SE, and H-47's on B, within 2 SE. Under both holds
  it is H-47's at 17 of 20.
- **Each held night with replay ran H-47's plain night's replays and depotentiations, night by night.**
- **The held units' synapses moved by exactly nothing in every held night**, where H-47's plain night moved them by −19.72 to −23.68 M a
  path over four nights, −4.73 to −6.56 M of each group's onto the other readout's excitatory units. Under the held forms the awake blocks
  moved them by −22 713 to +12 110 LSB a path to block 36.
- **Under the excitatory night the inhibition onto each group's excitatory units rose by 7.47 to 13.27 M** over four nights, 0.73 to
  1.52 M more than under H-47's plain night. The engine's excitatory sum fell by 0.03 to 0.17 M a night, where the plain night took 2.89 to
  9.36 M.
- **The pairings the hold leaves**: after the quiet ticks of each excitatory night, 10 879 to 11 619 of the 21 248 held synapses carry a
  pending gain, 482 753 to 663 569 LSB together, read at the presynaptic unit's next spike if it comes before the target's next spike. A
  presynaptic spike at the first tick after the image would read a loss of 47 851 to 94 482 LSB on them together, and a later one, before
  the target's next spike, no more.
- **The pair under the excitatory night** stands at 62.6 to 63.0 at block 36, and A's highest coupling at 1.151 to 1.196 of H-42's.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 020 to 024 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. A cell is named, as in H-47, by its seed, its image and the arm
it starts from. Nights follow blocks 6, 12, 18 and 24, then 12 blocks run with none to block 36, on paths 0, 5 and 6. Each checkpoint reads
A and the pair sixteen times each, after the night that follows its block, and at block 36 after the block.

**Places.** A period of the ring is 20 units. Stimulus A presents place 0 of each period and B place 11. **The odd places** are the odd
places but 11 and **the even places** the even places but 0: H-47's readout 0 and readout 1, each with its excitatory and its inhibitory
units. C's own units lie at place 5, among the odd places, and D's at place 16, among the even.

**Four cells**, explorations 020's and 024's: H-47's two that failed clause 1, seed 23's first mapping from the mirrored assignment and seed
24's eighth image from the assignment, and seed 22's two from the same starts. In all four the mapping in force is the mirrored one, so
stimulus A answers readout 1, the even places, and B readout 0, the odd places.

**The excitatory hold**, on the branch alone (`72bd3f5`). `Executor::set_night_hold_sources` takes a set of excitatory units. While it is
set and the engine is in slow-wave sleep or REM, the fan-out skips the consolidation of every block of a held unit's chain, onto whatever
target: no slot of it consolidates, at any modulation, so every weight stays where it stood while its trace takes the pairings. **The
night's last tick** is the tick whose window steps the engine from sleep to awake; the hold is in force through it, and at its end every
slot of a held unit's chain has its trace cleared to zero. Every other synapse consolidates as in H-47. Exploration 024's hold
(`set_night_hold`, inhibitory synapses held by their target) is as its branch built it, set apart from this one. Unset, the fan-out takes
its old path. Neither hold is in an image, so the next day's engine, decoded from the night's image, holds nothing.

**The held units** (`e025_held`) are exploration 024's 714, every excitatory unit of the two readouts, less C's own 25 and D's own 25:
664, 332 a readout. None of them is one of the zero sum's 152 sources, nor a unit of the six kinds, both held inside the test, so they are
the class matrix's classes 6 to 9: each readout's units that A's patterns reach and its other excitatory units. In each cell the hold holds
the held units' 21 248 synapses, of the cell's 26 240 excitatory synapses. The synapses from A's, B's and the pair's own units consolidate
as in H-47, the zero sum's rounds with them, and so does every inhibitory synapse unless exploration 024's hold is set as well.

**Four forms a cell, in one job** (`e025_job`):
- **the night without replay**, every row held inside the test to the row H-47 pinned;
- **the excitatory night**, H-47's plain night on A's patterns with the excitatory hold set;
- **the night under both holds**, the excitatory night with exploration 024's hold set as well, on its 714 units;
- **the excitatory night without replay**, the night without replay with the excitatory hold set, to read the hold apart from the replay.

The job is exploration 020's and 024's: A's patterns, the split and the class matrix of the start image, of the image before each night and
of the image after it, and the nights after blocks 6, 12, 18 and 24 watched by `ReplayWatch`. H-47's plain night on A's patterns is read from
H-47's logs, its class matrix and night lines from exploration 020's, and exploration 024's held night from 024's job.

**At each night with the excitatory hold** (`e025_night`) the job reads every excitatory synapse three times: before the night's first
tick, at its last, and after the quiet ticks, the awake ticks without drive that H-47's harness runs until the engine is quiescent, before
it writes the night's image. At the last it holds, inside the test, that every held synapse stands at its first weight with its trace
cleared and that some excitatory synapse from a unit outside the hold moved. After the quiet ticks it prints what they moved, and what the
held synapses would read at their presynaptic unit's next spike from the spikes on record since the night's first tick, if that spike came
before the target's next one: for the gain and for the loss, how many held synapses carry one, their sum and the largest. Under both
holds it reads exploration 024's held synapses as 024's job did, in the same lines.

**The excitatory rule** (`step_stdp_shaped`, in `crates/menteion-core/src/dynamics/synapse.rs`) reads two pairings at a presynaptic spike
at $t$, with $p$ the presynaptic unit's previous spike and $q$ the target's last spike: if $p < q \le t$, a gain of
$A_+ (1 - 2^{-11})^{q - p}$; and if $q < t$, a loss of $A_- (1 - 2^{-11})^{t - q}$ scaled by the slot's magnitude, by the exponent the
configuration sets ([ADR-0228](../adr/0228-a-zero-sum-at-the-stimulus-built.md)). Both are read only there, so the night's last pairings
escape the clearing. The job finds each with the rule itself, on a copy of the block, the slot's trace set to zero; with $t_0$ the first
tick after the image:
- **the pending gain**: a held synapse whose target's last spike and presynaptic unit's last spike are both on record since the night's
  first tick, $q > p$: $A_+ (1 - 2^{-11})^{q - p}$, read at the presynaptic unit's next spike if it comes before the target's next spike;
- **the pending loss**: a held synapse whose target fired since the night's first tick: the loss a presynaptic spike at $t_0$ would read,
  at the synapse's magnitude after the quiet ticks. It falls as $t$ grows, so it is the most the next presynaptic spike reads, if that spike
  comes before the target's next one.

A target spike before the next presynaptic one takes $q$'s place, and both readings change with it.

**The class matrix** is exploration 020's, as 021 reads it by readout and 023 and 024 by group. Each group's excitatory units are its
readout's two classes and the pair's own kind inside it, C's own in the odd places and D's in the even. An inhibitory entry is a
magnitude, the weight negated, so a rise is more inhibition. The held synapses are the rows of classes 6 and 7, the odd places' held
units, and 8 and 9, the even places', onto every class.

## Before a job ran

ADR-0240 asks whether any form but the night without replay must run bit for bit as one already in the logs. **None must.** Each held form
holds synapses that consolidate in the form it is set beside: over four nights the held units' synapses moved by −9.10 to −12.28 M a group
under H-47's plain night and by −0.52 to +1.49 M under the night without replay (section 4), so neither the excitatory night nor the night
under both holds nor the excitatory night without replay starts from a night that leaves them alone. The trial then printed each held
form's row at block 6 on path 0 beside H-47's of the form it is set beside and beside 024's trial's held row: none is the same, as the trial
prints. So all four forms ran.

## The runs are laid out as written

- **The trial** ran seed 23's first mapping from the mirrored assignment on path 0: the pair's head and the first night of six forms, all
  watched. With no hold set, the night without replay and the plain night on A's patterns held H-47's row at block 6 on path 0, inside the
  test. With exploration 024's hold alone, the held night held the row 024's trial printed at block 6 on path 0, inside the test: its
  image's CRC-64, A's first count of 57, its couplings and the gain. With the excitatory hold set, each night held every held synapse at its
  first weight with its trace cleared, inside the test: 21 248 of the cell's 26 240 excitatory synapses, while 4 986, 4 988 and 4 980
  excitatory synapses from other units moved in the excitatory night, the night under both holds and the excitatory night without replay.
  Under both holds exploration 024's hold held its 4 881 synapses the same way, and 1 587 inhibitory synapses onto other units moved.
- The three held forms' rows at block 6 on path 0 are not H-47's of the form beside them, nor 024's trial's held row: each image's CRC-64,
  A's couplings and the gain differ, and A's first count is 58 in each, where H-47's and 024's rows read 57, as the trial prints. The watch
  read `wrong 0` in its six nights, and the plain night, 024's held night, the excitatory night and the night under both holds each 1 339
  replays and 640 depotentiations.
- **Each of the four jobs** ran to the end. Their reproduction, cell by cell:

| Cell | Without replay: rows H-47's | Excitatory: rows H-47's plain on A's | Both holds: rows H-47's plain on A's | Both holds: rows 024's held | Excitatory without replay: rows H-47's without replay | Excitatory without replay: rows 024's held without replay | Without replay: its paths' lines in 020's job, label aside | Heads: lines in 020's job | Held nights: the excitatory hold held inside the test | Both holds: 024's hold held inside the test |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 15 of 15 | 0 of 15 | 0 of 15 | 0 of 15 | 0 of 15 | 0 of 15 | 1 680 of 1 680 | 3 of 3 | 36 of 36 | 12 of 12 |
| Seed 24, eighth, assignment | 15 of 15 | 0 of 15 | 0 of 15 | 0 of 15 | 0 of 15 | 0 of 15 | 1 656 of 1 656 | 3 of 3 | 36 of 36 | 12 of 12 |
| Seed 22, first mapping, mirrored | 15 of 15 | 0 of 15 | 0 of 15 | 0 of 15 | 0 of 15 | 0 of 15 | 1 632 of 1 632 | 3 of 3 | 36 of 36 | 12 of 12 |
| Seed 22, eighth, assignment | 15 of 15 | 0 of 15 | 0 of 15 | 0 of 15 | 0 of 15 | 0 of 15 | 1 632 of 1 632 | 3 of 3 | 36 of 36 | 12 of 12 |

- The night without replay's `DUMP` lines, its paths' 1 632 to 1 680 a cell, its 15 rows' and its notice, are exploration 020's job's
  lines of the same cell, label aside, all of them in each cell; so are the pair's head's three. Each read of the night without replay carries
  H-47's read's `stimuli`, `right` and `tied` masks, all 480 a cell. The excitatory night's and the night under both holds' reads carry H-47's
  plain night's `stimuli` masks, and the excitatory night without replay's H-47's night without replay's, so each trial presented the
  stimulus it presented in H-47.
- For each held form the job prints how many of its 15 rows are H-47's of the matching form: **none, in all four cells**, as the job prints;
  nothing in the job asserts it. Read from the logs, none of the night under both holds' rows is 024's held night's, and none of the
  excitatory night without replay's is 024's held night without replay's.
- **The watch read `wrong 0` in all 192 nights.** Every night without replay, held or not, read `replays 0` and `depotentiations 0`. The
  excitatory night and the night under both holds each read H-47's plain night's replays and depotentiations, as exploration 020's job
  printed them, in all 12 nights of each cell, and 024's held night's.
- Every held unit's synapse was read at each night: 664 held units, 332 a readout, out of exploration 024's 714, C's own 25 and D's own 25
  left out, as each job prints; that none of the zero sum's 152 sources is among them each job holds.

## 1. A's count under each held night

A's count at each checkpoint under each form. H-47's plain night on A's patterns is H-47's job's, and 024's held night 024's job's:

| Cell | Block | Without replay | Excitatory | Both holds | Excitatory without replay | Plain on A's (H-47) | 024's held |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | 55.0 | 56.6 | 56.8 | 55.4 | 56.9 | 56.8 |
| Seed 23, first mapping, mirrored | 12 | 52.4 | 55.5 | 56.1 | 51.9 | 54.4 | 54.5 |
| Seed 23, first mapping, mirrored | 18 | 52.0 | 54.2 | 54.0 | 51.6 | 52.4 | 51.9 |
| Seed 23, first mapping, mirrored | 24 | 53.5 | 53.3 | 53.6 | 53.8 | 50.9 | 50.4 |
| Seed 23, first mapping, mirrored | 36 | 53.1 | 51.4 | 51.4 | 52.4 | 48.3 | 48.3 |
| Seed 24, eighth, assignment | 6 | 38.8 | 39.3 | 39.0 | 38.6 | 39.1 | 38.5 |
| Seed 24, eighth, assignment | 12 | 34.8 | 36.8 | 36.5 | 36.2 | 34.2 | 33.0 |
| Seed 24, eighth, assignment | 18 | 34.4 | 35.6 | 33.1 | 33.0 | 28.5 | 29.0 |
| Seed 24, eighth, assignment | 24 | 35.6 | 36.7 | 35.7 | 28.8 | 29.6 | 28.6 |
| Seed 24, eighth, assignment | 36 | 25.6 | 34.4 | 33.9 | 20.7 | 27.4 | 26.7 |
| Seed 22, first mapping, mirrored | 6 | 56.6 | 58.1 | 58.3 | 57.3 | 57.7 | 57.8 |
| Seed 22, first mapping, mirrored | 12 | 49.5 | 52.7 | 52.5 | 49.5 | 52.5 | 52.4 |
| Seed 22, first mapping, mirrored | 18 | 49.0 | 52.3 | 52.4 | 50.6 | 52.7 | 52.8 |
| Seed 22, first mapping, mirrored | 24 | 45.9 | 50.7 | 51.0 | 48.3 | 50.6 | 51.1 |
| Seed 22, first mapping, mirrored | 36 | 44.1 | 48.8 | 49.4 | 48.5 | 49.1 | 49.2 |
| Seed 22, eighth, assignment | 6 | 50.3 | 52.6 | 52.7 | 50.3 | 52.3 | 52.0 |
| Seed 22, eighth, assignment | 12 | 47.6 | 52.8 | 53.0 | 48.6 | 50.6 | 51.2 |
| Seed 22, eighth, assignment | 18 | 47.2 | 51.8 | 50.6 | 48.6 | 50.6 | 50.4 |
| Seed 22, eighth, assignment | 24 | 47.3 | 52.1 | 50.6 | 48.1 | 50.2 | 49.3 |
| Seed 22, eighth, assignment | 36 | 44.2 | 48.8 | 48.0 | 47.4 | 46.4 | 45.7 |

The differences. "Less without replay" carries the paired SE, the forms run in one job; "less plain" and "less 024's held" carry
exploration 018's unpaired SE. The night without replay is H-47's, read by read, so "H-47: plain less without replay" at block 36 is H-47's
clause 1, and "024: held less without replay" is 024's own reading:

| Cell | Block | Excitatory less without replay | Both holds less without replay | H-47: plain less without replay | 024: held less without replay | Excitatory less plain (unpaired) | Both holds less plain (unpaired) | Excitatory less 024's held (unpaired) | Both holds less 024's held (unpaired) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | +1.5 ± 0.76 | +1.7 ± 0.74 | +1.9 ± 0.68 | +1.8 ± 0.71 | −0.3 ± 0.65 | −0.1 ± 0.68 | −0.3 ± 0.70 | −0.1 ± 0.72 |
| Seed 23, first mapping, mirrored | 12 | +3.1 ± 1.21 | +3.7 ± 1.10 | +2.0 ± 1.26 | +2.2 ± 1.15 | +1.1 ± 1.10 | +1.7 ± 1.04 | +1.0 ± 1.11 | +1.6 ± 1.01 |
| Seed 23, first mapping, mirrored | 18 | +2.2 ± 1.23 | +2.0 ± 1.27 | +0.3 ± 1.17 | −0.1 ± 1.08 | +1.8 ± 0.89 | +1.6 ± 0.92 | +2.3 ± 0.96 | +2.1 ± 0.98 |
| Seed 23, first mapping, mirrored | 24 | −0.1 ± 0.80 | +0.1 ± 1.07 | −2.6 ± 1.30 | −3.1 ± 1.57 | +2.4 ± 1.24 | +2.6 ± 1.11 | +2.9 ± 1.45 | +3.1 ± 1.47 |
| Seed 23, first mapping, mirrored | 36 | −1.7 ± 0.42 | −1.7 ± 0.50 | −4.9 ± 1.09 | −4.8 ± 1.45 | +3.2 ± 1.10 | +3.2 ± 1.30 | +3.1 ± 1.43 | +3.1 ± 1.65 |
| Seed 24, eighth, assignment | 6 | +0.5 ± 0.67 | +0.3 ± 0.82 | +0.3 ± 0.64 | −0.3 ± 0.81 | +0.2 ± 1.04 | 0.0 ± 1.14 | +0.7 ± 1.01 | +0.5 ± 1.09 |
| Seed 24, eighth, assignment | 12 | +2.0 ± 0.89 | +1.7 ± 1.20 | −0.6 ± 1.03 | −1.8 ± 1.12 | +2.6 ± 1.04 | +2.3 ± 1.17 | +3.8 ± 0.96 | +3.5 ± 1.07 |
| Seed 24, eighth, assignment | 18 | +1.2 ± 2.06 | −1.3 ± 1.97 | −5.9 ± 1.53 | −5.4 ± 2.42 | +7.1 ± 1.50 | +4.7 ± 1.23 | +6.6 ± 1.11 | +4.2 ± 1.42 |
| Seed 24, eighth, assignment | 24 | +1.1 ± 1.88 | +0.2 ± 1.78 | −5.9 ± 1.85 | −7.0 ± 1.62 | +7.0 ± 1.84 | +6.1 ± 1.53 | +8.1 ± 1.42 | +7.2 ± 1.61 |
| Seed 24, eighth, assignment | 36 | +8.8 ± 1.28 | +8.3 ± 1.53 | +1.8 ± 1.79 | +1.1 ± 1.20 | +7.0 ± 1.21 | +6.5 ± 1.31 | +7.7 ± 1.47 | +7.2 ± 1.45 |
| Seed 22, first mapping, mirrored | 6 | +1.5 ± 0.59 | +1.6 ± 0.54 | +1.1 ± 0.63 | +1.2 ± 0.69 | +0.4 ± 0.60 | +0.5 ± 0.57 | +0.3 ± 0.66 | +0.4 ± 0.64 |
| Seed 22, first mapping, mirrored | 12 | +3.2 ± 1.36 | +2.9 ± 1.61 | +2.9 ± 1.93 | +2.9 ± 1.69 | +0.3 ± 1.13 | 0.0 ± 1.02 | +0.3 ± 0.98 | +0.1 ± 0.93 |
| Seed 22, first mapping, mirrored | 18 | +3.4 ± 2.15 | +3.4 ± 2.51 | +3.7 ± 2.22 | +3.8 ± 2.27 | −0.3 ± 1.12 | −0.3 ± 1.18 | −0.4 ± 1.16 | −0.4 ± 1.20 |
| Seed 22, first mapping, mirrored | 24 | +4.8 ± 1.82 | +5.1 ± 1.96 | +4.7 ± 2.24 | +5.2 ± 2.42 | +0.1 ± 1.43 | +0.4 ± 1.30 | −0.4 ± 1.51 | −0.1 ± 1.37 |
| Seed 22, first mapping, mirrored | 36 | +4.7 ± 1.52 | +5.3 ± 1.72 | +5.0 ± 1.24 | +5.0 ± 1.56 | −0.3 ± 1.21 | +0.3 ± 1.04 | −0.4 ± 1.27 | +0.2 ± 1.00 |
| Seed 22, eighth, assignment | 6 | +2.3 ± 0.81 | +2.4 ± 0.66 | +1.9 ± 0.65 | +1.7 ± 0.57 | +0.3 ± 1.10 | +0.5 ± 0.98 | +0.6 ± 1.15 | +0.7 ± 0.99 |
| Seed 22, eighth, assignment | 12 | +5.2 ± 0.79 | +5.4 ± 1.14 | +3.0 ± 0.93 | +3.6 ± 0.93 | +2.2 ± 0.91 | +2.4 ± 0.94 | +1.6 ± 0.82 | +1.8 ± 0.96 |
| Seed 22, eighth, assignment | 18 | +4.6 ± 1.28 | +3.4 ± 1.54 | +3.4 ± 1.05 | +3.2 ± 0.96 | +1.3 ± 1.46 | 0.0 ± 1.80 | +1.4 ± 0.80 | +0.2 ± 0.96 |
| Seed 22, eighth, assignment | 24 | +4.8 ± 1.01 | +3.3 ± 0.95 | +2.8 ± 0.89 | +1.9 ± 1.06 | +2.0 ± 0.92 | +0.5 ± 0.87 | +2.9 ± 0.92 | +1.4 ± 0.84 |
| Seed 22, eighth, assignment | 36 | +4.6 ± 0.66 | +3.8 ± 1.24 | +2.2 ± 0.65 | +1.5 ± 0.92 | +2.4 ± 1.49 | +1.6 ± 1.79 | +3.1 ± 1.49 | +2.3 ± 1.80 |

- **The excitatory night less H-47's plain night** lies between −0.3 and +7.1, beyond twice its unpaired SE at 8 of the 20 checkpoints, all
  above, and at most 5.77 times it. **The night under both holds less the plain night** lies between −0.3 and +6.5, beyond 2 SE at 6 of 20,
  all above: seed 23's blocks 24 and 36, seed 24's blocks 18, 24 and 36, and seed 22's eighth image at block 12.
- **Less 024's held night** the excitatory night lies between −0.4 and +8.1, beyond 2 SE at 9 of 20, and the night under both holds between
  −0.4 and +7.2, beyond 2 SE at 6 of 20, all above.
- **The excitatory night less the night without replay** lies beyond twice its SE at 14 of the 20 checkpoints, 13 above and one below,
  seed 23's block 36, −1.7 ± 0.42. The night under both holds lies beyond at 12, 11 above and the same one below, −1.7 ± 0.50. H-47's plain
  night lies beyond at 11 and 024's held night at 9.

The forms without replay, and the forms against each other, each pair run in one job:

| Cell | Block | Excitatory without replay less without replay | Excitatory without replay less 024's held without replay (unpaired) | Excitatory less excitatory without replay | Both holds less excitatory |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | +0.4 ± 0.38 | +0.3 ± 1.05 | +1.2 ± 0.90 | +0.2 ± 0.13 |
| Seed 23, first mapping, mirrored | 12 | −0.4 ± 0.94 | −0.6 ± 1.24 | +3.6 ± 0.57 | +0.6 ± 0.37 |
| Seed 23, first mapping, mirrored | 18 | −0.4 ± 1.16 | −1.2 ± 1.48 | +2.6 ± 1.29 | −0.2 ± 0.19 |
| Seed 23, first mapping, mirrored | 24 | +0.3 ± 0.75 | 0.0 ± 1.04 | −0.5 ± 0.86 | +0.2 ± 0.48 |
| Seed 23, first mapping, mirrored | 36 | −0.8 ± 1.35 | −1.1 ± 1.12 | −0.9 ± 1.06 | 0.0 ± 0.24 |
| Seed 24, eighth, assignment | 6 | −0.2 ± 0.64 | −0.1 ± 1.40 | +0.7 ± 0.60 | −0.2 ± 0.35 |
| Seed 24, eighth, assignment | 12 | +1.4 ± 0.49 | +0.4 ± 0.90 | +0.6 ± 1.21 | −0.3 ± 0.19 |
| Seed 24, eighth, assignment | 18 | −1.4 ± 1.31 | −1.2 ± 1.84 | +2.5 ± 1.81 | −2.4 ± 1.14 |
| Seed 24, eighth, assignment | 24 | −6.7 ± 1.52 | −7.7 ± 1.75 | +7.8 ± 1.16 | −0.9 ± 0.58 |
| Seed 24, eighth, assignment | 36 | −4.9 ± 1.12 | −5.0 ± 0.99 | +13.7 ± 1.00 | −0.5 ± 0.70 |
| Seed 22, first mapping, mirrored | 6 | +0.7 ± 0.33 | +0.7 ± 0.89 | +0.8 ± 0.67 | +0.1 ± 0.16 |
| Seed 22, first mapping, mirrored | 12 | 0.0 ± 0.80 | 0.0 ± 1.33 | +3.3 ± 1.31 | −0.3 ± 0.29 |
| Seed 22, first mapping, mirrored | 18 | +1.7 ± 0.80 | +2.0 ± 0.88 | +1.7 ± 1.73 | +0.1 ± 0.43 |
| Seed 22, first mapping, mirrored | 24 | +2.4 ± 1.03 | +2.6 ± 1.22 | +2.4 ± 1.23 | +0.3 ± 0.23 |
| Seed 22, first mapping, mirrored | 36 | +4.4 ± 0.94 | +4.2 ± 1.27 | +0.3 ± 1.01 | +0.6 ± 0.57 |
| Seed 22, eighth, assignment | 6 | 0.0 ± 0.58 | 0.0 ± 1.05 | +2.3 ± 0.79 | +0.1 ± 0.26 |
| Seed 22, eighth, assignment | 12 | +1.0 ± 0.58 | +0.7 ± 1.20 | +4.2 ± 0.72 | +0.2 ± 0.52 |
| Seed 22, eighth, assignment | 18 | +1.4 ± 0.78 | +1.1 ± 1.40 | +3.3 ± 1.23 | −1.3 ± 0.47 |
| Seed 22, eighth, assignment | 24 | +0.8 ± 0.63 | +0.8 ± 1.20 | +4.0 ± 0.95 | −1.5 ± 0.17 |
| Seed 22, eighth, assignment | 36 | +3.2 ± 0.37 | +3.8 ± 0.96 | +1.5 ± 0.78 | −0.8 ± 0.79 |

- **The excitatory night without replay less the night without replay** lies between −6.7 and +4.4, beyond twice its SE at 8 of the 20
  checkpoints: below on seed 24's eighth image at blocks 24 and 36, −6.7 ± 1.52 and −4.9 ± 1.12, and above at the six others, among them
  seed 22's first mapping from block 18 on and its eighth image at block 36.
- **The excitatory night less the excitatory night without replay**, the replay with the hold set in both, lies between −0.9 and +13.7,
  beyond twice its SE at 8 of 20, all above; at block 36 it is −0.9 ± 1.06, +13.7 ± 1.00, +0.3 ± 1.01 and +1.5 ± 0.78.
- **The night under both holds less the excitatory night**, exploration 024's hold added, lies between −2.4 and +0.6, beyond twice its SE
  at 3 of 20, all below: seed 24's block 18, −2.4 ± 1.14, and seed 22's eighth image at blocks 18 and 24, −1.3 ± 0.47 and −1.5 ± 0.17.

Each stimulus's correct trials, the mean of the 48 reads, under each form:

| Cell | Block | A's: without replay | excitatory | both holds | excitatory without replay | plain on A's | 024's held | B's: without replay | excitatory | both holds | excitatory without replay | plain on A's | 024's held |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | 30.4 | 31.2 | 31.3 | 30.3 | 31.0 | 31.1 | 24.6 | 25.4 | 25.4 | 25.1 | 25.9 | 25.8 |
| Seed 23, first mapping, mirrored | 12 | 28.5 | 30.3 | 30.5 | 27.9 | 29.6 | 29.7 | 23.8 | 25.3 | 25.6 | 24.0 | 24.8 | 24.8 |
| Seed 23, first mapping, mirrored | 18 | 28.5 | 28.6 | 28.8 | 28.0 | 27.6 | 27.3 | 23.5 | 25.6 | 25.2 | 23.6 | 24.8 | 24.6 |
| Seed 23, first mapping, mirrored | 24 | 30.0 | 26.8 | 27.3 | 29.6 | 25.0 | 24.4 | 23.5 | 26.5 | 26.3 | 24.2 | 25.9 | 26.1 |
| Seed 23, first mapping, mirrored | 36 | 29.3 | 26.1 | 26.6 | 27.1 | 24.1 | 23.7 | 23.9 | 25.3 | 24.8 | 25.3 | 24.2 | 24.6 |
| Seed 24, eighth, assignment | 6 | 21.3 | 23.6 | 23.2 | 22.5 | 23.8 | 23.3 | 17.5 | 15.7 | 15.9 | 16.1 | 15.3 | 15.2 |
| Seed 24, eighth, assignment | 12 | 18.4 | 20.2 | 20.2 | 20.4 | 18.8 | 18.6 | 16.4 | 16.6 | 16.3 | 15.9 | 15.4 | 14.4 |
| Seed 24, eighth, assignment | 18 | 18.0 | 18.2 | 16.9 | 18.5 | 14.4 | 13.8 | 16.5 | 17.4 | 16.3 | 14.6 | 14.1 | 15.2 |
| Seed 24, eighth, assignment | 24 | 19.2 | 17.6 | 17.3 | 17.2 | 15.3 | 14.7 | 16.3 | 19.1 | 18.4 | 11.6 | 14.3 | 13.9 |
| Seed 24, eighth, assignment | 36 | 12.9 | 17.4 | 17.1 | 12.0 | 13.1 | 13.0 | 12.7 | 16.9 | 16.8 | 8.7 | 14.3 | 13.6 |
| Seed 22, first mapping, mirrored | 6 | 30.6 | 31.5 | 31.4 | 30.5 | 31.4 | 31.3 | 26.0 | 26.6 | 26.8 | 26.8 | 26.4 | 26.5 |
| Seed 22, first mapping, mirrored | 12 | 28.2 | 28.9 | 28.9 | 26.7 | 29.3 | 29.4 | 21.3 | 23.8 | 23.5 | 22.8 | 23.1 | 23.0 |
| Seed 22, first mapping, mirrored | 18 | 28.0 | 27.6 | 27.4 | 28.2 | 27.6 | 28.0 | 21.0 | 24.7 | 25.0 | 22.5 | 25.1 | 24.8 |
| Seed 22, first mapping, mirrored | 24 | 27.5 | 27.6 | 27.5 | 27.4 | 27.6 | 27.7 | 18.4 | 23.1 | 23.5 | 20.9 | 23.0 | 23.4 |
| Seed 22, first mapping, mirrored | 36 | 27.8 | 26.9 | 27.2 | 29.0 | 27.4 | 27.4 | 16.4 | 21.9 | 22.2 | 19.6 | 21.8 | 21.8 |
| Seed 22, eighth, assignment | 6 | 24.4 | 26.5 | 26.5 | 23.9 | 26.4 | 26.4 | 25.9 | 26.1 | 26.2 | 26.4 | 25.8 | 25.6 |
| Seed 22, eighth, assignment | 12 | 22.9 | 26.4 | 26.5 | 22.7 | 25.4 | 25.9 | 24.8 | 26.4 | 26.5 | 25.9 | 25.3 | 25.4 |
| Seed 22, eighth, assignment | 18 | 24.4 | 25.0 | 23.7 | 22.9 | 23.8 | 23.9 | 22.8 | 26.8 | 26.8 | 25.6 | 26.8 | 26.5 |
| Seed 22, eighth, assignment | 24 | 26.0 | 25.9 | 24.9 | 23.0 | 25.8 | 25.1 | 21.3 | 26.2 | 25.7 | 25.1 | 24.4 | 24.2 |
| Seed 22, eighth, assignment | 36 | 24.7 | 23.9 | 23.5 | 21.6 | 23.1 | 22.9 | 19.5 | 24.9 | 24.5 | 25.7 | 23.2 | 22.9 |

## 2. The side under each held night

The side under a held night is the stimulus whose difference from the night without replay is the larger. "A's less B's" is the paired
difference of the two, and "within 2 SE" marks it within twice that SE. H-47's side is the same reading of H-47's plain night, from H-47's
job, and 024's of its held night, from 024's job. Stimulus A answers the even places in all four cells, and B the odd places:

| Cell | Block | A's: excitatory less without replay | B's | A's less B's | Side under the excitatory night | A's: both holds less without replay | B's | A's less B's | Side under both holds | H-47's side | 024's side |
| :--- | ---: | ---: | ---: | ---: | :--- | ---: | ---: | ---: | :--- | :--- | :--- |
| Seed 23, first mapping, mirrored | 6 | +0.7 ± 0.38 | +0.8 ± 0.58 | −0.1 ± 0.61 | B, the odd places (within 2 SE) | +0.9 ± 0.36 | +0.9 ± 0.56 | 0.0 ± 0.58 | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 12 | +1.7 ± 0.51 | +1.4 ± 1.08 | +0.3 ± 1.17 | A, the even places (within 2 SE) | +2.0 ± 0.57 | +1.7 ± 0.97 | +0.3 ± 1.14 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 23, first mapping, mirrored | 18 | +0.1 ± 0.73 | +2.1 ± 0.73 | −2.0 ± 0.79 | B, the odd places | +0.3 ± 0.72 | +1.7 ± 0.78 | −1.5 ± 0.80 | B, the odd places (within 2 SE) | B, the odd places | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 24 | −3.2 ± 0.90 | +3.1 ± 0.57 | −6.3 ± 1.28 | B, the odd places | −2.7 ± 0.73 | +2.8 ± 0.60 | −5.5 ± 0.79 | B, the odd places | B, the odd places | B, the odd places |
| Seed 23, first mapping, mirrored | 36 | −3.1 ± 0.91 | +1.4 ± 1.07 | −4.5 ± 1.94 | B, the odd places | −2.6 ± 0.89 | +0.9 ± 1.14 | −3.5 ± 1.99 | B, the odd places (within 2 SE) | B, the odd places | B, the odd places |
| Seed 24, eighth, assignment | 6 | +2.3 ± 0.72 | −1.8 ± 0.68 | +4.1 ± 1.23 | A, the even places | +1.9 ± 0.87 | −1.6 ± 0.82 | +3.5 ± 1.47 | A, the even places | A, the even places | A, the even places |
| Seed 24, eighth, assignment | 12 | +1.8 ± 0.83 | +0.2 ± 1.26 | +1.6 ± 1.93 | A, the even places (within 2 SE) | +1.8 ± 0.97 | −0.1 ± 1.57 | +1.9 ± 2.32 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, eighth, assignment | 18 | +0.2 ± 1.05 | +1.0 ± 1.18 | −0.8 ± 0.87 | B, the odd places (within 2 SE) | −1.1 ± 0.81 | −0.2 ± 1.41 | −0.9 ± 1.19 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | 24 | −1.7 ± 1.40 | +2.8 ± 1.13 | −4.4 ± 1.71 | B, the odd places | −1.9 ± 1.06 | +2.1 ± 1.12 | −4.0 ± 1.26 | B, the odd places | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | 36 | +4.5 ± 0.69 | +4.3 ± 1.16 | +0.3 ± 1.42 | A, the even places (within 2 SE) | +4.2 ± 0.96 | +4.1 ± 0.94 | +0.1 ± 1.13 | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 22, first mapping, mirrored | 6 | +0.8 ± 0.35 | +0.6 ± 0.53 | +0.2 ± 0.67 | A, the even places (within 2 SE) | +0.8 ± 0.35 | +0.8 ± 0.52 | −0.1 ± 0.69 | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | 12 | +0.7 ± 0.47 | +2.5 ± 1.05 | −1.8 ± 0.90 | B, the odd places (within 2 SE) | +0.7 ± 0.73 | +2.2 ± 1.13 | −1.5 ± 1.00 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 22, first mapping, mirrored | 18 | −0.4 ± 1.22 | +3.7 ± 1.39 | −4.1 ± 1.49 | B, the odd places | −0.5 ± 1.54 | +4.0 ± 1.38 | −4.5 ± 1.51 | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, first mapping, mirrored | 24 | 0.0 ± 1.46 | +4.8 ± 0.84 | −4.7 ± 1.53 | B, the odd places | 0.0 ± 1.42 | +5.1 ± 0.97 | −5.1 ± 1.43 | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, first mapping, mirrored | 36 | −0.9 ± 1.23 | +5.6 ± 0.93 | −6.5 ± 1.55 | B, the odd places | −0.6 ± 1.13 | +5.9 ± 1.11 | −6.5 ± 1.44 | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | 6 | +2.1 ± 0.58 | +0.1 ± 0.58 | +2.0 ± 0.83 | A, the even places | +2.1 ± 0.65 | +0.3 ± 0.50 | +1.8 ± 0.95 | A, the even places (within 2 SE) | A, the even places | A, the even places |
| Seed 22, eighth, assignment | 12 | +3.5 ± 0.69 | +1.7 ± 0.75 | +1.8 ± 1.21 | A, the even places (within 2 SE) | +3.6 ± 0.63 | +1.8 ± 0.88 | +1.8 ± 1.01 | A, the even places (within 2 SE) | A, the even places | A, the even places (within 2 SE) |
| Seed 22, eighth, assignment | 18 | +0.6 ± 1.03 | +4.0 ± 0.78 | −3.4 ± 1.31 | B, the odd places | −0.7 ± 1.15 | +4.1 ± 0.91 | −4.7 ± 1.39 | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | 24 | −0.1 ± 0.64 | +4.9 ± 0.69 | −5.0 ± 0.86 | B, the odd places | −1.1 ± 0.69 | +4.4 ± 0.82 | −5.6 ± 1.18 | B, the odd places | B, the odd places (within 2 SE) | B, the odd places |
| Seed 22, eighth, assignment | 36 | −0.8 ± 1.22 | +5.4 ± 0.78 | −6.2 ± 1.98 | B, the odd places | −1.2 ± 1.20 | +5.0 ± 0.65 | −6.3 ± 1.52 | B, the odd places | B, the odd places | B, the odd places |

- **The side under the excitatory night is H-47's at 19 of the 20 checkpoints** and 024's at the same 19. It differs at seed 24's block 36,
  where it lies on A, the even places, within 2 SE, and H-47's and 024's on B, the odd places, within 2 SE. It lies beyond twice its SE at
  12 of the 20, H-47's at 11 and 024's at 10.
- **Under both holds it is H-47's at 17 of 20**, beyond twice its SE at 9. It differs at seed 24's block 36, on A, at seed 23's block 6,
  on A, and at seed 22's first mapping at block 6, on B, each within 2 SE on both sides.
- From block 18 to block 24 the side lies on B, the odd places, in all four cells under both held nights with replay, as in H-47.

Each stimulus's trials less H-47's plain night's and less 024's held night's, each with exploration 018's unpaired SE:

| Cell | Block | A's: excitatory less plain | B's: excitatory less plain | A's: both holds less plain | B's: both holds less plain | A's: excitatory less 024's held | B's: excitatory less 024's held | A's: both holds less 024's held | B's: both holds less 024's held |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | +0.1 ± 0.95 | −0.5 ± 0.91 | +0.3 ± 0.96 | −0.4 ± 0.92 | +0.1 ± 0.96 | −0.4 ± 0.97 | +0.2 ± 0.96 | −0.3 ± 0.98 |
| Seed 23, first mapping, mirrored | 12 | +0.7 ± 0.95 | +0.5 ± 1.27 | +0.9 ± 0.96 | +0.8 ± 1.25 | +0.5 ± 0.96 | +0.4 ± 1.28 | +0.8 ± 1.00 | +0.8 ± 1.22 |
| Seed 23, first mapping, mirrored | 18 | +1.0 ± 0.97 | +0.8 ± 1.09 | +1.2 ± 0.99 | +0.4 ± 1.12 | +1.3 ± 1.01 | +1.0 ± 1.20 | +1.5 ± 1.04 | +0.6 ± 1.22 |
| Seed 23, first mapping, mirrored | 24 | +1.8 ± 1.00 | +0.6 ± 1.13 | +2.3 ± 1.07 | +0.4 ± 1.06 | +2.4 ± 1.39 | +0.5 ± 1.03 | +2.9 ± 1.55 | +0.2 ± 1.19 |
| Seed 23, first mapping, mirrored | 36 | +2.1 ± 1.33 | +1.1 ± 0.93 | +2.6 ± 1.32 | +0.6 ± 0.92 | +2.4 ± 1.51 | +0.7 ± 0.85 | +2.9 ± 1.50 | +0.2 ± 0.92 |
| Seed 24, eighth, assignment | 6 | −0.3 ± 0.82 | +0.4 ± 0.98 | −0.7 ± 0.83 | +0.6 ± 0.95 | +0.3 ± 0.89 | +0.5 ± 1.07 | −0.1 ± 0.83 | +0.6 ± 1.03 |
| Seed 24, eighth, assignment | 12 | +1.4 ± 1.21 | +1.2 ± 1.02 | +1.4 ± 1.25 | +0.9 ± 1.11 | +1.6 ± 1.05 | +2.2 ± 0.97 | +1.6 ± 1.13 | +1.9 ± 1.07 |
| Seed 24, eighth, assignment | 18 | +3.8 ± 1.30 | +3.3 ± 0.84 | +2.5 ± 1.09 | +2.2 ± 1.09 | +4.4 ± 1.30 | +2.2 ± 1.42 | +3.1 ± 1.11 | +1.1 ± 1.68 |
| Seed 24, eighth, assignment | 24 | +2.3 ± 1.39 | +4.8 ± 0.88 | +2.0 ± 0.88 | +4.1 ± 0.95 | +2.9 ± 1.33 | +5.2 ± 1.36 | +2.6 ± 1.07 | +4.5 ± 1.27 |
| Seed 24, eighth, assignment | 36 | +4.3 ± 1.03 | +2.6 ± 1.06 | +4.0 ± 0.85 | +2.5 ± 1.40 | +4.4 ± 1.24 | +3.3 ± 1.09 | +4.1 ± 1.15 | +3.1 ± 1.08 |
| Seed 22, first mapping, mirrored | 6 | +0.1 ± 0.86 | +0.3 ± 0.78 | 0.0 ± 0.85 | +0.5 ± 0.79 | +0.1 ± 0.86 | +0.1 ± 0.79 | +0.1 ± 0.85 | +0.3 ± 0.80 |
| Seed 22, first mapping, mirrored | 12 | −0.4 ± 1.14 | +0.7 ± 1.16 | −0.4 ± 1.13 | +0.4 ± 1.12 | −0.5 ± 1.13 | +0.8 ± 1.18 | −0.5 ± 1.18 | +0.5 ± 1.15 |
| Seed 22, first mapping, mirrored | 18 | 0.0 ± 0.98 | −0.4 ± 1.06 | −0.1 ± 1.05 | −0.1 ± 1.08 | −0.4 ± 1.02 | 0.0 ± 1.06 | −0.6 ± 1.08 | +0.2 ± 1.08 |
| Seed 22, first mapping, mirrored | 24 | −0.1 ± 0.74 | +0.2 ± 1.59 | −0.1 ± 0.78 | +0.5 ± 1.47 | −0.2 ± 0.77 | −0.2 ± 1.69 | −0.2 ± 0.80 | +0.1 ± 1.55 |
| Seed 22, first mapping, mirrored | 36 | −0.5 ± 0.94 | +0.2 ± 1.16 | −0.2 ± 0.96 | +0.5 ± 1.05 | −0.5 ± 1.08 | +0.2 ± 1.16 | −0.3 ± 1.06 | +0.5 ± 0.99 |
| Seed 22, eighth, assignment | 6 | +0.1 ± 0.75 | +0.3 ± 1.29 | +0.1 ± 0.77 | +0.4 ± 1.28 | +0.1 ± 0.75 | +0.5 ± 1.32 | +0.1 ± 0.77 | +0.6 ± 1.28 |
| Seed 22, eighth, assignment | 12 | +1.0 ± 0.69 | +1.2 ± 1.10 | +1.1 ± 0.72 | +1.3 ± 1.08 | +0.5 ± 0.69 | +1.1 ± 1.10 | +0.6 ± 0.85 | +1.2 ± 1.08 |
| Seed 22, eighth, assignment | 18 | +1.3 ± 1.55 | 0.0 ± 0.67 | 0.0 ± 1.68 | 0.0 ± 0.76 | +1.1 ± 0.96 | +0.3 ± 0.83 | −0.1 ± 0.99 | +0.3 ± 1.04 |
| Seed 22, eighth, assignment | 24 | +0.2 ± 1.39 | +1.8 ± 1.06 | −0.8 ± 1.18 | +1.3 ± 1.01 | +0.8 ± 1.22 | +2.0 ± 0.90 | −0.2 ± 1.17 | +1.6 ± 0.94 |
| Seed 22, eighth, assignment | 36 | +0.8 ± 1.28 | +1.6 ± 1.13 | +0.4 ± 1.26 | +1.3 ± 1.35 | +1.1 ± 1.30 | +2.0 ± 1.17 | +0.6 ± 1.34 | +1.6 ± 1.26 |

- **Under the excitatory night less the plain night**, A's trials lie between −0.5 and +4.3, beyond 2 SE at 2 of 20, and B's between −0.5
  and +4.8, beyond 2 SE at 3 of 20, all on seed 24's eighth image: A's at blocks 18 and 36, B's at blocks 18, 24 and 36.

Under the excitatory night without replay, its side, read the same way against the night without replay:

| Cell | Block | A's: excitatory without replay less without replay | B's: the same | A's less B's | Its side |
| :--- | ---: | ---: | ---: | ---: | :--- |
| Seed 23, first mapping, mirrored | 6 | −0.1 ± 0.19 | +0.5 ± 0.46 | −0.7 ± 0.59 | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 12 | −0.6 ± 0.54 | +0.2 ± 0.92 | −0.8 ± 1.18 | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 18 | −0.5 ± 0.46 | +0.1 ± 0.89 | −0.6 ± 0.82 | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 24 | −0.4 ± 0.48 | +0.7 ± 0.56 | −1.1 ± 0.72 | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 36 | −2.2 ± 0.47 | +1.4 ± 1.40 | −3.5 ± 1.60 | B, the odd places |
| Seed 24, eighth, assignment | 6 | +1.2 ± 0.61 | −1.4 ± 0.45 | +2.6 ± 0.86 | A, the even places |
| Seed 24, eighth, assignment | 12 | +1.9 ± 0.92 | −0.5 ± 0.58 | +2.4 ± 1.45 | A, the even places (within 2 SE) |
| Seed 24, eighth, assignment | 18 | +0.5 ± 0.57 | −1.9 ± 1.25 | +2.4 ± 1.44 | A, the even places (within 2 SE) |
| Seed 24, eighth, assignment | 24 | −2.0 ± 0.97 | −4.7 ± 1.11 | +2.6 ± 1.42 | A, the even places (within 2 SE) |
| Seed 24, eighth, assignment | 36 | −1.0 ± 0.41 | −4.0 ± 1.07 | +3.0 ± 1.16 | A, the even places |
| Seed 22, first mapping, mirrored | 6 | −0.1 ± 0.16 | +0.8 ± 0.33 | −0.9 ± 0.40 | B, the odd places |
| Seed 22, first mapping, mirrored | 12 | −1.5 ± 0.24 | +1.4 ± 0.80 | −2.9 ± 0.82 | B, the odd places |
| Seed 22, first mapping, mirrored | 18 | +0.2 ± 0.39 | +1.5 ± 0.60 | −1.3 ± 0.62 | B, the odd places |
| Seed 22, first mapping, mirrored | 24 | −0.2 ± 0.46 | +2.5 ± 0.89 | −2.7 ± 0.97 | B, the odd places |
| Seed 22, first mapping, mirrored | 36 | +1.2 ± 0.06 | +3.2 ± 0.90 | −2.0 ± 0.86 | B, the odd places |
| Seed 22, eighth, assignment | 6 | −0.5 ± 0.32 | +0.5 ± 0.49 | −1.0 ± 0.60 | B, the odd places (within 2 SE) |
| Seed 22, eighth, assignment | 12 | −0.2 ± 0.55 | +1.1 ± 0.43 | −1.3 ± 0.79 | B, the odd places (within 2 SE) |
| Seed 22, eighth, assignment | 18 | −1.5 ± 0.46 | +2.9 ± 0.38 | −4.3 ± 0.32 | B, the odd places |
| Seed 22, eighth, assignment | 24 | −3.0 ± 0.51 | +3.8 ± 0.53 | −6.8 ± 0.83 | B, the odd places |
| Seed 22, eighth, assignment | 36 | −3.1 ± 0.46 | +6.3 ± 0.28 | −9.3 ± 0.67 | B, the odd places |

Its side lies on B, the odd places, at every checkpoint on seed 23's and seed 22's cells, and on A, the even places, at every checkpoint on
seed 24's; it is H-47's side at 13 of the 20 checkpoints.

## 3. The pair and A's couplings

The pair's count under the excitatory night and the night under both holds at each checkpoint, and at block 36 under the other forms. H-47's
floor for clause 2 is 56:

| Cell | Excitatory at 6 | 12 | 18 | 24 | 36 | Both holds at 6 | 12 | 18 | 24 | 36 | At 36: without replay | excitatory without replay | plain on A's | 024's held |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 48.9 | 59.7 | 62.3 | 62.6 | 63.0 | 48.9 | 59.5 | 62.1 | 62.7 | 63.2 | 63.5 | 63.2 | 63.2 | 63.1 |
| Seed 24, eighth, assignment | 61.6 | 62.3 | 62.0 | 61.7 | 62.6 | 61.6 | 62.3 | 62.4 | 62.0 | 62.8 | 62.6 | 63.0 | 62.9 | 63.2 |
| Seed 22, first mapping, mirrored | 54.6 | 61.2 | 61.9 | 62.0 | 62.9 | 54.5 | 61.5 | 62.0 | 62.1 | 62.5 | 63.1 | 62.9 | 62.5 | 62.2 |
| Seed 22, eighth, assignment | 63.0 | 62.6 | 63.4 | 62.1 | 62.9 | 62.9 | 62.6 | 63.3 | 62.4 | 63.2 | 62.6 | 63.0 | 62.9 | 63.0 |

A's highest coupling over H-42's under each form, in any image read and at any block's end, as clause 3 reads it; its bound is 1.30:

| Cell | Without replay | Excitatory | Both holds | Excitatory without replay | Plain on A's (H-47) | 024's held |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 1.082 | 1.151 | 1.145 | 1.082 | 1.158 | 1.156 |
| Seed 24, eighth, assignment | 1.179 | 1.196 | 1.179 | 1.179 | 1.200 | 1.192 |
| Seed 22, first mapping, mirrored | 1.114 | 1.183 | 1.181 | 1.114 | 1.182 | 1.181 |
| Seed 22, eighth, assignment | 1.133 | 1.193 | 1.180 | 1.133 | 1.184 | 1.174 |

Under the excitatory night the pair stands at 62.6 to 63.0 at block 36, and under both holds at 62.5 to 63.2, where H-47's plain night gave
62.5 to 63.2. A's highest coupling is 1.151 to 1.196 of H-42's under the excitatory night and 1.145 to 1.181 under both holds, where H-47's
plain night gave 1.158 to 1.200.

## 4. What each night moved

What the four nights moved, summed over the four, the mean over the three paths, from the class matrix in the images before and after each
night, in M. A group's held units are its readout's two classes, the pair's own kind left out; "onto their own group" counts the pair's own
kind among the targets, as exploration 024's "onto their own" did. H-47's plain night is exploration 020's job's, 024's held night 024's
job's:

| Cell | Form | The odd places' held units onto their own group | onto the even places' excitatory | onto the inhibitory units | onto A's and B's units | onto the other excitatory | The even places' held units onto their own group | onto the odd places' excitatory | onto the inhibitory units | onto A's and B's units | onto the other excitatory |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | without replay | +0.19 | −0.15 | +1.35 | +0.12 | −0.03 | −0.25 | −0.54 | +1.25 | −0.26 | −0.01 |
| Seed 23, first mapping, mirrored | excitatory | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, mirrored | both holds | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, mirrored | excitatory without replay | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, mirrored | plain on A's (H-47) | −2.02 | −5.68 | −2.03 | −0.67 | −0.02 | −1.66 | −5.56 | −1.94 | −0.52 | 0.00 |
| Seed 23, first mapping, mirrored | 024's held | −2.05 | −5.78 | −2.13 | −0.71 | −0.02 | −1.67 | −5.65 | −2.04 | −0.56 | 0.00 |
| Seed 24, eighth, assignment | without replay | −0.50 | +0.10 | +1.05 | −0.04 | +0.08 | −0.72 | −0.36 | +0.99 | −0.07 | +0.09 |
| Seed 24, eighth, assignment | excitatory | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, assignment | both holds | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, assignment | excitatory without replay | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, assignment | plain on A's (H-47) | −1.85 | −6.12 | −2.35 | −0.96 | −0.04 | −2.71 | −6.23 | −2.59 | −0.79 | +0.04 |
| Seed 24, eighth, assignment | 024's held | −2.08 | −6.35 | −2.68 | −1.12 | −0.05 | −2.80 | −6.58 | −2.89 | −0.92 | +0.03 |
| Seed 22, first mapping, mirrored | without replay | −0.41 | −0.42 | +1.38 | −0.02 | +0.03 | −0.78 | −0.55 | +1.36 | −0.20 | −0.02 |
| Seed 22, first mapping, mirrored | excitatory | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, mirrored | both holds | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, mirrored | excitatory without replay | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, mirrored | plain on A's (H-47) | −2.39 | −4.73 | −1.65 | −0.34 | +0.01 | −2.30 | −6.01 | −2.09 | −0.23 | −0.02 |
| Seed 22, first mapping, mirrored | 024's held | −2.44 | −4.76 | −1.70 | −0.35 | +0.01 | −2.31 | −6.06 | −2.13 | −0.24 | −0.02 |
| Seed 22, eighth, assignment | without replay | −0.52 | −0.37 | +0.77 | −0.01 | +0.04 | −0.83 | −0.68 | +1.15 | −0.15 | −0.03 |
| Seed 22, eighth, assignment | excitatory | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, assignment | both holds | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, assignment | excitatory without replay | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, assignment | plain on A's (H-47) | −2.49 | −5.68 | −2.05 | −0.46 | +0.03 | −2.79 | −6.56 | −2.27 | −0.41 | −0.01 |
| Seed 22, eighth, assignment | 024's held | −2.64 | −6.00 | −2.33 | −0.63 | +0.04 | −2.94 | −6.76 | −2.51 | −0.51 | −0.02 |

- **Under every held form the held units' synapses show 0.00**, and the class matrix's held entries moved by exactly nothing, in LSB, in
  every one of the 144 held nights; no quiet tick after a night moved any of the held units' synapses (below).
- **Under H-47's plain night the held units' synapses lost most onto the other readout's excitatory units**: −4.73 to −6.56 M a group over
  four nights, against −1.66 to −2.79 M onto their own group, −1.65 to −2.59 M onto the inhibitory units, −0.23 to −0.96 M onto A's and B's
  units and −0.04 to +0.04 M onto the other excitatory units. Under 024's held night the same splits are −4.76 to −6.76, −1.67 to −2.94,
  −1.70 to −2.89, −0.24 to −1.12 and −0.05 to +0.04 M.

The rest of each night's change: the inhibition onto each group's excitatory units and onto the other targets, and the excitatory units
outside the hold onto every target:

| Cell | Form | Inhibitory onto the odd places' excitatory units | onto the even places' | onto the inhibitory units | onto A's and B's units | onto the other excitatory | C's own onto all | D's own onto all | A's and B's units onto all | The other excitatory onto all |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | without replay | −1.79 | −2.04 | −0.59 | −0.90 | −0.04 | −0.01 | −0.05 | −0.04 | −0.02 |
| Seed 23, first mapping, mirrored | excitatory | +9.41 | +9.56 | +3.37 | +1.10 | −0.03 | −0.06 | −0.05 | −0.08 | −0.11 |
| Seed 23, first mapping, mirrored | both holds | 0.00 | 0.00 | +3.42 | +1.12 | −0.03 | −0.06 | −0.05 | −0.07 | −0.11 |
| Seed 23, first mapping, mirrored | excitatory without replay | −1.73 | −1.89 | −0.81 | −0.84 | −0.03 | −0.04 | −0.05 | −0.04 | −0.03 |
| Seed 23, first mapping, mirrored | plain on A's (H-47) | +8.33 | +8.77 | +2.94 | +1.07 | −0.03 | −0.04 | −0.04 | −0.07 | −0.12 |
| Seed 23, first mapping, mirrored | 024's held | 0.00 | 0.00 | +2.94 | +1.08 | −0.03 | −0.05 | −0.05 | −0.07 | −0.12 |
| Seed 24, eighth, assignment | without replay | +0.98 | +0.78 | +0.54 | −0.99 | +0.02 | −0.01 | −0.05 | −0.03 | −0.13 |
| Seed 24, eighth, assignment | excitatory | +12.61 | +13.27 | +4.07 | +0.25 | +0.04 | +0.05 | −0.07 | −0.10 | −0.26 |
| Seed 24, eighth, assignment | both holds | 0.00 | 0.00 | +4.27 | +0.25 | +0.05 | +0.04 | −0.08 | −0.10 | −0.25 |
| Seed 24, eighth, assignment | excitatory without replay | +0.97 | +0.72 | +0.29 | −0.92 | +0.01 | 0.00 | −0.05 | −0.04 | −0.13 |
| Seed 24, eighth, assignment | plain on A's (H-47) | +11.38 | +11.76 | +3.66 | +0.22 | +0.04 | +0.09 | −0.05 | −0.10 | −0.21 |
| Seed 24, eighth, assignment | 024's held | 0.00 | 0.00 | +3.73 | +0.21 | +0.04 | +0.09 | −0.05 | −0.10 | −0.21 |
| Seed 22, first mapping, mirrored | without replay | −1.92 | −2.23 | −0.39 | −0.99 | +0.03 | −0.03 | +0.01 | −0.03 | +0.13 |
| Seed 22, first mapping, mirrored | excitatory | +7.47 | +7.95 | +3.44 | +0.87 | +0.05 | −0.12 | −0.14 | −0.07 | −0.07 |
| Seed 22, first mapping, mirrored | both holds | 0.00 | 0.00 | +3.49 | +0.87 | +0.04 | −0.12 | −0.14 | −0.07 | −0.08 |
| Seed 22, first mapping, mirrored | excitatory without replay | −1.71 | −2.01 | −0.65 | −0.87 | +0.01 | −0.04 | 0.00 | −0.04 | +0.05 |
| Seed 22, first mapping, mirrored | plain on A's (H-47) | +6.52 | +7.22 | +3.05 | +0.70 | +0.06 | −0.10 | −0.13 | −0.08 | −0.04 |
| Seed 22, first mapping, mirrored | 024's held | 0.00 | 0.00 | +3.07 | +0.71 | +0.06 | −0.11 | −0.13 | −0.08 | −0.04 |
| Seed 22, eighth, assignment | without replay | +0.89 | +0.60 | +0.62 | −1.11 | +0.03 | −0.04 | +0.03 | −0.03 | +0.11 |
| Seed 22, eighth, assignment | excitatory | +11.88 | +12.35 | +4.11 | +0.08 | +0.05 | −0.07 | −0.10 | −0.05 | −0.12 |
| Seed 22, eighth, assignment | both holds | 0.00 | 0.00 | +4.36 | +0.09 | +0.05 | −0.07 | −0.12 | −0.05 | −0.12 |
| Seed 22, eighth, assignment | excitatory without replay | +0.86 | +0.58 | +0.38 | −1.06 | +0.01 | −0.04 | +0.03 | −0.03 | +0.06 |
| Seed 22, eighth, assignment | plain on A's (H-47) | +10.61 | +11.30 | +3.72 | 0.00 | +0.05 | −0.03 | −0.09 | −0.07 | −0.07 |
| Seed 22, eighth, assignment | 024's held | 0.00 | 0.00 | +3.77 | 0.00 | +0.05 | −0.04 | −0.10 | −0.07 | −0.07 |

- **Under the excitatory night the inhibition onto each group's excitatory units rose by 7.47 to 13.27 M** over four nights, where H-47's
  plain night raised it by 6.52 to 11.76 M: by 0.73 to 1.52 M more, group by group. Onto the inhibitory units it rose by 3.37 to 4.11 M,
  where the plain night's rose by 2.94 to 3.72 M.
- **Under both holds the inhibition onto each group's excitatory units shows 0.00**: exploration 024's held entries moved, night by night,
  by exactly what the quiet ticks moved of its held synapses, nothing in 43 of the 48 nights (section 5).
- **The excitatory units outside the hold**: C's own onto all moved by −0.12 to +0.05 M and D's own by −0.14 to −0.05 M under the excitatory
  night, A's and B's units onto all by −0.10 to −0.05 M, and the other excitatory units by −0.26 to −0.07 M.

The held units' synapses night by night, against what the quiet ticks after each night moved:

| Cell | Form | Nights the held units' entries moved by nothing | Nights they moved by what the quiet ticks moved | Nights the quiet ticks moved held synapses |
| :--- | :--- | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | excitatory | 12 of 12 | 12 of 12 | 0 |
| Seed 23, first mapping, mirrored | both holds | 12 of 12 | 12 of 12 | 0 |
| Seed 23, first mapping, mirrored | excitatory without replay | 12 of 12 | 12 of 12 | 0 |
| Seed 24, eighth, assignment | excitatory | 12 of 12 | 12 of 12 | 0 |
| Seed 24, eighth, assignment | both holds | 12 of 12 | 12 of 12 | 0 |
| Seed 24, eighth, assignment | excitatory without replay | 12 of 12 | 12 of 12 | 0 |
| Seed 22, first mapping, mirrored | excitatory | 12 of 12 | 12 of 12 | 0 |
| Seed 22, first mapping, mirrored | both holds | 12 of 12 | 12 of 12 | 0 |
| Seed 22, first mapping, mirrored | excitatory without replay | 12 of 12 | 12 of 12 | 0 |
| Seed 22, eighth, assignment | excitatory | 12 of 12 | 12 of 12 | 0 |
| Seed 22, eighth, assignment | both holds | 12 of 12 | 12 of 12 | 0 |
| Seed 22, eighth, assignment | excitatory without replay | 12 of 12 | 12 of 12 | 0 |

**The held units' synapses from the start image to each checkpoint's image**, the mean over the three paths, under the forms that do not
hold them. Under the three held forms they stood within −22 713 and +13 307 LSB of the start image at every checkpoint, path by path. That is
the awake blocks' change: summed over a path's awake blocks to block 36 it is −22 713 to +12 110 LSB under the held forms, where H-47's
plain night's awake blocks moved them by −26 086 to +11 533 LSB and its nights by −19 715 412 to −23 678 183 LSB a path:

| Cell | Block | Without replay: odd | even | Plain on A's: odd | even | 024's held: odd | even |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | −0.15 | −0.27 | −3.98 | −3.67 | −4.04 | −3.68 |
| Seed 23, first mapping, mirrored | 12 | +0.21 | −0.23 | −6.96 | −6.46 | −7.10 | −6.55 |
| Seed 23, first mapping, mirrored | 18 | +0.81 | −0.18 | −8.89 | −8.38 | −9.13 | −8.53 |
| Seed 23, first mapping, mirrored | 24 | +1.49 | +0.19 | −10.43 | −9.68 | −10.69 | −9.92 |
| Seed 23, first mapping, mirrored | 36 | +1.49 | +0.19 | −10.43 | −9.69 | −10.69 | −9.92 |
| Seed 24, eighth, assignment | 6 | −0.06 | −0.30 | −4.33 | −4.87 | −4.58 | −5.11 |
| Seed 24, eighth, assignment | 12 | +0.07 | −0.30 | −7.41 | −8.41 | −7.93 | −8.83 |
| Seed 24, eighth, assignment | 18 | +0.36 | −0.12 | −9.59 | −10.64 | −10.28 | −11.33 |
| Seed 24, eighth, assignment | 24 | +0.68 | −0.06 | −11.31 | −12.27 | −12.29 | −13.16 |
| Seed 24, eighth, assignment | 36 | +0.68 | −0.06 | −11.31 | −12.28 | −12.29 | −13.16 |
| Seed 22, first mapping, mirrored | 6 | −0.28 | −0.19 | −3.54 | −3.90 | −3.59 | −3.93 |
| Seed 22, first mapping, mirrored | 12 | −0.12 | −0.27 | −6.17 | −6.92 | −6.24 | −7.03 |
| Seed 22, first mapping, mirrored | 18 | +0.20 | −0.20 | −7.80 | −9.07 | −7.84 | −9.21 |
| Seed 22, first mapping, mirrored | 24 | +0.55 | −0.20 | −9.11 | −10.65 | −9.25 | −10.77 |
| Seed 22, first mapping, mirrored | 36 | +0.55 | −0.20 | −9.11 | −10.65 | −9.25 | −10.77 |
| Seed 22, eighth, assignment | 6 | −0.35 | −0.26 | −4.44 | −4.61 | −4.66 | −4.76 |
| Seed 22, eighth, assignment | 12 | −0.33 | −0.41 | −7.41 | −7.85 | −7.94 | −8.20 |
| Seed 22, eighth, assignment | 18 | −0.30 | −0.53 | −9.38 | −10.13 | −10.12 | −10.67 |
| Seed 22, eighth, assignment | 24 | −0.10 | −0.52 | −10.65 | −12.04 | −11.57 | −12.74 |
| Seed 22, eighth, assignment | 36 | −0.10 | −0.52 | −10.65 | −12.04 | −11.57 | −12.74 |

Each night's change of the engine's two sums, from its `start` line to its `end` line, the mean over the three paths:

| Cell | Gap | Without replay: inhibitory sum, M | excitatory sum | Excitatory: inhibitory | excitatory | Both holds: inhibitory | excitatory | Excitatory without replay: inhibitory | excitatory | Plain on A's: inhibitory | excitatory | 024's held: inhibitory | excitatory |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 6 | −1.19 | −0.48 | +9.31 | −0.08 | +1.85 | −0.08 | −1.17 | −0.06 | +8.85 | −7.72 | +1.74 | −7.79 |
| Seed 23, first mapping, mirrored | 12 | −1.40 | +0.37 | +6.46 | −0.07 | +1.14 | −0.08 | −1.42 | −0.04 | +5.81 | −5.84 | +1.07 | −6.01 |
| Seed 23, first mapping, mirrored | 18 | −1.43 | +0.64 | +4.35 | −0.09 | +0.84 | −0.08 | −1.34 | −0.02 | +3.79 | −3.93 | +0.67 | −4.09 |
| Seed 23, first mapping, mirrored | 24 | −1.33 | +1.02 | +3.30 | −0.07 | +0.68 | −0.06 | −1.38 | −0.05 | +2.62 | −2.89 | +0.52 | −3.01 |
| Seed 24, eighth, assignment | 6 | +0.32 | −0.47 | +9.29 | −0.17 | +1.39 | −0.19 | +0.33 | −0.09 | +8.80 | −9.36 | +1.31 | −9.85 |
| Seed 24, eighth, assignment | 12 | +0.25 | +0.10 | +8.07 | −0.08 | +1.14 | −0.10 | +0.34 | −0.03 | +7.16 | −6.68 | +0.99 | −7.14 |
| Seed 24, eighth, assignment | 18 | +0.32 | +0.43 | +6.97 | −0.08 | +1.07 | −0.06 | +0.23 | −0.04 | +5.92 | −4.45 | +0.87 | −4.88 |
| Seed 24, eighth, assignment | 24 | +0.44 | +0.34 | +5.90 | −0.05 | +0.96 | −0.04 | +0.17 | −0.06 | +5.16 | −3.38 | +0.81 | −3.85 |
| Seed 22, first mapping, mirrored | 6 | −1.32 | −0.44 | +7.80 | −0.12 | +1.72 | −0.12 | −1.32 | +0.02 | +7.36 | −7.55 | +1.61 | −7.63 |
| Seed 22, first mapping, mirrored | 12 | −1.39 | +0.09 | +5.47 | −0.12 | +1.12 | −0.13 | −1.33 | 0.00 | +4.75 | −5.75 | +0.96 | −5.85 |
| Seed 22, first mapping, mirrored | 18 | −1.39 | +0.40 | +3.71 | −0.08 | +0.89 | −0.07 | −1.34 | −0.01 | +3.21 | −3.85 | +0.74 | −3.85 |
| Seed 22, first mapping, mirrored | 24 | −1.40 | +0.37 | +2.81 | −0.09 | +0.68 | −0.09 | −1.24 | −0.03 | +2.24 | −2.96 | +0.53 | −3.03 |
| Seed 22, eighth, assignment | 6 | +0.28 | −0.61 | +8.83 | −0.13 | +1.40 | −0.13 | +0.25 | 0.00 | +8.35 | −9.17 | +1.29 | −9.55 |
| Seed 22, eighth, assignment | 12 | +0.23 | −0.11 | +7.48 | −0.12 | +1.15 | −0.12 | +0.16 | +0.01 | +6.67 | −6.30 | +0.96 | −6.83 |
| Seed 22, eighth, assignment | 18 | +0.24 | −0.06 | +6.53 | −0.07 | +1.02 | −0.07 | +0.20 | +0.01 | +5.74 | −4.29 | +0.82 | −4.68 |
| Seed 22, eighth, assignment | 24 | +0.28 | +0.23 | +5.63 | −0.03 | +0.93 | −0.03 | +0.17 | 0.00 | +4.92 | −3.19 | +0.75 | −3.54 |

An excitatory night, the mean of its three paths, raised the inhibitory sum by 2.81 to 9.31 M, where H-47's plain night raised it by 2.24 to
8.85 M, and lowered the excitatory sum by 0.03 to 0.17 M, where the plain night lowered it by 2.89 to 9.36 M. A night under both holds
raised the inhibitory sum by 0.68 to 1.85 M.

## 5. What the hold leaves

At each night with the excitatory hold, over its twelve nights a cell and form (ranges are over the twelve): the synapses held; their traces
cleared at the night's last tick, how many were not zero and what they held above and below zero; the excitatory synapses from other units
the night moved; the nights in which a held unit fired at or after the night's last tick, and the nights in which the quiet ticks moved held
synapses:

| Cell | Form | Nights | Held synapses | Traces cleared, not zero | cleared above zero, LSB | below zero | Other excitatory synapses moved | Nights a held synapse's presynaptic unit fired at or after the night's last tick | Nights held synapses moved in the quiet ticks: how many, their change |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | excitatory | 12 | 21 248 | 12 550 to 12 966 | 584 939 to 669 893 | −1 289 465 to −1 032 206 | 4 960 to 4 987 | 0 | none |
| Seed 23, first mapping, mirrored | both holds | 12 | 21 248 | 12 482 to 13 098 | 586 348 to 670 724 | −1 324 690 to −1 040 214 | 4 965 to 4 988 | 0 | none |
| Seed 23, first mapping, mirrored | excitatory without replay | 12 | 21 248 | 14 073 to 14 641 | 444 370 to 484 367 | −554 134 to −475 473 | 4 960 to 4 988 | 3 | none |
| Seed 24, eighth, assignment | excitatory | 12 | 21 248 | 12 725 to 13 149 | 641 070 to 716 667 | −1 294 859 to −1 152 116 | 4 951 to 4 989 | 0 | none |
| Seed 24, eighth, assignment | both holds | 12 | 21 248 | 13 112 to 13 375 | 635 119 to 725 774 | −1 363 010 to −1 238 193 | 4 953 to 4 986 | 0 | none |
| Seed 24, eighth, assignment | excitatory without replay | 12 | 21 248 | 14 397 to 14 876 | 468 846 to 562 757 | −641 198 to −552 760 | 4 942 to 4 973 | 3 | none |
| Seed 22, first mapping, mirrored | excitatory | 12 | 21 248 | 12 096 to 12 534 | 569 130 to 670 032 | −1 164 880 to −932 870 | 4 953 to 4 987 | 0 | none |
| Seed 22, first mapping, mirrored | both holds | 12 | 21 248 | 12 172 to 12 601 | 557 287 to 662 125 | −1 160 454 to −951 810 | 4 953 to 4 986 | 0 | none |
| Seed 22, first mapping, mirrored | excitatory without replay | 12 | 21 248 | 14 050 to 14 609 | 432 285 to 520 035 | −551 826 to −464 663 | 4 942 to 4 981 | 0 | none |
| Seed 22, eighth, assignment | excitatory | 12 | 21 248 | 12 476 to 12 901 | 583 093 to 706 291 | −1 311 940 to −1 052 211 | 4 949 to 4 989 | 3 | none |
| Seed 22, eighth, assignment | both holds | 12 | 21 248 | 12 655 to 13 126 | 602 026 to 706 820 | −1 396 306 to −1 182 601 | 4 949 to 4 988 | 3 | none |
| Seed 22, eighth, assignment | excitatory without replay | 12 | 21 248 | 14 177 to 14 515 | 441 547 to 500 490 | −602 671 to −536 329 | 4 934 to 4 977 | 0 | none |

- **Before the night's first tick** 12 337 to 13 416 of the 21 248 held synapses carried a trace that was not zero.
- **A held unit fired at or after the night's last tick** in 3 of the 48 excitatory nights and the same 3 of the 48 nights under both
  holds, all on seed 22's eighth image after block 12, and in 6 of the 48 excitatory nights without replay, 3 on seed 23 and 3 on seed 24.
  The quiet ticks moved none of the held units' synapses in any of the 144 nights; after them 0 to 18 of those synapses carried a trace.

The night's pairings still to be read, as What ran defines them, each read at the held synapse's presynaptic unit's next spike if that spike
comes before the target's next spike:

| Cell | Form | Pending gains: held synapses | their sum, LSB | their largest | Pending losses with a presynaptic spike at the next tick: held synapses | their sum, LSB | their largest |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | excitatory | 10 879 to 11 278 | 507 930 to 647 736 | 328 | 21 248 | 56 478 to 71 535 | 108 to 130 |
| Seed 23, first mapping, mirrored | both holds | 10 885 to 11 206 | 509 807 to 647 504 | 328 | 21 248 | 57 136 to 71 707 | 108 to 130 |
| Seed 23, first mapping, mirrored | excitatory without replay | 11 295 to 11 448 | 275 257 to 323 633 | 327 to 328 | 21 248 | 58 091 to 79 736 | 121 to 131 |
| Seed 24, eighth, assignment | excitatory | 11 042 to 11 257 | 568 110 to 663 569 | 328 | 21 248 | 47 851 to 73 871 | 106 to 147 |
| Seed 24, eighth, assignment | both holds | 10 910 to 11 149 | 589 721 to 674 358 | 328 | 21 248 | 46 563 to 75 375 | 113 to 142 |
| Seed 24, eighth, assignment | excitatory without replay | 11 187 to 11 536 | 284 046 to 341 160 | 327 to 328 | 21 248 | 56 594 to 91 163 | 106 to 131 |
| Seed 22, first mapping, mirrored | excitatory | 11 177 to 11 619 | 482 753 to 557 058 | 328 | 21 248 | 60 967 to 94 482 | 102 to 149 |
| Seed 22, first mapping, mirrored | both holds | 11 081 to 11 571 | 482 210 to 559 490 | 328 | 21 248 | 58 722 to 93 458 | 102 to 149 |
| Seed 22, first mapping, mirrored | excitatory without replay | 11 184 to 11 722 | 266 798 to 342 559 | 326 to 328 | 21 248 | 57 991 to 88 885 | 116 to 147 |
| Seed 22, eighth, assignment | excitatory | 11 006 to 11 367 | 499 297 to 581 746 | 328 | 21 248 | 52 869 to 79 743 | 108 to 129 |
| Seed 22, eighth, assignment | both holds | 10 899 to 11 391 | 559 587 to 595 416 | 328 | 21 248 | 52 261 to 80 355 | 108 to 129 |
| Seed 22, eighth, assignment | excitatory without replay | 11 363 to 11 535 | 279 758 to 348 779 | 326 to 328 | 21 248 | 65 132 to 79 391 | 115 to 125 |

- **The pending gain**, the target's spike after the presynaptic unit's last, is on 10 879 to 11 619 held synapses under the excitatory
  night, 482 753 to 663 569 LSB together, the largest 328 LSB, which is $A_+$. Under both holds it is on 10 885 to 11 571, 482 210 to
  674 358 LSB together, and under the excitatory night without replay on 11 184 to 11 722, 266 798 to 348 779 LSB together.
- **The pending loss** with a presynaptic spike at the first tick after the image is on all 21 248 held synapses, every one's target having
  fired since the night's first tick: 47 851 to 94 482 LSB together under the excitatory night, the largest 102 to 149 LSB; 46 563 to
  93 458 under both holds, and 56 594 to 91 163 under the excitatory night without replay. A later spike, before the target's next one,
  reads no more.
- **Against what the hold cleared.** These pairings were never in the trace the hold cleared, which held, synapse by synapse, the decayed
  net of what the trace carried into the night and the pairings read at the night's presynaptic spikes. Set beside it as sizes alone: the
  pending gains are 75.4 % to 102.9 % the size of the traces cleared above zero under the excitatory night, and 56.0 % to 75.0 % under the
  excitatory night without replay; the most the pending losses can be is 3.9 % to 10.1 % the size of the traces cleared below zero under
  the excitatory night, and 9.9 % to 16.9 % without replay.
- These are trace, not weight: at the next presynaptic spike, in the next day's engine, each synapse's trace takes its pairings, and its
  consolidation moves the weight under that day's modulation. That consolidation is not read here.

Exploration 024's hold under both holds, as 024's section 5 reads it:

| Cell | Nights | Held inhibitory synapses | Traces cleared, not zero | cleared above zero, LSB | below zero | Other inhibitory synapses moved | Nights a held synapse's presynaptic unit fired at or after the night's last tick | Nights held synapses moved in the quiet ticks | Pending pairings after the presynaptic spike | their sum, LSB | their largest | With a spike at the next tick, LSB |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 12 | 4 881 | 4 872 to 4 879 | 580 937 to 734 723 | −124 350 to −106 357 | 1 529 to 1 587 | 0 | none | 1 936 to 2 227 | 68 576 to 95 391 | 326 to 328 | 11 132 to 15 241 |
| Seed 24, eighth, assignment | 12 | 4 904 | 4 895 to 4 903 | 701 086 to 818 421 | −118 380 to −103 590 | 1 403 to 1 446 | 3 | 3: 15 to 25 synapses, −147 to −46 LSB | 2 064 to 2 233 | 83 848 to 111 793 | 327 to 328 | 11 508 to 15 565 |
| Seed 22, first mapping, mirrored | 12 | 4 863 | 4 844 to 4 863 | 571 918 to 677 296 | −127 652 to −114 728 | 1 543 to 1 600 | 0 | none | 1 894 to 2 110 | 62 327 to 81 281 | 327 to 328 | 12 920 to 17 280 |
| Seed 22, eighth, assignment | 12 | 4 863 | 4 853 to 4 862 | 642 926 to 784 991 | −117 267 to −107 065 | 1 406 to 1 500 | 2 | 2: 15 to 16 synapses, 210 to 222 LSB | 1 934 to 2 269 | 73 331 to 107 434 | 327 to 328 | 10 755 to 15 238 |

- **The quiet ticks moved 024's held synapses in 5 of the 48 nights**, 3 on seed 24's eighth image, by −147 to −46 LSB together, and 2 on
  seed 22's, by +210 to +222 LSB. Its pending pairings after the presynaptic spike are 62 327 to 111 793 LSB together, 10.3 % to 15.2 % the
  size of the positive trace it cleared, as 024 reads that share.

## 6. The ties

The tied trials of each cell's 240 reads of A and 240 of the pair, under each form:

| Cell | Without replay, A's and the pair's | H-47's without replay | Excitatory | Both holds | Excitatory without replay | H-47's plain on A's | 024's held |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 23, first mapping, mirrored | 792 and 293 | 792 and 293 | 826 and 360 | 767 and 365 | 791 and 363 | 866 and 365 | 838 and 386 |
| Seed 24, eighth, assignment | 1114 and 362 | 1114 and 362 | 1391 and 183 | 1392 and 189 | 1191 and 265 | 1324 and 174 | 1389 and 165 |
| Seed 22, first mapping, mirrored | 943 and 302 | 943 and 302 | 862 and 330 | 866 and 336 | 909 and 323 | 878 and 357 | 848 and 344 |
| Seed 22, eighth, assignment | 917 and 207 | 917 and 207 | 873 and 124 | 888 and 127 | 889 and 209 | 898 and 130 | 890 and 133 |

The night without replay's ties are H-47's, read by read. Under the excitatory night they lie 16 to 67 from H-47's plain night's on A's
reads and 5 to 27 on the pair's.

## What it costs

The trial took 420 s and the four jobs 7 441 to 7 704 s, at most 53.5 % of a job's 240 minutes. Exploration 024's jobs of three forms on the
same four cells took 5 796 to 6 057 s. The excitatory night without replay joined each job; nothing was cut. Five jobs ran, every one to
the end.

## What it suggests

**In these four cells A's count under the excitatory night lies within 2 SE of H-47's plain night's or above it at every checkpoint**,
above at 8 of the 20, on three of the four cells; on seed 22's first mapping it lies within 2 SE throughout. The held units' synapses, which
H-47's plain night moved by −19.72 to −23.68 M a path over four nights, did not move in the held nights. The excitatory night less the
night without replay at block 36 is −1.7 ± 0.42, +8.8 ± 1.28, +4.7 ± 1.52 and +4.6 ± 0.66, where H-47's clause 1 read −4.9 ± 1.09,
+1.8 ± 1.79, +5.0 ± 1.24 and +2.2 ± 0.65. The side is H-47's at 19 of the 20 checkpoints. Adding exploration 024's hold moved A's count by
−2.4 to +0.6 from the excitatory night's, beyond 2 SE at three checkpoints, all below.

**These runs do not part the hold's share from the rest.** Under the excitatory night the inhibition onto the readouts' excitatory units
rose by 0.73 to 1.52 M more than under the plain night, the excitatory night without replay moved A's count beyond 2 SE of the night
without replay at 8 of 20 checkpoints, and the hold leaves pending gains on about half the held synapses. The note does not read what the
next day does with them.

Questions this suggests for a later exploration or a round:
- The same hold on H-47's other eight cells, among them every cell where A answers readout 0.
- The held units' synapses held by target: onto the other readout's excitatory units, which carried the largest part of the plain night's
  change, apart from the rest.
- A read of what the next day's first consolidations do with the pending gains and losses.

## What did not run

- **H-47's other eight cells**, among them every cell where stimulus A answers readout 0.
- **Plain nights on both mappings and the awake gap**, under either hold or not: no reading here stands against them.
- **A hold through the quiet ticks**: the hold ends at the night's last tick, and the image is written after the quiet ticks. Here the quiet
  ticks moved no held excitatory synapse.
- **Both holds without replay.**
- **The excitatory synapses onto the readouts from A's, B's and the pair's own units**, which consolidate as in H-47 under every form, the
  zero sum's rounds with them, and **the inhibitory synapses onto the readouts' inhibitory units**, which neither hold reaches.
- **The pending pairings' effect on a weight**: the next day's consolidation of the held synapses is not read.
- **REM's depotentiations and the zero sum's rounds** are not split out of a night's change, as in explorations 020 to 024.
- **Any SE for sections 4 and 5**: the class matrix is a mean over the three paths, and the hold's readings are ranges.
- **Any other share, form, network or seed.**

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_the_synapses_from_the_readouts_excitatory_units_held_explore` | [42468595193](https://github.com/Menteion/Menteion/actions/runs/42468595193) | `2862685` | 420 |
| `the_synapses_from_the_readouts_excitatory_units_held_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [42469251250](https://github.com/Menteion/Menteion/actions/runs/42469251250) | `2862685` | 7704 |
| `the_synapses_from_the_readouts_excitatory_units_held_the_eighth_image_on_seed_24_from_the_assignment_explore` | [42469251250](https://github.com/Menteion/Menteion/actions/runs/42469251250) | `2862685` | 7617 |
| `the_synapses_from_the_readouts_excitatory_units_held_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [42469251250](https://github.com/Menteion/Menteion/actions/runs/42469251250) | `2862685` | 7458 |
| `the_synapses_from_the_readouts_excitatory_units_held_the_eighth_image_on_seed_22_from_the_assignment_explore` | [42469251250](https://github.com/Menteion/Menteion/actions/runs/42469251250) | `2862685` | 7441 |
