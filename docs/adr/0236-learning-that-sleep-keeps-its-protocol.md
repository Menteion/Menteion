---
status: accepted
date: 2026-11-02
depends-on: ADR-0235
title: Learning that sleep keeps, its protocol
kind: protocol
hypothesis: H-47
summary: H-47's protocol, committed before any arm runs. Twelve cells in 24 jobs, a row an image pinned from their logs, a twin of explorations 018 and 019 held to their logs by hash, and a short form of each cell on the weekly job.
decision-makers: Menteion maintainers
---

# ADR-0236: Learning that sleep keeps, its protocol

**In brief**
- **Decision.** H-47 runs as `nights_job` in `tests/inhibition.rs`: twelve cells, a network and a start, each in two `…_paused` jobs of two forms on three paths. Its clauses, constants and five predicted readings are tests' constants and rules from this commit on, and its twin holds the harness to the explorations' logs.
- **Why.** [ADR-0234](0234-learning-that-sleep-keeps.md) and brief 087 require them committed before any arm runs, and fixed after it.
- **Left open.** The verdict, the readings and the next decision: [H-47's report](../research/H-47.md) and the measured ADR, after the one run.

## Context and Problem Statement

ADR-0234 wrote H-47 on 2026-11-02 before any run, and [ADR-0235](0235-learning-that-sleep-keeps-built.md) built its three parts.
Brief 087 leaves the round to build the harness from the tags' code with the twin of step 2 held, to choose each short form's length, and to split a cell's two jobs further if the bound asks.

Explorations 014 to 019 ran this path on seed 22, on the tags `explore/014-what-a-night-keeps` to `explore/019-a-night-without-replay`, and were not reviewed. Their harness read far more than H-47 asks. Here it is written again, narrower: the night, the awake gap, the pair, the paths, the reads and the four forms, line for line where a line is the twin's.

## Decision Drivers

- **Nothing moves after a run**: clauses, constants and readings are code, held by the gate's test.
- **The twin decides whether the harness is the explorations'**: ADR-0234's step 2 stops the round on any line that is not their logs, the label aside.
- **One run each**, at `scope=arms`: two forms of exploration 018 took 3 814 to 4 014 s a job on AArch64.
- **An image is held bit for bit**, or a form's 36 blocks could drift unseen between its reads.

## Considered Options

1. **The engine**: (a) `Config::runs_a_settled_image()`, its kept addresses unset, and the slow-wave baseline at 1/2 on every engine; (b) the tags' `night_engine`, H-25's harness configuration with two addresses kept.
2. **The twin's check**: (a) a test that reads its own dump lines back and holds each group's count and hash to the explorations' logs; (b) a script outside the tree over its log.
3. **What is pinned**: (a) a row an image, its CRC-64, its 32 counts, A's couplings, their highest at the stretch's block ends and the gap's gains; (b) every read's line.
4. **The short form**: (a) path 0's head and the plain night after it, its image and the read as it was of A and of the pair; (b) a whole form on one path.
5. **A trial of the harness**: (a) the twin, run beside the jobs; (b) a trial dispatched before this commit.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).**
Committed before any dispatch ran an arm, in brief 087's pull request.

- *Why not 1(b)*: ADR-0234 runs H-47 under the running configuration. That the name is the explorations' engine is what the twin reads (brief 087, Context 9).
- *Why not 2(b)*: a check outside the tree is held by nobody at the merge.
- *Why not 3(b)*: 23 040 lines to hold what a row's CRC-64 and counts hold. The lines stay in each job's log.
- *Why not 4(b)*: a form on one path runs 36 blocks, four gaps and 160 reads where (a) runs 6 blocks, one gap and two reads, and the twelve share 3 600 s of the weekly job.
- *Why not 5(b)*: brief 087 runs no derivation beyond its dispatches, and the twin runs the same code from the same eighth image.

### The run

