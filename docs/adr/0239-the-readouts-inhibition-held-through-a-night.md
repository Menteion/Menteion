---
status: proposed
date: 2026-11-08
depends-on: ADR-0238
title: The readouts' inhibition held through a night, M11's next exploration
kind: decision
summary: Explorations 021 to 023 read the side of a replay's gain lying with a group of units, the readout's index a name, and plain nights adding inhibition onto A's readout. No hypothesis is written: an exploration holds that inhibition through H-47's plain nights. The sweep of 2026-11-04 is recorded.
decision-makers: Menteion maintainers
---

# ADR-0239: The readouts' inhibition held through a night, M11's next exploration

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 024 runs H-47's plain nights on A's patterns with the inhibitory synapses onto the readouts' excitatory units held where they stood, on exploration 020's four cells. It needs a change under `src/` on its own branch, and `main` takes nothing from it.
- **Why.** In 020's four cells plain nights added 17.56 to 27.05 M of inhibitory weight. In 021's twelve they added more onto stimulus A's readout's excitatory units in every cell. ADR-0238 named a night that leaves the readouts' synapses where they stood and put the logs first. The logs are read.
- **Left open.** Whether a hypothesis follows exploration 024; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).
- **The schedule's sweep of 2026-11-04 is recorded here**: no survivor, so no finding.

## Context and Problem Statement

[ADR-0238](0238-what-keeps-a-mapping-where-the-replay-did-not.md) answered H-47's step 5 and ordered exploration 021. At a checkpoint the **side** is the stimulus whose plain less without replay difference is the larger. The maintainers then chose explorations 022 (2026-11-03) and 023 (2026-11-04). Their readings are exploratory, not evidence.

- **[Exploration 021](../explorations/021-where-the-gain-falls.md)**, on H-47's twelve cells: at blocks 6 and 12 the side lies on stimulus A in 10 cells of 12. From block 18 it lies on readout 0 in 9, 9 and 8. Exchanging A's and B's roles within a start does not move it from block 18 in any of four cells.
- **[Exploration 022](../explorations/022-readout-or-the-pairs-own.md)**: from block 18 the side lies on the same readout in all four of its layouts, in all four cells. It goes with C's own units in none and with the role in none.
- **[Exploration 023](../explorations/023-the-readouts-index-or-its-group.md)**: the readouts' index exchanged ran H-47's runs bit for bit, and their places exchanged ran H-47's other arm. So the index is a name, and the readout 0 of 021 and 022 is the odd places but 11. From block 18 the side lies there in three cells of four, and on the even places on seed 24's eighth image from the mirrored assignment.

**What plain nights do to the inhibition**, against nights without replay:
- In [exploration 020](../explorations/020-what-a-replay-costs.md)'s four cells, four plain nights took 19.75 to 23.59 M of the readouts' excitatory weight and added 17.56 to 27.05 M of inhibitory weight. At block 36 the readouts fired 20 % to 35 % less on A's trials.
- In H-47's layout they add more inhibition onto stimulus A's readout's excitatory units than onto the other's, in all twelve cells (021): by 1.35 to 3.71 M where A answers readout 0, and by 0.10 to 1.41 M where it answers readout 1. A night without replay moved the inhibition onto each readout's excitatory units by −2.23 to +1.17 M.
- The readout that gains more is the side's at block 36 in 6 cells of 12. In 022's layout with the roles exchanged and the pair's own units moved, more falls onto A's readout in one cell of four.

**What the engine holds.** Under H-25's learning configuration every inhibitory synapse consolidates under the inhibitory baseline, asleep or awake ([ADR-0086](0086-the-inhibitory-baseline-built.md)), and ADR-0235's slow-wave baseline reaches none of them while that baseline is set. A baseline of zero would stop every inhibitory synapse consolidating, onto every unit and through the day as well. Nothing stops those onto a set of units, or for a night alone, and the baseline is the image's, which outranks a configuration.

## Decision Drivers

- **The maintainers' choices** (2026-11-08): explore a night that holds the inhibitory synapses onto the readouts, before any hypothesis, and run it once this ADR merges.
- **H-47's stopping rule**: step 8 allows no second attempt and no constant moved. A night changed is a new mechanism, and a hypothesis on it needs an ADR of its own.
- **ADR-0238** named the night that leaves the readouts' synapses where they stood, and put the logs first. Explorations 020 to 023 hold H-47's two nights on these cells, with their splits and class matrices.

## Considered Options

1. **Explore the night with the readouts' inhibition held**, on exploration 020's four cells.
2. **Write a hypothesis on that night now**, built by a brief and measured by a round.
3. **Explore readouts that are not the two parities**, to part a group of units from its places, from the pair's own units and from the units next to each stimulus.
4. **Pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.

## Decision Outcome

**Option 1.**

