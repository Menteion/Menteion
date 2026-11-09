---
status: exploratory
date: 2026-11-09
---

# 028: The part held within split by target

**Branch**: `explore/028-the-part-held-within-by-target`
**Commit**: `936f474`, which adds the harness, on `0361425`, which adds the three parts under `src/`, on `b986c20`: `main` at `878910a`,
ADR-0243's merge, with exploration 027's tag merged in, which carries explorations 024's, 025's and 027's changes under `src/`.
**Runs**: [42551926531](https://github.com/Menteion/Menteion/actions/runs/42551926531) (the harness's trial) and
[42552893754](https://github.com/Menteion/Menteion/actions/runs/42552893754) (twelve jobs, one a cell), both at `scope=arms`. H-47's
plain night on A's patterns is read from H-47's run [41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613), its class
matrix and night lines from exploration 020's runs [41500459562](https://github.com/Menteion/Menteion/actions/runs/41500459562) and
[41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777) for exploration 020's four cells and from exploration 021's run
[41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) for the other eight. The excitatory night is read from
exploration 025's run [42469251250](https://github.com/Menteion/Menteion/actions/runs/42469251250) for exploration 020's four cells and from
exploration 026's run [42489793510](https://github.com/Menteion/Menteion/actions/runs/42489793510) for the other eight, and exploration
027's night held within from exploration 027's run [42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) and its
trial's row from run [42512634982](https://github.com/Menteion/Menteion/actions/runs/42512634982). None of those was run again.

[ADR-0243](../adr/0243-the-part-held-within-split-by-target.md) ordered this exploration.
[Exploration 027](027-the-excitatory-hold-by-target.md) split [exploration 025](025-synapses-from-the-readouts-excitatory-units.md)'s
excitatory hold, every synapse from the readouts' excitatory units but the pair's own, by target: the part held across, onto the other
readout's excitatory units, and the part held within, the rest. Under the night held within, A's count lay within twice exploration 018's
unpaired SE of the excitatory night's at 56 of its 60 checkpoints. This note splits the part held within in three by target, onto the held
unit's own group, onto the inhibitory units and onto the units outside the readouts, and holds each alone through H-47's plain night on A's
patterns, on all twelve of H-47's cells. It asks, at blocks 6, 12, 18, 24 and 36:

1. A's count and each stimulus's correct trials under each held form: less the night without replay and less each other held form, with
   H-47's paired SE, the forms run in one job; and less H-47's plain night, less exploration 027's night held within and less the excitatory
   night, with [exploration 018](018-what-nights-keep.md)'s unpaired SE, the forms run in two jobs;
2. the side under each held form, by stimulus and by group of units, against H-47's side, the excitatory night's and exploration 027's
   night held within's on the same cell;
3. the pair under each held form and A's highest coupling, as H-47's clauses 2 and 3 read them;
4. what each night moved, from the class matrix by group, with the held units' synapses broken down by target, the held part's change shown
   nil and each other part's read beside the plain night's, and the inhibition onto each group's excitatory units;
5. what the hold leaves: what the quiet ticks after each night move, and the pairings still pending on the held part's synapses, the gain
   and the loss apart;
6. the twelve cells side by side at block 36: each held form less the night without replay, with the job's paired SE, and less H-47's plain
   night, less exploration 027's night held within and less the excitatory night, with exploration 018's unpaired SE.

**The side** is [ADR-0238](../adr/0238-what-keeps-a-mapping-where-the-replay-did-not.md)'s: at a checkpoint, the stimulus whose difference
from the night without replay is the larger of the two. The note writes no prediction and decides nothing. **Every figure here is
exploratory**: from one run of each test on the repository's AArch64 host, in H-47's terms. A count is a mean over a cell's 48 reads,
sixteen on each of three paths, and a difference carries its paired SE unless another is named.

**In brief**
- **Each hold held its part, and the night without replay ran H-47's.** In all 432 held nights every synapse of the held part ended the
  night at the weight it started with and its trace cleared, held inside the test, while synapses of the other parts moved. The quiet ticks
  after a night moved no synapse of the held part (section 5). Every row of the night without replay, 15 a cell, is H-47's, held inside the
  test.
- **The three parts**, by seed: 6 891 to 7 030 synapses onto the own group, 4 487 to 4 543 onto the inhibitory units and 2 339 to 2 366
  onto the units outside the readouts, disjoint and together exploration 027's part held within, 13 772 to 13 903, in every cell, held
  inside the test synapse by synapse. In the trial the three held together ran exploration 027's night held within's row bit for bit.
- **Less H-47's plain night**, A's count lies beyond twice exploration 018's unpaired SE at 3 of the 60 checkpoints under the night held
  onto the own group, at 6 under the night held onto the units outside the readouts, all above, and at 13 under the night held onto the
  inhibitory units, 7 above and 6 below, where exploration 027's night held within lay beyond at 17 and the excitatory night at 25.
- **Where the excitatory night lies beyond the plain night**, 25 checkpoints, the night held onto the inhibitory units lies beyond it in the
  same direction at 12, the night held onto the units outside the readouts at 4 and the night held onto the own group at 1.
- **Less exploration 027's night held within**, the night held onto the inhibitory units lies beyond 2 SE at 8 of the 60 checkpoints, all
  below; the night held onto the units outside the readouts at 13, 4 above and 9 below; and the night held onto the own group at 17, 3
  above and 14 below. **Less the excitatory night**, at 11, 19 and 20.
- **The held nights less each other**, run in one job, lie beyond twice their paired SE at 16 to 22 of the 60 checkpoints; less the night
  without replay they lie above beyond 2 SE at 42 to 48 of the 60.
- **The side under each held night is H-47's at 55 to 58 of the 60 checkpoints**; at block 36 under the night held onto the inhibitory
  units it is the excitatory night's in eleven of the twelve cells (section 2).
- **Each part held alone left the other three moving as under H-47's plain night**, within 0.2 M a group over four nights. The inhibition
  onto each group's excitatory units rose by 0.16 to 0.96 M more than under the plain night under the night held onto the inhibitory
  units, by −0.04 to +0.24 M under the night held onto the own group and by −0.03 to +0.11 M under the night held onto the units outside
  the readouts.
- **The held nights ran H-47's plain night's replays and depotentiations in 140 to 142 of their 144 nights each**, all others on seed 23's
  eighth image from the mirrored assignment. The watch read `wrong 0` in all 576 nights of the twelve jobs.
- **The pairings the hold leaves**: after the quiet ticks, 3 326 to 3 651 synapses onto the own group carry a pending gain, 2 372 to 2 923
  onto the inhibitory units and 988 to 1 292 onto the units outside the readouts, each read at the presynaptic unit's next spike if it
  comes before the target's next spike.
- **The pair** stands at 61.7 to 63.3 at block 36 under the three held nights, and A's highest coupling at 1.138 to 1.237 of H-42's.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 020 to 027 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. A cell is named, as in H-47, by its seed, its image and the arm
it starts from, not by the mapping in force. Nights follow blocks 6, 12, 18 and 24, then 12 blocks run with none to block 36, on paths 0, 5
and 6. Each checkpoint reads A and the pair sixteen times each, after the night that follows its block, and at block 36 after the block.

**Places.** A period of the ring is 20 units. Stimulus A presents place 0 of each period and B place 11. **The odd places** are the odd
places but 11 and **the even places** the even places but 0: H-47's readout 0 and readout 1, each with its excitatory and its inhibitory
units. C's own units lie at place 5, among the odd places, and D's at place 16, among the even, in every cell
([exploration 021](021-where-the-gain-falls.md), section 4).

**Twelve cells**, all of H-47's, in exploration 027's order: the six where A answers readout 0 first, then the six where it answers
readout 1. Under the assignment stimulus A answers readout 0, the odd places, and B readout 1, the even places; under the mirrored mapping
the other way. A answers readout 0 on the eighth images from the mirrored assignment and on the first mappings from the assignment. The
excitatory night's job is exploration 026's for its eight cells and exploration 025's for exploration 020's four; exploration 027's night
held within is exploration 027's job in all twelve:

| Cell | A answers | B answers | H-47: plain less without replay at block 36 (clause 1) | Held within (027) less without replay at block 36 | Excitatory less without replay at block 36 | Excitatory night's job |
| :--- | :--- | :--- | ---: | ---: | ---: | :--- |
| Seed 22, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +19.6 ± 1.06 | +16.8 ± 0.83 | +16.3 ± 0.84 | 026 |
| Seed 22, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +7.9 ± 0.51 | +6.5 ± 1.42 | +5.2 ± 1.24 | 026 |
| Seed 23, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +17.4 ± 1.08 | +15.5 ± 1.06 | +14.7 ± 1.54 | 026 |
| Seed 23, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +13.4 ± 1.56 | +13.3 ± 1.64 | +12.6 ± 1.56 | 026 |
| Seed 24, eighth, mirrored | readout 0, the odd places | readout 1, the even places | +11.3 ± 0.96 | +15.5 ± 1.27 | +16.7 ± 1.23 | 026 |
| Seed 24, first mapping, assignment | readout 0, the odd places | readout 1, the even places | +3.8 ± 1.57 | +6.1 ± 1.46 | +5.8 ± 1.60 | 026 |
| Seed 23, eighth, assignment | readout 1, the even places | readout 0, the odd places | +4.1 ± 1.45 | +10.2 ± 1.77 | +10.9 ± 2.49 | 026 |
| Seed 24, first mapping, mirrored | readout 1, the even places | readout 0, the odd places | +6.0 ± 1.54 | +7.7 ± 1.37 | +7.4 ± 1.09 | 026 |
| Seed 23, first mapping, mirrored | readout 1, the even places | readout 0, the odd places | −4.9 ± 1.09 | −2.6 ± 0.65 | −1.7 ± 0.42 | 025 |
| Seed 24, eighth, assignment | readout 1, the even places | readout 0, the odd places | +1.8 ± 1.79 | +4.1 ± 1.17 | +8.8 ± 1.28 | 025 |
| Seed 22, first mapping, mirrored | readout 1, the even places | readout 0, the odd places | +5.0 ± 1.24 | +5.6 ± 1.68 | +4.7 ± 1.52 | 025 |
| Seed 22, eighth, assignment | readout 1, the even places | readout 0, the odd places | +2.2 ± 0.65 | +4.5 ± 1.26 | +4.6 ± 0.66 | 025 |

**The three parts** are a change under `src/` on this branch alone (`0361425`), in no image, which `main` does not take. Exploration 027's
`Executor::set_night_hold_by_target` takes a set of excitatory units, each with a group, as held sources; a set of targets, each with a
group; and a part. Its part held across is each synapse from a held source onto a target of another group than its source's, and its part
held within each other synapse from a held source. This branch adds three parts of the part held within to `NightHoldPart`, and
`Executor::set_night_hold_by_target_parts`, which holds the slots of any parts it is given together:
- **`OwnGroup`**, each synapse from a held source onto a target of its own group;
- **`Inhibitory`**, each synapse from a held source onto an inhibitory target of no group;
- **`Outside`**, each synapse from a held source onto an excitatory target of no group.

A target's polarity is read from its unit's flags when the hold is set. Each part is a bit of its own beside exploration 027's two, so a
synapse within is held when the part held within is set or when its own part is, and exploration 027's parts hold what they held. A held
synapse is held as exploration 027 holds its part: while the engine is in slow-wave sleep or REM the fan-out consolidates every slot of a
held source's block but those held, slot by slot, so each held synapse keeps its weight while its trace takes the night's pairings, and
every other synapse, those of the other two parts and of the part held across among them, consolidates as in H-47. **The night's last
tick** is the tick whose window steps the engine from sleep to awake; the hold is in force through it, and at its end each held synapse has
its trace cleared to zero, and no other synapse. The hold is in no image, so the next day's engine, decoded from the night's image, holds
nothing. Exploration 024's hold (`set_night_hold`) and exploration 025's (`set_night_hold_sources`) are on the branch too, and no form here
sets either, nor either of exploration 027's two parts; only the trial sets the three parts together. A unit test on a small
engine whose held unit's chain reaches a target of each part holds that the hold, set and unset, or refused for no part, leaves a whole night
bit for bit as it was; that the three parts set together are exploration 027's part held within bit for bit, and with the part held across
exploration 027's two parts together; and that each part alone leaves its synapse at its first weight with its trace cleared while every
other synapse of the chain moves.

**The held units, the targets with a group and the classes** (`e025_held`, `e027_hold`, `e028_hold`) are exploration 027's: every
excitatory unit of the two readouts but the pair's own, 664 in each cell, 332 a readout, as each job prints, held inside the test in the
class matrix's classes 6 and 7, the odd places', and 8 and 9, the even places'; and every excitatory unit of the two readouts, the pair's
own units among them, 714 in each cell, 357 a readout, each with its readout's group, held inside the test in classes 4, 6 and 7 for the odd
places and 5, 8 and 9 for the even. So each part, from a held unit of the odd places and of the even places, lies in the class matrix's
columns:

| Part | From the odd places' held units, columns | From the even places' held units, columns |
| :--- | :--- | :--- |
| Onto the own group | 4, 6 and 7 | 5, 8 and 9 |
| Onto the inhibitory units | 11, 12 and 13 | 11, 12 and 13 |
| Onto the units outside the readouts | 0 to 3 and 10 | 0 to 3 and 10 |
| Held across (exploration 027) | 5, 8 and 9 | 4, 6 and 7 |

Each job prints how many units each class holds, and how many of them are the held units and the targets:

| Cell | Held units | of readout 0 and of readout 1 | The readouts' excitatory units | Inhibitory units: in readout 0 | in readout 1 | elsewhere | A's and B's units | Other excitatory units |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 664 | 332 and 332 | 714 | 102 | 102 | 0 | 102 | 4 |
| Seed 22, first mapping, assignment | 664 | 332 and 332 | 714 | 102 | 102 | 0 | 102 | 4 |
| Seed 23, eighth, mirrored | 664 | 332 and 332 | 714 | 102 | 102 | 0 | 102 | 4 |
| Seed 23, first mapping, assignment | 664 | 332 and 332 | 714 | 102 | 102 | 0 | 102 | 4 |
| Seed 24, eighth, mirrored | 664 | 332 and 332 | 714 | 102 | 102 | 0 | 102 | 4 |
| Seed 24, first mapping, assignment | 664 | 332 and 332 | 714 | 102 | 102 | 0 | 102 | 4 |
| Seed 23, eighth, assignment | 664 | 332 and 332 | 714 | 102 | 102 | 0 | 102 | 4 |
| Seed 24, first mapping, mirrored | 664 | 332 and 332 | 714 | 102 | 102 | 0 | 102 | 4 |
| Seed 23, first mapping, mirrored | 664 | 332 and 332 | 714 | 102 | 102 | 0 | 102 | 4 |
| Seed 24, eighth, assignment | 664 | 332 and 332 | 714 | 102 | 102 | 0 | 102 | 4 |
| Seed 22, first mapping, mirrored | 664 | 332 and 332 | 714 | 102 | 102 | 0 | 102 | 4 |
| Seed 22, eighth, assignment | 664 | 332 and 332 | 714 | 102 | 102 | 0 | 102 | 4 |

Each held night holds, inside the test, synapse by synapse (`e028_parts`): that the three parts are disjoint and together exactly the
synapses exploration 027's part held within names; that the part held across is exploration 027's; that exploration 027's two parts are
disjoint and together exploration 025's hold (`e027_parts`); and that each synapse of each part, by its held unit's group, has its target
in that part's columns. Each part's synapses, as each held night prints them:

| Cell | Onto the own group: synapses | from the odd places and from the even | Onto the inhibitory units | from the odd places and from the even | Onto the units outside the readouts | from the odd places and from the even | The three together | Exploration 027's part held within | Held across | Exploration 025's hold | Excitatory synapses |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 7 030 | 3 527 and 3 503 | 4 534 | 2 266 and 2 268 | 2 339 | 1 181 and 1 158 | 13 903 | 13 903 | 7 345 | 21 248 | 26 240 |
| Seed 22, first mapping, assignment | 7 030 | 3 527 and 3 503 | 4 534 | 2 266 and 2 268 | 2 339 | 1 181 and 1 158 | 13 903 | 13 903 | 7 345 | 21 248 | 26 240 |
| Seed 23, eighth, mirrored | 6 919 | 3 498 and 3 421 | 4 487 | 2 200 and 2 287 | 2 366 | 1 221 and 1 145 | 13 772 | 13 772 | 7 476 | 21 248 | 26 240 |
| Seed 23, first mapping, assignment | 6 919 | 3 498 and 3 421 | 4 487 | 2 200 and 2 287 | 2 366 | 1 221 and 1 145 | 13 772 | 13 772 | 7 476 | 21 248 | 26 240 |
| Seed 24, eighth, mirrored | 6 891 | 3 469 and 3 422 | 4 543 | 2 247 and 2 296 | 2 361 | 1 215 and 1 146 | 13 795 | 13 795 | 7 453 | 21 248 | 26 240 |
| Seed 24, first mapping, assignment | 6 891 | 3 469 and 3 422 | 4 543 | 2 247 and 2 296 | 2 361 | 1 215 and 1 146 | 13 795 | 13 795 | 7 453 | 21 248 | 26 240 |
| Seed 23, eighth, assignment | 6 919 | 3 498 and 3 421 | 4 487 | 2 200 and 2 287 | 2 366 | 1 221 and 1 145 | 13 772 | 13 772 | 7 476 | 21 248 | 26 240 |
| Seed 24, first mapping, mirrored | 6 891 | 3 469 and 3 422 | 4 543 | 2 247 and 2 296 | 2 361 | 1 215 and 1 146 | 13 795 | 13 795 | 7 453 | 21 248 | 26 240 |
| Seed 23, first mapping, mirrored | 6 919 | 3 498 and 3 421 | 4 487 | 2 200 and 2 287 | 2 366 | 1 221 and 1 145 | 13 772 | 13 772 | 7 476 | 21 248 | 26 240 |
| Seed 24, eighth, assignment | 6 891 | 3 469 and 3 422 | 4 543 | 2 247 and 2 296 | 2 361 | 1 215 and 1 146 | 13 795 | 13 795 | 7 453 | 21 248 | 26 240 |
| Seed 22, first mapping, mirrored | 7 030 | 3 527 and 3 503 | 4 534 | 2 266 and 2 268 | 2 339 | 1 181 and 1 158 | 13 903 | 13 903 | 7 345 | 21 248 | 26 240 |
| Seed 22, eighth, assignment | 7 030 | 3 527 and 3 503 | 4 534 | 2 266 and 2 268 | 2 339 | 1 181 and 1 158 | 13 903 | 13 903 | 7 345 | 21 248 | 26 240 |

The synapses from A's, B's and the pair's own units consolidate as in H-47 under every form, the zero sum's rounds with them, and so does
every inhibitory synapse.

**Four forms a cell, in one job** (`e028_job`), exploration 027's job with its two held forms replaced by three
([ADR-0243](../adr/0243-the-part-held-within-split-by-target.md)):
- **the night without replay**, every row held inside the test to the row H-47 pinned;
- **the night held onto the own group** (own group, in the tables), H-47's plain night on A's patterns with `OwnGroup` set;
- **the night held onto the inhibitory units** (inhibitory units), the same with `Inhibitory` set;
- **the night held onto the units outside the readouts** (outside the readouts), the same with `Outside` set.

The job is exploration 020's to 027's: A's patterns, the split and the class matrix of the start image, of the image before each night and
of the image after it, and the nights after blocks 6, 12, 18 and 24 watched by `ReplayWatch`.

**At each held night** (`e028_night`) the job reads every excitatory synapse three times: before the night's first tick, at its last, and
after the quiet ticks, the awake ticks without drive that H-47's harness runs until the engine is quiescent, before it writes the night's
image. At the last it holds, inside the test, that every synapse of the held part stands at its first weight with its trace cleared, that
some synapse of the held units outside the part moved, and that some excitatory synapse from a unit outside the hold moved; it prints, for
each other part of the four, the other two parts of the part held within and the part held across, how many of its synapses moved, their
change, and how many carry a trace that is not zero. These are counts: the jobs print no trace itself. After the quiet ticks it prints what
they moved, and what the held part's synapses would read at their presynaptic unit's next spike from the spikes on record since the night's
first tick, if that spike came before the target's next one: for the gain and for the loss, how many synapses carry one, their sum and the
largest.

**The excitatory rule** (`step_stdp_shaped`, in `crates/menteion-core/src/dynamics/synapse.rs`) reads two pairings at a presynaptic spike
at $t$, with $p$ the presynaptic unit's previous spike and $q$ the target's last spike: if $p < q \le t$, a gain of
$A_+ (1 - 2^{-11})^{q - p}$; and if $q < t$, a loss of $A_- (1 - 2^{-11})^{t - q}$ scaled by the slot's magnitude, by the exponent the
configuration sets ([ADR-0228](../adr/0228-a-zero-sum-at-the-stimulus-built.md)). `step_stdp_all_shaped` runs on every slot of a held
part as on any other, so its trace takes the night's pairings; only the consolidation is skipped. Both pairings are read only at a
presynaptic spike, so the night's last pairings escape the clearing. The job finds each with the rule itself, on a copy of the block, the
slot's trace set to zero; with $t_0$ the first tick after the image:
- **the pending gain**: a synapse of the held part whose target's last spike and presynaptic unit's last spike are both on record since the
  night's first tick, $q > p$: $A_+ (1 - 2^{-11})^{q - p}$, read at the presynaptic unit's next spike if it comes before the target's next
  spike;
- **the pending loss**: a synapse of the held part whose target fired since the night's first tick: the loss a presynaptic spike at $t_0$
  would read, at the synapse's magnitude after the quiet ticks. It falls as $t$ grows, so it is the most a later presynaptic spike reads, if
  that spike comes before the target's next one.

A target spike before the next presynaptic one takes $q$'s place, and both readings change with it.

**The class matrix** is exploration 020's, as 021 reads it by readout and 023 to 027 by group. Each group's excitatory units are its
readout's two classes and the pair's own kind inside it, C's own in the odd places and D's in the even. An inhibitory entry is a
magnitude, the weight negated, so a rise is more inhibition. The held units' synapses are the rows of classes 6 and 7, the odd places' held
units, and 8 and 9, the even places', onto every class; each part is their entries in its columns above.

## Before a job ran

ADR-0243 asks whether any form but the night without replay must run bit for bit as one already in the logs. **None must.** Each held form
holds one part of the part held within and lets the other two and the part held across consolidate. Over four nights, the mean over the
three paths, a group's held units' synapses moved, in each of the twelve cells:

| Form, its job | Onto the own group | Onto the inhibitory units | Onto the units outside the readouts | Held across |
| :--- | ---: | ---: | ---: | ---: |
| H-47's plain night on A's patterns (exploration 020's and 021's jobs) | −2.79 to −0.02 M | −2.71 to −1.45 M | −1.01 to +0.09 M | −6.98 to −4.73 M |
| Exploration 027's night held across | −2.96 to −0.27 M | −2.87 to −1.45 M | −1.18 to −0.05 M | 0.00, exactly |
| Exploration 027's night held within | 0.00, exactly | 0.00, exactly | 0.00, exactly | −6.84 to −4.80 M |
| The excitatory night (exploration 025's and 026's jobs) | 0.00, exactly | 0.00, exactly | 0.00, exactly | 0.00, exactly |

None of the 24 groups' entries is zero under the plain night or under the night held across, in any of the three parts. So each held form
holds synapses that the plain night and exploration 027's night held across move, and lets consolidate synapses that exploration 027's night
held within and the excitatory night hold. Exploration 024's hold holds no excitatory synapse, and the forms in the logs without replay run
no replay. The trial then printed each held form's row at block 6 on path 0 on seed 23's first mapping from the assignment beside H-47's
plain night's and beside exploration 027's trial's night held within's on the same cell: none is the same, as the trial prints. So all four
forms ran.

## The runs are laid out as written

- **The trial** ran on path 0, on seed 23's first mapping from the assignment: the pair's head and the first night of H-47's plain night on
  A's patterns, the night without replay, each of the three held nights and the plain night with the three parts held together, all
  watched. With no hold set, the night without replay and the plain night held H-47's row at block 6 on path 0, inside the test. On the
  head's image the three parts were disjoint and together exactly exploration 027's trial's 13 772 synapses held within, synapse by synapse,
  each target in its part's columns, held inside the test: 6 919 onto the own group, 3 498 from the odd places and 3 421 from the even;
  4 487 onto the inhibitory units, 2 200 and 2 287; and 2 366 onto the units outside the readouts, 1 221 and 1 145. The part held across
  is 7 476, 3 705 and 3 771.
- With each part's hold set, the night held, inside the test, every synapse of that part at its first weight with its trace cleared, a
  synapse of each other part of the part held within moved and, at the night's last tick, a synapse of the held units outside the part with
  a trace that is not zero. Each other part's synapses that moved and those with a trace at the night's last tick, as the trial prints them:

| Night | Onto the own group: moved, of 6 919; with a trace | Onto the inhibitory units: of 4 487 | Onto the units outside the readouts: of 2 366 | Held across: of 7 476 |
| :--- | ---: | ---: | ---: | ---: |
| Held onto the own group | held | 4 477; 2 448 | 2 364; 1 276 | 7 457; 3 175 |
| Held onto the inhibitory units | 6 894; 3 684 | held | 2 362; 1 248 | 7 458; 3 163 |
| Held onto the units outside the readouts | 6 895; 3 702 | 4 481; 2 470 | held | 7 459; 3 186 |
| The three held together | held | held | held | 7 457; 3 203 |

- **With the three parts held together** the plain night on A's patterns held, inside the test, the row exploration 027's trial printed for
  its night held within at block 6 on path 0 (run 42512634982): the image's CRC-64 16346174311379552951, A's first count of 57, its
  couplings and the gain. It cleared what exploration 027's trial printed its night held within clearing, 13 772 slots, 8 646 of them with a
  trace, 647 599 LSB above zero and −861 852 below, and the part held across moved 7 457 synapses with 3 203 carrying a trace, as there.
- The three held forms' rows at block 6 on path 0 are neither H-47's plain night's nor exploration 027's trial's night held within's, as
  the trial prints: each image's CRC-64, A's couplings and the gain differ. A's first count is 57 under the nights held onto the own group
  and onto the inhibitory units and 58 under the night held onto the units outside the readouts, where H-47's plain night and exploration
  027's night held within read 57. The watch read `wrong 0` in the trial's six nights. Each of the five nights with replay read 1 339
  replays and 640 depotentiations, and the night without replay none.
- **Each of the twelve jobs** ran to the end. Their reproduction, cell by cell:

| Cell | Without replay: rows H-47's | Own group, inhibitory units, outside the readouts: rows H-47's plain on A's | rows 027's held within | rows the excitatory night's | rows 027's held across | Without replay: its paths' lines in the plain night's job, label aside | Heads: lines in that job | Plain night's lines: job | Held nights: the part's hold held inside the test |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | :--- | ---: |
| Seed 22, eighth, mirrored | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 632 of 1 632 | 3 of 3 | 021 | 36 of 36 |
| Seed 22, first mapping, assignment | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 632 of 1 632 | 3 of 3 | 021 | 36 of 36 |
| Seed 23, eighth, mirrored | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 680 of 1 680 | 3 of 3 | 021 | 36 of 36 |
| Seed 23, first mapping, assignment | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 680 of 1 680 | 3 of 3 | 021 | 36 of 36 |
| Seed 24, eighth, mirrored | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 656 of 1 656 | 3 of 3 | 021 | 36 of 36 |
| Seed 24, first mapping, assignment | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 656 of 1 656 | 3 of 3 | 021 | 36 of 36 |
| Seed 23, eighth, assignment | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 680 of 1 680 | 3 of 3 | 021 | 36 of 36 |
| Seed 24, first mapping, mirrored | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 656 of 1 656 | 3 of 3 | 021 | 36 of 36 |
| Seed 23, first mapping, mirrored | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 680 of 1 680 | 3 of 3 | 020 | 36 of 36 |
| Seed 24, eighth, assignment | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 656 of 1 656 | 3 of 3 | 020 | 36 of 36 |
| Seed 22, first mapping, mirrored | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 632 of 1 632 | 3 of 3 | 020 | 36 of 36 |
| Seed 22, eighth, assignment | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 632 of 1 632 | 3 of 3 | 020 | 36 of 36 |

- The night without replay's `DUMP` lines, its paths' 1 632 to 1 680 a cell, its 15 rows' and its notice, are the lines of H-47's plain
  night's class matrix job on the same cell, label aside, all of them in each cell: exploration 021's run for exploration 026's eight cells
  and exploration 020's runs for exploration 020's four. So are the pair's head's three. Each read of the night without replay carries
  H-47's read's `stimuli`, `right` and `tied` masks, all 480 a cell. Each held form's reads carry H-47's plain night's `stimuli` masks, so
  each trial presented the stimulus it presented in H-47.
- For each held form the job prints how many of its 15 rows are H-47's plain night's rows: **none, in all twelve cells**, as the job
  prints; nothing in the job asserts it. Set beside exploration 027's rows for its night held within and its night held across and beside
  the excitatory night's rows in exploration 025's or 026's log, row by row with the form's name left out, none of the three held forms'
  rows is the same, and none of the three is the same as another's in its own job.
- **The watch read `wrong 0` in all 576 nights.** Every night without replay read `replays 0` and `depotentiations 0`. The nights held
  onto the inhibitory units and onto the units outside the readouts read H-47's plain night's replays and depotentiations, as the plain
  night's class matrix job printed them, in 142 of their 144 nights each, and the night held onto the own group in 140 of its 144. All the
  others are on seed 23's eighth image from the mirrored assignment. On path 0 each of the three held nights read 1 202 replays and 571
  depotentiations after block 18, where the plain night read 1 202 and 570, and 1 129 and 545 after block 24, where the plain night read
  1 128 and 545: there they read exploration 027's night held within's counts and the excitatory night's. On path 5 the night held onto the
  own group read 1 202 and 570 after block 18 and 1 128 and 545 after block 24, where the plain night, exploration 027's night held within
  and the excitatory night read 1 202 and 571 and 1 129 and 545. So the nights held onto the inhibitory units and onto the units outside the
  readouts read exploration 027's night held within's counts in all 144 of their nights each, and the night held onto the own group in 142.
- Every held unit's synapse was read at each night: 664 held units, 332 a readout, C's own 25 and D's own 25 left out, as each job prints.
  Each job holds that none of the zero sum's 152 sources is among them. Each held night held its part's hold inside the test, 36 a cell.

## 1. A's count under each held night

A's count at each checkpoint under each form. H-47's plain night on A's patterns is H-47's job's, exploration 027's night held within is
exploration 027's job's, and the excitatory night is exploration 025's or 026's job's:

| Cell | Block | Without replay | Own group | Inhibitory units | Outside the readouts | Plain on A's (H-47) | Held within (027) | Excitatory (025, 026) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 46.3 | 53.1 | 52.9 | 53.3 | 53.3 | 52.6 | 52.4 |
| Seed 22, eighth, mirrored | 12 | 30.2 | 41.0 | 40.7 | 42.7 | 42.3 | 40.9 | 41.0 |
| Seed 22, eighth, mirrored | 18 | 26.3 | 40.3 | 40.6 | 41.8 | 41.8 | 40.5 | 40.9 |
| Seed 22, eighth, mirrored | 24 | 22.6 | 37.5 | 35.3 | 42.0 | 40.2 | 37.0 | 37.1 |
| Seed 22, eighth, mirrored | 36 | 16.6 | 33.4 | 32.6 | 37.3 | 36.2 | 33.3 | 32.8 |
| Seed 22, first mapping, assignment | 6 | 52.4 | 56.7 | 56.3 | 56.5 | 56.5 | 56.7 | 56.8 |
| Seed 22, first mapping, assignment | 12 | 44.8 | 47.8 | 46.9 | 48.4 | 47.6 | 47.3 | 45.8 |
| Seed 22, first mapping, assignment | 18 | 40.2 | 48.1 | 46.8 | 48.1 | 46.9 | 46.6 | 46.2 |
| Seed 22, first mapping, assignment | 24 | 38.9 | 49.9 | 45.8 | 48.8 | 48.0 | 47.3 | 45.6 |
| Seed 22, first mapping, assignment | 36 | 36.3 | 46.6 | 42.4 | 44.6 | 44.3 | 42.8 | 41.5 |
| Seed 23, eighth, mirrored | 6 | 34.4 | 36.6 | 36.1 | 35.8 | 36.1 | 35.4 | 36.8 |
| Seed 23, eighth, mirrored | 12 | 19.5 | 27.5 | 26.1 | 27.6 | 28.5 | 26.3 | 25.5 |
| Seed 23, eighth, mirrored | 18 | 14.9 | 26.5 | 24.6 | 27.0 | 27.6 | 23.4 | 24.0 |
| Seed 23, eighth, mirrored | 24 | 10.6 | 28.0 | 25.9 | 28.5 | 29.3 | 26.0 | 25.2 |
| Seed 23, eighth, mirrored | 36 | 8.2 | 25.2 | 23.5 | 25.1 | 25.6 | 23.7 | 22.9 |
| Seed 23, first mapping, assignment | 6 | 54.2 | 56.4 | 56.4 | 56.1 | 56.3 | 56.3 | 56.1 |
| Seed 23, first mapping, assignment | 12 | 33.4 | 38.1 | 37.6 | 38.0 | 37.6 | 39.0 | 37.5 |
| Seed 23, first mapping, assignment | 18 | 27.2 | 34.4 | 34.4 | 34.2 | 35.0 | 34.8 | 35.1 |
| Seed 23, first mapping, assignment | 24 | 24.8 | 34.1 | 34.6 | 34.1 | 34.8 | 35.8 | 35.0 |
| Seed 23, first mapping, assignment | 36 | 19.7 | 32.4 | 32.3 | 32.6 | 33.0 | 33.0 | 32.3 |
| Seed 24, eighth, mirrored | 6 | 36.4 | 33.4 | 34.9 | 34.0 | 33.8 | 35.1 | 36.6 |
| Seed 24, eighth, mirrored | 12 | 31.5 | 29.0 | 31.3 | 29.6 | 28.6 | 33.8 | 33.6 |
| Seed 24, eighth, mirrored | 18 | 20.2 | 30.1 | 30.7 | 29.4 | 28.6 | 35.2 | 34.4 |
| Seed 24, eighth, mirrored | 24 | 16.4 | 25.1 | 28.7 | 27.0 | 25.8 | 32.9 | 33.9 |
| Seed 24, eighth, mirrored | 36 | 12.5 | 24.0 | 25.4 | 24.5 | 23.8 | 28.1 | 29.2 |
| Seed 24, first mapping, assignment | 6 | 58.3 | 61.3 | 61.6 | 61.5 | 61.2 | 61.6 | 61.1 |
| Seed 24, first mapping, assignment | 12 | 52.0 | 55.8 | 54.8 | 54.8 | 54.8 | 57.0 | 55.7 |
| Seed 24, first mapping, assignment | 18 | 47.5 | 52.4 | 51.9 | 51.5 | 51.3 | 52.4 | 52.5 |
| Seed 24, first mapping, assignment | 24 | 43.7 | 53.1 | 53.1 | 53.3 | 52.4 | 53.7 | 52.7 |
| Seed 24, first mapping, assignment | 36 | 40.1 | 44.5 | 46.6 | 44.9 | 43.8 | 46.2 | 45.8 |
| Seed 23, eighth, assignment | 6 | 47.4 | 50.1 | 50.5 | 50.2 | 50.4 | 50.4 | 50.7 |
| Seed 23, eighth, assignment | 12 | 40.8 | 44.2 | 45.9 | 43.5 | 43.4 | 47.2 | 47.5 |
| Seed 23, eighth, assignment | 18 | 37.3 | 44.7 | 47.8 | 48.6 | 45.2 | 50.0 | 50.1 |
| Seed 23, eighth, assignment | 24 | 38.0 | 44.9 | 47.6 | 47.3 | 43.7 | 52.1 | 51.0 |
| Seed 23, eighth, assignment | 36 | 36.2 | 40.6 | 42.9 | 42.7 | 40.3 | 46.4 | 47.1 |
| Seed 24, first mapping, mirrored | 6 | 54.3 | 57.1 | 57.4 | 57.5 | 57.7 | 57.2 | 57.9 |
| Seed 24, first mapping, mirrored | 12 | 46.3 | 51.3 | 50.0 | 50.7 | 51.0 | 50.7 | 52.3 |
| Seed 24, first mapping, mirrored | 18 | 44.0 | 52.3 | 51.0 | 52.0 | 51.8 | 51.5 | 53.3 |
| Seed 24, first mapping, mirrored | 24 | 44.4 | 53.0 | 52.2 | 52.6 | 52.0 | 53.3 | 53.7 |
| Seed 24, first mapping, mirrored | 36 | 42.7 | 49.8 | 49.5 | 49.7 | 48.7 | 50.4 | 50.1 |
| Seed 23, first mapping, mirrored | 6 | 55.0 | 56.5 | 56.9 | 56.8 | 56.9 | 56.6 | 56.6 |
| Seed 23, first mapping, mirrored | 12 | 52.4 | 53.5 | 55.6 | 55.2 | 54.4 | 54.8 | 55.5 |
| Seed 23, first mapping, mirrored | 18 | 52.0 | 51.5 | 54.4 | 52.1 | 52.4 | 53.8 | 54.2 |
| Seed 23, first mapping, mirrored | 24 | 53.5 | 49.8 | 52.5 | 51.2 | 50.9 | 52.6 | 53.3 |
| Seed 23, first mapping, mirrored | 36 | 53.1 | 48.6 | 50.1 | 48.5 | 48.3 | 50.5 | 51.4 |
| Seed 24, eighth, assignment | 6 | 38.8 | 37.6 | 38.8 | 38.8 | 39.1 | 38.3 | 39.3 |
| Seed 24, eighth, assignment | 12 | 34.8 | 33.4 | 34.1 | 35.1 | 34.2 | 34.5 | 36.8 |
| Seed 24, eighth, assignment | 18 | 34.4 | 29.9 | 30.0 | 31.7 | 28.5 | 31.6 | 35.6 |
| Seed 24, eighth, assignment | 24 | 35.6 | 29.7 | 29.7 | 31.7 | 29.6 | 32.4 | 36.7 |
| Seed 24, eighth, assignment | 36 | 25.6 | 26.3 | 27.0 | 29.1 | 27.4 | 29.7 | 34.4 |
| Seed 22, first mapping, mirrored | 6 | 56.6 | 58.0 | 58.2 | 57.4 | 57.7 | 58.2 | 58.1 |
| Seed 22, first mapping, mirrored | 12 | 49.5 | 52.6 | 53.1 | 53.9 | 52.5 | 53.0 | 52.7 |
| Seed 22, first mapping, mirrored | 18 | 49.0 | 52.5 | 53.3 | 54.1 | 52.7 | 53.2 | 52.3 |
| Seed 22, first mapping, mirrored | 24 | 45.9 | 51.3 | 50.8 | 52.3 | 50.6 | 51.8 | 50.7 |
| Seed 22, first mapping, mirrored | 36 | 44.1 | 49.6 | 49.0 | 50.2 | 49.1 | 49.7 | 48.8 |
| Seed 22, eighth, assignment | 6 | 50.3 | 52.2 | 52.4 | 52.7 | 52.3 | 52.6 | 52.6 |
| Seed 22, eighth, assignment | 12 | 47.6 | 51.1 | 51.6 | 51.4 | 50.6 | 52.9 | 52.8 |
| Seed 22, eighth, assignment | 18 | 47.2 | 50.3 | 51.6 | 52.2 | 50.6 | 51.8 | 51.8 |
| Seed 22, eighth, assignment | 24 | 47.3 | 49.8 | 50.9 | 51.5 | 50.2 | 52.6 | 52.1 |
| Seed 22, eighth, assignment | 36 | 44.2 | 45.7 | 47.5 | 48.4 | 46.4 | 48.7 | 48.8 |

The differences between the forms run in one job, each with the job's paired SE. The night without replay is H-47's, read by read, so
"H-47: plain less without replay" at block 36 is H-47's clause 1:

| Cell | Block | Own group less without replay | Inhibitory units less without replay | Outside the readouts less without replay | H-47: plain less without replay | 027: held within less without replay | Excitatory less without replay (025, 026) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +6.9 ± 0.92 | +6.6 ± 0.92 | +7.0 ± 0.82 | +7.1 ± 0.91 | +6.3 ± 0.87 | +6.1 ± 0.98 |
| Seed 22, eighth, mirrored | 12 | +10.8 ± 1.53 | +10.5 ± 1.54 | +12.5 ± 1.09 | +12.1 ± 0.85 | +10.7 ± 1.17 | +10.8 ± 0.98 |
| Seed 22, eighth, mirrored | 18 | +14.0 ± 2.34 | +14.3 ± 1.72 | +15.5 ± 1.48 | +15.5 ± 1.67 | +14.3 ± 1.80 | +14.6 ± 1.78 |
| Seed 22, eighth, mirrored | 24 | +14.9 ± 1.90 | +12.7 ± 1.13 | +19.4 ± 1.66 | +17.6 ± 1.49 | +14.4 ± 1.75 | +14.5 ± 1.46 |
| Seed 22, eighth, mirrored | 36 | +16.8 ± 0.88 | +16.1 ± 1.18 | +20.8 ± 0.64 | +19.6 ± 1.06 | +16.8 ± 0.83 | +16.3 ± 0.84 |
| Seed 22, first mapping, assignment | 6 | +4.3 ± 0.87 | +3.9 ± 1.00 | +4.1 ± 0.83 | +4.1 ± 0.82 | +4.3 ± 1.22 | +4.4 ± 1.10 |
| Seed 22, first mapping, assignment | 12 | +3.0 ± 0.75 | +2.2 ± 0.59 | +3.6 ± 0.67 | +2.9 ± 0.56 | +2.6 ± 0.74 | +1.0 ± 1.21 |
| Seed 22, first mapping, assignment | 18 | +7.9 ± 1.73 | +6.7 ± 0.99 | +8.0 ± 1.08 | +6.8 ± 1.12 | +6.4 ± 1.05 | +6.1 ± 1.11 |
| Seed 22, first mapping, assignment | 24 | +11.0 ± 1.11 | +6.9 ± 1.40 | +9.9 ± 1.55 | +9.1 ± 1.37 | +8.4 ± 1.05 | +6.7 ± 1.33 |
| Seed 22, first mapping, assignment | 36 | +10.2 ± 1.20 | +6.0 ± 0.78 | +8.3 ± 0.29 | +7.9 ± 0.51 | +6.5 ± 1.42 | +5.2 ± 1.24 |
| Seed 23, eighth, mirrored | 6 | +2.1 ± 1.34 | +1.7 ± 1.19 | +1.4 ± 1.05 | +1.7 ± 1.20 | +1.0 ± 1.17 | +2.4 ± 1.08 |
| Seed 23, eighth, mirrored | 12 | +8.0 ± 0.92 | +6.6 ± 0.58 | +8.1 ± 0.67 | +9.0 ± 1.07 | +6.8 ± 0.77 | +6.0 ± 0.97 |
| Seed 23, eighth, mirrored | 18 | +11.6 ± 0.60 | +9.8 ± 1.21 | +12.1 ± 0.97 | +12.7 ± 1.02 | +8.6 ± 1.29 | +9.1 ± 1.46 |
| Seed 23, eighth, mirrored | 24 | +17.4 ± 1.16 | +15.3 ± 1.70 | +17.9 ± 1.93 | +18.7 ± 1.62 | +15.4 ± 1.64 | +14.6 ± 1.96 |
| Seed 23, eighth, mirrored | 36 | +17.0 ± 0.92 | +15.3 ± 0.91 | +17.0 ± 0.69 | +17.4 ± 1.08 | +15.5 ± 1.06 | +14.7 ± 1.54 |
| Seed 23, first mapping, assignment | 6 | +2.2 ± 0.80 | +2.1 ± 0.82 | +1.9 ± 0.79 | +2.0 ± 0.79 | +2.1 ± 0.89 | +1.9 ± 0.87 |
| Seed 23, first mapping, assignment | 12 | +4.7 ± 1.05 | +4.2 ± 1.13 | +4.6 ± 1.37 | +4.1 ± 1.18 | +5.5 ± 1.09 | +4.1 ± 0.95 |
| Seed 23, first mapping, assignment | 18 | +7.2 ± 1.96 | +7.2 ± 2.48 | +7.0 ± 2.30 | +7.9 ± 2.54 | +7.6 ± 2.55 | +7.9 ± 2.40 |
| Seed 23, first mapping, assignment | 24 | +9.3 ± 1.67 | +9.9 ± 2.08 | +9.3 ± 2.12 | +10.0 ± 1.90 | +11.0 ± 1.93 | +10.3 ± 2.20 |
| Seed 23, first mapping, assignment | 36 | +12.7 ± 1.26 | +12.6 ± 1.64 | +12.9 ± 1.65 | +13.4 ± 1.56 | +13.3 ± 1.64 | +12.6 ± 1.56 |
| Seed 24, eighth, mirrored | 6 | −3.0 ± 1.32 | −1.5 ± 1.27 | −2.4 ± 1.16 | −2.6 ± 1.35 | −1.3 ± 1.35 | +0.3 ± 1.15 |
| Seed 24, eighth, mirrored | 12 | −2.6 ± 1.25 | −0.2 ± 1.33 | −1.9 ± 1.08 | −2.9 ± 1.19 | +2.3 ± 1.32 | +2.1 ± 1.50 |
| Seed 24, eighth, mirrored | 18 | +9.9 ± 1.19 | +10.5 ± 1.54 | +9.1 ± 1.86 | +8.3 ± 1.70 | +15.0 ± 1.31 | +14.2 ± 1.34 |
| Seed 24, eighth, mirrored | 24 | +8.7 ± 1.93 | +12.3 ± 2.40 | +10.5 ± 2.30 | +9.4 ± 1.58 | +16.5 ± 1.98 | +17.5 ± 1.30 |
| Seed 24, eighth, mirrored | 36 | +11.5 ± 1.28 | +12.9 ± 1.03 | +12.0 ± 1.87 | +11.3 ± 0.96 | +15.5 ± 1.27 | +16.7 ± 1.23 |
| Seed 24, first mapping, assignment | 6 | +3.0 ± 0.70 | +3.3 ± 0.68 | +3.2 ± 0.71 | +2.9 ± 0.72 | +3.4 ± 0.71 | +2.8 ± 0.68 |
| Seed 24, first mapping, assignment | 12 | +3.8 ± 0.77 | +2.9 ± 0.96 | +2.8 ± 0.78 | +2.8 ± 0.93 | +5.0 ± 1.06 | +3.7 ± 1.16 |
| Seed 24, first mapping, assignment | 18 | +5.0 ± 1.55 | +4.5 ± 0.89 | +4.0 ± 1.41 | +3.9 ± 1.41 | +5.0 ± 1.01 | +5.0 ± 1.19 |
| Seed 24, first mapping, assignment | 24 | +9.4 ± 2.85 | +9.4 ± 2.77 | +9.5 ± 2.45 | +8.7 ± 3.17 | +10.0 ± 2.04 | +9.0 ± 2.46 |
| Seed 24, first mapping, assignment | 36 | +4.4 ± 1.15 | +6.5 ± 1.02 | +4.8 ± 1.56 | +3.8 ± 1.57 | +6.1 ± 1.46 | +5.8 ± 1.60 |
| Seed 23, eighth, assignment | 6 | +2.8 ± 0.97 | +3.2 ± 0.98 | +2.8 ± 1.06 | +3.0 ± 0.96 | +3.0 ± 1.00 | +3.4 ± 1.07 |
| Seed 23, eighth, assignment | 12 | +3.3 ± 1.21 | +5.0 ± 1.12 | +2.7 ± 1.37 | +2.6 ± 0.94 | +6.4 ± 1.53 | +6.7 ± 1.57 |
| Seed 23, eighth, assignment | 18 | +7.4 ± 1.40 | +10.5 ± 1.00 | +11.2 ± 1.25 | +7.9 ± 0.90 | +12.7 ± 0.97 | +12.7 ± 1.45 |
| Seed 23, eighth, assignment | 24 | +6.9 ± 2.16 | +9.6 ± 2.10 | +9.3 ± 1.33 | +5.7 ± 1.34 | +14.1 ± 1.57 | +13.0 ± 2.14 |
| Seed 23, eighth, assignment | 36 | +4.5 ± 2.81 | +6.7 ± 2.53 | +6.5 ± 1.93 | +4.1 ± 1.45 | +10.2 ± 1.77 | +10.9 ± 2.49 |
| Seed 24, first mapping, mirrored | 6 | +2.8 ± 0.82 | +3.1 ± 0.82 | +3.2 ± 0.79 | +3.4 ± 0.79 | +2.9 ± 0.83 | +3.6 ± 0.88 |
| Seed 24, first mapping, mirrored | 12 | +4.9 ± 1.51 | +3.7 ± 1.31 | +4.4 ± 1.52 | +4.7 ± 1.50 | +4.4 ± 1.70 | +6.0 ± 1.30 |
| Seed 24, first mapping, mirrored | 18 | +8.3 ± 0.78 | +7.0 ± 1.07 | +8.0 ± 0.99 | +7.8 ± 0.89 | +7.5 ± 0.84 | +9.3 ± 0.89 |
| Seed 24, first mapping, mirrored | 24 | +8.7 ± 0.94 | +7.8 ± 1.69 | +8.2 ± 1.09 | +7.6 ± 1.32 | +8.9 ± 1.12 | +9.3 ± 1.35 |
| Seed 24, first mapping, mirrored | 36 | +7.0 ± 1.77 | +6.7 ± 1.57 | +7.0 ± 1.46 | +6.0 ± 1.54 | +7.7 ± 1.37 | +7.4 ± 1.09 |
| Seed 23, first mapping, mirrored | 6 | +1.5 ± 0.76 | +1.9 ± 0.83 | +1.8 ± 0.64 | +1.9 ± 0.68 | +1.6 ± 0.76 | +1.5 ± 0.76 |
| Seed 23, first mapping, mirrored | 12 | +1.1 ± 1.28 | +3.3 ± 1.14 | +2.8 ± 1.12 | +2.0 ± 1.26 | +2.4 ± 0.97 | +3.1 ± 1.21 |
| Seed 23, first mapping, mirrored | 18 | −0.5 ± 1.15 | +2.4 ± 0.94 | +0.1 ± 1.20 | +0.3 ± 1.17 | +1.8 ± 1.04 | +2.2 ± 1.23 |
| Seed 23, first mapping, mirrored | 24 | −3.7 ± 0.42 | −1.0 ± 1.37 | −2.3 ± 1.40 | −2.6 ± 1.30 | −0.9 ± 0.64 | −0.1 ± 0.80 |
| Seed 23, first mapping, mirrored | 36 | −4.6 ± 0.95 | −3.0 ± 0.78 | −4.7 ± 0.66 | −4.9 ± 1.09 | −2.6 ± 0.65 | −1.7 ± 0.42 |
| Seed 24, eighth, assignment | 6 | −1.2 ± 0.65 | 0.0 ± 0.90 | 0.0 ± 0.83 | +0.3 ± 0.64 | −0.5 ± 0.77 | +0.5 ± 0.67 |
| Seed 24, eighth, assignment | 12 | −1.4 ± 1.51 | −0.8 ± 1.01 | +0.3 ± 1.09 | −0.6 ± 1.03 | −0.3 ± 1.17 | +2.0 ± 0.89 |
| Seed 24, eighth, assignment | 18 | −4.6 ± 1.46 | −4.4 ± 2.52 | −2.7 ± 1.37 | −5.9 ± 1.53 | −2.9 ± 2.14 | +1.2 ± 2.06 |
| Seed 24, eighth, assignment | 24 | −5.9 ± 1.78 | −5.8 ± 2.36 | −3.8 ± 2.51 | −5.9 ± 1.85 | −3.1 ± 2.59 | +1.1 ± 1.88 |
| Seed 24, eighth, assignment | 36 | +0.7 ± 1.29 | +1.4 ± 0.82 | +3.5 ± 2.31 | +1.8 ± 1.79 | +4.1 ± 1.17 | +8.8 ± 1.28 |
| Seed 22, first mapping, mirrored | 6 | +1.3 ± 0.52 | +1.5 ± 0.61 | +0.7 ± 0.65 | +1.1 ± 0.63 | +1.6 ± 0.52 | +1.5 ± 0.59 |
| Seed 22, first mapping, mirrored | 12 | +3.1 ± 1.87 | +3.6 ± 1.84 | +4.3 ± 2.02 | +2.9 ± 1.93 | +3.5 ± 1.96 | +3.2 ± 1.36 |
| Seed 22, first mapping, mirrored | 18 | +3.5 ± 2.13 | +4.4 ± 2.35 | +5.1 ± 1.89 | +3.7 ± 2.22 | +4.3 ± 2.29 | +3.4 ± 2.15 |
| Seed 22, first mapping, mirrored | 24 | +5.4 ± 2.39 | +4.9 ± 2.27 | +6.4 ± 2.05 | +4.7 ± 2.24 | +5.9 ± 2.04 | +4.8 ± 1.82 |
| Seed 22, first mapping, mirrored | 36 | +5.4 ± 1.25 | +4.8 ± 1.23 | +6.0 ± 1.15 | +5.0 ± 1.24 | +5.6 ± 1.68 | +4.7 ± 1.52 |
| Seed 22, eighth, assignment | 6 | +1.9 ± 0.75 | +2.1 ± 0.65 | +2.4 ± 0.70 | +1.9 ± 0.65 | +2.3 ± 0.98 | +2.3 ± 0.81 |
| Seed 22, eighth, assignment | 12 | +3.5 ± 0.95 | +4.0 ± 0.72 | +3.8 ± 0.81 | +3.0 ± 0.93 | +5.3 ± 0.82 | +5.2 ± 0.79 |
| Seed 22, eighth, assignment | 18 | +3.1 ± 1.23 | +4.4 ± 0.77 | +5.0 ± 0.59 | +3.4 ± 1.05 | +4.6 ± 0.80 | +4.6 ± 1.28 |
| Seed 22, eighth, assignment | 24 | +2.4 ± 1.26 | +3.5 ± 1.13 | +4.2 ± 1.45 | +2.8 ± 0.89 | +5.2 ± 1.18 | +4.8 ± 1.01 |
| Seed 22, eighth, assignment | 36 | +1.5 ± 1.08 | +3.3 ± 0.36 | +4.2 ± 0.84 | +2.2 ± 0.65 | +4.5 ± 1.26 | +4.6 ± 0.66 |

- **Each held night less the night without replay**: the night held onto the own group lies between −5.9 and +17.4, beyond twice its SE
  at 48 of the 60 checkpoints, 42 above and 6 below; the night held onto the inhibitory units between −5.8 and +16.1, beyond at 50, 48
  above and 2 below; and the night held onto the units outside the readouts between −4.7 and +20.8, beyond at 49, 47 above and 2 below.
  H-47's plain night lies beyond at 49 of the 60, 45 above and 4 below, exploration 027's night held within at 49, 48 above and 1 below,
  and the excitatory night at 51, 50 above and 1 below.
- **Below the night without replay**, the night held onto the own group lies on seed 24's eighth image from the mirrored assignment at
  blocks 6 and 12, −3.0 ± 1.32 and −2.6 ± 1.25, on seed 23's first mapping from the mirrored assignment at blocks 24 and 36, −3.7 ± 0.42
  and −4.6 ± 0.95, and on seed 24's eighth image from the assignment at blocks 18 and 24, −4.6 ± 1.46 and −5.9 ± 1.78; the night held onto
  the inhibitory units on seed 23's first mapping from the mirrored assignment at block 36, −3.0 ± 0.78, and on seed 24's eighth image from
  the assignment at block 24, −5.8 ± 2.36; and the night held onto the units outside the readouts on seed 24's eighth image from the
  mirrored assignment at block 6, −2.4 ± 1.16, and on seed 23's first mapping from the mirrored assignment at block 36, −4.7 ± 0.66. H-47's
  plain night lies below on seed 24's eighth image from the mirrored assignment at block 12, −2.9 ± 1.19, on seed 23's first mapping from
  the mirrored assignment at block 36, −4.9 ± 1.09, and on seed 24's eighth image from the assignment at blocks 18 and 24, −5.9 ± 1.53 and
  −5.9 ± 1.85.

The three held nights less each other, with the job's paired SE:

| Cell | Block | Own group less inhibitory units | Own group less outside the readouts | Inhibitory units less outside the readouts |
| :--- | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +0.2 ± 0.24 | −0.2 ± 0.31 | −0.4 ± 0.29 |
| Seed 22, eighth, mirrored | 12 | +0.3 ± 0.25 | −1.7 ± 0.50 | −2.0 ± 0.44 |
| Seed 22, eighth, mirrored | 18 | −0.3 ± 1.20 | −1.5 ± 1.09 | −1.2 ± 0.65 |
| Seed 22, eighth, mirrored | 24 | +2.2 ± 1.18 | −4.5 ± 0.59 | −6.7 ± 0.77 |
| Seed 22, eighth, mirrored | 36 | +0.7 ± 1.50 | −3.9 ± 0.30 | −4.7 ± 1.27 |
| Seed 22, first mapping, assignment | 6 | +0.4 ± 0.37 | +0.2 ± 0.16 | −0.2 ± 0.24 |
| Seed 22, first mapping, assignment | 12 | +0.8 ± 0.30 | −0.6 ± 0.41 | −1.4 ± 0.56 |
| Seed 22, first mapping, assignment | 18 | +1.3 ± 0.84 | 0.0 ± 0.93 | −1.3 ± 0.39 |
| Seed 22, first mapping, assignment | 24 | +4.0 ± 0.74 | +1.1 ± 0.66 | −3.0 ± 0.56 |
| Seed 22, first mapping, assignment | 36 | +4.2 ± 0.93 | +2.0 ± 0.85 | −2.2 ± 0.31 |
| Seed 23, eighth, mirrored | 6 | +0.5 ± 0.94 | +0.8 ± 0.59 | +0.3 ± 0.62 |
| Seed 23, eighth, mirrored | 12 | +1.4 ± 0.66 | −0.1 ± 0.95 | −1.6 ± 0.70 |
| Seed 23, eighth, mirrored | 18 | +1.9 ± 1.11 | −0.5 ± 0.50 | −2.4 ± 0.83 |
| Seed 23, eighth, mirrored | 24 | +2.2 ± 0.89 | −0.5 ± 1.43 | −2.6 ± 0.87 |
| Seed 23, eighth, mirrored | 36 | +1.8 ± 0.30 | +0.1 ± 0.46 | −1.7 ± 0.20 |
| Seed 23, first mapping, assignment | 6 | +0.1 ± 0.11 | +0.3 ± 0.02 | +0.2 ± 0.12 |
| Seed 23, first mapping, assignment | 12 | +0.5 ± 0.25 | +0.1 ± 0.42 | −0.4 ± 0.37 |
| Seed 23, first mapping, assignment | 18 | 0.0 ± 0.59 | +0.2 ± 0.63 | +0.2 ± 0.62 |
| Seed 23, first mapping, assignment | 24 | −0.5 ± 0.68 | 0.0 ± 0.53 | +0.6 ± 0.84 |
| Seed 23, first mapping, assignment | 36 | 0.0 ± 0.80 | −0.2 ± 0.44 | −0.3 ± 0.60 |
| Seed 24, eighth, mirrored | 6 | −1.5 ± 0.52 | −0.6 ± 0.44 | +0.9 ± 0.69 |
| Seed 24, eighth, mirrored | 12 | −2.3 ± 0.95 | −0.7 ± 0.96 | +1.6 ± 0.40 |
| Seed 24, eighth, mirrored | 18 | −0.6 ± 0.89 | +0.7 ± 1.01 | +1.4 ± 1.08 |
| Seed 24, eighth, mirrored | 24 | −3.6 ± 0.75 | −1.9 ± 0.90 | +1.8 ± 1.50 |
| Seed 24, eighth, mirrored | 36 | −1.4 ± 0.76 | −0.5 ± 0.75 | +0.9 ± 1.18 |
| Seed 24, first mapping, assignment | 6 | −0.3 ± 0.02 | −0.2 ± 0.10 | +0.1 ± 0.09 |
| Seed 24, first mapping, assignment | 12 | +1.0 ± 0.50 | +1.0 ± 0.58 | 0.0 ± 0.39 |
| Seed 24, first mapping, assignment | 18 | +0.5 ± 0.74 | +1.0 ± 0.55 | +0.5 ± 0.66 |
| Seed 24, first mapping, assignment | 24 | 0.0 ± 0.43 | −0.1 ± 0.92 | −0.1 ± 1.00 |
| Seed 24, first mapping, assignment | 36 | −2.1 ± 0.48 | −0.4 ± 0.44 | +1.7 ± 0.74 |
| Seed 23, eighth, assignment | 6 | −0.4 ± 0.46 | 0.0 ± 0.31 | +0.4 ± 0.28 |
| Seed 23, eighth, assignment | 12 | −1.7 ± 0.67 | +0.7 ± 0.71 | +2.4 ± 0.61 |
| Seed 23, eighth, assignment | 18 | −3.1 ± 0.79 | −3.9 ± 1.09 | −0.7 ± 0.72 |
| Seed 23, eighth, assignment | 24 | −2.7 ± 0.45 | −2.4 ± 0.95 | +0.3 ± 0.95 |
| Seed 23, eighth, assignment | 36 | −2.3 ± 0.44 | −2.0 ± 1.18 | +0.3 ± 0.78 |
| Seed 24, first mapping, mirrored | 6 | −0.3 ± 0.18 | −0.4 ± 0.23 | −0.1 ± 0.25 |
| Seed 24, first mapping, mirrored | 12 | +1.3 ± 0.74 | +0.6 ± 0.72 | −0.7 ± 0.72 |
| Seed 24, first mapping, mirrored | 18 | +1.3 ± 0.82 | +0.3 ± 0.78 | −1.0 ± 0.07 |
| Seed 24, first mapping, mirrored | 24 | +0.9 ± 1.06 | +0.5 ± 0.54 | −0.4 ± 0.97 |
| Seed 24, first mapping, mirrored | 36 | +0.3 ± 0.68 | 0.0 ± 0.75 | −0.3 ± 0.31 |
| Seed 23, first mapping, mirrored | 6 | −0.4 ± 0.17 | −0.3 ± 0.18 | +0.1 ± 0.21 |
| Seed 23, first mapping, mirrored | 12 | −2.2 ± 0.93 | −1.7 ± 0.79 | +0.5 ± 0.45 |
| Seed 23, first mapping, mirrored | 18 | −3.0 ± 0.69 | −0.6 ± 0.66 | +2.3 ± 0.24 |
| Seed 23, first mapping, mirrored | 24 | −2.8 ± 1.23 | −1.5 ± 1.14 | +1.3 ± 0.30 |
| Seed 23, first mapping, mirrored | 36 | −1.5 ± 0.93 | +0.1 ± 0.92 | +1.6 ± 0.11 |
| Seed 24, eighth, assignment | 6 | −1.2 ± 0.65 | −1.2 ± 0.44 | 0.0 ± 0.49 |
| Seed 24, eighth, assignment | 12 | −0.7 ± 1.08 | −1.7 ± 0.77 | −1.0 ± 0.47 |
| Seed 24, eighth, assignment | 18 | −0.1 ± 1.34 | −1.9 ± 0.38 | −1.7 ± 1.29 |
| Seed 24, eighth, assignment | 24 | 0.0 ± 1.02 | −2.0 ± 2.04 | −2.0 ± 1.49 |
| Seed 24, eighth, assignment | 36 | −0.7 ± 0.93 | −2.8 ± 1.11 | −2.1 ± 1.73 |
| Seed 22, first mapping, mirrored | 6 | −0.2 ± 0.19 | +0.6 ± 0.11 | +0.8 ± 0.23 |
| Seed 22, first mapping, mirrored | 12 | −0.5 ± 0.29 | −1.3 ± 1.15 | −0.7 ± 0.86 |
| Seed 22, first mapping, mirrored | 18 | −0.8 ± 0.27 | −1.6 ± 0.67 | −0.8 ± 0.71 |
| Seed 22, first mapping, mirrored | 24 | +0.5 ± 0.24 | −1.0 ± 0.72 | −1.5 ± 0.50 |
| Seed 22, first mapping, mirrored | 36 | +0.6 ± 0.45 | −0.6 ± 0.46 | −1.2 ± 0.68 |
| Seed 22, eighth, assignment | 6 | −0.2 ± 0.32 | −0.5 ± 0.46 | −0.3 ± 0.38 |
| Seed 22, eighth, assignment | 12 | −0.5 ± 0.58 | −0.3 ± 0.37 | +0.2 ± 0.40 |
| Seed 22, eighth, assignment | 18 | −1.3 ± 1.15 | −1.9 ± 1.13 | −0.6 ± 0.34 |
| Seed 22, eighth, assignment | 24 | −1.1 ± 1.02 | −1.8 ± 1.07 | −0.7 ± 0.77 |
| Seed 22, eighth, assignment | 36 | −1.8 ± 0.95 | −2.6 ± 0.60 | −0.9 ± 0.60 |

- **The night held onto the own group less the night held onto the inhibitory units** lies between −3.6 and +4.2, beyond twice its SE at
  22 of the 60 checkpoints, 8 above and 14 below. Above: on seed 22's first mapping from the assignment at blocks 12, 24 and 36,
  +0.8 ± 0.30 to +4.2 ± 0.93; on seed 23's eighth image from the mirrored assignment at blocks 12, 24 and 36, +1.4 ± 0.66 to +2.2 ± 0.89;
  on seed 23's first mapping from the assignment at block 12, +0.5 ± 0.25; and on seed 22's first mapping from the mirrored assignment at
  block 24, +0.5 ± 0.24. Below: on seed 24's eighth image from the mirrored assignment at blocks 6, 12 and 24, −1.5 ± 0.52 to −3.6 ± 0.75;
  on seed 24's first mapping from the assignment at blocks 6 and 36, −0.3 ± 0.02 and −2.1 ± 0.48; on seed 23's eighth image from the
  assignment from block 12 on, −1.7 ± 0.67 to −3.1 ± 0.79; on seed 23's first mapping from the mirrored assignment at blocks 6, 12, 18 and
  24, −0.4 ± 0.17 to −3.0 ± 0.69; and on seed 22's first mapping from the mirrored assignment at block 18, −0.8 ± 0.27.
- **The night held onto the own group less the night held onto the units outside the readouts** lies between −4.5 and +2.0, beyond twice
  its SE at 16 of the 60, 3 above and 13 below. Above: on seed 22's first mapping from the assignment at block 36, +2.0 ± 0.85; on seed
  23's first mapping from the assignment at block 6, +0.3 ± 0.02; and on seed 22's first mapping from the mirrored assignment at block 6,
  +0.6 ± 0.11. Below: on seed 22's eighth image from the mirrored assignment at blocks 12, 24 and 36, −1.7 ± 0.50 to −4.5 ± 0.59; on seed
  24's eighth image from the mirrored assignment at block 24, −1.9 ± 0.90; on seed 23's eighth image from the assignment at blocks 18 and
  24, −3.9 ± 1.09 and −2.4 ± 0.95; on seed 23's first mapping from the mirrored assignment at block 12, −1.7 ± 0.79; on seed 24's eighth
  image from the assignment at blocks 6, 12, 18 and 36, −1.2 ± 0.44 to −2.8 ± 1.11; on seed 22's first mapping from the mirrored assignment
  at block 18, −1.6 ± 0.67; and on seed 22's eighth image from the assignment at block 36, −2.6 ± 0.60.
- **The night held onto the inhibitory units less the night held onto the units outside the readouts** lies between −6.7 and +2.4, beyond
  twice its SE at 21 of the 60, 7 above and 14 below. Above: on seed 24's eighth image from the mirrored assignment at block 12,
  +1.6 ± 0.40; on seed 24's first mapping from the assignment at block 36, +1.7 ± 0.74; on seed 23's eighth image from the assignment at
  block 12, +2.4 ± 0.61; on seed 23's first mapping from the mirrored assignment from block 18 on, +1.3 ± 0.30 to +2.3 ± 0.24; and on seed
  22's first mapping from the mirrored assignment at block 6, +0.8 ± 0.23. Below: on seed 22's eighth image from the mirrored assignment at
  blocks 12, 24 and 36, −2.0 ± 0.44 to −6.7 ± 0.77; on seed 22's first mapping from the assignment from block 12 on, −1.3 ± 0.39 to
  −3.0 ± 0.56; on seed 23's eighth image from the mirrored assignment from block 12 on, −1.6 ± 0.70 to −2.6 ± 0.87; on seed 24's first
  mapping from the mirrored assignment at block 18, −1.0 ± 0.07; on seed 24's eighth image from the assignment at block 12, −1.0 ± 0.47;
  and on seed 22's first mapping from the mirrored assignment at block 24, −1.5 ± 0.50.

Each held night less the nights run in other jobs, with [exploration 018](018-what-nights-keep.md)'s unpaired SE:

| Cell | Block | Own group less H-47's plain | less 027's held within | less excitatory | Inhibitory units less H-47's plain | less 027's held within | less excitatory | Outside the readouts less H-47's plain | less 027's held within | less excitatory |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | −0.2 ± 0.65 | +0.5 ± 0.63 | +0.7 ± 0.78 | −0.4 ± 0.69 | +0.3 ± 0.64 | +0.5 ± 0.73 | 0.0 ± 0.65 | +0.7 ± 0.63 | +0.9 ± 0.74 |
| Seed 22, eighth, mirrored | 12 | −1.3 ± 1.35 | +0.1 ± 1.19 | 0.0 ± 1.55 | −1.6 ± 1.31 | −0.2 ± 1.15 | −0.3 ± 1.52 | +0.4 ± 1.01 | +1.8 ± 0.97 | +1.7 ± 1.19 |
| Seed 22, eighth, mirrored | 18 | −1.5 ± 1.35 | −0.3 ± 1.35 | −0.6 ± 1.72 | −1.2 ± 0.73 | +0.1 ± 0.83 | −0.3 ± 0.93 | 0.0 ± 0.65 | +1.3 ± 0.79 | +0.9 ± 0.94 |
| Seed 22, eighth, mirrored | 24 | −2.7 ± 1.23 | +0.5 ± 1.09 | +0.4 ± 1.92 | −4.9 ± 1.32 | −1.7 ± 1.32 | −1.8 ± 1.23 | +1.8 ± 1.24 | +5.0 ± 1.04 | +5.0 ± 1.59 |
| Seed 22, eighth, mirrored | 36 | −2.8 ± 0.53 | +0.1 ± 0.88 | +0.6 ± 1.51 | −3.6 ± 1.52 | −0.7 ± 1.38 | −0.2 ± 1.20 | +1.1 ± 0.49 | +4.0 ± 0.72 | +4.5 ± 1.23 |
| Seed 22, first mapping, assignment | 6 | +0.3 ± 0.35 | 0.0 ± 0.85 | 0.0 ± 0.65 | −0.1 ± 0.55 | −0.4 ± 0.56 | −0.4 ± 0.46 | +0.1 ± 0.32 | −0.2 ± 0.65 | −0.2 ± 0.46 |
| Seed 22, first mapping, assignment | 12 | +0.2 ± 1.26 | +0.5 ± 1.47 | +2.0 ± 1.44 | −0.7 ± 1.18 | −0.4 ± 1.32 | +1.1 ± 1.33 | +0.8 ± 1.17 | +1.0 ± 1.41 | +2.6 ± 1.38 |
| Seed 22, first mapping, assignment | 18 | +1.2 ± 1.42 | +1.5 ± 1.88 | +1.9 ± 1.70 | −0.1 ± 0.99 | +0.2 ± 1.17 | +0.6 ± 1.33 | +1.2 ± 1.05 | +1.5 ± 1.22 | +1.9 ± 1.24 |
| Seed 22, first mapping, assignment | 24 | +1.9 ± 1.08 | +2.6 ± 1.18 | +4.3 ± 1.02 | −2.2 ± 1.10 | −1.4 ± 1.44 | +0.2 ± 1.41 | +0.8 ± 0.96 | +1.5 ± 1.68 | +3.2 ± 1.34 |
| Seed 22, first mapping, assignment | 36 | +2.3 ± 0.90 | +3.7 ± 1.75 | +5.0 ± 1.15 | −1.9 ± 0.70 | −0.5 ± 1.16 | +0.8 ± 0.88 | +0.3 ± 0.22 | +1.8 ± 1.30 | +3.0 ± 0.92 |
| Seed 23, eighth, mirrored | 6 | +0.5 ± 1.18 | +1.1 ± 1.29 | −0.2 ± 1.09 | 0.0 ± 1.18 | +0.7 ± 1.03 | −0.7 ± 1.10 | −0.3 ± 1.05 | +0.4 ± 1.17 | −1.0 ± 1.08 |
| Seed 23, eighth, mirrored | 12 | −1.0 ± 1.33 | +1.2 ± 1.25 | +2.0 ± 1.25 | −2.4 ± 1.12 | −0.2 ± 1.22 | +0.6 ± 1.19 | −0.8 ± 1.36 | +1.4 ± 1.37 | +2.2 ± 1.43 |
| Seed 23, eighth, mirrored | 18 | −1.1 ± 1.15 | +3.0 ± 1.44 | +2.5 ± 1.25 | −3.0 ± 1.06 | +1.2 ± 1.18 | +0.6 ± 1.48 | −0.6 ± 0.89 | +3.5 ± 1.27 | +3.0 ± 1.07 |
| Seed 23, eighth, mirrored | 24 | −1.2 ± 1.16 | +2.1 ± 1.24 | +2.8 ± 1.32 | −3.4 ± 0.98 | −0.1 ± 1.01 | +0.6 ± 1.10 | −0.8 ± 1.13 | +2.5 ± 1.03 | +3.3 ± 1.49 |
| Seed 23, eighth, mirrored | 36 | −0.4 ± 1.06 | +1.5 ± 0.96 | +2.3 ± 1.20 | −2.1 ± 1.10 | −0.3 ± 0.90 | +0.6 ± 1.28 | −0.4 ± 1.12 | +1.4 ± 0.96 | +2.3 ± 1.38 |
| Seed 23, first mapping, assignment | 6 | +0.2 ± 0.80 | +0.1 ± 0.81 | +0.4 ± 0.80 | +0.1 ± 0.88 | +0.1 ± 0.86 | +0.3 ± 0.85 | −0.1 ± 0.80 | −0.1 ± 0.81 | +0.1 ± 0.80 |
| Seed 23, first mapping, assignment | 12 | +0.6 ± 0.97 | −0.8 ± 0.84 | +0.6 ± 0.94 | 0.0 ± 1.09 | −1.4 ± 1.00 | +0.1 ± 1.05 | +0.5 ± 1.09 | −0.9 ± 1.00 | +0.5 ± 1.13 |
| Seed 23, first mapping, assignment | 18 | −0.6 ± 1.19 | −0.4 ± 1.19 | −0.7 ± 1.24 | −0.6 ± 1.05 | −0.4 ± 1.12 | −0.7 ± 1.32 | −0.8 ± 1.07 | −0.6 ± 1.13 | −0.9 ± 1.28 |
| Seed 23, first mapping, assignment | 24 | −0.6 ± 1.16 | −1.7 ± 1.11 | −0.9 ± 1.35 | −0.1 ± 1.16 | −1.1 ± 1.00 | −0.4 ± 1.14 | −0.7 ± 1.15 | −1.7 ± 1.17 | −1.0 ± 1.33 |
| Seed 23, first mapping, assignment | 36 | −0.7 ± 0.90 | −0.6 ± 0.99 | +0.1 ± 1.12 | −0.7 ± 1.00 | −0.6 ± 1.02 | +0.1 ± 0.91 | −0.4 ± 0.81 | −0.4 ± 0.85 | +0.3 ± 0.98 |
| Seed 24, eighth, mirrored | 6 | −0.4 ± 0.78 | −1.8 ± 0.85 | −3.3 ± 0.87 | +1.1 ± 0.97 | −0.3 ± 0.78 | −1.8 ± 0.81 | +0.2 ± 0.86 | −1.2 ± 0.93 | −2.7 ± 0.79 |
| Seed 24, eighth, mirrored | 12 | +0.3 ± 1.04 | −4.8 ± 1.65 | −4.6 ± 1.25 | +2.7 ± 1.30 | −2.5 ± 1.18 | −2.3 ± 1.39 | +1.0 ± 1.25 | −4.1 ± 1.13 | −3.9 ± 1.43 |
| Seed 24, eighth, mirrored | 18 | +1.5 ± 1.89 | −5.1 ± 1.65 | −4.3 ± 1.66 | +2.2 ± 1.85 | −4.5 ± 1.58 | −3.7 ± 1.91 | +0.8 ± 1.48 | −5.9 ± 1.77 | −5.1 ± 1.80 |
| Seed 24, eighth, mirrored | 24 | −0.7 ± 1.46 | −7.8 ± 1.69 | −8.8 ± 1.76 | +2.9 ± 1.81 | −4.2 ± 1.72 | −5.2 ± 2.13 | +1.2 ± 1.77 | −5.9 ± 2.26 | −7.0 ± 2.18 |
| Seed 24, eighth, mirrored | 36 | +0.2 ± 1.41 | −4.0 ± 0.99 | −5.2 ± 0.77 | +1.6 ± 0.99 | −2.6 ± 1.34 | −3.8 ± 1.27 | +0.7 ± 1.85 | −3.6 ± 1.44 | −4.7 ± 1.24 |
| Seed 24, first mapping, assignment | 6 | 0.0 ± 0.41 | −0.4 ± 0.40 | +0.1 ± 0.39 | +0.4 ± 0.42 | 0.0 ± 0.41 | +0.5 ± 0.40 | +0.2 ± 0.42 | −0.2 ± 0.39 | +0.3 ± 0.42 |
| Seed 24, first mapping, assignment | 12 | +1.0 ± 1.04 | −1.2 ± 1.02 | +0.1 ± 1.05 | +0.1 ± 0.96 | −2.1 ± 0.77 | −0.9 ± 0.85 | 0.0 ± 1.11 | −2.2 ± 0.93 | −0.9 ± 1.03 |
| Seed 24, first mapping, assignment | 18 | +1.1 ± 1.08 | 0.0 ± 1.10 | −0.1 ± 0.98 | +0.6 ± 1.22 | −0.5 ± 0.98 | −0.6 ± 1.00 | +0.1 ± 1.11 | −1.0 ± 1.07 | −1.0 ± 1.01 |
| Seed 24, first mapping, assignment | 24 | +0.7 ± 1.28 | −0.6 ± 1.47 | +0.4 ± 1.26 | +0.7 ± 1.26 | −0.6 ± 1.33 | +0.4 ± 1.30 | +0.9 ± 1.78 | −0.5 ± 1.52 | +0.5 ± 1.16 |
| Seed 24, first mapping, assignment | 36 | +0.7 ± 0.76 | −1.7 ± 1.30 | −1.3 ± 0.93 | +2.8 ± 0.94 | +0.4 ± 1.32 | +0.8 ± 1.09 | +1.1 ± 0.64 | −1.3 ± 1.22 | −0.9 ± 0.72 |
| Seed 23, eighth, assignment | 6 | −0.2 ± 1.11 | −0.2 ± 1.04 | −0.6 ± 1.10 | +0.2 ± 1.16 | +0.2 ± 1.13 | −0.2 ± 1.14 | −0.2 ± 1.05 | −0.2 ± 0.99 | −0.5 ± 1.04 |
| Seed 23, eighth, assignment | 12 | +0.8 ± 1.35 | −3.1 ± 1.33 | −3.3 ± 1.26 | +2.4 ± 1.19 | −1.4 ± 1.34 | −1.6 ± 1.30 | +0.1 ± 1.36 | −3.7 ± 1.01 | −4.0 ± 0.94 |
| Seed 23, eighth, assignment | 18 | −0.5 ± 1.32 | −5.3 ± 1.34 | −5.4 ± 1.18 | +2.6 ± 1.01 | −2.2 ± 0.92 | −2.2 ± 1.12 | +3.3 ± 1.47 | −1.5 ± 0.91 | −1.5 ± 0.91 |
| Seed 23, eighth, assignment | 24 | +1.2 ± 1.32 | −7.2 ± 1.07 | −6.1 ± 0.87 | +3.9 ± 1.14 | −4.5 ± 0.91 | −3.4 ± 0.84 | +3.6 ± 0.76 | −4.8 ± 0.75 | −3.7 ± 1.22 |
| Seed 23, eighth, assignment | 36 | +0.4 ± 2.07 | −5.7 ± 1.51 | −6.5 ± 1.22 | +2.7 ± 1.66 | −3.5 ± 1.21 | −4.2 ± 0.98 | +2.4 ± 0.98 | −3.7 ± 0.90 | −4.5 ± 1.08 |
| Seed 24, first mapping, mirrored | 6 | −0.6 ± 0.87 | −0.1 ± 0.85 | −0.9 ± 0.87 | −0.3 ± 0.79 | +0.2 ± 0.75 | −0.5 ± 0.80 | −0.2 ± 0.84 | +0.3 ± 0.84 | −0.4 ± 0.83 |
| Seed 24, first mapping, mirrored | 12 | +0.2 ± 1.15 | +0.6 ± 1.22 | −1.1 ± 1.24 | −1.0 ± 1.10 | −0.7 ± 1.11 | −2.3 ± 0.90 | −0.3 ± 1.12 | 0.0 ± 1.15 | −1.6 ± 1.17 |
| Seed 24, first mapping, mirrored | 18 | +0.5 ± 1.03 | +0.8 ± 0.87 | −1.0 ± 0.93 | −0.9 ± 0.97 | −0.6 ± 1.10 | −2.3 ± 0.88 | +0.1 ± 0.89 | +0.4 ± 1.00 | −1.3 ± 0.80 |
| Seed 24, first mapping, mirrored | 24 | +1.1 ± 0.87 | −0.2 ± 0.90 | −0.6 ± 0.97 | +0.2 ± 0.73 | −1.1 ± 0.94 | −1.5 ± 0.79 | +0.6 ± 0.82 | −0.7 ± 0.72 | −1.1 ± 0.88 |
| Seed 24, first mapping, mirrored | 36 | +1.1 ± 0.70 | −0.6 ± 0.82 | −0.4 ± 0.89 | +0.8 ± 0.84 | −1.0 ± 1.07 | −0.7 ± 0.95 | +1.0 ± 0.96 | −0.7 ± 1.11 | −0.4 ± 0.94 |
| Seed 23, first mapping, mirrored | 6 | −0.4 ± 0.61 | −0.1 ± 0.55 | −0.1 ± 0.61 | 0.0 ± 0.71 | +0.3 ± 0.62 | +0.4 ± 0.72 | −0.1 ± 0.68 | +0.3 ± 0.59 | +0.3 ± 0.67 |
| Seed 23, first mapping, mirrored | 12 | −0.9 ± 1.05 | −1.3 ± 1.26 | −2.0 ± 1.20 | +1.3 ± 1.20 | +0.9 ± 0.86 | +0.1 ± 1.16 | +0.8 ± 1.14 | +0.4 ± 0.89 | −0.3 ± 1.08 |
| Seed 23, first mapping, mirrored | 18 | −0.9 ± 0.81 | −2.3 ± 0.83 | −2.7 ± 0.96 | +2.1 ± 0.77 | +0.7 ± 0.78 | +0.3 ± 0.96 | −0.2 ± 0.78 | −1.6 ± 0.81 | −2.1 ± 0.97 |
| Seed 23, first mapping, mirrored | 24 | −1.2 ± 1.36 | −2.9 ± 0.88 | −3.6 ± 0.95 | +1.6 ± 0.93 | −0.1 ± 1.66 | −0.8 ± 1.25 | +0.3 ± 0.88 | −1.4 ± 1.64 | −2.1 ± 1.22 |
| Seed 23, first mapping, mirrored | 36 | +0.3 ± 0.96 | −1.9 ± 0.80 | −2.9 ± 0.85 | +1.9 ± 0.92 | −0.4 ± 1.12 | −1.3 ± 0.98 | +0.2 ± 0.96 | −2.0 ± 1.07 | −3.0 ± 0.93 |
| Seed 24, eighth, assignment | 6 | −1.5 ± 0.97 | −0.7 ± 1.01 | −1.6 ± 1.06 | −0.3 ± 1.10 | +0.5 ± 1.02 | −0.4 ± 1.23 | −0.3 ± 1.09 | +0.5 ± 1.11 | −0.5 ± 1.20 |
| Seed 24, eighth, assignment | 12 | −0.8 ± 1.05 | −1.1 ± 1.29 | −3.4 ± 1.41 | −0.1 ± 1.07 | −0.4 ± 1.14 | −2.8 ± 1.06 | +0.9 ± 0.91 | +0.6 ± 0.94 | −1.8 ± 0.95 |
| Seed 24, eighth, assignment | 18 | +1.4 ± 1.28 | −1.7 ± 1.54 | −5.7 ± 1.09 | +1.5 ± 1.52 | −1.6 ± 1.20 | −5.6 ± 1.08 | +3.2 ± 1.30 | +0.1 ± 1.57 | −3.9 ± 1.23 |
| Seed 24, eighth, assignment | 24 | +0.1 ± 2.42 | −2.7 ± 2.07 | −7.0 ± 1.14 | +0.1 ± 2.25 | −2.7 ± 1.65 | −6.9 ± 1.20 | +2.1 ± 1.65 | −0.7 ± 1.44 | −4.9 ± 1.74 |
| Seed 24, eighth, assignment | 36 | −1.1 ± 1.32 | −3.4 ± 1.13 | −8.1 ± 1.21 | −0.4 ± 1.32 | −2.7 ± 1.14 | −7.4 ± 0.94 | +1.7 ± 1.63 | −0.6 ± 1.74 | −5.3 ± 1.93 |
| Seed 22, first mapping, mirrored | 6 | +0.2 ± 0.51 | −0.3 ± 0.45 | −0.1 ± 0.54 | +0.4 ± 0.46 | 0.0 ± 0.39 | +0.1 ± 0.54 | −0.4 ± 0.52 | −0.8 ± 0.44 | −0.7 ± 0.50 |
| Seed 22, first mapping, mirrored | 12 | +0.1 ± 1.03 | −0.4 ± 0.94 | −0.1 ± 1.04 | +0.7 ± 0.88 | +0.1 ± 1.00 | +0.4 ± 1.00 | +1.4 ± 1.23 | +0.8 ± 1.68 | +1.1 ± 1.45 |
| Seed 22, first mapping, mirrored | 18 | −0.2 ± 1.10 | −0.7 ± 1.19 | +0.2 ± 1.09 | +0.7 ± 0.97 | +0.1 ± 1.13 | +1.0 ± 1.06 | +1.4 ± 1.20 | +0.9 ± 1.53 | +1.8 ± 1.39 |
| Seed 22, first mapping, mirrored | 24 | +0.7 ± 1.39 | −0.5 ± 1.36 | +0.6 ± 1.46 | +0.2 ± 1.27 | −1.0 ± 1.32 | +0.1 ± 1.36 | +1.7 ± 1.27 | +0.5 ± 1.48 | +1.6 ± 1.39 |
| Seed 22, first mapping, mirrored | 36 | +0.4 ± 0.72 | −0.2 ± 0.65 | +0.8 ± 0.70 | −0.1 ± 1.10 | −0.8 ± 0.92 | +0.2 ± 0.87 | +1.0 ± 0.40 | +0.4 ± 0.95 | +1.4 ± 1.04 |
| Seed 22, eighth, assignment | 6 | −0.1 ± 1.02 | −0.5 ± 0.98 | −0.4 ± 0.99 | +0.2 ± 0.96 | −0.2 ± 1.01 | −0.2 ± 1.01 | +0.4 ± 0.96 | 0.0 ± 1.10 | +0.1 ± 1.08 |
| Seed 22, eighth, assignment | 12 | +0.5 ± 0.98 | −1.8 ± 0.95 | −1.7 ± 0.88 | +1.0 ± 1.02 | −1.3 ± 0.92 | −1.2 ± 0.85 | +0.7 ± 1.05 | −1.5 ± 1.01 | −1.4 ± 0.92 |
| Seed 22, eighth, assignment | 18 | −0.3 ± 1.15 | −1.5 ± 1.30 | −1.5 ± 1.32 | +1.0 ± 1.26 | −0.2 ± 0.71 | −0.2 ± 0.87 | +1.6 ± 1.21 | +0.4 ± 0.92 | +0.4 ± 1.03 |
| Seed 22, eighth, assignment | 24 | −0.4 ± 1.01 | −2.8 ± 1.28 | −2.4 ± 1.10 | +0.7 ± 0.87 | −1.7 ± 0.88 | −1.3 ± 0.84 | +1.4 ± 1.27 | −1.0 ± 0.84 | −0.6 ± 1.00 |
| Seed 22, eighth, assignment | 36 | −0.7 ± 1.79 | −3.0 ± 2.24 | −3.1 ± 1.81 | +1.1 ± 1.33 | −1.2 ± 1.54 | −1.4 ± 1.25 | +2.0 ± 1.55 | −0.3 ± 1.96 | −0.5 ± 1.56 |

- **The night held onto the own group less H-47's plain night** lies between −2.8 and +2.3, beyond twice its unpaired SE at 3 of the 60
  checkpoints: below on seed 22's eighth image from the mirrored assignment at blocks 24 and 36, −2.7 ± 1.23 and −2.8 ± 0.53, and above on
  seed 22's first mapping from the assignment at block 36, +2.3 ± 0.90.
- **The night held onto the inhibitory units less H-47's plain night** lies between −4.9 and +3.9, beyond twice its unpaired SE at 13 of
  the 60, 7 above and 6 below. Above: on seed 24's eighth image from the mirrored assignment at block 12, +2.7 ± 1.30; on seed 24's first
  mapping from the assignment at block 36, +2.8 ± 0.94; on seed 23's eighth image from the assignment at blocks 12, 18 and 24, +2.4 ± 1.19
  to +3.9 ± 1.14; and on seed 23's first mapping from the mirrored assignment at blocks 18 and 36, +2.1 ± 0.77 and +1.9 ± 0.92. Below: on
  seed 22's eighth image from the mirrored assignment at blocks 24 and 36, −4.9 ± 1.32 and −3.6 ± 1.52; on seed 22's first mapping from the
  assignment at block 36, −1.9 ± 0.70; and on seed 23's eighth image from the mirrored assignment at blocks 12, 18 and 24, −2.4 ± 1.12 to
  −3.4 ± 0.98.
- **The night held onto the units outside the readouts less H-47's plain night** lies between −0.8 and +3.6, beyond twice its unpaired SE
  at 6 of the 60, all above: on seed 22's eighth image from the mirrored assignment at block 36, +1.1 ± 0.49; on seed 23's eighth image from
  the assignment from block 18 on, +2.4 ± 0.98 to +3.6 ± 0.76; on seed 24's eighth image from the assignment at block 18, +3.2 ± 1.30; and
  on seed 22's first mapping from the mirrored assignment at block 36, +1.0 ± 0.40.
- **Where exploration 027's night held within lay beyond twice its unpaired SE of H-47's plain night**, 17 checkpoints, 13 above and 4
  below, the night held onto the inhibitory units lies beyond it in the same direction at 8: on seed 22's eighth image from the mirrored
  assignment at blocks 24 and 36, on seed 23's eighth image from the mirrored assignment at blocks 18 and 24, on seed 24's eighth image from
  the mirrored assignment at block 12 and on seed 23's eighth image from the assignment at blocks 12, 18 and 24. The night held onto the
  units outside the readouts does so at 4, on seed 23's eighth image from the assignment from block 18 on and on seed 24's eighth image from
  the assignment at block 18, and the night held onto the own group at 2, on seed 22's eighth image from the mirrored assignment at blocks 24
  and 36.
- **Where the excitatory night lay beyond twice its unpaired SE of H-47's plain night**, 25 checkpoints, 19 above and 6 below, the night
  held onto the inhibitory units lies beyond it in the same direction at 12: on seed 22's eighth image from the mirrored assignment and
  seed 22's first mapping from the assignment, each at block 36; on seed 23's eighth image from the mirrored assignment at blocks 12, 18
  and 24; on seed 24's eighth image from the mirrored assignment at block 12; on seed 24's first mapping from the assignment at block 36; on
  seed 23's eighth image from the assignment at blocks 12, 18 and 24; and on seed 23's first mapping from the mirrored assignment at blocks
  18 and 36. These are 12 of the 13 checkpoints where it lies beyond the plain night; the other is seed 22's eighth image from the mirrored
  assignment at block 24. The night held onto the units outside the readouts does so at 4, the same 4 as above, and the night held onto the
  own group at 1, seed 22's eighth image from the mirrored assignment at block 36.
- **The night held onto the own group less exploration 027's night held within** lies between −7.8 and +3.7, beyond twice its unpaired SE
  at 17 of the 60, 3 above and 14 below. Above: on seed 22's first mapping from the assignment at blocks 24 and 36, +2.6 ± 1.18 and
  +3.7 ± 1.75, and on seed 23's eighth image from the mirrored assignment at block 18, +3.0 ± 1.44. Below: on seed 24's eighth image from the
  mirrored assignment at every checkpoint, −1.8 ± 0.85 to −7.8 ± 1.69; on seed 23's eighth image from the assignment from block 12 on,
  −3.1 ± 1.33 to −7.2 ± 1.07; on seed 23's first mapping from the mirrored assignment from block 18 on, −1.9 ± 0.80 to −2.9 ± 0.88; on seed
  24's eighth image from the assignment at block 36, −3.4 ± 1.13; and on seed 22's eighth image from the assignment at block 24,
  −2.8 ± 1.28.
- **The night held onto the inhibitory units less exploration 027's night held within** lies between −4.5 and +1.2, beyond twice its
  unpaired SE at 8 of the 60, all below: on seed 24's eighth image from the mirrored assignment at blocks 12, 18 and 24, −2.5 ± 1.18 to
  −4.5 ± 1.58; on seed 24's first mapping from the assignment at block 12, −2.1 ± 0.77; on seed 23's eighth image from the assignment from
  block 18 on, −2.2 ± 0.92 to −4.5 ± 0.91; and on seed 24's eighth image from the assignment at block 36, −2.7 ± 1.14.
- **The night held onto the units outside the readouts less exploration 027's night held within** lies between −5.9 and +5.0, beyond twice
  its unpaired SE at 13 of the 60, 4 above and 9 below. Above: on seed 22's eighth image from the mirrored assignment at blocks 24 and 36,
  +5.0 ± 1.04 and +4.0 ± 0.72, and on seed 23's eighth image from the mirrored assignment at blocks 18 and 24, +3.5 ± 1.27 and +2.5 ± 1.03.
  Below: on seed 24's eighth image from the mirrored assignment from block 12 on, −3.6 ± 1.44 to −5.9 ± 2.26; on seed 24's first mapping
  from the assignment at block 12, −2.2 ± 0.93; on seed 23's eighth image from the assignment at blocks 12, 24 and 36, −3.7 ± 1.01 to
  −4.8 ± 0.75; and on seed 23's first mapping from the mirrored assignment at block 18, −1.6 ± 0.81.
- **Less the excitatory night**, the night held onto the own group lies between −8.8 and +5.0, beyond twice its unpaired SE at 20 of the
  60, 3 above and 17 below; the night held onto the inhibitory units between −7.4 and +1.1, beyond at 11, all below; and the night held
  onto the units outside the readouts between −7.0 and +5.0, beyond at 19, 6 above and 13 below. Above: the night held onto the own group
  on seed 22's first mapping from the assignment at blocks 24 and 36 and on seed 23's eighth image from the mirrored assignment at block
  24; the night held onto the units outside the readouts on seed 22's eighth image from the mirrored assignment and seed 22's first mapping
  from the assignment, each at blocks 24 and 36, and on seed 23's eighth image from the mirrored assignment at blocks 18 and 24; all where A
  answers readout 0. Each lies below on seed 24's eighth image from the mirrored assignment, the night held onto the own
  group and the night held onto the units outside the readouts at every checkpoint and the night held onto the inhibitory units at blocks 6,
  24 and 36, and on seed 23's and seed 24's eighth images from the assignment: the night held onto the own group on seed 23's from block 12
  on and on seed 24's from block 12 on; the night held onto the inhibitory units on seed 23's at blocks 24 and 36 and on seed 24's from
  block 12 on; and the night held onto the units outside the readouts on seed 23's at blocks 12, 24 and 36 and on seed 24's from block 18
  on. The other checkpoints below are on seed 23's first mapping from the mirrored assignment, from block 18 on under the night held onto
  the own group and at blocks 18 and 36 under the night held onto the units outside the readouts; on seed 24's first mapping from the
  mirrored assignment at blocks 12 and 18 under the night held onto the inhibitory units; and on seed 22's eighth image from the assignment
  at block 24 under the night held onto the own group.

Each stimulus's correct trials, the mean of the 48 reads, under each form, A's and then B's:

| Cell | Block | A's: without replay | own group | inhibitory units | outside the readouts | plain on A's | held within (027) | excitatory |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 24.8 | 29.0 | 29.5 | 28.9 | 29.0 | 29.4 | 29.4 |
| Seed 22, eighth, mirrored | 12 | 11.3 | 17.9 | 19.7 | 19.0 | 19.1 | 19.0 | 20.1 |
| Seed 22, eighth, mirrored | 18 | 9.5 | 16.8 | 18.6 | 17.9 | 18.1 | 17.4 | 19.0 |
| Seed 22, eighth, mirrored | 24 | 8.6 | 12.8 | 14.7 | 15.5 | 14.7 | 14.3 | 15.8 |
| Seed 22, eighth, mirrored | 36 | 4.7 | 12.1 | 14.1 | 14.1 | 13.9 | 13.7 | 15.0 |
| Seed 22, first mapping, assignment | 6 | 25.5 | 29.5 | 29.5 | 29.6 | 29.6 | 29.7 | 29.7 |
| Seed 22, first mapping, assignment | 12 | 21.1 | 25.4 | 25.4 | 25.0 | 24.9 | 25.8 | 25.8 |
| Seed 22, first mapping, assignment | 18 | 16.4 | 25.7 | 25.3 | 25.1 | 24.8 | 25.2 | 25.0 |
| Seed 22, first mapping, assignment | 24 | 15.5 | 24.6 | 23.8 | 23.7 | 23.8 | 23.8 | 23.2 |
| Seed 22, first mapping, assignment | 36 | 13.2 | 23.0 | 22.3 | 21.5 | 21.5 | 21.5 | 21.3 |
| Seed 23, eighth, mirrored | 6 | 20.0 | 22.5 | 22.0 | 22.3 | 22.5 | 22.0 | 21.8 |
| Seed 23, eighth, mirrored | 12 | 15.3 | 20.8 | 19.5 | 21.1 | 21.5 | 19.4 | 18.0 |
| Seed 23, eighth, mirrored | 18 | 12.3 | 22.9 | 21.8 | 23.4 | 24.4 | 20.0 | 19.5 |
| Seed 23, eighth, mirrored | 24 | 8.9 | 24.3 | 23.6 | 24.5 | 25.8 | 22.3 | 21.4 |
| Seed 23, eighth, mirrored | 36 | 7.4 | 22.2 | 21.9 | 22.7 | 23.3 | 21.1 | 20.9 |
| Seed 23, first mapping, assignment | 6 | 29.2 | 30.9 | 31.0 | 30.9 | 30.9 | 31.0 | 30.8 |
| Seed 23, first mapping, assignment | 12 | 24.2 | 30.0 | 30.3 | 30.4 | 30.1 | 30.5 | 29.6 |
| Seed 23, first mapping, assignment | 18 | 19.5 | 29.6 | 30.3 | 29.7 | 30.1 | 29.9 | 29.2 |
| Seed 23, first mapping, assignment | 24 | 17.3 | 29.8 | 30.4 | 29.8 | 30.4 | 30.0 | 29.2 |
| Seed 23, first mapping, assignment | 36 | 15.7 | 28.8 | 28.8 | 29.1 | 29.2 | 29.1 | 28.0 |
| Seed 24, eighth, mirrored | 6 | 19.1 | 16.5 | 18.5 | 17.1 | 17.0 | 18.2 | 18.9 |
| Seed 24, eighth, mirrored | 12 | 15.3 | 12.3 | 14.5 | 13.1 | 12.1 | 16.4 | 16.2 |
| Seed 24, eighth, mirrored | 18 | 9.2 | 11.4 | 13.6 | 11.5 | 9.9 | 16.9 | 16.0 |
| Seed 24, eighth, mirrored | 24 | 7.4 | 9.4 | 12.4 | 10.4 | 9.0 | 16.5 | 16.6 |
| Seed 24, eighth, mirrored | 36 | 5.3 | 8.8 | 9.6 | 7.4 | 6.9 | 12.7 | 13.3 |
| Seed 24, first mapping, assignment | 6 | 28.8 | 30.7 | 30.9 | 30.8 | 30.7 | 30.9 | 30.6 |
| Seed 24, first mapping, assignment | 12 | 26.4 | 29.1 | 29.4 | 28.9 | 28.6 | 30.3 | 29.0 |
| Seed 24, first mapping, assignment | 18 | 24.9 | 29.6 | 29.9 | 29.4 | 28.9 | 30.1 | 29.1 |
| Seed 24, first mapping, assignment | 24 | 22.3 | 29.1 | 29.7 | 29.5 | 28.4 | 30.5 | 29.1 |
| Seed 24, first mapping, assignment | 36 | 21.2 | 26.3 | 26.2 | 25.3 | 24.5 | 27.6 | 26.1 |
| Seed 23, eighth, assignment | 6 | 24.3 | 27.1 | 26.8 | 26.7 | 26.9 | 27.2 | 27.0 |
| Seed 23, eighth, assignment | 12 | 18.3 | 22.2 | 21.9 | 21.4 | 21.2 | 23.1 | 23.2 |
| Seed 23, eighth, assignment | 18 | 15.6 | 22.6 | 21.5 | 23.0 | 20.8 | 24.0 | 24.4 |
| Seed 23, eighth, assignment | 24 | 16.3 | 24.0 | 21.4 | 22.2 | 19.6 | 25.3 | 25.2 |
| Seed 23, eighth, assignment | 36 | 13.0 | 18.8 | 17.0 | 17.5 | 15.3 | 20.3 | 21.0 |
| Seed 24, first mapping, mirrored | 6 | 28.1 | 29.6 | 29.9 | 30.1 | 30.1 | 29.7 | 30.2 |
| Seed 24, first mapping, mirrored | 12 | 24.6 | 27.3 | 26.8 | 27.4 | 27.3 | 27.0 | 27.2 |
| Seed 24, first mapping, mirrored | 18 | 21.7 | 26.8 | 26.5 | 27.1 | 27.0 | 26.5 | 26.8 |
| Seed 24, first mapping, mirrored | 24 | 22.1 | 25.8 | 25.5 | 26.2 | 25.8 | 26.0 | 26.1 |
| Seed 24, first mapping, mirrored | 36 | 22.0 | 25.0 | 25.1 | 26.1 | 25.2 | 25.3 | 25.5 |
| Seed 23, first mapping, mirrored | 6 | 30.4 | 31.2 | 31.1 | 31.2 | 31.0 | 31.1 | 31.2 |
| Seed 23, first mapping, mirrored | 12 | 28.5 | 29.6 | 29.8 | 29.5 | 29.6 | 29.8 | 30.3 |
| Seed 23, first mapping, mirrored | 18 | 28.5 | 28.4 | 27.7 | 27.8 | 27.6 | 28.5 | 28.6 |
| Seed 23, first mapping, mirrored | 24 | 30.0 | 25.4 | 24.9 | 25.6 | 25.0 | 26.3 | 26.8 |
| Seed 23, first mapping, mirrored | 36 | 29.3 | 25.4 | 24.3 | 24.6 | 24.1 | 25.5 | 26.1 |
| Seed 24, eighth, assignment | 6 | 21.3 | 23.0 | 23.2 | 23.6 | 23.8 | 23.4 | 23.6 |
| Seed 24, eighth, assignment | 12 | 18.4 | 18.5 | 18.9 | 18.8 | 18.8 | 20.0 | 20.2 |
| Seed 24, eighth, assignment | 18 | 18.0 | 13.6 | 14.3 | 16.0 | 14.4 | 16.3 | 18.2 |
| Seed 24, eighth, assignment | 24 | 19.2 | 14.6 | 13.8 | 17.0 | 15.3 | 16.0 | 17.6 |
| Seed 24, eighth, assignment | 36 | 12.9 | 13.1 | 13.4 | 14.9 | 13.1 | 16.0 | 17.4 |
| Seed 22, first mapping, mirrored | 6 | 30.6 | 31.4 | 31.4 | 31.3 | 31.4 | 31.4 | 31.5 |
| Seed 22, first mapping, mirrored | 12 | 28.2 | 29.5 | 29.0 | 29.9 | 29.3 | 29.0 | 28.9 |
| Seed 22, first mapping, mirrored | 18 | 28.0 | 27.8 | 27.6 | 28.5 | 27.6 | 27.9 | 27.6 |
| Seed 22, first mapping, mirrored | 24 | 27.5 | 28.1 | 27.4 | 28.4 | 27.6 | 28.2 | 27.6 |
| Seed 22, first mapping, mirrored | 36 | 27.8 | 27.7 | 26.9 | 28.1 | 27.4 | 27.7 | 26.9 |
| Seed 22, eighth, assignment | 6 | 24.4 | 26.4 | 26.2 | 26.3 | 26.4 | 26.6 | 26.5 |
| Seed 22, eighth, assignment | 12 | 22.9 | 25.7 | 25.2 | 25.7 | 25.4 | 26.2 | 26.4 |
| Seed 22, eighth, assignment | 18 | 24.4 | 23.8 | 24.4 | 25.5 | 23.8 | 24.9 | 25.0 |
| Seed 22, eighth, assignment | 24 | 26.0 | 25.6 | 25.0 | 26.5 | 25.8 | 26.1 | 25.9 |
| Seed 22, eighth, assignment | 36 | 24.7 | 23.1 | 23.8 | 24.2 | 23.1 | 23.6 | 23.9 |

| Cell | Block | B's: without replay | own group | inhibitory units | outside the readouts | plain on A's | held within (027) | excitatory |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 21.5 | 24.1 | 23.4 | 24.4 | 24.4 | 23.1 | 23.0 |
| Seed 22, eighth, mirrored | 12 | 18.9 | 23.0 | 21.0 | 23.6 | 23.2 | 21.9 | 20.8 |
| Seed 22, eighth, mirrored | 18 | 16.8 | 23.5 | 22.0 | 23.9 | 23.7 | 23.1 | 21.9 |
| Seed 22, eighth, mirrored | 24 | 14.0 | 24.7 | 20.6 | 26.5 | 25.5 | 22.7 | 21.3 |
| Seed 22, eighth, mirrored | 36 | 11.9 | 21.3 | 18.5 | 23.2 | 22.3 | 19.6 | 17.8 |
| Seed 22, first mapping, assignment | 6 | 26.9 | 27.2 | 26.8 | 27.0 | 26.9 | 27.0 | 27.1 |
| Seed 22, first mapping, assignment | 12 | 23.6 | 22.3 | 21.5 | 23.4 | 22.7 | 21.6 | 20.0 |
| Seed 22, first mapping, assignment | 18 | 23.7 | 22.4 | 21.5 | 23.0 | 22.2 | 21.4 | 21.3 |
| Seed 22, first mapping, assignment | 24 | 23.4 | 25.2 | 22.1 | 25.0 | 24.3 | 23.4 | 22.4 |
| Seed 22, first mapping, assignment | 36 | 23.1 | 23.6 | 20.1 | 23.0 | 22.7 | 21.3 | 20.3 |
| Seed 23, eighth, mirrored | 6 | 14.4 | 14.1 | 14.1 | 13.6 | 13.6 | 13.5 | 15.0 |
| Seed 23, eighth, mirrored | 12 | 4.2 | 6.8 | 6.6 | 6.5 | 7.0 | 6.9 | 7.4 |
| Seed 23, eighth, mirrored | 18 | 2.6 | 3.6 | 2.8 | 3.6 | 3.2 | 3.4 | 4.5 |
| Seed 23, eighth, mirrored | 24 | 1.7 | 3.8 | 2.3 | 4.0 | 3.5 | 3.6 | 3.8 |
| Seed 23, eighth, mirrored | 36 | 0.8 | 3.0 | 1.6 | 2.4 | 2.3 | 2.6 | 2.0 |
| Seed 23, first mapping, assignment | 6 | 25.0 | 25.6 | 25.4 | 25.2 | 25.4 | 25.3 | 25.3 |
| Seed 23, first mapping, assignment | 12 | 9.3 | 8.1 | 7.3 | 7.6 | 7.4 | 8.5 | 7.9 |
| Seed 23, first mapping, assignment | 18 | 7.6 | 4.8 | 4.1 | 4.5 | 4.9 | 4.9 | 5.9 |
| Seed 23, first mapping, assignment | 24 | 7.5 | 4.4 | 4.3 | 4.2 | 4.3 | 5.8 | 5.8 |
| Seed 23, first mapping, assignment | 36 | 4.0 | 3.5 | 3.6 | 3.5 | 3.9 | 3.9 | 4.3 |
| Seed 24, eighth, mirrored | 6 | 17.3 | 16.9 | 16.4 | 16.9 | 16.8 | 17.0 | 17.8 |
| Seed 24, eighth, mirrored | 12 | 16.3 | 16.7 | 16.8 | 16.5 | 16.6 | 17.4 | 17.4 |
| Seed 24, eighth, mirrored | 18 | 11.0 | 18.6 | 17.2 | 17.9 | 18.6 | 18.3 | 18.5 |
| Seed 24, eighth, mirrored | 24 | 9.0 | 15.7 | 16.3 | 16.5 | 16.8 | 16.4 | 17.3 |
| Seed 24, eighth, mirrored | 36 | 7.2 | 15.2 | 15.8 | 17.1 | 16.9 | 15.4 | 15.9 |
| Seed 24, first mapping, assignment | 6 | 29.5 | 30.6 | 30.7 | 30.7 | 30.6 | 30.8 | 30.6 |
| Seed 24, first mapping, assignment | 12 | 25.6 | 26.7 | 25.4 | 25.9 | 26.2 | 26.7 | 26.6 |
| Seed 24, first mapping, assignment | 18 | 22.6 | 22.9 | 22.1 | 22.0 | 22.5 | 22.3 | 23.4 |
| Seed 24, first mapping, assignment | 24 | 21.4 | 24.0 | 23.4 | 23.8 | 24.0 | 23.3 | 23.6 |
| Seed 24, first mapping, assignment | 36 | 18.8 | 18.2 | 20.4 | 19.6 | 19.3 | 18.6 | 19.7 |
| Seed 23, eighth, assignment | 6 | 23.1 | 23.0 | 23.8 | 23.5 | 23.4 | 23.2 | 23.7 |
| Seed 23, eighth, assignment | 12 | 22.6 | 22.0 | 23.9 | 22.1 | 22.2 | 24.2 | 24.3 |
| Seed 23, eighth, assignment | 18 | 21.7 | 22.1 | 26.3 | 25.6 | 24.4 | 26.0 | 25.7 |
| Seed 23, eighth, assignment | 24 | 21.7 | 20.9 | 26.2 | 25.1 | 24.1 | 26.8 | 25.8 |
| Seed 23, eighth, assignment | 36 | 23.2 | 21.9 | 25.9 | 25.2 | 25.0 | 26.1 | 26.1 |
| Seed 24, first mapping, mirrored | 6 | 26.2 | 27.5 | 27.5 | 27.4 | 27.6 | 27.5 | 27.8 |
| Seed 24, first mapping, mirrored | 12 | 21.7 | 24.0 | 23.3 | 23.3 | 23.8 | 23.7 | 25.1 |
| Seed 24, first mapping, mirrored | 18 | 22.3 | 25.5 | 24.5 | 24.8 | 24.8 | 25.0 | 26.5 |
| Seed 24, first mapping, mirrored | 24 | 22.3 | 27.3 | 26.7 | 26.4 | 26.2 | 27.3 | 27.6 |
| Seed 24, first mapping, mirrored | 36 | 20.8 | 24.8 | 24.4 | 23.6 | 23.5 | 25.1 | 24.6 |
| Seed 23, first mapping, mirrored | 6 | 24.6 | 25.3 | 25.8 | 25.7 | 25.9 | 25.5 | 25.4 |
| Seed 23, first mapping, mirrored | 12 | 23.8 | 23.9 | 25.9 | 25.6 | 24.8 | 25.0 | 25.3 |
| Seed 23, first mapping, mirrored | 18 | 23.5 | 23.1 | 26.7 | 24.3 | 24.8 | 25.3 | 25.6 |
| Seed 23, first mapping, mirrored | 24 | 23.5 | 24.4 | 27.7 | 25.6 | 25.9 | 26.3 | 26.5 |
| Seed 23, first mapping, mirrored | 36 | 23.9 | 23.2 | 25.8 | 23.8 | 24.2 | 25.0 | 25.3 |
| Seed 24, eighth, assignment | 6 | 17.5 | 14.6 | 15.6 | 15.2 | 15.3 | 14.9 | 15.7 |
| Seed 24, eighth, assignment | 12 | 16.4 | 14.9 | 15.2 | 16.3 | 15.4 | 14.5 | 16.6 |
| Seed 24, eighth, assignment | 18 | 16.5 | 16.2 | 15.6 | 15.7 | 14.1 | 15.3 | 17.4 |
| Seed 24, eighth, assignment | 24 | 16.3 | 15.1 | 15.9 | 14.7 | 14.3 | 16.4 | 19.1 |
| Seed 24, eighth, assignment | 36 | 12.7 | 13.2 | 13.6 | 14.3 | 14.3 | 13.7 | 16.9 |
| Seed 22, first mapping, mirrored | 6 | 26.0 | 26.6 | 26.8 | 26.1 | 26.4 | 26.9 | 26.6 |
| Seed 22, first mapping, mirrored | 12 | 21.3 | 23.1 | 24.2 | 24.0 | 23.1 | 24.0 | 23.8 |
| Seed 22, first mapping, mirrored | 18 | 21.0 | 24.7 | 25.7 | 25.6 | 25.1 | 25.3 | 24.7 |
| Seed 22, first mapping, mirrored | 24 | 18.4 | 23.1 | 23.4 | 23.9 | 23.0 | 23.6 | 23.1 |
| Seed 22, first mapping, mirrored | 36 | 16.4 | 21.9 | 22.0 | 22.0 | 21.8 | 22.0 | 21.9 |
| Seed 22, eighth, assignment | 6 | 25.9 | 25.8 | 26.2 | 26.4 | 25.8 | 26.1 | 26.1 |
| Seed 22, eighth, assignment | 12 | 24.8 | 25.4 | 26.4 | 25.7 | 25.3 | 26.7 | 26.4 |
| Seed 22, eighth, assignment | 18 | 22.8 | 26.5 | 27.2 | 26.7 | 26.8 | 27.0 | 26.8 |
| Seed 22, eighth, assignment | 24 | 21.3 | 24.2 | 25.9 | 25.0 | 24.4 | 26.5 | 26.2 |
| Seed 22, eighth, assignment | 36 | 19.5 | 22.6 | 23.7 | 24.1 | 23.2 | 25.1 | 24.9 |

## 2. The side under each held night

The side under a held night is the stimulus whose difference from the night without replay is the larger. "A's less B's" is the paired
difference of the two, and "within 2 SE" marks it within twice that SE. H-47's side is the same reading of H-47's plain night, from H-47's
job; exploration 027's night held within's from exploration 027's job; and the excitatory night's from exploration 025's or 026's job. The
group named is the one that stimulus answers in that cell:

| Cell | Block | Own group: A's less without replay | B's | A's less B's | Inhibitory units: A's | B's | A's less B's | Outside the readouts: A's | B's | A's less B's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +4.3 ± 0.97 | +2.6 ± 1.26 | +1.7 ± 2.06 | +4.8 ± 0.86 | +1.9 ± 1.10 | +2.9 ± 1.76 | +4.2 ± 0.89 | +2.9 ± 0.93 | +1.3 ± 1.62 |
| Seed 22, eighth, mirrored | 12 | +6.6 ± 1.32 | +4.2 ± 1.02 | +2.4 ± 1.81 | +8.3 ± 1.05 | +2.2 ± 0.71 | +6.2 ± 0.93 | +7.7 ± 1.12 | +4.8 ± 0.88 | +2.9 ± 1.70 |
| Seed 22, eighth, mirrored | 18 | +7.4 ± 1.87 | +6.6 ± 1.50 | +0.7 ± 2.44 | +9.1 ± 1.42 | +5.2 ± 0.88 | +4.0 ± 1.62 | +8.5 ± 1.39 | +7.1 ± 1.45 | +1.4 ± 2.42 |
| Seed 22, eighth, mirrored | 24 | +4.2 ± 1.99 | +10.7 ± 1.22 | −6.6 ± 2.69 | +6.1 ± 0.91 | +6.6 ± 0.69 | −0.5 ± 1.14 | +6.9 ± 1.35 | +12.5 ± 1.02 | −5.6 ± 1.73 |
| Seed 22, eighth, mirrored | 36 | +7.4 ± 0.75 | +9.4 ± 1.03 | −1.9 ± 1.57 | +9.4 ± 0.59 | +6.6 ± 0.55 | +2.8 ± 0.02 | +9.5 ± 0.43 | +11.3 ± 0.64 | −1.8 ± 0.89 |
| Seed 22, first mapping, assignment | 6 | +4.0 ± 0.44 | +0.3 ± 0.61 | +3.8 ± 0.61 | +4.0 ± 0.43 | −0.1 ± 0.62 | +4.1 ± 0.40 | +4.1 ± 0.38 | +0.1 ± 0.51 | +4.0 ± 0.36 |
| Seed 22, first mapping, assignment | 12 | +4.3 ± 0.73 | −1.3 ± 0.41 | +5.6 ± 0.91 | +4.3 ± 0.76 | −2.1 ± 0.31 | +6.4 ± 1.00 | +3.8 ± 0.59 | −0.2 ± 0.45 | +4.1 ± 0.81 |
| Seed 22, first mapping, assignment | 18 | +9.3 ± 1.23 | −1.4 ± 0.72 | +10.6 ± 1.04 | +8.9 ± 1.12 | −2.2 ± 0.39 | +11.0 ± 1.36 | +8.7 ± 0.93 | −0.7 ± 0.49 | +9.4 ± 1.03 |
| Seed 22, first mapping, assignment | 24 | +9.2 ± 1.21 | +1.8 ± 0.79 | +7.4 ± 1.72 | +8.3 ± 1.33 | −1.4 ± 0.94 | +9.6 ± 1.83 | +8.3 ± 1.59 | +1.6 ± 0.77 | +6.6 ± 1.96 |
| Seed 22, first mapping, assignment | 36 | +9.8 ± 1.31 | +0.4 ± 0.42 | +9.4 ± 1.53 | +9.1 ± 0.92 | −3.0 ± 0.50 | +12.1 ± 1.26 | +8.4 ± 0.75 | −0.1 ± 0.51 | +8.5 ± 1.29 |
| Seed 23, eighth, mirrored | 6 | +2.4 ± 1.06 | −0.3 ± 0.83 | +2.7 ± 1.35 | +2.0 ± 0.90 | −0.3 ± 0.84 | +2.3 ± 1.27 | +2.2 ± 0.84 | −0.9 ± 0.80 | +3.1 ± 1.27 |
| Seed 23, eighth, mirrored | 12 | +5.4 ± 0.86 | +2.6 ± 0.51 | +2.8 ± 1.08 | +4.1 ± 0.47 | +2.5 ± 0.54 | +1.7 ± 0.83 | +5.8 ± 0.71 | +2.4 ± 0.56 | +3.4 ± 1.10 |
| Seed 23, eighth, mirrored | 18 | +10.6 ± 0.55 | +1.0 ± 0.77 | +9.6 ± 1.20 | +9.5 ± 1.04 | +0.2 ± 0.60 | +9.3 ± 1.19 | +11.1 ± 0.85 | +1.0 ± 0.60 | +10.2 ± 1.11 |
| Seed 23, eighth, mirrored | 24 | +15.3 ± 0.85 | +2.1 ± 0.85 | +13.2 ± 1.24 | +14.7 ± 1.25 | +0.6 ± 0.59 | +14.1 ± 0.96 | +15.6 ± 1.33 | +2.3 ± 0.85 | +13.3 ± 1.13 |
| Seed 23, eighth, mirrored | 36 | +14.8 ± 0.79 | +2.2 ± 0.70 | +12.6 ± 1.17 | +14.5 ± 0.71 | +0.8 ± 0.43 | +13.7 ± 0.74 | +15.3 ± 0.60 | +1.6 ± 0.29 | +13.7 ± 0.64 |
| Seed 23, first mapping, assignment | 6 | +1.7 ± 0.45 | +0.6 ± 0.65 | +1.1 ± 0.78 | +1.8 ± 0.46 | +0.4 ± 0.73 | +1.4 ± 0.91 | +1.7 ± 0.47 | +0.2 ± 0.60 | +1.5 ± 0.73 |
| Seed 23, first mapping, assignment | 12 | +5.9 ± 0.85 | −1.1 ± 0.93 | +7.0 ± 1.44 | +6.1 ± 0.86 | −1.9 ± 1.00 | +8.0 ± 1.48 | +6.2 ± 0.94 | −1.6 ± 1.14 | +7.9 ± 1.58 |
| Seed 23, first mapping, assignment | 18 | +10.1 ± 1.53 | −2.8 ± 0.86 | +12.9 ± 1.53 | +10.8 ± 1.62 | −3.6 ± 1.12 | +14.3 ± 1.28 | +10.1 ± 1.55 | −3.1 ± 1.07 | +13.3 ± 1.35 |
| Seed 23, first mapping, assignment | 24 | +12.5 ± 1.53 | −3.2 ± 0.54 | +15.7 ± 1.58 | +13.1 ± 1.57 | −3.3 ± 0.69 | +16.4 ± 1.26 | +12.6 ± 1.81 | −3.3 ± 0.63 | +15.9 ± 1.69 |
| Seed 23, first mapping, assignment | 36 | +13.1 ± 1.27 | −0.4 ± 0.32 | +13.6 ± 1.36 | +13.1 ± 0.99 | −0.4 ± 0.84 | +13.5 ± 0.81 | +13.4 ± 1.33 | −0.5 ± 0.38 | +13.8 ± 1.05 |
| Seed 24, eighth, mirrored | 6 | −2.6 ± 0.83 | −0.4 ± 1.17 | −2.3 ± 1.54 | −0.6 ± 0.68 | −0.9 ± 1.16 | +0.3 ± 1.42 | −2.0 ± 0.91 | −0.4 ± 0.99 | −1.7 ± 1.51 |
| Seed 24, eighth, mirrored | 12 | −3.0 ± 0.86 | +0.4 ± 0.79 | −3.4 ± 1.08 | −0.7 ± 0.82 | +0.5 ± 0.91 | −1.2 ± 1.12 | −2.1 ± 0.84 | +0.2 ± 0.80 | −2.3 ± 1.23 |
| Seed 24, eighth, mirrored | 18 | +2.2 ± 1.14 | +7.6 ± 1.47 | −5.4 ± 2.35 | +4.3 ± 0.98 | +6.2 ± 1.18 | −1.8 ± 1.53 | +2.3 ± 0.95 | +6.9 ± 1.44 | −4.6 ± 1.57 |
| Seed 24, eighth, mirrored | 24 | +2.0 ± 1.86 | +6.7 ± 1.37 | −4.6 ± 2.64 | +5.0 ± 1.75 | +7.3 ± 1.53 | −2.3 ± 2.25 | +3.0 ± 1.62 | +7.5 ± 1.69 | −4.5 ± 2.38 |
| Seed 24, eighth, mirrored | 36 | +3.5 ± 0.69 | +8.0 ± 1.81 | −4.5 ± 2.42 | +4.3 ± 1.06 | +8.6 ± 1.29 | −4.4 ± 2.12 | +2.1 ± 0.32 | +9.9 ± 1.80 | −7.8 ± 1.79 |
| Seed 24, first mapping, assignment | 6 | +1.9 ± 0.58 | +1.1 ± 0.33 | +0.8 ± 0.63 | +2.1 ± 0.54 | +1.2 ± 0.38 | +0.9 ± 0.64 | +2.0 ± 0.58 | +1.2 ± 0.40 | +0.8 ± 0.70 |
| Seed 24, first mapping, assignment | 12 | +2.8 ± 0.45 | +1.0 ± 0.82 | +1.7 ± 1.08 | +3.0 ± 0.45 | −0.2 ± 1.04 | +3.2 ± 1.29 | +2.5 ± 0.48 | +0.3 ± 0.86 | +2.3 ± 1.16 |
| Seed 24, first mapping, assignment | 18 | +4.7 ± 0.49 | +0.3 ± 1.53 | +4.4 ± 1.67 | +5.0 ± 0.41 | −0.5 ± 0.84 | +5.5 ± 0.98 | +4.5 ± 0.68 | −0.5 ± 1.74 | +5.1 ± 2.24 |
| Seed 24, first mapping, assignment | 24 | +6.9 ± 0.96 | +2.5 ± 2.02 | +4.3 ± 1.37 | +7.5 ± 1.10 | +1.9 ± 1.81 | +5.5 ± 1.13 | +7.2 ± 1.06 | +2.3 ± 1.64 | +4.9 ± 1.26 |
| Seed 24, first mapping, assignment | 36 | +5.0 ± 0.52 | −0.6 ± 1.19 | +5.6 ± 1.42 | +5.0 ± 0.50 | +1.5 ± 0.96 | +3.5 ± 1.13 | +4.0 ± 0.75 | +0.8 ± 1.09 | +3.2 ± 1.04 |
| Seed 23, eighth, assignment | 6 | +2.9 ± 0.77 | −0.1 ± 0.68 | +2.9 ± 1.08 | +2.5 ± 0.71 | +0.7 ± 0.60 | +1.8 ± 0.88 | +2.4 ± 0.78 | +0.4 ± 0.66 | +2.0 ± 0.99 |
| Seed 23, eighth, assignment | 12 | +3.9 ± 0.86 | −0.6 ± 0.82 | +4.5 ± 1.17 | +3.7 ± 0.94 | +1.3 ± 0.69 | +2.4 ± 1.20 | +3.1 ± 0.83 | −0.5 ± 0.78 | +3.6 ± 0.86 |
| Seed 23, eighth, assignment | 18 | +7.0 ± 1.07 | +0.4 ± 0.58 | +6.6 ± 1.00 | +5.9 ± 0.95 | +4.6 ± 0.37 | +1.4 ± 1.03 | +7.4 ± 0.96 | +3.9 ± 0.76 | +3.5 ± 1.19 |
| Seed 23, eighth, assignment | 24 | +7.7 ± 1.57 | −0.8 ± 1.30 | +8.4 ± 1.90 | +5.1 ± 1.25 | +4.5 ± 1.17 | +0.5 ± 1.21 | +5.9 ± 1.08 | +3.4 ± 0.73 | +2.5 ± 1.27 |
| Seed 23, eighth, assignment | 36 | +5.8 ± 0.67 | −1.3 ± 2.57 | +7.0 ± 2.51 | +4.0 ± 0.58 | +2.7 ± 2.00 | +1.3 ± 1.50 | +4.5 ± 0.21 | +2.0 ± 1.92 | +2.4 ± 1.95 |
| Seed 24, first mapping, mirrored | 6 | +1.5 ± 0.47 | +1.3 ± 0.63 | +0.3 ± 0.75 | +1.8 ± 0.54 | +1.3 ± 0.60 | +0.6 ± 0.79 | +2.0 ± 0.51 | +1.2 ± 0.62 | +0.8 ± 0.81 |
| Seed 24, first mapping, mirrored | 12 | +2.6 ± 0.61 | +2.3 ± 1.11 | +0.4 ± 0.98 | +2.1 ± 0.62 | +1.5 ± 0.92 | +0.6 ± 0.86 | +2.8 ± 0.57 | +1.6 ± 1.05 | +1.1 ± 0.74 |
| Seed 24, first mapping, mirrored | 18 | +5.1 ± 0.52 | +3.2 ± 0.69 | +1.9 ± 0.95 | +4.8 ± 0.41 | +2.2 ± 0.91 | +2.6 ± 0.91 | +5.4 ± 0.46 | +2.5 ± 0.69 | +2.9 ± 0.64 |
| Seed 24, first mapping, mirrored | 24 | +3.7 ± 0.85 | +5.0 ± 0.58 | −1.4 ± 1.10 | +3.4 ± 1.13 | +4.4 ± 0.72 | −1.0 ± 0.87 | +4.1 ± 0.90 | +4.1 ± 0.62 | 0.0 ± 1.10 |
| Seed 24, first mapping, mirrored | 36 | +3.0 ± 0.83 | +4.0 ± 1.15 | −1.0 ± 0.95 | +3.1 ± 0.80 | +3.6 ± 0.97 | −0.5 ± 0.83 | +4.1 ± 0.54 | +2.9 ± 1.07 | +1.3 ± 0.87 |
| Seed 23, first mapping, mirrored | 6 | +0.8 ± 0.34 | +0.7 ± 0.65 | 0.0 ± 0.71 | +0.6 ± 0.34 | +1.3 ± 0.71 | −0.6 ± 0.74 | +0.7 ± 0.33 | +1.1 ± 0.53 | −0.4 ± 0.62 |
| Seed 23, first mapping, mirrored | 12 | +1.1 ± 0.58 | 0.0 ± 1.22 | +1.1 ± 1.42 | +1.3 ± 0.68 | +2.0 ± 0.92 | −0.8 ± 1.14 | +1.0 ± 0.64 | +1.8 ± 0.90 | −0.8 ± 1.08 |
| Seed 23, first mapping, mirrored | 18 | −0.1 ± 0.70 | −0.4 ± 0.78 | +0.3 ± 0.94 | −0.8 ± 0.81 | +3.3 ± 0.46 | −4.1 ± 0.93 | −0.7 ± 0.77 | +0.8 ± 0.71 | −1.6 ± 0.87 |
| Seed 23, first mapping, mirrored | 24 | −4.7 ± 0.92 | +0.9 ± 0.81 | −5.6 ± 1.68 | −5.2 ± 1.32 | +4.2 ± 0.46 | −9.4 ± 1.42 | −4.4 ± 1.16 | +2.1 ± 0.54 | −6.5 ± 1.16 |
| Seed 23, first mapping, mirrored | 36 | −3.9 ± 1.05 | −0.7 ± 1.11 | −3.1 ± 1.95 | −5.0 ± 1.34 | +1.9 ± 0.75 | −6.9 ± 2.02 | −4.6 ± 0.88 | −0.1 ± 0.34 | −4.5 ± 1.16 |
| Seed 24, eighth, assignment | 6 | +1.8 ± 0.78 | −2.9 ± 0.60 | +4.7 ± 1.23 | +1.9 ± 0.98 | −1.9 ± 0.57 | +3.9 ± 1.32 | +2.3 ± 0.87 | −2.3 ± 0.73 | +4.7 ± 1.38 |
| Seed 24, eighth, assignment | 12 | +0.1 ± 1.33 | −1.5 ± 1.69 | +1.6 ± 2.63 | +0.5 ± 1.17 | −1.2 ± 1.12 | +1.7 ± 2.06 | +0.3 ± 1.25 | −0.1 ± 1.56 | +0.4 ± 2.60 |
| Seed 24, eighth, assignment | 18 | −4.3 ± 1.03 | −0.2 ± 1.01 | −4.1 ± 1.42 | −3.6 ± 1.00 | −0.8 ± 1.81 | −2.8 ± 1.48 | −1.9 ± 0.73 | −0.8 ± 1.50 | −1.1 ± 1.92 |
| Seed 24, eighth, assignment | 24 | −4.7 ± 1.31 | −1.2 ± 0.72 | −3.5 ± 1.15 | −5.4 ± 1.14 | −0.4 ± 1.61 | −5.0 ± 1.48 | −2.2 ± 1.09 | −1.6 ± 1.74 | −0.6 ± 1.45 |
| Seed 24, eighth, assignment | 36 | +0.2 ± 0.73 | +0.5 ± 1.22 | −0.3 ± 1.53 | +0.5 ± 0.50 | +0.9 ± 0.95 | −0.5 ± 1.28 | +1.9 ± 1.08 | +1.6 ± 1.41 | +0.4 ± 1.00 |
| Seed 22, first mapping, mirrored | 6 | +0.7 ± 0.32 | +0.6 ± 0.51 | +0.1 ± 0.67 | +0.7 ± 0.35 | +0.8 ± 0.49 | −0.1 ± 0.59 | +0.6 ± 0.35 | +0.1 ± 0.57 | +0.5 ± 0.69 |
| Seed 22, first mapping, mirrored | 12 | +1.3 ± 0.48 | +1.8 ± 1.48 | −0.5 ± 1.17 | +0.8 ± 0.69 | +2.8 ± 1.51 | −2.1 ± 1.46 | +1.7 ± 0.39 | +2.6 ± 1.71 | −1.0 ± 1.45 |
| Seed 22, first mapping, mirrored | 18 | −0.2 ± 0.94 | +3.7 ± 1.49 | −3.9 ± 1.30 | −0.3 ± 1.46 | +4.7 ± 1.28 | −5.0 ± 1.41 | +0.6 ± 0.54 | +4.5 ± 1.55 | −4.0 ± 1.36 |
| Seed 22, first mapping, mirrored | 24 | +0.6 ± 1.15 | +4.8 ± 1.51 | −4.2 ± 1.22 | −0.1 ± 1.41 | +5.0 ± 1.08 | −5.1 ± 1.09 | +0.9 ± 1.19 | +5.5 ± 1.21 | −4.6 ± 1.24 |
| Seed 22, first mapping, mirrored | 36 | −0.1 ± 0.82 | +5.5 ± 1.39 | −5.5 ± 1.91 | −0.8 ± 0.89 | +5.7 ± 0.87 | −6.5 ± 1.26 | +0.4 ± 0.79 | +5.7 ± 1.25 | −5.3 ± 1.75 |
| Seed 22, eighth, assignment | 6 | +2.0 ± 0.62 | −0.1 ± 0.59 | +2.2 ± 0.96 | +1.8 ± 0.57 | +0.3 ± 0.62 | +1.5 ± 1.00 | +1.9 ± 0.43 | +0.4 ± 0.58 | +1.5 ± 0.73 |
| Seed 22, eighth, assignment | 12 | +2.8 ± 0.71 | +0.7 ± 0.92 | +2.1 ± 1.34 | +2.3 ± 0.61 | +1.7 ± 0.52 | +0.6 ± 0.88 | +2.8 ± 0.70 | +1.0 ± 0.77 | +1.8 ± 1.23 |
| Seed 22, eighth, assignment | 18 | −0.6 ± 0.74 | +3.8 ± 0.82 | −4.4 ± 0.96 | 0.0 ± 0.86 | +4.5 ± 0.63 | −4.5 ± 1.29 | +1.1 ± 0.51 | +3.9 ± 0.53 | −2.8 ± 0.86 |
| Seed 22, eighth, assignment | 24 | −0.5 ± 0.65 | +2.9 ± 0.98 | −3.4 ± 1.08 | −1.0 ± 0.99 | +4.6 ± 1.03 | −5.6 ± 1.67 | +0.5 ± 0.65 | +3.7 ± 0.93 | −3.3 ± 0.71 |
| Seed 22, eighth, assignment | 36 | −1.6 ± 0.70 | +3.2 ± 1.04 | −4.8 ± 1.41 | −1.0 ± 1.22 | +4.3 ± 1.11 | −5.2 ± 2.30 | −0.5 ± 0.90 | +4.7 ± 0.94 | −5.2 ± 1.64 |

| Cell | Block | Own group: side | Inhibitory units: side | Outside the readouts: side | H-47's side | Held within (027): side | Excitatory: side |
| :--- | ---: | :--- | :--- | :--- | :--- | :--- | :--- |
| Seed 22, eighth, mirrored | 6 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | 12 | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places | A, the odd places |
| Seed 22, eighth, mirrored | 18 | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | 24 | B, the even places | B, the even places (within 2 SE) | B, the even places | B, the even places | B, the even places (within 2 SE) | B, the even places (within 2 SE) |
| Seed 22, eighth, mirrored | 36 | B, the even places (within 2 SE) | A, the odd places | B, the even places | B, the even places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places |
| Seed 22, first mapping, assignment | 6 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 12 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 18 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 24 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 36 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | 6 | A, the odd places | A, the odd places (within 2 SE) | A, the odd places | A, the odd places | A, the odd places | A, the odd places (within 2 SE) |
| Seed 23, eighth, mirrored | 12 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places (within 2 SE) | B, the even places (within 2 SE) |
| Seed 23, eighth, mirrored | 18 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | 24 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | 36 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 6 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) |
| Seed 23, first mapping, assignment | 12 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 18 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 24 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 36 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 24, eighth, mirrored | 6 | B, the even places (within 2 SE) | A, the odd places (within 2 SE) | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places (within 2 SE) |
| Seed 24, eighth, mirrored | 12 | B, the even places | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places | B, the even places (within 2 SE) | B, the even places (within 2 SE) |
| Seed 24, eighth, mirrored | 18 | B, the even places | B, the even places (within 2 SE) | B, the even places | B, the even places | A, the odd places (within 2 SE) | B, the even places (within 2 SE) |
| Seed 24, eighth, mirrored | 24 | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 24, eighth, mirrored | 36 | B, the even places (within 2 SE) | B, the even places | B, the even places | B, the even places | B, the even places (within 2 SE) | B, the even places (within 2 SE) |
| Seed 24, first mapping, assignment | 6 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 24, first mapping, assignment | 12 | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) |
| Seed 24, first mapping, assignment | 18 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 24, first mapping, assignment | 24 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 24, first mapping, assignment | 36 | A, the odd places | A, the odd places | A, the odd places | A, the odd places (within 2 SE) | A, the odd places | A, the odd places |
| Seed 23, eighth, assignment | 6 | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places | A, the even places | A, the even places |
| Seed 23, eighth, assignment | 12 | A, the even places | A, the even places (within 2 SE) | A, the even places | A, the even places | A, the even places | A, the even places |
| Seed 23, eighth, assignment | 18 | A, the even places | A, the even places (within 2 SE) | A, the even places | A, the even places (within 2 SE) | A, the even places | A, the even places |
| Seed 23, eighth, assignment | 24 | A, the even places | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places | A, the even places |
| Seed 23, eighth, assignment | 36 | A, the even places | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places | A, the even places |
| Seed 24, first mapping, mirrored | 6 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | 12 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 24, first mapping, mirrored | 18 | A, the even places (within 2 SE) | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | 24 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places |
| Seed 24, first mapping, mirrored | 36 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 6 | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 12 | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 23, first mapping, mirrored | 18 | A, the even places (within 2 SE) | B, the odd places | B, the odd places (within 2 SE) | B, the odd places | B, the odd places (within 2 SE) | B, the odd places |
| Seed 23, first mapping, mirrored | 24 | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 23, first mapping, mirrored | 36 | B, the odd places (within 2 SE) | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 24, eighth, assignment | 6 | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places |
| Seed 24, eighth, assignment | 12 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, eighth, assignment | 18 | B, the odd places | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | 24 | B, the odd places | B, the odd places | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places | B, the odd places |
| Seed 24, eighth, assignment | 36 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | 6 | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | 12 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 22, first mapping, mirrored | 18 | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, first mapping, mirrored | 24 | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, first mapping, mirrored | 36 | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | 6 | A, the even places | A, the even places (within 2 SE) | A, the even places | A, the even places | A, the even places | A, the even places |
| Seed 22, eighth, assignment | 12 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 22, eighth, assignment | 18 | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | 24 | B, the odd places | B, the odd places | B, the odd places | B, the odd places (within 2 SE) | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | 36 | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |

