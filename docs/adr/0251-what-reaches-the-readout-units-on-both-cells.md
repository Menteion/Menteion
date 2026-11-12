---
status: proposed
date: 2026-11-12
depends-on: ADR-0250
title: What reaches the readout units, on both cells
kind: decision
summary: On seed 24's eighth image from the mirrored assignment single replayed nights moved the readouts' inputs as on the image from the assignment, yet A's answer's readout less the other moved the other way. Exploration 036 reads on both every unit's spikes and the weights times them onto the readouts.
decision-makers: Menteion maintainers
---

# ADR-0251: What reaches the readout units, on both cells

**In brief**
- **Decision.** No hypothesis is written and no constant moves. Exploration 036 reads seed 24's two eighth images, from the assignment and from the mirrored assignment, under exploration 033's six forms. On each of A's reads it counts every unit's spikes, not only the readouts', and pairs each synapse onto a readout unit with its presynaptic unit's spikes: the weight times those spikes, summed by presynaptic class. Two jobs a cell, four. The branch adds its harness only, on an engine that is `main`'s, and `main` takes nothing from it. It runs once this ADR merges.
- **Why.** On both images each single replayed night moved the inputs onto the readouts the same way, 32 of 32 changes with the same sign, and both readouts fired less on both stimuli's trials. Yet on A's trials A's answer's readout less the other lay above the night without replay after the three later single nights on the mirrored image and below it on the other. An input is a sum of weights: the spikes of the units outside the readouts are not counted, and no weight is paired with its presynaptic unit's spikes.
- **Left open.** Whether a hypothesis follows exploration 036; any other cell and its inputs; what the image a replayed night starts from decides; a unit's membrane, threshold or delays; the inputs of the units outside the readouts' sets; the pair's reads unit by unit; replay on two of the four nights on any cell, or on every night but one on any cell but seed 23's first mapping from the mirrored assignment; the night held onto the inhibitory and outside units without replay, or split finer; the part held across, or exploration 024's hold, beside any part; the synapses no hold reached; exploration 024's hold on the other eight cells; what the next day does with the pairings a night leaves; what keeps the night held within below the excitatory night on seed 24's eighth image from the assignment; what moves A's trials below the plain night's on seed 23's eighth image from the mirrored assignment; why after block 12 alone lies below the night without replay on seed 23's eighth image from the assignment; plain nights on both and the awake gap night by night; whether the slow-wave baseline enters the running configuration; chained episodes compiled in sleep (UPG-04).

## Context and Problem Statement

[ADR-0250](0250-the-mirrored-cells-units-and-inputs.md) ordered exploration 035: seed 24's eighth image from the mirrored assignment, H-25's other arm on the same seed, under exploration 033's six forms, each readout unit's spikes on A's reads and the weights onto it by presynaptic class, beside explorations 033 and 034 on the image from the assignment. Its readings are exploratory, not evidence ([exploration 035](../explorations/035-the-mirrored-cells-units-and-inputs.md)):

- **Every row and read is H-47's, exploration 031's or exploration 032's**, label aside, and each of the 191 images' input reads sums to that image's class matrix. On that cell A asks readout 0, the odd places, of stimulus A and readout 1, the even places, of B; on the image from the assignment the other way round.
- **Right after each single replayed night both readouts fired less on both stimuli's trials**, by 19 to 29 per cent of the night without replay's spikes, and each of the 16 differences lies the same side of 2 SE as on the image from the assignment.
- **Over each single replayed night the inputs moved as on the image from the assignment**: of the inhibitory, all excitatory and A's and B's kinds' changes onto each readout, 32 of 32 carry the same sign on both cells. The inhibitory input rose by 4.16 to 5.36 M here and 4.18 to 4.48 there, A's kinds' input onto A's answer's readout by 0.35 to 0.42 M and 0.23 to 0.31, and each place group's excitatory units lost more onto the other readout than onto their own, 8 of 8 on each.
- **The units that carried half of each readout's drop** lost input from the trial's stimulus's kinds against the rest of their readout in 16 of 16 sets on each cell, and over each readout's units a unit's input difference from those kinds correlated with its spikes' difference by +0.43 to +0.83 here and +0.51 to +0.72 there.
- **What reaches a unit was not read**: an input is a sum of weights; the readout units' own spikes are counted, the spikes of the units outside the readouts are not, and no weight is set against the spikes of the unit it comes from.

