---
status: exploratory
date: 2026-11-09
---

# 027: The readouts' excitatory hold split by target

**Branch**: `explore/027-the-excitatory-hold-by-target`
**Commit**: `65b0c59`, which adds the harness, on `7108e32`, which adds the hold by target under `src/`, on `55f5387`: `main` at
`72aa816`, ADR-0242's merge, with exploration 026's tag merged in, which carries explorations 024's and 025's changes under `src/`.
**Runs**: [42512634982](https://github.com/Menteion/Menteion/actions/runs/42512634982) (the harness's trial) and
[42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) (twelve jobs, one a cell), both at `scope=arms`. H-47's
plain night on A's patterns is read from H-47's run [41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613), its class
matrix and night lines from exploration 020's runs [41500459562](https://github.com/Menteion/Menteion/actions/runs/41500459562) and
[41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777) for exploration 020's four cells and from exploration 021's run
[41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) for the other eight. The excitatory night, exploration 025's
whole hold, is read from exploration 025's run [42469251250](https://github.com/Menteion/Menteion/actions/runs/42469251250) for exploration
020's four cells and from exploration 026's run [42489793510](https://github.com/Menteion/Menteion/actions/runs/42489793510) for the other
eight. None of those was run again.

[ADR-0242](../adr/0242-the-readouts-excitatory-hold-split-by-target.md) ordered this exploration.
[Exploration 025](025-synapses-from-the-readouts-excitatory-units.md) held every synapse from the readouts' excitatory units, the pair's
own units excepted as sources, from a night's first tick to its last, and [exploration 026](026-the-excitatory-hold-on-eight-cells.md) ran
the same hold on H-47's other eight cells. Under that excitatory night, at block 36, A's count stood within 2 SE of H-47's plain night's or
above it in all six cells where A answers readout 1, and below it beyond 2 SE in three of the six where A answers readout 0. Under H-47's
plain night the held units' synapses lost most onto the other readout's excitatory units in every cell. This note splits the hold by target
into two parts and holds each alone through H-47's plain night on A's patterns, on all twelve of H-47's cells. It asks, at blocks 6, 12,
18, 24 and 36:

1. A's count and each stimulus's correct trials under each held form: less the night without replay and less the other held form, with
   H-47's paired SE, the forms run in one job; and less H-47's plain night and less the excitatory night, with
   [exploration 018](018-what-nights-keep.md)'s unpaired SE, the forms run in two jobs;
2. the side under each held form, by stimulus and by group of units, against H-47's side and the excitatory night's on the same cell;
3. the pair under each held form and A's highest coupling, as H-47's clauses 2 and 3 read them;
4. what each night moved, from the class matrix by group, with the held units' synapses broken down by target, the held part's change shown
   nil and the other part's read beside the plain night's, and the inhibition onto each group's excitatory units;
5. what the hold leaves: what the quiet ticks after each night move, and the pairings still pending on the held part's synapses, the gain
   and the loss apart;
6. the twelve cells side by side at block 36: each held form less the night without replay, with the job's paired SE, and less H-47's plain
   night and less the excitatory night, with exploration 018's unpaired SE.

**The side** is [ADR-0238](../adr/0238-what-keeps-a-mapping-where-the-replay-did-not.md)'s: at a checkpoint, the stimulus whose difference
from the night without replay is the larger of the two. The note writes no prediction and decides nothing. **Every figure here is
exploratory**: from one run of each test on the repository's AArch64 host, in H-47's terms. A count is a mean over a cell's 48 reads,
sixteen on each of three paths, and a difference carries its paired SE unless another is named.

**In brief**
- **Each hold held its part, and the night without replay ran H-47's.** In all 288 held nights every synapse of the held part ended the
  night at the weight it started with and its trace cleared, held inside the test, while synapses of the other part moved, 13 451 to 13 881
  of the part held within's under the night held across and 7 241 to 7 465 of the part held across's under the night held within. The
  quiet ticks after a night moved no synapse of either part (section 5). Every row of the night without replay, 15 a cell, is H-47's, held
  inside the test.
- **The two parts**: 7 345 to 7 476 synapses held across and 13 772 to 13 903 held within, by seed, disjoint and together exploration
  025's 21 248 in every cell, held inside the test synapse by synapse.
- **Under the night held across A's count less H-47's plain night's** lies between −2.6 and +3.5, beyond twice its unpaired SE at 5 of the
  60 checkpoints, all above. **Under the night held within** it lies between −4.1 and +8.4, beyond 2 SE at 17 of the 60, 13 above and 4
  below: above at 8 on seed 24's eighth image from the mirrored assignment and seed 23's eighth image from the assignment from block 12 on,
  +3.8 ± 1.55 to +8.4 ± 0.75; below on seed 22's eighth image from the mirrored assignment at blocks 24 and 36 and seed 23's eighth image
  from the mirrored assignment at blocks 18 and 24, −2.9 ± 0.82 to −4.1 ± 1.13.
- **Less the excitatory night** (explorations 025 and 026), the night held within lies beyond 2 SE at 4 of the 60 checkpoints, all below
  and all on seed 24's eighth image from the assignment from block 12 on, −2.3 ± 0.87 to −4.7 ± 1.54. The night held across lies beyond
  at 15, 3 above and 12 below; the 12 below are on seed 24's eighth image from the mirrored assignment, seed 23's and seed 24's eighth
  images from the assignment, seed 24's first mapping from the mirrored assignment and seed 22's eighth image from the assignment.
- **The night held across less the night held within**, run in one job, lies beyond twice its paired SE at 17 of the 60, 6 above and 11
  below, the 11 below on seed 24's eighth image from the mirrored assignment and seed 23's eighth image from the assignment from block 12
  on, seed 23's first mapping from the assignment at block 12 and seed 22's eighth image from the assignment at blocks 12 and 24.
- **Less the night without replay**, the night held across lies above beyond 2 SE at 44 of the 60 checkpoints and the night held within
  at 48, with one below, seed 23's first mapping from the mirrored assignment at block 36, −2.6 ± 0.65, where H-47's clause 1 read
  −4.9 ± 1.09.
- **The side under the night held within is the excitatory night's at 56 of the 60 checkpoints**, and at block 36 in all twelve cells;
  under the night held across it is H-47's at 56 (section 2).
- **Under the night held within the part held across moved as under H-47's plain night**, −4.80 to −6.84 M a group over four nights, 0.97
  to 1.03 times the plain night's. **Under the night held across the part held within moved by 1.04 to 1.20 times the plain night's**,
  −2.85 to −6.38 M a group. The inhibition onto each group's excitatory units rose by 0.33 to 1.11 M more than under the plain night under
  either, where the excitatory night's rose by 0.69 to 1.92 M more.
- **The night held across ran H-47's plain night's replays and depotentiations in all 144 of its nights; the night held within ran the
  excitatory night's in all 144**, which are the plain night's but on two nights of one path of seed 23's eighth image from the mirrored
  assignment. The watch read `wrong 0` in all 432 nights of the twelve jobs.
- **The pairings the hold leaves**: after the quiet ticks, 3 693 to 3 929 synapses of the part held across carry a pending gain,
  125 102 to 227 847 LSB together, and 6 876 to 7 797 of the part held within, 307 920 to 487 561 LSB, each read at the presynaptic unit's
  next spike if it comes before the target's next spike.
- **The pair** stands at 62.1 to 63.4 at block 36 under the night held across and 61.3 to 63.2 under the night held within, and A's highest
  coupling at 1.140 to 1.225 and 1.145 to 1.236 of H-42's.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 020 to 026 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. A cell is named, as in H-47, by its seed, its image and the arm
it starts from, not by the mapping in force. Nights follow blocks 6, 12, 18 and 24, then 12 blocks run with none to block 36, on paths 0, 5
and 6. Each checkpoint reads A and the pair sixteen times each, after the night that follows its block, and at block 36 after the block.

**Places.** A period of the ring is 20 units. Stimulus A presents place 0 of each period and B place 11. **The odd places** are the odd
places but 11 and **the even places** the even places but 0: H-47's readout 0 and readout 1, each with its excitatory and its inhibitory
units. C's own units lie at place 5, among the odd places, and D's at place 16, among the even, in every cell
([exploration 021](021-where-the-gain-falls.md), section 4).

**Twelve cells**, all of H-47's: exploration 026's eight and exploration 020's four. Under the assignment stimulus A answers readout 0, the
odd places, and B readout 1, the even places; under the mirrored mapping the other way. A answers readout 0 on the eighth images from the
mirrored assignment and on the first mappings from the assignment, as the table below shows. The tables list the six cells where A answers
readout 0 first, then the six where it answers readout 1, in exploration 026's section 6 order. The excitatory night's job is exploration
026's for its eight cells and exploration 025's for exploration 020's four:

| Cell | A answers | B answers | H-47: plain less without replay at block 36 (clause 1) | Excitatory less without replay at block 36 | Excitatory night's job |
| :--- | :--- | :--- | ---: | ---: | :--- |
| Seed 22, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +19.6 ± 1.06 | +16.3 ± 0.84 | 026 |
| Seed 22, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +7.9 ± 0.51 | +5.2 ± 1.24 | 026 |
| Seed 23, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +17.4 ± 1.08 | +14.7 ± 1.54 | 026 |
| Seed 23, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +13.4 ± 1.56 | +12.6 ± 1.56 | 026 |
| Seed 24, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +11.3 ± 0.96 | +16.7 ± 1.23 | 026 |
| Seed 24, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +3.8 ± 1.57 | +5.8 ± 1.60 | 026 |
| Seed 23, eighth, assignment | readout 1, the even places | readout 0, the odd places | +4.1 ± 1.45 | +10.9 ± 2.49 | 026 |
| Seed 24, first mapping, mirrored | readout 1, the even places | readout 0, the odd places | +6.0 ± 1.54 | +7.4 ± 1.09 | 026 |
| Seed 23, first mapping, mirrored | readout 1, the even places | readout 0, the odd places | −4.9 ± 1.09 | −1.7 ± 0.42 | 025 |
| Seed 24, eighth, assignment | readout 1, the even places | readout 0, the odd places | +1.8 ± 1.79 | +8.8 ± 1.28 | 025 |
| Seed 22, first mapping, mirrored | readout 1, the even places | readout 0, the odd places | +5.0 ± 1.24 | +4.7 ± 1.52 | 025 |
| Seed 22, eighth, assignment | readout 1, the even places | readout 0, the odd places | +2.2 ± 0.65 | +4.6 ± 0.66 | 025 |

**The hold by target** is a change under `src/` on this branch alone (`7108e32`), in no image, which `main` does not take.
`Executor::set_night_hold_by_target` takes a set of excitatory units, each with a group, as held sources; a set of targets, each with a
group; and a part. **The part held across** is each synapse from a held source onto a target of another group than its source's; **the part
held within** is each other synapse from a held source. While the hold is set and the engine is in slow-wave sleep or REM, the fan-out
consolidates every slot of a held source's block but those in the part, slot by slot as exploration 024's hold does it, so each synapse of
the part keeps its weight while its trace takes the night's pairings, and every other synapse, the other part's among them, consolidates as
in H-47. **The night's last tick** is the tick whose window steps the engine from sleep to awake; the hold is in force through it, and at
its end each synapse of the part has its trace cleared to zero, and no other synapse. The other part keeps its trace. The hold is in no
image, so the next day's engine, decoded from the night's image, holds nothing. Exploration 024's hold (`set_night_hold`) and exploration
025's (`set_night_hold_sources`) are on the branch too, and no form here sets either. A unit test on a small engine holds that the hold,
unset, set and unset, or refused, leaves a whole night bit for bit as it was, that the two parts set together are exploration 025's hold of
the same source bit for bit, and that each part alone leaves its slots at their first weights with their traces cleared while the other
part's slots move.

**The held units and their groups** (`e025_held`, `e027_hold`) are exploration 025's: every excitatory unit of the two readouts but the
pair's own, 664 in each cell, 332 a readout, as each job prints, none of them a source of the zero sum nor a unit of the six kinds, held
inside the test, so they are the class matrix's classes 6 to 9. Each held unit's group is its readout. **The targets with a group** are
every excitatory unit of the two readouts, the pair's own units among them, 714 in each cell, 357 a readout, each with its readout's group,
as explorations 025 and 026 split the held synapses by target. So the part held across is the odd places' held units' synapses onto the
even places' excitatory units, the class matrix's columns 5, 8 and 9, and the even places' held units' synapses onto the odd places'
excitatory units, columns 4, 6 and 7. The part held within is the held units' other synapses: onto their own group, the pair's own kind in
it among the targets, onto the inhibitory units, onto A's and B's units and onto the other excitatory units. Each held night holds, inside
the test, synapse by synapse, that the two parts are disjoint and together exactly the synapses exploration 025's hold names (`e027_parts`):

| Cell | Held units | of readout 0 and of readout 1 | Targets, the readouts' excitatory units | of readout 0 and of readout 1 | Held across: synapses | Held within: synapses | Exploration 025's hold: synapses | Excitatory synapses |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 664 | 332 and 332 | 714 | 357 and 357 | 7 345 | 13 903 | 21 248 | 26 240 |
| Seed 22, first mapping, assignment | 664 | 332 and 332 | 714 | 357 and 357 | 7 345 | 13 903 | 21 248 | 26 240 |
| Seed 23, eighth, mirrored | 664 | 332 and 332 | 714 | 357 and 357 | 7 476 | 13 772 | 21 248 | 26 240 |
| Seed 23, first mapping, assignment | 664 | 332 and 332 | 714 | 357 and 357 | 7 476 | 13 772 | 21 248 | 26 240 |
| Seed 24, eighth, mirrored | 664 | 332 and 332 | 714 | 357 and 357 | 7 453 | 13 795 | 21 248 | 26 240 |
| Seed 24, first mapping, assignment | 664 | 332 and 332 | 714 | 357 and 357 | 7 453 | 13 795 | 21 248 | 26 240 |
| Seed 23, eighth, assignment | 664 | 332 and 332 | 714 | 357 and 357 | 7 476 | 13 772 | 21 248 | 26 240 |
| Seed 24, first mapping, mirrored | 664 | 332 and 332 | 714 | 357 and 357 | 7 453 | 13 795 | 21 248 | 26 240 |
| Seed 23, first mapping, mirrored | 664 | 332 and 332 | 714 | 357 and 357 | 7 476 | 13 772 | 21 248 | 26 240 |
| Seed 24, eighth, assignment | 664 | 332 and 332 | 714 | 357 and 357 | 7 453 | 13 795 | 21 248 | 26 240 |
| Seed 22, first mapping, mirrored | 664 | 332 and 332 | 714 | 357 and 357 | 7 345 | 13 903 | 21 248 | 26 240 |
| Seed 22, eighth, assignment | 664 | 332 and 332 | 714 | 357 and 357 | 7 345 | 13 903 | 21 248 | 26 240 |

The synapses from A's, B's and the pair's own units consolidate as in H-47 under every form, the zero sum's rounds with them, and so does
every inhibitory synapse.

**Three forms a cell, in one job** (`e027_job`), exploration 026's job with its two held forms replaced
([ADR-0242](../adr/0242-the-readouts-excitatory-hold-split-by-target.md)):
- **the night without replay**, every row held inside the test to the row H-47 pinned;
- **the night held across**, H-47's plain night on A's patterns with the part held across set;
- **the night held within**, H-47's plain night on A's patterns with the part held within set.

The job is exploration 020's to 026's: A's patterns, the split and the class matrix of the start image, of the image before each night and
of the image after it, and the nights after blocks 6, 12, 18 and 24 watched by `ReplayWatch`.

**At each held night** (`e027_night`) the job reads every excitatory synapse three times: before the night's first tick, at its last, and
after the quiet ticks, the awake ticks without drive that H-47's harness runs until the engine is quiescent, before it writes the night's
image. At the last it holds, inside the test, that every synapse of the held part stands at its first weight with its trace cleared, that
some synapse of the other part moved, and that some excitatory synapse from a unit outside the hold moved; it prints how many of the other
part's synapses moved and how many carry a trace that is not zero. After the quiet ticks it prints what they moved, and what the held
part's synapses would read at their presynaptic unit's next spike from the spikes on record since the night's first tick, if that spike
came before the target's next one: for the gain and for the loss, how many synapses carry one, their sum and the largest.

**The excitatory rule** (`step_stdp_shaped`, in `crates/menteion-core/src/dynamics/synapse.rs`) reads two pairings at a presynaptic spike
at $t$, with $p$ the presynaptic unit's previous spike and $q$ the target's last spike: if $p < q \le t$, a gain of
$A_+ (1 - 2^{-11})^{q - p}$; and if $q < t$, a loss of $A_- (1 - 2^{-11})^{t - q}$ scaled by the slot's magnitude, by the exponent the
configuration sets ([ADR-0228](../adr/0228-a-zero-sum-at-the-stimulus-built.md)). Both are read only there, so the night's last pairings
escape the clearing. The job finds each with the rule itself, on a copy of the block, the slot's trace set to zero; with $t_0$ the first
tick after the image:
- **the pending gain**: a synapse of the held part whose target's last spike and presynaptic unit's last spike are both on record since the
  night's first tick, $q > p$: $A_+ (1 - 2^{-11})^{q - p}$, read at the presynaptic unit's next spike if it comes before the target's next
  spike;
- **the pending loss**: a synapse of the held part whose target fired since the night's first tick: the loss a presynaptic spike at $t_0$
  would read, at the synapse's magnitude after the quiet ticks. It falls as $t$ grows, so it is the most a later presynaptic spike reads, if
  that spike comes before the target's next one.

A target spike before the next presynaptic one takes $q$'s place, and both readings change with it.

**The class matrix** is exploration 020's, as 021 reads it by readout and 023 to 026 by group. Each group's excitatory units are its
readout's two classes and the pair's own kind inside it, C's own in the odd places and D's in the even. An inhibitory entry is a
magnitude, the weight negated, so a rise is more inhibition. The held units' synapses are the rows of classes 6 and 7, the odd places' held
units, and 8 and 9, the even places', onto every class; the part held across is their entries onto the other group's excitatory units, and
the part held within the rest of their entries.

## Before a job ran

ADR-0242 asks whether any form but the night without replay must run bit for bit as one already in the logs. **None must.** Each held form
holds one part and lets the other consolidate, and each part moves under a night that consolidates it: under H-47's plain night, over four
nights, the mean over the three paths, the part held across moved by −4.73 to −6.98 M a group and the part held within by −2.50 to
−6.05 M a group, in each of the twelve cells (exploration 020's and 021's jobs), and under the excitatory night neither moved at all
(exploration 025's and 026's jobs). So the night held across holds synapses the plain night moves and lets consolidate synapses the
excitatory night holds, and so does the night held within. The trial then printed each held form's row at block 6 on path 0 on seed 23's
first mapping from the assignment beside H-47's plain night's and beside exploration 026's trial's excitatory night's on the same cell:
neither is the same, as the trial prints. So all three forms ran.

## The runs are laid out as written

- **The trial** ran on path 0. On seed 23's first mapping from the assignment: the pair's head and the first night of H-47's plain night on
  A's patterns, the night without replay, the night held across and the night held within, all watched. With no hold set, the night without
  replay and the plain night held H-47's row at block 6 on path 0, inside the test. With each part's hold set, the night held, inside the
  test, every synapse of that part at its first weight with its trace cleared, a synapse of the other part moved and, at the night's last
  tick, a synapse of the other part with a trace that is not zero; and the two parts disjoint and together exactly exploration 025's 21 248
  held synapses, synapse by synapse: 7 476 held across and 13 772 held within. In the night held across 13 736 of the other part's 13 772
  synapses moved and 7 458 carried a trace at the night's last tick; in the night held within 7 457 of the other part's 7 476 moved and
  3 203 carried a trace. On seed 23's first mapping from the mirrored assignment: the pair's head and the first night of the plain night on
  A's patterns with both parts held together, slot by slot, which held the row exploration 025's trial printed at block 6 on path 0 (run
  42468595193), inside the test: its image's CRC-64, A's first count of 58, its couplings and the gain. It cleared what exploration 025's
  trial printed its hold clearing: 21 248 slots, 12 780 of them with a trace, 669 893 LSB above zero and −1 211 118 below.
- The two held forms' rows at block 6 on path 0 on seed 23's first mapping from the assignment are neither H-47's plain night's nor
  exploration 026's trial's excitatory night's, as the trial prints: each image's CRC-64, A's couplings and the gain differ. A's first count
  is 57 under each held form, where H-47's plain night read 57 and the excitatory night 56. The watch read `wrong 0` in the trial's five
  nights. Each of the four nights with replay read 1 339 replays and 640 depotentiations, and the night without replay none.
- **Each of the twelve jobs** ran to the end. Their reproduction, cell by cell:

| Cell | Without replay: rows H-47's | Held across: rows H-47's plain on A's | Held within: rows H-47's plain on A's | Held across and held within: rows the excitatory night's | Without replay: its paths' lines in the plain night's job, label aside | Heads: lines in that job | Plain night's lines: job | Held nights: the part's hold held inside the test |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | :--- | ---: |
| Seed 22, eighth, mirrored | 15 of 15 | 0 of 15 | 0 of 15 | 0 and 0 | 1 632 of 1 632 | 3 of 3 | 021 | 24 of 24 |
| Seed 22, first mapping, assignment | 15 of 15 | 0 of 15 | 0 of 15 | 0 and 0 | 1 632 of 1 632 | 3 of 3 | 021 | 24 of 24 |
| Seed 23, eighth, mirrored | 15 of 15 | 0 of 15 | 0 of 15 | 0 and 0 | 1 680 of 1 680 | 3 of 3 | 021 | 24 of 24 |
| Seed 23, first mapping, assignment | 15 of 15 | 0 of 15 | 0 of 15 | 0 and 0 | 1 680 of 1 680 | 3 of 3 | 021 | 24 of 24 |
| Seed 24, eighth, mirrored | 15 of 15 | 0 of 15 | 0 of 15 | 0 and 0 | 1 656 of 1 656 | 3 of 3 | 021 | 24 of 24 |
| Seed 24, first mapping, assignment | 15 of 15 | 0 of 15 | 0 of 15 | 0 and 0 | 1 656 of 1 656 | 3 of 3 | 021 | 24 of 24 |
| Seed 23, eighth, assignment | 15 of 15 | 0 of 15 | 0 of 15 | 0 and 0 | 1 680 of 1 680 | 3 of 3 | 021 | 24 of 24 |
| Seed 24, first mapping, mirrored | 15 of 15 | 0 of 15 | 0 of 15 | 0 and 0 | 1 656 of 1 656 | 3 of 3 | 021 | 24 of 24 |
| Seed 23, first mapping, mirrored | 15 of 15 | 0 of 15 | 0 of 15 | 0 and 0 | 1 680 of 1 680 | 3 of 3 | 020 | 24 of 24 |
| Seed 24, eighth, assignment | 15 of 15 | 0 of 15 | 0 of 15 | 0 and 0 | 1 656 of 1 656 | 3 of 3 | 020 | 24 of 24 |
| Seed 22, first mapping, mirrored | 15 of 15 | 0 of 15 | 0 of 15 | 0 and 0 | 1 632 of 1 632 | 3 of 3 | 020 | 24 of 24 |
| Seed 22, eighth, assignment | 15 of 15 | 0 of 15 | 0 of 15 | 0 and 0 | 1 632 of 1 632 | 3 of 3 | 020 | 24 of 24 |

- The night without replay's `DUMP` lines, its paths' 1 632 to 1 680 a cell, its 15 rows' and its notice, are the lines of H-47's plain
  night's class matrix job on the same cell, label aside, all of them in each cell: exploration 021's run for exploration 026's eight cells
  and exploration 020's runs for exploration 020's four. So are the pair's head's three. Each read of the night without replay carries
  H-47's read's `stimuli`, `right` and `tied` masks, all 480 a cell. Each held form's reads carry H-47's plain night's `stimuli` masks, so
  each trial presented the stimulus it presented in H-47.
- For each held form the job prints how many of its 15 rows are H-47's plain night's rows: **none, in all twelve cells**, as the job
  prints; nothing in the job asserts it. Set beside the excitatory night's rows in exploration 025's or 026's log, row by row, none of either
  held form's rows is the same either.
- **The watch read `wrong 0` in all 432 nights.** Every night without replay read `replays 0` and `depotentiations 0`. The night held
  across read H-47's plain night's replays and depotentiations, as the plain night's class matrix job printed them, in all 144 of its
  nights. The night held within read them in 142 of its 144. The two others are on seed 23's eighth image from the mirrored assignment,
  path 0: the night after block 18 read 1 202 replays and 571 depotentiations, where the plain night read 1 202 and 570, and the night
  after block 24 read 1 129 replays and 545 depotentiations, where the plain night read 1 128 and 545. Those are the excitatory night's
  counts on those two nights (exploration 026), and the night held within read the excitatory night's in all 144 of its nights. The night
  held across read the excitatory night's in 142 of its 144, all but those two.
- Every held unit's synapse was read at each night: 664 held units, 332 a readout, C's own 25 and D's own 25 left out, as each job prints.
  Each job holds that none of the zero sum's 152 sources is among them. Each held night held its part's hold inside the test, 24 a cell.
## 1. A's count under each held night

A's count at each checkpoint under each form. H-47's plain night on A's patterns is H-47's job's, and the excitatory night is exploration
025's or 026's job's:

| Cell | Block | Without replay | Held across | Held within | Plain on A's (H-47) | Excitatory (025, 026) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 46.3 | 52.9 | 52.6 | 53.3 | 52.4 |
| Seed 22, eighth, mirrored | 12 | 30.2 | 42.5 | 40.9 | 42.3 | 41.0 |
| Seed 22, eighth, mirrored | 18 | 26.3 | 44.1 | 40.5 | 41.8 | 40.9 |
| Seed 22, eighth, mirrored | 24 | 22.6 | 41.4 | 37.0 | 40.2 | 37.1 |
| Seed 22, eighth, mirrored | 36 | 16.6 | 36.1 | 33.3 | 36.2 | 32.8 |
| Seed 22, first mapping, assignment | 6 | 52.4 | 57.0 | 56.7 | 56.5 | 56.8 |
| Seed 22, first mapping, assignment | 12 | 44.8 | 48.3 | 47.3 | 47.6 | 45.8 |
| Seed 22, first mapping, assignment | 18 | 40.2 | 47.1 | 46.6 | 46.9 | 46.2 |
| Seed 22, first mapping, assignment | 24 | 38.9 | 47.8 | 47.3 | 48.0 | 45.6 |
| Seed 22, first mapping, assignment | 36 | 36.3 | 44.0 | 42.8 | 44.3 | 41.5 |
| Seed 23, eighth, mirrored | 6 | 34.4 | 35.5 | 35.4 | 36.1 | 36.8 |
| Seed 23, eighth, mirrored | 12 | 19.5 | 26.7 | 26.3 | 28.5 | 25.5 |
| Seed 23, eighth, mirrored | 18 | 14.9 | 24.9 | 23.4 | 27.6 | 24.0 |
| Seed 23, eighth, mirrored | 24 | 10.6 | 27.2 | 26.0 | 29.3 | 25.2 |
| Seed 23, eighth, mirrored | 36 | 8.2 | 24.7 | 23.7 | 25.6 | 22.9 |
| Seed 23, first mapping, assignment | 6 | 54.2 | 56.2 | 56.3 | 56.3 | 56.1 |
| Seed 23, first mapping, assignment | 12 | 33.4 | 35.7 | 39.0 | 37.6 | 37.5 |
| Seed 23, first mapping, assignment | 18 | 27.2 | 34.6 | 34.8 | 35.0 | 35.1 |
| Seed 23, first mapping, assignment | 24 | 24.8 | 34.8 | 35.8 | 34.8 | 35.0 |
| Seed 23, first mapping, assignment | 36 | 19.7 | 32.9 | 33.0 | 33.0 | 32.3 |
| Seed 24, eighth, mirrored | 6 | 36.4 | 34.7 | 35.1 | 33.8 | 36.6 |
| Seed 24, eighth, mirrored | 12 | 31.5 | 30.1 | 33.8 | 28.6 | 33.6 |
| Seed 24, eighth, mirrored | 18 | 20.2 | 31.0 | 35.2 | 28.6 | 34.4 |
| Seed 24, eighth, mirrored | 24 | 16.4 | 29.3 | 32.9 | 25.8 | 33.9 |
| Seed 24, eighth, mirrored | 36 | 12.5 | 25.9 | 28.1 | 23.8 | 29.2 |
| Seed 24, first mapping, assignment | 6 | 58.3 | 61.1 | 61.6 | 61.2 | 61.1 |
| Seed 24, first mapping, assignment | 12 | 52.0 | 55.7 | 57.0 | 54.8 | 55.7 |
| Seed 24, first mapping, assignment | 18 | 47.5 | 52.3 | 52.4 | 51.3 | 52.5 |
| Seed 24, first mapping, assignment | 24 | 43.7 | 53.3 | 53.7 | 52.4 | 52.7 |
| Seed 24, first mapping, assignment | 36 | 40.1 | 47.0 | 46.2 | 43.8 | 45.8 |
| Seed 23, eighth, assignment | 6 | 47.4 | 50.5 | 50.4 | 50.4 | 50.7 |
| Seed 23, eighth, assignment | 12 | 40.8 | 44.3 | 47.2 | 43.4 | 47.5 |
| Seed 23, eighth, assignment | 18 | 37.3 | 46.2 | 50.0 | 45.2 | 50.1 |
| Seed 23, eighth, assignment | 24 | 38.0 | 45.9 | 52.1 | 43.7 | 51.0 |
| Seed 23, eighth, assignment | 36 | 36.2 | 42.3 | 46.4 | 40.3 | 47.1 |
| Seed 24, first mapping, mirrored | 6 | 54.3 | 57.8 | 57.2 | 57.7 | 57.9 |
| Seed 24, first mapping, mirrored | 12 | 46.3 | 50.6 | 50.7 | 51.0 | 52.3 |
| Seed 24, first mapping, mirrored | 18 | 44.0 | 51.5 | 51.5 | 51.8 | 53.3 |
| Seed 24, first mapping, mirrored | 24 | 44.4 | 51.9 | 53.3 | 52.0 | 53.7 |
| Seed 24, first mapping, mirrored | 36 | 42.7 | 49.8 | 50.4 | 48.7 | 50.1 |
| Seed 23, first mapping, mirrored | 6 | 55.0 | 56.4 | 56.6 | 56.9 | 56.6 |
| Seed 23, first mapping, mirrored | 12 | 52.4 | 55.3 | 54.8 | 54.4 | 55.5 |
| Seed 23, first mapping, mirrored | 18 | 52.0 | 52.6 | 53.8 | 52.4 | 54.2 |
| Seed 23, first mapping, mirrored | 24 | 53.5 | 52.9 | 52.6 | 50.9 | 53.3 |
| Seed 23, first mapping, mirrored | 36 | 53.1 | 50.5 | 50.5 | 48.3 | 51.4 |
| Seed 24, eighth, assignment | 6 | 38.8 | 39.6 | 38.3 | 39.1 | 39.3 |
| Seed 24, eighth, assignment | 12 | 34.8 | 35.3 | 34.5 | 34.2 | 36.8 |
| Seed 24, eighth, assignment | 18 | 34.4 | 31.8 | 31.6 | 28.5 | 35.6 |
| Seed 24, eighth, assignment | 24 | 35.6 | 30.9 | 32.4 | 29.6 | 36.7 |
| Seed 24, eighth, assignment | 36 | 25.6 | 28.3 | 29.7 | 27.4 | 34.4 |
| Seed 22, first mapping, mirrored | 6 | 56.6 | 57.9 | 58.2 | 57.7 | 58.1 |
| Seed 22, first mapping, mirrored | 12 | 49.5 | 52.3 | 53.0 | 52.5 | 52.7 |
| Seed 22, first mapping, mirrored | 18 | 49.0 | 52.4 | 53.2 | 52.7 | 52.3 |
| Seed 22, first mapping, mirrored | 24 | 45.9 | 50.8 | 51.8 | 50.6 | 50.7 |
| Seed 22, first mapping, mirrored | 36 | 44.1 | 48.7 | 49.7 | 49.1 | 48.8 |
| Seed 22, eighth, assignment | 6 | 50.3 | 52.3 | 52.6 | 52.3 | 52.6 |
| Seed 22, eighth, assignment | 12 | 47.6 | 51.1 | 52.9 | 50.6 | 52.8 |
| Seed 22, eighth, assignment | 18 | 47.2 | 51.4 | 51.8 | 50.6 | 51.8 |
| Seed 22, eighth, assignment | 24 | 47.3 | 50.4 | 52.6 | 50.2 | 52.1 |
| Seed 22, eighth, assignment | 36 | 44.2 | 47.4 | 48.7 | 46.4 | 48.8 |

The differences between the forms run in one job, each with the job's paired SE. The night without replay is H-47's, read by read, so
"H-47: plain less without replay" at block 36 is H-47's clause 1:

| Cell | Block | Held across less without replay | Held within less without replay | Held across less held within | H-47: plain less without replay | Excitatory less without replay (025, 026) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +6.6 ± 0.89 | +6.3 ± 0.87 | +0.3 ± 0.28 | +7.1 ± 0.91 | +6.1 ± 0.98 |
| Seed 22, eighth, mirrored | 12 | +12.3 ± 0.87 | +10.7 ± 1.17 | +1.6 ± 0.39 | +12.1 ± 0.85 | +10.8 ± 0.98 |
| Seed 22, eighth, mirrored | 18 | +17.9 ± 1.64 | +14.3 ± 1.80 | +3.6 ± 0.76 | +15.5 ± 1.67 | +14.6 ± 1.78 |
| Seed 22, eighth, mirrored | 24 | +18.8 ± 1.20 | +14.4 ± 1.75 | +4.4 ± 1.12 | +17.6 ± 1.49 | +14.5 ± 1.46 |
| Seed 22, eighth, mirrored | 36 | +19.5 ± 1.24 | +16.8 ± 0.83 | +2.8 ± 1.34 | +19.6 ± 1.06 | +16.3 ± 0.84 |
| Seed 22, first mapping, assignment | 6 | +4.6 ± 1.10 | +4.3 ± 1.22 | +0.3 ± 0.81 | +4.1 ± 0.82 | +4.4 ± 1.10 |
| Seed 22, first mapping, assignment | 12 | +3.5 ± 0.77 | +2.6 ± 0.74 | +1.0 ± 1.09 | +2.9 ± 0.56 | +1.0 ± 1.21 |
| Seed 22, first mapping, assignment | 18 | +6.9 ± 0.93 | +6.4 ± 1.05 | +0.5 ± 0.56 | +6.8 ± 1.12 | +6.1 ± 1.11 |
| Seed 22, first mapping, assignment | 24 | +8.9 ± 1.08 | +8.4 ± 1.05 | +0.6 ± 0.79 | +9.1 ± 1.37 | +6.7 ± 1.33 |
| Seed 22, first mapping, assignment | 36 | +7.7 ± 1.07 | +6.5 ± 1.42 | +1.2 ± 1.82 | +7.9 ± 0.51 | +5.2 ± 1.24 |
| Seed 23, eighth, mirrored | 6 | +1.1 ± 1.32 | +1.0 ± 1.17 | +0.1 ± 0.40 | +1.7 ± 1.20 | +2.4 ± 1.08 |
| Seed 23, eighth, mirrored | 12 | +7.2 ± 0.70 | +6.8 ± 0.77 | +0.4 ± 0.80 | +9.0 ± 1.07 | +6.0 ± 0.97 |
| Seed 23, eighth, mirrored | 18 | +10.1 ± 1.49 | +8.6 ± 1.29 | +1.5 ± 0.85 | +12.7 ± 1.02 | +9.1 ± 1.46 |
| Seed 23, eighth, mirrored | 24 | +16.6 ± 2.05 | +15.4 ± 1.64 | +1.2 ± 0.71 | +18.7 ± 1.62 | +14.6 ± 1.96 |
| Seed 23, eighth, mirrored | 36 | +16.5 ± 1.42 | +15.5 ± 1.06 | +1.0 ± 0.37 | +17.4 ± 1.08 | +14.7 ± 1.54 |
| Seed 23, first mapping, assignment | 6 | +2.0 ± 0.77 | +2.1 ± 0.89 | −0.1 ± 0.30 | +2.0 ± 0.79 | +1.9 ± 0.87 |
| Seed 23, first mapping, assignment | 12 | +2.3 ± 1.40 | +5.5 ± 1.09 | −3.3 ± 1.10 | +4.1 ± 1.18 | +4.1 ± 0.95 |
| Seed 23, first mapping, assignment | 18 | +7.4 ± 2.29 | +7.6 ± 2.55 | −0.2 ± 0.52 | +7.9 ± 2.54 | +7.9 ± 2.40 |
| Seed 23, first mapping, assignment | 24 | +10.0 ± 2.30 | +11.0 ± 1.93 | −1.0 ± 0.61 | +10.0 ± 1.90 | +10.3 ± 2.20 |
| Seed 23, first mapping, assignment | 36 | +13.2 ± 1.51 | +13.3 ± 1.64 | −0.1 ± 0.32 | +13.4 ± 1.56 | +12.6 ± 1.56 |
| Seed 24, eighth, mirrored | 6 | −1.7 ± 1.21 | −1.3 ± 1.35 | −0.4 ± 0.34 | −2.6 ± 1.35 | +0.3 ± 1.15 |
| Seed 24, eighth, mirrored | 12 | −1.4 ± 1.12 | +2.3 ± 1.32 | −3.7 ± 0.66 | −2.9 ± 1.19 | +2.1 ± 1.50 |
| Seed 24, eighth, mirrored | 18 | +10.8 ± 1.82 | +15.0 ± 1.31 | −4.2 ± 1.26 | +8.3 ± 1.70 | +14.2 ± 1.34 |
| Seed 24, eighth, mirrored | 24 | +12.9 ± 1.94 | +16.5 ± 1.98 | −3.6 ± 1.45 | +9.4 ± 1.58 | +17.5 ± 1.30 |
| Seed 24, eighth, mirrored | 36 | +13.4 ± 1.11 | +15.5 ± 1.27 | −2.2 ± 0.63 | +11.3 ± 0.96 | +16.7 ± 1.23 |
| Seed 24, first mapping, assignment | 6 | +2.8 ± 0.66 | +3.4 ± 0.71 | −0.5 ± 0.38 | +2.9 ± 0.72 | +2.8 ± 0.68 |
| Seed 24, first mapping, assignment | 12 | +3.7 ± 1.07 | +5.0 ± 1.06 | −1.3 ± 0.72 | +2.8 ± 0.93 | +3.7 ± 1.16 |
| Seed 24, first mapping, assignment | 18 | +4.8 ± 1.46 | +5.0 ± 1.01 | −0.1 ± 1.13 | +3.9 ± 1.41 | +5.0 ± 1.19 |
| Seed 24, first mapping, assignment | 24 | +9.6 ± 2.22 | +10.0 ± 2.04 | −0.4 ± 0.52 | +8.7 ± 3.17 | +9.0 ± 2.46 |
| Seed 24, first mapping, assignment | 36 | +7.0 ± 0.93 | +6.1 ± 1.46 | +0.8 ± 1.10 | +3.8 ± 1.57 | +5.8 ± 1.60 |
| Seed 23, eighth, assignment | 6 | +3.2 ± 0.86 | +3.0 ± 1.00 | +0.1 ± 0.21 | +3.0 ± 0.96 | +3.4 ± 1.07 |
| Seed 23, eighth, assignment | 12 | +3.5 ± 1.22 | +6.4 ± 1.53 | −2.9 ± 0.81 | +2.6 ± 0.94 | +6.7 ± 1.57 |
| Seed 23, eighth, assignment | 18 | +8.9 ± 1.23 | +12.7 ± 0.97 | −3.8 ± 0.64 | +7.9 ± 0.90 | +12.7 ± 1.45 |
| Seed 23, eighth, assignment | 24 | +8.0 ± 2.30 | +14.1 ± 1.57 | −6.1 ± 1.37 | +5.7 ± 1.34 | +13.0 ± 2.14 |
| Seed 23, eighth, assignment | 36 | +6.1 ± 2.39 | +10.2 ± 1.77 | −4.1 ± 0.78 | +4.1 ± 1.45 | +10.9 ± 2.49 |
| Seed 24, first mapping, mirrored | 6 | +3.5 ± 0.83 | +2.9 ± 0.83 | +0.6 ± 0.33 | +3.4 ± 0.79 | +3.6 ± 0.88 |
| Seed 24, first mapping, mirrored | 12 | +4.3 ± 1.38 | +4.4 ± 1.70 | −0.1 ± 0.78 | +4.7 ± 1.50 | +6.0 ± 1.30 |
| Seed 24, first mapping, mirrored | 18 | +7.5 ± 1.08 | +7.5 ± 0.84 | −0.1 ± 0.65 | +7.8 ± 0.89 | +9.3 ± 0.89 |
| Seed 24, first mapping, mirrored | 24 | +7.6 ± 1.73 | +8.9 ± 1.12 | −1.3 ± 0.98 | +7.6 ± 1.32 | +9.3 ± 1.35 |
| Seed 24, first mapping, mirrored | 36 | +7.1 ± 1.35 | +7.7 ± 1.37 | −0.6 ± 1.06 | +6.0 ± 1.54 | +7.4 ± 1.09 |
| Seed 23, first mapping, mirrored | 6 | +1.4 ± 0.69 | +1.6 ± 0.76 | −0.2 ± 0.21 | +1.9 ± 0.68 | +1.5 ± 0.76 |
| Seed 23, first mapping, mirrored | 12 | +3.0 ± 1.07 | +2.4 ± 0.97 | +0.5 ± 0.48 | +2.0 ± 1.26 | +3.1 ± 1.21 |
| Seed 23, first mapping, mirrored | 18 | +0.6 ± 1.57 | +1.8 ± 1.04 | −1.2 ± 0.70 | +0.3 ± 1.17 | +2.2 ± 1.23 |
| Seed 23, first mapping, mirrored | 24 | −0.6 ± 1.22 | −0.9 ± 0.64 | +0.3 ± 1.12 | −2.6 ± 1.30 | −0.1 ± 0.80 |
| Seed 23, first mapping, mirrored | 36 | −2.7 ± 1.61 | −2.6 ± 0.65 | 0.0 ± 1.42 | −4.9 ± 1.09 | −1.7 ± 0.42 |
| Seed 24, eighth, assignment | 6 | +0.9 ± 0.79 | −0.5 ± 0.77 | +1.3 ± 0.43 | +0.3 ± 0.64 | +0.5 ± 0.67 |
| Seed 24, eighth, assignment | 12 | +0.4 ± 1.29 | −0.3 ± 1.17 | +0.8 ± 0.95 | −0.6 ± 1.03 | +2.0 ± 0.89 |
| Seed 24, eighth, assignment | 18 | −2.6 ± 2.44 | −2.9 ± 2.14 | +0.2 ± 0.72 | −5.9 ± 1.53 | +1.2 ± 2.06 |
| Seed 24, eighth, assignment | 24 | −4.6 ± 2.41 | −3.1 ± 2.59 | −1.5 ± 1.67 | −5.9 ± 1.85 | +1.1 ± 1.88 |
| Seed 24, eighth, assignment | 36 | +2.8 ± 2.44 | +4.1 ± 1.17 | −1.3 ± 1.89 | +1.8 ± 1.79 | +8.8 ± 1.28 |
| Seed 22, first mapping, mirrored | 6 | +1.3 ± 0.64 | +1.6 ± 0.52 | −0.3 ± 0.22 | +1.1 ± 0.63 | +1.5 ± 0.59 |
| Seed 22, first mapping, mirrored | 12 | +2.8 ± 1.98 | +3.5 ± 1.96 | −0.7 ± 0.38 | +2.9 ± 1.93 | +3.2 ± 1.36 |
| Seed 22, first mapping, mirrored | 18 | +3.4 ± 2.73 | +4.3 ± 2.29 | −0.8 ± 0.93 | +3.7 ± 2.22 | +3.4 ± 2.15 |
| Seed 22, first mapping, mirrored | 24 | +4.9 ± 2.36 | +5.9 ± 2.04 | −0.9 ± 0.56 | +4.7 ± 2.24 | +4.8 ± 1.82 |
| Seed 22, first mapping, mirrored | 36 | +4.6 ± 1.99 | +5.6 ± 1.68 | −1.0 ± 0.73 | +5.0 ± 1.24 | +4.7 ± 1.52 |
| Seed 22, eighth, assignment | 6 | +2.0 ± 0.49 | +2.3 ± 0.98 | −0.4 ± 0.84 | +1.9 ± 0.65 | +2.3 ± 0.81 |
| Seed 22, eighth, assignment | 12 | +3.5 ± 0.75 | +5.3 ± 0.82 | −1.8 ± 0.27 | +3.0 ± 0.93 | +5.2 ± 0.79 |
| Seed 22, eighth, assignment | 18 | +4.3 ± 0.79 | +4.6 ± 0.80 | −0.4 ± 0.66 | +3.4 ± 1.05 | +4.6 ± 1.28 |
| Seed 22, eighth, assignment | 24 | +3.1 ± 1.11 | +5.2 ± 1.18 | −2.1 ± 0.74 | +2.8 ± 0.89 | +4.8 ± 1.01 |
| Seed 22, eighth, assignment | 36 | +3.2 ± 0.95 | +4.5 ± 1.26 | −1.3 ± 1.47 | +2.2 ± 0.65 | +4.6 ± 0.66 |

- **The night held across less the night without replay** lies between −4.6 and +19.5, beyond twice its SE at 44 of the 60 checkpoints,
  all above: at 26 of the 30 where A answers readout 0 and at 18 of the 30 where it answers readout 1. It lies within 2 SE at every
  checkpoint on seed 24's eighth image from the assignment and at all but block 12 on seed 23's first mapping from the mirrored assignment.
- **The night held within less the night without replay** lies between −3.1 and +16.8, beyond twice its SE at 49 of the 60: 48 above and
  one below, seed 23's first mapping from the mirrored assignment at block 36, −2.6 ± 0.65. H-47's plain night lies beyond at 49 of the
  60, 45 above and 4 below, and the excitatory night at 51, 50 above and one below.
- **The night held across less the night held within** lies between −6.1 and +4.4, beyond twice its SE at 17 of the 60 checkpoints, 6
  above and 11 below. Above: on seed 22's eighth image from the mirrored assignment from block 12 on, +1.6 ± 0.39 to +4.4 ± 1.12; on seed
  23's eighth image from the mirrored assignment at block 36, +1.0 ± 0.37; and on seed 24's eighth image from the assignment at block 6,
  +1.3 ± 0.43. Below: on seed 24's eighth image from the mirrored assignment from block 12 on, −2.2 ± 0.63 to −4.2 ± 1.26; on seed 23's
  eighth image from the assignment from block 12 on, −2.9 ± 0.81 to −6.1 ± 1.37; on seed 23's first mapping from the assignment at block
  12, −3.3 ± 1.10; and on seed 22's eighth image from the assignment at blocks 12 and 24, −1.8 ± 0.27 and −2.1 ± 0.74.

Each held form less the nights run in other jobs, with [exploration 018](018-what-nights-keep.md)'s unpaired SE:

| Cell | Block | Held across less H-47's plain | Held within less H-47's plain | Held across less excitatory | Held within less excitatory |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | −0.5 ± 0.73 | −0.8 ± 0.61 | +0.5 ± 0.80 | +0.2 ± 0.70 |
| Seed 22, eighth, mirrored | 12 | +0.2 ± 0.88 | −1.4 ± 1.03 | +1.5 ± 1.01 | −0.1 ± 1.19 |
| Seed 22, eighth, mirrored | 18 | +2.3 ± 0.83 | −1.3 ± 0.77 | +3.2 ± 1.06 | −0.4 ± 0.94 |
| Seed 22, eighth, mirrored | 24 | +1.2 ± 1.06 | −3.2 ± 1.31 | +4.3 ± 1.71 | −0.1 ± 1.68 |
| Seed 22, eighth, mirrored | 36 | −0.1 ± 1.18 | −2.9 ± 0.82 | +3.3 ± 1.87 | +0.5 ± 1.44 |
| Seed 22, first mapping, assignment | 6 | +0.5 ± 0.60 | +0.2 ± 0.89 | +0.3 ± 0.87 | −0.1 ± 0.48 |
| Seed 22, first mapping, assignment | 12 | +0.7 ± 1.15 | −0.3 ± 1.34 | +2.5 ± 1.39 | +1.5 ± 1.34 |
| Seed 22, first mapping, assignment | 18 | +0.1 ± 1.16 | −0.3 ± 1.19 | +0.9 ± 1.29 | +0.4 ± 1.51 |
| Seed 22, first mapping, assignment | 24 | −0.2 ± 1.06 | −0.8 ± 1.57 | +2.2 ± 1.06 | +1.6 ± 1.32 |
| Seed 22, first mapping, assignment | 36 | −0.2 ± 0.64 | −1.4 ± 1.49 | +2.5 ± 1.33 | +1.3 ± 1.13 |
| Seed 23, eighth, mirrored | 6 | −0.6 ± 1.26 | −0.7 ± 1.17 | −1.3 ± 1.09 | −1.4 ± 1.08 |
| Seed 23, eighth, mirrored | 12 | −1.8 ± 1.07 | −2.2 ± 1.59 | +1.2 ± 1.26 | +0.8 ± 1.38 |
| Seed 23, eighth, mirrored | 18 | −2.6 ± 1.41 | −4.1 ± 1.13 | +1.0 ± 1.94 | −0.5 ± 1.54 |
| Seed 23, eighth, mirrored | 24 | −2.1 ± 1.08 | −3.3 ± 0.89 | +1.9 ± 1.24 | +0.8 ± 1.16 |
| Seed 23, eighth, mirrored | 36 | −0.9 ± 1.19 | −1.9 ± 0.94 | +1.8 ± 1.16 | +0.8 ± 1.01 |
| Seed 23, first mapping, assignment | 6 | −0.1 ± 0.74 | 0.0 ± 0.84 | +0.1 ± 0.80 | +0.2 ± 0.87 |
| Seed 23, first mapping, assignment | 12 | −1.9 ± 1.24 | +1.4 ± 0.97 | −1.8 ± 1.25 | +1.5 ± 0.98 |
| Seed 23, first mapping, assignment | 18 | −0.4 ± 1.08 | −0.2 ± 1.12 | −0.5 ± 1.25 | −0.3 ± 1.08 |
| Seed 23, first mapping, assignment | 24 | 0.0 ± 1.18 | +1.0 ± 1.08 | −0.3 ± 1.14 | +0.7 ± 1.18 |
| Seed 23, first mapping, assignment | 36 | −0.1 ± 0.87 | −0.1 ± 0.78 | +0.6 ± 0.98 | +0.7 ± 0.98 |
| Seed 24, eighth, mirrored | 6 | +0.9 ± 0.92 | +1.4 ± 0.87 | −1.9 ± 0.81 | −1.5 ± 0.86 |
| Seed 24, eighth, mirrored | 12 | +1.5 ± 1.22 | +5.2 ± 1.52 | −3.5 ± 1.39 | +0.2 ± 1.64 |
| Seed 24, eighth, mirrored | 18 | +2.5 ± 1.76 | +6.7 ± 1.58 | −3.4 ± 1.78 | +0.8 ± 1.60 |
| Seed 24, eighth, mirrored | 24 | +3.5 ± 1.68 | +7.1 ± 1.72 | −4.6 ± 1.95 | −1.0 ± 1.87 |
| Seed 24, eighth, mirrored | 36 | +2.1 ± 1.36 | +4.3 ± 1.55 | −3.3 ± 0.80 | −1.1 ± 0.93 |
| Seed 24, first mapping, assignment | 6 | −0.1 ± 0.50 | +0.4 ± 0.42 | 0.0 ± 0.46 | +0.5 ± 0.44 |
| Seed 24, first mapping, assignment | 12 | +0.9 ± 0.98 | +2.2 ± 0.93 | 0.0 ± 0.99 | +1.3 ± 0.80 |
| Seed 24, first mapping, assignment | 18 | +1.0 ± 1.41 | +1.1 ± 1.07 | −0.2 ± 1.21 | −0.1 ± 0.87 |
| Seed 24, first mapping, assignment | 24 | +0.9 ± 1.59 | +1.3 ± 1.65 | +0.6 ± 1.35 | +1.0 ± 1.40 |
| Seed 24, first mapping, assignment | 36 | +3.2 ± 1.12 | +2.4 ± 1.23 | +1.2 ± 1.18 | +0.4 ± 0.90 |
| Seed 23, eighth, assignment | 6 | +0.2 ± 1.03 | 0.0 ± 1.05 | −0.2 ± 1.01 | −0.4 ± 1.04 |
| Seed 23, eighth, assignment | 12 | +0.9 ± 1.26 | +3.8 ± 1.55 | −3.2 ± 0.86 | −0.3 ± 0.93 |
| Seed 23, eighth, assignment | 18 | +1.0 ± 1.56 | +4.8 ± 1.22 | −3.8 ± 1.05 | 0.0 ± 1.15 |
| Seed 23, eighth, assignment | 24 | +2.2 ± 1.59 | +8.4 ± 0.75 | −5.0 ± 0.97 | +1.1 ± 1.15 |
| Seed 23, eighth, assignment | 36 | +2.0 ± 1.42 | +6.1 ± 1.11 | −4.9 ± 1.13 | −0.8 ± 1.16 |
| Seed 24, first mapping, mirrored | 6 | +0.1 ± 0.84 | −0.5 ± 0.86 | −0.2 ± 0.83 | −0.8 ± 0.87 |
| Seed 24, first mapping, mirrored | 12 | −0.4 ± 1.10 | −0.3 ± 1.08 | −1.7 ± 1.12 | −1.6 ± 1.03 |
| Seed 24, first mapping, mirrored | 18 | −0.4 ± 0.98 | −0.3 ± 1.04 | −1.8 ± 0.90 | −1.8 ± 0.98 |
| Seed 24, first mapping, mirrored | 24 | −0.1 ± 0.94 | +1.3 ± 0.72 | −1.8 ± 1.02 | −0.4 ± 0.78 |
| Seed 24, first mapping, mirrored | 36 | +1.1 ± 0.91 | +1.7 ± 0.84 | −0.4 ± 0.87 | +0.3 ± 0.70 |
| Seed 23, first mapping, mirrored | 6 | −0.5 ± 0.68 | −0.3 ± 0.59 | −0.2 ± 0.67 | 0.0 ± 0.60 |
| Seed 23, first mapping, mirrored | 12 | +0.9 ± 1.15 | +0.4 ± 1.11 | −0.2 ± 1.04 | −0.7 ± 1.01 |
| Seed 23, first mapping, mirrored | 18 | +0.2 ± 0.88 | +1.4 ± 0.75 | −1.6 ± 1.00 | −0.4 ± 0.88 |
| Seed 23, first mapping, mirrored | 24 | +2.0 ± 1.37 | +1.7 ± 1.62 | −0.4 ± 1.26 | −0.7 ± 1.12 |
| Seed 23, first mapping, mirrored | 36 | +2.2 ± 0.91 | +2.3 ± 1.21 | −1.0 ± 1.49 | −0.9 ± 0.79 |
| Seed 24, eighth, assignment | 6 | +0.6 ± 1.03 | −0.8 ± 1.03 | +0.4 ± 1.15 | −0.9 ± 1.14 |
| Seed 24, eighth, assignment | 12 | +1.0 ± 1.53 | +0.3 ± 1.01 | −1.6 ± 1.06 | −2.3 ± 0.87 |
| Seed 24, eighth, assignment | 18 | +3.3 ± 1.59 | +3.1 ± 1.37 | −3.8 ± 1.99 | −4.0 ± 1.52 |
| Seed 24, eighth, assignment | 24 | +1.3 ± 1.59 | +2.8 ± 1.85 | −5.8 ± 2.27 | −4.3 ± 1.53 |
| Seed 24, eighth, assignment | 36 | +0.9 ± 2.00 | +2.3 ± 1.93 | −6.0 ± 2.22 | −4.7 ± 1.54 |
| Seed 22, first mapping, mirrored | 6 | +0.2 ± 0.47 | +0.5 ± 0.48 | −0.2 ± 0.60 | +0.1 ± 0.49 |
| Seed 22, first mapping, mirrored | 12 | −0.1 ± 0.88 | +0.6 ± 1.15 | −0.4 ± 1.01 | +0.3 ± 1.16 |
| Seed 22, first mapping, mirrored | 18 | −0.3 ± 1.17 | +0.6 ± 1.22 | +0.1 ± 1.24 | +0.9 ± 1.09 |
| Seed 22, first mapping, mirrored | 24 | +0.2 ± 1.38 | +1.1 ± 1.48 | +0.1 ± 1.41 | +1.0 ± 1.30 |
| Seed 22, first mapping, mirrored | 36 | −0.4 ± 1.02 | +0.6 ± 1.09 | −0.1 ± 1.24 | +0.9 ± 0.78 |
| Seed 22, eighth, assignment | 6 | 0.0 ± 1.09 | +0.4 ± 1.11 | −0.3 ± 1.28 | +0.1 ± 0.99 |
| Seed 22, eighth, assignment | 12 | +0.4 ± 0.88 | +2.3 ± 0.98 | −1.7 ± 0.73 | +0.1 ± 0.87 |
| Seed 22, eighth, assignment | 18 | +0.9 ± 0.92 | +1.3 ± 1.26 | −0.4 ± 0.90 | 0.0 ± 1.00 |
| Seed 22, eighth, assignment | 24 | +0.3 ± 0.99 | +2.4 ± 1.09 | −1.7 ± 0.87 | +0.4 ± 0.80 |
| Seed 22, eighth, assignment | 36 | +1.0 ± 1.66 | +2.3 ± 1.75 | −1.4 ± 1.69 | −0.1 ± 1.44 |

- **The night held across less H-47's plain night** lies between −2.6 and +3.5, beyond twice its unpaired SE at 5 of the 60 checkpoints,
  all above and none beyond 2.88 times it: on seed 22's eighth image from the mirrored assignment at block 18, +2.3 ± 0.83; on seed 24's
  eighth image from the mirrored assignment at block 24, +3.5 ± 1.68; on seed 24's first mapping from the assignment at block 36,
  +3.2 ± 1.12; on seed 23's first mapping from the mirrored assignment at block 36, +2.2 ± 0.91; and on seed 24's eighth image from the
  assignment at block 18, +3.3 ± 1.59.
- **The night held within less H-47's plain night** lies between −4.1 and +8.4, beyond twice its unpaired SE at 17 of the 60, 13 above and
  4 below. Above: on seed 24's eighth image from the mirrored assignment from block 12 on, +4.3 ± 1.55 to +7.1 ± 1.72; on seed 23's eighth
  image from the assignment from block 12 on, +3.8 ± 1.55 to +8.4 ± 0.75; on seed 22's eighth image from the assignment at blocks 12 and
  24, +2.3 ± 0.98 and +2.4 ± 1.09; on seed 24's first mapping from the assignment at block 12, +2.2 ± 0.93; on seed 24's eighth image from
  the assignment at block 18, +3.1 ± 1.37; and on seed 24's first mapping from the mirrored assignment at block 36, +1.7 ± 0.84. Below: on
  seed 22's eighth image from the mirrored assignment at blocks 24 and 36, −3.2 ± 1.31 and −2.9 ± 0.82, and on seed 23's eighth image from
  the mirrored assignment at blocks 18 and 24, −4.1 ± 1.13 and −3.3 ± 0.89.
- **The night held across less the excitatory night** lies between −6.0 and +4.3, beyond twice its unpaired SE at 15 of the 60, 3 above
  and 12 below. Above: on seed 22's eighth image from the mirrored assignment at blocks 18 and 24, +3.2 ± 1.06 and +4.3 ± 1.71, and on seed
  22's first mapping from the assignment at block 24, +2.2 ± 1.06. Below: on seed 24's eighth image from the mirrored assignment at blocks
  6, 12, 24 and 36, −1.9 ± 0.81 to −4.6 ± 1.95; on seed 23's eighth image from the assignment from block 12 on, −3.2 ± 0.86 to
  −5.0 ± 0.97; on seed 24's eighth image from the assignment at blocks 24 and 36, −5.8 ± 2.27 and −6.0 ± 2.22; on seed 24's first mapping
  from the mirrored assignment at block 18, −1.8 ± 0.90; and on seed 22's eighth image from the assignment at block 12, −1.7 ± 0.73.
- **The night held within less the excitatory night** lies between −4.7 and +1.6, beyond twice its unpaired SE at 4 of the 60, all below
  and all on seed 24's eighth image from the assignment, from block 12 on: −2.3 ± 0.87, −4.0 ± 1.52, −4.3 ± 1.53 and −4.7 ± 1.54, the
  largest 3.05 times it.
- The excitatory night less H-47's plain night, on the same cells, lay beyond twice its unpaired SE at 8 of exploration 025's 20
  checkpoints and at 17 of exploration 026's 40.

Each stimulus's correct trials, the mean of the 48 reads, under each form:

| Cell | Block | A's: without replay | held across | held within | plain on A's | excitatory | B's: without replay | held across | held within | plain on A's | excitatory |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 24.8 | 28.8 | 29.4 | 29.0 | 29.4 | 21.5 | 24.1 | 23.1 | 24.4 | 23.0 |
| Seed 22, eighth, mirrored | 12 | 11.3 | 20.4 | 19.0 | 19.1 | 20.1 | 18.9 | 22.1 | 21.9 | 23.2 | 20.8 |
| Seed 22, eighth, mirrored | 18 | 9.5 | 20.5 | 17.4 | 18.1 | 19.0 | 16.8 | 23.7 | 23.1 | 23.7 | 21.9 |
| Seed 22, eighth, mirrored | 24 | 8.6 | 15.9 | 14.3 | 14.7 | 15.8 | 14.0 | 25.5 | 22.7 | 25.5 | 21.3 |
| Seed 22, eighth, mirrored | 36 | 4.7 | 14.5 | 13.7 | 13.9 | 15.0 | 11.9 | 21.6 | 19.6 | 22.3 | 17.8 |
| Seed 22, first mapping, assignment | 6 | 25.5 | 30.0 | 29.7 | 29.6 | 29.7 | 26.9 | 27.0 | 27.0 | 26.9 | 27.1 |
| Seed 22, first mapping, assignment | 12 | 21.1 | 26.1 | 25.8 | 24.9 | 25.8 | 23.6 | 22.2 | 21.6 | 22.7 | 20.0 |
| Seed 22, first mapping, assignment | 18 | 16.4 | 24.6 | 25.2 | 24.8 | 25.0 | 23.7 | 22.5 | 21.4 | 22.2 | 21.3 |
| Seed 22, first mapping, assignment | 24 | 15.5 | 23.1 | 23.8 | 23.8 | 23.2 | 23.4 | 24.7 | 23.4 | 24.3 | 22.4 |
| Seed 22, first mapping, assignment | 36 | 13.2 | 21.3 | 21.5 | 21.5 | 21.3 | 23.1 | 22.8 | 21.3 | 22.7 | 20.3 |
| Seed 23, eighth, mirrored | 6 | 20.0 | 21.3 | 22.0 | 22.5 | 21.8 | 14.4 | 14.2 | 13.5 | 13.6 | 15.0 |
| Seed 23, eighth, mirrored | 12 | 15.3 | 19.3 | 19.4 | 21.5 | 18.0 | 4.2 | 7.4 | 6.9 | 7.0 | 7.4 |
| Seed 23, eighth, mirrored | 18 | 12.3 | 21.2 | 20.0 | 24.4 | 19.5 | 2.6 | 3.7 | 3.4 | 3.2 | 4.5 |
| Seed 23, eighth, mirrored | 24 | 8.9 | 23.0 | 22.3 | 25.8 | 21.4 | 1.7 | 4.2 | 3.6 | 3.5 | 3.8 |
| Seed 23, eighth, mirrored | 36 | 7.4 | 22.0 | 21.1 | 23.3 | 20.9 | 0.8 | 2.7 | 2.6 | 2.3 | 2.0 |
| Seed 23, first mapping, assignment | 6 | 29.2 | 30.8 | 31.0 | 30.9 | 30.8 | 25.0 | 25.4 | 25.3 | 25.4 | 25.3 |
| Seed 23, first mapping, assignment | 12 | 24.2 | 29.0 | 30.5 | 30.1 | 29.6 | 9.3 | 6.8 | 8.5 | 7.4 | 7.9 |
| Seed 23, first mapping, assignment | 18 | 19.5 | 29.5 | 29.9 | 30.1 | 29.2 | 7.6 | 5.1 | 4.9 | 4.9 | 5.9 |
| Seed 23, first mapping, assignment | 24 | 17.3 | 29.7 | 30.0 | 30.4 | 29.2 | 7.5 | 5.1 | 5.8 | 4.3 | 5.8 |
| Seed 23, first mapping, assignment | 36 | 15.7 | 28.8 | 29.1 | 29.2 | 28.0 | 4.0 | 4.1 | 3.9 | 3.9 | 4.3 |
| Seed 24, eighth, mirrored | 6 | 19.1 | 17.5 | 18.2 | 17.0 | 18.9 | 17.3 | 17.3 | 17.0 | 16.8 | 17.8 |
| Seed 24, eighth, mirrored | 12 | 15.3 | 13.5 | 16.4 | 12.1 | 16.2 | 16.3 | 16.6 | 17.4 | 16.6 | 17.4 |
| Seed 24, eighth, mirrored | 18 | 9.2 | 12.1 | 16.9 | 9.9 | 16.0 | 11.0 | 18.9 | 18.3 | 18.6 | 18.5 |
| Seed 24, eighth, mirrored | 24 | 7.4 | 11.4 | 16.5 | 9.0 | 16.6 | 9.0 | 17.8 | 16.4 | 16.8 | 17.3 |
| Seed 24, eighth, mirrored | 36 | 5.3 | 8.0 | 12.7 | 6.9 | 13.3 | 7.2 | 17.9 | 15.4 | 16.9 | 15.9 |
| Seed 24, first mapping, assignment | 6 | 28.8 | 30.5 | 30.9 | 30.7 | 30.6 | 29.5 | 30.6 | 30.8 | 30.6 | 30.6 |
| Seed 24, first mapping, assignment | 12 | 26.4 | 28.4 | 30.3 | 28.6 | 29.0 | 25.6 | 27.3 | 26.7 | 26.2 | 26.6 |
| Seed 24, first mapping, assignment | 18 | 24.9 | 27.4 | 30.1 | 28.9 | 29.1 | 22.6 | 24.9 | 22.3 | 22.5 | 23.4 |
| Seed 24, first mapping, assignment | 24 | 22.3 | 27.0 | 30.5 | 28.4 | 29.1 | 21.4 | 26.3 | 23.3 | 24.0 | 23.6 |
| Seed 24, first mapping, assignment | 36 | 21.2 | 24.5 | 27.6 | 24.5 | 26.1 | 18.8 | 22.5 | 18.6 | 19.3 | 19.7 |
| Seed 23, eighth, assignment | 6 | 24.3 | 27.1 | 27.2 | 26.9 | 27.0 | 23.1 | 23.4 | 23.2 | 23.4 | 23.7 |
| Seed 23, eighth, assignment | 12 | 18.3 | 22.3 | 23.1 | 21.2 | 23.2 | 22.6 | 22.0 | 24.2 | 22.2 | 24.3 |
| Seed 23, eighth, assignment | 18 | 15.6 | 23.1 | 24.0 | 20.8 | 24.4 | 21.7 | 23.1 | 26.0 | 24.4 | 25.7 |
| Seed 23, eighth, assignment | 24 | 16.3 | 22.9 | 25.3 | 19.6 | 25.2 | 21.7 | 23.0 | 26.8 | 24.1 | 25.8 |
| Seed 23, eighth, assignment | 36 | 13.0 | 18.2 | 20.3 | 15.3 | 21.0 | 23.2 | 24.1 | 26.1 | 25.0 | 26.1 |
| Seed 24, first mapping, mirrored | 6 | 28.1 | 30.3 | 29.7 | 30.1 | 30.2 | 26.2 | 27.4 | 27.5 | 27.6 | 27.8 |
| Seed 24, first mapping, mirrored | 12 | 24.6 | 27.3 | 27.0 | 27.3 | 27.2 | 21.7 | 23.3 | 23.7 | 23.8 | 25.1 |
| Seed 24, first mapping, mirrored | 18 | 21.7 | 26.5 | 26.5 | 27.0 | 26.8 | 22.3 | 24.9 | 25.0 | 24.8 | 26.5 |
| Seed 24, first mapping, mirrored | 24 | 22.1 | 25.4 | 26.0 | 25.8 | 26.1 | 22.3 | 26.5 | 27.3 | 26.2 | 27.6 |
| Seed 24, first mapping, mirrored | 36 | 22.0 | 25.6 | 25.3 | 25.2 | 25.5 | 20.8 | 24.2 | 25.1 | 23.5 | 24.6 |
| Seed 23, first mapping, mirrored | 6 | 30.4 | 31.1 | 31.1 | 31.0 | 31.2 | 24.6 | 25.3 | 25.5 | 25.9 | 25.4 |
| Seed 23, first mapping, mirrored | 12 | 28.5 | 30.2 | 29.8 | 29.6 | 30.3 | 23.8 | 25.1 | 25.0 | 24.8 | 25.3 |
| Seed 23, first mapping, mirrored | 18 | 28.5 | 28.0 | 28.5 | 27.6 | 28.6 | 23.5 | 24.6 | 25.3 | 24.8 | 25.6 |
| Seed 23, first mapping, mirrored | 24 | 30.0 | 26.4 | 26.3 | 25.0 | 26.8 | 23.5 | 26.5 | 26.3 | 25.9 | 26.5 |
| Seed 23, first mapping, mirrored | 36 | 29.3 | 25.4 | 25.5 | 24.1 | 26.1 | 23.9 | 25.1 | 25.0 | 24.2 | 25.3 |
| Seed 24, eighth, assignment | 6 | 21.3 | 23.4 | 23.4 | 23.8 | 23.6 | 17.5 | 16.2 | 14.9 | 15.3 | 15.7 |
| Seed 24, eighth, assignment | 12 | 18.4 | 19.1 | 20.0 | 18.8 | 20.2 | 16.4 | 16.1 | 14.5 | 15.4 | 16.6 |
| Seed 24, eighth, assignment | 18 | 18.0 | 15.2 | 16.3 | 14.4 | 18.2 | 16.5 | 16.6 | 15.3 | 14.1 | 17.4 |
| Seed 24, eighth, assignment | 24 | 19.2 | 15.7 | 16.0 | 15.3 | 17.6 | 16.3 | 15.2 | 16.4 | 14.3 | 19.1 |
| Seed 24, eighth, assignment | 36 | 12.9 | 14.4 | 16.0 | 13.1 | 17.4 | 12.7 | 13.9 | 13.7 | 14.3 | 16.9 |
| Seed 22, first mapping, mirrored | 6 | 30.6 | 31.4 | 31.4 | 31.4 | 31.5 | 26.0 | 26.5 | 26.9 | 26.4 | 26.6 |
| Seed 22, first mapping, mirrored | 12 | 28.2 | 29.0 | 29.0 | 29.3 | 28.9 | 21.3 | 23.3 | 24.0 | 23.1 | 23.8 |
| Seed 22, first mapping, mirrored | 18 | 28.0 | 27.5 | 27.9 | 27.6 | 27.6 | 21.0 | 24.9 | 25.3 | 25.1 | 24.7 |
| Seed 22, first mapping, mirrored | 24 | 27.5 | 27.7 | 28.2 | 27.6 | 27.6 | 18.4 | 23.2 | 23.6 | 23.0 | 23.1 |
| Seed 22, first mapping, mirrored | 36 | 27.8 | 27.1 | 27.7 | 27.4 | 26.9 | 16.4 | 21.6 | 22.0 | 21.8 | 21.9 |
| Seed 22, eighth, assignment | 6 | 24.4 | 26.6 | 26.6 | 26.4 | 26.5 | 25.9 | 25.7 | 26.1 | 25.8 | 26.1 |
| Seed 22, eighth, assignment | 12 | 22.9 | 25.5 | 26.2 | 25.4 | 26.4 | 24.8 | 25.6 | 26.7 | 25.3 | 26.4 |
| Seed 22, eighth, assignment | 18 | 24.4 | 24.9 | 24.9 | 23.8 | 25.0 | 22.8 | 26.5 | 27.0 | 26.8 | 26.8 |
| Seed 22, eighth, assignment | 24 | 26.0 | 25.5 | 26.1 | 25.8 | 25.9 | 21.3 | 24.9 | 26.5 | 24.4 | 26.2 |
| Seed 22, eighth, assignment | 36 | 24.7 | 24.1 | 23.6 | 23.1 | 23.9 | 19.5 | 23.3 | 25.1 | 23.2 | 24.9 |

## 2. The side under each held night

The side under a held night is the stimulus whose difference from the night without replay is the larger. "A's less B's" is the paired
difference of the two, and "within 2 SE" marks it within twice that SE. H-47's side is the same reading of H-47's plain night, from H-47's
job, and the excitatory night's of the excitatory night, from exploration 025's or 026's job. The group named is the one that stimulus
answers in that cell:

| Cell | Block | A's: held across less without replay | B's | A's less B's | A's: held within less without replay | B's | A's less B's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +4.1 ± 1.03 | +2.6 ± 1.13 | +1.5 ± 1.96 | +4.7 ± 0.91 | +1.6 ± 1.11 | +3.0 ± 1.84 |
| Seed 22, eighth, mirrored | 12 | +9.0 ± 1.01 | +3.3 ± 0.93 | +5.8 ± 1.73 | +7.6 ± 1.01 | +3.0 ± 0.69 | +4.6 ± 1.28 |
| Seed 22, eighth, mirrored | 18 | +11.0 ± 1.23 | +6.9 ± 1.54 | +4.1 ± 2.25 | +7.9 ± 1.49 | +6.3 ± 1.08 | +1.6 ± 1.88 |
| Seed 22, eighth, mirrored | 24 | +7.3 ± 0.72 | +11.5 ± 1.16 | −4.2 ± 1.51 | +5.6 ± 1.67 | +8.8 ± 0.92 | −3.1 ± 2.06 |
| Seed 22, eighth, mirrored | 36 | +9.8 ± 0.98 | +9.8 ± 0.48 | 0.0 ± 0.92 | +9.0 ± 1.11 | +7.8 ± 0.89 | +1.2 ± 1.83 |
| Seed 22, first mapping, assignment | 6 | +4.5 ± 0.58 | +0.1 ± 0.61 | +4.5 ± 0.46 | +4.2 ± 0.52 | +0.1 ± 0.75 | +4.0 ± 0.39 |
| Seed 22, first mapping, assignment | 12 | +4.9 ± 0.90 | −1.4 ± 0.63 | +6.4 ± 1.35 | +4.6 ± 0.60 | −2.0 ± 0.80 | +6.6 ± 1.21 |
| Seed 22, first mapping, assignment | 18 | +8.1 ± 0.91 | −1.2 ± 0.78 | +9.4 ± 1.42 | +8.7 ± 1.01 | −2.3 ± 0.98 | +11.0 ± 1.68 |
| Seed 22, first mapping, assignment | 24 | +7.7 ± 1.02 | +1.3 ± 0.67 | +6.4 ± 1.34 | +8.4 ± 1.02 | 0.0 ± 1.16 | +8.3 ± 1.91 |
| Seed 22, first mapping, assignment | 36 | +8.1 ± 1.47 | −0.4 ± 1.36 | +8.5 ± 2.62 | +8.3 ± 1.06 | −1.8 ± 0.81 | +10.2 ± 1.23 |
| Seed 23, eighth, mirrored | 6 | +1.3 ± 1.08 | −0.2 ± 0.83 | +1.5 ± 1.40 | +2.0 ± 0.86 | −1.0 ± 0.82 | +2.9 ± 1.19 |
| Seed 23, eighth, mirrored | 12 | +3.9 ± 1.16 | +3.3 ± 0.72 | +0.7 ± 1.79 | +4.0 ± 0.57 | +2.8 ± 0.65 | +1.3 ± 0.95 |
| Seed 23, eighth, mirrored | 18 | +9.0 ± 1.57 | +1.1 ± 0.51 | +7.8 ± 1.80 | +7.8 ± 0.99 | +0.8 ± 0.62 | +6.9 ± 1.02 |
| Seed 23, eighth, mirrored | 24 | +14.1 ± 2.12 | +2.5 ± 0.25 | +11.6 ± 2.21 | +13.4 ± 1.21 | +2.0 ± 0.50 | +11.5 ± 0.86 |
| Seed 23, eighth, mirrored | 36 | +14.6 ± 1.18 | +1.9 ± 0.29 | +12.7 ± 0.96 | +13.7 ± 0.79 | +1.8 ± 0.30 | +11.9 ± 0.61 |
| Seed 23, first mapping, assignment | 6 | +1.6 ± 0.51 | +0.4 ± 0.57 | +1.2 ± 0.77 | +1.8 ± 0.45 | +0.3 ± 0.66 | +1.5 ± 0.70 |
| Seed 23, first mapping, assignment | 12 | +4.8 ± 1.05 | −2.5 ± 1.13 | +7.3 ± 1.68 | +6.3 ± 1.01 | −0.8 ± 0.90 | +7.1 ± 1.58 |
| Seed 23, first mapping, assignment | 18 | +10.0 ± 1.81 | −2.5 ± 0.89 | +12.5 ± 1.70 | +10.3 ± 1.64 | −2.7 ± 1.19 | +13.0 ± 1.31 |
| Seed 23, first mapping, assignment | 24 | +12.5 ± 1.71 | −2.5 ± 0.92 | +15.0 ± 1.49 | +12.7 ± 1.77 | −1.7 ± 0.62 | +14.4 ± 1.82 |
| Seed 23, first mapping, assignment | 36 | +13.1 ± 1.18 | +0.1 ± 0.89 | +13.0 ± 1.44 | +13.4 ± 1.13 | −0.1 ± 1.00 | +13.4 ± 1.37 |
| Seed 24, eighth, mirrored | 6 | −1.6 ± 0.64 | 0.0 ± 1.02 | −1.6 ± 1.20 | −0.9 ± 0.79 | −0.3 ± 1.09 | −0.6 ± 1.34 |
| Seed 24, eighth, mirrored | 12 | −1.7 ± 0.89 | +0.3 ± 0.83 | −2.0 ± 1.30 | +1.1 ± 0.77 | +1.2 ± 0.86 | −0.1 ± 0.96 |
| Seed 24, eighth, mirrored | 18 | +2.9 ± 1.04 | +7.9 ± 0.95 | −5.0 ± 0.81 | +7.7 ± 0.88 | +7.3 ± 1.22 | +0.3 ± 1.68 |
| Seed 24, eighth, mirrored | 24 | +4.0 ± 0.89 | +8.8 ± 1.30 | −4.8 ± 1.11 | +9.1 ± 1.18 | +7.4 ± 1.45 | +1.8 ± 1.76 |
| Seed 24, eighth, mirrored | 36 | +2.7 ± 0.33 | +10.7 ± 1.25 | −8.0 ± 1.46 | +7.4 ± 0.42 | +8.2 ± 0.89 | −0.8 ± 0.56 |
| Seed 24, first mapping, assignment | 6 | +1.7 ± 0.56 | +1.1 ± 0.31 | +0.6 ± 0.62 | +2.1 ± 0.49 | +1.3 ± 0.45 | +0.8 ± 0.61 |
| Seed 24, first mapping, assignment | 12 | +2.0 ± 0.48 | +1.7 ± 1.00 | +0.4 ± 1.15 | +3.9 ± 0.42 | +1.1 ± 1.02 | +2.8 ± 1.16 |
| Seed 24, first mapping, assignment | 18 | +2.5 ± 0.75 | +2.3 ± 1.07 | +0.2 ± 1.14 | +5.2 ± 0.48 | −0.3 ± 1.04 | +5.5 ± 1.27 |
| Seed 24, first mapping, assignment | 24 | +4.7 ± 1.14 | +4.9 ± 1.21 | −0.2 ± 0.76 | +8.2 ± 1.07 | +1.8 ± 1.16 | +6.4 ± 0.91 |
| Seed 24, first mapping, assignment | 36 | +3.3 ± 0.59 | +3.6 ± 0.83 | −0.3 ± 1.10 | +6.4 ± 0.59 | −0.3 ± 1.70 | +6.7 ± 2.07 |
| Seed 23, eighth, assignment | 6 | +2.8 ± 0.67 | +0.4 ± 0.61 | +2.4 ± 0.94 | +2.9 ± 0.69 | +0.1 ± 0.61 | +2.7 ± 0.83 |
| Seed 23, eighth, assignment | 12 | +4.0 ± 0.66 | −0.5 ± 0.92 | +4.5 ± 1.04 | +4.8 ± 1.03 | +1.6 ± 0.83 | +3.2 ± 1.09 |
| Seed 23, eighth, assignment | 18 | +7.5 ± 0.81 | +1.4 ± 0.51 | +6.1 ± 0.57 | +8.4 ± 0.77 | +4.3 ± 0.65 | +4.1 ± 1.05 |
| Seed 23, eighth, assignment | 24 | +6.6 ± 1.26 | +1.4 ± 1.40 | +5.3 ± 1.35 | +9.0 ± 1.07 | +5.1 ± 0.88 | +3.8 ± 1.17 |
| Seed 23, eighth, assignment | 36 | +5.2 ± 0.67 | +0.9 ± 2.06 | +4.3 ± 1.91 | +7.3 ± 0.32 | +2.9 ± 1.89 | +4.4 ± 2.05 |
| Seed 24, first mapping, mirrored | 6 | +2.2 ± 0.54 | +1.2 ± 0.62 | +1.0 ± 0.83 | +1.6 ± 0.49 | +1.3 ± 0.68 | +0.3 ± 0.85 |
| Seed 24, first mapping, mirrored | 12 | +2.7 ± 0.54 | +1.6 ± 1.13 | +1.1 ± 1.11 | +2.3 ± 0.77 | +2.0 ± 1.19 | +0.3 ± 1.06 |
| Seed 24, first mapping, mirrored | 18 | +4.8 ± 0.59 | +2.6 ± 0.97 | +2.2 ± 1.19 | +4.8 ± 0.43 | +2.8 ± 0.52 | +2.0 ± 0.45 |
| Seed 24, first mapping, mirrored | 24 | +3.3 ± 0.95 | +4.3 ± 0.89 | −1.0 ± 0.63 | +3.9 ± 0.94 | +5.0 ± 0.69 | −1.1 ± 1.21 |
| Seed 24, first mapping, mirrored | 36 | +3.6 ± 0.62 | +3.4 ± 0.97 | +0.2 ± 0.92 | +3.4 ± 0.63 | +4.3 ± 0.87 | −0.9 ± 0.63 |
| Seed 23, first mapping, mirrored | 6 | +0.6 ± 0.34 | +0.7 ± 0.59 | −0.1 ± 0.66 | +0.6 ± 0.34 | +0.9 ± 0.62 | −0.3 ± 0.65 |
| Seed 23, first mapping, mirrored | 12 | +1.7 ± 0.60 | +1.3 ± 1.11 | +0.4 ± 1.44 | +1.2 ± 0.55 | +1.2 ± 0.86 | 0.0 ± 1.07 |
| Seed 23, first mapping, mirrored | 18 | −0.5 ± 0.88 | +1.1 ± 0.86 | −1.6 ± 0.74 | 0.0 ± 0.48 | +1.8 ± 0.89 | −1.8 ± 0.98 |
| Seed 23, first mapping, mirrored | 24 | −3.6 ± 1.01 | +3.0 ± 0.50 | −6.6 ± 1.02 | −3.7 ± 0.59 | +2.8 ± 0.43 | −6.5 ± 0.81 |
| Seed 23, first mapping, mirrored | 36 | −3.8 ± 1.07 | +1.2 ± 1.25 | −5.0 ± 1.67 | −3.7 ± 0.54 | +1.1 ± 0.84 | −4.8 ± 1.24 |
| Seed 24, eighth, assignment | 6 | +2.1 ± 0.96 | −1.3 ± 0.60 | +3.4 ± 1.39 | +2.1 ± 0.77 | −2.6 ± 0.66 | +4.7 ± 1.21 |
| Seed 24, eighth, assignment | 12 | +0.7 ± 1.00 | −0.3 ± 1.81 | +1.0 ± 2.62 | +1.6 ± 1.26 | −1.9 ± 1.84 | +3.5 ± 2.93 |
| Seed 24, eighth, assignment | 18 | −2.8 ± 1.21 | +0.1 ± 1.59 | −2.9 ± 1.42 | −1.7 ± 1.09 | −1.2 ± 1.70 | −0.5 ± 1.89 |
| Seed 24, eighth, assignment | 24 | −3.5 ± 0.98 | −1.1 ± 1.62 | −2.4 ± 1.18 | −3.2 ± 1.45 | 0.0 ± 1.51 | −3.2 ± 1.45 |
| Seed 24, eighth, assignment | 36 | +1.5 ± 1.18 | +1.3 ± 1.41 | +0.2 ± 0.88 | +3.1 ± 0.74 | +1.0 ± 0.92 | +2.0 ± 1.20 |
| Seed 22, first mapping, mirrored | 6 | +0.8 ± 0.35 | +0.5 ± 0.59 | +0.3 ± 0.74 | +0.7 ± 0.34 | +0.9 ± 0.47 | −0.1 ± 0.64 |
| Seed 22, first mapping, mirrored | 12 | +0.9 ± 0.70 | +2.0 ± 1.37 | −1.1 ± 0.90 | +0.8 ± 0.58 | +2.7 ± 1.59 | −1.8 ± 1.38 |
| Seed 22, first mapping, mirrored | 18 | −0.4 ± 1.29 | +3.9 ± 1.68 | −4.3 ± 1.25 | 0.0 ± 1.09 | +4.3 ± 1.48 | −4.3 ± 1.24 |
| Seed 22, first mapping, mirrored | 24 | +0.1 ± 1.52 | +4.8 ± 1.01 | −4.6 ± 1.04 | +0.7 ± 1.20 | +5.2 ± 0.97 | −4.5 ± 0.78 |
| Seed 22, first mapping, mirrored | 36 | −0.6 ± 1.26 | +5.2 ± 1.37 | −5.9 ± 1.73 | −0.1 ± 0.74 | +5.7 ± 1.26 | −5.8 ± 1.21 |
| Seed 22, eighth, assignment | 6 | +2.2 ± 0.53 | −0.2 ± 0.61 | +2.4 ± 1.03 | +2.2 ± 0.72 | +0.1 ± 0.67 | +2.0 ± 0.99 |
| Seed 22, eighth, assignment | 12 | +2.6 ± 0.62 | +0.9 ± 0.83 | +1.8 ± 1.26 | +3.3 ± 0.64 | +1.9 ± 0.74 | +1.4 ± 1.11 |
| Seed 22, eighth, assignment | 18 | +0.5 ± 0.58 | +3.7 ± 0.54 | −3.2 ± 0.80 | +0.5 ± 0.72 | +4.2 ± 0.70 | −3.7 ± 1.18 |
| Seed 22, eighth, assignment | 24 | −0.5 ± 0.71 | +3.6 ± 0.94 | −4.1 ± 1.24 | +0.1 ± 0.73 | +5.2 ± 1.10 | −5.1 ± 1.45 |
| Seed 22, eighth, assignment | 36 | −0.7 ± 0.75 | +3.9 ± 1.25 | −4.5 ± 1.82 | −1.1 ± 1.60 | +5.6 ± 0.65 | −6.8 ± 2.11 |

| Cell | Block | Held across: side | Held within: side | H-47's side | Excitatory: side |
| :--- | ---: | :--- | :--- | :--- | :--- |
| Seed 22, eighth, mirrored | 6 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | 12 | A, the odd places | A, the odd places | A, the odd places (within 2 SE) | A, the odd places |
| Seed 22, eighth, mirrored | 18 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | 24 | B, the even places | B, the even places (within 2 SE) | B, the even places | B, the even places (within 2 SE) |
| Seed 22, eighth, mirrored | 36 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | B, the even places (within 2 SE) | A, the odd places |
| Seed 22, first mapping, assignment | 6 | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 12 | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 18 | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 24 | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 36 | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | 6 | A, the odd places (within 2 SE) | A, the odd places | A, the odd places | A, the odd places (within 2 SE) |
| Seed 23, eighth, mirrored | 12 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places | B, the even places (within 2 SE) |
| Seed 23, eighth, mirrored | 18 | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | 24 | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | 36 | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 6 | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 23, first mapping, assignment | 12 | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 18 | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 24 | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 36 | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 24, eighth, mirrored | 6 | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places (within 2 SE) |
| Seed 24, eighth, mirrored | 12 | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places | B, the even places (within 2 SE) |
| Seed 24, eighth, mirrored | 18 | B, the even places | A, the odd places (within 2 SE) | B, the even places | B, the even places (within 2 SE) |
| Seed 24, eighth, mirrored | 24 | B, the even places | A, the odd places (within 2 SE) | B, the even places | A, the odd places (within 2 SE) |
| Seed 24, eighth, mirrored | 36 | B, the even places | B, the even places (within 2 SE) | B, the even places | B, the even places (within 2 SE) |
| Seed 24, first mapping, assignment | 6 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 24, first mapping, assignment | 12 | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 24, first mapping, assignment | 18 | A, the odd places (within 2 SE) | A, the odd places | A, the odd places | A, the odd places |
| Seed 24, first mapping, assignment | 24 | B, the even places (within 2 SE) | A, the odd places | A, the odd places | A, the odd places |
| Seed 24, first mapping, assignment | 36 | B, the even places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places |
| Seed 23, eighth, assignment | 6 | A, the even places | A, the even places | A, the even places | A, the even places |
| Seed 23, eighth, assignment | 12 | A, the even places | A, the even places | A, the even places | A, the even places |
| Seed 23, eighth, assignment | 18 | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places |
| Seed 23, eighth, assignment | 24 | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places |
| Seed 23, eighth, assignment | 36 | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places |
| Seed 24, first mapping, mirrored | 6 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | 12 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 24, first mapping, mirrored | 18 | A, the even places (within 2 SE) | A, the even places | A, the even places | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | 24 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places |
| Seed 24, first mapping, mirrored | 36 | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 6 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 12 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 23, first mapping, mirrored | 18 | B, the odd places | B, the odd places (within 2 SE) | B, the odd places | B, the odd places |
| Seed 23, first mapping, mirrored | 24 | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 23, first mapping, mirrored | 36 | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 24, eighth, assignment | 6 | A, the even places | A, the even places | A, the even places | A, the even places |
| Seed 24, eighth, assignment | 12 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, eighth, assignment | 18 | B, the odd places | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | 24 | B, the odd places | B, the odd places | B, the odd places (within 2 SE) | B, the odd places |
| Seed 24, eighth, assignment | 36 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | 6 | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | 12 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 22, first mapping, mirrored | 18 | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, first mapping, mirrored | 24 | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, first mapping, mirrored | 36 | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | 6 | A, the even places | A, the even places | A, the even places | A, the even places |
| Seed 22, eighth, assignment | 12 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places | A, the even places (within 2 SE) |
| Seed 22, eighth, assignment | 18 | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | 24 | B, the odd places | B, the odd places | B, the odd places (within 2 SE) | B, the odd places |
| Seed 22, eighth, assignment | 36 | B, the odd places | B, the odd places | B, the odd places | B, the odd places |

- **The side under the night held across is H-47's at 56 of the 60 checkpoints**, and the excitatory night's at 54. It lies beyond twice
  its SE at 35 of the 60, as do H-47's and the excitatory night's. It differs from H-47's at four:
  - seed 22's eighth image from the mirrored assignment at block 36: on A, the odd places, within 2 SE, where H-47's lies on B, the even
    places, within 2 SE;
  - seed 24's first mapping from the assignment at blocks 24 and 36: on B, the even places, within 2 SE, where H-47's lies on A, the odd
    places, beyond 2 SE at block 24 and within 2 SE at block 36;
  - seed 24's eighth image from the assignment at block 36: on A, the even places, within 2 SE, where H-47's lies on B, the odd places,
    within 2 SE.
- **The side under the night held within is the excitatory night's at 56 of the 60 checkpoints**, and H-47's at 54. It lies beyond twice
  its SE at 36 of the 60. It differs from the excitatory night's at four, each within 2 SE on both sides:
  - seed 23's eighth image from the mirrored assignment at block 12 and seed 24's eighth image from the mirrored assignment at block 18: on
    A, the odd places, where the excitatory night's lies on B, the even places;
  - seed 24's first mapping from the mirrored assignment at block 12: on A, the even places, where the excitatory night's lies on B, the
    odd places;
  - seed 22's first mapping from the mirrored assignment at block 6: on B, the odd places, where the excitatory night's lies on A, the even
    places.
- The side under the night held within differs from H-47's at six. At four the excitatory night's differs from H-47's too and lies where
  the night held within's lies: seed 22's eighth image from the mirrored assignment at block 36, seed 24's eighth image from the mirrored
  assignment at block 24, seed 24's first mapping from the mirrored assignment at block 36 and seed 24's eighth image from the assignment
  at block 36. At the other two, seed 24's eighth image from the mirrored assignment at block 18 and seed 22's first mapping from the
  mirrored assignment at block 6, it differs from both, which lie on the same stimulus.

Each stimulus's trials less H-47's plain night's and less the excitatory night's, each with exploration 018's unpaired SE:

| Cell | Block | A's: held across less H-47's plain | B's | A's: held within less H-47's plain | B's | A's: held across less excitatory | B's | A's: held within less excitatory | B's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | −0.2 ± 0.91 | −0.3 ± 1.04 | +0.5 ± 0.88 | −1.2 ± 0.91 | −0.6 ± 0.97 | +1.0 ± 1.05 | +0.1 ± 0.90 | +0.1 ± 0.92 |
| Seed 22, eighth, mirrored | 12 | +1.3 ± 0.79 | −1.1 ± 0.80 | −0.1 ± 0.96 | −1.3 ± 0.81 | +0.2 ± 0.88 | +1.3 ± 0.99 | −1.2 ± 0.93 | +1.1 ± 0.88 |
| Seed 22, eighth, mirrored | 18 | +2.4 ± 1.10 | −0.1 ± 1.17 | −0.7 ± 0.94 | −0.6 ± 1.04 | +1.5 ± 1.11 | +1.7 ± 1.32 | −1.6 ± 1.05 | +1.2 ± 1.11 |
| Seed 22, eighth, mirrored | 24 | +1.3 ± 0.82 | −0.1 ± 0.99 | −0.4 ± 0.99 | −2.8 ± 1.08 | +0.2 ± 0.61 | +4.1 ± 1.57 | −1.5 ± 1.06 | +1.4 ± 1.42 |
| Seed 22, eighth, mirrored | 36 | +0.6 ± 1.39 | −0.7 ± 0.50 | −0.3 ± 1.23 | −2.6 ± 0.99 | −0.5 ± 1.32 | +3.8 ± 1.07 | −1.3 ± 1.01 | +1.8 ± 1.59 |
| Seed 22, first mapping, assignment | 6 | +0.5 ± 0.91 | +0.1 ± 0.66 | +0.1 ± 0.87 | +0.1 ± 0.79 | +0.4 ± 0.89 | −0.1 ± 0.80 | 0.0 ± 0.79 | −0.1 ± 0.62 |
| Seed 22, first mapping, assignment | 12 | +1.1 ± 1.05 | −0.5 ± 1.30 | +0.8 ± 0.78 | −1.1 ± 1.41 | +0.3 ± 1.03 | +2.1 ± 1.28 | 0.0 ± 0.88 | +1.5 ± 1.22 |
| Seed 22, first mapping, assignment | 18 | −0.2 ± 0.93 | +0.3 ± 0.92 | +0.4 ± 0.87 | −0.7 ± 1.09 | −0.4 ± 1.09 | +1.2 ± 0.97 | +0.2 ± 1.29 | +0.2 ± 0.72 |
| Seed 22, first mapping, assignment | 24 | −0.6 ± 0.98 | +0.4 ± 0.97 | +0.1 ± 1.01 | −0.8 ± 1.25 | −0.1 ± 0.84 | +2.3 ± 1.23 | +0.6 ± 0.82 | +1.1 ± 1.06 |
| Seed 22, first mapping, assignment | 36 | −0.3 ± 1.02 | 0.0 ± 1.12 | 0.0 ± 0.92 | −1.4 ± 1.23 | 0.0 ± 0.93 | +2.5 ± 1.58 | +0.2 ± 0.66 | +1.1 ± 0.94 |
| Seed 23, eighth, mirrored | 6 | −1.2 ± 1.25 | +0.6 ± 0.89 | −0.6 ± 1.19 | −0.1 ± 0.90 | −0.5 ± 1.25 | −0.8 ± 0.79 | +0.2 ± 1.19 | −1.5 ± 0.78 |
| Seed 23, eighth, mirrored | 12 | −2.2 ± 0.95 | +0.4 ± 0.89 | −2.1 ± 1.31 | −0.1 ± 0.73 | +1.2 ± 1.26 | 0.0 ± 0.78 | +1.3 ± 0.96 | −0.5 ± 0.64 |
| Seed 23, eighth, mirrored | 18 | −3.1 ± 0.99 | +0.5 ± 0.56 | −4.4 ± 1.07 | +0.2 ± 0.72 | +1.7 ± 1.36 | −0.7 ± 1.09 | +0.5 ± 1.04 | −1.0 ± 0.85 |
| Seed 23, eighth, mirrored | 24 | −2.8 ± 1.20 | +0.7 ± 0.41 | −3.5 ± 0.91 | +0.2 ± 0.42 | +1.6 ± 1.43 | +0.4 ± 0.87 | +0.9 ± 0.96 | −0.2 ± 0.49 |
| Seed 23, eighth, mirrored | 36 | −1.3 ± 1.14 | +0.4 ± 0.32 | −2.2 ± 1.06 | +0.3 ± 0.35 | +1.1 ± 1.04 | +0.7 ± 0.53 | +0.2 ± 0.99 | +0.6 ± 0.38 |
| Seed 23, first mapping, assignment | 6 | −0.1 ± 0.83 | 0.0 ± 1.00 | +0.1 ± 0.84 | −0.1 ± 1.00 | 0.0 ± 0.87 | +0.1 ± 0.95 | +0.2 ± 0.89 | 0.0 ± 0.94 |
| Seed 23, first mapping, assignment | 12 | −1.2 ± 0.95 | −0.7 ± 0.94 | +0.4 ± 0.81 | +1.0 ± 0.98 | −0.6 ± 1.05 | −1.2 ± 0.74 | +0.9 ± 0.91 | +0.5 ± 0.94 |
| Seed 23, first mapping, assignment | 18 | −0.6 ± 1.12 | +0.2 ± 0.70 | −0.3 ± 1.06 | 0.0 ± 0.78 | +0.3 ± 1.16 | −0.8 ± 0.80 | +0.7 ± 1.14 | −0.9 ± 0.70 |
| Seed 23, first mapping, assignment | 24 | −0.7 ± 0.92 | +0.7 ± 0.94 | −0.5 ± 0.92 | +1.5 ± 0.77 | +0.5 ± 0.82 | −0.8 ± 0.78 | +0.8 ± 0.82 | 0.0 ± 0.77 |
| Seed 23, first mapping, assignment | 36 | −0.4 ± 0.96 | +0.2 ± 0.65 | −0.1 ± 0.92 | 0.0 ± 0.65 | +0.8 ± 1.12 | −0.2 ± 0.52 | +1.1 ± 1.10 | −0.4 ± 0.60 |
| Seed 24, eighth, mirrored | 6 | +0.5 ± 1.06 | +0.5 ± 0.98 | +1.2 ± 1.05 | +0.2 ± 0.97 | −1.4 ± 0.93 | −0.5 ± 0.96 | −0.7 ± 1.08 | −0.8 ± 0.97 |
| Seed 24, eighth, mirrored | 12 | +1.5 ± 1.08 | 0.0 ± 1.13 | +4.3 ± 1.11 | +0.9 ± 1.10 | −2.7 ± 0.89 | −0.8 ± 1.29 | +0.1 ± 0.93 | +0.1 ± 1.44 |
| Seed 24, eighth, mirrored | 18 | +2.2 ± 0.94 | +0.3 ± 1.53 | +7.0 ± 0.81 | −0.3 ± 1.09 | −3.8 ± 1.06 | +0.4 ± 1.33 | +0.9 ± 1.20 | −0.1 ± 1.31 |
| Seed 24, eighth, mirrored | 24 | +2.4 ± 1.51 | +1.1 ± 1.21 | +7.5 ± 1.21 | −0.4 ± 1.38 | −5.2 ± 1.11 | +0.5 ± 1.20 | −0.1 ± 1.27 | −1.0 ± 1.17 |
| Seed 24, eighth, mirrored | 36 | +1.1 ± 1.00 | +0.9 ± 0.83 | +5.8 ± 1.34 | −1.5 ± 0.72 | −5.3 ± 0.93 | +1.9 ± 1.00 | −0.6 ± 0.97 | −0.6 ± 0.70 |
| Seed 24, first mapping, assignment | 6 | −0.2 ± 1.06 | 0.0 ± 0.91 | +0.2 ± 0.99 | +0.2 ± 0.92 | −0.1 ± 1.05 | 0.0 ± 0.91 | +0.3 ± 0.99 | +0.2 ± 0.93 |
| Seed 24, first mapping, assignment | 12 | −0.2 ± 0.92 | +1.1 ± 1.06 | +1.7 ± 0.93 | +0.5 ± 1.00 | −0.6 ± 1.00 | +0.6 ± 1.08 | +1.2 ± 0.91 | +0.1 ± 1.06 |
| Seed 24, first mapping, assignment | 18 | −1.5 ± 0.94 | +2.4 ± 1.52 | +1.2 ± 0.87 | −0.1 ± 1.14 | −1.7 ± 0.88 | +1.5 ± 0.99 | +1.0 ± 0.77 | −1.0 ± 0.94 |
| Seed 24, first mapping, assignment | 24 | −1.4 ± 1.22 | +2.3 ± 1.38 | +2.1 ± 1.11 | −0.8 ± 1.51 | −2.1 ± 0.97 | +2.7 ± 1.30 | +1.4 ± 0.86 | −0.4 ± 1.39 |
| Seed 24, first mapping, assignment | 36 | +0.1 ± 1.01 | +3.1 ± 0.89 | +3.1 ± 1.23 | −0.8 ± 0.84 | −1.6 ± 1.29 | +2.8 ± 0.74 | +1.5 ± 1.48 | −1.1 ± 1.04 |
| Seed 23, eighth, assignment | 6 | +0.1 ± 0.81 | 0.0 ± 1.18 | +0.2 ± 0.77 | −0.2 ± 1.21 | +0.1 ± 0.83 | −0.3 ± 1.18 | +0.1 ± 0.79 | −0.5 ± 1.17 |
| Seed 23, eighth, assignment | 12 | +1.1 ± 1.35 | −0.2 ± 1.25 | +1.9 ± 1.56 | +1.9 ± 1.21 | −1.0 ± 1.11 | −2.3 ± 1.22 | −0.1 ± 1.19 | −0.1 ± 1.33 |
| Seed 23, eighth, assignment | 18 | +2.3 ± 0.92 | −1.3 ± 1.46 | +3.2 ± 1.03 | +1.6 ± 1.45 | −1.3 ± 1.04 | −2.5 ± 1.20 | −0.4 ± 1.01 | +0.3 ± 1.20 |
| Seed 23, eighth, assignment | 24 | +3.3 ± 0.96 | −1.1 ± 1.90 | +5.7 ± 1.04 | +2.7 ± 1.32 | −2.3 ± 1.07 | −2.7 ± 1.18 | 0.0 ± 1.11 | +1.0 ± 1.10 |
| Seed 23, eighth, assignment | 36 | +2.9 ± 1.37 | −0.9 ± 1.63 | +5.0 ± 1.28 | +1.1 ± 1.41 | −2.8 ± 1.07 | −2.0 ± 1.02 | −0.8 ± 1.01 | 0.0 ± 1.14 |
| Seed 24, first mapping, mirrored | 6 | +0.2 ± 1.01 | −0.1 ± 0.98 | −0.4 ± 0.98 | −0.1 ± 0.96 | +0.1 ± 0.99 | −0.3 ± 0.98 | −0.5 ± 0.96 | −0.3 ± 0.96 |
| Seed 24, first mapping, mirrored | 12 | +0.1 ± 1.04 | −0.5 ± 0.95 | −0.3 ± 0.92 | 0.0 ± 0.88 | +0.1 ± 1.03 | −1.9 ± 1.22 | −0.2 ± 0.95 | −1.4 ± 1.01 |
| Seed 24, first mapping, mirrored | 18 | −0.5 ± 1.17 | +0.2 ± 0.97 | −0.6 ± 1.16 | +0.3 ± 1.04 | −0.3 ± 1.09 | −1.5 ± 0.92 | −0.3 ± 1.09 | −1.4 ± 1.02 |
| Seed 24, first mapping, mirrored | 24 | −0.4 ± 1.02 | +0.3 ± 1.16 | +0.2 ± 0.98 | +1.0 ± 1.14 | −0.7 ± 1.02 | −1.1 ± 1.18 | −0.1 ± 0.95 | −0.4 ± 1.13 |
| Seed 24, first mapping, mirrored | 36 | +0.4 ± 0.55 | +0.7 ± 0.95 | +0.1 ± 0.46 | +1.6 ± 0.86 | +0.1 ± 0.64 | −0.4 ± 0.91 | −0.2 ± 0.59 | +0.4 ± 0.85 |
| Seed 23, first mapping, mirrored | 6 | +0.1 ± 0.99 | −0.6 ± 0.94 | 0.0 ± 0.98 | −0.4 ± 0.87 | −0.1 ± 0.95 | −0.1 ± 0.94 | −0.1 ± 0.94 | +0.1 ± 0.89 |
| Seed 23, first mapping, mirrored | 12 | +0.6 ± 0.90 | +0.3 ± 1.27 | +0.2 ± 0.89 | +0.2 ± 1.41 | −0.1 ± 0.98 | −0.1 ± 1.20 | −0.5 ± 0.93 | −0.2 ± 1.24 |
| Seed 23, first mapping, mirrored | 18 | +0.4 ± 1.13 | −0.2 ± 1.00 | +0.9 ± 1.19 | +0.5 ± 1.00 | −0.6 ± 0.94 | −1.0 ± 1.07 | −0.1 ± 0.95 | −0.3 ± 1.13 |
| Seed 23, first mapping, mirrored | 24 | +1.4 ± 0.99 | +0.6 ± 1.16 | +1.3 ± 1.50 | +0.4 ± 1.05 | −0.4 ± 1.02 | −0.1 ± 1.06 | −0.5 ± 1.26 | −0.3 ± 0.96 |
| Seed 23, first mapping, mirrored | 36 | +1.4 ± 1.00 | +0.9 ± 1.21 | +1.5 ± 1.47 | +0.8 ± 0.84 | −0.7 ± 1.13 | −0.3 ± 1.03 | −0.6 ± 0.93 | −0.4 ± 0.85 |
| Seed 24, eighth, assignment | 6 | −0.4 ± 0.80 | +1.0 ± 0.91 | −0.4 ± 0.77 | −0.3 ± 0.92 | −0.1 ± 0.91 | +0.5 ± 0.98 | −0.2 ± 0.83 | −0.8 ± 0.97 |
| Seed 24, eighth, assignment | 12 | +0.3 ± 1.15 | +0.7 ± 1.17 | +1.2 ± 0.94 | −0.9 ± 0.97 | −1.1 ± 1.03 | −0.5 ± 1.21 | −0.2 ± 1.13 | −2.1 ± 1.06 |
| Seed 24, eighth, assignment | 18 | +0.8 ± 1.06 | +2.5 ± 1.18 | +1.9 ± 0.93 | +1.2 ± 1.00 | −3.0 ± 1.18 | −0.9 ± 1.30 | −1.9 ± 1.29 | −2.1 ± 1.02 |
| Seed 24, eighth, assignment | 24 | +0.4 ± 1.03 | +0.9 ± 1.12 | +0.7 ± 1.12 | +2.0 ± 1.05 | −1.9 ± 1.77 | −3.9 ± 1.23 | −1.5 ± 1.13 | −2.7 ± 1.02 |
| Seed 24, eighth, assignment | 36 | +1.3 ± 0.85 | −0.4 ± 1.53 | +2.9 ± 0.76 | −0.6 ± 1.61 | −3.0 ± 1.39 | −3.0 ± 1.22 | −1.5 ± 0.89 | −3.2 ± 1.32 |
| Seed 22, first mapping, mirrored | 6 | 0.0 ± 0.85 | +0.1 ± 0.70 | 0.0 ± 0.84 | +0.5 ± 0.71 | −0.1 ± 0.86 | −0.1 ± 0.79 | −0.1 ± 0.84 | +0.2 ± 0.76 |
| Seed 22, first mapping, mirrored | 12 | −0.3 ± 1.10 | +0.2 ± 1.04 | −0.3 ± 1.08 | +0.9 ± 1.16 | +0.1 ± 1.14 | −0.5 ± 1.10 | +0.1 ± 1.07 | +0.2 ± 1.26 |
| Seed 22, first mapping, mirrored | 18 | 0.0 ± 0.94 | −0.2 ± 1.13 | +0.4 ± 1.03 | +0.2 ± 1.05 | −0.1 ± 0.90 | +0.1 ± 1.20 | +0.3 ± 0.92 | +0.6 ± 1.12 |
| Seed 22, first mapping, mirrored | 24 | 0.0 ± 0.85 | +0.2 ± 1.59 | +0.5 ± 0.71 | +0.6 ± 1.62 | +0.1 ± 0.70 | 0.0 ± 1.46 | +0.6 ± 0.69 | +0.4 ± 1.48 |
| Seed 22, first mapping, mirrored | 36 | −0.3 ± 0.97 | −0.1 ± 0.89 | +0.3 ± 0.90 | +0.3 ± 1.45 | +0.3 ± 0.98 | −0.3 ± 0.96 | +0.8 ± 1.11 | +0.1 ± 0.98 |
| Seed 22, eighth, assignment | 6 | +0.1 ± 0.70 | −0.1 ± 1.45 | +0.1 ± 0.78 | +0.3 ± 1.28 | +0.1 ± 0.72 | −0.4 ± 1.51 | +0.1 ± 0.74 | 0.0 ± 1.27 |
| Seed 22, eighth, assignment | 12 | +0.1 ± 0.67 | +0.3 ± 1.08 | +0.8 ± 0.71 | +1.4 ± 1.09 | −0.9 ± 0.67 | −0.8 ± 1.03 | −0.2 ± 0.74 | +0.3 ± 1.04 |
| Seed 22, eighth, assignment | 18 | +1.2 ± 1.07 | −0.3 ± 0.56 | +1.1 ± 1.27 | +0.2 ± 0.69 | −0.1 ± 1.07 | −0.3 ± 0.65 | −0.2 ± 1.05 | +0.2 ± 0.73 |
| Seed 22, eighth, assignment | 24 | −0.2 ± 1.07 | +0.5 ± 1.07 | +0.4 ± 1.17 | +2.0 ± 0.97 | −0.4 ± 1.19 | −1.3 ± 1.02 | +0.2 ± 1.20 | +0.3 ± 1.11 |
| Seed 22, eighth, assignment | 36 | +0.9 ± 1.21 | +0.1 ± 1.44 | +0.5 ± 1.40 | +1.9 ± 1.26 | +0.1 ± 1.27 | −1.5 ± 1.23 | −0.3 ± 1.18 | +0.2 ± 1.07 |

- **Under the night held across less the plain night**, A's trials lie between −3.1 and +3.3, beyond 2 SE at 8 of the 60 checkpoints: below
  on seed 23's eighth image from the mirrored assignment at blocks 12, 18 and 24, −2.2 ± 0.95 to −3.1 ± 0.99; above on seed 23's eighth
  image from the assignment from block 18 on, +2.3 ± 0.92 to +3.3 ± 0.96, on seed 22's eighth image from the mirrored assignment at block
  18, +2.4 ± 1.10, and on seed 24's eighth image from the mirrored assignment at block 18, +2.2 ± 0.94. B's lie between −1.3 and +3.1,
  beyond 2 SE at 2, both above: on seed 24's first mapping from the assignment at block 36, +3.1 ± 0.89, and on seed 24's eighth image from
  the assignment at block 18, +2.5 ± 1.18.
- **Under the night held within less the plain night**, A's trials lie between −4.4 and +7.5, beyond 2 SE at 13 of the 60: below on seed
  23's eighth image from the mirrored assignment from block 18 on, −2.2 ± 1.06 to −4.4 ± 1.07; above on seed 24's eighth image from the
  mirrored assignment from block 12 on, +4.3 ± 1.11 to +7.5 ± 1.21, on seed 23's eighth image from the assignment from block 18 on,
  +3.2 ± 1.03 to +5.7 ± 1.04, on seed 24's eighth image from the assignment at blocks 18 and 36, +1.9 ± 0.93 and +2.9 ± 0.76, and on seed
  24's first mapping from the assignment at block 36, +3.1 ± 1.23. B's lie between −2.8 and +2.7, beyond 2 SE at 5: below on seed 22's
  eighth image from the mirrored assignment at blocks 24 and 36 and on seed 24's eighth image from the mirrored assignment at block 36;
  above on seed 23's eighth image from the assignment at block 24 and on seed 22's eighth image from the assignment at block 24.
- **Under the night held across less the excitatory night**, A's trials lie between −5.3 and +1.7, beyond 2 SE at 9 of the 60, all below:
  on seed 24's eighth image from the mirrored assignment from block 12 on, −2.7 ± 0.89 to −5.3 ± 0.93; on seed 23's eighth image from the
  assignment at blocks 24 and 36; on seed 24's eighth image from the assignment at blocks 18 and 36; and on seed 24's first mapping from the
  assignment at block 24. B's lie between −3.9 and +4.1, beyond 2 SE at 8: above on seed 22's eighth image from the mirrored assignment and
  on seed 24's first mapping from the assignment, each at blocks 24 and 36; below on seed 23's eighth image from the assignment at blocks 18
  and 24 and on seed 24's eighth image from the assignment at blocks 24 and 36.
- **Under the night held within less the excitatory night**, A's trials lie between −1.9 and +1.5, within 2 SE at every one of the 60
  checkpoints. B's lie between −3.2 and +1.8, beyond 2 SE at 4, all below and all on seed 24's eighth image from the assignment, from block
  12 on, −2.1 ± 1.06 to −3.2 ± 1.32.

## 3. The pair and A's couplings

The pair's count under each held form at each checkpoint, and at block 36 under the other forms. H-47's floor for clause 2 is 56:

| Cell | Held across at 6 | 12 | 18 | 24 | 36 | Held within at 6 | 12 | 18 | 24 | 36 | At 36: without replay | plain on A's | excitatory |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 45.0 | 62.9 | 62.7 | 62.9 | 62.9 | 45.3 | 62.9 | 62.5 | 62.7 | 62.9 | 63.0 | 62.8 | 62.9 |
| Seed 22, first mapping, assignment | 42.8 | 61.7 | 61.9 | 62.7 | 63.0 | 41.6 | 61.5 | 61.1 | 61.8 | 62.5 | 63.3 | 63.1 | 62.7 |
| Seed 23, eighth, mirrored | 57.9 | 61.4 | 62.2 | 61.8 | 62.9 | 58.3 | 61.8 | 61.8 | 61.8 | 62.6 | 62.6 | 63.1 | 62.8 |
| Seed 23, first mapping, assignment | 27.5 | 56.4 | 60.9 | 60.1 | 62.1 | 26.9 | 55.0 | 60.9 | 60.1 | 61.7 | 63.1 | 61.9 | 61.9 |
| Seed 24, eighth, mirrored | 61.7 | 63.0 | 62.0 | 62.0 | 62.6 | 61.6 | 62.7 | 62.3 | 62.1 | 63.0 | 62.7 | 63.0 | 63.0 |
| Seed 24, first mapping, assignment | 34.8 | 50.9 | 60.0 | 59.9 | 62.1 | 34.5 | 51.3 | 58.9 | 57.7 | 61.3 | 62.1 | 61.8 | 61.7 |
| Seed 23, eighth, assignment | 54.7 | 62.1 | 61.8 | 63.0 | 63.3 | 55.0 | 62.0 | 62.2 | 63.0 | 63.1 | 63.4 | 63.4 | 62.8 |
| Seed 24, first mapping, mirrored | 51.4 | 61.8 | 61.8 | 62.1 | 62.6 | 52.2 | 61.5 | 61.7 | 61.5 | 62.1 | 63.4 | 62.4 | 62.4 |
| Seed 23, first mapping, mirrored | 49.0 | 59.4 | 62.1 | 63.1 | 63.4 | 49.0 | 59.7 | 62.0 | 62.2 | 63.2 | 63.5 | 63.2 | 63.0 |
| Seed 24, eighth, assignment | 61.4 | 62.3 | 62.4 | 62.2 | 63.1 | 61.5 | 62.3 | 62.7 | 62.1 | 62.7 | 62.6 | 62.9 | 62.6 |
| Seed 22, first mapping, mirrored | 54.1 | 61.1 | 62.3 | 61.6 | 62.5 | 54.8 | 61.2 | 62.0 | 61.7 | 62.6 | 63.1 | 62.5 | 62.9 |
| Seed 22, eighth, assignment | 63.2 | 62.7 | 63.1 | 62.8 | 63.1 | 63.0 | 62.4 | 62.5 | 61.5 | 62.8 | 62.6 | 62.9 | 62.9 |

A's highest coupling over H-42's under each form, in any image read and at any block's end, as clause 3 reads it; its bound is 1.30:

| Cell | Without replay | Held across | Held within | Plain on A's (H-47) | Excitatory (025, 026) |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 1.144 | 1.199 | 1.202 | 1.197 | 1.201 |
| Seed 22, first mapping, assignment | 1.112 | 1.199 | 1.205 | 1.198 | 1.202 |
| Seed 23, eighth, mirrored | 1.147 | 1.210 | 1.211 | 1.211 | 1.212 |
| Seed 23, first mapping, assignment | 1.125 | 1.211 | 1.216 | 1.212 | 1.212 |
| Seed 24, eighth, mirrored | 1.078 | 1.140 | 1.145 | 1.141 | 1.146 |
| Seed 24, first mapping, assignment | 1.085 | 1.156 | 1.170 | 1.162 | 1.165 |
| Seed 23, eighth, assignment | 1.118 | 1.168 | 1.167 | 1.171 | 1.162 |
| Seed 24, first mapping, mirrored | 1.177 | 1.225 | 1.236 | 1.223 | 1.236 |
| Seed 23, first mapping, mirrored | 1.082 | 1.158 | 1.153 | 1.158 | 1.151 |
| Seed 24, eighth, assignment | 1.179 | 1.191 | 1.184 | 1.200 | 1.196 |
| Seed 22, first mapping, mirrored | 1.114 | 1.184 | 1.192 | 1.182 | 1.183 |
| Seed 22, eighth, assignment | 1.133 | 1.181 | 1.197 | 1.184 | 1.193 |

At block 36 the pair stands at 62.1 to 63.4 under the night held across and at 61.3 to 63.2 under the night held within, where H-47's plain
night gave 61.8 to 63.4, the night without replay 62.1 to 63.5 and the excitatory night 61.7 to 63.0. A's highest coupling is 1.140 to
1.225 of H-42's under the night held across and 1.145 to 1.236 under the night held within, where H-47's plain night gave 1.141 to 1.223,
the excitatory night 1.146 to 1.236 and the night without replay 1.078 to 1.179. The H-42 couplings each job reads against are the same in
this job, in H-47's and in the excitatory night's, in every cell.

## 4. What each night moved

What the four nights moved, summed over the four, the mean over the three paths, from the class matrix in the images before and after each
night, in M. A group's held units are its readout's two classes, the pair's own kind left out; "onto their own group" counts the pair's own
kind among the targets. The column marked held across is the part held across; the other columns are the part held within. H-47's plain
night is exploration 021's run of exploration 020's job for exploration 026's eight cells and exploration 020's runs for exploration 020's
four, and the excitatory night's rows read 0.00, since it moved none of these:

| Cell | Form | The odd places' held units onto their own group | onto the even places' excitatory (held across) | onto the inhibitory units | onto A's and B's units | onto the other excitatory | The even places' held units onto their own group | onto the odd places' excitatory (held across) | onto the inhibitory units | onto A's and B's units | onto the other excitatory |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | without replay | −0.50 | −0.47 | +1.28 | −0.02 | +0.05 | −0.74 | −0.65 | +1.02 | −0.18 | −0.03 |
| Seed 22, eighth, mirrored | held across | −0.98 | 0.00 | −2.31 | −0.54 | −0.01 | −1.19 | 0.00 | −2.08 | −0.92 | −0.01 |
| Seed 22, eighth, mirrored | held within | 0.00 | −5.94 | 0.00 | 0.00 | 0.00 | 0.00 | −6.62 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, mirrored | plain on A's (H-47) | −0.66 | −5.98 | −2.19 | −0.40 | 0.00 | −0.92 | −6.72 | −1.96 | −0.74 | −0.01 |
| Seed 22, eighth, mirrored | excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, assignment | without replay | −0.45 | −0.52 | +1.49 | −0.04 | +0.04 | −0.75 | −0.61 | +1.41 | −0.24 | −0.03 |
| Seed 22, first mapping, assignment | held across | −0.87 | 0.00 | −1.97 | −0.33 | 0.00 | −0.97 | 0.00 | −1.67 | −0.85 | −0.01 |
| Seed 22, first mapping, assignment | held within | 0.00 | −5.60 | 0.00 | 0.00 | 0.00 | 0.00 | −6.27 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, assignment | plain on A's (H-47) | −0.73 | −5.66 | −2.02 | −0.25 | +0.01 | −0.76 | −6.32 | −1.56 | −0.73 | −0.01 |
| Seed 22, first mapping, assignment | excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, mirrored | without replay | +0.23 | −0.01 | +1.35 | −0.04 | −0.03 | −0.20 | −0.59 | +1.34 | −0.19 | 0.00 |
| Seed 23, eighth, mirrored | held across | −0.83 | 0.00 | −2.87 | −0.76 | 0.00 | −1.52 | 0.00 | −2.21 | −0.71 | −0.04 |
| Seed 23, eighth, mirrored | held within | 0.00 | −6.75 | 0.00 | 0.00 | 0.00 | 0.00 | −6.67 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, mirrored | plain on A's (H-47) | −0.56 | −6.98 | −2.71 | −0.59 | +0.01 | −1.17 | −6.55 | −2.07 | −0.47 | −0.03 |
| Seed 23, eighth, mirrored | excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, assignment | without replay | +0.12 | −0.17 | +1.53 | +0.04 | −0.03 | −0.30 | −0.38 | +1.53 | −0.27 | 0.00 |
| Seed 23, first mapping, assignment | held across | −0.27 | 0.00 | −2.39 | −0.33 | 0.00 | −1.03 | 0.00 | −1.66 | −0.55 | −0.06 |
| Seed 23, first mapping, assignment | held within | 0.00 | −6.25 | 0.00 | 0.00 | 0.00 | 0.00 | −5.71 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, assignment | plain on A's (H-47) | −0.02 | −6.24 | −2.31 | −0.18 | 0.00 | −0.80 | −5.79 | −1.55 | −0.35 | −0.06 |
| Seed 23, first mapping, assignment | excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, mirrored | without replay | −0.39 | +0.07 | +1.08 | −0.07 | +0.07 | −0.78 | −0.31 | +0.88 | −0.11 | +0.10 |
| Seed 24, eighth, mirrored | held across | −1.04 | 0.00 | −2.51 | −0.37 | −0.04 | −1.59 | 0.00 | −1.92 | −1.09 | +0.05 |
| Seed 24, eighth, mirrored | held within | 0.00 | −5.40 | 0.00 | 0.00 | 0.00 | 0.00 | −6.84 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, mirrored | plain on A's (H-47) | −0.90 | −5.39 | −2.43 | −0.25 | −0.05 | −1.39 | −6.95 | −1.82 | −0.97 | +0.06 |
| Seed 24, eighth, mirrored | excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, assignment | without replay | −0.37 | +0.14 | +1.23 | +0.19 | +0.14 | −0.46 | −0.03 | +1.42 | −0.07 | +0.10 |
| Seed 24, first mapping, assignment | held across | −0.71 | 0.00 | −2.09 | −0.02 | −0.02 | −1.31 | 0.00 | −1.45 | −0.95 | +0.05 |
| Seed 24, first mapping, assignment | held within | 0.00 | −5.20 | 0.00 | 0.00 | 0.00 | 0.00 | −5.93 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, assignment | plain on A's (H-47) | −0.62 | −5.10 | −1.98 | +0.12 | −0.03 | −1.17 | −6.04 | −1.45 | −0.87 | +0.05 |
| Seed 24, first mapping, assignment | excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, assignment | without replay | +0.49 | −0.17 | +1.19 | −0.03 | −0.03 | −0.15 | −0.57 | +1.10 | −0.23 | −0.01 |
| Seed 23, eighth, assignment | held across | −2.60 | 0.00 | −2.52 | −0.93 | −0.01 | −2.43 | 0.00 | −2.25 | −0.87 | −0.02 |
| Seed 23, eighth, assignment | held within | 0.00 | −6.29 | 0.00 | 0.00 | 0.00 | 0.00 | −6.06 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, assignment | plain on A's (H-47) | −2.38 | −6.32 | −2.44 | −0.82 | −0.01 | −2.18 | −5.94 | −2.12 | −0.70 | −0.02 |
| Seed 23, eighth, assignment | excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, mirrored | without replay | −0.39 | +0.25 | +1.09 | +0.19 | +0.14 | −0.58 | −0.05 | +1.38 | −0.01 | +0.10 |
| Seed 24, first mapping, mirrored | held across | −1.63 | 0.00 | −2.12 | −0.92 | 0.00 | −2.51 | 0.00 | −2.31 | −0.73 | +0.02 |
| Seed 24, first mapping, mirrored | held within | 0.00 | −5.34 | 0.00 | 0.00 | 0.00 | 0.00 | −5.57 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, mirrored | plain on A's (H-47) | −1.60 | −5.34 | −2.09 | −0.79 | −0.03 | −2.39 | −5.55 | −2.18 | −0.60 | +0.01 |
| Seed 24, first mapping, mirrored | excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, mirrored | without replay | +0.19 | −0.15 | +1.35 | +0.12 | −0.03 | −0.25 | −0.54 | +1.25 | −0.26 | −0.01 |
| Seed 23, first mapping, mirrored | held across | −2.26 | 0.00 | −2.12 | −0.79 | −0.02 | −1.85 | 0.00 | −2.03 | −0.62 | +0.01 |
| Seed 23, first mapping, mirrored | held within | 0.00 | −5.85 | 0.00 | 0.00 | 0.00 | 0.00 | −5.56 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, mirrored | plain on A's (H-47) | −2.02 | −5.68 | −2.03 | −0.67 | −0.02 | −1.66 | −5.56 | −1.94 | −0.52 | 0.00 |
| Seed 23, first mapping, mirrored | excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, assignment | without replay | −0.50 | +0.10 | +1.05 | −0.04 | +0.08 | −0.72 | −0.36 | +0.99 | −0.07 | +0.09 |
| Seed 24, eighth, assignment | held across | −1.99 | 0.00 | −2.36 | −1.12 | −0.06 | −2.88 | 0.00 | −2.64 | −0.90 | +0.04 |
| Seed 24, eighth, assignment | held within | 0.00 | −6.08 | 0.00 | 0.00 | 0.00 | 0.00 | −6.23 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, assignment | plain on A's (H-47) | −1.85 | −6.12 | −2.35 | −0.96 | −0.04 | −2.71 | −6.23 | −2.59 | −0.79 | +0.04 |
| Seed 24, eighth, assignment | excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, mirrored | without replay | −0.41 | −0.42 | +1.38 | −0.02 | +0.03 | −0.78 | −0.55 | +1.36 | −0.20 | −0.02 |
| Seed 22, first mapping, mirrored | held across | −2.56 | 0.00 | −1.69 | −0.46 | +0.01 | −2.40 | 0.00 | −2.17 | −0.34 | −0.02 |
| Seed 22, first mapping, mirrored | held within | 0.00 | −4.80 | 0.00 | 0.00 | 0.00 | 0.00 | −5.98 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, mirrored | plain on A's (H-47) | −2.39 | −4.73 | −1.65 | −0.34 | +0.01 | −2.30 | −6.01 | −2.09 | −0.23 | −0.02 |
| Seed 22, first mapping, mirrored | excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, assignment | without replay | −0.52 | −0.37 | +0.77 | −0.01 | +0.04 | −0.83 | −0.68 | +1.15 | −0.15 | −0.03 |
| Seed 22, eighth, assignment | held across | −2.77 | 0.00 | −2.28 | −0.62 | +0.04 | −2.96 | 0.00 | −2.35 | −0.53 | −0.01 |
| Seed 22, eighth, assignment | held within | 0.00 | −5.71 | 0.00 | 0.00 | 0.00 | 0.00 | −6.56 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, assignment | plain on A's (H-47) | −2.49 | −5.68 | −2.05 | −0.46 | +0.03 | −2.79 | −6.56 | −2.27 | −0.41 | −0.01 |
| Seed 22, eighth, assignment | excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |

- **The part each night holds shows 0.00**, and its class matrix entries moved by exactly nothing, in LSB, in every one of the 288 held
  nights (below): under the night held across the held units' synapses onto the other group's excitatory units, under the night held
  within all their other synapses.
- **Under the night held within, the part held across moved as under H-47's plain night**: −4.80 to −6.84 M a group over four nights,
  where the plain night moved it by −4.73 to −6.98 M; less the plain night's, −0.17 to +0.22 M, 0.97 to 1.03 times it, group by group.
- **Under the night held across, the part held within moved by more than under H-47's plain night**: −2.85 to −6.38 M a group over four
  nights, where the plain night moved it by −2.50 to −6.05 M; less the plain night's, −0.74 to −0.17 M, 1.04 to 1.20 times it, group by
  group. Onto their own group the held units' synapses moved by −0.27 to −2.96 M, where the plain night's moved by −0.02 to −2.79 M; onto
  the inhibitory units by −1.45 to −2.87 M, where the plain night's moved by −1.45 to −2.71 M; onto A's and B's units by −0.02 to −1.12 M,
  where the plain night's moved by −0.97 to +0.12 M; and onto the other excitatory units by −0.06 to +0.05 M.
- Under the night without replay the part held across moved by −0.68 to +0.25 M a group and the part held within by +0.06 to +1.67 M.

The rest of each night's change: the inhibition onto each group's excitatory units and onto the other targets, and the excitatory units
outside the hold onto every target:

| Cell | Form | Inhibitory onto the odd places' excitatory units | onto the even places' | onto the inhibitory units | onto A's and B's units | onto the other excitatory | C's own onto all | D's own onto all | A's and B's units onto all | The other excitatory onto all |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | without replay | +1.10 | +0.70 | +0.66 | −1.17 | +0.03 | −0.04 | +0.01 | −0.02 | +0.11 |
| Seed 22, eighth, mirrored | held across | +14.92 | +12.90 | +4.52 | +0.09 | +0.03 | −0.09 | −0.05 | −0.04 | −0.10 |
| Seed 22, eighth, mirrored | held within | +15.03 | +12.79 | +4.54 | +0.14 | +0.04 | −0.05 | −0.02 | −0.04 | −0.15 |
| Seed 22, eighth, mirrored | plain on A's (H-47) | +14.23 | +12.21 | +4.39 | +0.09 | +0.04 | −0.05 | −0.02 | −0.04 | −0.10 |
| Seed 22, eighth, mirrored | excitatory (025, 026) | +15.87 | +13.70 | +4.79 | +0.14 | +0.04 | −0.08 | −0.06 | −0.04 | −0.15 |
| Seed 22, first mapping, assignment | without replay | −1.89 | −2.20 | −0.35 | −0.97 | +0.04 | −0.04 | +0.01 | −0.02 | +0.13 |
| Seed 22, first mapping, assignment | held across | +9.74 | +8.48 | +3.62 | +1.01 | +0.02 | −0.07 | −0.09 | −0.06 | −0.06 |
| Seed 22, first mapping, assignment | held within | +9.89 | +8.48 | +3.66 | +1.04 | +0.03 | −0.06 | −0.07 | −0.07 | −0.09 |
| Seed 22, first mapping, assignment | plain on A's (H-47) | +9.29 | +7.95 | +3.43 | +0.99 | +0.02 | −0.06 | −0.08 | −0.07 | −0.06 |
| Seed 22, first mapping, assignment | excitatory (025, 026) | +10.28 | +8.98 | +3.89 | +1.08 | +0.03 | −0.07 | −0.08 | −0.06 | −0.10 |
| Seed 23, eighth, mirrored | without replay | +1.13 | +0.88 | +0.79 | −1.02 | +0.02 | 0.00 | −0.05 | −0.03 | −0.02 |
| Seed 23, eighth, mirrored | held across | +15.19 | +13.69 | +4.73 | +0.14 | +0.05 | −0.08 | −0.04 | −0.07 | −0.23 |
| Seed 23, eighth, mirrored | held within | +15.50 | +13.55 | +4.91 | +0.20 | +0.06 | −0.05 | −0.01 | −0.08 | −0.24 |
| Seed 23, eighth, mirrored | plain on A's (H-47) | +14.40 | +12.88 | +4.55 | +0.13 | +0.05 | −0.05 | −0.03 | −0.07 | −0.24 |
| Seed 23, eighth, mirrored | excitatory (025, 026) | +16.32 | +14.41 | +5.18 | +0.21 | +0.05 | −0.08 | −0.02 | −0.07 | −0.23 |
| Seed 23, first mapping, assignment | without replay | −1.65 | −1.99 | −0.59 | −0.87 | −0.03 | −0.03 | −0.05 | −0.04 | −0.03 |
| Seed 23, first mapping, assignment | held across | +10.36 | +8.62 | +3.25 | +1.19 | +0.01 | −0.09 | −0.08 | −0.09 | −0.13 |
| Seed 23, first mapping, assignment | held within | +10.44 | +8.52 | +3.47 | +1.23 | 0.00 | −0.06 | −0.06 | −0.09 | −0.15 |
| Seed 23, first mapping, assignment | plain on A's (H-47) | +9.88 | +8.08 | +3.09 | +1.16 | +0.01 | −0.07 | −0.06 | −0.09 | −0.15 |
| Seed 23, first mapping, assignment | excitatory (025, 026) | +10.88 | +9.08 | +3.68 | +1.25 | 0.00 | −0.09 | −0.07 | −0.09 | −0.13 |
| Seed 24, eighth, mirrored | without replay | +0.93 | +0.94 | +0.52 | −1.00 | +0.02 | −0.02 | −0.05 | −0.03 | −0.13 |
| Seed 24, eighth, mirrored | held across | +15.19 | +11.67 | +4.02 | +0.06 | +0.01 | +0.03 | −0.04 | −0.09 | −0.20 |
| Seed 24, eighth, mirrored | held within | +15.44 | +11.65 | +4.03 | +0.14 | +0.02 | +0.05 | 0.00 | −0.08 | −0.24 |
| Seed 24, eighth, mirrored | plain on A's (H-47) | +14.61 | +11.20 | +3.89 | +0.05 | 0.00 | +0.06 | −0.02 | −0.08 | −0.22 |
| Seed 24, eighth, mirrored | excitatory (025, 026) | +15.90 | +12.12 | +4.11 | +0.15 | +0.02 | +0.04 | −0.02 | −0.08 | −0.24 |
| Seed 24, first mapping, assignment | without replay | −1.73 | −1.86 | −0.73 | −0.86 | +0.03 | 0.00 | −0.06 | −0.05 | −0.17 |
| Seed 24, first mapping, assignment | held across | +10.83 | +7.12 | +2.81 | +1.04 | +0.01 | +0.03 | −0.03 | −0.10 | −0.23 |
| Seed 24, first mapping, assignment | held within | +11.02 | +7.11 | +2.87 | +1.11 | +0.04 | +0.05 | 0.00 | −0.09 | −0.20 |
| Seed 24, first mapping, assignment | plain on A's (H-47) | +10.49 | +6.77 | +2.76 | +1.03 | +0.01 | +0.07 | −0.01 | −0.10 | −0.23 |
| Seed 24, first mapping, assignment | excitatory (025, 026) | +11.39 | +7.46 | +3.01 | +1.13 | +0.04 | +0.03 | −0.02 | −0.09 | −0.22 |
| Seed 23, eighth, assignment | without replay | +1.17 | +1.02 | +0.67 | −1.03 | +0.02 | −0.06 | −0.05 | −0.03 | −0.02 |
| Seed 23, eighth, assignment | held across | +12.73 | +14.03 | +4.10 | +0.09 | +0.01 | −0.05 | −0.08 | −0.06 | −0.19 |
| Seed 23, eighth, assignment | held within | +13.13 | +14.24 | +4.20 | +0.15 | +0.02 | −0.05 | −0.06 | −0.06 | −0.18 |
| Seed 23, eighth, assignment | plain on A's (H-47) | +12.07 | +13.48 | +3.92 | +0.10 | +0.02 | −0.05 | −0.06 | −0.06 | −0.18 |
| Seed 23, eighth, assignment | excitatory (025, 026) | +13.82 | +14.93 | +4.43 | +0.15 | +0.02 | −0.05 | −0.08 | −0.07 | −0.18 |
| Seed 24, first mapping, mirrored | without replay | −1.77 | −1.92 | −0.73 | −0.87 | +0.03 | +0.01 | −0.06 | −0.05 | −0.17 |
| Seed 24, first mapping, mirrored | held across | +8.14 | +8.18 | +2.45 | +1.18 | +0.07 | −0.05 | −0.01 | −0.10 | −0.26 |
| Seed 24, first mapping, mirrored | held within | +8.26 | +8.47 | +2.71 | +1.24 | +0.07 | −0.02 | −0.01 | −0.10 | −0.25 |
| Seed 24, first mapping, mirrored | plain on A's (H-47) | +7.72 | +7.83 | +2.32 | +1.18 | +0.06 | −0.03 | −0.01 | −0.10 | −0.26 |
| Seed 24, first mapping, mirrored | excitatory (025, 026) | +8.60 | +8.80 | +2.81 | +1.24 | +0.07 | −0.04 | −0.02 | −0.10 | −0.24 |
| Seed 23, first mapping, mirrored | without replay | −1.79 | −2.04 | −0.59 | −0.90 | −0.04 | −0.01 | −0.05 | −0.04 | −0.02 |
| Seed 23, first mapping, mirrored | held across | +8.72 | +9.16 | +3.08 | +1.08 | −0.03 | −0.04 | −0.05 | −0.07 | −0.11 |
| Seed 23, first mapping, mirrored | held within | +8.96 | +9.22 | +3.18 | +1.10 | −0.03 | −0.06 | −0.05 | −0.07 | −0.11 |
| Seed 23, first mapping, mirrored | plain on A's (H-47) | +8.33 | +8.77 | +2.94 | +1.07 | −0.03 | −0.04 | −0.04 | −0.07 | −0.12 |
| Seed 23, first mapping, mirrored | excitatory (025, 026) | +9.41 | +9.56 | +3.37 | +1.10 | −0.03 | −0.06 | −0.05 | −0.08 | −0.11 |
| Seed 24, eighth, assignment | without replay | +0.98 | +0.78 | +0.54 | −0.99 | +0.02 | −0.01 | −0.05 | −0.03 | −0.13 |
| Seed 24, eighth, assignment | held across | +11.71 | +12.13 | +3.72 | +0.20 | +0.04 | +0.07 | −0.07 | −0.10 | −0.22 |
| Seed 24, eighth, assignment | held within | +12.19 | +12.85 | +3.98 | +0.25 | +0.05 | +0.08 | −0.04 | −0.10 | −0.26 |
| Seed 24, eighth, assignment | plain on A's (H-47) | +11.38 | +11.76 | +3.66 | +0.22 | +0.04 | +0.09 | −0.05 | −0.10 | −0.21 |
| Seed 24, eighth, assignment | excitatory (025, 026) | +12.61 | +13.27 | +4.07 | +0.25 | +0.04 | +0.05 | −0.07 | −0.10 | −0.26 |
| Seed 22, first mapping, mirrored | without replay | −1.92 | −2.23 | −0.39 | −0.99 | +0.03 | −0.03 | +0.01 | −0.03 | +0.13 |
| Seed 22, first mapping, mirrored | held across | +7.01 | +7.63 | +3.17 | +0.77 | +0.05 | −0.12 | −0.15 | −0.07 | −0.04 |
| Seed 22, first mapping, mirrored | held within | +7.05 | +7.56 | +3.28 | +0.82 | +0.05 | −0.10 | −0.13 | −0.07 | −0.07 |
| Seed 22, first mapping, mirrored | plain on A's (H-47) | +6.52 | +7.22 | +3.05 | +0.70 | +0.06 | −0.10 | −0.13 | −0.08 | −0.04 |
| Seed 22, first mapping, mirrored | excitatory (025, 026) | +7.47 | +7.95 | +3.44 | +0.87 | +0.05 | −0.12 | −0.14 | −0.07 | −0.07 |
| Seed 22, eighth, assignment | without replay | +0.89 | +0.60 | +0.62 | −1.11 | +0.03 | −0.04 | +0.03 | −0.03 | +0.11 |
| Seed 22, eighth, assignment | held across | +11.18 | +11.79 | +3.86 | +0.03 | +0.05 | −0.05 | −0.11 | −0.07 | −0.07 |
| Seed 22, eighth, assignment | held within | +11.34 | +11.90 | +3.93 | +0.05 | +0.05 | −0.05 | −0.10 | −0.06 | −0.11 |
| Seed 22, eighth, assignment | plain on A's (H-47) | +10.61 | +11.30 | +3.72 | 0.00 | +0.05 | −0.03 | −0.09 | −0.07 | −0.07 |
| Seed 22, eighth, assignment | excitatory (025, 026) | +11.88 | +12.35 | +4.11 | +0.08 | +0.05 | −0.07 | −0.10 | −0.05 | −0.12 |

- **The inhibition onto each group's excitatory units rose by 7.01 to 15.19 M** over four nights under the night held across and by 7.05
  to 15.50 M under the night held within, where H-47's plain night raised it by 6.52 to 14.61 M and the excitatory night by 7.46 to
  16.32 M: by 0.33 to 0.81 M more than the plain night under the night held across, 0.33 to 1.11 M under the night held within and 0.69 to
  1.92 M under the excitatory night, group by group. Under every form with replay, in every cell, it rose by more onto the group A answers
  than onto the other: by 0.05 to 3.71 M under the night held across and 0.21 to 3.91 M under the night held within. Onto the inhibitory
  units it rose by 2.45 to 4.73 M and 2.71 to 4.91 M, where the plain night's rose by 2.32 to 4.55 M.
- **The excitatory units outside the hold**: the pair's own units onto all moved by −0.15 to +0.07 M under the night held across and by
  −0.13 to +0.08 M under the night held within, A's and B's units onto all by −0.10 to −0.04 M under each, and the other excitatory units
  by −0.26 to −0.04 M and −0.26 to −0.07 M.

The held part's entries night by night, against what the quiet ticks after each night moved:

| Cell | Form | Nights the held part's entries moved by nothing | Nights they moved by what the quiet ticks moved | Nights the quiet ticks moved held synapses | Nights the other part's entries moved |
| :--- | :--- | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | held across | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 22, eighth, mirrored | held within | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 22, first mapping, assignment | held across | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 22, first mapping, assignment | held within | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 23, eighth, mirrored | held across | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 23, eighth, mirrored | held within | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 23, first mapping, assignment | held across | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 23, first mapping, assignment | held within | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 24, eighth, mirrored | held across | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 24, eighth, mirrored | held within | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 24, first mapping, assignment | held across | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 24, first mapping, assignment | held within | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 23, eighth, assignment | held across | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 23, eighth, assignment | held within | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 24, first mapping, mirrored | held across | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 24, first mapping, mirrored | held within | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 23, first mapping, mirrored | held across | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 23, first mapping, mirrored | held within | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 24, eighth, assignment | held across | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 24, eighth, assignment | held within | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 22, first mapping, mirrored | held across | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 22, first mapping, mirrored | held within | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 22, eighth, assignment | held across | 12 of 12 | 12 of 12 | 0 | 12 of 12 |
| Seed 22, eighth, assignment | held within | 12 of 12 | 12 of 12 | 0 | 12 of 12 |

Each night's change of the engine's two sums, from its `start` line to its `end` line, the mean over the three paths:

| Cell | Gap | Without replay: inhibitory sum, M | excitatory sum | Held across: inhibitory | excitatory | Held within: inhibitory | excitatory | Plain on A's: inhibitory | excitatory | Excitatory: inhibitory | excitatory |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +0.31 | −0.49 | +10.41 | −3.74 | +10.41 | −4.96 | +10.11 | −8.37 | +10.75 | −0.12 |
| Seed 22, eighth, mirrored | 12 | +0.31 | −0.14 | +8.47 | −2.53 | +8.52 | −3.56 | +8.07 | −5.67 | +9.15 | −0.09 |
| Seed 22, eighth, mirrored | 18 | +0.30 | +0.07 | +7.34 | −1.23 | +7.36 | −2.57 | +6.90 | −3.55 | +7.82 | −0.10 |
| Seed 22, eighth, mirrored | 24 | +0.39 | +0.37 | +6.24 | −0.80 | +6.25 | −1.72 | +5.89 | −2.20 | +6.82 | −0.03 |
| Seed 22, first mapping, assignment | 6 | −1.22 | −0.51 | +9.38 | −2.59 | +9.32 | −4.50 | +9.08 | −6.90 | +9.57 | −0.10 |
| Seed 22, first mapping, assignment | 12 | −1.36 | +0.11 | +6.35 | −2.27 | +6.42 | −3.31 | +6.02 | −5.36 | +6.74 | −0.07 |
| Seed 22, first mapping, assignment | 18 | −1.46 | +0.45 | +4.33 | −1.30 | +4.39 | −2.41 | +3.98 | −3.43 | +4.71 | −0.07 |
| Seed 22, first mapping, assignment | 24 | −1.33 | +0.34 | +2.83 | −0.80 | +2.97 | −1.93 | +2.60 | −2.62 | +3.24 | −0.06 |
| Seed 23, eighth, mirrored | 6 | +0.33 | −0.69 | +10.90 | −3.83 | +10.96 | −5.36 | +10.56 | −8.75 | +11.35 | −0.16 |
| Seed 23, eighth, mirrored | 12 | +0.37 | +0.12 | +8.95 | −2.48 | +8.94 | −3.74 | +8.49 | −5.70 | +9.43 | −0.12 |
| Seed 23, eighth, mirrored | 18 | +0.46 | +0.83 | +7.58 | −1.77 | +7.86 | −2.66 | +7.05 | −4.18 | +8.38 | −0.06 |
| Seed 23, eighth, mirrored | 24 | +0.63 | +1.49 | +6.36 | −1.28 | +6.45 | −2.05 | +5.90 | −2.89 | +7.01 | −0.06 |
| Seed 23, first mapping, assignment | 6 | −1.03 | −0.46 | +9.81 | −2.56 | +9.90 | −4.46 | +9.58 | −6.79 | +10.13 | −0.11 |
| Seed 23, first mapping, assignment | 12 | −1.29 | +0.47 | +6.60 | −2.19 | +6.74 | −3.38 | +6.32 | −5.31 | +7.07 | −0.10 |
| Seed 23, first mapping, assignment | 18 | −1.41 | +0.76 | +4.11 | −1.13 | +4.09 | −2.44 | +3.72 | −3.20 | +4.48 | −0.06 |
| Seed 23, first mapping, assignment | 24 | −1.42 | +1.16 | +2.91 | −0.80 | +2.92 | −2.06 | +2.60 | −2.37 | +3.21 | −0.10 |
| Seed 24, eighth, mirrored | 6 | +0.32 | −0.58 | +9.65 | −3.47 | +9.73 | −5.00 | +9.47 | −8.12 | +9.95 | −0.14 |
| Seed 24, eighth, mirrored | 12 | +0.26 | +0.18 | +8.15 | −2.37 | +8.30 | −3.35 | +7.82 | −5.64 | +8.56 | −0.09 |
| Seed 24, eighth, mirrored | 18 | +0.35 | +0.31 | +7.06 | −1.73 | +7.07 | −2.40 | +6.61 | −3.88 | +7.37 | −0.05 |
| Seed 24, eighth, mirrored | 24 | +0.48 | +0.38 | +6.09 | −1.24 | +6.17 | −1.77 | +5.85 | −2.71 | +6.41 | −0.02 |
| Seed 24, first mapping, assignment | 6 | −1.21 | +0.12 | +8.90 | −2.36 | +8.93 | −4.13 | +8.72 | −6.29 | +9.08 | −0.11 |
| Seed 24, first mapping, assignment | 12 | −1.33 | +0.45 | +6.05 | −2.21 | +6.14 | −3.18 | +5.92 | −5.07 | +6.38 | −0.07 |
| Seed 24, first mapping, assignment | 18 | −1.30 | +0.67 | +4.05 | −1.23 | +4.18 | −2.34 | +3.84 | −3.53 | +4.43 | −0.06 |
| Seed 24, first mapping, assignment | 24 | −1.32 | +0.77 | +2.82 | −1.03 | +2.89 | −1.72 | +2.56 | −2.48 | +3.13 | −0.06 |
| Seed 23, eighth, assignment | 6 | +0.36 | −0.82 | +9.96 | −5.03 | +10.11 | −4.87 | +9.72 | −9.49 | +10.37 | −0.16 |
| Seed 23, eighth, assignment | 12 | +0.35 | +0.09 | +8.17 | −3.39 | +8.44 | −3.54 | +7.84 | −6.46 | +8.86 | −0.10 |
| Seed 23, eighth, assignment | 18 | +0.45 | +0.65 | +6.96 | −2.23 | +7.12 | −2.43 | +6.47 | −4.38 | +7.63 | −0.06 |
| Seed 23, eighth, assignment | 24 | +0.68 | +1.52 | +5.87 | −1.35 | +6.08 | −1.86 | +5.54 | −2.94 | +6.50 | −0.05 |
| Seed 24, first mapping, mirrored | 6 | −1.28 | +0.04 | +8.16 | −4.03 | +8.27 | −4.14 | +7.94 | −7.93 | +8.49 | −0.18 |
| Seed 24, first mapping, mirrored | 12 | −1.36 | +0.43 | +5.21 | −3.02 | +5.41 | −2.98 | +4.95 | −5.78 | +5.63 | −0.12 |
| Seed 24, first mapping, mirrored | 18 | −1.30 | +0.68 | +3.74 | −2.08 | +3.95 | −2.30 | +3.55 | −4.10 | +4.10 | −0.05 |
| Seed 24, first mapping, mirrored | 24 | −1.33 | +0.69 | +2.91 | −1.49 | +3.11 | −1.86 | +2.67 | −3.14 | +3.30 | −0.05 |
| Seed 23, first mapping, mirrored | 6 | −1.19 | −0.48 | +9.03 | −3.88 | +9.11 | −4.13 | +8.85 | −7.72 | +9.31 | −0.08 |
| Seed 23, first mapping, mirrored | 12 | −1.40 | +0.37 | +6.11 | −3.05 | +6.11 | −3.21 | +5.81 | −5.84 | +6.46 | −0.07 |
| Seed 23, first mapping, mirrored | 18 | −1.43 | +0.64 | +3.97 | −1.80 | +4.18 | −2.42 | +3.79 | −3.93 | +4.35 | −0.09 |
| Seed 23, first mapping, mirrored | 24 | −1.33 | +1.02 | +2.90 | −1.24 | +3.03 | −1.94 | +2.62 | −2.89 | +3.30 | −0.07 |
| Seed 24, eighth, assignment | 6 | +0.32 | −0.47 | +8.95 | −4.86 | +9.02 | −4.82 | +8.80 | −9.36 | +9.29 | −0.17 |
| Seed 24, eighth, assignment | 12 | +0.25 | +0.10 | +7.42 | −3.45 | +7.81 | −3.50 | +7.16 | −6.68 | +8.07 | −0.08 |
| Seed 24, eighth, assignment | 18 | +0.32 | +0.43 | +6.13 | −2.13 | +6.67 | −2.48 | +5.92 | −4.45 | +6.97 | −0.08 |
| Seed 24, eighth, assignment | 24 | +0.44 | +0.34 | +5.30 | −1.77 | +5.81 | −1.83 | +5.16 | −3.38 | +5.90 | −0.05 |
| Seed 22, first mapping, mirrored | 6 | −1.32 | −0.44 | +7.60 | −3.74 | +7.64 | −4.02 | +7.36 | −7.55 | +7.80 | −0.12 |
| Seed 22, first mapping, mirrored | 12 | −1.39 | +0.09 | +5.10 | −3.04 | +5.12 | −3.03 | +4.75 | −5.75 | +5.47 | −0.12 |
| Seed 22, first mapping, mirrored | 18 | −1.39 | +0.40 | +3.46 | −1.84 | +3.49 | −2.23 | +3.21 | −3.85 | +3.71 | −0.08 |
| Seed 22, first mapping, mirrored | 24 | −1.40 | +0.37 | +2.48 | −1.39 | +2.52 | −1.86 | +2.24 | −2.96 | +2.81 | −0.09 |
| Seed 22, eighth, assignment | 6 | +0.28 | −0.61 | +8.53 | −4.80 | +8.64 | −4.66 | +8.35 | −9.17 | +8.83 | −0.13 |
| Seed 22, eighth, assignment | 12 | +0.23 | −0.11 | +6.92 | −3.27 | +7.15 | −3.47 | +6.67 | −6.30 | +7.48 | −0.12 |
| Seed 22, eighth, assignment | 18 | +0.24 | −0.06 | +6.10 | −2.04 | +6.14 | −2.58 | +5.74 | −4.29 | +6.53 | −0.07 |
| Seed 22, eighth, assignment | 24 | +0.28 | +0.23 | +5.36 | −1.68 | +5.35 | −1.87 | +4.92 | −3.19 | +5.63 | −0.03 |

A night held across, the mean of its three paths, raised the inhibitory sum by 2.48 to 10.90 M and lowered the excitatory sum by 0.80 to
5.03 M; a night held within raised the inhibitory sum by 2.52 to 10.96 M and lowered the excitatory sum by 1.72 to 5.36 M. H-47's plain
night raised the first by 2.24 to 10.56 M and lowered the second by 2.20 to 9.49 M, and the excitatory night raised the first by 2.81 to
11.35 M and lowered the second by 0.02 to 0.18 M.

**The ties**, the tied trials of each cell's 240 reads of A and 240 of the pair, under each form:

| Cell | Without replay, A's and the pair's | H-47's without replay | Held across | Held within | H-47's plain on A's | Excitatory (025, 026) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 1160 and 382 | 1160 and 382 | 1141 and 287 | 1260 and 300 | 1202 and 287 | 1324 and 338 |
| Seed 22, first mapping, assignment | 1141 and 408 | 1141 and 408 | 1145 and 448 | 1128 and 501 | 1183 and 491 | 1213 and 472 |
| Seed 23, eighth, mirrored | 935 and 332 | 935 and 332 | 1190 and 236 | 1076 and 240 | 1042 and 228 | 1090 and 242 |
| Seed 23, first mapping, assignment | 1086 and 483 | 1086 and 483 | 819 and 620 | 785 and 699 | 742 and 618 | 883 and 629 |
| Seed 24, eighth, mirrored | 1137 and 322 | 1137 and 322 | 1331 and 176 | 1426 and 164 | 1372 and 137 | 1403 and 170 |
| Seed 24, first mapping, assignment | 968 and 520 | 968 and 520 | 774 and 516 | 711 and 642 | 835 and 584 | 857 and 588 |
| Seed 23, eighth, assignment | 1049 and 256 | 1049 and 256 | 1087 and 219 | 854 and 246 | 1039 and 213 | 907 and 251 |
| Seed 24, first mapping, mirrored | 1066 and 315 | 1066 and 315 | 879 and 376 | 910 and 374 | 887 and 332 | 835 and 421 |
| Seed 23, first mapping, mirrored | 792 and 293 | 792 and 293 | 833 and 366 | 844 and 365 | 866 and 365 | 826 and 360 |
| Seed 24, eighth, assignment | 1114 and 362 | 1114 and 362 | 1308 and 180 | 1391 and 162 | 1324 and 174 | 1391 and 183 |
| Seed 22, first mapping, mirrored | 943 and 302 | 943 and 302 | 919 and 373 | 833 and 336 | 878 and 357 | 862 and 330 |
| Seed 22, eighth, assignment | 917 and 207 | 917 and 207 | 883 and 110 | 829 and 175 | 898 and 130 | 873 and 124 |

The night without replay's ties are H-47's, read by read. Under the night held across they lie 8 to 148 from H-47's plain night's on A's
reads and 0 to 68 on the pair's; under the night held within 22 to 185 and 0 to 81.

## 5. What the hold leaves

At each held night, over its twelve nights a cell and form (ranges are over the twelve): the synapses of the held part; their traces cleared
at the night's last tick, how many were not zero and what they held above and below zero; the other part's synapses the night moved and
those that carried a trace that is not zero at its last tick; the excitatory synapses from units outside the hold the night moved; the
nights in which a held synapse's presynaptic unit fired at or after the night's last tick, and the nights in which the quiet ticks moved
synapses of the held part:

| Cell | Form | Nights | Held synapses | Traces cleared, not zero | cleared above zero, LSB | below zero | The other part at the night's last tick: synapses moved | with a trace | Excitatory synapses from other units moved | Nights a held synapse's presynaptic unit fired at or after the night's last tick | Nights held synapses moved in the quiet ticks: how many, their change |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | held across | 12 | 7 345 | 4 105 to 4 432 | 99 344 to 148 580 | −524 555 to −419 594 | 13 589 to 13 881 | 7 168 to 7 938 | 4 940 to 4 986 | 0 | none |
| Seed 22, eighth, mirrored | held within | 12 | 13 903 | 8 879 to 9 137 | 600 262 to 694 710 | −929 293 to −766 558 | 7 241 to 7 331 | 3 146 to 3 572 | 4 945 to 4 982 | 0 | none |
| Seed 22, first mapping, assignment | held across | 12 | 7 345 | 3 912 to 4 207 | 97 943 to 136 768 | −430 952 to −378 226 | 13 621 to 13 869 | 6 833 to 7 307 | 4 958 to 4 991 | 0 | none |
| Seed 22, first mapping, assignment | held within | 12 | 13 903 | 8 513 to 8 761 | 599 178 to 685 726 | −849 730 to −667 245 | 7 246 to 7 331 | 2 879 to 3 139 | 4 961 to 4 991 | 0 | none |
| Seed 23, eighth, mirrored | held across | 12 | 7 476 | 4 343 to 4 625 | 121 854 to 146 131 | −532 807 to −448 330 | 13 489 to 13 759 | 7 164 to 8 034 | 4 947 to 4 982 | 3 | none |
| Seed 23, eighth, mirrored | held within | 12 | 13 772 | 8 685 to 9 058 | 560 148 to 664 598 | −991 131 to −790 144 | 7 368 to 7 465 | 3 110 to 3 643 | 4 958 to 4 982 | 3 | none |
| Seed 23, first mapping, assignment | held across | 12 | 7 476 | 4 077 to 4 323 | 116 900 to 137 524 | −446 507 to −406 490 | 13 492 to 13 749 | 7 110 to 7 582 | 4 965 to 4 992 | 0 | none |
| Seed 23, first mapping, assignment | held within | 12 | 13 772 | 8 535 to 8 865 | 574 927 to 665 740 | −861 852 to −707 542 | 7 388 to 7 465 | 2 895 to 3 285 | 4 960 to 4 990 | 0 | none |
| Seed 24, eighth, mirrored | held across | 12 | 7 453 | 4 052 to 4 421 | 117 290 to 150 437 | −516 154 to −462 513 | 13 481 to 13 779 | 7 079 to 7 545 | 4 956 to 4 985 | 1 | none |
| Seed 24, eighth, mirrored | held within | 12 | 13 795 | 8 604 to 8 890 | 595 806 to 661 470 | −878 860 to −775 588 | 7 332 to 7 439 | 3 213 to 3 373 | 4 963 to 4 986 | 0 | none |
| Seed 24, first mapping, assignment | held across | 12 | 7 453 | 3 987 to 4 284 | 108 023 to 133 172 | −486 062 to −364 931 | 13 505 to 13 760 | 6 738 to 7 562 | 4 968 to 4 988 | 0 | none |
| Seed 24, first mapping, assignment | held within | 12 | 13 795 | 8 395 to 8 760 | 539 376 to 721 618 | −874 454 to −731 640 | 7 336 to 7 442 | 2 964 to 3 408 | 4 974 to 4 990 | 0 | none |
| Seed 23, eighth, assignment | held across | 12 | 7 476 | 4 128 to 4 468 | 118 841 to 144 510 | −466 214 to −397 980 | 13 451 to 13 753 | 6 634 to 7 657 | 4 953 to 4 991 | 3 | none |
| Seed 23, eighth, assignment | held within | 12 | 13 772 | 8 570 to 8 824 | 442 293 to 552 099 | −894 560 to −710 164 | 7 389 to 7 464 | 3 066 to 3 601 | 4 956 to 4 989 | 3 | none |
| Seed 24, first mapping, mirrored | held across | 12 | 7 453 | 4 020 to 4 217 | 121 550 to 150 090 | −414 940 to −323 542 | 13 474 to 13 760 | 6 402 to 7 090 | 4 957 to 4 987 | 0 | none |
| Seed 24, first mapping, mirrored | held within | 12 | 13 795 | 8 330 to 8 522 | 475 250 to 553 537 | −815 611 to −654 147 | 7 366 to 7 444 | 2 960 to 3 381 | 4 966 to 4 987 | 0 | none |
| Seed 23, first mapping, mirrored | held across | 12 | 7 476 | 4 004 to 4 390 | 118 990 to 141 145 | −442 963 to −336 554 | 13 489 to 13 743 | 6 519 to 7 217 | 4 952 to 4 989 | 3 | none |
| Seed 23, first mapping, mirrored | held within | 12 | 13 772 | 8 372 to 8 478 | 444 257 to 536 456 | −845 924 to −627 020 | 7 418 to 7 460 | 2 992 to 3 420 | 4 964 to 4 986 | 0 | none |
| Seed 24, eighth, assignment | held across | 12 | 7 453 | 4 192 to 4 498 | 133 308 to 159 325 | −456 350 to −369 963 | 13 469 to 13 767 | 6 659 to 7 567 | 4 944 to 4 990 | 0 | none |
| Seed 24, eighth, assignment | held within | 12 | 13 795 | 8 557 to 8 839 | 499 008 to 557 632 | −849 535 to −715 622 | 7 355 to 7 440 | 3 071 to 3 597 | 4 953 to 4 989 | 0 | none |
| Seed 22, first mapping, mirrored | held across | 12 | 7 345 | 3 793 to 4 054 | 87 387 to 118 627 | −374 294 to −303 234 | 13 615 to 13 869 | 6 370 to 6 980 | 4 952 to 4 992 | 0 | none |
| Seed 22, first mapping, mirrored | held within | 12 | 13 903 | 8 191 to 8 511 | 471 190 to 567 745 | −756 233 to −586 834 | 7 270 to 7 330 | 2 792 to 3 153 | 4 954 to 4 986 | 3 | none |
| Seed 22, eighth, assignment | held across | 12 | 7 345 | 4 001 to 4 238 | 113 022 to 141 353 | −438 595 to −348 360 | 13 533 to 13 880 | 6 697 to 7 280 | 4 947 to 4 989 | 3 | none |
| Seed 22, eighth, assignment | held within | 12 | 13 903 | 8 486 to 8 784 | 485 262 to 579 698 | −862 280 to −674 130 | 7 253 to 7 338 | 2 858 to 3 427 | 4 960 to 4 991 | 3 | none |

- **Before the night's first tick**, in the nights that hold it, 4 036 to 4 606 of the 7 345 to 7 476 synapses of the part held across
  carried a trace that was not zero, and 8 001 to 9 097 of the 13 772 to 13 903 synapses of the part held within.
- **The other part moved** in every held night: under the night held across 13 451 to 13 881 of the part held within's synapses, 6 370 to
  8 034 of them with a trace at the night's last tick; under the night held within 7 241 to 7 465 of the part held across's, 2 792 to
  3 643 with a trace. The excitatory synapses from units outside the hold moved: 4 940 to 4 992 under either.
- **A held synapse's presynaptic unit fired at or after the night's last tick** in 13 of the 144 nights held across, 3 on each of four
  cells, seed 23's eighth image from each arm, seed 23's first mapping from the mirrored assignment and seed 22's eighth image from the
  assignment, and 1 on seed 24's eighth image from the mirrored assignment; and in 12 of the 144 held within, 3 on each of four cells, seed
  23's eighth image from each arm, seed 22's first mapping from the mirrored assignment and seed 22's eighth image from the assignment. The
  quiet ticks moved no synapse of either part in any of the 288 held nights. After them, in the nights that hold it, 0 to 7 synapses of
  the part held across carried a trace, and 0 to 11 of the part held within. Of the part not held, as many synapses carried a trace as at
  the night's last tick in 268 of the 288 held nights, and 1 to 3 more or fewer in the other 20, all nights in which a held synapse's
  presynaptic unit fired at or after the night's last tick. The jobs print these counts, not the traces themselves.

The night's pairings still to be read on the held part's synapses, as What ran defines them, each read at the synapse's presynaptic unit's
next spike if that spike comes before the target's next spike:

| Cell | Form | Pending gains: held synapses | their sum, LSB | their largest | Pending losses with a presynaptic spike at the next tick: held synapses | their sum, LSB | their largest |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | held across | 3 715 to 3 800 | 177 606 to 214 288 | 327 to 328 | 7 345 | 19 816 to 22 954 | 96 to 129 |
| Seed 22, eighth, mirrored | held within | 7 215 to 7 386 | 372 847 to 486 593 | 328 | 13 903 | 35 323 to 51 472 | 104 to 126 |
| Seed 22, first mapping, assignment | held across | 3 693 to 3 793 | 146 526 to 184 512 | 327 to 328 | 7 345 | 19 116 to 24 888 | 85 to 123 |
| Seed 22, first mapping, assignment | held within | 7 156 to 7 689 | 344 369 to 431 501 | 328 | 13 903 | 31 386 to 47 067 | 101 to 150 |
| Seed 23, eighth, mirrored | held across | 3 739 to 3 854 | 183 744 to 227 847 | 326 to 328 | 7 476 | 18 276 to 27 014 | 93 to 119 |
| Seed 23, eighth, mirrored | held within | 6 876 to 7 340 | 397 677 to 482 764 | 328 | 13 772 | 43 751 to 55 077 | 105 to 150 |
| Seed 23, first mapping, assignment | held across | 3 749 to 3 920 | 159 466 to 202 672 | 327 to 328 | 7 476 | 21 243 to 25 537 | 95 to 130 |
| Seed 23, first mapping, assignment | held within | 7 098 to 7 348 | 345 727 to 463 997 | 328 | 13 772 | 37 151 to 49 429 | 105 to 125 |
| Seed 24, eighth, mirrored | held across | 3 774 to 3 860 | 204 914 to 226 405 | 328 | 7 453 | 22 095 to 27 481 | 103 to 119 |
| Seed 24, eighth, mirrored | held within | 7 039 to 7 332 | 390 087 to 487 561 | 328 | 13 795 | 40 480 to 65 582 | 106 to 143 |
| Seed 24, first mapping, assignment | held across | 3 789 to 3 863 | 144 362 to 200 223 | 327 to 328 | 7 453 | 18 133 to 22 953 | 99 to 120 |
| Seed 24, first mapping, assignment | held within | 7 036 to 7 355 | 364 165 to 470 250 | 328 | 13 795 | 31 659 to 44 480 | 103 to 132 |
| Seed 23, eighth, assignment | held across | 3 719 to 3 832 | 172 123 to 203 277 | 327 to 328 | 7 476 | 18 052 to 26 826 | 93 to 118 |
| Seed 23, eighth, assignment | held within | 7 159 to 7 500 | 390 253 to 452 937 | 328 | 13 772 | 36 081 to 53 450 | 106 to 146 |
| Seed 24, first mapping, mirrored | held across | 3 740 to 3 900 | 131 424 to 200 757 | 326 to 328 | 7 453 | 16 829 to 24 607 | 99 to 111 |
| Seed 24, first mapping, mirrored | held within | 7 096 to 7 399 | 307 920 to 400 730 | 328 | 13 795 | 27 769 to 52 245 | 103 to 132 |
| Seed 23, first mapping, mirrored | held across | 3 740 to 3 929 | 143 025 to 209 947 | 327 to 328 | 7 476 | 18 987 to 24 739 | 90 to 130 |
| Seed 23, first mapping, mirrored | held within | 7 069 to 7 409 | 318 228 to 427 779 | 328 | 13 772 | 38 102 to 50 071 | 105 to 121 |
| Seed 24, eighth, assignment | held across | 3 770 to 3 855 | 158 004 to 213 488 | 327 to 328 | 7 453 | 15 935 to 24 151 | 91 to 110 |
| Seed 24, eighth, assignment | held within | 7 175 to 7 445 | 356 821 to 422 738 | 328 | 13 795 | 33 083 to 49 555 | 106 to 142 |
| Seed 22, first mapping, mirrored | held across | 3 715 to 3 871 | 125 102 to 166 384 | 327 to 328 | 7 345 | 22 456 to 25 651 | 87 to 125 |
| Seed 22, first mapping, mirrored | held within | 7 417 to 7 797 | 317 244 to 379 424 | 328 | 13 903 | 33 818 to 65 374 | 101 to 149 |
| Seed 22, eighth, assignment | held across | 3 748 to 3 830 | 144 068 to 173 590 | 327 to 328 | 7 345 | 16 545 to 32 814 | 108 to 128 |
| Seed 22, eighth, assignment | held within | 7 214 to 7 607 | 338 057 to 402 894 | 328 | 13 903 | 36 039 to 55 223 | 107 to 123 |

- **The pending gain**, the target's spike after the presynaptic unit's last, is on 3 693 to 3 929 synapses of the part held across,
  125 102 to 227 847 LSB together, the largest 326 to 328 LSB; and on 6 876 to 7 797 of the part held within, 307 920 to 487 561 LSB
  together, the largest 328 LSB, which is $A_+$.
- **The pending loss** with a presynaptic spike at the first tick after the image is on every synapse of the held part: 15 935 to
  32 814 LSB together on the part held across, the largest 85 to 130 LSB, and 27 769 to 65 582 LSB on the part held within, the largest
  101 to 150. A later spike, before the target's next one, reads no more.
- **Against what the hold cleared.** These pairings were never in the trace the hold cleared, which held, synapse by synapse, the decayed
  net of what the trace carried into the night and the pairings read at the night's presynaptic spikes. Set beside it as sizes alone: the
  pending gains are 97.6 % to 193.0 % the size of the traces cleared above zero on the part held across, and 56.5 % to 93.3 % on the part
  held within; the most the pending losses can be is 3.6 % to 8.5 % the size of the traces cleared below zero on the part held across, and
  3.7 % to 11.0 % on the part held within.
- These are trace, not weight: at the next presynaptic spike, in the next day's engine, each synapse's trace takes its pairings, and its
  consolidation moves the weight under that day's modulation. That consolidation is not read here.

## 6. The twelve cells at block 36

H-47's twelve cells at block 36, the rows ordered by the readout A answers, the six where it answers readout 0 first. Every held form is
this exploration's job; the excitatory night is exploration 026's job for its eight cells and exploration 025's (run
[42469251250](https://github.com/Menteion/Menteion/actions/runs/42469251250)) for exploration 020's four; H-47's plain night is H-47's job's
in all twelve. "Less without replay" and "less held within" carry the paired SE of the job that ran both forms, and "less H-47's plain" and
"less excitatory" exploration 018's unpaired SE. No figure is pooled across cells:

| Cell | A answers | Held across less without replay | Held within less without replay | Held across less held within | H-47: plain less without replay | Excitatory less without replay |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | +19.5 ± 1.24 | +16.8 ± 0.83 | +2.8 ± 1.34 | +19.6 ± 1.06 | +16.3 ± 0.84 |
| Seed 22, first mapping, assignment | readout 0, the odd places | +7.7 ± 1.07 | +6.5 ± 1.42 | +1.2 ± 1.82 | +7.9 ± 0.51 | +5.2 ± 1.24 |
| Seed 23, eighth, mirrored | readout 0, the odd places | +16.5 ± 1.42 | +15.5 ± 1.06 | +1.0 ± 0.37 | +17.4 ± 1.08 | +14.7 ± 1.54 |
| Seed 23, first mapping, assignment | readout 0, the odd places | +13.2 ± 1.51 | +13.3 ± 1.64 | −0.1 ± 0.32 | +13.4 ± 1.56 | +12.6 ± 1.56 |
| Seed 24, eighth, mirrored | readout 0, the odd places | +13.4 ± 1.11 | +15.5 ± 1.27 | −2.2 ± 0.63 | +11.3 ± 0.96 | +16.7 ± 1.23 |
| Seed 24, first mapping, assignment | readout 0, the odd places | +7.0 ± 0.93 | +6.1 ± 1.46 | +0.8 ± 1.10 | +3.8 ± 1.57 | +5.8 ± 1.60 |
| Seed 23, eighth, assignment | readout 1, the even places | +6.1 ± 2.39 | +10.2 ± 1.77 | −4.1 ± 0.78 | +4.1 ± 1.45 | +10.9 ± 2.49 |
| Seed 24, first mapping, mirrored | readout 1, the even places | +7.1 ± 1.35 | +7.7 ± 1.37 | −0.6 ± 1.06 | +6.0 ± 1.54 | +7.4 ± 1.09 |
| Seed 23, first mapping, mirrored | readout 1, the even places | −2.7 ± 1.61 | −2.6 ± 0.65 | 0.0 ± 1.42 | −4.9 ± 1.09 | −1.7 ± 0.42 |
| Seed 24, eighth, assignment | readout 1, the even places | +2.8 ± 2.44 | +4.1 ± 1.17 | −1.3 ± 1.89 | +1.8 ± 1.79 | +8.8 ± 1.28 |
| Seed 22, first mapping, mirrored | readout 1, the even places | +4.6 ± 1.99 | +5.6 ± 1.68 | −1.0 ± 0.73 | +5.0 ± 1.24 | +4.7 ± 1.52 |
| Seed 22, eighth, assignment | readout 1, the even places | +3.2 ± 0.95 | +4.5 ± 1.26 | −1.3 ± 1.47 | +2.2 ± 0.65 | +4.6 ± 0.66 |

| Cell | A answers | Held across less H-47's plain | Held within less H-47's plain | Held across less excitatory | Held within less excitatory |
| :--- | :--- | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | −0.1 ± 1.18 | −2.9 ± 0.82 | +3.3 ± 1.87 | +0.5 ± 1.44 |
| Seed 22, first mapping, assignment | readout 0, the odd places | −0.2 ± 0.64 | −1.4 ± 1.49 | +2.5 ± 1.33 | +1.3 ± 1.13 |
| Seed 23, eighth, mirrored | readout 0, the odd places | −0.9 ± 1.19 | −1.9 ± 0.94 | +1.8 ± 1.16 | +0.8 ± 1.01 |
| Seed 23, first mapping, assignment | readout 0, the odd places | −0.1 ± 0.87 | −0.1 ± 0.78 | +0.6 ± 0.98 | +0.7 ± 0.98 |
| Seed 24, eighth, mirrored | readout 0, the odd places | +2.1 ± 1.36 | +4.3 ± 1.55 | −3.3 ± 0.80 | −1.1 ± 0.93 |
| Seed 24, first mapping, assignment | readout 0, the odd places | +3.2 ± 1.12 | +2.4 ± 1.23 | +1.2 ± 1.18 | +0.4 ± 0.90 |
| Seed 23, eighth, assignment | readout 1, the even places | +2.0 ± 1.42 | +6.1 ± 1.11 | −4.9 ± 1.13 | −0.8 ± 1.16 |
| Seed 24, first mapping, mirrored | readout 1, the even places | +1.1 ± 0.91 | +1.7 ± 0.84 | −0.4 ± 0.87 | +0.3 ± 0.70 |
| Seed 23, first mapping, mirrored | readout 1, the even places | +2.2 ± 0.91 | +2.3 ± 1.21 | −1.0 ± 1.49 | −0.9 ± 0.79 |
| Seed 24, eighth, assignment | readout 1, the even places | +0.9 ± 2.00 | +2.3 ± 1.93 | −6.0 ± 2.22 | −4.7 ± 1.54 |
| Seed 22, first mapping, mirrored | readout 1, the even places | −0.4 ± 1.02 | +0.6 ± 1.09 | −0.1 ± 1.24 | +0.9 ± 0.78 |
| Seed 22, eighth, assignment | readout 1, the even places | +1.0 ± 1.66 | +2.3 ± 1.75 | −1.4 ± 1.69 | −0.1 ± 1.44 |

| Cell | A answers | Held across: side | Held within: side | H-47's side | Excitatory: side |
| :--- | :--- | :--- | :--- | :--- | :--- |
| Seed 22, eighth, mirrored | readout 0, the odd places | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | B, the even places (within 2 SE) | A, the odd places |
| Seed 22, first mapping, assignment | readout 0, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | readout 0, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | readout 0, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 24, eighth, mirrored | readout 0, the odd places | B, the even places | B, the even places (within 2 SE) | B, the even places | B, the even places (within 2 SE) |
| Seed 24, first mapping, assignment | readout 0, the odd places | B, the even places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places |
| Seed 23, eighth, assignment | readout 1, the even places | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places |
| Seed 24, first mapping, mirrored | readout 1, the even places | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | readout 1, the even places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 24, eighth, assignment | readout 1, the even places | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | readout 1, the even places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | readout 1, the even places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |

- **Less the night without replay** the night held across lies beyond 2 SE above in ten of the twelve cells and within 2 SE in two, seed
  23's first mapping from the mirrored assignment, −2.7 ± 1.61, and seed 24's eighth image from the assignment, +2.8 ± 2.44. The night held
  within lies above in eleven and below in one, seed 23's first mapping from the mirrored assignment, −2.6 ± 0.65, where H-47's clause 1
  read −4.9 ± 1.09.
- **The night held across less the night held within** lies beyond 2 SE in four cells: above on seed 22's eighth image from the mirrored
  assignment, +2.8 ± 1.34, and seed 23's eighth image from the mirrored assignment, +1.0 ± 0.37, both where A answers readout 0; below on
  seed 24's eighth image from the mirrored assignment, −2.2 ± 0.63, where A answers readout 0, and seed 23's eighth image from the
  assignment, −4.1 ± 0.78, where it answers readout 1.
- **Less H-47's plain night** the night held across lies beyond 2 SE above in two cells, seed 24's first mapping from the assignment,
  +3.2 ± 1.12, where A answers readout 0, and seed 23's first mapping from the mirrored assignment, +2.2 ± 0.91, where it answers readout 1;
  and within 2 SE in the other ten. The night held within lies above in three, seed 24's eighth image from the mirrored assignment,
  +4.3 ± 1.55, where A answers readout 0, and seed 23's eighth image from the assignment, +6.1 ± 1.11, and seed 24's first mapping from the
  mirrored assignment, +1.7 ± 0.84, where it answers readout 1; below in one, seed 22's eighth image from the mirrored assignment,
  −2.9 ± 0.82, where A answers readout 0; and within 2 SE in the other eight.
- **Less the excitatory night** the night held across lies beyond 2 SE below in three cells, seed 24's eighth image from the mirrored
  assignment, −3.3 ± 0.80, where A answers readout 0, and seed 23's eighth image from the assignment, −4.9 ± 1.13, and seed 24's eighth
  image from the assignment, −6.0 ± 2.22, where it answers readout 1; and within 2 SE in the other nine. The night held within lies below
  in one, seed 24's eighth image from the assignment, −4.7 ± 1.54, and within 2 SE in the other eleven.
- **The side** at block 36 under the night held within is the excitatory night's in all twelve cells: on the odd places in nine, on A in
  five where A answers readout 0 and on B in four where it answers readout 1, and on the even places on seed 24's eighth image from the
  mirrored assignment, on B, and on seed 23's and seed 24's eighth images from the assignment, on A. Under the night held across it lies on
  the odd places in seven: on A in four where A answers readout 0 and on B in three where it answers readout 1. It lies on the even places
  on seed 24's eighth image from the mirrored assignment and seed 24's first mapping from the assignment, on B, and on seed 23's eighth
  image from the assignment, seed 24's first mapping from the mirrored assignment and seed 24's eighth image from the assignment, on A.
## What it costs

The trial took 785 s and the twelve jobs 5 718 to 6 153 s, 95.3 to 102.6 minutes, at most 42.7 % of a job's 240 minutes; ADR-0242 put a
job at about 95 to 105 minutes. Nothing was cut: every cell ran its three forms. Thirteen jobs ran, the trial's and twelve, every one to the
end.

## What it suggests

**In these twelve cells the night held within lies within 2 SE of the excitatory night at 56 of the 60 checkpoints, and the night held
across within 2 SE of H-47's plain night at 55.** The four checkpoints where the night held within lies beyond are all on seed 24's eighth
image from the assignment, from block 12 on, all below; there the night held across lies below the excitatory night beyond 2 SE too at
blocks 24 and 36. Of the 17 checkpoints where exploration 026 read the excitatory night beyond 2 SE of the plain night, the night held
within lies beyond 2 SE of the plain night in the same direction at 11: above on seed 24's eighth image from the mirrored assignment and on
seed 23's eighth image from the assignment, from block 12 on; below on seed 23's eighth image from the mirrored assignment at blocks 18 and
24 and on seed 22's eighth image from the mirrored assignment at block 36. The night held across does so at 2, seed 24's eighth image from
the mirrored assignment at block 24 and seed 24's first mapping from the assignment at block 36. Of the 8 where exploration 025 read it,
all above and all on exploration 020's four cells, the night held within lies above the plain night beyond 2 SE at 3 and the night held
across at 2.

**In exploration 026's eight cells, then, A's count under the excitatory night follows the hold of the part held within more than the
hold of the part held across**, though in every one of the twelve cells the part held across is the target the plain night moved most of
the held units' synapses onto: −4.73 to −6.98 M a group over four nights. Over the twelve cells, held alone, the part held across left the
part held within moving by 1.04 to 1.20 times the plain night's change; held alone, the part held within left the part held across moving
as under the plain night, 0.97 to 1.03 times it.
The side under the night held within is the excitatory night's at 56 of the 60 checkpoints, and at block 36 in all twelve cells.

**These runs do not part the part held within by target.** It holds the held units' synapses onto their own group, onto the inhibitory
units, onto A's and B's units and onto the other excitatory units, and nothing here holds one of them and not another. Under each held
night the inhibition onto the readouts' excitatory units rose by 0.33 to 1.11 M more than under the plain night, and each hold leaves
pending gains on about half the synapses of its part. The note does not read what the next day does with them.

Questions this suggests for a later exploration or a round:
- The part held within split by target: the held units' synapses onto their own group, onto the inhibitory units and onto A's and B's
  units, each held alone, to find which carries the excitatory night's count.
- What on seed 24's eighth image from the assignment keeps the hold of the part held within below the excitatory night from block 12 on,
  where exploration 025 read the excitatory night above the plain night beyond 2 SE from block 12 on.
- A read of what the next day's first consolidations do with the pending gains and losses.

## What did not run

- **Exploration 024's hold**, alone or with either part: the inhibitory synapses onto the readouts' excitatory units consolidate as in
  H-47 under every form here.
- **The excitatory night again**: it is read from exploration 025's and 026's jobs, and no form here holds both parts.
- **Either part held in the night without replay**: the night without replay ran with no hold set.
- **A finer split by target**: the part held within holds four targets together, and section 4 breaks them down without holding one apart.
- **Plain nights on both mappings and the awake gap**, under a hold or not: no reading here stands against them.
- **A hold through the quiet ticks**: the hold ends at the night's last tick, and the image is written after the quiet ticks. Here the
  quiet ticks moved no synapse of either part in any of the 288 held nights.
- **The excitatory synapses onto the readouts from A's, B's and the pair's own units**, which consolidate as in H-47 under every form, the
  zero sum's rounds with them, and **the inhibitory synapses onto the readouts' inhibitory units**, which no hold here reaches.
- **The pending pairings' effect on a weight**: the next day's consolidation of the held synapses is not read.
- **A diagnosis of seed 23's eighth image from the mirrored assignment alone**: it runs here as one cell of twelve.
- **REM's depotentiations and the zero sum's rounds** are not split out of a night's change, as in explorations 020 to 026.
- **Any SE for sections 4 and 5**: the class matrix is a mean over the three paths, and the hold's readings are ranges.
- **Any figure pooled across cells**, in section 6 or elsewhere.
- **Any other share, form, network or seed.**

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_the_excitatory_hold_by_target_explore` | [42512634982](https://github.com/Menteion/Menteion/actions/runs/42512634982) | `65b0c59` | 785 |
| `the_excitatory_hold_by_target_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) | `65b0c59` | 5992 |
| `the_excitatory_hold_by_target_a_first_mapping_on_seed_22_from_the_assignment_explore` | [42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) | `65b0c59` | 6007 |
| `the_excitatory_hold_by_target_the_eighth_image_on_seed_23_from_the_mirrored_assignment_explore` | [42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) | `65b0c59` | 5718 |
| `the_excitatory_hold_by_target_a_first_mapping_on_seed_23_from_the_assignment_explore` | [42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) | `65b0c59` | 5858 |
| `the_excitatory_hold_by_target_the_eighth_image_on_seed_24_from_the_mirrored_assignment_explore` | [42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) | `65b0c59` | 5933 |
| `the_excitatory_hold_by_target_a_first_mapping_on_seed_24_from_the_assignment_explore` | [42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) | `65b0c59` | 6145 |
| `the_excitatory_hold_by_target_the_eighth_image_on_seed_23_from_the_assignment_explore` | [42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) | `65b0c59` | 5991 |
| `the_excitatory_hold_by_target_a_first_mapping_on_seed_24_from_the_mirrored_assignment_explore` | [42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) | `65b0c59` | 5996 |
| `the_excitatory_hold_by_target_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) | `65b0c59` | 6046 |
| `the_excitatory_hold_by_target_the_eighth_image_on_seed_24_from_the_assignment_explore` | [42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) | `65b0c59` | 5902 |
| `the_excitatory_hold_by_target_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) | `65b0c59` | 6153 |
| `the_excitatory_hold_by_target_the_eighth_image_on_seed_22_from_the_assignment_explore` | [42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) | `65b0c59` | 5989 |