`nights_job(n, s, forms)`, one body for the 24 jobs; `with_nights` lays a start out:
1. **The calibration of the network**: `probe_made`, its derivation run again and held to H-42's pinned tables, in every job.
2. **The groups**: H-46's (`zero_groups`), each held value read from H-42's image and held to H-42's pinned couplings; the pair at the share (`shared_pair`); and the zero sum's six groups, each kind of unit apart at its sum in the start image (`shared_groups`).
3. **The start image** (`nights_image`): H-46's eighth image, held to its CRC-64; or a first mapping, `zero_schedule` over `FLIP`'s 1 536 trials from H-42's image, its CRC-64 dumped and held where it is pinned.
4. **Every engine** (`night_engine`): decoded through `zero_from`, then again under `Config::runs_a_settled_image()` at H-46's sizing, the slow-wave baseline at 1/2, no address kept, and the six groups set. An image takes no group and no baseline in slow-wave sleep with it.
5. **On each path**: the pair's 6 blocks from the start image (`nights_run`, the head, run once a path), the form's gap (`nights_gap`), 6 more blocks and a gap to the gap at block 24, then 12 blocks with none. Every stretch starts from the path's first index again.
6. **A night** (`night_on`): an engine with room for its patterns in slow-wave sleep at shift 7 and a pressure of 1.0, every older episode spent, its patterns tagged at priority 200 or none, run under the drive to the wake, then quiet. The image it leaves is decoded with no room, its shift at zero, and written again.
7. **The reads** (`nights_reads`): each checkpoint's image, its kinds' and groups' sums dumped (`nights_state`), then sixteen reads of A and sixteen of the pair, two at a time, each a block with the reward withheld on a fork (`spread_read`).
8. **The rows**, dumped as `PIN row` and held where they are pinned.

### H-47's constants, restated from ADR-0234 and pinned in the tests

| Constant | Value | In the tests |
| :--- | :--- | :--- |
| The networks | seeds 22, 23 and 24 at 1 024 units, H-46's | `NIGHTS_SEEDS`, `SCAN_UNITS` |
| The starts | the eighth image and a first mapping, each from both arms | `NIGHTS_STARTS`, `NightFrom` |
| The forms, a job's two | without replay with plain on A's, then plain on both with awake | `NIGHTS_GAPS`, `NIGHTS_JOBS` |
| The gaps | every 6 blocks to block 24, then 12 with none | `NIGHTS_EVERY`, `NIGHTS_LAST`, `NIGHTS_ON` |
| The checkpoints | blocks 6, 12, 18, 24 and 36 | `NIGHTS_CHECKPOINTS` |
| The paths | 0, 5 and 6; trial `4 096 j`, the drive's seed plus `j` past 4 | `NIGHTS_PATHS`, `PATH_TRIALS`, `PATHS_ON_ONE_DRIVE`, `path_from` |
| The reads | 16 a mapping: as it was, and `Both` 1 to 15, trial `64 i`, seed plus `i` | `NIGHTS_READS`, `nights_read_from` |
| The night | shift 7, pressure 1.0, slow-wave baseline 1/2, priority 200 | `NIGHT_SHIFT`, `NIGHT_SLOW_WAVE_Q16`, `NIGHT_PRIORITY` |
| The awake gap | 4 037 214 ticks under the drive | `NIGHT_AWAKE_TICKS` |
| A night's bound | 512 windows, 32 768 ripples of 2 048 ticks | `NIGHT_RIPPLES_MAX`, `NIGHT_RIPPLE` |
| The pair | places 5 and 16; the periods `j` with bit `j & 3` of `0b0101`, 26 of 51 | `SECOND_C_OFFSET`, `SECOND_D_OFFSET`, `NIGHTS_SHARE` |
| A's patterns | from the start image; the pair's from the image the night follows | `mapping_patterns`, `nights_gap` |
| The task | H-46's shape, cancel and drawn delivery; runs rewarded, reads withheld | `task_over`, `SHAPE_F46`, `DRAWN_DELIVERY` |
| The prediction | yes, and each reading | `NIGHTS_PREDICTED`, `NIGHTS_READINGS_PREDICTED` |
| Clause 2's floor | 56 of 64 | `NIGHTS_PAIR_FLOOR` |
| Readings 1 and 5 | 10 of 12 cells | `NIGHTS_READING_CELLS` |
| Reading 3 | 2 M | `NIGHTS_MARGIN_OVER` |

`NIGHT_CAPTURE_TICKS`, 512, is read by a night's window lines alone, as exploration 014's.

### Its terms, in the code