- **The side under the night held onto the own group is H-47's at 57 of the 60 checkpoints**, exploration 027's night held within's at 53
  and the excitatory night's at 53. It lies beyond twice its SE at 36 of the 60, where H-47's and the excitatory night's lie beyond at 35
  and exploration 027's night held within's at 36. It differs from H-47's at three:
  - seed 24's first mapping from the mirrored assignment at block 36: on B, the odd places, within 2 SE, where H-47's lies on A, the even
    places, within 2 SE;
  - seed 23's first mapping from the mirrored assignment at blocks 6 and 18: on A, the even places, within 2 SE, where H-47's lies on B, the
    odd places, within 2 SE at block 6 and beyond 2 SE at block 18.
- **The side under the night held onto the inhibitory units is H-47's at 55 of the 60 checkpoints**, exploration 027's night held within's
  at 55 and the excitatory night's at 53. It lies beyond twice its SE at 34 of the 60. It differs from H-47's at five:
  - seed 22's eighth image from the mirrored assignment at block 36: on A, the odd places, beyond 2 SE, where H-47's lies on B, the even
    places, within 2 SE;
  - seed 24's eighth image from the mirrored assignment at block 6: on A, the odd places, within 2 SE, where H-47's lies on B, the even
    places, within 2 SE;
  - seed 24's first mapping from the mirrored assignment at block 36, seed 23's first mapping from the mirrored assignment at block 12 and
    seed 22's first mapping from the mirrored assignment at block 6: on B, the odd places, within 2 SE, where H-47's lies on A, the even
    places, within 2 SE.
