---
status: proposed
date: 2026-11-08
depends-on: ADR-0239
title: Synapses from the readouts' excitatory units held through a night
kind: decision
summary: Exploration 024 held the inhibition onto the readouts' excitatory units through plain nights, and A's count stood within 2 SE of the plain night's at every checkpoint of its four cells. No hypothesis is written; exploration 025 holds the synapses from those units.
decision-makers: Menteion maintainers
---

# ADR-0240: Synapses from the readouts' excitatory units held through a night

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 025 runs H-47's plain nights on A's patterns with the synapses from the readouts' excitatory units held where they stood, the pair's own units excepted, alone and with exploration 024's hold, on exploration 020's four cells. Its change under `src/` lives on its own branch, and `main` takes nothing from it. It runs on the maintainers' word, not on this merge.
- **Why.** Under exploration 024's held night A's count stood within 2 SE of H-47's plain night's at all 20 checkpoints of its four cells, and the side was H-47's at all 20, though the inhibition onto the readouts' excitatory units did not move in the nights. The same nights took 9.35 to 13.21 M from the synapses from those units, the plain night 9.20 to 12.33 M. Holding them is the excitatory part of what [ADR-0238](0238-what-keeps-a-mapping-where-the-replay-did-not.md)'s night left after 024.
- **Left open.** Whether a hypothesis follows exploration 025; the inhibitory synapses onto the readouts' inhibitory units, which neither hold reaches; H-47's other eight cells; what the next day does with the pairings a hold leaves; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0239](0239-the-readouts-inhibition-held-through-a-night.md) ordered exploration 024: H-47's plain night on A's patterns with every inhibitory synapse onto an excitatory unit of either readout held from the night's first tick to its last, its trace cleared at the last. It ran exploration 020's four cells, in all of which stimulus A answers readout 1, the even places. Its readings are exploratory, not evidence ([exploration 024](../explorations/024-the-readouts-inhibition-held.md)):

- **The hold held.** In all 96 held nights every held synapse ended the night at the weight it started with, its trace cleared, while 1 369 to 1 623 inhibitory synapses onto other units moved. The night without replay ran H-47's, row for row.
- **A's count under the held night lies within 2 SE of H-47's plain night's at every checkpoint of the four cells**: held less plain −1.2 to +0.6, at most 1.04 times exploration 018's unpaired SE.
- **At block 36 held less without replay** is −4.8 ± 1.45 on seed 23's first mapping from the mirrored assignment, +1.1 ± 1.20 on seed 24's eighth image from the assignment, and +5.0 ± 1.56 and +1.5 ± 0.92 on seed 22's first mapping and eighth image from the same starts. H-47's clause 1 read −4.9 ± 1.09, +1.8 ± 1.79, +5.0 ± 1.24 and +2.2 ± 0.65.
- **The side is H-47's at all 20 checkpoints**, from block 18 on B, the odd places, in all four cells. Each held night ran the plain night's replays and depotentiations, night by night.
- **What the hold left**: the quiet ticks after the night moved held synapses in 14 of 96 nights, by up to 931 LSB together. Pairings 10.1 % to 17.7 % the size of the positive trace cleared stay pending under the held night, and 7.3 % to 12.5 % under the held night without replay. The synapses from the readouts' excitatory units lost up to 0.97 M more under the held night than under the plain night. The note does not part the inhibition's share from the rest.

**What plain nights take from the synapses from the readouts' excitatory units**, over four nights:
- [Exploration 021](../explorations/021-where-the-gain-falls.md) read each readout's excitatory units onto all losing 9.20 to 12.33 M where A answers readout 1 and 7.54 to 11.10 M where it answers readout 0. Without replay they moved by −0.58 to +1.49 M.
- Where A answers readout 1, 1.48 to 2.75 M of it was lost from their synapses onto the readout's own units, and the rest from those onto other units, which 021 does not split.
- 021 counts the pair's own units inside each readout. Over four plain nights their synapses onto all moved, net, by at most 0.10 M for C's and 0.13 M for D's. Onto one readout's excitatory units they moved by up to 0.20 M in 024's four cells.
- In those four cells the held night took 9.35 to 13.21 M from the readouts' excitatory units' synapses and the plain night 9.20 to 12.33 M.

