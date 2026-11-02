---
status: exploratory
date: 2026-11-02
---

# 019: A night without replay

**Branch**: `explore/019-a-night-without-replay`
**Commit**: `0a6024f`, for the harness's trial and the four starts. It merges `a316b0f`, `main` at exploration 018's note, with the tag
`explore/018-what-nights-keep` and what that carries under `src/`, and adds none.
**Runs**: [41408442255](https://github.com/Menteion/Menteion/actions/runs/41408442255) (the harness's trial, and the eighth image from the
assignment) and [41413189972](https://github.com/Menteion/Menteion/actions/runs/41413189972) (the other three starts), both at `scope=arms`.
The rows of exploration 018's four forms are read from its runs [41367828454](https://github.com/Menteion/Menteion/actions/runs/41367828454)
and [41374137568](https://github.com/Menteion/Menteion/actions/runs/41374137568), not run again.

[Exploration 018](018-what-nights-keep.md) set four nights, one every 6 of the pair's first 24 blocks, against four awake gaps on the same
paths. Its plain night on A's patterns stood over the awake row by +3.4, +5.3, +7.5 and +5.9 at block 36, the margin by about 3 M. A night
there is more than its replays: the slow-wave baseline, the sleep's homeostasis, the inhibitory sum each night raised and the gain each
night lowered all come with it. [ADR-0232](../adr/0232-a-running-configuration-and-m12-closed.md) names the control that 014 to 018 did not
run, a night with replay against one without. This note runs that one form and asks, by 018's reading:

1. where A's count and margin stand after four nights with no replay, against the awake row and against 018's three nights;
2. what the pair pays;
3. what such a night does to the couplings, the sums and the gain.

It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the repository's
AArch64 host, on seed 22 alone, on three paths.

**In brief**
- **The night without replay replays nothing, and it wakes on the plain night's tick.** All 48 nights read `replays 0` and
  `depotentiations 0`. Each wakes on the tick its plain night on A's patterns woke on, 48 of 48, and from the same image, at block 6, both
  run the same ticks, 12 of 12.
- **Against the awake row it does not move A one way.** At block 24 +3.6 ± 1.4, −9.4 ± 0.9, +1.6 ± 1.6, +0.9 ± 1.9, and at block 36
  +1.2 ± 0.8, −14.4 ± 1.1, −0.4 ± 1.2, +0.9 ± 1.4. One start stands above 2 SE at block 24 and none at block 36; one stands below at each.
- **From the eighth image and the mirrored assignment it takes A down at every checkpoint**, −2.9, −3.9, −7.0, −9.4, −14.4, to 16.6 of 64 at
  block 36 at a margin of −0.25 M.
- **018's plain night on A's patterns stands over it in all four starts.** +2.8 ± 0.9, +17.6 ± 1.5, +9.1 ± 1.4, +4.7 ± 2.2 at block 24 and
  +2.2 ± 0.6, +19.6 ± 1.1, +7.9 ± 0.5, +5.0 ± 1.2 at block 36, all above 2 SE, the margin +2.71 to +3.67 M at block 24 and +2.64 to +3.75 M
  at block 36. 018's differences from the awake row are, start by start, the sum of this row and the one above, to rounding.
- **Plain on both stands over it above 2 SE in all four starts at both blocks, the twin on the pair's patterns in three.** At block 36 +9.8,
  +14.2, +5.9, +4.7, and +5.6, +0.8, +9.9, +9.2.
- **The pair learns under it.** Against the awake row its count stands −2.5 to +0.2 at block 24 and −1.0 to +0.5 at block 36, one start
  below 2 SE at each, and its margin −0.36 to +0.03 M and −0.03 to +0.06 M, two below 2 SE at block 24 and none at block 36. The plain night on A's
  patterns stands 1.28 to 1.81 M and 1.15 to 1.90 M under it in the pair's margin.
- **A's highest coupling under it is the head's**, 1.112 to 1.144 of H-42's image's, and at block 36's end 1.015 to 1.063, the awake row's 1.036 to
  1.053. The excitatory sum stays at 0.997 to 1.002 of H-42's image's, the awake row's 1.000, where 018's plain night took it to 0.893 to 0.914.
- **It raises the gain.** Each of the 48 raises it, by 1 716 to 2 580, where each of 018's 144 nights lowered it. The inhibitory sum rises
  0.21 to 0.40 M a night on one path from the eighth image and falls 1.21 to 1.48 M from a first mapping, where each of 018's 144 nights
  raised it.

## What ran

**The network, the rules, the starts, the paths and the reads** are 018's, unchanged: H-42's network of seed 22 at 1 024 units, its
derivation run again and held to H-42's pinned tables (`probe_made`), H-46's arm, H-25's configuration, the slow-wave baseline at 1/2 on
this branch alone, the eighth image and a first mapping, each from the assignment and the mirrored assignment. The pair shares 26 of 51
units, conflicting, each kind of unit held apart. Paths 0, 5 and 6. Four gaps, at blocks 6, 12, 18 and 24, then 12 blocks with none to
block 36. Sixteen reads of A and sixteen of the pair at each checkpoint, on forks: the read as it was and fifteen by `Both`.

**Five forms.** The fifth is this note's. The other four are 018's rows from its logs, on the same paths and streams:

| Form | Label | The gap | Its logs |
| :--- | :--- | :--- | :--- |
| Awake | `awake` | 4 037 214 idle ticks, 017's awake gap | 018's |
| No replay | `no-replay` | Plain on A's patterns with no episode tagged | This note's |
| Plain, A's patterns | `plain-first` | A plain night on A's patterns, from the start image | 018's |
| Twin, the pair's patterns | `twin-second` | A twin night on the pair's patterns, from the image the night follows | 018's |
| Plain, both | `plain-both` | A plain night on both sets | 018's |

**The night without replay** is `plain-first`'s night: the same engine with room in its ledger for A's patterns, shift 7, slow-wave
sleep at a pressure of 1.0, the drive, the groups held, slow wave at 1/2, and every older episode spent. Only the tagging is left out, so
the ledger holds no episode that is not spent. Each of the 48 nights' `end` line reads `replays 0` and `depotentiations 0`, and each of their
1 488 `window` lines reads `ripples: 0`, the harness counting a ripple only when an unspent episode stands under the replay hand. A's
patterns' sums are still dumped at the night's `start` and `end`, beside the plain night's. `night_read` takes a `tag` switch for this,
and every other caller passes true.

**How long it runs.** In `menteion-homeostasis`'s `step_sleep` a night ends when the falling pressure meets the wake threshold, and the
ledger is not read there. Each of the 48 nights without replay wakes on the tick its plain night on A's patterns woke on, reading that tick
as the `start` line's `tick` plus the `end` line's `ticks`. At block 6 both start from the same head, and their `start` lines are equal, the
label aside, 12 of 12, with equal ticks. Past block 6 they start from different images, 1 to 1 500 ticks apart, and since both wake on
one tick their ticks differ by exactly that. The harness pads neither night. The awake row's gap is 4 037 214 idle ticks at every block. A night without replay runs 4 034 134 to
4 058 164 ticks, −3 080 to +20 950 from that. After either gap the engine runs to quiet, 2 255 to 2 934 ticks after an awake gap and 2 292
to 2 808 after a night without replay.

| Night at block | Nights | Ticks | Replays | Depotentiations | Gain before | Gain after | Ticks less plain-first's | Equal ticks | Quiet ticks |
| ---: | ---: | :--- | :--- | :--- | :--- | :--- | :--- | ---: | :--- |
| 6 | 12 | 4034134 to 4055074 | 0 | 0 | 111213 to 112456 | 113359 to 114328 | 0 | 12 of 12 | 2292 to 2468 |
| 12 | 12 | 4057731 to 4058164 | 0 | 0 | 111031 to 112089 | 113126 to 113814 | −809 to 190 | 0 of 12 | 2456 to 2808 |
| 18 | 12 | 4056565 to 4057840 | 0 | 0 | 110612 to 111756 | 113095 to 113571 | −1500 to 177 | 0 of 12 | 2306 to 2639 |
| 24 | 12 | 4057444 to 4058031 | 0 | 0 | 110302 to 111474 | 112474 to 113555 | −464 to 123 | 0 of 12 | 2299 to 2580 |

**Reading the logs.** A label is `shared zero22 <arm> <from> share 0b0101 Apart conflict true half nights path <j> every 6 <form> blocks <b>`,
`<arm>` being `AssignmentFirst` or `MirroredFirst`, `<from>` `Eighth` or `FirstMapping`, `<j>` 0, 5 or 6, and `<form>` a label of the table above.
A `read`, `kinds` or `held` line of `blocks <b>` is the image after the gap at `<b>`, and at block 36 the image at that block's end. A read's
label ends in `first` for A or `second` for the pair. A `run` line of `blocks <b>` is the stretch that ends at `<b>`, the first, the head, being `none blocks 6`. A gap's lines end in `gap <b>`.

The head is printed by every job of a start: 24 lines repeat a head, each equal to the first, and the tables take one of each.

Unless a table says otherwise, **a mean of a count** is of `correct` over the 48 read lines of one `<arm>`, `<from>`, `<form>`, `<b>` and
`first` or `second`, three paths by sixteen streams, those whose `spread` is `as-it-was 0` or `Both 1` to `Both 15`. **A's margin** is
`c0 + c3 − c1 − c2` of the `couplings` of `state` in `first`'s stream 0, negated when `read mirrored` is `true`. **The pair's margin** is
018's, from that image's `kinds` line: `c0` and `c1` are rows 1 and 2 summed in columns 2 and 3, `c2` and `c3` rows 4 and 5, counted from 0,
negated when `second`'s `read mirrored` is `true`. A margin's cell is the mean over the three paths.

**A difference** of two forms is, on each path, the first form's mean of sixteen less the second's, then the mean over the three paths. **Its SE**:

- **Against the awake row** it is 018's, `√((the two forms' streams²)/16 + (SD over the paths)²/3)`, each stream the cell's, split as in section 2,
  a negative one as zero. The splits are printed here for the night without replay, and in 018 for A's count at blocks 24 and 36.
- **Between two nights** it is paired, `√((the difference's stream)²/16 + (SD over the paths)²/3)`. The difference's stream comes from the same
  split of the 48 differences, one a path and a stream, a negative one as zero. Two nights stand 0 to 697 ticks apart and share part of a stream,
  where 018's form counts each night's stream whole. Section 2 sets the two side by side.
- **A margin's SE** is its SD over the paths over √3. An SD over three paths has two degrees of freedom.

**Bit for bit.** All of these match:

- `probe_made` held H-42's tables in the trial and the four jobs, each of which ended with exit 0.
- The trial's lines that stand beside 018's: 22 `PIN` lines, the groups' `kinds` line, the heads of paths 0 and 5, and the plain night on A's patterns at
  block 6 on both paths, its 2 `start`, 62 `window` and 2 `end` lines. Then those two images' 2 `kinds` and 2 `held` lines and their 8 reads, A
  and the pair by `as-it-was 0` and `Both 1`.
- The four jobs' 88 `PIN` lines, 4 groups' `kinds` lines and 12 heads against 018's.
- The first start's job against the trial, which ran the night without replay at block 6 on paths 0 and 5 and on path 0 to block 18: 22 `PIN`, 4 `run`,
  3 `start`, 93 `window` and 3 `end` lines, the groups' `kinds` line, 3 images' `kinds` lines, 3 `held` and 12 read lines. Each line counts once:
  the trial ran path 0 to block 6 twice, and the 40 lines of its two copies are equal, the head, the night's 33, and that image's `kinds`, `held`
  and 4 reads. The trial read block 18 with no gap there, so its 6 lines of block 18 differ from the job's, which read after a gap.

**Under `src/`** nothing changes.

## 1. What a night without replay leaves of A

A's mean count of 64, at blocks 6, 12, 18 and 24 right after that gap and then at block 36, each list in that order:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 48.8, 45.4, 43.4, 43.8, 43.0 | 49.2, 34.1, 33.2, 32.0, 30.9 | 51.8, 42.2, 41.2, 37.3, 36.8 | 54.3, 48.9, 46.1, 45.0, 43.2 |
| No replay (`no-replay`) | 50.3, 47.6, 47.2, 47.3, 44.2 | 46.3, 30.2, 26.3, 22.6, 16.6 | 52.4, 44.8, 40.2, 38.9, 36.3 | 56.6, 49.5, 49.0, 45.9, 44.1 |
| Plain, A's patterns (`plain-first`) | 52.3, 50.6, 50.6, 50.2, 46.4 | 53.3, 42.3, 41.8, 40.2, 36.2 | 56.5, 47.6, 46.9, 48.0, 44.3 | 57.7, 52.5, 52.7, 50.6, 49.1 |
| Twin, the pair's patterns (`twin-second`) | 54.5, 56.4, 57.2, 54.5, 49.8 | 50.8, 32.3, 27.4, 22.6, 17.3 | 55.6, 50.0, 50.4, 50.1, 46.2 | 58.9, 55.7, 57.3, 55.8, 53.4 |
| Plain, both (`plain-both`) | 53.1, 54.0, 56.3, 56.2, 54.0 | 51.3, 38.1, 36.6, 34.5, 30.7 | 56.4, 46.1, 45.0, 44.7, 42.3 | 57.5, 53.3, 53.2, 50.5, 48.8 |

A's margin, M, in the same order:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 1.44, 1.25, 1.17, 1.12, 1.06 | 1.15, 0.40, 0.31, 0.27, 0.23 | 1.09, 0.51, 0.41, 0.29, 0.21 | 2.04, 1.54, 1.43, 1.32, 1.22 |
| No replay (`no-replay`) | 1.52, 1.34, 1.30, 1.18, 1.00 | 1.03, 0.28, 0.15, −0.01, −0.25 | 1.11, 0.56, 0.43, 0.39, 0.25 | 2.27, 1.81, 1.83, 1.74, 1.62 |
| Plain, A's patterns (`plain-first`) | 2.66, 3.43, 4.03, 4.51, 4.26 | 2.27, 2.51, 3.14, 3.66, 3.49 | 2.15, 2.38, 3.11, 3.65, 3.46 | 3.24, 3.49, 3.98, 4.45, 4.26 |
| Twin, the pair's patterns (`twin-second`) | 1.95, 2.18, 2.33, 2.44, 1.97 | 1.26, 0.54, 0.53, 0.36, 0.18 | 1.48, 1.19, 1.32, 1.44, 1.22 | 2.82, 2.77, 3.18, 3.55, 3.24 |
| Plain, both (`plain-both`) | 1.90, 2.18, 2.63, 2.77, 2.56 | 1.36, 0.96, 1.29, 1.40, 1.30 | 1.32, 1.05, 1.23, 1.43, 1.34 | 2.45, 2.32, 2.47, 2.64, 2.50 |

Each difference, the first form less the second, path against path, mean over the three paths, in the same order:

| Difference | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| no-replay less awake | +1.5, +2.2, +3.8, +3.6, +1.2 | −2.9, −3.9, −7.0, −9.4, −14.4 | +0.6, +2.5, −1.1, +1.6, −0.4 | +2.3, +0.6, +2.9, +0.9, +0.9 |
| plain-first less no-replay | +1.9, +3.0, +3.4, +2.8, +2.2 | +7.1, +12.1, +15.5, +17.6, +19.6 | +4.1, +2.9, +6.8, +9.1, +7.9 | +1.1, +2.9, +3.7, +4.7, +5.0 |
| plain-both less no-replay | +2.8, +6.4, +9.1, +8.9, +9.8 | +5.0, +7.9, +10.3, +11.9, +14.2 | +4.0, +1.4, +4.8, +5.8, +5.9 | +0.9, +3.7, +4.2, +4.6, +4.7 |
| twin-second less no-replay | +4.1, +8.7, +10.0, +7.1, +5.6 | +4.5, +2.1, +1.1, 0.0, +0.8 | +3.2, +5.2, +10.3, +11.2, +9.9 | +2.2, +6.2, +8.3, +9.9, +9.2 |

And of A's margin, M:

| Difference | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| no-replay less awake | +0.08, +0.09, +0.13, +0.07, −0.06 | −0.12, −0.13, −0.16, −0.28, −0.48 | +0.02, +0.05, +0.03, +0.10, +0.05 | +0.23, +0.27, +0.41, +0.42, +0.40 |
| plain-first less no-replay | +1.14, +2.09, +2.73, +3.32, +3.26 | +1.24, +2.24, +3.00, +3.67, +3.75 | +1.04, +1.83, +2.68, +3.26, +3.21 | +0.97, +1.68, +2.15, +2.71, +2.64 |
| plain-both less no-replay | +0.38, +0.84, +1.33, +1.58, +1.56 | +0.33, +0.68, +1.14, +1.41, +1.56 | +0.21, +0.49, +0.79, +1.04, +1.09 | +0.19, +0.51, +0.64, +0.90, +0.88 |
| twin-second less no-replay | +0.43, +0.84, +1.03, +1.26, +0.97 | +0.22, +0.27, +0.39, +0.38, +0.43 | +0.37, +0.63, +0.89, +1.05, +0.97 | +0.55, +0.95, +1.34, +1.81, +1.62 |

**The readings.** Each cell is a difference ± its SE. The tally counts the starts above, within and below 2 SE.

A's count at block 24:

| Difference | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| no-replay less awake | +3.6 ± 1.4 | −9.4 ± 0.9 | +1.6 ± 1.6 | +0.9 ± 1.9 | −0.8 | 1, 2, 1 |
| plain-first less no-replay | +2.8 ± 0.9 | +17.6 ± 1.5 | +9.1 ± 1.4 | +4.7 ± 2.2 | +8.6 | 4, 0, 0 |
| plain-both less no-replay | +8.9 ± 1.2 | +11.9 ± 1.6 | +5.8 ± 1.0 | +4.6 ± 1.5 | +7.8 | 4, 0, 0 |
| twin-second less no-replay | +7.1 ± 1.0 | 0.0 ± 1.5 | +11.2 ± 1.5 | +9.9 ± 1.2 | +7.1 | 3, 1, 0 |

A's count at block 36:

| Difference | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| no-replay less awake | +1.2 ± 0.8 | −14.4 ± 1.1 | −0.4 ± 1.2 | +0.9 ± 1.4 | −3.2 | 0, 3, 1 |
| plain-first less no-replay | +2.2 ± 0.6 | +19.6 ± 1.1 | +7.9 ± 0.5 | +5.0 ± 1.2 | +8.7 | 4, 0, 0 |
| plain-both less no-replay | +9.8 ± 0.8 | +14.2 ± 1.0 | +5.9 ± 1.5 | +4.7 ± 1.3 | +8.7 | 4, 0, 0 |
| twin-second less no-replay | +5.6 ± 0.4 | +0.8 ± 1.2 | +9.9 ± 1.7 | +9.2 ± 1.4 | +6.4 | 3, 1, 0 |

A's margin at block 24, M:

| Difference | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| no-replay less awake | +0.07 ± 0.02 | −0.28 ± 0.03 | +0.10 ± 0.06 | +0.42 ± 0.06 | +0.08 | 2, 1, 1 |
| plain-first less no-replay | +3.32 ± 0.02 | +3.67 ± 0.05 | +3.26 ± 0.06 | +2.71 ± 0.10 | +3.24 | 4, 0, 0 |
| plain-both less no-replay | +1.58 ± 0.04 | +1.41 ± 0.01 | +1.04 ± 0.02 | +0.90 ± 0.06 | +1.24 | 4, 0, 0 |
| twin-second less no-replay | +1.26 ± 0.02 | +0.38 ± 0.03 | +1.05 ± 0.10 | +1.81 ± 0.07 | +1.12 | 4, 0, 0 |

A's margin at block 36, M:

| Difference | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| no-replay less awake | −0.06 ± 0.03 | −0.48 ± 0.04 | +0.05 ± 0.01 | +0.40 ± 0.03 | −0.02 | 2, 0, 2 |
| plain-first less no-replay | +3.26 ± 0.04 | +3.75 ± 0.04 | +3.21 ± 0.07 | +2.64 ± 0.08 | +3.21 | 4, 0, 0 |
| plain-both less no-replay | +1.56 ± 0.09 | +1.56 ± 0.01 | +1.09 ± 0.04 | +0.88 ± 0.04 | +1.27 | 4, 0, 0 |
| twin-second less no-replay | +0.97 ± 0.04 | +0.43 ± 0.03 | +0.97 ± 0.08 | +1.62 ± 0.02 | +1.00 | 4, 0, 0 |

A's mean at block 24 less at block 36:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 0.8 | 1.1 | 0.5 | 1.8 |
| No replay (`no-replay`) | 3.1 | 6.0 | 2.5 | 1.8 |
| Plain, A's patterns (`plain-first`) | 3.8 | 4.0 | 3.8 | 1.5 |
| Twin, the pair's patterns (`twin-second`) | 4.7 | 5.3 | 3.9 | 2.5 |
| Plain, both (`plain-both`) | 2.2 | 3.7 | 2.4 | 1.7 |

- **Against the awake row a night without replay does not move A one way.** By sign it stands above in three starts at blocks 6, 12 and 24,
  and in two at blocks 18 and 36. By 2 SE it stands 1, 2, 1 at block 24 and 0, 3, 1 at block 36. Its mean over the starts is −0.8 and −3.2.
  A's margin stands +0.07, −0.28, +0.10, +0.42 M against the awake row at block 24 and −0.06, −0.48, +0.05, +0.40 M at block 36.
- **From the eighth image and the mirrored assignment a night without replay takes A down at every checkpoint**, from 46.3 at block 6
  to 16.6 at block 36, the awake row 49.2 to 30.9. Its margin goes from 1.03 to −0.25 M. There the twin on the pair's patterns reads
  +4.5, +2.1, +1.1, 0.0, +0.8 over it. From the other three starts it stands −1.1 to +3.8 over the awake row at every checkpoint.
- **018's plain night on A's patterns stands over the night without replay in all four starts at both blocks.** The smallest is +2.2 ± 0.6.
  Its margin stands +2.71 to +3.67 M over it at block 24 and +2.64 to +3.75 M at block 36. Over the awake row 018 read +3.13 to +3.39 M and
  +3.04 to +3.27 M.
- **018's differences from the awake row are the sum of the two.** At block 36 its +3.4, +5.3, +7.5, +5.9 are +1.2, −14.4, −0.4, +0.9 from the
  night without replay and +2.2, +19.6, +7.9, +5.0 between it and the plain night, start by start, on the same paths and streams, to rounding:
  from the eighth image and the mirrored assignment −14.354 and +19.646 make +5.292.
- **Plain on both and the twin stand over it too.** Plain on both stands +4.6 to +11.9 at block 24 and +4.7 to +14.2 at block 36, above 2 SE in all four
  starts. The twin stands above 2 SE in three starts at both blocks. From the eighth image and the mirrored assignment it reads 0.0 ± 1.5 and +0.8 ± 1.2.
- **A falls from block 24 to block 36 under every form.** It falls 1.8 to 6.0 after nights without replay and 0.5 to 1.8 on the awake row.
- **Under the other SE one tally moves.** The plain night on A's patterns less the night without replay at block 36 reads 3, 1, 0 under 018's form,
  the eighth image from the assignment's +2.2 standing within it, ± 1.1. Under the paired SE the night without replay less the awake row reads as
  above at both blocks.

## 2. The split, and what stands near what

**The night without replay's cells**, split as 017 and 018 split them: a count is the mean, a path, a stream and the rest. With `P` paths and
`R` streams, **a read** is the SD within a path, the variance averaged over the paths. **The rest** is the root of the residual sum of
squares over `(P − 1)(R − 1)`. **A path** is the root of the variance of the paths' means less rest²/`R`, and **a stream** the root of the
variance of the streams' means less rest²/`P`. A negative one prints with a minus, the sign of its variance, and pools as it came.

A's count:

| Start | Block | Mean | Paths' means, 0, 5, 6 | A read | A path | A stream | The rest | Margin, M, mean and SD |
| :--- | ---: | ---: | :--- | ---: | ---: | ---: | ---: | :--- |
| Eighth, assignment | 6 | 50.3 | 50.9, 49.9, 50.1 | 2.93 | 0.45 | 2.62 | 1.31 | 1.52, 0.04 |
| Eighth, mirrored | 6 | 46.3 | 47.1, 45.1, 46.6 | 4.28 | 0.88 | 3.80 | 1.96 | 1.03, 0.05 |
| First mapping, assignment | 6 | 52.4 | 52.3, 53.3, 51.6 | 3.45 | 0.77 | 3.12 | 1.48 | 1.11, 0.08 |
| First mapping, mirrored | 6 | 56.6 | 56.0, 56.9, 57.0 | 2.69 | 0.44 | 2.31 | 1.37 | 2.27, 0.06 |
| Eighth, assignment | 12 | 47.6 | 49.4, 47.1, 46.3 | 3.98 | 1.57 | 3.67 | 1.54 | 1.34, 0.08 |
| Eighth, mirrored | 12 | 30.2 | 28.0, 33.8, 28.8 | 3.49 | 3.03 | 2.11 | 2.77 | 0.28, 0.13 |
| First mapping, assignment | 12 | 44.8 | 43.8, 44.9, 45.6 | 3.88 | 0.79 | 3.54 | 1.58 | 0.56, 0.05 |
| First mapping, mirrored | 12 | 49.5 | 50.0, 51.7, 46.9 | 3.59 | 2.40 | 3.10 | 1.82 | 1.81, 0.14 |
| Eighth, assignment | 18 | 47.2 | 47.9, 47.1, 46.6 | 3.45 | 0.57 | 3.16 | 1.37 | 1.30, 0.04 |
| Eighth, mirrored | 18 | 26.3 | 23.3, 30.4, 25.1 | 2.68 | 3.68 | 1.65 | 2.10 | 0.15, 0.14 |
| First mapping, assignment | 18 | 40.2 | 37.9, 40.8, 41.8 | 3.91 | 2.01 | 3.55 | 1.63 | 0.43, 0.11 |
| First mapping, mirrored | 18 | 49.0 | 49.2, 51.2, 46.6 | 2.99 | 2.28 | 2.49 | 1.66 | 1.83, 0.10 |
| Eighth, assignment | 24 | 47.3 | 48.1, 47.6, 46.4 | 4.25 | 0.78 | 3.98 | 1.49 | 1.18, 0.03 |
| Eighth, mirrored | 24 | 22.6 | 20.6, 25.2, 22.0 | 3.85 | 2.25 | 2.89 | 2.54 | −0.01, 0.09 |
| First mapping, assignment | 24 | 38.9 | 36.8, 39.0, 40.8 | 4.14 | 1.92 | 3.43 | 2.31 | 0.39, 0.11 |
| First mapping, mirrored | 24 | 45.9 | 45.8, 48.0, 43.9 | 3.05 | 1.98 | 2.38 | 1.90 | 1.74, 0.13 |
| Eighth, assignment | 36 | 44.2 | 44.4, 43.6, 44.6 | 3.55 | −0.74 | 0.17 | 3.55 | 1.00, 0.06 |
| Eighth, mirrored | 36 | 16.6 | 14.8, 18.1, 16.8 | 2.93 | 1.59 | 2.20 | 1.94 | −0.25, 0.08 |
| First mapping, assignment | 36 | 36.3 | 34.8, 36.9, 37.3 | 3.80 | 1.18 | 2.49 | 2.87 | 0.25, 0.09 |
| First mapping, mirrored | 36 | 44.1 | 44.3, 46.0, 42.1 | 3.67 | 1.81 | 2.30 | 2.86 | 1.62, 0.09 |

The pair's count:

| Start | Block | Mean | Paths' means, 0, 5, 6 | A read | A path | A stream | The rest | Margin, M, mean and SD |
| :--- | ---: | ---: | :--- | ---: | ---: | ---: | ---: | :--- |
| Eighth, assignment | 6 | 62.1 | 62.2, 61.9, 62.1 | 1.31 | −0.12 | 1.10 | 0.70 | 2.10, 0.10 |
| Eighth, mirrored | 6 | 43.3 | 40.5, 46.3, 43.2 | 4.05 | 2.83 | 3.47 | 2.10 | 0.84, 0.12 |
| First mapping, assignment | 6 | 45.7 | 45.4, 42.6, 49.0 | 4.17 | 3.18 | 3.61 | 2.09 | 0.90, 0.16 |
| First mapping, mirrored | 6 | 53.1 | 54.1, 52.1, 53.2 | 3.02 | 0.93 | 2.65 | 1.44 | 0.98, 0.09 |
| Eighth, assignment | 12 | 61.3 | 61.0, 61.1, 61.8 | 1.77 | 0.33 | 1.50 | 0.93 | 2.27, 0.19 |
| Eighth, mirrored | 12 | 60.5 | 60.7, 59.4, 61.4 | 1.71 | 0.98 | 0.90 | 1.45 | 2.07, 0.14 |
| First mapping, assignment | 12 | 60.9 | 61.8, 60.6, 60.3 | 1.70 | 0.72 | 1.42 | 0.94 | 2.16, 0.13 |
| First mapping, mirrored | 12 | 61.0 | 61.1, 60.3, 61.8 | 1.64 | 0.71 | 1.29 | 1.01 | 1.91, 0.23 |
| Eighth, assignment | 18 | 61.6 | 61.2, 61.4, 62.1 | 1.13 | 0.43 | 0.51 | 1.01 | 2.29, 0.13 |
| Eighth, mirrored | 18 | 60.7 | 61.6, 59.4, 61.2 | 1.62 | 1.13 | 1.10 | 1.19 | 2.05, 0.15 |
| First mapping, assignment | 18 | 61.6 | 62.4, 60.9, 61.4 | 1.31 | 0.69 | 0.39 | 1.25 | 2.27, 0.22 |
| First mapping, mirrored | 18 | 62.6 | 62.7, 62.1, 62.9 | 0.89 | 0.41 | 0.52 | 0.72 | 2.07, 0.16 |
| Eighth, assignment | 24 | 60.9 | 60.6, 60.8, 61.3 | 1.92 | 0.29 | 1.63 | 1.01 | 2.33, 0.06 |
| Eighth, mirrored | 24 | 61.1 | 61.9, 60.4, 61.0 | 1.50 | 0.68 | 0.81 | 1.26 | 2.12, 0.05 |
| First mapping, assignment | 24 | 61.5 | 62.7, 61.0, 60.8 | 1.54 | 1.01 | 0.92 | 1.24 | 2.36, 0.21 |
| First mapping, mirrored | 24 | 62.4 | 62.8, 61.2, 63.1 | 1.00 | 1.01 | 0.66 | 0.75 | 2.18, 0.20 |
| Eighth, assignment | 36 | 62.6 | 62.4, 62.6, 62.8 | 1.24 | −0.16 | 0.85 | 0.89 | 2.62, 0.06 |
| Eighth, mirrored | 36 | 63.0 | 62.9, 63.1, 62.9 | 0.98 | −0.17 | 0.37 | 0.91 | 2.56, 0.09 |
| First mapping, assignment | 36 | 63.3 | 63.4, 63.2, 63.3 | 0.70 | −0.09 | 0.32 | 0.62 | 2.68, 0.16 |
| First mapping, mirrored | 36 | 63.1 | 63.3, 62.5, 63.6 | 0.97 | 0.52 | 0.28 | 0.93 | 2.39, 0.16 |

Pooled, variances averaged, A's count:

| Cells pooled | Cells | A read | A path | A stream | The rest | Negative path, stream |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Every form, every block | 100 | 3.44 | 1.37 | 2.51 | 2.35 | 19, 0 |
| 018's four forms, every block | 80 | 3.41 | 1.25 | 2.41 | 2.41 | 18, 0 |
| No replay, every block | 20 | 3.56 | 1.77 | 2.88 | 2.09 | 1, 0 |
| No replay, block 6 | 4 | 3.39 | 0.66 | 3.02 | 1.55 | 0, 0 |
| No replay, block 12 | 4 | 3.74 | 2.12 | 3.17 | 1.99 | 0, 0 |
| No replay, block 18 | 4 | 3.29 | 2.40 | 2.81 | 1.71 | 0, 0 |
| No replay, block 24 | 4 | 3.85 | 1.82 | 3.23 | 2.10 | 0, 0 |
| No replay, block 36 | 4 | 3.50 | 1.29 | 2.02 | 2.86 | 1, 0 |

The pair's count:

| Cells pooled | Cells | A read | A path | A stream | The rest | Negative path, stream |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Every form, every block | 100 | 1.86 | 1.20 | 1.28 | 1.36 | 18, 10 |
| 018's four forms, every block | 80 | 1.84 | 1.21 | 1.20 | 1.40 | 14, 10 |
| No replay, every block | 20 | 1.95 | 1.14 | 1.54 | 1.19 | 4, 0 |
| No replay, block 6 | 4 | 3.34 | 2.18 | 2.89 | 1.68 | 1, 0 |
| No replay, block 12 | 4 | 1.70 | 0.72 | 1.30 | 1.10 | 0, 0 |
| No replay, block 18 | 4 | 1.27 | 0.73 | 0.69 | 1.06 | 0, 0 |
| No replay, block 24 | 4 | 1.53 | 0.80 | 1.07 | 1.08 | 0, 0 |
| No replay, block 36 | 4 | 0.99 | 0.23 | 0.51 | 0.85 | 3, 0 |

The two SEs of A's count side by side, each the mean over the four starts:

| Difference | Block | 018's SE | Paired SE | The paired over 018's, the two means' ratio |
| :--- | ---: | ---: | ---: | ---: |
| no-replay less awake | 6 | 1.06 | 1.04 | 0.98 |
| no-replay less awake | 12 | 1.44 | 1.55 | 1.07 |
| no-replay less awake | 18 | 1.78 | 1.77 | 0.99 |
| no-replay less awake | 24 | 1.45 | 1.47 | 1.01 |
| no-replay less awake | 36 | 1.12 | 1.15 | 1.03 |
| plain-first less no-replay | 6 | 0.90 | 0.75 | 0.83 |
| plain-first less no-replay | 12 | 1.43 | 1.07 | 0.74 |
| plain-first less no-replay | 18 | 1.66 | 1.51 | 0.91 |
| plain-first less no-replay | 24 | 1.59 | 1.50 | 0.94 |
| plain-first less no-replay | 36 | 1.08 | 0.86 | 0.80 |
| plain-both less no-replay | 6 | 0.99 | 0.73 | 0.74 |
| plain-both less no-replay | 12 | 1.40 | 1.05 | 0.75 |
| plain-both less no-replay | 18 | 1.33 | 1.05 | 0.79 |
| plain-both less no-replay | 24 | 1.43 | 1.29 | 0.90 |
| plain-both less no-replay | 36 | 1.24 | 1.14 | 0.92 |
| twin-second less no-replay | 6 | 1.00 | 0.61 | 0.60 |
| twin-second less no-replay | 12 | 1.26 | 0.93 | 0.74 |
| twin-second less no-replay | 18 | 1.14 | 0.99 | 0.86 |
| twin-second less no-replay | 24 | 1.50 | 1.30 | 0.86 |
| twin-second less no-replay | 36 | 1.26 | 1.17 | 0.93 |

- **The night without replay's path and stream stand above 018's.** A read 3.56, a path 1.77, a stream 2.88 and the rest 2.09 over its 20 cells, against
  3.41, 1.25, 2.41 and 2.41 over 018's four forms' 80. Its path's variance is about twice theirs.
- **Against the awake row the two SEs' means over the starts differ by under 8 %.** Their ratio is 0.98 to 1.07 at the five checkpoints. Between
  two nights it is 0.60 to 0.94 on A's count, and 0.53 to 0.97 on the pair's.

**What stands near what.** The correlation of two images' sixteen counts of A, on one path, over every pair of forms with the night without
replay in it at every checkpoint. The ticks apart are the difference of the two images' `state` ticks. The bands are 018's, 0 to 15, 16 to
127, 128 to 511, 512 to 2 260, 2 261 to 16 928 and 16 929 or more, and each row prints the least and greatest seen in its band:

| Ticks apart, least to greatest seen | Pairs of images | Mean correlation | Least | Greatest |
| :--- | ---: | ---: | ---: | ---: |
| 0 to 13 | 38 | +0.53 | −0.09 | +0.80 |
| 18 to 126 | 77 | +0.41 | −0.27 | +0.80 |
| 130 to 476 | 55 | +0.20 | −0.21 | +0.69 |
| 648 to 697 | 10 | +0.02 | −0.46 | +0.50 |
| 2598 to 3205 | 6 | −0.07 | −0.46 | +0.31 |
| 16982 to 79553 | 54 | −0.06 | −0.57 | +0.64 |

By pair of forms and checkpoint, a start's correlation the mean over its three paths, of A's counts and of the pair's:

| Forms | Block | Ticks apart, least to greatest | A start's correlation of A, mean over the paths, least to greatest | The same of the pair |
| :--- | ---: | :--- | :--- | :--- |
| awake and no-replay | 6 | 2598 to 17769 | −0.24 to +0.13 | +0.07 to +0.21 |
| no-replay and plain-first | 6 | 9 to 696 | −0.04 to +0.48 | −0.03 to +0.39 |
| no-replay and twin-second | 6 | 0 to 9 | +0.43 to +0.76 | +0.62 to +0.68 |
| no-replay and plain-both | 6 | 8 to 697 | −0.29 to +0.63 | +0.09 to +0.41 |
| awake and no-replay | 12 | 17524 to 38299 | −0.36 to −0.11 | −0.04 to +0.28 |
| no-replay and plain-first | 12 | 4 to 463 | +0.34 to +0.61 | +0.17 to +0.47 |
| no-replay and twin-second | 12 | 3 to 277 | +0.32 to +0.65 | +0.23 to +0.52 |
| no-replay and plain-both | 12 | 0 to 271 | +0.24 to +0.68 | −0.05 to +0.54 |
| awake and no-replay | 18 | 38207 to 58840 | −0.18 to +0.28 | +0.04 to +0.17 |
| no-replay and plain-first | 18 | 26 to 229 | +0.01 to +0.58 | −0.11 to +0.45 |
| no-replay and twin-second | 18 | 18 to 451 | −0.06 to +0.76 | −0.07 to +0.25 |
| no-replay and plain-both | 18 | 25 to 125 | +0.29 to +0.57 | 0.00 to +0.52 |
| awake and no-replay | 24 | 58704 to 79396 | −0.16 to −0.01 | −0.02 to +0.20 |
| no-replay and plain-first | 24 | 87 to 224 | +0.08 to +0.29 | +0.05 to +0.54 |
| no-replay and twin-second | 24 | 7 to 229 | +0.32 to +0.49 | +0.07 to +0.39 |
| no-replay and plain-both | 24 | 62 to 226 | 0.00 to +0.38 | −0.04 to +0.50 |
| awake and no-replay | 36 | 58443 to 79553 | −0.12 to +0.28 | −0.13 to +0.04 |
| no-replay and plain-first | 36 | 1 to 476 | +0.20 to +0.29 | −0.02 to +0.33 |
| no-replay and twin-second | 36 | 0 to 379 | +0.18 to +0.47 | +0.10 to +0.33 |
| no-replay and plain-both | 36 | 27 to 476 | +0.15 to +0.31 | −0.16 to +0.25 |

- **The night without replay and the awake row stand 2 598 to 79 553 ticks apart**, 58 443 to 79 553 at blocks 24 and 36. A start's correlation of
  A's counts is −0.16 to −0.01 at block 24 and −0.12 to +0.28 at block 36. That is why the SE against the awake row keeps each form's streams apart.
- **It stands 0 to 697 ticks from 018's three nights**, and a start's correlation of A's counts with theirs is −0.29 to +0.76. Two nights share
  part of a stream, and the paired SE takes that out.

## 3. What it does to the pair

The pair's mean count of 64, in the same order as section 1:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 62.3, 63.1, 63.4, 63.4, 63.6 | 41.8, 60.7, 62.5, 62.1, 63.0 | 44.5, 60.1, 61.4, 62.2, 63.1 | 52.8, 60.7, 61.7, 62.2, 62.7 |
| No replay (`no-replay`) | 62.1, 61.3, 61.6, 60.9, 62.6 | 43.3, 60.5, 60.7, 61.1, 63.0 | 45.7, 60.9, 61.6, 61.5, 63.3 | 53.1, 61.0, 62.6, 62.4, 63.1 |
| Plain, A's patterns (`plain-first`) | 63.1, 62.6, 63.3, 61.9, 62.9 | 45.8, 63.1, 62.8, 62.6, 62.8 | 42.1, 61.7, 61.4, 61.7, 63.1 | 54.3, 61.4, 62.4, 61.6, 62.5 |
| Twin, the pair's patterns (`twin-second`) | 61.0, 58.9, 58.7, 58.1, 62.3 | 38.7, 58.8, 60.8, 62.0, 63.1 | 42.5, 60.0, 61.1, 60.5, 62.5 | 50.0, 59.5, 59.4, 59.3, 61.5 |
| Plain, both (`plain-both`) | 63.3, 63.0, 62.8, 62.2, 62.9 | 44.4, 62.7, 62.9, 63.4, 63.4 | 46.5, 62.1, 62.8, 63.1, 63.4 | 56.0, 62.1, 62.7, 62.6, 62.9 |

The pair's margin, M, from `kinds`:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 2.08, 2.36, 2.46, 2.57, 2.65 | 0.84, 2.20, 2.40, 2.48, 2.57 | 0.81, 2.05, 2.25, 2.49, 2.70 | 0.93, 1.81, 2.00, 2.15, 2.32 |
| No replay (`no-replay`) | 2.10, 2.27, 2.29, 2.33, 2.62 | 0.84, 2.07, 2.05, 2.12, 2.56 | 0.90, 2.16, 2.27, 2.36, 2.68 | 0.98, 1.91, 2.07, 2.18, 2.39 |
| Plain, A's patterns (`plain-first`) | 1.46, 1.16, 0.98, 0.73, 1.14 | 0.13, 0.98, 0.74, 0.30, 0.66 | 0.06, 0.97, 0.67, 0.59, 1.03 | 0.38, 0.93, 0.91, 0.90, 1.23 |
| Twin, the pair's patterns (`twin-second`) | 1.46, 1.10, 0.96, 0.83, 1.61 | 0.20, 1.15, 0.86, 0.72, 1.11 | 0.20, 1.01, 0.76, 0.56, 1.02 | 0.30, 0.78, 0.60, 0.38, 0.88 |
| Plain, both (`plain-both`) | 2.32, 2.65, 2.69, 2.85, 3.15 | 1.01, 2.51, 2.66, 2.93, 3.14 | 1.08, 2.60, 2.89, 3.10, 3.31 | 1.29, 2.27, 2.70, 3.00, 3.25 |

Each difference, the pair's count:

| Difference | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| no-replay less awake | −0.2, −1.8, −1.8, −2.5, −1.0 | +1.5, −0.2, −1.8, −1.0, 0.0 | +1.1, +0.8, +0.1, −0.7, +0.2 | +0.3, +0.3, +0.9, +0.2, +0.5 |
| plain-first less no-replay | +1.1, +1.4, +1.7, +1.0, +0.3 | +2.5, +2.6, +2.1, +1.6, −0.1 | −3.5, +0.8, −0.2, +0.2, −0.2 | +1.1, +0.4, −0.2, −0.8, −0.7 |
| plain-both less no-replay | +1.2, +1.7, +1.2, +1.3, +0.3 | +1.1, +2.2, +2.2, +2.3, +0.4 | +0.8, +1.2, +1.2, +1.6, 0.0 | +2.8, +1.0, +0.1, +0.3, −0.2 |
| twin-second less no-replay | −1.0, −2.4, −2.9, −2.8, −0.3 | −4.6, −1.8, +0.1, +0.9, +0.1 | −3.2, −0.9, −0.5, −1.0, −0.8 | −3.1, −1.5, −3.1, −3.1, −1.6 |

And its margin, M:

| Difference | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| no-replay less awake | +0.01, −0.09, −0.18, −0.24, −0.03 | +0.01, −0.12, −0.35, −0.36, −0.01 | +0.09, +0.11, +0.02, −0.13, −0.02 | +0.05, +0.09, +0.07, +0.03, +0.06 |
| plain-first less no-replay | −0.64, −1.11, −1.30, −1.60, −1.49 | −0.71, −1.10, −1.31, −1.81, −1.90 | −0.84, −1.20, −1.60, −1.77, −1.65 | −0.60, −0.97, −1.16, −1.28, −1.15 |
| plain-both less no-replay | +0.23, +0.38, +0.40, +0.52, +0.53 | +0.17, +0.44, +0.61, +0.82, +0.58 | +0.19, +0.44, +0.62, +0.74, +0.63 | +0.31, +0.36, +0.63, +0.82, +0.87 |
| twin-second less no-replay | −0.63, −1.17, −1.32, −1.50, −1.01 | −0.65, −0.92, −1.19, −1.39, −1.45 | −0.69, −1.15, −1.51, −1.80, −1.66 | −0.69, −1.12, −1.47, −1.80, −1.51 |

The pair's count at block 24:

| Difference | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| no-replay less awake | −2.5 ± 0.5 | −1.0 ± 0.7 | −0.7 ± 0.6 | +0.2 ± 0.5 | −1.0 | 0, 3, 1 |
| plain-first less no-replay | +1.0 ± 0.3 | +1.6 ± 0.5 | +0.2 ± 0.5 | −0.8 ± 0.6 | +0.5 | 2, 2, 0 |
| plain-both less no-replay | +1.3 ± 0.5 | +2.3 ± 0.4 | +1.6 ± 0.5 | +0.3 ± 0.6 | +1.4 | 3, 1, 0 |
| twin-second less no-replay | −2.8 ± 0.7 | +0.9 ± 0.5 | −1.0 ± 0.9 | −3.1 ± 1.0 | −1.5 | 0, 2, 2 |

At block 36:

| Difference | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| no-replay less awake | −1.0 ± 0.2 | 0.0 ± 0.1 | +0.2 ± 0.1 | +0.5 ± 0.4 | −0.1 | 0, 3, 1 |
| plain-first less no-replay | +0.3 ± 0.3 | −0.1 ± 0.3 | −0.2 ± 0.2 | −0.7 ± 0.1 | −0.2 | 0, 3, 1 |
| plain-both less no-replay | +0.3 ± 0.2 | +0.4 ± 0.1 | 0.0 ± 0.1 | −0.2 ± 0.3 | +0.1 | 1, 3, 0 |
| twin-second less no-replay | −0.3 ± 0.2 | +0.1 ± 0.2 | −0.8 ± 0.2 | −1.6 ± 0.4 | −0.7 | 0, 2, 2 |

Its margin at block 24, M:

| Difference | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| no-replay less awake | −0.24 ± 0.02 | −0.36 ± 0.03 | −0.13 ± 0.10 | +0.03 ± 0.08 | −0.18 | 0, 2, 2 |
| plain-first less no-replay | −1.60 ± 0.02 | −1.81 ± 0.03 | −1.77 ± 0.11 | −1.28 ± 0.09 | −1.62 | 0, 0, 4 |
| plain-both less no-replay | +0.52 ± 0.01 | +0.82 ± 0.01 | +0.74 ± 0.05 | +0.82 ± 0.10 | +0.72 | 4, 0, 0 |
| twin-second less no-replay | −1.50 ± 0.06 | −1.39 ± 0.04 | −1.80 ± 0.16 | −1.80 ± 0.12 | −1.62 | 0, 0, 4 |

At block 36:

| Difference | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| no-replay less awake | −0.03 ± 0.07 | −0.01 ± 0.08 | −0.02 ± 0.02 | +0.06 ± 0.06 | 0.00 | 0, 4, 0 |
| plain-first less no-replay | −1.49 ± 0.03 | −1.90 ± 0.06 | −1.65 ± 0.02 | −1.15 ± 0.07 | −1.55 | 0, 0, 4 |
| plain-both less no-replay | +0.53 ± 0.09 | +0.58 ± 0.05 | +0.63 ± 0.02 | +0.87 ± 0.07 | +0.65 | 4, 0, 0 |
| twin-second less no-replay | −1.01 ± 0.09 | −1.45 ± 0.08 | −1.66 ± 0.07 | −1.51 ± 0.04 | −1.41 | 0, 0, 4 |

**Its rewarded blocks.** `correct` of each form's `run` lines. Each cell lists five stretches, blocks 1 to 6 (the head), 7 to 12, 13 to 18, 19 to 24
and 25 to 36, each the mean of all its blocks over the three paths, and in brackets the mean of its first block:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 44.4 (12.3), 63.1 (62.3), 62.9 (63.3), 63.7 (63.3), 63.7 (63.3) | 20.0 (3.3), 57.2 (49.7), 62.0 (61.0), 63.0 (61.3), 63.0 (63.3) | 24.2 (7.3), 56.2 (45.3), 61.2 (60.7), 61.8 (60.0), 62.8 (61.0) | 33.8 (13.3), 58.6 (56.7), 61.3 (61.0), 62.1 (63.7), 62.7 (61.7) |
| No replay (`no-replay`) | 44.4 (12.3), 62.6 (62.0), 62.4 (61.3), 62.3 (60.0), 62.7 (62.0) | 20.0 (3.3), 56.6 (46.7), 61.9 (61.3), 62.4 (60.0), 62.3 (60.0) | 24.2 (7.3), 57.3 (51.7), 62.6 (61.7), 62.0 (62.7), 62.7 (61.3) | 33.8 (13.3), 59.8 (56.3), 62.2 (59.7), 62.7 (63.0), 63.2 (63.7) |
| Plain, A's patterns (`plain-first`) | 44.4 (12.3), 62.7 (63.3), 62.7 (62.7), 62.6 (62.7), 62.8 (62.7) | 20.0 (3.3), 56.7 (48.3), 62.2 (63.0), 62.7 (62.3), 62.7 (62.7) | 24.2 (7.3), 55.5 (46.0), 61.6 (62.0), 61.6 (62.0), 62.0 (61.7) | 33.8 (13.3), 57.9 (55.0), 60.8 (59.7), 61.9 (62.7), 62.2 (61.0) |
| Twin, the pair's patterns (`twin-second`) | 44.4 (12.3), 62.0 (61.7), 60.0 (60.0), 60.3 (59.3), 60.9 (59.0) | 20.0 (3.3), 53.7 (41.3), 60.3 (59.3), 61.7 (62.0), 62.4 (60.7) | 24.2 (7.3), 55.9 (48.3), 61.4 (59.3), 60.3 (60.3), 61.8 (62.0) | 33.8 (13.3), 57.5 (53.7), 60.5 (59.0), 59.8 (60.3), 61.0 (60.7) |
| Plain, both (`plain-both`) | 44.4 (12.3), 62.6 (63.7), 63.1 (62.7), 62.6 (62.7), 62.8 (63.3) | 20.0 (3.3), 56.7 (46.0), 62.6 (63.3), 62.8 (63.3), 63.3 (63.3) | 24.2 (7.3), 56.4 (49.0), 62.3 (63.0), 62.6 (63.0), 62.8 (63.0) | 33.8 (13.3), 59.5 (59.3), 61.4 (60.3), 62.5 (63.7), 63.0 (62.0) |

Each difference of those stretch means, for the four stretches after the head:

| Difference | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| no-replay less awake | −0.4, −0.6, −1.4, −1.0 | −0.6, −0.1, −0.6, −0.7 | +1.2, +1.4, +0.2, −0.1 | +1.2, +0.9, +0.7, +0.4 |
| plain-first less no-replay | +0.1, +0.3, +0.3, +0.1 | +0.2, +0.3, +0.3, +0.4 | −1.8, −0.9, −0.4, −0.7 | −1.9, −1.4, −0.8, −1.0 |
| plain-both less no-replay | 0.0, +0.7, +0.3, +0.1 | +0.1, +0.7, +0.4, +1.0 | −0.9, −0.2, +0.6, +0.1 | −0.3, −0.8, −0.2, −0.2 |
| twin-second less no-replay | −0.6, −2.4, −1.9, −1.8 | −2.8, −1.6, −0.7, +0.1 | −1.4, −1.1, −1.7, −0.9 | −2.3, −1.7, −2.9, −2.2 |

The pair's first rewarded block at 40 of 64, then at 60, one number a path in the order 0, 5, 6, counted from its first block over all 36:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 3 3 3; 5 5 4 | 7 6 6; 11 8 11 | 7 8 6; 11 10 10 | 5 5 4; 9 12 10 |
| No replay (`no-replay`) | 3 3 3; 5 5 4 | 7 6 6; 12 9 11 | 7 7 6; 12 10 10 | 5 5 4; 8 8 11 |
| Plain, A's patterns (`plain-first`) | 3 3 3; 5 5 4 | 7 6 6; 12 9 10 | 7 7 6; 10 11 11 | 5 5 4; 10 10 12 |
| Twin, the pair's patterns (`twin-second`) | 3 3 3; 5 5 4 | 8 6 6; 12 9 11 | 7 7 6; 12 10 10 | 5 5 4; 9 12 11 |
| Plain, both (`plain-both`) | 3 3 3; 5 5 4 | 8 6 6; 9 10 10 | 7 7 6; 10 11 11 | 5 5 4; 9 9 7 |

**Where the margins lie.** Each kind's sum on the readouts, from `kinds`, in M, the mean over the three paths of: A's margin on its own units
(kinds 0 and 3), on the shared units (kinds 1 and 4), the pair's margin on its own units (kinds 2 and 5), on the shared units:

| Block | Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| ---: | :--- | :--- | :--- | :--- | :--- |
| 24 | awake | 2.06, −0.95, 1.52, 1.05 | 0.92, −0.65, 1.67, 0.81 | 0.81, −0.52, 1.81, 0.68 | 1.60, −0.28, 1.76, 0.39 |
| 24 | no-replay | 1.75, −0.56, 1.72, 0.61 | 0.35, −0.36, 1.68, 0.44 | 0.88, −0.49, 1.80, 0.57 | 1.90, −0.16, 1.97, 0.21 |
| 24 | plain-first | 3.39, 1.12, 1.50, −0.78 | 2.68, 0.98, 0.93, −0.62 | 2.69, 0.96, 1.08, −0.49 | 3.11, 1.35, 1.87, −0.97 |
| 24 | twin-second | 2.43, 0.02, 0.89, −0.06 | 0.28, 0.08, 0.55, 0.18 | 0.79, 0.64, 0.70, −0.14 | 2.60, 0.94, 1.34, −0.96 |
| 24 | plain-both | 3.11, −0.34, 2.29, 0.56 | 1.83, −0.43, 2.06, 0.87 | 1.81, −0.38, 2.19, 0.91 | 2.79, −0.14, 2.62, 0.38 |
| 36 | awake | 2.06, −1.00, 1.55, 1.10 | 0.92, −0.69, 1.71, 0.85 | 0.81, −0.61, 1.93, 0.77 | 1.60, −0.39, 1.83, 0.50 |
| 36 | no-replay | 1.75, −0.74, 1.84, 0.79 | 0.35, −0.60, 1.89, 0.67 | 0.88, −0.63, 1.98, 0.70 | 1.90, −0.29, 2.05, 0.34 |
| 36 | plain-first | 3.39, 0.87, 1.67, −0.53 | 2.68, 0.81, 1.12, −0.46 | 2.69, 0.77, 1.33, −0.30 | 3.11, 1.15, 2.01, −0.77 |
| 36 | twin-second | 2.43, −0.46, 1.19, 0.42 | 0.28, −0.10, 0.74, 0.37 | 0.79, 0.43, 0.94, 0.08 | 2.60, 0.63, 1.52, −0.65 |
| 36 | plain-both | 3.11, −0.55, 2.39, 0.76 | 1.83, −0.53, 2.17, 0.97 | 1.81, −0.47, 2.31, 1.00 | 2.79, −0.29, 2.73, 0.52 |

- **The pair learns under the night without replay.** It reads 60.9 to 62.4 at block 24 and 62.6 to 63.3 at block 36, the awake row 62.1 to 63.4
  and 62.7 to 63.6. Against the awake row it stands −2.5, −1.0, −0.7, +0.2 at block 24 and −1.0, 0.0, +0.2, +0.5 at block 36.
- **Its margin against the awake row's.** −0.24, −0.36, −0.13, +0.03 M at block 24, two of them below 2 SE, and −0.03, −0.01, −0.02, +0.06 M at
  block 36, all four within 2 SE.
- **The plain night on A's patterns stands under it in the pair's margin.** −1.60, −1.81, −1.77, −1.28 M at block 24 and −1.49, −1.90, −1.65, −1.15 M at
  block 36, all below 2 SE. In count it stands +1.0, +1.6, +0.2, −0.8 and +0.3, −0.1, −0.2, −0.7.
- **The twin stands under it in the pair's margin too**, by 1.39 to 1.80 M at block 24 and 1.01 to 1.66 M at block 36, and plain on both over it,
  by 0.52 to 0.82 M and 0.53 to 0.87 M.
- **Its rewarded blocks.** A stretch's mean stands −1.4 to +1.4 against the awake row's. The first block at 60 of 64 moves by −4 to +1 on a
  path.
- **Where its margins lie.** At block 24 A's own units carry 1.75, 0.35, 0.88, 1.90 M, the awake row's 2.06, 0.92, 0.81, 1.60. From the eighth image
  and the mirrored assignment that is 0.35 against 0.92, and the twin's 0.28 there. From block 24 to block 36 A's own units move by at most 1 456 on one path
  under it, and by at most 2 276 under 018's four forms.
- **Under the other SE two of the pair's tallies move.** The plain night on A's patterns less the night without replay reads 1, 3, 0 at block 24, and the night
  without replay less the awake row reads 1, 2, 1 at block 36.

## 4. Couplings, sums and gain

**A's highest coupling**, the largest of A's four over each block's end of every form's `run` lines on three paths and each image read, each over
H-42's image's, on `PIN couplings`. **At the last block** is the largest at block 36's end. **Where** is the path, the stretch by its last block, the
block in it, or the image read, and the coupling in the order A onto readouts 0 and 1, B onto 0 and 1:

| Start | Form | Greatest ratio | Where | At the last block |
| :--- | :--- | ---: | :--- | ---: |
| Eighth, assignment | awake | 1.133 | path 0 run to 6 block 1, coupling 2 | 1.053 |
| Eighth, assignment | no-replay | 1.133 | path 0 run to 6 block 1, coupling 2 | 1.040 |
| Eighth, assignment | plain-first | 1.184 | path 6 run to 36 block 1, coupling 2 | 1.175 |
| Eighth, assignment | twin-second | 1.133 | path 0 run to 6 block 1, coupling 2 | 1.088 |
| Eighth, assignment | plain-both | 1.133 | path 0 run to 6 block 1, coupling 2 | 1.116 |
| Eighth, mirrored | awake | 1.144 | path 6 run to 6 block 1, coupling 0 | 1.036 |
| Eighth, mirrored | no-replay | 1.144 | path 6 run to 6 block 1, coupling 0 | 1.015 |
| Eighth, mirrored | plain-first | 1.197 | path 5 run to 36 block 5, coupling 0 | 1.196 |
| Eighth, mirrored | twin-second | 1.144 | path 6 run to 6 block 1, coupling 0 | 1.033 |
| Eighth, mirrored | plain-both | 1.144 | path 6 run to 6 block 1, coupling 0 | 1.103 |
| First mapping, assignment | awake | 1.112 | path 6 run to 6 block 1, coupling 0 | 1.046 |
| First mapping, assignment | no-replay | 1.112 | path 6 run to 6 block 1, coupling 0 | 1.038 |
| First mapping, assignment | plain-first | 1.198 | path 6 read at 24, coupling 0 | 1.190 |
| First mapping, assignment | twin-second | 1.112 | path 6 run to 6 block 1, coupling 0 | 1.100 |
| First mapping, assignment | plain-both | 1.112 | path 6 run to 6 block 1, coupling 0 | 1.103 |
| First mapping, mirrored | awake | 1.114 | path 5 run to 6 block 1, coupling 2 | 1.052 |
| First mapping, mirrored | no-replay | 1.114 | path 5 run to 6 block 1, coupling 2 | 1.063 |
| First mapping, mirrored | plain-first | 1.182 | path 6 read at 24, coupling 2 | 1.171 |
| First mapping, mirrored | twin-second | 1.145 | path 5 read at 24, coupling 2 | 1.130 |
| First mapping, mirrored | plain-both | 1.116 | path 5 read at 24, coupling 2 | 1.109 |

The pair's highest coupling at each image read, in M, the largest of any path's four from `kinds`:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 6.46, 6.51, 6.53, 6.56, 6.57 | 5.97, 6.25, 6.32, 6.34, 6.37 | 5.96, 6.23, 6.28, 6.32, 6.35 | 6.19, 6.40, 6.46, 6.45, 6.48 |
| No replay (`no-replay`) | 6.46, 6.54, 6.54, 6.57, 6.62 | 5.95, 6.23, 6.19, 6.17, 6.34 | 6.00, 6.28, 6.35, 6.33, 6.35 | 6.22, 6.46, 6.48, 6.51, 6.55 |
| Plain, A's patterns (`plain-first`) | 6.32, 6.21, 6.16, 6.15, 6.22 | 5.75, 5.92, 5.91, 5.90, 5.83 | 6.05, 5.95, 5.90, 5.90, 5.93 | 6.06, 6.21, 6.12, 6.14, 6.24 |
| Twin, the pair's patterns (`twin-second`) | 6.32, 6.26, 6.11, 6.17, 6.27 | 5.83, 6.06, 5.95, 5.90, 5.97 | 6.06, 5.99, 5.90, 5.82, 5.98 | 6.08, 6.16, 6.01, 5.99, 6.08 |
| Plain, both (`plain-both`) | 6.56, 6.62, 6.63, 6.68, 6.74 | 6.03, 6.34, 6.38, 6.46, 6.49 | 6.04, 6.40, 6.50, 6.56, 6.57 | 6.32, 6.53, 6.57, 6.68, 6.75 |

**The inhibitory sum**, the `inhibitory` of `first`'s stream 0, over H-42's image's 148235483 (`PIN quiet`), mean over the three paths:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 0.517, 0.518, 0.520, 0.521, 0.520 | 0.527, 0.530, 0.533, 0.535, 0.536 | 0.941, 0.921, 0.898, 0.876, 0.849 | 0.937, 0.915, 0.893, 0.871, 0.843 |
| No replay (`no-replay`) | 0.517, 0.520, 0.522, 0.525, 0.527 | 0.527, 0.532, 0.536, 0.541, 0.545 | 0.941, 0.920, 0.896, 0.872, 0.841 | 0.937, 0.914, 0.890, 0.865, 0.834 |
| Plain, A's patterns (`plain-first`) | 0.572, 0.606, 0.634, 0.656, 0.635 | 0.593, 0.638, 0.674, 0.703, 0.682 | 1.011, 1.034, 1.041, 1.037, 0.995 | 0.995, 1.010, 1.012, 1.006, 0.966 |
| Twin, the pair's patterns (`twin-second`) | 0.557, 0.584, 0.608, 0.632, 0.615 | 0.569, 0.601, 0.629, 0.653, 0.635 | 0.986, 0.996, 0.997, 0.991, 0.946 | 0.981, 0.991, 0.992, 0.989, 0.945 |
| Plain, both (`plain-both`) | 0.565, 0.594, 0.615, 0.633, 0.610 | 0.583, 0.621, 0.649, 0.671, 0.649 | 1.000, 1.020, 1.026, 1.024, 0.983 | 0.990, 1.004, 1.007, 1.004, 0.963 |

**The excitatory sum**, the same over H-42's image's 213566021:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 1.000, 1.000, 1.000, 1.000, 1.000 | 1.000, 1.000, 1.000, 1.000, 1.000 | 1.000, 1.000, 1.000, 1.000, 1.000 | 1.000, 1.000, 1.000, 1.000, 1.000 |
| No replay (`no-replay`) | 0.997, 0.997, 0.997, 0.998, 0.998 | 0.998, 0.997, 0.997, 0.999, 0.999 | 0.998, 0.998, 1.000, 1.002, 1.002 | 0.998, 0.998, 1.000, 1.002, 1.002 |
| Plain, A's patterns (`plain-first`) | 0.957, 0.928, 0.908, 0.893, 0.893 | 0.961, 0.934, 0.918, 0.907, 0.907 | 0.968, 0.943, 0.927, 0.914, 0.914 | 0.965, 0.938, 0.920, 0.906, 0.906 |
| Twin, the pair's patterns (`twin-second`) | 0.968, 0.946, 0.931, 0.922, 0.922 | 0.966, 0.944, 0.929, 0.920, 0.920 | 0.972, 0.953, 0.942, 0.934, 0.934 | 0.974, 0.955, 0.944, 0.936, 0.936 |
| Plain, both (`plain-both`) | 0.962, 0.932, 0.910, 0.893, 0.893 | 0.963, 0.936, 0.915, 0.901, 0.901 | 0.969, 0.943, 0.925, 0.911, 0.911 | 0.968, 0.942, 0.922, 0.905, 0.905 |

What each night took from it, M, `excitatory` of the gap's `start` line less its `end` line, mean over the three paths, and last the image at block
24 less at block 36:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| No replay (`no-replay`) | 0.61, 0.11, 0.06, −0.23, 0.00 | 0.49, 0.14, −0.07, −0.37, 0.00 | 0.51, −0.11, −0.45, −0.34, 0.00 | 0.44, −0.09, −0.40, −0.37, 0.00 |
| Plain, A's patterns (`plain-first`) | 9.17, 6.30, 4.29, 3.19, 0.00 | 8.37, 5.67, 3.55, 2.20, 0.00 | 6.90, 5.36, 3.43, 2.62, 0.00 | 7.55, 5.75, 3.85, 2.96, 0.00 |
| Twin, the pair's patterns (`twin-second`) | 7.01, 4.69, 3.16, 1.92, 0.00 | 7.13, 4.75, 3.35, 1.85, 0.00 | 5.98, 4.09, 2.35, 1.71, 0.00 | 5.64, 4.03, 2.33, 1.66, 0.00 |
| Plain, both (`plain-both`) | 8.17, 6.36, 4.66, 3.70, 0.00 | 7.86, 5.86, 4.44, 3.05, 0.00 | 6.71, 5.41, 3.88, 3.12, 0.00 | 6.86, 5.48, 4.36, 3.54, 0.00 |

What each night added to the inhibitory sum, M, `inhibitory` of the gap's `end` line less its `start` line, mean over the three paths:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| No replay (`no-replay`) | 0.28, 0.23, 0.24, 0.28 | 0.31, 0.31, 0.30, 0.39 | −1.22, −1.36, −1.46, −1.33 | −1.32, −1.39, −1.39, −1.40 |
| Plain, A's patterns (`plain-first`) | 8.35, 6.67, 5.74, 4.92 | 10.11, 8.07, 6.90, 5.89 | 9.08, 6.02, 3.98, 2.60 | 7.36, 4.75, 3.21, 2.24 |
| Twin, the pair's patterns (`twin-second`) | 6.20, 5.35, 5.12, 5.05 | 6.60, 5.80, 5.40, 4.98 | 5.45, 4.06, 3.31, 2.67 | 5.16, 4.05, 3.15, 2.61 |
| Plain, both (`plain-both`) | 7.38, 5.82, 4.97, 4.35 | 8.54, 7.12, 5.90, 4.92 | 7.52, 5.41, 3.96, 2.94 | 6.58, 4.65, 3.37, 2.53 |

**The gain**, `gain` of `state`, mean over the three paths:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 113585, 113646, 113757, 113677, 111496 | 113686, 113623, 113762, 113731, 111541 | 114541, 114152, 114297, 114404, 112201 | 114518, 114148, 114287, 114370, 112193 |
| No replay (`no-replay`) | 113365, 113135, 113170, 112494, 110144 | 113415, 113131, 113109, 112515, 110013 | 114278, 113785, 113545, 113483, 110943 | 114311, 113806, 113565, 113537, 110989 |
| Plain, A's patterns (`plain-first`) | 107045, 107713, 108301, 108645, 111450 | 106360, 107125, 107624, 107902, 111273 | 107324, 107864, 108232, 108790, 111939 | 107984, 108459, 108786, 109361, 112015 |
| Twin, the pair's patterns (`twin-second`) | 109560, 109855, 110017, 109779, 110691 | 109707, 109832, 109985, 109791, 110605 | 110626, 110750, 110572, 110639, 111450 | 110595, 110533, 110545, 110642, 111483 |
| Plain, both (`plain-both`) | 107651, 108384, 108700, 109174, 111532 | 107458, 108104, 108298, 108688, 111388 | 108300, 108769, 108748, 109565, 112058 | 108589, 108946, 108982, 109808, 112101 |

- **A's highest coupling under the night without replay is the head's**, 1.112 to 1.144 of H-42's image's, at the head's first block, the same as the awake
  row's. At block 36's end it stands at 1.015 to 1.063, the awake row's 1.036 to 1.053. 018's plain night on A's patterns reached 1.198. H-46's clause 1
  named 1.30. This is another run, and that bound is not applied here.
- **The pair's highest coupling** reads 5.95 to 6.62 M under it, the awake row 5.96 to 6.57.
- **The excitatory sum.** On one path at the reads it stands at 0.9965 to 1.0022 of H-42's image's. A night moves it
  by −0.64 to +0.45 M on one path, and the 12 blocks after the last by −5 119 to +1 870. 018's plain night on A's patterns took 6.90 to 9.17 M on the first night.
- **The inhibitory sum moves one way from the eighth image and the other from a first mapping.** From the eighth image each night without replay raises it, by 0.21 to 0.40 M on one path. From a first
  mapping each lowers it, by 1.21 to 1.48 M. 24 of the 48 raise it, where all 144 of 018's nights did. At block 36 it stands +1.08 and +1.37 M over the awake
  row from the eighth image and −1.12 and −1.31 M from a first mapping, and 15.97 to 22.79 M under the plain night on A's patterns.
- **A night without replay raises the gain.** Each of the 48 does, by 1 716 to 2 580, from 110 302 to 112 456 before to 112 474 to 114 328 after. Every one of
  018's 144 nights lowered it, to 106 291 to 110 762. At block 24 its gain stands 833 to 1 216 under the awake row's and 3 849 to 4 693 over the plain
  night's, and at block 36 1 204 to 1 528 under the awake row's and 996 to 1 306 under the plain night's.
- **The groups hold.** No group's sum stood more than 87 from its value over the night without replay's 60 images, and 399 over all five forms' 300.

## What it costs

The harness's trial took 451 s and the four jobs 2 103 to 2 357 s each, 8 903 s in all, none near a job's 240 minutes. That is five jobs, the
four the note was scoped to and the trial. Nothing was cut. That is 1 920 reads on forks, and 28 in the trial.

## What it suggests

**The plain night's difference from the awake row and its difference from a night without replay read differently by start.** From the eighth image and
the mirrored assignment the night without replay stands 14.4 under the awake row at block 36 and the plain night 19.6 over the night without replay.
From the other three the night without replay stands +1.2, −0.4 and +0.9 against the awake row there, and the plain night 2.2 to 7.9 over it.

Questions this suggests:
- what in a night without replay takes A down from the eighth image and the mirrored assignment, and not from the other three;
- why a night without replay raises the gain where each of 018's 144 nights lowered it.

## What did not run

- **A night without replay at any other setting**: another shift, the slow-wave baseline unset or whole, the drive off. So nothing here parts the slow-wave
  baseline from the rest of a night.
- **The twin on A's patterns**, ADR-0232's twin with the replay's drive divided by the gain, beside this night. 018's twin ran on the pair's patterns.
- **Seeds 23 and 24**, so nothing here says what a seed adds.
- **Any share but 26 of 51**, an agreeing pair, or the `Kept` cut.
- **A fourth path, or any read past block 36.**
- **A's count just before a gap.** Blocks 6 to 24 read after the gap.
- **018's four forms.** Their rows are 018's logs. Each start's head and pins in this note's jobs equal theirs, bit for bit.

## The runs

| Test | Run | Seconds |
| :--- | :--- | ---: |
| `nights_without_replay_of_the_eighth_image_on_seed_22_from_the_assignment_explore` | [41408442255](https://github.com/Menteion/Menteion/actions/runs/41408442255) | 2157 |
| `nights_without_replay_of_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [41413189972](https://github.com/Menteion/Menteion/actions/runs/41413189972) | 2103 |
| `nights_without_replay_of_a_first_mapping_on_seed_22_from_the_assignment_explore` | [41413189972](https://github.com/Menteion/Menteion/actions/runs/41413189972) | 2286 |
| `nights_without_replay_of_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [41413189972](https://github.com/Menteion/Menteion/actions/runs/41413189972) | 2357 |

The trial, `the_harness_of_a_night_without_replay_explore`, ran in [41408442255](https://github.com/Menteion/Menteion/actions/runs/41408442255),
451 s. Every test ran at `0a6024f`.