### Exploration 024 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **The held night** is H-47's plain night on A's patterns in which no inhibitory synapse onto an excitatory unit of either readout consolidates, from the night's first tick to its last. Every other synapse consolidates as in H-47. The readouts are H-47's: the odd places but 11 and the even places but 0.
- **Its branch** adds what the hold needs under `src/`, unset by default. `main` takes nothing from it; an engine change would be an ADR of its own.
- **Its cells** are exploration 020's four. H-47's two that failed: seed 23's first mapping from the mirrored assignment, and seed 24's eighth image from the assignment. Seed 22's two of the same starts, where clause 1 held.
- **Its forms**: the held night, and if the jobs allow, the night without replay with the same synapses held, to read the hold apart from the replay. H-47's two nights are read from H-47's logs.
- **Its trial** holds H-47's rows with the hold unset, bit for bit. With it set, every held synapse ends a night at the weight it started with.

It reads at blocks 6, 12, 18, 24 and 36, in H-47's terms and with its paired SE:
1. **A's count and each stimulus's trials** under the held night, against H-47's two nights on the same cell: held less without replay, as clause 1 reads it, and held less plain.
2. **The side** under the held night, by stimulus and by group of units, against H-47's.
3. **The pair** under the held night and A's highest coupling, as clauses 2 and 3 read them.
4. **What each night moved**, from the class matrix by group as exploration 021 reads it, with the held synapses' change shown nil.

### The sweep

**The schedule's sweep of 2026-11-04, recorded here** ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).
The schedule's run on `main`, [41944581270](https://github.com/Menteion/Menteion/actions/runs/41944581270) at `7701a4f`, ended green at 05:59Z on 2026-11-05. No ADR merged since has recorded it.
- It was created at 00:52:50Z on 2026-11-05, 4 h 35 min after its cron of 20:17Z.
- Its eighteen weekly shards took 34.3 to 69.3 minutes against their bound of 240.
- Its seventeen sweep jobs started at 02:02Z. The crates' took 41.6 minutes, and the runtime's sixteen 149.1 to 237.2, under their bound of 330.

| Mutants | Count |
| :--- | ---: |
| Listed | 4 279 |
| Tested | 4 279 |
| Caught | 4 073: 2 482 in the state crates, 1 591 in the runtime |
| Missed | 0 |
| Unviable | 171 |
| Timeouts | 35 |

- **23 timeouts are where [ADR-0062](0062-the-first-complete-sweeps-list.md) reads them as inherent**, where 2026-10-28 had 22:
  - `menteion-core`'s chain iterators, six: `DeltaChain`, `MailboxDrain` twice and `Chain` three times;
  - the injector, twelve;
  - the barrier, two, at lines 35 and 41, where 2026-10-28 had one;
  - the workers' stop and run, three.
- **12 others, where 2026-10-28 had 17**, read by [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence:

| Where | The mutant | Beside a timeout | Its run |
| :--- | :--- | :--- | :--- |
| `image.rs:1212:33`, `Image::decode` | `>` replaced by `>=` | yes | had stopped finishing tests |
| `image.rs:1236:24`, `Image::decode` | `!` deleted | yes | still finishing tests |
| `image.rs:1236:44`, `Image::decode` | `\|\|` replaced by `&&` | yes | still finishing |
| `image.rs:1236:66`, `Image::decode` | `!` deleted | yes | had stopped |
| `image.rs:1243:31`, `Image::decode` | `>` replaced by `<` | yes | had stopped |
| `image.rs:1259:24`, `Image::decode` | `!` deleted | yes | had stopped |
| `executor.rs:3016:9`, `depotentiations` | replaced by `1` | yes | still finishing |
| `executor.rs:3092:9`, `compact` | replaced by `Ok(Default::default())` | yes | had stopped |
| `executor.rs:3120:9`, `discoveries` | replaced by `Vec::leak(vec![Default::default()])` | yes | still finishing |
| `executor.rs:3196:36`, `induce` | `<=` replaced by `>` | yes | still finishing |
| `executor.rs:3258:9`, `load_clause` | replaced by `true` | yes | still finishing |
| `branching.rs:98:5`, `trace` | replaced by `Ok(vec![(1, 0)])` | no | still finishing, F-42's kind |

- Of the 12, 7 were still finishing tests and 5 had stopped.
- The one beside no timeout hung no test of its own module, which makes it F-42's kind.
- The runtime's sixteen bounds were 1 617 to 2 856 s, median 2 246, on 2026-10-28, and 1 377 to 2 904 s, median 2 121, here. The runtime's timeouts fell from 33 to 29.
- **There is no survivor**, so the sweep leaves no finding.

## Consequences

- **Good**: the one reading that held in every cell of H-47's layout is put to a run, on cells whose two nights are already in the logs.
- **Bad**: the hold is a change under `src/` that `main` does not take, so what the exploration reads is not the engine's.
- **Bad**: M11 stays open, and no hypothesis is in view.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, a hypothesis now**: no run has held these synapses. A prediction would rest on sums of weight, and the side has followed the inhibition in 6 cells of 12.
- **Option 3, readouts that are not the two parities**: each first mapping would be learned again under them. An eighth image would need H-46's eight schedules learned again, whose first halves took 101.9 to 110.8 minutes. It parts places and moves no mechanism.
- **Option 4, pause M11**: the replay helped in ten cells of twelve, and whether the inhibition it adds is what costs A is unread.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 024's note, when it merges, holds what it read.