**What the engine holds.** A synapse consolidates at one place: the fan-out of its presynaptic unit's spike reads the pairings of that unit's blocks into their traces and consolidates them. An excitatory block reads two pairings there ([ADR-0055](0055-a-weight-that-settles.md), [ADR-0228](0228-a-zero-sum-at-the-stimulus-built.md)). Where the target fired after the previous presynaptic spike the trace gains $A_+ (1 - 2^{-11})^{q - p}$. Where it fired before this spike the trace loses an amount scaled by the slot's magnitude. An inhibitory block gains for both. In slow-wave sleep an excitatory slot consolidates under [ADR-0235](0235-learning-that-sleep-keeps-built.md)'s slow-wave baseline while it is set.

**The zero sum** in H-47 holds six groups, each kind of unit apart: A alone, A's and C's, C alone, and the same of B and D. Its targets are every unit of the two readouts. After a tick whose consolidation moved one of a group's synapses onto a target, a round scales that group back to its value, its sum in the start image. C's own units lie at place 5, among the odd places, and D's at place 16, among the even: they are the groups C alone and D alone. A's and B's units lie at places 0 and 11, outside the readouts. Nothing in `main` holds a set of units' synapses for a night. Exploration 024's branch holds inhibitory slots by their target.

## Decision Drivers

- **The maintainers' choices** (2026-11-08): explore the synapses from the readouts' excitatory units held, alone and with exploration 024's hold, on the same four cells; and wait for their word before it runs.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A held night is another form, so a hypothesis on it needs an ADR of its own.
- **ADR-0238** named a night that leaves the readouts' synapses where they stood. ADR-0239 took part of its inhibitory half, the synapses onto the readouts' excitatory units, and exploration 024's note lists the excitatory synapses first among the questions it suggests.

## Considered Options

1. **Explore the synapses from the readouts' excitatory units held**, by presynaptic unit, alone and with exploration 024's hold, on exploration 020's four cells.
2. **Explore the excitatory synapses onto the readouts' units held**, by target, as exploration 024's half was cut.
3. **Explore exploration 024's held night on H-47's other eight cells**, among them every cell where A answers readout 0.
4. **Write a hypothesis on a held night now**, built by a brief and measured by a round.
5. **Pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.

## Decision Outcome

**Option 1.**

### Exploration 025 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **The excitatory hold**: from the night's first tick to its last, no synapse whose presynaptic unit is a held unit consolidates, onto whatever target. The held units are every excitatory unit of the two readouts but the pair's own: exploration 024's 714 less C's own 25 and D's own 25, so 664. The readouts are H-47's, the odd places but 11 and the even places but 0. At the night's last tick each held synapse's trace is cleared. A pairing with a spike in the night's last ticks is still read at the synapse's first presynaptic spike after it. Every synapse outside the hold consolidates as in H-47.
- **The two holds are cut differently.** Exploration 024's holds by target, the inhibitory synapses onto the readouts' excitatory units; this one by source, the synapses from them. So the excitatory synapses onto the readouts from A's and B's units, under the zero sum, and from the pair's own units still consolidate. So do the inhibitory synapses onto the readouts' inhibitory units.
- **The pair's own units are left out by choice.** It keeps the hold off the zero sum's sources, and leaves the pair's couplings, which carry the count clause 2 reads, as in H-47. Their synapses moved little under plain nights (021, above).
- **Its branch** starts from `main` with exploration 024's tag merged in, and adds what the excitatory hold needs under `src/`, unset by default. Exploration 024's hold stays as that branch built it. `main` takes nothing from either; an engine change would be an ADR of its own.
- **Its cells** are exploration 020's four, as 024 ran them. H-47's two that failed: seed 23's first mapping from the mirrored assignment, and seed 24's eighth image from the assignment. Seed 22's two of the same starts, where clause 1 held.

