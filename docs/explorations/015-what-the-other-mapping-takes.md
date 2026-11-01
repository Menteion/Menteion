---
status: exploratory
date: 2026-11-01
---

# 015: What the other mapping takes

**Branch**: `explore/015-what-the-other-mapping-takes`
**Commit**: `3ffe6a5`, which adds tests and nothing under `src/`. It carries `dd89630`, `main` at ADR-0233 and exploration
014's note, and the tag `explore/014-what-a-night-keeps` with that branch's three changes under `src/`, each unset the
engine it was.
**Runs**: [41297361996](https://github.com/Menteion/Menteion/actions/runs/41297361996) (the harness's trial) and
[41298954420](https://github.com/Menteion/Menteion/actions/runs/41298954420) (the ten tests), both at `scope=arms`

[Exploration 014](014-what-a-night-keeps.md) found that "kept", read as the first block's correct, has no contrast: nothing
takes a mapping away but another mapping, and a mapping's count after the other had run did not run there. Nor did a
replay's pairings, so why about three spikes a unit raise the margin and one or two lower it stayed unread. This note
asks, on seed 22, at 1 024 units, on H-25's two arms under H-46's rules, the slow-wave baseline at 1/2 on this branch
alone:

1. part of the other mapping: a block count of B shorter than the crossing, then A's count and margin, with no gap, an
   equal awake gap, a plain night and F-47's twin, a night before B and a night after it;
2. a second pair: a pair of stimuli sharing the readouts and not the mirror of the first, whether learning it takes
   anything from A, and what a night does then;
3. a replay's pairings: each pre and post interval, what enters the eligibility, what reaches the weights, and why the
   sign falls as it does.

It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the
repository's AArch64 host.

**In brief**
- **B is A's mirror, so "A after B" reads how far the reversal has gone, and nothing else.** The two mappings share their
  stimuli, readouts and synapses, and a trial's answer is correct under exactly one of them. Question 1 reads what a night
  does to that one axis. It cannot read interference.
- **A plain night before B raises the margin and does not hold A's count.** At shift 7 it moves the margin's crossing 1.6
  to 4.3 blocks later. A's count crosses 32 where the rows with no night cross it, or within 0.6 of a block.
- **The margin a night adds is not the margin B takes.** From the eighth image, 1.77 / 1.42 M after two blocks with no gap
  reads 54 / 60, and 1.68 / 1.35 M after four blocks and a plain night on A's patterns reads 24 / 34.
- **The twin takes A sooner**, by 1.1 to 2.2 blocks against no gap at shift 7 in three of the four starts. After B a night
  moves the margin by about a million either way, and the plain night on A's patterns and the twin on B's go the same way.
- **A second pair with synapses of its own takes nothing from A.** Through 24 blocks of it, A read 60 to 64 in every start
  and its margin moved by at most 2 719 of 2.19 to 3.56 M. On this form there is no interference for a night to reduce.
- **Learning does not cross between the pairs; replay does.** A's replay moves the second pair's margin by up to 0.73 M
  and the second pair's replay moves A's by up to 0.54 M. The twin does not reverse that second one.
- **A replay's sign is not its spike count. It is how many of its pairings share a tick.** The rule exempts a post in the
  same tick from depression, and a later spike that shares its post's tick gains 297 and pays nothing. A later pairing
  whose post fired beside it, not on it, about cancels, a spike whose post stayed silent pays, and a first spike has almost
  nothing to gain. From the eighth image, quiet, the units firing twice net more a pairing than those firing three times.

## What ran

**The network and the rules** are 014's: H-42's network of seed 22 at 1 024 units, its derivation run again and held to
H-42's pinned tables (`probe_made`), every one held; H-46's arm, `zero_from`'s, an exponent of 1/4 and the host setting
the zero sum's groups at each load; H-25's configuration; the assignment and the mirrored assignment.

**Two starts**, 014's: the eighth image, whose reversals stand near their floor, and a first mapping, 1 536 trials from
H-42's image. **A** is the mapping in force in the start, **B** its mirror.

**A read.** One block of 64 trials on a fork, the reward withheld, "correct" and its ties beside the state. **The margin** is
014's: the two couplings of the mapping read less the two of the other, here in millions. **Blocks** of B are rewarded and
run on from the image the last length left, so every length of a form is one run.

**A gap** stands between two engines, as in 014. A night is 014's: shift 7 or 5, slow-wave sleep, the pressure at 1.0, under
the drive, the slow-wave baseline at 1/2, the zero sum held. Tagged with the mapping's 102 patterns, each a stimulus unit
and up to eleven of its strongest targets in the answer's readout. Awake gaps are the same ticks with no sleep.

**Nothing under `src/` changed.** `Config::slow_wave_baseline_q16`, `Config::replay_after_gain` and
`Executor::spend_episodes` are 014's, carried by the tag. All three questions are in `tests/inhibition.rs`.

## 1. Part of the other mapping

**What "A after B" can read.** B asks the same stimuli for the other readouts. A trial's answer is correct under exactly one
mapping and a tie under neither, so a withheld block of A reads 64 less its ties less what the same block would read
under B. It is the reversal read from the other end, not a test of memory beside it. All a night can do to it is to move
where the reversal starts or how fast it goes.

Correct of 64 on A after that many blocks of B, a night before B, assignment / mirrored. From the eighth image:

| Gap before B | 0 | 1 | 2 | 3 | 4 | 6 | 8 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| No gap | 64 / 63 | 64 / 61 | 54 / 60 | 44 / 45 | 26 / 34 | 2 / 2 | 0 / 0 |
| Awake, shift 7's length | 64 / 64 | 62 / 63 | 57 / 54 | 54 / 33 | 26 / 34 | 2 / 5 | 1 / 0 |
| Awake, shift 5's length | 63 / 62 | 62 / 63 | 55 / 56 | 36 / 56 | 20 / 31 | 2 / 3 | 0 / 0 |
| Plain, shift 7 | 64 / 64 | 60 / 63 | 57 / 52 | 52 / 41 | 31 / 34 | 2 / 5 | 0 / 1 |
| Twin, shift 7 | 64 / 63 | 58 / 58 | 42 / 46 | 26 / 21 | 7 / 16 | 1 / 1 | 1 / 0 |
| Plain, shift 5 | 64 / 63 | 59 / 64 | 54 / 55 | 28 / 54 | 23 / 28 | 6 / 0 | 1 / 0 |
| Twin, shift 5 | 64 / 64 | 59 / 63 | 53 / 53 | 27 / 44 | 18 / 24 | 1 / 0 | 0 / 0 |

From a first mapping:

| Gap before B | 0 | 2 | 4 | 6 | 8 | 12 | 16 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| No gap | 62 / 62 | 60 / 55 | 50 / 48 | 32 / 43 | 27 / 32 | 7 / 18 | 5 / 3 |
| Awake, shift 7's length | 62 / 63 | 56 / 56 | 46 / 45 | 39 / 37 | 28 / 34 | 11 / 11 | 6 / 3 |
| Awake, shift 5's length | 60 / 62 | 53 / 53 | 47 / 47 | 42 / 39 | 31 / 31 | 13 / 11 | 4 / 4 |
| Plain, shift 7 | 64 / 62 | 56 / 61 | 43 / 52 | 39 / 37 | 23 / 37 | 19 / 14 | 5 / 3 |
| Twin, shift 7 | 62 / 61 | 51 / 53 | 40 / 46 | 32 / 30 | 14 / 25 | 13 / 5 | 6 / 1 |
| Plain, shift 5 | 64 / 63 | 54 / 57 | 43 / 45 | 40 / 45 | 27 / 37 | 16 / 17 | 3 / 6 |
| Twin, shift 5 | 64 / 62 | 53 / 58 | 44 / 48 | 36 / 45 | 20 / 37 | 13 / 14 | 3 / 5 |

**How far a cell can be trusted.** The same couplings read at once and after an awake gap of shift 7's length, twelve pairs
of reads, differ by 3.5 on average and 8 at most. The three rows with no night differ by up to 23 at the steepest block
(33 to 56 at block 3), each its own run of B. A single cell on that slope says little.

Where each row crosses, by linear interpolation over the table's blocks, assignment / mirrored. The margin is at 0 blocks,
A falls through 32, the margin through zero and B's own run rises through 32. From the eighth image:

| Gap before B | Margin, M | A through 32 | Margin through zero | B through 32 |
| :--- | ---: | ---: | ---: | ---: |
| No gap | 3.56 / 3.31 | 3.7 / 4.1 | 4.5 / 4.2 | 4.3 / 5.0 |
| Awake, shift 7's length | 3.56 / 3.31 | 3.8 / 4.1 | 5.0 / 4.1 | 4.6 / 4.5 |
| Awake, shift 5's length | 3.56 / 3.31 | 3.3 / 4.0 | 4.5 / 4.3 | 4.5 / 4.4 |
| Plain, shift 7 | 4.58 / 4.31 | 4.0 / 4.1 | 6.4 / 5.8 | 4.6 / 4.8 |
| Twin, shift 7 | 2.66 / 2.30 | 2.6 / 2.6 | 2.9 / 2.5 | 3.1 / 3.8 |
| Plain, shift 5 | 3.90 / 3.59 | 2.8 / 3.8 | 5.0 / 4.5 | 4.7 / 4.3 |
| Twin, shift 5 | 3.33 / 3.00 | 2.8 / 3.6 | 4.1 / 3.6 | 4.3 / 3.7 |

From a first mapping:

| Gap before B | Margin, M | A through 32 | Margin through zero | B through 32 |
| :--- | ---: | ---: | ---: | ---: |
| No gap | 2.19 / 2.97 | 6.0 / 8.0 | 7.5 / 11.5 | 9.6 / 8.7 |
| Awake, shift 7's length | 2.18 / 2.97 | 7.3 / 8.3 | 7.7 / 11.2 | 10.0 / 8.9 |
| Awake, shift 5's length | 2.18 / 2.97 | 7.8 / 7.8 | 7.8 / 12.1 | 9.0 / 10.5 |
| Plain, shift 7 | 3.27 / 3.87 | 6.9 / 8.9 | 11.8 / 15.6 | 8.0 / 9.9 |
| Twin, shift 7 | 1.11 / 2.21 | 6.0 / 5.8 | 2.3 / 6.8 | 6.6 / 7.7 |
| Plain, shift 5 | 2.56 / 3.21 | 7.2 / 9.0 | 8.8 / 13.3 | 8.8 / 10.7 |
| Twin, shift 5 | 1.91 / 2.75 | 6.5 / 8.9 | 5.5 / 11.0 | 8.0 / 10.8 |

- **A plain night does not hold A's count against B.** The rows with no night cross 32 at 3.3 to 4.1 blocks on the eighth
  image, and at 6.0 to 7.8 from the assignment and 7.8 to 8.3 from the mirrored on a first mapping. Plain at shift 7 reads
  4.0 / 4.1 and 6.9 / 8.9, while its margin's crossing moves 1.9 / 1.6 and 4.3 / 4.1 blocks later. Shift 5 reads the same way.
- **The count does not follow the margin a night bought.** After four blocks the plain night at shift 7 stands at 2.00 /
  1.55 M and reads 31 / 34, where no gap at 1.77 / 1.42 M, after two blocks, reads 54 / 60. Over the 256 reads of question
  1, a margin of 2 to 3 M reads 37 to 64 of 64.
- **The twin takes A sooner.** At shift 7 A crosses 32 at 2.6 / 2.6 and 6.0 / 5.8, ahead of the rows with no night by 0.7 to
  1.5 and 0 to 2.5 blocks. Only the first mapping from the assignment stays at those rows' edge. At shift 5 it stays near them.
- **B rises as 014's reversals did.** The twin at shift 7 brings B through 32 ahead of the rows with no night, at 3.1 / 3.8 and
  6.6 / 7.7. Plain leaves it at 4.6 / 4.8 and 8.0 / 9.9.

**A night after B**, shift 7, before the read: the patterns of A from the start image, or B's from the image after B, each
night spending the older episodes. Correct of 64 on A and the margin in millions, assignment / mirrored. From the eighth image:

| Night after B | 2 blocks | 4 blocks | 8 blocks | Margin, 2 blocks | 4 blocks | 8 blocks |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Awake | 54 / 53 | 22 / 37 | 2 / 0 | 1.77 / 1.41 | 0.49 / 0.20 | −2.15 / −2.73 |
| Plain, A's patterns | 60 / 56 | 24 / 34 | 0 / 0 | 2.85 / 2.39 | 1.68 / 1.35 | −0.84 / −1.40 |
| Twin, A's patterns | 53 / 39 | 17 / 19 | 0 / 0 | 0.99 / 0.36 | −0.22 / −0.85 | −2.64 / −3.44 |
| Plain, B's patterns | 56 / 51 | 26 / 24 | 0 / 1 | 0.59 / 0.19 | −0.89 / −1.18 | −3.21 / −3.86 |
| Twin, B's patterns | 63 / 59 | 37 / 39 | 1 / 0 | 2.89 / 2.28 | 1.41 / 0.84 | −1.21 / −1.91 |

From a first mapping:

| Night after B | 4 blocks | 8 blocks | 12 blocks | Margin, 4 blocks | 8 blocks | 12 blocks |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Awake | 45 / 49 | 24 / 30 | 14 / 10 | 0.76 / 1.63 | −0.11 / 0.87 | −1.14 / −0.13 |
| Plain, A's patterns | 45 / 58 | 26 / 39 | 14 / 13 | 1.93 / 2.65 | 0.89 / 1.94 | 0.08 / 0.90 |
| Twin, A's patterns | 39 / 55 | 20 / 28 | 7 / 7 | −0.32 / 0.86 | −1.36 / −0.01 | −2.29 / −1.19 |
| Plain, B's patterns | 40 / 50 | 20 / 25 | 9 / 5 | −0.20 / 0.41 | −1.20 / −0.27 | −2.28 / −1.50 |
| Twin, B's patterns | 50 / 57 | 33 / 43 | 15 / 16 | 1.85 / 3.02 | 0.79 / 2.12 | −0.27 / 0.85 |

- **The night moves the state along one axis.** Against the awake read, over all twelve cells: plain on A's patterns
  +0.98 to +1.33 M, plain on B's −0.96 to −1.38, twin on A's −0.49 to −1.25, twin on B's +0.64 to +1.39. A strengthened
  mapping and its mirror weakened are nearly one move.
- **It gives back margin and little count.** Past the crossing nothing comes back: at eight blocks from the eighth image
  every night reads 0 or 1. Plain on A's patterns after four blocks reads 24 / 34 against the awake read's 22 / 37, though it
  sets the margin where two blocks left it. Of the twelve cells, it stands more than 8 above the awake row in two, both on a
  first mapping from the mirrored assignment, and the twin on B's patterns in four.
- **The twin on B's patterns read as high as plain on A's or higher in eleven of twelve cells**, the margins about equal.
  Most of those gaps lie within a read's spread, and this ran once.

## 2. A second pair

**The pair.** The question asked for stimuli outside the existing sets. There are none: every place of a period lies in
exactly one set, and only the places past the last whole period lie in none. So the pair comes from the readouts, with no
change to `Task::check`, which refuses overlapping sets within a task. C is place 5, of R0, and D is place 16, of R1, 51
units each, eleven places apart as A and B are. The second task's sets are C, D, R0 less place 5 and R1 less place 16. A's
task keeps its full readouts, so C's and D's units stay among A's readouts and the zero sum's targets. No place is nine or
more from both A and B, so C and D lie within a local synapse of them. Each start runs the pair both ways round, as it came
and flipped.

**The zero sum holds the new groups.** `ZeroGroups::further`, in the harness, adds two groups at each load, over the same
918 targets as A's and B's. Their values are each pair's sum in the start image, not H-42's, so that the hold begins where the
pair stands: from [11 219 414, 11 910 369] to [11 261 576, 11 916 892] across the four starts, H-42's being [11 229 658,
11 911 949], at most 0.3 % apart. The groups' own sums were not dumped. The pair's two couplings summed by stimulus, a
narrower sum, ranged over at most 32 293 on 10.9 M and 31 419 on 11.6 M in the forty reads of each start.

No gap, correct of 64 after that many blocks of the second pair, each read on a fork. The second pair's column is as it came /
flipped, and the two after it are its two ways round:

| Start, arm | A at the start | A after 6, 12, 24 blocks | Second at the start | Second after 6, 12, 24 | Flipped |
| :--- | ---: | :--- | ---: | :--- | :--- |
| Eighth, assignment | 64 | 63 to 64 | 33 / 22 | 60, 62, 62 | 60, 60, 64 |
| Eighth, mirrored | 63 | 62 to 64 | 19 / 36 | 60, 57, 62 | 59, 63, 62 |
| First mapping, assignment | 62 | 60 to 63 | 30 / 27 | 57, 62, 64 | 59, 59, 64 |
| First mapping, mirrored | 62 | 62 to 64 | 31 / 29 | 60, 59, 64 | 58, 62, 64 |

- **A stands.** Its margin moved by at most 2 719 of 3 564 337 on the eighth image and 2 415 of 2 185 564 on a first
  mapping, in all 24 reads. Its count stands where 014's awake gaps read it.
- **The second pair learns like a new mapping.** From 19 to 36 at the start and margins of ±0.21 to ±0.23 M, it first reads 48
  by block 3 to 6, 56 by block 5 to 7, and 58 to 64 in its last four blocks. Its margin ends at 2.06 to 2.72 M.
- **That is how the rule has it**, as a reading. An excitatory coupling of A's moves at an A unit's spike under the reward's
  gate, and a reward in the second pair's trials reaches the units the critic counted in their window. What the pairs share
  is readout units as targets, the inhibition and the gain. Whether A's units fired in those trials was not read.

A night before the second pair changed neither count: after 24 blocks A read 58 to 64 and the pair 61 to 64 under every gap,
the twin's lower margin on A giving the lowest of A's. The pair's own margin at 24 blocks moved both ways, 1.87 to 3.34 M
after plain and 2.13 to 3.17 M after the twin, against 2.06 to 2.72 M with no gap.

After the 24 blocks, a night at shift 7. Every count read 61 to 64, so only the margins have a contrast. Change from the awake
read, in millions, the range over the four starts and both ways round, the median in brackets:

| Night, its patterns | A's margin | The second pair's margin |
| :--- | :--- | :--- |
| Plain, A's | +0.94 to +1.30 (+1.02) | −0.63 to +0.73 (+0.09) |
| Twin, A's | −1.06 to −0.47 (−0.95) | −0.26 to +0.65 (+0.16) |
| Plain, the second pair's | −0.31 to +0.44 (0.00) | +0.86 to +1.02 (+0.97) |
| Twin, the second pair's | −0.35 to +0.54 (+0.02) | −0.85 to −0.61 (−0.74) |
| Plain, both | +0.45 to +0.79 (+0.54) | +0.16 to +0.80 (+0.53) |
| Twin, both | −0.58 to +0.01 (−0.43) | −0.50 to −0.16 (−0.34) |

- **A pair's replay does to it what A's does to A.** The second pair's margin rises by +0.86 to +1.02 M and falls by 0.61 to
  0.85 M, F-47's sign holding on a pair that has had one mapping.
- **Replay crosses where learning does not.** A's plain replay moves the second pair's margin by up to 0.73 M, the sign turning
  with the pair's way round on two of the four starts, so it moves the couplings and not the mapping. The second pair's replay
  moves A's margin by up to 0.54 M, and plain and twin move it alike, where on its own pair they differ in sign.
- **Both together split the dose.** A night's replays go round the ledger, so each pair takes about half of what it takes alone.
- **How it crosses was not taken apart.** Plain and twin agree in sign on the other pair and differ on their own, so it is not
  the arithmetic of a pattern's own pairs in section 3.

## 3. What a replay pairs

**The watch.** `PairWatch`, in the harness, reads every pairing of a night's pattern units tick by tick from the arena and the
train, computes it by the rule and holds it to the trace and the weight the engine wrote. Sixteen nights at shift 5, a half,
plain and twin, driven and quiet, both starts, both arms: 5 714 816 pairings, `wrong` 0 in all sixteen. One replay of each
night is dumped whole. The margins they read are 014's section 2 and 4 figures bit for bit: +330 757, −235 114, +556 143,
−140 048 from the assignment and +282 060, −311 298, +468 236, −154 727 from the mirrored.

**The rule.** At a presynaptic spike at $t$, the previous one at $q$, the postsynaptic unit's last at $p$: a potentiation
$A_+ w(p - q)$ when $p > q$, a depression $A_- w(t - p) (M/M_{\text{ref}})^{1/4}$ when $p < t$, $w(n) = (1 - 2^{-11})^n$,
$A_+ = 328$ and $A_- = 344$. At a modulation of a half, half of the trace then moves into the weight. **Three things follow
here.**
- The window's time constant is 2 048 ticks and a unit's refractory 200, so a replay's volleys stand 201 ticks apart, 3, 204,
  405 in the plain driven replays dumped, and everything inside a ripple reads nearly whole: $w(201)$ is 0.91.
- A pair's offset is set in the first volley and carried by the refractory clock. Under the drive the share of a stimulus
  unit's pairings in the same tick is the same at its first and second spike, and plain at its third: 31 to 36 % plain, 13
  to 16 % twin, at every start.
- `since_post > 0` is strict. A post in the same tick pays no depression.

**So a pairing is one of these.** The mean per pairing, over the sixteen replays dumped whole, within the pattern:

| Pairing | Potentiation | Depression | Net |
| :--- | ---: | ---: | ---: |
| The first spike, its post in the same tick | 0 | 0 | 0 |
| The first spike, its post 1 to 7 ticks ahead | 0 | 326 to 328 | −326 to −328 |
| The first spike, its post in neither | 48 to 58 | 0 | +48 to +58 |
| A later spike, its post in the same tick | 297 | 0 | **+297** |
| A later spike, its post fired since the previous spike | 292 to 319 | 274 to 319 | −5 to +18 |
| A later spike, its post silent since the previous spike | 0 | 229 to 260 | **−229 to −260** |
| The next spike, 14 000 ticks or more on, its post fired since the replay's last | 192 | 1 | +191 |

The first spike has no previous spike in reach. A later pairing with its post near it, not on it, earns and pays almost
equally: the window is ten times the gap between volleys, so the rule barely tells the order.

The pattern's own synapses, a stimulus unit onto its strongest targets, summed over the night by the spike that read them.
From the eighth image, the assignment; the units' spikes are of none, one, two, three, more:

| Form | Units' spikes | Same tick, first / second spike | First spike | Second | Third on | Next spike | Net | Into the weights |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Plain, driven | 5, 0, 61, 2 721, 314 | 34 / 34 % | −425 754 | +469 122 | +415 095 | +128 489 | +586 952 | +276 488 |
| Twin, driven | 10, 159, 2 471, 436, 25 | 15 / 15 % | −547 412 | +244 640 | −57 743 | +183 269 | −177 246 | −226 840 |
| Plain, quiet | 0, 0, 2 005, 1 079, 17 | 97 / 97 % | −32 555 | +1 217 502 | −147 603 | +5 626 | +1 042 970 | +683 363 |
| Twin, quiet | 0, 2 803, 286, 12, 0 | 90 / 0 % | −74 814 | −186 178 | 0 | +1 197 | −259 795 | −133 609 |

The sign of the net and of what reaches the weights is the same in all four starts: +261 416 to +586 952 and +860 743 to
+1 435 272 for plain, −419 038 to −177 246 and −278 293 to −259 795 for the twin, driven and quiet.

**Why the sign falls as it does.**
- **A first spike has almost nothing to gain.** Under the drive a third to two-fifths of a stimulus unit's pairings have the
  post just ahead, and they pay about −327 with nothing against it. Quiet, nearly every pair shares the tick and the first
  spike costs almost nothing.
- **A later spike gains only from posts in its own tick.** A later volley pays +297 a same-tick pair and about nothing for
  the rest. Plain under the drive has two later volleys at 34 %, so 0.34 × 297 twice beats the first spike's −100 a pairing.
  The twin has one at 15 % against −129, and it does not.
- **A spike without its post pays.** A third spike is positive only as a volley. The twin's third spikes under the drive are
  stragglers, none in the tick, and net −57 743. Quiet, from the eighth image and the mirrored assignment, plain's third spikes
  share the tick at 2 % and net −446 162; from a first mapping they share it at 49 % and net +65 231. A fourth spike nets
  −40 603 under the plain drive.
- **The first spike's loss reaches the weight first.** Half of it moves at once, before anything can offset it. Under the twin
  the second spike's net +244 640 only pays that back, and moves −272 into the weights. What stands in the trace after the
  last spike waits for the unit's next spike.

**So the count is not the cause.** Three spikes and two go with the sign because the drive sets both the volleys and the
share in a tick:
- **Quiet, plain**: from the eighth image the units firing twice net 274 and 287 a pairing, those firing three times 220 and
  53, a third spike being the one that loses its post. From a first mapping, where the third spike keeps it half the time,
  they net 295 and 264 against 306 and 310.
- **Under the twin's drive three spikes do not help**: units firing three times net −50 470 over 680 pairings.
- **Under the plain drive two spikes do not hurt by their number**: of the 105 pairings of units firing twice, 3 share a tick,
  and they net −20 870.

**The margin.** A pattern's stimulus unit onto the answer's readout, the stimulus units' other spikes onto it, and the same onto
the other readout add to the night's change in margin with its sign in all sixteen nights, at 0.90 to 1.22 of it. The twin's
four nights from the eighth image lie at 0.99 to 1.03. The remainder is unread: REM's depotentiations and the zero sum's
rounds were not counted. Under the drive part of plain's rise is not the pattern's. The other readout falls by 100 951 of the
360 137 from the eighth image and the assignment, and by 205 776 of 210 768 from a first mapping and the mirrored assignment.
Under the twin it rises on the eighth image and falls on a first mapping. Those posts are not driven, and why they move was
not taken apart.

## What it costs

The ten tests took 371 to 3 514 s a job: the pairings 371 and 375 s, the other mapping 1 958 to 3 207 s for seven gaps and
twelve nights after B, the second pair 3 257 to 3 514 s. The harness's trial took 843 s. Nothing neared a job's 240 minutes,
and every question ran whole.

## What it suggests

**Read against the mirror, "kept" reads a reversal.** That is one axis, and a night moves along it. The count did not follow the
margin there, so a test of the margin and a test of the count would disagree on what a night kept.

**Read against a second pair, "kept" has nothing to read.** A stands at the ceiling with or without a night, as 014's
gaps did. Interference on this task lives where one synapse is asked two things, and the only such ask here is the mirror.
A pair that shares some of A's sources would ask it elsewhere. That did not run.

**F-47's sign rests on one tick.** What the plain night adds lies in the pairs of a later volley that share a tick, the
pairs a strict inequality exempts from depression. Almost everything else in a ripple cancels, because the window is ten
times the gap between volleys. So the sign is a property of tick-exact coincidence, of the refractory clock that repeats it,
and of a consolidation that spends half the first spike's loss at once. The decision ADR that settles F-47 settles that
too, whichever sign it wants.

**A reading, not measured.** The twin's lower message may spread the first volley's crossings over more ticks, and the plain
drive's overshoot pull them together. 014's first-spike latencies, 3.0 against 5.7 ticks, fit that; quiet, at 5.0 ticks and 97 %
in the tick, shows that latency is not all of it. Nothing here read a unit's potential.

Questions this suggests for a later exploration or a round:
- a same-tick post paid as either order, or the window nearer a volley's gap, read for the night's sign;
- where the count lives: the weights' distribution across a reversal and a night, since the margin does not carry it;
- a stimulus pair sharing some of A's sources, as a task of its own;
- why the second pair's replay moves A's margin the same way plain and twin, and why the other readout moves as it does;
- the same on seeds 23 and 24.

## What did not run

- **Seeds 23 and 24.**
- **The default as a control.** Every run here is under H-46's rules.
- **Nights over more than one reversal**, or two nights in one form, and any shift but 7 and 5. A night after B ran at shift 7
  alone.
- **A second stimulus pair sharing sources with A**, or a reversal of the second pair.
- **A's units' spikes in the second pair's trials**, and the groups' own sums for the second pair.
- **The weights' distribution.** So why the count leaves the margin behind stays unread.
- **A unit's potential in a replay**, so what sets the share in a tick stays unread.
- **REM's depotentiations and the zero sum's rounds in the pairing tally.**

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_what_the_other_mapping_takes_explore` | [41297361996](https://github.com/Menteion/Menteion/actions/runs/41297361996) | `3ffe6a5` | 843 |
| `what_the_other_mapping_takes_of_the_eighth_image_on_seed_22_from_the_assignment_explore` | [41298954420](https://github.com/Menteion/Menteion/actions/runs/41298954420) | `3ffe6a5` | 1988 |
| `what_the_other_mapping_takes_of_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [41298954420](https://github.com/Menteion/Menteion/actions/runs/41298954420) | `3ffe6a5` | 1958 |
| `what_the_other_mapping_takes_of_a_first_mapping_on_seed_22_from_the_assignment_explore` | [41298954420](https://github.com/Menteion/Menteion/actions/runs/41298954420) | `3ffe6a5` | 3078 |
| `what_the_other_mapping_takes_of_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [41298954420](https://github.com/Menteion/Menteion/actions/runs/41298954420) | `3ffe6a5` | 3207 |
| `what_a_second_pair_takes_of_the_eighth_image_on_seed_22_from_the_assignment_explore` | [41298954420](https://github.com/Menteion/Menteion/actions/runs/41298954420) | `3ffe6a5` | 3257 |
| `what_a_second_pair_takes_of_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [41298954420](https://github.com/Menteion/Menteion/actions/runs/41298954420) | `3ffe6a5` | 3321 |
| `what_a_second_pair_takes_of_a_first_mapping_on_seed_22_from_the_assignment_explore` | [41298954420](https://github.com/Menteion/Menteion/actions/runs/41298954420) | `3ffe6a5` | 3405 |
| `what_a_second_pair_takes_of_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [41298954420](https://github.com/Menteion/Menteion/actions/runs/41298954420) | `3ffe6a5` | 3514 |
| `what_a_replay_pairs_on_seed_22_from_the_assignment_explore` | [41298954420](https://github.com/Menteion/Menteion/actions/runs/41298954420) | `3ffe6a5` | 375 |
| `what_a_replay_pairs_on_seed_22_from_the_mirrored_assignment_explore` | [41298954420](https://github.com/Menteion/Menteion/actions/runs/41298954420) | `3ffe6a5` | 371 |