On A's trials, A's answer's readout's spikes less the other's, less the night without replay's, and its excitatory input less inhibitory input less the other's, less the night without replay's, right after each single replayed night:

| Form | Spikes: the mirrored image | the image from the assignment | Excitatory less inhibitory input, M: the mirrored image | the image from the assignment |
| :--- | ---: | ---: | ---: | ---: |
| After block 6 alone | −0.84 ± 0.21 | +0.48 ± 0.29 | −1.76 | −0.01 |
| After block 12 alone | +0.69 ± 0.18 | −1.46 ± 0.22 | −1.21 | −0.48 |
| After block 18 alone | +2.42 ± 0.23 | −2.96 ± 0.19 | −1.42 | −0.46 |
| After block 24 alone | +1.31 ± 0.31 | −3.73 ± 0.16 | −1.86 | −0.95 |

On the mirrored image A's answer's readout lost more excitatory input less inhibitory input than the other after every single night, more than on the image from the assignment, yet after the three later nights its spikes fell less below the night without replay's than the other's. Of the eight differences of each stimulus's answer's readout less the other on its own trials, after each single night, 0 of 8 lie the same side of 2 SE on both cells.

## Decision Drivers

- **The maintainers' choices** (2026-11-12): on both of seed 24's eighth images, count every unit's spikes on A's reads and read the weight times the presynaptic spikes onto each readout unit by class, two jobs a cell; and run it once this ADR merges.
- **H-47's stopping rule**, step 8: "no constant moves after such a run; no second attempt, and no other baseline, shift, form, path or network." A read changes no form, and any hypothesis still needs an ADR of its own.
- **The harness has both halves**: on the tag `explore/035-the-mirrored-cells-units-and-inputs`, exploration 033's unit read counts each readout unit's spikes from the executor's train by `Readout::count_window`'s rule, and exploration 034's input read walks every synapse of the arena (`e034_synapses`), both run by `e030_job_reading` and `e030_trial_reading` for either cell; neither needed a change under `src/`.

## Considered Options

1. **What reaches the readout units on both cells**, every unit's spikes and the weights times them, six forms, four jobs.
2. **The input read on the other ten cells**, exploration 034's read, two jobs a cell.
3. **A reading of the logs already held**: the units that carried half of each drop against their inputs on the image before the night, and the two cells' start images, with no run.
4. **Write a hypothesis now**, or **pause M11** and turn to M13, [ADR-0187](0187-the-upgrade-suite-planned.md)'s next milestone.

## Decision Outcome

**Option 1.**

### Exploration 036 (Specified; it decides nothing)

It writes no prediction and decides nothing.
- **Its cells** are seed 24's eighth image from the assignment and from the mirrored assignment, H-25's two arms, on H-47's paths 0, 5 and 6, read sixteen times at blocks 6, 12, 18, 24 and 36 in H-47's terms.
- **Its forms** are exploration 033's six: the night without replay in each job, `alone-6`, `alone-12`, `alone-18` and `alone-24`, and `plain-first`, H-47's plain night on all four nights. No hold is set.
- **Its reads**, on each of A's reads: exploration 033's unit read and, on each image whose class matrix the job writes, exploration 034's input read, as exploration 035 ran them; every unit of the arena counted in each trial's readout window by the same rule, summed over each stimulus's trials; and **the drive**, for each readout unit and presynaptic class, the sum over the synapses onto it from that class of the synapse's weight at the trial's end times its presynaptic unit's spikes in the trial's window, an inhibitory unit's negated, summed over each stimulus's trials.
- **What it prints**: every unit's spikes after each read; the drive summed over each readout's units by class after each read; and the drive unit by unit, summed over the sixteen reads of each checkpoint on each path.
- **Held inside the test**: the readout units' counts among every unit's are exploration 033's unit read's, on every trial; and a test holds that the drive's walk, with every presynaptic count one, gives the input read's sums.
- **Its branch** starts from `main` with exploration 035's tag merged in, for the harness of explorations 033 to 035; that tag changes nothing under `src/`, so the engine is `main`'s. The branch adds its harness only; if the drive cannot be read without a change under `src/`, it stops and says why.

