---
status: proposed
date: 2026-11-02
title: Learning that sleep keeps, a night with replay against one without
kind: decision
hypothesis: H-47
summary: The replay stays plain, scaled by the gain, and the documents give the value that arrives. A slow-wave baseline, the ledger spent and a trial's own units enter unset. H-47 asks on three networks whether nights with replay keep a mapping a conflicting one takes, against nights without.
depends-on: ADR-0232
decision-makers: Menteion maintainers
---

# ADR-0234: Learning that sleep keeps, a night with replay against one without

**In brief**
- **Decision.** The replay stays as the engine runs it, its drive scaled by the tick's gain, and the documents give the value that arrives: 4.375 at a gain of 1.75 (F-47). Three parts enter unset: a baseline for the excitatory synapses in slow-wave sleep, the ledger spent on a call, and a trial that presents the units the host gives. H-47 asks, on three networks, whether four nights with replay keep a mapping that a conflicting second one takes, against four nights without, predicted yes.
- **Why.** M11's exit test is a learned mapping held through a sleep with replay, against one without. Explorations 014 to 019 read it on seed 22: at H-25's baseline of zero no replay reaches an excitatory synapse, and at a slow-wave baseline of 1/2 plain nights on A's patterns stood over nights without replay in all four starts.
- **Left open.** Whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04), which no exploration read; F-47, until brief 087 rewrites the documents.

## Context and Problem Statement

[Appendix C](../WHITEPAPER.md#appendix-c-roadmap)'s M11 asks for a learned mapping held through a sleep with replay, against one without. [ADR-0232](0232-a-running-configuration-and-m12-closed.md) opened it with an exploration. Explorations 014 to 019 read it at 1 024 units, on seed 22 alone, from H-46's arm. They are exploratory, not evidence:
- **What fades, and what a replay reaches** ([014](../explorations/014-what-a-night-keeps.md)): a mapping does not fade under the drive, 60 to 64 of 64 through 960 windows. Under H-25's configuration the excitatory baseline is zero, so 5 363 replays moved no excitatory synapse. With a slow-wave baseline of 1/2 on its branch, plain replay raised A's margin, and the twin, its drive divided by the gain, lowered it.
- **What takes a mapping** ([015](../explorations/015-what-the-other-mapping-takes.md), [016](../explorations/016-what-a-shared-source-takes.md)): a second mapping with synapses of its own took nothing from A. One sharing 26 of A's 51 source units and conflicting took A by the share, to 42, 27, 33 and 50 of 64 at block 24 in four starts.
- **The noise** ([017](../explorations/017-what-a-read-spreads.md)): one read of A's count is one draw, its SD 3.52, and a path's SD is 1.20. A difference of 8 of 64 stands at 3 SE on eight reads on one path.
- **Four nights** ([018](../explorations/018-what-nights-keep.md)): a gap at blocks 6, 12, 18 and 24 of the pair, then 12 blocks with none. Plain nights on A's patterns stood over the awake row by +3.4, +5.3, +7.5 and +5.9 at block 36, and the pair read 61.5 to 63.4. A's highest coupling read 1.198 of H-42's image's.
- **A night without replay** ([019](../explorations/019-a-night-without-replay.md)): 018's plain night with no episode tagged. All 48 such nights replayed nothing, and each woke on its plain night's tick. Against the awake row it read +1.2, −14.4, −0.4 and +0.9 at block 36. The plain night stood over it by +2.2, +19.6, +7.9 and +5.0, each above 2 SE.

**What the engine holds today**: ADR-0038's replay, two messages of 1.25 into each unit of an episode, scaled by the tick's gain where they arrive (`scaled`, F-47). The modulation's baseline is the configuration's in every stage. No call spends the ledger, and a stimulus presents its own set alone.

## Decision Drivers

- **The maintainers' choices** (2026-11-02): exploration 019 before deciding; the replay plain, the twin kept in the explorations' tags; H-47's arm on A's patterns, plain on both a reading; the exit test read at block 36.
- **M11's exit test** is a hypothesis written before any run reading yes.
- **Latest ≠ Newest**: integer rules of the engine's own, and no dependency.
- **Principle 5**: each new part, unset, is the engine as it is, held by property tests over the lattice.
- **The cost**: one form took 2 103 to 2 357 s a start on AArch64 in exploration 019, and two forms 3 814 to 4 014 s in 018.

## Considered Options

1. **F-47's replay**: (a) plain, scaled by the gain, the documents giving the value that arrives; (b) the twin, its drive divided by the gain, in the engine.
2. **The slow-wave baseline**: (a) a configuration's, unset by default, 1/2 in H-47; (b) set in the running configuration; (c) none, H-25's zero kept.
3. **H-47's arm**: (a) plain nights on A's patterns, plain on both a reading; (b) plain on both; (c) the twin on the pair's patterns.
4. **Its control**: (a) the night without replay, the awake gap a reading; (b) the awake gap.
5. **Its reading**: (a) block 36, twelve blocks after the last night, block 24 a reading; (b) block 24.
6. **The networks**: (a) seeds 22, 23 and 24; (b) seed 22.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a) and 6(a).**

