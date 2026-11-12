---
status: exploratory
date: 2026-11-12
---

# 037: What the night writes, on both cells

**Branch**: `explore/037-what-the-night-writes-on-both-cells`
**Commit**: `bbf805d`, which reads the pairings below zero, on `7233796`, which adds the harness, on `5266327`: `main` at `a8e91c5`,
ADR-0252's merge, with exploration 036's tag merged in, which changes nothing under `src/`. This branch changes nothing under
`src/`, so the engine is `main`'s.
**Runs**: [43238887408](https://github.com/Menteion/Menteion/actions/runs/43238887408) (two jobs) and
[43236110064](https://github.com/Menteion/Menteion/actions/runs/43236110064) (the harness's trial), both at `bbf805d` and
`scope=arms`. Before them, at `7233796`, the harness's first trial, run
[43232507958](https://github.com/Menteion/Menteion/actions/runs/43232507958), which stopped on a count ADR-0252 held at none. The
logs set beside them are, on the image from the assignment, exploration 033's jobs, run
[43011654795](https://github.com/Menteion/Menteion/actions/runs/43011654795), exploration 034's, run
[43052358536](https://github.com/Menteion/Menteion/actions/runs/43052358536), and exploration 036's first two, run
[43192758106](https://github.com/Menteion/Menteion/actions/runs/43192758106), and on the image from the mirrored assignment
exploration 035's jobs, run [43134386489](https://github.com/Menteion/Menteion/actions/runs/43134386489), and exploration 036's
other two, run 43192758106. None of those was run again.

[ADR-0252](../adr/0252-what-the-night-writes-on-both-cells.md) ordered this exploration. On seed 24's eighth image from the assignment and
from the mirrored assignment, after the three later single replayed nights, the volley's drive from A's kinds onto A's answer's readout less
the other parted the two cells, carried by the weights ([exploration 036](036-what-reaches-the-readout-units-on-both-cells.md)).
Explorations 034 and 035 read those weights before and after each night as each readout unit's input by class ([exploration
035](035-the-mirrored-cells-units-and-inputs.md)). No log held which readout units fire in a night's replays, or what consolidation and the
zero sum write onto each readout unit. This note runs the nights alone on both cells, reads that unit by unit, and asks:

1. whether every night, its image, its replays and its watch's lines, and every input read before and after it, are the logs' of the same
   night, path and block;
2. what each single replayed night wrote onto each readout from each of A's and B's kinds, less the night without replay from the same
   image, by the readout unit's class;
3. how it was written: what consolidation moved in each bin of when it fell, what the zero sum's rounds moved, and what the watch does not
   see;
4. which readout units fired in the replays of each stimulus's patterns, and whether a unit's spikes there line up with what it gained;
5. each of those on the two cells side by side, each readout named by the stimulus whose answer it is.

The note writes no prediction and decides nothing. **Every figure here is exploratory**: from one run of each test on the repository's
AArch64 host, in H-47's terms. **A night's figure is the mean over the three paths**, one night on each, with the paths' lowest and highest
beside it in brackets; no SE is claimed. **A weight, an input and what moved them** are written in thousands (k) of the class matrix's
units: a weight of 1, 32 768, is 32.768 k. A's and B's kinds are excitatory, so every weight read here is excitatory; an inhibitory readout
unit is one that receives it. Every sum, difference and range is worked from the unrounded values and rounded once, an exact half away from
zero.

**In brief**
- **Every night is the logs'.** On each cell all 24 nights held their row inside the test: the image after the night, A's couplings at their
  highest before it and its gain. Label aside, every line each night's watch printed is the logs' night's, in order. The input reads before
  and after each night are explorations 034's to 036's, line for line.
- **ADR-0252 held the pairings read below zero at none; the harness's first trial read some, and the read changed.** The harness's first
  trial read 896 on the image from the mirrored assignment, on path 0's night without replay after block 18. A night's engine is decoded
  from an image, so every unit is an addressed source and target. An excitatory synapse there consolidates under the baseline plus the
  dopamine term, which can fall below zero. The harness now checks such a pairing by the engine's rule below zero, a miss counted in
  `wrong`, and counts its weight's move with its own sign. In the jobs only 2 of the 48 nights read any. Both start from that path's image
  before the night after block 18 on the image from the mirrored assignment: 896 pairings on the night without replay and 2 272 on the night
  after block 18 alone. All of them fell in the first 22 231 ticks the watch saw, in slow-wave sleep, at a modulation as low as −1.000.
- **What each single replayed night wrote.** A's kinds onto A's answer's readout less B's answer's, less the night without replay from the
  same image, moved +688.9, +623.4, +595.9 and +489.8 k after block 6, 12, 18 and 24 alone on the image from the assignment, and +669.4,
  +812.2, +794.5 and +688.4 on the image from the mirrored assignment. These are exploration 035's input changes, read again. On every night
  and path each kind's change onto one readout is the negation of its change onto the other within 121 of the class matrix's units: the zero
  sum.
- **How it was written.** Of that change, consolidation in the presynaptic unit's own replay carried +364.5, +386.3, +289.4 and +264.8 k on
  the image from the assignment and +510.4, +616.3, +565.4 and +472.7 on the image from the mirrored assignment. At its next spike after the
  replay it carried +146.8, +161.0, +129.5 and +130.9 and +148.5, +153.8, +195.0 and +179.8. At its other spikes in slow-wave sleep it
  carried +185.9, +87.7, +168.6 and +115.5 and +17.5, +52.6, +48.0 and +46.5. Nothing was consolidated outside slow-wave sleep on any night.
  The zero sum's rounds moved −8.3, −11.5, +8.3 and −21.5 and −7.1, −10.4, −11.1 and −10.7. What the watch does not see reads 0.0 on every
  one of these differences.
- **By the readout unit's class.** On the image from the assignment A's patterns' targets took +718.1, +646.7, +636.1 and +585.6 k of it,
  more than all of it. The inhibitory units took −70.4, −87.2, −87.4 and −77.7. On the image from the mirrored assignment the targets took
  +509.5, +604.4, +562.0 and +501.2, the other excitatory units +51.4, +87.3, +72.0 and +58.3 and the inhibitory units +49.8, +46.7, +94.9
  and +67.6.
- **Where the two cells part.** The image from the mirrored assignment less the image from the assignment reads −19.5, +188.8, +198.6 and
  +198.6 k. Consolidation in the units' own replays makes +145.9, +230.0, +276.0 and +207.9 of that, and at their other spikes in slow-wave
  sleep −168.3, −35.1, −120.6 and −69.0. By class, the targets make −208.7, −42.3, −74.2 and −84.3, the other excitatory units +57.8, +80.4,
  +78.5 and +93.9 and the inhibitory units +120.2, +133.9, +182.3 and +145.3. The first night stands apart: there the own replays' +145.9 is
  offset by the other spikes' −168.3.
- **Which readout units fired.** In the replays of stimulus A's patterns A's answer's readout fired 22 969.3 to 23 695.3 spikes a night on
  the image from the assignment and 24 487.3 to 25 018.3 on the image from the mirrored assignment. Of them, 15 204.3 to 15 534.3 and
  17 050.7 to 17 336.3 were spikes of the replayed pattern's own units. B's answer's readout fired 9 470.7 to 10 047.3 and 9 502.0 to
  9 858.3 there, none as a pattern's unit. Every readout unit fired in each stimulus's replays on each replayed night and path, but one unit
  of B's answer's readout in the replays of stimulus A's patterns after block 12 alone on the image from the mirrored assignment, on each
  path. A unit's spikes in stimulus A's replays line up weakly with its gain from A's kinds over the units of A's answer's readout:
  correlations of +0.04 to +0.19 and +0.09 to +0.15.

## What ran

**The engine, the network, the nights and the read** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 020 to 036 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. The pair's blocks run from the start, a night after blocks 6,
12, 18 and 24, on paths 0, 5 and 6.

**Two cells**, seed 24's eighth image from the assignment, explorations 033's, 034's and 036's, and seed 24's eighth image from the mirrored
assignment, explorations 035's and 036's: H-25's two arms on the same network. On the image from the assignment A asks readout 1, the even
places, of stimulus A and readout 0, the odd places, of B. On the image from the mirrored assignment A asks readout 0 of A and readout 1 of
B. Each job's pattern lines say so. **A's answer's readout** is readout 1 on the image from the assignment and readout 0 on the image from
the mirrored assignment, **B's answer's readout** the other.

**A's patterns** are H-47's 102, drawn from the start image, one starting at each of the 102 stimulus units. **Stimulus A's patterns** are
the 51 that start at A's kinds' units, **stimulus B's** the 51 at B's. On both cells each pattern's targets past its first unit all lie in
its stimulus's answer's readout: 545 distinct targets on the image from the assignment and 576 on the image from the mirrored assignment, as
each job's pattern lines print them.

**The kinds**, exploration 016's: **A alone**, A's 25 units that C does not share, **A and C**, the 26 it shares, **B alone** and **B and
D** the same of B; **A's kinds** are the first two and **B's kinds** the last two. Each kind is a group of the zero sum
([ADR-0226](../adr/0226-a-zero-sum-at-the-stimulus.md), [ADR-0228](../adr/0228-a-zero-sum-at-the-stimulus-built.md)). After every tick whose
consolidations moved one of a group's synapses onto a target, the group's weights onto the targets, the two readouts' 918 units, are scaled
back to its value.

**Each readout unit's class**, exploration 020's, from each cell's start image (`e020_class`), which each job prints. The readout units'
classes make four groups here: C's or D's own, the pair's units inside the readout, C's 25 at place 5 in readout 0 and D's 25 at place 16 in
readout 1; A's patterns' targets; the other excitatory units; and the inhibitory units.

| The cell | Readout | C's or D's own | A's patterns' targets | Other excitatory | Inhibitory | All |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| From the assignment | A's answer's readout: readout 1, the even places | 25 | 252 | 80 | 102 | 459 |
| From the assignment | B's answer's readout: readout 0, the odd places | 25 | 245 | 87 | 102 | 459 |
| From the mirrored assignment | A's answer's readout: readout 0, the odd places | 25 | 280 | 52 | 102 | 459 |
| From the mirrored assignment | B's answer's readout: readout 1, the even places | 25 | 254 | 78 | 102 | 459 |

**The nights.** On each path of each cell **the chain** runs the pair's blocks from the start to block 24, with H-47's night without replay
after block 6, 12, 18 and 24. From the image before each of those four nights **a single replayed night** runs, H-47's plain night on A's
patterns: A's 102 patterns tagged at priority 200. Nothing runs after it. Eight nights a path, 24 a cell. Until its night, the form after
block N alone ran the chain's nights on the chain's images, so each replayed night is that form's night in the logs. No hold is set, no
block runs after block 24, and neither A nor the pair is read.

**The watch** is exploration 020's `ReplayWatch`, built after a night's lead ticks and ticked one tick at a time over every ripple to the
night's end. It reads every pairing of A's and B's 102 stimulus units by the rule as the block held it, holds what consolidation took from
the trace to the trace the engine then holds, and counts each miss (`wrong`). Every watched unit is a source of the zero sum, so the weight
itself is not checked. In a ripple that replays one of A's patterns it classes every spike against the pattern. The harness adds to it,
summed by unit over the arena and printed for the readout units:
- **each unit's spikes in the ripples that replay one of A's patterns**, by the pattern's stimulus, as one of the pattern's units or outside
  it;
- **what consolidation moved each kind's weights onto each unit by**, in five bins: the presynaptic unit's own replay (its first, second,
  third and later spikes there), its next spike after that replay, any other of its spikes in slow-wave sleep, any other outside slow-wave
  sleep, and every pairing read at a modulation below zero, whatever its ordinal;
- **the zero sum's rounds** onto each unit from each kind: its weights onto the unit after the watch's last tick, before the night's quiet
  ticks, less its weights when the watch was built, less what consolidation moved them by;
- **the pairings read below zero**, the ticks read below zero by stage, the first and the last of them from the watch's start, and the
  lowest modulation.

**A bin says when a trace was consolidated, not when it was written.** A consolidation takes its share of the whole trace the synapse then
holds, half of it at the slow-wave baseline. That trace holds the earlier pairings and what the day left. So a kind's first spike of a night
consolidates part of the day's trace in whichever bin that spike falls; on the night without replay no spike falls in a replay.

**Held inside the test on every night**: the image after it, by CRC-64, A's couplings at their highest over the block ends before it and its
gain are the row the logs give for that night, path and cell. The chain's nights hold H-47's pinned row of the night without replay at that
block. The night after block 6 alone holds H-47's plain night's at block 6. The nights after block 12, 18 and 24 alone hold the rows
explorations 033 and 035 printed, read from their artifacts by a script and pinned in the harness. A replayed night reads replays and
depotentiations that are not zero, and a night without replay none. The watch misses nothing, below zero as above it. No round moves a
weight onto any of the 106 units outside the readouts. Every unit's spikes in the replays, summed over the arena by stimulus and class, are
the watch's ripple spikes lines'. What every unit's pairings took from the trace, summed over the arena by kind, bin of ordinals and the
target's class, is the watch's pairings lines' `absorbed`.

**Beside it, exploration 034's input read** on the image before and after each night: each readout unit's input by presynaptic class, A's
and B's four kinds among them, as explorations 034 to 036 ran it. A readout unit's change from a kind over a night, less what consolidation
moved and the rounds, is **what the watch does not see**: the night's lead ticks before the watch is built and its quiet ticks after the
night.

## Pairings below zero

ADR-0252 held the pairings read at a modulation below zero at none, and the harness's first trial, run
[43232507958](https://github.com/Menteion/Menteion/actions/runs/43232507958) at `7233796`, held it so. On the image from the mirrored
assignment, path 0, the night without replay after block 18 read 896, and the test stopped there, that night's row held. The image from the
assignment ran path 0 whole, its eight nights held.

A night's engine is decoded from an image, and a decoded executor starts with every unit an addressed source and target (ADR-0068). Neither
the image nor the night's harness changes that. So an excitatory synapse consolidates under the addressed modulation, the baseline plus the
dopamine term, clamped to −1 and 1 under the signed gate, and below zero `SynapseBlock::consolidate_signed` moves the weight against its
trace's sign. The watch had checked nothing there, and its `absorbed`, the trace's loss, carried the weight's move with the other sign.
Commit `bbf805d` reads such a pairing by that rule, held to `consolidate_signed` by a unit test over 432 cases, each miss counted in
`wrong`. Its weight's move goes into each unit's consolidation with its own sign, in its own bin. The count is printed each night instead of
held. The watch's other lines are unchanged: above zero its rule is the same, and every night of the jobs reads `wrong 0`.

In the jobs:

| The cell | Night | Path | Pairings below zero | Watched ticks below zero, all in slow-wave sleep | The first and the last, ticks from the watch's start | The lowest modulation | Their move: A's kinds onto A's answer's readout, k | onto B's answer's readout | B's kinds onto A's answer's readout | onto B's answer's readout |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| From the mirrored assignment | Without replay, after block 18 | 0 | 896 | 22 231 | 0 to 22 230 | −1.000 | −5.6 | −2.4 | −2.2 | −2.1 |
| From the mirrored assignment | After block 18 alone | 0 | 2 272 | 22 231 | 0 to 22 230 | −1.000 | −14.2 | −2.6 | 0.0 | −1.5 |

Both nights start from the same image, read below zero over the same 22 231 watched ticks of slow-wave sleep, the first to the 22 231st from
the watch's start, and reach −1.000. The other 46 nights read none.

## Before a job ran

ADR-0252 asks whether any night must run bit for bit as one already in the logs. **Each does**: every night's image, replays and watch lines
are in explorations 033's to 036's logs, and its input reads in 034's to 036's. **No log holds a readout unit's spikes in the replays,
consolidation by readout unit, or the zero sum's rounds**, so each night ran, and its images and lines are set beside those logs' here.

## The runs are laid out as written

**The second trial**, run [43236110064](https://github.com/Menteion/Menteion/actions/runs/43236110064) at `bbf805d`, took 533 s. It ran path
0 of both cells whole, the two at once, as a job runs two paths: each cell's 24 blocks, its chain's four nights and its four replayed
nights, every night held as above. Each night held its row: on each cell four of H-47's night without replay, H-47's plain night's at block
6, and three of exploration 033's or 035's. A path took 373 s on the image from the assignment and 379 s on the image from the mirrored
assignment. Twice that, 746 and 758 s, was each job's projection, against an arms job's 240 minutes, so each cell ran as one job.

**The jobs**, run [43238887408](https://github.com/Menteion/Menteion/actions/runs/43238887408) at `bbf805d`, one a cell, each its three
paths two at a time:

| The cell | Nights | Held inside the test: the watch's misses 0, the rounds 0 outside the readouts, the sums its lines' | Replayed nights: replays | depotentiations | Nights without replay reading replays 0 and depotentiations 0 | Rows held inside the test |
| :--- | ---: | :--- | :--- | :--- | :--- | :--- |
| From the assignment | 24 | 24 of 24 | 1 329 to 1 341 | 640 | 12 of 12 | H-47's night without replay's, 12; its plain night's at block 6, 3; exploration 033's after block 12, 18 and 24 alone, 3 each |
| From the mirrored assignment | 24 | 24 of 24 | 1 330 to 1 341 | 640 | 12 of 12 | H-47's night without replay's, 12; its plain night's at block 6, 3; exploration 035's after block 12, 18 and 24 alone, 3 each |

## 1. The nights against the logs

Each night of the chain is set beside the night without replay after the same block on the same path in each log, and each replayed night
beside the night after that block alone. Each input read before a night is set beside the logs' reads before that night, of the night
without replay and of the night after that block alone, and each after a night beside the same night's. Exploration 033 ran no input read,
so its lines there are the split and the class matrix alone. After a night the logs' checkpoint reads each kind's sums again; that line is
not this harness's and is left out. The night without replay ran in both of each exploration's jobs, so each night of the chain, its lines
and its read after it are counted once against each job, and each read before a night against each of the logs' nights from that image.

| The cell | Logs set beside | Nights whose image, highest and gain are the logs' row | Nights whose every line is the logs' night's, in order, label aside | Of the logs' night lines, found | Input reads before a night, every line found | Their lines found | Input reads after a night, every line found | Their lines found |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| From the assignment | exploration 033's, run 43011654795, arm-0 and arm-1 | 24 of 24 | 24 of 24 | 6 914 of 6 914 | 12 of 12 | 108 of 108 | 24 of 24 | 72 of 72 |
| From the assignment | exploration 034's, run 43052358536, arm-0 and arm-1 | 24 of 24 | 24 of 24 | 6 914 of 6 914 | 12 of 12 | 180 of 180 | 24 of 24 | 144 of 144 |
| From the assignment | exploration 036's, run 43192758106, arm-0 and arm-1 | 24 of 24 | 24 of 24 | 6 914 of 6 914 | 12 of 12 | 180 of 180 | 24 of 24 | 144 of 144 |
| From the mirrored assignment | exploration 035's, run 43134386489, arm-0 and arm-1 | 24 of 24 | 24 of 24 | 6 932 of 6 932 | 12 of 12 | 180 of 180 | 24 of 24 | 144 of 144 |
| From the mirrored assignment | exploration 036's, run 43192758106, arm-2 and arm-3 | 24 of 24 | 24 of 24 | 6 932 of 6 932 | 12 of 12 | 180 of 180 | 24 of 24 | 144 of 144 |

Every night's image, highest and gain are each log's row, and every line its watch printed is the log's night's, in order, label aside: the
start of the night, each window, the watch's lines and the end. Every input read before and after a night is in each log that ran one.

## 2. What each replayed night wrote onto each readout

Each replayed night's input change less that of the night without replay from the same image: from each kind onto each readout's units, by
the units' class.

| The cell | Night, less the night without replay from the same image | From | Onto | C's or D's own, k | A's patterns' targets, k | Other excitatory, k | Inhibitory, k | All, k |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| From the assignment | After block 6 alone | A alone | A's answer's readout | +6.2 (+5.6 to +6.7) | +213.3 (+208.1 to +218.5) | +0.1 (−0.3 to +0.5) | −64.0 (−64.6 to −62.7) | +155.7 (+152.4 to +160.5) |
| From the assignment | After block 6 alone | A alone | B's answer's readout | −4.7 (−4.8 to −4.5) | −141.9 (−144.4 to −140.5) | +4.6 (+3.0 to +5.6) | −13.6 (−14.5 to −12.2) | −155.6 (−160.5 to −152.4) |
| From the assignment | After block 6 alone | A and C | A's answer's readout | +37.3 (+33.2 to +40.6) | +182.8 (+180.2 to +185.1) | 0.0 (0.0) | −31.3 (−33.3 to −30.0) | +188.8 (+182.9 to +195.0) |
| From the assignment | After block 6 alone | A and C | B's answer's readout | +0.6 (+0.4 to +0.9) | −180.1 (−185.2 to −174.0) | +2.0 (−0.1 to +4.8) | −11.3 (−12.5 to −9.4) | −188.8 (−195.0 to −182.9) |
| From the assignment | After block 6 alone | A's kinds | A's answer's readout | +43.5 (+38.8 to +46.6) | +396.1 (+388.3 to +403.6) | +0.1 (−0.3 to +0.5) | −95.3 (−97.9 to −92.7) | +344.4 (+337.0 to +355.5) |
| From the assignment | After block 6 alone | A's kinds | B's answer's readout | −4.1 (−4.3 to −3.7) | −322.0 (−329.6 to −314.8) | +6.5 (+4.2 to +9.8) | −24.9 (−26.4 to −23.6) | −344.4 (−355.5 to −337.0) |
| From the assignment | After block 6 alone | B alone | A's answer's readout | −28.2 (−29.0 to −27.8) | −84.2 (−86.7 to −83.0) | −9.5 (−10.4 to −9.0) | −28.1 (−29.1 to −26.3) | −150.1 (−153.6 to −147.8) |
| From the assignment | After block 6 alone | B alone | B's answer's readout | +34.3 (+32.8 to +35.5) | +145.3 (+143.1 to +147.6) | 0.0 (0.0) | −29.6 (−31.1 to −26.8) | +150.1 (+147.8 to +153.6) |
| From the assignment | After block 6 alone | B and D | A's answer's readout | +0.1 (+0.1) | −80.6 (−88.1 to −71.8) | −21.0 (−23.0 to −17.0) | −61.2 (−64.2 to −58.1) | −162.7 (−169.1 to −152.9) |
| From the assignment | After block 6 alone | B and D | B's answer's readout | +6.0 (+5.7 to +6.3) | +227.5 (+220.6 to +235.4) | 0.0 (0.0) | −70.8 (−73.4 to −66.5) | +162.8 (+152.9 to +169.1) |
| From the assignment | After block 6 alone | B's kinds | A's answer's readout | −28.1 (−28.9 to −27.6) | −164.9 (−174.7 to −154.9) | −30.5 (−32.2 to −27.4) | −89.3 (−90.5 to −86.9) | −312.8 (−322.7 to −300.7) |
| From the assignment | After block 6 alone | B's kinds | B's answer's readout | +40.3 (+38.9 to +41.2) | +372.8 (+363.7 to +382.9) | 0.0 (0.0) | −100.3 (−104.3 to −97.6) | +312.8 (+300.7 to +322.7) |
| From the assignment | After block 12 alone | A alone | A's answer's readout | +11.1 (+10.8 to +11.4) | +251.6 (+249.4 to +252.8) | −0.1 (−0.1) | −78.4 (−80.2 to −77.4) | +184.2 (+179.9 to +186.6) |
| From the assignment | After block 12 alone | A alone | B's answer's readout | −5.2 (−6.2 to −4.0) | −153.4 (−159.1 to −145.7) | +1.3 (0.0 to +2.1) | −26.9 (−28.0 to −25.3) | −184.2 (−186.6 to −179.9) |
| From the assignment | After block 12 alone | A and C | A's answer's readout | +39.1 (+38.1 to +39.7) | +123.1 (+120.6 to +126.5) | 0.0 (0.0) | −34.7 (−35.9 to −33.5) | +127.5 (+124.4 to +131.1) |
| From the assignment | After block 12 alone | A and C | B's answer's readout | −1.7 (−1.9 to −1.2) | −118.5 (−125.7 to −114.8) | −8.3 (−10.0 to −6.6) | +1.0 (−1.8 to +5.9) | −127.5 (−131.1 to −124.4) |
| From the assignment | After block 12 alone | A's kinds | A's answer's readout | +50.2 (+49.0 to +50.8) | +374.8 (+371.8 to +379.1) | −0.1 (−0.1) | −113.1 (−113.7 to −112.1) | +311.7 (+307.0 to +317.7) |
| From the assignment | After block 12 alone | A's kinds | B's answer's readout | −6.9 (−8.1 to −5.3) | −271.9 (−284.9 to −260.7) | −7.0 (−8.3 to −4.5) | −25.9 (−29.9 to −19.4) | −311.7 (−317.7 to −307.0) |
| From the assignment | After block 12 alone | B alone | A's answer's readout | −14.0 (−15.2 to −12.9) | −76.9 (−81.5 to −71.6) | +6.1 (+5.7 to +6.7) | −33.9 (−37.5 to −30.0) | −118.8 (−123.3 to −115.4) |
| From the assignment | After block 12 alone | B alone | B's answer's readout | +37.4 (+36.1 to +38.9) | +103.8 (+98.4 to +107.1) | +0.1 (+0.1) | −22.6 (−24.7 to −20.4) | +118.8 (+115.4 to +123.3) |
| From the assignment | After block 12 alone | B and D | A's answer's readout | −0.7 (−1.0 to −0.2) | −70.7 (−76.5 to −63.1) | −21.5 (−22.2 to −20.7) | −61.0 (−64.3 to −56.5) | −153.9 (−159.9 to −146.2) |
| From the assignment | After block 12 alone | B and D | B's answer's readout | +7.1 (+6.9 to +7.3) | +238.2 (+236.2 to +241.8) | 0.0 (0.0) | −91.3 (−97.2 to −87.9) | +153.9 (+146.3 to +159.9) |
| From the assignment | After block 12 alone | B's kinds | A's answer's readout | −14.7 (−16.0 to −13.9) | −147.6 (−158.0 to −140.8) | −15.4 (−16.5 to −14.8) | −95.0 (−99.7 to −86.5) | −272.7 (−275.3 to −269.6) |
| From the assignment | After block 12 alone | B's kinds | B's answer's readout | +44.5 (+43.4 to +45.8) | +342.0 (+340.2 to +343.6) | +0.1 (+0.1) | −114.0 (−119.9 to −109.3) | +272.7 (+269.6 to +275.3) |
| From the assignment | After block 18 alone | A alone | A's answer's readout | +5.5 (+5.2 to +5.9) | +211.8 (+208.1 to +215.4) | −0.2 (−0.3 to −0.2) | −68.2 (−71.3 to −66.6) | +149.0 (+146.0 to +154.5) |
| From the assignment | After block 18 alone | A alone | B's answer's readout | −3.8 (−5.3 to −2.8) | −131.7 (−135.0 to −127.2) | +6.6 (+4.7 to +7.8) | −20.1 (−23.5 to −16.0) | −149.0 (−154.4 to −146.0) |
| From the assignment | After block 18 alone | A and C | A's answer's readout | +42.3 (+40.6 to +44.7) | +149.0 (+143.4 to +152.3) | 0.0 (0.0) | −42.4 (−43.9 to −41.0) | +149.0 (+144.1 to +153.1) |
| From the assignment | After block 18 alone | A and C | B's answer's readout | −1.9 (−2.2 to −1.7) | −143.6 (−148.0 to −140.2) | −0.3 (−3.2 to +3.5) | −3.1 (−4.6 to −2.2) | −149.0 (−153.1 to −144.1) |
| From the assignment | After block 18 alone | A's kinds | A's answer's readout | +47.9 (+46.5 to +50.1) | +360.8 (+351.5 to +366.7) | −0.2 (−0.3 to −0.2) | −110.6 (−115.2 to −107.6) | +297.9 (+290.6 to +304.1) |
| From the assignment | After block 18 alone | A's kinds | B's answer's readout | −5.7 (−7.2 to −4.5) | −275.3 (−277.7 to −273.0) | +6.3 (+1.5 to +11.2) | −23.2 (−28.1 to −18.7) | −297.9 (−304.1 to −290.5) |
| From the assignment | After block 18 alone | B alone | A's answer's readout | −16.2 (−17.3 to −15.4) | −83.3 (−90.6 to −74.2) | +20.4 (+19.6 to +21.0) | −42.7 (−43.4 to −42.2) | −121.9 (−128.9 to −113.2) |
| From the assignment | After block 18 alone | B alone | B's answer's readout | −0.1 (−1.4 to +0.9) | +134.5 (+128.2 to +138.7) | −0.2 (−0.2) | −12.4 (−15.7 to −7.8) | +121.9 (+113.3 to +128.9) |
| From the assignment | After block 18 alone | B and D | A's answer's readout | 0.0 (−0.1 to +0.1) | −55.0 (−55.8 to −54.2) | −24.6 (−27.9 to −21.6) | −63.5 (−68.2 to −61.1) | −143.1 (−152.0 to −137.6) |
| From the assignment | After block 18 alone | B and D | B's answer's readout | +8.2 (+7.9 to +8.5) | +203.8 (+197.7 to +210.1) | 0.0 (0.0) | −68.8 (−78.3 to −59.6) | +143.1 (+137.5 to +152.0) |
| From the assignment | After block 18 alone | B's kinds | A's answer's readout | −16.3 (−17.4 to −15.5) | −138.2 (−144.7 to −129.1) | −4.2 (−6.8 to −2.0) | −106.2 (−110.4 to −103.8) | −265.0 (−275.5 to −250.8) |
| From the assignment | After block 18 alone | B's kinds | B's answer's readout | +8.1 (+6.7 to +9.4) | +338.3 (+325.8 to +346.9) | −0.2 (−0.2) | −81.2 (−86.1 to −73.2) | +265.0 (+250.8 to +275.5) |
| From the assignment | After block 24 alone | A alone | A's answer's readout | +3.4 (+3.0 to +4.0) | +201.0 (+198.0 to +202.8) | +0.2 (+0.1 to +0.2) | −80.1 (−81.2 to −78.6) | +124.4 (+121.5 to +127.6) |
| From the assignment | After block 24 alone | A alone | B's answer's readout | −3.5 (−4.5 to −1.7) | −114.4 (−116.4 to −112.9) | +22.0 (+19.6 to +24.2) | −28.5 (−31.0 to −26.5) | −124.4 (−127.6 to −121.5) |
| From the assignment | After block 24 alone | A and C | A's answer's readout | +10.1 (+9.2 to +11.3) | +145.4 (+135.6 to +152.1) | 0.0 (0.0) | −35.1 (−37.0 to −33.3) | +120.4 (+113.6 to +125.0) |
| From the assignment | After block 24 alone | A and C | B's answer's readout | −0.3 (−1.2 to +0.1) | −124.8 (−130.2 to −116.8) | +13.8 (+10.1 to +17.5) | −9.1 (−13.2 to −6.2) | −120.5 (−125.1 to −113.7) |
| From the assignment | After block 24 alone | A's kinds | A's answer's readout | +13.5 (+12.2 to +14.4) | +346.4 (+338.4 to +350.6) | +0.2 (+0.1 to +0.2) | −115.2 (−117.7 to −111.8) | +244.9 (+241.2 to +246.8) |
| From the assignment | After block 24 alone | A's kinds | B's answer's readout | −3.9 (−5.6 to −1.6) | −239.2 (−244.0 to −233.2) | +35.8 (+29.8 to +41.7) | −37.6 (−44.2 to −34.1) | −244.9 (−246.8 to −241.3) |
| From the assignment | After block 24 alone | B alone | A's answer's readout | −29.3 (−31.8 to −25.3) | −97.7 (−98.2 to −96.6) | −12.6 (−14.4 to −11.1) | −42.6 (−48.6 to −37.5) | −182.1 (−189.7 to −174.8) |
| From the assignment | After block 24 alone | B alone | B's answer's readout | +32.0 (+28.9 to +34.8) | +180.0 (+175.0 to +187.1) | 0.0 (0.0 to +0.1) | −29.9 (−32.2 to −25.6) | +182.1 (+174.8 to +189.7) |
| From the assignment | After block 24 alone | B and D | A's answer's readout | +0.1 (0.0 to +0.2) | −66.8 (−75.4 to −59.5) | −13.8 (−18.8 to −7.3) | −56.8 (−63.9 to −49.8) | −137.2 (−158.1 to −116.5) |
| From the assignment | After block 24 alone | B and D | B's answer's readout | +7.4 (+7.0 to +7.7) | +193.4 (+180.3 to +203.8) | 0.0 (0.0) | −63.6 (−70.8 to −53.2) | +137.2 (+116.5 to +158.1) |
| From the assignment | After block 24 alone | B's kinds | A's answer's readout | −29.2 (−31.6 to −25.3) | −164.4 (−172.0 to −157.7) | −26.4 (−29.9 to −21.7) | −99.4 (−105.7 to −87.3) | −319.4 (−332.9 to −298.4) |
| From the assignment | After block 24 alone | B's kinds | B's answer's readout | +39.4 (+36.4 to +42.5) | +373.4 (+355.3 to +383.3) | 0.0 (0.0 to +0.1) | −93.5 (−99.0 to −85.1) | +319.4 (+298.4 to +332.9) |
| From the mirrored assignment | After block 6 alone | A alone | A's answer's readout | +29.1 (+25.9 to +32.3) | +177.4 (+175.9 to +179.1) | −0.2 (−0.2) | −21.4 (−23.0 to −20.1) | +184.9 (+183.1 to +186.8) |
| From the mirrored assignment | After block 6 alone | A alone | B's answer's readout | −0.5 (−0.7 to −0.4) | −57.4 (−58.1 to −56.7) | −35.7 (−37.1 to −34.3) | −91.2 (−91.6 to −90.9) | −184.8 (−186.7 to −183.0) |
| From the mirrored assignment | After block 6 alone | A and C | A's answer's readout | +16.3 (+15.7 to +17.2) | +186.5 (+181.8 to +192.6) | 0.0 (0.0) | −53.0 (−56.8 to −46.4) | +149.8 (+141.0 to +154.4) |
| From the mirrored assignment | After block 6 alone | A and C | B's answer's readout | −12.8 (−16.0 to −10.5) | −88.1 (−92.3 to −85.9) | −15.9 (−20.9 to −11.1) | −33.0 (−34.5 to −31.3) | −149.8 (−154.4 to −141.0) |
| From the mirrored assignment | After block 6 alone | A's kinds | A's answer's readout | +45.4 (+41.9 to +48.0) | +363.9 (+360.9 to +369.7) | −0.2 (−0.2) | −74.4 (−78.6 to −67.6) | +334.7 (+325.7 to +341.2) |
| From the mirrored assignment | After block 6 alone | A's kinds | B's answer's readout | −13.3 (−16.4 to −10.9) | −145.6 (−150.4 to −142.6) | −51.6 (−55.2 to −46.8) | −124.2 (−125.4 to −122.9) | −334.7 (−341.1 to −325.7) |
| From the mirrored assignment | After block 6 alone | B alone | A's answer's readout | −23.3 (−24.4 to −22.6) | −59.1 (−59.7 to −58.5) | +2.4 (+1.4 to +3.0) | −20.3 (−23.2 to −18.7) | −100.4 (−103.5 to −97.8) |
| From the mirrored assignment | After block 6 alone | B alone | B's answer's readout | +26.5 (+25.6 to +27.9) | +149.1 (+145.3 to +154.1) | +0.1 (0.0 to +0.1) | −75.3 (−76.8 to −73.6) | +100.4 (+97.8 to +103.5) |
| From the mirrored assignment | After block 6 alone | B and D | A's answer's readout | 0.0 (−0.1 to +0.1) | −100.7 (−107.5 to −95.5) | −15.0 (−17.7 to −12.6) | −52.8 (−57.2 to −48.3) | −168.5 (−179.3 to −159.9) |
| From the mirrored assignment | After block 6 alone | B and D | B's answer's readout | +2.7 (+2.1 to +3.1) | +241.4 (+239.1 to +243.6) | 0.0 (0.0) | −75.6 (−81.4 to −67.1) | +168.5 (+159.9 to +179.3) |
| From the mirrored assignment | After block 6 alone | B's kinds | A's answer's readout | −23.3 (−24.5 to −22.5) | −159.8 (−166.0 to −155.2) | −12.7 (−14.7 to −11.2) | −73.2 (−76.3 to −71.5) | −268.9 (−277.1 to −263.4) |
| From the mirrored assignment | After block 6 alone | B's kinds | B's answer's readout | +29.2 (+28.2 to +30.7) | +390.5 (+388.9 to +393.2) | +0.1 (0.0 to +0.1) | −150.9 (−158.1 to −142.6) | +268.9 (+263.4 to +277.1) |
| From the mirrored assignment | After block 12 alone | A alone | A's answer's readout | +48.5 (+47.3 to +50.8) | +226.1 (+224.6 to +229.1) | +0.1 (+0.1) | −39.2 (−39.7 to −39.0) | +235.6 (+232.5 to +237.7) |
| From the mirrored assignment | After block 12 alone | A alone | B's answer's readout | −0.2 (−0.6 to +0.1) | −84.0 (−86.9 to −81.7) | −58.0 (−58.3 to −57.4) | −93.3 (−97.3 to −91.0) | −235.5 (−237.7 to −232.5) |
| From the mirrored assignment | After block 12 alone | A and C | A's answer's readout | +13.7 (+13.2 to +14.5) | +202.5 (+199.2 to +204.3) | 0.0 (0.0) | −45.6 (−46.3 to −45.2) | +170.6 (+167.5 to +172.2) |
| From the mirrored assignment | After block 12 alone | A and C | B's answer's readout | −11.3 (−12.7 to −10.6) | −91.8 (−93.6 to −90.7) | −29.2 (−30.6 to −27.8) | −38.2 (−43.0 to −35.1) | −170.6 (−172.2 to −167.5) |
| From the mirrored assignment | After block 12 alone | A's kinds | A's answer's readout | +62.2 (+60.5 to +65.4) | +428.6 (+428.4 to +429.0) | +0.1 (+0.1) | −84.8 (−85.4 to −84.2) | +406.1 (+404.7 to +408.5) |
| From the mirrored assignment | After block 12 alone | A's kinds | B's answer's readout | −11.6 (−12.9 to −10.6) | −175.8 (−180.5 to −172.9) | −87.2 (−88.7 to −85.2) | −131.5 (−134.8 to −127.4) | −406.1 (−408.5 to −404.7) |
| From the mirrored assignment | After block 12 alone | B alone | A's answer's readout | −8.2 (−9.7 to −6.0) | −63.8 (−64.7 to −62.6) | +3.9 (+2.1 to +4.9) | −21.0 (−23.0 to −17.3) | −89.1 (−92.9 to −82.7) |
| From the mirrored assignment | After block 12 alone | B alone | B's answer's readout | +12.7 (+9.7 to +14.5) | +122.4 (+119.6 to +124.8) | −0.2 (−0.2 to −0.1) | −45.8 (−46.4 to −45.4) | +89.1 (+82.7 to +92.9) |
| From the mirrored assignment | After block 12 alone | B and D | A's answer's readout | −0.9 (−1.0 to −0.8) | −108.4 (−109.8 to −107.4) | −11.0 (−14.3 to −8.9) | −52.6 (−59.4 to −45.4) | −172.9 (−178.1 to −162.5) |
| From the mirrored assignment | After block 12 alone | B and D | B's answer's readout | +4.2 (+4.0 to +4.5) | +258.2 (+256.0 to +259.4) | 0.0 (0.0) | −89.5 (−97.4 to −85.5) | +172.9 (+162.5 to +178.1) |
| From the mirrored assignment | After block 12 alone | B's kinds | A's answer's readout | −9.1 (−10.7 to −6.8) | −172.2 (−172.6 to −171.6) | −7.0 (−12.2 to −4.1) | −73.6 (−82.4 to −62.7) | −262.0 (−271.1 to −245.2) |
| From the mirrored assignment | After block 12 alone | B's kinds | B's answer's readout | +16.9 (+13.7 to +18.8) | +380.6 (+375.5 to +384.1) | −0.2 (−0.2 to −0.1) | −135.3 (−143.8 to −130.9) | +262.1 (+245.2 to +271.1) |
| From the mirrored assignment | After block 18 alone | A alone | A's answer's readout | +30.7 (+29.0 to +32.3) | +181.6 (+177.6 to +184.6) | +0.2 (+0.2 to +0.3) | −12.8 (−14.8 to −11.6) | +199.8 (+197.1 to +202.4) |
| From the mirrored assignment | After block 18 alone | A alone | B's answer's readout | −0.7 (−0.9 to −0.5) | −54.4 (−55.2 to −53.8) | −33.3 (−34.4 to −31.6) | −111.5 (−114.9 to −108.6) | −199.8 (−202.3 to −197.1) |
| From the mirrored assignment | After block 18 alone | A and C | A's answer's readout | +16.2 (+15.3 to +17.6) | +218.9 (+207.9 to +226.4) | 0.0 (0.0) | −37.7 (−39.1 to −36.1) | +197.4 (+184.2 to +206.1) |
| From the mirrored assignment | After block 18 alone | A and C | B's answer's readout | −18.0 (−18.3 to −17.6) | −107.1 (−111.8 to −100.7) | −38.5 (−42.2 to −32.7) | −33.9 (−35.5 to −32.8) | −197.4 (−206.1 to −184.2) |
| From the mirrored assignment | After block 18 alone | A's kinds | A's answer's readout | +47.0 (+44.3 to +48.5) | +400.5 (+390.7 to +411.0) | +0.2 (+0.2 to +0.3) | −50.4 (−51.0 to −49.4) | +397.2 (+384.2 to +408.4) |
| From the mirrored assignment | After block 18 alone | A's kinds | B's answer's readout | −18.6 (−19.0 to −18.1) | −161.5 (−167.0 to −154.8) | −71.8 (−76.1 to −67.1) | −145.4 (−150.4 to −141.4) | −397.3 (−408.4 to −384.2) |
| From the mirrored assignment | After block 18 alone | B alone | A's answer's readout | −25.0 (−26.8 to −21.9) | −64.0 (−65.7 to −60.7) | −1.1 (−2.0 to −0.2) | −20.4 (−23.5 to −18.7) | −110.5 (−113.2 to −106.4) |
| From the mirrored assignment | After block 18 alone | B alone | B's answer's readout | +4.2 (+1.6 to +6.2) | +160.8 (+157.5 to +164.0) | +0.2 (+0.1 to +0.5) | −54.7 (−56.2 to −52.7) | +110.5 (+106.4 to +113.2) |
| From the mirrored assignment | After block 18 alone | B and D | A's answer's readout | +0.1 (−0.1 to +0.2) | −103.8 (−113.9 to −93.6) | −11.4 (−14.7 to −9.6) | −39.1 (−49.0 to −33.1) | −154.2 (−167.5 to −136.1) |
| From the mirrored assignment | After block 18 alone | B and D | B's answer's readout | +5.6 (+4.9 to +6.3) | +204.6 (+194.5 to +214.7) | 0.0 (0.0) | −56.0 (−63.9 to −50.6) | +154.2 (+136.1 to +167.5) |
| From the mirrored assignment | After block 18 alone | B's kinds | A's answer's readout | −25.0 (−26.6 to −22.0) | −167.9 (−179.5 to −159.4) | −12.5 (−15.0 to −10.9) | −59.5 (−68.1 to −51.7) | −264.8 (−273.9 to −249.3) |
| From the mirrored assignment | After block 18 alone | B's kinds | B's answer's readout | +9.8 (+7.8 to +11.2) | +365.4 (+358.5 to +372.1) | +0.2 (+0.1 to +0.5) | −110.7 (−120.1 to −105.8) | +264.7 (+249.3 to +273.9) |
| From the mirrored assignment | After block 24 alone | A alone | A's answer's readout | +33.1 (+30.1 to +35.6) | +191.6 (+185.8 to +197.1) | −0.3 (−0.3) | −21.4 (−22.3 to −19.7) | +203.0 (+196.8 to +207.2) |
| From the mirrored assignment | After block 24 alone | A alone | B's answer's readout | −2.3 (−2.5 to −2.0) | −79.5 (−84.3 to −72.6) | −40.2 (−42.9 to −38.8) | −81.1 (−83.0 to −80.1) | −203.0 (−207.2 to −196.8) |
| From the mirrored assignment | After block 24 alone | A and C | A's answer's readout | +10.5 (+8.9 to +11.7) | +150.4 (+138.0 to +165.9) | 0.0 (0.0) | −19.8 (−20.5 to −18.7) | +141.1 (+126.7 to +158.2) |
| From the mirrored assignment | After block 24 alone | A and C | B's answer's readout | −15.4 (−17.5 to −13.0) | −79.7 (−91.0 to −71.9) | −18.4 (−19.2 to −17.2) | −27.7 (−30.5 to −22.0) | −141.2 (−158.2 to −126.7) |
| From the mirrored assignment | After block 24 alone | A's kinds | A's answer's readout | +43.6 (+41.0 to +47.3) | +342.1 (+323.8 to +363.0) | −0.3 (−0.3) | −41.2 (−42.7 to −38.3) | +344.2 (+323.5 to +365.4) |
| From the mirrored assignment | After block 24 alone | A's kinds | B's answer's readout | −17.6 (−20.0 to −14.9) | −159.2 (−172.4 to −144.5) | −58.6 (−62.1 to −56.1) | −108.8 (−110.9 to −104.9) | −344.2 (−365.4 to −323.5) |
| From the mirrored assignment | After block 24 alone | B alone | A's answer's readout | −14.1 (−16.0 to −13.1) | −88.6 (−92.9 to −85.3) | +3.5 (+2.5 to +4.0) | −28.8 (−29.5 to −27.6) | −128.0 (−131.3 to −126.0) |
| From the mirrored assignment | After block 24 alone | B alone | B's answer's readout | +15.1 (+11.8 to +18.0) | +169.1 (+165.0 to +171.9) | +0.5 (+0.4 to +0.6) | −56.7 (−57.3 to −56.3) | +128.0 (+126.0 to +131.3) |
| From the mirrored assignment | After block 24 alone | B and D | A's answer's readout | +0.2 (+0.1 to +0.3) | −130.0 (−139.8 to −112.1) | −19.8 (−20.3 to −19.5) | −72.2 (−77.6 to −68.7) | −221.9 (−236.1 to −201.8) |
| From the mirrored assignment | After block 24 alone | B and D | B's answer's readout | +2.8 (+2.3 to +3.2) | +279.1 (+264.1 to +288.9) | 0.0 (0.0) | −59.9 (−65.2 to −50.3) | +221.9 (+201.9 to +236.1) |
| From the mirrored assignment | After block 24 alone | B's kinds | A's answer's readout | −13.9 (−15.8 to −12.8) | −218.6 (−231.2 to −197.4) | −16.4 (−17.9 to −15.5) | −101.0 (−105.2 to −98.1) | −349.9 (−367.4 to −328.4) |
| From the mirrored assignment | After block 24 alone | B's kinds | B's answer's readout | +17.9 (+14.8 to +21.2) | +448.1 (+434.5 to +456.0) | +0.5 (+0.4 to +0.6) | −116.7 (−121.6 to −107.0) | +349.9 (+328.5 to +367.4) |

**A's answer's readout less B's**, by the units' class, from A's kinds and from B's:

| The cell | Night, less the night without replay from the same image | From | C's or D's own: A's answer's readout's less B's, k | A's patterns' targets: A's answer's readout's less B's, k | Other excitatory: A's answer's readout's less B's, k | Inhibitory: A's answer's readout's less B's, k | All: A's answer's readout's less B's, k |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| From the assignment | After block 6 alone | A's kinds | +47.5 (+43.0 to +50.3) | +718.1 (+709.9 to +733.2) | −6.4 (−9.6 to −3.7) | −70.4 (−74.3 to −67.9) | +688.9 (+674.0 to +711.0) |
| From the assignment | After block 6 alone | B's kinds | −68.5 (−69.1 to −67.8) | −537.7 (−557.7 to −518.6) | −30.5 (−32.2 to −27.4) | +11.1 (+7.2 to +13.8) | −625.6 (−645.4 to −601.4) |
| From the assignment | After block 12 alone | A's kinds | +57.1 (+56.1 to +58.0) | +646.7 (+632.5 to +664.0) | +6.9 (+4.4 to +8.2) | −87.2 (−92.7 to −83.8) | +623.4 (+614.0 to +635.4) |
| From the assignment | After block 12 alone | B's kinds | −59.2 (−60.3 to −57.3) | −489.7 (−500.2 to −484.3) | −15.5 (−16.6 to −14.9) | +19.0 (+10.7 to +26.1) | −545.3 (−550.6 to −539.1) |
| From the assignment | After block 18 alone | A's kinds | +53.6 (+51.0 to +57.3) | +636.1 (+629.2 to +639.7) | −6.5 (−11.4 to −1.7) | −87.4 (−92.3 to −80.8) | +595.9 (+581.1 to +608.2) |
| From the assignment | After block 18 alone | B's kinds | −24.3 (−25.3 to −23.5) | −476.5 (−491.6 to −454.9) | −4.1 (−6.7 to −1.9) | −25.0 (−37.1 to −18.4) | −530.0 (−551.0 to −501.6) |
| From the assignment | After block 24 alone | A's kinds | +17.4 (+15.6 to +20.0) | +585.6 (+571.7 to +594.6) | −35.6 (−41.6 to −29.6) | −77.7 (−83.3 to −67.6) | +489.8 (+482.5 to +493.6) |
| From the assignment | After block 24 alone | B's kinds | −68.6 (−73.0 to −61.7) | −537.9 (−553.6 to −513.1) | −26.4 (−30.0 to −21.8) | −5.9 (−20.6 to +9.1) | −638.7 (−665.8 to −596.8) |
| From the mirrored assignment | After block 6 alone | A's kinds | +58.7 (+52.8 to +62.7) | +509.5 (+504.6 to +512.4) | +51.4 (+46.6 to +55.0) | +49.8 (+44.3 to +57.7) | +669.4 (+651.4 to +682.3) |
| From the mirrored assignment | After block 6 alone | B's kinds | −52.5 (−53.5 to −50.8) | −550.3 (−554.9 to −544.7) | −12.7 (−14.8 to −11.3) | +77.7 (+66.3 to +86.6) | −537.8 (−554.2 to −526.8) |
| From the mirrored assignment | After block 12 alone | A's kinds | +73.8 (+71.1 to +78.3) | +604.4 (+601.2 to +608.9) | +87.3 (+85.3 to +88.8) | +46.7 (+42.1 to +49.9) | +812.2 (+809.4 to +816.9) |
| From the mirrored assignment | After block 12 alone | B's kinds | −26.1 (−28.9 to −20.5) | −552.8 (−556.5 to −547.2) | −6.9 (−12.1 to −3.8) | +61.7 (+48.5 to +81.1) | −524.1 (−542.2 to −490.4) |
| From the mirrored assignment | After block 18 alone | A's kinds | +65.6 (+62.4 to +67.4) | +562.0 (+545.4 to +578.0) | +72.0 (+67.3 to +76.3) | +94.9 (+92.0 to +99.5) | +794.5 (+768.3 to +816.9) |
| From the mirrored assignment | After block 18 alone | B's kinds | −34.7 (−36.9 to −33.2) | −533.3 (−545.0 to −517.9) | −12.7 (−15.0 to −11.1) | +51.2 (+38.1 to +68.3) | −529.5 (−547.8 to −498.5) |
| From the mirrored assignment | After block 24 alone | A's kinds | +61.2 (+60.5 to +62.2) | +501.2 (+468.4 to +535.4) | +58.3 (+55.8 to +61.8) | +67.6 (+62.4 to +72.6) | +688.4 (+647.0 to +730.8) |
| From the mirrored assignment | After block 24 alone | B's kinds | −31.8 (−34.0 to −30.5) | −666.8 (−687.2 to −631.9) | −16.9 (−18.4 to −16.1) | +15.6 (+1.7 to +23.4) | −699.8 (−734.8 to −656.9) |

From A's kinds onto A's answer's readout less B's: on the image from the assignment +688.9, +623.4, +595.9 and +489.8 k after block 6, 12,
18 and 24 alone, and on the image from the mirrored assignment +669.4, +812.2, +794.5 and +688.4. From B's kinds onto B's answer's readout
less A's: +625.6, +545.3, +530.0 and +638.7 and +537.8, +524.1, +529.5 and +699.8. Onto A's answer's readout alone, A's kinds moved +344.4,
+311.7, +297.9 and +244.9 and +334.7, +406.1, +397.2 and +344.2, and onto B's answer's readout the negation of those within 0.040 k. On
every one of the 48 nights each kind's change summed over the two readouts lies within 121 of the class matrix's units of zero.

By class, from A's kinds onto A's answer's readout less B's: A's patterns' targets +718.1, +646.7, +636.1 and +585.6 on the image from the
assignment and +509.5, +604.4, +562.0 and +501.2 on the image from the mirrored assignment; the other excitatory units −6.4, +6.9, −6.5 and
−35.6 and +51.4, +87.3, +72.0 and +58.3; the inhibitory units −70.4, −87.2, −87.4 and −77.7 and +49.8, +46.7, +94.9 and +67.6; C's or D's
own +47.5, +57.1, +53.6 and +17.4 and +58.7, +73.8, +65.6 and +61.2.

## 3. How it was written

The same differences, split into what moved them: consolidation in each bin, the zero sum's rounds, and what the watch does not see. Onto
each readout, from A's kinds and from B's:

| The cell | Night, less the night without replay from the same image | From | Onto | The inputs' change, k | Consolidation in its own replay, k | at its next spike, k | at another in slow-wave sleep, k | at another outside it, k | below zero, k | The zero sum's rounds, k | What the watch does not see, k |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| From the assignment | After block 6 alone | A's kinds | A's answer's readout | +344.4 (+337.0 to +355.5) | +180.8 (+176.1 to +188.8) | +168.1 (+167.0 to +169.7) | −24.2 (−28.4 to −20.7) | 0.0 (0.0) | 0.0 (0.0) | +19.9 (+15.7 to +27.7) | 0.0 (0.0) |
| From the assignment | After block 6 alone | A's kinds | B's answer's readout | −344.4 (−355.5 to −337.0) | −183.8 (−189.6 to −176.0) | +21.2 (+20.7 to +21.6) | −210.1 (−221.3 to −197.8) | 0.0 (0.0) | 0.0 (0.0) | +28.2 (+25.0 to +34.0) | 0.0 (0.0) |
| From the assignment | After block 6 alone | A's kinds | A's answer's readout less B's | +688.9 (+674.0 to +711.0) | +364.5 (+352.1 to +378.3) | +146.8 (+145.4 to +149.0) | +185.9 (+174.2 to +192.9) | 0.0 (0.0) | 0.0 (0.0) | −8.3 (−10.0 to −6.3) | 0.0 (0.0) |
| From the assignment | After block 6 alone | B's kinds | A's answer's readout | −312.8 (−322.7 to −300.7) | −211.4 (−214.7 to −208.0) | +56.8 (+51.4 to +60.7) | −133.5 (−149.4 to −114.7) | 0.0 (0.0) | 0.0 (0.0) | −24.6 (−35.0 to −15.3) | 0.0 (0.0) |
| From the assignment | After block 6 alone | B's kinds | B's answer's readout | +312.8 (+300.7 to +322.7) | +233.1 (+228.4 to +240.0) | +185.9 (+184.6 to +188.3) | −75.0 (−79.0 to −70.5) | 0.0 (0.0) | 0.0 (0.0) | −31.1 (−40.4 to −21.6) | 0.0 (0.0) |
| From the assignment | After block 6 alone | B's kinds | A's answer's readout less B's | −625.6 (−645.4 to −601.4) | −444.5 (−448.0 to −440.0) | −129.1 (−133.5 to −126.2) | −58.5 (−78.9 to −39.0) | 0.0 (0.0) | 0.0 (0.0) | +6.5 (+5.4 to +7.7) | 0.0 (0.0) |
| From the assignment | After block 12 alone | A's kinds | A's answer's readout | +311.7 (+307.0 to +317.7) | +208.0 (+204.8 to +210.4) | +185.3 (+182.6 to +187.0) | −50.9 (−52.7 to −50.0) | 0.0 (0.0) | 0.0 (0.0) | −30.7 (−35.8 to −21.5) | 0.0 (0.0) |
| From the assignment | After block 12 alone | A's kinds | B's answer's readout | −311.7 (−317.7 to −307.0) | −178.3 (−182.5 to −172.3) | +24.3 (+24.2 to +24.6) | −138.6 (−149.2 to −131.7) | 0.0 (0.0) | 0.0 (0.0) | −19.1 (−24.0 to −10.5) | 0.0 (0.0) |
| From the assignment | After block 12 alone | A's kinds | A's answer's readout less B's | +623.4 (+614.0 to +635.4) | +386.3 (+382.7 to +388.8) | +161.0 (+158.3 to +162.5) | +87.7 (+81.7 to +96.6) | 0.0 (0.0) | 0.0 (0.0) | −11.5 (−11.8 to −10.9) | 0.0 (0.0) |
| From the assignment | After block 12 alone | B's kinds | A's answer's readout | −272.7 (−275.3 to −269.6) | −177.3 (−179.0 to −174.3) | +36.4 (+32.3 to +42.1) | −141.3 (−147.1 to −135.0) | 0.0 (0.0) | 0.0 (0.0) | +9.5 (+5.9 to +12.1) | 0.0 (0.0) |
| From the assignment | After block 12 alone | B's kinds | B's answer's readout | +272.7 (+269.6 to +275.3) | +187.4 (+183.8 to +194.6) | +176.5 (+173.3 to +181.5) | −90.3 (−92.6 to −87.8) | 0.0 (0.0) | 0.0 (0.0) | −0.9 (−4.8 to +1.6) | 0.0 (0.0) |
| From the assignment | After block 12 alone | B's kinds | A's answer's readout less B's | −545.3 (−550.6 to −539.1) | −364.7 (−368.8 to −362.5) | −140.0 (−142.3 to −138.4) | −51.0 (−54.5 to −44.6) | 0.0 (0.0) | 0.0 (0.0) | +10.4 (+10.1 to +10.6) | 0.0 (0.0) |
| From the assignment | After block 18 alone | A's kinds | A's answer's readout | +297.9 (+290.6 to +304.1) | +147.0 (+140.8 to +153.2) | +165.1 (+163.9 to +165.7) | −84.1 (−89.5 to −77.3) | 0.0 (0.0) | 0.0 (0.0) | +69.9 (+62.6 to +79.8) | 0.0 (0.0) |
| From the assignment | After block 18 alone | A's kinds | B's answer's readout | −297.9 (−304.1 to −290.5) | −142.4 (−146.0 to −136.3) | +35.6 (+32.1 to +38.9) | −252.7 (−261.4 to −247.6) | 0.0 (0.0) | 0.0 (0.0) | +61.6 (+53.8 to +68.3) | 0.0 (0.0) |
| From the assignment | After block 18 alone | A's kinds | A's answer's readout less B's | +595.9 (+581.1 to +608.2) | +289.4 (+283.4 to +299.2) | +129.5 (+125.0 to +133.6) | +168.6 (+159.5 to +176.0) | 0.0 (0.0) | 0.0 (0.0) | +8.3 (+4.7 to +11.5) | 0.0 (0.0) |
| From the assignment | After block 18 alone | B's kinds | A's answer's readout | −265.0 (−275.5 to −250.8) | −191.0 (−199.3 to −174.9) | +44.7 (+41.3 to +49.3) | −136.9 (−143.0 to −131.4) | 0.0 (0.0) | 0.0 (0.0) | +18.2 (+6.2 to +25.5) | 0.0 (0.0) |
| From the assignment | After block 18 alone | B's kinds | B's answer's readout | +265.0 (+250.8 to +275.5) | +190.7 (+184.1 to +195.2) | +159.0 (+153.3 to +163.0) | −90.9 (−98.8 to −83.4) | 0.0 (0.0) | 0.0 (0.0) | +6.1 (−3.8 to +11.8) | 0.0 (0.0) |
| From the assignment | After block 18 alone | B's kinds | A's answer's readout less B's | −530.0 (−551.0 to −501.6) | −381.7 (−394.0 to −367.7) | −114.2 (−121.7 to −109.7) | −46.0 (−59.6 to −32.6) | 0.0 (0.0) | 0.0 (0.0) | +12.1 (+10.1 to +13.7) | 0.0 (0.0) |
| From the assignment | After block 24 alone | A's kinds | A's answer's readout | +244.9 (+241.2 to +246.8) | +135.1 (+130.0 to +139.1) | +181.9 (+179.0 to +185.2) | −122.1 (−129.1 to −117.5) | 0.0 (0.0) | 0.0 (0.0) | +49.9 (+43.7 to +60.4) | 0.0 (0.0) |
| From the assignment | After block 24 alone | A's kinds | B's answer's readout | −244.9 (−246.8 to −241.3) | −129.7 (−138.8 to −120.9) | +51.0 (+49.5 to +54.0) | −237.6 (−246.7 to −229.9) | 0.0 (0.0) | 0.0 (0.0) | +71.4 (+66.4 to +80.0) | 0.0 (0.0) |
| From the assignment | After block 24 alone | A's kinds | A's answer's readout less B's | +489.8 (+482.5 to +493.6) | +264.8 (+257.1 to +277.8) | +130.9 (+127.6 to +135.8) | +115.5 (+112.4 to +117.6) | 0.0 (0.0) | 0.0 (0.0) | −21.5 (−24.2 to −19.6) | 0.0 (0.0) |
| From the assignment | After block 24 alone | B's kinds | A's answer's readout | −319.4 (−332.9 to −298.4) | −171.5 (−179.3 to −166.3) | +69.7 (+68.2 to +72.0) | −166.3 (−205.6 to −142.1) | 0.0 (0.0) | 0.0 (0.0) | −51.2 (−64.4 to −33.1) | 0.0 (0.0) |
| From the assignment | After block 24 alone | B's kinds | B's answer's readout | +319.4 (+298.4 to +332.9) | +291.6 (+281.1 to +304.0) | +188.6 (+188.1 to +189.5) | −99.3 (−105.0 to −93.3) | 0.0 (0.0) | 0.0 (0.0) | −61.5 (−73.4 to −45.1) | 0.0 (0.0) |
| From the assignment | After block 24 alone | B's kinds | A's answer's readout less B's | −638.7 (−665.8 to −596.8) | −463.1 (−483.2 to −450.2) | −118.9 (−121.3 to −116.1) | −67.0 (−105.9 to −37.1) | 0.0 (0.0) | 0.0 (0.0) | +10.3 (+9.0 to +12.0) | 0.0 (0.0) |
| From the mirrored assignment | After block 6 alone | A's kinds | A's answer's readout | +334.7 (+325.7 to +341.2) | +258.9 (+251.5 to +270.2) | +184.8 (+180.6 to +192.2) | −103.9 (−116.2 to −93.8) | 0.0 (0.0) | 0.0 (0.0) | −5.0 (−19.4 to +9.9) | 0.0 (0.0) |
| From the mirrored assignment | After block 6 alone | A's kinds | B's answer's readout | −334.7 (−341.1 to −325.7) | −251.6 (−256.0 to −246.2) | +36.2 (+32.8 to +39.6) | −121.4 (−126.2 to −115.4) | 0.0 (0.0) | 0.0 (0.0) | +2.2 (−11.9 to +16.8) | 0.0 (0.0) |
| From the mirrored assignment | After block 6 alone | A's kinds | A's answer's readout less B's | +669.4 (+651.4 to +682.3) | +510.4 (+504.0 to +516.4) | +148.5 (+144.4 to +152.5) | +17.5 (+10.0 to +21.6) | 0.0 (0.0) | 0.0 (0.0) | −7.1 (−7.5 to −6.9) | 0.0 (0.0) |
| From the mirrored assignment | After block 6 alone | B's kinds | A's answer's readout | −268.9 (−277.1 to −263.4) | −156.9 (−161.0 to −152.0) | +30.9 (+27.9 to +33.6) | −79.7 (−105.5 to −61.6) | 0.0 (0.0) | 0.0 (0.0) | −63.2 (−83.8 to −38.4) | 0.0 (0.0) |
| From the mirrored assignment | After block 6 alone | B's kinds | B's answer's readout | +268.9 (+263.4 to +277.1) | +255.1 (+254.0 to +256.7) | +155.1 (+154.1 to +156.8) | −61.8 (−68.8 to −55.5) | 0.0 (0.0) | 0.0 (0.0) | −79.6 (−91.8 to −63.4) | 0.0 (0.0) |
| From the mirrored assignment | After block 6 alone | B's kinds | A's answer's readout less B's | −537.8 (−554.2 to −526.8) | −412.0 (−415.7 to −408.7) | −124.2 (−126.6 to −120.5) | −17.9 (−36.7 to −6.1) | 0.0 (0.0) | 0.0 (0.0) | +16.4 (+7.9 to +24.9) | 0.0 (0.0) |
| From the mirrored assignment | After block 12 alone | A's kinds | A's answer's readout | +406.1 (+404.7 to +408.5) | +348.0 (+343.1 to +350.9) | +192.5 (+187.8 to +196.4) | −70.2 (−74.0 to −64.8) | 0.0 (0.0) | 0.0 (0.0) | −64.2 (−67.9 to −57.7) | 0.0 (0.0) |
| From the mirrored assignment | After block 12 alone | A's kinds | B's answer's readout | −406.1 (−408.5 to −404.7) | −268.2 (−273.2 to −261.1) | +38.7 (+34.5 to +42.4) | −122.8 (−126.6 to −116.5) | 0.0 (0.0) | 0.0 (0.0) | −53.8 (−57.9 to −46.1) | 0.0 (0.0) |
| From the mirrored assignment | After block 12 alone | A's kinds | A's answer's readout less B's | +812.2 (+809.4 to +816.9) | +616.3 (+611.3 to +624.0) | +153.8 (+150.9 to +157.3) | +52.6 (+44.7 to +61.8) | 0.0 (0.0) | 0.0 (0.0) | −10.4 (−11.6 to −9.2) | 0.0 (0.0) |
| From the mirrored assignment | After block 12 alone | B's kinds | A's answer's readout | −262.0 (−271.1 to −245.2) | −198.3 (−205.8 to −189.2) | +40.9 (+39.9 to +41.9) | −82.2 (−84.7 to −80.2) | 0.0 (0.0) | 0.0 (0.0) | −22.5 (−37.8 to −6.2) | 0.0 (0.0) |
| From the mirrored assignment | After block 12 alone | B's kinds | B's answer's readout | +262.1 (+245.2 to +271.1) | +188.6 (+178.3 to +198.9) | +178.6 (+176.2 to +181.0) | −77.4 (−96.1 to −64.5) | 0.0 (0.0) | 0.0 (0.0) | −27.7 (−43.1 to −13.1) | 0.0 (0.0) |
| From the mirrored assignment | After block 12 alone | B's kinds | A's answer's readout less B's | −524.1 (−542.2 to −490.4) | −386.9 (−394.4 to −378.1) | −137.6 (−141.1 to −135.2) | −4.8 (−20.2 to +16.0) | 0.0 (0.0) | 0.0 (0.0) | +5.2 (+3.5 to +6.9) | 0.0 (0.0) |
| From the mirrored assignment | After block 18 alone | A's kinds | A's answer's readout | +397.2 (+384.2 to +408.4) | +312.3 (+298.4 to +329.4) | +218.5 (+216.4 to +221.1) | −113.5 (−119.2 to −107.9) | 0.0 (0.0) | −2.9 (−8.6 to 0.0) | −17.1 (−29.4 to −8.5) | 0.0 (0.0) |
| From the mirrored assignment | After block 18 alone | A's kinds | B's answer's readout | −397.3 (−408.4 to −384.2) | −253.1 (−256.8 to −251.2) | +23.4 (+21.5 to +26.6) | −161.5 (−173.8 to −151.4) | 0.0 (0.0) | −0.1 (−0.2 to 0.0) | −6.0 (−19.0 to +4.5) | 0.0 (0.0) |
| From the mirrored assignment | After block 18 alone | A's kinds | A's answer's readout less B's | +794.5 (+768.3 to +816.9) | +565.4 (+549.7 to +586.3) | +195.0 (+189.7 to +198.9) | +48.0 (+38.0 to +54.6) | 0.0 (0.0) | −2.8 (−8.5 to 0.0) | −11.1 (−13.0 to −9.8) | 0.0 (0.0) |
| From the mirrored assignment | After block 18 alone | B's kinds | A's answer's readout | −264.8 (−273.9 to −249.3) | −189.6 (−193.9 to −181.4) | +28.4 (+24.6 to +30.5) | −107.4 (−132.3 to −91.0) | 0.0 (0.0) | +0.7 (0.0 to +2.2) | +3.1 (−19.3 to +15.1) | 0.0 (0.0) |
| From the mirrored assignment | After block 18 alone | B's kinds | B's answer's readout | +264.7 (+249.3 to +273.9) | +190.9 (+165.9 to +210.2) | +161.6 (+156.8 to +166.5) | −66.7 (−70.4 to −60.8) | 0.0 (0.0) | +0.2 (0.0 to +0.6) | −21.3 (−40.4 to −9.3) | 0.0 (0.0) |
| From the mirrored assignment | After block 18 alone | B's kinds | A's answer's readout less B's | −529.5 (−547.8 to −498.5) | −380.5 (−403.7 to −359.8) | −133.2 (−136.3 to −131.0) | −40.8 (−61.9 to −30.2) | 0.0 (0.0) | +0.5 (0.0 to +1.6) | +24.5 (+21.2 to +27.8) | 0.0 (0.0) |
| From the mirrored assignment | After block 24 alone | A's kinds | A's answer's readout | +344.2 (+323.5 to +365.4) | +254.4 (+232.3 to +268.8) | +223.1 (+221.8 to +225.3) | −79.5 (−89.1 to −68.3) | 0.0 (0.0) | 0.0 (0.0) | −53.9 (−60.5 to −50.1) | 0.0 (0.0) |
| From the mirrored assignment | After block 24 alone | A's kinds | B's answer's readout | −344.2 (−365.4 to −323.5) | −218.3 (−222.8 to −215.8) | +43.3 (+41.5 to +45.0) | −126.0 (−144.5 to −107.4) | 0.0 (0.0) | 0.0 (0.0) | −43.2 (−47.9 to −39.9) | 0.0 (0.0) |
| From the mirrored assignment | After block 24 alone | A's kinds | A's answer's readout less B's | +688.4 (+647.0 to +730.8) | +472.7 (+448.1 to +485.0) | +179.8 (+176.8 to +182.0) | +46.5 (+26.4 to +76.3) | 0.0 (0.0) | 0.0 (0.0) | −10.7 (−12.6 to −8.2) | 0.0 (0.0) |
| From the mirrored assignment | After block 24 alone | B's kinds | A's answer's readout | −349.9 (−367.4 to −328.4) | −149.9 (−153.2 to −145.0) | +36.4 (+34.9 to +39.3) | −135.9 (−180.3 to −99.2) | 0.0 (0.0) | 0.0 (0.0) | −100.6 (−112.8 to −77.0) | 0.0 (0.0) |
| From the mirrored assignment | After block 24 alone | B's kinds | B's answer's readout | +349.9 (+328.5 to +367.4) | +321.7 (+311.1 to +328.0) | +193.0 (+186.3 to +197.1) | −39.4 (−41.0 to −36.1) | 0.0 (0.0) | 0.0 (0.0) | −125.4 (−135.1 to −113.1) | 0.0 (0.0) |
| From the mirrored assignment | After block 24 alone | B's kinds | A's answer's readout less B's | −699.8 (−734.8 to −656.9) | −471.5 (−481.2 to −462.4) | −156.6 (−160.6 to −151.3) | −96.5 (−139.3 to −58.3) | 0.0 (0.0) | 0.0 (0.0) | +24.8 (+15.1 to +36.1) | 0.0 (0.0) |

From A's kinds onto A's answer's readout less B's, by the units' class:

| The cell | Night, less the night without replay from the same image | Units of each readout | The inputs' change, k | Consolidation in its own replay, k | at its next spike, k | at another in slow-wave sleep, k | at another outside it, k | below zero, k | The zero sum's rounds, k | What the watch does not see, k |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| From the assignment | After block 6 alone | C's or D's own | +47.5 (+43.0 to +50.3) | +39.5 (+36.7 to +41.4) | +10.9 (+9.4 to +12.8) | −3.7 (−5.3 to −2.2) | 0.0 (0.0) | 0.0 (0.0) | +0.8 (+0.5 to +1.2) | 0.0 (0.0) |
| From the assignment | After block 6 alone | A's patterns' targets | +718.1 (+709.9 to +733.2) | +395.7 (+380.9 to +405.2) | +150.4 (+148.5 to +151.8) | +180.5 (+166.7 to +188.5) | 0.0 (0.0) | 0.0 (0.0) | −8.5 (−10.4 to −6.6) | 0.0 (0.0) |
| From the assignment | After block 6 alone | other excitatory | −6.4 (−9.6 to −3.7) | +24.7 (+22.2 to +26.1) | −23.5 (−23.9 to −23.0) | −3.5 (−9.2 to −0.4) | 0.0 (0.0) | 0.0 (0.0) | −4.1 (−5.4 to −3.3) | 0.0 (0.0) |
| From the assignment | After block 6 alone | inhibitory | −70.4 (−74.3 to −67.9) | −95.5 (−96.9 to −94.3) | +9.0 (+8.0 to +10.7) | +12.5 (+9.7 to +16.5) | 0.0 (0.0) | 0.0 (0.0) | +3.5 (+2.5 to +5.1) | 0.0 (0.0) |
| From the assignment | After block 6 alone | all | +688.9 (+674.0 to +711.0) | +364.5 (+352.1 to +378.3) | +146.8 (+145.4 to +149.0) | +185.9 (+174.2 to +192.9) | 0.0 (0.0) | 0.0 (0.0) | −8.3 (−10.0 to −6.3) | 0.0 (0.0) |
| From the assignment | After block 12 alone | C's or D's own | +57.1 (+56.1 to +58.0) | +37.5 (+36.9 to +38.0) | +13.0 (+12.6 to +13.4) | +5.1 (+4.2 to +5.6) | 0.0 (0.0) | 0.0 (0.0) | +1.4 (+1.3 to +1.5) | 0.0 (0.0) |
| From the assignment | After block 12 alone | A's patterns' targets | +646.7 (+632.5 to +664.0) | +393.9 (+390.4 to +398.6) | +153.2 (+150.6 to +154.8) | +109.7 (+99.6 to +119.4) | 0.0 (0.0) | 0.0 (0.0) | −10.1 (−11.1 to −8.8) | 0.0 (0.0) |
| From the assignment | After block 12 alone | other excitatory | +6.9 (+4.4 to +8.2) | +39.5 (+38.4 to +41.3) | −23.1 (−23.8 to −22.8) | −13.2 (−15.2 to −11.7) | 0.0 (0.0) | 0.0 (0.0) | +3.7 (+2.3 to +4.6) | 0.0 (0.0) |
| From the assignment | After block 12 alone | inhibitory | −87.2 (−92.7 to −83.8) | −84.6 (−90.6 to −78.0) | +17.8 (+17.4 to +18.3) | −13.8 (−18.5 to −8.7) | 0.0 (0.0) | 0.0 (0.0) | −6.6 (−7.3 to −5.8) | 0.0 (0.0) |
| From the assignment | After block 12 alone | all | +623.4 (+614.0 to +635.4) | +386.3 (+382.7 to +388.8) | +161.0 (+158.3 to +162.5) | +87.7 (+81.7 to +96.6) | 0.0 (0.0) | 0.0 (0.0) | −11.5 (−11.8 to −10.9) | 0.0 (0.0) |
| From the assignment | After block 18 alone | C's or D's own | +53.6 (+51.0 to +57.3) | +37.6 (+36.2 to +39.9) | +4.0 (+3.2 to +4.7) | +17.1 (+15.8 to +18.7) | 0.0 (0.0) | 0.0 (0.0) | −5.1 (−5.9 to −4.1) | 0.0 (0.0) |
| From the assignment | After block 18 alone | A's patterns' targets | +636.1 (+629.2 to +639.7) | +336.6 (+332.9 to +343.0) | +137.2 (+133.1 to +140.7) | +160.3 (+154.7 to +169.6) | 0.0 (0.0) | 0.0 (0.0) | +2.0 (−0.2 to +4.0) | 0.0 (0.0) |
| From the assignment | After block 18 alone | other excitatory | −6.5 (−11.4 to −1.7) | +20.4 (+18.3 to +21.6) | −19.3 (−19.5 to −19.0) | +1.0 (−0.9 to +3.2) | 0.0 (0.0) | 0.0 (0.0) | −8.7 (−9.4 to −7.3) | 0.0 (0.0) |
| From the assignment | After block 18 alone | inhibitory | −87.4 (−92.3 to −80.8) | −105.2 (−108.7 to −101.4) | +7.5 (+6.5 to +8.3) | −9.8 (−13.0 to −6.2) | 0.0 (0.0) | 0.0 (0.0) | +20.1 (+18.3 to +22.9) | 0.0 (0.0) |
| From the assignment | After block 18 alone | all | +595.9 (+581.1 to +608.2) | +289.4 (+283.4 to +299.2) | +129.5 (+125.0 to +133.6) | +168.6 (+159.5 to +176.0) | 0.0 (0.0) | 0.0 (0.0) | +8.3 (+4.7 to +11.5) | 0.0 (0.0) |
| From the assignment | After block 24 alone | C's or D's own | +17.4 (+15.6 to +20.0) | +2.9 (+0.8 to +6.0) | +4.5 (+3.8 to +5.5) | +5.2 (+3.5 to +6.6) | 0.0 (0.0) | 0.0 (0.0) | +4.9 (+4.5 to +5.3) | 0.0 (0.0) |
| From the assignment | After block 24 alone | A's patterns' targets | +585.6 (+571.7 to +594.6) | +328.9 (+317.7 to +343.8) | +143.3 (+140.5 to +147.7) | +131.9 (+128.9 to +135.5) | 0.0 (0.0) | 0.0 (0.0) | −18.5 (−20.1 to −17.7) | 0.0 (0.0) |
| From the assignment | After block 24 alone | other excitatory | −35.6 (−41.6 to −29.6) | +15.3 (+10.5 to +19.8) | −27.1 (−27.8 to −26.5) | −12.7 (−14.4 to −10.5) | 0.0 (0.0) | 0.0 (0.0) | −11.1 (−12.5 to −9.9) | 0.0 (0.0) |
| From the assignment | After block 24 alone | inhibitory | −77.7 (−83.3 to −67.6) | −82.2 (−86.2 to −77.2) | +10.3 (+8.6 to +11.4) | −8.9 (−14.0 to −3.8) | 0.0 (0.0) | 0.0 (0.0) | +3.2 (+1.4 to +6.1) | 0.0 (0.0) |
| From the assignment | After block 24 alone | all | +489.8 (+482.5 to +493.6) | +264.8 (+257.1 to +277.8) | +130.9 (+127.6 to +135.8) | +115.5 (+112.4 to +117.6) | 0.0 (0.0) | 0.0 (0.0) | −21.5 (−24.2 to −19.6) | 0.0 (0.0) |
| From the mirrored assignment | After block 6 alone | C's or D's own | +58.7 (+52.8 to +62.7) | +39.4 (+36.8 to +41.6) | +15.4 (+13.9 to +16.5) | +4.4 (+2.6 to +5.8) | 0.0 (0.0) | 0.0 (0.0) | −0.5 (−0.6 to −0.4) | 0.0 (0.0) |
| From the mirrored assignment | After block 6 alone | A's patterns' targets | +509.5 (+504.6 to +512.4) | +380.5 (+378.4 to +384.3) | +156.9 (+153.3 to +162.9) | −21.7 (−27.9 to −18.3) | 0.0 (0.0) | 0.0 (0.0) | −6.3 (−11.7 to −0.9) | 0.0 (0.0) |
| From the mirrored assignment | After block 6 alone | other excitatory | +51.4 (+46.6 to +55.0) | +38.8 (+36.7 to +40.7) | −14.6 (−16.9 to −12.3) | +27.4 (+26.7 to +27.8) | 0.0 (0.0) | 0.0 (0.0) | −0.2 (−3.3 to +2.8) | 0.0 (0.0) |
| From the mirrored assignment | After block 6 alone | inhibitory | +49.8 (+44.3 to +57.7) | +51.8 (+44.3 to +59.2) | −9.2 (−10.0 to −7.9) | +7.4 (+6.7 to +8.1) | 0.0 (0.0) | 0.0 (0.0) | −0.2 (−2.1 to +1.8) | 0.0 (0.0) |
| From the mirrored assignment | After block 6 alone | all | +669.4 (+651.4 to +682.3) | +510.4 (+504.0 to +516.4) | +148.5 (+144.4 to +152.5) | +17.5 (+10.0 to +21.6) | 0.0 (0.0) | 0.0 (0.0) | −7.1 (−7.5 to −6.9) | 0.0 (0.0) |
| From the mirrored assignment | After block 12 alone | C's or D's own | +73.8 (+71.1 to +78.3) | +59.4 (+55.9 to +62.4) | +14.2 (+13.8 to +14.6) | −0.1 (−1.6 to +1.9) | 0.0 (0.0) | 0.0 (0.0) | +0.3 (−0.6 to +1.8) | 0.0 (0.0) |
| From the mirrored assignment | After block 12 alone | A's patterns' targets | +604.4 (+601.2 to +608.9) | +475.7 (+471.3 to +478.9) | +163.9 (+162.1 to +166.3) | −5.9 (−11.8 to +2.5) | 0.0 (0.0) | 0.0 (0.0) | −29.3 (−31.5 to −27.1) | 0.0 (0.0) |
| From the mirrored assignment | After block 12 alone | other excitatory | +87.3 (+85.3 to +88.8) | +49.2 (+46.6 to +51.0) | −8.2 (−8.7 to −7.8) | +33.9 (+33.3 to +35.0) | 0.0 (0.0) | 0.0 (0.0) | +12.4 (+10.4 to +13.4) | 0.0 (0.0) |
| From the mirrored assignment | After block 12 alone | inhibitory | +46.7 (+42.1 to +49.9) | +31.9 (+29.6 to +34.1) | −16.1 (−17.5 to −15.1) | +24.7 (+22.3 to +27.2) | 0.0 (0.0) | 0.0 (0.0) | +6.2 (+5.7 to +6.9) | 0.0 (0.0) |
| From the mirrored assignment | After block 12 alone | all | +812.2 (+809.4 to +816.9) | +616.3 (+611.3 to +624.0) | +153.8 (+150.9 to +157.3) | +52.6 (+44.7 to +61.8) | 0.0 (0.0) | 0.0 (0.0) | −10.4 (−11.6 to −9.2) | 0.0 (0.0) |
| From the mirrored assignment | After block 18 alone | C's or D's own | +65.6 (+62.4 to +67.4) | +49.9 (+48.4 to +51.8) | +13.8 (+11.5 to +16.0) | +0.6 (−2.4 to +3.1) | 0.0 (0.0) | −0.6 (−1.7 to 0.0) | +1.9 (+1.0 to +2.6) | 0.0 (0.0) |
| From the mirrored assignment | After block 18 alone | A's patterns' targets | +562.0 (+545.4 to +578.0) | +412.2 (+402.0 to +431.1) | +190.0 (+189.3 to +191.4) | −22.1 (−26.5 to −17.4) | 0.0 (0.0) | −2.5 (−7.5 to 0.0) | −15.6 (−20.1 to −12.9) | 0.0 (0.0) |
| From the mirrored assignment | After block 18 alone | other excitatory | +72.0 (+67.3 to +76.3) | +49.0 (+46.3 to +52.3) | −9.4 (−10.7 to −8.2) | +29.8 (+28.0 to +32.3) | 0.0 (0.0) | 0.0 (0.0 to +0.1) | +2.6 (0.0 to +5.5) | 0.0 (0.0) |
| From the mirrored assignment | After block 18 alone | inhibitory | +94.9 (+92.0 to +99.5) | +54.4 (+53.0 to +55.2) | +0.5 (−0.4 to +1.3) | +39.7 (+36.7 to +43.3) | 0.0 (0.0) | +0.2 (0.0 to +0.7) | +0.1 (−1.1 to +1.5) | 0.0 (0.0) |
| From the mirrored assignment | After block 18 alone | all | +794.5 (+768.3 to +816.9) | +565.4 (+549.7 to +586.3) | +195.0 (+189.7 to +198.9) | +48.0 (+38.0 to +54.6) | 0.0 (0.0) | −2.8 (−8.5 to 0.0) | −11.1 (−13.0 to −9.8) | 0.0 (0.0) |
| From the mirrored assignment | After block 24 alone | C's or D's own | +61.2 (+60.5 to +62.2) | +47.0 (+44.8 to +49.7) | +20.1 (+19.8 to +20.6) | −1.5 (−2.4 to −1.1) | 0.0 (0.0) | 0.0 (0.0) | −4.3 (−4.9 to −3.2) | 0.0 (0.0) |
| From the mirrored assignment | After block 24 alone | A's patterns' targets | +501.2 (+468.4 to +535.4) | +340.8 (+316.3 to +362.2) | +182.6 (+181.1 to +185.2) | +2.2 (−7.1 to +15.2) | 0.0 (0.0) | 0.0 (0.0) | −24.3 (−27.2 to −21.9) | 0.0 (0.0) |
| From the mirrored assignment | After block 24 alone | other excitatory | +58.3 (+55.8 to +61.8) | +38.2 (+33.1 to +41.4) | −18.5 (−19.7 to −17.2) | +27.4 (+19.9 to +36.9) | 0.0 (0.0) | 0.0 (0.0) | +11.2 (+10.3 to +11.7) | 0.0 (0.0) |
| From the mirrored assignment | After block 24 alone | inhibitory | +67.6 (+62.4 to +72.6) | +46.7 (+44.0 to +51.2) | −4.4 (−5.9 to −3.2) | +18.5 (+14.6 to +25.5) | 0.0 (0.0) | 0.0 (0.0) | +6.8 (+6.2 to +7.2) | 0.0 (0.0) |
| From the mirrored assignment | After block 24 alone | all | +688.4 (+647.0 to +730.8) | +472.7 (+448.1 to +485.0) | +179.8 (+176.8 to +182.0) | +46.5 (+26.4 to +76.3) | 0.0 (0.0) | 0.0 (0.0) | −10.7 (−12.6 to −8.2) | 0.0 (0.0) |

**Each night alone**, not less the other: from A's kinds onto each readout, the replayed night and the night without replay from the same
image.

| The cell | Night | Onto | The inputs' change, k | Consolidation in its own replay, k | at its next spike, k | at another in slow-wave sleep, k | at another outside it, k | below zero, k | The zero sum's rounds, k | What the watch does not see, k |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| From the assignment | After block 6 alone | A's answer's readout | +308.4 (+302.2 to +314.4) | +180.8 (+176.1 to +188.8) | +168.1 (+167.0 to +169.7) | +62.8 (+59.0 to +66.5) | 0.0 (0.0) | 0.0 (0.0) | −102.9 (−104.5 to −101.7) | −0.4 (−0.6 to −0.1) |
| From the assignment | After block 6 alone | B's answer's readout | −308.4 (−314.4 to −302.2) | −183.8 (−189.6 to −176.0) | +21.2 (+20.7 to +21.6) | −55.0 (−56.4 to −53.5) | 0.0 (0.0) | 0.0 (0.0) | −91.1 (−93.0 to −89.8) | +0.3 (+0.1 to +0.6) |
| From the assignment | Without replay, after block 6 | A's answer's readout | −36.1 (−41.1 to −28.5) | 0.0 (0.0) | 0.0 (0.0) | +87.1 (+79.7 to +91.4) | 0.0 (0.0) | 0.0 (0.0) | −122.8 (−132.2 to −118.0) | −0.4 (−0.6 to −0.1) |
| From the assignment | Without replay, after block 6 | B's answer's readout | +36.0 (+28.5 to +41.0) | 0.0 (0.0) | 0.0 (0.0) | +155.0 (+142.6 to +167.7) | 0.0 (0.0) | 0.0 (0.0) | −119.4 (−127.0 to −114.8) | +0.3 (+0.1 to +0.6) |
| From the assignment | After block 12 alone | A's answer's readout | +303.8 (+299.6 to +308.2) | +208.0 (+204.8 to +210.4) | +185.3 (+182.6 to +187.0) | +45.4 (+40.7 to +50.5) | 0.0 (0.0) | 0.0 (0.0) | −134.9 (−138.4 to −129.0) | 0.0 (0.0) |
| From the assignment | After block 12 alone | B's answer's readout | −303.8 (−308.2 to −299.8) | −178.3 (−182.5 to −172.3) | +24.3 (+24.2 to +24.6) | −23.8 (−25.4 to −21.0) | 0.0 (0.0) | 0.0 (0.0) | −126.1 (−130.7 to −120.9) | 0.0 (0.0) |
| From the assignment | Without replay, after block 12 | A's answer's readout | −7.9 (−14.1 to −2.2) | 0.0 (0.0) | 0.0 (0.0) | +96.3 (+93.4 to +100.5) | 0.0 (0.0) | 0.0 (0.0) | −104.2 (−107.5 to −102.6) | 0.0 (0.0) |
| From the assignment | Without replay, after block 12 | B's answer's readout | +7.9 (+2.2 to +14.1) | 0.0 (0.0) | 0.0 (0.0) | +114.8 (+106.2 to +124.4) | 0.0 (0.0) | 0.0 (0.0) | −107.0 (−110.3 to −104.0) | 0.0 (0.0) |
| From the assignment | After block 18 alone | A's answer's readout | +248.2 (+241.1 to +255.4) | +147.0 (+140.8 to +153.2) | +165.1 (+163.9 to +165.7) | +54.5 (+49.6 to +60.5) | 0.0 (0.0) | 0.0 (0.0) | −118.4 (−125.1 to −113.2) | 0.0 (0.0) |
| From the assignment | After block 18 alone | B's answer's readout | −248.2 (−255.4 to −241.1) | −142.4 (−146.0 to −136.3) | +35.6 (+32.1 to +38.9) | −14.4 (−19.3 to −11.8) | 0.0 (0.0) | 0.0 (0.0) | −127.0 (−131.8 to −123.4) | 0.0 (0.0) |
| From the assignment | Without replay, after block 18 | A's answer's readout | −49.7 (−58.1 to −42.4) | 0.0 (0.0) | 0.0 (0.0) | +138.6 (+130.8 to +150.0) | 0.0 (0.0) | 0.0 (0.0) | −188.3 (−193.0 to −179.5) | 0.0 (0.0) |
| From the assignment | Without replay, after block 18 | B's answer's readout | +49.7 (+42.4 to +58.0) | 0.0 (0.0) | 0.0 (0.0) | +238.3 (+228.3 to +249.6) | 0.0 (0.0) | 0.0 (0.0) | −188.5 (−194.4 to −179.6) | 0.0 (0.0) |
| From the assignment | After block 24 alone | A's answer's readout | +227.2 (+224.7 to +231.7) | +135.1 (+130.0 to +139.1) | +181.9 (+179.0 to +185.2) | +58.0 (+55.2 to +61.6) | 0.0 (0.0) | 0.0 (0.0) | −147.7 (−152.0 to −140.7) | 0.0 (0.0) |
| From the assignment | After block 24 alone | B's answer's readout | −227.3 (−231.7 to −224.8) | −129.7 (−138.8 to −120.9) | +51.0 (+49.5 to +54.0) | −11.0 (−17.0 to −2.9) | 0.0 (0.0) | 0.0 (0.0) | −137.6 (−140.4 to −134.8) | 0.0 (0.0) |
| From the assignment | Without replay, after block 24 | A's answer's readout | −17.6 (−21.5 to −14.8) | 0.0 (0.0) | 0.0 (0.0) | +180.1 (+172.7 to +186.3) | 0.0 (0.0) | 0.0 (0.0) | −197.7 (−201.2 to −194.2) | 0.0 (0.0) |
| From the assignment | Without replay, after block 24 | B's answer's readout | +17.6 (+14.9 to +21.5) | 0.0 (0.0) | 0.0 (0.0) | +226.7 (+223.3 to +229.7) | 0.0 (0.0) | 0.0 (0.0) | −209.1 (−214.8 to −205.5) | 0.0 (0.0) |
| From the mirrored assignment | After block 6 alone | A's answer's readout | +353.8 (+350.7 to +359.7) | +258.9 (+251.5 to +270.2) | +184.8 (+180.6 to +192.2) | +59.4 (+56.6 to +62.4) | 0.0 (0.0) | 0.0 (0.0) | −149.2 (−165.0 to −140.7) | 0.0 (0.0) |
| From the mirrored assignment | After block 6 alone | B's answer's readout | −353.8 (−359.7 to −350.7) | −251.6 (−256.0 to −246.2) | +36.2 (+32.8 to +39.6) | +4.5 (−1.6 to +9.5) | 0.0 (0.0) | 0.0 (0.0) | −142.9 (−158.9 to −132.7) | 0.0 (0.0) |
| From the mirrored assignment | Without replay, after block 6 | A's answer's readout | +19.1 (+13.8 to +25.0) | 0.0 (0.0) | 0.0 (0.0) | +163.4 (+150.3 to +175.6) | 0.0 (0.0) | 0.0 (0.0) | −144.3 (−150.6 to −136.6) | 0.0 (0.0) |
| From the mirrored assignment | Without replay, after block 6 | B's answer's readout | −19.1 (−24.9 to −13.9) | 0.0 (0.0) | 0.0 (0.0) | +125.9 (+124.6 to +128.4) | 0.0 (0.0) | 0.0 (0.0) | −145.1 (−149.5 to −138.7) | 0.0 (0.0) |
| From the mirrored assignment | After block 12 alone | A's answer's readout | +402.4 (+399.9 to +405.3) | +348.0 (+343.1 to +350.9) | +192.5 (+187.8 to +196.4) | +80.7 (+78.2 to +82.5) | 0.0 (0.0) | 0.0 (0.0) | −218.8 (−228.0 to −207.2) | 0.0 (0.0) |
| From the mirrored assignment | After block 12 alone | B's answer's readout | −402.4 (−405.3 to −399.9) | −268.2 (−273.2 to −261.1) | +38.7 (+34.5 to +42.4) | +40.4 (+34.8 to +44.5) | 0.0 (0.0) | 0.0 (0.0) | −213.2 (−222.4 to −200.8) | 0.0 (0.0) |
| From the mirrored assignment | Without replay, after block 12 | A's answer's readout | −3.7 (−6.6 to +0.1) | 0.0 (0.0) | 0.0 (0.0) | +150.9 (+143.0 to +155.3) | 0.0 (0.0) | 0.0 (0.0) | −154.6 (−160.1 to −149.5) | 0.0 (0.0) |
| From the mirrored assignment | Without replay, after block 12 | B's answer's readout | +3.7 (−0.1 to +6.6) | 0.0 (0.0) | 0.0 (0.0) | +163.2 (+158.4 to +169.8) | 0.0 (0.0) | 0.0 (0.0) | −159.4 (−165.0 to −154.8) | 0.0 (0.0) |
| From the mirrored assignment | After block 18 alone | A's answer's readout | +417.1 (+411.9 to +423.4) | +312.3 (+298.4 to +329.4) | +218.5 (+216.4 to +221.1) | +105.2 (+101.3 to +107.4) | 0.0 (0.0) | −4.7 (−14.2 to 0.0) | −214.1 (−223.7 to −200.5) | 0.0 (0.0) |
| From the mirrored assignment | After block 18 alone | B's answer's readout | −417.1 (−423.4 to −412.0) | −253.1 (−256.8 to −251.2) | +23.4 (+21.5 to +26.6) | +28.8 (+21.9 to +32.3) | 0.0 (0.0) | −0.9 (−2.6 to 0.0) | −215.4 (−225.3 to −202.3) | 0.0 (0.0) |
| From the mirrored assignment | Without replay, after block 18 | A's answer's readout | +19.8 (+14.9 to +27.7) | 0.0 (0.0) | 0.0 (0.0) | +218.7 (+209.2 to +226.6) | 0.0 (0.0) | −1.9 (−5.6 to 0.0) | −197.0 (−209.7 to −187.1) | 0.0 (0.0) |
| From the mirrored assignment | Without replay, after block 18 | B's answer's readout | −19.9 (−27.8 to −14.9) | 0.0 (0.0) | 0.0 (0.0) | +190.3 (+173.3 to +206.1) | 0.0 (0.0) | −0.8 (−2.4 to 0.0) | −209.3 (−223.0 to −198.7) | 0.0 (0.0) |
| From the mirrored assignment | After block 24 alone | A's answer's readout | +375.2 (+365.8 to +380.9) | +254.4 (+232.3 to +268.8) | +223.1 (+221.8 to +225.3) | +89.5 (+88.1 to +92.1) | 0.0 (0.0) | 0.0 (0.0) | −191.9 (−203.3 to −180.9) | 0.0 (0.0) |
| From the mirrored assignment | After block 24 alone | B's answer's readout | −375.2 (−380.9 to −365.8) | −218.3 (−222.8 to −215.8) | +43.3 (+41.5 to +45.0) | −7.0 (−9.0 to −4.3) | 0.0 (0.0) | 0.0 (0.0) | −193.2 (−201.6 to −183.9) | 0.0 (0.0) |
| From the mirrored assignment | Without replay, after block 24 | A's answer's readout | +31.0 (+13.5 to +42.3) | 0.0 (0.0) | 0.0 (0.0) | +169.0 (+156.3 to +177.5) | 0.0 (0.0) | 0.0 (0.0) | −138.0 (−142.8 to −130.8) | 0.0 (0.0) |
| From the mirrored assignment | Without replay, after block 24 | B's answer's readout | −31.0 (−42.3 to −13.5) | 0.0 (0.0) | 0.0 (0.0) | +119.0 (+99.7 to +140.2) | 0.0 (0.0) | 0.0 (0.0) | −150.0 (−154.2 to −141.9) | 0.0 (0.0) |

What the readings show, from A's kinds onto A's answer's readout less B's, less the night without replay:
- **Consolidation in the units' own replays** carried +364.5, +386.3, +289.4 and +264.8 k on the image from the assignment and +510.4,
  +616.3, +565.4 and +472.7 on the image from the mirrored assignment. Onto A's answer's readout alone it read +180.8, +208.0, +147.0 and
  +135.1 and +258.9, +348.0, +312.3 and +254.4, onto B's answer's readout −183.8, −178.3, −142.4 and −129.7 and −251.6, −268.2, −253.1 and
  −218.3.
- **At the next spike after a unit's replay** it carried +146.8, +161.0, +129.5 and +130.9 and +148.5, +153.8, +195.0 and +179.8. Onto A's
  answer's readout it read +168.1, +185.3, +165.1 and +181.9 and +184.8, +192.5, +218.5 and +223.1, onto B's +21.2, +24.3, +35.6 and +51.0
  and +36.2, +38.7, +23.4 and +43.3.
- **At the units' other spikes in slow-wave sleep** it carried +185.9, +87.7, +168.6 and +115.5 and +17.5, +52.6, +48.0 and +46.5. Onto A's
  answer's readout it read −24.2, −50.9, −84.1 and −122.1 and −103.9, −70.2, −113.5 and −79.5, onto B's −210.1, −138.6, −252.7 and −237.6
  and −121.4, −122.8, −161.5 and −126.0.
- **Outside slow-wave sleep** nothing was consolidated on any of the 48 nights, from any kind onto any unit.
- **Below zero** the replayed nights less the nights without replay read 0.0, 0.0, −2.8 and 0.0 on the image from the mirrored assignment
  and 0.0, 0.0, 0.0 and 0.0 on the image from the assignment. The one that is not zero comes from path 0's two nights from the image before
  the night after block 18, the table of pairings below zero.
- **The zero sum's rounds** moved −8.3, −11.5, +8.3 and −21.5 and −7.1, −10.4, −11.1 and −10.7. On each night alone they moved A's kinds'
  weights onto each readout by −218.8 to −91.1 k, the table above, and between the two nights from the same image most of that cancels.
- **What the watch does not see** reads 0.0 on every difference. On 6 nights it is not zero: on the image from the assignment, both nights
  from the image before block 6's night, on each path, in the lead ticks before the watch, from A and C alone, 645 of the class matrix's
  units at most onto a readout. The quiet ticks moved nothing on any night.

By class on the image from the assignment, A's patterns' targets took +395.7, +393.9, +336.6 and +328.9 in the units' own replays, +150.4,
+153.2, +137.2 and +143.3 at their next spikes and +180.5, +109.7, +160.3 and +131.9 at their other spikes in slow-wave sleep, and the
inhibitory units −95.5, −84.6, −105.2 and −82.2 in the units' own replays. On the image from the mirrored assignment the targets took
+380.5, +475.7, +412.2 and +340.8 in the units' own replays, +156.9, +163.9, +190.0 and +182.6 at their next spikes and −21.7, −5.9, −22.1
and +2.2 at their other spikes in slow-wave sleep; the other excitatory units +38.8, +49.2, +49.0 and +38.2 in the own replays and +27.4,
+33.9, +29.8 and +27.4 at the other spikes; the inhibitory units +51.8, +31.9, +54.4 and +46.7 in the own replays and +7.4, +24.7, +39.7 and
+18.5 at the other spikes.

## 4. Which readout units fired in the replays

Each readout's spikes on each replayed night in the ripples that replay each stimulus's patterns, by the units' class, the paths' lowest and
highest in brackets; how many of all were spikes of the replayed pattern's own units; and how many of the readout's 459 units fired at least
once, each the mean over the three paths:

| The cell | Night | Replays of | Readout | Spikes: C's or D's own | Spikes: A's patterns' targets | Spikes: other excitatory | Spikes: inhibitory | Spikes: all | Of all, as one of the pattern's units | Units that fired |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| From the assignment | After block 6 alone | Stimulus A's patterns | A's answer's readout | 1 810.0 (1 809 to 1 811) | 16 720.3 (16 695 to 16 750) | 1 200.3 (1 193 to 1 210) | 3 238.7 (3 228 to 3 252) | 22 969.3 (22 926 to 23 021) | 15 204.3 | 459.0 |
| From the assignment | After block 6 alone | Stimulus A's patterns | B's answer's readout | 459.0 (458 to 460) | 4 441.3 (4 406 to 4 474) | 1 694.3 (1 684 to 1 702) | 2 876.0 (2 867 to 2 882) | 9 470.7 (9 442 to 9 517) | 0.0 | 459.0 |
| From the assignment | After block 6 alone | Stimulus B's patterns | A's answer's readout | 321.0 (318 to 324) | 4 215.7 (4 200 to 4 238) | 1 454.0 (1 450 to 1 458) | 2 640.7 (2 625 to 2 649) | 8 631.3 (8 593 to 8 661) | 0.0 | 459.0 |
| From the assignment | After block 6 alone | Stimulus B's patterns | B's answer's readout | 1 515.7 (1 514 to 1 519) | 15 630.0 (15 610 to 15 651) | 1 317.0 (1 312 to 1 324) | 2 990.7 (2 980 to 2 997) | 21 453.3 (21 440 to 21 477) | 14 259.7 | 459.0 |
| From the assignment | After block 12 alone | Stimulus A's patterns | A's answer's readout | 1 892.7 (1 889 to 1 896) | 17 041.3 (17 018 to 17 055) | 1 292.0 (1 286 to 1 297) | 3 469.3 (3 455 to 3 484) | 23 695.3 (23 663 to 23 714) | 15 534.3 | 459.0 |
| From the assignment | After block 12 alone | Stimulus A's patterns | B's answer's readout | 447.7 (443 to 452) | 4 649.0 (4 634 to 4 658) | 1 744.3 (1 742 to 1 748) | 3 033.3 (3 008 to 3 047) | 9 874.3 (9 849 to 9 901) | 0.0 | 459.0 |
| From the assignment | After block 12 alone | Stimulus B's patterns | A's answer's readout | 308.0 (305 to 310) | 4 086.3 (4 074 to 4 108) | 1 410.7 (1 407 to 1 417) | 2 635.7 (2 631 to 2 639) | 8 440.7 (8 424 to 8 460) | 0.0 | 459.0 |
| From the assignment | After block 12 alone | Stimulus B's patterns | B's answer's readout | 1 469.0 (1 467 to 1 473) | 15 407.7 (15 395 to 15 422) | 1 275.7 (1 269 to 1 284) | 3 058.7 (3 053 to 3 068) | 21 211.0 (21 189 to 21 231) | 14 044.7 | 459.0 |
| From the assignment | After block 18 alone | Stimulus A's patterns | A's answer's readout | 1 878.7 (1 874 to 1 886) | 16 875.7 (16 867 to 16 882) | 1 239.3 (1 232 to 1 244) | 3 419.7 (3 395 to 3 434) | 23 413.3 (23 403 to 23 430) | 15 460.0 | 459.0 |
| From the assignment | After block 18 alone | Stimulus A's patterns | B's answer's readout | 417.7 (413 to 425) | 4 562.0 (4 547 to 4 572) | 1 720.7 (1 718 to 1 725) | 3 066.7 (3 061 to 3 073) | 9 767.0 (9 739 to 9 789) | 0.0 | 459.0 |
| From the assignment | After block 18 alone | Stimulus B's patterns | A's answer's readout | 309.7 (308 to 311) | 4 069.0 (4 029 to 4 091) | 1 392.7 (1 388 to 1 395) | 2 798.3 (2 785 to 2 814) | 8 569.7 (8 512 to 8 604) | 0.0 | 459.0 |
| From the assignment | After block 18 alone | Stimulus B's patterns | B's answer's readout | 1 452.3 (1 451 to 1 455) | 15 340.7 (15 322 to 15 351) | 1 293.7 (1 275 to 1 313) | 3 076.7 (3 064 to 3 090) | 21 163.3 (21 139 to 21 209) | 13 972.0 | 459.0 |
| From the assignment | After block 24 alone | Stimulus A's patterns | A's answer's readout | 1 872.0 (1 866 to 1 876) | 16 770.7 (16 760 to 16 786) | 1 249.3 (1 233 to 1 266) | 3 590.0 (3 539 to 3 625) | 23 482.0 (23 406 to 23 536) | 15 371.3 | 459.0 |
| From the assignment | After block 24 alone | Stimulus A's patterns | B's answer's readout | 433.3 (428 to 438) | 4 681.0 (4 668 to 4 691) | 1 718.0 (1 716 to 1 721) | 3 215.0 (3 191 to 3 230) | 10 047.3 (10 004 to 10 070) | 0.0 | 459.0 |
| From the assignment | After block 24 alone | Stimulus B's patterns | A's answer's readout | 279.7 (276 to 283) | 3 958.3 (3 952 to 3 969) | 1 433.3 (1 422 to 1 441) | 2 755.7 (2 749 to 2 769) | 8 427.0 (8 399 to 8 442) | 0.0 | 459.0 |
| From the assignment | After block 24 alone | Stimulus B's patterns | B's answer's readout | 1 454.0 (1 451 to 1 458) | 15 235.7 (15 208 to 15 254) | 1 261.7 (1 254 to 1 277) | 3 149.0 (3 128 to 3 168) | 21 100.3 (21 041 to 21 152) | 13 944.3 | 459.0 |
| From the mirrored assignment | After block 6 alone | Stimulus A's patterns | A's answer's readout | 1 852.3 (1 850 to 1 856) | 18 532.0 (18 513 to 18 567) | 872.3 (871 to 873) | 3 230.7 (3 224 to 3 240) | 24 487.3 (24 462 to 24 520) | 17 050.7 | 459.0 |
| From the mirrored assignment | After block 6 alone | Stimulus A's patterns | B's answer's readout | 343.0 (342 to 345) | 4 720.7 (4 695 to 4 769) | 1 548.7 (1 542 to 1 553) | 2 889.7 (2 876 to 2 899) | 9 502.0 (9 470 to 9 563) | 0.0 | 459.0 |
| From the mirrored assignment | After block 6 alone | Stimulus B's patterns | A's answer's readout | 413.7 (410 to 417) | 4 280.7 (4 243 to 4 315) | 919.7 (916 to 923) | 2 724.0 (2 702 to 2 741) | 8 338.0 (8 271 to 8 393) | 0.0 | 459.0 |
| From the mirrored assignment | After block 6 alone | Stimulus B's patterns | B's answer's readout | 1 286.7 (1 285 to 1 289) | 16 652.7 (16 637 to 16 684) | 1 054.0 (1 052 to 1 055) | 2 879.0 (2 874 to 2 882) | 21 872.3 (21 851 to 21 910) | 14 981.7 | 459.0 |
| From the mirrored assignment | After block 12 alone | Stimulus A's patterns | A's answer's readout | 1 890.3 (1 887 to 1 893) | 18 967.3 (18 949 to 18 989) | 847.0 (841 to 851) | 3 313.7 (3 295 to 3 327) | 25 018.3 (24 986 to 25 058) | 17 336.3 | 459.0 |
| From the mirrored assignment | After block 12 alone | Stimulus A's patterns | B's answer's readout | 349.7 (345 to 354) | 4 695.3 (4 681 to 4 716) | 1 593.0 (1 591 to 1 595) | 3 047.3 (3 031 to 3 058) | 9 685.3 (9 648 to 9 718) | 0.0 | 458.0 |
| From the mirrored assignment | After block 12 alone | Stimulus B's patterns | A's answer's readout | 437.3 (434 to 440) | 4 184.0 (4 163 to 4 205) | 881.3 (877 to 890) | 2 829.3 (2 825 to 2 836) | 8 332.0 (8 307 to 8 365) | 0.0 | 459.0 |
| From the mirrored assignment | After block 12 alone | Stimulus B's patterns | B's answer's readout | 1 234.3 (1 230 to 1 237) | 16 516.7 (16 509 to 16 527) | 1 038.7 (1 036 to 1 041) | 2 890.0 (2 883 to 2 896) | 21 679.7 (21 672 to 21 692) | 14 804.0 | 459.0 |
| From the mirrored assignment | After block 18 alone | Stimulus A's patterns | A's answer's readout | 1 854.7 (1 848 to 1 863) | 18 832.3 (18 816 to 18 862) | 831.0 (823 to 838) | 3 390.7 (3 381 to 3 401) | 24 908.7 (24 873 to 24 964) | 17 281.7 | 459.0 |
| From the mirrored assignment | After block 18 alone | Stimulus A's patterns | B's answer's readout | 329.0 (320 to 334) | 4 699.0 (4 682 to 4 721) | 1 575.0 (1 566 to 1 583) | 3 081.7 (3 070 to 3 092) | 9 684.7 (9 648 to 9 729) | 0.0 | 459.0 |
| From the mirrored assignment | After block 18 alone | Stimulus B's patterns | A's answer's readout | 445.0 (443 to 449) | 4 321.3 (4 318 to 4 325) | 864.0 (856 to 869) | 2 853.7 (2 846 to 2 868) | 8 484.0 (8 467 to 8 502) | 0.0 | 459.0 |
| From the mirrored assignment | After block 18 alone | Stimulus B's patterns | B's answer's readout | 1 246.3 (1 245 to 1 248) | 16 409.3 (16 395 to 16 429) | 1 022.0 (1 018 to 1 028) | 3 068.3 (3 055 to 3 075) | 21 746.0 (21 713 to 21 780) | 14 709.0 | 459.0 |
| From the mirrored assignment | After block 24 alone | Stimulus A's patterns | A's answer's readout | 1 902.0 (1 899 to 1 904) | 18 809.0 (18 799 to 18 828) | 827.7 (827 to 829) | 3 438.7 (3 433 to 3 444) | 24 977.3 (24 958 to 24 997) | 17 208.7 | 459.0 |
| From the mirrored assignment | After block 24 alone | Stimulus A's patterns | B's answer's readout | 337.3 (333 to 340) | 4 842.7 (4 827 to 4 856) | 1 501.0 (1 495 to 1 506) | 3 177.3 (3 166 to 3 188) | 9 858.3 (9 827 to 9 880) | 0.0 | 459.0 |
| From the mirrored assignment | After block 24 alone | Stimulus B's patterns | A's answer's readout | 444.3 (440 to 450) | 4 237.3 (4 214 to 4 249) | 880.3 (872 to 889) | 2 880.0 (2 864 to 2 898) | 8 442.0 (8 408 to 8 459) | 0.0 | 459.0 |
| From the mirrored assignment | After block 24 alone | Stimulus B's patterns | B's answer's readout | 1 231.0 (1 227 to 1 233) | 16 299.3 (16 277 to 16 314) | 1 006.3 (1 006 to 1 007) | 3 038.3 (3 033 to 3 044) | 21 575.0 (21 549 to 21 597) | 14 665.3 | 459.0 |

A pattern's targets all lie in its stimulus's answer's readout, so only there do spikes fall as a pattern's units. Of the 96 counts of a
readout's units that fired in one stimulus's replays on one replayed night and path, 3 are short of 459, each 458. All three are B's
answer's readout in the replays of stimulus A's patterns after block 12 alone on the image from the mirrored assignment, one on each path.

**A unit's spikes against what it gained.** Over each readout's 459 units, on the units' three-path means: the correlation of a unit's
spikes in the replays of a stimulus's patterns on the replayed night, where the night without replay replays none, with its input change
from that stimulus's kinds, and with what consolidation moved their weights onto it, each the replayed night's less the night without
replay's.

| The cell | Night | Replays of, and the stimulus's kinds | Readout | Units | Spikes with the input change, less the night without replay | Spikes with the consolidation, less the night without replay |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: |
| From the assignment | After block 6 alone | Stimulus A's patterns, A's kinds | A's answer's readout | 459 | +0.19 | +0.18 |
| From the assignment | After block 6 alone | Stimulus A's patterns, A's kinds | B's answer's readout | 459 | −0.18 | −0.19 |
| From the assignment | After block 6 alone | Stimulus B's patterns, B's kinds | A's answer's readout | 459 | −0.32 | −0.33 |
| From the assignment | After block 6 alone | Stimulus B's patterns, B's kinds | B's answer's readout | 459 | +0.24 | +0.26 |
| From the assignment | After block 12 alone | Stimulus A's patterns, A's kinds | A's answer's readout | 459 | +0.12 | +0.13 |
| From the assignment | After block 12 alone | Stimulus A's patterns, A's kinds | B's answer's readout | 459 | −0.18 | −0.18 |
| From the assignment | After block 12 alone | Stimulus B's patterns, B's kinds | A's answer's readout | 459 | −0.31 | −0.32 |
| From the assignment | After block 12 alone | Stimulus B's patterns, B's kinds | B's answer's readout | 459 | +0.18 | +0.18 |
| From the assignment | After block 18 alone | Stimulus A's patterns, A's kinds | A's answer's readout | 459 | +0.16 | +0.14 |
| From the assignment | After block 18 alone | Stimulus A's patterns, A's kinds | B's answer's readout | 459 | −0.16 | −0.15 |
| From the assignment | After block 18 alone | Stimulus B's patterns, B's kinds | A's answer's readout | 459 | −0.32 | −0.34 |
| From the assignment | After block 18 alone | Stimulus B's patterns, B's kinds | B's answer's readout | 459 | +0.16 | +0.16 |
| From the assignment | After block 24 alone | Stimulus A's patterns, A's kinds | A's answer's readout | 459 | +0.04 | +0.02 |
| From the assignment | After block 24 alone | Stimulus A's patterns, A's kinds | B's answer's readout | 459 | −0.16 | −0.18 |
| From the assignment | After block 24 alone | Stimulus B's patterns, B's kinds | A's answer's readout | 459 | −0.32 | −0.33 |
| From the assignment | After block 24 alone | Stimulus B's patterns, B's kinds | B's answer's readout | 459 | +0.19 | +0.21 |
| From the mirrored assignment | After block 6 alone | Stimulus A's patterns, A's kinds | A's answer's readout | 459 | +0.09 | +0.09 |
| From the mirrored assignment | After block 6 alone | Stimulus A's patterns, A's kinds | B's answer's readout | 459 | −0.28 | −0.28 |
| From the mirrored assignment | After block 6 alone | Stimulus B's patterns, B's kinds | A's answer's readout | 459 | −0.24 | −0.20 |
| From the mirrored assignment | After block 6 alone | Stimulus B's patterns, B's kinds | B's answer's readout | 459 | +0.15 | +0.18 |
| From the mirrored assignment | After block 12 alone | Stimulus A's patterns, A's kinds | A's answer's readout | 459 | +0.15 | +0.17 |
| From the mirrored assignment | After block 12 alone | Stimulus A's patterns, A's kinds | B's answer's readout | 459 | −0.31 | −0.30 |
| From the mirrored assignment | After block 12 alone | Stimulus B's patterns, B's kinds | A's answer's readout | 459 | −0.23 | −0.22 |
| From the mirrored assignment | After block 12 alone | Stimulus B's patterns, B's kinds | B's answer's readout | 459 | +0.19 | +0.20 |
| From the mirrored assignment | After block 18 alone | Stimulus A's patterns, A's kinds | A's answer's readout | 459 | +0.13 | +0.13 |
| From the mirrored assignment | After block 18 alone | Stimulus A's patterns, A's kinds | B's answer's readout | 459 | −0.30 | −0.30 |
| From the mirrored assignment | After block 18 alone | Stimulus B's patterns, B's kinds | A's answer's readout | 459 | −0.21 | −0.22 |
| From the mirrored assignment | After block 18 alone | Stimulus B's patterns, B's kinds | B's answer's readout | 459 | +0.20 | +0.21 |
| From the mirrored assignment | After block 24 alone | Stimulus A's patterns, A's kinds | A's answer's readout | 459 | +0.10 | +0.13 |
| From the mirrored assignment | After block 24 alone | Stimulus A's patterns, A's kinds | B's answer's readout | 459 | −0.25 | −0.23 |
| From the mirrored assignment | After block 24 alone | Stimulus B's patterns, B's kinds | A's answer's readout | 459 | −0.27 | −0.23 |
| From the mirrored assignment | After block 24 alone | Stimulus B's patterns, B's kinds | B's answer's readout | 459 | +0.18 | +0.23 |

Over A's answer's readout, a unit's spikes in stimulus A's replays line up with its input change from A's kinds at +0.04 to +0.19 on the
image from the assignment and +0.09 to +0.15 on the image from the mirrored assignment. Over B's answer's readout they line up at −0.18 to
−0.16 and −0.31 to −0.25. With the consolidation, the first two read +0.02 to +0.18 and +0.09 to +0.17, the second two −0.19 to −0.15 and
−0.30 to −0.23. For B's stimulus, over B's answer's readout +0.16 to +0.24 and +0.15 to +0.20, over A's −0.32 to −0.31 and −0.27 to −0.21.

## 5. The two cells side by side

From A's kinds onto A's answer's readout less B's, less the night without replay from the same image, each part on both cells, and the image
from the mirrored assignment's less the image from the assignment's:

| Night, less the night without replay from the same image | A's kinds onto A's answer's readout less B's | From the assignment, k | From the mirrored assignment | The mirrored image's less the assignment's |
| :--- | :--- | ---: | ---: | ---: |
| After block 6 alone | The inputs' change | +688.9 (+674.0 to +711.0) | +669.4 (+651.4 to +682.3) | −19.5 |
| After block 6 alone | Consolidation in its own replay | +364.5 (+352.1 to +378.3) | +510.4 (+504.0 to +516.4) | +145.9 |
| After block 6 alone | at its next spike | +146.8 (+145.4 to +149.0) | +148.5 (+144.4 to +152.5) | +1.7 |
| After block 6 alone | at another in slow-wave sleep | +185.9 (+174.2 to +192.9) | +17.5 (+10.0 to +21.6) | −168.3 |
| After block 6 alone | at another outside it | 0.0 (0.0) | 0.0 (0.0) | 0.0 |
| After block 6 alone | below zero | 0.0 (0.0) | 0.0 (0.0) | 0.0 |
| After block 6 alone | The zero sum's rounds | −8.3 (−10.0 to −6.3) | −7.1 (−7.5 to −6.9) | +1.2 |
| After block 6 alone | What the watch does not see | 0.0 (0.0) | 0.0 (0.0) | 0.0 |
| After block 12 alone | The inputs' change | +623.4 (+614.0 to +635.4) | +812.2 (+809.4 to +816.9) | +188.8 |
| After block 12 alone | Consolidation in its own replay | +386.3 (+382.7 to +388.8) | +616.3 (+611.3 to +624.0) | +230.0 |
| After block 12 alone | at its next spike | +161.0 (+158.3 to +162.5) | +153.8 (+150.9 to +157.3) | −7.2 |
| After block 12 alone | at another in slow-wave sleep | +87.7 (+81.7 to +96.6) | +52.6 (+44.7 to +61.8) | −35.1 |
| After block 12 alone | at another outside it | 0.0 (0.0) | 0.0 (0.0) | 0.0 |
| After block 12 alone | below zero | 0.0 (0.0) | 0.0 (0.0) | 0.0 |
| After block 12 alone | The zero sum's rounds | −11.5 (−11.8 to −10.9) | −10.4 (−11.6 to −9.2) | +1.1 |
| After block 12 alone | What the watch does not see | 0.0 (0.0) | 0.0 (0.0) | 0.0 |
| After block 18 alone | The inputs' change | +595.9 (+581.1 to +608.2) | +794.5 (+768.3 to +816.9) | +198.6 |
| After block 18 alone | Consolidation in its own replay | +289.4 (+283.4 to +299.2) | +565.4 (+549.7 to +586.3) | +276.0 |
| After block 18 alone | at its next spike | +129.5 (+125.0 to +133.6) | +195.0 (+189.7 to +198.9) | +65.5 |
| After block 18 alone | at another in slow-wave sleep | +168.6 (+159.5 to +176.0) | +48.0 (+38.0 to +54.6) | −120.6 |
| After block 18 alone | at another outside it | 0.0 (0.0) | 0.0 (0.0) | 0.0 |
| After block 18 alone | below zero | 0.0 (0.0) | −2.8 (−8.5 to 0.0) | −2.8 |
| After block 18 alone | The zero sum's rounds | +8.3 (+4.7 to +11.5) | −11.1 (−13.0 to −9.8) | −19.4 |
| After block 18 alone | What the watch does not see | 0.0 (0.0) | 0.0 (0.0) | 0.0 |
| After block 24 alone | The inputs' change | +489.8 (+482.5 to +493.6) | +688.4 (+647.0 to +730.8) | +198.6 |
| After block 24 alone | Consolidation in its own replay | +264.8 (+257.1 to +277.8) | +472.7 (+448.1 to +485.0) | +207.9 |
| After block 24 alone | at its next spike | +130.9 (+127.6 to +135.8) | +179.8 (+176.8 to +182.0) | +48.9 |
| After block 24 alone | at another in slow-wave sleep | +115.5 (+112.4 to +117.6) | +46.5 (+26.4 to +76.3) | −69.0 |
| After block 24 alone | at another outside it | 0.0 (0.0) | 0.0 (0.0) | 0.0 |
| After block 24 alone | below zero | 0.0 (0.0) | 0.0 (0.0) | 0.0 |
| After block 24 alone | The zero sum's rounds | −21.5 (−24.2 to −19.6) | −10.7 (−12.6 to −8.2) | +10.8 |
| After block 24 alone | What the watch does not see | 0.0 (0.0) | 0.0 (0.0) | 0.0 |

The image from the mirrored assignment less the image from the assignment reads −19.5, +188.8, +198.6 and +198.6 k after block 6, 12, 18 and
24 alone. Consolidation in the units' own replays makes +145.9, +230.0, +276.0 and +207.9 of it, at their next spikes +1.7, −7.2, +65.5 and
+48.9, at their other spikes in slow-wave sleep −168.3, −35.1, −120.6 and −69.0, below zero 0.0, 0.0, −2.8 and 0.0, and the rounds +1.2,
+1.1, −19.4 and +10.8. By the units' class, the same differences of A's answer's readout less B's are −208.7, −42.3, −74.2 and −84.3 on A's
patterns' targets, +57.8, +80.4, +78.5 and +93.9 on the other excitatory units, +120.2, +133.9, +182.3 and +145.3 on the inhibitory units
and +11.2, +16.8, +12.0 and +43.8 on C's or D's own.

**The first night against the later three.** After block 6 alone the own replays carry +145.9 more on the image from the mirrored assignment
and the other spikes in slow-wave sleep −168.3, and the sum reads −19.5. After the later three nights the own replays carry +230.0, +276.0
and +207.9 more and the other spikes −35.1, −120.6 and −69.0, and the sums read +188.8, +198.6 and +198.6. On the image from the assignment
the own replays carried +364.5, +386.3, +289.4 and +264.8 over the four nights and on the image from the mirrored assignment +510.4, +616.3,
+565.4 and +472.7; the other spikes in slow-wave sleep carried +185.9, +87.7, +168.6 and +115.5 and +17.5, +52.6, +48.0 and +46.5.

## What it costs

The first trial took 508 s and stopped, the second 533 s. The two jobs took 893 s on the image from the assignment and 866 s on the image
from the mirrored assignment, 14.9 and 14.4 minutes, against the trial's projection of 746 and 758 s; each spent 157 and 149 s before its
first path. Their logs are 6.7 and 6.6 MB. Nothing was cut: each job ran its three paths' 72 blocks and 24 nights, every night watched and
read before and after.

## What it suggests

**On both cells each single replayed night moved A's kinds' weights toward A's answer's readout and away from B's, by the same zero sum,
mostly by consolidation.** The units' own replays and their next spikes after it carried most of it. The rounds and what the watch does not
see moved little of the difference between the two nights from one image. **The two cells part, after the three later nights, in the own
replays and in which units the weights go to**: on the image from the mirrored assignment more is consolidated in the own replays and less
at the other spikes in slow-wave sleep, on all four nights. Of A's answer's readout less B's, more falls on the other excitatory and
inhibitory units and less on A's patterns' targets, on all four nights. A unit's spikes in the replays line up only weakly with what it
gained.

Questions this suggests for a later exploration or a round:
- What the trace holds when a night begins, by kind and readout, and how much of each bin is the day's trace rather than the night's
  pairings: the bins say when a trace was consolidated, not when it was written.
- Why consolidation in the own replays moves more onto A's answer's readout's other excitatory and inhibitory units on the image from the
  mirrored assignment, and less onto its targets: the order in which those units fire after a replayed pattern's first unit is not read
  here.
- What sets the dopamine term that held the modulation below zero for the first 22 231 watched ticks of both nights from one image, and on
  no other: the dopamine term itself is not read.
- Whether the parting by the own replays holds on the other ten cells.

## What did not run

- **Any other cell**: the ten cells but the two eighth images of seed 24.
- **Replay on two of the four nights, or on every night but one.**
- **Any hold**, on any night.
- **H-47's plain night on all four nights.**
- **The twelve blocks after the last night**, to block 36.
- **Plain nights on both mappings and the awake gap.**
- **Any read of A or of the pair.**
- **The inputs of the units outside the readouts' sets**: what consolidation moved onto them is held to the watch's lines and its rounds to
  zero, but not printed.
- **A spike's arrival after its synapse's delay.**
- **A unit's membrane or threshold.**
- **Any path but H-47's three**, 0, 5 and 6.
- **The next day's reading of the pairings a night leaves** pending.
- **Any SE**: a night's figure is the mean over three paths, one night each.
- **A night split apart**: no part of a night ran apart; consolidation is only binned by when it falls.
- **The trace a night starts with, and the dopamine term**: neither is read.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_what_the_night_writes_on_both_cells_explore` | [43232507958](https://github.com/Menteion/Menteion/actions/runs/43232507958) | `7233796` | 508 |
| `the_harness_of_what_the_night_writes_on_both_cells_explore` | [43236110064](https://github.com/Menteion/Menteion/actions/runs/43236110064) | `bbf805d` | 533 |
| `what_the_night_writes_from_the_assignment_explore` | [43238887408](https://github.com/Menteion/Menteion/actions/runs/43238887408) | `bbf805d` | 893 |
| `what_the_night_writes_from_the_mirrored_assignment_explore` | [43238887408](https://github.com/Menteion/Menteion/actions/runs/43238887408) | `bbf805d` | 866 |
