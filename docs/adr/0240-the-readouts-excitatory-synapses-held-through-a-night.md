---
status: proposed
date: 2026-11-08
depends-on: ADR-0239
title: The readouts' excitatory synapses held through a night, M11's next exploration
kind: decision
summary: Exploration 024 held the inhibition onto the readouts' excitatory units through plain nights, and A's count stood within 2 SE of the plain night's everywhere. No hypothesis is written; exploration 025 holds those units' own synapses, alone and with that inhibition.
decision-makers: Menteion maintainers
---

# ADR-0240: The readouts' excitatory synapses held through a night, M11's next exploration

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 025 runs H-47's plain nights on A's patterns with the synapses of the readouts' excitatory units held where they stood, alone and with exploration 024's hold, on exploration 020's four cells. Its change under `src/` lives on its own branch, and `main` takes nothing from it. It runs on the maintainers' word, not on this merge.
- **Why.** Under exploration 024's held night A's count stood within 2 SE of H-47's plain night's at all 20 checkpoints and the side was H-47's at all 20, though the inhibition onto the readouts' excitatory units did not move in the nights. The same nights took 9.35 to 13.21 M from those units' own synapses, as the plain night took 9.20 to 12.33 M. That is the rest of the night [ADR-0238](0238-what-keeps-a-mapping-where-the-replay-did-not.md) named.
- **Left open.** Whether a hypothesis follows exploration 025; H-47's other eight cells; what the next day does with the pairings a hold leaves; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0239](0239-the-readouts-inhibition-held-through-a-night.md) ordered exploration 024: H-47's plain night on A's patterns with every inhibitory synapse onto an excitatory unit of either readout held from the night's first tick to its last, its trace cleared at the last. It ran exploration 020's four cells, in all of which stimulus A answers readout 1, the even places. Its readings are exploratory, not evidence ([exploration 024](../explorations/024-the-readouts-inhibition-held.md)):

- **The hold held.** In all 96 held nights every held synapse ended the night at the weight it started with, its trace cleared, while 1 369 to 1 623 inhibitory synapses onto other units moved. The night without replay ran H-47's, row for row.
- **A's count under the held night lies within 2 SE of H-47's plain night's at every checkpoint in every cell**: held less plain −1.2 to +0.6, at most 1.04 times exploration 018's unpaired SE. At block 36 held less without replay is −4.8 ± 1.45, +1.1 ± 1.20, +5.0 ± 1.56 and +1.5 ± 0.92, where H-47's clause 1 read −4.9 ± 1.09, +1.8 ± 1.79, +5.0 ± 1.24 and +2.2 ± 0.65.
- **The side is H-47's at all 20 checkpoints**, from block 18 on B, the odd places, in all four cells. Each held night ran the plain night's replays and depotentiations, night by night.
- **What the hold left**: the quiet ticks after the night moved held synapses in 14 of 96 nights, by up to 931 LSB together, and pairings 10.1 % to 17.7 % the size of the positive trace cleared stay pending. The note does not part the inhibition's share from the rest.

**What plain nights take from the readouts' excitatory units' own synapses**, over four nights:
- [Exploration 021](../explorations/021-where-the-gain-falls.md) read each readout's excitatory units onto all losing 9.20 to 12.33 M where A answers readout 1 and 7.54 to 11.10 M where it answers readout 0. Without replay they moved by −0.58 to +1.49 M.
- Where A answers readout 1, 1.48 to 2.75 M of it was onto the readout's own units. The rest fell onto other units, which 021 does not split.
- In 024's four cells the held night took 9.35 to 13.21 M from them and the plain night 9.20 to 12.33 M.

**What the engine holds.** A synapse consolidates at one place: the fan-out of its presynaptic unit's spike reads the pairings of that unit's blocks into their traces and consolidates them. In slow-wave sleep an excitatory slot consolidates under [ADR-0235](0235-learning-that-sleep-keeps-built.md)'s slow-wave baseline while it is set. H-47's zero sum holds six groups, each kind of unit apart: A alone, A's and C's, C alone, and the same of B and D. After a tick whose consolidation moved one of a group's synapses onto the readouts' units, a round scales that group back to its sum. C's own units lie at place 5, among the odd places, and D's at place 16, among the even: they are the zero sum's groups C alone and D alone. Nothing in `main` holds a set of units' synapses for a night; exploration 024's branch holds inhibitory slots by their target.

## Decision Drivers

- **The maintainers' choices** (2026-11-08): explore the readouts' excitatory units' synapses held, alone and with exploration 024's hold, on the same four cells; and wait for their word before it runs.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A held night is another form, so a hypothesis on it needs an ADR of its own.
- **ADR-0238** named a night that leaves the readouts' synapses where they stood. ADR-0239 took its inhibitory part, and exploration 024's note lists the rest first among the questions it suggests.

## Considered Options

