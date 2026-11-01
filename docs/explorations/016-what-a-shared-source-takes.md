---
status: exploratory
date: 2026-11-01
---

# 016: What a shared source takes

**Branch**: `explore/016-what-a-shared-source-takes`
**Commit**: `d1fe35d` for the harness's trial and question 1; `e9ddfe6` for question 2, which adds tests and nothing under
`src/`. Both carry `90aead0`, `main` at exploration 015's note, and the tag `explore/015-what-the-other-mapping-takes` with
014's three changes under `src/`. `d1fe35d` adds one more, each unset the engine it was.
**Runs**: [41309320406](https://github.com/Menteion/Menteion/actions/runs/41309320406) (the harness's trial),
[41310255988](https://github.com/Menteion/Menteion/actions/runs/41310255988) (question 1) and
[41315401902](https://github.com/Menteion/Menteion/actions/runs/41315401902) (question 2), all at `scope=arms`

[Exploration 015](015-what-the-other-mapping-takes.md) read "kept" against the mirror as a reversal, and against a second
pair with sources of its own as nothing at all: A stood. Interference, it said, lives where one synapse is asked two
things, and the only such ask there was the mirror. This note gives the second pair some of A's and B's sources, as a task
of its own. It asks, on seed 22, at 1 024 units, on H-25's two arms under H-46's rules, the slow-wave baseline at 1/2 on this
branch alone:

1. with no night, whether such a pair takes anything from A, at several shares, its answers conflicting with A's and, as a
   control, agreeing with them; and whether the pair learned, without which A's standing would say nothing;
2. where A's count falls, what a night saves: a night before the pair and a night halfway through it, plain and F-47's
   twin, against no gap and an equal awake gap, with A's patterns, the pair's and both.

It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the
repository's AArch64 host.

**In brief**
- **A pair that shares A's sources and conflicts with A takes A, by the share.** After 24 blocks A reads 58, 46, 47, 58 at 13
  of 51 units shared, 42, 27, 33, 50 at 26, and 8, 5, 20, 16 at 39, against 64, 63, 63, 63 at no share. The pair has learned
  by then, at 60 to 64.
- **A pair that agrees with A takes nothing.** At every share A reads 61 to 64, and its margin ends 0.18 to 0.80 M higher.
- **All of it shows on the shared units.** With each kind of unit held apart, the unshared units' margin moved by at most 3 592
  in 72 reads, conflicting and agreeing. Conflicting, the shared units' fell through zero, from +0.64 to +3.26 M to −0.03 to
  −2.08 M. It reads as a reversal of part of A's sources.
- **A unit is not its share of the margin.** 13 units of 51 hold 18 to 56 % of it, and A's count fell by 6 and 5 where they
  held 18 and 22 %, and by 17 and 16 where they held 56 %.
- **The cut of the zero sum's groups did not decide it.** Held apart, or left in A's and B's groups, the shared units gave
  counts within 6 of each other at every read.
- **A night moved the margin, and the count did not follow it.** At 26 units a plain night of A's patterns halfway left the
  margin about 1.1 M higher twelve blocks later. After the 24 blocks its counts were 48, 31, 51, 47 before the pair and 47, 26,
  49, 42 halfway, where the forms with no night read 42 to 50, 25 to 37, 33 to 42 and 41 to 50 with their margins within 0.13 M.
  Of the 32 night cells 18 lie inside those ranges, 3 below and 11 above, 7 of the 11 in one start.
- **Right after a night the count stayed within a read's spread.** No night stood more than 8 above both rows with no night, and
  twelve blocks later plain on A's patterns read +5, −11, +10, −6 against the awake row, with its margin up in all four.

## What ran

**The network and the rules** are 015's: H-42's network of seed 22 at 1 024 units, its derivation run again and held to
H-42's pinned tables (`probe_made`), every one held; H-46's arm, an exponent of 1/4 and the host setting the zero sum's groups
at each load; H-25's configuration; the assignment and the mirrored assignment.

**Two starts**, 014's: the eighth image and a first mapping, 1 536 trials from H-42's image. **A** is the mapping in force in
the start. Its two stimuli are A and B, as in the geometry; where a sentence means the stimulus it says so.

**A read** is one block of 64 trials on a fork, the reward withheld, "correct" and its ties beside the state. **The margin** is
014's: the mapping's two couplings less the other two, in millions.

**The pair.** It is 015's C and D, places 5 and 16, as a task of its own, its sets C, D, R0 less place 5 and R1 less place 16 at
every share, so its readouts do not change with the share. `Task::check` refuses overlapping sets only within one task, so A's
task and the pair's may share units. A's task keeps its full readouts. What changes is the units each stimulus presents, one
a period, 51 in all: in a period the share names, C presents stimulus A's unit in place of its own and D presents stimulus B's.
The periods are numbered 0 to 50 from the rotation:

| Units shared of 51 | Periods |
| ---: | :--- |
| 0 | none, 015's pair |
| 13 | `j % 4` is 0 |
| 26 | `j % 4` is 0 or 2 |
| 39 | `j % 4` is 0, 1 or 2 |
| 51 | all, the mirror |

The shares nest, and each spreads round the ring, so that a shared unit and an unshared one differ in no place on it.
**Conflicting** gives C the readout A's answer is not, and D the one B's is not. **Agreeing** gives C A's answer and D B's.
**The pair's margin** is the same difference over the units each stimulus presents, onto the pair's own two readouts, worked out
from the kinds below. The couplings a read's state dumps for the pair are places 5 and 16 alone, and no figure here uses them.

**Lengths.** Each of 6, 12 and 24 blocks runs from the start image, as 015's second pair did. Every shorter run's curve is the
head of the longest's. Each read also dumps six kinds of unit onto each readout: A alone, shared with C, C alone, and the same of
B and D. The kinds are held to add up to A's and B's couplings at every read.

**The geometry gives way a little.** Every stimulus unit stands at least nine places from every other in a task, and the pair
keeps that within C and within D. Between them it does not: where a period outside the share is followed by one inside it, D's
place 16 stands four places before C's place 0. That is 13, 25 and 12 pairs of units within a local synapse at the three shares,
one of the 13 at the ring's join, eight places apart, and none at 0 or 51. Whether those synapses exist was not read.

### How the groups were cut

`Executor::set_zero_sum` refuses a source named twice, whole, and it stayed as it is. So stimulus A's group and C's cannot both
hold a shared unit. Two cuts ran, over the same 918 targets:

- **Apart**, the main one. Six groups, each kind of unit its own, at its sum in the start image. It holds both stimuli's sums,
  each being two held groups, and at no share it comes to 015's four groups and at 51 to H-46's two, each at its sum in the start
  image. It was chosen for that. What it adds is a wall H-46 did not have: no weight crosses between kinds, so the shared units
  can move weight only between the two readouts, and the unshared units cannot pay for them.
- **Kept**, the control. The shared units stay in A's and B's groups at H-42's values, as H-46 ran them, and C's and D's own
  units are held beside them at their sums in the start image. A's group is exactly H-46's. C's sum is not held, and what a
  shared unit gains its group's other units pay for.

**What the cut could do to the readings.** Apart might understate what the pair takes, since it walls the unshared units off.
Kept might overstate it, since a shared unit can draw weight through the group. So Kept ran at all three shares, conflicting,
and section 1 reads the difference. Stimulus A's kinds and stimulus B's sum to H-42's values within 7 and 18 in every start.
Under Apart no kind's sum moved by more than 476 in question 1 or 344 through question 2's nights. Under Kept at the three shares
A's whole group moved by at most 75 and B's by 38, while their kinds exchanged up to 408 339. At no share, 015's groups, the
two moved by at most 507 and 631.

### Under `src/`

A `Set` is one mask over a period of at most 32 places, and the geometry's period is 20, so no set names "stimulus A's unit in
every fourth period". `d1fe35d` adds, on this branch alone, `Task::trial_presenting`, which takes each stimulus's units as a
list, with `Stimulus::inject_into` and `Stimulus::cancel_into`. `Task::trial` is `trial_presenting` with no list. `Task::check`
reads the sets alone, so the harness holds that the units presented lie in neither of the pair's readouts and in one stimulus.

**It changes nothing unset, and nothing at no share.** Through the list, the harness's trial ran two blocks of 015's pair to
the same trace and the same image's CRC-64 as through the sets. Question 1's row at no share reads as 015's second pair, its
second way round: all 24 reads hold their counts, ties, traces, couplings, gains and sums to 015's logs, in all four starts.

## 1. Does a shared source take anything?

A's count of 64 after 6, 12 and 24 blocks of the pair, no gap, each kind held apart, the first row 015's pair under its
groups. A read at the start 64, 63, 62, 62:

| Units shared, answers | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| 0, exploration 015's pair | 63, 64, 64 | 64, 63, 63 | 62, 61, 63 | 62, 64, 63 |
| 13, conflicting | 62, 61, 58 | 59, 57, 46 | 55, 52, 47 | 59, 61, 58 |
| 26, conflicting | 48, 44, 42 | 54, 37, 27 | 53, 47, 33 | 57, 54, 50 |
| 39, conflicting | 21, 13, 8 | 27, 9, 5 | 47, 28, 20 | 50, 44, 16 |
| 51, conflicting | 9, 1, 0 | 10, 2, 0 | 43, 18, 5 | 45, 24, 4 |
| 13, agreeing | 63, 64, 64 | 64, 63, 64 | 62, 63, 63 | 63, 64, 63 |
| 26, agreeing | 63, 64, 64 | 64, 63, 64 | 62, 63, 63 | 64, 64, 63 |
| 39, agreeing | 64, 64, 64 | 63, 63, 64 | 61, 62, 64 | 64, 64, 63 |

- **The baseline.** 015 read the same couplings twice 3.5 apart on average and 8 at most. Twelve blocks into the
  form at 26 units, read at once and after an awake gap, A differs here by 1, 1, 6 and 6.
- **Conflicting, A falls by the share.** Against the first row at the same block, at 13 units the fall passes 8 in one start at
  block 12 (9) and two at block 24 (17, 16). At 26 units it passes 8 in three starts by block 6 (15, 10, 9) and in all four by
  block 12 (20, 26, 14, 10). At 39 units it does so by block 6 in all four.
- **Agreeing, A does not fall.** It reads 61 to 64 at every share, its ties 0 to 2.
- **51 units is the mirror again**, from the pair's smaller readouts. A reads 9 / 10 at block 6 from the eighth image and 43 / 45
  from a first mapping, assignment / mirrored, where 015's rows with no gap read 2 / 2 and 32 / 43.

A's ties at the same reads, conflicting. A count that falls here is not all a count of the other answer:

| Units shared | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| 13 | 1, 1, 2 | 2, 2, 5 | 4, 2, 4 | 1, 1, 3 |
| 26 | 4, 7, 4 | 4, 5, 9 | 3, 4, 11 | 3, 3, 2 |
| 39 | 6, 3, 3 | 4, 3, 4 | 10, 12, 5 | 3, 7, 9 |
| 51 | 2, 1, 0 | 4, 1, 1 | 8, 9, 0 | 8, 15, 3 |

The pair's own count, withheld: at the start, then after 6, 12 and 24 blocks, the same forms:

| Units shared, answers | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| 0, exploration 015's pair | 22; 60, 60, 64 | 36; 59, 63, 62 | 27; 59, 59, 64 | 29; 58, 62, 64 |
| 13, conflicting | 18; 62, 61, 64 | 2; 43, 61, 63 | 12; 54, 61, 62 | 19; 57, 62, 64 |
| 26, conflicting | 6; 63, 64, 64 | 1; 39, 60, 62 | 7; 50, 61, 64 | 13; 55, 63, 62 |
| 39, conflicting | 0; 56, 63, 64 | 1; 53, 64, 64 | 11; 43, 60, 63 | 3; 25, 44, 60 |
| 51, conflicting | 0; 59, 64, 64 | 1; 56, 64, 64 | 3; 26, 45, 60 | 2; 11, 33, 62 |
| 13, agreeing | 42; 61, 63, 63 | 60; 63, 63, 64 | 50; 61, 63, 64 | 39; 63, 60, 63 |
| 26, agreeing | 56; 59, 63, 64 | 63; 64, 62, 64 | 53; 62, 63, 64 | 45; 61, 63, 63 |
| 39, agreeing | 64; 63, 64, 64 | 62; 63, 62, 64 | 51; 58, 62, 62 | 58; 61, 62, 63 |

- **The pair learned in every form.** Conflicting at a share, it reads 60 to 64 at block 24 and 57 to 64 through the last four
  rewarded blocks. So A's fall is not a pair that failed, and A's standing when agreeing is not one either.
- **Conflict slows it, most at the higher shares.** Its rewarded blocks first reach 48 by block 4 to 8 at 13 and 26 units, 6 to 13 at 39,
  and 5 to 16 at 51, against 4 to 6 at no share. Agreeing, by block 1 or 2, the shared units already giving the answer: it starts
  at 39 to 64.

A's margin in millions, each from the start to block 24, conflicting, each kind held apart: the unshared units; the shared units:

| Units shared | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| 13 | 2.92 to 2.92; 0.64 to −0.78 | 1.45 to 1.44; 1.86 to −0.18 | 0.96 to 0.97; 1.22 to −0.03 | 2.32 to 2.32; 0.66 to −0.32 |
| 26 | 2.06 to 2.06; 1.50 to −1.01 | 0.92 to 0.92; 2.39 to −0.64 | 0.81 to 0.81; 1.37 to −0.52 | 1.60 to 1.60; 1.37 to −0.27 |
| 39 | 0.30 to 0.31; 3.26 to −0.92 | 1.14 to 1.14; 2.17 to −2.08 | 1.05 to 1.05; 1.13 to −1.51 | 0.27 to 0.27; 2.70 to −0.33 |

- **The unshared units' margin stands.** Across these shares, under this cut, conflicting and agreeing, it moved by at most 3 592
  in all 72 reads.
- **The shared units' margin crosses zero at every share**, ending at −0.03 to −2.08 M.
- **Agreeing, the shared units' margin rises**, and the whole ends 0.18 to 0.80 M above the start.
- **A's unshared units do fire in the pair's trials.** 015 left that unread. Over the last 12 of the 24 blocks, conflicting at 26
  units, a unit of A alone fires 6.7 to 7.3 times a block in C's trials, a shared unit 37.4 to 38.2, and a unit of A at no
  share 7.0 to 7.5. So A's unshared units fire at about a fifth of a presented unit's rate, and their couplings do not move.
  Why not was not read.

**A unit is not its share of the margin.** The shared units hold, at the start, in the columns' order:

| Units shared | Part of A's margin |
| ---: | :--- |
| 13 | 0.18, 0.56, 0.56, 0.22 |
| 26 | 0.42, 0.72, 0.63, 0.46 |
| 39 | 0.91, 0.65, 0.52, 0.91 |

The periods with `j % 4` of 0, 1, 2 and 3 hold, in millions, 0.64, 1.76, 0.86, 0.30 from the eighth image and the assignment, and
1.22, −0.24, 0.15, 1.05 from a first mapping and the assignment. Why the margin lies so unevenly was not read. It does read in the
count: at 13 units A's fall at block 24 is 6 and 5 where they hold 18 and 22 %, and 17 and 16 where they hold 56 %.

Over all 84 reads of A at a share, conflicting, by either cut:

| Margin, M | Reads | Correct of 64 |
| :--- | ---: | ---: |
| −3 to −1 | 7 | 0 to 5 |
| −1 to 0 | 18 | 5 to 32 |
| 0 to 0.5 | 11 | 18 to 47 |
| 0.5 to 1 | 8 | 39 to 51 |
| 1 to 1.5 | 19 | 42 to 57 |
| 1.5 to 2 | 10 | 45 to 59 |
| 2 to 2.5 | 11 | 56 to 62 |

The count follows the whole margin here more closely than 015 read across a reversal, where 2 to 3 M read 37 to 64. Neighbouring
bands still overlap, by up to 14 of 64.

**The cut did not decide it.** A's count, Apart / Kept, at 6, 12 and 24 blocks, conflicting:

| Units shared | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| 13 | 62/61, 61/60, 58/58 | 59/57, 57/51, 46/47 | 55/56, 52/51, 47/46 | 59/59, 61/60, 58/58 |
| 26 | 48/49, 44/42, 42/39 | 54/52, 37/34, 27/24 | 53/54, 47/46, 33/37 | 57/56, 54/55, 50/50 |
| 39 | 21/18, 13/14, 8/8 | 27/23, 9/9, 5/4 | 47/51, 28/32, 20/19 | 50/49, 44/44, 16/18 |

The two differ by at most 6 in A's count, 7 in the pair's and 207 012 in A's margin. Over these 36 reads Kept let the unshared units'
margin move by up to 92 888 against Apart's 3 301, and A falls under either. Whether a hold that names a shared unit in both groups
would differ could not run: the engine refuses it.

## 2. What a night saves

Question 1's counts fell, so this ran, at 26 units of 51, conflicting, each kind held apart. It is the share whose 24 blocks leave
A at 42, 27, 33 and 50, with room on both sides for a night to read.

**Before the pair:** an awake gap of shift 7's length under the drive, or a night at shift 7, plain or the twin, of A's patterns.
The pair then runs 6, 12 and 24 blocks from what the gap left. A's patterns alone, since the pair has learned nothing to tag yet.
The row with no gap is question 1's.

**Halfway:** 12 blocks of the pair from the start image, then nothing, the same awake gap, or a night at shift 7, plain or the twin,
of A's patterns, the pair's or both, and 12 blocks more, run on from the image the gap wrote. A's patterns are the start image's,
the pair's the image after 12 blocks, each night spending the older episodes. A's patterns take a shared unit to A's answer and
the pair's take it to the other. A night is otherwise 014's: under the drive, the slow-wave baseline at 1/2, the zero sum held.
The first 12 blocks are question 1's 12-block run, its reads trace for trace.

A's count of 64 after 12 and 24 blocks of the pair. A halfway row's 12 is read right after its gap:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| No gap | 44, 42 | 37, 27 | 47, 33 | 54, 50 |
| Awake before | 47, 50 | 38, 27 | 45, 42 | 52, 41 |
| Plain before, A's patterns | 46, 48 | 36, 31 | 51, 51 | 48, 47 |
| Twin before, A's patterns | 36, 39 | 24, 26 | 40, 42 | 46, 47 |
| Nothing halfway | 44, 50 | 37, 25 | 47, 37 | 54, 49 |
| Awake halfway | 45, 42 | 36, 37 | 41, 39 | 48, 48 |
| Plain halfway, A's patterns | 48, 47 | 45, 26 | 52, 49 | 51, 42 |
| Twin halfway, A's patterns | 44, 40 | 29, 20 | 39, 50 | 53, 44 |
| Plain halfway, the pair's | 48, 53 | 36, 25 | 42, 51 | 53, 43 |
| Twin halfway, the pair's | 50, 51 | 42, 25 | 49, 53 | 54, 49 |
| Plain halfway, both | 48, 52 | 36, 29 | 48, 51 | 54, 42 |
| Twin halfway, both | 49, 51 | 38, 26 | 48, 50 | 52, 47 |

A's margin in millions at the same reads:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| No gap | 1.24, 1.06 | 0.36, 0.28 | 0.49, 0.29 | 1.62, 1.33 |
| Awake before | 1.24, 1.19 | 0.27, 0.18 | 0.57, 0.41 | 1.56, 1.28 |
| Plain before, A's patterns | 2.22, 2.10 | 1.52, 1.46 | 1.63, 1.31 | 2.40, 2.19 |
| Twin before, A's patterns | 0.59, 0.48 | −0.43, −0.49 | −0.69, −0.85 | 0.68, 0.46 |
| Nothing halfway | 1.24, 1.18 | 0.36, 0.22 | 0.49, 0.35 | 1.62, 1.34 |
| Awake halfway | 1.24, 1.19 | 0.36, 0.19 | 0.49, 0.34 | 1.62, 1.30 |
| Plain halfway, A's patterns | 2.45, 2.33 | 1.65, 1.32 | 1.75, 1.48 | 2.69, 2.43 |
| Twin halfway, A's patterns | 0.74, 0.69 | −0.37, −0.55 | −0.46, −0.65 | 0.78, 0.54 |
| Plain halfway, the pair's | 0.88, 0.73 | −0.31, −0.48 | 0.10, −0.06 | 1.23, 0.94 |
| Twin halfway, the pair's | 1.76, 1.40 | 0.61, 0.26 | 0.93, 0.67 | 2.25, 1.89 |
| Plain halfway, both | 1.71, 1.60 | 0.64, 0.48 | 0.96, 0.69 | 1.91, 1.60 |
| Twin halfway, both | 1.27, 1.10 | 0.10, −0.08 | 0.33, 0.10 | 1.58, 1.23 |

- **The rows with no night spread by themselves.** At block 24 the four read 42 to 50, 25 to 37, 33 to 42 and 41 to 50, their
  margins within 0.13 M of each other in each start. On this slope a single count moves by 8 to 12 with the margin standing still.
  015's three rows with no night spread by up to 23 at their steepest block.
- **A plain night of A's patterns before the pair adds margin that lasts.** Against no gap at block 24: +1.04, +1.19, +1.02, +0.87 M,
  of which the unshared units keep +0.34 to +0.67 and the shared +0.50 to +0.53. The twin of A's patterns leaves −0.58 to −1.14 M.
- **The same night halfway does likewise.** Against the awake row it adds 1.07 to 1.29 M at once and stands 1.13 to 1.14 M up twelve
  blocks on. The twin of A's patterns takes 0.50 to 0.95 M, and 0.50 to 0.99 M stays taken.
- **The pair's patterns move it the other way.** Against the same row plain takes 0.36 to 0.68 M at once and the twin adds 0.25 to
  0.63 M.
- **The count does not carry it.** Of the 32 night cells at block 24, 18 lie inside their start's four rows with no night, 11
  above and 3 below. Each cell's change in count and in margin, from the mean of those four rows, agree in sign in 18 of the 32.
- **Right after the night the count stays within a read's spread.** The two rows with no night at block 12 hold the same margin and
  read 44 / 45, 37 / 36, 47 / 41, 54 / 48. Against the higher of each, plain on A's patterns reads +3, +8, +5, −3, the twin on the
  pair's +5, +5, +2, 0, the twin on A's −1, −8, −8, −1, and the other three −5 to +4. None passes 015's 8.
- **Twelve blocks on there is no steady sign.** Against the awake row at block 24, plain on A's patterns reads +5, −11, +10, −6 with
  its margin up in all four, and the twin on the pair's +9, −12, +14, +1.
- **One start stands apart.** From a first mapping and the assignment seven of the eight nights read 49 to 53 at block 24, their
  margins from −0.65 to +1.48 M, where the rows with no night read 33 to 42. Those are 7 of the 11 cells above. The nights raise
  the inhibitory sum there from about 134 M to 139 to 142 M, and from about 133 M to 138 to 140 M from a first mapping and the
  mirrored assignment, which stays inside its rows. So the sum alone does not mark it, and what does was not read.
- **The pair stays learned**, reading 60 to 64 at block 24 in every row. No kind's sum moved by more than 344.

**On the shared units a night moves one axis, as on the mirror.** Each night's change at block 12 from the awake row, in millions: A's
margin on the unshared units / on the shared units / the pair's margin:

| Night | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Plain, A's patterns | +0.44 / +0.77 / −0.65 | +0.63 / +0.66 / −0.86 | +0.64 / +0.63 / −0.76 | +0.48 / +0.59 / −0.45 |
| Twin, A's patterns | −0.36 / −0.14 / +0.04 | −0.42 / −0.31 / +0.26 | −0.46 / −0.50 / +0.46 | −0.33 / −0.51 / +0.44 |
| Plain, the pair's | +0.08 / −0.44 / +1.05 | −0.12 / −0.56 / +1.04 | +0.04 / −0.42 / +0.97 | +0.21 / −0.60 / +1.16 |
| Twin, the pair's | +0.12 / +0.40 / −0.57 | −0.11 / +0.35 / −0.73 | +0.04 / +0.41 / −0.71 | +0.28 / +0.36 / −0.53 |
| Plain, both | +0.33 / +0.15 / +0.28 | +0.27 / +0.01 / +0.18 | +0.33 / +0.14 / +0.02 | +0.29 / 0.00 / +0.33 |
| Twin, both | −0.10 / +0.14 / −0.29 | −0.25 / 0.00 / −0.25 | −0.19 / +0.04 / −0.25 | −0.02 / −0.02 / −0.16 |

- **Plain on A's patterns and the twin on the pair's move the shared units the same way**, up by 0.59 to 0.77 and 0.35 to 0.41, and
  the twin on A's and plain on the pair's down by 0.14 to 0.51 and 0.42 to 0.60. 015 read the same of the mirror.
- **A's own patterns also reach the unshared units**, +0.44 to +0.64 plain and −0.33 to −0.46 the twin. The pair's reach them
  by −0.12 to +0.28, the replay crossing as 015 read it, with no steady sign.
- **The pair's margin moves against the shared units'** in all four rows of one pattern set, so such a night trades one mapping's
  margin for the other's there.
- **Both patterns together nearly cancel on the shared units**, 0.00 to +0.15 plain and −0.02 to +0.14 the twin.

## What it costs

The harness's trial took 253 s, question 1's four tests 3 296 to 3 429 s for eleven forms, and question 2's 2 048 to 2 275 s.
That is nine jobs, nothing near a job's 240 minutes, and no share or start was cut.

## What it suggests

**"Kept" has a contrast here.** Under the drive 014's gaps left A at the ceiling, and 015's mirror read a reversal. Here another task
with its own stimuli takes A to 27 to 50 of 64 at 26 units after 24 blocks, with the pair learned, where an agreeing pair of the same
share leaves 63 to 64 and a pair of no share 63 to 64. Of these three notes it is the first reading where a mapping is taken by
another learned mapping that is not its mirror.

**What is taken stays where it was asked.** Under either cut the unshared units' margin moves by at most 92 888, so what shows here
shows on the shared units.

**A night here read as it read on the mirror.** Twelve blocks on, a plain night of A's patterns stands about 1.1 M up and the count
does not follow it. A test of the margin and a test of the count would again disagree on what a night kept. And four rows with no
night spread by up to 12 of 64 at one margin, so a difference a night makes on this slope needs more than a single block's
read to be seen at all.

**A reading, not measured.** A falls as the shared units' summed margin crosses zero with the unshared units' standing, which reads as
part of A's sources voting the other way. Only sums by kind were read, not a unit's weights or its part in a trial's count, and from a
first mapping and the assignment 13 units end at −0.03 M with A 16 down. The vote is a picture and no more.

Questions this suggests for a later exploration or a round:
- a read of A's count over many blocks of one image, so that a count on this slope has a spread of its own;
- where the count lives: the weights by unit across a night, since a night's change in margin does not carry its change in count;
- why the margin lies so unevenly over A's units, and why an unshared unit that fires does not move;
- why a first mapping from the assignment reads higher after seven of its eight nights;
- nights at other doses here, several through the pair, and the same on seeds 23 and 24.

## What did not run

- **Seeds 23 and 24.**
- **The default as a control.** Every run here is under H-46's rules.
- **A hold naming a shared unit in two groups.** The engine refuses it, and this branch left that as it is.
- **Question 2 at any share but 26 of 51**, under Kept, for an agreeing pair, or with a shift but 7.
- **Two nights in one form**, or a night after the 24 blocks.
- **The pair's patterns before it has learned.**
- **A return to A**: no form read what rewarded blocks of A do after the pair.
- **The weights' distribution**, so why the count leaves the margin behind stays unread, as in 015.
- **Whether a local synapse joins the 13, 25 or 12 pairs of C and D units within reach.**
- **A second read of an image past the one awake gap of section 2.** Every count here is one block.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_what_a_shared_source_takes_explore` | [41309320406](https://github.com/Menteion/Menteion/actions/runs/41309320406) | `d1fe35d` | 253 |
| `what_a_shared_source_takes_of_the_eighth_image_on_seed_22_from_the_assignment_explore` | [41310255988](https://github.com/Menteion/Menteion/actions/runs/41310255988) | `d1fe35d` | 3296 |
| `what_a_shared_source_takes_of_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [41310255988](https://github.com/Menteion/Menteion/actions/runs/41310255988) | `d1fe35d` | 3338 |
| `what_a_shared_source_takes_of_a_first_mapping_on_seed_22_from_the_assignment_explore` | [41310255988](https://github.com/Menteion/Menteion/actions/runs/41310255988) | `d1fe35d` | 3429 |
| `what_a_shared_source_takes_of_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [41310255988](https://github.com/Menteion/Menteion/actions/runs/41310255988) | `d1fe35d` | 3422 |
| `what_a_night_saves_of_a_shared_source_of_the_eighth_image_on_seed_22_from_the_assignment_explore` | [41315401902](https://github.com/Menteion/Menteion/actions/runs/41315401902) | `e9ddfe6` | 2111 |
| `what_a_night_saves_of_a_shared_source_of_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [41315401902](https://github.com/Menteion/Menteion/actions/runs/41315401902) | `e9ddfe6` | 2048 |
| `what_a_night_saves_of_a_shared_source_of_a_first_mapping_on_seed_22_from_the_assignment_explore` | [41315401902](https://github.com/Menteion/Menteion/actions/runs/41315401902) | `e9ddfe6` | 2275 |
| `what_a_night_saves_of_a_shared_source_of_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [41315401902](https://github.com/Menteion/Menteion/actions/runs/41315401902) | `e9ddfe6` | 2210 |
