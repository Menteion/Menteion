---
status: exploratory
date: 2026-11-02
---

# 018: What nights keep

**Branch**: `explore/018-what-nights-keep`
**Commit**: `5f0bf27` for the harness's trial, `1043fa5` for the rest, which takes the paths from four to three. Both carry `c202f95`,
`main` at exploration 017's note, and the tag `explore/017-what-a-read-spreads` with what it carries under `src/`. Neither adds any.
**Runs**: [41365818240](https://github.com/Menteion/Menteion/actions/runs/41365818240) (the harness's trial),
[41367828454](https://github.com/Menteion/Menteion/actions/runs/41367828454) (question 1 from the eighth image and the assignment) and
[41374137568](https://github.com/Menteion/Menteion/actions/runs/41374137568) (the other three starts), all at `scope=arms`

[Exploration 017](017-what-a-read-spreads.md) read one plain night on A's patterns by the mean of sixteen reads on nine paths. A stood
+2.4, +3.5, +6.7 and +2.1 over the awake row right after the gap and +0.0, +0.9, +4.7 and +0.0 twelve blocks on, the margin up 1.05 to
1.30 M at both. Twelve blocks on, one night's count stood over that row in one start of four. This note asks, by 017's reading:

1. where A's count stands after four nights, one every 6 of the pair's first 24 blocks, and after 12 more blocks with no night;
2. what the density of nights changes: every 12 blocks and every 3. This one did not run, and What ran says why;
3. what the nights cost: the pair's count and margin, the highest coupling, the groups' sums, the inhibitory sum and the gain.

It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the repository's
AArch64 host, on seed 22 alone, on three paths.

**In brief**
- **Plain on A's patterns stands over the awake row in all four starts.** Four nights less four awake gaps: +6.4, +8.2, +10.7, +5.6 right
  after the last, SE 1.1 to 1.4, and +3.4, +5.3, +7.5, +5.9 twelve blocks on, SE 0.6 to 1.3. A's margin stands up +3.13 to +3.39 M and
  +3.04 to +3.27 M.
- **The twin on the pair's patterns parts the starts.** +10.7, −9.4, +12.8, +10.8, then +6.8, −13.6, +9.5, +10.1. From the eighth image and
  the mirrored assignment it leaves A at 17.3 of 64 at block 36, the margin 0.05 M under the awake row's.
- **Plain on both stands over the awake row in three of the four at block 36.** +12.4, +2.4, +7.4, +5.5, then +11.0, −0.2, +5.5, +5.6.
- **No form has the highest count in every start.** Plain on A's patterns has the highest margin of the four forms in all four starts, and the
  highest count in one.
- **The pair learns under every form.** At block 36 it reads 61.5 to 63.4 of 64, the awake row 62.7 to 63.6. Its first block at 60 of 64
  moves by −3 to +2 blocks against the same path's awake row.
- **The pair's margin falls on plain on A's patterns and on the twin, and rises on plain on both.** At block 24, of the awake row's 2.15 to 2.57 M, plain
  on A's patterns takes 1.25 to 2.17 M and the twin 1.74 to 1.93 M, and at block 36 they take 1.09 to 1.91 M and 1.04 to 1.68 M. Plain on both adds 0.28
  to 0.85 M and 0.50 to 0.93 M. The twin also moves the pair's count: −5.4 to −0.1 at block 24, −1.3 to +0.1 at block 36.
- **A's highest coupling is 1.198 of H-42's image's at most**, on plain on A's patterns, the awake row's being 1.112 to 1.144. H-46's clause 1 named 1.30.
- **Nights move the sums, and the 12 blocks after leave the excitatory sum where it was.** Four nights leave it at 0.893 to 0.936 of H-42's image's,
  each taking less than the night before. The inhibitory sum stands +16.58 to +24.85 M over the awake row at block 24. A night takes the
  gain down, and 12 blocks bring it back.
- **The split stands where 017's did.** Pooled over 80 cells, a read's SD is 3.41, a path's 1.25, a stream's 2.41 and the rest 2.41, on three paths a cell.
- **Each night and the awake row stand 58 448 to 79 554 ticks apart at blocks 24 and 36**, and a start's correlation of their counts of A is −0.25 to
  +0.30. The three nights stand 0 to 477 apart and share part of a stream.
- **Question 2 did not run, and a fourth path did not.** What ran says why.

## What ran

**The network, the rules and the starts** are 017's, unchanged: H-42's network of seed 22 at 1 024 units, its derivation run again and held
to H-42's pinned tables (`probe_made`), H-46's arm, H-25's configuration, the slow-wave baseline at 1/2 on this branch alone, the assignment
and the mirrored assignment, the eighth image and a first mapping. The pair shares 26 of 51 units, conflicting, each kind of unit held
apart. **A**, **a read**, **a stream**, **a path** and **the margin** are 016's and 017's.

**Four forms of the pair's 36 blocks.** Six blocks, a gap, and so on to the gap at block 24, then 12 blocks with no gap to block 36. The
gap is the form's, the same at blocks 6, 12, 18 and 24:

| Form | Label | The gap |
| :--- | :--- | :--- |
| Awake | `awake` | 4 037 214 idle ticks, 017's awake gap |
| Plain, A's patterns | `plain-first` | A plain night on A's patterns, from the start image |
| Twin, the pair's patterns | `twin-second` | A twin night on the pair's patterns, from the image the night follows |
| Plain, both | `plain-both` | A plain night on both sets, A's from the start image and the pair's from the image the night follows |

Each night is otherwise 016's: shift 7, driven, the groups held, slow wave at 1/2, spending.

**Three paths**, from 017's definition. Path `j` runs each stretch of the pair's blocks from trial 4 096 × `j`, every stretch from that
trial again, as 017's second stretch began again. Path 0 is 016's stimuli and drive, and paths 5 and 6 run on the drive's seed 3 + `j`. The
gaps run under the drive as it was. Every form runs on all three, so a night and the awake row pair by path.

**Reads.** At blocks 6, 12, 18 and 24 right after that block's gap, and at block 36, sixteen reads of A and sixteen of the pair by `Both`, on
forks, the first being stream 0, the read as it was. That is 960 reads a job. Each job runs two forms, `awake` with `plain-first` and
`twin-second` with `plain-both`, so eight jobs make the four starts.

**What was cut, by the task's order.** The harness's trial timed a read at 3.0 s, a block at 2.95 s and a gap at 11 s, running two at a time.
On those figures question 1 on four paths would take eight jobs of about 87 minutes each, and question 2 another job of about 120 minutes a start. The
task scoped about eight jobs of under an hour each, and named its order of cuts: question 2, then the paths to three, then the first mapping.
Question 2 went and the paths went to three, leaving out path 7. The jobs then took 64 to 67 minutes each, past the hour by that much, and the first
mapping stayed. Question 2's four tests are on the branch, not run.

**Reading the logs.** A label is `shared zero22 <arm> <from> share 0b0101 Apart conflict true half nights path <j> every 6 <form> blocks <b>`,
`<arm>` being `AssignmentFirst` or `MirroredFirst`, `<from>` `Eighth` or `FirstMapping`, `<j>` 0, 5 or 6, and `<form>` the table's label.

- A `read`, `kinds` or `held` line of `blocks <b>` is the image after the gap at `<b>`, and at block 36 the image at that block's end. A read's label ends in
  `first` for A or `second` for the pair.
- A `run` line of `blocks <b>` is the stretch that ends at `<b>`, before any gap there. The first stretch, the head, is `none blocks 6`.
- A gap's lines end in `gap <b>`. A read's `state` is the image's own, taken before the fork runs.

Unless a table says otherwise, **a mean of a count** is of `correct` over the 48 read lines of one `<arm>`, `<from>`, `<form>`, `<b>` and `first` or
`second`, three paths by sixteen streams, those whose `spread` is `as-it-was 0` or `Both 1` to `Both 15`. **A's margin** is `c0 + c3 − c1 − c2` of the
`couplings` of `state` in `first`'s stream 0, negated when `read mirrored` is `true`. **The pair's margin** is the same sum of the pair's four
couplings from that image's `kinds` line, whose six rows are 016's six kinds and whose four columns are the two readouts of A and the two of
the pair: `c0` and `c1` are rows 1 and 2 summed in columns 2 and 3, `c2` and `c3` rows 4 and 5, rows and columns counted from 0, negated when `second`'s `read mirrored` is `true`. A margin's cell is the mean over the three paths. An **SD** is the sample's, over `n − 1`.

**Bit for bit.** All of these match:

- `probe_made` held H-42's tables in all nine jobs, each of which ended with exit 0.
- In the trial 68 of 68 `run` and `read` lines that stand beside 016's and 017's logs match theirs: 016's 6-block run and its read, 017's
  `half none blocks 12` run and its reads of A and the pair, 017's `half path 5 none blocks 12` run, and its `half path 5 awake` and
  `half path 5 plain-first` reads of A. The 34 lines of those two gaps match 017's too.
- 56 of 56 checks on the eight jobs' heads match. In each job path 0's head line equals 016's `blocks 6` from `run mirrored` through
  `inhibitory`, and the first six of `correct` and of `ties` on paths 5 and 6 equal 017's `half path <j> none blocks 12`. Each start's two jobs
  print the same three heads and the same `PIN` lines.
- From the eighth image and the assignment 42 `run` and `read` lines of the first start's two jobs stand beside the trial's and match, and so do 200 of
  their gaps' lines and 16 `kinds` and `held` lines. Their 16 reads at block 18 are of another image, the trial having made no gap there.

Reads past block 12 have no older line to stand beside.

**Under `src/`** nothing changes. `nights_run` is `shared_run_from`'s run with A's couplings and the gain read at each block's end.

## 1. What the nights leave of A

A's mean count of 64, at blocks 6, 12, 18 and 24 right after that gap and then at block 36, each list in that order:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 48.8, 45.4, 43.4, 43.8, 43.0 | 49.2, 34.1, 33.2, 32.0, 30.9 | 51.8, 42.2, 41.2, 37.3, 36.8 | 54.3, 48.9, 46.1, 45.0, 43.2 |
| Plain, A's patterns (`plain-first`) | 52.3, 50.6, 50.6, 50.2, 46.4 | 53.3, 42.3, 41.8, 40.2, 36.2 | 56.5, 47.6, 46.9, 48.0, 44.3 | 57.7, 52.5, 52.7, 50.6, 49.1 |
| Twin, the pair's patterns (`twin-second`) | 54.5, 56.4, 57.2, 54.5, 49.8 | 50.8, 32.3, 27.4, 22.6, 17.3 | 55.6, 50.0, 50.4, 50.1, 46.2 | 58.9, 55.7, 57.3, 55.8, 53.4 |
| Plain, both (`plain-both`) | 53.1, 54.0, 56.3, 56.2, 54.0 | 51.3, 38.1, 36.6, 34.5, 30.7 | 56.4, 46.1, 45.0, 44.7, 42.3 | 57.5, 53.3, 53.2, 50.5, 48.8 |

A's margin, M, in the same order:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 1.44, 1.25, 1.17, 1.12, 1.06 | 1.15, 0.40, 0.31, 0.27, 0.23 | 1.09, 0.51, 0.41, 0.29, 0.21 | 2.04, 1.54, 1.43, 1.32, 1.22 |
| Plain, A's patterns (`plain-first`) | 2.66, 3.43, 4.03, 4.51, 4.26 | 2.27, 2.51, 3.14, 3.66, 3.49 | 2.15, 2.38, 3.11, 3.65, 3.46 | 3.24, 3.49, 3.98, 4.45, 4.26 |
| Twin, the pair's patterns (`twin-second`) | 1.95, 2.18, 2.33, 2.44, 1.97 | 1.26, 0.54, 0.53, 0.36, 0.18 | 1.48, 1.19, 1.32, 1.44, 1.22 | 2.82, 2.77, 3.18, 3.55, 3.24 |
| Plain, both (`plain-both`) | 1.90, 2.18, 2.63, 2.77, 2.56 | 1.36, 0.96, 1.29, 1.40, 1.30 | 1.32, 1.05, 1.23, 1.43, 1.34 | 2.45, 2.32, 2.47, 2.64, 2.50 |

Each night less the awake row, path against path, mean over the three paths, in the same order:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Plain, A's patterns (`plain-first`) | +3.4, +5.2, +7.1, +6.4, +3.4 | +4.1, +8.2, +8.6, +8.2, +5.3 | +4.6, +5.4, +5.7, +10.7, +7.5 | +3.4, +3.6, +6.6, +5.6, +5.9 |
| Twin, the pair's patterns (`twin-second`) | +5.6, +10.9, +13.8, +10.7, +6.8 | +1.6, −1.8, −5.8, −9.4, −13.6 | +3.8, +7.7, +9.2, +12.8, +9.5 | +4.5, +6.8, +11.2, +10.8, +10.1 |
| Plain, both (`plain-both`) | +4.3, +8.6, +12.8, +12.4, +11.0 | +2.1, +4.0, +3.4, +2.4, −0.2 | +4.5, +3.9, +3.8, +7.4, +5.5 | +3.2, +4.4, +7.1, +5.5, +5.6 |

**The three readings.** Each cell is a night's mean less the awake row's on each path, averaged over the three, ± an SE. The SE keeps the
streams, as 017's did: `√((Awake's stream² + the night's stream²)/16 + (SD over the paths)²/3)`, each stream the cell's in section 2. A margin's SE
is its SD over the paths over √3. An SD over three paths has two degrees of freedom. Section 2 holds these against the pooled split.

Right after the last night, block 24, A's count:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| plain-first | +6.4 ± 1.1 | +8.2 ± 1.4 | +10.7 ± 1.3 | +5.6 ± 1.1 | +7.7 | 4, 0, 0 |
| twin-second | +10.7 ± 0.8 | −9.4 ± 1.6 | +12.8 ± 1.1 | +10.8 ± 1.1 | +6.2 | 3, 0, 1 |
| plain-both | +12.4 ± 0.9 | +2.4 ± 1.2 | +7.4 ± 1.7 | +5.5 ± 1.2 | +6.9 | 3, 1, 0 |

Twelve blocks on, block 36, A's count:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| plain-first | +3.4 ± 1.3 | +5.3 ± 0.8 | +7.5 ± 0.9 | +5.9 ± 0.6 | +5.5 | 4, 0, 0 |
| twin-second | +6.8 ± 0.8 | −13.6 ± 0.9 | +9.5 ± 1.1 | +10.1 ± 1.2 | +3.2 | 3, 0, 1 |
| plain-both | +11.0 ± 1.0 | −0.2 ± 1.1 | +5.5 ± 2.1 | +5.6 ± 1.0 | +5.5 | 3, 1, 0 |

A's margin at block 24:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| plain-first | +3.39 ± 0.04 | +3.39 ± 0.04 | +3.35 ± 0.06 | +3.13 ± 0.07 | +3.32 | 4, 0, 0 |
| twin-second | +1.33 ± 0.03 | +0.10 ± 0.02 | +1.14 ± 0.06 | +2.22 ± 0.03 | +1.20 | 4, 0, 0 |
| plain-both | +1.65 ± 0.06 | +1.13 ± 0.03 | +1.14 ± 0.05 | +1.32 ± 0.03 | +1.31 | 4, 0, 0 |

A's margin at block 36:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| plain-first | +3.20 ± 0.02 | +3.27 ± 0.04 | +3.26 ± 0.06 | +3.04 ± 0.07 | +3.19 | 4, 0, 0 |
| twin-second | +0.91 ± 0.01 | −0.05 ± 0.02 | +1.02 ± 0.09 | +2.02 ± 0.00 | +0.97 | 3, 0, 1 |
| plain-both | +1.50 ± 0.06 | +1.08 ± 0.05 | +1.13 ± 0.04 | +1.28 ± 0.03 | +1.25 | 4, 0, 0 |

A's mean at block 24 less at block 36, each form on its own row:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 0.8 | 1.1 | 0.5 | 1.8 |
| Plain, A's patterns (`plain-first`) | 3.8 | 4.0 | 3.8 | 1.5 |
| Twin, the pair's patterns (`twin-second`) | 4.7 | 5.3 | 3.9 | 2.5 |
| Plain, both (`plain-both`) | 2.2 | 3.7 | 2.4 | 1.7 |

- **Plain on A's patterns stands over the awake row in all four starts at both blocks.** The smallest is +3.4 ± 1.3. 017's one night read +0.0,
  +0.9, +4.7, +0.0 twelve blocks on, a different schedule: its night stood at block 12 of 24.
- **A's mean falls from block 24 to block 36 on every row**, by 1.5 to 5.3 on the nights and 0.5 to 1.8 on the awake row. In ten of the twelve cells a
  night falls further than its start's awake row. A's count was not read between those two blocks or past them.
- **The twin on the pair's patterns raises A in three starts and takes it down in the fourth.** From the eighth image and the mirrored assignment A's
  difference falls at every checkpoint, +1.6, −1.8, −5.8, −9.4, −13.6. The other three stand at +6.8 to +10.1 at block 36.
- **There the count fell where the whole margin did not.** At block 36 A reads 17.3 at a margin of 0.18 M, the awake row 30.9 at 0.23 M.
  Section 3 reads where that margin lies.
- **Plain on both reads most of all from the eighth image and the assignment**, 56.2 and 54.0, and 2.4 and −0.2 over the awake row from the mirrored assignment.
- **The highest count moves with the start.** At blocks 24 and 36 it is plain on both's, plain on A's patterns', and the twin's from a first mapping
  each way, in the columns' order. Plain on A's patterns has the highest margin in all four, at both blocks. Those are rankings of nights near one tick,
  and the eight gaps from the highest to the next night run 1.7 to 5.8.
- **From block 24 to block 36 the margin's difference falls** by 0.09 to 0.20 M on plain on A's patterns, 0.13 to 0.42 M on the twin and 0.01 to 0.15 M
  on plain on both.

## 2. The split, and what stands near what

**Every path reads the same sixteen streams**, so each cell splits three paths by sixteen streams, as 017's did: a count is the mean, a path, a
stream and the rest. With `P` paths and `R` streams, **a read** is the SD within a path, the variance averaged over the paths. **The rest** is the root of the
residual sum of squares over `(P − 1)(R − 1)`. **A path** is the root of the variance of the paths' means less rest²/`R`, and **a stream** the root of the
variance of the streams' means less rest²/`P`. A's count at blocks 24 and 36:

| Start | Form, block | Mean | Paths' means, 0, 5, 6 | A read | A path | A stream | The rest | Margin, M, mean and SD |
| :--- | :--- | ---: | :--- | ---: | ---: | ---: | ---: | :--- |
| Eighth, assignment | awake, 24 | 43.8 | 43.3, 44.3, 43.8 | 3.84 | −0.36 | 2.94 | 2.47 | 1.12, 0.05 |
| Eighth, mirrored | awake, 24 | 32.0 | 30.5, 33.6, 31.9 | 3.78 | 1.29 | 1.38 | 3.52 | 0.27, 0.04 |
| First mapping, assignment | awake, 24 | 37.3 | 37.3, 37.7, 36.9 | 3.35 | −0.56 | 1.98 | 2.70 | 0.29, 0.08 |
| First mapping, mirrored | awake, 24 | 45.0 | 44.4, 44.4, 46.3 | 3.10 | 0.85 | 1.09 | 2.91 | 1.32, 0.05 |
| Eighth, assignment | plain-first, 24 | 50.2 | 50.9, 50.1, 49.5 | 3.20 | 0.42 | 2.17 | 2.35 | 4.51, 0.03 |
| Eighth, mirrored | plain-first, 24 | 40.2 | 40.4, 42.3, 38.0 | 3.36 | 2.11 | 2.80 | 1.85 | 3.66, 0.09 |
| First mapping, assignment | plain-first, 24 | 48.0 | 46.0, 49.8, 48.3 | 3.40 | 1.80 | 2.56 | 2.24 | 3.65, 0.04 |
| First mapping, mirrored | plain-first, 24 | 50.6 | 51.2, 48.9, 51.7 | 3.74 | 1.42 | 3.45 | 1.46 | 4.45, 0.06 |
| Eighth, assignment | twin-second, 24 | 54.5 | 54.3, 54.9, 54.2 | 2.06 | 0.14 | 1.41 | 1.50 | 2.44, 0.06 |
| Eighth, mirrored | twin-second, 24 | 22.6 | 23.1, 25.1, 19.8 | 3.08 | 2.65 | 2.48 | 1.81 | 0.36, 0.05 |
| First mapping, assignment | twin-second, 24 | 50.1 | 49.2, 52.0, 49.1 | 3.44 | 1.54 | 2.33 | 2.53 | 1.44, 0.18 |
| First mapping, mirrored | twin-second, 24 | 55.8 | 55.8, 56.1, 55.6 | 3.06 | −0.25 | 2.71 | 1.42 | 3.55, 0.10 |
| Eighth, assignment | plain-both, 24 | 56.2 | 56.3, 56.4, 56.0 | 2.39 | −0.28 | 1.97 | 1.35 | 2.77, 0.11 |
| Eighth, mirrored | plain-both, 24 | 34.5 | 34.0, 34.2, 35.2 | 3.52 | 0.36 | 2.83 | 2.11 | 1.40, 0.09 |
| First mapping, assignment | plain-both, 24 | 44.7 | 41.9, 45.5, 46.6 | 3.64 | 2.40 | 3.11 | 1.88 | 1.43, 0.07 |
| First mapping, mirrored | plain-both, 24 | 50.5 | 49.8, 51.1, 50.6 | 4.37 | 0.42 | 3.86 | 2.05 | 2.64, 0.10 |
| Eighth, assignment | awake, 36 | 43.0 | 42.7, 43.3, 42.9 | 3.63 | −0.53 | 2.67 | 2.46 | 1.06, 0.03 |
| Eighth, mirrored | awake, 36 | 30.9 | 30.7, 31.9, 30.1 | 4.28 | 0.19 | 2.27 | 3.63 | 0.23, 0.04 |
| First mapping, assignment | awake, 36 | 36.8 | 35.7, 38.5, 36.1 | 4.56 | 1.15 | 2.32 | 3.92 | 0.21, 0.08 |
| First mapping, mirrored | awake, 36 | 43.2 | 42.5, 43.7, 43.4 | 4.12 | −0.64 | 2.03 | 3.59 | 1.22, 0.06 |
| Eighth, assignment | plain-first, 36 | 46.4 | 46.6, 46.0, 46.6 | 5.00 | −0.52 | 4.36 | 2.44 | 4.26, 0.01 |
| Eighth, mirrored | plain-first, 36 | 36.2 | 35.7, 38.4, 34.6 | 2.99 | 1.82 | 0.70 | 2.90 | 3.49, 0.10 |
| First mapping, assignment | plain-first, 36 | 44.3 | 43.4, 44.8, 44.6 | 3.63 | −0.49 | 0.27 | 3.62 | 3.46, 0.04 |
| First mapping, mirrored | plain-first, 36 | 49.1 | 48.6, 49.3, 49.5 | 3.31 | −0.56 | 1.42 | 2.99 | 4.26, 0.06 |
| Eighth, assignment | twin-second, 36 | 49.8 | 49.8, 49.4, 50.3 | 3.11 | −0.57 | 1.36 | 2.79 | 1.97, 0.02 |
| Eighth, mirrored | twin-second, 36 | 17.3 | 17.5, 18.8, 15.8 | 3.24 | 1.41 | 2.45 | 2.11 | 0.18, 0.05 |
| First mapping, assignment | twin-second, 36 | 46.2 | 46.3, 48.4, 44.0 | 3.58 | 2.06 | 1.37 | 3.31 | 1.22, 0.23 |
| First mapping, mirrored | twin-second, 36 | 53.4 | 54.6, 52.4, 53.1 | 3.09 | 0.86 | 1.24 | 2.83 | 3.24, 0.06 |
| Eighth, assignment | plain-both, 36 | 54.0 | 54.8, 54.5, 52.8 | 2.66 | 0.95 | 1.37 | 2.28 | 2.56, 0.12 |
| Eighth, mirrored | plain-both, 36 | 30.7 | 29.9, 30.8, 31.5 | 2.82 | 0.61 | 2.03 | 1.95 | 1.30, 0.10 |
| First mapping, assignment | plain-both, 36 | 42.3 | 41.9, 40.4, 44.5 | 3.87 | 1.94 | 2.72 | 2.75 | 1.34, 0.02 |
| First mapping, mirrored | plain-both, 36 | 48.8 | 46.8, 50.8, 49.0 | 3.57 | 1.83 | 1.44 | 3.26 | 2.50, 0.10 |

A cell's path or stream can come out negative, the rest standing above what the means show. It is printed with a minus, the sign of its
variance, and pooled as it came, and in an SE a negative stream counts as zero. Pooled, variances averaged:

| Cells pooled | Cells | A read | A path | A stream | The rest | Negative path, stream |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Every block | 80 | 3.41 | 1.25 | 2.41 | 2.41 | 18, 0 |
| Block 6 | 16 | 3.07 | 0.81 | 2.18 | 2.16 | 2, 0 |
| Block 12 | 16 | 3.57 | 1.25 | 2.66 | 2.38 | 4, 0 |
| Block 18 | 16 | 3.36 | 1.65 | 2.53 | 2.21 | 2, 0 |
| Block 24 | 16 | 3.37 | 1.30 | 2.55 | 2.21 | 4, 0 |
| Block 36 | 16 | 3.65 | 1.07 | 2.09 | 2.98 | 6, 0 |

The pair's count, the same way:

| Cells pooled | Cells | A read | A path | A stream | The rest | Negative path, stream |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Every block | 80 | 1.84 | 1.21 | 1.20 | 1.40 | 14, 10 |
| Block 6 | 16 | 3.07 | 2.50 | 2.01 | 2.33 | 2, 2 |
| Block 12 | 16 | 1.53 | 0.50 | 1.00 | 1.15 | 3, 1 |
| Block 18 | 16 | 1.42 | 0.63 | 1.02 | 0.99 | 1, 1 |
| Block 24 | 16 | 1.45 | 0.60 | 0.98 | 1.06 | 2, 0 |
| Block 36 | 16 | 1.05 | 0.29 | 0.41 | 0.97 | 6, 6 |

- **A's split stands where 017's did.** 3.41, 1.25, 2.41 and 2.41 here, against its 3.52, 1.20, 2.64 and 2.34, on nights and awake gaps
  it did not run.
- **Three paths a cell leave a cell's path poor.** 18 of A's 80 cells read a negative path. The pooled rows are what to read.
- **The pooled split gives another SE.** Two arms, three paths, streams not shared and the path not cancelled: 1.47 at block 24 and 1.30
  at block 36, against section 1's 0.8 to 1.7 and 0.6 to 2.1. Held to those, every count in section 1's two tables keeps its side of 2 SE.
- **How far a shared path cancels moves from block to block.** 017 held a difference's SD over the paths against what the rest alone gives, rest × √2/4.
  The median of this note's twelve cells a block is 0.65, 0.94, 1.85, 1.33 and 1.15 at blocks 6, 12, 18, 24 and 36, and the rest alone, at each block's pooled
  rest, gives 0.76, 0.84, 0.78, 0.78 and 1.05. Each cell's SD has two degrees of freedom, and they run from 0.16 to 4.09.
- **The pair's split at block 6 is the widest**, a read at 3.07, three starts reading it at 38.7 to 56.0 there. Past block 6 its read is 1.05 to 1.53.

**What stands near what.** All four forms share the head, so they stand on one tick up to block 6. An awake gap is 4 037 214 ticks. A first
night is 4 034 134 to 4 055 074, and each later one 4 057 198 to 4 058 540. The correlation of two images' sixteen counts of A, on one path, over
every pair of forms at every checkpoint, the ticks apart being the difference of their `state` ticks and each band's edges the least and greatest seen in it:

| Ticks apart, least to greatest seen | Pairs of images | Mean correlation | Least | Greatest |
| :--- | ---: | ---: | ---: | ---: |
| 0 to 15 | 74 | +0.56 | −0.23 | +0.88 |
| 16 to 126 | 61 | +0.36 | −0.64 | +0.77 |
| 136 to 477 | 27 | +0.30 | −0.16 | +0.69 |
| 574 to 706 | 18 | −0.00 | −0.41 | +0.50 |
| 2558 to 16927 | 26 | +0.06 | −0.49 | +0.46 |
| 16986 to 79554 | 154 | −0.02 | −0.68 | +0.72 |

By pair of forms at blocks 24 and 36:

| Forms | Block | Ticks apart, least to greatest | A start's correlation, mean over the paths, least to greatest |
| :--- | ---: | :--- | :--- |
| awake and plain-first | 24 | 58685 to 79177 | −0.05 to +0.11 |
| awake and twin-second | 24 | 58654 to 79401 | −0.20 to +0.18 |
| awake and plain-both | 24 | 58657 to 79173 | −0.14 to +0.28 |
| plain-first and twin-second | 24 | 3 to 224 | +0.23 to +0.51 |
| plain-first and plain-both | 24 | 0 to 46 | +0.30 to +0.71 |
| twin-second and plain-both | 24 | 0 to 230 | +0.12 to +0.59 |
| awake and plain-first | 36 | 58523 to 79077 | −0.18 to +0.06 |
| awake and twin-second | 36 | 58448 to 79554 | −0.10 to +0.30 |
| awake and plain-both | 36 | 58539 to 79082 | −0.25 to +0.22 |
| plain-first and twin-second | 36 | 2 to 477 | −0.18 to +0.38 |
| plain-first and plain-both | 36 | 0 to 221 | +0.13 to +0.68 |
| twin-second and plain-both | 36 | 8 to 477 | +0.26 to +0.52 |

- **The correlation falls with the ticks as 017's did**, from +0.56 at 0 to 15 to about 0 at 574 or more.
- **Each night against the awake row shares no stream to speak of.** Those two stand 58 448 to 79 554 ticks apart at blocks 24 and 36, and a
  start's correlation is −0.25 to +0.30, the pair's counts reading −0.33 to +0.33 the same way. So the SE above holds each arm's streams apart.
- **The three nights share part of a stream.** They stand 0 to 477 ticks apart and a start's correlation is −0.18 to +0.71. A difference of two nights is
  firmer than their two ± say, and this note gives no SE for one.
- **A form's own three paths span 0 to 1 004 ticks at block 24 and 80 to 1 570 at block 36**, greatest less least, so they share part of each stream too,
  and that is why the SE keeps the streams.

## 3. What the nights do to the pair

The pair's mean count of 64, in the same order as section 1:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 62.3, 63.1, 63.4, 63.4, 63.6 | 41.8, 60.7, 62.5, 62.1, 63.0 | 44.5, 60.1, 61.4, 62.2, 63.1 | 52.8, 60.7, 61.7, 62.2, 62.7 |
| Plain, A's patterns (`plain-first`) | 63.1, 62.6, 63.3, 61.9, 62.9 | 45.8, 63.1, 62.8, 62.6, 62.8 | 42.1, 61.7, 61.4, 61.7, 63.1 | 54.3, 61.4, 62.4, 61.6, 62.5 |
| Twin, the pair's patterns (`twin-second`) | 61.0, 58.9, 58.7, 58.1, 62.3 | 38.7, 58.8, 60.8, 62.0, 63.1 | 42.5, 60.0, 61.1, 60.5, 62.5 | 50.0, 59.5, 59.4, 59.3, 61.5 |
| Plain, both (`plain-both`) | 63.3, 63.0, 62.8, 62.2, 62.9 | 44.4, 62.7, 62.9, 63.4, 63.4 | 46.5, 62.1, 62.8, 63.1, 63.4 | 56.0, 62.1, 62.7, 62.6, 62.9 |

The pair's margin, M, from `kinds`:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 2.08, 2.36, 2.46, 2.57, 2.65 | 0.84, 2.20, 2.40, 2.48, 2.57 | 0.81, 2.05, 2.25, 2.49, 2.70 | 0.93, 1.81, 2.00, 2.15, 2.32 |
| Plain, A's patterns (`plain-first`) | 1.46, 1.16, 0.98, 0.73, 1.14 | 0.13, 0.98, 0.74, 0.30, 0.66 | 0.06, 0.97, 0.67, 0.59, 1.03 | 0.38, 0.93, 0.91, 0.90, 1.23 |
| Twin, the pair's patterns (`twin-second`) | 1.46, 1.10, 0.96, 0.83, 1.61 | 0.20, 1.15, 0.86, 0.72, 1.11 | 0.20, 1.01, 0.76, 0.56, 1.02 | 0.30, 0.78, 0.60, 0.38, 0.88 |
| Plain, both (`plain-both`) | 2.32, 2.65, 2.69, 2.85, 3.15 | 1.01, 2.51, 2.66, 2.93, 3.14 | 1.08, 2.60, 2.89, 3.10, 3.31 | 1.29, 2.27, 2.70, 3.00, 3.25 |

Each night less the awake row, its count:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Plain, A's patterns (`plain-first`) | +0.9, −0.4, −0.1, −1.5, −0.7 | +3.9, +2.4, +0.3, +0.5, −0.2 | −2.4, +1.6, −0.0, −0.5, +0.0 | +1.4, +0.7, +0.7, −0.6, −0.2 |
| Twin, the pair's patterns (`twin-second`) | −1.3, −4.1, −4.7, −5.4, −1.3 | −3.2, −2.0, −1.7, −0.1, +0.1 | −2.0, −0.1, −0.3, −1.6, −0.6 | −2.8, −1.2, −2.3, −2.9, −1.2 |
| Plain, both (`plain-both`) | +1.0, −0.1, −0.6, −1.2, −0.7 | +2.5, +2.0, +0.4, +1.3, +0.4 | +2.0, +2.0, +1.4, +0.9, +0.2 | +3.1, +1.3, +1.0, +0.5, +0.3 |

The pair's count at block 24 and block 36, then its margin at each. The SE is section 1's, with a negative stream taken as zero:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| plain-first | −1.5 ± 0.3 | +0.5 ± 0.4 | −0.5 ± 0.4 | −0.6 ± 0.6 | −0.5 | 0, 3, 1 |
| twin-second | −5.4 ± 0.6 | −0.1 ± 0.4 | −1.6 ± 0.6 | −2.9 ± 0.9 | −2.5 | 0, 1, 3 |
| plain-both | −1.2 ± 0.3 | +1.3 ± 0.3 | +0.9 ± 0.3 | +0.5 ± 0.5 | +0.4 | 2, 1, 1 |

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| plain-first | −0.7 ± 0.2 | −0.2 ± 0.4 | +0.0 ± 0.2 | −0.2 ± 0.3 | −0.3 | 0, 3, 1 |
| twin-second | −1.3 ± 0.2 | +0.1 ± 0.2 | −0.6 ± 0.2 | −1.2 ± 0.6 | −0.7 | 0, 2, 2 |
| plain-both | −0.7 ± 0.2 | +0.4 ± 0.2 | +0.2 ± 0.1 | +0.3 ± 0.2 | +0.0 | 1, 2, 1 |

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| plain-first | −1.85 ± 0.02 | −2.17 ± 0.04 | −1.90 ± 0.04 | −1.25 ± 0.08 | −1.79 | 0, 0, 4 |
| twin-second | −1.74 ± 0.07 | −1.75 ± 0.02 | −1.93 ± 0.07 | −1.77 ± 0.03 | −1.80 | 0, 0, 4 |
| plain-both | +0.28 ± 0.02 | +0.45 ± 0.03 | +0.61 ± 0.06 | +0.85 ± 0.02 | +0.55 | 4, 0, 0 |

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored | Mean of the starts | Above, within, below 2 SE |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: |
| plain-first | −1.51 ± 0.04 | −1.91 ± 0.10 | −1.67 ± 0.04 | −1.09 ± 0.09 | −1.55 | 0, 0, 4 |
| twin-second | −1.04 ± 0.03 | −1.46 ± 0.01 | −1.68 ± 0.09 | −1.45 ± 0.04 | −1.41 | 0, 0, 4 |
| plain-both | +0.50 ± 0.04 | +0.58 ± 0.07 | +0.61 ± 0.00 | +0.93 ± 0.04 | +0.65 | 4, 0, 0 |

**Its rewarded blocks.** `correct` of each form's `run` lines. Each cell lists five stretches, blocks 1 to 6 (the head), 7 to 12, 13 to 18, 19 to 24 and
25 to 36, each the mean of all its blocks over the three paths, and in brackets the mean of its first block:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 44.4 (12.3), 63.1 (62.3), 62.9 (63.3), 63.7 (63.3), 63.7 (63.3) | 20.0 (3.3), 57.2 (49.7), 62.0 (61.0), 63.0 (61.3), 63.0 (63.3) | 24.2 (7.3), 56.2 (45.3), 61.2 (60.7), 61.8 (60.0), 62.8 (61.0) | 33.8 (13.3), 58.6 (56.7), 61.3 (61.0), 62.1 (63.7), 62.7 (61.7) |
| Plain, A's patterns (`plain-first`) | 44.4 (12.3), 62.7 (63.3), 62.7 (62.7), 62.6 (62.7), 62.8 (62.7) | 20.0 (3.3), 56.7 (48.3), 62.2 (63.0), 62.7 (62.3), 62.7 (62.7) | 24.2 (7.3), 55.5 (46.0), 61.6 (62.0), 61.6 (62.0), 62.0 (61.7) | 33.8 (13.3), 57.9 (55.0), 60.8 (59.7), 61.9 (62.7), 62.2 (61.0) |
| Twin, the pair's patterns (`twin-second`) | 44.4 (12.3), 62.0 (61.7), 60.0 (60.0), 60.3 (59.3), 60.9 (59.0) | 20.0 (3.3), 53.7 (41.3), 60.3 (59.3), 61.7 (62.0), 62.4 (60.7) | 24.2 (7.3), 55.9 (48.3), 61.4 (59.3), 60.3 (60.3), 61.8 (62.0) | 33.8 (13.3), 57.5 (53.7), 60.5 (59.0), 59.8 (60.3), 61.0 (60.7) |
| Plain, both (`plain-both`) | 44.4 (12.3), 62.6 (63.7), 63.1 (62.7), 62.6 (62.7), 62.8 (63.3) | 20.0 (3.3), 56.7 (46.0), 62.6 (63.3), 62.8 (63.3), 63.3 (63.3) | 24.2 (7.3), 56.4 (49.0), 62.3 (63.0), 62.6 (63.0), 62.8 (63.0) | 33.8 (13.3), 59.5 (59.3), 61.4 (60.3), 62.5 (63.7), 63.0 (62.0) |

Those stretch means less the awake row's, for the four stretches after the head:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Plain, A's patterns (`plain-first`) | −0.3, −0.2, −1.1, −0.9 | −0.4, +0.2, −0.3, −0.3 | −0.7, +0.4, −0.2, −0.9 | −0.7, −0.5, −0.2, −0.6 |
| Twin, the pair's patterns (`twin-second`) | −1.1, −2.9, −3.3, −2.8 | −3.4, −1.7, −1.3, −0.6 | −0.3, +0.3, −1.4, −1.1 | −1.1, −0.8, −2.2, −1.7 |
| Plain, both (`plain-both`) | −0.4, +0.2, −1.1, −0.9 | −0.5, +0.6, −0.2, +0.3 | +0.3, +1.2, +0.8, −0.0 | +0.9, +0.1, +0.4, +0.3 |

The pair's first rewarded block at 40 of 64, then at 60, one number a path in the order 0, 5, 6, counted from its first block over all 36:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 3 3 3; 5 5 4 | 7 6 6; 11 8 11 | 7 8 6; 11 10 10 | 5 5 4; 9 12 10 |
| Plain, A's patterns (`plain-first`) | 3 3 3; 5 5 4 | 7 6 6; 12 9 10 | 7 7 6; 10 11 11 | 5 5 4; 10 10 12 |
| Twin, the pair's patterns (`twin-second`) | 3 3 3; 5 5 4 | 8 6 6; 12 9 11 | 7 7 6; 12 10 10 | 5 5 4; 9 12 11 |
| Plain, both (`plain-both`) | 3 3 3; 5 5 4 | 8 6 6; 9 10 10 | 7 7 6; 10 11 11 | 5 5 4; 9 9 7 |

**Where the margins lie.** Each kind's sum on the readouts, from `kinds`. Each cell lists, in M, the mean over the three paths of: A's margin on its own units
(kinds 0 and 3), on the shared units (kinds 1 and 4), the pair's margin on its own units (kinds 2 and 5), on the shared units. A cell's first two
add up to A's margin and its last two to the pair's:

| Block | Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| ---: | :--- | :--- | :--- | :--- | :--- |
| 24 | awake | 2.06, −0.95, 1.52, 1.05 | 0.92, −0.65, 1.67, 0.81 | 0.81, −0.52, 1.81, 0.68 | 1.60, −0.28, 1.76, 0.39 |
| 24 | plain-first | 3.39, 1.12, 1.50, −0.78 | 2.68, 0.98, 0.93, −0.62 | 2.69, 0.96, 1.08, −0.49 | 3.11, 1.35, 1.87, −0.97 |
| 24 | twin-second | 2.43, 0.02, 0.89, −0.06 | 0.28, 0.08, 0.55, 0.18 | 0.79, 0.64, 0.70, −0.14 | 2.60, 0.94, 1.34, −0.96 |
| 24 | plain-both | 3.11, −0.34, 2.29, 0.56 | 1.83, −0.43, 2.06, 0.87 | 1.81, −0.38, 2.19, 0.91 | 2.79, −0.14, 2.62, 0.38 |
| 36 | awake | 2.06, −1.00, 1.55, 1.10 | 0.92, −0.69, 1.71, 0.85 | 0.81, −0.61, 1.93, 0.77 | 1.60, −0.39, 1.83, 0.50 |
| 36 | plain-first | 3.39, 0.87, 1.67, −0.53 | 2.68, 0.81, 1.12, −0.46 | 2.69, 0.77, 1.33, −0.30 | 3.11, 1.15, 2.01, −0.77 |
| 36 | twin-second | 2.43, −0.46, 1.19, 0.42 | 0.28, −0.10, 0.74, 0.37 | 0.79, 0.43, 0.94, 0.08 | 2.60, 0.63, 1.52, −0.65 |
| 36 | plain-both | 3.11, −0.55, 2.39, 0.76 | 1.83, −0.53, 2.17, 0.97 | 1.81, −0.47, 2.31, 1.00 | 2.79, −0.29, 2.73, 0.52 |

- **No form left the pair unlearned.** It reads 58.1 to 63.4 at block 24, the awake row 62.1 to 63.4, and 61.5 to 63.4 at block 36, the awake
  row 62.7 to 63.6. The lowest of those are the twin's. Every form reaches 60 of 64 by block 12 on every path.
- **Its rewarded blocks.** Against the awake row a stretch's mean moves −1.1 to +0.4 on plain on A's patterns, −3.4 to +0.3 on the twin and −1.1 to +1.2 on
  plain on both. A stretch's first block moves further, by as much as −8.3 on the twin, 41.3 against 49.7 at block 7 from the eighth image and the mirrored
  assignment. The first block at 60 of 64 moves −3 to +2 on one path.
- **The twin's cost in count.** On withheld reads it stands −5.4 to −0.1 right after the fourth night and −1.3 to +0.1 twelve blocks on. On the rewarded
  blocks 25 to 36 it stands −2.8 to −0.6.
- **At block 24 on plain on A's patterns the pair's count moves by 1.5 at most and its margin falls.** It reads 61.6 to 62.6 at block 24 at a margin of 0.30 to 0.90 M, the awake
  row 62.1 to 63.4 at 2.15 to 2.57 M.
- **Plain on both raises both margins.** A's by +1.13 to +1.65 M and the pair's by +0.28 to +0.85 M at block 24.
- **Near the ceiling the tallies above are thin.** On the pair's pooled split an SE is 0.64 at block 24 and 0.34 at block 36. Held to those, three of the six
  rows of the pair's count change: plain on both at block 24 to 1, 3, 0, plain on A's patterns at block 36 to 0, 4, 0, and plain on both at block 36 to 0, 3, 1.
- **The two margins meet on the shared units.** On plain on A's patterns A's margin there is +0.96 to +1.35 M at block 24, the awake row's −0.95 to −0.28, and the pair's
  −0.97 to −0.49, the awake row's +0.39 to +1.05. On plain on both those units stay the pair's, and each side's own units rise: A's 1.81 to 3.11
  M against 0.81 to 2.06, the pair's 2.06 to 2.62 against 1.52 to 1.81.
- **A's own units do not move in the 12 blocks.** Each cell's first figure stands within 0.001 M at blocks 24 and 36. What A's margin loses there it
  loses on the shared units: −0.11 to −0.04 M on the awake row, −0.25 to −0.16 on plain on A's patterns, −0.48 to −0.19 on the twin, −0.21 to −0.09 on plain on both.
- **From the start where the twin takes A's count down, A's own units lose margin.** It is 0.28 M there against the awake row's 0.92. From the other starts the twin
  reads 2.43 against 2.06, 0.79 against 0.81 and 2.60 against 1.60. Why was not read.

## 4. Couplings, sums and gain

**A's highest coupling**, the largest of A's four over each block's end of every form's `run` lines on three paths and each image read, each coupling over
H-42's image's, on `PIN couplings`. **At the last block** is the largest at block 36's end. **Where** is the path, the stretch by its last block, the block in it, or the image read,
and the coupling in the order A onto readouts 0 and 1, B onto 0 and 1:

| Start | Form | Greatest ratio | Where | At the last block |
| :--- | :--- | ---: | :--- | ---: |
| Eighth, assignment | awake | 1.133 | path 0 run to 6 block 1, coupling 2 | 1.053 |
| Eighth, assignment | plain-first | 1.184 | path 6 run to 36 block 1, coupling 2 | 1.175 |
| Eighth, assignment | twin-second | 1.133 | path 0 run to 6 block 1, coupling 2 | 1.088 |
| Eighth, assignment | plain-both | 1.133 | path 0 run to 6 block 1, coupling 2 | 1.116 |
| Eighth, mirrored | awake | 1.144 | path 6 run to 6 block 1, coupling 0 | 1.036 |
| Eighth, mirrored | plain-first | 1.197 | path 5 run to 36 block 5, coupling 0 | 1.196 |
| Eighth, mirrored | twin-second | 1.144 | path 6 run to 6 block 1, coupling 0 | 1.033 |
| Eighth, mirrored | plain-both | 1.144 | path 6 run to 6 block 1, coupling 0 | 1.103 |
| First mapping, assignment | awake | 1.112 | path 6 run to 6 block 1, coupling 0 | 1.046 |
| First mapping, assignment | plain-first | 1.198 | path 6 read at 24, coupling 0 | 1.190 |
| First mapping, assignment | twin-second | 1.112 | path 6 run to 6 block 1, coupling 0 | 1.100 |
| First mapping, assignment | plain-both | 1.112 | path 6 run to 6 block 1, coupling 0 | 1.103 |
| First mapping, mirrored | awake | 1.114 | path 5 run to 6 block 1, coupling 2 | 1.052 |
| First mapping, mirrored | plain-first | 1.182 | path 6 read at 24, coupling 2 | 1.171 |
| First mapping, mirrored | twin-second | 1.145 | path 5 read at 24, coupling 2 | 1.130 |
| First mapping, mirrored | plain-both | 1.116 | path 5 read at 24, coupling 2 | 1.109 |

The pair's highest coupling at each image read, in M, the largest of any path's four from `kinds`. H-42's image holds no values for them, so no
ratio is printed:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 6.46, 6.51, 6.53, 6.56, 6.57 | 5.97, 6.25, 6.32, 6.34, 6.37 | 5.96, 6.23, 6.28, 6.32, 6.35 | 6.19, 6.40, 6.46, 6.45, 6.48 |
| Plain, A's patterns (`plain-first`) | 6.32, 6.21, 6.16, 6.15, 6.22 | 5.75, 5.92, 5.91, 5.90, 5.83 | 6.05, 5.95, 5.90, 5.90, 5.93 | 6.06, 6.21, 6.12, 6.14, 6.24 |
| Twin, the pair's patterns (`twin-second`) | 6.32, 6.26, 6.11, 6.17, 6.27 | 5.83, 6.06, 5.95, 5.90, 5.97 | 6.06, 5.99, 5.90, 5.82, 5.98 | 6.08, 6.16, 6.01, 5.99, 6.08 |
| Plain, both (`plain-both`) | 6.56, 6.62, 6.63, 6.68, 6.74 | 6.03, 6.34, 6.38, 6.46, 6.49 | 6.04, 6.40, 6.50, 6.56, 6.57 | 6.32, 6.53, 6.57, 6.68, 6.75 |

**The inhibitory sum**, the `inhibitory` of `first`'s stream 0, over H-42's image's 148235483 (`PIN quiet`), mean over the three paths:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 0.517, 0.518, 0.520, 0.521, 0.520 | 0.527, 0.530, 0.533, 0.535, 0.536 | 0.941, 0.921, 0.898, 0.876, 0.849 | 0.937, 0.915, 0.893, 0.871, 0.843 |
| Plain, A's patterns (`plain-first`) | 0.572, 0.606, 0.634, 0.656, 0.635 | 0.593, 0.638, 0.674, 0.703, 0.682 | 1.011, 1.034, 1.041, 1.037, 0.995 | 0.995, 1.010, 1.012, 1.006, 0.966 |
| Twin, the pair's patterns (`twin-second`) | 0.557, 0.584, 0.608, 0.632, 0.615 | 0.569, 0.601, 0.629, 0.653, 0.635 | 0.986, 0.996, 0.997, 0.991, 0.946 | 0.981, 0.991, 0.992, 0.989, 0.945 |
| Plain, both (`plain-both`) | 0.565, 0.594, 0.615, 0.633, 0.610 | 0.583, 0.621, 0.649, 0.671, 0.649 | 1.000, 1.020, 1.026, 1.024, 0.983 | 0.990, 1.004, 1.007, 1.004, 0.963 |

**The excitatory sum**, the same over H-42's image's 213566021:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 1.000, 1.000, 1.000, 1.000, 1.000 | 1.000, 1.000, 1.000, 1.000, 1.000 | 1.000, 1.000, 1.000, 1.000, 1.000 | 1.000, 1.000, 1.000, 1.000, 1.000 |
| Plain, A's patterns (`plain-first`) | 0.957, 0.928, 0.908, 0.893, 0.893 | 0.961, 0.934, 0.918, 0.907, 0.907 | 0.968, 0.943, 0.927, 0.914, 0.914 | 0.965, 0.938, 0.920, 0.906, 0.906 |
| Twin, the pair's patterns (`twin-second`) | 0.968, 0.946, 0.931, 0.922, 0.922 | 0.966, 0.944, 0.929, 0.920, 0.920 | 0.972, 0.953, 0.942, 0.934, 0.934 | 0.974, 0.955, 0.944, 0.936, 0.936 |
| Plain, both (`plain-both`) | 0.962, 0.932, 0.910, 0.893, 0.893 | 0.963, 0.936, 0.915, 0.901, 0.901 | 0.969, 0.943, 0.925, 0.911, 0.911 | 0.968, 0.942, 0.922, 0.905, 0.905 |

What each night took from it, M, `excitatory` of the gap's `start` line less its `end` line, mean over the three paths, and last the image at block 24 less at block 36:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Plain, A's patterns (`plain-first`) | 9.17, 6.30, 4.29, 3.19, 0.00 | 8.37, 5.67, 3.55, 2.20, 0.00 | 6.90, 5.36, 3.43, 2.62, 0.00 | 7.55, 5.75, 3.85, 2.96, 0.00 |
| Twin, the pair's patterns (`twin-second`) | 7.01, 4.69, 3.16, 1.92, 0.00 | 7.13, 4.75, 3.35, 1.85, 0.00 | 5.98, 4.09, 2.35, 1.71, 0.00 | 5.64, 4.03, 2.33, 1.66, 0.00 |
| Plain, both (`plain-both`) | 8.17, 6.36, 4.66, 3.70, 0.00 | 7.86, 5.86, 4.44, 3.05, 0.00 | 6.71, 5.41, 3.88, 3.12, 0.00 | 6.86, 5.48, 4.36, 3.54, 0.00 |

**The gain**, `gain` of `state`, mean over the three paths:

| Form | Eighth, assignment | Eighth, mirrored | First mapping, assignment | First mapping, mirrored |
| :--- | :--- | :--- | :--- | :--- |
| Awake (`awake`) | 113585, 113646, 113757, 113677, 111496 | 113686, 113623, 113762, 113731, 111541 | 114541, 114152, 114297, 114404, 112201 | 114518, 114148, 114287, 114370, 112193 |
| Plain, A's patterns (`plain-first`) | 107045, 107713, 108301, 108645, 111450 | 106360, 107125, 107624, 107902, 111273 | 107324, 107864, 108232, 108790, 111939 | 107984, 108459, 108786, 109361, 112015 |
| Twin, the pair's patterns (`twin-second`) | 109560, 109855, 110017, 109779, 110691 | 109707, 109832, 109985, 109791, 110605 | 110626, 110750, 110572, 110639, 111450 | 110595, 110533, 110545, 110642, 111483 |
| Plain, both (`plain-both`) | 107651, 108384, 108700, 109174, 111532 | 107458, 108104, 108298, 108688, 111388 | 108300, 108769, 108748, 109565, 112058 | 108589, 108946, 108982, 109808, 112101 |

**The nights**, all 144, from their `start` and `end` lines:

| Night at block | Nights | Ticks | Replays | Depotentiations | Gain before | Gain after |
| ---: | ---: | :--- | :--- | :--- | :--- | :--- |
| 6 | 36 | 4034134 to 4055074 | 1329 to 1340 | 640 | 111213 to 112456 | 106291 to 110629 |
| 12 | 36 | 4057412 to 4058540 | 1265 to 1268 | 601 to 604 | 111269 to 112381 | 107070 to 110762 |
| 18 | 36 | 4057530 to 4058065 | 1200 to 1202 | 571 | 111124 to 112358 | 107553 to 110608 |
| 24 | 36 | 4057198 to 4058405 | 1124 to 1129 | 545 to 549 | 110796 to 112327 | 107848 to 110674 |

**The groups.** Each `held` line sets each of six groups' sums beside the value it is held at, 240 images in all. The six values sum to 36.73 to 36.78 M.

- **A's highest coupling reads 1.198 of H-42's image's at most**, from a first mapping and the assignment on plain on A's patterns. H-46's clause 1 named 1.30.
  The awake row's 1.112 to 1.144 are all at the head's first block, and by block 36 it stands at 1.036 to 1.053. Plain on A's patterns ends at 1.171 to
  1.196, past every start's head. The twin from a first mapping and the mirrored assignment ends at 1.130, past that start's 1.114, and every other night ends under its own.
- **The groups hold.** No sum stood more than 399 from its value over the 240 images. So the 13.65 to 22.96 M the excitatory sum stands under the awake row at block 24 did not come out of them.
- **Each night takes less than the last.** On plain on A's patterns 6.90 to 9.17 M, then 2.20 to 3.19 M by the fourth.
  Depotentiations fall from 640 to 545 to 549 and replays from 1 329 to 1 340 to 1 124 to 1 129.
- **The 12 blocks after leave it where it was.** On one path a night's excitatory sum moves by −9 215 to +3 523 between blocks 24 and 36.
- **Every night raises the inhibitory sum**, all 144 by their `start` and `end` lines, and every stretch after a night lowers it. At block 24 it stands +16.58 to
  +24.85 M over the awake row, and at block 36 +13.41 to +21.66 M. In a cell's mean over the paths, as a share of H-42's image's: from the eighth image a night adds 0.029 to
  0.068 and each of the three stretches between nights takes 0.007 to 0.012, so at block 24 four of the six nights read 0.653 to 0.703 and the awake row 0.521 and 0.535. From a first
  mapping a night adds 0.015 to 0.061 and each of those stretches takes 0.017 to 0.023. There the fourth night, at 0.015 to 0.020, adds less than its stretch took, 0.020 to
  0.023, and the awake row falls from 0.937 to 0.941 at block 6 to 0.843 to 0.849 at block 36. H-46's reading 4 asked 0.45 to 0.65 at the blocks' ends of its
  schedules 9 to 16. This is another run, and neither that band nor 1.30 is applied here.
- **A night takes the gain down and the blocks bring it back.** Right after a night it reads 106 360 to 110 750 over the paths, right after an
  awake gap 113 585 to 114 541, and at block 36 all four forms read 110 605 to 112 201. So at block 24 a night's gain stands 3 727 to 5 829 under the awake row's, and at block 36
  −36 to +936 under it.

## What it costs

The harness's trial took 922 s, and the eight jobs 3 814 to 4 014 s each, 31 518 s in all. That is nine jobs, one past the 8 this note was scoped
to, the trial being the ninth, none near a job's 240 minutes and each a few minutes past the hour. Question 2 and a fourth path were cut, and
sixteen reads and the first mapping held. That is 7 680 reads on forks, and 113 in the trial.

## What it suggests

**A night's count and its margin are different readings here.** The form with the highest margin in every start has the highest count in one. From the eighth
image and the mirrored assignment A reads 17.3 on the twin at a margin of 0.18 M and 30.9 on the awake row at 0.23 M. Those margins divide differently between A's own units and the shared ones, +0.28 and −0.10 M on the
twin, +0.92 and −0.69 M on the awake row.

Questions this suggests:
- where A's count goes past block 36;
- what the density of nights changes, question 2 here;
- why the twin on the pair's patterns moves A's own units, down from one start and up from two.

## What did not run

- **Question 2**, nights every 12 blocks or every 3. Its four tests are on the branch, `what_nights_keep_by_density_…`, not run.
- **A fourth path.** Path 7 was cut. On the pooled split a fourth path narrows a difference's SE by 8 to 9 %, the streams being shared across paths.
- **Seeds 23 and 24**, so nothing here says what a seed adds.
- **Any share but 26 of 51**, an agreeing pair, or the `Kept` cut.
- **A sleep with no replay.** A night here stands against an awake gap, not against a night without its replays.
- **Any other night**: the twin on A's patterns or on both, plain on the pair's patterns, any shift but 7, any slow-wave baseline but 1/2.
- **Any read past block 36**, or a night past block 24.
- **A's count just before a gap.** Blocks 6 to 24 read after the gap. So what a stretch loses between gaps is read only by block 36.
- **A form with no gap at all**, 017's No gap. The awake row's four gaps stand in its place, and its idle ticks are part of it.
- **The pair's couplings at a block's end.** They come from `kinds`, at the images read.
- **A stream of its own on each path.** Three paths read one set of sixteen.
- **A gap on a drive of its own.** Every gap runs under the drive as it was.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_harness_of_what_nights_keep_explore` | [41365818240](https://github.com/Menteion/Menteion/actions/runs/41365818240) | `5f0bf27` | 922 |
| `what_nights_keep_awake_and_plain_of_the_eighth_image_on_seed_22_from_the_assignment_explore` | [41367828454](https://github.com/Menteion/Menteion/actions/runs/41367828454) | `1043fa5` | 4011 |
| `what_nights_keep_twin_and_both_of_the_eighth_image_on_seed_22_from_the_assignment_explore` | [41367828454](https://github.com/Menteion/Menteion/actions/runs/41367828454) | `1043fa5` | 3889 |
| `what_nights_keep_awake_and_plain_of_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [41374137568](https://github.com/Menteion/Menteion/actions/runs/41374137568) | `1043fa5` | 3989 |
| `what_nights_keep_twin_and_both_of_the_eighth_image_on_seed_22_from_the_mirrored_assignment_explore` | [41374137568](https://github.com/Menteion/Menteion/actions/runs/41374137568) | `1043fa5` | 3875 |
| `what_nights_keep_awake_and_plain_of_a_first_mapping_on_seed_22_from_the_assignment_explore` | [41374137568](https://github.com/Menteion/Menteion/actions/runs/41374137568) | `1043fa5` | 3946 |
| `what_nights_keep_twin_and_both_of_a_first_mapping_on_seed_22_from_the_assignment_explore` | [41374137568](https://github.com/Menteion/Menteion/actions/runs/41374137568) | `1043fa5` | 3980 |
| `what_nights_keep_awake_and_plain_of_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [41374137568](https://github.com/Menteion/Menteion/actions/runs/41374137568) | `1043fa5` | 3814 |
| `what_nights_keep_twin_and_both_of_a_first_mapping_on_seed_22_from_the_mirrored_assignment_explore` | [41374137568](https://github.com/Menteion/Menteion/actions/runs/41374137568) | `1043fa5` | 4014 |