| Term of [H-47's report](../research/H-47.md#its-terms) | Where |
| :--- | :--- |
| The starts, A, the pair, the gaps, the four forms | `NIGHTS_STARTS`, `nights_mirrored`, `shared_pair`, `nights_gap`, `night_on`, `idle_on` |
| A read, the paths | `spread_read`, `nights_read_from`, `path_from` |
| A mean count | `nights_counts`, 48 a mapping, a form and a checkpoint |
| The split and the paired SE | `nights_split`, `nights_above`, below |
| A's four couplings | `couplings` over A's sets, `c0` to `c3` as A→R0, A→R1, B→R0, B→R1 |
| A's margin | `nights_margin`, negated where the mapping in force is mirrored |

**The split** of 48 values $x_{pr}$, three paths by sixteen streams: a value is the mean, a path, a stream and the rest. The rest's variance is the residual sum of squares over $2 \cdot 15$. A stream's is the variance of the streams' means less the rest's over 3, and a path's that of the paths' means less the rest's over 16, each over $n - 1$.

**Clause 1's rule in integers.** With $S$ the sum, $S_p$ a path's and $S_r$ a stream's, $A = 6\sum_r (16 S_r - S)^2 - \sum (48 x - 3 S_p - 16 S_r + S)^2$ is $90 \cdot 48^2$ times the stream's variance, and $B = 45 \sum_p (3 S_p - S)^2$ the same multiple of the paths' means' variance. The mean is above twice $\sqrt{s^2/16 + d^2/3}$ exactly when $S > 0$ and $1080 S^2 > 3 \max(A, 0) + 16 B$. On exploration 019's first mapping from the mirrored assignment at block 24 the gate holds the terms a script computed, 3 372 768, 2 488 590 and 226, which give +4.7 ± 2.24.

### H-47's clauses and verdict

| Clause | Rule (`nights_clauses`) |
| :--- | :--- |
| 1 | A's 48 differences at block 36, plain on A's less without replay, above twice their paired SE (`nights_above`) |
| 2 | the pair's 48 counts at block 36 under plain on A's summing to at least $56 \cdot 48$ |
| 3 | in every row of the four forms, no coupling past 1.30 of H-42's image's (`over_bound`), in the image and at the stretch's highest (`nights_bounded`) |

- **The verdict** (`nights_verdict`): yes when the three hold in all twelve cells.
- **The stopping rule's step** (`nights_stop`), the first that applies: 3 for yes; 4 for clause 3; 5 for clause 1; 6 for clause 2.

### The five predicted readings as rules (`nights_predicted`)

| Reading | Rule |
| :--- | :--- |
| 1. A over the awake gap | at block 36, the sum of plain on A's less awake above zero in at least 10 cells |
| 2. Both replayed | at block 36, plain on both less without replay above zero in every cell |
| 3. A's margin | at block 36, A's margin summed over the paths under plain on A's at least 6 M over without replay's, in every cell |
| 4. The gain | every night without replay's end gain above its start's, every plain night's below, every path and gap (`nights_gains`) |
| 5. Right after the last night | clause 1's rule at block 24 holding in at least 10 cells |

None is read until every row is pinned.

### An image's row

`NightsRow`, in `tests/inhibition/pins/h47.rs`, one an image, fifteen a form, path by path: the image's CRC-64; A's and the pair's sixteen counts; A's four couplings in the image; each one's highest at the block ends of the stretch before it; and the gap's gain at its start and end, none at block 36. `NIGHTS_FIRST_CRC` holds each first mapping's image. A form's rows are pinned whole or not at all, and every form's highest at block 6 on a path is the same, since the head is one run.

### The twin

`the_twin_of_explorations_018_and_019_on_seed_22_from_the_eighth_image_and_the_assignment_paused` runs `nights_twin`: on seed 22 from the eighth image and the assignment, on path 0, the head, the plain nights on A's patterns at blocks 6 and 12 with the blocks between, and the first night without replay, each image's two lines and its 32 reads.

- **It reads its own lines**: the file's `eprintln!` prints what `std::eprintln!` prints and, while the twin listens, keeps the line.
- **Its groups** (`twin_line`, `twin_groups`): the derivation's 22 `PIN` lines, the groups' two lines, the head's and blocks 7 to 12's run lines, each night's `start`, 31 `window` and `end` lines, and each image's `kinds` and `held` lines and its reads of each mapping. Each group's lines are read from their marker on, sorted and hashed.
- **Its pins** (`NIGHTS_TWIN`): seventeen groups' counts and FNV-1a hashes from exploration 018's run [41367828454](https://github.com/Menteion/Menteion/actions/runs/41367828454) (job `arm-0`) and exploration 019's run [41408442255](https://github.com/Menteion/Menteion/actions/runs/41408442255) (job `arm-0`). A reference and not evidence ([ADR-0193](0193-an-exploration-lane.md)): the two logs agree on the four groups both hold.

### The tests and the dispatches

- **The gate's one test**, `the_nights_of_h_47_their_clauses_and_their_predicted_readings_rules`: the constants above; clause 1's rule at its edges and on exploration 019's terms; the clauses, the steps and the readings at their edges over tables written by hand; the twin's reader; `nights_tables` over the pinned tables, empty until the runs.
- **The 24 jobs**, `…_paused` ([ADR-0127](0127-the-paused-line-leaves-the-weekly.md)): `nights_with_and_without_replay_` and `nights_on_both_mappings_and_awake_`, then `{of_the_eighth_image,of_a_first_mapping}_on_seed_{22,23,24}_from_the_{assignment,mirrored_assignment}_paused`.
- **The twelve short forms**: `a_plain_night_after_the_pair_s_head_{of_the_eighth_image,of_a_first_mapping}_on_seed_{22,23,24}_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive` (`nights_short`), held to the plain night's row on path 0 at block 6.
- **The order** ([briefs/README.md](../../briefs/README.md#evidencing-a-round)):
  1. on the commit that carries this ADR, the calibration at `scope=exhaustive` with the twelve short forms excluded, and the 24 jobs and the twin at `scope=arms`, dispatched together;
  2. the calibration's shards and the twin read first; no job's log is read before all are green;
  3. the rows pinned from the 24 logs;
  4. as soon as those are pushed, the twelve short forms at `scope=exhaustive` with `only`, and the 24 jobs again at `scope=arms`, every pinned row held.

### What `npm run round:additive` read on this commit

Exit 1: the change does not only add, so the calibration runs every test but the round's own ([ADR-0215](0215-the-calibration-a-change-needs.md)).

```text
runtime/menteion-runtime/src/executor.rs: a file a Rust build reads, outside a package's tests/
runtime/menteion-runtime/src/task.rs: a file a Rust build reads, outside a package's tests/
runtime/menteion-runtime/tests/contention.rs:58: an insertion inside an item, or after a line that does not end one
runtime/menteion-runtime/tests/differential.rs:375: an insertion inside an item, or after a line that does not end one
runtime/menteion-runtime/tests/inhibition.rs:220: an insertion inside an item, or after a line that does not end one
runtime/menteion-runtime/tests/inhibition.rs:43748: an insertion inside an item, or after a line that does not end one
runtime/menteion-runtime/tests/inhibition.rs:43785: an insertion inside an item, or after a line that does not end one
runtime/menteion-runtime/tests/inhibition.rs:44672: an insertion with a `use`, which can shadow a glob import
runtime/menteion-runtime/tests/instrument/harness.rs:666: an insertion inside an item, or after a line that does not end one
runtime/menteion-runtime/tests/instrument/harness.rs:723: 1 line(s) of the base removed or changed
runtime/menteion-runtime/tests/learning.rs:59: 1 line(s) of the base removed or changed
runtime/menteion-runtime/tests/sleep.rs:24: 1 line(s) of the base removed or changed
runtime/menteion-runtime/tests/sleep.rs:520: an item named `Held`, a name the base already uses
runtime/menteion-runtime/tests/sleep.rs:520: an item named `driven`, a name the base already uses
check-additive: 754c966...<this commit> does not only add, 14 reason(s): the calibration runs every test (ADR-0215)
```

### What does not move after a run

The configuration, the slow-wave baseline, the shift, the priority, the pair, its share and its groups, the forms, the paths, the reads, the checkpoints, the networks, the starts, the clauses, the constants and the five readings.
There is no second attempt.

## Consequences

- Good: an image's row holds a form bit for bit at each checkpoint, and its counts are what every reading reads.
- Good: the twin reads the derivation, the groups, the runs, the nights and the reads together, so a harness that is not the explorations' stops the round before any job's log is read.
- Bad: 24 jobs of about 65 minutes on AArch64, and the same again to reproduce them; every job runs its network's derivation again, and a first mapping's job its 24 blocks.
- Bad: every dump line of the binary is formatted once more, for the twin.
- Neutral: the harness's `run_on_scheduled_over` runs through `run_on_presenting_from`, from index 0 with the sets, as it ran.

## Confirmation

- `cargo test --workspace --locked` runs the gate's test.
- The calibration, the twin and the 24 jobs on this commit, as above.
- `npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