- **The side under the night held onto the units outside the readouts is H-47's at 58 of the 60 checkpoints**, exploration 027's night
  held within's at 54 and the excitatory night's at 54. It lies beyond twice its SE at 35 of the 60. It differs from H-47's at two:
  - seed 23's first mapping from the mirrored assignment at block 12: on B, the odd places, within 2 SE, where H-47's lies on A, the even
    places, within 2 SE;
  - seed 24's eighth image from the assignment at block 36: on A, the even places, within 2 SE, where H-47's lies on B, the odd places,
    within 2 SE.
- **Where each differs from exploration 027's night held within's**: the night held onto the own group at seven checkpoints, seed 22's
  eighth image from the mirrored assignment at block 36, seed 24's eighth image from the mirrored assignment at blocks 18 and 24, seed 23's
  first mapping from the mirrored assignment at blocks 6 and 18, seed 24's eighth image from the assignment at block 36 and seed 22's first
  mapping from the mirrored assignment at block 6; the night held onto the inhibitory units at five, seed 24's eighth image from the
  mirrored assignment at blocks 6, 18 and 24, seed 23's first mapping from the mirrored assignment at block 12 and seed 24's eighth image
  from the assignment at block 36; the night held onto the units outside the readouts at six, seed 22's eighth image from the mirrored
  assignment at block 36, seed 24's eighth image from the mirrored assignment at blocks 18 and 24, seed 24's first mapping from the mirrored
  assignment at block 36, seed 23's first mapping from the mirrored assignment at block 12 and seed 22's first mapping from the mirrored
  assignment at block 6.