**Its jobs**, four, two a cell, as exploration 035's: one runs the night without replay, after block 6 alone and H-47's plain night, nine chains; the other the night without replay and after block 12, 18 or 24 alone, twelve chains. On the mirrored image exploration 035's took 5 971 and 7 485 s, on the image from the assignment exploration 034's 5 822 and 7 925. Counting every unit and walking the synapses add to each trial, and the trial times them. If the trial puts a job past an arms job's 240 minutes, that job drops a form, H-47's plain night first and then after block 12 alone, and the note names the cut.

**Inside each job** the rows are held to H-47's as in exploration 033's jobs (`held_to`), each form's replayed nights read replays and depotentiations that are not zero and its other nights none (`e030_night`), and the unit read's and input read's guards hold as there. Every line of explorations 033's and 034's jobs on the image from the assignment and of exploration 035's on the mirrored image, label aside, is looked for in exploration 036's.

**Its trial** runs the pair's head and each form's first night on path 0 of each cell, as exploration 035's trial did, each holding H-47's row at block 6 on path 0 of the form whose first night it ran.

It reads, on each cell, in exploration 033's terms for spikes and exploration 034's for inputs, spikes and each readout's drive by class with their SEs paired within a job and unpaired across jobs as exploration 033 did; a unit's drive, summed over its sixteen reads, carries none. Where the cells are set side by side each readout is named by the stimulus whose answer it is, as exploration 035 named them:
1. **The rows and reads against the logs**: each form's rows, reads, unit reads and input reads beside explorations 033, 034 and 035's, so that every spike and input figure is those notes'.
2. **Every unit's spikes by class**: each class's spikes a trial on each stimulus's trials under each form, A's and B's kinds and the outside units among them, each form less the night without replay right after its replayed night and at block 36.
3. **The drive onto each readout** by presynaptic group and class on each stimulus's trials, each form less the night without replay right after and at block 36; on each stimulus's trials its answer's readout's drive less the other's beside the spikes'; each class's drive difference beside its input difference and its units' spikes difference; the units that carried half of each readout's change against the rest; and the correlation over each readout's units of a unit's drive difference with its spikes' difference, beside the input's.
4. **The two cells side by side**, each of those by stimulus, the same form, night and checkpoint.

Before it spends a job it states whether any form must run bit for bit as one already in the logs: every form's images, rows and reads on both cells are in explorations 033, 034 and 035's logs, but no log holds every unit's spikes or the drive, so each form runs, and its rows and reads are set beside those logs'.
It does not run any other cell; replay on two nights or on every night but one; any hold; plain nights on both or the awake gap; the pair's reads unit by unit or its drive; the inputs of the units outside the readouts' sets; a spike's arrival after its synapse's delay, since the drive pairs a weight with its presynaptic unit's spikes in the same window; a unit's membrane or threshold; any path but H-47's three; any part of a night apart; or the next day's reading of the pairings a night leaves.

### The sweep

**The schedule's sweep of 2026-11-11, recorded here** ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).
The schedule's run on `main`, [43137595179](https://github.com/Menteion/Menteion/actions/runs/43137595179) at `64db9c1`, ended green at 07:20Z on 2026-11-12. No ADR merged since has recorded it.
- It was created at 01:53:34Z on 2026-11-12, 5 h 36 min after its cron of 20:17Z.
- Its eighteen weekly shards took 36.5 to 67.6 minutes against their bound of 240.
- Its seventeen sweep jobs started at 03:01Z. The crates' took 24.7 minutes, and the runtime's sixteen 147.4 to 259.2, under their bound of 330.