### The replay (Specified; brief 087 writes it down)

- **The replay is ADR-0038's, unchanged.** Its two messages of 1.25 are scaled by the tick's gain where they arrive, as every injected message is. At the reference network's gain of 1.75 a unit of the episode receives 4.375.
- **Brief 087 rewrites the documents** that give the replay's drive as 2.5 to the value that arrives, and its built ADR resolves F-47.
- **The twin enters no engine.** It stays on the tags `explore/014-what-a-night-keeps` to `explore/019-a-night-without-replay`.

### The slow-wave baseline (Specified; brief 087 builds it)

- **A configuration's** (`Config::slow_wave_baseline_q16`, the name brief 087's to change): while set, a tick in slow-wave sleep consolidates the excitatory synapses under it in the modulation baseline's place. Every other stage keeps the configuration's baseline.
- **Unset, every run is the run it was**, bit for bit. It is in no named configuration and not in the image, and the format stays 26.
- **H-47 sets it to 1/2**, exploration 014's value.

### The ledger spent, and a trial's own units (Specified; brief 087 builds them)

- **The ledger spent** (`Executor::spend_episodes`, the name brief 087's): between ticks, every episode is marked spent, so a night replays only what is tagged after the call.
- **A trial that presents the units the host gives** (`Task::trial_presenting`, the name brief 087's): the stimulus's messages and its cancel go to those units in place of its set. Given each set's own units, a trial is the trial it was.
- **The explorations' forms of both** are on the tags and were not reviewed. Brief 087 builds them with property tests over the lattice.

### The runs (Specified; brief 087 builds them in the tests)

On each of seeds 22, 23 and 24 at 1 024 units, under `Config::runs_a_settled_image()` with its groups set and the slow-wave baseline at 1/2, in H-25's learning configuration:
- **Four starts a network**, as exploration 016 drew them: H-46's eighth image in each of H-25's two arms, and a first mapping learned from H-42's image in each arm. A is the mapping in force there.
- **The pair**: a second mapping whose stimuli share 26 of A's 51 source units and conflict with A, each kind of unit held apart. It runs 24 blocks with a gap at blocks 6, 12, 18 and 24, then 12 blocks with none.
- **Four forms of the gap**: a night without replay; a plain night on A's patterns; a plain night on both mappings' patterns; and 4 037 214 idle ticks awake. Each night is exploration 018's: shift 7, slow-wave sleep at a pressure of 1.0, the drive on, the groups held, every older episode spent.
- **Three paths**, 0, 5 and 6, and sixteen reads of A and of the pair at each checkpoint, on forks: the read as it was and fifteen by `Both`.
- **Two jobs a cell**, a network and a start: the night without replay with the plain night on A's patterns, then plain on both with the awake gap. That is 24 `…_paused` jobs of about 65 minutes on AArch64.
- **A short form of each cell** is a weekly `…_exhaustive` test, the twelve together at most 3 600 s in the cost table.
- **The reproduction**: the short forms at `scope=exhaustive` on x86-64, and the 24 long jobs again at `scope=arms` on the reproduction's commit, every pinned table held, as ADR-0226 reproduced H-46's.

### H-47 (Hypothesis; written before any run)

The text, with its clauses, readings and stopping rule, is [H-47's report](../research/H-47.md).
- **Clause 1**: in each of the twelve cells, A's mean count at block 36 after plain nights on A's patterns exceeds that after nights without replay by more than twice its paired SE.
- **Clause 2**: in each cell, the pair's mean count at block 36 after plain nights on A's patterns is at least 56 of 64.
- **Clause 3**: in each cell, under plain nights on A's patterns, none of A's four couplings passes 1.30 of H-42's image's on its network, at any block's end or image read.
- **Yes** when all three hold; otherwise **no**, with the network, the start and the clause.

**A mean count** is of `correct` over an image's 48 reads, three paths by sixteen. **The paired SE** is exploration 019's: on each path the two forms' means differ, and the SE is $\sqrt{s^2/16 + d^2/3}$. Here $s$ is the stream's SD in the split of the 48 differences, a negative variance as zero, and $d$ the paths' SD.

**Predicted: yes.** On seed 22 the plain night stood over the night without replay by +2.2 ± 0.6 to +19.6 ± 1.1 at block 36, the pair read 62.5 to 63.1 under it, and A's highest coupling 1.198.

**Five readings are predicted:**
1. A's mean count at block 36 after plain nights on A's patterns over the awake gap's, positive in at least 10 of the 12 cells;
2. after plain nights on both over nights without replay at block 36, positive in every cell;
3. A's margin at block 36 after plain nights on A's patterns at least 2 M over that after nights without replay, in every cell;
4. every night without replay raises the gain, and every plain night lowers it;
5. clause 1's difference at block 24 above twice its paired SE in at least 10 of the 12 cells.

**What would refute it:**
- a network on which nights without replay keep A as plain nights do;
- the pair unlearned under nights that replay A;
- a coupling past 1.30 under the night, which ADR-0232 names a finding that reopens M12.

### H-47's stopping rule

1. **One round**: brief 087 builds the three parts with their tests and ADR, then runs H-47 once. Its clauses, constants and readings are committed before any arm runs.
2. **A calibration stops the round**, and is a finding, if a pinned number moves with the three parts unset. So does a twin of exploration 019 on seed 22 that is not its log: the eighth image from the assignment, path 0, the head and the first night without replay with its reads.
3. **Yes**: M11's exit test reads yes. The next decision is whether the slow-wave baseline enters the running configuration, and whether M11 closes.
4. **No on clause 3**: M12 reopens ([ADR-0232](0232-a-running-configuration-and-m12-closed.md)), and an ADR on the zero sum through a night.
5. **No on clause 1**: an ADR on what keeps A on that network, the replay or the rest of the night.
6. **No on clause 2**: an ADR on what a replay of A costs the pair.
7. **Where several steps apply**, the first in this order is taken.
8. **No constant moves after such a run.** There is no second attempt, and no other baseline, shift, form, path or network.

### F-47, and the sweep

- **F-47 stays open** until brief 087 rewrites the documents to the value that arrives. Its built ADR resolves it.
- **The schedule's run of 2026-11-04 at 20:17Z** has not begun when this is written. The first ADR that merges after it ends records it.

## Consequences

- **Good**: M11's exit test is read by the comparison it names, a night with replay against one without, on three networks.
- **Good**: unset, the three parts are the engine as it is, and no format moves.
- **Good**: the documents give the replay's drive as it arrives, and F-47's last open part closes.
- **Bad**: the modulation gains a baseline that depends on the sleep stage, outside the image. An engine decoded from an image takes it from its configuration.
- **Bad**: the round runs 24 jobs of about 65 minutes on AArch64, and the same again to reproduce them.
- **Bad**: the weekly job gains twelve short forms, at most 3 600 s together.
- **Neutral**: the long runs stay in the tree, runnable by name.

## Alternatives considered and why rejected

- **Option 1(b), the twin**: on the pair's patterns it read −13.6 against the awake row in one start of four at block 36, and on A's patterns no exploration ran it.
- **Option 2(b), the running configuration**: it would set a value before any hypothesis read it on more than one network.
- **Option 2(c), none**: at H-25's zero no replay reaches an excitatory synapse, so a night could keep nothing by replay.
- **Option 3(b), plain on both**: it replays the pair as well as A, so it does not ask what replaying A keeps. It stays a reading.
- **Option 3(c), the twin on the pair's patterns**: within 2 SE of the night without replay in one start of four at block 36.
- **Option 4(b), the awake gap**: it asks sleep against no sleep, and from one start a night without replay read 14.4 under it.
- **Option 5(b), block 24**: right after the last night, it reads what a night gives, not what lasts.
- **Option 6(b), seed 22**: the explorations' network, where the comparison would rest on their numbers.
- **Chained episodes compiled in sleep** (UPG-04), Appendix C's named candidate: no exploration read it, and its layout is undecided ([ADR-0187](0187-the-upgrade-suite-planned.md)).

## Confirmation

`briefs/087_learning-that-sleep-keeps.md` builds the three parts and runs H-47 once.
[H-47's report](../research/H-47.md) holds the hypothesis as written before any run.
`npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