- **Where each differs from the excitatory night's**: the night held onto the own group at seven, seed 22's eighth image from the mirrored
  assignment at block 36, seed 23's eighth image from the mirrored assignment at block 12, seed 24's eighth image from the mirrored
  assignment at block 24, seed 24's first mapping from the mirrored assignment at block 12, seed 23's first mapping from the mirrored
  assignment at blocks 6 and 18 and seed 24's eighth image from the assignment at block 36; the night held onto the inhibitory units at
  seven, seed 23's eighth image from the mirrored assignment at block 12, seed 24's eighth image from the mirrored assignment at blocks 6 and
  24, seed 24's first mapping from the mirrored assignment at block 12, seed 23's first mapping from the mirrored assignment at block 12,
  seed 24's eighth image from the assignment at block 36 and seed 22's first mapping from the mirrored assignment at block 6; the night held
  onto the units outside the readouts at six, seed 22's eighth image from the mirrored assignment at block 36, seed 23's eighth image from
  the mirrored assignment at block 12, seed 24's eighth image from the mirrored assignment at block 24, seed 24's first mapping from the
  mirrored assignment at blocks 12 and 36 and seed 23's first mapping from the mirrored assignment at block 12.

Each stimulus's trials less H-47's plain night's, less exploration 027's night held within's and less the excitatory night's, each with
exploration 018's unpaired SE:

| Cell | Block | Own group less H-47's plain: A's | B's | Inhibitory units: A's | B's | Outside the readouts: A's | B's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 0.0 ± 0.86 | −0.3 ± 0.98 | +0.5 ± 0.90 | −1.0 ± 0.96 | −0.1 ± 0.87 | 0.0 ± 0.97 |
| Seed 22, eighth, mirrored | 12 | −1.2 ± 1.10 | −0.1 ± 0.88 | +0.6 ± 1.20 | −2.1 ± 0.92 | −0.1 ± 0.88 | +0.5 ± 0.79 |
| Seed 22, eighth, mirrored | 18 | −1.3 ± 1.14 | −0.3 ± 0.98 | +0.5 ± 1.07 | −1.8 ± 1.10 | −0.1 ± 0.87 | +0.1 ± 0.97 |
| Seed 22, eighth, mirrored | 24 | −1.9 ± 1.28 | −0.8 ± 0.98 | 0.0 ± 0.72 | −4.9 ± 1.28 | +0.9 ± 0.82 | +1.0 ± 0.97 |
| Seed 22, eighth, mirrored | 36 | −1.8 ± 1.05 | −1.0 ± 0.89 | +0.2 ± 1.01 | −3.8 ± 1.07 | +0.2 ± 1.05 | +0.9 ± 0.62 |
| Seed 22, first mapping, assignment | 6 | 0.0 ± 0.81 | +0.3 ± 0.63 | 0.0 ± 0.81 | −0.1 ± 0.64 | 0.0 ± 0.83 | +0.1 ± 0.61 |
| Seed 22, first mapping, assignment | 12 | +0.5 ± 0.85 | −0.3 ± 1.30 | +0.5 ± 0.89 | −1.2 ± 1.21 | 0.0 ± 0.74 | +0.7 ± 1.30 |
| Seed 22, first mapping, assignment | 18 | +1.0 ± 1.06 | +0.2 ± 0.95 | +0.5 ± 1.00 | −0.6 ± 0.81 | +0.4 ± 0.88 | +0.8 ± 0.76 |
| Seed 22, first mapping, assignment | 24 | +0.9 ± 0.81 | +1.0 ± 1.16 | 0.0 ± 0.78 | −2.2 ± 1.00 | 0.0 ± 0.55 | +0.8 ± 0.87 |
| Seed 22, first mapping, assignment | 36 | +1.5 ± 0.89 | +0.9 ± 0.83 | +0.7 ± 0.82 | −2.6 ± 0.90 | 0.0 ± 0.97 | +0.3 ± 0.74 |
| Seed 23, eighth, mirrored | 6 | −0.1 ± 1.17 | +0.5 ± 0.94 | −0.6 ± 1.19 | +0.5 ± 0.83 | −0.3 ± 1.23 | 0.0 ± 0.91 |
| Seed 23, eighth, mirrored | 12 | −0.7 ± 1.04 | −0.2 ± 0.82 | −2.0 ± 0.88 | −0.4 ± 0.73 | −0.4 ± 1.18 | −0.5 ± 0.75 |
| Seed 23, eighth, mirrored | 18 | −1.5 ± 1.12 | +0.4 ± 0.47 | −2.6 ± 0.86 | −0.4 ± 0.52 | −1.0 ± 0.80 | +0.4 ± 0.30 |
| Seed 23, eighth, mirrored | 24 | −1.6 ± 1.20 | +0.3 ± 0.93 | −2.2 ± 0.96 | −1.2 ± 0.57 | −1.3 ± 1.34 | +0.6 ± 0.77 |
| Seed 23, eighth, mirrored | 36 | −1.1 ± 1.42 | +0.7 ± 0.86 | −1.4 ± 1.37 | −0.7 ± 0.54 | −0.6 ± 1.28 | +0.1 ± 0.31 |
| Seed 23, first mapping, assignment | 6 | 0.0 ± 0.82 | +0.2 ± 1.06 | +0.1 ± 0.84 | 0.0 ± 1.15 | +0.1 ± 0.83 | −0.2 ± 0.99 |
| Seed 23, first mapping, assignment | 12 | −0.1 ± 0.81 | +0.7 ± 0.99 | +0.1 ± 0.84 | −0.1 ± 1.02 | +0.3 ± 0.81 | +0.2 ± 1.05 |
| Seed 23, first mapping, assignment | 18 | −0.5 ± 1.06 | −0.1 ± 0.77 | +0.2 ± 1.08 | −0.8 ± 0.65 | −0.5 ± 1.04 | −0.4 ± 0.72 |
| Seed 23, first mapping, assignment | 24 | −0.7 ± 0.91 | 0.0 ± 0.70 | −0.1 ± 0.90 | 0.0 ± 0.55 | −0.6 ± 0.96 | −0.1 ± 0.62 |
| Seed 23, first mapping, assignment | 36 | −0.4 ± 0.91 | −0.3 ± 0.62 | −0.4 ± 0.90 | −0.3 ± 0.56 | −0.1 ± 0.96 | −0.3 ± 0.47 |
| Seed 24, eighth, mirrored | 6 | −0.5 ± 0.94 | +0.1 ± 1.02 | +1.5 ± 1.08 | −0.4 ± 1.04 | +0.1 ± 1.04 | +0.1 ± 0.99 |
| Seed 24, eighth, mirrored | 12 | +0.2 ± 0.82 | +0.1 ± 1.05 | +2.5 ± 0.99 | +0.2 ± 1.09 | +1.1 ± 0.98 | −0.1 ± 1.20 |
| Seed 24, eighth, mirrored | 18 | +1.5 ± 1.06 | 0.0 ± 1.43 | +3.6 ± 0.77 | −1.5 ± 1.27 | +1.5 ± 0.70 | −0.8 ± 1.19 |
| Seed 24, eighth, mirrored | 24 | +0.4 ± 1.24 | −1.1 ± 1.07 | +3.4 ± 1.15 | −0.5 ± 1.07 | +1.4 ± 1.31 | −0.3 ± 0.95 |
| Seed 24, eighth, mirrored | 36 | +1.9 ± 0.74 | −1.7 ± 1.22 | +2.7 ± 0.91 | −1.1 ± 0.88 | +0.5 ± 1.02 | +0.2 ± 1.16 |
| Seed 24, first mapping, assignment | 6 | 0.0 ± 1.03 | 0.0 ± 0.95 | +0.3 ± 0.99 | +0.1 ± 0.91 | +0.1 ± 1.01 | +0.1 ± 0.92 |
| Seed 24, first mapping, assignment | 12 | +0.6 ± 0.92 | +0.5 ± 1.05 | +0.8 ± 0.83 | −0.8 ± 1.02 | +0.3 ± 0.91 | −0.3 ± 1.05 |
| Seed 24, first mapping, assignment | 18 | +0.7 ± 0.93 | +0.4 ± 1.20 | +1.0 ± 0.88 | −0.4 ± 1.34 | +0.5 ± 0.84 | −0.4 ± 1.10 |
| Seed 24, first mapping, assignment | 24 | +0.8 ± 0.98 | −0.1 ± 0.80 | +1.4 ± 1.13 | −0.7 ± 0.97 | +1.1 ± 1.04 | −0.3 ± 1.18 |
| Seed 24, first mapping, assignment | 36 | +1.8 ± 0.84 | −1.1 ± 0.75 | +1.8 ± 1.02 | +1.0 ± 0.93 | +0.8 ± 1.11 | +0.3 ± 0.80 |
| Seed 23, eighth, assignment | 6 | +0.2 ± 0.77 | −0.4 ± 1.23 | −0.1 ± 0.82 | +0.3 ± 1.21 | −0.3 ± 0.78 | +0.1 ± 1.19 |
| Seed 23, eighth, assignment | 12 | +1.0 ± 1.30 | −0.2 ± 1.27 | +0.8 ± 1.39 | +1.7 ± 1.16 | +0.2 ± 1.37 | −0.1 ± 1.16 |
| Seed 23, eighth, assignment | 18 | +1.8 ± 0.88 | −2.3 ± 1.05 | +0.7 ± 0.84 | +1.9 ± 1.05 | +2.1 ± 0.80 | +1.2 ± 1.58 |
| Seed 23, eighth, assignment | 24 | +4.4 ± 1.03 | −3.2 ± 1.54 | +1.8 ± 0.91 | +2.1 ± 1.59 | +2.6 ± 1.08 | +0.9 ± 1.31 |
| Seed 23, eighth, assignment | 36 | +3.5 ± 0.75 | −3.1 ± 2.01 | +1.7 ± 0.83 | +0.9 ± 1.49 | +2.2 ± 0.97 | +0.2 ± 1.44 |
| Seed 24, first mapping, mirrored | 6 | −0.5 ± 1.00 | −0.1 ± 0.91 | −0.2 ± 1.00 | −0.1 ± 0.93 | 0.0 ± 1.01 | −0.2 ± 0.96 |
| Seed 24, first mapping, mirrored | 12 | 0.0 ± 1.06 | +0.2 ± 0.87 | −0.5 ± 0.90 | −0.5 ± 0.94 | +0.1 ± 0.93 | −0.5 ± 0.88 |
| Seed 24, first mapping, mirrored | 18 | −0.3 ± 1.17 | +0.7 ± 0.98 | −0.6 ± 1.17 | −0.3 ± 0.85 | +0.1 ± 1.21 | +0.1 ± 0.94 |
| Seed 24, first mapping, mirrored | 24 | 0.0 ± 0.93 | +1.1 ± 1.19 | −0.3 ± 1.07 | +0.5 ± 0.99 | +0.4 ± 0.98 | +0.2 ± 1.14 |
| Seed 24, first mapping, mirrored | 36 | −0.3 ± 0.56 | +1.3 ± 0.75 | −0.1 ± 0.61 | +0.9 ± 0.92 | +0.9 ± 0.54 | +0.1 ± 0.88 |
| Seed 23, first mapping, mirrored | 6 | +0.2 ± 0.99 | −0.6 ± 0.89 | +0.1 ± 0.99 | 0.0 ± 0.99 | +0.1 ± 0.99 | −0.2 ± 0.90 |
| Seed 23, first mapping, mirrored | 12 | 0.0 ± 0.90 | −0.9 ± 1.22 | +0.2 ± 0.94 | +1.1 ± 1.34 | −0.1 ± 0.91 | +0.9 ± 1.27 |
| Seed 23, first mapping, mirrored | 18 | +0.8 ± 1.10 | −1.7 ± 1.02 | +0.1 ± 1.18 | +2.0 ± 0.90 | +0.2 ± 1.18 | −0.5 ± 0.96 |
| Seed 23, first mapping, mirrored | 24 | +0.3 ± 0.86 | −1.5 ± 1.28 | −0.2 ± 0.82 | +1.8 ± 1.09 | +0.6 ± 0.82 | −0.3 ± 1.08 |
| Seed 23, first mapping, mirrored | 36 | +1.3 ± 0.99 | −1.0 ± 0.85 | +0.2 ± 1.15 | +1.6 ± 0.84 | +0.6 ± 1.12 | −0.4 ± 1.03 |
| Seed 24, eighth, assignment | 6 | −0.8 ± 0.82 | −0.7 ± 0.86 | −0.6 ± 0.76 | +0.3 ± 0.97 | −0.2 ± 0.83 | −0.1 ± 0.97 |
| Seed 24, eighth, assignment | 12 | −0.3 ± 0.95 | −0.5 ± 1.09 | +0.1 ± 0.97 | −0.2 ± 1.10 | −0.1 ± 0.96 | +0.9 ± 0.95 |
| Seed 24, eighth, assignment | 18 | −0.8 ± 0.97 | +2.1 ± 1.10 | 0.0 ± 1.05 | +1.5 ± 0.99 | +1.7 ± 0.89 | +1.6 ± 1.01 |
| Seed 24, eighth, assignment | 24 | −0.8 ± 1.43 | +0.8 ± 1.30 | −1.5 ± 1.04 | +1.6 ± 1.44 | +1.7 ± 1.04 | +0.4 ± 1.08 |
| Seed 24, eighth, assignment | 36 | 0.0 ± 0.92 | −1.1 ± 0.83 | +0.3 ± 0.92 | −0.7 ± 1.28 | +1.8 ± 0.65 | 0.0 ± 1.21 |
| Seed 22, first mapping, mirrored | 6 | 0.0 ± 0.87 | +0.3 ± 0.70 | 0.0 ± 0.85 | +0.4 ± 0.71 | −0.1 ± 0.85 | −0.3 ± 0.75 |
| Seed 22, first mapping, mirrored | 12 | +0.2 ± 1.11 | 0.0 ± 1.20 | −0.4 ± 1.16 | +1.0 ± 0.98 | +0.5 ± 1.22 | +0.9 ± 1.09 |
| Seed 22, first mapping, mirrored | 18 | +0.2 ± 0.95 | −0.4 ± 1.02 | +0.1 ± 1.04 | +0.6 ± 1.01 | +1.0 ± 1.08 | +0.5 ± 1.03 |
| Seed 22, first mapping, mirrored | 24 | +0.5 ± 0.71 | +0.2 ± 1.58 | −0.3 ± 0.81 | +0.4 ± 1.56 | +0.8 ± 0.66 | +0.9 ± 1.49 |
| Seed 22, first mapping, mirrored | 36 | +0.3 ± 0.86 | +0.1 ± 1.09 | −0.4 ± 0.82 | +0.3 ± 1.37 | +0.8 ± 0.82 | +0.3 ± 0.88 |
| Seed 22, eighth, assignment | 6 | 0.0 ± 0.75 | 0.0 ± 1.28 | −0.2 ± 0.66 | +0.4 ± 1.26 | −0.1 ± 0.74 | +0.5 ± 1.26 |
| Seed 22, eighth, assignment | 12 | +0.3 ± 0.73 | +0.1 ± 1.13 | −0.2 ± 0.63 | +1.1 ± 1.11 | +0.3 ± 0.76 | +0.4 ± 1.12 |
| Seed 22, eighth, assignment | 18 | 0.0 ± 1.12 | −0.3 ± 0.69 | +0.6 ± 1.33 | +0.4 ± 0.64 | +1.7 ± 1.27 | −0.1 ± 0.75 |
| Seed 22, eighth, assignment | 24 | −0.2 ± 1.28 | −0.2 ± 0.88 | −0.8 ± 1.06 | +1.4 ± 0.87 | +0.8 ± 1.45 | +0.6 ± 1.08 |
| Seed 22, eighth, assignment | 36 | −0.1 ± 1.27 | −0.6 ± 1.31 | +0.6 ± 1.24 | +0.5 ± 1.18 | +1.1 ± 1.33 | +0.9 ± 1.29 |