1. **Explore the night with the readouts' excitatory synapses held**, alone and with their inhibition, on exploration 020's four cells.
2. **Explore exploration 024's held night on H-47's other eight cells**, among them every cell where A answers readout 0.
3. **Write a hypothesis on a held night now**, built by a brief and measured by a round.
4. **Pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.

## Decision Outcome

**Option 1.**

### Exploration 025 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **The excitatory hold**: from the night's first tick to its last, no synapse whose presynaptic unit is an excitatory unit of either readout consolidates, onto whatever target. The readouts are H-47's, the odd places but 11 and the even places but 0. At the night's last tick each held synapse's trace is cleared. A pairing with a spike in the night's last ticks is still read at the synapse's first presynaptic spike after it. Every other synapse consolidates as in H-47.
- **The pair's own units are not held** by it. Their synapses are C's and D's own couplings, which the zero sum holds as groups of their own; holding them would stop those groups' rounds. Over four plain nights C's own moved by at most 0.10 M and D's by at most 0.13 M (021).
- **Its branch** starts from `main` with exploration 024's tag merged in, and adds what the excitatory hold needs under `src/`, unset by default. Exploration 024's hold stays as that branch built it. `main` takes nothing from either; an engine change would be an ADR of its own.
- **Its cells** are exploration 020's four, as 024 ran them. H-47's two that failed: seed 23's first mapping from the mirrored assignment, and seed 24's eighth image from the assignment. Seed 22's two of the same starts, where clause 1 held.

**Its job, one a cell**, runs four forms. On exploration 024's figures, 5 796 to 6 057 s for three, four take about 8 000 s, within an arms job's 240 minutes:
- **the night without replay**, its rows held to H-47's;
- **the excitatory night**: H-47's plain night on A's patterns with the excitatory hold set;
- **the whole night**: H-47's plain night with the excitatory hold and exploration 024's hold both set;
- **the excitatory night without replay**, to read that hold apart from the replay.

H-47's plain night is read from H-47's logs and its class matrices from exploration 020's; exploration 024's held night from 024's logs.

**Its trial**, on seed 23's first mapping from the mirrored assignment, runs the pair's head and each form's first night on path 0:
- with no hold set, the night without replay and the plain night hold H-47's row at block 6 on path 0, bit for bit;
- with exploration 024's hold alone, the held night holds 024's trial's row at block 6 on path 0, bit for bit;
- with the excitatory hold set, every held synapse ends the night at the weight it started with and its trace cleared, and an excitatory synapse from a unit outside the held set moves.

It reads at blocks 6, 12, 18, 24 and 36, in H-47's terms:
1. **A's count and each stimulus's trials** under each held night: less without replay, with H-47's paired SE, the forms run in one job; less H-47's plain night and less exploration 024's held night, each with exploration 018's unpaired SE.
2. **The side under each held night**: the stimulus whose difference from the night without replay is the larger, by stimulus and by group of units, against H-47's side and 024's.
3. **The pair** under each held night and A's highest coupling, as clauses 2 and 3 read them.
4. **What each night moved**, from the class matrix by group as exploration 024 reads it. The readouts' excitatory units' synapses are split by target: onto their own group, onto the other readout's excitatory units, onto the inhibitory units, onto A's and B's units. The held synapses' change is shown nil.
5. **What the hold leaves**: what the quiet ticks after each night move, and the pairings still pending, as exploration 024 read them.

Before it spends a job it states whether any form must run bit for bit as one already in the logs, and runs none that does.
It does not run H-47's other eight cells, among them every cell where A answers readout 0, nor plain nights on both or the awake gap. Nor does it run a hold through the quiet ticks, the whole night without replay, or the inhibitory synapses onto the readouts' inhibitory units held.

## Consequences

- **Good**: the rest of the night ADR-0238 named is put to a run, beside exploration 024's half, on cells whose nights are already in the logs.
- **Bad**: the hold is a change under `src/` that `main` does not take, so what the exploration reads is not the engine's.
- **Bad**: in all four cells A answers readout 1, and no hold has run where A answers readout 0.
- **Bad**: the pair's own units' synapses still consolidate, so the readouts' excitatory weight is held but for them.
- **Bad**: M11 stays open, and no hypothesis is in view.
- **Neutral**: nothing in the engine changes, and the exploration waits for the maintainers' word.

## Alternatives considered and why rejected

- **Option 2, 024's hold on the other eight cells**: there plain nights lean the inhibition more toward A's readout, by 1.35 to 3.71 M where A answers readout 0, and no hold has run. It would read the same half of the night on more cells and leave the excitatory half unread in every cell.
- **Option 3, a hypothesis now**: holding the inhibition left A's count within 2 SE of the plain night's in every cell, and no run has held the excitatory synapses. A prediction would rest on which half of the night carries the plain night's effect, which no run has parted.
- **Option 4, pause M11**: the replay helped in ten cells of twelve, and what part of the night carries it is unread.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 025's note, when it merges, holds what it read.