**Its job, one a cell**, runs four forms. On exploration 024's figures, 5 796 to 6 057 s for three, four take about 7 700 to 8 100 s, within an arms job's 240 minutes:
- **the night without replay**, its rows held to H-47's. It runs H-47's night bit for bit, and runs to hold the branch to H-47's rows over the whole run, as exploration 024's did;
- **the excitatory night**: H-47's plain night on A's patterns with the excitatory hold set;
- **the night under both holds**: H-47's plain night with the excitatory hold and exploration 024's hold set;
- **the excitatory night without replay**, to read that hold apart from the replay.

H-47's plain night is read from H-47's logs and its class matrices from exploration 020's; exploration 024's held night from 024's logs.

**Its trial**, on seed 23's first mapping from the mirrored assignment, runs the pair's head and each form's first night on path 0:
- with no hold set, the night without replay and the plain night hold H-47's row at block 6 on path 0, bit for bit;
- with exploration 024's hold alone, the held night holds the row 024's trial printed at block 6 on path 0 (run 42405480901: the image's CRC-64, A's first count, its couplings and the gain), bit for bit;
- with the excitatory hold set, every held synapse ends the night at the weight it started with and its trace cleared, and an excitatory synapse from a unit outside the hold moves;
- with both set, the synapses each hold holds end the night so.

It reads at blocks 6, 12, 18, 24 and 36, in H-47's terms:
1. **A's count and each stimulus's trials** under each held night: less without replay, with H-47's paired SE, the forms run in one job; less H-47's plain night and less exploration 024's held night, each with exploration 018's unpaired SE.
2. **The side under each held night**: the stimulus whose difference from the night without replay is the larger, by stimulus and by group of units, against H-47's side and 024's.
3. **The pair** under each held night and A's highest coupling, as clauses 2 and 3 read them.
4. **What each night moved**, from the class matrix by group as exploration 024 reads it. The synapses from the held units are split by target: onto their own group, onto the other readout's excitatory units, onto the inhibitory units, onto A's and B's units, onto the other excitatory units. Their change in the night is shown nil, apart from what the quiet ticks after it move.
5. **What the hold leaves**: what the quiet ticks after each night move, and the pairings still pending under the excitatory rule, the gain and the loss apart, each with the condition under which a spike would read it.

Before it spends a job it states whether any form but the night without replay must run bit for bit as one already in the logs, and runs none that does.
It does not run H-47's other eight cells, among them every cell where A answers readout 0, nor plain nights on both or the awake gap. Nor does it run a hold through the quiet ticks, or both holds without replay.

## Consequences

- **Good**: the excitatory part of what ADR-0238's night left is put to a run, beside exploration 024's, on cells whose nights are already in the logs.
- **Bad**: the hold is a change under `src/` that `main` does not take, so what the exploration reads is not the engine's.
- **Bad**: in all four cells A answers readout 1, and no hold has run where A answers readout 0.
- **Bad**: the two holds, cut by target and by source, still leave the synapses from A's, B's and the pair's own units and those onto the readouts' inhibitory units consolidating, so no run holds every synapse of the readouts.
- **Bad**: M11 stays open, and no hypothesis is in view.
- **Neutral**: nothing in the engine changes, and the exploration waits for the maintainers' word.

## Alternatives considered and why rejected

- **Option 2, the excitatory synapses onto the readouts held by target**: those include A's and B's couplings, which carry A's count, and every synapse the zero sum's six groups hold. The night's replay would not reach the mapping it replays.
- **Option 3, 024's hold on the other eight cells**: where A answers readout 0 plain nights lean the inhibition more toward A's readout, by 1.35 to 3.71 M (021), and no hold has run there. It would read the same half on more cells and leave the excitatory synapses unread in every cell.
- **Option 4, a hypothesis now**: in 024's four cells holding the inhibition left A's count within 2 SE of the plain night's, and no run has held the synapses from the readouts' excitatory units. A prediction would rest on which part of the night carries the plain night's effect, which no run has parted.
- **Option 5, pause M11**: the replay helped in ten cells of twelve ([H-47](../research/H-47.md)), and what part of the night carries it is unread.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 025's note, when it merges, holds what it read.