| Cell | Block | Own group less 027's held within: A's | B's | Inhibitory units: A's | B's | Outside the readouts: A's | B's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | −0.4 ± 0.86 | +1.0 ± 0.88 | +0.1 ± 0.88 | +0.2 ± 0.89 | −0.5 ± 0.88 | +1.2 ± 0.92 |
| Seed 22, eighth, mirrored | 12 | −1.0 ± 1.06 | +1.1 ± 0.90 | +0.7 ± 0.99 | −0.9 ± 0.82 | +0.1 ± 0.89 | +1.8 ± 0.82 |
| Seed 22, eighth, mirrored | 18 | −0.6 ± 1.06 | +0.3 ± 1.15 | +1.2 ± 1.03 | −1.2 ± 1.03 | +0.6 ± 0.95 | +0.7 ± 1.08 |
| Seed 22, eighth, mirrored | 24 | −1.5 ± 0.72 | +2.0 ± 1.11 | +0.5 ± 1.10 | −2.1 ± 1.15 | +1.3 ± 0.79 | +3.8 ± 1.04 |
| Seed 22, eighth, mirrored | 36 | −1.5 ± 0.85 | +1.6 ± 0.85 | +0.5 ± 0.88 | −1.1 ± 1.64 | +0.5 ± 1.21 | +3.5 ± 1.13 |
| Seed 22, first mapping, assignment | 6 | −0.1 ± 0.86 | +0.2 ± 0.76 | −0.1 ± 0.81 | −0.2 ± 0.60 | −0.1 ± 0.83 | −0.1 ± 0.68 |
| Seed 22, first mapping, assignment | 12 | −0.3 ± 0.78 | +0.8 ± 1.41 | −0.3 ± 0.75 | −0.1 ± 1.26 | −0.8 ± 0.78 | +1.8 ± 1.42 |
| Seed 22, first mapping, assignment | 18 | +0.6 ± 0.95 | +0.9 ± 1.41 | +0.1 ± 0.90 | +0.1 ± 0.88 | 0.0 ± 0.98 | +1.6 ± 0.99 |
| Seed 22, first mapping, assignment | 24 | +0.8 ± 0.76 | +1.8 ± 0.98 | −0.1 ± 0.82 | −1.4 ± 1.14 | −0.1 ± 1.05 | +1.6 ± 1.29 |
| Seed 22, first mapping, assignment | 36 | +1.5 ± 1.09 | +2.3 ± 1.11 | +0.8 ± 0.77 | −1.2 ± 0.84 | 0.0 ± 0.85 | +1.7 ± 1.05 |
| Seed 23, eighth, mirrored | 6 | +0.5 ± 1.24 | +0.7 ± 0.87 | 0.0 ± 1.19 | +0.7 ± 0.71 | +0.3 ± 1.24 | +0.1 ± 0.88 |
| Seed 23, eighth, mirrored | 12 | +1.4 ± 1.03 | −0.1 ± 0.83 | +0.1 ± 0.87 | −0.3 ± 0.65 | +1.7 ± 1.03 | −0.4 ± 0.62 |
| Seed 23, eighth, mirrored | 18 | +2.9 ± 0.97 | +0.2 ± 0.90 | +1.8 ± 1.02 | −0.6 ± 0.55 | +3.4 ± 0.97 | +0.1 ± 0.72 |
| Seed 23, eighth, mirrored | 24 | +1.9 ± 0.94 | +0.1 ± 0.73 | +1.3 ± 0.87 | −1.4 ± 0.34 | +2.2 ± 0.98 | +0.4 ± 0.49 |
| Seed 23, eighth, mirrored | 36 | +1.1 ± 1.10 | +0.4 ± 0.70 | +0.8 ± 1.07 | −1.0 ± 0.35 | +1.6 ± 1.12 | −0.2 ± 0.24 |
| Seed 23, first mapping, assignment | 6 | −0.1 ± 0.86 | +0.3 ± 0.99 | 0.0 ± 0.87 | +0.1 ± 1.08 | −0.1 ± 0.86 | −0.1 ± 0.94 |
| Seed 23, first mapping, assignment | 12 | −0.5 ± 0.77 | −0.4 ± 1.03 | −0.2 ± 0.82 | −1.1 ± 1.05 | −0.1 ± 0.74 | −0.8 ± 1.08 |
| Seed 23, first mapping, assignment | 18 | −0.3 ± 1.06 | −0.1 ± 0.78 | +0.4 ± 1.09 | −0.9 ± 0.63 | −0.2 ± 1.04 | −0.4 ± 0.83 |
| Seed 23, first mapping, assignment | 24 | −0.2 ± 0.87 | −1.5 ± 0.61 | +0.4 ± 0.94 | −1.5 ± 0.65 | −0.1 ± 0.83 | −1.6 ± 0.82 |
| Seed 23, first mapping, assignment | 36 | −0.2 ± 0.96 | −0.4 ± 0.86 | −0.3 ± 1.08 | −0.3 ± 0.51 | 0.0 ± 1.11 | −0.4 ± 0.78 |
| Seed 24, eighth, mirrored | 6 | −1.7 ± 0.97 | −0.1 ± 0.97 | +0.3 ± 1.01 | −0.6 ± 0.98 | −1.1 ± 1.09 | −0.1 ± 0.92 |
| Seed 24, eighth, mirrored | 12 | −4.1 ± 1.28 | −0.8 ± 1.08 | −1.8 ± 0.92 | −0.7 ± 1.06 | −3.2 ± 0.88 | −0.9 ± 1.21 |
| Seed 24, eighth, mirrored | 18 | −5.5 ± 0.96 | +0.3 ± 1.48 | −3.3 ± 0.87 | −1.2 ± 1.15 | −5.4 ± 1.07 | −0.5 ± 1.25 |
| Seed 24, eighth, mirrored | 24 | −7.1 ± 1.66 | −0.7 ± 1.67 | −4.1 ± 1.42 | −0.1 ± 0.88 | −6.1 ± 1.62 | +0.2 ± 1.66 |
| Seed 24, eighth, mirrored | 36 | −3.9 ± 1.27 | −0.2 ± 1.25 | −3.1 ± 1.59 | +0.4 ± 0.79 | −5.3 ± 0.99 | +1.7 ± 1.29 |
| Seed 24, first mapping, assignment | 6 | −0.2 ± 1.01 | −0.2 ± 0.95 | +0.1 ± 0.95 | −0.1 ± 0.89 | −0.1 ± 0.99 | −0.1 ± 0.92 |
| Seed 24, first mapping, assignment | 12 | −1.1 ± 0.95 | 0.0 ± 1.04 | −0.9 ± 0.89 | −1.3 ± 1.04 | −1.4 ± 0.92 | −0.8 ± 1.07 |
| Seed 24, first mapping, assignment | 18 | −0.5 ± 0.78 | +0.5 ± 1.16 | −0.3 ± 0.73 | −0.3 ± 1.03 | −0.7 ± 0.79 | −0.3 ± 1.28 |
| Seed 24, first mapping, assignment | 24 | −1.3 ± 0.83 | +0.7 ± 1.40 | −0.7 ± 0.86 | +0.1 ± 1.05 | −1.0 ± 0.92 | +0.5 ± 1.48 |
| Seed 24, first mapping, assignment | 36 | −1.4 ± 1.20 | −0.3 ± 0.99 | −1.4 ± 1.10 | +1.8 ± 1.32 | −2.4 ± 1.56 | +1.1 ± 1.00 |
| Seed 23, eighth, assignment | 6 | 0.0 ± 0.81 | −0.2 ± 1.23 | −0.4 ± 0.81 | +0.5 ± 1.21 | −0.5 ± 0.82 | +0.3 ± 1.20 |
| Seed 23, eighth, assignment | 12 | −0.9 ± 1.47 | −2.2 ± 1.22 | −1.1 ± 1.29 | −0.3 ± 1.29 | −1.7 ± 1.25 | −2.1 ± 1.13 |
| Seed 23, eighth, assignment | 18 | −1.4 ± 1.19 | −3.9 ± 1.32 | −2.5 ± 1.04 | +0.3 ± 1.02 | −1.0 ± 1.01 | −0.4 ± 0.87 |
| Seed 23, eighth, assignment | 24 | −1.3 ± 1.28 | −5.9 ± 1.17 | −3.9 ± 0.99 | −0.6 ± 1.08 | −3.1 ± 1.02 | −1.7 ± 0.97 |
| Seed 23, eighth, assignment | 36 | −1.5 ± 1.20 | −4.2 ± 1.51 | −3.3 ± 1.10 | −0.2 ± 1.04 | −2.8 ± 0.91 | −0.9 ± 0.95 |
| Seed 24, first mapping, mirrored | 6 | −0.1 ± 0.98 | 0.0 ± 0.91 | +0.2 ± 0.98 | 0.0 ± 0.95 | +0.4 ± 0.99 | −0.1 ± 0.99 |
| Seed 24, first mapping, mirrored | 12 | +0.3 ± 1.03 | +0.3 ± 0.93 | −0.2 ± 0.86 | −0.5 ± 0.93 | +0.4 ± 0.91 | −0.4 ± 0.90 |
| Seed 24, first mapping, mirrored | 18 | +0.3 ± 1.17 | +0.4 ± 0.85 | 0.0 ± 1.13 | −0.6 ± 0.95 | +0.6 ± 1.14 | −0.2 ± 0.89 |
| Seed 24, first mapping, mirrored | 24 | −0.2 ± 0.91 | 0.0 ± 1.25 | −0.5 ± 0.99 | −0.6 ± 1.07 | +0.2 ± 1.02 | −0.9 ± 1.11 |
| Seed 24, first mapping, mirrored | 36 | −0.4 ± 0.55 | −0.3 ± 0.90 | −0.3 ± 0.67 | −0.7 ± 1.03 | +0.8 ± 0.57 | −1.4 ± 1.00 |
| Seed 23, first mapping, mirrored | 6 | +0.1 ± 0.99 | −0.2 ± 0.85 | 0.0 ± 0.98 | +0.3 ± 0.94 | +0.1 ± 0.98 | +0.1 ± 0.87 |
| Seed 23, first mapping, mirrored | 12 | −0.1 ± 0.90 | −1.2 ± 1.48 | 0.0 ± 0.99 | +0.8 ± 1.22 | −0.2 ± 0.92 | +0.6 ± 1.12 |
| Seed 23, first mapping, mirrored | 18 | −0.1 ± 1.07 | −2.2 ± 1.10 | −0.8 ± 1.19 | +1.5 ± 1.10 | −0.7 ± 1.17 | −1.0 ± 1.05 |
| Seed 23, first mapping, mirrored | 24 | −1.0 ± 1.20 | −1.9 ± 1.16 | −1.5 ± 1.58 | +1.4 ± 0.98 | −0.7 ± 1.40 | −0.7 ± 1.02 |
| Seed 23, first mapping, mirrored | 36 | −0.1 ± 1.10 | −1.8 ± 0.86 | −1.3 ± 1.23 | +0.9 ± 0.79 | −0.9 ± 1.01 | −1.1 ± 1.01 |
| Seed 24, eighth, assignment | 6 | −0.4 ± 0.80 | −0.4 ± 0.86 | −0.2 ± 0.77 | +0.6 ± 0.92 | +0.2 ± 0.82 | +0.2 ± 0.98 |
| Seed 24, eighth, assignment | 12 | −1.5 ± 1.00 | +0.4 ± 1.11 | −1.1 ± 0.96 | +0.7 ± 1.17 | −1.3 ± 0.95 | +1.9 ± 0.91 |
| Seed 24, eighth, assignment | 18 | −2.6 ± 0.98 | +0.9 ± 1.35 | −1.9 ± 1.03 | +0.4 ± 0.88 | −0.2 ± 1.05 | +0.4 ± 1.09 |
| Seed 24, eighth, assignment | 24 | −1.5 ± 1.24 | −1.2 ± 1.21 | −2.3 ± 1.07 | −0.4 ± 1.00 | +1.0 ± 0.99 | −1.7 ± 1.07 |
| Seed 24, eighth, assignment | 36 | −2.9 ± 0.68 | −0.5 ± 1.16 | −2.6 ± 0.80 | −0.1 ± 1.00 | −1.1 ± 0.89 | +0.6 ± 1.21 |
| Seed 22, first mapping, mirrored | 6 | 0.0 ± 0.86 | −0.3 ± 0.70 | 0.0 ± 0.84 | −0.1 ± 0.71 | −0.1 ± 0.84 | −0.8 ± 0.74 |
| Seed 22, first mapping, mirrored | 12 | +0.5 ± 1.05 | −0.9 ± 1.09 | −0.1 ± 1.06 | +0.2 ± 1.16 | +0.9 ± 1.19 | 0.0 ± 1.27 |
| Seed 22, first mapping, mirrored | 18 | −0.1 ± 0.98 | −0.6 ± 1.03 | −0.3 ± 1.10 | +0.4 ± 1.14 | +0.6 ± 1.14 | +0.3 ± 1.11 |
| Seed 22, first mapping, mirrored | 24 | −0.1 ± 0.71 | −0.4 ± 1.64 | −0.8 ± 0.79 | −0.2 ± 1.42 | +0.2 ± 0.66 | +0.3 ± 1.56 |
| Seed 22, first mapping, mirrored | 36 | 0.0 ± 1.01 | −0.2 ± 1.04 | −0.8 ± 0.91 | 0.0 ± 1.14 | +0.4 ± 0.92 | 0.0 ± 1.27 |
| Seed 22, eighth, assignment | 6 | −0.2 ± 0.71 | −0.3 ± 1.28 | −0.4 ± 0.76 | +0.1 ± 1.24 | −0.3 ± 0.89 | +0.3 ± 1.26 |
| Seed 22, eighth, assignment | 12 | −0.5 ± 0.72 | −1.3 ± 1.06 | −1.0 ± 0.63 | −0.3 ± 1.03 | −0.5 ± 0.73 | −1.0 ± 1.05 |
| Seed 22, eighth, assignment | 18 | −1.1 ± 1.29 | −0.4 ± 0.74 | −0.5 ± 0.72 | +0.3 ± 0.69 | +0.6 ± 0.90 | −0.3 ± 0.85 |
| Seed 22, eighth, assignment | 24 | −0.5 ± 1.30 | −2.3 ± 0.99 | −1.1 ± 1.14 | −0.6 ± 0.97 | +0.4 ± 1.24 | −1.4 ± 0.97 |
| Seed 22, eighth, assignment | 36 | −0.5 ± 1.52 | −2.5 ± 1.52 | +0.1 ± 1.15 | −1.4 ± 1.24 | +0.6 ± 1.49 | −0.9 ± 1.26 |

| Cell | Block | Own group less the excitatory night: A's | B's | Inhibitory units: A's | B's | Outside the readouts: A's | B's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | −0.4 ± 0.91 | +1.1 ± 1.00 | +0.1 ± 0.90 | +0.4 ± 0.97 | −0.5 ± 0.92 | +1.4 ± 0.98 |
| Seed 22, eighth, mirrored | 12 | −2.2 ± 1.12 | +2.2 ± 1.18 | −0.5 ± 1.04 | +0.2 ± 0.92 | −1.1 ± 0.92 | +2.8 ± 1.01 |
| Seed 22, eighth, mirrored | 18 | −2.2 ± 1.28 | +1.5 ± 1.26 | −0.4 ± 1.16 | 0.0 ± 1.12 | −1.0 ± 0.94 | +1.9 ± 1.20 |
| Seed 22, eighth, mirrored | 24 | −3.0 ± 1.39 | +3.4 ± 1.68 | −1.0 ± 0.54 | −0.7 ± 1.19 | −0.2 ± 0.76 | +5.2 ± 1.42 |
| Seed 22, eighth, mirrored | 36 | −2.9 ± 0.81 | +3.4 ± 1.58 | −0.9 ± 0.68 | +0.7 ± 1.08 | −0.9 ± 0.70 | +5.4 ± 1.23 |
| Seed 22, first mapping, assignment | 6 | −0.1 ± 0.80 | +0.1 ± 0.74 | −0.1 ± 0.76 | −0.3 ± 0.61 | −0.1 ± 0.78 | −0.1 ± 0.67 |
| Seed 22, first mapping, assignment | 12 | −0.3 ± 0.89 | +2.3 ± 1.32 | −0.3 ± 0.90 | +1.5 ± 1.18 | −0.8 ± 0.87 | +3.3 ± 1.33 |
| Seed 22, first mapping, assignment | 18 | +0.8 ± 1.24 | +1.1 ± 1.40 | +0.3 ± 1.27 | +0.3 ± 0.87 | +0.2 ± 1.06 | +1.7 ± 1.03 |
| Seed 22, first mapping, assignment | 24 | +1.4 ± 0.73 | +2.8 ± 0.88 | +0.5 ± 0.76 | −0.3 ± 1.15 | +0.5 ± 0.79 | +2.7 ± 1.25 |
| Seed 22, first mapping, assignment | 36 | +1.7 ± 1.01 | +3.3 ± 0.78 | +1.0 ± 0.75 | −0.1 ± 0.74 | +0.3 ± 0.85 | +2.8 ± 0.77 |
| Seed 23, eighth, mirrored | 6 | +0.6 ± 1.16 | −0.9 ± 0.89 | +0.2 ± 1.20 | −0.9 ± 0.80 | +0.4 ± 1.23 | −1.4 ± 0.91 |
| Seed 23, eighth, mirrored | 12 | +2.7 ± 0.92 | −0.7 ± 1.03 | +1.4 ± 0.76 | −0.8 ± 0.79 | +3.1 ± 0.98 | −0.9 ± 0.72 |
| Seed 23, eighth, mirrored | 18 | +3.3 ± 0.97 | −0.9 ± 0.55 | +2.3 ± 0.99 | −1.6 ± 0.76 | +3.9 ± 0.86 | −0.9 ± 0.55 |
| Seed 23, eighth, mirrored | 24 | +2.8 ± 1.12 | 0.0 ± 0.76 | +2.2 ± 1.01 | −1.6 ± 0.44 | +3.1 ± 1.37 | +0.2 ± 0.38 |
| Seed 23, eighth, mirrored | 36 | +1.3 ± 1.34 | +1.0 ± 0.56 | +1.0 ± 1.30 | −0.4 ± 0.24 | +1.8 ± 1.25 | +0.4 ± 0.43 |
| Seed 23, first mapping, assignment | 6 | +0.1 ± 0.87 | +0.3 ± 0.97 | +0.2 ± 0.89 | +0.1 ± 1.05 | +0.1 ± 0.88 | −0.1 ± 0.92 |
| Seed 23, first mapping, assignment | 12 | +0.5 ± 0.93 | +0.2 ± 0.94 | +0.7 ± 0.99 | −0.6 ± 0.97 | +0.8 ± 0.92 | −0.3 ± 0.90 |
| Seed 23, first mapping, assignment | 18 | +0.4 ± 1.16 | −1.1 ± 0.71 | +1.1 ± 1.21 | −1.8 ± 0.75 | +0.5 ± 1.12 | −1.3 ± 0.87 |
| Seed 23, first mapping, assignment | 24 | +0.5 ± 0.88 | −1.5 ± 0.82 | +1.1 ± 0.93 | −1.5 ± 0.67 | +0.6 ± 0.87 | −1.6 ± 0.96 |
| Seed 23, first mapping, assignment | 36 | +0.8 ± 1.05 | −0.7 ± 0.63 | +0.8 ± 0.99 | −0.7 ± 0.48 | +1.1 ± 1.04 | −0.8 ± 0.59 |
| Seed 24, eighth, mirrored | 6 | −2.4 ± 1.02 | −0.9 ± 1.01 | −0.4 ± 0.98 | −1.4 ± 1.07 | −1.8 ± 1.11 | −0.9 ± 0.94 |
| Seed 24, eighth, mirrored | 12 | −4.0 ± 1.23 | −0.7 ± 1.31 | −1.7 ± 0.89 | −0.6 ± 1.42 | −3.1 ± 0.87 | −0.9 ± 1.49 |
| Seed 24, eighth, mirrored | 18 | −4.5 ± 1.46 | +0.2 ± 1.52 | −2.4 ± 1.21 | −1.3 ± 1.36 | −4.5 ± 1.07 | −0.6 ± 1.44 |
| Seed 24, eighth, mirrored | 24 | −7.2 ± 1.82 | −1.6 ± 1.41 | −4.2 ± 1.71 | −1.0 ± 1.24 | −6.2 ± 1.59 | −0.8 ± 1.65 |
| Seed 24, eighth, mirrored | 36 | −4.5 ± 1.35 | −0.7 ± 1.25 | −3.7 ± 1.62 | −0.1 ± 0.76 | −5.9 ± 1.01 | +1.1 ± 1.32 |
| Seed 24, first mapping, assignment | 6 | +0.1 ± 1.03 | 0.0 ± 0.96 | +0.4 ± 0.98 | +0.1 ± 0.92 | +0.2 ± 1.01 | +0.1 ± 0.93 |
| Seed 24, first mapping, assignment | 12 | +0.1 ± 0.98 | 0.0 ± 1.12 | +0.4 ± 0.95 | −1.2 ± 1.12 | −0.1 ± 0.95 | −0.8 ± 1.18 |
| Seed 24, first mapping, assignment | 18 | +0.4 ± 0.83 | −0.5 ± 0.95 | +0.7 ± 0.79 | −1.3 ± 1.05 | +0.3 ± 0.88 | −1.3 ± 1.27 |
| Seed 24, first mapping, assignment | 24 | +0.1 ± 0.74 | +0.4 ± 1.03 | +0.6 ± 0.86 | −0.3 ± 1.20 | +0.4 ± 0.83 | +0.1 ± 0.81 |
| Seed 24, first mapping, assignment | 36 | +0.1 ± 1.21 | −1.4 ± 0.80 | +0.1 ± 1.25 | +0.7 ± 0.88 | −0.9 ± 1.18 | 0.0 ± 0.76 |
| Seed 23, eighth, assignment | 6 | +0.1 ± 0.86 | −0.7 ± 1.23 | −0.2 ± 0.82 | 0.0 ± 1.19 | −0.3 ± 0.86 | −0.2 ± 1.18 |
| Seed 23, eighth, assignment | 12 | −1.0 ± 1.46 | −2.3 ± 1.31 | −1.3 ± 1.31 | −0.4 ± 1.51 | −1.8 ± 1.23 | −2.2 ± 1.26 |
| Seed 23, eighth, assignment | 18 | −1.8 ± 1.15 | −3.6 ± 1.50 | −2.8 ± 0.99 | +0.6 ± 1.43 | −1.4 ± 0.95 | −0.1 ± 1.22 |
| Seed 23, eighth, assignment | 24 | −1.3 ± 1.18 | −4.9 ± 1.31 | −3.9 ± 1.04 | +0.4 ± 1.08 | −3.0 ± 1.13 | −0.7 ± 1.17 |
| Seed 23, eighth, assignment | 36 | −2.3 ± 1.25 | −4.2 ± 1.69 | −4.0 ± 1.03 | −0.2 ± 1.25 | −3.6 ± 0.97 | −0.9 ± 1.06 |
| Seed 24, first mapping, mirrored | 6 | −0.6 ± 0.98 | −0.3 ± 0.91 | −0.3 ± 0.99 | −0.3 ± 0.94 | −0.1 ± 0.99 | −0.4 ± 0.97 |
| Seed 24, first mapping, mirrored | 12 | +0.1 ± 1.04 | −1.1 ± 1.18 | −0.4 ± 0.89 | −1.9 ± 0.94 | +0.2 ± 0.96 | −1.8 ± 1.11 |
| Seed 24, first mapping, mirrored | 18 | 0.0 ± 1.10 | −1.0 ± 0.92 | −0.3 ± 1.10 | −2.0 ± 0.84 | +0.3 ± 1.13 | −1.6 ± 0.91 |
| Seed 24, first mapping, mirrored | 24 | −0.3 ± 0.92 | −0.3 ± 1.32 | −0.6 ± 1.08 | −0.9 ± 0.95 | +0.1 ± 1.00 | −1.2 ± 1.23 |
| Seed 24, first mapping, mirrored | 36 | −0.5 ± 0.70 | +0.2 ± 0.76 | −0.4 ± 0.83 | −0.3 ± 0.89 | +0.6 ± 0.57 | −1.0 ± 0.86 |
| Seed 23, first mapping, mirrored | 6 | 0.0 ± 0.95 | −0.1 ± 0.91 | −0.1 ± 0.95 | +0.4 ± 1.03 | 0.0 ± 0.95 | +0.3 ± 0.92 |
| Seed 23, first mapping, mirrored | 12 | −0.6 ± 0.95 | −1.4 ± 1.35 | −0.5 ± 1.03 | +0.6 ± 1.28 | −0.7 ± 0.97 | +0.4 ± 1.17 |
| Seed 23, first mapping, mirrored | 18 | −0.2 ± 0.88 | −2.5 ± 1.15 | −0.9 ± 0.98 | +1.1 ± 1.07 | −0.8 ± 0.97 | −1.3 ± 1.04 |
| Seed 23, first mapping, mirrored | 24 | −1.4 ± 0.90 | −2.1 ± 1.11 | −1.9 ± 1.05 | +1.1 ± 1.07 | −1.1 ± 0.97 | −1.0 ± 1.15 |
| Seed 23, first mapping, mirrored | 36 | −0.7 ± 1.11 | −2.1 ± 0.89 | −1.9 ± 1.00 | +0.5 ± 0.90 | −1.5 ± 1.02 | −1.5 ± 1.23 |
| Seed 24, eighth, assignment | 6 | −0.5 ± 0.86 | −1.1 ± 0.94 | −0.3 ± 0.89 | −0.1 ± 1.01 | +0.1 ± 0.92 | −0.5 ± 1.05 |
| Seed 24, eighth, assignment | 12 | −1.7 ± 1.24 | −1.7 ± 1.15 | −1.3 ± 1.11 | −1.4 ± 0.99 | −1.5 ± 1.16 | −0.3 ± 0.98 |
| Seed 24, eighth, assignment | 18 | −4.5 ± 0.76 | −1.2 ± 0.82 | −3.8 ± 0.81 | −1.8 ± 1.03 | −2.1 ± 1.32 | −1.8 ± 0.83 |
| Seed 24, eighth, assignment | 24 | −3.0 ± 0.75 | −4.0 ± 0.99 | −3.8 ± 0.88 | −3.2 ± 1.26 | −0.5 ± 1.08 | −4.4 ± 1.23 |
| Seed 24, eighth, assignment | 36 | −4.3 ± 0.96 | −3.8 ± 0.83 | −4.1 ± 0.67 | −3.3 ± 1.07 | −2.6 ± 1.27 | −2.7 ± 1.03 |
| Seed 22, first mapping, mirrored | 6 | −0.1 ± 0.88 | 0.0 ± 0.75 | −0.1 ± 0.86 | +0.1 ± 0.76 | −0.2 ± 0.85 | −0.5 ± 0.77 |
| Seed 22, first mapping, mirrored | 12 | +0.6 ± 1.10 | −0.7 ± 1.25 | +0.1 ± 1.17 | +0.3 ± 1.18 | +1.0 ± 1.18 | +0.1 ± 1.34 |
| Seed 22, first mapping, mirrored | 18 | +0.2 ± 0.91 | 0.0 ± 1.09 | 0.0 ± 0.98 | +1.0 ± 1.07 | +0.9 ± 1.16 | +0.8 ± 1.10 |
| Seed 22, first mapping, mirrored | 24 | +0.6 ± 0.71 | 0.0 ± 1.76 | −0.2 ± 0.75 | +0.3 ± 1.46 | +0.8 ± 0.68 | +0.8 ± 1.44 |
| Seed 22, first mapping, mirrored | 36 | +0.8 ± 1.09 | −0.1 ± 1.01 | +0.1 ± 0.97 | +0.1 ± 0.92 | +1.3 ± 1.00 | +0.1 ± 0.95 |
| Seed 22, eighth, assignment | 6 | −0.1 ± 0.73 | −0.3 ± 1.29 | −0.3 ± 0.76 | +0.1 ± 1.26 | −0.2 ± 0.85 | +0.3 ± 1.28 |
| Seed 22, eighth, assignment | 12 | −0.7 ± 0.73 | −1.0 ± 1.07 | −1.2 ± 0.67 | 0.0 ± 1.06 | −0.7 ± 0.78 | −0.7 ± 1.06 |
| Seed 22, eighth, assignment | 18 | −1.3 ± 1.36 | −0.3 ± 0.73 | −0.7 ± 1.00 | +0.4 ± 0.70 | +0.5 ± 0.96 | −0.1 ± 0.80 |
| Seed 22, eighth, assignment | 24 | −0.4 ± 1.27 | −2.0 ± 0.97 | −0.9 ± 1.32 | −0.4 ± 0.97 | +0.6 ± 1.13 | −1.2 ± 1.02 |
| Seed 22, eighth, assignment | 36 | −0.9 ± 1.31 | −2.3 ± 1.35 | −0.2 ± 1.12 | −1.2 ± 1.07 | +0.3 ± 1.29 | −0.7 ± 1.16 |

- **Less H-47's plain night.** Under the night held onto the own group A's trials lie between −1.9 and +4.4, beyond 2 SE at 5 of the 60
  checkpoints, all above: on seed 24's eighth image from the mirrored assignment and seed 24's first mapping from the assignment, each at
  block 36, +1.9 ± 0.74 and +1.8 ± 0.84, and on seed 23's eighth image from the assignment from block 18 on, +1.8 ± 0.88 to +4.4 ± 1.03.
  B's lie between −3.2 and +2.1, beyond 2 SE at 2, both below, on seed 23's eighth image from the assignment at blocks 18 and 24,
  −2.3 ± 1.05 and −3.2 ± 1.54. Under the night held onto the inhibitory units A's lie between −2.6 and +3.6, beyond 2 SE at 8: below on
  seed 23's eighth image from the mirrored assignment at blocks 12, 18 and 24, −2.0 ± 0.88 to −2.6 ± 0.86; above on seed 24's eighth image
  from the mirrored assignment from block 12 on, +2.5 ± 0.99 to +3.6 ± 0.77, and on seed 23's eighth image from the assignment at block 36,
  +1.7 ± 0.83. B's lie between −4.9 and +2.1, beyond 2 SE at 7: below on seed 22's eighth image from the mirrored assignment at blocks 12,
  24 and 36, −2.1 ± 0.92 to −4.9 ± 1.28, on seed 22's first mapping from the assignment at blocks 24 and 36, −2.2 ± 1.00 and −2.6 ± 0.90,
  and on seed 23's eighth image from the mirrored assignment at block 24, −1.2 ± 0.57; above on seed 23's first mapping from the mirrored
  assignment at block 18, +2.0 ± 0.90. Under the night held onto the units outside the readouts A's lie between −1.3 and +2.6, beyond 2 SE
  at 5, all above: on seed 24's eighth image from the mirrored assignment at block 18, +1.5 ± 0.70; on seed 23's eighth image from the
  assignment from block 18 on, +2.1 ± 0.80 to +2.6 ± 1.08; and on seed 24's eighth image from the assignment at block 36, +1.8 ± 0.65. B's
  lie between −0.8 and +1.6, within 2 SE at every one of the 60.
- **Less exploration 027's night held within.** Under the night held onto the own group A's trials lie between −7.1 and +2.9, beyond 2 SE
  at 9 of the 60: above on seed 23's eighth image from the mirrored assignment at blocks 18 and 24; below on seed 22's eighth image from the
  mirrored assignment at block 24, on seed 24's eighth image from the mirrored assignment from block 12 on, −3.9 ± 1.27 to −7.1 ± 1.66, and
  on seed 24's eighth image from the assignment at blocks 18 and 36. B's lie between −5.9 and +2.3, beyond 2 SE at 7: above on seed 22's
  first mapping from the assignment at block 36; below on seed 23's first mapping from the assignment at block 24, on seed 23's eighth image
  from the assignment from block 18 on, −3.9 ± 1.32 to −5.9 ± 1.17, on seed 23's first mapping from the mirrored assignment at block 36 and
  on seed 22's eighth image from the assignment at block 24. Under the night held onto the inhibitory units A's lie between −4.1 and +1.8,
  beyond 2 SE at 7, all below: on seed 24's eighth image from the mirrored assignment at blocks 18 and 24, on seed 23's eighth image from
  the assignment from block 18 on and on seed 24's eighth image from the assignment at blocks 24 and 36. B's lie between −2.1 and +1.8,
  beyond 2 SE at 3, all below: on seed 23's eighth image from the mirrored assignment at blocks 24 and 36 and on seed 23's first mapping
  from the assignment at block 24. Under the night held onto the units outside the readouts A's lie between −6.1 and +3.4, beyond 2 SE at
  8: above on seed 23's eighth image from the mirrored assignment at blocks 18 and 24; below on seed 24's eighth image from the mirrored
  assignment from block 12 on, −3.2 ± 0.88 to −6.1 ± 1.62, and on seed 23's eighth image from the assignment at blocks 24 and 36. B's lie
  between −2.1 and +3.8, beyond 2 SE at 4, all above: on seed 22's eighth image from the mirrored assignment at blocks 12, 24 and 36 and on
  seed 24's eighth image from the assignment at block 12.
