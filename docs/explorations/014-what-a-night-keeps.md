---
status: exploratory
date: 2026-11-01
---

# 014: What a night keeps

**Branch**: `explore/014-what-a-night-keeps`
**Commit**: `ebb0ebe` for the harness's trial; `770a25d` for the nights on a learned mapping; `8997ecc` for the nights at
the flips and the empty nights; `26527a1` for the twin's nights at the flips. The last two add tests and nothing else
that runs. All carry `eacc1fd`, `main` at ADR-0231, and from `770a25d` on all three of this exploration's changes under
`src/`, each unset the engine it was.
**Runs**: [41243125439](https://github.com/Menteion/Menteion/actions/runs/41243125439) (the harness's trial),
[41244196338](https://github.com/Menteion/Menteion/actions/runs/41244196338) (the nights on a learned mapping),
[41250066678](https://github.com/Menteion/Menteion/actions/runs/41250066678) (the nights at the flips and the empty
nights) and [41256871759](https://github.com/Menteion/Menteion/actions/runs/41256871759) (the twin's nights at the
flips), all at `scope=arms`

[ADR-0232](../adr/0232-a-running-configuration-and-m12-closed.md) closed M12 and opened M11, which asks whether a learned
mapping survives a sleep with replay better than one without. No run of the task had slept:
`runtime/menteion-runtime/src/task.rs` and `tests/inhibition.rs` tagged no episode and read no sleep stage, and the
harness's `config` set `sleep_shift` to 0, so there was no baseline. This note asks, on seed 22, at 1 024 units, on H-25's
two arms under H-46's rules:

1. a night without sleep: how much of a learned mapping stays after a gap as long as a night, under the drive and
   without it, and how far the four couplings move; and, if the mapping does not fade by itself, whether M11's exit test
   can read "kept" at all;
2. a night with replay: the same retention, the replays, the couplings, the gain and the count against the target, the
   inhibitory sum, and whether the zero sum acts through the night;
3. a night between two mappings: H-20's schedule returns to the old mapping at its reversals, so whether a night at each
   flip saves blocks on the way back;
4. F-47's twin: the replay's drive over the gain, read beside question 2;
5. a control: the same under the default, both rules unset.

It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the
repository's AArch64 host.

**In brief**
- **A mapping does not fade by itself.** Under the drive it stood at 60 to 64 of 64 through 960 windows, a schedule's
  length, with no stimulus and no reward, and from the eighth image all four couplings came out equal bit for bit.
  "Kept", read as the first block's correct, has no contrast there: with a night or without one it reads the ceiling.
- **Without the drive what fades is the gain, not the weights.** The couplings stay equal bit for bit while the gain
  climbs to 1.93, 2.81 and its ceiling of 4.0, and the count falls to 62, 40 and 36 of 64.
- **At H-25's baseline no replay reaches an excitatory synapse.** That baseline is zero and a night has no reward: 5 363
  replays moved no excitatory weight, and moved the inhibitory sum by +37 to +43 %. ADR-0044's night ran at a baseline of
  1.0. To read consolidation at all this branch adds a baseline for slow-wave sleep alone.
- **F-47 sets the sign.** Given that baseline, under H-46's rules, the replay as the engine has it, about three spikes a
  unit, raises the mapping's margin, and its twin over the gain, one or two spikes a unit, lowers it, in both arms, from
  both starts, driven and quiet. The drive arriving as ADR-0038 measured it is the one that moves a mapping's couplings
  against the mapping.
- **The zero sum holds through every night; the default does not.** At the same dose under the drive the plain night
  keeps 64 / 63 under H-46's rules at shift 9, and the default erodes to 35 / 29. Against a night with no episode, the
  replays add 3.5 M to the margin under the zero sum and take 1.1 to 1.4 M from it under the default.
- **No night saved blocks on the way back to an old mapping.** Consolidating toward the mapping left the reversals where
  they were or slowed them, and replaying the older mapping too did nothing for the return to it.
- **A night that weakens the mapping made the next reversal cheaper, in proportion to its dose.** The consolidated twin
  took the three reversals summed from 36 to 29, 23 and 13 blocks at shifts 5, 7 and 9 from the assignment on the first
  schedule, and from 16 to 13, 12 and 9 on the ninth, the mapping still at 64 / 63 after shift 7 and at 47 / 36 after
  shift 9. The first flip, which returns to nothing, fell as well. That reads as a weakening of the old mapping, not as
  savings.

## What ran

**The network and the rules.** Every test starts from H-42's network of seed 22 at 1 024 units and first runs its
derivation again, held to H-42's pinned tables (`probe_made`); every one held. H-46's arm is `zero_from`'s: an exponent of
1/4 at the decode, the host setting the zero sum's groups at each load with the values of H-42's image. The default is
the same harness with both unset. Both run under H-25's configuration, from the assignment and from the mirrored
assignment.

**Two starts.** A night needs a mapping already learned, and a mapping learned once is far from one learned many times:
- **The eighth image.** The tree holds H-46's eighth images, read from `zero_image_path` and held to their CRC-64s. The
  mapping in force is the last of the eighth schedule, the reversals stand near their floor of about five blocks, and no
  schedule has to run first. It is the cheaper start and the main one.
- **A first mapping.** The first span of a schedule from H-42's image, 1 536 trials under the form's rules, then quiet.
  It costs a quarter of a schedule and shows a mapping learned once, the next reversal at 10 to 15 blocks.

**A gap** stands between two engines. The image is written, the gap's engine is decoded from it and runs, and its image
goes to the reading, so every form of a start begins from the same bytes. The reading is taken on two forks of that
image: one block of 64 trials with the reward withheld, and four blocks rewarded. "Correct" is the first of these.

**A night.** The homeostasis record is patched as [ADR-0044](../adr/0044-reference-network.md)'s night starts:
`sleep_shift` set, the stage slow-wave sleep, the pressure at 1.0. The engine runs its stages to the wake, under the drive
or without it, then quiet; its shift goes back to zero and the image is written. Every window dumps the state.

| Shift | Ticks | In blocks | Replays | Depotentiations |
| ---: | ---: | ---: | ---: | ---: |
| 5 | 1 022 558 | 1.0 | 371 | 128 |
| 7 | 4 037 214 | 3.9 | 1 331 | 640 |
| 9 | 16 489 054 | 15.7 | 5 363 | 2 688 |

From the eighth image; from a first mapping the ripples' phase gives ten more replays. Shift 5 is ADR-0044's.

**The episodes.** `PATTERN_MAX` is 12. A mapping's 102 episodes are one for each excitatory unit of a stimulus set: that
unit first, then the excitatory units of its answer's readout set it reaches, the strongest summed weight first, up to
eleven; 854 units from the assignment and 920 from the mirrored assignment. They are tagged at a priority of 200, more
REM ripples than a night gives any of them. [ADR-0048](../adr/0048-episodes-tagged-from-the-train.md)'s own rule ran
beside them: `capture` over a block of the mapping gives 64 episodes, whose 768 units lie 730 in the stimulus set, 34 in
the answer's readout, 3 in the other readout and 1 in the other stimulus, 22 of them inhibitory. What the rule captures
is the stimulus, not the mapping.

**Under `src/`, on this branch alone.** Commits `ebb0ebe` and `770a25d`. None is in an image and each, unset, is the run
it was:
- **`Config::replay_after_gain`, the twin of F-47.** Each of a ripple's messages is `REPLAY_DRIVE_Q16` over the tick's
  gain, so the drive arrives at the 2.5 [ADR-0038](../adr/0038-episodic-ledger-and-replay.md) measured, not at 4.375 under
  a gain of 1.75.
- **`Config::slow_wave_baseline_q16`, which the questions did not ask for.** H-25's configuration sets the excitatory
  baseline to zero, so an excitatory synapse consolidates only under the reward's gate, and a night has no reward. At the
  record's baseline a replay cannot move an excitatory weight (section 2). This field gives a tick in slow-wave sleep a
  baseline of its own; every other stage keeps the record's. "Half" and "whole" below are 1/2 and 1, ADR-0044's.
- **`Executor::spend_episodes`**, so a night at a flip replays the mapping just in force and not the older ones.

**The flips** (section 3). One schedule of H-20, each of its four mappings on an engine of its own from trial 0, a gap at
each of the three flips. Each night tags the mapping just in force and runs under the drive, with the zero sum where the
form holds it; the awake engines carry neither the twin nor the slow-wave baseline. Four engines are not H-46's one: the
kept addresses and the task's trial count start again at each mapping, so the same harness with no gap can differ from
H-46's row. Every comparison here is against the harness with no gap. The first mapping's trace came out one value over
every gap of a start, in both runs.

## 1. A night without sleep

From each image, the ticks of the three nights awake with no stimulus, and under the drive on through 960 windows, a
schedule's length. Correct of 64, the reward withheld, assignment / mirrored:

| Start, rules | No gap | Under the drive, three lengths | Under the drive, 120, 240, 480, 960 windows | Without the drive, three lengths |
| :--- | ---: | :--- | :--- | :--- |
| Eighth, H-46's | 64 / 63 | 63 / 62, 64 / 64, 63 / 63 | 64 / 64, 64 / 64, 63 / 64, 63 / 63 | 62 / 62, 40 / 33, 36 / 27 |
| Eighth, default | 64 / 63 | 62 / 63, 64 / 62, 64 / 63 | 64 / 62, 63 / 63, 64 / 64, 63 / 64 | 61 / 62, 42 / 38, 39 / 28 |
| First mapping, H-46's | 62 / 62 | 63 / 60, 62 / 62, 63 / 63 | 64 / 62, 61 / 60, 63 / 60, 62 / 61 | 55 / 58, 33 / 39, 29 / 34 |

- **Under the drive a mapping does not fade.** It stood at 60 to 64 of 64 through a schedule's length, in every start,
  form and arm.
- **The couplings do not move at all.** From the eighth image all four couplings and the excitatory arena's sum are
  equal bit for bit before and after every gap, driven or not. From a first mapping a residue arrives once, the same at
  every length: (−917, +808, +47, +247) from the assignment and (−41, +8, +1, −1) from the mirrored assignment, the arena
  by −5 783 and +894. An excitatory synapse moves only under the reward's gate, and a gap has none.
- **The inhibitory sum moves little under the drive**: +1.20, +0.35, +0.03, −1.12 % a reading at the eighth image, and
  −3.33, −3.73, −7.20, −12.63 % at a first mapping, the fall a long run's and not a night's. The gain stands 0.03 to 0.05
  above where it began.
- **Without the drive what fades is the gain.** Nothing fires, so from 1.698 the gain climbs to 1.93, 2.81 and 4.0, its
  ceiling, at the three lengths. The couplings are equal bit for bit and the inhibitory sum moves by 0.00 %, and the
  count of 62, 40, 36 reads a network firing too much: the stimulus's spikes a block rise from about 1 300 to 6 000 to
  8 000 and 13 000 to 17 000.
- **Rewarded blocks at that gain do lasting harm.** From the eighth image at the middle length, four blocks rewarded read
  35, 33, 33, 36 and leave the couplings at (12 187 236, 283 071; 12 114 166, 808 359), one answer for either stimulus;
  from the mirrored assignment 23, 9, 0, 1. After a quiet night of that length the same blocks read 53, 61, 64, 64 with
  the couplings kept.

**So "kept", read as the first block's correct, has no contrast under the drive**: both sides of any comparison stand at
the ceiling. **Without the drive it reads the gain and not the mapping's weights.**

## 2. A night with replay

Under H-46's rules, from the eighth image, at the record's baseline, assignment / mirrored:

| Shift | Form | Correct after | Gain, from 1.698 / 1.696 | Inhibitory sum | Couplings |
| ---: | :--- | ---: | ---: | ---: | :--- |
| 5 | driven | 64 / 63 | 1.637 / 1.630 | +3.96 / +4.44 % | unmoved |
| 7 | driven | 64 / 64 | 1.632 / 1.620 | +11.54 / +13.53 % | unmoved |
| 9 | driven | 64 / 64 | 1.646 / 1.635 | +36.57 / +42.90 % | unmoved |
| 5 | quiet | 63 / 63 | 1.820 / 1.814 | +0.35 / +0.54 % | unmoved |
| 7 | quiet | 55 / 50 | 2.172 / 2.133 | +2.29 / +3.54 % | unmoved |
| 9 | quiet | 35 / 38 | 3.244 / 3.078 | +31.03 / +34.54 % | unmoved |

The awake gaps of the same lengths read 63 / 62, 64 / 64, 63 / 63 under the drive, the inhibitory sum at +0.01, +0.43,
+1.16 %, and 62 / 62, 40 / 33, 36 / 27 without it, the gain at 1.93, 2.81, 4.0.

- **At H-25's baseline no excitatory weight moves.** Couplings, arena and every pattern's sums are equal bit for bit over
  5 363 replays. What a replay moves is the inhibitory rule, whose baseline is a half: under the drive a window with
  replays fired 2 858 to 3 975 spikes and one without 499 to 1 001, the target 2 330, and the sum rose by more than a
  third over the longest night.
- **The replays' windows pull the gain down and REM's let it rise**, so under the drive it ends below where it began, and
  the stimulus's spikes a block after the night fall by 3 to 14 %, the mapping unchanged.
- **A quiet night beats a quiet gap at one length, by the gain.** At shift 7, 55 / 50 against 40 / 33, the gain at 2.17 /
  2.13 against 2.81: the replays' spikes slow its rise. No weight differs, and at shift 9 they read 35 / 38 and 36 / 27.
- **A replay fires its pattern about three times a ripple at a gain of 1.7**: of the patterns' units over the night at
  shift 5, 5 fired none, 60 twice, 2 718 three times and 318 more, the first spike at a mean of 3.0 ticks and the second
  at 204.
- **Captured episodes moved as little.** Quiet at a half, at shifts 5 and 7, the couplings moved by at most 119 651 with
  no steady sign, and the count read 63 / 63 and 60 / 56.

**With the slow-wave baseline** the same replays reach the weights. The change in the margin, the two couplings of the
mapping in force less the two of the other, at shifts 5, 7, 9; the eighth image's margin is 3 564 337 from the assignment
and 3 311 218 from the mirrored assignment:

| Form | Margin change, assignment | Margin change, mirrored | Correct after |
| :--- | ---: | ---: | :--- |
| half, driven | +330 757, +1 016 113, +2 472 942 | +282 060, +1 003 236, +2 329 428 | 64 / 63, 64 / 64, 64 / 63 |
| whole, driven | +581 056, +1 717 500, +4 158 296 | +525 231, +1 728 163, +4 102 447 | 64 / 63, 64 / 64, 64 / 63 |
| half, quiet | +556 143, +3 400 215, +13 249 455 | +468 236, +3 638 195, +13 062 562 | 63 / 64, 57 / 61, 45 / 44 |
| whole, quiet | +833 604, +4 952 444, +15 091 994 | +689 892, +5 377 926, +14 333 590 | 63 / 64, 60 / 61, 45 / 45 |
| half, driven, no episode | +8 845, −118 957, −980 321 | −50 810, −251 342, −1 212 687 | 63 / 64, 64 / 62, 60 / 56 |

- **The last row is the drive alone.** At the record's baseline a night with no episode reads as an awake gap does, no
  excitatory weight moved. At a half the drive's own pairings consolidate, and by shift 9 they take about a million from
  the margin and the count falls to 60 / 56, where the night with the replays holds 64 / 63. The replays' part at shift 9
  is +3.45 M and +3.54 M.
- **Quiet, the margin rises furthest and the count falls.** The gain still climbs without the drive, to about 3 at
  shift 9.
- **Under the drive the arena falls**, by 22.6 M at a half at shift 9 from the assignment, where with no episode it falls
  by 0.7 M and quiet it rises by 26.4 M. What the drive and the replays do together was not taken apart.
- **The zero sum acts all night.** Each group's sum moved by at most 618 at any window's end, in every form that holds it,
  and ended within 282 of its start; unset, it ended 35 477 to 9 770 295 away. At the record's baseline it has nothing to
  hold.
- **Unset, replay adds to the right couplings** and does not take from the wrong: quiet at a half at shift 9,
  (−969 052, +8 930 100, +9 491 436, −686 729), the count about the same. Under the zero sum the same dose is
  (−3 273 670, +3 273 685, +3 351 047, −3 351 053), each coupling's gain its pair's loss.
- **The same from a first mapping**: at a half or whole, driven, the margin rises by 232 830 to 5 145 220 and the count
  stands at 61 to 64.

## 3. A night between two mappings

Blocks to the crossing at each of the three flips, and the three summed. The nights run under the drive at shift 7 where
no shift is named. The last column is the margin after the night over the margin before it, at each flip:

| Gap, H-46's rules | First, assignment | First, mirrored | Ninth, assignment | Ninth, mirrored | Margin kept |
| :--- | ---: | ---: | ---: | ---: | ---: |
| H-46's row | 10, 12, 10 = 32 | 13, 11, 11 = 35 | 5, 6, 5 = 16 | 5, 5, 5 = 15 | |
| No gap | 11, 14, 11 = 36 | 10, 10, 16 = 36 | 5, 6, 5 = 16 | 6, 5, 6 = 17 | 1.00 |
| Awake, under the drive | 12, 12, 12 = 36 | 10, 11, 13 = 34 | 6, 6, 5 = 17 | 5, 6, 6 = 17 | 1.00 |
| Night, the record's baseline | 11, 11, 12 = 34 | 11, 11, 13 = 35 | 6, 5, 5 = 16 | 5, 5, 6 = 16 | 1.00 |
| Twin, the record's baseline | 11, 12, 11 = 34 | 10, 11, 13 = 34 | 6, 5, 5 = 16 | 5, 5, 6 = 16 | 1.00 |
| Half | 11, 12, 12 = 35 | 13, 11, 12 = 36 | 5, 5, 5 = 15 | 5, 6, 6 = 17 | 1.30 to 2.26 |
| Half, older mappings kept | 11, 12, 11 = 34 | 13, 9, 14 = 36 | 5, 5, 5 = 15 | 5, 5, 7 = 17 | 1.05 to 1.49 |
| Whole, shift 9 | 15, 19, 12 = 46 | 17, 14, 13 = 44 | 6, 10, 8 = 24 | 7, 8, 11 = 26 | 2.6 to 3.4 at the first flip |
| Half twin, shift 5 | 9, 10, 10 = 29 | 12, 9, 9 = 30 | 5, 4, 4 = 13 | 5, 5, 4 = 14 | 0.85 to 0.96 |
| Half twin | 9, 8, 6 = 23 | 10, 7, 7 = 24 | 4, 4, 4 = 12 | 4, 3, 4 = 11 | 0.51 to 0.84 |
| Half twin, older mappings kept | 9, 12, 8 = 29 | 10, 9, 7 = 26 | 4, 4, 3 = 11 | 4, 4, 4 = 12 | 0.51 to 1.09 |
| Half twin, shift 9 | 4, 5, 4 = 13 | 5, 3, 3 = 11 | 3, 3, 3 = 9 | 2, 3, 2 = 7 | −0.87 to 0.45 |

| Gap, default | First, assignment | First, mirrored | Ninth, assignment | Ninth, mirrored | Margin kept |
| :--- | ---: | ---: | ---: | ---: | ---: |
| H-46's row | 11, 12, 11 = 34 | 14, 13, 13 = 40 | 5, 5, 5 = 15 | 5, 5, 6 = 16 | |
| No gap | 13, 14, 12 = 39 | 15, 12, 16 = 43 | 5, 5, 5 = 15 | 6, 5, 6 = 17 | 1.00 |
| Night, the record's baseline | 11, 12, 12 = 35 | 15, 14, 16 = 45 | 5, 5, 6 = 16 | 5, 6, 6 = 17 | 1.00 |
| Half | 11, 16, 12 = 39 | 13, 13, 19 = 45 | 4, 7, 6 = 17 | 4, 6, 8 = 18 | 0.51 to 1.23 |
| Half twin | 7, 11, 9 = 27 | 10, 9, 11 = 30 | 3, 4, 4 = 11 | 3, 4, 5 = 12 | −0.08 to 0.75 |

- **The band.** An awake gap against no gap moved one flip by at most three blocks and the sum by at most two. Every
  mapping of every schedule was learned, at 115 to 128 of its last 128. H-25's four clauses held in every schedule under
  H-46's rules but the two from the assignment with a whole night at shift 9, which fail the bound's clause; the default's
  ninth schedule fails that clause with no gap too.
- **A night at the record's baseline is inside the band**, plain or twin, as no excitatory weight moved. So is the inhibitory
  sum's rise: it stood 9 to 14 % up after each such night from the eighth image, and the reversals did not move.
- **Consolidating toward the mapping did not slow a reversal at a half.** The margin rose by 30 % or more and the sum
  stayed at 35, 36, 15, 17. A whole night at shift 9 slowed it by 8 to 10 blocks.
- **No savings.** Three readings agree. Half with the older mappings kept in the ledger, replayed beside the one just in
  force, read 34, 36, 15, 17 against half's 35, 36, 15, 17, and under the twin 29, 26, 11, 12 against 23, 24, 12, 11, so
  replaying the old mapping did nothing for the return to it. The return flips after a night that strengthens a mapping
  stayed inside the band of no gap's. And the first flip of the first schedule returns to nothing, the mapping new, and fell
  as far as the returns at the largest dose: 4 and 5 blocks against 11 and 10.
- **The twin's night lifts the next mapping's start, by its dose.** Mean correct in the first block after a flip, no gap / shift 5 /
  shift 7 / shift 9: 2.2, 1.2, 2.3, 13.7 on the first schedule and 0.7, 0.3, 3.0, 22.0 on the ninth; in the second block
  2.8, 3.5, 5.8, 25.2 and 2.8, 4.8, 12.3, 37.7. At shift 9 the night takes the margin past zero at five of the twelve flips,
  and section 4 reads the mapping at 47 / 36 after it. At shift 7 the mapping reads 64 / 63 after the same night, and the
  reversals are a quarter to a third shorter.
- **It needs the consolidation.** At the record's baseline the twin reads 34, 34, 16, 16.
- **The default follows the twin too**, and the plain night does not help it. Plain at a half lowers the default's margin
  on the ninth schedule to 0.51 to 0.84 and its later flips slow, 4, 7, 6 and 4, 6, 8 against 5, 5, 5 and 6, 5, 6. The twin
  lowers it to −0.08 to 0.51 and they fall, 3, 4, 4 and 3, 4, 5.
- **The summed margin does not carry it all.** Twin with the older mappings kept left the margin at 0.90 to 0.95 at the
  ninth schedule's second flip, the summed margin about as no gap's, and that flip took 4 blocks against 6 and 5. After a whole
  night at shift 9 the second mapping ended with its margin at −1.8 to −2.1 M, the couplings against it, and 123 to 128 of
  its last 128 correct. Where else the mapping lives was not read.

## 4. F-47's twin

Beside section 2, under H-46's rules, from the eighth image, the assignment:

| Form | Pattern's units firing none, once, twice, three times, more, at shift 5 | First spike, mean ticks | Second |
| :--- | :--- | ---: | ---: |
| Plain, driven | 5, 0, 60, 2 718, 318 | 3.0 | 204 |
| Twin, driven | 10, 157, 2 448, 459, 27 | 5.7 | 216 |
| Plain, quiet | 0, 0, 2 031, 1 052, 18 | 5.0 | 206 |
| Twin, quiet | 0, 2 798, 291, 12, 0 | 12.9 | 274 |

- **The twin fires one spike fewer**: about two a ripple under the drive and one without it.
- **At the record's baseline it reads as the plain night does, a little less.** Correct 64 / 62, 64 / 64, 64 / 64 driven;
  the gain falls to 1.662, 1.667, 1.680 against 1.637, 1.632, 1.646; the inhibitory sum +3.19, +9.73, +31.58 % against
  +3.96, +11.54, +36.57 %.
- **Consolidated, its sign reverses under H-46's rules**, in both arms, from both starts, driven and quiet. All three of a
  pattern's sums fall where the plain night's rise, and the margin falls where the plain night's rises:

| Form | Margin change, assignment | Margin change, mirrored | Correct after |
| :--- | ---: | ---: | :--- |
| half twin, driven | −235 114, −907 510, −3 421 559 | −311 298, −1 015 993, −4 084 708 | 64 / 64, 64 / 63, 47 / 36 |
| half twin, quiet | −140 048, −740 201, −3 363 892 | −154 727, −802 436, −4 589 431 | 64 / 61, 47 / 41, 35 / 27 |

  From a first mapping, driven, the margin falls by 224 269 to 4 078 390 and the count reads 64 / 62, 62 / 61, 45 / 51. Under
  the default the plain night's margin falls under the drive too, and the twin's further, by 934 415 to 7 025 195; it
  reads 63 / 62, 57 / 49, 14 / 1, the last with the couplings moved by (−3 531 492, −264 348, −211 049, −3 969 100), the
  mapping inverted. Quiet, the default's sign reverses as the zero sum's does.
- **Why the sign reverses was not measured.** The spike counts differ and the latencies with them, so a replay's pairings
  differ, and the logs hold no pairing.

## 5. The default as a control

It ran every form of section 2 but the zero sum unset, the empty nights, and the flips of section 3. At the eighth image
its couplings stand at (8 224 643, 10 714 524, 10 376 020, 8 564 832), against the zero sum's (5 254 005, 7 216 299,
7 262 287, 5 660 244), about half as high again.

- **At the record's baseline it is the same null**: no excitatory weight moves, and driven the mapping stands at 62 to 64.
- **Consolidated under the drive the mapping erodes**, where under the zero sum the same dose holds it. Correct at shifts
  5, 7, 9:

| Form | H-46's rules | Default |
| :--- | :--- | :--- |
| half, driven | 64 / 63, 64 / 64, 64 / 63 | 63 / 62, 57 / 55, 35 / 29 |
| whole, driven | 64 / 63, 64 / 64, 64 / 63 | 63 / 63, 56 / 51, 43 / 28 |
| half, driven, no episode | 63 / 64, 64 / 62, 60 / 56 | 63 / 64, 64 / 61, 62 / 54 |

- **All four couplings fall**, the arena by 6.4, 16.1 and 34.7 M at a half, and the margin by 495 467, 1 515 237 and
  2 410 427 from the assignment. With no episode the margin falls by 1 039 511 at shift 9 and the count reads 62 / 54, so
  the erosion past that is the replays'.
- **Quiet, the default's margin rises as the zero sum's does**: +214 902, +3 082 271, +13 692 168 at a half from the
  assignment. So what erodes the default is replay under the drive, not replay.

## What it costs

The nights on a learned mapping took 3 269 to 3 932 s a job, the flips 2 578 to 3 516 s for three or four schedules with
their nights, the default's flips 4 686 to 4 875 s for six, and the empty nights about 1 100 s. Nothing neared a job's
240 minutes, and question 5 ran whole. The twin adds a division to each ripple's message and was not timed apart.

## What it suggests

**"Kept" as M11 words it cannot be read by the first block's correct.** On this task nothing takes a mapping away but
another mapping, or a gain left to climb with nothing firing. Three readings here have a contrast, and one that did not
run may:
- **A mapping's count after the other mapping has run a while**, with a night or without. Interference is what takes a
  mapping away here. This did not run, and it is the nearest to what M11 asks.
- **The margin.** It moves with replay while the count stands at the ceiling, so a mapping can be read as held harder. It
  is a quantity and not a behaviour, and section 3 shows it does not carry all of the mapping.
- **The next reversal's blocks.** It moves both ways with the dose, but what it reads is how much of the old mapping a night
  took, not how much it kept.
- **The count against the night's own erosion**: 64 / 63 with the replays against 60 / 56 with none at shift 9. This
  needs the slow-wave baseline and reads a damage that baseline makes.

**Two decisions come before any hypothesis.** H-25's baseline of zero shuts every night off from the excitatory synapses;
ADR-0044's night ran at 1.0, and nothing read the difference because no task slept. And F-47 is no longer a matter of a
drive's size: under a slow-wave baseline and H-46's rules the plain replay strengthens the mapping in force and the twin
weakens it, at every dose read. The decision ADR that settles F-47 settles that sign.

**A reading, not measured.** A night at the twin trades the mapping's margin for the next reversal, and shift 7 is where
the count still stands at the ceiling while the reversals cost a quarter to a third less. It resembles Tononi and Cirelli's synaptic
homeostasis, a depression in sleep that restores the capacity to learn, but only loosely: the arena falls as far under the
plain night, which speeds nothing. A literature's shape is a prior and no more.

**The zero sum and replay fit under the drive; the default and replay do not.** Under the zero sum the replays' effect on
the margin kept its sign with the drive on; under the default the drive reversed it. That is one more reading for the rules
of ADR-0232 being the ones a night should be tried under.

Questions this suggests for a later exploration or a round:
- a replay's pairings: why two spikes and three give opposite signs, which the logs here cannot say;
- the doses between: shifts 6 and 8, a baseline of a quarter, and where the count leaves the ceiling;
- the weights' distribution through a night, as exploration 013 read it across schedules, since the summed margin does not
  carry everything;
- nights at the flips over many schedules: whether the reversals' floor stays moved and the couplings stay bounded;
- interference: a mapping's count after a stretch of the other, with a night and without;
- the same on seeds 23 and 24.

## What did not run

- **Seeds 23 and 24.**
- **Nights over more than one schedule.** Each schedule here is a single one, from an image no night wrote.
- **A mapping's count after the other mapping has run.** The note reads the mapping in force and the next one's learning.
- **Quiet gaps at the flips.** They take the gain to 2.8 or more and rewarded blocks then rewrite the couplings (section
  1), so a flip after one would read that.
- **A drive-alone night at the flips.** Section 2 reads it at 3 to 8 % of the margin at shift 7.
- **Captured episodes** under the drive or at the flips; they ran quiet at a half, at shifts 5 and 7.
- **A whole twin under the drive**, and any baseline but 0, 1/2 and 1.
- **REM's depotentiations apart.** Every night ran them, and none ran without them.
- **The pairings of a replay, and the weights' distribution**: no run dumped them. So the cause of the twin's sign and of
  the reversals the summed margin does not carry stay unread.
- **A night at any point but a flip** of a schedule.
- **UPG-04's chained episodes.** Every episode here is a set of units with no order of its own past its first.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_what_a_night_keeps_explore` | [41243125439](https://github.com/Menteion/Menteion/actions/runs/41243125439) | `ebb0ebe` | 226 |
| `what_a_night_keeps_under_the_zero_sum_on_seed_22_from_the_assignment_explore` | [41244196338](https://github.com/Menteion/Menteion/actions/runs/41244196338) | `770a25d` | 3741 |
| `what_a_night_keeps_under_the_zero_sum_on_seed_22_from_the_mirrored_assignment_explore` | [41244196338](https://github.com/Menteion/Menteion/actions/runs/41244196338) | `770a25d` | 3638 |
| `what_a_night_keeps_under_the_default_on_seed_22_from_the_assignment_explore` | [41244196338](https://github.com/Menteion/Menteion/actions/runs/41244196338) | `770a25d` | 3382 |
| `what_a_night_keeps_under_the_default_on_seed_22_from_the_mirrored_assignment_explore` | [41244196338](https://github.com/Menteion/Menteion/actions/runs/41244196338) | `770a25d` | 3269 |
| `what_a_night_keeps_of_a_first_mapping_on_seed_22_from_the_assignment_explore` | [41244196338](https://github.com/Menteion/Menteion/actions/runs/41244196338) | `770a25d` | 3835 |
| `what_a_night_keeps_of_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [41244196338](https://github.com/Menteion/Menteion/actions/runs/41244196338) | `770a25d` | 3932 |
| `a_night_at_each_flip_of_the_first_schedule_on_seed_22_from_the_assignment_explore` | [41250066678](https://github.com/Menteion/Menteion/actions/runs/41250066678) | `8997ecc` | 3052 |
| `a_night_at_each_flip_of_the_first_schedule_on_seed_22_from_the_mirrored_assignment_explore` | [41250066678](https://github.com/Menteion/Menteion/actions/runs/41250066678) | `8997ecc` | 3023 |
| `a_night_at_each_flip_of_the_ninth_schedule_on_seed_22_from_the_assignment_explore` | [41250066678](https://github.com/Menteion/Menteion/actions/runs/41250066678) | `8997ecc` | 3234 |
| `a_night_at_each_flip_of_the_ninth_schedule_on_seed_22_from_the_mirrored_assignment_explore` | [41250066678](https://github.com/Menteion/Menteion/actions/runs/41250066678) | `8997ecc` | 3209 |
| `further_nights_at_each_flip_of_the_first_schedule_on_seed_22_from_the_assignment_explore` | [41250066678](https://github.com/Menteion/Menteion/actions/runs/41250066678) | `8997ecc` | 2648 |
| `further_nights_at_each_flip_of_the_first_schedule_on_seed_22_from_the_mirrored_assignment_explore` | [41250066678](https://github.com/Menteion/Menteion/actions/runs/41250066678) | `8997ecc` | 2703 |
| `further_nights_at_each_flip_of_the_ninth_schedule_on_seed_22_from_the_assignment_explore` | [41250066678](https://github.com/Menteion/Menteion/actions/runs/41250066678) | `8997ecc` | 2594 |
| `further_nights_at_each_flip_of_the_ninth_schedule_on_seed_22_from_the_mirrored_assignment_explore` | [41250066678](https://github.com/Menteion/Menteion/actions/runs/41250066678) | `8997ecc` | 2578 |
| `a_night_at_each_flip_of_the_default_first_schedule_on_seed_22_explore` | [41250066678](https://github.com/Menteion/Menteion/actions/runs/41250066678) | `8997ecc` | 4875 |
| `a_night_at_each_flip_of_the_default_ninth_schedule_on_seed_22_explore` | [41250066678](https://github.com/Menteion/Menteion/actions/runs/41250066678) | `8997ecc` | 4686 |
| `what_an_empty_night_moves_under_the_zero_sum_on_seed_22_explore` | [41250066678](https://github.com/Menteion/Menteion/actions/runs/41250066678) | `8997ecc` | 1099 |
| `what_an_empty_night_moves_under_the_default_on_seed_22_explore` | [41250066678](https://github.com/Menteion/Menteion/actions/runs/41250066678) | `8997ecc` | 1067 |
| `the_twins_nights_at_each_flip_of_the_first_schedule_on_seed_22_from_the_assignment_explore` | [41256871759](https://github.com/Menteion/Menteion/actions/runs/41256871759) | `26527a1` | 3348 |
| `the_twins_nights_at_each_flip_of_the_first_schedule_on_seed_22_from_the_mirrored_assignment_explore` | [41256871759](https://github.com/Menteion/Menteion/actions/runs/41256871759) | `26527a1` | 3516 |
| `the_twins_nights_at_each_flip_of_the_ninth_schedule_on_seed_22_from_the_assignment_explore` | [41256871759](https://github.com/Menteion/Menteion/actions/runs/41256871759) | `26527a1` | 3407 |
| `the_twins_nights_at_each_flip_of_the_ninth_schedule_on_seed_22_from_the_mirrored_assignment_explore` | [41256871759](https://github.com/Menteion/Menteion/actions/runs/41256871759) | `26527a1` | 3341 |
| `the_twin_at_each_flip_of_the_default_first_schedule_on_seed_22_explore` | [41256871759](https://github.com/Menteion/Menteion/actions/runs/41256871759) | `26527a1` | 1807 |
| `the_twin_at_each_flip_of_the_default_ninth_schedule_on_seed_22_explore` | [41256871759](https://github.com/Menteion/Menteion/actions/runs/41256871759) | `26527a1` | 1797 |
