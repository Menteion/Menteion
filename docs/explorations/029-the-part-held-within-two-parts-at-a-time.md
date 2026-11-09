---
status: exploratory
date: 2026-11-09
---

# 029: The part held within, two parts at a time

**Branch**: `explore/029-the-part-held-within-two-parts-at-a-time`
**Commit**: `be301a0`, which adds the harness, on `e3be1ca`: `main` at `bc36347`, ADR-0244's merge, with exploration 028's tag merged
in, which carries explorations 024's, 025's, 027's and 028's changes under `src/`. This branch changes nothing under `src/`.
**Runs**: [42582358085](https://github.com/Menteion/Menteion/actions/runs/42582358085) (the harness's trial) and
[42583562331](https://github.com/Menteion/Menteion/actions/runs/42583562331) (twelve jobs, one a cell), both at `scope=arms`. H-47's
plain night on A's patterns is read from H-47's run [41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613), its class
matrix and night lines from exploration 020's runs [41500459562](https://github.com/Menteion/Menteion/actions/runs/41500459562) and
[41510955777](https://github.com/Menteion/Menteion/actions/runs/41510955777) for exploration 020's four cells and from exploration 021's run
[41591077514](https://github.com/Menteion/Menteion/actions/runs/41591077514) for the other eight. The excitatory night is read from
exploration 025's run [42469251250](https://github.com/Menteion/Menteion/actions/runs/42469251250) for exploration 020's four cells and from
exploration 026's run [42489793510](https://github.com/Menteion/Menteion/actions/runs/42489793510) for the other eight; exploration 027's
night held within from exploration 027's run [42514033935](https://github.com/Menteion/Menteion/actions/runs/42514033935) and its trial's
row from run [42512634982](https://github.com/Menteion/Menteion/actions/runs/42512634982); and each part held alone from exploration 028's
run [42552893754](https://github.com/Menteion/Menteion/actions/runs/42552893754) and its trial's rows from run
[42551926531](https://github.com/Menteion/Menteion/actions/runs/42551926531). None of those was run again.

[ADR-0244](../adr/0244-the-part-held-within-two-parts-at-a-time.md) ordered this exploration.
[Exploration 028](028-the-part-held-within-by-target.md) split [exploration 027](027-the-excitatory-hold-by-target.md)'s part held within,
the synapses from the readouts' excitatory units but the pair's own onto every unit but the other readout's excitatory units, in three by
target: onto the held unit's own group, onto the inhibitory units and onto the units outside the readouts. Held alone through H-47's plain
night on A's patterns, each left A's count beyond twice exploration 018's unpaired SE of the plain night's at 3, 13 and 6 of the 60
checkpoints, where exploration 027's night held within lay beyond at 17. This note holds the three parts two at a time, each pair of parts
being the part held within with one part left to consolidate, through H-47's plain night on A's patterns, on all twelve of H-47's cells. In
this note **a pair of parts** is two of the three parts held together; **the pair** is H-47's second mapping, C and D. It asks, at blocks
6, 12, 18, 24 and 36:

1. A's count and each stimulus's correct trials under each held form: less the night without replay and less each other held form, with
   H-47's paired SE, the forms run in one job; and less H-47's plain night, less exploration 027's night held within, less the excitatory
   night and less exploration 028's night for each of the form's two parts held alone, with [exploration 018](018-what-nights-keep.md)'s
   unpaired SE, the forms run in two jobs;
2. the side under each held form, by stimulus and by group of units, against H-47's side, the excitatory night's, exploration 027's night
   held within's and exploration 028's nights' on the same cell;
3. the pair under each held form and A's highest coupling, as H-47's clauses 2 and 3 read them;
4. what each night moved, from the class matrix by group, with the held units' synapses broken down by target, the two held parts' change
   shown nil and the change of the part left free and of the part held across read beside the plain night's, and the inhibition onto each
   group's excitatory units;
5. what the hold leaves: what the quiet ticks after each night move, and the pairings still pending on the two held parts' synapses, the
   gain and the loss apart;
6. the twelve cells side by side at block 36: each held form less the night without replay and less each other held form, with the job's
   paired SE, and less H-47's plain night, less exploration 027's night held within, less the excitatory night and less exploration 028's
   night for each of its two parts held alone, with exploration 018's unpaired SE.

**The side** is [ADR-0238](../adr/0238-what-keeps-a-mapping-where-the-replay-did-not.md)'s: at a checkpoint, the stimulus whose difference
from the night without replay is the larger of the two. The note writes no prediction and decides nothing. **Every figure here is
exploratory**: from one run of each test on the repository's AArch64 host, in H-47's terms. A count is a mean over a cell's 48 reads,
sixteen on each of three paths, and a difference carries its paired SE unless another is named.

**In brief**
- **Each hold held its two parts, and the night without replay ran H-47's.** In all 432 held nights every synapse of the two held parts
  ended the night at the weight it started with and its trace cleared, held inside the test, while synapses of the part left free moved,
  held inside the test too. The quiet ticks after a night moved no synapse of the two held parts (section 5). Every row of the night
  without replay, 15 a cell, is H-47's, held inside the test.
- **The three pairs of parts**, by seed: 11 406 to 11 564 synapses onto the own group and the inhibitory units, 9 252 to 9 369 onto the own
  group and the units outside the readouts and 6 853 to 6 904 onto the inhibitory units and the units outside the readouts, each exactly
  its two parts of exploration 028 and exactly exploration 027's part held within less the part it leaves free, in every cell, held inside
  the test synapse by synapse. In the trial the three parts held together ran exploration 027's night held within's row bit for bit.
- **Less exploration 027's night held within**, A's count under the night held onto the inhibitory units and the units outside the
  readouts, the part held within with the own group's part left to consolidate, lies within twice exploration 018's unpaired SE at all 60
  checkpoints. Under the night held onto the own group and the inhibitory units it lies beyond 2 SE at 7, all below, and under the night
  held onto the own group and the units outside the readouts at 13, 4 above and 9 below.
- **Less the excitatory night** it lies beyond 2 SE at 6, 11 and 17 of the 60, in the order the three pairs are named above, where
  exploration 027's night held within lay beyond at 4. **Less H-47's plain night** at 14, 15 and 8, where exploration 027's night held
  within lay beyond at 17 and the excitatory night at 25. Where the excitatory night lies beyond 2 SE of the plain night, the night held
  onto the inhibitory units and the units outside the readouts lies beyond the plain night in the same direction at 14, all 14 of its own.
- **Less exploration 028's nights**, the night held onto the own group and the inhibitory units lies within 2 SE of the night with the
  inhibitory units held alone at all 60 checkpoints and beyond 2 SE of the night with the own group held alone at 12. The other two pairs
  lie beyond 2 SE of the nights with their parts held alone at 3 to 8 of the 60.
- **The held nights less each other**, run in one job, lie beyond twice their paired SE at 21 to 23 of the 60 checkpoints; less the night
  without replay they lie beyond 2 SE at 48 to 49 of the 60, 44 to 48 of them above.
- **The side under each held night is H-47's at 51 to 56 of the 60 checkpoints**; at block 36 it is exploration 027's night held within's
  and the excitatory night's in eleven of the twelve cells under each (section 2).
- **The part left free and the part held across moved as under H-47's plain night**, within 0.22 M a group over four nights. The
  inhibition onto each group's excitatory units rose by +0.33 to +1.03 M more than under the plain night under the night held onto the own
  group and the inhibitory units, by +0.16 to +1.05 M under the night held onto the inhibitory units and the units outside the readouts and
  by −0.01 to +0.35 M under the night held onto the own group and the units outside the readouts, where exploration 027's night held
  within's rose by +0.33 to +1.11 M more.
- **The held nights ran H-47's plain night's replays and depotentiations in 142 of their 144 nights each**, the two others on seed 23's
  eighth image from the mirrored assignment, and exploration 027's night held within's in all 144. The watch read `wrong 0` in all 576
  nights of the twelve jobs.
- **The pairings the hold leaves**: after the quiet ticks, 5 808 to 6 587 synapses held onto the own group and the inhibitory units carry a
  pending gain, 4 405 to 4 879 held onto the own group and the units outside the readouts and 3 364 to 4 121 held onto the inhibitory units
  and the units outside the readouts, each read at the presynaptic unit's next spike if it comes before the target's next spike.
- **The pair** stands at 61.6 to 63.3 at block 36 under the three held nights, and A's highest coupling at 1.139 to 1.238 of H-42's.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 020 to 028 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. A cell is named, as in H-47, by its seed, its image and the
arm it starts from, not by the mapping in force. Nights follow blocks 6, 12, 18 and 24, then 12 blocks run with none to block 36, on paths
0, 5 and 6. Each checkpoint reads A and the pair sixteen times each, after the night that follows its block, and at block 36 after the
block.

**Places.** A period of the ring is 20 units. Stimulus A presents place 0 of each period and B place 11. **The odd places** are the odd
places but 11 and **the even places** the even places but 0: H-47's readout 0 and readout 1, each with its excitatory and its inhibitory
units. C's own units lie at place 5, among the odd places, and D's at place 16, among the even, in every cell
([exploration 021](021-where-the-gain-falls.md), section 4).

**Twelve cells**, all of H-47's, in exploration 028's order: the six where A answers readout 0 first, then the six where it answers
readout 1. Under the assignment stimulus A answers readout 0, the odd places, and B readout 1, the even places; under the mirrored mapping
the other way. A answers readout 0 on the eighth images from the mirrored assignment and on the first mappings from the assignment. The
excitatory night's job is exploration 026's for its eight cells and exploration 025's for exploration 020's four; exploration 027's night
held within is exploration 027's job and each part held alone exploration 028's job in all twelve:

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

Each part held alone less exploration 028's night without replay at block 36, with that job's paired SE:

| Cell | Own group alone (028) less without replay at block 36 | Inhibitory units alone (028) | Outside the readouts alone (028) |
| :--- | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | +16.8 ± 0.88 | +16.1 ± 1.18 | +20.8 ± 0.64 |
| Seed 22, first mapping, assignment | +10.2 ± 1.20 | +6.0 ± 0.78 | +8.3 ± 0.29 |
| Seed 23, eighth, mirrored | +17.0 ± 0.92 | +15.3 ± 0.91 | +17.0 ± 0.69 |
| Seed 23, first mapping, assignment | +12.7 ± 1.26 | +12.6 ± 1.64 | +12.9 ± 1.65 |
| Seed 24, eighth, mirrored | +11.5 ± 1.28 | +12.9 ± 1.03 | +12.0 ± 1.87 |
| Seed 24, first mapping, assignment | +4.4 ± 1.15 | +6.5 ± 1.02 | +4.8 ± 1.56 |
| Seed 23, eighth, assignment | +4.5 ± 2.81 | +6.7 ± 2.53 | +6.5 ± 1.93 |
| Seed 24, first mapping, mirrored | +7.0 ± 1.77 | +6.7 ± 1.57 | +7.0 ± 1.46 |
| Seed 23, first mapping, mirrored | −4.6 ± 0.95 | −3.0 ± 0.78 | −4.7 ± 0.66 |
| Seed 24, eighth, assignment | +0.7 ± 1.29 | +1.4 ± 0.82 | +3.5 ± 2.31 |
| Seed 22, first mapping, mirrored | +5.4 ± 1.25 | +4.8 ± 1.23 | +6.0 ± 1.15 |
| Seed 22, eighth, assignment | +1.5 ± 1.08 | +3.3 ± 0.36 | +4.2 ± 0.84 |

**The hold** is exploration 028's, a change under `src/` on exploration 028's branch, merged in here, in no image, which `main` does not
take. `Executor::set_night_hold_by_target_parts` takes a set of excitatory units, each with a group, as held sources; a set of targets, each
with a group; and the parts to hold, whose slots it holds together. Of exploration 027's part held within, exploration 028 named three
parts:
- **`OwnGroup`**, each synapse from a held source onto a target of its own group;
- **`Inhibitory`**, each synapse from a held source onto an inhibitory target of no group;
- **`Outside`**, each synapse from a held source onto an excitatory target of no group.

**The three pairs of parts** are each set by giving that call its two parts (`E029Form`, `e029_job`):
- **own group and inhibitory units**: `OwnGroup` and `Inhibitory`, which leaves the part onto the units outside the readouts to
  consolidate;
- **own group and outside the readouts**: `OwnGroup` and `Outside`, which leaves the part onto the inhibitory units;
- **inhibitory units and outside the readouts**: `Inhibitory` and `Outside`, which leaves the part onto the own group.

A held synapse is held as exploration 028 holds its part: while the engine is in slow-wave sleep or REM the fan-out consolidates every slot
of a held source's block but those of the two parts, slot by slot, so each held synapse keeps its weight while its trace takes the night's
pairings, and every other synapse, the part left free's and the part held across's among them, consolidates as in H-47. **The night's last
tick** is the tick whose window steps the engine from sleep to awake; the hold is in force through it, and at its end each synapse of the
two parts has its trace cleared to zero, and no other synapse. The hold is in no image, so the next day's engine, decoded from the night's
image, holds nothing. Exploration 024's hold (`set_night_hold`) and exploration 025's (`set_night_hold_sources`) are on the branch too, and
no form here sets either; only the trial sets the three parts together.

**The held units, the targets with a group and the classes** (`e025_held`, `e027_hold`, `e028_hold`) are exploration 028's: every
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

Each held night holds, inside the test, synapse by synapse (`e029_pairs`, on `e028_parts`): that the three parts are disjoint and together
exactly the synapses exploration 027's part held within names; that the part held across is exploration 027's; that exploration 027's two
parts are disjoint and together exploration 025's hold; that each synapse of each part, by its held unit's group, has its target in that
part's columns; and that each pair of parts is exactly the union of its two parts and exactly the part held within less the part it leaves
free, with as many synapses as its two parts. Each pair's synapses and each part's, as each held night prints them:

| Cell | Own group and inhibitory units: synapses | from the odd places and from the even | Own group and outside the readouts | from the odd places and from the even | Inhibitory units and outside the readouts | from the odd places and from the even | Parts: onto the own group | onto the inhibitory units | onto the units outside the readouts | The three together | Exploration 027's part held within | Held across | Excitatory synapses |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 11 564 | 5 793 and 5 771 | 9 369 | 4 708 and 4 661 | 6 873 | 3 447 and 3 426 | 7 030 | 4 534 | 2 339 | 13 903 | 13 903 | 7 345 | 26 240 |
| Seed 22, first mapping, assignment | 11 564 | 5 793 and 5 771 | 9 369 | 4 708 and 4 661 | 6 873 | 3 447 and 3 426 | 7 030 | 4 534 | 2 339 | 13 903 | 13 903 | 7 345 | 26 240 |
| Seed 23, eighth, mirrored | 11 406 | 5 698 and 5 708 | 9 285 | 4 719 and 4 566 | 6 853 | 3 421 and 3 432 | 6 919 | 4 487 | 2 366 | 13 772 | 13 772 | 7 476 | 26 240 |
| Seed 23, first mapping, assignment | 11 406 | 5 698 and 5 708 | 9 285 | 4 719 and 4 566 | 6 853 | 3 421 and 3 432 | 6 919 | 4 487 | 2 366 | 13 772 | 13 772 | 7 476 | 26 240 |
| Seed 24, eighth, mirrored | 11 434 | 5 716 and 5 718 | 9 252 | 4 684 and 4 568 | 6 904 | 3 462 and 3 442 | 6 891 | 4 543 | 2 361 | 13 795 | 13 795 | 7 453 | 26 240 |
| Seed 24, first mapping, assignment | 11 434 | 5 716 and 5 718 | 9 252 | 4 684 and 4 568 | 6 904 | 3 462 and 3 442 | 6 891 | 4 543 | 2 361 | 13 795 | 13 795 | 7 453 | 26 240 |
| Seed 23, eighth, assignment | 11 406 | 5 698 and 5 708 | 9 285 | 4 719 and 4 566 | 6 853 | 3 421 and 3 432 | 6 919 | 4 487 | 2 366 | 13 772 | 13 772 | 7 476 | 26 240 |
| Seed 24, first mapping, mirrored | 11 434 | 5 716 and 5 718 | 9 252 | 4 684 and 4 568 | 6 904 | 3 462 and 3 442 | 6 891 | 4 543 | 2 361 | 13 795 | 13 795 | 7 453 | 26 240 |
| Seed 23, first mapping, mirrored | 11 406 | 5 698 and 5 708 | 9 285 | 4 719 and 4 566 | 6 853 | 3 421 and 3 432 | 6 919 | 4 487 | 2 366 | 13 772 | 13 772 | 7 476 | 26 240 |
| Seed 24, eighth, assignment | 11 434 | 5 716 and 5 718 | 9 252 | 4 684 and 4 568 | 6 904 | 3 462 and 3 442 | 6 891 | 4 543 | 2 361 | 13 795 | 13 795 | 7 453 | 26 240 |
| Seed 22, first mapping, mirrored | 11 564 | 5 793 and 5 771 | 9 369 | 4 708 and 4 661 | 6 873 | 3 447 and 3 426 | 7 030 | 4 534 | 2 339 | 13 903 | 13 903 | 7 345 | 26 240 |
| Seed 22, eighth, assignment | 11 564 | 5 793 and 5 771 | 9 369 | 4 708 and 4 661 | 6 873 | 3 447 and 3 426 | 7 030 | 4 534 | 2 339 | 13 903 | 13 903 | 7 345 | 26 240 |

The synapses from A's, B's and the pair's own units consolidate as in H-47 under every form, the zero sum's rounds with them, and so does
every inhibitory synapse.

**Four forms a cell, in one job** (`e029_job`), exploration 028's job with its three held forms replaced by the three pairs of parts
([ADR-0244](../adr/0244-the-part-held-within-two-parts-at-a-time.md)):
- **the night without replay**, every row held inside the test to the row H-47 pinned;
- **the night held onto the own group and the inhibitory units** (own group and inhibitory units, in the tables), H-47's plain night on A's
  patterns with `OwnGroup` and `Inhibitory` set;
- **the night held onto the own group and the units outside the readouts** (own group and outside the readouts), the same with `OwnGroup`
  and `Outside` set;
- **the night held onto the inhibitory units and the units outside the readouts** (inhibitory units and outside the readouts), the same with
  `Inhibitory` and `Outside` set.

Exploration 028's three nights are named in the tables **own group alone (028)**, **inhibitory units alone (028)** and **outside the
readouts alone (028)**. The job is exploration 020's to 028's: A's patterns, the split and the class matrix of the start image, of the
image before each night and of the image after it, and the nights after blocks 6, 12, 18 and 24 watched by `ReplayWatch`.

**At each held night** (`e028_night`) the job reads every excitatory synapse three times: before the night's first tick, at its last, and
after the quiet ticks, the awake ticks without drive that H-47's harness runs until the engine is quiescent, before it writes the night's
image. At the last it holds, inside the test, that every synapse of the two held parts stands at its first weight with its trace cleared,
that some synapse of the part left free moved, and that some excitatory synapse from a unit outside the hold moved; it prints, for the part
left free and the part held across, how many of its synapses moved, their change, and how many carry a trace that is not zero. These are
counts: the jobs print no trace itself. After the quiet ticks it prints what they moved, and what the two held parts' synapses would read at
their presynaptic unit's next spike from the spikes on record since the night's first tick, if that spike came before the target's next
one: for the gain and for the loss, how many synapses carry one, their sum and the largest. To hold the part left free's move inside every
job's night, this branch gives `e028_night` a third level of what it holds of the parts its hold leaves free (`E028Free::Each`), between
exploration 028's job's and its trial's, which hold what they held.

**The excitatory rule** (`step_stdp_shaped`, in `crates/menteion-core/src/dynamics/synapse.rs`) reads two pairings at a presynaptic spike
at $t$, with $p$ the presynaptic unit's previous spike and $q$ the target's last spike: if $p < q \le t$, a gain of
$A_+ (1 - 2^{-11})^{q - p}$; and if $q < t$, a loss of $A_- (1 - 2^{-11})^{t - q}$ scaled by the slot's magnitude, by the exponent the
configuration sets ([ADR-0228](../adr/0228-a-zero-sum-at-the-stimulus-built.md)). `step_stdp_all_shaped` runs on every slot of a held
part as on any other, so its trace takes the night's pairings; only the consolidation is skipped. Both pairings are read only at a
presynaptic spike, so the night's last pairings escape the clearing. The job finds each with the rule itself, on a copy of the block, the
slot's trace set to zero; with $t_0$ the first tick after the image:
- **the pending gain**: a synapse of the two held parts whose target's last spike and presynaptic unit's last spike are both on record since
  the night's first tick, $q > p$: $A_+ (1 - 2^{-11})^{q - p}$, read at the presynaptic unit's next spike if it comes before the target's
  next spike;
- **the pending loss**: a synapse of the two held parts whose target fired since the night's first tick: the loss a presynaptic spike at
  $t_0$ would read, at the synapse's magnitude after the quiet ticks. It falls as $t$ grows, so it is the most a later presynaptic spike
  reads, if that spike comes before the target's next one.

A target spike before the next presynaptic one takes $q$'s place, and both readings change with it.

**The class matrix** is exploration 020's, as 021 reads it by readout and 023 to 028 by group. Each group's excitatory units are its
readout's two classes and the pair's own kind inside it, C's own in the odd places and D's in the even. An inhibitory entry is a
magnitude, the weight negated, so a rise is more inhibition. The held units' synapses are the rows of classes 6 and 7, the odd places' held
units, and 8 and 9, the even places', onto every class; each part is their entries in its columns above.

## Before a job ran

ADR-0244 asks whether any form but the night without replay must run bit for bit as one already in the logs. **None must.** Each held form
holds two parts of the part held within and lets the third and the part held across consolidate. Over four nights, the mean over the three
paths, a group's held units' synapses moved, in each of the twelve cells:

| Form, its job | Onto the own group | Onto the inhibitory units | Onto the units outside the readouts | Held across |
| :--- | ---: | ---: | ---: | ---: |
| H-47's plain night on A's patterns (exploration 020's and 021's jobs) | −2.79 to −0.02 M | −2.71 to −1.45 M | −1.01 to +0.09 M | −6.98 to −4.73 M |
| Exploration 027's night held across | −2.96 to −0.27 M | −2.87 to −1.45 M | −1.18 to −0.05 M | 0.00, exactly |
| Exploration 027's night held within | 0.00, exactly | 0.00, exactly | 0.00, exactly | −6.84 to −4.80 M |
| The excitatory night (exploration 025's and 026's jobs) | 0.00, exactly | 0.00, exactly | 0.00, exactly | 0.00, exactly |
| Exploration 028's night, own group alone | 0.00, exactly | −2.70 to −1.49 M | −1.08 to +0.05 M | −6.90 to −4.83 M |
| Exploration 028's night, inhibitory units alone | −2.76 to +0.0048 M | 0.00, exactly | −1.01 to +0.07 M | −6.86 to −4.69 M |
| Exploration 028's night, outside the readouts alone | −2.77 to −0.0014 M | −2.65 to −1.44 M | 0.00, exactly | −6.99 to −4.79 M |

Where a range is not "exactly", none of its 24 groups' entries is zero. Of all those entries the three nearest zero are on seed 23's first
mapping from the assignment, the odd places' held units onto their own group: −0.0014 M under exploration 028's night with the units outside
the readouts held alone, +0.0048 M under its night with the inhibitory units held alone and −0.02 M under the plain night. So each pair of
parts holds a part that the plain night, exploration 027's night held across and each of exploration 028's three nights move: the own
group's part with the inhibitory units' holds the inhibitory units' part, which exploration 028's night with the own group held alone moves,
and the own group's, which its other two nights move; the own group's part with the units outside the readouts' holds the units outside the
readouts' part, which exploration 028's night with the own group held alone moves, and the own group's, which its other two move; and the
inhibitory units' part with the units outside the readouts' holds the inhibitory units' part, which exploration 028's nights with the own
group or the units outside the readouts held alone move, and the units outside the readouts' part, which its night with the inhibitory units
held alone moves. And each lets consolidate a part that exploration 027's night held within and the excitatory night hold, as does
exploration 025's night under both holds, which holds every synapse the excitatory night holds and exploration 024's inhibitory synapses
beside them. Exploration 024's hold holds no excitatory synapse, and the forms in the logs without replay run no replay. The trial then
held, inside the test, that the part each pair leaves free moved in its night, and printed each held form's row at block 6 on path 0 on seed
23's first mapping from the assignment beside H-47's plain night's, exploration 027's trial's night held within's and exploration 028's
trial's rows for the form's two parts held alone on the same cell: none is the same, as the trial prints. So all four forms ran.

## The runs are laid out as written

- **The trial** ran on path 0, on seed 23's first mapping from the assignment: the pair's head and the first night of H-47's plain night on
  A's patterns, the night without replay, each of the three held nights and the plain night with the three parts held together, all
  watched. With no hold set, the night without replay and the plain night held H-47's row at block 6 on path 0, inside the test. On the
  head's image the three parts were exploration 028's trial's, disjoint and together exactly exploration 027's trial's 13 772 synapses held
  within, synapse by synapse, held inside the test with their totals: 6 919 onto the own group, 4 487 onto the inhibitory units and 2 366
  onto the units outside the readouts. Each pair of parts was exactly its two parts' union and exactly the part held within less the part
  it leaves free, synapse by synapse, held inside the test with its total: 11 406 onto the own group and the inhibitory units, 9 285 onto
  the own group and the units outside the readouts and 6 853 onto the inhibitory units and the units outside the readouts. By group, as the
  trial prints them, from the odd places and from the even: the parts 3 498 and 3 421, 2 200 and 2 287, and 1 221 and 1 145; the pairs
  5 698 and 5 708, 4 719 and 4 566, and 3 421 and 3 432. The part held across is 7 476, 3 705 and 3 771, as the trial prints.
- With each pair's hold set, the night held, inside the test, every synapse of both parts at its first weight with its trace cleared, a
  synapse of the part left free moved and, at the night's last tick, a synapse of the held units outside the two parts with a trace that is
  not zero. The part left free's synapses and the part held across's that moved and those with a trace at the night's last tick, and what
  the hold cleared, as the trial prints them:

| Night | Onto the own group: moved, of 6 919; with a trace | Onto the inhibitory units: of 4 487 | Onto the units outside the readouts: of 2 366 | Held across: of 7 476 | Held synapses: traces cleared, not zero | cleared above zero, LSB | below zero |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Own group and inhibitory units | held | held | 2 364; 1 274 | 7 458; 3 210 | 7 165 of 11 406 | 527 448 | −707 278 |
| Own group and outside the readouts | held | 4 477; 2 458 | held | 7 456; 3 182 | 5 834 of 9 285 | 516 325 | −586 245 |
| Inhibitory units and outside the readouts | 6 897; 3 716 | held | held | 7 460; 3 181 | 4 269 of 6 853 | 257 987 | −408 330 |
| The three held together | held | held | held | 7 457; 3 203 | 8 646 of 13 772 | 647 599 | −861 852 |

- **With the three parts held together** the plain night on A's patterns held, inside the test, the row exploration 027's trial printed for
  its night held within at block 6 on path 0 (run 42512634982): the image's CRC-64 16346174311379552951, A's first count of 57, its
  couplings and the gain, as exploration 028's trial held it. It cleared what exploration 028's trial printed its three parts held together
  clearing, 13 772 slots, 8 646 of them with a trace, 647 599 LSB above zero and −861 852 below, and the part held across moved 7 457
  synapses with 3 203 carrying a trace, as there.
- Each held form's row at block 6 on path 0, as the trial prints it beside H-47's plain night's row, exploration 027's trial's row for its
  night held within and exploration 028's trial's rows for the form's two parts held alone:

| Night | Image's CRC-64 | A's first count | A's couplings | Gain at the night's end | The same as the night's row, as the trial prints |
| :--- | ---: | ---: | ---: | ---: | :--- |
| H-47's plain night on A's patterns | 7582150002090434622 | 57 | 7 405 970, 5 357 375, 6 066 402, 7 180 052 | 107 421 | – |
| Exploration 027's trial: night held within | 16346174311379552951 | 57 | 7 433 707, 5 329 660, 6 068 177, 7 178 280 | 107 411 | – |
| Exploration 028's trial: own group alone | 17161476349453458528 | 57 | 7 408 792, 5 354 546, 6 072 077, 7 174 358 | 107 422 | – |
| Exploration 028's trial: inhibitory units alone | 16037928774145384866 | 57 | 7 414 445, 5 348 916, 6 066 899, 7 179 549 | 107 414 | – |
| Exploration 028's trial: outside the readouts alone | 11971943527930658059 | 58 | 7 415 802, 5 347 530, 6 060 140, 7 186 290 | 107 448 | – |
| Own group and inhibitory units | 14238490822434375201 | 57 | 7 420 858, 5 342 498, 6 072 789, 7 173 659 | 107 399 | H-47's plain: no; 027's held within: no; 028's own group alone: no; 028's inhibitory units alone: no |
| Own group and outside the readouts | 13195711694865969397 | 58 | 7 421 522, 5 341 816, 6 063 242, 7 183 194 | 107 428 | H-47's plain: no; 027's held within: no; 028's own group alone: no; 028's outside the readouts alone: no |
| Inhibitory units and outside the readouts | 17021947810932160098 | 58 | 7 420 493, 5 342 854, 6 064 384, 7 182 056 | 107 417 | H-47's plain: no; 027's held within: no; 028's inhibitory units alone: no; 028's outside the readouts alone: no |
| The three held together | 16346174311379552951 | 57 | 7 433 707, 5 329 660, 6 068 177, 7 178 280 | 107 411 | 027's held within: held inside the test |

- None of the three pairs' rows is any of the rows printed beside it, as the trial prints: each image's CRC-64, A's couplings and the gain
  differ. A's first count is 57 under the night held onto the own group and the inhibitory units and 58 under the other two, where H-47's
  plain night, exploration 027's night held within and exploration 028's nights with the own group or the inhibitory units held alone read
  57, and its night with the units outside the readouts held alone 58. The watch read `wrong 0` in the trial's six nights. Each of the five
  nights with replay read 1 339 replays and 640 depotentiations, and the night without replay none.

- **Each of the twelve jobs** ran to the end. Their reproduction, cell by cell:

| Cell | Without replay: rows H-47's | Own group and inhibitory units, own group and outside, inhibitory units and outside: rows H-47's plain on A's | rows 027's held within | rows the excitatory night's | rows 027's held across | rows 028's three nights, together | Without replay: its paths' lines in the plain night's job, label aside | Heads: lines in that job | Plain night's lines: job | Held nights: both parts held and the pairs held inside the test |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | :--- | ---: |
| Seed 22, eighth, mirrored | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 632 of 1 632 | 3 of 3 | 021 | 36 of 36 |
| Seed 22, first mapping, assignment | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 632 of 1 632 | 3 of 3 | 021 | 36 of 36 |
| Seed 23, eighth, mirrored | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 680 of 1 680 | 3 of 3 | 021 | 36 of 36 |
| Seed 23, first mapping, assignment | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 680 of 1 680 | 3 of 3 | 021 | 36 of 36 |
| Seed 24, eighth, mirrored | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 656 of 1 656 | 3 of 3 | 021 | 36 of 36 |
| Seed 24, first mapping, assignment | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 656 of 1 656 | 3 of 3 | 021 | 36 of 36 |
| Seed 23, eighth, assignment | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 680 of 1 680 | 3 of 3 | 021 | 36 of 36 |
| Seed 24, first mapping, mirrored | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 656 of 1 656 | 3 of 3 | 021 | 36 of 36 |
| Seed 23, first mapping, mirrored | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 680 of 1 680 | 3 of 3 | 020 | 36 of 36 |
| Seed 24, eighth, assignment | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 656 of 1 656 | 3 of 3 | 020 | 36 of 36 |
| Seed 22, first mapping, mirrored | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 632 of 1 632 | 3 of 3 | 020 | 36 of 36 |
| Seed 22, eighth, assignment | 15 of 15 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 0, 0, 0 | 1 632 of 1 632 | 3 of 3 | 020 | 36 of 36 |

- The night without replay's `DUMP` lines, its paths' 1 632 to 1 680 a cell, its 15 rows' and its notice, are the lines of H-47's plain
  night's class matrix job on the same cell, label aside, all of them in each cell: exploration 021's run for exploration 026's eight cells
  and exploration 020's runs for exploration 020's four. So are the pair's head's three. Each read of the night without replay carries
  H-47's read's `stimuli`, `right` and `tied` masks, all 480 a cell. Each held form's reads carry H-47's plain night's `stimuli` masks, so
  each trial presented the stimulus it presented in H-47. The night without replay's reads at every checkpoint are those of H-47's, of
  exploration 027's, of exploration 028's and of the excitatory night's job, read by read.
- For each held form the job prints how many of its 15 rows are H-47's plain night's rows: **none, in all twelve cells**, as the job
  prints; nothing in the job asserts it. Set beside exploration 027's rows for its night held within and its night held across, beside the
  excitatory night's rows in exploration 025's or 026's log and beside exploration 028's rows for each of its three nights, row by row with
  the form's name left out, none of the three held forms' rows is the same, and none of the three is the same as another's in its own job.
- Every held unit's synapse was read at each night: 664 held units, 332 a readout, C's own 25 and D's own 25 left out, as each job prints.
  Each job holds that none of the zero sum's 152 sources is among them. Each held night held its two parts' hold and its pairs of parts
  inside the test, 36 a cell.

## 1. A's count under each held night

A's count at each checkpoint under each form. H-47's plain night on A's patterns is H-47's job's, exploration 027's night held within is
exploration 027's job's, the excitatory night is exploration 025's or 026's job's, and each part held alone is exploration 028's job's:

| Cell | Block | Without replay | Own group and inhibitory units | Own group and outside the readouts | Inhibitory units and outside the readouts | Plain on A's (H-47) | Held within (027) | Excitatory (025, 026) | Own group alone (028) | Inhibitory units alone (028) | Outside the readouts alone (028) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 46.3 | 52.7 | 52.9 | 53.0 | 53.3 | 52.6 | 52.4 | 53.1 | 52.9 | 53.3 |
| Seed 22, eighth, mirrored | 12 | 30.2 | 40.2 | 41.8 | 42.0 | 42.3 | 40.9 | 41.0 | 41.0 | 40.7 | 42.7 |
| Seed 22, eighth, mirrored | 18 | 26.3 | 40.1 | 40.4 | 41.0 | 41.8 | 40.5 | 40.9 | 40.3 | 40.6 | 41.8 |
| Seed 22, eighth, mirrored | 24 | 22.6 | 34.7 | 38.8 | 38.5 | 40.2 | 37.0 | 37.1 | 37.5 | 35.3 | 42.0 |
| Seed 22, eighth, mirrored | 36 | 16.6 | 31.9 | 35.1 | 34.4 | 36.2 | 33.3 | 32.8 | 33.4 | 32.6 | 37.3 |
| Seed 22, first mapping, assignment | 6 | 52.4 | 56.6 | 56.8 | 56.3 | 56.5 | 56.7 | 56.8 | 56.7 | 56.3 | 56.5 |
| Seed 22, first mapping, assignment | 12 | 44.8 | 47.0 | 48.2 | 47.2 | 47.6 | 47.3 | 45.8 | 47.8 | 46.9 | 48.4 |
| Seed 22, first mapping, assignment | 18 | 40.2 | 47.0 | 48.6 | 48.1 | 46.9 | 46.6 | 46.2 | 48.1 | 46.8 | 48.1 |
| Seed 22, first mapping, assignment | 24 | 38.9 | 46.5 | 50.0 | 47.1 | 48.0 | 47.3 | 45.6 | 49.9 | 45.8 | 48.8 |
| Seed 22, first mapping, assignment | 36 | 36.3 | 42.7 | 46.3 | 43.3 | 44.3 | 42.8 | 41.5 | 46.6 | 42.4 | 44.6 |
| Seed 23, eighth, mirrored | 6 | 34.4 | 36.0 | 35.8 | 35.6 | 36.1 | 35.4 | 36.8 | 36.6 | 36.1 | 35.8 |
| Seed 23, eighth, mirrored | 12 | 19.5 | 26.0 | 28.0 | 25.7 | 28.5 | 26.3 | 25.5 | 27.5 | 26.1 | 27.6 |
| Seed 23, eighth, mirrored | 18 | 14.9 | 24.4 | 26.5 | 24.0 | 27.6 | 23.4 | 24.0 | 26.5 | 24.6 | 27.0 |
| Seed 23, eighth, mirrored | 24 | 10.6 | 26.5 | 27.6 | 25.9 | 29.3 | 26.0 | 25.2 | 28.0 | 25.9 | 28.5 |
| Seed 23, eighth, mirrored | 36 | 8.2 | 23.2 | 24.8 | 24.7 | 25.6 | 23.7 | 22.9 | 25.2 | 23.5 | 25.1 |
| Seed 23, first mapping, assignment | 6 | 54.2 | 56.5 | 56.2 | 56.5 | 56.3 | 56.3 | 56.1 | 56.4 | 56.4 | 56.1 |
| Seed 23, first mapping, assignment | 12 | 33.4 | 39.1 | 38.0 | 37.8 | 37.6 | 39.0 | 37.5 | 38.1 | 37.6 | 38.0 |
| Seed 23, first mapping, assignment | 18 | 27.2 | 34.9 | 34.8 | 34.1 | 35.0 | 34.8 | 35.1 | 34.4 | 34.4 | 34.2 |
| Seed 23, first mapping, assignment | 24 | 24.8 | 35.1 | 34.8 | 34.3 | 34.8 | 35.8 | 35.0 | 34.1 | 34.6 | 34.1 |
| Seed 23, first mapping, assignment | 36 | 19.7 | 32.5 | 32.8 | 32.9 | 33.0 | 33.0 | 32.3 | 32.4 | 32.3 | 32.6 |
| Seed 24, eighth, mirrored | 6 | 36.4 | 34.9 | 34.1 | 35.1 | 33.8 | 35.1 | 36.6 | 33.4 | 34.9 | 34.0 |
| Seed 24, eighth, mirrored | 12 | 31.5 | 32.2 | 30.4 | 33.2 | 28.6 | 33.8 | 33.6 | 29.0 | 31.3 | 29.6 |
| Seed 24, eighth, mirrored | 18 | 20.2 | 32.6 | 32.0 | 33.4 | 28.6 | 35.2 | 34.4 | 30.1 | 30.7 | 29.4 |
| Seed 24, eighth, mirrored | 24 | 16.4 | 29.5 | 28.1 | 31.7 | 25.8 | 32.9 | 33.9 | 25.1 | 28.7 | 27.0 |
| Seed 24, eighth, mirrored | 36 | 12.5 | 27.5 | 25.8 | 29.0 | 23.8 | 28.1 | 29.2 | 24.0 | 25.4 | 24.5 |
| Seed 24, first mapping, assignment | 6 | 58.3 | 61.6 | 61.3 | 61.5 | 61.2 | 61.6 | 61.1 | 61.3 | 61.6 | 61.5 |
| Seed 24, first mapping, assignment | 12 | 52.0 | 55.8 | 56.6 | 55.2 | 54.8 | 57.0 | 55.7 | 55.8 | 54.8 | 54.8 |
| Seed 24, first mapping, assignment | 18 | 47.5 | 51.6 | 52.1 | 52.5 | 51.3 | 52.4 | 52.5 | 52.4 | 51.9 | 51.5 |
| Seed 24, first mapping, assignment | 24 | 43.7 | 52.3 | 54.0 | 53.3 | 52.4 | 53.7 | 52.7 | 53.1 | 53.1 | 53.3 |
| Seed 24, first mapping, assignment | 36 | 40.1 | 45.2 | 45.3 | 46.2 | 43.8 | 46.2 | 45.8 | 44.5 | 46.6 | 44.9 |
| Seed 23, eighth, assignment | 6 | 47.4 | 50.6 | 50.0 | 50.9 | 50.4 | 50.4 | 50.7 | 50.1 | 50.5 | 50.2 |
| Seed 23, eighth, assignment | 12 | 40.8 | 46.3 | 43.6 | 46.0 | 43.4 | 47.2 | 47.5 | 44.2 | 45.9 | 43.5 |
| Seed 23, eighth, assignment | 18 | 37.3 | 47.9 | 45.3 | 49.2 | 45.2 | 50.0 | 50.1 | 44.7 | 47.8 | 48.6 |
| Seed 23, eighth, assignment | 24 | 38.0 | 49.1 | 47.5 | 51.3 | 43.7 | 52.1 | 51.0 | 44.9 | 47.6 | 47.3 |
| Seed 23, eighth, assignment | 36 | 36.2 | 43.6 | 42.5 | 45.5 | 40.3 | 46.4 | 47.1 | 40.6 | 42.9 | 42.7 |
| Seed 24, first mapping, mirrored | 6 | 54.3 | 57.4 | 57.0 | 57.5 | 57.7 | 57.2 | 57.9 | 57.1 | 57.4 | 57.5 |
| Seed 24, first mapping, mirrored | 12 | 46.3 | 50.8 | 52.4 | 49.8 | 51.0 | 50.7 | 52.3 | 51.3 | 50.0 | 50.7 |
| Seed 24, first mapping, mirrored | 18 | 44.0 | 50.9 | 53.6 | 50.5 | 51.8 | 51.5 | 53.3 | 52.3 | 51.0 | 52.0 |
| Seed 24, first mapping, mirrored | 24 | 44.4 | 51.7 | 54.2 | 52.0 | 52.0 | 53.3 | 53.7 | 53.0 | 52.2 | 52.6 |
| Seed 24, first mapping, mirrored | 36 | 42.7 | 48.9 | 50.7 | 49.8 | 48.7 | 50.4 | 50.1 | 49.8 | 49.5 | 49.7 |
| Seed 23, first mapping, mirrored | 6 | 55.0 | 56.4 | 56.6 | 56.8 | 56.9 | 56.6 | 56.6 | 56.5 | 56.9 | 56.8 |
| Seed 23, first mapping, mirrored | 12 | 52.4 | 54.7 | 54.1 | 55.7 | 54.4 | 54.8 | 55.5 | 53.5 | 55.6 | 55.2 |
| Seed 23, first mapping, mirrored | 18 | 52.0 | 53.7 | 51.3 | 53.6 | 52.4 | 53.8 | 54.2 | 51.5 | 54.4 | 52.1 |
| Seed 23, first mapping, mirrored | 24 | 53.5 | 52.0 | 50.5 | 52.8 | 50.9 | 52.6 | 53.3 | 49.8 | 52.5 | 51.2 |
| Seed 23, first mapping, mirrored | 36 | 53.1 | 50.3 | 49.2 | 50.4 | 48.3 | 50.5 | 51.4 | 48.6 | 50.1 | 48.5 |
| Seed 24, eighth, assignment | 6 | 38.8 | 37.8 | 38.4 | 38.8 | 39.1 | 38.3 | 39.3 | 37.6 | 38.8 | 38.8 |
| Seed 24, eighth, assignment | 12 | 34.8 | 34.4 | 35.6 | 35.5 | 34.2 | 34.5 | 36.8 | 33.4 | 34.1 | 35.1 |
| Seed 24, eighth, assignment | 18 | 34.4 | 30.4 | 31.3 | 31.7 | 28.5 | 31.6 | 35.6 | 29.9 | 30.0 | 31.7 |
| Seed 24, eighth, assignment | 24 | 35.6 | 29.0 | 32.3 | 32.6 | 29.6 | 32.4 | 36.7 | 29.7 | 29.7 | 31.7 |
| Seed 24, eighth, assignment | 36 | 25.6 | 26.6 | 27.2 | 31.2 | 27.4 | 29.7 | 34.4 | 26.3 | 27.0 | 29.1 |
| Seed 22, first mapping, mirrored | 6 | 56.6 | 58.3 | 57.8 | 58.1 | 57.7 | 58.2 | 58.1 | 58.0 | 58.2 | 57.4 |
| Seed 22, first mapping, mirrored | 12 | 49.5 | 53.0 | 53.1 | 53.0 | 52.5 | 53.0 | 52.7 | 52.6 | 53.1 | 53.9 |
| Seed 22, first mapping, mirrored | 18 | 49.0 | 52.8 | 52.6 | 53.1 | 52.7 | 53.2 | 52.3 | 52.5 | 53.3 | 54.1 |
| Seed 22, first mapping, mirrored | 24 | 45.9 | 51.1 | 51.5 | 52.0 | 50.6 | 51.8 | 50.7 | 51.3 | 50.8 | 52.3 |
| Seed 22, first mapping, mirrored | 36 | 44.1 | 49.3 | 49.9 | 50.4 | 49.1 | 49.7 | 48.8 | 49.6 | 49.0 | 50.2 |
| Seed 22, eighth, assignment | 6 | 50.3 | 52.8 | 52.0 | 52.9 | 52.3 | 52.6 | 52.6 | 52.2 | 52.4 | 52.7 |
| Seed 22, eighth, assignment | 12 | 47.6 | 52.6 | 51.8 | 52.9 | 50.6 | 52.9 | 52.8 | 51.1 | 51.6 | 51.4 |
| Seed 22, eighth, assignment | 18 | 47.2 | 51.4 | 52.0 | 52.9 | 50.6 | 51.8 | 51.8 | 50.3 | 51.6 | 52.2 |
| Seed 22, eighth, assignment | 24 | 47.3 | 50.2 | 52.2 | 53.4 | 50.2 | 52.6 | 52.1 | 49.8 | 50.9 | 51.5 |
| Seed 22, eighth, assignment | 36 | 44.2 | 45.9 | 48.1 | 48.3 | 46.4 | 48.7 | 48.8 | 45.7 | 47.5 | 48.4 |

The differences between the forms run in one job, each with the job's paired SE. The night without replay is H-47's, read by read, so
"H-47: plain less without replay" at block 36 is H-47's clause 1. Each part held alone is set against exploration 028's night without
replay, which is H-47's too:

| Cell | Block | Own group and inhibitory units less without replay | Own group and outside the readouts less without replay | Inhibitory units and outside the readouts less without replay | H-47: plain less without replay | 027: held within less without replay | Excitatory less without replay (025, 026) | 028: own group alone less without replay | 028: inhibitory units alone | 028: outside the readouts alone |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +6.5 ± 1.03 | +6.7 ± 0.87 | +6.8 ± 0.89 | +7.1 ± 0.91 | +6.3 ± 0.87 | +6.1 ± 0.98 | +6.9 ± 0.92 | +6.6 ± 0.92 | +7.0 ± 0.82 |
| Seed 22, eighth, mirrored | 12 | +10.0 ± 1.42 | +11.6 ± 1.51 | +11.8 ± 1.13 | +12.1 ± 0.85 | +10.7 ± 1.17 | +10.8 ± 0.98 | +10.8 ± 1.53 | +10.5 ± 1.54 | +12.5 ± 1.09 |
| Seed 22, eighth, mirrored | 18 | +13.8 ± 1.83 | +14.1 ± 2.09 | +14.8 ± 1.70 | +15.5 ± 1.67 | +14.3 ± 1.80 | +14.6 ± 1.78 | +14.0 ± 2.34 | +14.3 ± 1.72 | +15.5 ± 1.48 |
| Seed 22, eighth, mirrored | 24 | +12.1 ± 1.34 | +16.2 ± 1.50 | +15.9 ± 1.68 | +17.6 ± 1.49 | +14.4 ± 1.75 | +14.5 ± 1.46 | +14.9 ± 1.90 | +12.7 ± 1.13 | +19.4 ± 1.66 |
| Seed 22, eighth, mirrored | 36 | +15.4 ± 1.01 | +18.5 ± 0.77 | +17.8 ± 1.23 | +19.6 ± 1.06 | +16.8 ± 0.83 | +16.3 ± 0.84 | +16.8 ± 0.88 | +16.1 ± 1.18 | +20.8 ± 0.64 |
| Seed 22, first mapping, assignment | 6 | +4.2 ± 1.06 | +4.4 ± 0.89 | +3.9 ± 1.16 | +4.1 ± 0.82 | +4.3 ± 1.22 | +4.4 ± 1.10 | +4.3 ± 0.87 | +3.9 ± 1.00 | +4.1 ± 0.83 |
| Seed 22, first mapping, assignment | 12 | +2.3 ± 0.55 | +3.4 ± 0.70 | +2.5 ± 0.60 | +2.9 ± 0.56 | +2.6 ± 0.74 | +1.0 ± 1.21 | +3.0 ± 0.75 | +2.2 ± 0.59 | +3.6 ± 0.67 |
| Seed 22, first mapping, assignment | 18 | +6.8 ± 0.67 | +8.5 ± 0.92 | +7.9 ± 0.68 | +6.8 ± 1.12 | +6.4 ± 1.05 | +6.1 ± 1.11 | +7.9 ± 1.73 | +6.7 ± 0.99 | +8.0 ± 1.08 |
| Seed 22, first mapping, assignment | 24 | +7.6 ± 1.16 | +11.2 ± 1.02 | +8.2 ± 1.50 | +9.1 ± 1.37 | +8.4 ± 1.05 | +6.7 ± 1.33 | +11.0 ± 1.11 | +6.9 ± 1.40 | +9.9 ± 1.55 |
| Seed 22, first mapping, assignment | 36 | +6.4 ± 1.01 | +9.9 ± 1.13 | +7.0 ± 0.80 | +7.9 ± 0.51 | +6.5 ± 1.42 | +5.2 ± 1.24 | +10.2 ± 1.20 | +6.0 ± 0.78 | +8.3 ± 0.29 |
| Seed 23, eighth, mirrored | 6 | +1.5 ± 1.07 | +1.4 ± 1.08 | +1.2 ± 1.01 | +1.7 ± 1.20 | +1.0 ± 1.17 | +2.4 ± 1.08 | +2.1 ± 1.34 | +1.7 ± 1.19 | +1.4 ± 1.05 |
| Seed 23, eighth, mirrored | 12 | +6.5 ± 0.98 | +8.5 ± 0.82 | +6.2 ± 0.74 | +9.0 ± 1.07 | +6.8 ± 0.77 | +6.0 ± 0.97 | +8.0 ± 0.92 | +6.6 ± 0.58 | +8.1 ± 0.67 |
| Seed 23, eighth, mirrored | 18 | +9.5 ± 1.36 | +11.6 ± 2.36 | +9.2 ± 1.45 | +12.7 ± 1.02 | +8.6 ± 1.29 | +9.1 ± 1.46 | +11.6 ± 0.60 | +9.8 ± 1.21 | +12.1 ± 0.97 |
| Seed 23, eighth, mirrored | 24 | +15.9 ± 1.64 | +17.0 ± 2.24 | +15.3 ± 1.49 | +18.7 ± 1.62 | +15.4 ± 1.64 | +14.6 ± 1.96 | +17.4 ± 1.16 | +15.3 ± 1.70 | +17.9 ± 1.93 |
| Seed 23, eighth, mirrored | 36 | +15.0 ± 0.99 | +16.6 ± 1.20 | +16.5 ± 1.63 | +17.4 ± 1.08 | +15.5 ± 1.06 | +14.7 ± 1.54 | +17.0 ± 0.92 | +15.3 ± 0.91 | +17.0 ± 0.69 |
| Seed 23, first mapping, assignment | 6 | +2.3 ± 0.81 | +2.0 ± 0.82 | +2.3 ± 0.88 | +2.0 ± 0.79 | +2.1 ± 0.89 | +1.9 ± 0.87 | +2.2 ± 0.80 | +2.1 ± 0.82 | +1.9 ± 0.79 |
| Seed 23, first mapping, assignment | 12 | +5.6 ± 1.00 | +4.5 ± 1.04 | +4.3 ± 1.80 | +4.1 ± 1.18 | +5.5 ± 1.09 | +4.1 ± 0.95 | +4.7 ± 1.05 | +4.2 ± 1.13 | +4.6 ± 1.37 |
| Seed 23, first mapping, assignment | 18 | +7.7 ± 2.10 | +7.7 ± 2.36 | +6.9 ± 1.96 | +7.9 ± 2.54 | +7.6 ± 2.55 | +7.9 ± 2.40 | +7.2 ± 1.96 | +7.2 ± 2.48 | +7.0 ± 2.30 |
| Seed 23, first mapping, assignment | 24 | +10.3 ± 1.75 | +10.0 ± 1.98 | +9.5 ± 2.01 | +10.0 ± 1.90 | +11.0 ± 1.93 | +10.3 ± 2.20 | +9.3 ± 1.67 | +9.9 ± 2.08 | +9.3 ± 2.12 |
| Seed 23, first mapping, assignment | 36 | +12.8 ± 1.46 | +13.1 ± 1.81 | +13.2 ± 1.45 | +13.4 ± 1.56 | +13.3 ± 1.64 | +12.6 ± 1.56 | +12.7 ± 1.26 | +12.6 ± 1.64 | +12.9 ± 1.65 |
| Seed 24, eighth, mirrored | 6 | −1.5 ± 1.25 | −2.3 ± 1.44 | −1.3 ± 1.06 | −2.6 ± 1.35 | −1.3 ± 1.35 | +0.3 ± 1.15 | −3.0 ± 1.32 | −1.5 ± 1.27 | −2.4 ± 1.16 |
| Seed 24, eighth, mirrored | 12 | +0.7 ± 1.29 | −1.2 ± 1.34 | +1.6 ± 1.44 | −2.9 ± 1.19 | +2.3 ± 1.32 | +2.1 ± 1.50 | −2.6 ± 1.25 | −0.2 ± 1.33 | −1.9 ± 1.08 |
| Seed 24, eighth, mirrored | 18 | +12.4 ± 1.47 | +11.8 ± 1.12 | +13.2 ± 1.67 | +8.3 ± 1.70 | +15.0 ± 1.31 | +14.2 ± 1.34 | +9.9 ± 1.19 | +10.5 ± 1.54 | +9.1 ± 1.86 |
| Seed 24, eighth, mirrored | 24 | +13.1 ± 1.72 | +11.7 ± 2.49 | +15.3 ± 1.92 | +9.4 ± 1.58 | +16.5 ± 1.98 | +17.5 ± 1.30 | +8.7 ± 1.93 | +12.3 ± 2.40 | +10.5 ± 2.30 |
| Seed 24, eighth, mirrored | 36 | +14.9 ± 0.99 | +13.3 ± 2.62 | +16.4 ± 1.39 | +11.3 ± 0.96 | +15.5 ± 1.27 | +16.7 ± 1.23 | +11.5 ± 1.28 | +12.9 ± 1.03 | +12.0 ± 1.87 |
| Seed 24, first mapping, assignment | 6 | +3.3 ± 0.70 | +3.0 ± 0.69 | +3.2 ± 0.68 | +2.9 ± 0.72 | +3.4 ± 0.71 | +2.8 ± 0.68 | +3.0 ± 0.70 | +3.3 ± 0.68 | +3.2 ± 0.71 |
| Seed 24, first mapping, assignment | 12 | +3.8 ± 0.93 | +4.7 ± 1.13 | +3.2 ± 0.79 | +2.8 ± 0.93 | +5.0 ± 1.06 | +3.7 ± 1.16 | +3.8 ± 0.77 | +2.9 ± 0.96 | +2.8 ± 0.78 |
| Seed 24, first mapping, assignment | 18 | +4.2 ± 0.75 | +4.6 ± 1.94 | +5.0 ± 1.14 | +3.9 ± 1.41 | +5.0 ± 1.01 | +5.0 ± 1.19 | +5.0 ± 1.55 | +4.5 ± 0.89 | +4.0 ± 1.41 |
| Seed 24, first mapping, assignment | 24 | +8.5 ± 2.02 | +10.3 ± 2.67 | +9.6 ± 2.59 | +8.7 ± 3.17 | +10.0 ± 2.04 | +9.0 ± 2.46 | +9.4 ± 2.85 | +9.4 ± 2.77 | +9.5 ± 2.45 |
| Seed 24, first mapping, assignment | 36 | +5.2 ± 1.65 | +5.2 ± 0.59 | +6.1 ± 1.12 | +3.8 ± 1.57 | +6.1 ± 1.46 | +5.8 ± 1.60 | +4.4 ± 1.15 | +6.5 ± 1.02 | +4.8 ± 1.56 |
| Seed 23, eighth, assignment | 6 | +3.3 ± 0.91 | +2.6 ± 1.05 | +3.6 ± 0.89 | +3.0 ± 0.96 | +3.0 ± 1.00 | +3.4 ± 1.07 | +2.8 ± 0.97 | +3.2 ± 0.98 | +2.8 ± 1.06 |
| Seed 23, eighth, assignment | 12 | +5.5 ± 1.27 | +2.8 ± 1.74 | +5.1 ± 1.01 | +2.6 ± 0.94 | +6.4 ± 1.53 | +6.7 ± 1.57 | +3.3 ± 1.21 | +5.0 ± 1.12 | +2.7 ± 1.37 |
| Seed 23, eighth, assignment | 18 | +10.5 ± 1.19 | +8.0 ± 1.35 | +11.8 ± 1.24 | +7.9 ± 0.90 | +12.7 ± 0.97 | +12.7 ± 1.45 | +7.4 ± 1.40 | +10.5 ± 1.00 | +11.2 ± 1.25 |
| Seed 23, eighth, assignment | 24 | +11.1 ± 1.57 | +9.5 ± 1.67 | +13.3 ± 1.72 | +5.7 ± 1.34 | +14.1 ± 1.57 | +13.0 ± 2.14 | +6.9 ± 2.16 | +9.6 ± 2.10 | +9.3 ± 1.33 |
| Seed 23, eighth, assignment | 36 | +7.4 ± 1.46 | +6.3 ± 1.84 | +9.3 ± 1.35 | +4.1 ± 1.45 | +10.2 ± 1.77 | +10.9 ± 2.49 | +4.5 ± 2.81 | +6.7 ± 2.53 | +6.5 ± 1.93 |
| Seed 24, first mapping, mirrored | 6 | +3.1 ± 0.82 | +2.8 ± 0.85 | +3.2 ± 0.79 | +3.4 ± 0.79 | +2.9 ± 0.83 | +3.6 ± 0.88 | +2.8 ± 0.82 | +3.1 ± 0.82 | +3.2 ± 0.79 |
| Seed 24, first mapping, mirrored | 12 | +4.4 ± 1.15 | +6.1 ± 1.91 | +3.5 ± 1.63 | +4.7 ± 1.50 | +4.4 ± 1.70 | +6.0 ± 1.30 | +4.9 ± 1.51 | +3.7 ± 1.31 | +4.4 ± 1.52 |
| Seed 24, first mapping, mirrored | 18 | +6.9 ± 1.01 | +9.6 ± 0.93 | +6.5 ± 1.15 | +7.8 ± 0.89 | +7.5 ± 0.84 | +9.3 ± 0.89 | +8.3 ± 0.78 | +7.0 ± 1.07 | +8.0 ± 0.99 |
| Seed 24, first mapping, mirrored | 24 | +7.4 ± 1.19 | +9.9 ± 0.92 | +7.6 ± 1.74 | +7.6 ± 1.32 | +8.9 ± 1.12 | +9.3 ± 1.35 | +8.7 ± 0.94 | +7.8 ± 1.69 | +8.2 ± 1.09 |
| Seed 24, first mapping, mirrored | 36 | +6.2 ± 1.66 | +7.9 ± 1.69 | +7.1 ± 1.23 | +6.0 ± 1.54 | +7.7 ± 1.37 | +7.4 ± 1.09 | +7.0 ± 1.77 | +6.7 ± 1.57 | +7.0 ± 1.46 |
| Seed 23, first mapping, mirrored | 6 | +1.4 ± 0.81 | +1.6 ± 0.68 | +1.7 ± 0.85 | +1.9 ± 0.68 | +1.6 ± 0.76 | +1.5 ± 0.76 | +1.5 ± 0.76 | +1.9 ± 0.83 | +1.8 ± 0.64 |
| Seed 23, first mapping, mirrored | 12 | +2.3 ± 1.14 | +1.8 ± 1.09 | +3.4 ± 1.19 | +2.0 ± 1.26 | +2.4 ± 0.97 | +3.1 ± 1.21 | +1.1 ± 1.28 | +3.3 ± 1.14 | +2.8 ± 1.12 |
| Seed 23, first mapping, mirrored | 18 | +1.7 ± 0.89 | −0.7 ± 0.85 | +1.6 ± 1.40 | +0.3 ± 1.17 | +1.8 ± 1.04 | +2.2 ± 1.23 | −0.5 ± 1.15 | +2.4 ± 0.94 | +0.1 ± 1.20 |
| Seed 23, first mapping, mirrored | 24 | −1.5 ± 0.97 | −3.0 ± 0.66 | −0.6 ± 0.59 | −2.6 ± 1.30 | −0.9 ± 0.64 | −0.1 ± 0.80 | −3.7 ± 0.42 | −1.0 ± 1.37 | −2.3 ± 1.40 |
| Seed 23, first mapping, mirrored | 36 | −2.9 ± 0.96 | −4.0 ± 1.10 | −2.7 ± 0.53 | −4.9 ± 1.09 | −2.6 ± 0.65 | −1.7 ± 0.42 | −4.6 ± 0.95 | −3.0 ± 0.78 | −4.7 ± 0.66 |
| Seed 24, eighth, assignment | 6 | −1.0 ± 0.72 | −0.4 ± 0.82 | 0.0 ± 0.74 | +0.3 ± 0.64 | −0.5 ± 0.77 | +0.5 ± 0.67 | −1.2 ± 0.65 | 0.0 ± 0.90 | 0.0 ± 0.83 |
| Seed 24, eighth, assignment | 12 | −0.4 ± 1.24 | +0.7 ± 1.22 | +0.6 ± 1.08 | −0.6 ± 1.03 | −0.3 ± 1.17 | +2.0 ± 0.89 | −1.4 ± 1.51 | −0.8 ± 1.01 | +0.3 ± 1.09 |
| Seed 24, eighth, assignment | 18 | −4.1 ± 1.93 | −3.1 ± 1.29 | −2.7 ± 2.28 | −5.9 ± 1.53 | −2.9 ± 2.14 | +1.2 ± 2.06 | −4.6 ± 1.46 | −4.4 ± 2.52 | −2.7 ± 1.37 |
| Seed 24, eighth, assignment | 24 | −6.6 ± 1.89 | −3.3 ± 1.31 | −2.9 ± 2.84 | −5.9 ± 1.85 | −3.1 ± 2.59 | +1.1 ± 1.88 | −5.9 ± 1.78 | −5.8 ± 2.36 | −3.8 ± 2.51 |
| Seed 24, eighth, assignment | 36 | +1.0 ± 0.72 | +1.6 ± 1.09 | +5.6 ± 1.68 | +1.8 ± 1.79 | +4.1 ± 1.17 | +8.8 ± 1.28 | +0.7 ± 1.29 | +1.4 ± 0.82 | +3.5 ± 2.31 |
| Seed 22, first mapping, mirrored | 6 | +1.6 ± 0.59 | +1.2 ± 0.43 | +1.5 ± 0.51 | +1.1 ± 0.63 | +1.6 ± 0.52 | +1.5 ± 0.59 | +1.3 ± 0.52 | +1.5 ± 0.61 | +0.7 ± 0.65 |
| Seed 22, first mapping, mirrored | 12 | +3.5 ± 1.85 | +3.6 ± 1.82 | +3.5 ± 1.99 | +2.9 ± 1.93 | +3.5 ± 1.96 | +3.2 ± 1.36 | +3.1 ± 1.87 | +3.6 ± 1.84 | +4.3 ± 2.02 |
| Seed 22, first mapping, mirrored | 18 | +3.8 ± 2.22 | +3.6 ± 2.29 | +4.1 ± 2.27 | +3.7 ± 2.22 | +4.3 ± 2.29 | +3.4 ± 2.15 | +3.5 ± 2.13 | +4.4 ± 2.35 | +5.1 ± 1.89 |
| Seed 22, first mapping, mirrored | 24 | +5.2 ± 2.08 | +5.6 ± 2.17 | +6.1 ± 1.90 | +4.7 ± 2.24 | +5.9 ± 2.04 | +4.8 ± 1.82 | +5.4 ± 2.39 | +4.9 ± 2.27 | +6.4 ± 2.05 |
| Seed 22, first mapping, mirrored | 36 | +5.1 ± 1.72 | +5.7 ± 1.53 | +6.2 ± 1.66 | +5.0 ± 1.24 | +5.6 ± 1.68 | +4.7 ± 1.52 | +5.4 ± 1.25 | +4.8 ± 1.23 | +6.0 ± 1.15 |
| Seed 22, eighth, assignment | 6 | +2.5 ± 0.84 | +1.7 ± 0.87 | +2.5 ± 0.56 | +1.9 ± 0.65 | +2.3 ± 0.98 | +2.3 ± 0.81 | +1.9 ± 0.75 | +2.1 ± 0.65 | +2.4 ± 0.70 |
| Seed 22, eighth, assignment | 12 | +5.0 ± 0.83 | +4.2 ± 0.87 | +5.3 ± 0.74 | +3.0 ± 0.93 | +5.3 ± 0.82 | +5.2 ± 0.79 | +3.5 ± 0.95 | +4.0 ± 0.72 | +3.8 ± 0.81 |
| Seed 22, eighth, assignment | 18 | +4.2 ± 0.92 | +4.8 ± 1.10 | +5.7 ± 1.00 | +3.4 ± 1.05 | +4.6 ± 0.80 | +4.6 ± 1.28 | +3.1 ± 1.23 | +4.4 ± 0.77 | +5.0 ± 0.59 |
| Seed 22, eighth, assignment | 24 | +2.9 ± 0.85 | +4.8 ± 1.48 | +6.1 ± 1.12 | +2.8 ± 0.89 | +5.2 ± 1.18 | +4.8 ± 1.01 | +2.4 ± 1.26 | +3.5 ± 1.13 | +4.2 ± 1.45 |
| Seed 22, eighth, assignment | 36 | +1.7 ± 1.43 | +4.0 ± 0.95 | +4.1 ± 1.19 | +2.2 ± 0.65 | +4.5 ± 1.26 | +4.6 ± 0.66 | +1.5 ± 1.08 | +3.3 ± 0.36 | +4.2 ± 0.84 |

- **Each held night less the night without replay**: the night held onto the own group and the inhibitory units lies between −6.6 and
  +15.9, beyond twice its SE at 48 of the 60 checkpoints, 45 above and 3 below; the night held onto the own group and the units outside the
  readouts between −4.0 and +18.5, beyond at 48, 44 above and 4 below; and the night held onto the inhibitory units and the units outside
  the readouts between −2.9 and +17.8, beyond at 49, 48 above and 1 below. H-47's plain night lies beyond at 49 of the 60, 45 above and 4
  below, exploration 027's night held within at 49, 48 above and 1 below, and the excitatory night at 51, 50 above and 1 below.
- **Below the night without replay**, every held night lies only on seed 23's first mapping from the mirrored assignment and seed 24's
  eighth image from the assignment: the night held onto the own group and the inhibitory units on the first at block 36, −2.9 ± 0.96, and on
  the second at blocks 18 and 24, −4.1 ± 1.93 and −6.6 ± 1.89; the night held onto the own group and the units outside the readouts on the
  first at blocks 24 and 36, −3.0 ± 0.66 and −4.0 ± 1.10, and on the second at blocks 18 and 24, −3.1 ± 1.29 and −3.3 ± 1.31; and the night
  held onto the inhibitory units and the units outside the readouts on the first at block 36, −2.7 ± 0.53. H-47's plain night lies below on
  seed 24's eighth image from the mirrored assignment at block 12, −2.9 ± 1.19, on seed 23's first mapping from the mirrored assignment at
  block 36, −4.9 ± 1.09, and on seed 24's eighth image from the assignment at blocks 18 and 24, −5.9 ± 1.53 and −5.9 ± 1.85.

The three held nights less each other, with the job's paired SE:

| Cell | Block | Own group and inhibitory units less own group and outside | Own group and inhibitory units less inhibitory units and outside | Own group and outside less inhibitory units and outside |
| :--- | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | −0.2 ± 0.55 | −0.3 ± 0.35 | −0.1 ± 0.37 |
| Seed 22, eighth, mirrored | 12 | −1.6 ± 0.49 | −1.8 ± 0.41 | −0.2 ± 0.70 |
| Seed 22, eighth, mirrored | 18 | −0.3 ± 0.57 | −0.9 ± 0.62 | −0.6 ± 0.79 |
| Seed 22, eighth, mirrored | 24 | −4.1 ± 0.86 | −3.8 ± 0.62 | +0.3 ± 0.64 |
| Seed 22, eighth, mirrored | 36 | −3.2 ± 0.60 | −2.5 ± 0.85 | +0.7 ± 1.14 |
| Seed 22, first mapping, assignment | 6 | −0.2 ± 0.24 | +0.3 ± 0.27 | +0.5 ± 0.41 |
| Seed 22, first mapping, assignment | 12 | −1.2 ± 0.60 | −0.2 ± 0.63 | +0.9 ± 0.70 |
| Seed 22, first mapping, assignment | 18 | −1.6 ± 0.33 | −1.1 ± 0.42 | +0.6 ± 0.16 |
| Seed 22, first mapping, assignment | 24 | −3.5 ± 0.06 | −0.6 ± 0.70 | +3.0 ± 0.73 |
| Seed 22, first mapping, assignment | 36 | −3.6 ± 1.07 | −0.6 ± 0.36 | +3.0 ± 0.75 |
| Seed 23, eighth, mirrored | 6 | +0.1 ± 0.46 | +0.4 ± 0.47 | +0.2 ± 0.48 |
| Seed 23, eighth, mirrored | 12 | −2.0 ± 0.99 | +0.3 ± 1.17 | +2.3 ± 0.63 |
| Seed 23, eighth, mirrored | 18 | −2.1 ± 1.60 | +0.4 ± 0.41 | +2.4 ± 1.40 |
| Seed 23, eighth, mirrored | 24 | −1.1 ± 0.80 | +0.6 ± 0.71 | +1.7 ± 1.11 |
| Seed 23, eighth, mirrored | 36 | −1.6 ± 0.42 | −1.5 ± 0.76 | +0.1 ± 0.48 |
| Seed 23, first mapping, assignment | 6 | +0.3 ± 0.31 | −0.1 ± 0.19 | −0.3 ± 0.39 |
| Seed 23, first mapping, assignment | 12 | +1.1 ± 0.31 | +1.3 ± 1.30 | +0.2 ± 1.24 |
| Seed 23, first mapping, assignment | 18 | 0.0 ± 0.31 | +0.8 ± 0.57 | +0.8 ± 0.83 |
| Seed 23, first mapping, assignment | 24 | +0.3 ± 0.53 | +0.8 ± 0.41 | +0.5 ± 0.54 |
| Seed 23, first mapping, assignment | 36 | −0.4 ± 0.42 | −0.4 ± 0.29 | −0.1 ± 0.49 |
| Seed 24, eighth, mirrored | 6 | +0.7 ± 0.19 | −0.2 ± 0.25 | −0.9 ± 0.45 |
| Seed 24, eighth, mirrored | 12 | +1.8 ± 0.65 | −1.0 ± 0.52 | −2.8 ± 0.95 |
| Seed 24, eighth, mirrored | 18 | +0.6 ± 1.34 | −0.8 ± 2.00 | −1.4 ± 0.96 |
| Seed 24, eighth, mirrored | 24 | +1.4 ± 1.11 | −2.2 ± 1.03 | −3.5 ± 0.87 |
| Seed 24, eighth, mirrored | 36 | +1.7 ± 1.88 | −1.5 ± 0.76 | −3.2 ± 1.33 |
| Seed 24, first mapping, assignment | 6 | +0.3 ± 0.06 | +0.1 ± 0.09 | −0.2 ± 0.09 |
| Seed 24, first mapping, assignment | 12 | −0.9 ± 0.71 | +0.6 ± 0.18 | +1.4 ± 0.61 |
| Seed 24, first mapping, assignment | 18 | −0.4 ± 1.34 | −0.8 ± 0.62 | −0.4 ± 0.98 |
| Seed 24, first mapping, assignment | 24 | −1.8 ± 1.13 | −1.1 ± 1.12 | +0.7 ± 0.46 |
| Seed 24, first mapping, assignment | 36 | 0.0 ± 1.19 | −1.0 ± 1.21 | −1.0 ± 0.51 |
| Seed 23, eighth, assignment | 6 | +0.6 ± 0.48 | −0.3 ± 0.27 | −1.0 ± 0.23 |
| Seed 23, eighth, assignment | 12 | +2.7 ± 0.76 | +0.4 ± 1.14 | −2.4 ± 1.33 |
| Seed 23, eighth, assignment | 18 | +2.5 ± 0.79 | −1.3 ± 0.57 | −3.9 ± 1.07 |
| Seed 23, eighth, assignment | 24 | +1.7 ± 0.84 | −2.1 ± 0.61 | −3.8 ± 0.60 |
| Seed 23, eighth, assignment | 36 | +1.1 ± 0.65 | −1.9 ± 0.34 | −3.0 ± 0.78 |
| Seed 24, first mapping, mirrored | 6 | +0.3 ± 0.24 | −0.1 ± 0.11 | −0.5 ± 0.22 |
| Seed 24, first mapping, mirrored | 12 | −1.7 ± 1.22 | +0.9 ± 0.85 | +2.6 ± 1.06 |
| Seed 24, first mapping, mirrored | 18 | −2.7 ± 0.78 | +0.4 ± 0.55 | +3.1 ± 0.62 |
| Seed 24, first mapping, mirrored | 24 | −2.5 ± 0.52 | −0.3 ± 0.80 | +2.2 ± 1.14 |
| Seed 24, first mapping, mirrored | 36 | −1.8 ± 0.16 | −0.9 ± 0.54 | +0.9 ± 0.64 |
| Seed 23, first mapping, mirrored | 6 | −0.2 ± 0.31 | −0.3 ± 0.14 | −0.1 ± 0.29 |
| Seed 23, first mapping, mirrored | 12 | +0.6 ± 0.75 | −1.1 ± 0.53 | −1.6 ± 0.52 |
| Seed 23, first mapping, mirrored | 18 | +2.4 ± 0.27 | +0.1 ± 0.83 | −2.3 ± 0.88 |
| Seed 23, first mapping, mirrored | 24 | +1.5 ± 0.87 | −0.8 ± 0.90 | −2.4 ± 0.74 |
| Seed 23, first mapping, mirrored | 36 | +1.1 ± 0.79 | −0.2 ± 0.72 | −1.3 ± 0.74 |
| Seed 24, eighth, assignment | 6 | −0.6 ± 0.04 | −1.0 ± 0.15 | −0.4 ± 0.32 |
| Seed 24, eighth, assignment | 12 | −1.1 ± 0.45 | −1.1 ± 0.92 | +0.1 ± 0.94 |
| Seed 24, eighth, assignment | 18 | −0.9 ± 0.98 | −1.4 ± 0.54 | −0.4 ± 1.46 |
| Seed 24, eighth, assignment | 24 | −3.3 ± 1.34 | −3.6 ± 1.13 | −0.3 ± 2.01 |
| Seed 24, eighth, assignment | 36 | −0.6 ± 1.27 | −4.6 ± 1.59 | −4.0 ± 2.07 |
| Seed 22, first mapping, mirrored | 6 | +0.5 ± 0.25 | +0.2 ± 0.25 | −0.3 ± 0.13 |
| Seed 22, first mapping, mirrored | 12 | −0.1 ± 0.14 | 0.0 ± 0.27 | +0.1 ± 0.55 |
| Seed 22, first mapping, mirrored | 18 | +0.2 ± 0.23 | −0.3 ± 0.31 | −0.6 ± 0.33 |
| Seed 22, first mapping, mirrored | 24 | −0.4 ± 0.45 | −0.8 ± 0.38 | −0.5 ± 0.35 |
| Seed 22, first mapping, mirrored | 36 | −0.6 ± 0.48 | −1.1 ± 0.31 | −0.5 ± 0.46 |
| Seed 22, eighth, assignment | 6 | +0.8 ± 0.21 | 0.0 ± 0.58 | −0.9 ± 0.55 |
| Seed 22, eighth, assignment | 12 | +0.8 ± 0.46 | −0.3 ± 0.41 | −1.1 ± 0.49 |
| Seed 22, eighth, assignment | 18 | −0.6 ± 0.82 | −1.5 ± 0.55 | −0.9 ± 1.19 |
| Seed 22, eighth, assignment | 24 | −1.9 ± 1.01 | −3.2 ± 0.66 | −1.3 ± 1.21 |
| Seed 22, eighth, assignment | 36 | −2.3 ± 1.12 | −2.4 ± 1.07 | −0.1 ± 1.59 |

- **The night held onto the own group and the inhibitory units less the night held onto the own group and the units outside the
  readouts** lies between −4.1 and +2.7, beyond twice its SE at 23 of the 60 checkpoints, 8 above and 15 below. Above: on seed 23's first
  mapping from the assignment at block 12, +1.1 ± 0.31; on seed 24's eighth image from the mirrored assignment at blocks 6 and 12,
  +0.7 ± 0.19 and +1.8 ± 0.65; on seed 24's first mapping from the assignment at block 6, +0.3 ± 0.06; on seed 23's eighth image from the
  assignment at blocks 12 and 18, +2.7 ± 0.76 and +2.5 ± 0.79; on seed 23's first mapping from the mirrored assignment at block 18,
  +2.4 ± 0.27; and on seed 22's eighth image from the assignment at block 6, +0.8 ± 0.21. Below: on seed 22's eighth image from the mirrored
  assignment at blocks 12, 24 and 36, −1.6 ± 0.49 to −4.1 ± 0.86; on seed 22's first mapping from the assignment from block 18 on,
  −1.6 ± 0.33 to −3.6 ± 1.07; on seed 23's eighth image from the mirrored assignment at blocks 12 and 36, −2.0 ± 0.99 and −1.6 ± 0.42; on
  seed 24's first mapping from the mirrored assignment from block 18 on, −1.8 ± 0.16 to −2.7 ± 0.78; on seed 24's eighth image from the
  assignment at blocks 6, 12 and 24, −0.6 ± 0.04 to −3.3 ± 1.34; and on seed 22's eighth image from the assignment at block 36,
  −2.3 ± 1.12.
- **The night held onto the own group and the inhibitory units less the night held onto the inhibitory units and the units outside the
  readouts** lies between −4.6 and +1.3, beyond twice its SE at 21 of the 60, 1 above and 20 below. Above: on seed 24's first mapping from
  the assignment at block 12, +0.6 ± 0.18. Below: on seed 22's eighth image from the mirrored assignment at blocks 12, 24 and 36,
  −1.8 ± 0.41 to −3.8 ± 0.62; on seed 22's first mapping from the assignment at block 18, −1.1 ± 0.42; on seed 23's eighth image from the
  mirrored assignment at block 36, −1.5 ± 0.76; on seed 24's eighth image from the mirrored assignment at block 24, −2.2 ± 1.03; on seed
  23's eighth image from the assignment from block 18 on, −1.3 ± 0.57 to −2.1 ± 0.61; on seed 23's first mapping from the mirrored
  assignment at blocks 6 and 12, −0.3 ± 0.14 and −1.1 ± 0.53; on seed 24's eighth image from the assignment at blocks 6, 18, 24 and 36,
  −1.0 ± 0.15 to −4.6 ± 1.59; on seed 22's first mapping from the mirrored assignment at blocks 24 and 36, −0.8 ± 0.38 and −1.1 ± 0.31; and
  on seed 22's eighth image from the assignment from block 18 on, −1.5 ± 0.55 to −3.2 ± 0.66.
- **The night held onto the own group and the units outside the readouts less the night held onto the inhibitory units and the units
  outside the readouts** lies between −4.0 and +3.1, beyond twice its SE at 22 of the 60, 7 above and 15 below. Above: on seed 22's first
  mapping from the assignment from block 18 on, +0.6 ± 0.16 to +3.0 ± 0.75; on seed 23's eighth image from the mirrored assignment at block
  12, +2.3 ± 0.63; on seed 24's first mapping from the assignment at block 12, +1.4 ± 0.61; and on seed 24's first mapping from the mirrored
  assignment at blocks 12 and 18, +2.6 ± 1.06 and +3.1 ± 0.62. Below: on seed 24's eighth image from the mirrored assignment at blocks 6,
  12, 24 and 36, −0.9 ± 0.45 to −3.5 ± 0.87; on seed 24's first mapping from the assignment at block 6, −0.2 ± 0.09; on seed 23's eighth
  image from the assignment at blocks 6, 18, 24 and 36, −1.0 ± 0.23 to −3.9 ± 1.07; on seed 24's first mapping from the mirrored assignment
  at block 6, −0.5 ± 0.22; on seed 23's first mapping from the mirrored assignment at blocks 12, 18 and 24, −1.6 ± 0.52 to −2.4 ± 0.74; on
  seed 22's first mapping from the mirrored assignment at block 6, −0.3 ± 0.13; and on seed 22's eighth image from the assignment at block
  12, −1.1 ± 0.49.

Each held night less H-47's plain night, less exploration 027's night held within and less the excitatory night, each from another job,
with exploration 018's unpaired SE:

| Cell | Block | Own group and inhibitory units less H-47's plain | less 027's held within | less excitatory | Own group and outside the readouts less H-47's plain | less 027's held within | less excitatory | Inhibitory units and outside the readouts less H-47's plain | less 027's held within | less excitatory |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | −0.6 ± 0.81 | +0.1 ± 0.75 | +0.3 ± 0.80 | −0.4 ± 0.69 | +0.4 ± 0.69 | +0.5 ± 0.85 | −0.3 ± 0.68 | +0.4 ± 0.64 | +0.6 ± 0.73 |
| Seed 22, eighth, mirrored | 12 | −2.0 ± 1.33 | −0.6 ± 1.18 | −0.7 ± 1.56 | −0.5 ± 1.21 | +0.9 ± 1.05 | +0.8 ± 1.41 | −0.3 ± 1.21 | +1.1 ± 1.15 | +1.0 ± 1.43 |
| Seed 22, eighth, mirrored | 18 | −1.7 ± 0.77 | −0.4 ± 0.84 | −0.8 ± 1.05 | −1.4 ± 0.94 | −0.1 ± 0.98 | −0.5 ± 1.29 | −0.8 ± 0.84 | +0.5 ± 0.95 | +0.1 ± 1.01 |
| Seed 22, eighth, mirrored | 24 | −5.5 ± 1.37 | −2.3 ± 1.16 | −2.4 ± 1.20 | −1.4 ± 0.99 | +1.8 ± 0.99 | +1.7 ± 1.63 | −1.7 ± 1.30 | +1.5 ± 1.06 | +1.4 ± 1.56 |
| Seed 22, eighth, mirrored | 36 | −4.3 ± 0.73 | −1.4 ± 1.02 | −0.9 ± 1.68 | −1.1 ± 0.41 | +1.8 ± 0.82 | +2.3 ± 1.50 | −1.8 ± 0.94 | +1.1 ± 1.01 | +1.6 ± 1.45 |
| Seed 22, first mapping, assignment | 6 | +0.1 ± 0.64 | −0.1 ± 0.57 | −0.2 ± 0.50 | +0.3 ± 0.51 | +0.1 ± 0.56 | 0.0 ± 0.41 | −0.2 ± 0.82 | −0.4 ± 0.40 | −0.5 ± 0.47 |
| Seed 22, first mapping, assignment | 12 | −0.6 ± 1.30 | −0.3 ± 1.21 | +1.2 ± 1.34 | +0.6 ± 1.32 | +0.9 ± 1.24 | +2.4 ± 1.37 | −0.4 ± 1.20 | −0.1 ± 1.46 | +1.4 ± 1.41 |
| Seed 22, first mapping, assignment | 18 | +0.1 ± 1.14 | +0.4 ± 1.11 | +0.8 ± 1.18 | +1.7 ± 1.21 | +2.0 ± 1.22 | +2.4 ± 1.15 | +1.1 ± 1.19 | +1.5 ± 1.11 | +1.9 ± 1.13 |
| Seed 22, first mapping, assignment | 24 | −1.5 ± 1.29 | −0.7 ± 1.03 | +0.9 ± 1.03 | +2.0 ± 1.28 | +2.8 ± 0.97 | +4.4 ± 1.02 | −0.9 ± 1.35 | −0.2 ± 1.56 | +1.5 ± 1.16 |
| Seed 22, first mapping, assignment | 36 | −1.5 ± 1.18 | −0.1 ± 0.96 | +1.2 ± 1.03 | +2.0 ± 0.89 | +3.4 ± 1.43 | +4.7 ± 0.92 | −1.0 ± 0.89 | +0.5 ± 1.02 | +1.8 ± 0.82 |
| Seed 23, eighth, mirrored | 6 | −0.2 ± 1.01 | +0.5 ± 1.04 | −0.9 ± 0.92 | −0.3 ± 1.08 | +0.4 ± 1.17 | −1.0 ± 1.09 | −0.5 ± 1.05 | +0.2 ± 1.01 | −1.2 ± 1.00 |
| Seed 23, eighth, mirrored | 12 | −2.5 ± 0.95 | −0.3 ± 1.40 | +0.5 ± 1.32 | −0.5 ± 1.42 | +1.7 ± 1.50 | +2.5 ± 1.49 | −2.8 ± 1.64 | −0.6 ± 1.47 | +0.2 ± 1.64 |
| Seed 23, eighth, mirrored | 18 | −3.2 ± 1.19 | +1.0 ± 1.34 | +0.4 ± 1.55 | −1.1 ± 1.80 | +3.0 ± 1.98 | +2.5 ± 2.11 | −3.5 ± 1.12 | +0.6 ± 1.30 | 0.0 ± 1.50 |
| Seed 23, eighth, mirrored | 24 | −2.8 ± 0.94 | +0.5 ± 0.97 | +1.3 ± 1.11 | −1.6 ± 1.13 | +1.7 ± 1.15 | +2.4 ± 1.08 | −3.4 ± 1.06 | −0.1 ± 1.01 | +0.7 ± 1.40 |
| Seed 23, eighth, mirrored | 36 | −2.4 ± 1.15 | −0.5 ± 0.92 | +0.3 ± 1.32 | −0.8 ± 1.06 | +1.1 ± 0.79 | +1.9 ± 1.06 | −0.9 ± 1.21 | +1.0 ± 0.93 | +1.8 ± 1.03 |
| Seed 23, first mapping, assignment | 6 | +0.2 ± 0.85 | +0.2 ± 0.84 | +0.4 ± 0.83 | 0.0 ± 0.74 | −0.1 ± 0.82 | +0.2 ± 0.82 | +0.3 ± 0.86 | +0.3 ± 0.82 | +0.5 ± 0.82 |
| Seed 23, first mapping, assignment | 12 | +1.5 ± 1.12 | +0.1 ± 0.99 | +1.6 ± 1.03 | +0.4 ± 1.08 | −1.0 ± 0.94 | +0.5 ± 1.05 | +0.2 ± 1.43 | −1.2 ± 1.42 | +0.3 ± 1.45 |
| Seed 23, first mapping, assignment | 18 | −0.2 ± 1.24 | +0.1 ± 1.11 | −0.2 ± 1.11 | −0.2 ± 1.16 | 0.0 ± 0.95 | −0.2 ± 0.99 | −1.0 ± 1.28 | −0.7 ± 1.25 | −1.0 ± 1.28 |
| Seed 23, first mapping, assignment | 24 | +0.4 ± 1.13 | −0.6 ± 1.13 | +0.1 ± 1.36 | +0.1 ± 1.15 | −0.9 ± 1.17 | −0.2 ± 1.38 | −0.4 ± 1.06 | −1.5 ± 0.98 | −0.7 ± 1.16 |
| Seed 23, first mapping, assignment | 36 | −0.6 ± 0.79 | −0.5 ± 0.88 | +0.2 ± 1.12 | −0.2 ± 0.91 | −0.1 ± 0.93 | +0.5 ± 1.17 | −0.2 ± 0.82 | −0.1 ± 0.89 | +0.6 ± 1.03 |
| Seed 24, eighth, mirrored | 6 | +1.1 ± 0.86 | −0.3 ± 0.87 | −1.8 ± 0.86 | +0.4 ± 0.80 | −1.0 ± 0.77 | −2.5 ± 0.85 | +1.3 ± 0.89 | −0.1 ± 0.88 | −1.6 ± 0.77 |
| Seed 24, eighth, mirrored | 12 | +3.6 ± 1.20 | −1.6 ± 1.45 | −1.4 ± 1.32 | +1.7 ± 1.08 | −3.4 ± 1.66 | −3.2 ± 1.27 | +4.5 ± 1.32 | −0.6 ± 1.24 | −0.4 ± 1.39 |
| Seed 24, eighth, mirrored | 18 | +4.1 ± 2.06 | −2.6 ± 1.91 | −1.8 ± 1.57 | +3.4 ± 1.66 | −3.3 ± 1.45 | −2.4 ± 1.52 | +4.9 ± 1.86 | −1.8 ± 1.59 | −1.0 ± 1.99 |
| Seed 24, eighth, mirrored | 24 | +3.7 ± 1.52 | −3.4 ± 2.02 | −4.4 ± 1.77 | +2.3 ± 1.71 | −4.8 ± 2.09 | −5.8 ± 2.23 | +5.9 ± 1.52 | −1.2 ± 1.74 | −2.3 ± 1.82 |
| Seed 24, eighth, mirrored | 36 | +3.7 ± 1.19 | −0.6 ± 1.29 | −1.8 ± 1.19 | +2.0 ± 2.45 | −2.3 ± 2.15 | −3.4 ± 1.99 | +5.2 ± 1.48 | +0.9 ± 1.31 | −0.3 ± 1.15 |
| Seed 24, first mapping, assignment | 6 | +0.3 ± 0.40 | −0.1 ± 0.37 | +0.4 ± 0.40 | +0.1 ± 0.43 | −0.4 ± 0.41 | +0.2 ± 0.41 | +0.3 ± 0.45 | −0.2 ± 0.42 | +0.4 ± 0.45 |
| Seed 24, first mapping, assignment | 12 | +1.0 ± 0.97 | −1.2 ± 0.78 | +0.1 ± 0.87 | +1.9 ± 1.05 | −0.3 ± 1.08 | +1.0 ± 1.07 | +0.4 ± 1.03 | −1.8 ± 0.90 | −0.5 ± 0.97 |
| Seed 24, first mapping, assignment | 18 | +0.3 ± 1.35 | −0.8 ± 1.03 | −0.9 ± 1.09 | +0.7 ± 1.42 | −0.4 ± 1.50 | −0.4 ± 1.34 | +1.1 ± 1.21 | 0.0 ± 1.04 | 0.0 ± 1.01 |
| Seed 24, first mapping, assignment | 24 | −0.1 ± 1.87 | −1.5 ± 1.19 | −0.5 ± 1.73 | +1.6 ± 1.20 | +0.3 ± 1.22 | +1.3 ± 1.20 | +0.9 ± 1.38 | −0.4 ± 1.28 | +0.6 ± 1.20 |
| Seed 24, first mapping, assignment | 36 | +1.4 ± 0.84 | −1.0 ± 1.52 | −0.6 ± 1.01 | +1.4 ± 1.16 | −0.9 ± 1.12 | −0.6 ± 1.15 | +2.4 ± 1.15 | 0.0 ± 0.78 | +0.4 ± 0.96 |
| Seed 23, eighth, assignment | 6 | +0.3 ± 1.10 | +0.2 ± 1.06 | −0.1 ± 1.08 | −0.4 ± 1.04 | −0.4 ± 0.97 | −0.8 ± 1.05 | +0.6 ± 1.00 | +0.6 ± 0.94 | +0.2 ± 0.99 |
| Seed 23, eighth, assignment | 12 | +2.9 ± 1.34 | −0.9 ± 1.22 | −1.2 ± 1.14 | +0.2 ± 1.70 | −3.6 ± 1.08 | −3.9 ± 0.98 | +2.5 ± 1.17 | −1.3 ± 1.41 | −1.5 ± 1.40 |
| Seed 23, eighth, assignment | 18 | +2.6 ± 1.33 | −2.2 ± 1.06 | −2.2 ± 1.04 | +0.1 ± 1.54 | −4.7 ± 1.04 | −4.8 ± 0.90 | +3.9 ± 1.06 | −0.9 ± 1.09 | −0.9 ± 1.13 |
| Seed 23, eighth, assignment | 24 | +5.4 ± 0.83 | −2.9 ± 0.80 | −1.9 ± 1.26 | +3.8 ± 1.07 | −4.6 ± 0.98 | −3.5 ± 1.09 | +7.6 ± 0.91 | −0.8 ± 0.76 | +0.3 ± 0.93 |
| Seed 23, eighth, assignment | 36 | +3.3 ± 1.29 | −2.8 ± 1.20 | −3.6 ± 1.61 | +2.3 ± 1.12 | −3.9 ± 0.88 | −4.6 ± 1.12 | +5.2 ± 1.43 | −0.9 ± 1.27 | −1.6 ± 1.72 |
| Seed 24, first mapping, mirrored | 6 | −0.3 ± 0.82 | +0.2 ± 0.81 | −0.6 ± 0.81 | −0.6 ± 0.89 | −0.1 ± 0.86 | −0.9 ± 0.89 | −0.2 ± 0.80 | +0.3 ± 0.78 | −0.4 ± 0.79 |
| Seed 24, first mapping, mirrored | 12 | −0.3 ± 1.26 | +0.1 ± 1.29 | −1.6 ± 1.01 | +1.4 ± 1.24 | +1.7 ± 1.31 | +0.1 ± 1.48 | −1.2 ± 1.06 | −0.9 ± 1.03 | −2.5 ± 0.93 |
| Seed 24, first mapping, mirrored | 18 | −0.9 ± 0.98 | −0.6 ± 1.11 | −2.4 ± 0.85 | +1.8 ± 1.06 | +2.1 ± 0.86 | +0.3 ± 1.01 | −1.3 ± 0.90 | −1.0 ± 0.98 | −2.8 ± 0.83 |
| Seed 24, first mapping, mirrored | 24 | −0.3 ± 0.59 | −1.5 ± 0.68 | −1.9 ± 0.69 | +2.2 ± 0.82 | +1.0 ± 0.86 | +0.5 ± 0.93 | 0.0 ± 0.90 | −1.3 ± 1.00 | −1.7 ± 0.91 |
| Seed 24, first mapping, mirrored | 36 | +0.2 ± 0.67 | −1.5 ± 0.99 | −1.2 ± 0.87 | +2.0 ± 0.80 | +0.3 ± 1.11 | +0.5 ± 1.00 | +1.1 ± 0.78 | −0.6 ± 0.78 | −0.3 ± 0.61 |
| Seed 23, first mapping, mirrored | 6 | −0.5 ± 0.65 | −0.2 ± 0.56 | −0.1 ± 0.67 | −0.3 ± 0.64 | +0.1 ± 0.56 | +0.1 ± 0.63 | −0.1 ± 0.70 | +0.2 ± 0.63 | +0.2 ± 0.72 |
| Seed 23, first mapping, mirrored | 12 | +0.3 ± 1.24 | −0.1 ± 0.76 | −0.8 ± 1.10 | −0.3 ± 0.94 | −0.7 ± 0.94 | −1.4 ± 0.99 | +1.4 ± 1.12 | +1.0 ± 0.83 | +0.2 ± 1.11 |
| Seed 23, first mapping, mirrored | 18 | +1.4 ± 0.92 | −0.1 ± 0.92 | −0.5 ± 1.10 | −1.0 ± 0.89 | −2.5 ± 0.87 | −2.9 ± 1.03 | +1.3 ± 0.79 | −0.1 ± 0.86 | −0.5 ± 1.00 |
| Seed 23, first mapping, mirrored | 24 | +1.1 ± 0.95 | −0.6 ± 1.25 | −1.3 ± 0.95 | −0.4 ± 1.47 | −2.1 ± 0.87 | −2.9 ± 1.00 | +1.9 ± 1.57 | +0.2 ± 0.97 | −0.5 ± 1.13 |
| Seed 23, first mapping, mirrored | 36 | +2.0 ± 0.78 | −0.3 ± 0.94 | −1.2 ± 0.92 | +0.9 ± 1.34 | −1.3 ± 0.92 | −2.3 ± 1.09 | +2.2 ± 1.26 | −0.1 ± 0.73 | −1.0 ± 0.82 |
| Seed 24, eighth, assignment | 6 | −1.3 ± 0.98 | −0.6 ± 0.97 | −1.5 ± 1.09 | −0.7 ± 1.04 | +0.1 ± 1.02 | −0.9 ± 1.14 | −0.3 ± 1.07 | +0.4 ± 1.06 | −0.5 ± 1.17 |
| Seed 24, eighth, assignment | 12 | +0.2 ± 1.04 | −0.1 ± 0.98 | −2.4 ± 1.00 | +1.4 ± 0.99 | +1.1 ± 0.93 | −1.3 ± 0.95 | +1.3 ± 1.15 | +1.0 ± 1.25 | −1.4 ± 1.16 |
| Seed 24, eighth, assignment | 18 | +1.9 ± 1.23 | −1.2 ± 1.17 | −5.2 ± 1.29 | +2.8 ± 1.35 | −0.3 ± 1.67 | −4.3 ± 1.37 | +3.2 ± 1.34 | +0.1 ± 1.08 | −3.9 ± 1.49 |
| Seed 24, eighth, assignment | 24 | −0.6 ± 1.65 | −3.4 ± 1.49 | −7.7 ± 1.08 | +2.7 ± 1.66 | −0.1 ± 2.11 | −4.4 ± 1.72 | +3.0 ± 2.01 | +0.2 ± 1.27 | −4.0 ± 1.59 |
| Seed 24, eighth, assignment | 36 | −0.8 ± 1.74 | −3.1 ± 1.30 | −7.8 ± 1.21 | −0.2 ± 1.44 | −2.5 ± 1.50 | −7.2 ± 1.16 | +3.8 ± 2.57 | +1.5 ± 1.28 | −3.1 ± 2.17 |
| Seed 22, first mapping, mirrored | 6 | +0.6 ± 0.57 | +0.1 ± 0.45 | +0.2 ± 0.50 | +0.1 ± 0.53 | −0.4 ± 0.49 | −0.3 ± 0.53 | +0.4 ± 0.45 | −0.1 ± 0.40 | 0.0 ± 0.51 |
| Seed 22, first mapping, mirrored | 12 | +0.5 ± 0.93 | 0.0 ± 0.98 | +0.3 ± 1.02 | +0.6 ± 1.04 | +0.1 ± 1.14 | +0.4 ± 1.09 | +0.6 ± 0.98 | 0.0 ± 1.04 | +0.3 ± 1.16 |
| Seed 22, first mapping, mirrored | 18 | +0.1 ± 1.13 | −0.4 ± 1.11 | +0.5 ± 1.04 | −0.1 ± 1.12 | −0.7 ± 1.11 | +0.2 ± 1.05 | +0.5 ± 1.00 | −0.1 ± 1.14 | +0.8 ± 1.06 |
| Seed 22, first mapping, mirrored | 24 | +0.5 ± 1.27 | −0.6 ± 1.46 | +0.4 ± 1.39 | +0.9 ± 1.39 | −0.3 ± 1.34 | +0.8 ± 1.39 | +1.4 ± 1.30 | +0.2 ± 1.24 | +1.3 ± 1.24 |
| Seed 22, first mapping, mirrored | 36 | +0.1 ± 0.84 | −0.5 ± 0.81 | +0.5 ± 1.01 | +0.8 ± 0.89 | +0.1 ± 0.54 | +1.1 ± 0.69 | +1.2 ± 0.98 | +0.6 ± 0.74 | +1.5 ± 0.87 |
| Seed 22, eighth, assignment | 6 | +0.6 ± 1.06 | +0.2 ± 0.92 | +0.2 ± 0.95 | −0.3 ± 1.05 | −0.6 ± 0.95 | −0.6 ± 0.98 | +0.6 ± 0.89 | +0.2 ± 1.03 | +0.3 ± 1.01 |
| Seed 22, eighth, assignment | 12 | +2.0 ± 0.92 | −0.3 ± 0.93 | −0.2 ± 0.85 | +1.2 ± 1.02 | −1.1 ± 0.96 | −1.0 ± 0.90 | +2.3 ± 0.92 | 0.0 ± 0.89 | +0.1 ± 0.79 |
| Seed 22, eighth, assignment | 18 | +0.8 ± 1.20 | −0.4 ± 0.89 | −0.4 ± 0.89 | +1.4 ± 1.11 | +0.2 ± 1.35 | +0.2 ± 1.38 | +2.3 ± 1.51 | +1.1 ± 0.98 | +1.1 ± 0.91 |
| Seed 22, eighth, assignment | 24 | +0.1 ± 0.83 | −2.3 ± 1.07 | −1.9 ± 0.92 | +2.0 ± 1.18 | −0.4 ± 1.18 | 0.0 ± 1.10 | +3.3 ± 1.07 | +0.9 ± 0.78 | +1.3 ± 0.89 |
| Seed 22, eighth, assignment | 36 | −0.5 ± 1.93 | −2.8 ± 1.46 | −3.0 ± 1.64 | +1.8 ± 1.65 | −0.5 ± 1.84 | −0.7 ± 1.55 | +1.9 ± 1.62 | −0.4 ± 1.05 | −0.5 ± 1.34 |

- **Less H-47's plain night**, the night held onto the own group and the inhibitory units lies between −5.5 and +5.4, beyond twice its SE at
  15 of the 60 checkpoints, 8 above and 7 below; the night held onto the own group and the units outside the readouts between −1.6 and +3.8,
  beyond at 8, 7 above and 1 below; and the night held onto the inhibitory units and the units outside the readouts between −3.5 and +7.6,
  beyond at 14, 12 above and 2 below. Exploration 027's night held within lies beyond at 17, 13 above and 4 below, the excitatory night at
  25, 19 above and 6 below, and exploration 028's nights with the own group, the inhibitory units and the units outside the readouts held
  alone at 3, 13 and 6.
  - The night held onto the own group and the inhibitory units lies above on seed 24's eighth image from the mirrored assignment at blocks
    12, 24 and 36, +3.6 ± 1.20 to +3.7 ± 1.52; on seed 23's eighth image from the assignment at blocks 12, 24 and 36, +2.9 ± 1.34 to
    +5.4 ± 0.83; on seed 23's first mapping from the mirrored assignment at block 36, +2.0 ± 0.78; and on seed 22's eighth image from the
    assignment at block 12, +2.0 ± 0.92. It lies below on seed 22's eighth image from the mirrored assignment from block 18 on, −1.7 ± 0.77
    to −5.5 ± 1.37, and on seed 23's eighth image from the mirrored assignment from block 12 on, −2.4 ± 1.15 to −3.2 ± 1.19.
  - The night held onto the own group and the units outside the readouts lies above on seed 22's first mapping from the assignment at block
    36, +2.0 ± 0.89; on seed 24's eighth image from the mirrored assignment at block 18, +3.4 ± 1.66; on seed 23's eighth image from the
    assignment at blocks 24 and 36, +3.8 ± 1.07 and +2.3 ± 1.12; on seed 24's first mapping from the mirrored assignment at blocks 24 and
    36, +2.2 ± 0.82 and +2.0 ± 0.80; and on seed 24's eighth image from the assignment at block 18, +2.8 ± 1.35. It lies below on seed 22's
    eighth image from the mirrored assignment at block 36, −1.1 ± 0.41.
  - The night held onto the inhibitory units and the units outside the readouts lies above on seed 24's eighth image from the mirrored
    assignment from block 12 on, +4.5 ± 1.32 to +5.9 ± 1.52; on seed 24's first mapping from the assignment at block 36, +2.4 ± 1.15; on
    seed 23's eighth image from the assignment from block 12 on, +2.5 ± 1.17 to +7.6 ± 0.91; on seed 24's eighth image from the assignment
    at block 18, +3.2 ± 1.34; and on seed 22's eighth image from the assignment at blocks 12 and 24, +2.3 ± 0.92 and +3.3 ± 1.07. It lies
    below on seed 23's eighth image from the mirrored assignment at blocks 18 and 24, −3.5 ± 1.12 and −3.4 ± 1.06.
- **Where the excitatory night lies beyond 2 SE of the plain night**, 25 checkpoints, the night held onto the inhibitory units and the units
  outside the readouts lies beyond the plain night in the same direction at 14, all 14 of its own; the night held onto the own group and the
  inhibitory units at 13, of its 15; and the night held onto the own group and the units outside the readouts at 6, of its 8. Exploration
  028's nights with the inhibitory units, the units outside the readouts and the own group held alone did so at 12, 4 and 1. Where
  exploration 027's night held within lies beyond 2 SE of the plain night, 17 checkpoints, the three pairs lie beyond it in the same
  direction at 13, 11 and 6, in the same order.
- **Less exploration 027's night held within**, the night held onto the inhibitory units and the units outside the readouts lies within
  twice its SE at all 60 checkpoints, between −1.8 and +1.5. The night held onto the own group and the inhibitory units lies between −3.4
  and +1.0, beyond at 7, all below: on seed 23's eighth image from the assignment from block 18 on, −2.2 ± 1.06 to −2.9 ± 0.80; on seed 24's
  first mapping from the mirrored assignment at block 24, −1.5 ± 0.68; on seed 24's eighth image from the assignment at blocks 24 and 36,
  −3.4 ± 1.49 and −3.1 ± 1.30; and on seed 22's eighth image from the assignment at block 24, −2.3 ± 1.07. The night held onto the own group
  and the units outside the readouts lies between −4.8 and +3.4, beyond at 13, 4 above and 9 below. Above: on seed 22's eighth image from
  the mirrored assignment at block 36, +1.8 ± 0.82; on seed 22's first mapping from the assignment at blocks 24 and 36, +2.8 ± 0.97 and +3.4
  ± 1.43; and on seed 24's first mapping from the mirrored assignment at block 18, +2.1 ± 0.86. Below: on seed 24's eighth image from the
  mirrored assignment at blocks 12, 18 and 24, −3.3 ± 1.45 to −4.8 ± 2.09; on seed 23's eighth image from the assignment from block 12 on,
  −3.6 ± 1.08 to −4.7 ± 1.04; and on seed 23's first mapping from the mirrored assignment at blocks 18 and 24, −2.5 ± 0.87 and −2.1 ± 0.87.
- **Less the excitatory night**, the night held onto the inhibitory units and the units outside the readouts lies between −4.0 and +1.9,
  beyond twice its SE at 6 of the 60, 1 above and 5 below: above on seed 22's first mapping from the assignment at block 36, +1.8 ± 0.82;
  below on seed 24's eighth image from the mirrored assignment at block 6, −1.6 ± 0.77, on seed 24's first mapping from the mirrored
  assignment at blocks 12 and 18, −2.5 ± 0.93 and −2.8 ± 0.83, and on seed 24's eighth image from the assignment at blocks 18 and 24,
  −3.9 ± 1.49 and −4.0 ± 1.59. The night held onto the own group and the inhibitory units lies between −7.8 and +1.6, beyond at 11, all
  below: on seed 24's eighth image from the mirrored assignment at blocks 6 and 24, −1.8 ± 0.86 and −4.4 ± 1.77; on seed 23's eighth image
  from the assignment at blocks 18 and 36, −2.2 ± 1.04 and −3.6 ± 1.61; on seed 24's first mapping from the mirrored assignment at blocks
  18 and 24, −2.4 ± 0.85 and −1.9 ± 0.69; on seed 24's eighth image from the assignment from block 12 on, −2.4 ± 1.00 to −7.8 ± 1.21; and
  on seed 22's eighth image from the assignment at block 24, −1.9 ± 0.92. The night held onto the own group and the units outside the
  readouts lies between −7.2 and +4.7, beyond at 17, 4 above and 13 below. Above: on seed 22's first mapping from the assignment from block
  18 on, +2.4 ± 1.15 to +4.7 ± 0.92, and on seed 23's eighth image from the mirrored assignment at block 24, +2.4 ± 1.08. Below: on seed
  24's eighth image from the mirrored assignment at blocks 6, 12 and 24, −2.5 ± 0.85 to −5.8 ± 2.23; on seed 23's eighth image from the
  assignment from block 12 on, −3.5 ± 1.09 to −4.8 ± 0.90; on seed 23's first mapping from the mirrored assignment from block 18 on,
  −2.3 ± 1.09 to −2.9 ± 1.03; and on seed 24's eighth image from the assignment from block 18 on, −4.3 ± 1.37 to −7.2 ± 1.16. Exploration
  027's night held within lies beyond 2 SE of the excitatory night at 4 of the 60, all on seed 24's eighth image from the assignment.

Each held night less exploration 028's night for each of its two parts held alone, from another job, with exploration 018's unpaired SE:

| Cell | Block | Own group and inhibitory units less own group alone (028) | less inhibitory units alone (028) | Own group and outside the readouts less own group alone (028) | less outside the readouts alone (028) | Inhibitory units and outside the readouts less inhibitory units alone (028) | less outside the readouts alone (028) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | −0.4 ± 0.78 | −0.2 ± 0.74 | −0.2 ± 0.70 | −0.4 ± 0.74 | +0.1 ± 0.68 | −0.3 ± 0.69 |
| Seed 22, eighth, mirrored | 12 | −0.7 ± 1.20 | −0.5 ± 1.15 | +0.8 ± 1.10 | −0.9 ± 1.03 | +1.3 ± 1.21 | −0.7 ± 1.08 |
| Seed 22, eighth, mirrored | 18 | −0.2 ± 1.05 | −0.5 ± 0.78 | +0.1 ± 0.69 | −1.4 ± 0.82 | +0.4 ± 0.93 | −0.8 ± 0.81 |
| Seed 22, eighth, mirrored | 24 | −2.8 ± 1.31 | −0.6 ± 0.99 | +1.3 ± 0.94 | −3.3 ± 0.91 | +3.2 ± 1.22 | −3.5 ± 0.99 |
| Seed 22, eighth, mirrored | 36 | −1.5 ± 0.84 | −0.7 ± 1.58 | +1.7 ± 0.37 | −2.2 ± 0.34 | +1.8 ± 1.16 | −2.9 ± 0.91 |
| Seed 22, first mapping, assignment | 6 | −0.2 ± 0.62 | +0.2 ± 0.46 | +0.1 ± 0.48 | +0.3 ± 0.27 | −0.1 ± 0.50 | −0.3 ± 0.59 |
| Seed 22, first mapping, assignment | 12 | −0.8 ± 1.21 | +0.1 ± 1.08 | +0.4 ± 1.45 | −0.2 ± 1.40 | +0.3 ± 1.16 | −1.1 ± 1.26 |
| Seed 22, first mapping, assignment | 18 | −1.1 ± 1.65 | +0.2 ± 1.09 | +0.5 ± 1.67 | +0.5 ± 1.11 | +1.3 ± 1.15 | −0.1 ± 1.10 |
| Seed 22, first mapping, assignment | 24 | −3.3 ± 0.89 | +0.7 ± 1.29 | +0.2 ± 0.86 | +1.3 ± 1.32 | +1.3 ± 1.50 | −1.7 ± 1.27 |
| Seed 22, first mapping, assignment | 36 | −3.9 ± 1.54 | +0.3 ± 0.94 | −0.3 ± 0.73 | +1.7 ± 0.84 | +0.9 ± 0.72 | −1.3 ± 0.72 |
| Seed 23, eighth, mirrored | 6 | −0.6 ± 0.94 | −0.1 ± 1.06 | −0.8 ± 1.15 | 0.0 ± 1.08 | −0.5 ± 1.01 | −0.2 ± 1.05 |
| Seed 23, eighth, mirrored | 12 | −1.5 ± 1.11 | −0.1 ± 0.88 | +0.5 ± 1.29 | +0.4 ± 1.33 | −0.4 ± 1.21 | −1.9 ± 1.29 |
| Seed 23, eighth, mirrored | 18 | −2.1 ± 1.51 | −0.2 ± 1.28 | 0.0 ± 2.52 | −0.5 ± 2.05 | −0.6 ± 1.22 | −2.9 ± 1.26 |
| Seed 23, eighth, mirrored | 24 | −1.5 ± 1.18 | +0.6 ± 1.03 | −0.4 ± 1.62 | −0.9 ± 1.29 | 0.0 ± 1.19 | −2.6 ± 1.10 |
| Seed 23, eighth, mirrored | 36 | −2.0 ± 1.12 | −0.3 ± 1.04 | −0.4 ± 1.08 | −0.4 ± 1.11 | +1.3 ± 1.24 | −0.4 ± 1.36 |
| Seed 23, first mapping, assignment | 6 | 0.0 ± 0.76 | +0.1 ± 0.80 | −0.2 ± 0.77 | +0.1 ± 0.77 | +0.2 ± 0.76 | +0.4 ± 0.74 |
| Seed 23, first mapping, assignment | 12 | +0.9 ± 1.02 | +1.5 ± 1.14 | −0.2 ± 0.97 | −0.1 ± 1.16 | +0.2 ± 1.36 | −0.3 ± 1.24 |
| Seed 23, first mapping, assignment | 18 | +0.5 ± 1.04 | +0.5 ± 1.20 | +0.4 ± 1.08 | +0.6 ± 1.10 | −0.3 ± 1.25 | −0.1 ± 1.16 |
| Seed 23, first mapping, assignment | 24 | +1.0 ± 1.15 | +0.5 ± 1.26 | +0.7 ± 1.20 | +0.8 ± 1.16 | −0.3 ± 1.01 | +0.3 ± 1.11 |
| Seed 23, first mapping, assignment | 36 | +0.1 ± 0.89 | +0.1 ± 1.23 | +0.4 ± 1.11 | +0.2 ± 0.97 | +0.5 ± 1.09 | +0.3 ± 0.91 |
| Seed 24, eighth, mirrored | 6 | +1.5 ± 0.85 | 0.0 ± 0.89 | +0.8 ± 0.80 | +0.2 ± 0.89 | +0.2 ± 0.87 | +1.1 ± 0.78 |
| Seed 24, eighth, mirrored | 12 | +3.2 ± 1.26 | +0.9 ± 1.23 | +1.4 ± 1.10 | +0.7 ± 1.41 | +1.9 ± 1.06 | +3.5 ± 1.14 |
| Seed 24, eighth, mirrored | 18 | +2.5 ± 1.78 | +1.9 ± 2.23 | +1.9 ± 1.62 | +2.6 ± 1.81 | +2.7 ± 1.71 | +4.1 ± 2.05 |
| Seed 24, eighth, mirrored | 24 | +4.4 ± 1.66 | +0.8 ± 2.12 | +3.0 ± 1.69 | +1.2 ± 1.69 | +2.9 ± 1.72 | +4.7 ± 1.88 |
| Seed 24, eighth, mirrored | 36 | +3.4 ± 1.12 | +2.0 ± 1.09 | +1.8 ± 1.85 | +1.3 ± 1.31 | +3.5 ± 1.22 | +4.5 ± 1.15 |
| Seed 24, first mapping, assignment | 6 | +0.3 ± 0.36 | 0.0 ± 0.38 | 0.0 ± 0.38 | −0.2 ± 0.41 | −0.1 ± 0.43 | 0.0 ± 0.42 |
| Seed 24, first mapping, assignment | 12 | 0.0 ± 1.00 | +1.0 ± 0.84 | +0.9 ± 1.01 | +1.9 ± 1.27 | +0.4 ± 0.94 | +0.4 ± 1.01 |
| Seed 24, first mapping, assignment | 18 | −0.8 ± 1.33 | −0.3 ± 1.00 | −0.4 ± 1.20 | +0.6 ± 1.61 | +0.5 ± 1.09 | +1.0 ± 1.27 |
| Seed 24, first mapping, assignment | 24 | −0.9 ± 1.78 | −0.9 ± 1.59 | +0.9 ± 1.03 | +0.8 ± 1.34 | +0.2 ± 1.13 | +0.1 ± 1.29 |
| Seed 24, first mapping, assignment | 36 | +0.8 ± 0.91 | −1.4 ± 1.13 | +0.8 ± 0.99 | +0.4 ± 1.20 | −0.4 ± 1.07 | +1.3 ± 1.16 |
| Seed 23, eighth, assignment | 6 | +0.5 ± 1.12 | +0.1 ± 1.14 | −0.2 ± 1.03 | −0.2 ± 0.96 | +0.4 ± 1.04 | +0.8 ± 0.93 |
| Seed 23, eighth, assignment | 12 | +2.1 ± 0.98 | +0.5 ± 1.13 | −0.6 ± 1.31 | +0.1 ± 1.08 | +0.1 ± 1.14 | +2.5 ± 1.25 |
| Seed 23, eighth, assignment | 18 | +3.2 ± 1.11 | 0.0 ± 0.96 | +0.6 ± 1.26 | −3.3 ± 0.83 | +1.3 ± 0.78 | +0.6 ± 1.15 |
| Seed 23, eighth, assignment | 24 | +4.3 ± 1.13 | +1.6 ± 1.01 | +2.6 ± 1.20 | +0.2 ± 1.01 | +3.7 ± 0.77 | +4.0 ± 0.83 |
| Seed 23, eighth, assignment | 36 | +2.9 ± 1.91 | +0.6 ± 1.65 | +1.9 ± 1.48 | −0.2 ± 0.87 | +2.6 ± 1.74 | +2.8 ± 1.45 |
| Seed 24, first mapping, mirrored | 6 | +0.3 ± 0.82 | 0.0 ± 0.73 | 0.0 ± 0.88 | −0.5 ± 0.86 | +0.1 ± 0.69 | 0.0 ± 0.75 |
| Seed 24, first mapping, mirrored | 12 | −0.5 ± 1.28 | +0.8 ± 1.09 | +1.2 ± 1.15 | +1.7 ± 1.22 | −0.2 ± 1.05 | −0.9 ± 1.16 |
| Seed 24, first mapping, mirrored | 18 | −1.3 ± 1.04 | 0.0 ± 1.01 | +1.4 ± 0.88 | +1.7 ± 1.02 | −0.4 ± 0.86 | −1.4 ± 0.78 |
| Seed 24, first mapping, mirrored | 24 | −1.3 ± 0.80 | −0.4 ± 0.80 | +1.2 ± 0.78 | +1.6 ± 0.87 | −0.2 ± 0.85 | −0.6 ± 1.13 |
| Seed 24, first mapping, mirrored | 36 | −0.9 ± 0.53 | −0.5 ± 0.55 | +0.9 ± 0.68 | +0.9 ± 0.88 | +0.4 ± 0.81 | +0.1 ± 0.85 |
| Seed 23, first mapping, mirrored | 6 | −0.1 ± 0.58 | −0.5 ± 0.64 | +0.1 ± 0.60 | −0.2 ± 0.65 | −0.2 ± 0.70 | −0.1 ± 0.72 |
| Seed 23, first mapping, mirrored | 12 | +1.2 ± 1.41 | −1.0 ± 0.97 | +0.6 ± 0.94 | −1.1 ± 1.00 | +0.1 ± 0.91 | +0.6 ± 0.99 |
| Seed 23, first mapping, mirrored | 18 | +2.2 ± 0.97 | −0.7 ± 0.86 | −0.2 ± 0.94 | −0.8 ± 0.92 | −0.8 ± 0.90 | +1.5 ± 0.86 |
| Seed 23, first mapping, mirrored | 24 | +2.3 ± 1.00 | −0.5 ± 0.92 | +0.7 ± 0.82 | −0.7 ± 1.42 | +0.3 ± 1.60 | +1.6 ± 1.59 |
| Seed 23, first mapping, mirrored | 36 | +1.7 ± 0.72 | +0.1 ± 1.00 | +0.6 ± 0.80 | +0.7 ± 1.51 | +0.3 ± 1.13 | +2.0 ± 1.07 |
| Seed 24, eighth, assignment | 6 | +0.2 ± 0.91 | −1.0 ± 0.96 | +0.8 ± 0.96 | −0.4 ± 1.11 | −0.1 ± 1.06 | 0.0 ± 1.14 |
| Seed 24, eighth, assignment | 12 | +1.0 ± 1.25 | +0.4 ± 1.22 | +2.2 ± 1.20 | +0.5 ± 0.96 | +1.4 ± 1.01 | +0.4 ± 1.14 |
| Seed 24, eighth, assignment | 18 | +0.5 ± 1.30 | +0.4 ± 1.13 | +1.4 ± 1.13 | −0.4 ± 1.18 | +1.7 ± 1.04 | 0.0 ± 1.59 |
| Seed 24, eighth, assignment | 24 | −0.7 ± 1.50 | −0.7 ± 1.43 | +2.6 ± 2.06 | +0.6 ± 2.08 | +2.9 ± 1.64 | +0.9 ± 1.44 |
| Seed 24, eighth, assignment | 36 | +0.3 ± 1.33 | −0.4 ± 0.87 | +0.9 ± 1.29 | −1.9 ± 2.05 | +4.3 ± 1.77 | +2.1 ± 2.00 |
| Seed 22, first mapping, mirrored | 6 | +0.3 ± 0.52 | +0.1 ± 0.49 | −0.1 ± 0.50 | +0.5 ± 0.48 | 0.0 ± 0.41 | +0.8 ± 0.43 |
| Seed 22, first mapping, mirrored | 12 | +0.4 ± 0.88 | −0.1 ± 0.80 | +0.5 ± 1.03 | −0.8 ± 1.34 | −0.1 ± 0.87 | −0.8 ± 1.34 |
| Seed 22, first mapping, mirrored | 18 | +0.3 ± 1.12 | −0.5 ± 1.05 | +0.1 ± 1.12 | −1.5 ± 1.43 | −0.2 ± 0.93 | −1.0 ± 1.26 |
| Seed 22, first mapping, mirrored | 24 | −0.1 ± 1.44 | +0.3 ± 1.31 | +0.2 ± 1.37 | −0.8 ± 1.42 | +1.2 ± 1.24 | −0.3 ± 1.29 |
| Seed 22, first mapping, mirrored | 36 | −0.3 ± 0.67 | +0.3 ± 1.05 | +0.3 ± 0.42 | −0.3 ± 0.73 | +1.4 ± 0.96 | +0.2 ± 0.85 |
| Seed 22, eighth, assignment | 6 | +0.6 ± 0.93 | +0.4 ± 0.96 | −0.2 ± 0.95 | −0.7 ± 1.03 | +0.4 ± 0.84 | +0.2 ± 0.87 |
| Seed 22, eighth, assignment | 12 | +1.5 ± 0.93 | +1.0 ± 0.98 | +0.7 ± 0.98 | +0.5 ± 0.98 | +1.3 ± 0.79 | +1.5 ± 0.83 |
| Seed 22, eighth, assignment | 18 | +1.1 ± 1.06 | −0.2 ± 0.80 | +1.7 ± 0.94 | −0.2 ± 1.25 | +1.3 ± 0.83 | +0.7 ± 1.02 |
| Seed 22, eighth, assignment | 24 | +0.5 ± 1.07 | −0.6 ± 0.89 | +2.4 ± 0.77 | +0.6 ± 1.17 | +2.6 ± 0.86 | +1.9 ± 1.05 |
| Seed 22, eighth, assignment | 36 | +0.1 ± 2.18 | −1.6 ± 1.72 | +2.4 ± 1.51 | −0.2 ± 1.47 | +0.8 ± 1.42 | −0.1 ± 1.91 |

- **The night held onto the own group and the inhibitory units** lies within twice its SE of exploration 028's night with the inhibitory
  units held alone at all 60 checkpoints, between −1.6 and +2.0. Less exploration 028's night with the own group held alone it lies between
  −3.9 and +4.4, beyond at 12, 9 above and 3 below: above on seed 24's eighth image from the mirrored assignment at blocks 12, 24 and 36,
  +3.2 ± 1.26 to +4.4 ± 1.66, on seed 23's eighth image from the assignment at blocks 12, 18 and 24, +2.1 ± 0.98 to +4.3 ± 1.13, and on seed
  23's first mapping from the mirrored assignment from block 18 on, +1.7 ± 0.72 to +2.3 ± 1.00; below on seed 22's eighth image from the
  mirrored assignment at block 24, −2.8 ± 1.31, and on seed 22's first mapping from the assignment at blocks 24 and 36, −3.3 ± 0.89 and
  −3.9 ± 1.54.
- **The night held onto the own group and the units outside the readouts** lies, less exploration 028's night with the own group held
  alone, between −0.8 and +3.0, beyond at 3, all above: on seed 22's eighth image from the mirrored assignment at block 36, +1.7 ± 0.37, on
  seed 23's eighth image from the assignment at block 24, +2.6 ± 1.20, and on seed 22's eighth image from the assignment at block 24,
  +2.4 ± 0.77. Less its night with the units outside the readouts held alone it lies between −3.3 and +2.6, beyond at 4, 1 above and 3
  below: above on seed 22's first mapping from the assignment at block 36, +1.7 ± 0.84; below on seed 22's eighth image from the mirrored
  assignment at blocks 24 and 36, −3.3 ± 0.91 and −2.2 ± 0.34, and on seed 23's eighth image from the assignment at block 18, −3.3 ± 0.83.
- **The night held onto the inhibitory units and the units outside the readouts** lies, less exploration 028's night with the inhibitory
  units held alone, between −0.8 and +4.3, beyond at 5, all above: on seed 22's eighth image from the mirrored assignment at block 24,
  +3.2 ± 1.22, on seed 24's eighth image from the mirrored assignment at block 36, +3.5 ± 1.22, on seed 23's eighth image from the
  assignment at block 24, +3.7 ± 0.77, on seed 24's eighth image from the assignment at block 36, +4.3 ± 1.77, and on seed 22's eighth
  image from the assignment at block 24, +2.6 ± 0.86. Less its night with the units outside the readouts held alone it lies between −3.5 and
  +4.7, beyond at 8, 4 above and 4 below: above on seed 24's eighth image from the mirrored assignment at blocks 12, 24 and 36, +3.5 ± 1.14
  to +4.7 ± 1.88, and on seed 23's eighth image from the assignment at block 24, +4.0 ± 0.83; below on seed 22's eighth image from the
  mirrored assignment at blocks 24 and 36, −3.5 ± 0.99 and −2.9 ± 0.91, and on seed 23's eighth image from the mirrored assignment at blocks
  18 and 24, −2.9 ± 1.26 and −2.6 ± 1.10.

Each stimulus's correct trials, of A's, at each checkpoint under each form (stimulus A then stimulus B):

| Cell | Block | A's: without replay | own group and inhibitory units | own group and outside the readouts | inhibitory units and outside the readouts | plain on A's | held within (027) | excitatory | own group alone (028) | inhibitory units alone (028) | outside the readouts alone (028) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 24.8 | 29.5 | 28.9 | 29.5 | 29.0 | 29.4 | 29.4 | 29.0 | 29.5 | 28.9 |
| Seed 22, eighth, mirrored | 12 | 11.3 | 19.0 | 18.2 | 19.9 | 19.1 | 19.0 | 20.1 | 17.9 | 19.7 | 19.0 |
| Seed 22, eighth, mirrored | 18 | 9.5 | 17.8 | 16.6 | 18.6 | 18.1 | 17.4 | 19.0 | 16.8 | 18.6 | 17.9 |
| Seed 22, eighth, mirrored | 24 | 8.6 | 13.8 | 13.3 | 16.3 | 14.7 | 14.3 | 15.8 | 12.8 | 14.7 | 15.5 |
| Seed 22, eighth, mirrored | 36 | 4.7 | 12.9 | 13.0 | 15.5 | 13.9 | 13.7 | 15.0 | 12.1 | 14.1 | 14.1 |
| Seed 22, first mapping, assignment | 6 | 25.5 | 29.6 | 29.6 | 29.4 | 29.6 | 29.7 | 29.7 | 29.5 | 29.5 | 29.6 |
| Seed 22, first mapping, assignment | 12 | 21.1 | 25.8 | 25.5 | 25.4 | 24.9 | 25.8 | 25.8 | 25.4 | 25.4 | 25.0 |
| Seed 22, first mapping, assignment | 18 | 16.4 | 25.7 | 25.3 | 25.9 | 24.8 | 25.2 | 25.0 | 25.7 | 25.3 | 25.1 |
| Seed 22, first mapping, assignment | 24 | 15.5 | 23.9 | 23.8 | 24.6 | 23.8 | 23.8 | 23.2 | 24.6 | 23.8 | 23.7 |
| Seed 22, first mapping, assignment | 36 | 13.2 | 22.0 | 22.5 | 22.5 | 21.5 | 21.5 | 21.3 | 23.0 | 22.3 | 21.5 |
| Seed 23, eighth, mirrored | 6 | 20.0 | 21.9 | 22.5 | 22.1 | 22.5 | 22.0 | 21.8 | 22.5 | 22.0 | 22.3 |
| Seed 23, eighth, mirrored | 12 | 15.3 | 18.8 | 21.3 | 20.1 | 21.5 | 19.4 | 18.0 | 20.8 | 19.5 | 21.1 |
| Seed 23, eighth, mirrored | 18 | 12.3 | 21.3 | 22.4 | 21.5 | 24.4 | 20.0 | 19.5 | 22.9 | 21.8 | 23.4 |
| Seed 23, eighth, mirrored | 24 | 8.9 | 23.5 | 23.6 | 23.1 | 25.8 | 22.3 | 21.4 | 24.3 | 23.6 | 24.5 |
| Seed 23, eighth, mirrored | 36 | 7.4 | 20.8 | 22.2 | 22.4 | 23.3 | 21.1 | 20.9 | 22.2 | 21.9 | 22.7 |
| Seed 23, first mapping, assignment | 6 | 29.2 | 30.9 | 31.0 | 30.9 | 30.9 | 31.0 | 30.8 | 30.9 | 31.0 | 30.9 |
| Seed 23, first mapping, assignment | 12 | 24.2 | 30.2 | 30.1 | 30.6 | 30.1 | 30.5 | 29.6 | 30.0 | 30.3 | 30.4 |
| Seed 23, first mapping, assignment | 18 | 19.5 | 29.8 | 29.7 | 29.8 | 30.1 | 29.9 | 29.2 | 29.6 | 30.3 | 29.7 |
| Seed 23, first mapping, assignment | 24 | 17.3 | 29.6 | 29.6 | 30.2 | 30.4 | 30.0 | 29.2 | 29.8 | 30.4 | 29.8 |
| Seed 23, first mapping, assignment | 36 | 15.7 | 28.7 | 29.1 | 29.3 | 29.2 | 29.1 | 28.0 | 28.8 | 28.8 | 29.1 |
| Seed 24, eighth, mirrored | 6 | 19.1 | 18.2 | 16.7 | 18.5 | 17.0 | 18.2 | 18.9 | 16.5 | 18.5 | 17.1 |
| Seed 24, eighth, mirrored | 12 | 15.3 | 15.6 | 13.7 | 15.9 | 12.1 | 16.4 | 16.2 | 12.3 | 14.5 | 13.1 |
| Seed 24, eighth, mirrored | 18 | 9.2 | 14.6 | 14.0 | 15.1 | 9.9 | 16.9 | 16.0 | 11.4 | 13.6 | 11.5 |
| Seed 24, eighth, mirrored | 24 | 7.4 | 15.2 | 11.6 | 15.1 | 9.0 | 16.5 | 16.6 | 9.4 | 12.4 | 10.4 |
| Seed 24, eighth, mirrored | 36 | 5.3 | 13.0 | 10.0 | 12.0 | 6.9 | 12.7 | 13.3 | 8.8 | 9.6 | 7.4 |
| Seed 24, first mapping, assignment | 6 | 28.8 | 30.8 | 30.8 | 30.8 | 30.7 | 30.9 | 30.6 | 30.7 | 30.9 | 30.8 |
| Seed 24, first mapping, assignment | 12 | 26.4 | 29.8 | 29.7 | 29.7 | 28.6 | 30.3 | 29.0 | 29.1 | 29.4 | 28.9 |
| Seed 24, first mapping, assignment | 18 | 24.9 | 29.9 | 30.0 | 30.1 | 28.9 | 30.1 | 29.1 | 29.6 | 29.9 | 29.4 |
| Seed 24, first mapping, assignment | 24 | 22.3 | 29.6 | 30.2 | 30.1 | 28.4 | 30.5 | 29.1 | 29.1 | 29.7 | 29.5 |
| Seed 24, first mapping, assignment | 36 | 21.2 | 27.4 | 26.4 | 26.6 | 24.5 | 27.6 | 26.1 | 26.3 | 26.2 | 25.3 |
| Seed 23, eighth, assignment | 6 | 24.3 | 27.2 | 27.1 | 27.0 | 26.9 | 27.2 | 27.0 | 27.1 | 26.8 | 26.7 |
| Seed 23, eighth, assignment | 12 | 18.3 | 22.5 | 21.8 | 22.8 | 21.2 | 23.1 | 23.2 | 22.2 | 21.9 | 21.4 |
| Seed 23, eighth, assignment | 18 | 15.6 | 22.7 | 24.0 | 23.6 | 20.8 | 24.0 | 24.4 | 22.6 | 21.5 | 23.0 |
| Seed 23, eighth, assignment | 24 | 16.3 | 23.9 | 25.2 | 24.3 | 19.6 | 25.3 | 25.2 | 24.0 | 21.4 | 22.2 |
| Seed 23, eighth, assignment | 36 | 13.0 | 19.3 | 20.0 | 19.0 | 15.3 | 20.3 | 21.0 | 18.8 | 17.0 | 17.5 |
| Seed 24, first mapping, mirrored | 6 | 28.1 | 29.8 | 29.6 | 30.0 | 30.1 | 29.7 | 30.2 | 29.6 | 29.9 | 30.1 |
| Seed 24, first mapping, mirrored | 12 | 24.6 | 27.1 | 27.6 | 26.8 | 27.3 | 27.0 | 27.2 | 27.3 | 26.8 | 27.4 |
| Seed 24, first mapping, mirrored | 18 | 21.7 | 26.1 | 27.5 | 26.7 | 27.0 | 26.5 | 26.8 | 26.8 | 26.5 | 27.1 |
| Seed 24, first mapping, mirrored | 24 | 22.1 | 25.8 | 26.4 | 25.9 | 25.8 | 26.0 | 26.1 | 25.8 | 25.5 | 26.2 |
| Seed 24, first mapping, mirrored | 36 | 22.0 | 25.0 | 25.8 | 25.8 | 25.2 | 25.3 | 25.5 | 25.0 | 25.1 | 26.1 |
| Seed 23, first mapping, mirrored | 6 | 30.4 | 31.2 | 31.2 | 31.0 | 31.0 | 31.1 | 31.2 | 31.2 | 31.1 | 31.2 |
| Seed 23, first mapping, mirrored | 12 | 28.5 | 29.7 | 29.5 | 29.6 | 29.6 | 29.8 | 30.3 | 29.6 | 29.8 | 29.5 |
| Seed 23, first mapping, mirrored | 18 | 28.5 | 28.4 | 28.2 | 27.6 | 27.6 | 28.5 | 28.6 | 28.4 | 27.7 | 27.8 |
| Seed 23, first mapping, mirrored | 24 | 30.0 | 25.4 | 26.3 | 26.3 | 25.0 | 26.3 | 26.8 | 25.4 | 24.9 | 25.6 |
| Seed 23, first mapping, mirrored | 36 | 29.3 | 24.5 | 25.7 | 24.9 | 24.1 | 25.5 | 26.1 | 25.4 | 24.3 | 24.6 |
| Seed 24, eighth, assignment | 6 | 21.3 | 22.8 | 23.3 | 23.5 | 23.8 | 23.4 | 23.6 | 23.0 | 23.2 | 23.6 |
| Seed 24, eighth, assignment | 12 | 18.4 | 19.3 | 19.5 | 19.4 | 18.8 | 20.0 | 20.2 | 18.5 | 18.9 | 18.8 |
| Seed 24, eighth, assignment | 18 | 18.0 | 13.9 | 15.2 | 16.5 | 14.4 | 16.3 | 18.2 | 13.6 | 14.3 | 16.0 |
| Seed 24, eighth, assignment | 24 | 19.2 | 13.4 | 17.3 | 16.5 | 15.3 | 16.0 | 17.6 | 14.6 | 13.8 | 17.0 |
| Seed 24, eighth, assignment | 36 | 12.9 | 13.4 | 13.8 | 16.5 | 13.1 | 16.0 | 17.4 | 13.1 | 13.4 | 14.9 |
| Seed 22, first mapping, mirrored | 6 | 30.6 | 31.3 | 31.3 | 31.4 | 31.4 | 31.4 | 31.5 | 31.4 | 31.4 | 31.3 |
| Seed 22, first mapping, mirrored | 12 | 28.2 | 28.9 | 29.6 | 28.6 | 29.3 | 29.0 | 28.9 | 29.5 | 29.0 | 29.9 |
| Seed 22, first mapping, mirrored | 18 | 28.0 | 27.3 | 28.1 | 27.7 | 27.6 | 27.9 | 27.6 | 27.8 | 27.6 | 28.5 |
| Seed 22, first mapping, mirrored | 24 | 27.5 | 27.4 | 28.5 | 28.0 | 27.6 | 28.2 | 27.6 | 28.1 | 27.4 | 28.4 |
| Seed 22, first mapping, mirrored | 36 | 27.8 | 27.0 | 28.2 | 27.5 | 27.4 | 27.7 | 26.9 | 27.7 | 26.9 | 28.1 |
| Seed 22, eighth, assignment | 6 | 24.4 | 26.8 | 26.3 | 26.1 | 26.4 | 26.6 | 26.5 | 26.4 | 26.2 | 26.3 |
| Seed 22, eighth, assignment | 12 | 22.9 | 25.9 | 26.1 | 26.2 | 25.4 | 26.2 | 26.4 | 25.7 | 25.2 | 25.7 |
| Seed 22, eighth, assignment | 18 | 24.4 | 24.3 | 25.5 | 25.8 | 23.8 | 24.9 | 25.0 | 23.8 | 24.4 | 25.5 |
| Seed 22, eighth, assignment | 24 | 26.0 | 24.8 | 26.9 | 27.3 | 25.8 | 26.1 | 25.9 | 25.6 | 25.0 | 26.5 |
| Seed 22, eighth, assignment | 36 | 24.7 | 22.0 | 24.5 | 23.7 | 23.1 | 23.6 | 23.9 | 23.1 | 23.8 | 24.2 |

| Cell | Block | B's: without replay | own group and inhibitory units | own group and outside the readouts | inhibitory units and outside the readouts | plain on A's | held within (027) | excitatory | own group alone (028) | inhibitory units alone (028) | outside the readouts alone (028) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 21.5 | 23.2 | 24.0 | 23.5 | 24.4 | 23.1 | 23.0 | 24.1 | 23.4 | 24.4 |
| Seed 22, eighth, mirrored | 12 | 18.9 | 21.2 | 23.6 | 22.0 | 23.2 | 21.9 | 20.8 | 23.0 | 21.0 | 23.6 |
| Seed 22, eighth, mirrored | 18 | 16.8 | 22.3 | 23.8 | 22.4 | 23.7 | 23.1 | 21.9 | 23.5 | 22.0 | 23.9 |
| Seed 22, eighth, mirrored | 24 | 14.0 | 20.9 | 25.5 | 22.3 | 25.5 | 22.7 | 21.3 | 24.7 | 20.6 | 26.5 |
| Seed 22, eighth, mirrored | 36 | 11.9 | 19.0 | 22.1 | 18.9 | 22.3 | 19.6 | 17.8 | 21.3 | 18.5 | 23.2 |
| Seed 22, first mapping, assignment | 6 | 26.9 | 26.9 | 27.2 | 26.8 | 26.9 | 27.0 | 27.1 | 27.2 | 26.8 | 27.0 |
| Seed 22, first mapping, assignment | 12 | 23.6 | 21.2 | 22.7 | 21.9 | 22.7 | 21.6 | 20.0 | 22.3 | 21.5 | 23.4 |
| Seed 22, first mapping, assignment | 18 | 23.7 | 21.3 | 23.4 | 22.2 | 22.2 | 21.4 | 21.3 | 22.4 | 21.5 | 23.0 |
| Seed 22, first mapping, assignment | 24 | 23.4 | 22.7 | 26.2 | 22.5 | 24.3 | 23.4 | 22.4 | 25.2 | 22.1 | 25.0 |
| Seed 22, first mapping, assignment | 36 | 23.1 | 20.7 | 23.8 | 20.8 | 22.7 | 21.3 | 20.3 | 23.6 | 20.1 | 23.0 |
| Seed 23, eighth, mirrored | 6 | 14.4 | 14.1 | 13.4 | 13.5 | 13.6 | 13.5 | 15.0 | 14.1 | 14.1 | 13.6 |
| Seed 23, eighth, mirrored | 12 | 4.2 | 7.2 | 6.7 | 5.6 | 7.0 | 6.9 | 7.4 | 6.8 | 6.6 | 6.5 |
| Seed 23, eighth, mirrored | 18 | 2.6 | 3.1 | 4.1 | 2.6 | 3.2 | 3.4 | 4.5 | 3.6 | 2.8 | 3.6 |
| Seed 23, eighth, mirrored | 24 | 1.7 | 3.0 | 4.0 | 2.9 | 3.5 | 3.6 | 3.8 | 3.8 | 2.3 | 4.0 |
| Seed 23, eighth, mirrored | 36 | 0.8 | 2.4 | 2.6 | 2.4 | 2.3 | 2.6 | 2.0 | 3.0 | 1.6 | 2.4 |
| Seed 23, first mapping, assignment | 6 | 25.0 | 25.6 | 25.2 | 25.6 | 25.4 | 25.3 | 25.3 | 25.6 | 25.4 | 25.2 |
| Seed 23, first mapping, assignment | 12 | 9.3 | 8.9 | 7.9 | 7.2 | 7.4 | 8.5 | 7.9 | 8.1 | 7.3 | 7.6 |
| Seed 23, first mapping, assignment | 18 | 7.6 | 5.1 | 5.1 | 4.2 | 4.9 | 4.9 | 5.9 | 4.8 | 4.1 | 4.5 |
| Seed 23, first mapping, assignment | 24 | 7.5 | 5.5 | 5.2 | 4.1 | 4.3 | 5.8 | 5.8 | 4.4 | 4.3 | 4.2 |
| Seed 23, first mapping, assignment | 36 | 4.0 | 3.8 | 3.7 | 3.6 | 3.9 | 3.9 | 4.3 | 3.5 | 3.6 | 3.5 |
| Seed 24, eighth, mirrored | 6 | 17.3 | 16.6 | 17.4 | 16.6 | 16.8 | 17.0 | 17.8 | 16.9 | 16.4 | 16.9 |
| Seed 24, eighth, mirrored | 12 | 16.3 | 16.6 | 16.6 | 17.3 | 16.6 | 17.4 | 17.4 | 16.7 | 16.8 | 16.5 |
| Seed 24, eighth, mirrored | 18 | 11.0 | 18.1 | 18.0 | 18.3 | 18.6 | 18.3 | 18.5 | 18.6 | 17.2 | 17.9 |
| Seed 24, eighth, mirrored | 24 | 9.0 | 14.3 | 16.5 | 16.6 | 16.8 | 16.4 | 17.3 | 15.7 | 16.3 | 16.5 |
| Seed 24, eighth, mirrored | 36 | 7.2 | 14.5 | 15.8 | 17.0 | 16.9 | 15.4 | 15.9 | 15.2 | 15.8 | 17.1 |
| Seed 24, first mapping, assignment | 6 | 29.5 | 30.7 | 30.5 | 30.6 | 30.6 | 30.8 | 30.6 | 30.6 | 30.7 | 30.7 |
| Seed 24, first mapping, assignment | 12 | 25.6 | 26.0 | 26.9 | 25.5 | 26.2 | 26.7 | 26.6 | 26.7 | 25.4 | 25.9 |
| Seed 24, first mapping, assignment | 18 | 22.6 | 21.7 | 22.1 | 22.4 | 22.5 | 22.3 | 23.4 | 22.9 | 22.1 | 22.0 |
| Seed 24, first mapping, assignment | 24 | 21.4 | 22.7 | 23.9 | 23.3 | 24.0 | 23.3 | 23.6 | 24.0 | 23.4 | 23.8 |
| Seed 24, first mapping, assignment | 36 | 18.8 | 17.9 | 18.9 | 19.6 | 19.3 | 18.6 | 19.7 | 18.2 | 20.4 | 19.6 |
| Seed 23, eighth, assignment | 6 | 23.1 | 23.4 | 22.9 | 23.9 | 23.4 | 23.2 | 23.7 | 23.0 | 23.8 | 23.5 |
| Seed 23, eighth, assignment | 12 | 22.6 | 23.9 | 21.8 | 23.2 | 22.2 | 24.2 | 24.3 | 22.0 | 23.9 | 22.1 |
| Seed 23, eighth, assignment | 18 | 21.7 | 25.2 | 21.4 | 25.6 | 24.4 | 26.0 | 25.7 | 22.1 | 26.3 | 25.6 |
| Seed 23, eighth, assignment | 24 | 21.7 | 25.3 | 22.3 | 27.0 | 24.1 | 26.8 | 25.8 | 20.9 | 26.2 | 25.1 |
| Seed 23, eighth, assignment | 36 | 23.2 | 24.3 | 22.5 | 26.5 | 25.0 | 26.1 | 26.1 | 21.9 | 25.9 | 25.2 |
| Seed 24, first mapping, mirrored | 6 | 26.2 | 27.6 | 27.4 | 27.5 | 27.6 | 27.5 | 27.8 | 27.5 | 27.5 | 27.4 |
| Seed 24, first mapping, mirrored | 12 | 21.7 | 23.7 | 24.8 | 23.1 | 23.8 | 23.7 | 25.1 | 24.0 | 23.3 | 23.3 |
| Seed 24, first mapping, mirrored | 18 | 22.3 | 24.8 | 26.2 | 23.9 | 24.8 | 25.0 | 26.5 | 25.5 | 24.5 | 24.8 |
| Seed 24, first mapping, mirrored | 24 | 22.3 | 26.0 | 27.8 | 26.1 | 26.2 | 27.3 | 27.6 | 27.3 | 26.7 | 26.4 |
| Seed 24, first mapping, mirrored | 36 | 20.8 | 23.9 | 24.8 | 24.0 | 23.5 | 25.1 | 24.6 | 24.8 | 24.4 | 23.6 |
| Seed 23, first mapping, mirrored | 6 | 24.6 | 25.3 | 25.4 | 25.8 | 25.9 | 25.5 | 25.4 | 25.3 | 25.8 | 25.7 |
| Seed 23, first mapping, mirrored | 12 | 23.8 | 25.0 | 24.6 | 26.1 | 24.8 | 25.0 | 25.3 | 23.9 | 25.9 | 25.6 |
| Seed 23, first mapping, mirrored | 18 | 23.5 | 25.4 | 23.1 | 26.0 | 24.8 | 25.3 | 25.6 | 23.1 | 26.7 | 24.3 |
| Seed 23, first mapping, mirrored | 24 | 23.5 | 26.6 | 24.1 | 26.5 | 25.9 | 26.3 | 26.5 | 24.4 | 27.7 | 25.6 |
| Seed 23, first mapping, mirrored | 36 | 23.9 | 25.8 | 23.4 | 25.5 | 24.2 | 25.0 | 25.3 | 23.2 | 25.8 | 23.8 |
| Seed 24, eighth, assignment | 6 | 17.5 | 15.0 | 15.1 | 15.3 | 15.3 | 14.9 | 15.7 | 14.6 | 15.6 | 15.2 |
| Seed 24, eighth, assignment | 12 | 16.4 | 15.1 | 16.0 | 16.1 | 15.4 | 14.5 | 16.6 | 14.9 | 15.2 | 16.3 |
| Seed 24, eighth, assignment | 18 | 16.5 | 16.4 | 16.1 | 15.2 | 14.1 | 15.3 | 17.4 | 16.2 | 15.6 | 15.7 |
| Seed 24, eighth, assignment | 24 | 16.3 | 15.6 | 15.0 | 16.1 | 14.3 | 16.4 | 19.1 | 15.1 | 15.9 | 14.7 |
| Seed 24, eighth, assignment | 36 | 12.7 | 13.1 | 13.4 | 14.7 | 14.3 | 13.7 | 16.9 | 13.2 | 13.6 | 14.3 |
| Seed 22, first mapping, mirrored | 6 | 26.0 | 27.0 | 26.6 | 26.7 | 26.4 | 26.9 | 26.6 | 26.6 | 26.8 | 26.1 |
| Seed 22, first mapping, mirrored | 12 | 21.3 | 24.1 | 23.5 | 24.4 | 23.1 | 24.0 | 23.8 | 23.1 | 24.2 | 24.0 |
| Seed 22, first mapping, mirrored | 18 | 21.0 | 25.5 | 24.5 | 25.4 | 25.1 | 25.3 | 24.7 | 24.7 | 25.7 | 25.6 |
| Seed 22, first mapping, mirrored | 24 | 18.4 | 23.7 | 23.0 | 24.0 | 23.0 | 23.6 | 23.1 | 23.1 | 23.4 | 23.9 |
| Seed 22, first mapping, mirrored | 36 | 16.4 | 22.3 | 21.7 | 22.9 | 21.8 | 22.0 | 21.9 | 21.9 | 22.0 | 22.0 |
| Seed 22, eighth, assignment | 6 | 25.9 | 26.0 | 25.8 | 26.7 | 25.8 | 26.1 | 26.1 | 25.8 | 26.2 | 26.4 |
| Seed 22, eighth, assignment | 12 | 24.8 | 26.8 | 25.8 | 26.7 | 25.3 | 26.7 | 26.4 | 25.4 | 26.4 | 25.7 |
| Seed 22, eighth, assignment | 18 | 22.8 | 27.1 | 26.5 | 27.1 | 26.8 | 27.0 | 26.8 | 26.5 | 27.2 | 26.7 |
| Seed 22, eighth, assignment | 24 | 21.3 | 25.5 | 25.3 | 26.1 | 24.4 | 26.5 | 26.2 | 24.2 | 25.9 | 25.0 |
| Seed 22, eighth, assignment | 36 | 19.5 | 23.9 | 23.6 | 24.6 | 23.2 | 25.1 | 24.9 | 22.6 | 23.7 | 24.1 |

- **Stimulus A's trials** less the night without replay lie beyond twice their paired SE at 47, 50 and 46 of the 60 checkpoints under the
  nights held onto the own group and the inhibitory units, onto the own group and the units outside the readouts, and onto the inhibitory
  units and the units outside the readouts, 42, 44 and 43 of them above; **stimulus B's** at 33, 26 and 32, 27, 23 and 27 of them above.
  Less exploration 027's night held within, stimulus A's trials lie beyond twice the unpaired SE at 4, 5 and 2 of the 60, in the same order,
  and stimulus B's at 0, 10 and 2; less the excitatory night, stimulus A's at 3, 11 and 2 and stimulus B's at 5, 16 and 4.

## 2. The side under each held night

The side under a held night is the stimulus whose difference from the night without replay is the larger. "A's less B's" is the paired
difference of the two, and "within 2 SE" marks it within twice that SE. H-47's side is the same reading of H-47's plain night, from H-47's
job; exploration 027's night held within's from exploration 027's job; the excitatory night's from exploration 025's or 026's job; and
each part held alone's from exploration 028's job. The group named is the one that stimulus answers in that cell; "neither" marks a
checkpoint where the two stimuli's differences are equal:

| Cell | Block | Own group and inhibitory units: A's less without replay | B's | A's less B's | Own group and outside the readouts: A's | B's | A's less B's | Inhibitory units and outside the readouts: A's | B's | A's less B's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +4.7 ± 0.86 | +1.7 ± 1.28 | +3.0 ± 1.92 | +4.2 ± 0.87 | +2.5 ± 1.11 | +1.6 ± 1.79 | +4.7 ± 0.87 | +2.0 ± 1.06 | +2.7 ± 1.73 |
| Seed 22, eighth, mirrored | 12 | +7.7 ± 1.17 | +2.3 ± 1.04 | +5.4 ± 1.71 | +6.8 ± 1.19 | +4.8 ± 0.84 | +2.1 ± 1.40 | +8.6 ± 0.64 | +3.2 ± 0.98 | +5.4 ± 1.22 |
| Seed 22, eighth, mirrored | 18 | +8.3 ± 1.94 | +5.5 ± 1.08 | +2.8 ± 2.55 | +7.2 ± 1.72 | +6.9 ± 1.32 | +0.2 ± 2.25 | +9.1 ± 1.11 | +5.6 ± 1.02 | +3.5 ± 1.28 |
| Seed 22, eighth, mirrored | 24 | +5.2 ± 1.70 | +6.9 ± 0.82 | −1.7 ± 2.32 | +4.7 ± 1.50 | +11.5 ± 0.88 | −6.8 ± 1.96 | +7.6 ± 1.17 | +8.3 ± 0.85 | −0.7 ± 1.17 |
| Seed 22, eighth, mirrored | 36 | +8.3 ± 1.19 | +7.1 ± 1.08 | +1.1 ± 2.04 | +8.3 ± 0.55 | +10.3 ± 0.74 | −2.0 ± 1.04 | +10.9 ± 0.67 | +7.0 ± 0.53 | +3.9 ± 0.04 |
| Seed 22, first mapping, assignment | 6 | +4.1 ± 0.56 | 0.0 ± 0.61 | +4.1 ± 0.53 | +4.1 ± 0.37 | +0.3 ± 0.61 | +3.9 ± 0.47 | +3.9 ± 0.47 | −0.1 ± 0.73 | +4.0 ± 0.38 |
| Seed 22, first mapping, assignment | 12 | +4.6 ± 0.79 | −2.4 ± 0.69 | +7.0 ± 1.38 | +4.4 ± 0.52 | −0.9 ± 0.72 | +5.3 ± 1.04 | +4.2 ± 0.79 | −1.8 ± 0.55 | +6.0 ± 1.22 |
| Seed 22, first mapping, assignment | 18 | +9.2 ± 1.17 | −2.4 ± 0.93 | +11.6 ± 2.00 | +8.8 ± 0.96 | −0.3 ± 0.76 | +9.1 ± 1.47 | +9.5 ± 1.05 | −1.5 ± 0.89 | +11.0 ± 1.83 |
| Seed 22, first mapping, assignment | 24 | +8.4 ± 1.33 | −0.8 ± 1.23 | +9.1 ± 2.29 | +8.4 ± 1.16 | +2.8 ± 0.85 | +5.5 ± 1.76 | +9.1 ± 1.55 | −0.9 ± 0.97 | +10.1 ± 2.09 |
| Seed 22, first mapping, assignment | 36 | +8.8 ± 0.95 | −2.4 ± 0.82 | +11.3 ± 1.46 | +9.3 ± 1.23 | +0.6 ± 0.35 | +8.6 ± 1.41 | +9.4 ± 0.67 | −2.4 ± 0.58 | +11.8 ± 0.96 |
| Seed 23, eighth, mirrored | 6 | +1.9 ± 0.82 | −0.3 ± 0.78 | +2.2 ± 1.19 | +2.4 ± 0.77 | −1.1 ± 0.80 | +3.5 ± 1.12 | +2.1 ± 0.80 | −0.9 ± 0.76 | +3.0 ± 1.19 |
| Seed 23, eighth, mirrored | 12 | +3.5 ± 0.64 | +3.0 ± 0.57 | +0.4 ± 0.70 | +5.9 ± 0.58 | +2.6 ± 0.52 | +3.4 ± 0.75 | +4.8 ± 0.55 | +1.4 ± 0.39 | +3.4 ± 0.59 |
| Seed 23, eighth, mirrored | 18 | +9.0 ± 1.20 | +0.5 ± 0.92 | +8.5 ± 1.66 | +10.1 ± 1.34 | +1.5 ± 1.10 | +8.6 ± 0.66 | +9.2 ± 1.24 | 0.0 ± 0.78 | +9.3 ± 1.48 |
| Seed 23, eighth, mirrored | 24 | +14.6 ± 1.30 | +1.3 ± 0.68 | +13.2 ± 1.27 | +14.7 ± 1.81 | +2.3 ± 0.58 | +12.4 ± 1.49 | +14.1 ± 1.26 | +1.2 ± 0.48 | +13.0 ± 1.19 |
| Seed 23, eighth, mirrored | 36 | +13.4 ± 0.73 | +1.6 ± 0.47 | +11.8 ± 0.71 | +14.8 ± 1.18 | +1.8 ± 0.06 | +13.0 ± 1.16 | +15.0 ± 1.35 | +1.6 ± 0.45 | +13.4 ± 1.18 |
| Seed 23, first mapping, assignment | 6 | +1.7 ± 0.45 | +0.6 ± 0.64 | +1.1 ± 0.75 | +1.8 ± 0.46 | +0.2 ± 0.61 | +1.6 ± 0.69 | +1.7 ± 0.47 | +0.6 ± 0.75 | +1.1 ± 0.89 |
| Seed 23, first mapping, assignment | 12 | +6.0 ± 0.83 | −0.4 ± 0.98 | +6.4 ± 1.52 | +5.9 ± 0.84 | −1.4 ± 0.82 | +7.3 ± 1.30 | +6.4 ± 0.98 | −2.0 ± 1.56 | +8.4 ± 1.90 |
| Seed 23, first mapping, assignment | 18 | +10.3 ± 1.65 | −2.6 ± 0.86 | +12.8 ± 1.59 | +10.2 ± 1.58 | −2.5 ± 1.11 | +12.8 ± 1.38 | +10.3 ± 1.32 | −3.4 ± 1.03 | +13.7 ± 1.32 |
| Seed 23, first mapping, assignment | 24 | +12.4 ± 1.65 | −2.0 ± 0.41 | +14.4 ± 1.65 | +12.4 ± 1.65 | −2.4 ± 0.69 | +14.8 ± 1.58 | +13.0 ± 1.81 | −3.4 ± 0.55 | +16.4 ± 1.76 |
| Seed 23, first mapping, assignment | 36 | +13.0 ± 1.51 | −0.2 ± 0.39 | +13.2 ± 1.65 | +13.4 ± 1.17 | −0.3 ± 0.82 | +13.6 ± 0.90 | +13.6 ± 1.19 | −0.4 ± 0.32 | +14.0 ± 0.97 |
| Seed 24, eighth, mirrored | 6 | −0.9 ± 0.80 | −0.6 ± 0.96 | −0.3 ± 1.25 | −2.4 ± 0.80 | +0.1 ± 1.22 | −2.5 ± 1.47 | −0.6 ± 0.83 | −0.7 ± 0.91 | 0.0 ± 1.38 |
| Seed 24, eighth, mirrored | 12 | +0.3 ± 0.81 | +0.3 ± 0.86 | 0.0 ± 1.06 | −1.5 ± 0.64 | +0.4 ± 1.11 | −1.9 ± 1.23 | +0.7 ± 0.83 | +1.0 ± 0.97 | −0.3 ± 1.09 |
| Seed 24, eighth, mirrored | 18 | +5.3 ± 0.83 | +7.1 ± 1.24 | −1.7 ± 1.51 | +4.7 ± 0.89 | +7.0 ± 1.25 | −2.3 ± 1.87 | +5.9 ± 1.00 | +7.3 ± 1.40 | −1.4 ± 1.77 |
| Seed 24, eighth, mirrored | 24 | +7.8 ± 0.90 | +5.3 ± 1.49 | +2.4 ± 1.76 | +4.2 ± 1.39 | +7.5 ± 2.08 | −3.3 ± 2.52 | +7.6 ± 1.25 | +7.6 ± 1.49 | 0.0 ± 1.98 |
| Seed 24, eighth, mirrored | 36 | +7.6 ± 0.53 | +7.3 ± 1.08 | +0.3 ± 1.39 | +4.7 ± 0.43 | +8.6 ± 2.31 | −3.9 ± 2.03 | +6.7 ± 0.55 | +9.8 ± 1.30 | −3.1 ± 1.42 |
| Seed 24, first mapping, assignment | 6 | +2.0 ± 0.52 | +1.2 ± 0.43 | +0.8 ± 0.64 | +2.0 ± 0.57 | +1.0 ± 0.32 | +1.0 ± 0.63 | +2.0 ± 0.50 | +1.1 ± 0.37 | +0.9 ± 0.57 |
| Seed 24, first mapping, assignment | 12 | +3.4 ± 0.52 | +0.4 ± 1.09 | +3.1 ± 1.44 | +3.4 ± 0.46 | +1.3 ± 1.20 | +2.0 ± 1.42 | +3.3 ± 0.54 | −0.1 ± 1.02 | +3.4 ± 1.42 |
| Seed 24, first mapping, assignment | 18 | +5.0 ± 0.38 | −0.9 ± 0.62 | +5.9 ± 0.70 | +5.1 ± 0.52 | −0.5 ± 1.77 | +5.6 ± 1.73 | +5.3 ± 0.39 | −0.2 ± 1.16 | +5.5 ± 1.30 |
| Seed 24, first mapping, assignment | 24 | +7.3 ± 0.76 | +1.2 ± 1.52 | +6.1 ± 1.30 | +7.9 ± 1.05 | +2.4 ± 1.78 | +5.5 ± 1.21 | +7.8 ± 0.95 | +1.8 ± 1.79 | +6.0 ± 1.22 |
| Seed 24, first mapping, assignment | 36 | +6.1 ± 0.61 | −1.0 ± 2.02 | +7.1 ± 2.49 | +5.2 ± 0.48 | 0.0 ± 0.93 | +5.1 ± 1.32 | +5.4 ± 0.55 | +0.7 ± 0.90 | +4.7 ± 0.97 |
| Seed 23, eighth, assignment | 6 | +2.9 ± 0.56 | +0.3 ± 0.66 | +2.6 ± 0.80 | +2.8 ± 0.75 | −0.1 ± 0.58 | +2.9 ± 0.84 | +2.8 ± 0.65 | +0.8 ± 0.58 | +1.9 ± 0.84 |
| Seed 23, eighth, assignment | 12 | +4.2 ± 0.87 | +1.3 ± 0.81 | +2.9 ± 1.11 | +3.5 ± 1.02 | −0.8 ± 0.99 | +4.3 ± 1.01 | +4.5 ± 0.72 | +0.6 ± 0.72 | +3.9 ± 1.02 |
| Seed 23, eighth, assignment | 18 | +7.1 ± 0.86 | +3.5 ± 0.57 | +3.6 ± 0.85 | +8.4 ± 0.83 | −0.4 ± 0.80 | +8.7 ± 0.91 | +8.0 ± 1.27 | +3.9 ± 0.49 | +4.1 ± 1.48 |
| Seed 23, eighth, assignment | 24 | +7.5 ± 1.01 | +3.6 ± 0.94 | +3.9 ± 1.14 | +8.9 ± 1.06 | +0.6 ± 0.92 | +8.2 ± 1.07 | +8.0 ± 1.14 | +5.3 ± 1.03 | +2.7 ± 1.33 |
| Seed 23, eighth, assignment | 36 | +6.3 ± 0.39 | +1.1 ± 1.64 | +5.3 ± 1.89 | +7.0 ± 0.37 | −0.7 ± 1.74 | +7.7 ± 1.71 | +6.0 ± 0.32 | +3.3 ± 1.59 | +2.7 ± 1.89 |
| Seed 24, first mapping, mirrored | 6 | +1.7 ± 0.56 | +1.4 ± 0.63 | +0.3 ± 0.88 | +1.6 ± 0.51 | +1.2 ± 0.68 | +0.4 ± 0.85 | +1.9 ± 0.52 | +1.3 ± 0.64 | +0.5 ± 0.86 |
| Seed 24, first mapping, mirrored | 12 | +2.4 ± 0.68 | +2.0 ± 0.79 | +0.5 ± 0.92 | +3.0 ± 0.76 | +3.1 ± 1.50 | −0.1 ± 1.42 | +2.1 ± 0.68 | +1.4 ± 1.24 | +0.8 ± 1.16 |
| Seed 24, first mapping, mirrored | 18 | +4.4 ± 0.43 | +2.5 ± 0.74 | +1.9 ± 0.66 | +5.8 ± 0.49 | +3.9 ± 0.56 | +1.9 ± 0.50 | +5.0 ± 0.44 | +1.6 ± 0.85 | +3.4 ± 0.72 |
| Seed 24, first mapping, mirrored | 24 | +3.7 ± 1.12 | +3.7 ± 0.56 | 0.0 ± 1.30 | +4.3 ± 0.75 | +5.5 ± 0.54 | −1.2 ± 0.93 | +3.8 ± 1.25 | +3.9 ± 0.82 | −0.1 ± 1.19 |
| Seed 24, first mapping, mirrored | 36 | +3.1 ± 0.77 | +3.1 ± 1.09 | −0.1 ± 0.90 | +3.9 ± 0.75 | +4.1 ± 1.19 | −0.2 ± 1.05 | +3.8 ± 0.72 | +3.3 ± 0.64 | +0.5 ± 0.58 |
| Seed 23, first mapping, mirrored | 6 | +0.7 ± 0.33 | +0.7 ± 0.67 | +0.1 ± 0.68 | +0.8 ± 0.34 | +0.9 ± 0.56 | −0.1 ± 0.62 | +0.5 ± 0.36 | +1.2 ± 0.71 | −0.7 ± 0.74 |
| Seed 23, first mapping, mirrored | 12 | +1.1 ± 0.57 | +1.2 ± 0.97 | 0.0 ± 1.12 | +0.9 ± 0.61 | +0.8 ± 0.91 | +0.1 ± 1.10 | +1.1 ± 0.65 | +2.3 ± 0.89 | −1.2 ± 1.00 |
| Seed 23, first mapping, mirrored | 18 | −0.2 ± 0.44 | +1.9 ± 0.73 | −2.1 ± 0.81 | −0.3 ± 0.48 | −0.4 ± 0.75 | 0.0 ± 0.92 | −0.9 ± 0.68 | +2.6 ± 0.84 | −3.5 ± 0.62 |
| Seed 23, first mapping, mirrored | 24 | −4.6 ± 0.96 | +3.2 ± 0.43 | −7.8 ± 1.13 | −3.7 ± 0.56 | +0.7 ± 0.55 | −4.4 ± 0.88 | −3.7 ± 0.75 | +3.1 ± 0.69 | −6.8 ± 1.31 |
| Seed 23, first mapping, mirrored | 36 | −4.8 ± 0.77 | +1.9 ± 0.83 | −6.6 ± 1.29 | −3.5 ± 0.74 | −0.5 ± 1.07 | −3.1 ± 1.47 | −4.3 ± 0.74 | +1.6 ± 0.99 | −6.0 ± 1.66 |
| Seed 24, eighth, assignment | 6 | +1.5 ± 0.76 | −2.5 ± 0.63 | +4.1 ± 1.20 | +2.0 ± 0.90 | −2.4 ± 0.62 | +4.4 ± 1.31 | +2.2 ± 0.91 | −2.2 ± 0.66 | +4.4 ± 1.41 |
| Seed 24, eighth, assignment | 12 | +0.9 ± 0.98 | −1.3 ± 1.59 | +2.2 ± 2.33 | +1.1 ± 1.32 | −0.4 ± 1.95 | +1.4 ± 3.09 | +0.9 ± 1.16 | −0.3 ± 1.09 | +1.2 ± 1.98 |
| Seed 24, eighth, assignment | 18 | −4.0 ± 1.06 | 0.0 ± 1.22 | −4.0 ± 1.21 | −2.7 ± 0.92 | −0.4 ± 1.06 | −2.3 ± 1.51 | −1.5 ± 0.78 | −1.2 ± 2.26 | −0.3 ± 2.50 |
| Seed 24, eighth, assignment | 24 | −5.9 ± 1.05 | −0.7 ± 1.27 | −5.2 ± 1.37 | −2.0 ± 0.91 | −1.3 ± 0.88 | −0.7 ± 1.22 | −2.7 ± 1.08 | −0.2 ± 2.14 | −2.5 ± 1.85 |
| Seed 24, eighth, assignment | 36 | +0.5 ± 0.72 | +0.5 ± 0.99 | 0.0 ± 1.57 | +0.8 ± 0.84 | +0.8 ± 0.97 | +0.1 ± 1.46 | +3.6 ± 0.65 | +2.0 ± 1.32 | +1.6 ± 1.23 |
| Seed 22, first mapping, mirrored | 6 | +0.7 ± 0.37 | +1.0 ± 0.49 | −0.3 ± 0.64 | +0.6 ± 0.34 | +0.6 ± 0.47 | +0.1 ± 0.70 | +0.8 ± 0.34 | +0.7 ± 0.47 | +0.1 ± 0.63 |
| Seed 22, first mapping, mirrored | 12 | +0.8 ± 0.56 | +2.7 ± 1.52 | −2.0 ± 1.35 | +1.4 ± 0.63 | +2.2 ± 1.41 | −0.8 ± 1.21 | +0.5 ± 0.64 | +3.0 ± 1.62 | −2.6 ± 1.44 |
| Seed 22, first mapping, mirrored | 18 | −0.6 ± 1.23 | +4.4 ± 1.36 | −5.1 ± 1.35 | +0.1 ± 1.00 | +3.4 ± 1.53 | −3.3 ± 1.17 | −0.3 ± 1.26 | +4.4 ± 1.32 | −4.6 ± 1.21 |
| Seed 22, first mapping, mirrored | 24 | −0.1 ± 1.15 | +5.3 ± 1.26 | −5.4 ± 1.24 | +1.0 ± 1.10 | +4.6 ± 1.35 | −3.6 ± 1.16 | +0.5 ± 1.27 | +5.6 ± 0.92 | −5.1 ± 1.13 |
| Seed 22, first mapping, mirrored | 36 | −0.8 ± 0.86 | +5.9 ± 1.29 | −6.6 ± 1.37 | +0.4 ± 0.86 | +5.3 ± 1.33 | −4.9 ± 1.64 | −0.3 ± 1.13 | +6.5 ± 1.03 | −6.8 ± 1.38 |
| Seed 22, eighth, assignment | 6 | +2.4 ± 0.62 | +0.1 ± 0.60 | +2.3 ± 0.89 | +1.9 ± 0.61 | −0.2 ± 0.52 | +2.0 ± 0.73 | +1.8 ± 0.54 | +0.8 ± 0.55 | +1.0 ± 0.94 |
| Seed 22, eighth, assignment | 12 | +3.0 ± 0.50 | +2.0 ± 0.66 | +1.0 ± 0.84 | +3.2 ± 0.79 | +1.0 ± 0.89 | +2.2 ± 1.44 | +3.3 ± 0.50 | +2.0 ± 0.53 | +1.3 ± 0.71 |
| Seed 22, eighth, assignment | 18 | −0.1 ± 0.52 | +4.4 ± 0.68 | −4.5 ± 0.79 | +1.1 ± 0.47 | +3.7 ± 0.72 | −2.6 ± 0.52 | +1.4 ± 0.90 | +4.3 ± 0.56 | −2.9 ± 1.13 |
| Seed 22, eighth, assignment | 24 | −1.3 ± 0.42 | +4.2 ± 0.68 | −5.5 ± 0.75 | +0.9 ± 0.52 | +4.0 ± 1.14 | −3.1 ± 0.98 | +1.3 ± 0.80 | +4.8 ± 0.65 | −3.5 ± 0.94 |
| Seed 22, eighth, assignment | 36 | −2.8 ± 1.73 | +4.4 ± 0.76 | −7.2 ± 2.25 | −0.2 ± 1.32 | +4.1 ± 1.03 | −4.3 ± 2.17 | −1.1 ± 1.53 | +5.1 ± 0.69 | −6.2 ± 2.06 |

| Cell | Block | Own group and inhibitory units: side | Own group and outside the readouts: side | Inhibitory units and outside the readouts: side | H-47's side | Held within (027): side | Excitatory: side | Own group alone (028): side | Inhibitory units alone (028): side | Outside the readouts alone (028): side |
| :--- | ---: | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Seed 22, eighth, mirrored | 6 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | 12 | A, the odd places | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places | A, the odd places | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | 18 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) |
| Seed 22, eighth, mirrored | 24 | B, the even places (within 2 SE) | B, the even places | B, the even places (within 2 SE) | B, the even places | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places | B, the even places (within 2 SE) | B, the even places |
| Seed 22, eighth, mirrored | 36 | A, the odd places (within 2 SE) | B, the even places (within 2 SE) | A, the odd places | B, the even places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places | B, the even places (within 2 SE) | A, the odd places | B, the even places |
| Seed 22, first mapping, assignment | 6 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 12 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 18 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 24 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 22, first mapping, assignment | 36 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | 6 | A, the odd places (within 2 SE) | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places |
| Seed 23, eighth, mirrored | 12 | A, the odd places (within 2 SE) | A, the odd places | A, the odd places | A, the odd places | A, the odd places (within 2 SE) | B, the even places (within 2 SE) | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | 18 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | 24 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | 36 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 6 | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places |
| Seed 23, first mapping, assignment | 12 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 18 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 24 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | 36 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 24, eighth, mirrored | 6 | B, the even places (within 2 SE) | B, the even places (within 2 SE) | A, the odd places (within 2 SE) | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places (within 2 SE) | A, the odd places (within 2 SE) | B, the even places (within 2 SE) |
| Seed 24, eighth, mirrored | 12 | neither | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places | B, the even places (within 2 SE) | B, the even places (within 2 SE) |
| Seed 24, eighth, mirrored | 18 | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places | A, the odd places (within 2 SE) | B, the even places (within 2 SE) | B, the even places | B, the even places (within 2 SE) | B, the even places |
| Seed 24, eighth, mirrored | 24 | A, the odd places (within 2 SE) | B, the even places (within 2 SE) | A, the odd places (within 2 SE) | B, the even places | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places (within 2 SE) |
| Seed 24, eighth, mirrored | 36 | A, the odd places (within 2 SE) | B, the even places (within 2 SE) | B, the even places | B, the even places | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places | B, the even places |
| Seed 24, first mapping, assignment | 6 | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) |
| Seed 24, first mapping, assignment | 12 | A, the odd places | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places | A, the odd places (within 2 SE) |
| Seed 24, first mapping, assignment | 18 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 24, first mapping, assignment | 24 | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 24, first mapping, assignment | 36 | A, the odd places | A, the odd places | A, the odd places | A, the odd places (within 2 SE) | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, assignment | 6 | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places (within 2 SE) |
| Seed 23, eighth, assignment | 12 | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places |
| Seed 23, eighth, assignment | 18 | A, the even places | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places |
| Seed 23, eighth, assignment | 24 | A, the even places | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 23, eighth, assignment | 36 | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | 6 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | 12 | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | 18 | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places | A, the even places |
| Seed 24, first mapping, mirrored | 24 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 24, first mapping, mirrored | 36 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 23, first mapping, mirrored | 6 | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 12 | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 18 | B, the odd places | A, the even places (within 2 SE) | B, the odd places | B, the odd places | B, the odd places (within 2 SE) | B, the odd places | A, the even places (within 2 SE) | B, the odd places | B, the odd places (within 2 SE) |
| Seed 23, first mapping, mirrored | 24 | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 23, first mapping, mirrored | 36 | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places (within 2 SE) | B, the odd places | B, the odd places |
| Seed 24, eighth, assignment | 6 | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places |
| Seed 24, eighth, assignment | 12 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, eighth, assignment | 18 | B, the odd places | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | 24 | B, the odd places | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places (within 2 SE) |
| Seed 24, eighth, assignment | 36 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | 6 | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | 12 | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) |
| Seed 22, first mapping, mirrored | 18 | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, first mapping, mirrored | 24 | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, first mapping, mirrored | 36 | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | 6 | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places | A, the even places | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places |
| Seed 22, eighth, assignment | 12 | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 22, eighth, assignment | 18 | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | 24 | B, the odd places | B, the odd places | B, the odd places | B, the odd places (within 2 SE) | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | 36 | B, the odd places | B, the odd places (within 2 SE) | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |

- **The side under the night held onto the own group and the inhibitory units is H-47's at 51 of the 60 checkpoints**, exploration 027's
  night held within's at 55 and the excitatory night's at 53. It lies beyond twice its SE at 36 of the 60, where H-47's and the excitatory
  night's lie beyond at 35 and exploration 027's night held within's at 36. On seed 24's eighth image from the mirrored assignment at block
  12 it is neither: A's and B's differences are equal. It differs from H-47's at nine:
  - seed 22's eighth image from the mirrored assignment at block 36: on A, the odd places, within 2 SE, where H-47's lies on B, the even
    places, within 2 SE;
  - seed 24's eighth image from the mirrored assignment at block 12, where it is neither, and at blocks 24 and 36, on A, the odd places,
    within 2 SE, where H-47's lies on B, the even places, beyond 2 SE at all three;
  - seed 24's first mapping from the mirrored assignment at block 36, seed 23's first mapping from the mirrored assignment at block 12 and
    seed 22's first mapping from the mirrored assignment at block 6: on B, the odd places, within 2 SE, where H-47's lies on A, the even
    places, within 2 SE;
  - seed 23's first mapping from the mirrored assignment at block 6 and seed 24's eighth image from the assignment at block 36: on A, the
    even places, within 2 SE, where H-47's lies on B, the odd places, within 2 SE.
- **The side under the night held onto the own group and the units outside the readouts is H-47's at 56 of the 60 checkpoints**,
  exploration 027's night held within's at 54 and the excitatory night's at 56. It lies beyond twice its SE at 34 of the 60. It differs from
  H-47's at four:
  - seed 24's first mapping from the mirrored assignment at blocks 12 and 36: on B, the odd places, within 2 SE, where H-47's lies on A, the
    even places, within 2 SE;
  - seed 23's first mapping from the mirrored assignment at block 18: on A, the even places, within 2 SE, where H-47's lies on B, the odd
    places, beyond 2 SE;
  - seed 24's eighth image from the assignment at block 36: on A, the even places, within 2 SE, where H-47's lies on B, the odd places,
    within 2 SE.
- **The side under the night held onto the inhibitory units and the units outside the readouts is H-47's at 55 of the 60 checkpoints**,
  exploration 027's night held within's at 55 and the excitatory night's at 55. It lies beyond twice its SE at 37 of the 60. It differs from
  H-47's at five:
  - seed 22's eighth image from the mirrored assignment at block 36: on A, the odd places, beyond 2 SE, where H-47's lies on B, the even
    places, within 2 SE;
  - seed 24's eighth image from the mirrored assignment at blocks 6 and 24: on A, the odd places, within 2 SE, where H-47's lies on B, the
    even places, within 2 SE at block 6 and beyond 2 SE at block 24;
  - seed 23's first mapping from the mirrored assignment at block 12: on B, the odd places, within 2 SE, where H-47's lies on A, the even
    places, within 2 SE;
  - seed 24's eighth image from the assignment at block 36: on A, the even places, within 2 SE, where H-47's lies on B, the odd places,
    within 2 SE.
- **At block 36** each held night's side is exploration 027's night held within's and the excitatory night's in eleven of the twelve cells.
  The cell where it is not is seed 24's eighth image from the mirrored assignment under the night held onto the own group and the inhibitory
  units, where its side lies on A, the odd places, within 2 SE, and theirs on B, the even places, within 2 SE; seed 22's eighth image from
  the mirrored assignment under the night held onto the own group and the units outside the readouts, on B, the even places, within 2 SE,
  where theirs lies on A, the odd places, within 2 SE under exploration 027's night held within and beyond 2 SE under the excitatory night;
  and seed 24's first mapping from the mirrored assignment under the night held onto the inhibitory units and the units outside the
  readouts, on A, the even places, within 2 SE, where theirs lies on B, the odd places, within 2 SE. At block 36 the side is H-47's in eight
  of the twelve cells under the night held onto the own group and the inhibitory units and in ten under each of the other two.
- **Against exploration 028's nights** the side under the night held onto the own group and the inhibitory units is that of the night with
  the own group held alone at 52 of the 60, the inhibitory units alone at 54 and the units outside the readouts alone at 53; under the
  night held onto the own group and the units outside the readouts at 57, 53 and 56; and under the night held onto the inhibitory units and
  the units outside the readouts at 52, 56 and 57.

Each stimulus's trials less H-47's plain night's, less exploration 027's night held within's and less the excitatory night's, each with
exploration 018's unpaired SE:

| Cell | Block | Own group and inhibitory units less H-47's plain: A's | B's | Own group and outside the readouts: A's | B's | Inhibitory units and outside the readouts: A's | B's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +0.5 ± 0.92 | −1.1 ± 0.99 | −0.1 ± 0.84 | −0.3 ± 0.92 | +0.5 ± 0.90 | −0.8 ± 0.98 |
| Seed 22, eighth, mirrored | 12 | −0.1 ± 1.06 | −2.0 ± 0.87 | −0.9 ± 1.02 | +0.5 ± 0.88 | +0.8 ± 1.04 | −1.1 ± 0.86 |
| Seed 22, eighth, mirrored | 18 | −0.3 ± 1.07 | −1.4 ± 1.09 | −1.4 ± 1.00 | 0.0 ± 0.97 | +0.5 ± 1.24 | −1.3 ± 1.31 |
| Seed 22, eighth, mirrored | 24 | −0.9 ± 0.99 | −4.7 ± 1.18 | −1.4 ± 0.92 | −0.1 ± 1.01 | +1.6 ± 0.80 | −3.3 ± 1.39 |
| Seed 22, eighth, mirrored | 36 | −1.0 ± 1.30 | −3.3 ± 1.16 | −0.9 ± 0.83 | −0.2 ± 0.76 | +1.6 ± 0.84 | −3.4 ± 0.85 |
| Seed 22, first mapping, assignment | 6 | 0.0 ± 0.85 | +0.1 ± 0.66 | 0.0 ± 0.78 | +0.3 ± 0.67 | −0.1 ± 0.82 | 0.0 ± 0.78 |
| Seed 22, first mapping, assignment | 12 | +0.9 ± 0.86 | −1.5 ± 1.39 | +0.6 ± 0.75 | 0.0 ± 1.39 | +0.4 ± 0.88 | −0.8 ± 1.28 |
| Seed 22, first mapping, assignment | 18 | +0.9 ± 0.93 | −0.8 ± 1.24 | +0.5 ± 0.84 | +1.2 ± 0.98 | +1.1 ± 0.87 | 0.0 ± 1.21 |
| Seed 22, first mapping, assignment | 24 | +0.1 ± 0.73 | −1.6 ± 1.38 | +0.1 ± 0.83 | +2.0 ± 1.14 | +0.9 ± 0.59 | −1.8 ± 1.27 |
| Seed 22, first mapping, assignment | 36 | +0.5 ± 0.90 | −2.0 ± 1.13 | +0.9 ± 0.84 | +1.1 ± 0.97 | +1.0 ± 0.97 | −2.0 ± 1.19 |
| Seed 23, eighth, mirrored | 6 | −0.7 ± 1.14 | +0.5 ± 0.95 | −0.1 ± 1.18 | −0.2 ± 0.92 | −0.4 ± 1.19 | −0.1 ± 0.81 |
| Seed 23, eighth, mirrored | 12 | −2.7 ± 0.88 | +0.2 ± 0.71 | −0.2 ± 1.16 | −0.3 ± 0.70 | −1.4 ± 1.21 | −1.4 ± 0.77 |
| Seed 23, eighth, mirrored | 18 | −3.1 ± 1.09 | −0.1 ± 0.73 | −2.0 ± 0.83 | +0.9 ± 1.26 | −2.9 ± 0.98 | −0.6 ± 0.68 |
| Seed 23, eighth, mirrored | 24 | −2.3 ± 1.09 | −0.4 ± 0.69 | −2.2 ± 0.96 | +0.5 ± 0.54 | −2.8 ± 1.24 | −0.6 ± 0.49 |
| Seed 23, eighth, mirrored | 36 | −2.5 ± 1.35 | +0.1 ± 0.50 | −1.1 ± 1.14 | +0.3 ± 0.24 | −0.9 ± 1.33 | +0.1 ± 0.58 |
| Seed 23, first mapping, assignment | 6 | +0.1 ± 0.83 | +0.2 ± 1.07 | +0.1 ± 0.83 | −0.2 ± 0.97 | +0.1 ± 0.83 | +0.2 ± 1.12 |
| Seed 23, first mapping, assignment | 12 | +0.1 ± 0.78 | +1.4 ± 1.07 | 0.0 ± 0.81 | +0.4 ± 0.99 | +0.4 ± 0.84 | −0.2 ± 1.34 |
| Seed 23, first mapping, assignment | 18 | −0.4 ± 1.11 | +0.2 ± 0.80 | −0.4 ± 1.05 | +0.2 ± 0.75 | −0.3 ± 1.09 | −0.7 ± 0.82 |
| Seed 23, first mapping, assignment | 24 | −0.8 ± 1.00 | +1.2 ± 0.62 | −0.8 ± 0.93 | +0.9 ± 0.69 | −0.2 ± 0.90 | −0.2 ± 0.56 |
| Seed 23, first mapping, assignment | 36 | −0.5 ± 0.99 | −0.1 ± 0.60 | −0.1 ± 0.89 | −0.1 ± 0.55 | +0.1 ± 0.86 | −0.3 ± 0.53 |
| Seed 24, eighth, mirrored | 6 | +1.2 ± 0.98 | −0.1 ± 0.97 | −0.3 ± 0.98 | +0.6 ± 0.98 | +1.5 ± 1.05 | −0.2 ± 1.03 |
| Seed 24, eighth, mirrored | 12 | +3.5 ± 0.91 | 0.0 ± 1.03 | +1.6 ± 0.80 | +0.1 ± 1.23 | +3.9 ± 1.11 | +0.7 ± 1.07 |
| Seed 24, eighth, mirrored | 18 | +4.6 ± 0.91 | −0.6 ± 1.44 | +4.0 ± 0.87 | −0.6 ± 1.12 | +5.2 ± 0.93 | −0.3 ± 1.21 |
| Seed 24, eighth, mirrored | 24 | +6.1 ± 1.13 | −2.4 ± 0.82 | +2.6 ± 1.12 | −0.3 ± 1.17 | +6.0 ± 1.16 | −0.2 ± 1.08 |
| Seed 24, eighth, mirrored | 36 | +6.1 ± 0.91 | −2.4 ± 0.60 | +3.1 ± 1.15 | −1.2 ± 1.54 | +5.1 ± 1.10 | 0.0 ± 0.92 |
| Seed 24, first mapping, assignment | 6 | +0.2 ± 1.02 | +0.2 ± 0.93 | +0.1 ± 1.04 | 0.0 ± 0.93 | +0.2 ± 1.00 | +0.1 ± 0.91 |
| Seed 24, first mapping, assignment | 12 | +1.2 ± 0.98 | −0.2 ± 0.99 | +1.1 ± 0.88 | +0.7 ± 1.16 | +1.1 ± 0.91 | −0.7 ± 0.99 |
| Seed 24, first mapping, assignment | 18 | +1.0 ± 0.91 | −0.7 ± 1.52 | +1.1 ± 0.87 | −0.4 ± 1.56 | +1.3 ± 0.96 | −0.1 ± 1.40 |
| Seed 24, first mapping, assignment | 24 | +1.2 ± 1.13 | −1.4 ± 1.57 | +1.8 ± 1.14 | −0.2 ± 0.99 | +1.7 ± 1.11 | −0.8 ± 0.88 |
| Seed 24, first mapping, assignment | 36 | +2.9 ± 1.00 | −1.5 ± 0.96 | +1.9 ± 0.98 | −0.5 ± 0.98 | +2.2 ± 1.14 | +0.2 ± 0.88 |
| Seed 23, eighth, assignment | 6 | +0.3 ± 0.83 | 0.0 ± 1.18 | +0.1 ± 0.82 | −0.5 ± 1.23 | +0.1 ± 0.78 | +0.5 ± 1.16 |
| Seed 23, eighth, assignment | 12 | +1.3 ± 1.45 | +1.6 ± 1.33 | +0.6 ± 1.52 | −0.4 ± 1.29 | +1.6 ± 1.37 | +1.0 ± 1.12 |
| Seed 23, eighth, assignment | 18 | +1.8 ± 0.97 | +0.8 ± 1.41 | +3.1 ± 0.78 | −3.0 ± 1.50 | +2.7 ± 1.11 | +1.2 ± 1.24 |
| Seed 23, eighth, assignment | 24 | +4.3 ± 1.11 | +1.1 ± 1.34 | +5.6 ± 1.02 | −1.8 ± 1.54 | +4.7 ± 1.09 | +2.8 ± 1.56 |
| Seed 23, eighth, assignment | 36 | +4.0 ± 1.31 | −0.7 ± 1.27 | +4.8 ± 0.94 | −2.5 ± 1.23 | +3.7 ± 1.30 | +1.5 ± 1.22 |
| Seed 24, first mapping, mirrored | 6 | −0.3 ± 1.03 | 0.0 ± 0.90 | −0.5 ± 0.99 | −0.2 ± 0.95 | −0.1 ± 1.02 | 0.0 ± 0.95 |
| Seed 24, first mapping, mirrored | 12 | −0.2 ± 0.96 | −0.1 ± 1.05 | +0.4 ± 1.05 | +1.0 ± 0.99 | −0.5 ± 0.87 | −0.7 ± 0.97 |
| Seed 24, first mapping, mirrored | 18 | −0.9 ± 1.22 | 0.0 ± 0.93 | +0.4 ± 1.19 | +1.4 ± 1.07 | −0.4 ± 1.17 | −0.9 ± 0.90 |
| Seed 24, first mapping, mirrored | 24 | 0.0 ± 0.97 | −0.3 ± 1.12 | +0.6 ± 0.93 | +1.6 ± 1.17 | +0.1 ± 1.11 | −0.1 ± 1.12 |
| Seed 24, first mapping, mirrored | 36 | −0.2 ± 0.54 | +0.4 ± 0.84 | +0.6 ± 0.53 | +1.3 ± 0.91 | +0.6 ± 0.34 | +0.5 ± 0.89 |
| Seed 23, first mapping, mirrored | 6 | +0.1 ± 0.98 | −0.6 ± 0.89 | +0.2 ± 0.98 | −0.4 ± 0.87 | −0.1 ± 0.97 | −0.1 ± 0.95 |
| Seed 23, first mapping, mirrored | 12 | +0.1 ± 0.91 | +0.2 ± 1.45 | −0.1 ± 0.91 | −0.1 ± 1.16 | 0.0 ± 0.97 | +1.3 ± 1.38 |
| Seed 23, first mapping, mirrored | 18 | +0.8 ± 1.28 | +0.6 ± 0.96 | +0.6 ± 1.14 | −1.7 ± 0.99 | 0.0 ± 1.16 | +1.3 ± 1.03 |
| Seed 23, first mapping, mirrored | 24 | +0.4 ± 0.97 | +0.7 ± 1.04 | +1.3 ± 1.17 | −1.8 ± 1.04 | +1.3 ± 1.10 | +0.6 ± 1.27 |
| Seed 23, first mapping, mirrored | 36 | +0.4 ± 1.16 | +1.6 ± 0.89 | +1.7 ± 1.29 | −0.8 ± 0.83 | +0.9 ± 1.36 | +1.3 ± 0.83 |
| Seed 24, eighth, assignment | 6 | −1.0 ± 0.72 | −0.3 ± 0.93 | −0.6 ± 0.81 | −0.1 ± 0.92 | −0.4 ± 0.78 | 0.0 ± 0.94 |
| Seed 24, eighth, assignment | 12 | +0.5 ± 1.08 | −0.3 ± 0.94 | +0.7 ± 1.04 | +0.6 ± 1.08 | +0.5 ± 0.97 | +0.7 ± 1.23 |
| Seed 24, eighth, assignment | 18 | −0.4 ± 1.02 | +2.3 ± 0.87 | +0.9 ± 0.90 | +2.0 ± 1.09 | +2.1 ± 0.87 | +1.1 ± 1.39 |
| Seed 24, eighth, assignment | 24 | −2.0 ± 0.86 | +1.3 ± 1.23 | +2.0 ± 1.04 | +0.7 ± 1.19 | +1.2 ± 0.69 | +1.8 ± 1.72 |
| Seed 24, eighth, assignment | 36 | +0.3 ± 0.70 | −1.1 ± 1.34 | +0.6 ± 0.91 | −0.9 ± 0.70 | +3.4 ± 0.82 | +0.4 ± 2.31 |
| Seed 22, first mapping, mirrored | 6 | 0.0 ± 0.87 | +0.6 ± 0.71 | −0.1 ± 0.86 | +0.2 ± 0.71 | +0.1 ± 0.83 | +0.3 ± 0.72 |
| Seed 22, first mapping, mirrored | 12 | −0.4 ± 1.10 | +0.9 ± 1.02 | +0.3 ± 1.12 | +0.4 ± 1.10 | −0.7 ± 1.16 | +1.3 ± 1.03 |
| Seed 22, first mapping, mirrored | 18 | −0.2 ± 1.00 | +0.4 ± 1.07 | +0.5 ± 0.96 | −0.6 ± 1.06 | +0.1 ± 0.99 | +0.3 ± 1.01 |
| Seed 22, first mapping, mirrored | 24 | −0.2 ± 0.66 | +0.7 ± 1.50 | +0.9 ± 0.63 | 0.0 ± 1.52 | +0.4 ± 0.78 | +1.0 ± 1.53 |
| Seed 22, first mapping, mirrored | 36 | −0.4 ± 0.88 | +0.5 ± 1.14 | +0.8 ± 0.75 | −0.1 ± 1.13 | +0.1 ± 0.95 | +1.1 ± 1.23 |
| Seed 22, eighth, assignment | 6 | +0.4 ± 0.75 | +0.2 ± 1.25 | −0.2 ± 0.77 | −0.1 ± 1.27 | −0.3 ± 0.69 | +0.9 ± 1.21 |
| Seed 22, eighth, assignment | 12 | +0.5 ± 0.59 | +1.5 ± 1.12 | +0.7 ± 0.84 | +0.5 ± 1.15 | +0.8 ± 0.63 | +1.5 ± 1.08 |
| Seed 22, eighth, assignment | 18 | +0.5 ± 1.17 | +0.4 ± 0.63 | +1.7 ± 0.99 | −0.3 ± 0.76 | +2.0 ± 1.61 | +0.3 ± 0.73 |
| Seed 22, eighth, assignment | 24 | −1.0 ± 1.25 | +1.1 ± 1.19 | +1.2 ± 1.33 | +0.8 ± 0.85 | +1.6 ± 1.40 | +1.7 ± 1.10 |
| Seed 22, eighth, assignment | 36 | −1.2 ± 1.51 | +0.7 ± 1.36 | +1.4 ± 1.40 | +0.4 ± 1.24 | +0.5 ± 1.30 | +1.4 ± 1.14 |

| Cell | Block | Own group and inhibitory units less 027's held within: A's | B's | Own group and outside the readouts: A's | B's | Inhibitory units and outside the readouts: A's | B's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | 0.0 ± 0.88 | +0.1 ± 0.90 | −0.5 ± 0.87 | +0.9 ± 0.83 | 0.0 ± 0.87 | +0.4 ± 0.91 |
| Seed 22, eighth, mirrored | 12 | +0.1 ± 0.96 | −0.7 ± 0.94 | −0.8 ± 0.95 | +1.7 ± 0.81 | +1.0 ± 0.95 | +0.1 ± 0.84 |
| Seed 22, eighth, mirrored | 18 | +0.4 ± 1.09 | −0.8 ± 1.17 | −0.8 ± 0.97 | +0.6 ± 1.07 | +1.2 ± 1.10 | −0.7 ± 1.17 |
| Seed 22, eighth, mirrored | 24 | −0.4 ± 0.56 | −1.9 ± 1.10 | −0.9 ± 0.61 | +2.7 ± 0.98 | +2.0 ± 0.92 | −0.5 ± 1.25 |
| Seed 22, eighth, mirrored | 36 | −0.7 ± 0.92 | −0.7 ± 1.07 | −0.7 ± 1.00 | +2.5 ± 0.97 | +1.9 ± 0.92 | −0.8 ± 1.45 |
| Seed 22, first mapping, assignment | 6 | 0.0 ± 0.81 | −0.1 ± 0.67 | 0.0 ± 0.78 | +0.1 ± 0.64 | −0.2 ± 0.79 | −0.2 ± 0.55 |
| Seed 22, first mapping, assignment | 12 | 0.0 ± 0.73 | −0.4 ± 1.23 | −0.3 ± 0.70 | +1.1 ± 1.30 | −0.4 ± 0.80 | +0.3 ± 1.32 |
| Seed 22, first mapping, assignment | 18 | +0.5 ± 0.83 | −0.1 ± 0.84 | +0.1 ± 0.91 | +2.0 ± 0.78 | +0.7 ± 0.86 | +0.8 ± 0.81 |
| Seed 22, first mapping, assignment | 24 | 0.0 ± 0.87 | −0.8 ± 0.91 | 0.0 ± 0.76 | +2.8 ± 0.88 | +0.8 ± 1.07 | −1.0 ± 1.05 |
| Seed 22, first mapping, assignment | 36 | +0.5 ± 0.77 | −0.6 ± 0.82 | +1.0 ± 0.84 | +2.5 ± 1.12 | +1.0 ± 0.87 | −0.6 ± 0.82 |
| Seed 23, eighth, mirrored | 6 | −0.1 ± 1.19 | +0.6 ± 0.75 | +0.5 ± 1.19 | −0.1 ± 0.87 | +0.1 ± 1.18 | 0.0 ± 0.77 |
| Seed 23, eighth, mirrored | 12 | −0.6 ± 0.94 | +0.3 ± 0.71 | +1.9 ± 1.05 | −0.2 ± 0.65 | +0.8 ± 0.96 | −1.3 ± 0.82 |
| Seed 23, eighth, mirrored | 18 | +1.3 ± 1.11 | −0.3 ± 0.74 | +2.4 ± 1.39 | +0.7 ± 0.74 | +1.5 ± 1.13 | −0.9 ± 0.59 |
| Seed 23, eighth, mirrored | 24 | +1.1 ± 0.91 | −0.6 ± 0.46 | +1.3 ± 1.05 | +0.4 ± 0.34 | +0.7 ± 0.97 | −0.8 ± 0.32 |
| Seed 23, eighth, mirrored | 36 | −0.3 ± 1.13 | −0.2 ± 0.33 | +1.1 ± 0.99 | 0.0 ± 0.32 | +1.3 ± 1.09 | −0.3 ± 0.40 |
| Seed 23, first mapping, assignment | 6 | −0.1 ± 0.87 | +0.3 ± 1.00 | 0.0 ± 0.87 | −0.1 ± 0.93 | −0.1 ± 0.87 | +0.3 ± 1.03 |
| Seed 23, first mapping, assignment | 12 | −0.3 ± 0.70 | +0.4 ± 1.11 | −0.4 ± 0.76 | −0.6 ± 1.06 | 0.0 ± 0.80 | −1.3 ± 1.31 |
| Seed 23, first mapping, assignment | 18 | −0.1 ± 1.11 | +0.1 ± 0.74 | −0.1 ± 1.05 | +0.2 ± 0.59 | 0.0 ± 1.08 | −0.7 ± 0.88 |
| Seed 23, first mapping, assignment | 24 | −0.3 ± 0.89 | −0.3 ± 0.53 | −0.3 ± 0.84 | −0.6 ± 0.78 | +0.3 ± 0.83 | −1.7 ± 0.67 |
| Seed 23, first mapping, assignment | 36 | −0.4 ± 0.94 | −0.1 ± 0.81 | 0.0 ± 0.93 | −0.2 ± 0.72 | +0.3 ± 1.01 | −0.3 ± 0.86 |
| Seed 24, eighth, mirrored | 6 | 0.0 ± 0.98 | −0.3 ± 0.90 | −1.5 ± 0.98 | +0.5 ± 0.93 | +0.3 ± 1.06 | −0.4 ± 0.95 |
| Seed 24, eighth, mirrored | 12 | −0.8 ± 0.98 | −0.8 ± 1.09 | −2.6 ± 0.88 | −0.8 ± 1.38 | −0.4 ± 0.88 | −0.2 ± 1.11 |
| Seed 24, eighth, mirrored | 18 | −2.3 ± 1.08 | −0.3 ± 1.33 | −2.9 ± 0.87 | −0.3 ± 1.13 | −1.8 ± 0.94 | 0.0 ± 1.15 |
| Seed 24, eighth, mirrored | 24 | −1.4 ± 1.18 | −2.0 ± 1.26 | −4.9 ± 1.32 | +0.2 ± 1.35 | −1.5 ± 1.20 | +0.3 ± 0.89 |
| Seed 24, eighth, mirrored | 36 | +0.3 ± 1.21 | −0.9 ± 0.66 | −2.7 ± 1.05 | +0.4 ± 1.75 | −0.7 ± 1.25 | +1.6 ± 0.91 |
| Seed 24, first mapping, assignment | 6 | 0.0 ± 0.98 | −0.1 ± 0.90 | −0.1 ± 1.02 | −0.3 ± 0.95 | 0.0 ± 0.97 | −0.1 ± 0.90 |
| Seed 24, first mapping, assignment | 12 | −0.5 ± 0.93 | −0.7 ± 0.99 | −0.5 ± 0.90 | +0.2 ± 1.14 | −0.6 ± 0.93 | −1.2 ± 1.00 |
| Seed 24, first mapping, assignment | 18 | −0.2 ± 0.71 | −0.6 ± 1.10 | −0.1 ± 0.74 | −0.3 ± 1.46 | 0.0 ± 0.75 | 0.0 ± 1.13 |
| Seed 24, first mapping, assignment | 24 | −0.9 ± 0.92 | −0.6 ± 0.83 | −0.3 ± 0.87 | +0.6 ± 1.07 | −0.4 ± 0.89 | 0.0 ± 1.13 |
| Seed 24, first mapping, assignment | 36 | −0.3 ± 1.13 | −0.7 ± 1.14 | −1.2 ± 1.13 | +0.3 ± 1.13 | −1.0 ± 1.14 | +1.0 ± 1.09 |
| Seed 23, eighth, assignment | 6 | 0.0 ± 0.79 | +0.2 ± 1.18 | −0.1 ± 0.83 | −0.3 ± 1.17 | −0.1 ± 0.77 | +0.7 ± 1.15 |
| Seed 23, eighth, assignment | 12 | −0.6 ± 1.17 | −0.3 ± 1.31 | −1.3 ± 1.24 | −2.4 ± 1.18 | −0.3 ± 1.33 | −1.0 ± 1.25 |
| Seed 23, eighth, assignment | 18 | −1.3 ± 1.09 | −0.8 ± 0.89 | 0.0 ± 1.02 | −4.7 ± 0.88 | −0.4 ± 1.21 | −0.4 ± 0.86 |
| Seed 23, eighth, assignment | 24 | −1.4 ± 1.01 | −1.5 ± 0.91 | −0.1 ± 0.98 | −4.5 ± 1.07 | −1.0 ± 1.03 | +0.2 ± 1.05 |
| Seed 23, eighth, assignment | 36 | −1.0 ± 1.01 | −1.9 ± 1.12 | −0.3 ± 1.00 | −3.6 ± 0.94 | −1.3 ± 0.94 | +0.4 ± 1.15 |
| Seed 24, first mapping, mirrored | 6 | +0.1 ± 1.00 | +0.1 ± 0.90 | 0.0 ± 0.96 | −0.1 ± 0.92 | +0.3 ± 0.99 | +0.1 ± 0.97 |
| Seed 24, first mapping, mirrored | 12 | +0.1 ± 0.92 | 0.0 ± 1.09 | +0.7 ± 1.01 | +1.1 ± 0.98 | −0.2 ± 0.83 | −0.7 ± 0.90 |
| Seed 24, first mapping, mirrored | 18 | −0.3 ± 1.17 | −0.3 ± 0.91 | +1.0 ± 1.15 | +1.1 ± 0.76 | +0.2 ± 1.12 | −1.2 ± 0.88 |
| Seed 24, first mapping, mirrored | 24 | −0.2 ± 0.86 | −1.3 ± 1.00 | +0.4 ± 0.92 | +0.5 ± 1.25 | −0.1 ± 1.03 | −1.1 ± 1.00 |
| Seed 24, first mapping, mirrored | 36 | −0.3 ± 0.58 | −1.2 ± 1.02 | +0.5 ± 0.55 | −0.3 ± 1.13 | +0.4 ± 0.41 | −1.0 ± 0.85 |
| Seed 23, first mapping, mirrored | 6 | +0.1 ± 0.97 | −0.3 ± 0.84 | +0.1 ± 0.97 | −0.1 ± 0.84 | −0.1 ± 0.97 | +0.3 ± 0.90 |
| Seed 23, first mapping, mirrored | 12 | −0.1 ± 0.92 | 0.0 ± 1.12 | −0.3 ± 0.89 | −0.4 ± 1.15 | −0.1 ± 0.97 | +1.1 ± 1.11 |
| Seed 23, first mapping, mirrored | 18 | −0.1 ± 1.07 | +0.1 ± 1.00 | −0.3 ± 0.97 | −2.2 ± 1.11 | −0.9 ± 1.12 | +0.8 ± 0.98 |
| Seed 23, first mapping, mirrored | 24 | −0.9 ± 1.24 | +0.3 ± 0.97 | 0.0 ± 0.92 | −2.1 ± 0.81 | 0.0 ± 1.12 | +0.2 ± 1.07 |
| Seed 23, first mapping, mirrored | 36 | −1.0 ± 0.90 | +0.8 ± 0.76 | +0.2 ± 1.03 | −1.5 ± 0.84 | −0.6 ± 0.83 | +0.6 ± 0.80 |
| Seed 24, eighth, assignment | 6 | −0.6 ± 0.71 | 0.0 ± 0.88 | −0.1 ± 0.79 | +0.2 ± 0.90 | +0.1 ± 0.76 | +0.4 ± 0.94 |
| Seed 24, eighth, assignment | 12 | −0.7 ± 1.02 | +0.6 ± 0.93 | −0.5 ± 0.99 | +1.6 ± 0.95 | −0.7 ± 0.98 | +1.7 ± 1.33 |
| Seed 24, eighth, assignment | 18 | −2.3 ± 0.98 | +1.1 ± 1.00 | −1.0 ± 0.96 | +0.8 ± 1.32 | +0.2 ± 1.05 | −0.1 ± 1.17 |
| Seed 24, eighth, assignment | 24 | −2.7 ± 1.05 | −0.7 ± 0.96 | +1.2 ± 1.18 | −1.3 ± 1.40 | +0.5 ± 0.99 | −0.3 ± 1.20 |
| Seed 24, eighth, assignment | 36 | −2.5 ± 0.56 | −0.5 ± 1.42 | −2.2 ± 0.84 | −0.3 ± 1.31 | +0.5 ± 0.70 | +1.0 ± 1.30 |
| Seed 22, first mapping, mirrored | 6 | 0.0 ± 0.85 | +0.1 ± 0.70 | −0.1 ± 0.84 | −0.3 ± 0.71 | +0.1 ± 0.82 | −0.2 ± 0.73 |
| Seed 22, first mapping, mirrored | 12 | −0.1 ± 1.01 | +0.1 ± 1.09 | +0.6 ± 1.06 | −0.5 ± 1.18 | −0.4 ± 1.08 | +0.4 ± 1.11 |
| Seed 22, first mapping, mirrored | 18 | −0.6 ± 0.98 | +0.2 ± 1.10 | +0.2 ± 0.99 | −0.8 ± 1.01 | −0.2 ± 1.05 | +0.1 ± 1.08 |
| Seed 22, first mapping, mirrored | 24 | −0.8 ± 0.67 | +0.1 ± 1.59 | +0.3 ± 0.65 | −0.6 ± 1.55 | −0.2 ± 0.77 | +0.4 ± 1.41 |
| Seed 22, first mapping, mirrored | 36 | −0.7 ± 0.91 | +0.2 ± 0.94 | +0.5 ± 0.91 | −0.4 ± 0.93 | −0.2 ± 1.01 | +0.9 ± 1.04 |
| Seed 22, eighth, assignment | 6 | +0.2 ± 0.66 | −0.1 ± 1.24 | −0.3 ± 0.76 | −0.3 ± 1.27 | −0.4 ± 0.80 | +0.6 ± 1.22 |
| Seed 22, eighth, assignment | 12 | −0.4 ± 0.71 | +0.1 ± 1.04 | −0.1 ± 0.79 | −0.9 ± 1.05 | 0.0 ± 0.71 | 0.0 ± 1.00 |
| Seed 22, eighth, assignment | 18 | −0.6 ± 0.86 | +0.2 ± 0.71 | +0.6 ± 1.15 | −0.5 ± 0.80 | +0.9 ± 1.09 | +0.1 ± 0.81 |
| Seed 22, eighth, assignment | 24 | −1.4 ± 1.16 | −1.0 ± 1.22 | +0.8 ± 1.24 | −1.2 ± 0.96 | +1.2 ± 1.19 | −0.4 ± 1.09 |
| Seed 22, eighth, assignment | 36 | −1.6 ± 1.11 | −1.2 ± 1.28 | +0.9 ± 1.24 | −1.5 ± 1.40 | +0.1 ± 1.09 | −0.5 ± 0.94 |

| Cell | Block | Own group and inhibitory units less the excitatory night: A's | B's | Own group and outside the readouts: A's | B's | Inhibitory units and outside the readouts: A's | B's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +0.1 ± 0.89 | +0.2 ± 1.00 | −0.5 ± 0.91 | +1.0 ± 0.94 | +0.1 ± 0.89 | +0.5 ± 0.99 |
| Seed 22, eighth, mirrored | 12 | −1.1 ± 1.02 | +0.4 ± 1.19 | −2.0 ± 1.00 | +2.8 ± 1.02 | −0.2 ± 0.92 | +1.2 ± 1.10 |
| Seed 22, eighth, mirrored | 18 | −1.2 ± 1.22 | +0.4 ± 1.23 | −2.4 ± 1.14 | +1.8 ± 1.17 | −0.4 ± 1.30 | +0.5 ± 1.25 |
| Seed 22, eighth, mirrored | 24 | −1.9 ± 1.00 | −0.5 ± 1.39 | −2.4 ± 1.04 | +4.1 ± 1.35 | +0.5 ± 0.87 | +0.9 ± 1.18 |
| Seed 22, eighth, mirrored | 36 | −2.1 ± 1.20 | +1.2 ± 1.77 | −2.0 ± 0.82 | +4.3 ± 1.31 | +0.5 ± 0.78 | +1.0 ± 1.15 |
| Seed 22, first mapping, assignment | 6 | 0.0 ± 0.79 | −0.1 ± 0.67 | 0.0 ± 0.73 | +0.1 ± 0.65 | −0.2 ± 0.76 | −0.3 ± 0.60 |
| Seed 22, first mapping, assignment | 12 | 0.0 ± 0.87 | +1.2 ± 1.21 | −0.3 ± 0.86 | +2.6 ± 1.26 | −0.4 ± 0.89 | +1.8 ± 1.24 |
| Seed 22, first mapping, assignment | 18 | +0.7 ± 1.11 | +0.1 ± 0.76 | +0.3 ± 1.03 | +2.1 ± 0.74 | +0.9 ± 1.07 | +0.9 ± 0.74 |
| Seed 22, first mapping, assignment | 24 | +0.6 ± 0.76 | +0.3 ± 1.03 | +0.6 ± 0.74 | +3.9 ± 0.90 | +1.4 ± 0.82 | +0.1 ± 1.01 |
| Seed 22, first mapping, assignment | 36 | +0.7 ± 0.78 | +0.5 ± 0.95 | +1.2 ± 0.81 | +3.5 ± 0.82 | +1.3 ± 0.86 | +0.5 ± 0.75 |
| Seed 23, eighth, mirrored | 6 | +0.1 ± 1.14 | −0.9 ± 0.81 | +0.6 ± 1.19 | −1.6 ± 0.90 | +0.3 ± 1.19 | −1.5 ± 0.81 |
| Seed 23, eighth, mirrored | 12 | +0.8 ± 0.81 | −0.3 ± 0.87 | +3.2 ± 0.98 | −0.7 ± 0.80 | +2.1 ± 0.92 | −1.9 ± 1.07 |
| Seed 23, eighth, mirrored | 18 | +1.8 ± 1.16 | −1.4 ± 0.60 | +2.8 ± 1.24 | −0.4 ± 1.14 | +1.9 ± 1.12 | −1.9 ± 0.62 |
| Seed 23, eighth, mirrored | 24 | +2.1 ± 1.14 | −0.8 ± 0.53 | +2.2 ± 1.16 | +0.2 ± 0.49 | +1.6 ± 1.27 | −1.0 ± 0.52 |
| Seed 23, eighth, mirrored | 36 | −0.1 ± 1.30 | +0.4 ± 0.34 | +1.3 ± 1.05 | +0.6 ± 0.56 | +1.5 ± 1.22 | +0.4 ± 0.30 |
| Seed 23, first mapping, assignment | 6 | +0.1 ± 0.89 | +0.3 ± 0.98 | +0.2 ± 0.87 | −0.1 ± 0.92 | +0.2 ± 0.89 | +0.3 ± 0.99 |
| Seed 23, first mapping, assignment | 12 | +0.6 ± 0.80 | +0.9 ± 1.09 | +0.5 ± 0.89 | −0.1 ± 1.05 | +1.0 ± 1.03 | −0.7 ± 1.04 |
| Seed 23, first mapping, assignment | 18 | +0.6 ± 1.16 | −0.8 ± 0.65 | +0.5 ± 1.11 | −0.8 ± 0.69 | +0.6 ± 1.16 | −1.6 ± 0.85 |
| Seed 23, first mapping, assignment | 24 | +0.4 ± 0.91 | −0.3 ± 0.77 | +0.4 ± 0.86 | −0.6 ± 0.94 | +1.0 ± 0.86 | −1.7 ± 0.84 |
| Seed 23, first mapping, assignment | 36 | +0.7 ± 1.15 | −0.5 ± 0.58 | +1.1 ± 1.04 | −0.5 ± 0.70 | +1.3 ± 0.96 | −0.7 ± 0.66 |
| Seed 24, eighth, mirrored | 6 | −0.6 ± 1.04 | −1.1 ± 0.93 | −2.1 ± 1.02 | −0.4 ± 1.00 | −0.4 ± 1.09 | −1.2 ± 0.98 |
| Seed 24, eighth, mirrored | 12 | −0.6 ± 0.91 | −0.8 ± 1.24 | −2.5 ± 0.87 | −0.7 ± 1.23 | −0.3 ± 0.89 | −0.1 ± 1.28 |
| Seed 24, eighth, mirrored | 18 | −1.4 ± 1.16 | −0.4 ± 1.19 | −2.0 ± 1.24 | −0.4 ± 1.36 | −0.9 ± 1.30 | −0.1 ± 1.54 |
| Seed 24, eighth, mirrored | 24 | −1.4 ± 1.09 | −3.0 ± 1.33 | −5.0 ± 1.39 | −0.8 ± 1.80 | −1.5 ± 1.26 | −0.7 ± 1.27 |
| Seed 24, eighth, mirrored | 36 | −0.3 ± 1.26 | −1.4 ± 0.71 | −3.3 ± 1.03 | −0.2 ± 1.80 | −1.3 ± 1.25 | +1.0 ± 0.92 |
| Seed 24, first mapping, assignment | 6 | +0.3 ± 1.01 | +0.2 ± 0.94 | +0.2 ± 1.04 | 0.0 ± 0.94 | +0.3 ± 0.99 | +0.1 ± 0.92 |
| Seed 24, first mapping, assignment | 12 | +0.8 ± 0.96 | −0.6 ± 1.04 | +0.7 ± 0.93 | +0.3 ± 1.13 | +0.6 ± 0.94 | −1.1 ± 1.06 |
| Seed 24, first mapping, assignment | 18 | +0.8 ± 0.73 | −1.6 ± 1.09 | +0.9 ± 0.81 | −1.3 ± 1.14 | +1.0 ± 0.75 | −1.0 ± 0.98 |
| Seed 24, first mapping, assignment | 24 | +0.5 ± 0.84 | −1.0 ± 1.60 | +1.1 ± 0.86 | +0.2 ± 1.20 | +1.0 ± 0.84 | −0.4 ± 1.02 |
| Seed 24, first mapping, assignment | 36 | +1.2 ± 1.42 | −1.8 ± 1.35 | +0.3 ± 1.34 | −0.8 ± 0.83 | +0.5 ± 1.22 | −0.1 ± 0.73 |
| Seed 23, eighth, assignment | 6 | +0.2 ± 0.78 | −0.3 ± 1.17 | 0.0 ± 0.85 | −0.8 ± 1.17 | 0.0 ± 0.79 | +0.2 ± 1.14 |
| Seed 23, eighth, assignment | 12 | −0.8 ± 1.17 | −0.4 ± 1.39 | −1.4 ± 1.25 | −2.5 ± 1.23 | −0.4 ± 1.25 | −1.1 ± 1.49 |
| Seed 23, eighth, assignment | 18 | −1.7 ± 1.03 | −0.5 ± 1.06 | −0.4 ± 0.94 | −4.3 ± 1.02 | −0.8 ± 1.22 | −0.1 ± 1.30 |
| Seed 23, eighth, assignment | 24 | −1.4 ± 1.16 | −0.5 ± 1.05 | −0.1 ± 1.08 | −3.5 ± 1.09 | −0.9 ± 1.16 | +1.2 ± 1.05 |
| Seed 23, eighth, assignment | 36 | −1.7 ± 1.04 | −1.9 ± 1.41 | −1.0 ± 1.04 | −3.6 ± 1.23 | −2.0 ± 0.99 | +0.4 ± 1.51 |
| Seed 24, first mapping, mirrored | 6 | −0.4 ± 1.00 | −0.2 ± 0.90 | −0.5 ± 0.95 | −0.4 ± 0.95 | −0.2 ± 1.00 | −0.2 ± 0.96 |
| Seed 24, first mapping, mirrored | 12 | −0.1 ± 0.91 | −1.4 ± 0.98 | +0.4 ± 0.98 | −0.3 ± 1.34 | −0.4 ± 0.88 | −2.1 ± 1.00 |
| Seed 24, first mapping, mirrored | 18 | −0.7 ± 1.15 | −1.7 ± 0.88 | +0.6 ± 1.12 | −0.3 ± 1.06 | −0.1 ± 1.09 | −2.6 ± 0.90 |
| Seed 24, first mapping, mirrored | 24 | −0.3 ± 0.97 | −1.6 ± 1.15 | +0.4 ± 0.91 | +0.2 ± 1.28 | −0.2 ± 1.15 | −1.5 ± 1.04 |
| Seed 24, first mapping, mirrored | 36 | −0.5 ± 0.73 | −0.8 ± 0.82 | +0.3 ± 0.71 | +0.2 ± 0.89 | +0.3 ± 0.60 | −0.6 ± 0.83 |
| Seed 23, first mapping, mirrored | 6 | 0.0 ± 0.94 | −0.1 ± 0.92 | 0.0 ± 0.94 | 0.0 ± 0.89 | −0.2 ± 0.93 | +0.4 ± 0.98 |
| Seed 23, first mapping, mirrored | 12 | −0.6 ± 0.96 | −0.3 ± 1.27 | −0.8 ± 0.94 | −0.6 ± 1.14 | −0.6 ± 1.01 | +0.9 ± 1.25 |
| Seed 23, first mapping, mirrored | 18 | −0.3 ± 1.03 | −0.2 ± 1.06 | −0.4 ± 0.90 | −2.5 ± 1.14 | −1.0 ± 0.94 | +0.5 ± 1.12 |
| Seed 23, first mapping, mirrored | 24 | −1.4 ± 0.99 | +0.1 ± 1.07 | −0.5 ± 1.02 | −2.4 ± 0.88 | −0.5 ± 1.00 | 0.0 ± 1.05 |
| Seed 23, first mapping, mirrored | 36 | −1.6 ± 1.01 | +0.4 ± 0.89 | −0.4 ± 1.27 | −1.9 ± 0.88 | −1.2 ± 0.87 | +0.2 ± 0.85 |
| Seed 24, eighth, assignment | 6 | −0.8 ± 0.79 | −0.7 ± 0.97 | −0.3 ± 0.90 | −0.6 ± 0.98 | −0.1 ± 0.88 | −0.4 ± 1.01 |
| Seed 24, eighth, assignment | 12 | −0.9 ± 1.04 | −1.5 ± 0.98 | −0.7 ± 1.20 | −0.6 ± 1.21 | −0.9 ± 1.15 | −0.5 ± 1.07 |
| Seed 24, eighth, assignment | 18 | −4.2 ± 0.94 | −1.0 ± 0.81 | −2.9 ± 1.01 | −1.4 ± 0.82 | −1.7 ± 1.36 | −2.2 ± 1.57 |
| Seed 24, eighth, assignment | 24 | −4.2 ± 1.27 | −3.5 ± 0.98 | −0.3 ± 1.28 | −4.1 ± 1.07 | −1.0 ± 1.31 | −3.0 ± 1.66 |
| Seed 24, eighth, assignment | 36 | −4.0 ± 0.87 | −3.8 ± 1.39 | −3.7 ± 1.09 | −3.5 ± 1.01 | −0.9 ± 0.89 | −2.2 ± 1.83 |
| Seed 22, first mapping, mirrored | 6 | −0.1 ± 0.86 | +0.3 ± 0.73 | −0.2 ± 0.86 | −0.1 ± 0.75 | 0.0 ± 0.84 | 0.0 ± 0.77 |
| Seed 22, first mapping, mirrored | 12 | 0.0 ± 1.10 | +0.2 ± 1.18 | +0.7 ± 1.13 | −0.3 ± 1.20 | −0.3 ± 1.16 | +0.5 ± 1.24 |
| Seed 22, first mapping, mirrored | 18 | −0.3 ± 0.85 | +0.7 ± 1.13 | +0.5 ± 0.92 | −0.3 ± 1.13 | +0.1 ± 0.96 | +0.7 ± 1.07 |
| Seed 22, first mapping, mirrored | 24 | −0.1 ± 0.69 | +0.5 ± 1.51 | +1.0 ± 0.67 | −0.2 ± 1.63 | +0.4 ± 0.74 | +0.8 ± 1.36 |
| Seed 22, first mapping, mirrored | 36 | +0.1 ± 1.06 | +0.3 ± 0.92 | +1.3 ± 0.93 | −0.3 ± 0.96 | +0.6 ± 1.01 | +1.0 ± 0.83 |
| Seed 22, eighth, assignment | 6 | +0.3 ± 0.69 | −0.1 ± 1.26 | −0.3 ± 0.75 | −0.3 ± 1.27 | −0.4 ± 0.80 | +0.6 ± 1.23 |
| Seed 22, eighth, assignment | 12 | −0.5 ± 0.71 | +0.3 ± 1.07 | −0.3 ± 0.85 | −0.7 ± 1.11 | −0.2 ± 0.74 | +0.3 ± 1.02 |
| Seed 22, eighth, assignment | 18 | −0.8 ± 0.89 | +0.4 ± 0.69 | +0.5 ± 1.23 | −0.3 ± 0.77 | +0.8 ± 0.95 | +0.3 ± 0.78 |
| Seed 22, eighth, assignment | 24 | −1.2 ± 1.12 | −0.7 ± 0.94 | +1.0 ± 1.16 | −1.0 ± 1.06 | +1.4 ± 1.14 | −0.1 ± 0.86 |
| Seed 22, eighth, assignment | 36 | −2.0 ± 1.23 | −1.0 ± 1.26 | +0.6 ± 1.20 | −1.3 ± 1.22 | −0.3 ± 1.19 | −0.3 ± 0.94 |

Each stimulus's trials less exploration 028's night for each of the form's two parts held alone, with exploration 018's unpaired SE:

| Cell | Block | Own group and inhibitory units less own group alone (028): A's | B's | less inhibitory units alone (028): A's | B's | Own group and outside the readouts less own group alone (028): A's | B's | less outside the readouts alone (028): A's | B's | Inhibitory units and outside the readouts less inhibitory units alone (028): A's | B's | less outside the readouts alone (028): A's | B's |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +0.5 ± 0.90 | −0.9 ± 0.93 | 0.0 ± 0.87 | −0.1 ± 0.95 | −0.1 ± 0.85 | −0.1 ± 0.90 | 0.0 ± 0.85 | −0.4 ± 0.93 | 0.0 ± 0.87 | +0.2 ± 0.96 | +0.6 ± 0.89 | −0.8 ± 0.97 |
| Seed 22, eighth, mirrored | 12 | +1.1 ± 1.03 | −1.9 ± 0.89 | −0.6 ± 0.95 | +0.2 ± 1.05 | +0.3 ± 1.01 | +0.6 ± 0.90 | −0.9 ± 0.89 | 0.0 ± 0.86 | +0.3 ± 1.00 | +1.0 ± 0.93 | +0.9 ± 0.98 | −1.6 ± 0.83 |
| Seed 22, eighth, mirrored | 18 | +1.0 ± 0.88 | −1.1 ± 1.19 | −0.8 ± 1.13 | +0.4 ± 1.22 | −0.2 ± 0.78 | +0.3 ± 0.97 | −1.3 ± 1.13 | −0.1 ± 0.95 | 0.0 ± 1.08 | +0.5 ± 1.13 | +0.6 ± 1.10 | −1.4 ± 1.38 |
| Seed 22, eighth, mirrored | 24 | +1.0 ± 0.67 | −3.8 ± 1.08 | −0.9 ± 1.09 | +0.3 ± 1.20 | +0.5 ± 0.77 | +0.8 ± 0.97 | −2.2 ± 0.79 | −1.0 ± 1.01 | +1.5 ± 0.79 | +1.7 ± 1.07 | +0.7 ± 0.86 | −4.2 ± 1.12 |
| Seed 22, eighth, mirrored | 36 | +0.8 ± 1.02 | −2.3 ± 0.91 | −1.2 ± 1.06 | +0.5 ± 1.82 | +0.9 ± 0.82 | +0.9 ± 0.85 | −1.2 ± 0.85 | −1.0 ± 0.92 | +1.4 ± 0.72 | +0.3 ± 1.01 | +1.4 ± 0.85 | −4.3 ± 0.84 |
| Seed 22, first mapping, assignment | 6 | +0.1 ± 0.84 | −0.3 ± 0.65 | +0.1 ± 0.79 | +0.1 ± 0.59 | +0.1 ± 0.77 | 0.0 ± 0.66 | +0.1 ± 0.76 | +0.2 ± 0.60 | −0.1 ± 0.76 | 0.0 ± 0.58 | −0.1 ± 0.79 | −0.1 ± 0.67 |
| Seed 22, first mapping, assignment | 12 | +0.4 ± 0.69 | −1.1 ± 1.35 | +0.4 ± 0.60 | −0.3 ± 1.23 | +0.1 ± 0.78 | +0.3 ± 1.39 | +0.5 ± 0.75 | −0.7 ± 1.40 | −0.1 ± 0.69 | +0.4 ± 1.19 | +0.4 ± 0.85 | −1.5 ± 1.32 |
| Seed 22, first mapping, assignment | 18 | −0.1 ± 0.91 | −1.0 ± 1.44 | +0.4 ± 0.92 | −0.2 ± 0.96 | −0.5 ± 1.02 | +1.0 ± 1.19 | +0.1 ± 0.84 | +0.4 ± 0.89 | +0.6 ± 0.94 | +0.6 ± 0.93 | +0.8 ± 0.89 | −0.8 ± 1.10 |
| Seed 22, first mapping, assignment | 24 | −0.8 ± 0.71 | −2.5 ± 0.95 | +0.1 ± 0.69 | +0.6 ± 1.21 | −0.8 ± 0.72 | +1.0 ± 0.77 | +0.1 ± 0.87 | +1.2 ± 1.13 | +0.9 ± 0.87 | +0.4 ± 1.14 | +0.9 ± 0.63 | −2.6 ± 1.24 |
| Seed 22, first mapping, assignment | 36 | −1.0 ± 1.02 | −2.9 ± 1.08 | −0.3 ± 0.76 | +0.6 ± 0.79 | −0.5 ± 0.83 | +0.2 ± 0.76 | +0.9 ± 0.96 | +0.8 ± 0.76 | +0.3 ± 0.77 | +0.6 ± 0.75 | +1.0 ± 0.88 | −2.3 ± 0.93 |
| Seed 23, eighth, mirrored | 6 | −0.6 ± 1.15 | 0.0 ± 0.85 | −0.1 ± 1.21 | 0.0 ± 0.79 | 0.0 ± 1.23 | −0.8 ± 0.88 | +0.2 ± 1.24 | −0.2 ± 0.90 | +0.1 ± 1.16 | −0.6 ± 0.75 | −0.1 ± 1.25 | −0.1 ± 0.81 |
| Seed 23, eighth, mirrored | 12 | −2.0 ± 0.83 | +0.4 ± 0.87 | −0.7 ± 0.57 | +0.6 ± 0.73 | +0.5 ± 1.02 | 0.0 ± 0.81 | +0.2 ± 1.00 | +0.2 ± 0.68 | +0.7 ± 0.79 | −1.0 ± 0.78 | −1.0 ± 0.94 | −1.0 ± 0.82 |
| Seed 23, eighth, mirrored | 18 | −1.6 ± 1.11 | −0.5 ± 0.61 | −0.5 ± 1.01 | +0.3 ± 0.70 | −0.5 ± 1.48 | +0.5 ± 1.33 | −1.0 ± 1.06 | +0.5 ± 1.19 | −0.3 ± 0.99 | −0.3 ± 0.60 | −1.9 ± 1.06 | −1.0 ± 0.57 |
| Seed 23, eighth, mirrored | 24 | −0.8 ± 1.05 | −0.8 ± 0.41 | −0.1 ± 1.01 | +0.8 ± 0.33 | −0.6 ± 1.46 | +0.2 ± 0.63 | −0.9 ± 1.39 | 0.0 ± 0.52 | −0.6 ± 1.09 | +0.6 ± 0.30 | −1.4 ± 0.92 | −1.2 ± 0.57 |
| Seed 23, eighth, mirrored | 36 | −1.4 ± 1.16 | −0.6 ± 0.70 | −1.1 ± 1.12 | +0.8 ± 0.36 | 0.0 ± 1.27 | −0.4 ± 0.74 | −0.5 ± 1.35 | +0.2 ± 0.30 | +0.5 ± 1.31 | +0.8 ± 0.21 | −0.4 ± 1.44 | −0.1 ± 0.45 |
| Seed 23, first mapping, assignment | 6 | 0.0 ± 0.85 | 0.0 ± 1.03 | −0.1 ± 0.87 | +0.2 ± 1.11 | +0.1 ± 0.85 | −0.3 ± 0.97 | +0.1 ± 0.86 | 0.0 ± 0.91 | 0.0 ± 0.87 | +0.2 ± 1.12 | 0.0 ± 0.86 | +0.4 ± 1.01 |
| Seed 23, first mapping, assignment | 12 | +0.2 ± 0.74 | +0.8 ± 1.13 | −0.1 ± 0.79 | +1.6 ± 1.14 | +0.1 ± 0.78 | −0.2 ± 1.07 | −0.3 ± 0.77 | +0.3 ± 1.17 | +0.3 ± 0.85 | −0.1 ± 1.33 | +0.1 ± 0.81 | −0.4 ± 1.15 |
| Seed 23, first mapping, assignment | 18 | +0.2 ± 1.10 | +0.3 ± 0.58 | −0.5 ± 1.17 | +1.0 ± 0.68 | +0.1 ± 1.02 | +0.3 ± 0.72 | +0.1 ± 1.02 | +0.6 ± 0.78 | −0.5 ± 1.13 | +0.1 ± 0.77 | +0.2 ± 1.05 | −0.3 ± 0.77 |
| Seed 23, first mapping, assignment | 24 | −0.1 ± 0.94 | +1.1 ± 0.49 | −0.7 ± 1.08 | +1.2 ± 0.57 | −0.1 ± 0.89 | +0.8 ± 0.70 | −0.2 ± 0.90 | +1.0 ± 0.67 | −0.1 ± 0.82 | −0.2 ± 0.60 | +0.4 ± 0.83 | −0.1 ± 0.54 |
| Seed 23, first mapping, assignment | 36 | −0.1 ± 0.96 | +0.2 ± 0.38 | −0.1 ± 1.17 | +0.2 ± 0.63 | +0.3 ± 0.94 | +0.2 ± 0.83 | 0.0 ± 1.02 | +0.2 ± 0.63 | +0.5 ± 0.88 | 0.0 ± 0.72 | +0.2 ± 0.90 | 0.0 ± 0.39 |
| Seed 24, eighth, mirrored | 6 | +1.8 ± 0.90 | −0.2 ± 0.96 | −0.3 ± 0.96 | +0.3 ± 0.99 | +0.3 ± 0.90 | +0.5 ± 0.98 | −0.4 ± 1.01 | +0.5 ± 0.97 | 0.0 ± 1.03 | +0.2 ± 1.05 | +1.4 ± 1.07 | −0.3 ± 0.94 |
| Seed 24, eighth, mirrored | 12 | +3.3 ± 0.96 | −0.1 ± 1.04 | +1.0 ± 0.86 | −0.1 ± 1.08 | +1.4 ± 0.87 | 0.0 ± 1.28 | +0.6 ± 0.77 | +0.1 ± 1.46 | +1.4 ± 0.90 | +0.5 ± 1.10 | +2.8 ± 0.87 | +0.8 ± 1.28 |
| Seed 24, eighth, mirrored | 18 | +3.1 ± 1.30 | −0.6 ± 1.32 | +1.0 ± 1.17 | +0.9 ± 1.41 | +2.5 ± 0.91 | −0.6 ± 1.43 | +2.5 ± 1.15 | +0.1 ± 1.24 | +1.5 ± 0.92 | +1.1 ± 1.25 | +3.6 ± 1.17 | +0.4 ± 1.47 |
| Seed 24, eighth, mirrored | 24 | +5.7 ± 1.46 | −1.3 ± 1.06 | +2.7 ± 1.39 | −2.0 ± 0.94 | +2.2 ± 1.28 | +0.9 ± 1.59 | +1.2 ± 1.29 | 0.0 ± 1.24 | +2.6 ± 1.37 | +0.3 ± 0.74 | +4.6 ± 1.47 | +0.1 ± 1.29 |
| Seed 24, eighth, mirrored | 36 | +4.1 ± 0.91 | −0.7 ± 0.96 | +3.4 ± 1.12 | −1.3 ± 0.61 | +1.2 ± 1.18 | +0.5 ± 1.06 | +2.6 ± 0.88 | −1.3 ± 0.89 | +2.4 ± 1.14 | +1.1 ± 0.67 | +4.6 ± 1.00 | −0.1 ± 0.96 |
| Seed 24, first mapping, assignment | 6 | +0.1 ± 1.03 | +0.1 ± 0.96 | −0.1 ± 0.98 | 0.0 ± 0.90 | +0.1 ± 1.05 | −0.1 ± 0.96 | 0.0 ± 1.03 | −0.2 ± 0.94 | −0.1 ± 0.97 | 0.0 ± 0.89 | +0.1 ± 0.99 | 0.0 ± 0.91 |
| Seed 24, first mapping, assignment | 12 | +0.7 ± 0.98 | −0.7 ± 1.05 | +0.4 ± 0.95 | +0.6 ± 1.03 | +0.6 ± 0.90 | +0.3 ± 1.12 | +0.8 ± 0.88 | +1.0 ± 1.32 | +0.3 ± 0.88 | +0.1 ± 1.01 | +0.8 ± 0.89 | −0.4 ± 1.04 |
| Seed 24, first mapping, assignment | 18 | +0.4 ± 0.77 | −1.1 ± 1.37 | +0.1 ± 0.73 | −0.4 ± 1.05 | +0.4 ± 0.81 | −0.8 ± 1.10 | +0.6 ± 0.80 | 0.0 ± 1.69 | +0.3 ± 0.77 | +0.3 ± 1.16 | +0.7 ± 0.85 | +0.3 ± 1.56 |
| Seed 24, first mapping, assignment | 24 | +0.4 ± 0.83 | −1.3 ± 1.46 | −0.1 ± 0.91 | −0.7 ± 1.02 | +1.0 ± 0.82 | −0.1 ± 0.92 | +0.7 ± 0.91 | +0.1 ± 1.34 | +0.4 ± 0.87 | −0.1 ± 0.75 | +0.6 ± 0.90 | −0.5 ± 1.13 |
| Seed 24, first mapping, assignment | 36 | +1.1 ± 0.90 | −0.4 ± 1.19 | +1.1 ± 1.04 | −2.5 ± 1.33 | +0.1 ± 0.89 | +0.6 ± 0.92 | +1.1 ± 1.26 | −0.8 ± 0.85 | +0.4 ± 1.05 | −0.8 ± 0.86 | +1.4 ± 1.34 | −0.1 ± 0.75 |
| Seed 23, eighth, assignment | 6 | +0.1 ± 0.87 | +0.4 ± 1.21 | +0.4 ± 0.82 | −0.4 ± 1.19 | −0.1 ± 0.86 | −0.1 ± 1.25 | +0.4 ± 0.87 | −0.6 ± 1.20 | +0.3 ± 0.81 | +0.1 ± 1.16 | +0.4 ± 0.82 | +0.4 ± 1.14 |
| Seed 23, eighth, assignment | 12 | +0.3 ± 1.33 | +1.9 ± 1.35 | +0.5 ± 1.19 | −0.1 ± 1.33 | −0.4 ± 1.41 | −0.2 ± 1.17 | +0.4 ± 1.24 | −0.3 ± 1.11 | +0.8 ± 1.28 | −0.7 ± 1.14 | +1.4 ± 1.22 | +1.1 ± 1.21 |
| Seed 23, eighth, assignment | 18 | +0.1 ± 1.09 | +3.1 ± 1.13 | +1.1 ± 0.99 | −1.1 ± 0.97 | +1.4 ± 0.90 | −0.7 ± 1.22 | +1.0 ± 0.81 | −4.2 ± 0.92 | +2.0 ± 1.07 | −0.7 ± 0.89 | +0.6 ± 1.00 | 0.0 ± 0.92 |
| Seed 23, eighth, assignment | 24 | −0.1 ± 1.39 | +4.4 ± 1.12 | +2.5 ± 1.04 | −0.9 ± 1.04 | +1.2 ± 1.23 | +1.4 ± 1.46 | +3.0 ± 1.00 | −2.8 ± 1.13 | +2.9 ± 1.03 | +0.8 ± 1.13 | +2.1 ± 1.08 | +1.9 ± 1.12 |
| Seed 23, eighth, assignment | 36 | +0.6 ± 1.22 | +2.4 ± 1.60 | +2.3 ± 1.16 | −1.7 ± 1.16 | +1.3 ± 0.91 | +0.6 ± 1.35 | +2.6 ± 0.77 | −2.7 ± 0.94 | +2.0 ± 1.15 | +0.6 ± 1.11 | +1.5 ± 0.83 | +1.3 ± 1.18 |
| Seed 24, first mapping, mirrored | 6 | +0.2 ± 1.01 | +0.1 ± 0.86 | −0.1 ± 1.03 | +0.1 ± 0.90 | 0.0 ± 0.98 | −0.1 ± 0.90 | −0.5 ± 0.99 | 0.0 ± 0.98 | 0.0 ± 1.02 | +0.1 ± 0.96 | −0.1 ± 1.02 | +0.1 ± 0.99 |
| Seed 24, first mapping, mirrored | 12 | −0.2 ± 1.00 | −0.3 ± 1.12 | +0.3 ± 0.84 | +0.4 ± 0.98 | +0.4 ± 1.06 | +0.8 ± 0.97 | +0.3 ± 1.00 | +1.5 ± 0.99 | 0.0 ± 0.78 | −0.2 ± 0.97 | −0.6 ± 0.86 | −0.3 ± 0.99 |
| Seed 24, first mapping, mirrored | 18 | −0.6 ± 1.25 | −0.7 ± 0.86 | −0.3 ± 1.17 | +0.3 ± 0.88 | +0.7 ± 1.19 | +0.7 ± 0.89 | +0.3 ± 1.18 | +1.3 ± 0.92 | +0.2 ± 1.12 | −0.6 ± 0.80 | −0.5 ± 1.12 | −1.0 ± 0.86 |
| Seed 24, first mapping, mirrored | 24 | 0.0 ± 0.89 | −1.3 ± 1.17 | +0.3 ± 0.94 | −0.7 ± 1.08 | +0.6 ± 0.86 | +0.5 ± 1.14 | +0.2 ± 0.91 | +1.4 ± 1.14 | +0.4 ± 1.06 | −0.5 ± 1.01 | −0.3 ± 1.12 | −0.3 ± 1.20 |
| Seed 24, first mapping, mirrored | 36 | +0.1 ± 0.65 | −0.9 ± 0.63 | −0.1 ± 0.70 | −0.5 ± 0.81 | +0.9 ± 0.63 | 0.0 ± 0.67 | −0.3 ± 0.65 | +1.2 ± 0.79 | +0.7 ± 0.55 | −0.3 ± 0.92 | −0.3 ± 0.50 | +0.4 ± 0.93 |
| Seed 23, first mapping, mirrored | 6 | 0.0 ± 0.98 | −0.1 ± 0.87 | +0.1 ± 0.98 | −0.6 ± 0.95 | 0.0 ± 0.97 | +0.1 ± 0.86 | 0.0 ± 0.97 | −0.2 ± 0.87 | −0.1 ± 0.96 | 0.0 ± 1.00 | −0.2 ± 0.96 | +0.1 ± 0.95 |
| Seed 23, first mapping, mirrored | 12 | +0.1 ± 0.93 | +1.1 ± 1.54 | −0.1 ± 0.99 | −0.9 ± 1.28 | −0.1 ± 0.90 | +0.8 ± 1.20 | −0.1 ± 0.92 | −1.0 ± 1.08 | −0.2 ± 1.07 | +0.3 ± 1.18 | +0.1 ± 0.99 | +0.5 ± 1.14 |
| Seed 23, first mapping, mirrored | 18 | 0.0 ± 1.12 | +2.3 ± 1.06 | +0.6 ± 1.21 | −1.4 ± 0.99 | −0.2 ± 1.02 | 0.0 ± 1.11 | +0.4 ± 1.10 | −1.2 ± 1.04 | −0.1 ± 1.12 | −0.7 ± 1.11 | −0.2 ± 1.07 | +1.7 ± 1.06 |
| Seed 23, first mapping, mirrored | 24 | 0.0 ± 0.78 | +2.2 ± 1.23 | +0.5 ± 1.02 | −1.0 ± 1.00 | +1.0 ± 0.86 | −0.3 ± 1.11 | +0.7 ± 1.05 | −1.4 ± 1.04 | +1.5 ± 1.17 | −1.1 ± 1.21 | +0.7 ± 1.07 | +1.0 ± 1.34 |
| Seed 23, first mapping, mirrored | 36 | −0.9 ± 0.87 | +2.6 ± 0.95 | +0.2 ± 1.15 | −0.1 ± 0.81 | +0.3 ± 0.95 | +0.3 ± 0.77 | +1.1 ± 1.01 | −0.4 ± 1.11 | +0.6 ± 1.10 | −0.3 ± 0.82 | +0.3 ± 0.97 | +1.7 ± 1.10 |
| Seed 24, eighth, assignment | 6 | −0.2 ± 0.73 | +0.4 ± 0.86 | −0.4 ± 0.72 | −0.6 ± 0.91 | +0.2 ± 0.80 | +0.6 ± 0.87 | −0.4 ± 0.83 | 0.0 ± 0.99 | +0.2 ± 0.71 | −0.3 ± 0.97 | −0.2 ± 0.79 | +0.1 ± 1.01 |
| Seed 24, eighth, assignment | 12 | +0.8 ± 1.13 | +0.2 ± 0.94 | +0.4 ± 1.02 | −0.1 ± 1.09 | +1.0 ± 1.11 | +1.2 ± 1.07 | +0.8 ± 1.03 | −0.3 ± 1.03 | +0.5 ± 0.98 | +1.0 ± 0.97 | +0.6 ± 0.99 | −0.2 ± 1.21 |
| Seed 24, eighth, assignment | 18 | +0.3 ± 0.77 | +0.2 ± 1.05 | −0.4 ± 0.82 | +0.8 ± 1.00 | +1.6 ± 0.78 | −0.2 ± 0.88 | −0.8 ± 0.94 | +0.4 ± 1.07 | +2.1 ± 1.15 | −0.4 ± 1.05 | +0.4 ± 0.86 | −0.4 ± 1.55 |
| Seed 24, eighth, assignment | 24 | −1.2 ± 1.33 | +0.5 ± 0.74 | −0.4 ± 1.03 | −0.3 ± 0.85 | +2.7 ± 1.35 | −0.1 ± 1.13 | +0.3 ± 1.12 | +0.3 ± 1.59 | +2.7 ± 1.00 | +0.2 ± 1.08 | −0.5 ± 0.95 | +1.4 ± 1.47 |
| Seed 24, eighth, assignment | 36 | +0.3 ± 0.63 | 0.0 ± 1.41 | +0.1 ± 0.78 | −0.4 ± 1.27 | +0.6 ± 0.90 | +0.3 ± 0.94 | −1.1 ± 1.00 | −0.8 ± 1.32 | +3.1 ± 0.79 | +1.1 ± 1.40 | +1.7 ± 0.97 | +0.4 ± 1.56 |
| Seed 22, first mapping, mirrored | 6 | 0.0 ± 0.88 | +0.4 ± 0.68 | 0.0 ± 0.87 | +0.2 ± 0.69 | −0.1 ± 0.87 | 0.0 ± 0.70 | 0.0 ± 0.85 | +0.5 ± 0.74 | +0.1 ± 0.83 | −0.1 ± 0.73 | +0.2 ± 0.83 | +0.6 ± 0.75 |
| Seed 22, first mapping, mirrored | 12 | −0.6 ± 1.08 | +1.0 ± 1.15 | 0.0 ± 1.10 | −0.1 ± 0.99 | +0.1 ± 1.09 | +0.4 ± 1.21 | −0.3 ± 1.19 | −0.5 ± 1.18 | −0.3 ± 1.15 | +0.2 ± 0.98 | −1.2 ± 1.29 | +0.4 ± 1.05 |
| Seed 22, first mapping, mirrored | 18 | −0.5 ± 0.95 | +0.8 ± 1.08 | −0.3 ± 1.02 | −0.2 ± 1.10 | +0.3 ± 0.93 | −0.3 ± 1.03 | −0.4 ± 1.07 | −1.1 ± 1.12 | +0.1 ± 1.00 | −0.3 ± 1.02 | −0.8 ± 1.21 | −0.1 ± 1.05 |
| Seed 22, first mapping, mirrored | 24 | −0.7 ± 0.69 | +0.5 ± 1.66 | 0.0 ± 0.74 | +0.3 ± 1.54 | +0.4 ± 0.60 | −0.2 ± 1.59 | +0.1 ± 0.63 | −0.9 ± 1.56 | +0.6 ± 0.80 | +0.6 ± 1.41 | −0.4 ± 0.72 | +0.1 ± 1.44 |
| Seed 22, first mapping, mirrored | 36 | −0.7 ± 1.04 | +0.4 ± 0.87 | +0.1 ± 0.86 | +0.2 ± 1.13 | +0.5 ± 0.83 | −0.2 ± 0.83 | +0.1 ± 0.82 | −0.4 ± 1.02 | +0.5 ± 0.87 | +0.9 ± 0.99 | −0.7 ± 0.96 | +0.9 ± 1.03 |
| Seed 22, eighth, assignment | 6 | +0.4 ± 0.66 | +0.2 ± 1.26 | +0.6 ± 0.74 | −0.2 ± 1.23 | −0.2 ± 0.72 | 0.0 ± 1.27 | −0.1 ± 0.84 | −0.6 ± 1.25 | −0.1 ± 0.71 | +0.5 ± 1.19 | −0.2 ± 0.84 | +0.4 ± 1.19 |
| Seed 22, eighth, assignment | 12 | +0.2 ± 0.74 | +1.4 ± 1.10 | +0.7 ± 0.63 | +0.4 ± 1.05 | +0.4 ± 0.76 | +0.3 ± 1.09 | +0.4 ± 0.68 | 0.0 ± 1.12 | +1.0 ± 0.59 | +0.3 ± 0.96 | +0.5 ± 0.70 | +1.0 ± 1.03 |
| Seed 22, eighth, assignment | 18 | +0.5 ± 1.00 | +0.6 ± 0.72 | −0.1 ± 0.86 | −0.1 ± 0.67 | +1.7 ± 0.86 | 0.0 ± 0.80 | 0.0 ± 1.00 | −0.2 ± 0.87 | +1.4 ± 1.04 | −0.1 ± 0.78 | +0.3 ± 0.97 | +0.4 ± 0.85 |
| Seed 22, eighth, assignment | 24 | −0.8 ± 1.06 | +1.3 ± 1.08 | −0.3 ± 1.27 | −0.4 ± 1.09 | +1.4 ± 0.98 | +1.0 ± 0.89 | +0.4 ± 1.07 | +0.2 ± 1.06 | +2.3 ± 1.27 | +0.3 ± 0.99 | +0.8 ± 1.17 | +1.1 ± 0.96 |
| Seed 22, eighth, assignment | 36 | −1.1 ± 1.57 | +1.3 ± 1.53 | −1.8 ± 1.20 | +0.2 ± 1.39 | +1.5 ± 1.31 | +1.0 ± 1.30 | +0.3 ± 1.22 | −0.5 ± 1.36 | −0.1 ± 1.16 | +0.9 ± 1.10 | −0.5 ± 1.57 | +0.5 ± 1.14 |

- **Less H-47's plain night**, stimulus A's trials lie beyond twice the unpaired SE at 11 of the 60 checkpoints under the night held onto
  the own group and the inhibitory units, 7 above, at 9 under the night held onto the own group and the units outside the readouts, 7 above,
  and at 11 under the night held onto the inhibitory units and the units outside the readouts, 9 above; stimulus B's at 6, 2 and 2, of them
  1, 0 and 0 above.
- **Less exploration 027's night held within.** Under the night held onto the inhibitory units and the units outside the readouts A's trials
  lie between −1.8 and +2.0, beyond 2 SE at 2 of the 60, both above, on seed 22's eighth image from the mirrored assignment at blocks 24 and
  36, +2.0 ± 0.92 and +1.9 ± 0.92; B's between −1.7 and +1.7, beyond 2 SE at 2, both below, on seed 23's eighth image from the mirrored
  assignment at block 24, −0.8 ± 0.32, and on seed 23's first mapping from the assignment at block 24, −1.7 ± 0.67. Under the night held
  onto the own group and the inhibitory units A's lie between −2.7 and +1.3, beyond 2 SE at 4, all below, on seed 24's eighth image from the
  mirrored assignment at block 18, −2.3 ± 1.08, and on seed 24's eighth image from the assignment from block 18 on, −2.3 ± 0.98 to
  −2.7 ± 1.05; B's between −2.0 and +1.1, within 2 SE at every one of the 60. Under the night held onto the own group and the units outside
  the readouts A's lie between −4.9 and +2.4, beyond 2 SE at 5, all below, on seed 24's eighth image from the mirrored assignment from block
  12 on, −2.6 ± 0.88 to −4.9 ± 1.32, and on seed 24's eighth image from the assignment at block 36, −2.2 ± 0.84; B's between −4.7 and +2.8,
  beyond 2 SE at 10: above on seed 22's eighth image from the mirrored assignment at blocks 12, 24 and 36, +1.7 ± 0.81 to +2.7 ± 0.98, and
  on seed 22's first mapping from the assignment from block 18 on, +2.0 ± 0.78 to +2.8 ± 0.88; below on seed 23's eighth image from the
  assignment from block 18 on, −3.6 ± 0.94 to −4.7 ± 0.88, and on seed 23's first mapping from the mirrored assignment at block 24,
  −2.1 ± 0.81.
- **Less the excitatory night.** Under the night held onto the inhibitory units and the units outside the readouts A's trials lie between
  −2.0 and +2.1, beyond 2 SE at 2 of the 60, above on seed 23's eighth image from the mirrored assignment at block 12, +2.1 ± 0.92, and
  below on seed 23's eighth image from the assignment at block 36, −2.0 ± 0.99; B's between −3.0 and +1.8, beyond 2 SE at 4, all below, on
  seed 23's eighth image from the mirrored assignment at block 18, −1.9 ± 0.62, on seed 23's first mapping from the assignment at block 24,
  −1.7 ± 0.84, and on seed 24's first mapping from the mirrored assignment at blocks 12 and 18, −2.1 ± 1.00 and −2.6 ± 0.90. Under the night
  held onto the own group and the inhibitory units A's lie between −4.2 and +2.1, beyond 2 SE at 3, all below, on seed 24's eighth image
  from the assignment from block 18 on, −4.0 ± 0.87 to −4.2 ± 0.94; B's between −3.8 and +1.2, beyond 2 SE at 5, all below, on seed 23's
  eighth image from the mirrored assignment at block 18, −1.4 ± 0.60, on seed 24's eighth image from the mirrored assignment at blocks 24
  and 36, −3.0 ± 1.33 and −1.4 ± 0.71, and on seed 24's eighth image from the assignment at blocks 24 and 36, −3.5 ± 0.98 and −3.8 ± 1.39.
  Under the night held onto the own group and the units outside the readouts A's lie between −5.0 and +3.2, beyond 2 SE at 11, 2 above and
  9 below, and B's between −4.3 and +4.3, beyond 2 SE at 16, 7 above and 9 below.
- **Less exploration 028's nights**, under the night held onto the own group and the inhibitory units A's trials lie beyond 2 SE of the
  night with the own group held alone at 5 of the 60, 4 above, and B's at 10, 5 above; of the night with the inhibitory units held alone,
  A's at 2, both above, and B's at 5, 3 above. Under the night held onto the own group and the units outside the readouts A's lie beyond
  2 SE of the night with the own group held alone at 4, all above, and B's at none; of the night with the units outside the readouts held
  alone, A's at 5, 4 above, and B's at 3, all below. Under the night held onto the inhibitory units and the units outside the readouts A's
  lie beyond 2 SE of the night with the inhibitory units held alone at 4, all above, and B's at 2, both above; of the night with the units
  outside the readouts held alone, A's at 4, all above, and B's at 5, all below.

## 3. The pair and A's couplings

The pair's count under each held form at each checkpoint, and at block 36 under every form. H-47's floor for clause 2 is 56:

| Cell | Own group and inhibitory units at 6 | 12 | 18 | 24 | 36 | Own group and outside the readouts at 6 | 12 | 18 | 24 | 36 | Inhibitory units and outside the readouts at 6 | 12 | 18 | 24 | 36 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 45.2 | 63.2 | 62.5 | 62.8 | 62.8 | 45.7 | 62.8 | 62.7 | 62.9 | 63.0 | 44.7 | 62.7 | 62.6 | 62.7 | 63.0 |
| Seed 22, first mapping, assignment | 41.7 | 61.5 | 60.8 | 61.9 | 62.5 | 42.6 | 61.8 | 61.3 | 61.9 | 62.6 | 42.0 | 61.4 | 60.7 | 61.5 | 62.6 |
| Seed 23, eighth, mirrored | 58.1 | 61.6 | 61.9 | 61.8 | 62.6 | 58.3 | 61.5 | 61.3 | 61.6 | 62.5 | 58.1 | 62.1 | 62.2 | 62.0 | 62.9 |
| Seed 23, first mapping, assignment | 27.4 | 55.8 | 61.0 | 60.1 | 62.0 | 27.1 | 55.9 | 60.7 | 60.0 | 61.6 | 27.3 | 56.4 | 61.2 | 61.0 | 61.9 |
| Seed 24, eighth, mirrored | 61.6 | 62.4 | 62.6 | 62.2 | 62.8 | 61.8 | 62.9 | 62.2 | 62.0 | 62.9 | 61.7 | 62.7 | 62.4 | 62.3 | 62.9 |
| Seed 24, first mapping, assignment | 34.6 | 51.5 | 59.7 | 58.6 | 62.1 | 34.4 | 51.3 | 59.2 | 57.5 | 61.7 | 35.0 | 53.3 | 60.0 | 58.8 | 62.0 |
| Seed 23, eighth, assignment | 55.1 | 62.2 | 62.3 | 63.3 | 63.1 | 55.3 | 62.4 | 62.3 | 62.9 | 63.3 | 54.6 | 62.0 | 61.9 | 62.6 | 63.3 |
| Seed 24, first mapping, mirrored | 51.8 | 61.2 | 62.2 | 61.9 | 62.7 | 52.0 | 60.6 | 61.4 | 61.4 | 62.5 | 52.3 | 61.5 | 61.7 | 61.8 | 62.6 |
| Seed 23, first mapping, mirrored | 49.2 | 59.6 | 61.8 | 62.3 | 63.1 | 49.4 | 59.3 | 62.4 | 63.0 | 63.2 | 48.8 | 59.5 | 61.7 | 62.7 | 63.2 |
| Seed 24, eighth, assignment | 61.6 | 62.4 | 62.5 | 62.4 | 62.8 | 61.3 | 62.4 | 62.4 | 61.6 | 63.2 | 61.4 | 62.4 | 62.6 | 62.4 | 62.9 |
| Seed 22, first mapping, mirrored | 54.2 | 61.1 | 62.0 | 61.3 | 62.4 | 54.7 | 61.2 | 62.3 | 61.6 | 62.7 | 54.4 | 60.9 | 62.0 | 61.5 | 62.4 |
| Seed 22, eighth, assignment | 63.1 | 62.6 | 63.1 | 61.9 | 63.0 | 63.0 | 62.5 | 62.7 | 61.7 | 62.8 | 63.0 | 62.3 | 62.8 | 61.2 | 63.0 |

| Cell | At 36: without replay | own group and inhibitory units | own group and outside the readouts | inhibitory units and outside the readouts | plain on A's (H-47) | held within (027) | excitatory (025, 026) | own group alone (028) | inhibitory units alone (028) | outside the readouts alone (028) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 63.0 | 62.8 | 63.0 | 63.0 | 62.8 | 62.9 | 62.9 | 63.1 | 63.0 | 63.1 |
| Seed 22, first mapping, assignment | 63.3 | 62.5 | 62.6 | 62.6 | 63.1 | 62.5 | 62.7 | 62.8 | 62.8 | 63.0 |
| Seed 23, eighth, mirrored | 62.6 | 62.6 | 62.5 | 62.9 | 63.1 | 62.6 | 62.8 | 62.4 | 62.7 | 62.8 |
| Seed 23, first mapping, assignment | 63.1 | 62.0 | 61.6 | 61.9 | 61.9 | 61.7 | 61.9 | 61.9 | 61.9 | 61.8 |
| Seed 24, eighth, mirrored | 62.7 | 62.8 | 62.9 | 62.9 | 63.0 | 63.0 | 63.0 | 62.8 | 63.1 | 63.2 |
| Seed 24, first mapping, assignment | 62.1 | 62.1 | 61.7 | 62.0 | 61.8 | 61.3 | 61.7 | 62.0 | 61.9 | 61.7 |
| Seed 23, eighth, assignment | 63.4 | 63.1 | 63.3 | 63.3 | 63.4 | 63.1 | 62.8 | 63.3 | 63.3 | 63.3 |
| Seed 24, first mapping, mirrored | 63.4 | 62.7 | 62.5 | 62.6 | 62.4 | 62.1 | 62.4 | 62.5 | 62.5 | 62.7 |
| Seed 23, first mapping, mirrored | 63.5 | 63.1 | 63.2 | 63.2 | 63.2 | 63.2 | 63.0 | 63.3 | 63.3 | 63.3 |
| Seed 24, eighth, assignment | 62.6 | 62.8 | 63.2 | 62.9 | 62.9 | 62.7 | 62.6 | 63.2 | 63.3 | 62.9 |
| Seed 22, first mapping, mirrored | 63.1 | 62.4 | 62.7 | 62.4 | 62.5 | 62.6 | 62.9 | 62.6 | 62.2 | 62.5 |
| Seed 22, eighth, assignment | 62.6 | 63.0 | 62.8 | 63.0 | 62.9 | 62.8 | 62.9 | 63.0 | 63.1 | 62.8 |

A's highest coupling over H-42's under each form, in any image read and at any block's end, as clause 3 reads it; its bound is 1.30:

| Cell | Without replay | Own group and inhibitory units | Own group and outside the readouts | Inhibitory units and outside the readouts | Plain on A's (H-47) | Held within (027) | Excitatory (025, 026) | Own group alone (028) | Inhibitory units alone (028) | Outside the readouts alone (028) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 1.144 | 1.195 | 1.199 | 1.204 | 1.197 | 1.202 | 1.201 | 1.192 | 1.196 | 1.201 |
| Seed 22, first mapping, assignment | 1.112 | 1.201 | 1.206 | 1.206 | 1.198 | 1.205 | 1.202 | 1.204 | 1.204 | 1.205 |
| Seed 23, eighth, mirrored | 1.147 | 1.206 | 1.215 | 1.214 | 1.211 | 1.211 | 1.212 | 1.208 | 1.209 | 1.220 |
| Seed 23, first mapping, assignment | 1.125 | 1.208 | 1.211 | 1.212 | 1.212 | 1.216 | 1.212 | 1.211 | 1.210 | 1.213 |
| Seed 24, eighth, mirrored | 1.078 | 1.139 | 1.147 | 1.146 | 1.141 | 1.145 | 1.146 | 1.141 | 1.138 | 1.150 |
| Seed 24, first mapping, assignment | 1.085 | 1.163 | 1.168 | 1.170 | 1.162 | 1.170 | 1.165 | 1.159 | 1.160 | 1.167 |
| Seed 23, eighth, assignment | 1.118 | 1.163 | 1.165 | 1.164 | 1.171 | 1.167 | 1.162 | 1.155 | 1.164 | 1.165 |
| Seed 24, first mapping, mirrored | 1.177 | 1.217 | 1.234 | 1.238 | 1.223 | 1.236 | 1.236 | 1.223 | 1.228 | 1.237 |
| Seed 23, first mapping, mirrored | 1.082 | 1.155 | 1.152 | 1.149 | 1.158 | 1.153 | 1.151 | 1.154 | 1.154 | 1.159 |
| Seed 24, eighth, assignment | 1.179 | 1.184 | 1.196 | 1.187 | 1.200 | 1.184 | 1.196 | 1.193 | 1.182 | 1.193 |
| Seed 22, first mapping, mirrored | 1.114 | 1.177 | 1.192 | 1.189 | 1.182 | 1.192 | 1.183 | 1.184 | 1.179 | 1.191 |
| Seed 22, eighth, assignment | 1.133 | 1.187 | 1.197 | 1.195 | 1.184 | 1.197 | 1.193 | 1.184 | 1.188 | 1.199 |

At block 36 the pair stands at 62.0 to 63.1 under the night held onto the own group and the inhibitory units, 61.6 to 63.3 under the night
held onto the own group and the units outside the readouts and 61.9 to 63.3 under the night held onto the inhibitory units and the units
outside the readouts, where H-47's plain night gave 61.8 to 63.4, exploration 027's night held within 61.3 to 63.2, the excitatory night
61.7 to 63.0, exploration 028's nights with the own group, the inhibitory units and the units outside the readouts held alone 61.9 to 63.3,
61.9 to 63.3 and 61.7 to 63.3, and the night without replay 62.1 to 63.5. A's highest coupling is 1.139 to 1.217 of H-42's under the night
held onto the own group and the inhibitory units, 1.147 to 1.234 under the night held onto the own group and the units outside the readouts
and 1.146 to 1.238 under the night held onto the inhibitory units and the units outside the readouts, where H-47's plain night gave 1.141
to 1.223, exploration 027's night held within 1.145 to 1.236, the excitatory night 1.146 to 1.236, exploration 028's three nights 1.141 to
1.223, 1.138 to 1.228 and 1.150 to 1.237, and the night without replay 1.078 to 1.179. The H-42 couplings each job reads against are the
same in this job, in H-47's, in exploration 027's, in exploration 028's and in the excitatory night's, in every cell.

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
| Seed 22, eighth, mirrored | Own group and inhibitory units | 0.00 | −5.92 | 0.00 | −0.47 | +0.01 | 0.00 | −6.56 | 0.00 | −0.73 | −0.02 |
| Seed 22, eighth, mirrored | Own group and outside the readouts | 0.00 | −5.97 | −2.27 | 0.00 | 0.00 | 0.00 | −6.65 | −1.99 | 0.00 | 0.00 |
| Seed 22, eighth, mirrored | Inhibitory units and outside the readouts | −0.71 | −5.99 | 0.00 | 0.00 | 0.00 | −0.89 | −6.75 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, mirrored | Plain on A's (H-47) | −0.66 | −5.98 | −2.19 | −0.40 | 0.00 | −0.92 | −6.72 | −1.96 | −0.74 | −0.01 |
| Seed 22, eighth, mirrored | Held within (027) | 0.00 | −5.94 | 0.00 | 0.00 | 0.00 | 0.00 | −6.62 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, mirrored | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, assignment | Without replay | −0.45 | −0.52 | +1.49 | −0.04 | +0.04 | −0.75 | −0.61 | +1.41 | −0.24 | −0.03 |
| Seed 22, first mapping, assignment | Own group and inhibitory units | 0.00 | −5.59 | 0.00 | −0.26 | +0.02 | 0.00 | −6.22 | 0.00 | −0.74 | −0.02 |
| Seed 22, first mapping, assignment | Own group and outside the readouts | 0.00 | −5.59 | −2.01 | 0.00 | 0.00 | 0.00 | −6.26 | −1.57 | 0.00 | 0.00 |
| Seed 22, first mapping, assignment | Inhibitory units and outside the readouts | −0.66 | −5.69 | 0.00 | 0.00 | 0.00 | −0.78 | −6.33 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, assignment | Plain on A's (H-47) | −0.73 | −5.66 | −2.02 | −0.25 | +0.01 | −0.76 | −6.32 | −1.56 | −0.73 | −0.01 |
| Seed 22, first mapping, assignment | Held within (027) | 0.00 | −5.60 | 0.00 | 0.00 | 0.00 | 0.00 | −6.27 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, assignment | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, mirrored | Without replay | +0.23 | −0.01 | +1.35 | −0.04 | −0.03 | −0.20 | −0.59 | +1.34 | −0.19 | 0.00 |
| Seed 23, eighth, mirrored | Own group and inhibitory units | 0.00 | −6.76 | 0.00 | −0.62 | +0.01 | 0.00 | −6.64 | 0.00 | −0.52 | −0.04 |
| Seed 23, eighth, mirrored | Own group and outside the readouts | 0.00 | −6.78 | −2.67 | 0.00 | 0.00 | 0.00 | −6.75 | −2.17 | 0.00 | 0.00 |
| Seed 23, eighth, mirrored | Inhibitory units and outside the readouts | −0.58 | −6.86 | 0.00 | 0.00 | 0.00 | −1.21 | −6.55 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, mirrored | Plain on A's (H-47) | −0.56 | −6.98 | −2.71 | −0.59 | +0.01 | −1.17 | −6.55 | −2.07 | −0.47 | −0.03 |
| Seed 23, eighth, mirrored | Held within (027) | 0.00 | −6.75 | 0.00 | 0.00 | 0.00 | 0.00 | −6.67 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, mirrored | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, assignment | Without replay | +0.12 | −0.17 | +1.53 | +0.04 | −0.03 | −0.30 | −0.38 | +1.53 | −0.27 | 0.00 |
| Seed 23, first mapping, assignment | Own group and inhibitory units | 0.00 | −6.27 | 0.00 | −0.25 | −0.01 | 0.00 | −5.71 | 0.00 | −0.36 | −0.06 |
| Seed 23, first mapping, assignment | Own group and outside the readouts | 0.00 | −6.22 | −2.38 | 0.00 | 0.00 | 0.00 | −5.82 | −1.62 | 0.00 | 0.00 |
| Seed 23, first mapping, assignment | Inhibitory units and outside the readouts | −0.03 | −6.28 | 0.00 | 0.00 | 0.00 | −0.76 | −5.71 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, assignment | Plain on A's (H-47) | −0.02 | −6.24 | −2.31 | −0.18 | 0.00 | −0.80 | −5.79 | −1.55 | −0.35 | −0.06 |
| Seed 23, first mapping, assignment | Held within (027) | 0.00 | −6.25 | 0.00 | 0.00 | 0.00 | 0.00 | −5.71 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, assignment | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, mirrored | Without replay | −0.39 | +0.07 | +1.08 | −0.07 | +0.07 | −0.78 | −0.31 | +0.88 | −0.11 | +0.10 |
| Seed 24, eighth, mirrored | Own group and inhibitory units | 0.00 | −5.38 | 0.00 | −0.24 | −0.02 | 0.00 | −6.84 | 0.00 | −0.99 | +0.06 |
| Seed 24, eighth, mirrored | Own group and outside the readouts | 0.00 | −5.41 | −2.38 | 0.00 | 0.00 | 0.00 | −6.92 | −1.92 | 0.00 | 0.00 |
| Seed 24, eighth, mirrored | Inhibitory units and outside the readouts | −0.93 | −5.38 | 0.00 | 0.00 | 0.00 | −1.37 | −6.90 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, mirrored | Plain on A's (H-47) | −0.90 | −5.39 | −2.43 | −0.25 | −0.05 | −1.39 | −6.95 | −1.82 | −0.97 | +0.06 |
| Seed 24, eighth, mirrored | Held within (027) | 0.00 | −5.40 | 0.00 | 0.00 | 0.00 | 0.00 | −6.84 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, mirrored | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, assignment | Without replay | −0.37 | +0.14 | +1.23 | +0.19 | +0.14 | −0.46 | −0.03 | +1.42 | −0.07 | +0.10 |
| Seed 24, first mapping, assignment | Own group and inhibitory units | 0.00 | −5.19 | 0.00 | +0.05 | −0.02 | 0.00 | −5.91 | 0.00 | −0.86 | +0.05 |
| Seed 24, first mapping, assignment | Own group and outside the readouts | 0.00 | −5.19 | −2.03 | 0.00 | 0.00 | 0.00 | −5.96 | −1.44 | 0.00 | 0.00 |
| Seed 24, first mapping, assignment | Inhibitory units and outside the readouts | −0.58 | −5.14 | 0.00 | 0.00 | 0.00 | −1.15 | −5.97 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, assignment | Plain on A's (H-47) | −0.62 | −5.10 | −1.98 | +0.12 | −0.03 | −1.17 | −6.04 | −1.45 | −0.87 | +0.05 |
| Seed 24, first mapping, assignment | Held within (027) | 0.00 | −5.20 | 0.00 | 0.00 | 0.00 | 0.00 | −5.93 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, assignment | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, assignment | Without replay | +0.49 | −0.17 | +1.19 | −0.03 | −0.03 | −0.15 | −0.57 | +1.10 | −0.23 | −0.01 |
| Seed 23, eighth, assignment | Own group and inhibitory units | 0.00 | −6.26 | 0.00 | −0.90 | −0.01 | 0.00 | −6.07 | 0.00 | −0.72 | −0.02 |
| Seed 23, eighth, assignment | Own group and outside the readouts | 0.00 | −6.42 | −2.54 | 0.00 | 0.00 | 0.00 | −6.13 | −2.16 | 0.00 | 0.00 |
| Seed 23, eighth, assignment | Inhibitory units and outside the readouts | −2.44 | −6.21 | 0.00 | 0.00 | 0.00 | −2.22 | −5.99 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, assignment | Plain on A's (H-47) | −2.38 | −6.32 | −2.44 | −0.82 | −0.01 | −2.18 | −5.94 | −2.12 | −0.70 | −0.02 |
| Seed 23, eighth, assignment | Held within (027) | 0.00 | −6.29 | 0.00 | 0.00 | 0.00 | 0.00 | −6.06 | 0.00 | 0.00 | 0.00 |
| Seed 23, eighth, assignment | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, mirrored | Without replay | −0.39 | +0.25 | +1.09 | +0.19 | +0.14 | −0.58 | −0.05 | +1.38 | −0.01 | +0.10 |
| Seed 24, first mapping, mirrored | Own group and inhibitory units | 0.00 | −5.31 | 0.00 | −0.77 | −0.02 | 0.00 | −5.54 | 0.00 | −0.62 | 0.00 |
| Seed 24, first mapping, mirrored | Own group and outside the readouts | 0.00 | −5.40 | −2.16 | 0.00 | 0.00 | 0.00 | −5.62 | −2.19 | 0.00 | 0.00 |
| Seed 24, first mapping, mirrored | Inhibitory units and outside the readouts | −1.61 | −5.28 | 0.00 | 0.00 | 0.00 | −2.33 | −5.56 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, mirrored | Plain on A's (H-47) | −1.60 | −5.34 | −2.09 | −0.79 | −0.03 | −2.39 | −5.55 | −2.18 | −0.60 | +0.01 |
| Seed 24, first mapping, mirrored | Held within (027) | 0.00 | −5.34 | 0.00 | 0.00 | 0.00 | 0.00 | −5.57 | 0.00 | 0.00 | 0.00 |
| Seed 24, first mapping, mirrored | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, mirrored | Without replay | +0.19 | −0.15 | +1.35 | +0.12 | −0.03 | −0.25 | −0.54 | +1.25 | −0.26 | −0.01 |
| Seed 23, first mapping, mirrored | Own group and inhibitory units | 0.00 | −5.81 | 0.00 | −0.69 | −0.02 | 0.00 | −5.54 | 0.00 | −0.57 | 0.00 |
| Seed 23, first mapping, mirrored | Own group and outside the readouts | 0.00 | −5.88 | −2.06 | 0.00 | 0.00 | 0.00 | −5.66 | −1.97 | 0.00 | 0.00 |
| Seed 23, first mapping, mirrored | Inhibitory units and outside the readouts | −2.01 | −5.67 | 0.00 | 0.00 | 0.00 | −1.72 | −5.52 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, mirrored | Plain on A's (H-47) | −2.02 | −5.68 | −2.03 | −0.67 | −0.02 | −1.66 | −5.56 | −1.94 | −0.52 | 0.00 |
| Seed 23, first mapping, mirrored | Held within (027) | 0.00 | −5.85 | 0.00 | 0.00 | 0.00 | 0.00 | −5.56 | 0.00 | 0.00 | 0.00 |
| Seed 23, first mapping, mirrored | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, assignment | Without replay | −0.50 | +0.10 | +1.05 | −0.04 | +0.08 | −0.72 | −0.36 | +0.99 | −0.07 | +0.09 |
| Seed 24, eighth, assignment | Own group and inhibitory units | 0.00 | −6.03 | 0.00 | −1.04 | −0.02 | 0.00 | −6.16 | 0.00 | −0.81 | +0.04 |
| Seed 24, eighth, assignment | Own group and outside the readouts | 0.00 | −6.16 | −2.46 | 0.00 | 0.00 | 0.00 | −6.25 | −2.62 | 0.00 | 0.00 |
| Seed 24, eighth, assignment | Inhibitory units and outside the readouts | −1.78 | −6.07 | 0.00 | 0.00 | 0.00 | −2.71 | −6.16 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, assignment | Plain on A's (H-47) | −1.85 | −6.12 | −2.35 | −0.96 | −0.04 | −2.71 | −6.23 | −2.59 | −0.79 | +0.04 |
| Seed 24, eighth, assignment | Held within (027) | 0.00 | −6.08 | 0.00 | 0.00 | 0.00 | 0.00 | −6.23 | 0.00 | 0.00 | 0.00 |
| Seed 24, eighth, assignment | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, mirrored | Without replay | −0.41 | −0.42 | +1.38 | −0.02 | +0.03 | −0.78 | −0.55 | +1.36 | −0.20 | −0.02 |
| Seed 22, first mapping, mirrored | Own group and inhibitory units | 0.00 | −4.74 | 0.00 | −0.40 | 0.00 | 0.00 | −6.01 | 0.00 | −0.28 | −0.02 |
| Seed 22, first mapping, mirrored | Own group and outside the readouts | 0.00 | −4.88 | −1.67 | 0.00 | 0.00 | 0.00 | −6.08 | −2.11 | 0.00 | 0.00 |
| Seed 22, first mapping, mirrored | Inhibitory units and outside the readouts | −2.38 | −4.73 | 0.00 | 0.00 | 0.00 | −2.24 | −5.89 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, mirrored | Plain on A's (H-47) | −2.39 | −4.73 | −1.65 | −0.34 | +0.01 | −2.30 | −6.01 | −2.09 | −0.23 | −0.02 |
| Seed 22, first mapping, mirrored | Held within (027) | 0.00 | −4.80 | 0.00 | 0.00 | 0.00 | 0.00 | −5.98 | 0.00 | 0.00 | 0.00 |
| Seed 22, first mapping, mirrored | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, assignment | Without replay | −0.52 | −0.37 | +0.77 | −0.01 | +0.04 | −0.83 | −0.68 | +1.15 | −0.15 | −0.03 |
| Seed 22, eighth, assignment | Own group and inhibitory units | 0.00 | −5.68 | 0.00 | −0.49 | +0.03 | 0.00 | −6.55 | 0.00 | −0.48 | −0.01 |
| Seed 22, eighth, assignment | Own group and outside the readouts | 0.00 | −5.77 | −2.19 | 0.00 | 0.00 | 0.00 | −6.61 | −2.28 | 0.00 | 0.00 |
| Seed 22, eighth, assignment | Inhibitory units and outside the readouts | −2.47 | −5.70 | 0.00 | 0.00 | 0.00 | −2.76 | −6.53 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, assignment | Plain on A's (H-47) | −2.49 | −5.68 | −2.05 | −0.46 | +0.03 | −2.79 | −6.56 | −2.27 | −0.41 | −0.01 |
| Seed 22, eighth, assignment | Held within (027) | 0.00 | −5.71 | 0.00 | 0.00 | 0.00 | 0.00 | −6.56 | 0.00 | 0.00 | 0.00 |
| Seed 22, eighth, assignment | Excitatory (025, 026) | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 |

- **The two held parts' change is nil.** Under each held night the entries of its two parts, both groups' held units onto the columns of
  those parts, moved by exactly nothing, in LSB, in every one of the 432 held nights (below). An entry of the part left free or of the part
  held across may show 0.00 too, rounded: those entries are not held.
- **The part left free and the part held across moved as under H-47's plain night**, within 0.22 M a group of the plain night's change over
  the four nights, group by group. Under the night held onto the own group and the inhibitory units the held units' synapses onto the units
  outside the readouts, the part left free, moved by −1.06 to +0.03 M a group, −0.08 to +0.03 M less the plain night's, and across by
  −6.84 to −4.74 M, −0.13 to +0.22 M less the plain night's. Under the night held onto the own group and the units outside the readouts the
  part onto the inhibitory units moved by −2.67 to −1.44 M, −0.14 to +0.05 M less the plain night's, and across by −6.92 to −4.88 M, −0.21
  to +0.20 M less. Under the night held onto the inhibitory units and the units outside the readouts the part onto the own group moved by
  −2.76 to −0.03 M, −0.07 to +0.07 M less the plain night's, and across by −6.90 to −4.73 M, −0.05 to +0.12 M less. The plain night moved
  them by −2.79 to −0.02 M a group onto their own group, −2.71 to −1.45 M onto the inhibitory units, −1.01 to +0.09 M onto the units
  outside the readouts and −6.98 to −4.73 M across.
- Under the night without replay the held units' synapses moved by −0.83 to +0.49 M a group onto their own group, +0.77 to +1.53 M onto the
  inhibitory units, −0.27 to +0.33 M onto the units outside the readouts and −0.68 to +0.25 M across.

The rest of each night's change: the inhibition onto each group's excitatory units and onto the other targets, and the excitatory units
outside the hold onto every target:

| Cell | Form | Inhibitory onto the odd places' excitatory units | onto the even places' | onto the inhibitory units | onto A's and B's units | onto the other excitatory | C's own onto all | D's own onto all | A's and B's units onto all | The other excitatory onto all |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | Without replay | +1.10 | +0.70 | +0.66 | −1.17 | +0.03 | −0.04 | +0.01 | −0.02 | +0.11 |
| Seed 22, eighth, mirrored | Own group and inhibitory units | +14.99 | +12.73 | +4.53 | +0.12 | +0.03 | −0.07 | −0.02 | −0.04 | −0.10 |
| Seed 22, eighth, mirrored | Own group and outside the readouts | +14.26 | +12.36 | +4.45 | +0.12 | +0.04 | −0.05 | −0.02 | −0.04 | −0.14 |
| Seed 22, eighth, mirrored | Inhibitory units and outside the readouts | +15.01 | +12.70 | +4.56 | +0.13 | +0.04 | −0.04 | −0.03 | −0.04 | −0.15 |
| Seed 22, eighth, mirrored | Plain on A's (H-47) | +14.23 | +12.21 | +4.39 | +0.09 | +0.04 | −0.05 | −0.02 | −0.04 | −0.10 |
| Seed 22, eighth, mirrored | Held within (027) | +15.03 | +12.79 | +4.54 | +0.14 | +0.04 | −0.05 | −0.02 | −0.04 | −0.15 |
| Seed 22, eighth, mirrored | Excitatory (025, 026) | +15.87 | +13.70 | +4.79 | +0.14 | +0.04 | −0.08 | −0.06 | −0.04 | −0.15 |
| Seed 22, first mapping, assignment | Without replay | −1.89 | −2.20 | −0.35 | −0.97 | +0.04 | −0.04 | +0.01 | −0.02 | +0.13 |
| Seed 22, first mapping, assignment | Own group and inhibitory units | +9.87 | +8.45 | +3.64 | +1.03 | +0.03 | −0.07 | −0.06 | −0.07 | −0.05 |
| Seed 22, first mapping, assignment | Own group and outside the readouts | +9.45 | +8.15 | +3.50 | +1.03 | +0.03 | −0.07 | −0.06 | −0.07 | −0.09 |
| Seed 22, first mapping, assignment | Inhibitory units and outside the readouts | +9.79 | +8.36 | +3.66 | +1.04 | +0.02 | −0.05 | −0.08 | −0.07 | −0.09 |
| Seed 22, first mapping, assignment | Plain on A's (H-47) | +9.29 | +7.95 | +3.43 | +0.99 | +0.02 | −0.06 | −0.08 | −0.07 | −0.06 |
| Seed 22, first mapping, assignment | Held within (027) | +9.89 | +8.48 | +3.66 | +1.04 | +0.03 | −0.06 | −0.07 | −0.07 | −0.09 |
| Seed 22, first mapping, assignment | Excitatory (025, 026) | +10.28 | +8.98 | +3.89 | +1.08 | +0.03 | −0.07 | −0.08 | −0.06 | −0.10 |
| Seed 23, eighth, mirrored | Without replay | +1.13 | +0.88 | +0.79 | −1.02 | +0.02 | 0.00 | −0.05 | −0.03 | −0.02 |
| Seed 23, eighth, mirrored | Own group and inhibitory units | +15.43 | +13.51 | +4.87 | +0.19 | +0.06 | −0.04 | −0.01 | −0.07 | −0.23 |
| Seed 23, eighth, mirrored | Own group and outside the readouts | +14.60 | +13.03 | +4.62 | +0.14 | +0.05 | −0.05 | −0.02 | −0.08 | −0.24 |
| Seed 23, eighth, mirrored | Inhibitory units and outside the readouts | +15.45 | +13.56 | +4.87 | +0.20 | +0.06 | −0.04 | −0.02 | −0.08 | −0.24 |
| Seed 23, eighth, mirrored | Plain on A's (H-47) | +14.40 | +12.88 | +4.55 | +0.13 | +0.05 | −0.05 | −0.03 | −0.07 | −0.24 |
| Seed 23, eighth, mirrored | Held within (027) | +15.50 | +13.55 | +4.91 | +0.20 | +0.06 | −0.05 | −0.01 | −0.08 | −0.24 |
| Seed 23, eighth, mirrored | Excitatory (025, 026) | +16.32 | +14.41 | +5.18 | +0.21 | +0.05 | −0.08 | −0.02 | −0.07 | −0.23 |
| Seed 23, first mapping, assignment | Without replay | −1.65 | −1.99 | −0.59 | −0.87 | −0.03 | −0.03 | −0.05 | −0.04 | −0.03 |
| Seed 23, first mapping, assignment | Own group and inhibitory units | +10.36 | +8.50 | +3.44 | +1.23 | +0.01 | −0.06 | −0.07 | −0.09 | −0.14 |
| Seed 23, first mapping, assignment | Own group and outside the readouts | +10.06 | +8.19 | +3.12 | +1.18 | +0.01 | −0.07 | −0.06 | −0.09 | −0.16 |
| Seed 23, first mapping, assignment | Inhibitory units and outside the readouts | +10.38 | +8.47 | +3.44 | +1.23 | 0.00 | −0.06 | −0.06 | −0.09 | −0.15 |
| Seed 23, first mapping, assignment | Plain on A's (H-47) | +9.88 | +8.08 | +3.09 | +1.16 | +0.01 | −0.07 | −0.06 | −0.09 | −0.15 |
| Seed 23, first mapping, assignment | Held within (027) | +10.44 | +8.52 | +3.47 | +1.23 | 0.00 | −0.06 | −0.06 | −0.09 | −0.15 |
| Seed 23, first mapping, assignment | Excitatory (025, 026) | +10.88 | +9.08 | +3.68 | +1.25 | 0.00 | −0.09 | −0.07 | −0.09 | −0.13 |
| Seed 24, eighth, mirrored | Without replay | +0.93 | +0.94 | +0.52 | −1.00 | +0.02 | −0.02 | −0.05 | −0.03 | −0.13 |
| Seed 24, eighth, mirrored | Own group and inhibitory units | +15.37 | +11.62 | +4.00 | +0.10 | 0.00 | +0.05 | 0.00 | −0.09 | −0.23 |
| Seed 24, eighth, mirrored | Own group and outside the readouts | +14.59 | +11.34 | +3.91 | +0.09 | +0.02 | +0.04 | −0.01 | −0.08 | −0.25 |
| Seed 24, eighth, mirrored | Inhibitory units and outside the readouts | +15.42 | +11.54 | +4.00 | +0.13 | +0.02 | +0.06 | −0.01 | −0.09 | −0.24 |
| Seed 24, eighth, mirrored | Plain on A's (H-47) | +14.61 | +11.20 | +3.89 | +0.05 | 0.00 | +0.06 | −0.02 | −0.08 | −0.22 |
| Seed 24, eighth, mirrored | Held within (027) | +15.44 | +11.65 | +4.03 | +0.14 | +0.02 | +0.05 | 0.00 | −0.08 | −0.24 |
| Seed 24, eighth, mirrored | Excitatory (025, 026) | +15.90 | +12.12 | +4.11 | +0.15 | +0.02 | +0.04 | −0.02 | −0.08 | −0.24 |
| Seed 24, first mapping, assignment | Without replay | −1.73 | −1.86 | −0.73 | −0.86 | +0.03 | 0.00 | −0.06 | −0.05 | −0.17 |
| Seed 24, first mapping, assignment | Own group and inhibitory units | +11.01 | +7.13 | +2.87 | +1.09 | +0.01 | +0.05 | 0.00 | −0.10 | −0.22 |
| Seed 24, first mapping, assignment | Own group and outside the readouts | +10.53 | +6.91 | +2.77 | +1.06 | +0.04 | +0.05 | −0.01 | −0.09 | −0.21 |
| Seed 24, first mapping, assignment | Inhibitory units and outside the readouts | +10.95 | +7.02 | +2.86 | +1.10 | +0.03 | +0.06 | 0.00 | −0.09 | −0.20 |
| Seed 24, first mapping, assignment | Plain on A's (H-47) | +10.49 | +6.77 | +2.76 | +1.03 | +0.01 | +0.07 | −0.01 | −0.10 | −0.23 |
| Seed 24, first mapping, assignment | Held within (027) | +11.02 | +7.11 | +2.87 | +1.11 | +0.04 | +0.05 | 0.00 | −0.09 | −0.20 |
| Seed 24, first mapping, assignment | Excitatory (025, 026) | +11.39 | +7.46 | +3.01 | +1.13 | +0.04 | +0.03 | −0.02 | −0.09 | −0.22 |
| Seed 23, eighth, assignment | Without replay | +1.17 | +1.02 | +0.67 | −1.03 | +0.02 | −0.06 | −0.05 | −0.03 | −0.02 |
| Seed 23, eighth, assignment | Own group and inhibitory units | +13.09 | +14.22 | +4.20 | +0.12 | +0.02 | −0.04 | −0.06 | −0.06 | −0.18 |
| Seed 23, eighth, assignment | Own group and outside the readouts | +12.21 | +13.64 | +3.95 | +0.14 | +0.02 | −0.05 | −0.06 | −0.06 | −0.19 |
| Seed 23, eighth, assignment | Inhibitory units and outside the readouts | +12.92 | +14.09 | +4.14 | +0.14 | +0.02 | −0.05 | −0.06 | −0.07 | −0.18 |
| Seed 23, eighth, assignment | Plain on A's (H-47) | +12.07 | +13.48 | +3.92 | +0.10 | +0.02 | −0.05 | −0.06 | −0.06 | −0.18 |
| Seed 23, eighth, assignment | Held within (027) | +13.13 | +14.24 | +4.20 | +0.15 | +0.02 | −0.05 | −0.06 | −0.06 | −0.18 |
| Seed 23, eighth, assignment | Excitatory (025, 026) | +13.82 | +14.93 | +4.43 | +0.15 | +0.02 | −0.05 | −0.08 | −0.07 | −0.18 |
| Seed 24, first mapping, mirrored | Without replay | −1.77 | −1.92 | −0.73 | −0.87 | +0.03 | +0.01 | −0.06 | −0.05 | −0.17 |
| Seed 24, first mapping, mirrored | Own group and inhibitory units | +8.19 | +8.38 | +2.66 | +1.22 | +0.07 | −0.02 | −0.01 | −0.10 | −0.24 |
| Seed 24, first mapping, mirrored | Own group and outside the readouts | +7.93 | +8.04 | +2.43 | +1.20 | +0.07 | −0.01 | −0.01 | −0.10 | −0.25 |
| Seed 24, first mapping, mirrored | Inhibitory units and outside the readouts | +8.05 | +8.27 | +2.65 | +1.23 | +0.07 | −0.03 | −0.01 | −0.10 | −0.26 |
| Seed 24, first mapping, mirrored | Plain on A's (H-47) | +7.72 | +7.83 | +2.32 | +1.18 | +0.06 | −0.03 | −0.01 | −0.10 | −0.26 |
| Seed 24, first mapping, mirrored | Held within (027) | +8.26 | +8.47 | +2.71 | +1.24 | +0.07 | −0.02 | −0.01 | −0.10 | −0.25 |
| Seed 24, first mapping, mirrored | Excitatory (025, 026) | +8.60 | +8.80 | +2.81 | +1.24 | +0.07 | −0.04 | −0.02 | −0.10 | −0.24 |
| Seed 23, first mapping, mirrored | Without replay | −1.79 | −2.04 | −0.59 | −0.90 | −0.04 | −0.01 | −0.05 | −0.04 | −0.02 |
| Seed 23, first mapping, mirrored | Own group and inhibitory units | +8.94 | +9.21 | +3.18 | +1.08 | −0.02 | −0.05 | −0.04 | −0.07 | −0.12 |
| Seed 23, first mapping, mirrored | Own group and outside the readouts | +8.51 | +8.93 | +2.95 | +1.09 | −0.03 | −0.06 | −0.05 | −0.07 | −0.11 |
| Seed 23, first mapping, mirrored | Inhibitory units and outside the readouts | +8.81 | +9.13 | +3.17 | +1.10 | −0.03 | −0.05 | −0.05 | −0.07 | −0.11 |
| Seed 23, first mapping, mirrored | Plain on A's (H-47) | +8.33 | +8.77 | +2.94 | +1.07 | −0.03 | −0.04 | −0.04 | −0.07 | −0.12 |
| Seed 23, first mapping, mirrored | Held within (027) | +8.96 | +9.22 | +3.18 | +1.10 | −0.03 | −0.06 | −0.05 | −0.07 | −0.11 |
| Seed 23, first mapping, mirrored | Excitatory (025, 026) | +9.41 | +9.56 | +3.37 | +1.10 | −0.03 | −0.06 | −0.05 | −0.08 | −0.11 |
| Seed 24, eighth, assignment | Without replay | +0.98 | +0.78 | +0.54 | −0.99 | +0.02 | −0.01 | −0.05 | −0.03 | −0.13 |
| Seed 24, eighth, assignment | Own group and inhibitory units | +12.15 | +12.76 | +3.95 | +0.25 | +0.04 | +0.08 | −0.05 | −0.10 | −0.22 |
| Seed 24, eighth, assignment | Own group and outside the readouts | +11.72 | +12.11 | +3.73 | +0.24 | +0.05 | +0.09 | −0.05 | −0.10 | −0.24 |
| Seed 24, eighth, assignment | Inhibitory units and outside the readouts | +11.94 | +12.59 | +3.93 | +0.25 | +0.05 | +0.09 | −0.05 | −0.10 | −0.25 |
| Seed 24, eighth, assignment | Plain on A's (H-47) | +11.38 | +11.76 | +3.66 | +0.22 | +0.04 | +0.09 | −0.05 | −0.10 | −0.21 |
| Seed 24, eighth, assignment | Held within (027) | +12.19 | +12.85 | +3.98 | +0.25 | +0.05 | +0.08 | −0.04 | −0.10 | −0.26 |
| Seed 24, eighth, assignment | Excitatory (025, 026) | +12.61 | +13.27 | +4.07 | +0.25 | +0.04 | +0.05 | −0.07 | −0.10 | −0.26 |
| Seed 22, first mapping, mirrored | Without replay | −1.92 | −2.23 | −0.39 | −0.99 | +0.03 | −0.03 | +0.01 | −0.03 | +0.13 |
| Seed 22, first mapping, mirrored | Own group and inhibitory units | +7.03 | +7.56 | +3.29 | +0.79 | +0.05 | −0.11 | −0.13 | −0.07 | −0.03 |
| Seed 22, first mapping, mirrored | Own group and outside the readouts | +6.65 | +7.37 | +3.09 | +0.74 | +0.05 | −0.10 | −0.14 | −0.07 | −0.07 |
| Seed 22, first mapping, mirrored | Inhibitory units and outside the readouts | +6.91 | +7.38 | +3.20 | +0.81 | +0.05 | −0.10 | −0.13 | −0.07 | −0.07 |
| Seed 22, first mapping, mirrored | Plain on A's (H-47) | +6.52 | +7.22 | +3.05 | +0.70 | +0.06 | −0.10 | −0.13 | −0.08 | −0.04 |
| Seed 22, first mapping, mirrored | Held within (027) | +7.05 | +7.56 | +3.28 | +0.82 | +0.05 | −0.10 | −0.13 | −0.07 | −0.07 |
| Seed 22, first mapping, mirrored | Excitatory (025, 026) | +7.47 | +7.95 | +3.44 | +0.87 | +0.05 | −0.12 | −0.14 | −0.07 | −0.07 |
| Seed 22, eighth, assignment | Without replay | +0.89 | +0.60 | +0.62 | −1.11 | +0.03 | −0.04 | +0.03 | −0.03 | +0.11 |
| Seed 22, eighth, assignment | Own group and inhibitory units | +11.35 | +11.89 | +3.94 | +0.03 | +0.05 | −0.04 | −0.09 | −0.06 | −0.07 |
| Seed 22, eighth, assignment | Own group and outside the readouts | +10.77 | +11.50 | +3.73 | +0.04 | +0.05 | −0.05 | −0.10 | −0.06 | −0.11 |
| Seed 22, eighth, assignment | Inhibitory units and outside the readouts | +11.14 | +11.70 | +3.89 | +0.04 | +0.05 | −0.03 | −0.10 | −0.06 | −0.11 |
| Seed 22, eighth, assignment | Plain on A's (H-47) | +10.61 | +11.30 | +3.72 | 0.00 | +0.05 | −0.03 | −0.09 | −0.07 | −0.07 |
| Seed 22, eighth, assignment | Held within (027) | +11.34 | +11.90 | +3.93 | +0.05 | +0.05 | −0.05 | −0.10 | −0.06 | −0.11 |
| Seed 22, eighth, assignment | Excitatory (025, 026) | +11.88 | +12.35 | +4.11 | +0.08 | +0.05 | −0.07 | −0.10 | −0.05 | −0.12 |

- **The inhibition onto each group's excitatory units** rose over the four nights by 7.03 to 15.43 M under the night held onto the own group
  and the inhibitory units, by 6.65 to 14.60 M under the night held onto the own group and the units outside the readouts and by 6.91 to
  15.45 M under the night held onto the inhibitory units and the units outside the readouts, where H-47's plain night raised it by 6.52 to
  14.61 M, exploration 027's night held within by 7.05 to 15.50 M and the excitatory night by 7.46 to 16.32 M. Less the plain night's,
  group by group: +0.33 to +1.03 M under the night held onto the own group and the inhibitory units, the least onto the even places on seed
  22's first mapping from the mirrored assignment and the most onto the odd places on seed 23's eighth image from the mirrored assignment;
  −0.01 to +0.35 M under the night held onto the own group and the units outside the readouts, the least onto the odd places on seed 24's
  eighth image from the mirrored assignment and the most onto the even places on seed 24's eighth image from the assignment; and +0.16 to
  +1.05 M under the night held onto the inhibitory units and the units outside the readouts, the least and the most where the first pair's
  were. Exploration 027's night held within's was +0.33 to +1.11 M and the excitatory night's +0.69 to +1.92 M. Less exploration 027's
  night held within's: −0.09 to +0.02 M, −0.92 to −0.19 M and −0.25 to +0.01 M, in the same order. Under each form with replay, in each of
  the twelve cells over the four nights, it rose by more onto the group A answers than onto the other: by 0.19 to 3.89 M, 0.11 to 3.62 M and
  0.22 to 3.93 M under the three held nights, and 0.10 to 3.71 M under the plain night, the least under each on seed 24's first mapping from
  the mirrored assignment and the most on seed 24's first mapping from the assignment. Onto the inhibitory units it rose by 2.66 to 4.87 M,
  2.43 to 4.62 M and 2.65 to 4.87 M, where the plain night's rose by 2.32 to 4.55 M.
- **The excitatory units outside the hold**: the pair's own units onto all moved by −0.13 to +0.08 M under the night held onto the own
  group and the inhibitory units, −0.14 to +0.09 M under the night held onto the own group and the units outside the readouts and −0.13 to
  +0.09 M under the night held onto the inhibitory units and the units outside the readouts; A's and B's units onto all by −0.10 to −0.04 M
  under each; and the other excitatory units by −0.24 to −0.03 M, −0.25 to −0.07 M and −0.26 to −0.07 M.

The two held parts' entries night by night, against what the quiet ticks after each night moved:

| Cell | Form | Nights the two held parts' entries moved by nothing | Nights they moved by what the quiet ticks moved | Nights the quiet ticks moved held synapses | Nights the part left free's entries moved and the part held across's, of 12 |
| :--- | :--- | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | Own group and inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 22, eighth, mirrored | Own group and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 22, eighth, mirrored | Inhibitory units and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 22, first mapping, assignment | Own group and inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 22, first mapping, assignment | Own group and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 22, first mapping, assignment | Inhibitory units and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 23, eighth, mirrored | Own group and inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 23, eighth, mirrored | Own group and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 23, eighth, mirrored | Inhibitory units and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 23, first mapping, assignment | Own group and inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 23, first mapping, assignment | Own group and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 23, first mapping, assignment | Inhibitory units and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 24, eighth, mirrored | Own group and inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 24, eighth, mirrored | Own group and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 24, eighth, mirrored | Inhibitory units and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 24, first mapping, assignment | Own group and inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 24, first mapping, assignment | Own group and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 24, first mapping, assignment | Inhibitory units and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 23, eighth, assignment | Own group and inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 23, eighth, assignment | Own group and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 23, eighth, assignment | Inhibitory units and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 24, first mapping, mirrored | Own group and inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 24, first mapping, mirrored | Own group and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 24, first mapping, mirrored | Inhibitory units and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 23, first mapping, mirrored | Own group and inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 23, first mapping, mirrored | Own group and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 23, first mapping, mirrored | Inhibitory units and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 24, eighth, assignment | Own group and inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 24, eighth, assignment | Own group and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 24, eighth, assignment | Inhibitory units and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 22, first mapping, mirrored | Own group and inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 22, first mapping, mirrored | Own group and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 22, first mapping, mirrored | Inhibitory units and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 22, eighth, assignment | Own group and inhibitory units | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 22, eighth, assignment | Own group and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |
| Seed 22, eighth, assignment | Inhibitory units and outside the readouts | 12 of 12 | 12 of 12 | 0 | 12, 12 |

- In every cell, under every held night, the two held parts' entries moved by exactly nothing in all twelve nights, and so by what the
  quiet ticks moved of them, nothing; the entries of the part left free and of the part held across moved in all twelve.

Each night's change of the engine's two sums, from its `start` line to its `end` line, the mean over the three paths:

| Cell | Gap | Without replay: inhibitory and excitatory sums, M | Own group and inhibitory units | Own group and outside the readouts | Inhibitory units and outside the readouts | Plain on A's | Held within (027) | Excitatory |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 6 | +0.31, −0.49 | +10.40, −5.62 | +10.16, −6.59 | +10.37, −6.04 | +10.11, −8.37 | +10.41, −4.96 | +10.75, −0.12 |
| Seed 22, eighth, mirrored | 12 | +0.31, −0.14 | +8.50, −3.90 | +8.12, −4.74 | +8.47, −4.18 | +8.07, −5.67 | +8.52, −3.56 | +9.15, −0.09 |
| Seed 22, eighth, mirrored | 18 | +0.30, +0.07 | +7.30, −2.62 | +6.97, −3.45 | +7.32, −2.72 | +6.90, −3.55 | +7.36, −2.57 | +7.82, −0.10 |
| Seed 22, eighth, mirrored | 24 | +0.39, +0.37 | +6.21, −1.77 | +5.97, −2.36 | +6.28, −1.65 | +5.89, −2.20 | +6.25, −1.72 | +6.82, −0.03 |
| Seed 22, first mapping, assignment | 6 | −1.22, −0.51 | +9.29, −4.85 | +9.15, −5.77 | +9.30, −5.23 | +9.08, −6.90 | +9.32, −4.50 | +9.57, −0.10 |
| Seed 22, first mapping, assignment | 12 | −1.36, +0.11 | +6.37, −3.63 | +6.13, −4.29 | +6.37, −3.98 | +6.02, −5.36 | +6.42, −3.31 | +6.74, −0.07 |
| Seed 22, first mapping, assignment | 18 | −1.46, +0.45 | +4.37, −2.55 | +4.11, −3.16 | +4.33, −2.56 | +3.98, −3.43 | +4.39, −2.41 | +4.71, −0.07 |
| Seed 22, first mapping, assignment | 24 | −1.33, +0.34 | +2.97, −2.02 | +2.76, −2.50 | +2.86, −1.98 | +2.60, −2.62 | +2.97, −1.93 | +3.24, −0.06 |
| Seed 23, eighth, mirrored | 6 | +0.33, −0.69 | +10.96, −5.94 | +10.64, −7.29 | +10.90, −6.31 | +10.56, −8.75 | +10.96, −5.36 | +11.35, −0.16 |
| Seed 23, eighth, mirrored | 12 | +0.37, +0.12 | +8.92, −3.99 | +8.56, −5.05 | +8.89, −4.25 | +8.49, −5.70 | +8.94, −3.74 | +9.43, −0.12 |
| Seed 23, eighth, mirrored | 18 | +0.46, +0.83 | +7.72, −2.75 | +7.22, −3.76 | +7.86, −2.97 | +7.05, −4.18 | +7.86, −2.66 | +8.38, −0.06 |
| Seed 23, eighth, mirrored | 24 | +0.63, +1.49 | +6.45, −2.24 | +6.02, −2.66 | +6.48, −2.06 | +5.90, −2.89 | +6.45, −2.05 | +7.01, −0.06 |
| Seed 23, first mapping, assignment | 6 | −1.03, −0.46 | +9.91, −4.76 | +9.61, −5.95 | +9.86, −5.07 | +9.58, −6.79 | +9.90, −4.46 | +10.13, −0.11 |
| Seed 23, first mapping, assignment | 12 | −1.29, +0.47 | +6.71, −3.62 | +6.35, −4.66 | +6.72, −3.79 | +6.32, −5.31 | +6.74, −3.38 | +7.07, −0.10 |
| Seed 23, first mapping, assignment | 18 | −1.41, +0.76 | +4.06, −2.60 | +3.87, −3.25 | +4.06, −2.32 | +3.72, −3.20 | +4.09, −2.44 | +4.48, −0.06 |
| Seed 23, first mapping, assignment | 24 | −1.42, +1.16 | +2.85, −2.03 | +2.73, −2.55 | +2.89, −1.97 | +2.60, −2.37 | +2.92, −2.06 | +3.21, −0.10 |
| Seed 24, eighth, mirrored | 6 | +0.32, −0.58 | +9.70, −5.47 | +9.52, −6.59 | +9.69, −6.14 | +9.47, −8.12 | +9.73, −5.00 | +9.95, −0.14 |
| Seed 24, eighth, mirrored | 12 | +0.26, +0.18 | +8.26, −3.58 | +7.87, −4.44 | +8.27, −4.20 | +7.82, −5.64 | +8.30, −3.35 | +8.56, −0.09 |
| Seed 24, eighth, mirrored | 18 | +0.35, +0.31 | +7.08, −2.60 | +6.63, −3.38 | +7.03, −2.72 | +6.61, −3.88 | +7.07, −2.40 | +7.37, −0.05 |
| Seed 24, eighth, mirrored | 24 | +0.48, +0.38 | +6.07, −2.03 | +5.92, −2.52 | +6.11, −1.80 | +5.85, −2.71 | +6.17, −1.77 | +6.41, −0.02 |
| Seed 24, first mapping, assignment | 6 | −1.21, +0.12 | +8.92, −4.40 | +8.78, −5.29 | +8.91, −4.85 | +8.72, −6.29 | +8.93, −4.13 | +9.08, −0.11 |
| Seed 24, first mapping, assignment | 12 | −1.33, +0.45 | +6.12, −3.44 | +6.01, −4.04 | +6.06, −3.98 | +5.92, −5.07 | +6.14, −3.18 | +6.38, −0.07 |
| Seed 24, first mapping, assignment | 18 | −1.30, +0.67 | +4.18, −2.50 | +3.86, −3.20 | +4.12, −2.43 | +3.84, −3.53 | +4.18, −2.34 | +4.43, −0.06 |
| Seed 24, first mapping, assignment | 24 | −1.32, +0.77 | +2.89, −1.81 | +2.66, −2.34 | +2.87, −1.81 | +2.56, −2.48 | +2.89, −1.72 | +3.13, −0.06 |
| Seed 23, eighth, assignment | 6 | +0.36, −0.82 | +10.11, −5.59 | +9.75, −6.77 | +10.02, −7.01 | +9.72, −9.49 | +10.11, −4.87 | +10.37, −0.16 |
| Seed 23, eighth, assignment | 12 | +0.35, +0.09 | +8.37, −3.85 | +7.87, −4.90 | +8.31, −4.83 | +7.84, −6.46 | +8.44, −3.54 | +8.86, −0.10 |
| Seed 23, eighth, assignment | 18 | +0.45, +0.65 | +7.09, −2.78 | +6.69, −3.44 | +6.97, −3.08 | +6.47, −4.38 | +7.12, −2.43 | +7.63, −0.06 |
| Seed 23, eighth, assignment | 24 | +0.68, +1.52 | +6.07, −2.10 | +5.64, −2.49 | +6.01, −2.29 | +5.54, −2.94 | +6.08, −1.86 | +6.50, −0.05 |
| Seed 24, first mapping, mirrored | 6 | −1.28, +0.04 | +8.24, −4.70 | +8.05, −5.63 | +8.22, −5.90 | +7.94, −7.93 | +8.27, −4.14 | +8.49, −0.18 |
| Seed 24, first mapping, mirrored | 12 | −1.36, +0.43 | +5.34, −3.37 | +5.09, −4.22 | +5.28, −4.16 | +4.95, −5.78 | +5.41, −2.98 | +5.63, −0.12 |
| Seed 24, first mapping, mirrored | 18 | −1.30, +0.68 | +3.88, −2.58 | +3.70, −3.32 | +3.79, −2.84 | +3.55, −4.10 | +3.95, −2.30 | +4.10, −0.05 |
| Seed 24, first mapping, mirrored | 24 | −1.33, +0.69 | +3.05, −1.97 | +2.85, −2.56 | +2.96, −2.28 | +2.67, −3.14 | +3.11, −1.86 | +3.30, −0.05 |
| Seed 23, first mapping, mirrored | 6 | −1.19, −0.48 | +9.10, −4.59 | +8.90, −5.57 | +9.04, −5.90 | +8.85, −7.72 | +9.11, −4.13 | +9.31, −0.08 |
| Seed 23, first mapping, mirrored | 12 | −1.40, +0.37 | +6.12, −3.51 | +5.91, −4.39 | +6.05, −4.38 | +5.81, −5.84 | +6.11, −3.21 | +6.46, −0.07 |
| Seed 23, first mapping, mirrored | 18 | −1.43, +0.64 | +4.14, −2.67 | +3.89, −3.35 | +4.15, −2.83 | +3.79, −3.93 | +4.18, −2.42 | +4.35, −0.09 |
| Seed 23, first mapping, mirrored | 24 | −1.33, +1.02 | +3.02, −2.15 | +2.75, −2.56 | +2.95, −2.10 | +2.62, −2.89 | +3.03, −1.94 | +3.30, −0.07 |
| Seed 24, eighth, assignment | 6 | +0.32, −0.47 | +8.99, −5.64 | +8.88, −6.64 | +8.98, −6.80 | +8.80, −9.36 | +9.02, −4.82 | +9.29, −0.17 |
| Seed 24, eighth, assignment | 12 | +0.25, +0.10 | +7.79, −3.99 | +7.35, −4.99 | +7.65, −4.73 | +7.16, −6.68 | +7.81, −3.50 | +8.07, −0.08 |
| Seed 24, eighth, assignment | 18 | +0.32, +0.43 | +6.62, −2.77 | +6.20, −3.52 | +6.51, −3.11 | +5.92, −4.45 | +6.67, −2.48 | +6.97, −0.08 |
| Seed 24, eighth, assignment | 24 | +0.44, +0.34 | +5.74, −1.91 | +5.42, −2.65 | +5.61, −2.39 | +5.16, −3.38 | +5.81, −1.83 | +5.90, −0.05 |
| Seed 22, first mapping, mirrored | 6 | −1.32, −0.44 | +7.59, −4.22 | +7.48, −5.47 | +7.56, −5.92 | +7.36, −7.55 | +7.64, −4.02 | +7.80, −0.12 |
| Seed 22, first mapping, mirrored | 12 | −1.39, +0.09 | +5.11, −3.18 | +4.86, −4.17 | +4.94, −4.50 | +4.75, −5.75 | +5.12, −3.03 | +5.47, −0.12 |
| Seed 22, first mapping, mirrored | 18 | −1.39, +0.40 | +3.53, −2.42 | +3.27, −3.05 | +3.43, −2.87 | +3.21, −3.85 | +3.49, −2.23 | +3.71, −0.08 |
| Seed 22, first mapping, mirrored | 24 | −1.40, +0.37 | +2.49, −1.99 | +2.29, −2.43 | +2.43, −2.31 | +2.24, −2.96 | +2.52, −1.86 | +2.81, −0.09 |
| Seed 22, eighth, assignment | 6 | +0.28, −0.61 | +8.62, −5.06 | +8.41, −6.44 | +8.55, −7.03 | +8.35, −9.17 | +8.64, −4.66 | +8.83, −0.13 |
| Seed 22, eighth, assignment | 12 | +0.23, −0.11 | +7.18, −3.57 | +6.79, −4.77 | +7.06, −5.00 | +6.67, −6.30 | +7.15, −3.47 | +7.48, −0.12 |
| Seed 22, eighth, assignment | 18 | +0.24, −0.06 | +6.11, −2.66 | +5.79, −3.46 | +6.02, −3.31 | +5.74, −4.29 | +6.14, −2.58 | +6.53, −0.07 |
| Seed 22, eighth, assignment | 24 | +0.28, +0.23 | +5.33, −2.16 | +5.09, −2.49 | +5.19, −2.44 | +4.92, −3.19 | +5.35, −1.87 | +5.63, −0.03 |

- A night held onto the own group and the inhibitory units raised the inhibitory sum by 2.49 to 10.96 M and lowered the excitatory sum by
  1.77 to 5.94 M; a night held onto the own group and the units outside the readouts raised the first by 2.29 to 10.64 M and lowered the
  second by 2.34 to 7.29 M; and a night held onto the inhibitory units and the units outside the readouts raised the first by 2.43 to
  10.90 M and lowered the second by 1.65 to 7.03 M. H-47's plain night raised the first by 2.24 to 10.56 M and lowered the second by 2.20 to
  9.49 M, exploration 027's night held within raised the first by 2.52 to 10.96 M and lowered the second by 1.72 to 5.36 M, and the
  excitatory night raised the first by 2.81 to 11.35 M and lowered the second by 0.02 to 0.18 M.
- **The replays.** The watch read `wrong 0` in all 576 nights. Every night without replay read `replays 0` and `depotentiations 0`. Each
  held night read H-47's plain night's replays and depotentiations, as the plain night's class matrix job printed them, in 142 of its 144
  nights; the two others of each are on seed 23's eighth image from the mirrored assignment, on path 0, where each held night read 1 202
  replays and 571 depotentiations after block 18 and 1 129 and 545 after block 24, and the plain night 1 202 and 570 and 1 128 and 545:
  there they read exploration 027's night held within's counts and the excitatory night's. So each held night read exploration 027's night
  held within's counts in all 144 of its nights.

**The ties**, the tied trials of each cell's 240 reads of A and 240 of the pair, under each form:

| Cell | Without replay, A's and the pair's | H-47's without replay | Own group and inhibitory units | Own group and outside the readouts | Inhibitory units and outside the readouts | H-47's plain on A's | Held within (027) | Excitatory (025, 026) |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | 1160 and 382 | 1160 and 382 | 1274 and 277 | 1139 and 272 | 1208 and 317 | 1202 and 287 | 1260 and 300 | 1324 and 338 |
| Seed 22, first mapping, assignment | 1141 and 408 | 1141 and 408 | 1120 and 475 | 1114 and 482 | 1148 and 510 | 1183 and 491 | 1128 and 501 | 1213 and 472 |
| Seed 23, eighth, mirrored | 935 and 332 | 935 and 332 | 1077 and 251 | 1040 and 270 | 1071 and 232 | 1042 and 228 | 1076 and 240 | 1090 and 242 |
| Seed 23, first mapping, assignment | 1086 and 483 | 1086 and 483 | 794 and 639 | 757 and 665 | 731 and 611 | 742 and 618 | 785 and 699 | 883 and 629 |
| Seed 24, eighth, mirrored | 1137 and 322 | 1137 and 322 | 1452 and 155 | 1441 and 174 | 1491 and 145 | 1372 and 137 | 1426 and 164 | 1403 and 170 |
| Seed 24, first mapping, assignment | 968 and 520 | 968 and 520 | 796 and 594 | 804 and 642 | 780 and 566 | 835 and 584 | 711 and 642 | 857 and 588 |
| Seed 23, eighth, assignment | 1049 and 256 | 1049 and 256 | 971 and 219 | 1079 and 213 | 926 and 245 | 1039 and 213 | 854 and 246 | 907 and 251 |
| Seed 24, first mapping, mirrored | 1066 and 315 | 1066 and 315 | 921 and 392 | 828 and 394 | 944 and 322 | 887 and 332 | 910 and 374 | 835 and 421 |
| Seed 23, first mapping, mirrored | 792 and 293 | 792 and 293 | 878 and 372 | 869 and 349 | 780 and 374 | 866 and 365 | 844 and 365 | 826 and 360 |
| Seed 24, eighth, assignment | 1114 and 362 | 1114 and 362 | 1395 and 175 | 1403 and 181 | 1344 and 160 | 1324 and 174 | 1391 and 162 | 1391 and 183 |
| Seed 22, first mapping, mirrored | 943 and 302 | 943 and 302 | 847 and 374 | 858 and 336 | 840 and 388 | 878 and 357 | 833 and 336 | 862 and 330 |
| Seed 22, eighth, assignment | 917 and 207 | 917 and 207 | 910 and 126 | 889 and 140 | 830 and 161 | 898 and 130 | 829 and 175 | 873 and 124 |

The night without replay's ties are H-47's, read by read. From H-47's plain night's, the night held onto the own group and the inhibitory
units' ties lie 12 to 80 away on A's reads and 1 to 60 on the pair's, the night held onto the own group and the units outside the readouts'
2 to 79 and 0 to 62, and the night held onto the inhibitory units and the units outside the readouts' 6 to 119 and 4 to 32. From
exploration 027's night held within's they lie 1 to 117 and 7 to 60, 12 to 225 and 0 to 35, and 1 to 72 and 1 to 88.

## 5. What the hold leaves

At each held night, over its twelve nights a cell and form (ranges are over the twelve): the synapses of the two held parts; their traces
cleared at the night's last tick, how many were not zero and what they held above and below zero; the excitatory synapses from units
outside the hold the night moved; the nights in which a held synapse's presynaptic unit fired at or after the night's last tick, and the
nights in which the quiet ticks moved synapses of the two held parts:

| Cell | Form | Nights | Held synapses | Traces cleared, not zero | cleared above zero, LSB | below zero | Excitatory synapses from units outside the hold moved | Nights a held synapse's presynaptic unit fired at or after the night's last tick | Nights held synapses moved in the quiet ticks: how many, their change |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | Own group and inhibitory units | 12 | 11 564 | 7 468 to 7 679 | 477 547 to 589 529 | −770 081 to −613 866 | 4 939 to 4 985 | 0 | none |
| Seed 22, eighth, mirrored | Own group and outside the readouts | 12 | 9 369 | 5 860 to 6 090 | 449 110 to 544 349 | −631 840 to −544 073 | 4 944 to 4 984 | 0 | none |
| Seed 22, eighth, mirrored | Inhibitory units and outside the readouts | 12 | 6 873 | 4 429 to 4 622 | 267 275 to 296 608 | −474 617 to −375 255 | 4 948 to 4 987 | 0 | none |
| Seed 22, first mapping, assignment | Own group and inhibitory units | 12 | 11 564 | 7 062 to 7 303 | 472 087 to 558 957 | −668 401 to −547 825 | 4 948 to 4 991 | 1 | none |
| Seed 22, first mapping, assignment | Own group and outside the readouts | 12 | 9 369 | 5 719 to 5 943 | 448 475 to 554 094 | −611 634 to −478 033 | 4 952 to 4 990 | 0 | none |
| Seed 22, first mapping, assignment | Inhibitory units and outside the readouts | 12 | 6 873 | 4 205 to 4 303 | 233 627 to 267 211 | −417 872 to −318 225 | 4 954 to 4 992 | 1 | none |
| Seed 23, eighth, mirrored | Own group and inhibitory units | 12 | 11 406 | 7 198 to 7 529 | 441 888 to 542 139 | −791 644 to −632 219 | 4 949 to 4 982 | 3 | none |
| Seed 23, eighth, mirrored | Own group and outside the readouts | 12 | 9 285 | 5 846 to 6 134 | 413 997 to 527 298 | −683 048 to −560 766 | 4 947 to 4 983 | 4 | none |
| Seed 23, eighth, mirrored | Inhibitory units and outside the readouts | 12 | 6 853 | 4 305 to 4 528 | 236 995 to 272 035 | −488 588 to −367 280 | 4 953 to 4 984 | 3 | none |
| Seed 23, first mapping, assignment | Own group and inhibitory units | 12 | 11 406 | 7 132 to 7 414 | 464 996 to 544 276 | −725 564 to −580 625 | 4 971 to 4 989 | 0 | none |
| Seed 23, first mapping, assignment | Own group and outside the readouts | 12 | 9 285 | 5 787 to 5 990 | 452 046 to 532 556 | −617 297 to −499 121 | 4 963 to 4 988 | 0 | none |
| Seed 23, first mapping, assignment | Inhibitory units and outside the readouts | 12 | 6 853 | 4 212 to 4 435 | 232 745 to 276 052 | −418 110 to −328 805 | 4 958 to 4 989 | 3 | none |
| Seed 24, eighth, mirrored | Own group and inhibitory units | 12 | 11 434 | 7 109 to 7 420 | 480 507 to 542 774 | −723 410 to −647 887 | 4 970 to 4 987 | 0 | none |
| Seed 24, eighth, mirrored | Own group and outside the readouts | 12 | 9 252 | 5 785 to 5 912 | 448 156 to 506 441 | −605 674 to −540 740 | 4 962 to 4 985 | 0 | none |
| Seed 24, eighth, mirrored | Inhibitory units and outside the readouts | 12 | 6 904 | 4 231 to 4 478 | 257 965 to 307 040 | −426 067 to −365 886 | 4 963 to 4 985 | 0 | none |
| Seed 24, first mapping, assignment | Own group and inhibitory units | 12 | 11 434 | 6 928 to 7 299 | 422 784 to 598 223 | −718 760 to −604 933 | 4 972 to 4 988 | 0 | none |
| Seed 24, first mapping, assignment | Own group and outside the readouts | 12 | 9 252 | 5 642 to 5 920 | 402 907 to 563 179 | −608 714 to −503 918 | 4 972 to 4 990 | 0 | none |
| Seed 24, first mapping, assignment | Inhibitory units and outside the readouts | 12 | 6 904 | 4 173 to 4 371 | 234 987 to 282 913 | −413 369 to −329 153 | 4 975 to 4 987 | 0 | none |
| Seed 23, eighth, assignment | Own group and inhibitory units | 12 | 11 406 | 7 048 to 7 307 | 354 408 to 439 639 | −712 915 to −589 130 | 4 961 to 4 987 | 3 | none |
| Seed 23, eighth, assignment | Own group and outside the readouts | 12 | 9 285 | 5 592 to 5 770 | 321 944 to 401 249 | −606 235 to −489 821 | 4 960 to 4 990 | 3 | none |
| Seed 23, eighth, assignment | Inhibitory units and outside the readouts | 12 | 6 853 | 4 277 to 4 469 | 203 274 to 258 961 | −441 785 to −356 633 | 4 950 to 4 990 | 3 | none |
| Seed 24, first mapping, mirrored | Own group and inhibitory units | 12 | 11 434 | 6 860 to 7 072 | 360 128 to 446 330 | −647 820 to −544 127 | 4 961 to 4 988 | 0 | none |
| Seed 24, first mapping, mirrored | Own group and outside the readouts | 12 | 9 252 | 5 578 to 5 649 | 355 815 to 424 020 | −535 831 to −445 466 | 4 966 to 4 989 | 0 | none |
| Seed 24, first mapping, mirrored | Inhibitory units and outside the readouts | 12 | 6 904 | 4 263 to 4 410 | 226 682 to 255 226 | −425 720 to −306 006 | 4 954 to 4 987 | 0 | none |
| Seed 23, first mapping, mirrored | Own group and inhibitory units | 12 | 11 406 | 6 919 to 7 057 | 352 325 to 426 325 | −677 770 to −517 037 | 4 962 to 4 991 | 0 | none |
| Seed 23, first mapping, mirrored | Own group and outside the readouts | 12 | 9 285 | 5 505 to 5 734 | 331 653 to 400 338 | −554 229 to −452 533 | 4 962 to 4 985 | 0 | none |
| Seed 23, first mapping, mirrored | Inhibitory units and outside the readouts | 12 | 6 853 | 4 180 to 4 331 | 198 280 to 249 236 | −426 190 to −299 922 | 4 959 to 4 986 | 3 | none |
| Seed 24, eighth, assignment | Own group and inhibitory units | 12 | 11 434 | 7 052 to 7 241 | 401 484 to 433 808 | −694 323 to −572 817 | 4 947 to 4 989 | 0 | none |
| Seed 24, eighth, assignment | Own group and outside the readouts | 12 | 9 252 | 5 641 to 5 881 | 354 789 to 423 006 | −566 122 to −502 878 | 4 948 to 4 988 | 0 | none |
| Seed 24, eighth, assignment | Inhibitory units and outside the readouts | 12 | 6 904 | 4 301 to 4 557 | 236 287 to 275 228 | −443 366 to −354 780 | 4 950 to 4 985 | 0 | none |
| Seed 22, first mapping, mirrored | Own group and inhibitory units | 12 | 11 564 | 6 779 to 7 046 | 360 740 to 429 401 | −612 004 to −484 556 | 4 947 to 4 986 | 3 | none |
| Seed 22, first mapping, mirrored | Own group and outside the readouts | 12 | 9 369 | 5 497 to 5 680 | 335 032 to 423 111 | −504 724 to −422 450 | 4 948 to 4 987 | 3 | none |
| Seed 22, first mapping, mirrored | Inhibitory units and outside the readouts | 12 | 6 873 | 4 108 to 4 310 | 227 146 to 275 828 | −392 829 to −274 735 | 4 952 to 4 987 | 3 | none |
| Seed 22, eighth, assignment | Own group and inhibitory units | 12 | 11 564 | 7 031 to 7 316 | 374 908 to 465 324 | −690 656 to −568 549 | 4 947 to 4 990 | 3 | none |
| Seed 22, eighth, assignment | Own group and outside the readouts | 12 | 9 369 | 5 546 to 5 899 | 353 669 to 419 409 | −618 894 to −479 128 | 4 953 to 4 990 | 3 | none |
| Seed 22, eighth, assignment | Inhibitory units and outside the readouts | 12 | 6 873 | 4 268 to 4 357 | 237 549 to 294 952 | −413 041 to −317 582 | 4 955 to 4 986 | 3 | none |

The part left free's synapses and the part held across's that the night moved, and those of them that carried a trace that is not zero at
its last tick:

| Cell | Form | Onto the own group: synapses moved at the night's last tick, of; with a trace | Onto the inhibitory units | Onto the units outside the readouts | Held across |
| :--- | :--- | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | Own group and inhibitory units | held | held | 2 316 to 2 338 of 2 339; 1 181 to 1 308 | 7 237 to 7 332 of 7 345; 3 089 to 3 545 |
| Seed 22, eighth, mirrored | Own group and outside the readouts | held | 4 398 to 4 528 of 4 534; 2 400 to 2 660 | held | 7 238 to 7 332 of 7 345; 3 105 to 3 555 |
| Seed 22, eighth, mirrored | Inhibitory units and outside the readouts | 6 851 to 7 017 of 7 030; 3 581 to 3 949 | held | held | 7 221 to 7 333 of 7 345; 3 089 to 3 613 |
| Seed 22, first mapping, assignment | Own group and inhibitory units | held | held | 2 312 to 2 337 of 2 339; 1 117 to 1 244 | 7 239 to 7 333 of 7 345; 2 816 to 3 174 |
| Seed 22, first mapping, assignment | Own group and outside the readouts | held | 4 431 to 4 520 of 4 534; 2 209 to 2 416 | held | 7 260 to 7 333 of 7 345; 2 926 to 3 151 |
| Seed 22, first mapping, assignment | Inhibitory units and outside the readouts | 6 853 to 7 010 of 7 030; 3 359 to 3 665 | held | held | 7 244 to 7 334 of 7 345; 2 862 to 3 192 |
| Seed 23, eighth, mirrored | Own group and inhibitory units | held | held | 2 344 to 2 363 of 2 366; 1 183 to 1 315 | 7 371 to 7 466 of 7 476; 3 099 to 3 616 |
| Seed 23, eighth, mirrored | Own group and outside the readouts | held | 4 398 to 4 484 of 4 487; 2 333 to 2 548 | held | 7 372 to 7 466 of 7 476; 3 156 to 3 666 |
| Seed 23, eighth, mirrored | Inhibitory units and outside the readouts | 6 757 to 6 911 of 6 919; 3 590 to 4 000 | held | held | 7 348 to 7 462 of 7 476; 3 182 to 3 699 |
| Seed 23, first mapping, assignment | Own group and inhibitory units | held | held | 2 340 to 2 366 of 2 366; 1 196 to 1 274 | 7 393 to 7 465 of 7 476; 2 940 to 3 286 |
| Seed 23, first mapping, assignment | Own group and outside the readouts | held | 4 407 to 4 482 of 4 487; 2 173 to 2 458 | held | 7 390 to 7 463 of 7 476; 2 970 to 3 322 |
| Seed 23, first mapping, assignment | Inhibitory units and outside the readouts | 6 708 to 6 905 of 6 919; 3 540 to 3 775 | held | held | 7 400 to 7 462 of 7 476; 2 967 to 3 324 |
| Seed 24, eighth, mirrored | Own group and inhibitory units | held | held | 2 333 to 2 360 of 2 361; 1 144 to 1 286 | 7 332 to 7 441 of 7 453; 3 250 to 3 390 |
| Seed 24, eighth, mirrored | Own group and outside the readouts | held | 4 433 to 4 540 of 4 543; 2 349 to 2 552 | held | 7 336 to 7 439 of 7 453; 3 262 to 3 417 |
| Seed 24, eighth, mirrored | Inhibitory units and outside the readouts | 6 692 to 6 885 of 6 891; 3 435 to 3 678 | held | held | 7 323 to 7 442 of 7 453; 3 266 to 3 425 |
| Seed 24, first mapping, assignment | Own group and inhibitory units | held | held | 2 344 to 2 361 of 2 361; 1 188 to 1 256 | 7 333 to 7 441 of 7 453; 2 960 to 3 395 |
| Seed 24, first mapping, assignment | Own group and outside the readouts | held | 4 474 to 4 533 of 4 543; 2 223 to 2 445 | held | 7 340 to 7 441 of 7 453; 3 031 to 3 402 |
| Seed 24, first mapping, assignment | Inhibitory units and outside the readouts | 6 693 to 6 872 of 6 891; 3 326 to 3 828 | held | held | 7 308 to 7 440 of 7 453; 2 919 to 3 386 |
| Seed 23, eighth, assignment | Own group and inhibitory units | held | held | 2 342 to 2 363 of 2 366; 1 102 to 1 262 | 7 393 to 7 458 of 7 476; 3 018 to 3 617 |
| Seed 23, eighth, assignment | Own group and outside the readouts | held | 4 401 to 4 484 of 4 487; 2 358 to 2 632 | held | 7 399 to 7 460 of 7 476; 3 185 to 3 549 |
| Seed 23, eighth, assignment | Inhibitory units and outside the readouts | 6 704 to 6 913 of 6 919; 3 108 to 3 630 | held | held | 7 384 to 7 463 of 7 476; 3 091 to 3 575 |
| Seed 24, first mapping, mirrored | Own group and inhibitory units | held | held | 2 335 to 2 360 of 2 361; 1 114 to 1 281 | 7 372 to 7 443 of 7 453; 2 972 to 3 331 |
| Seed 24, first mapping, mirrored | Own group and outside the readouts | held | 4 427 to 4 532 of 4 543; 2 163 to 2 405 | held | 7 369 to 7 444 of 7 453; 2 952 to 3 363 |
| Seed 24, first mapping, mirrored | Inhibitory units and outside the readouts | 6 695 to 6 880 of 6 891; 3 084 to 3 361 | held | held | 7 379 to 7 444 of 7 453; 2 962 to 3 357 |
| Seed 23, first mapping, mirrored | Own group and inhibitory units | held | held | 2 354 to 2 364 of 2 366; 1 083 to 1 250 | 7 420 to 7 462 of 7 476; 3 047 to 3 427 |
| Seed 23, first mapping, mirrored | Own group and outside the readouts | held | 4 419 to 4 484 of 4 487; 2 197 to 2 444 | held | 7 425 to 7 461 of 7 476; 3 031 to 3 425 |
| Seed 23, first mapping, mirrored | Inhibitory units and outside the readouts | 6 720 to 6 906 of 6 919; 3 143 to 3 429 | held | held | 7 415 to 7 459 of 7 476; 3 016 to 3 462 |
| Seed 24, eighth, assignment | Own group and inhibitory units | held | held | 2 338 to 2 358 of 2 361; 1 182 to 1 289 | 7 345 to 7 441 of 7 453; 3 126 to 3 605 |
| Seed 24, eighth, assignment | Own group and outside the readouts | held | 4 437 to 4 534 of 4 543; 2 275 to 2 585 | held | 7 363 to 7 440 of 7 453; 3 160 to 3 641 |
| Seed 24, eighth, assignment | Inhibitory units and outside the readouts | 6 702 to 6 882 of 6 891; 3 209 to 3 549 | held | held | 7 364 to 7 444 of 7 453; 3 127 to 3 638 |
| Seed 22, first mapping, mirrored | Own group and inhibitory units | held | held | 2 310 to 2 338 of 2 339; 1 063 to 1 190 | 7 268 to 7 331 of 7 345; 2 735 to 3 114 |
| Seed 22, first mapping, mirrored | Own group and outside the readouts | held | 4 461 to 4 527 of 4 534; 2 179 to 2 393 | held | 7 272 to 7 331 of 7 345; 2 789 to 3 156 |
| Seed 22, first mapping, mirrored | Inhibitory units and outside the readouts | 6 829 to 7 014 of 7 030; 3 037 to 3 405 | held | held | 7 274 to 7 330 of 7 345; 2 797 to 3 112 |
| Seed 22, eighth, assignment | Own group and inhibitory units | held | held | 2 313 to 2 338 of 2 339; 1 104 to 1 191 | 7 243 to 7 337 of 7 345; 2 850 to 3 392 |
| Seed 22, eighth, assignment | Own group and outside the readouts | held | 4 423 to 4 528 of 4 534; 2 323 to 2 543 | held | 7 245 to 7 335 of 7 345; 2 973 to 3 441 |
| Seed 22, eighth, assignment | Inhibitory units and outside the readouts | 6 785 to 7 018 of 7 030; 3 219 to 3 527 | held | held | 7 252 to 7 336 of 7 345; 2 878 to 3 419 |

- **The hold cleared**, at the night's last tick, a trace that was not zero on 6 779 to 7 679 of the 11 406 to 11 564 synapses held onto the
  own group and the inhibitory units, 352 325 to 598 223 LSB above zero and −791 644 to −484 556 below; on 5 497 to 6 134 of the 9 252 to
  9 369 held onto the own group and the units outside the readouts, 321 944 to 563 179 LSB above and −683 048 to −422 450 below; and on
  4 108 to 4 622 of the 6 853 to 6 904 held onto the inhibitory units and the units outside the readouts, 198 280 to 307 040 LSB above and
  −488 588 to −274 735 below.
- **The part left free moved**, at the night's last tick, on 2 310 to 2 366 of the 2 339 to 2 366 synapses onto the units outside the
  readouts under the night held onto the own group and the inhibitory units, 1 063 to 1 315 of them with a trace; on 4 398 to 4 540 of the
  4 487 to 4 543 onto the inhibitory units under the night held onto the own group and the units outside the readouts, 2 163 to 2 660 with
  a trace; and on 6 692 to 7 018 of the 6 891 to 7 030 onto the own group under the night held onto the inhibitory units and the units
  outside the readouts, 3 037 to 4 000 with a trace. These are counts: the jobs print no trace itself.
- **A held synapse's presynaptic unit fired at or after the night's last tick** in 13 of the 144 nights held onto the own group and the
  inhibitory units: 1 on seed 22's first mapping from the assignment and 3 on each of seed 23's eighth image from the mirrored assignment,
  seed 23's eighth image from the assignment, seed 22's first mapping from the mirrored assignment and seed 22's eighth image from the
  assignment. In 13 of the 144 held onto the own group and the units outside the readouts: 4 on seed 23's eighth image from the mirrored
  assignment and 3 on each of seed 23's eighth image from the assignment, seed 22's first mapping from the mirrored assignment and seed 22's
  eighth image from the assignment. In 19 of the 144 held onto the inhibitory units and the units outside the readouts: 1 on seed 22's
  first mapping from the assignment and 3 on each of seed 23's eighth image from the mirrored assignment, seed 23's first mapping from the
  assignment, seed 23's eighth image from the assignment, seed 23's first mapping from the mirrored assignment, seed 22's first mapping from
  the mirrored assignment and seed 22's eighth image from the assignment.
- **The quiet ticks moved no synapse of the two held parts in any of the 432 held nights**, and no synapse of the part left free or of the
  part held across either. After them, in the nights that hold it, 0 to 9 held synapses carried a trace under the night held onto the own
  group and the inhibitory units, 0 to 7 under the night held onto the own group and the units outside the readouts and 0 to 8 under the
  night held onto the inhibitory units and the units outside the readouts. As many synapses of the part left free carried a trace after the
  quiet ticks as at the night's last tick in 134, 140 and 131 of the 144 nights of each held form, in the same order, and of the part held
  across in 134, 134 and 132. The jobs print these counts, not the traces themselves.

The night's pairings still to be read on the two held parts' synapses, as What ran defines them, each read at the synapse's presynaptic
unit's next spike if that spike comes before the target's next spike:

| Cell | Form | Pending gains: held synapses | their sum, LSB | their largest | Pending losses with a presynaptic spike at the next tick: held synapses | their sum, LSB | their largest |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | Own group and inhibitory units | 6 150 to 6 287 | 315 592 to 418 997 | 328 | 11 564 | 30 829 to 42 808 | 104 to 126 |
| Seed 22, eighth, mirrored | Own group and outside the readouts | 4 523 to 4 702 | 212 714 to 274 729 | 328 | 9 369 | 20 340 to 32 579 | 98 to 124 |
| Seed 22, eighth, mirrored | Inhibitory units and outside the readouts | 3 698 to 3 917 | 212 346 to 295 111 | 328 | 6 873 | 18 603 to 26 662 | 89 to 126 |
| Seed 22, first mapping, assignment | Own group and inhibitory units | 6 003 to 6 452 | 310 674 to 357 211 | 328 | 11 564 | 26 815 to 41 842 | 98 to 150 |
| Seed 22, first mapping, assignment | Own group and outside the readouts | 4 557 to 4 879 | 192 881 to 243 990 | 328 | 9 369 | 21 838 to 29 223 | 101 to 109 |
| Seed 22, first mapping, assignment | Inhibitory units and outside the readouts | 3 592 to 4 083 | 193 786 to 255 737 | 328 | 6 873 | 14 581 to 24 795 | 53 to 150 |
| Seed 23, eighth, mirrored | Own group and inhibitory units | 5 808 to 6 162 | 359 506 to 402 453 | 328 | 11 406 | 27 683 to 45 579 | 105 to 125 |
| Seed 23, eighth, mirrored | Own group and outside the readouts | 4 477 to 4 706 | 224 068 to 283 060 | 328 | 9 285 | 22 538 to 34 807 | 105 to 150 |
| Seed 23, eighth, mirrored | Inhibitory units and outside the readouts | 3 364 to 3 817 | 211 130 to 263 489 | 328 | 6 853 | 18 998 to 29 003 | 89 to 113 |
| Seed 23, first mapping, assignment | Own group and inhibitory units | 5 880 to 6 180 | 285 797 to 411 159 | 328 | 11 406 | 31 653 to 40 532 | 105 to 115 |
| Seed 23, first mapping, assignment | Own group and outside the readouts | 4 457 to 4 680 | 208 945 to 277 458 | 328 | 9 285 | 20 067 to 31 390 | 105 to 125 |
| Seed 23, first mapping, assignment | Inhibitory units and outside the readouts | 3 576 to 3 854 | 194 926 to 267 935 | 328 | 6 853 | 16 198 to 28 107 | 95 to 115 |
| Seed 24, eighth, mirrored | Own group and inhibitory units | 5 905 to 6 175 | 345 866 to 407 971 | 328 | 11 434 | 34 738 to 54 571 | 106 to 143 |
| Seed 24, eighth, mirrored | Own group and outside the readouts | 4 454 to 4 660 | 217 835 to 277 658 | 328 | 9 252 | 25 742 to 37 160 | 106 to 130 |
| Seed 24, eighth, mirrored | Inhibitory units and outside the readouts | 3 672 to 3 840 | 240 445 to 289 118 | 328 | 6 904 | 17 428 to 34 175 | 77 to 143 |
| Seed 24, first mapping, assignment | Own group and inhibitory units | 5 856 to 6 251 | 310 927 to 399 316 | 328 | 11 434 | 25 788 to 38 269 | 95 to 132 |
| Seed 24, first mapping, assignment | Own group and outside the readouts | 4 405 to 4 632 | 184 072 to 266 802 | 328 | 9 252 | 21 932 to 30 182 | 103 to 132 |
| Seed 24, first mapping, assignment | Inhibitory units and outside the readouts | 3 656 to 3 890 | 213 648 to 279 797 | 328 | 6 904 | 15 880 to 25 666 | 95 to 125 |
| Seed 23, eighth, assignment | Own group and inhibitory units | 6 066 to 6 303 | 327 609 to 392 467 | 328 | 11 406 | 31 047 to 46 430 | 106 to 149 |
| Seed 23, eighth, assignment | Own group and outside the readouts | 4 515 to 4 738 | 209 259 to 251 444 | 328 | 9 285 | 23 324 to 33 958 | 78 to 146 |
| Seed 23, eighth, assignment | Inhibitory units and outside the readouts | 3 611 to 3 937 | 216 474 to 246 709 | 328 | 6 853 | 16 437 to 29 125 | 86 to 119 |
| Seed 24, first mapping, mirrored | Own group and inhibitory units | 5 955 to 6 126 | 261 545 to 321 147 | 328 | 11 434 | 22 677 to 40 319 | 103 to 132 |
| Seed 24, first mapping, mirrored | Own group and outside the readouts | 4 603 to 4 696 | 160 478 to 223 595 | 328 | 9 252 | 19 802 to 37 695 | 103 to 132 |
| Seed 24, first mapping, mirrored | Inhibitory units and outside the readouts | 3 718 to 3 977 | 192 939 to 241 077 | 328 | 6 904 | 13 088 to 29 927 | 95 to 125 |
| Seed 23, first mapping, mirrored | Own group and inhibitory units | 5 847 to 6 308 | 267 363 to 358 525 | 328 | 11 406 | 29 961 to 39 825 | 98 to 116 |
| Seed 23, first mapping, mirrored | Own group and outside the readouts | 4 607 to 4 731 | 199 065 to 236 163 | 328 | 9 285 | 24 233 to 30 881 | 105 to 121 |
| Seed 23, first mapping, mirrored | Inhibitory units and outside the readouts | 3 563 to 3 860 | 171 119 to 238 796 | 328 | 6 853 | 18 715 to 30 250 | 102 to 121 |
| Seed 24, eighth, assignment | Own group and inhibitory units | 6 056 to 6 275 | 293 317 to 346 212 | 328 | 11 434 | 25 767 to 44 125 | 106 to 142 |
| Seed 24, eighth, assignment | Own group and outside the readouts | 4 599 to 4 681 | 189 768 to 239 185 | 328 | 9 252 | 19 908 to 28 805 | 98 to 113 |
| Seed 24, eighth, assignment | Inhibitory units and outside the readouts | 3 697 to 4 013 | 199 838 to 261 644 | 328 | 6 904 | 18 671 to 31 058 | 100 to 142 |
| Seed 22, first mapping, mirrored | Own group and inhibitory units | 6 226 to 6 587 | 271 229 to 321 301 | 328 | 11 564 | 32 149 to 53 775 | 90 to 149 |
| Seed 22, first mapping, mirrored | Own group and outside the readouts | 4 651 to 4 838 | 183 136 to 221 040 | 328 | 9 369 | 22 094 to 35 696 | 101 to 111 |
| Seed 22, first mapping, mirrored | Inhibitory units and outside the readouts | 3 855 to 4 121 | 178 961 to 224 698 | 328 | 6 873 | 16 711 to 35 535 | 56 to 149 |
| Seed 22, eighth, assignment | Own group and inhibitory units | 6 185 to 6 402 | 296 959 to 344 819 | 328 | 11 564 | 31 413 to 47 412 | 107 to 129 |
| Seed 22, eighth, assignment | Own group and outside the readouts | 4 589 to 4 721 | 193 181 to 227 823 | 328 | 9 369 | 20 236 to 32 092 | 104 to 123 |
| Seed 22, eighth, assignment | Inhibitory units and outside the readouts | 3 683 to 4 035 | 199 809 to 237 927 | 328 | 6 873 | 18 865 to 33 090 | 104 to 120 |

- **The pending gain**, the target's spike after the presynaptic unit's last, is on 5 808 to 6 587 synapses held onto the own group and
  the inhibitory units, 261 545 to 418 997 LSB together; on 4 405 to 4 879 held onto the own group and the units outside the readouts,
  160 478 to 283 060 LSB together; and on 3 364 to 4 121 held onto the inhibitory units and the units outside the readouts, 171 119 to
  295 111 LSB together. The largest is 328 LSB, which is $A_+$, in every night of each.
- **The pending loss** with a presynaptic spike at the first tick after the image is on every synapse of the two held parts: 22 677 to
  54 571 LSB together under the night held onto the own group and the inhibitory units, the largest 90 to 150 LSB; 19 802 to 37 695 LSB
  under the night held onto the own group and the units outside the readouts, the largest 78 to 150; and 13 088 to 35 535 LSB under the
  night held onto the inhibitory units and the units outside the readouts, the largest 53 to 150. A later spike, before the target's next
  one, reads no more.
- **Against what the hold cleared.** These pairings were never in the trace the hold cleared, which held, synapse by synapse, the decayed
  net of what the trace carried into the night and the pairings read at the night's presynaptic spikes. Set beside it as sizes alone: the
  pending gains are 54.9 % to 101.5 % the size of the traces cleared above zero under the night held onto the own group and the inhibitory
  units, 40.6 % to 66.1 % under the night held onto the own group and the units outside the readouts and 67.7 % to 113.0 % under the night
  held onto the inhibitory units and the units outside the readouts; the most the pending losses can be is 3.9 % to 11.0 %, 3.3 % to 8.2 %
  and 3.5 % to 12.8 % the size of the traces cleared below zero.
- These are trace, not weight: at the next presynaptic spike, in the next day's engine, each synapse's trace takes its pairings, and its
  consolidation moves the weight under that day's modulation. That consolidation is not read here.

## 6. The twelve cells at block 36

H-47's twelve cells at block 36, the rows ordered by the readout A answers, the six where it answers readout 0 first. Every held form is
this exploration's job; exploration 027's night held within is exploration 027's job; the excitatory night is exploration 026's job for its
eight cells and exploration 025's for exploration 020's four; each part held alone is exploration 028's job; H-47's plain night is H-47's
job's in all twelve. "Less without replay" and the held forms less each other carry the paired SE of this job, and "less H-47's plain",
"less 027's held within", "less excitatory" and "less … alone (028)" exploration 018's unpaired SE. No figure is pooled across cells:

| Cell | A answers | Own group and inhibitory units less without replay | Own group and outside the readouts less without replay | Inhibitory units and outside the readouts less without replay | H-47: plain less without replay | 027: held within less without replay | Excitatory less without replay |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | +15.4 ± 1.01 | +18.5 ± 0.77 | +17.8 ± 1.23 | +19.6 ± 1.06 | +16.8 ± 0.83 | +16.3 ± 0.84 |
| Seed 22, first mapping, assignment | readout 0, the odd places | +6.4 ± 1.01 | +9.9 ± 1.13 | +7.0 ± 0.80 | +7.9 ± 0.51 | +6.5 ± 1.42 | +5.2 ± 1.24 |
| Seed 23, eighth, mirrored | readout 0, the odd places | +15.0 ± 0.99 | +16.6 ± 1.20 | +16.5 ± 1.63 | +17.4 ± 1.08 | +15.5 ± 1.06 | +14.7 ± 1.54 |
| Seed 23, first mapping, assignment | readout 0, the odd places | +12.8 ± 1.46 | +13.1 ± 1.81 | +13.2 ± 1.45 | +13.4 ± 1.56 | +13.3 ± 1.64 | +12.6 ± 1.56 |
| Seed 24, eighth, mirrored | readout 0, the odd places | +14.9 ± 0.99 | +13.3 ± 2.62 | +16.4 ± 1.39 | +11.3 ± 0.96 | +15.5 ± 1.27 | +16.7 ± 1.23 |
| Seed 24, first mapping, assignment | readout 0, the odd places | +5.2 ± 1.65 | +5.2 ± 0.59 | +6.1 ± 1.12 | +3.8 ± 1.57 | +6.1 ± 1.46 | +5.8 ± 1.60 |
| Seed 23, eighth, assignment | readout 1, the even places | +7.4 ± 1.46 | +6.3 ± 1.84 | +9.3 ± 1.35 | +4.1 ± 1.45 | +10.2 ± 1.77 | +10.9 ± 2.49 |
| Seed 24, first mapping, mirrored | readout 1, the even places | +6.2 ± 1.66 | +7.9 ± 1.69 | +7.1 ± 1.23 | +6.0 ± 1.54 | +7.7 ± 1.37 | +7.4 ± 1.09 |
| Seed 23, first mapping, mirrored | readout 1, the even places | −2.9 ± 0.96 | −4.0 ± 1.10 | −2.7 ± 0.53 | −4.9 ± 1.09 | −2.6 ± 0.65 | −1.7 ± 0.42 |
| Seed 24, eighth, assignment | readout 1, the even places | +1.0 ± 0.72 | +1.6 ± 1.09 | +5.6 ± 1.68 | +1.8 ± 1.79 | +4.1 ± 1.17 | +8.8 ± 1.28 |
| Seed 22, first mapping, mirrored | readout 1, the even places | +5.1 ± 1.72 | +5.7 ± 1.53 | +6.2 ± 1.66 | +5.0 ± 1.24 | +5.6 ± 1.68 | +4.7 ± 1.52 |
| Seed 22, eighth, assignment | readout 1, the even places | +1.7 ± 1.43 | +4.0 ± 0.95 | +4.1 ± 1.19 | +2.2 ± 0.65 | +4.5 ± 1.26 | +4.6 ± 0.66 |

| Cell | A answers | Own group and inhibitory units less own group and outside | Own group and inhibitory units less inhibitory units and outside | Own group and outside less inhibitory units and outside |
| :--- | :--- | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | −3.2 ± 0.60 | −2.5 ± 0.85 | +0.7 ± 1.14 |
| Seed 22, first mapping, assignment | readout 0, the odd places | −3.6 ± 1.07 | −0.6 ± 0.36 | +3.0 ± 0.75 |
| Seed 23, eighth, mirrored | readout 0, the odd places | −1.6 ± 0.42 | −1.5 ± 0.76 | +0.1 ± 0.48 |
| Seed 23, first mapping, assignment | readout 0, the odd places | −0.4 ± 0.42 | −0.4 ± 0.29 | −0.1 ± 0.49 |
| Seed 24, eighth, mirrored | readout 0, the odd places | +1.7 ± 1.88 | −1.5 ± 0.76 | −3.2 ± 1.33 |
| Seed 24, first mapping, assignment | readout 0, the odd places | 0.0 ± 1.19 | −1.0 ± 1.21 | −1.0 ± 0.51 |
| Seed 23, eighth, assignment | readout 1, the even places | +1.1 ± 0.65 | −1.9 ± 0.34 | −3.0 ± 0.78 |
| Seed 24, first mapping, mirrored | readout 1, the even places | −1.8 ± 0.16 | −0.9 ± 0.54 | +0.9 ± 0.64 |
| Seed 23, first mapping, mirrored | readout 1, the even places | +1.1 ± 0.79 | −0.2 ± 0.72 | −1.3 ± 0.74 |
| Seed 24, eighth, assignment | readout 1, the even places | −0.6 ± 1.27 | −4.6 ± 1.59 | −4.0 ± 2.07 |
| Seed 22, first mapping, mirrored | readout 1, the even places | −0.6 ± 0.48 | −1.1 ± 0.31 | −0.5 ± 0.46 |
| Seed 22, eighth, assignment | readout 1, the even places | −2.3 ± 1.12 | −2.4 ± 1.07 | −0.1 ± 1.59 |

| Cell | A answers | Own group and inhibitory units less H-47's plain | less 027's held within | less excitatory | Own group and outside the readouts less H-47's plain | less 027's held within | less excitatory | Inhibitory units and outside the readouts less H-47's plain | less 027's held within | less excitatory |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | −4.3 ± 0.73 | −1.4 ± 1.02 | −0.9 ± 1.68 | −1.1 ± 0.41 | +1.8 ± 0.82 | +2.3 ± 1.50 | −1.8 ± 0.94 | +1.1 ± 1.01 | +1.6 ± 1.45 |
| Seed 22, first mapping, assignment | readout 0, the odd places | −1.5 ± 1.18 | −0.1 ± 0.96 | +1.2 ± 1.03 | +2.0 ± 0.89 | +3.4 ± 1.43 | +4.7 ± 0.92 | −1.0 ± 0.89 | +0.5 ± 1.02 | +1.8 ± 0.82 |
| Seed 23, eighth, mirrored | readout 0, the odd places | −2.4 ± 1.15 | −0.5 ± 0.92 | +0.3 ± 1.32 | −0.8 ± 1.06 | +1.1 ± 0.79 | +1.9 ± 1.06 | −0.9 ± 1.21 | +1.0 ± 0.93 | +1.8 ± 1.03 |
| Seed 23, first mapping, assignment | readout 0, the odd places | −0.6 ± 0.79 | −0.5 ± 0.88 | +0.2 ± 1.12 | −0.2 ± 0.91 | −0.1 ± 0.93 | +0.5 ± 1.17 | −0.2 ± 0.82 | −0.1 ± 0.89 | +0.6 ± 1.03 |
| Seed 24, eighth, mirrored | readout 0, the odd places | +3.7 ± 1.19 | −0.6 ± 1.29 | −1.8 ± 1.19 | +2.0 ± 2.45 | −2.3 ± 2.15 | −3.4 ± 1.99 | +5.2 ± 1.48 | +0.9 ± 1.31 | −0.3 ± 1.15 |
| Seed 24, first mapping, assignment | readout 0, the odd places | +1.4 ± 0.84 | −1.0 ± 1.52 | −0.6 ± 1.01 | +1.4 ± 1.16 | −0.9 ± 1.12 | −0.6 ± 1.15 | +2.4 ± 1.15 | 0.0 ± 0.78 | +0.4 ± 0.96 |
| Seed 23, eighth, assignment | readout 1, the even places | +3.3 ± 1.29 | −2.8 ± 1.20 | −3.6 ± 1.61 | +2.3 ± 1.12 | −3.9 ± 0.88 | −4.6 ± 1.12 | +5.2 ± 1.43 | −0.9 ± 1.27 | −1.6 ± 1.72 |
| Seed 24, first mapping, mirrored | readout 1, the even places | +0.2 ± 0.67 | −1.5 ± 0.99 | −1.2 ± 0.87 | +2.0 ± 0.80 | +0.3 ± 1.11 | +0.5 ± 1.00 | +1.1 ± 0.78 | −0.6 ± 0.78 | −0.3 ± 0.61 |
| Seed 23, first mapping, mirrored | readout 1, the even places | +2.0 ± 0.78 | −0.3 ± 0.94 | −1.2 ± 0.92 | +0.9 ± 1.34 | −1.3 ± 0.92 | −2.3 ± 1.09 | +2.2 ± 1.26 | −0.1 ± 0.73 | −1.0 ± 0.82 |
| Seed 24, eighth, assignment | readout 1, the even places | −0.8 ± 1.74 | −3.1 ± 1.30 | −7.8 ± 1.21 | −0.2 ± 1.44 | −2.5 ± 1.50 | −7.2 ± 1.16 | +3.8 ± 2.57 | +1.5 ± 1.28 | −3.1 ± 2.17 |
| Seed 22, first mapping, mirrored | readout 1, the even places | +0.1 ± 0.84 | −0.5 ± 0.81 | +0.5 ± 1.01 | +0.8 ± 0.89 | +0.1 ± 0.54 | +1.1 ± 0.69 | +1.2 ± 0.98 | +0.6 ± 0.74 | +1.5 ± 0.87 |
| Seed 22, eighth, assignment | readout 1, the even places | −0.5 ± 1.93 | −2.8 ± 1.46 | −3.0 ± 1.64 | +1.8 ± 1.65 | −0.5 ± 1.84 | −0.7 ± 1.55 | +1.9 ± 1.62 | −0.4 ± 1.05 | −0.5 ± 1.34 |

| Cell | A answers | Own group and inhibitory units less own group alone (028) | less inhibitory units alone (028) | Own group and outside the readouts less own group alone (028) | less outside the readouts alone (028) | Inhibitory units and outside the readouts less inhibitory units alone (028) | less outside the readouts alone (028) |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Seed 22, eighth, mirrored | readout 0, the odd places | −1.5 ± 0.84 | −0.7 ± 1.58 | +1.7 ± 0.37 | −2.2 ± 0.34 | +1.8 ± 1.16 | −2.9 ± 0.91 |
| Seed 22, first mapping, assignment | readout 0, the odd places | −3.9 ± 1.54 | +0.3 ± 0.94 | −0.3 ± 0.73 | +1.7 ± 0.84 | +0.9 ± 0.72 | −1.3 ± 0.72 |
| Seed 23, eighth, mirrored | readout 0, the odd places | −2.0 ± 1.12 | −0.3 ± 1.04 | −0.4 ± 1.08 | −0.4 ± 1.11 | +1.3 ± 1.24 | −0.4 ± 1.36 |
| Seed 23, first mapping, assignment | readout 0, the odd places | +0.1 ± 0.89 | +0.1 ± 1.23 | +0.4 ± 1.11 | +0.2 ± 0.97 | +0.5 ± 1.09 | +0.3 ± 0.91 |
| Seed 24, eighth, mirrored | readout 0, the odd places | +3.4 ± 1.12 | +2.0 ± 1.09 | +1.8 ± 1.85 | +1.3 ± 1.31 | +3.5 ± 1.22 | +4.5 ± 1.15 |
| Seed 24, first mapping, assignment | readout 0, the odd places | +0.8 ± 0.91 | −1.4 ± 1.13 | +0.8 ± 0.99 | +0.4 ± 1.20 | −0.4 ± 1.07 | +1.3 ± 1.16 |
| Seed 23, eighth, assignment | readout 1, the even places | +2.9 ± 1.91 | +0.6 ± 1.65 | +1.9 ± 1.48 | −0.2 ± 0.87 | +2.6 ± 1.74 | +2.8 ± 1.45 |
| Seed 24, first mapping, mirrored | readout 1, the even places | −0.9 ± 0.53 | −0.5 ± 0.55 | +0.9 ± 0.68 | +0.9 ± 0.88 | +0.4 ± 0.81 | +0.1 ± 0.85 |
| Seed 23, first mapping, mirrored | readout 1, the even places | +1.7 ± 0.72 | +0.1 ± 1.00 | +0.6 ± 0.80 | +0.7 ± 1.51 | +0.3 ± 1.13 | +2.0 ± 1.07 |
| Seed 24, eighth, assignment | readout 1, the even places | +0.3 ± 1.33 | −0.4 ± 0.87 | +0.9 ± 1.29 | −1.9 ± 2.05 | +4.3 ± 1.77 | +2.1 ± 2.00 |
| Seed 22, first mapping, mirrored | readout 1, the even places | −0.3 ± 0.67 | +0.3 ± 1.05 | +0.3 ± 0.42 | −0.3 ± 0.73 | +1.4 ± 0.96 | +0.2 ± 0.85 |
| Seed 22, eighth, assignment | readout 1, the even places | +0.1 ± 2.18 | −1.6 ± 1.72 | +2.4 ± 1.51 | −0.2 ± 1.47 | +0.8 ± 1.42 | −0.1 ± 1.91 |

| Cell | A answers | Own group and inhibitory units: side | Own group and outside the readouts: side | Inhibitory units and outside the readouts: side | H-47's side | Held within (027): side | Excitatory: side | Own group alone (028): side | Inhibitory units alone (028): side | Outside the readouts alone (028): side |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Seed 22, eighth, mirrored | readout 0, the odd places | A, the odd places (within 2 SE) | B, the even places (within 2 SE) | A, the odd places | B, the even places (within 2 SE) | A, the odd places (within 2 SE) | A, the odd places | B, the even places (within 2 SE) | A, the odd places | B, the even places |
| Seed 22, first mapping, assignment | readout 0, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, mirrored | readout 0, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, first mapping, assignment | readout 0, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 24, eighth, mirrored | readout 0, the odd places | A, the odd places (within 2 SE) | B, the even places (within 2 SE) | B, the even places | B, the even places | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places (within 2 SE) | B, the even places | B, the even places |
| Seed 24, first mapping, assignment | readout 0, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places (within 2 SE) | A, the odd places | A, the odd places | A, the odd places | A, the odd places | A, the odd places |
| Seed 23, eighth, assignment | readout 1, the even places | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places | A, the even places | A, the even places | A, the even places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 24, first mapping, mirrored | readout 1, the even places | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 23, first mapping, mirrored | readout 1, the even places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places (within 2 SE) | B, the odd places | B, the odd places |
| Seed 24, eighth, assignment | readout 1, the even places | A, the even places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) | A, the even places (within 2 SE) | B, the odd places (within 2 SE) | B, the odd places (within 2 SE) | A, the even places (within 2 SE) |
| Seed 22, first mapping, mirrored | readout 1, the even places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |
| Seed 22, eighth, assignment | readout 1, the even places | B, the odd places | B, the odd places (within 2 SE) | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places | B, the odd places |

- **Less the night without replay** the night held onto the own group and the inhibitory units lies beyond 2 SE above in nine of the
  twelve cells, below in one, seed 23's first mapping from the mirrored assignment, −2.9 ± 0.96, and within 2 SE in two, seed 24's eighth
  image from the assignment, +1.0 ± 0.72, and seed 22's eighth image from the assignment, +1.7 ± 1.43. The night held onto the own group and
  the units outside the readouts lies above in ten, below in one, seed 23's first mapping from the mirrored assignment, −4.0 ± 1.10, and
  within 2 SE on seed 24's eighth image from the assignment, +1.6 ± 1.09. The night held onto the inhibitory units and the units outside the
  readouts lies above in eleven and below in one, seed 23's first mapping from the mirrored assignment, −2.7 ± 0.53. H-47's clause 1 read
  −4.9 ± 1.09 on seed 23's first mapping from the mirrored assignment and +1.8 ± 1.79 on seed 24's eighth image from the assignment, and
  exploration 027's night held within less its night without replay lay above in eleven and below in one, on the first, −2.6 ± 0.65.
- **The held nights less each other.** The night held onto the own group and the inhibitory units less the night held onto the own group
  and the units outside the readouts lies beyond 2 SE in five cells, all below: seed 22's eighth image from the mirrored assignment,
  −3.2 ± 0.60, seed 22's first mapping from the assignment, −3.6 ± 1.07, and seed 23's eighth image from the mirrored assignment,
  −1.6 ± 0.42, where A answers readout 0, and seed 24's first mapping from the mirrored assignment, −1.8 ± 0.16, and seed 22's eighth image
  from the assignment, −2.3 ± 1.12, where it answers readout 1. Less the night held onto the inhibitory units and the units outside the
  readouts it lies beyond in six, all below: seed 22's eighth image from the mirrored assignment, −2.5 ± 0.85, and seed 23's eighth image
  from the mirrored assignment, −1.5 ± 0.76, where A answers readout 0, and seed 23's eighth image from the assignment, −1.9 ± 0.34, seed
  24's eighth image from the assignment, −4.6 ± 1.59, seed 22's first mapping from the mirrored assignment, −1.1 ± 0.31, and seed 22's
  eighth image from the assignment, −2.4 ± 1.07, where it answers readout 1. The night held onto the own group and the units outside the
  readouts less the night held onto the inhibitory units and the units outside the readouts lies beyond in three: above on seed 22's first
  mapping from the assignment, +3.0 ± 0.75; below on seed 24's eighth image from the mirrored assignment, −3.2 ± 1.33, where A answers
  readout 0, and seed 23's eighth image from the assignment, −3.0 ± 0.78, where it answers readout 1.
- **Less H-47's plain night** the night held onto the own group and the inhibitory units lies beyond 2 SE in five cells: above on seed 24's
  eighth image from the mirrored assignment, +3.7 ± 1.19, where A answers readout 0, and seed 23's eighth image from the assignment,
  +3.3 ± 1.29, and seed 23's first mapping from the mirrored assignment, +2.0 ± 0.78, where it answers readout 1; below on seed 22's eighth
  image from the mirrored assignment, −4.3 ± 0.73, and seed 23's eighth image from the mirrored assignment, −2.4 ± 1.15, where A answers
  readout 0. The night held onto the own group and the units outside the readouts lies beyond in four: above on seed 22's first mapping from
  the assignment, +2.0 ± 0.89, where A answers readout 0, and seed 23's eighth image from the assignment, +2.3 ± 1.12, and seed 24's first
  mapping from the mirrored assignment, +2.0 ± 0.80, where it answers readout 1; below on seed 22's eighth image from the mirrored
  assignment, −1.1 ± 0.41. The night held onto the inhibitory units and the units outside the readouts lies beyond in three, all above:
  seed 24's eighth image from the mirrored assignment, +5.2 ± 1.48, and seed 24's first mapping from the assignment, +2.4 ± 1.15, where A
  answers readout 0, and seed 23's eighth image from the assignment, +5.2 ± 1.43, where it answers readout 1.
- **Less exploration 027's night held within** the night held onto the inhibitory units and the units outside the readouts lies within
  2 SE in all twelve cells. The night held onto the own group and the inhibitory units lies beyond in two, both below and where A answers
  readout 1: seed 23's eighth image from the assignment, −2.8 ± 1.20, and seed 24's eighth image from the assignment, −3.1 ± 1.30. The
  night held onto the own group and the units outside the readouts lies beyond in three: above on seed 22's eighth image from the mirrored
  assignment, +1.8 ± 0.82, and seed 22's first mapping from the assignment, +3.4 ± 1.43, where A answers readout 0; below on seed 23's
  eighth image from the assignment, −3.9 ± 0.88, where it answers readout 1.
- **Less the excitatory night** the night held onto the inhibitory units and the units outside the readouts lies beyond 2 SE in one cell,
  above, seed 22's first mapping from the assignment, +1.8 ± 0.82, and within 2 SE on seed 24's eighth image from the mirrored assignment,
  −0.3 ± 1.15, on seed 23's eighth image from the assignment, −1.6 ± 1.72, and on seed 24's eighth image from the assignment, −3.1 ± 2.17,
  the three cells where each part held alone lay below the excitatory night in exploration 028. The night held onto the own group and the
  inhibitory units lies beyond in two, both below and where A answers readout 1: seed 23's eighth image from the assignment, −3.6 ± 1.61,
  and seed 24's eighth image from the assignment, −7.8 ± 1.21. The night held onto the own group and the units outside the readouts lies
  beyond in four: above on seed 22's first mapping from the assignment, +4.7 ± 0.92, where A answers readout 0; below on seed 23's eighth
  image from the assignment, −4.6 ± 1.12, seed 23's first mapping from the mirrored assignment, −2.3 ± 1.09, and seed 24's eighth image from
  the assignment, −7.2 ± 1.16, where it answers readout 1.
- **Less exploration 028's nights.** The night held onto the own group and the inhibitory units lies within 2 SE of the night with the
  inhibitory units held alone in all twelve cells, and beyond 2 SE of the night with the own group held alone in three: above on seed 24's
  eighth image from the mirrored assignment, +3.4 ± 1.12, and seed 23's first mapping from the mirrored assignment, +1.7 ± 0.72; below on
  seed 22's first mapping from the assignment, −3.9 ± 1.54. The night held onto the own group and the units outside the readouts lies beyond
  2 SE of the night with the own group held alone in one, above, seed 22's eighth image from the mirrored assignment, +1.7 ± 0.37, and of
  the night with the units outside the readouts held alone in two: above on seed 22's first mapping from the assignment, +1.7 ± 0.84, and
  below on seed 22's eighth image from the mirrored assignment, −2.2 ± 0.34. The night held onto the inhibitory units and the units outside
  the readouts lies beyond 2 SE of the night with the inhibitory units held alone in two, both above: seed 24's eighth image from the
  mirrored assignment, +3.5 ± 1.22, and seed 24's eighth image from the assignment, +4.3 ± 1.77; and of the night with the units outside the
  readouts held alone in two: above on seed 24's eighth image from the mirrored assignment, +4.5 ± 1.15, and below on seed 22's eighth image
  from the mirrored assignment, −2.9 ± 0.91.
- **The side** at block 36 under the night held onto the inhibitory units and the units outside the readouts is exploration 027's night held
  within's and the excitatory night's in eleven of the twelve cells, all but seed 24's first mapping from the mirrored assignment, and
  H-47's in ten, all but seed 22's eighth image from the mirrored assignment and seed 24's eighth image from the assignment. It lies on the
  odd places in eight: on A in the five cells where A answers readout 0 but seed 24's eighth image from the mirrored assignment, where it
  lies on B, the even places, and on B in three of the six where it answers readout 1, seed 23's first mapping from the mirrored assignment,
  seed 22's first mapping from the mirrored assignment and seed 22's eighth image from the assignment; on the other three, seed 23's eighth
  image from the assignment, seed 24's first mapping from the mirrored assignment and seed 24's eighth image from the assignment, it lies on
  A, the even places, within 2 SE. Under the night held onto the own group and the inhibitory units the side is exploration 027's night held
  within's and the excitatory night's in eleven, all but seed 24's eighth image from the mirrored assignment, and H-47's in eight. Under the
  night held onto the own group and the units outside the readouts it is theirs in eleven, all but seed 22's eighth image from the mirrored
  assignment, and H-47's in ten.

## What it costs

The trial took 427 s and the twelve jobs 7 269 to 7 949 s, 121.2 to 132.5 minutes, at most 55.2 % of a job's 240 minutes; ADR-0244 put a
job at about 120 to 130 minutes, and three ran past 130: seed 22's eighth image from the mirrored assignment, 7 865 s, seed 24's first
mapping from the assignment, 7 824 s, and seed 24's first mapping from the mirrored assignment, 7 949 s. Nothing was cut: every cell ran its
four forms. Thirteen jobs ran, the trial's and twelve, every one to the end.

## What it suggests

**In these twelve cells the part held within with the own group's part left to consolidate leaves A's count where the part held within
held whole left it.** Under the night held onto the inhibitory units and the units outside the readouts A's count lies within twice
exploration 018's unpaired SE of exploration 027's night held within's at all 60 checkpoints, and within 2 SE of the excitatory night's at
54. Of the 25 checkpoints where the excitatory night lies beyond 2 SE of the plain night, it lies beyond the plain night in the same
direction at 14, all 14 of its own; at block 36 it lies within 2 SE of the excitatory night on seed 24's eighth image from the mirrored
assignment, seed 23's eighth image from the assignment and seed 24's eighth image from the assignment, where each part held alone in
exploration 028 lay below it. The own group's part, left free, moved over four nights within 0.07 M a group of the plain night's change.

**Holding the own group's part with either other part does not leave A's count there in every cell.** The night held onto the own group
and the inhibitory units lies within 2 SE of exploration 028's night with the inhibitory units held alone at all 60 checkpoints, and below
exploration 027's night held within beyond 2 SE at 7, all on cells where A answers readout 1, five of them on seed 23's eighth image from
the assignment from block 18 on and on seed 24's eighth image from the assignment at blocks 24 and 36. The night held onto the own group and
the units outside the readouts lies beyond 2 SE of exploration 027's night held within at 13 of the 60, of the plain night at 8, and of
exploration 028's nights with its two parts held alone at 3 and 4.

**The three pairs differ from each other beyond 2 SE at 21 to 23 of the 60 checkpoints**, run in one job, where exploration 028's three
nights each part held alone did at 16 to 22. Under every held night here the inhibition onto the group A answers rose by more than onto the
other in each of the twelve cells, as under the plain night.

Questions this suggests for a later exploration or a round:
- The night held onto the inhibitory units and the units outside the readouts without replay, to read the hold apart from the replay.
- The two parts it holds split finer, the inhibitory units by readout and the units outside the readouts by class, to find whether A's
  count there needs all of both.
- What on seed 24's eighth image from the assignment keeps the night held onto the inhibitory units and the units outside the readouts
  below the excitatory night beyond 2 SE at blocks 18 and 24, where exploration 027's night held within lay below it too.
- A read of what the next day's first consolidations do with the pending gains and losses.

## What did not run

- **Any part held alone again**: each part held alone is read from exploration 028's job.
- **The three parts held together**, but in the trial: it held them for one night, to hold exploration 027's night held within's row; no
  job holds all three.
- **The part held across**, alone or with any part: it consolidates as in H-47 under every form here.
- **Exploration 024's hold**, alone or with any part: the inhibitory synapses onto the readouts' excitatory units consolidate as in H-47
  under every form here.
- **Exploration 027's night held within again, or the excitatory night again**: both are read from exploration 027's, 025's and 026's
  jobs.
- **Any pair of parts held in the night without replay**: the night without replay ran with no hold set, so a hold here is not read apart
  from the replay.
- **A finer split by target**: the inhibitory units by readout, or the units outside the readouts by class; section 4 breaks the units
  outside the readouts down into A's and B's units and the other excitatory units without holding one apart.
- **Plain nights on both mappings and the awake gap**, under a hold or not: no reading here stands against them.
- **A hold through the quiet ticks**: the hold ends at the night's last tick, and the image is written after the quiet ticks. Here the
  quiet ticks moved no synapse of the two held parts in any of the 432 held nights.
- **The excitatory synapses onto the readouts from A's, B's and the pair's own units**, which consolidate as in H-47 under every form, the
  zero sum's rounds with them, and **the inhibitory synapses onto the readouts' inhibitory units**, which no hold here reaches.
- **The next day's reading of the pending pairings**: the next day's consolidation of the held synapses is not read.
- **A diagnosis of seed 24's eighth image from the mirrored assignment, seed 23's eighth image from the assignment or seed 24's eighth image
  from the assignment alone**: each runs here as one cell of twelve.
- **REM's depotentiations and the zero sum's rounds** are not split out of a night's change, as in explorations 020 to 028.
- **Any SE for sections 4 and 5**: the class matrix is a mean over the three paths, and the hold's readings are ranges.
- **Any figure pooled across cells**, in section 6 or elsewhere.
- **Any other share, form, network or seed.**

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_the_part_held_within_two_parts_at_a_time_explore` | [42582358085](https://github.com/Menteion/Menteion/actions/runs/42582358085) | `be301a0` | 427 |
| `the_part_held_within_two_parts_at_a_time_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [42583562331](https://github.com/Menteion/Menteion/actions/runs/42583562331) | `be301a0` | 7865 |
| `the_part_held_within_two_parts_at_a_time_a_first_mapping_on_seed_22_from_the_assignment_explore` | [42583562331](https://github.com/Menteion/Menteion/actions/runs/42583562331) | `be301a0` | 7716 |
| `the_part_held_within_two_parts_at_a_time_the_eighth_image_on_seed_23_from_the_mirrored_assignment_explore` | [42583562331](https://github.com/Menteion/Menteion/actions/runs/42583562331) | `be301a0` | 7278 |
| `the_part_held_within_two_parts_at_a_time_a_first_mapping_on_seed_23_from_the_assignment_explore` | [42583562331](https://github.com/Menteion/Menteion/actions/runs/42583562331) | `be301a0` | 7463 |
| `the_part_held_within_two_parts_at_a_time_the_eighth_image_on_seed_24_from_the_mirrored_assignment_explore` | [42583562331](https://github.com/Menteion/Menteion/actions/runs/42583562331) | `be301a0` | 7269 |
| `the_part_held_within_two_parts_at_a_time_a_first_mapping_on_seed_24_from_the_assignment_explore` | [42583562331](https://github.com/Menteion/Menteion/actions/runs/42583562331) | `be301a0` | 7824 |
| `the_part_held_within_two_parts_at_a_time_the_eighth_image_on_seed_23_from_the_assignment_explore` | [42583562331](https://github.com/Menteion/Menteion/actions/runs/42583562331) | `be301a0` | 7429 |
| `the_part_held_within_two_parts_at_a_time_a_first_mapping_on_seed_24_from_the_mirrored_assignment_explore` | [42583562331](https://github.com/Menteion/Menteion/actions/runs/42583562331) | `be301a0` | 7949 |
| `the_part_held_within_two_parts_at_a_time_a_first_mapping_on_seed_23_from_the_mirrored_assignment_explore` | [42583562331](https://github.com/Menteion/Menteion/actions/runs/42583562331) | `be301a0` | 7722 |
| `the_part_held_within_two_parts_at_a_time_the_eighth_image_on_seed_24_from_the_assignment_explore` | [42583562331](https://github.com/Menteion/Menteion/actions/runs/42583562331) | `be301a0` | 7609 |
| `the_part_held_within_two_parts_at_a_time_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [42583562331](https://github.com/Menteion/Menteion/actions/runs/42583562331) | `be301a0` | 7622 |
| `the_part_held_within_two_parts_at_a_time_the_eighth_image_on_seed_22_from_the_assignment_explore` | [42583562331](https://github.com/Menteion/Menteion/actions/runs/42583562331) | `be301a0` | 7412 |