- **Less the excitatory night.** Under the night held onto the own group A's trials lie between −7.2 and +3.3, beyond 2 SE at 13 of the
  60: above on seed 23's eighth image from the mirrored assignment at blocks 12, 18 and 24; below on seed 22's eighth image from the
  mirrored assignment at blocks 24 and 36, on seed 24's eighth image from the mirrored assignment at every checkpoint, −2.4 ± 1.02 to
  −7.2 ± 1.82, and on seed 24's eighth image from the assignment from block 18 on. B's lie between −4.9 and +3.4, beyond 2 SE at 12: above
  on seed 22's eighth image from the mirrored assignment and seed 22's first mapping from the assignment, each at blocks 24 and 36; below
  on seed 23's eighth image from the assignment from block 18 on, on seed 23's first mapping from the mirrored assignment at blocks 18 and
  36, on seed 24's eighth image from the assignment at blocks 24 and 36 and on seed 22's eighth image from the assignment at block 24. Under
  the night held onto the inhibitory units A's lie between −4.2 and +2.3, beyond 2 SE at 10: above on seed 23's eighth image from the
  mirrored assignment at blocks 18 and 24; below on seed 24's eighth image from the mirrored assignment at blocks 24 and 36 and on seed 23's
  and seed 24's eighth images from the assignment from block 18 on. B's lie between −3.3 and +1.5, beyond 2 SE at 7, all below: on seed
  23's eighth image from the mirrored assignment and seed 23's first mapping from the assignment, each at blocks 18 and 24, on seed 24's
  first mapping from the mirrored assignment at block 18 and on seed 24's eighth image from the assignment at blocks 24 and 36. Under the
  night held onto the units outside the readouts A's lie between −6.2 and +3.9, beyond 2 SE at 10: above on seed 23's eighth image from the
  mirrored assignment at blocks 12, 18 and 24; below on seed 24's eighth image from the mirrored assignment from block 12 on, on seed 23's
  eighth image from the assignment at blocks 24 and 36 and on seed 24's eighth image from the assignment at block 36. B's lie between −4.4
  and +5.4, beyond 2 SE at 9: above on seed 22's eighth image from the mirrored assignment and seed 22's first mapping from the assignment,
  each at blocks 12, 24 and 36; below on seed 24's eighth image from the assignment from block 18 on.

## 3. The pair and A's couplings

The pair's count under each held form at each checkpoint, and at block 36 under every form. H-47's floor for clause 2 is 56:

| Cell | Own group at 6 | 12 | 18 | 24 | 36 | Inhibitory units at 6 | 12 | 18 | 24 | 36 | Outside the readouts at 6 | 12 | 18 | 24 | 36 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 45.6 | 63.0 | 62.8 | 62.8 | 63.1 | 44.8 | 62.8 | 62.5 | 62.9 | 63.0 | 45.6 | 62.9 | 62.8 | 62.9 | 63.1 |
| Seed 22, first mapping, assignment | 42.2 | 61.7 | 61.5 | 62.0 | 62.8 | 42.1 | 61.4 | 60.9 | 62.1 | 62.8 | 42.8 | 61.7 | 61.6 | 61.7 | 63.0 |
| Seed 23, eighth, mirrored | 58.0 | 61.6 | 61.9 | 61.7 | 62.4 | 57.7 | 62.2 | 61.5 | 61.3 | 62.7 | 58.1 | 61.8 | 61.8 | 61.6 | 62.8 |
| Seed 23, first mapping, assignment | 27.4 | 55.6 | 61.1 | 60.2 | 61.9 | 27.3 | 56.1 | 61.4 | 60.9 | 61.9 | 27.5 | 55.4 | 61.0 | 60.7 | 61.8 |
| Seed 24, eighth, mirrored | 61.9 | 62.8 | 62.1 | 62.4 | 62.8 | 61.7 | 62.8 | 62.5 | 62.6 | 63.1 | 62.0 | 63.1 | 62.5 | 62.3 | 63.2 |
| Seed 24, first mapping, assignment | 34.5 | 51.1 | 59.4 | 58.1 | 62.0 | 35.0 | 53.6 | 60.2 | 59.4 | 61.9 | 34.5 | 52.6 | 59.8 | 59.2 | 61.7 |
| Seed 23, eighth, assignment | 55.2 | 62.4 | 62.6 | 62.7 | 63.3 | 54.9 | 62.1 | 62.2 | 63.0 | 63.3 | 54.6 | 62.1 | 62.1 | 62.7 | 63.3 |
| Seed 24, first mapping, mirrored | 51.5 | 61.3 | 61.9 | 61.9 | 62.5 | 51.7 | 61.6 | 62.0 | 62.0 | 62.5 | 51.8 | 61.4 | 61.6 | 61.7 | 62.7 |
| Seed 23, first mapping, mirrored | 49.3 | 59.5 | 61.9 | 62.9 | 63.3 | 48.7 | 59.5 | 61.8 | 62.7 | 63.3 | 49.1 | 59.3 | 62.1 | 63.0 | 63.3 |
| Seed 24, eighth, assignment | 61.4 | 62.5 | 62.5 | 62.2 | 63.2 | 61.5 | 62.3 | 62.6 | 62.6 | 63.3 | 61.0 | 62.4 | 62.3 | 62.1 | 62.9 |
| Seed 22, first mapping, mirrored | 54.4 | 61.1 | 62.4 | 61.5 | 62.6 | 54.1 | 61.1 | 61.9 | 61.3 | 62.2 | 54.6 | 60.8 | 61.8 | 61.5 | 62.5 |
| Seed 22, eighth, assignment | 63.1 | 62.6 | 63.2 | 61.8 | 63.0 | 63.1 | 62.6 | 62.9 | 62.4 | 63.1 | 63.0 | 62.5 | 62.9 | 61.7 | 62.8 |

| Cell | At 36: without replay | own group | inhibitory units | outside the readouts | plain on A's (H-47) | held within (027) | excitatory (025, 026) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 63.0 | 63.1 | 63.0 | 63.1 | 62.8 | 62.9 | 62.9 |
| Seed 22, first mapping, assignment | 63.3 | 62.8 | 62.8 | 63.0 | 63.1 | 62.5 | 62.7 |
| Seed 23, eighth, mirrored | 62.6 | 62.4 | 62.7 | 62.8 | 63.1 | 62.6 | 62.8 |
| Seed 23, first mapping, assignment | 63.1 | 61.9 | 61.9 | 61.8 | 61.9 | 61.7 | 61.9 |
| Seed 24, eighth, mirrored | 62.7 | 62.8 | 63.1 | 63.2 | 63.0 | 63.0 | 63.0 |
| Seed 24, first mapping, assignment | 62.1 | 62.0 | 61.9 | 61.7 | 61.8 | 61.3 | 61.7 |
| Seed 23, eighth, assignment | 63.4 | 63.3 | 63.3 | 63.3 | 63.4 | 63.1 | 62.8 |
| Seed 24, first mapping, mirrored | 63.4 | 62.5 | 62.5 | 62.7 | 62.4 | 62.1 | 62.4 |
| Seed 23, first mapping, mirrored | 63.5 | 63.3 | 63.3 | 63.3 | 63.2 | 63.2 | 63.0 |
| Seed 24, eighth, assignment | 62.6 | 63.2 | 63.3 | 62.9 | 62.9 | 62.7 | 62.6 |
| Seed 22, first mapping, mirrored | 63.1 | 62.6 | 62.2 | 62.5 | 62.5 | 62.6 | 62.9 |
| Seed 22, eighth, assignment | 62.6 | 63.0 | 63.1 | 62.8 | 62.9 | 62.8 | 62.9 |

A's highest coupling over H-42's under each form, in any image read and at any block's end, as clause 3 reads it; its bound is 1.30:

| Cell | Without replay | Own group | Inhibitory units | Outside the readouts | Plain on A's (H-47) | Held within (027) | Excitatory (025, 026) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 1.144 | 1.192 | 1.196 | 1.201 | 1.197 | 1.202 | 1.201 |
| Seed 22, first mapping, assignment | 1.112 | 1.204 | 1.204 | 1.205 | 1.198 | 1.205 | 1.202 |
| Seed 23, eighth, mirrored | 1.147 | 1.208 | 1.209 | 1.220 | 1.211 | 1.211 | 1.212 |
| Seed 23, first mapping, assignment | 1.125 | 1.211 | 1.210 | 1.213 | 1.212 | 1.216 | 1.212 |
| Seed 24, eighth, mirrored | 1.078 | 1.141 | 1.138 | 1.150 | 1.141 | 1.145 | 1.146 |
| Seed 24, first mapping, assignment | 1.085 | 1.159 | 1.160 | 1.167 | 1.162 | 1.170 | 1.165 |
| Seed 23, eighth, assignment | 1.118 | 1.155 | 1.164 | 1.165 | 1.171 | 1.167 | 1.162 |
| Seed 24, first mapping, mirrored | 1.177 | 1.223 | 1.228 | 1.237 | 1.223 | 1.236 | 1.236 |
| Seed 23, first mapping, mirrored | 1.082 | 1.154 | 1.154 | 1.159 | 1.158 | 1.153 | 1.151 |
| Seed 24, eighth, assignment | 1.179 | 1.193 | 1.182 | 1.193 | 1.200 | 1.184 | 1.196 |
| Seed 22, first mapping, mirrored | 1.114 | 1.184 | 1.179 | 1.191 | 1.182 | 1.192 | 1.183 |
| Seed 22, eighth, assignment | 1.133 | 1.184 | 1.188 | 1.199 | 1.184 | 1.197 | 1.193 |

At block 36 the pair stands at 61.9 to 63.3 under the night held onto the own group, 61.9 to 63.3 under the night held onto the
inhibitory units and 61.7 to 63.3 under the night held onto the units outside the readouts, where H-47's plain night gave 61.8 to 63.4,
exploration 027's night held within 61.3 to 63.2, the excitatory night 61.7 to 63.0 and the night without replay 62.1 to 63.5. A's highest
coupling is 1.141 to 1.223 of H-42's under the night held onto the own group, 1.138 to 1.228 under the night held onto the inhibitory units
and 1.150 to 1.237 under the night held onto the units outside the readouts, where H-47's plain night gave 1.141 to 1.223, exploration
027's night held within 1.145 to 1.236, the excitatory night 1.146 to 1.236 and the night without replay 1.078 to 1.179. The H-42
couplings each job reads against are the same in this job, in H-47's, in exploration 027's and in the excitatory night's, in every cell.

## 4. What each night moved

What the four nights moved, summed over the four, the mean over the three paths, from the class matrix in the images before and after each
night, in M. A group's held units are its readout's two classes, the pair's own kind left out; "onto their own group" counts the pair's own
kind among the targets. Onto their own group is the first part, onto the inhibitory units the second, and onto A's and B's units and onto
the other excitatory units together the third; the column marked held across is exploration 027's part held across, which no form here
holds. H-47's plain night is exploration 021's run of exploration 020's job for exploration 026's eight cells and exploration 020's runs for
exploration 020's four; exploration 027's night held within is exploration 027's job; and the excitatory night's rows read 0.00, since it
moved none of these:

| Cell | Form | The odd places' held units onto their own group | onto the even places' excitatory (held across) | onto the inhibitory units | onto A's and B's units | onto the other excitatory | The even places' held units onto their own group | onto the odd places' excitatory (held across) | onto the inhibitory units | onto A's and B's units | onto the other excitatory |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | Without replay | −0.50 | −0.47 | +1.28 | −0.02 | +0.05 | −0.74 | −0.65 | +1.02 | −0.18 | −0.03 |
| Seed 22, eighth, mirrored | Own group | 0.00 | −5.91 | −2.22 | −0.46 | 0.00 | 0.00 | −6.62 | −1.95 | −0.75 | −0.01 |
| Seed 22, eighth, mirrored | Inhibitory units | −0.69 | −5.99 | 0.00 | −0.46 | +0.01 | −0.89 | −6.71 | 0.00 | −0.73 | −0.01 |
| Seed 22, eighth, mirrored | Outside the readouts | −0.68 | −5.95 | −2.23 | 0.00 | 0.00 | −0.88 | −6.78 | −2.02 | 0.00 | 0.00 |
| Seed 22, eighth, mirrored | Plain on A's (H-47) | −0.66 | −5.98 | −2.19 | −0.40 | 0.00 | −0.92 | −6.72 | −1.96 | −0.74 | −0.01 |
| Seed 22, eighth, mirrored | Held within (027) | 0.00 | −5.94 | 0.00 | 0.00 | 0.00 | 0.00 | −6.62 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, mirrored | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, assignment | Without replay | −0.45 | −0.52 | +1.49 | −0.04 | +0.04 | −0.75 | −0.61 | +1.41 | −0.24 | −0.03 |
| Seed 22, first mapping, assignment | Own group | 0.00 | −5.57 | −2.02 | −0.25 | +0.01 | 0.00 | −6.25 | −1.56 | −0.75 | −0.01 |
| Seed 22, first mapping, assignment | Inhibitory units | −0.67 | −5.68 | 0.00 | −0.27 | +0.02 | −0.77 | −6.35 | 0.00 | −0.71 | −0.01 |
| Seed 22, first mapping, assignment | Outside the readouts | −0.70 | −5.69 | −1.97 | 0.00 | 0.00 | −0.76 | −6.32 | −1.58 | 0.00 | 0.00 |
| Seed 22, first mapping, assignment | Plain on A's (H-47) | −0.73 | −5.66 | −2.02 | −0.25 | +0.01 | −0.76 | −6.32 | −1.56 | −0.73 | −0.01 |
| Seed 22, first mapping, assignment | Held within (027) | 0.00 | −5.60 | 0.00 | 0.00 | 0.00 | 0.00 | −6.27 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, assignment | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, mirrored | Without replay | +0.23 | −0.01 | +1.35 | −0.04 | −0.03 | −0.20 | −0.59 | +1.34 | −0.19 | 0.00 |
| Seed 23, eighth, mirrored | Own group | 0.00 | −6.81 | −2.70 | −0.60 | +0.01 | 0.00 | −6.72 | −2.12 | −0.56 | −0.04 |
| Seed 23, eighth, mirrored | Inhibitory units | −0.60 | −6.86 | 0.00 | −0.55 | +0.01 | −1.18 | −6.52 | 0.00 | −0.47 | −0.03 |
| Seed 23, eighth, mirrored | Outside the readouts | −0.59 | −6.96 | −2.65 | 0.00 | 0.00 | −1.26 | −6.64 | −2.10 | 0.00 | 0.00 |
| Seed 23, eighth, mirrored | Plain on A's (H-47) | −0.56 | −6.98 | −2.71 | −0.59 | +0.01 | −1.17 | −6.55 | −2.07 | −0.47 | −0.03 |
| Seed 23, eighth, mirrored | Held within (027) | 0.00 | −6.75 | 0.00 | 0.00 | 0.00 | 0.00 | −6.67 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, mirrored | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, assignment | Without replay | +0.12 | −0.17 | +1.53 | +0.04 | −0.03 | −0.30 | −0.38 | +1.53 | −0.27 | 0.00 |
| Seed 23, first mapping, assignment | Own group | 0.00 | −6.26 | −2.39 | −0.25 | 0.00 | 0.00 | −5.83 | −1.59 | −0.42 | −0.06 |
| Seed 23, first mapping, assignment | Inhibitory units | 0.00 | −6.26 | 0.00 | −0.18 | 0.00 | −0.80 | −5.71 | 0.00 | −0.32 | −0.06 |
| Seed 23, first mapping, assignment | Outside the readouts | 0.00 | −6.23 | −2.31 | 0.00 | 0.00 | −0.78 | −5.80 | −1.60 | 0.00 | 0.00 |
| Seed 23, first mapping, assignment | Plain on A's (H-47) | −0.02 | −6.24 | −2.31 | −0.18 | 0.00 | −0.80 | −5.79 | −1.55 | −0.35 | −0.06 |
| Seed 23, first mapping, assignment | Held within (027) | 0.00 | −6.25 | 0.00 | 0.00 | 0.00 | 0.00 | −5.71 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, assignment | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, mirrored | Without replay | −0.39 | +0.07 | +1.08 | −0.07 | +0.07 | −0.78 | −0.31 | +0.88 | −0.11 | +0.10 |
| Seed 24, eighth, mirrored | Own group | 0.00 | −5.42 | −2.37 | −0.25 | −0.04 | 0.00 | −6.90 | −1.92 | −1.00 | +0.05 |
| Seed 24, eighth, mirrored | Inhibitory units | −0.90 | −5.40 | 0.00 | −0.19 | −0.03 | −1.41 | −6.86 | 0.00 | −0.99 | +0.06 |
| Seed 24, eighth, mirrored | Outside the readouts | −0.90 | −5.40 | −2.42 | 0.00 | 0.00 | −1.40 | −6.99 | −1.78 | 0.00 | 0.00 |
| Seed 24, eighth, mirrored | Plain on A's (H-47) | −0.90 | −5.39 | −2.43 | −0.25 | −0.05 | −1.39 | −6.95 | −1.82 | −0.97 | +0.06 |
| Seed 24, eighth, mirrored | Held within (027) | 0.00 | −5.40 | 0.00 | 0.00 | 0.00 | 0.00 | −6.84 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, mirrored | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, assignment | Without replay | −0.37 | +0.14 | +1.23 | +0.19 | +0.14 | −0.46 | −0.03 | +1.42 | −0.07 | +0.10 |
| Seed 24, first mapping, assignment | Own group | 0.00 | −5.17 | −2.01 | +0.07 | −0.02 | 0.00 | −5.95 | −1.49 | −0.88 | +0.04 |
| Seed 24, first mapping, assignment | Inhibitory units | −0.60 | −5.13 | 0.00 | +0.09 | −0.02 | −1.17 | −5.98 | 0.00 | −0.85 | +0.05 |
| Seed 24, first mapping, assignment | Outside the readouts | −0.61 | −5.14 | −1.98 | 0.00 | 0.00 | −1.13 | −6.01 | −1.44 | 0.00 | 0.00 |
| Seed 24, first mapping, assignment | Plain on A's (H-47) | −0.62 | −5.10 | −1.98 | +0.12 | −0.03 | −1.17 | −6.04 | −1.45 | −0.87 | +0.05 |
| Seed 24, first mapping, assignment | Held within (027) | 0.00 | −5.20 | 0.00 | 0.00 | 0.00 | 0.00 | −5.93 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, assignment | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, assignment | Without replay | +0.49 | −0.17 | +1.19 | −0.03 | −0.03 | −0.15 | −0.57 | +1.10 | −0.23 | −0.01 |
| Seed 23, eighth, assignment | Own group | 0.00 | −6.39 | −2.51 | −0.88 | 0.00 | 0.00 | −6.10 | −2.18 | −0.71 | −0.01 |
| Seed 23, eighth, assignment | Inhibitory units | −2.41 | −6.19 | 0.00 | −0.83 | −0.01 | −2.20 | −5.97 | 0.00 | −0.74 | −0.02 |
| Seed 23, eighth, assignment | Outside the readouts | −2.40 | −6.36 | −2.49 | 0.00 | 0.00 | −2.21 | −6.01 | −2.13 | 0.00 | 0.00 |
| Seed 23, eighth, assignment | Plain on A's (H-47) | −2.38 | −6.32 | −2.44 | −0.82 | −0.01 | −2.18 | −5.94 | −2.12 | −0.70 | −0.02 |
| Seed 23, eighth, assignment | Held within (027) | 0.00 | −6.29 | 0.00 | 0.00 | 0.00 | 0.00 | −6.06 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, assignment | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, mirrored | Without replay | −0.39 | +0.25 | +1.09 | +0.19 | +0.14 | −0.58 | −0.05 | +1.38 | −0.01 | +0.10 |
| Seed 24, first mapping, mirrored | Own group | 0.00 | −5.43 | −2.17 | −0.82 | −0.04 | 0.00 | −5.58 | −2.20 | −0.63 | 0.00 |
| Seed 24, first mapping, mirrored | Inhibitory units | −1.59 | −5.28 | 0.00 | −0.75 | −0.01 | −2.34 | −5.53 | 0.00 | −0.58 | +0.02 |
| Seed 24, first mapping, mirrored | Outside the readouts | −1.61 | −5.38 | −2.08 | 0.00 | 0.00 | −2.36 | −5.56 | −2.17 | 0.00 | 0.00 |
| Seed 24, first mapping, mirrored | Plain on A's (H-47) | −1.60 | −5.34 | −2.09 | −0.79 | −0.03 | −2.39 | −5.55 | −2.18 | −0.60 | +0.01 |
| Seed 24, first mapping, mirrored | Held within (027) | 0.00 | −5.34 | 0.00 | 0.00 | 0.00 | 0.00 | −5.57 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, mirrored | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, mirrored | Without replay | +0.19 | −0.15 | +1.35 | +0.12 | −0.03 | −0.25 | −0.54 | +1.25 | −0.26 | −0.01 |
| Seed 23, first mapping, mirrored | Own group | 0.00 | −5.84 | −2.08 | −0.68 | −0.02 | 0.00 | −5.63 | −1.98 | −0.56 | 0.00 |
| Seed 23, first mapping, mirrored | Inhibitory units | −2.01 | −5.65 | 0.00 | −0.68 | −0.02 | −1.72 | −5.54 | 0.00 | −0.54 | +0.01 |
| Seed 23, first mapping, mirrored | Outside the readouts | −2.03 | −5.69 | −2.04 | 0.00 | 0.00 | −1.68 | −5.60 | −1.97 | 0.00 | 0.00 |
| Seed 23, first mapping, mirrored | Plain on A's (H-47) | −2.02 | −5.68 | −2.03 | −0.67 | −0.02 | −1.66 | −5.56 | −1.94 | −0.52 | 0.00 |
| Seed 23, first mapping, mirrored | Held within (027) | 0.00 | −5.85 | 0.00 | 0.00 | 0.00 | 0.00 | −5.56 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, mirrored | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, assignment | Without replay | −0.50 | +0.10 | +1.05 | −0.04 | +0.08 | −0.72 | −0.36 | +0.99 | −0.07 | +0.09 |
| Seed 24, eighth, assignment | Own group | 0.00 | −6.10 | −2.44 | −1.05 | −0.04 | 0.00 | −6.26 | −2.63 | −0.85 | +0.04 |
| Seed 24, eighth, assignment | Inhibitory units | −1.81 | −6.01 | 0.00 | −0.97 | −0.03 | −2.70 | −6.19 | 0.00 | −0.76 | +0.04 |
| Seed 24, eighth, assignment | Outside the readouts | −1.86 | −6.13 | −2.37 | 0.00 | 0.00 | −2.70 | −6.26 | −2.60 | 0.00 | 0.00 |
| Seed 24, eighth, assignment | Plain on A's (H-47) | −1.85 | −6.12 | −2.35 | −0.96 | −0.04 | −2.71 | −6.23 | −2.59 | −0.79 | +0.04 |
| Seed 24, eighth, assignment | Held within (027) | 0.00 | −6.08 | 0.00 | 0.00 | 0.00 | 0.00 | −6.23 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, assignment | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, mirrored | Without replay | −0.41 | −0.42 | +1.38 | −0.02 | +0.03 | −0.78 | −0.55 | +1.36 | −0.20 | −0.02 |
| Seed 22, first mapping, mirrored | Own group | 0.00 | −4.83 | −1.65 | −0.39 | 0.00 | 0.00 | −6.06 | −2.13 | −0.27 | −0.02 |
| Seed 22, first mapping, mirrored | Inhibitory units | −2.36 | −4.69 | 0.00 | −0.37 | +0.01 | −2.24 | −5.91 | 0.00 | −0.23 | −0.02 |
| Seed 22, first mapping, mirrored | Outside the readouts | −2.41 | −4.79 | −1.69 | 0.00 | 0.00 | −2.30 | −5.98 | −2.08 | 0.00 | 0.00 |
| Seed 22, first mapping, mirrored | Plain on A's (H-47) | −2.39 | −4.73 | −1.65 | −0.34 | +0.01 | −2.30 | −6.01 | −2.09 | −0.23 | −0.02 |
| Seed 22, first mapping, mirrored | Held within (027) | 0.00 | −4.80 | 0.00 | 0.00 | 0.00 | 0.00 | −5.98 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, mirrored | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, assignment | Without replay | −0.52 | −0.37 | +0.77 | −0.01 | +0.04 | −0.83 | −0.68 | +1.15 | −0.15 | −0.03 |
| Seed 22, eighth, assignment | Own group | 0.00 | −5.72 | −2.15 | −0.52 | +0.02 | 0.00 | −6.64 | −2.33 | −0.47 | −0.01 |
| Seed 22, eighth, assignment | Inhibitory units | −2.51 | −5.68 | 0.00 | −0.44 | +0.03 | −2.76 | −6.55 | 0.00 | −0.41 | −0.01 |
| Seed 22, eighth, assignment | Outside the readouts | −2.48 | −5.74 | −2.08 | 0.00 | 0.00 | −2.77 | −6.52 | −2.24 | 0.00 | 0.00 |
| Seed 22, eighth, assignment | Plain on A's (H-47) | −2.49 | −5.68 | −2.05 | −0.46 | +0.03 | −2.79 | −6.56 | −2.27 | −0.41 | −0.01 |
| Seed 22, eighth, assignment | Held within (027) | 0.00 | −5.71 | 0.00 | 0.00 | 0.00 | 0.00 | −6.56 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, assignment | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |

- **The part each night holds shows 0.00**, both columns of it under the night held onto the units outside the readouts, and its class
  matrix entries moved by exactly nothing, in LSB, in every one of the 432 held nights (below). An entry of another part may show 0.00 too,
  rounded: these entries are not held.
- **Each part held alone left the other three moving as under H-47's plain night**, within 0.2 M a group of the plain night's change over
  the four nights, group by group. Under the night held onto the own group the held units' synapses onto the inhibitory units moved by
  −0.10 to +0.06 M less the plain night's, onto the units outside the readouts by −0.09 to 0.00 M and across by −0.17 to +0.16 M. Under the
  night held onto the inhibitory units, onto their own group by −0.06 to +0.07 M, onto the units outside the readouts by −0.05 to +0.08 M
  and across by −0.03 to +0.12 M. Under the night held onto the units outside the readouts, onto their own group by −0.09 to +0.04 M, onto
  the inhibitory units by −0.06 to +0.07 M and across by −0.10 to +0.04 M. The plain night moved them by −0.02 to −2.79 M a group onto
  their own group, −1.45 to −2.71 M onto the inhibitory units, −1.01 to +0.09 M onto the units outside the readouts and −4.73 to −6.98 M
  across.
- Under the night without replay the held units' synapses moved by −0.83 to +0.49 M a group onto their own group, +0.77 to +1.53 M onto the
  inhibitory units, −0.27 to +0.33 M onto the units outside the readouts and −0.68 to +0.25 M across.

The rest of each night's change: the inhibition onto each group's excitatory units and onto the other targets, and the excitatory units
outside the hold onto every target:

| Cell | Form | Inhibitory onto the odd places' excitatory units | onto the even places' | onto the inhibitory units | onto A's and B's units | onto the other excitatory | C's own onto all | D's own onto all | A's and B's units onto all | The other excitatory onto all |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | Without replay | +1.10 | +0.70 | +0.66 | −1.17 | +0.03 | −0.04 | +0.01 | −0.02 | +0.11 |
| Seed 22, eighth, mirrored | Own group | +14.20 | +12.27 | +4.42 | +0.10 | +0.03 | −0.06 | −0.02 | −0.04 | −0.08 |
| Seed 22, eighth, mirrored | Inhibitory units | +14.97 | +12.60 | +4.50 | +0.11 | +0.03 | −0.06 | −0.02 | −0.04 | −0.10 |
| Seed 22, eighth, mirrored | Outside the readouts | +14.25 | +12.29 | +4.41 | +0.11 | +0.05 | −0.04 | −0.02 | −0.04 | −0.15 |
| Seed 22, eighth, mirrored | Plain on A's (H-47) | +14.23 | +12.21 | +4.39 | +0.09 | +0.04 | −0.05 | −0.02 | −0.04 | −0.10 |
| Seed 22, eighth, mirrored | Held within (027) | +15.03 | +12.79 | +4.54 | +0.14 | +0.04 | −0.05 | −0.02 | −0.04 | −0.15 |
| Seed 22, eighth, mirrored | Excitatory (025, 026) | +15.87 | +13.70 | +4.79 | +0.14 | +0.04 | −0.08 | −0.06 | −0.04 | −0.15 |
| Seed 22, first mapping, assignment | Without replay | −1.89 | −2.20 | −0.35 | −0.97 | +0.04 | −0.04 | +0.01 | −0.02 | +0.13 |
| Seed 22, first mapping, assignment | Own group | +9.40 | +8.13 | +3.49 | +1.00 | +0.03 | −0.07 | −0.06 | −0.07 | −0.05 |
| Seed 22, first mapping, assignment | Inhibitory units | +9.73 | +8.34 | +3.64 | +1.01 | +0.02 | −0.06 | −0.07 | −0.07 | −0.05 |
| Seed 22, first mapping, assignment | Outside the readouts | +9.35 | +7.97 | +3.44 | +1.02 | +0.03 | −0.05 | −0.08 | −0.07 | −0.09 |
| Seed 22, first mapping, assignment | Plain on A's (H-47) | +9.29 | +7.95 | +3.43 | +0.99 | +0.02 | −0.06 | −0.08 | −0.07 | −0.06 |
| Seed 22, first mapping, assignment | Held within (027) | +9.89 | +8.48 | +3.66 | +1.04 | +0.03 | −0.06 | −0.07 | −0.07 | −0.09 |
| Seed 22, first mapping, assignment | Excitatory (025, 026) | +10.28 | +8.98 | +3.89 | +1.08 | +0.03 | −0.07 | −0.08 | −0.06 | −0.10 |
| Seed 23, eighth, mirrored | Without replay | +1.13 | +0.88 | +0.79 | −1.02 | +0.02 | 0.00 | −0.05 | −0.03 | −0.02 |
| Seed 23, eighth, mirrored | Own group | +14.54 | +12.96 | +4.61 | +0.14 | +0.05 | −0.05 | −0.02 | −0.07 | −0.24 |
| Seed 23, eighth, mirrored | Inhibitory units | +15.35 | +13.50 | +4.85 | +0.19 | +0.06 | −0.04 | −0.02 | −0.07 | −0.23 |
| Seed 23, eighth, mirrored | Outside the readouts | +14.48 | +12.92 | +4.58 | +0.14 | +0.05 | −0.05 | −0.02 | −0.08 | −0.24 |
| Seed 23, eighth, mirrored | Plain on A's (H-47) | +14.40 | +12.88 | +4.55 | +0.13 | +0.05 | −0.05 | −0.03 | −0.07 | −0.24 |
| Seed 23, eighth, mirrored | Held within (027) | +15.50 | +13.55 | +4.91 | +0.20 | +0.06 | −0.05 | −0.01 | −0.08 | −0.24 |
| Seed 23, eighth, mirrored | Excitatory (025, 026) | +16.32 | +14.41 | +5.18 | +0.21 | +0.05 | −0.08 | −0.02 | −0.07 | −0.23 |
| Seed 23, first mapping, assignment | Without replay | −1.65 | −1.99 | −0.59 | −0.87 | −0.03 | −0.03 | −0.05 | −0.04 | −0.03 |
| Seed 23, first mapping, assignment | Own group | +10.00 | +8.20 | +3.12 | +1.18 | +0.01 | −0.07 | −0.07 | −0.09 | −0.15 |
| Seed 23, first mapping, assignment | Inhibitory units | +10.26 | +8.45 | +3.44 | +1.22 | +0.01 | −0.06 | −0.06 | −0.09 | −0.15 |
| Seed 23, first mapping, assignment | Outside the readouts | +9.95 | +8.07 | +3.09 | +1.18 | +0.01 | −0.06 | −0.06 | −0.09 | −0.15 |
| Seed 23, first mapping, assignment | Plain on A's (H-47) | +9.88 | +8.08 | +3.09 | +1.16 | +0.01 | −0.07 | −0.06 | −0.09 | −0.15 |
| Seed 23, first mapping, assignment | Held within (027) | +10.44 | +8.52 | +3.47 | +1.23 | 0.00 | −0.06 | −0.06 | −0.09 | −0.15 |
| Seed 23, first mapping, assignment | Excitatory (025, 026) | +10.88 | +9.08 | +3.68 | +1.25 | 0.00 | −0.09 | −0.07 | −0.09 | −0.13 |
| Seed 24, eighth, mirrored | Without replay | +0.93 | +0.94 | +0.52 | −1.00 | +0.02 | −0.02 | −0.05 | −0.03 | −0.13 |
| Seed 24, eighth, mirrored | Own group | +14.58 | +11.34 | +3.91 | +0.06 | 0.00 | +0.06 | −0.01 | −0.09 | −0.22 |
| Seed 24, eighth, mirrored | Inhibitory units | +15.36 | +11.51 | +3.98 | +0.11 | 0.00 | +0.06 | 0.00 | −0.09 | −0.23 |
| Seed 24, eighth, mirrored | Outside the readouts | +14.62 | +11.24 | +3.91 | +0.08 | +0.02 | +0.04 | −0.02 | −0.09 | −0.24 |
| Seed 24, eighth, mirrored | Plain on A's (H-47) | +14.61 | +11.20 | +3.89 | +0.05 | 0.00 | +0.06 | −0.02 | −0.08 | −0.22 |
| Seed 24, eighth, mirrored | Held within (027) | +15.44 | +11.65 | +4.03 | +0.14 | +0.02 | +0.05 | 0.00 | −0.08 | −0.24 |
| Seed 24, eighth, mirrored | Excitatory (025, 026) | +15.90 | +12.12 | +4.11 | +0.15 | +0.02 | +0.04 | −0.02 | −0.08 | −0.24 |
| Seed 24, first mapping, assignment | Without replay | −1.73 | −1.86 | −0.73 | −0.86 | +0.03 | 0.00 | −0.06 | −0.05 | −0.17 |
| Seed 24, first mapping, assignment | Own group | +10.55 | +6.93 | +2.77 | +1.04 | +0.01 | +0.06 | −0.01 | −0.10 | −0.23 |
| Seed 24, first mapping, assignment | Inhibitory units | +10.92 | +7.03 | +2.85 | +1.08 | 0.00 | +0.07 | −0.01 | −0.10 | −0.22 |
| Seed 24, first mapping, assignment | Outside the readouts | +10.48 | +6.81 | +2.76 | +1.06 | +0.04 | +0.06 | −0.01 | −0.09 | −0.21 |
| Seed 24, first mapping, assignment | Plain on A's (H-47) | +10.49 | +6.77 | +2.76 | +1.03 | +0.01 | +0.07 | −0.01 | −0.10 | −0.23 |
| Seed 24, first mapping, assignment | Held within (027) | +11.02 | +7.11 | +2.87 | +1.11 | +0.04 | +0.05 | 0.00 | −0.09 | −0.20 |
| Seed 24, first mapping, assignment | Excitatory (025, 026) | +11.39 | +7.46 | +3.01 | +1.13 | +0.04 | +0.03 | −0.02 | −0.09 | −0.22 |
| Seed 23, eighth, assignment | Without replay | +1.17 | +1.02 | +0.67 | −1.03 | +0.02 | −0.06 | −0.05 | −0.03 | −0.02 |
| Seed 23, eighth, assignment | Own group | +12.18 | +13.59 | +3.95 | +0.10 | +0.02 | −0.05 | −0.06 | −0.06 | −0.20 |
| Seed 23, eighth, assignment | Inhibitory units | +12.88 | +14.01 | +4.14 | +0.11 | +0.02 | −0.04 | −0.06 | −0.06 | −0.17 |
| Seed 23, eighth, assignment | Outside the readouts | +12.04 | +13.47 | +3.91 | +0.14 | +0.02 | −0.05 | −0.06 | −0.06 | −0.19 |
| Seed 23, eighth, assignment | Plain on A's (H-47) | +12.07 | +13.48 | +3.92 | +0.10 | +0.02 | −0.05 | −0.06 | −0.06 | −0.18 |
| Seed 23, eighth, assignment | Held within (027) | +13.13 | +14.24 | +4.20 | +0.15 | +0.02 | −0.05 | −0.06 | −0.06 | −0.18 |
| Seed 23, eighth, assignment | Excitatory (025, 026) | +13.82 | +14.93 | +4.43 | +0.15 | +0.02 | −0.05 | −0.08 | −0.07 | −0.18 |
| Seed 24, first mapping, mirrored | Without replay | −1.77 | −1.92 | −0.73 | −0.87 | +0.03 | +0.01 | −0.06 | −0.05 | −0.17 |
| Seed 24, first mapping, mirrored | Own group | +7.90 | +7.92 | +2.37 | +1.19 | +0.06 | −0.01 | −0.01 | −0.10 | −0.25 |
| Seed 24, first mapping, mirrored | Inhibitory units | +8.02 | +8.23 | +2.63 | +1.21 | +0.07 | −0.02 | −0.01 | −0.10 | −0.25 |
| Seed 24, first mapping, mirrored | Outside the readouts | +7.74 | +7.93 | +2.38 | +1.19 | +0.07 | −0.02 | −0.01 | −0.10 | −0.26 |
| Seed 24, first mapping, mirrored | Plain on A's (H-47) | +7.72 | +7.83 | +2.32 | +1.18 | +0.06 | −0.03 | −0.01 | −0.10 | −0.26 |
| Seed 24, first mapping, mirrored | Held within (027) | +8.26 | +8.47 | +2.71 | +1.24 | +0.07 | −0.02 | −0.01 | −0.10 | −0.25 |
| Seed 24, first mapping, mirrored | Excitatory (025, 026) | +8.60 | +8.80 | +2.81 | +1.24 | +0.07 | −0.04 | −0.02 | −0.10 | −0.24 |
| Seed 23, first mapping, mirrored | Without replay | −1.79 | −2.04 | −0.59 | −0.90 | −0.04 | −0.01 | −0.05 | −0.04 | −0.02 |
| Seed 23, first mapping, mirrored | Own group | +8.46 | +8.89 | +2.96 | +1.07 | −0.03 | −0.05 | −0.04 | −0.07 | −0.11 |
| Seed 23, first mapping, mirrored | Inhibitory units | +8.77 | +9.12 | +3.16 | +1.07 | −0.02 | −0.04 | −0.04 | −0.07 | −0.11 |
| Seed 23, first mapping, mirrored | Outside the readouts | +8.35 | +8.79 | +2.90 | +1.09 | −0.03 | −0.05 | −0.05 | −0.07 | −0.11 |
| Seed 23, first mapping, mirrored | Plain on A's (H-47) | +8.33 | +8.77 | +2.94 | +1.07 | −0.03 | −0.04 | −0.04 | −0.07 | −0.12 |
| Seed 23, first mapping, mirrored | Held within (027) | +8.96 | +9.22 | +3.18 | +1.10 | −0.03 | −0.06 | −0.05 | −0.07 | −0.11 |
| Seed 23, first mapping, mirrored | Excitatory (025, 026) | +9.41 | +9.56 | +3.37 | +1.10 | −0.03 | −0.06 | −0.05 | −0.08 | −0.11 |
| Seed 24, eighth, assignment | Without replay | +0.98 | +0.78 | +0.54 | −0.99 | +0.02 | −0.01 | −0.05 | −0.03 | −0.13 |
| Seed 24, eighth, assignment | Own group | +11.61 | +11.97 | +3.69 | +0.22 | +0.04 | +0.08 | −0.05 | −0.10 | −0.22 |
| Seed 24, eighth, assignment | Inhibitory units | +11.87 | +12.55 | +3.91 | +0.25 | +0.04 | +0.09 | −0.05 | −0.10 | −0.22 |
| Seed 24, eighth, assignment | Outside the readouts | +11.48 | +11.87 | +3.71 | +0.24 | +0.05 | +0.08 | −0.05 | −0.10 | −0.24 |
| Seed 24, eighth, assignment | Plain on A's (H-47) | +11.38 | +11.76 | +3.66 | +0.22 | +0.04 | +0.09 | −0.05 | −0.10 | −0.21 |
| Seed 24, eighth, assignment | Held within (027) | +12.19 | +12.85 | +3.98 | +0.25 | +0.05 | +0.08 | −0.04 | −0.10 | −0.26 |
| Seed 24, eighth, assignment | Excitatory (025, 026) | +12.61 | +13.27 | +4.07 | +0.25 | +0.04 | +0.05 | −0.07 | −0.10 | −0.26 |
| Seed 22, first mapping, mirrored | Without replay | −1.92 | −2.23 | −0.39 | −0.99 | +0.03 | −0.03 | +0.01 | −0.03 | +0.13 |
| Seed 22, first mapping, mirrored | Own group | +6.65 | +7.36 | +3.10 | +0.71 | +0.05 | −0.11 | −0.13 | −0.07 | −0.03 |
| Seed 22, first mapping, mirrored | Inhibitory units | +6.91 | +7.38 | +3.23 | +0.78 | +0.06 | −0.11 | −0.13 | −0.07 | −0.04 |
| Seed 22, first mapping, mirrored | Outside the readouts | +6.53 | +7.23 | +3.05 | +0.73 | +0.05 | −0.09 | −0.14 | −0.07 | −0.07 |
| Seed 22, first mapping, mirrored | Plain on A's (H-47) | +6.52 | +7.22 | +3.05 | +0.70 | +0.06 | −0.10 | −0.13 | −0.08 | −0.04 |
| Seed 22, first mapping, mirrored | Held within (027) | +7.05 | +7.56 | +3.28 | +0.82 | +0.05 | −0.10 | −0.13 | −0.07 | −0.07 |
| Seed 22, first mapping, mirrored | Excitatory (025, 026) | +7.47 | +7.95 | +3.44 | +0.87 | +0.05 | −0.12 | −0.14 | −0.07 | −0.07 |
| Seed 22, eighth, assignment | Without replay | +0.89 | +0.60 | +0.62 | −1.11 | +0.03 | −0.04 | +0.03 | −0.03 | +0.11 |
| Seed 22, eighth, assignment | Own group | +10.76 | +11.48 | +3.74 | +0.02 | +0.05 | −0.04 | −0.09 | −0.06 | −0.07 |
| Seed 22, eighth, assignment | Inhibitory units | +11.13 | +11.69 | +3.90 | +0.02 | +0.05 | −0.03 | −0.09 | −0.06 | −0.07 |
| Seed 22, eighth, assignment | Outside the readouts | +10.58 | +11.30 | +3.72 | +0.02 | +0.05 | −0.03 | −0.11 | −0.06 | −0.11 |
| Seed 22, eighth, assignment | Plain on A's (H-47) | +10.61 | +11.30 | +3.72 | 0.00 | +0.05 | −0.03 | −0.09 | −0.07 | −0.07 |
| Seed 22, eighth, assignment | Held within (027) | +11.34 | +11.90 | +3.93 | +0.05 | +0.05 | −0.05 | −0.10 | −0.06 | −0.11 |
| Seed 22, eighth, assignment | Excitatory (025, 026) | +11.88 | +12.35 | +4.11 | +0.08 | +0.05 | −0.07 | −0.10 | −0.05 | −0.12 |

