---
status: accepted
date: 2026-11-02
depends-on: ADR-0236
title: Learning that sleep keeps, measured
kind: measured
hypothesis: H-47
summary: H-47 = no at step 5, on clause 1 in two cells of twelve. On seed 23 from a first mapping and the mirrored assignment, plain nights kept 4.9 less of A than nights without replay; on seed 24 from the eighth image, 1.8 more, within 2 SE. Clauses 2 and 3 held everywhere; reading 2 missed.
decision-makers: Menteion maintainers
---

# ADR-0237: Learning that sleep keeps, measured

**In brief**
- **Decision.** H-47 reads no, against [ADR-0234](0234-learning-that-sleep-keeps.md)'s prediction, at step 5 of its stopping rule: clause 1 failed in two of the twelve cells, one on seed 23 and one on seed 24. Clauses 2 and 3 held in all twelve. Readings 1, 3, 4 and 5 held, and reading 2 missed.
- **Why.** In ten cells plain nights on A's patterns kept more of A at block 36 than nights without replay, by more than twice the paired SE. In two they did not: on one the night without replay kept more.
- **Left open.** Step 5's next decision: an ADR on what keeps A on those two networks, the replay or the rest of the night. It is named here and not taken. M11 stays open.

## Context and Problem Statement

ADR-0234 wrote H-47, [ADR-0235](0235-learning-that-sleep-keeps-built.md) built its three parts, and [ADR-0236](0236-learning-that-sleep-keeps-its-protocol.md) committed the protocol on `5e5946d` before any arm ran.
Brief 087 ran it once.
The readings' tables and the dispatches are in [H-47's report](../research/H-47.md).

## Decision Outcome

**H-47 = no, on clause 1 in two cells, step 5 of its stopping rule.**

| Network | Start | Clause 1: plain on A's less without replay at 36 | Clause 2: the pair at 36 | Clause 3: highest coupling |
| :--- | :--- | :--- | :--- | :--- |
| Seed 22 | Eighth, assignment | yes, +2.2 ± 0.65 | yes, 62.9 | yes, 1.184 |
| Seed 22 | Eighth, mirrored | yes, +19.6 ± 1.06 | yes, 62.8 | yes, 1.197 |
| Seed 22 | First mapping, assignment | yes, +7.9 ± 0.51 | yes, 63.1 | yes, 1.198 |
| Seed 22 | First mapping, mirrored | yes, +5.0 ± 1.24 | yes, 62.5 | yes, 1.182 |
| Seed 23 | Eighth, assignment | yes, +4.1 ± 1.45 | yes, 63.4 | yes, 1.171 |
| Seed 23 | Eighth, mirrored | yes, +17.4 ± 1.08 | yes, 63.1 | yes, 1.211 |
| Seed 23 | First mapping, assignment | yes, +13.4 ± 1.56 | yes, 61.9 | yes, 1.212 |
| Seed 23 | First mapping, mirrored | **no, −4.9 ± 1.09** | yes, 63.2 | yes, 1.158 |
| Seed 24 | Eighth, assignment | **no, +1.8 ± 1.79** | yes, 62.9 | yes, 1.200 |
| Seed 24 | Eighth, mirrored | yes, +11.3 ± 0.96 | yes, 63.0 | yes, 1.141 |
| Seed 24 | First mapping, assignment | yes, +3.8 ± 1.57 | yes, 61.8 | yes, 1.162 |
| Seed 24 | First mapping, mirrored | yes, +6.0 ± 1.54 | yes, 62.4 | yes, 1.223 |

Clause 1's cell is the mean of A's 48 differences ± its paired SE; clause 2's the pair's mean of 48 under plain nights on A's patterns; clause 3's A's highest coupling over H-42's image's in any row of the four forms.

**The five predicted readings:**
1. Held: plain on A's stood over the awake gap at block 36 in 11 cells of 12, the twelfth seed 24's eighth image from the assignment at −2.2.
2. Missed: plain on both stood over the night without replay in 11 of 12, and on seed 23's first mapping from the mirrored assignment it read −3.0.
3. Held: A's margin under plain on A's stood 2.64 to 4.17 M over the night without replay's in every cell.
4. Held: all 144 nights without replay raised the gain from their start to their end, and all 288 plain nights lowered it.
5. Held, at its bound: clause 1's rule at block 24 held in 10 cells of 12. It failed in the same two, at −2.6 ± 1.30 and −5.9 ± 1.85.