| Mutants | Count |
| :--- | ---: |
| Listed | 4 279 |
| Tested | 4 279 |
| Caught | 4 077: 2 482 in the state crates, 1 595 in the runtime |
| Missed | 0 |
| Unviable | 171 |
| Timeouts | 31 |

- **22 timeouts are where [ADR-0062](0062-the-first-complete-sweeps-list.md) reads them as inherent**, where 2026-11-04 had 23:
  - `menteion-core`'s chain iterators, six: `DeltaChain`, `MailboxDrain` twice and `Chain` three times;
  - the injector, twelve;
  - the barrier, one, at line 35, where 2026-11-04 had two;
  - the workers' stop and run, three.
- **9 others, where 2026-11-04 had 12**, read by [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence:

| Where | The mutant | Beside a timeout | Its run |
| :--- | :--- | :--- | :--- |
| `image.rs:1209:40`, `Image::decode` | `>` replaced by `>=` | yes | still finishing tests |
| `image.rs:1212:33`, `Image::decode` | `>` replaced by `>=` | yes | had stopped finishing tests |
| `image.rs:1236:24`, `Image::decode` | `!` deleted | yes | had stopped |
| `image.rs:1236:44`, `Image::decode` | `\|\|` replaced by `&&` | yes | had stopped |
| `image.rs:1236:63`, `Image::decode` | `\|\|` replaced by `&&` | yes | had stopped |
| `image.rs:1236:66`, `Image::decode` | `!` deleted | yes | had stopped |
| `image.rs:1243:31`, `Image::decode` | `>` replaced by `<` | yes | had stopped |
| `image.rs:1259:24`, `Image::decode` | `!` deleted | yes | still finishing |
| `executor.rs:3075:9`, `untagged` | replaced by `0` | no | still finishing, F-42's kind |

- Of the 9, 3 were still finishing tests and 6 had stopped.
- The one beside no timeout hung no test of its own module: no test warned, and its run was still finishing tests, which makes it F-42's kind.
- The runtime's sixteen bounds were 1 377 to 2 904 s, median 2 121, on 2026-11-04, and 1 713 to 2 580 s, median 2 322, here. The runtime's timeouts rose from 29 to 31, and the crates' fell from 6 to 0.
- **There is no survivor**, so the sweep leaves no finding.

## Consequences

- **Good**: the read that explorations 034 and 035 both name as unread, what reaches each readout unit on A's trials, is taken on the two cells whose spikes and inputs are already read, where the same input changes left A's answer's readout less the other on opposite sides of the night without replay.
- **Good**: both cells in one exploration, set side by side by stimulus.
- **Bad**: two cells of twelve; a contrast between them says nothing of the other ten.
- **Bad**: the drive pairs a weight with its presynaptic unit's spikes in the same window, not after the synapse's delay, and a unit's membrane and threshold are not read.
- **Bad**: the forms run again, though their images are in the logs by checksum, and four jobs' logs are larger than exploration 035's two.
- **Bad**: the harness change is on a branch that `main` does not take.
- **Bad**: M11 stays open; any hypothesis needs its own ADR, brief and round.
- **Neutral**: nothing in the engine changes.

## Alternatives considered and why rejected

- **Option 2, the input read on the other ten cells**: it asks whether the inputs move alike elsewhere before anything says why, on the two cells read, the same input changes left the spikes of A's answer's readout less the other's on opposite sides.
- **Option 3, the logs already held**: the logs hold no presynaptic unit's spikes outside the readouts, and two start images cannot say which of their differences sets the direction.
- **Option 4, a hypothesis now or a pause**: on A's trials no single later night left A's answer's readout less the other on the same side of the night without replay on both cells, so a prediction would rest on a split reading; and the read both explorations name as missing is one exploration away on cells whose other reads are in the logs.

## Confirmation

`npm run spec` holds the links, the tables and the version.
Exploration 036's note, when it merges, holds what it read.