- **The inhibition onto each group's excitatory units rose by 6.65 to 14.58 M** over four nights under the night held onto the own group,
  by 6.91 to 15.36 M under the night held onto the inhibitory units and by 6.53 to 14.62 M under the night held onto the units outside the
  readouts, where H-47's plain night raised it by 6.52 to 14.61 M, exploration 027's night held within by 7.05 to 15.50 M and the
  excitatory night by 7.46 to 16.32 M. Less the plain night's, group by group: −0.04 to +0.24 M under the night held onto the own group,
  +0.16 to +0.96 M under the night held onto the inhibitory units and −0.03 to +0.11 M under the night held onto the units outside the
  readouts, where exploration 027's night held within's was +0.33 to +1.11 M and the excitatory night's +0.69 to +1.92 M. Under each form
  with replay, in each of the twelve cells over the four nights, it rose by more onto the group A answers than onto the other: by 0.02 to
  3.63 M, 0.21 to 3.89 M and 0.19 to 3.67 M under the three held nights, and 0.10 to 3.71 M under the plain night. Onto the inhibitory
  units it rose by 2.37 to 4.61 M, 2.63 to 4.85 M and 2.38 to 4.58 M, where the plain night's rose by 2.32 to 4.55 M.
- **The excitatory units outside the hold**: the pair's own units onto all moved by −0.13 to +0.08 M under the night held onto the own
  group, −0.13 to +0.09 M under the night held onto the inhibitory units and −0.14 to +0.08 M under the night held onto the units outside
  the readouts; A's and B's units onto all by −0.10 to −0.04 M under each; and the other excitatory units by −0.25 to −0.03 M, −0.25 to
  −0.04 M and −0.26 to −0.07 M.

The held part's entries night by night, against what the quiet ticks after each night moved:

| Cell | Form | Nights the held part's entries moved by nothing | Nights they moved by what the quiet ticks moved | Nights the quiet ticks moved held synapses | Nights each other part's entries moved, of 12, in the order own group, inhibitory units, outside the readouts, held across, the held part left out |
| :--- | :--- | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | Own group | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 22, eighth, mirrored | Inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 22, eighth, mirrored | Outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 22, first mapping, assignment | Own group | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 22, first mapping, assignment | Inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 22, first mapping, assignment | Outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 23, eighth, mirrored | Own group | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 23, eighth, mirrored | Inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 23, eighth, mirrored | Outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 23, first mapping, assignment | Own group | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 23, first mapping, assignment | Inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 23, first mapping, assignment | Outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 24, eighth, mirrored | Own group | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 24, eighth, mirrored | Inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 24, eighth, mirrored | Outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 24, first mapping, assignment | Own group | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 24, first mapping, assignment | Inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 24, first mapping, assignment | Outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 23, eighth, assignment | Own group | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 23, eighth, assignment | Inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 23, eighth, assignment | Outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 24, first mapping, mirrored | Own group | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 24, first mapping, mirrored | Inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 24, first mapping, mirrored | Outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 23, first mapping, mirrored | Own group | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 23, first mapping, mirrored | Inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 23, first mapping, mirrored | Outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 24, eighth, assignment | Own group | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 24, eighth, assignment | Inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 24, eighth, assignment | Outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 22, first mapping, mirrored | Own group | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 22, first mapping, mirrored | Inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 22, first mapping, mirrored | Outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 22, eighth, assignment | Own group | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 22, eighth, assignment | Inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |
| Seed 22, eighth, assignment | Outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12, 12 |

Each night's change of the engine's two sums, from its `start` line to its `end` line, the mean over the three paths:

| Cell | Gap | Without replay: inhibitory and excitatory sums, M | Own group | Inhibitory units | Outside the readouts | Plain on A's | Held within (027) | Excitatory |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +0.31, −0.49 | +10.11, −7.27 | +10.36, −6.68 | +10.15, −7.72 | +10.11, −8.37 | +10.41, −4.96 | +10.75, −0.12 |
| Seed 22, eighth, mirrored | 12 | +0.31, −0.14 | +8.09, −5.10 | +8.43, −4.51 | +8.07, −5.40 | +8.07, −5.67 | +8.52, −3.56 | +9.15, −0.09 |
| Seed 22, eighth, mirrored | 18 | +0.30, +0.07 | +6.90, −3.43 | +7.23, −2.76 | +6.96, −3.50 | +6.90, −3.55 | +7.36, −2.57 | +7.82, −0.10 |
| Seed 22, eighth, mirrored | 24 | +0.39, +0.37 | +5.91, −2.34 | +6.19, −1.75 | +5.93, −2.16 | +5.89, −2.20 | +6.25, −1.72 | +6.82, −0.03 |
| Seed 22, first mapping, assignment | 6 | −1.22, −0.51 | +9.12, −6.12 | +9.28, −5.60 | +9.12, −6.53 | +9.08, −6.90 | +9.32, −4.50 | +9.57, −0.10 |
| Seed 22, first mapping, assignment | 12 | −1.36, +0.11 | +6.10, −4.62 | +6.34, −4.30 | +6.03, −5.00 | +6.02, −5.36 | +6.42, −3.31 | +6.74, −0.07 |
| Seed 22, first mapping, assignment | 18 | −1.46, +0.45 | +4.07, −3.33 | +4.30, −2.69 | +4.02, −3.26 | +3.98, −3.43 | +4.39, −2.41 | +4.71, −0.07 |
| Seed 22, first mapping, assignment | 24 | −1.33, +0.34 | +2.76, −2.59 | +2.83, −2.09 | +2.64, −2.53 | +2.60, −2.62 | +2.97, −1.93 | +3.24, −0.06 |
| Seed 23, eighth, mirrored | 6 | +0.33, −0.69 | +10.62, −7.86 | +10.89, −6.84 | +10.57, −8.19 | +10.56, −8.75 | +10.96, −5.36 | +11.35, −0.16 |
| Seed 23, eighth, mirrored | 12 | +0.37, +0.12 | +8.53, −5.31 | +8.90, −4.46 | +8.50, −5.63 | +8.49, −5.70 | +8.94, −3.74 | +9.43, −0.12 |
| Seed 23, eighth, mirrored | 18 | +0.46, +0.83 | +7.15, −3.89 | +7.74, −3.04 | +7.18, −4.07 | +7.05, −4.18 | +7.86, −2.66 | +8.38, −0.06 |
| Seed 23, eighth, mirrored | 24 | +0.63, +1.49 | +6.01, −2.87 | +6.42, −2.25 | +5.93, −2.69 | +5.90, −2.89 | +6.45, −2.05 | +7.01, −0.06 |
| Seed 23, first mapping, assignment | 6 | −1.03, −0.46 | +9.61, −6.24 | +9.86, −5.35 | +9.58, −6.55 | +9.58, −6.79 | +9.90, −4.46 | +10.13, −0.11 |
| Seed 23, first mapping, assignment | 12 | −1.29, +0.47 | +6.37, −4.90 | +6.70, −4.12 | +6.35, −5.03 | +6.32, −5.31 | +6.74, −3.38 | +7.07, −0.10 |
| Seed 23, first mapping, assignment | 18 | −1.41, +0.76 | +3.86, −3.47 | +4.03, −2.36 | +3.72, −3.08 | +3.72, −3.20 | +4.09, −2.44 | +4.48, −0.06 |
| Seed 23, first mapping, assignment | 24 | −1.42, +1.16 | +2.66, −2.58 | +2.79, −1.85 | +2.64, −2.42 | +2.60, −2.37 | +2.92, −2.06 | +3.21, −0.10 |
| Seed 24, eighth, mirrored | 6 | +0.32, −0.58 | +9.50, −7.05 | +9.67, −6.56 | +9.48, −7.70 | +9.47, −8.12 | +9.73, −5.00 | +9.95, −0.14 |
| Seed 24, eighth, mirrored | 12 | +0.26, +0.18 | +7.84, −4.72 | +8.24, −4.44 | +7.82, −5.36 | +7.82, −5.64 | +8.30, −3.35 | +8.56, −0.09 |
| Seed 24, eighth, mirrored | 18 | +0.35, +0.31 | +6.63, −3.61 | +7.03, −2.94 | +6.65, −3.69 | +6.61, −3.88 | +7.07, −2.40 | +7.37, −0.05 |
| Seed 24, eighth, mirrored | 24 | +0.48, +0.38 | +5.91, −2.73 | +6.01, −2.03 | +5.91, −2.46 | +5.85, −2.71 | +6.17, −1.77 | +6.41, −0.02 |
| Seed 24, first mapping, assignment | 6 | −1.21, +0.12 | +8.79, −5.57 | +8.90, −5.13 | +8.74, −6.00 | +8.72, −6.29 | +8.93, −4.13 | +9.08, −0.11 |
| Seed 24, first mapping, assignment | 12 | −1.33, +0.45 | +5.99, −4.26 | +6.05, −4.20 | +5.95, −4.85 | +5.92, −5.07 | +6.14, −3.18 | +6.38, −0.07 |
| Seed 24, first mapping, assignment | 18 | −1.30, +0.67 | +3.84, −3.36 | +4.10, −2.59 | +3.83, −3.33 | +3.84, −3.53 | +4.18, −2.34 | +4.43, −0.06 |
| Seed 24, first mapping, assignment | 24 | −1.32, +0.77 | +2.67, −2.50 | +2.84, −1.96 | +2.62, −2.38 | +2.56, −2.48 | +2.89, −1.72 | +3.13, −0.06 |
| Seed 23, eighth, assignment | 6 | +0.36, −0.82 | +9.75, −7.47 | +10.04, −7.74 | +9.69, −8.84 | +9.72, −9.49 | +10.11, −4.87 | +10.37, −0.16 |
| Seed 23, eighth, assignment | 12 | +0.35, +0.09 | +7.87, −5.24 | +8.23, −5.13 | +7.82, −6.20 | +7.84, −6.46 | +8.44, −3.54 | +8.86, −0.10 |
| Seed 23, eighth, assignment | 18 | +0.45, +0.65 | +6.66, −3.77 | +6.97, −3.34 | +6.55, −4.11 | +6.47, −4.38 | +7.12, −2.43 | +7.63, −0.06 |
| Seed 23, eighth, assignment | 24 | +0.68, +1.52 | +5.56, −2.68 | +5.93, −2.50 | +5.52, −2.80 | +5.54, −2.94 | +6.08, −1.86 | +6.50, −0.05 |
| Seed 24, first mapping, mirrored | 6 | −1.28, +0.04 | +8.01, −6.20 | +8.20, −6.44 | +7.98, −7.38 | +7.94, −7.93 | +8.27, −4.14 | +8.49, −0.18 |
| Seed 24, first mapping, mirrored | 12 | −1.36, +0.43 | +5.00, −4.66 | +5.26, −4.50 | +4.96, −5.44 | +4.95, −5.78 | +5.41, −2.98 | +5.63, −0.12 |
| Seed 24, first mapping, mirrored | 18 | −1.30, +0.68 | +3.64, −3.64 | +3.76, −3.08 | +3.63, −3.82 | +3.55, −4.10 | +3.95, −2.30 | +4.10, −0.05 |
| Seed 24, first mapping, mirrored | 24 | −1.33, +0.69 | +2.79, −2.75 | +2.93, −2.43 | +2.75, −2.92 | +2.67, −3.14 | +3.11, −1.86 | +3.30, −0.05 |
| Seed 23, first mapping, mirrored | 6 | −1.19, −0.48 | +8.85, −6.00 | +9.02, −6.34 | +8.87, −7.28 | +8.85, −7.72 | +9.11, −4.13 | +9.31, −0.08 |
| Seed 23, first mapping, mirrored | 12 | −1.40, +0.37 | +5.90, −4.68 | +6.03, −4.70 | +5.81, −5.60 | +5.81, −5.84 | +6.11, −3.21 | +6.46, −0.07 |
| Seed 23, first mapping, mirrored | 18 | −1.43, +0.64 | +3.87, −3.57 | +4.11, −3.07 | +3.81, −3.73 | +3.79, −3.93 | +4.18, −2.42 | +4.35, −0.09 |
| Seed 23, first mapping, mirrored | 24 | −1.33, +1.02 | +2.72, −2.82 | +2.95, −2.30 | +2.62, −2.69 | +2.62, −2.89 | +3.03, −1.94 | +3.30, −0.07 |
| Seed 24, eighth, assignment | 6 | +0.32, −0.47 | +8.88, −7.46 | +8.94, −7.60 | +8.81, −8.56 | +8.80, −9.36 | +9.02, −4.82 | +9.29, −0.17 |
| Seed 24, eighth, assignment | 12 | +0.25, +0.10 | +7.30, −5.51 | +7.63, −5.21 | +7.22, −6.24 | +7.16, −6.68 | +7.81, −3.50 | +8.07, −0.08 |
| Seed 24, eighth, assignment | 18 | +0.32, +0.43 | +6.12, −3.88 | +6.46, −3.40 | +6.03, −4.12 | +5.92, −4.45 | +6.67, −2.48 | +6.97, −0.08 |
| Seed 24, eighth, assignment | 24 | +0.44, +0.34 | +5.25, −2.77 | +5.58, −2.50 | +5.29, −3.30 | +5.16, −3.38 | +5.81, −1.83 | +5.90, −0.05 |
| Seed 22, first mapping, mirrored | 6 | −1.32, −0.44 | +7.43, −5.67 | +7.51, −6.12 | +7.38, −7.34 | +7.36, −7.55 | +7.64, −4.02 | +7.80, −0.12 |
| Seed 22, first mapping, mirrored | 12 | −1.39, +0.09 | +4.87, −4.27 | +4.95, −4.61 | +4.78, −5.67 | +4.75, −5.75 | +5.12, −3.03 | +5.47, −0.12 |
| Seed 22, first mapping, mirrored | 18 | −1.39, +0.40 | +3.29, −3.26 | +3.44, −3.07 | +3.23, −3.69 | +3.21, −3.85 | +3.49, −2.23 | +3.71, −0.08 |
| Seed 22, first mapping, mirrored | 24 | −1.40, +0.37 | +2.28, −2.50 | +2.45, −2.35 | +2.20, −2.91 | +2.24, −2.96 | +2.52, −1.86 | +2.81, −0.09 |
| Seed 22, eighth, assignment | 6 | +0.28, −0.61 | +8.40, −6.83 | +8.55, −7.40 | +8.35, −8.80 | +8.35, −9.17 | +8.64, −4.66 | +8.83, −0.13 |
| Seed 22, eighth, assignment | 12 | +0.23, −0.11 | +6.77, −4.88 | +7.05, −5.07 | +6.66, −6.23 | +6.67, −6.30 | +7.15, −3.47 | +7.48, −0.12 |
| Seed 22, eighth, assignment | 18 | +0.24, −0.06 | +5.80, −3.56 | +6.01, −3.41 | +5.72, −4.16 | +5.74, −4.29 | +6.14, −2.58 | +6.53, −0.07 |
| Seed 22, eighth, assignment | 24 | +0.28, +0.23 | +5.07, −2.81 | +5.17, −2.69 | +4.93, −2.95 | +4.92, −3.19 | +5.35, −1.87 | +5.63, −0.03 |

A night held onto the own group, the mean of its three paths, raised the inhibitory sum by 2.28 to 10.62 M and lowered the excitatory sum
by 2.34 to 7.86 M; a night held onto the inhibitory units raised the first by 2.45 to 10.89 M and lowered the second by 1.75 to 7.74 M; and
a night held onto the units outside the readouts raised the first by 2.20 to 10.57 M and lowered the second by 2.16 to 8.84 M. H-47's
plain night raised the first by 2.24 to 10.56 M and lowered the second by 2.20 to 9.49 M, exploration 027's night held within raised the
first by 2.52 to 10.96 M and lowered the second by 1.72 to 5.36 M, and the excitatory night raised the first by 2.81 to 11.35 M and lowered
the second by 0.02 to 0.18 M.

**The ties**, the tied trials of each cell's 240 reads of A and 240 of the pair, under each form:

| Cell | Without replay, A's and the pair's | H-47's without replay | Own group | Inhibitory units | Outside the readouts | H-47's plain on A's | Held within (027) | Excitatory (025, 026) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 1160 and 382 | 1160 and 382 | 1228 and 285 | 1277 and 286 | 1179 and 260 | 1202 and 287 | 1260 and 300 | 1324 and 338 |
| Seed 22, first mapping, assignment | 1141 and 408 | 1141 and 408 | 1091 and 472 | 1137 and 471 | 1132 and 446 | 1183 and 491 | 1128 and 501 | 1213 and 472 |
| Seed 23, eighth, mirrored | 935 and 332 | 935 and 332 | 1056 and 241 | 1017 and 267 | 1005 and 242 | 1042 and 228 | 1076 and 240 | 1090 and 242 |
| Seed 23, first mapping, assignment | 1086 and 483 | 1086 and 483 | 782 and 631 | 733 and 623 | 770 and 646 | 742 and 618 | 785 and 699 | 883 and 629 |
| Seed 24, eighth, mirrored | 1137 and 322 | 1137 and 322 | 1420 and 161 | 1429 and 147 | 1380 and 119 | 1372 and 137 | 1426 and 164 | 1403 and 170 |
| Seed 24, first mapping, assignment | 968 and 520 | 968 and 520 | 788 and 611 | 804 and 537 | 792 and 586 | 835 and 584 | 711 and 642 | 857 and 588 |
| Seed 23, eighth, assignment | 1049 and 256 | 1049 and 256 | 1136 and 209 | 946 and 232 | 979 and 231 | 1039 and 213 | 854 and 246 | 907 and 251 |
| Seed 24, first mapping, mirrored | 1066 and 315 | 1066 and 315 | 925 and 380 | 896 and 363 | 890 and 349 | 887 and 332 | 910 and 374 | 835 and 421 |
| Seed 23, first mapping, mirrored | 792 and 293 | 792 and 293 | 920 and 360 | 790 and 382 | 821 and 362 | 866 and 365 | 844 and 365 | 826 and 360 |
| Seed 24, eighth, assignment | 1114 and 362 | 1114 and 362 | 1408 and 170 | 1430 and 151 | 1370 and 184 | 1324 and 174 | 1391 and 162 | 1391 and 183 |
| Seed 22, first mapping, mirrored | 943 and 302 | 943 and 302 | 848 and 345 | 839 and 372 | 790 and 359 | 878 and 357 | 833 and 336 | 862 and 330 |
| Seed 22, eighth, assignment | 917 and 207 | 917 and 207 | 947 and 136 | 902 and 129 | 870 and 135 | 898 and 130 | 829 and 175 | 873 and 124 |

The night without replay's ties are H-47's, read by read. From H-47's plain night's, the night held onto the own group's ties lie 14 to 97
away on A's reads and 2 to 48 on the pair's, the night held onto the inhibitory units' 4 to 106 and 1 to 47, and the night held onto the
units outside the readouts' 3 to 88 and 2 to 45. From exploration 027's night held within's they lie 3 to 282 and 1 to 68, 3 to 93 and 11
to 105, and 4 to 125 and 2 to 56.

## 5. What the hold leaves

At each held night, over its twelve nights a cell and form (ranges are over the twelve): the synapses of the held part; their traces cleared
at the night's last tick, how many were not zero and what they held above and below zero; the excitatory synapses from units outside the
hold the night moved; the nights in which a held synapse's presynaptic unit fired at or after the night's last tick, and the nights in
which the quiet ticks moved synapses of the held part:

| Cell | Form | Nights | Held synapses | Traces cleared, not zero | cleared above zero, LSB | below zero | Excitatory synapses from units outside the hold moved | Nights a held synapse's presynaptic unit fired at or after the night's last tick | Nights held synapses moved in the quiet ticks: how many, their change |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | Own group | 12 | 7 030 | 4 422 to 4 611 | 329 143 to 424 174 | −484 425 to −400 893 | 4 940 to 4 987 | 0 | none |
| Seed 22, eighth, mirrored | Inhibitory units | 12 | 4 534 | 2 943 to 3 109 | 145 564 to 179 706 | −299 987 to −227 593 | 4 945 to 4 985 | 0 | none |
| Seed 22, eighth, mirrored | Outside the readouts | 12 | 2 339 | 1 439 to 1 518 | 106 534 to 137 625 | −168 671 to −132 611 | 4 946 to 4 986 | 0 | none |
| Seed 22, first mapping, assignment | Own group | 12 | 7 030 | 4 310 to 4 489 | 343 091 to 429 082 | −430 228 to −340 448 | 4 946 to 4 991 | 0 | none |
| Seed 22, first mapping, assignment | Inhibitory units | 12 | 4 534 | 2 777 to 2 887 | 123 712 to 153 963 | −266 754 to −198 808 | 4 949 to 4 990 | 1 | none |
| Seed 22, first mapping, assignment | Outside the readouts | 12 | 2 339 | 1 406 to 1 459 | 107 153 to 125 857 | −171 124 to −110 442 | 4 953 to 4 990 | 1 | none |
| Seed 23, eighth, mirrored | Own group | 12 | 6 919 | 4 391 to 4 581 | 293 633 to 397 545 | −491 859 to −401 976 | 4 951 to 4 983 | 4 | none |
| Seed 23, eighth, mirrored | Inhibitory units | 12 | 4 487 | 2 835 to 3 010 | 122 613 to 145 005 | −299 603 to −222 879 | 4 962 to 4 982 | 3 | none |
| Seed 23, eighth, mirrored | Outside the readouts | 12 | 2 366 | 1 464 to 1 551 | 100 806 to 131 178 | −187 168 to −139 617 | 4 948 to 4 984 | 4 | none |
| Seed 23, first mapping, assignment | Own group | 12 | 6 919 | 4 333 to 4 538 | 339 623 to 418 387 | −455 532 to −383 195 | 4 970 to 4 988 | 0 | none |
| Seed 23, first mapping, assignment | Inhibitory units | 12 | 4 487 | 2 770 to 2 899 | 116 142 to 147 653 | −274 654 to −198 137 | 4 965 to 4 988 | 0 | none |
| Seed 23, first mapping, assignment | Outside the readouts | 12 | 2 366 | 1 424 to 1 506 | 107 002 to 131 026 | −162 755 to −112 680 | 4 961 to 4 991 | 3 | none |
| Seed 24, eighth, mirrored | Own group | 12 | 6 891 | 4 272 to 4 469 | 317 573 to 366 311 | −449 899 to −392 810 | 4 965 to 4 987 | 0 | none |
| Seed 24, eighth, mirrored | Inhibitory units | 12 | 4 543 | 2 786 to 2 981 | 129 330 to 180 438 | −284 924 to −227 608 | 4 967 to 4 984 | 0 | none |
| Seed 24, eighth, mirrored | Outside the readouts | 12 | 2 361 | 1 429 to 1 498 | 105 195 to 146 019 | −144 387 to −135 645 | 4 957 to 4 986 | 0 | none |
| Seed 24, first mapping, assignment | Own group | 12 | 6 891 | 4 172 to 4 440 | 293 513 to 443 684 | −460 962 to −369 655 | 4 970 to 4 987 | 0 | none |
| Seed 24, first mapping, assignment | Inhibitory units | 12 | 4 543 | 2 719 to 2 864 | 133 135 to 160 042 | −266 116 to −199 788 | 4 974 to 4 990 | 0 | none |
| Seed 24, first mapping, assignment | Outside the readouts | 12 | 2 361 | 1 409 to 1 492 | 99 384 to 126 981 | −149 805 to −116 878 | 4 970 to 4 987 | 0 | none |
| Seed 23, eighth, assignment | Own group | 12 | 6 919 | 4 197 to 4 315 | 237 840 to 289 875 | −445 726 to −352 756 | 4 967 to 4 988 | 3 | none |
| Seed 23, eighth, assignment | Inhibitory units | 12 | 4 487 | 2 821 to 2 978 | 127 760 to 149 862 | −278 309 to −233 782 | 4 961 to 4 988 | 3 | none |
| Seed 23, eighth, assignment | Outside the readouts | 12 | 2 366 | 1 391 to 1 525 | 81 705 to 113 691 | −158 124 to −132 564 | 4 939 to 4 989 | 3 | none |
| Seed 24, first mapping, mirrored | Own group | 12 | 6 891 | 4 075 to 4 204 | 256 821 to 307 586 | −384 271 to −326 615 | 4 961 to 4 990 | 0 | none |
| Seed 24, first mapping, mirrored | Inhibitory units | 12 | 4 543 | 2 781 to 2 888 | 114 962 to 143 380 | −266 177 to −212 484 | 4 966 to 4 989 | 0 | none |
| Seed 24, first mapping, mirrored | Outside the readouts | 12 | 2 361 | 1 415 to 1 527 | 99 903 to 119 745 | −157 357 to −94 452 | 4 961 to 4 987 | 0 | none |
| Seed 23, first mapping, mirrored | Own group | 12 | 6 919 | 4 088 to 4 214 | 243 684 to 293 733 | −395 347 to −331 401 | 4 960 to 4 991 | 0 | none |
| Seed 23, first mapping, mirrored | Inhibitory units | 12 | 4 487 | 2 763 to 2 859 | 107 368 to 145 973 | −264 896 to −188 780 | 4 960 to 4 988 | 3 | none |
| Seed 23, first mapping, mirrored | Outside the readouts | 12 | 2 366 | 1 421 to 1 487 | 90 888 to 109 136 | −162 188 to −112 367 | 4 951 to 4 988 | 3 | none |
| Seed 24, eighth, assignment | Own group | 12 | 6 891 | 4 170 to 4 359 | 261 801 to 298 612 | −412 590 to −358 341 | 4 949 to 4 991 | 0 | none |
| Seed 24, eighth, assignment | Inhibitory units | 12 | 4 543 | 2 855 to 3 035 | 120 090 to 157 707 | −285 627 to −213 026 | 4 937 to 4 988 | 0 | none |
| Seed 24, eighth, assignment | Outside the readouts | 12 | 2 361 | 1 446 to 1 542 | 102 335 to 121 559 | −160 822 to −134 078 | 4 952 to 4 986 | 0 | none |
| Seed 22, first mapping, mirrored | Own group | 12 | 7 030 | 4 066 to 4 284 | 233 551 to 286 504 | −373 962 to −320 382 | 4 943 to 4 988 | 3 | none |
| Seed 22, first mapping, mirrored | Inhibitory units | 12 | 4 534 | 2 696 to 2 877 | 113 101 to 142 404 | −247 278 to −166 820 | 4 951 to 4 989 | 3 | none |
| Seed 22, first mapping, mirrored | Outside the readouts | 12 | 2 339 | 1 387 to 1 449 | 91 813 to 138 065 | −143 204 to −103 336 | 4 941 to 4 986 | 3 | none |
| Seed 22, eighth, assignment | Own group | 12 | 7 030 | 4 157 to 4 394 | 239 681 to 306 529 | −453 932 to −354 180 | 4 948 to 4 988 | 3 | none |
| Seed 22, eighth, assignment | Inhibitory units | 12 | 4 534 | 2 791 to 2 949 | 130 361 to 170 525 | −257 644 to −216 316 | 4 952 to 4 986 | 3 | none |
| Seed 22, eighth, assignment | Outside the readouts | 12 | 2 339 | 1 404 to 1 474 | 102 332 to 131 946 | −161 412 to −110 992 | 4 949 to 4 989 | 3 | none |

Each other part's synapses that the night moved, and those of them that carried a trace that is not zero at its last tick:

| Cell | Form | Onto the own group: synapses moved at the night's last tick, of; with a trace | Onto the inhibitory units | Onto the units outside the readouts | Held across |
| :--- | :--- | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | Own group | held | 4 400 to 4 527 of 4 534; 2 355 to 2 684 | 2 320 to 2 337 of 2 339; 1 182 to 1 316 | 7 240 to 7 331 of 7 345; 3 090 to 3 555 |
| Seed 22, eighth, mirrored | Inhibitory units | 6 858 to 7 016 of 7 030; 3 582 to 3 963 | held | 2 316 to 2 339 of 2 339; 1 169 to 1 287 | 7 220 to 7 333 of 7 345; 3 071 to 3 625 |
| Seed 22, eighth, mirrored | Outside the readouts | 6 867 to 7 016 of 7 030; 3 575 to 3 930 | 4 411 to 4 528 of 4 534; 2 334 to 2 665 | held | 7 217 to 7 332 of 7 345; 3 101 to 3 605 |
| Seed 22, first mapping, assignment | Own group | held | 4 443 to 4 520 of 4 534; 2 213 to 2 410 | 2 310 to 2 338 of 2 339; 1 128 to 1 258 | 7 245 to 7 333 of 7 345; 2 851 to 3 156 |
| Seed 22, first mapping, assignment | Inhibitory units | 6 852 to 7 009 of 7 030; 3 351 to 3 662 | held | 2 318 to 2 338 of 2 339; 1 101 to 1 227 | 7 249 to 7 334 of 7 345; 2 849 to 3 214 |
| Seed 22, first mapping, assignment | Outside the readouts | 6 850 to 7 011 of 7 030; 3 374 to 3 658 | 4 438 to 4 516 of 4 534; 2 225 to 2 410 | held | 7 246 to 7 335 of 7 345; 2 893 to 3 169 |
| Seed 23, eighth, mirrored | Own group | held | 4 395 to 4 484 of 4 487; 2 327 to 2 542 | 2 345 to 2 363 of 2 366; 1 154 to 1 313 | 7 380 to 7 465 of 7 476; 3 103 to 3 639 |
| Seed 23, eighth, mirrored | Inhibitory units | 6 751 to 6 910 of 6 919; 3 607 to 4 049 | held | 2 345 to 2 363 of 2 366; 1 170 to 1 308 | 7 362 to 7 463 of 7 476; 3 158 to 3 706 |
| Seed 23, eighth, mirrored | Outside the readouts | 6 752 to 6 913 of 6 919; 3 593 to 4 025 | 4 388 to 4 484 of 4 487; 2 316 to 2 568 | held | 7 359 to 7 466 of 7 476; 3 205 to 3 716 |
| Seed 23, first mapping, assignment | Own group | held | 4 408 to 4 481 of 4 487; 2 157 to 2 448 | 2 342 to 2 365 of 2 366; 1 196 to 1 276 | 7 384 to 7 462 of 7 476; 2 944 to 3 341 |
| Seed 23, first mapping, assignment | Inhibitory units | 6 731 to 6 901 of 6 919; 3 568 to 3 798 | held | 2 341 to 2 364 of 2 366; 1 166 to 1 295 | 7 388 to 7 463 of 7 476; 2 963 to 3 321 |
| Seed 23, first mapping, assignment | Outside the readouts | 6 730 to 6 903 of 6 919; 3 522 to 3 786 | 4 415 to 4 481 of 4 487; 2 201 to 2 470 | held | 7 387 to 7 463 of 7 476; 2 959 to 3 306 |
| Seed 24, eighth, mirrored | Own group | held | 4 439 to 4 536 of 4 543; 2 326 to 2 564 | 2 336 to 2 360 of 2 361; 1 132 to 1 275 | 7 336 to 7 439 of 7 453; 3 252 to 3 420 |
| Seed 24, eighth, mirrored | Inhibitory units | 6 700 to 6 885 of 6 891; 3 421 to 3 699 | held | 2 336 to 2 359 of 2 361; 1 177 to 1 277 | 7 324 to 7 443 of 7 453; 3 230 to 3 386 |
| Seed 24, eighth, mirrored | Outside the readouts | 6 699 to 6 886 of 6 891; 3 435 to 3 681 | 4 442 to 4 542 of 4 543; 2 284 to 2 561 | held | 7 341 to 7 442 of 7 453; 3 240 to 3 402 |
| Seed 24, first mapping, assignment | Own group | held | 4 481 to 4 532 of 4 543; 2 248 to 2 449 | 2 347 to 2 361 of 2 361; 1 168 to 1 266 | 7 333 to 7 442 of 7 453; 2 925 to 3 425 |
| Seed 24, first mapping, assignment | Inhibitory units | 6 684 to 6 875 of 6 891; 3 308 to 3 808 | held | 2 343 to 2 359 of 2 361; 1 147 to 1 264 | 7 315 to 7 439 of 7 453; 2 936 to 3 370 |
| Seed 24, first mapping, assignment | Outside the readouts | 6 690 to 6 875 of 6 891; 3 351 to 3 849 | 4 467 to 4 528 of 4 543; 2 221 to 2 459 | held | 7 320 to 7 439 of 7 453; 2 941 to 3 414 |
| Seed 23, eighth, assignment | Own group | held | 4 398 to 4 483 of 4 487; 2 308 to 2 617 | 2 340 to 2 363 of 2 366; 1 090 to 1 260 | 7 390 to 7 461 of 7 476; 3 139 to 3 606 |
| Seed 23, eighth, assignment | Inhibitory units | 6 706 to 6 916 of 6 919; 3 112 to 3 621 | held | 2 339 to 2 364 of 2 366; 1 093 to 1 257 | 7 380 to 7 463 of 7 476; 3 087 to 3 599 |
| Seed 23, eighth, assignment | Outside the readouts | 6 724 to 6 913 of 6 919; 3 089 to 3 598 | 4 388 to 4 487 of 4 487; 2 289 to 2 635 | held | 7 394 to 7 462 of 7 476; 3 179 to 3 546 |
| Seed 24, first mapping, mirrored | Own group | held | 4 436 to 4 530 of 4 543; 2 127 to 2 396 | 2 336 to 2 360 of 2 361; 1 111 to 1 292 | 7 370 to 7 446 of 7 453; 2 957 to 3 344 |
| Seed 24, first mapping, mirrored | Inhibitory units | 6 691 to 6 880 of 6 891; 3 088 to 3 354 | held | 2 332 to 2 361 of 2 361; 1 136 to 1 288 | 7 379 to 7 442 of 7 453; 3 012 to 3 319 |
| Seed 24, first mapping, mirrored | Outside the readouts | 6 700 to 6 881 of 6 891; 3 035 to 3 353 | 4 432 to 4 525 of 4 543; 2 144 to 2 414 | held | 7 377 to 7 444 of 7 453; 2 973 to 3 324 |
| Seed 23, first mapping, mirrored | Own group | held | 4 411 to 4 483 of 4 487; 2 174 to 2 445 | 2 350 to 2 364 of 2 366; 1 090 to 1 229 | 7 421 to 7 459 of 7 476; 3 016 to 3 448 |
| Seed 23, first mapping, mirrored | Inhibitory units | 6 729 to 6 907 of 6 919; 3 122 to 3 427 | held | 2 351 to 2 362 of 2 366; 1 087 to 1 242 | 7 411 to 7 461 of 7 476; 3 017 to 3 433 |
| Seed 23, first mapping, mirrored | Outside the readouts | 6 723 to 6 904 of 6 919; 3 111 to 3 442 | 4 418 to 4 482 of 4 487; 2 215 to 2 455 | held | 7 425 to 7 459 of 7 476; 3 012 to 3 411 |
| Seed 24, eighth, assignment | Own group | held | 4 440 to 4 536 of 4 543; 2 232 to 2 596 | 2 336 to 2 358 of 2 361; 1 176 to 1 294 | 7 359 to 7 441 of 7 453; 3 160 to 3 615 |
| Seed 24, eighth, assignment | Inhibitory units | 6 712 to 6 879 of 6 891; 3 202 to 3 572 | held | 2 336 to 2 359 of 2 361; 1 162 to 1 284 | 7 359 to 7 444 of 7 453; 3 096 to 3 606 |
| Seed 24, eighth, assignment | Outside the readouts | 6 709 to 6 875 of 6 891; 3 219 to 3 566 | 4 428 to 4 532 of 4 543; 2 300 to 2 564 | held | 7 355 to 7 443 of 7 453; 3 114 to 3 600 |
| Seed 22, first mapping, mirrored | Own group | held | 4 458 to 4 527 of 4 534; 2 175 to 2 370 | 2 312 to 2 336 of 2 339; 1 068 to 1 197 | 7 278 to 7 330 of 7 345; 2 763 to 3 146 |
| Seed 22, first mapping, mirrored | Inhibitory units | 6 841 to 7 017 of 7 030; 3 047 to 3 421 | held | 2 315 to 2 338 of 2 339; 1 079 to 1 193 | 7 277 to 7 329 of 7 345; 2 780 to 3 121 |
| Seed 22, first mapping, mirrored | Outside the readouts | 6 835 to 7 014 of 7 030; 3 077 to 3 430 | 4 453 to 4 526 of 4 534; 2 178 to 2 367 | held | 7 288 to 7 330 of 7 345; 2 804 to 3 122 |
| Seed 22, eighth, assignment | Own group | held | 4 430 to 4 526 of 4 534; 2 312 to 2 538 | 2 314 to 2 338 of 2 339; 1 114 to 1 201 | 7 249 to 7 335 of 7 345; 2 949 to 3 419 |
| Seed 22, eighth, assignment | Inhibitory units | 6 786 to 7 018 of 7 030; 3 189 to 3 526 | held | 2 309 to 2 337 of 2 339; 1 125 to 1 205 | 7 252 to 7 336 of 7 345; 2 887 to 3 395 |
| Seed 22, eighth, assignment | Outside the readouts | 6 778 to 7 018 of 7 030; 3 211 to 3 560 | 4 425 to 4 523 of 4 534; 2 297 to 2 553 | held | 7 246 to 7 335 of 7 345; 2 974 to 3 445 |