**The stopping rule** reaches step 5, a no on clause 1, since no clause 3 failed.
Its next decision is an ADR on what keeps A on those networks, the replay or the rest of the night; this ADR takes none.
Under step 8 no constant moves, and there is no second attempt.

**Seed 22 reproduces the explorations.** Its four cells read exploration 019's +2.2, +19.6, +7.9 and +5.0, and the pair 62.5 to 63.1, as ADR-0234 expected.

**The two cells that failed.**
- **Seed 23, first mapping, mirrored**: the night without replay held A at 53.1 of 64 at block 36, from 55.0 at block 6. Plain nights on A's patterns fell to 48.3, and the awake gap to 43.6. A replay of A cost A there.
- **Seed 24, eighth image, assignment**: A fell under every form, to 25.6 without replay, 27.4 plain on A's, 29.6 plain on both and 29.6 awake. Plain on A's read below the night without replay at blocks 18 and 24, by −5.9 at 24.
- In both the margin under plain nights on A's still stood 2.88 and 3.87 M over the night without replay's: the couplings favoured A and the reads did not.

**The calibration and the twin** held before any job's log was read.
- Dispatch [41441229981](https://github.com/Menteion/Menteion/actions/runs/41441229981) at `scope=exhaustive` on `5e5946d` ran the tree's other 227 tests in 18 shards with the three parts unset, each green, in 39 to 67 minutes of the 240-minute bound.
- The twin, in dispatch [41441234613](https://github.com/Menteion/Menteion/actions/runs/41441234613) on `5e5946d`, held all 17 groups of its lines to explorations 018's and 019's logs, in 533 s.

**The evidence.**
- The 24 jobs ran at `scope=arms` on AArch64, dispatch 41441234613 on `5e5946d`, in 3 670 to 4 049 s, 25.5 % to 28.1 % of the bound.
- The tables are pinned in `6b31a80`, pull request #271. From `5e5946d` to it only `tests/inhibition/pins/h47.rs` changed: no file under `src/`, and no test or helper the calibration ran.
- The reproduction ran on `6b31a80`. Dispatch [41468250551](https://github.com/Menteion/Menteion/actions/runs/41468250551) at `scope=exhaustive`, `only` the twelve short forms, was green on x86-64, each holding its cell's first plain night on path 0, in 130 to 374 s, 2 936 s together. Dispatch [41468257319](https://github.com/Menteion/Menteion/actions/runs/41468257319) at `scope=arms` ran the 24 jobs again on AArch64, each green in 3 719 to 4 118 s, at most 28.6 % of the bound, every pinned row held by its own job.
- The cost table is written from the calibration: 227 tests. The twelve short forms ran alone, so they have no line and keep the default of 2 400 s until a full deal measures them ([ADR-0220](0220-the-cost-of-a-test-run-alone.md)).
- No commit under `runtime/`, `crates/` or `benches/` follows `6b31a80`.

**The sweep.** The schedule's run of 2026-11-04 at 20:17Z had not begun when this was written, on 2026-11-02. The first ADR that merges after it ends records it.

## Consequences

- Good: on seed 22 the comparison is the explorations' to the read, and on seeds 23 and 24 six cells of eight read it the same way.
- Good: no coupling passed 1.30, and the pair learned under every night, so M12's bound held through four nights a form.
- Bad: M11's exit test does not read yes, and the two cells that fail are one of each new network's.
- Bad: reading 5 held at its bound, so what a night gives right after it is barely above the rule in ten cells.
- Neutral: the twelve short forms join the weekly job.

## Confirmation

- `the_nights_of_h_47_their_clauses_and_their_predicted_readings_rules` reads the verdict, its step and the five readings over `pins/h47.rs` in the gate.
- The reproduction's dispatches hold every pinned row.
- `npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