- **Before the night's first tick**, in the nights that hold it, 3 728 to 4 355 of the 6 891 to 7 030 synapses onto the own group carried a
  trace that was not zero, 2 612 to 3 005 of the 4 487 to 4 543 onto the inhibitory units and 1 569 to 1 807 of the 2 339 to 2 366 onto the
  units outside the readouts.
- **At the night's last tick the hold cleared** 4 066 to 4 611 traces that were not zero onto the own group, 233 551 to 443 684 LSB above
  zero and −491 859 to −320 382 below; 2 696 to 3 109 onto the inhibitory units, 107 368 to 180 438 above and −299 987 to −166 820 below;
  and 1 387 to 1 551 onto the units outside the readouts, 81 705 to 146 019 above and −187 168 to −94 452 below.
- **The other parts moved** in every held night. The synapses of the held units outside the held part moved: 13 966 to 14 332 of them under
  the night held onto the own group, 6 087 to 7 529 with a trace at the night's last tick; 16 348 to 16 731 under the night held onto the
  inhibitory units, 6 920 to 9 063 with a trace; and 18 475 to 18 875 under the night held onto the units outside the readouts, 8 064 to
  10 295 with a trace. The excitatory synapses from units outside the hold moved: 4 940 to 4 991, 4 937 to 4 990 and 4 939 to 4 991.
- **A held synapse's presynaptic unit fired at or after the night's last tick** in 13 of the 144 nights held onto the own group: 4 on seed
  23's eighth image from the mirrored assignment and 3 on each of seed 23's eighth image from the assignment, seed 22's first mapping from
  the mirrored assignment and seed 22's eighth image from the assignment. In 16 of the 144 held onto the inhibitory units: 3 on each of seed
  23's eighth image from the mirrored assignment, seed 23's eighth image from the assignment, seed 23's first mapping from the mirrored
  assignment, seed 22's first mapping from the mirrored assignment and seed 22's eighth image from the assignment, and 1 on seed 22's first
  mapping from the assignment. In 20 of the 144 held onto the units outside the readouts: 4 on seed 23's eighth image from the mirrored
  assignment, 3 on each of seed 23's first mapping from the assignment, seed 23's eighth image from the assignment, seed 23's first mapping
  from the mirrored assignment, seed 22's first mapping from the mirrored assignment and seed 22's eighth image from the assignment, and 1
  on seed 22's first mapping from the assignment.
- **The quiet ticks moved no synapse of the held part in any of the 432 held nights**, and no synapse of any other part either. After them,
  in the nights that hold it, 0 to 6 synapses onto the own group carried a trace, 0 to 6 onto the inhibitory units and 0 to 4 onto the units
  outside the readouts. Of each other part, as many synapses carried a trace after the quiet ticks as at the night's last tick in 130 to
  141 of the 144 nights of each held form, part by part. The jobs print these counts, not the traces themselves.

The night's pairings still to be read on the held part's synapses, as What ran defines them, each read at the synapse's presynaptic unit's
next spike if that spike comes before the target's next spike:

| Cell | Form | Pending gains: held synapses | their sum, LSB | their largest | Pending losses with a presynaptic spike at the next tick: held synapses | their sum, LSB | their largest |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | Own group | 3 490 to 3 595 | 156 806 to 213 978 | 328 | 7 030 | 15 893 to 22 425 | 98 to 124 |
| Seed 22, eighth, mirrored | Inhibitory units | 2 607 to 2 770 | 165 678 to 232 330 | 326 to 328 | 4 534 | 14 154 to 21 684 | 89 to 126 |
| Seed 22, eighth, mirrored | Outside the readouts | 988 to 1 174 | 44 701 to 66 508 | 328 | 2 339 | 4 049 to 7 996 | 51 to 89 |
| Seed 22, first mapping, assignment | Own group | 3 458 to 3 639 | 144 765 to 175 858 | 328 | 7 030 | 16 708 to 22 238 | 82 to 109 |
| Seed 22, first mapping, assignment | Inhibitory units | 2 497 to 2 807 | 159 093 to 179 787 | 327 to 328 | 4 534 | 10 051 to 19 081 | 53 to 150 |
| Seed 22, first mapping, assignment | Outside the readouts | 1 043 to 1 253 | 42 681 to 73 516 | 328 | 2 339 | 2 860 to 8 897 | 50 to 102 |
| Seed 23, eighth, mirrored | Own group | 3 431 to 3 550 | 180 633 to 216 273 | 328 | 6 919 | 16 228 to 25 473 | 105 to 116 |
| Seed 23, eighth, mirrored | Inhibitory units | 2 372 to 2 625 | 167 658 to 191 668 | 327 to 328 | 4 487 | 10 454 to 20 016 | 85 to 111 |
| Seed 23, eighth, mirrored | Outside the readouts | 998 to 1 221 | 48 067 to 72 352 | 328 | 2 366 | 5 632 to 10 702 | 54 to 107 |
| Seed 23, first mapping, assignment | Own group | 3 378 to 3 513 | 156 122 to 214 564 | 328 | 6 919 | 15 743 to 21 180 | 94 to 115 |
| Seed 23, first mapping, assignment | Inhibitory units | 2 402 to 2 649 | 142 346 to 194 059 | 327 to 328 | 4 487 | 12 070 to 19 496 | 96 to 108 |
| Seed 23, first mapping, assignment | Outside the readouts | 1 089 to 1 217 | 52 685 to 78 943 | 328 | 2 366 | 4 998 to 10 237 | 73 to 115 |
| Seed 24, eighth, mirrored | Own group | 3 377 to 3 531 | 167 800 to 210 282 | 328 | 6 891 | 20 405 to 28 160 | 104 to 130 |
| Seed 24, eighth, mirrored | Inhibitory units | 2 503 to 2 661 | 181 824 to 214 581 | 326 to 328 | 4 543 | 12 026 to 22 578 | 77 to 143 |
| Seed 24, eighth, mirrored | Outside the readouts | 1 060 to 1 159 | 58 981 to 85 413 | 328 | 2 361 | 3 979 to 9 504 | 48 to 126 |
| Seed 24, first mapping, assignment | Own group | 3 326 to 3 475 | 135 419 to 189 330 | 328 | 6 891 | 15 627 to 20 824 | 90 to 132 |
| Seed 24, first mapping, assignment | Inhibitory units | 2 501 to 2 762 | 164 892 to 203 624 | 327 to 328 | 4 543 | 9 952 to 19 883 | 76 to 96 |
| Seed 24, first mapping, assignment | Outside the readouts | 1 049 to 1 170 | 43 884 to 83 056 | 328 | 2 361 | 6 330 to 11 176 | 75 to 125 |
| Seed 23, eighth, assignment | Own group | 3 454 to 3 589 | 161 831 to 195 219 | 328 | 6 919 | 18 594 to 25 815 | 78 to 149 |
| Seed 23, eighth, assignment | Inhibitory units | 2 526 to 2 761 | 159 137 to 189 317 | 327 to 328 | 4 487 | 11 290 to 20 524 | 86 to 119 |
| Seed 23, eighth, assignment | Outside the readouts | 1 066 to 1 199 | 46 534 to 67 929 | 328 | 2 366 | 4 320 to 7 360 | 51 to 72 |
| Seed 24, first mapping, mirrored | Own group | 3 381 to 3 463 | 123 075 to 155 187 | 328 | 6 891 | 14 663 to 25 140 | 86 to 132 |
| Seed 24, first mapping, mirrored | Inhibitory units | 2 570 to 2 717 | 138 636 to 175 734 | 326 to 328 | 4 543 | 8 344 to 16 717 | 73 to 108 |
| Seed 24, first mapping, mirrored | Outside the readouts | 1 175 to 1 292 | 43 147 to 69 773 | 328 | 2 361 | 4 656 to 15 137 | 62 to 125 |
| Seed 23, first mapping, mirrored | Own group | 3 454 to 3 547 | 141 868 to 162 065 | 328 | 6 919 | 15 644 to 23 187 | 87 to 114 |
| Seed 23, first mapping, mirrored | Inhibitory units | 2 404 to 2 756 | 116 943 to 178 296 | 326 to 328 | 4 487 | 11 612 to 19 533 | 94 to 116 |
| Seed 23, first mapping, mirrored | Outside the readouts | 1 080 to 1 255 | 48 048 to 68 632 | 328 | 2 366 | 5 094 to 12 090 | 73 to 121 |
| Seed 24, eighth, assignment | Own group | 3 408 to 3 541 | 136 322 to 166 770 | 328 | 6 891 | 13 518 to 24 088 | 99 to 131 |
| Seed 24, eighth, assignment | Inhibitory units | 2 568 to 2 731 | 159 133 to 182 648 | 327 to 328 | 4 543 | 11 537 to 22 842 | 86 to 142 |
| Seed 24, eighth, assignment | Outside the readouts | 1 087 to 1 190 | 33 831 to 75 115 | 328 | 2 361 | 4 899 to 7 752 | 49 to 85 |
| Seed 22, first mapping, mirrored | Own group | 3 546 to 3 651 | 135 343 to 149 720 | 328 | 7 030 | 18 617 to 26 329 | 82 to 110 |
| Seed 22, first mapping, mirrored | Inhibitory units | 2 696 to 2 923 | 139 446 to 172 626 | 327 to 328 | 4 534 | 12 554 to 29 916 | 50 to 149 |
| Seed 22, first mapping, mirrored | Outside the readouts | 1 090 to 1 253 | 41 598 to 69 282 | 328 | 2 339 | 3 464 to 8 367 | 50 to 105 |
| Seed 22, eighth, assignment | Own group | 3 523 to 3 624 | 144 565 to 163 469 | 328 | 7 030 | 15 570 to 22 126 | 104 to 129 |
| Seed 22, eighth, assignment | Inhibitory units | 2 627 to 2 796 | 147 902 to 184 255 | 325 to 328 | 4 534 | 13 792 to 22 471 | 90 to 120 |
| Seed 22, eighth, assignment | Outside the readouts | 1 020 to 1 176 | 37 634 to 57 030 | 328 | 2 339 | 4 347 to 11 417 | 58 to 108 |

- **The pending gain**, the target's spike after the presynaptic unit's last, is on 3 326 to 3 651 synapses onto the own group, 123 075 to
  216 273 LSB together, the largest 328 LSB, which is $A_+$; on 2 372 to 2 923 onto the inhibitory units, 116 943 to 232 330 LSB together,
  the largest 325 to 328 LSB; and on 988 to 1 292 onto the units outside the readouts, 33 831 to 85 413 LSB together, the largest 328 LSB.
- **The pending loss** with a presynaptic spike at the first tick after the image is on every synapse of the held part: 13 518 to 28 160
  LSB together onto the own group, the largest 78 to 149 LSB; 8 344 to 29 916 LSB onto the inhibitory units, the largest 50 to 150; and
  2 860 to 15 137 LSB onto the units outside the readouts, the largest 48 to 126. A later spike, before the target's next one, reads no more.
- **Against what the hold cleared.** These pairings were never in the trace the hold cleared, which held, synapse by synapse, the decayed
  net of what the trace carried into the night and the pairings read at the night's presynaptic spikes. Set beside it as sizes alone: the
  pending gains are 37.7 % to 71.0 % the size of the traces cleared above zero onto the own group, 87.1 % to 163.7 % onto the inhibitory
  units and 29.7 % to 81.2 % onto the units outside the readouts; the most the pending losses can be is 3.4 % to 8.2 %, 3.9 % to 16.8 % and
  2.4 % to 9.9 % the size of the traces cleared below zero.
- These are trace, not weight: at the next presynaptic spike, in the next day's engine, each synapse's trace takes its pairings, and its
  consolidation moves the weight under that day's modulation. That consolidation is not read here.

## 6. The twelve cells at block 36

H-47's twelve cells at block 36, the rows ordered by the readout A answers, the six where it answers readout 0 first. Every held form is
this exploration's job; exploration 027's night held within is exploration 027's job; the excitatory night is exploration 026's job for its
eight cells and exploration 025's for exploration 020's four; H-47's plain night is H-47's job's in all twelve. "Less without replay" and
the held forms less each other carry the paired SE of this job, and "less H-47's plain", "less 027's held within" and "less excitatory"
exploration 018's unpaired SE. No figure is pooled across cells:

| Cell | A answers | Own group less without replay | Inhibitory units less without replay | Outside the readouts less without replay | H-47: plain less without replay | 027: held within less without replay | Excitatory less without replay |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | +16.8 ± 0.88 | +16.1 ± 1.18 | +20.8 ± 0.64 | +19.6 ± 1.06 | +16.8 ± 0.83 | +16.3 ± 0.84 |
| Seed 22, first mapping, assignment | readout 0, the odd places | +10.2 ± 1.20 | +6.0 ± 0.78 | +8.3 ± 0.29 | +7.9 ± 0.51 | +6.5 ± 1.42 | +5.2 ± 1.24 |
| Seed 23, eighth, mirrored | readout 0, the odd places | +17.0 ± 0.92 | +15.3 ± 0.91 | +17.0 ± 0.69 | +17.4 ± 1.08 | +15.5 ± 1.06 | +14.7 ± 1.54 |
| Seed 23, first mapping, assignment | readout 0, the odd places | +12.7 ± 1.26 | +12.6 ± 1.64 | +12.9 ± 1.65 | +13.4 ± 1.56 | +13.3 ± 1.64 | +12.6 ± 1.56 |
| Seed 24, eighth, mirrored | readout 0, the odd places | +11.5 ± 1.28 | +12.9 ± 1.03 | +12.0 ± 1.87 | +11.3 ± 0.96 | +15.5 ± 1.27 | +16.7 ± 1.23 |
| Seed 24, first mapping, assignment | readout 0, the odd places | +4.4 ± 1.15 | +6.5 ± 1.02 | +4.8 ± 1.56 | +3.8 ± 1.57 | +6.1 ± 1.46 | +5.8 ± 1.60 |
| Seed 23, eighth, assignment | readout 1, the even places | +4.5 ± 2.81 | +6.7 ± 2.53 | +6.5 ± 1.93 | +4.1 ± 1.45 | +10.2 ± 1.77 | +10.9 ± 2.49 |
| Seed 24, first mapping, mirrored | readout 1, the even places | +7.0 ± 1.77 | +6.7 ± 1.57 | +7.0 ± 1.46 | +6.0 ± 1.54 | +7.7 ± 1.37 | +7.4 ± 1.09 |
| Seed 23, first mapping, mirrored | readout 1, the even places | −4.6 ± 0.95 | −3.0 ± 0.78 | −4.7 ± 0.66 | −4.9 ± 1.09 | −2.6 ± 0.65 | −1.7 ± 0.42 |
| Seed 24, eighth, assignment | readout 1, the even places | +0.7 ± 1.29 | +1.4 ± 0.82 | +3.5 ± 2.31 | +1.8 ± 1.79 | +4.1 ± 1.17 | +8.8 ± 1.28 |
| Seed 22, first mapping, mirrored | readout 1, the even places | +5.4 ± 1.25 | +4.8 ± 1.23 | +6.0 ± 1.15 | +5.0 ± 1.24 | +5.6 ± 1.68 | +4.7 ± 1.52 |
| Seed 22, eighth, assignment | readout 1, the even places | +1.5 ± 1.08 | +3.3 ± 0.36 | +4.2 ± 0.84 | +2.2 ± 0.65 | +4.5 ± 1.26 | +4.6 ± 0.66 |

| Cell | A answers | Own group less inhibitory units | Own group less outside the readouts | Inhibitory units less outside the readouts |
| :--- | :--- | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | +0.7 ± 1.50 | −3.9 ± 0.30 | −4.7 ± 1.27 |
| Seed 22, first mapping, assignment | readout 0, the odd places | +4.2 ± 0.93 | +2.0 ± 0.85 | −2.2 ± 0.31 |
| Seed 23, eighth, mirrored | readout 0, the odd places | +1.8 ± 0.30 | +0.1 ± 0.46 | −1.7 ± 0.20 |
| Seed 23, first mapping, assignment | readout 0, the odd places | 0.0 ± 0.80 | −0.2 ± 0.44 | −0.3 ± 0.60 |
| Seed 24, eighth, mirrored | readout 0, the odd places | −1.4 ± 0.76 | −0.5 ± 0.75 | +0.9 ± 1.18 |
| Seed 24, first mapping, assignment | readout 0, the odd places | −2.1 ± 0.48 | −0.4 ± 0.44 | +1.7 ± 0.74 |
| Seed 23, eighth, assignment | readout 1, the even places | −2.3 ± 0.44 | −2.0 ± 1.18 | +0.3 ± 0.78 |
| Seed 24, first mapping, mirrored | readout 1, the even places | +0.3 ± 0.68 | 0.0 ± 0.75 | −0.3 ± 0.31 |
| Seed 23, first mapping, mirrored | readout 1, the even places | −1.5 ± 0.93 | +0.1 ± 0.92 | +1.6 ± 0.11 |
| Seed 24, eighth, assignment | readout 1, the even places | −0.7 ± 0.93 | −2.8 ± 1.11 | −2.1 ± 1.73 |
| Seed 22, first mapping, mirrored | readout 1, the even places | +0.6 ± 0.45 | −0.6 ± 0.46 | −1.2 ± 0.68 |
| Seed 22, eighth, assignment | readout 1, the even places | −1.8 ± 0.95 | −2.6 ± 0.60 | −0.9 ± 0.60 |

| Cell | A answers | Own group less H-47's plain | less 027's held within | less excitatory | Inhibitory units less H-47's plain | less 027's held within | less excitatory | Outside the readouts less H-47's plain | less 027's held within | less excitatory |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | −2.8 ± 0.53 | +0.1 ± 0.88 | +0.6 ± 1.51 | −3.6 ± 1.52 | −0.7 ± 1.38 | −0.2 ± 1.20 | +1.1 ± 0.49 | +4.0 ± 0.72 | +4.5 ± 1.23 |
| Seed 22, first mapping, assignment | readout 0, the odd places | +2.3 ± 0.90 | +3.7 ± 1.75 | +5.0 ± 1.15 | −1.9 ± 0.70 | −0.5 ± 1.16 | +0.8 ± 0.88 | +0.3 ± 0.22 | +1.8 ± 1.30 | +3.0 ± 0.92 |
| Seed 23, eighth, mirrored | readout 0, the odd places | −0.4 ± 1.06 | +1.5 ± 0.96 | +2.3 ± 1.20 | −2.1 ± 1.10 | −0.3 ± 0.90 | +0.6 ± 1.28 | −0.4 ± 1.12 | +1.4 ± 0.96 | +2.3 ± 1.38 |
| Seed 23, first mapping, assignment | readout 0, the odd places | −0.7 ± 0.90 | −0.6 ± 0.99 | +0.1 ± 1.12 | −0.7 ± 1.00 | −0.6 ± 1.02 | +0.1 ± 0.91 | −0.4 ± 0.81 | −0.4 ± 0.85 | +0.3 ± 0.98 |
| Seed 24, eighth, mirrored | readout 0, the odd places | +0.2 ± 1.41 | −4.0 ± 0.99 | −5.2 ± 0.77 | +1.6 ± 0.99 | −2.6 ± 1.34 | −3.8 ± 1.27 | +0.7 ± 1.85 | −3.6 ± 1.44 | −4.7 ± 1.24 |
| Seed 24, first mapping, assignment | readout 0, the odd places | +0.7 ± 0.76 | −1.7 ± 1.30 | −1.3 ± 0.93 | +2.8 ± 0.94 | +0.4 ± 1.32 | +0.8 ± 1.09 | +1.1 ± 0.64 | −1.3 ± 1.22 | −0.9 ± 0.72 |
| Seed 23, eighth, assignment | readout 1, the even places | +0.4 ± 2.07 | −5.7 ± 1.51 | −6.5 ± 1.22 | +2.7 ± 1.66 | −3.5 ± 1.21 | −4.2 ± 0.98 | +2.4 ± 0.98 | −3.7 ± 0.90 | −4.5 ± 1.08 |
| Seed 24, first mapping, mirrored | readout 1, the even places | +1.1 ± 0.70 | −0.6 ± 0.82 | −0.4 ± 0.89 | +0.8 ± 0.84 | −1.0 ± 1.07 | −0.7 ± 0.95 | +1.0 ± 0.96 | −0.7 ± 1.11 | −0.4 ± 0.94 |
| Seed 23, first mapping, mirrored | readout 1, the even places | +0.3 ± 0.96 | −1.9 ± 0.80 | −2.9 ± 0.85 | +1.9 ± 0.92 | −0.4 ± 1.12 | −1.3 ± 0.98 | +0.2 ± 0.96 | −2.0 ± 1.07 | −3.0 ± 0.93 |
| Seed 24, eighth, assignment | readout 1, the even places | −1.1 ± 1.32 | −3.4 ± 1.13 | −8.1 ± 1.21 | −0.4 ± 1.32 | −2.7 ± 1.14 | −7.4 ± 0.94 | +1.7 ± 1.63 | −0.6 ± 1.74 | −5.3 ± 1.93 |
| Seed 22, first mapping, mirrored | readout 1, the even places | +0.4 ± 0.72 | −0.2 ± 0.65 | +0.8 ± 0.70 | −0.1 ± 1.10 | −0.8 ± 0.92 | +0.2 ± 0.87 | +1.0 ± 0.40 | +0.4 ± 0.95 | +1.4 ± 1.04 |
| Seed 22, eighth, assignment | readout 1, the even places | −0.7 ± 1.79 | −3.0 ± 2.24 | −3.1 ± 1.81 | +1.1 ± 1.33 | −1.2 ± 1.54 | −1.4 ± 1.25 | +2.0 ± 1.55 | −0.3 ± 1.96 | −0.5 ± 1.56 |

| Cell | A answers | Own group: side | Inhibitory units: side | Outside the readouts: side | H-47's side | Held within (027): side | Excitatory: side |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Seed 22, eighth, mirrored | readout 0, the odd places | B, the even places (within 2 SE) | A, the odd places | B, the even places | B, the even places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places |
| Seed 22, first mapping, assignment | readout 0, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | readout 0, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | readout 0, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 24, eighth, mirrored | readout 0, the odd places | B, the even places (within 2 SE) | B, the even places | B, the even places | B, the even places | B, the even places (within 2 SE) | B, the even places (within 2 SE) |
| Seed 24, first mapping, assignment | readout 0, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places (within 2 SE) | A, the odd places | A, the odd places |
| Seed 23, eighth, assignment | readout 1, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places | A, the even places |
| Seed 24, first mapping, mirrored | readout 1, the even places | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | readout 1, the even places | B, the odd places (within 2 SE) | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 24, eighth, assignment | readout 1, the even places | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | readout 1, the even places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | readout 1, the even places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |

- **Less the night without replay** the night held onto the own group lies beyond 2 SE above in eight of the twelve cells, below in one,
  seed 23's first mapping from the mirrored assignment, −4.6 ± 0.95, and within 2 SE in three, seed 23's eighth image from the assignment,
  +4.5 ± 2.81, seed 24's eighth image from the assignment, +0.7 ± 1.29, and seed 22's eighth image from the assignment, +1.5 ± 1.08. The
  night held onto the inhibitory units lies above in ten, below in one, seed 23's first mapping from the mirrored assignment, −3.0 ± 0.78,
  and within 2 SE on seed 24's eighth image from the assignment, +1.4 ± 0.82. The night held onto the units outside the readouts lies above
  in ten, below in one, seed 23's first mapping from the mirrored assignment, −4.7 ± 0.66, and within 2 SE on seed 24's eighth image from
  the assignment, +3.5 ± 2.31. H-47's clause 1 read −4.9 ± 1.09 on seed 23's first mapping from the mirrored assignment and +1.8 ± 1.79
  on seed 24's eighth image from the assignment.
- **The held nights less each other.** The night held onto the own group less the night held onto the inhibitory units lies beyond 2 SE in
  four cells: above on seed 22's first mapping from the assignment, +4.2 ± 0.93, and seed 23's eighth image from the mirrored assignment,
  +1.8 ± 0.30, both where A answers readout 0; below on seed 24's first mapping from the assignment, −2.1 ± 0.48, where A answers readout 0,
  and seed 23's eighth image from the assignment, −2.3 ± 0.44, where it answers readout 1. Less the night held onto the units outside the
  readouts it lies beyond in four: above on seed 22's first mapping from the assignment, +2.0 ± 0.85; below on seed 22's eighth image from
  the mirrored assignment, −3.9 ± 0.30, where A answers readout 0, and seed 24's eighth image from the assignment, −2.8 ± 1.11, and seed
  22's eighth image from the assignment, −2.6 ± 0.60, where it answers readout 1. The night held onto the inhibitory units less the night
  held onto the units outside the readouts lies beyond in five: above on seed 24's first mapping from the assignment, +1.7 ± 0.74, where A
  answers readout 0, and seed 23's first mapping from the mirrored assignment, +1.6 ± 0.11, where it answers readout 1; below on seed 22's
  eighth image from the mirrored assignment, −4.7 ± 1.27, seed 22's first mapping from the assignment, −2.2 ± 0.31, and seed 23's eighth
  image from the mirrored assignment, −1.7 ± 0.20, all where A answers readout 0.
- **Less H-47's plain night** the night held onto the own group lies beyond 2 SE in two cells, above on seed 22's first mapping from the
  assignment, +2.3 ± 0.90, and below on seed 22's eighth image from the mirrored assignment, −2.8 ± 0.53, both where A answers readout 0.
  The night held onto the inhibitory units lies beyond in four: above on seed 24's first mapping from the assignment, +2.8 ± 0.94, where A
  answers readout 0, and seed 23's first mapping from the mirrored assignment, +1.9 ± 0.92, where it answers readout 1; below on seed 22's
  eighth image from the mirrored assignment, −3.6 ± 1.52, and seed 22's first mapping from the assignment, −1.9 ± 0.70, where A answers
  readout 0. The night held onto the units outside the readouts lies beyond in three, all above: seed 22's eighth image from the mirrored
  assignment, +1.1 ± 0.49, where A answers readout 0, and seed 23's eighth image from the assignment, +2.4 ± 0.98, and seed 22's first
  mapping from the mirrored assignment, +1.0 ± 0.40, where it answers readout 1.
- **Less exploration 027's night held within** the night held onto the own group lies beyond 2 SE in five cells: above on seed 22's first
  mapping from the assignment, +3.7 ± 1.75; below on seed 24's eighth image from the mirrored assignment, −4.0 ± 0.99, where A answers
  readout 0, and seed 23's eighth image from the assignment, −5.7 ± 1.51, seed 23's first mapping from the mirrored assignment,
  −1.9 ± 0.80, and seed 24's eighth image from the assignment, −3.4 ± 1.13, where it answers readout 1. The night held onto the inhibitory
  units lies beyond in two, both below and where A answers readout 1: seed 23's eighth image from the assignment, −3.5 ± 1.21, and seed
  24's eighth image from the assignment, −2.7 ± 1.14. The night held onto the units outside the readouts lies beyond in three: above on seed
  22's eighth image from the mirrored assignment, +4.0 ± 0.72; below on seed 24's eighth image from the mirrored assignment, −3.6 ± 1.44,
  both where A answers readout 0, and seed 23's eighth image from the assignment, −3.7 ± 0.90, where it answers readout 1.
- **Less the excitatory night** the night held onto the own group lies beyond 2 SE in five cells: above on seed 22's first mapping from the
  assignment, +5.0 ± 1.15; below on seed 24's eighth image from the mirrored assignment, −5.2 ± 0.77, where A answers readout 0, and seed
  23's eighth image from the assignment, −6.5 ± 1.22, seed 23's first mapping from the mirrored assignment, −2.9 ± 0.85, and seed 24's
  eighth image from the assignment, −8.1 ± 1.21, where it answers readout 1. The night held onto the inhibitory units lies beyond in three,
  all below: seed 24's eighth image from the mirrored assignment, −3.8 ± 1.27, where A answers readout 0, and seed 23's eighth image from
  the assignment, −4.2 ± 0.98, and seed 24's eighth image from the assignment, −7.4 ± 0.94, where it answers readout 1. The night held onto
  the units outside the readouts lies beyond in six: above on seed 22's eighth image from the mirrored assignment, +4.5 ± 1.23, and seed
  22's first mapping from the assignment, +3.0 ± 0.92; below on seed 24's eighth image from the mirrored assignment, −4.7 ± 1.24, where A
  answers readout 0, and seed 23's eighth image from the assignment, −4.5 ± 1.08, seed 23's first mapping from the mirrored assignment,
  −3.0 ± 0.93, and seed 24's eighth image from the assignment, −5.3 ± 1.93, where it answers readout 1.
- **The side** at block 36 under the night held onto the inhibitory units is the excitatory night's in eleven of the twelve cells, all but
  seed 24's eighth image from the assignment, and H-47's in ten, all but seed 22's eighth image from the mirrored assignment and seed 24's
  first mapping from the mirrored assignment. It lies on the odd places in ten: on A in the five where A answers readout 0 but seed 24's
  eighth image from the mirrored assignment, and on B in the five where it answers readout 1 but seed 23's eighth image from the
  assignment; on those two it lies on the even places, on B and on A. Under the night held onto the own group the side is H-47's in eleven,
  all but seed 24's first mapping from the mirrored assignment, and the excitatory night's in ten, all but seed 22's eighth image from the
  mirrored assignment and seed 24's eighth image from the assignment. Under the night held onto the units outside the readouts it is H-47's
  in eleven, all but seed 24's eighth image from the assignment, and the excitatory night's in ten, all but seed 22's eighth image from the
  mirrored assignment and seed 24's first mapping from the mirrored assignment.

## What it costs

The trial took 418 s and the twelve jobs 7 315 to 7 791 s, 121.9 to 129.9 minutes, at most 54.1 % of a job's 240 minutes; ADR-0243 put a
job at about 120 to 135 minutes. Nothing was cut: every cell ran its four forms. Thirteen jobs ran, the trial's and twelve, every one to
the end.

## What it suggests

**In these twelve cells each part of the part held within, held alone, leaves A's count near H-47's plain night.** Less the plain night,
the night held onto the own group lies beyond twice exploration 018's unpaired SE at 3 of the 60 checkpoints, the night held onto the
units outside the readouts at 6 and the night held onto the inhibitory units at 13, where exploration 027's night held within lay beyond
at 17 and the excitatory night at 25. Each part held alone left the other three moving within 0.2 M a group of the plain night's change
over four nights.

**Of the three, the night held onto the inhibitory units lies nearest exploration 027's night held within and the excitatory night.** Of
the 25 checkpoints where the excitatory night lies beyond 2 SE of the plain night, it lies beyond the plain night in the same direction at
12, and 12 of its own 13 are among them; the night held onto the units outside the readouts does so at 4 and the night held onto the own
group at 1. Less exploration 027's night held within it lies beyond 2 SE at 8 of the 60, all below, where the night held onto the units
outside the readouts lies beyond at 13 and the night held onto the own group at 17; less the excitatory night at 11, all below, against 19
and 20. Its side at block 36 is the excitatory night's in eleven of the twelve cells. It is the only one of the three under which the
inhibition onto each group's excitatory units rose by more than under the plain night in every group, by 0.16 to 0.96 M, where exploration
027's night held within's rose by 0.33 to 1.11 M more.

**No part alone gives exploration 027's night held within.** Of the 8 checkpoints where the night held onto the inhibitory units lies
below it beyond 2 SE, 6 are on seed 24's eighth image from the mirrored assignment at blocks 12, 18 and 24 and on seed 23's eighth image
from the assignment from block 18 on, and every held night here lies below the excitatory night beyond 2 SE on seed 24's eighth image from the mirrored assignment and on seed
23's and seed 24's eighth images from the assignment, at block 36 among other checkpoints. Where exploration 027 held the three parts
together, its count lay within 2 SE of the excitatory night's at 56 of the 60 checkpoints.

Questions this suggests for a later exploration or a round:
- Two parts held together, the part onto the inhibitory units with each of the other two, to find whether the count of the night held
  within needs more than one of its parts held.
- What on seed 24's eighth images from both arms and seed 23's eighth image from the assignment keeps every single part's hold below the
  excitatory night at block 36.
- A read of what the next day's first consolidations do with the pending gains and losses.

## What did not run

- **Exploration 024's hold**, alone or with any part: the inhibitory synapses onto the readouts' excitatory units consolidate as in H-47
  under every form here.
- **Any two parts held together**: only the trial holds the three together, for one night, to hold exploration 027's night held within's
  row; no job holds more than one part.
- **The part held across**, alone or with any part: it consolidates as in H-47 under every form here.
- **Exploration 027's night held within again, or the excitatory night again**: both are read from exploration 027's, 025's and 026's
  jobs.
- **Any part held in the night without replay**: the night without replay ran with no hold set.
- **A finer split by target**: the part onto the units outside the readouts holds A's and B's units and the other excitatory units
  together, and section 4 breaks them down without holding one apart.
- **Plain nights on both mappings and the awake gap**, under a hold or not: no reading here stands against them.
- **A hold through the quiet ticks**: the hold ends at the night's last tick, and the image is written after the quiet ticks. Here the
  quiet ticks moved no synapse of the held part in any of the 432 held nights.
- **The excitatory synapses onto the readouts from A's, B's and the pair's own units**, which consolidate as in H-47 under every form, the
  zero sum's rounds with them, and **the inhibitory synapses onto the readouts' inhibitory units**, which no hold here reaches.
- **The next day's reading of the pending pairings**: the next day's consolidation of the held synapses is not read.
- **A diagnosis of seed 24's eighth image from the assignment alone**, or **of seed 23's eighth image from the mirrored assignment
  alone**: each runs here as one cell of twelve.
- **REM's depotentiations and the zero sum's rounds** are not split out of a night's change, as in explorations 020 to 027.
- **Any SE for sections 4 and 5**: the class matrix is a mean over the three paths, and the hold's readings are ranges.
- **Any figure pooled across cells**, in section 6 or elsewhere.
- **Any other share, form, network or seed.**

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_the_part_held_within_by_target_explore` | [42551926531](https://github.com/Menteion/Menteion/actions/runs/42551926531) | `936f474` | 418 |
| `the_part_held_within_by_target_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [42552893754](https://github.com/Menteion/Menteion/actions/runs/42552893754) | `936f474` | 7325 |
| `the_part_held_within_by_target_a_first_mapping_on_seed_22_from_the_assignment_explore` | [42552893754](https://github.com/Menteion/Menteion/actions/runs/42552893754) | `936f474` | 7351 |
| `the_part_held_within_by_target_the_eighth_image_on_seed_23_from_the_mirrored_assignment_explore` | [42552893754](https://github.com/Menteion/Menteion/actions/runs/42552893754) | `936f474` | 7700 |
| `the_part_held_within_by_target_a_first_mapping_on_seed_23_from_the_assignment_explore` | [42552893754](https://github.com/Menteion/Menteion/actions/runs/42552893754) | `936f474` | 7665 |
| `the_part_held_within_by_target_the_eighth_image_on_seed_24_from_the_mirrored_assignment_explore` | [42552893754](https://github.com/Menteion/Menteion/actions/runs/42552893754) | `936f474` | 7702 |
| `the_part_held_within_by_target_a_first_mapping_on_seed_24_from_the_assignment_explore` | [42552893754](https://github.com/Menteion/Menteion/actions/runs/42552893754) | `936f474` | 7500 |
| `the_part_held_within_by_target_the_eighth_image_on_seed_23_from_the_assignment_explore` | [42552893754](https://github.com/Menteion/Menteion/actions/runs/42552893754) | `936f474` | 7389 |
| `the_part_held_within_by_target_a_first_mapping_on_seed_24_from_the_mirrored_assignment_explore` | [42552893754](https://github.com/Menteion/Menteion/actions/runs/42552893754) | `936f474` | 7790 |
| `the_part_held_within_by_target_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [42552893754](https://github.com/Menteion/Menteion/actions/runs/42552893754) | `936f474` | 7603 |
| `the_part_held_within_by_target_the_eighth_image_on_seed_24_from_the_assignment_explore` | [42552893754](https://github.com/Menteion/Menteion/actions/runs/42552893754) | `936f474` | 7315 |
| `the_part_held_within_by_target_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [42552893754](https://github.com/Menteion/Menteion/actions/runs/42552893754) | `936f474` | 7791 |
| `the_part_held_within_by_target_the_eighth_image_on_seed_22_from_the_assignment_explore` | [42552893754](https://github.com/Menteion/Menteion/actions/runs/42552893754) | `936f474` | 7578 |
