---
status: exploratory
date: 2026-11-12
---

# 034: One cell's readout units' inputs

**Branch**: `explore/034-one-cells-readout-units-inputs`
**Commit**: `3bbccde`, which adds the harness, on `a772c40`: `main` at `633f1e6`, ADR-0249's merge, with exploration 033's tag merged
in, which changes nothing under `src/`. This branch changes nothing under `src/`, so the engine is `main`'s.
**Runs**: [43051132329](https://github.com/Menteion/Menteion/actions/runs/43051132329) (the harness's trial) and
[43052358536](https://github.com/Menteion/Menteion/actions/runs/43052358536) (two jobs), both at `scope=arms`. The logs set beside them are
exploration 033's runs [43010140120](https://github.com/Menteion/Menteion/actions/runs/43010140120) (its trial) and
[43011654795](https://github.com/Menteion/Menteion/actions/runs/43011654795) (its two jobs), whose rows and reads are those of H-47's,
exploration 031's and exploration 032's jobs and whose class matrices are those of exploration 020's, 031's and 032's jobs, as exploration
033 found them. None of those was run again.

[ADR-0249](../adr/0249-one-cells-readout-units-inputs.md) ordered this exploration. On seed 24's eighth image from the assignment, right
after each single replayed night both readouts fired less on both stimuli's trials, by 18 to 33 per cent, and on A's trials readout 1 less
readout 0, less the night without replay, read +0.48 ± 0.29 after block 6 alone and −1.46 ± 0.22, −2.96 ± 0.19 and −3.73 ± 0.16 after
block 12, 18 and 24 alone; 13 to 19 units carried half of each readout's drop ([exploration 033](033-one-cells-readouts-unit-by-unit.md)).
Exploration 033 counted the units' spikes, not what each receives. This note reads, on each image the jobs split, the weights onto each of
the two readouts' 918 units summed by the class of the presynaptic unit, under the same six forms, and asks:

1. whether every row, read and unit read under each form is exploration 033's, so that every count and spike is that note's and is not read
   anew;
2. what each replayed night changed in the inputs of each readout's units, by presynaptic group and class, beside the night without
   replay's same night; on the units that carried half of each readout's drop against the rest of their readout; with each image's gain;
   and what differs in the image each replayed night starts from;
3. whether a unit's input difference lines up with its spikes' difference, and whether readout 1's units lost more input from A's kinds
   than readout 0's where their spikes on A's trials fell more;
4. what each later night and each stretch of blocks changed in the same inputs, to block 36;
5. the same of H-47's plain night on all four nights.

The note writes no prediction and decides nothing. **Every figure here is exploratory**: from one run of each test on the repository's
AArch64 host, in H-47's terms. **An input is the mean over the three paths** of one image each, in the class matrix's units: a sum of
Q1.15 weights, 32 768 to a weight of 1, written in millions (M) where it is a readout's or a class's; an inhibitory unit's weight is its
magnitude, so a + is more inhibition. No SE is claimed for an input. The spikes and their SEs are exploration 033's, from the same reads.

**In brief**
- **Every row, read and unit read is exploration 033's, and every input read sums to its image's class matrix.** 105 rows, 1 680 of A's
  reads, 1 680 of the pair's and 1 680 unit reads are exploration 033's, label aside, so the counts and spikes this note sets beside the
  inputs are that note's. On each of the 191 images the jobs read, each presynaptic class's sums over the readouts' units of each class are
  the class matrix's entries, held inside the test.
- **Over each single replayed night the inhibitory input onto both readouts rose and the excitatory input fell**: the inhibitory by 4.18 to
  4.48 M onto a readout and all excitatory by 3.37 to 4.29, where the night without replay's same nights raised the inhibitory input by 0.24
  to 0.36 and moved each excitatory group by 0.18 or less. Each place group's excitatory units lost more onto the other readout than onto
  their own. The gain fell by 0.061 to 0.065 over each single replayed night and rose by 0.033 to 0.037 over the night without replay's same
  nights.
- **Readout 1's units did not lose more input from A's kinds than readout 0's.** Right after each single replayed night A's kinds' input
  difference onto readout 1 less onto readout 0 read +0.49 to +0.69 M, and A's margin rose by 1.05 to 1.26 M over each single replayed
  night. Readout 1's excitatory input less inhibitory input difference lay below readout 0's by 0.01, 0.48, 0.46 and 0.95 M after block 6,
  12, 18 and 24 alone, where the spikes' readout 1 less readout 0 on A's trials read +0.48, −1.46, −2.96 and −3.73 (exploration 033).
- **The units that carried half of each readout's drop lost input from the trial's stimulus's kinds, and gained less inhibitory input than
  the rest.** Right after each single replayed night, on A's trials their input difference from A's kinds read −10 330 to −7 226 a unit
  where the rest of their readout had −433 to +1 065, and on B's trials from B's kinds −11 194 to −6 365 against −434 to +1 047; their
  inhibitory input difference read +455 to +6 269 a unit against the rest's +8 568 to +9 573. Both were so in each of the 16 sets and in
  each of the 16 under H-47's plain night at blocks 6 to 24, and in 19 of the 20 at block 36: there, after block 12 alone, readout 0's three
  units on B's trials gained +500 a unit from B's kinds, less than the rest's +535. Over each readout's 459 units a unit's input difference
  from the trial's stimulus's kinds correlated with its spikes' difference by +0.51 to +0.72 right after, from the other stimulus's kinds by
  −0.07 to +0.12, its excitatory input less inhibitory input difference by +0.12 to +0.28 and its inhibitory input difference by +0.05 to
  +0.20.
- **By block 36 the inhibitory difference had come back by 42 to 81 per cent.** Each stretch of blocks and each later night after each
  single replayed night lowered it, onto both readouts, where from right after the replayed night to block 36 A's and B's kinds' and each
  place group's difference onto each readout moved by 0.43 M or less.
- **Under H-47's plain night on all four nights the inhibitory input's, each place group's and A's and B's kinds' differences grew with each
  night**: the inhibitory input onto readout 1 read +4.15, +6.56, +8.32 and +9.49 M less the night without replay's at blocks 6, 12, 18 and
  24 and +7.48 at block 36, A's kinds' onto readout 1 +0.34, +0.57, +0.75 and +0.97, and +1.01 at block 36.

## What ran

**The engine, the network, the forms and the reads** are H-47's ([ADR-0236](../adr/0236-learning-that-sleep-keeps-its-protocol.md)), as
explorations 020 to 033 restate them. **A** is the mapping in force at the start, its stimuli A and B; **the pair** is the second mapping, C
sharing 26 of A's 51 source units and D 26 of B's, its answers the other way. Nights follow blocks 6, 12, 18 and 24, then 12 blocks run with
none to block 36, on paths 0, 5 and 6. Each checkpoint reads A and the pair sixteen times each, a read one block of 64 trials on a fork of
the image with the reward withheld.

**One cell**, seed 24's eighth image from the assignment, exploration 033's. There A asks readout 1, the even places, of stimulus A and
readout 0, the odd places, of B. A period of the ring is 20 units; stimulus A presents place 0 of each period and B place 11. **The odd
places** are the odd places but 11 and **the even places** the even places but 0: H-47's readout 0 and readout 1, nine places each over 51
periods, 459 units each. The readouts' units by class are exploration 033's, from the start image:

| Readout | Class | Places | How many units |
| :--- | :--- | ---: | ---: |
| readout 1, the even places | D's own | 16 | 25 |
| readout 1, the even places | readout 1's pattern targets | 2, 6, 8, 10, 12, 16, 18 | 252 |
| readout 1, the even places | readout 1's other excitatory | 2, 6, 8, 10, 12, 16, 18 | 80 |
| readout 1, the even places | inhibitory in readout 1 | 4, 14 | 102 |
| readout 0, the odd places | C's own | 5 | 25 |
| readout 0, the odd places | readout 0's pattern targets | 1, 3, 5, 7, 13, 15, 17 | 245 |
| readout 0, the odd places | readout 0's other excitatory | 1, 3, 5, 7, 13, 15, 17 | 87 |
| readout 0, the odd places | inhibitory in readout 0 | 9, 19 | 102 |

**Six forms, in two jobs**, exploration 033's. A night that replays is H-47's plain night on A's patterns: A's 102 patterns, drawn from the
start image, tagged at priority 200. Every other night is H-47's night without replay, the same night with nothing tagged. No hold is set.
Every night of every form is watched.

| Form | In the logs | Night after block 6 | after 12 | after 18 | after 24 | Job |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Without replay | `no-replay` | without replay | without replay | without replay | without replay | both |
| After block 6 alone | `alone-6` | replays | without replay | without replay | without replay | one |
| Plain on all four nights (H-47) | `plain-first` | replays | replays | replays | replays | one |
| After block 12 alone | `alone-12` | without replay | replays | without replay | without replay | two |
| After block 18 alone | `alone-18` | without replay | without replay | replays | without replay | two |
| After block 24 alone | `alone-24` | without replay | without replay | without replay | replays | two |

**The unit read** is exploration 033's: each of A's reads counts each readout unit's spikes in each trial's window, for **a unit's spikes a
trial** on each stimulus's trials, and the rows, the reads and their SEs are read as there.

**The input read.** On each image whose class matrix the job writes, the start image, the image before each night and the image after it,
and block 36's, the harness walks every unit's fan-out, as exploration 020's class matrix does (`e020_classes`), and for each synapse onto a
unit of the two readouts' sets adds its weight to that unit's sum for the class of the presynaptic unit, an inhibitory unit's weight
negated: fourteen sums a unit, 12 852 an image. Held inside the test on each image: each readout unit keeps the class it had on the
start image, and summed over the units of each of the eight classes the readouts' units make up, each presynaptic class's sums are that
class matrix's entry, 112 entries an image. Nothing under `src/` changed: the walk is `exec.units()` and `unit.fan_out(exec.blocks())`.

**The presynaptic groups**: A's kinds (classes 0 and 1, A alone and A and C), B's kinds (2 and 3, B alone and B and D), the odd places'
excitatory units (4, 6 and 7: C's own and readout 0's pattern targets and other excitatory units), the even places' (5, 8 and 9: D's own and
readout 1's two classes), the other excitatory units (10) and the inhibitory units (11 to 13). On this cell, as ADR-0249 gives it, class 10
holds only units 1 020 to 1 023, past the last whole period, and class 13 none; class 13's sum is 0 on every readout unit on each of the 191
images the jobs read.

**What is read**, on the mean over the three paths of each image: **a night's change** in an input is the image after the night less the
image before it; **a stretch's** the image before the next night, or block 36's, less the image after the night before it; **an input's
difference** is the form's image less the night without replay's on the same path at the same point, in the form's own job. A checkpoint's
image is the image after the night that follows its block, so right after its replayed night a single night's difference is that night's
change less the night without replay's same night on the same image: until then the two ran the same nights. A readout's input is the sum
over its 459 units; **a unit's input difference** in a set of units is the mean over the set.

**The gain** scales every unit's summed input and is read beside the inputs: an image's gain before a night is the night's start state's
and after it the end state's, the two the rows carry as the night's gain, held equal here; block 36's is the run line's of the twelve
blocks. A gain is written as a multiple of 1, its Q16.16 value over 65 536.

**The units that carried half of each readout's drop** are exploration 033's, named in its sections 3 and 4 by comparison, stimulus and
readout, and worked again here from the unit reads by exploration 033's rule, the same units in all 52 sets. **The rest** of a
set's readout is its other units.

**The correlation** of a readout's units' input difference from a group with their spikes' difference, on one stimulus's trials, is
Pearson's over the readout's 459 units, each unit's input difference the mean over the paths and its spikes' difference exploration 033's
mean over the 48 reads; a unit's input difference is the same on both stimuli's trials, and only its spikes' difference differs with the
stimulus.

## Before a job ran

ADR-0249 asks whether any form must run bit for bit as one already in the logs. **Each of the six does**: each is exploration 033's, every
image, row and read of it in exploration 033's logs. **No log holds a unit's inputs**, so each form ran again, and its rows, reads and unit
reads are set beside exploration 033's here.

## The runs are laid out as written

- **The trial** ran exploration 033's trial on path 0 of the cell: the pair's head and the first night of each of the six forms. Inside the
  test each image, its reads and its gain held H-47's row at block 6 on path 0 of the form whose first night it ran, and each image's input
  read was held to its class matrix: each of the six images' 918 units' sums made its class matrix's 112 entries, 672 of 672 in all, as this
  note's parser finds them too, and the trial prints it after each, 6 of 6. Each image's splits give each kind's sum onto each readout as
  the input read does, 6 of 6. All 1 228 lines of exploration 033's trial are among the trial's 1 240, label aside, and its other 12 are the
  input reads and their lines saying they held, 12.

- **The two jobs** ran to the end, each form on the three paths, everything exploration 033's jobs hold held inside the test, and each
  image's input read held to its class matrix:

| Job | Images read: the start image and each chain's nine | Their inputs the class matrix's 112 entries, by this note's parser | Entries | Lines saying so, as the job prints | Images whose splits give the same sums by kind onto each readout | Exploration 033's job's lines among these, label aside | Its other lines, input reads aside | Input read lines |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | ---: |
| Job one | 82 of 82 | 82 of 82 | 9 184 of 9 184 | 82 of 82 | 82 of 82 | 10 480 of 10 480 | 0 | 164 |
| Job two | 109 of 109 | 109 of 109 | 12 208 of 12 208 | 109 of 109 | 109 of 109 | 10 589 of 10 589 | 0 | 218 |

- **The lines.** Of exploration 033's job one's 10 480 lines, all are among job one's 10 644 here, label aside, and its other 164 are the
  input reads, two lines an image; of exploration 033's job two's 10 589, all are among job two's 10 807, and its other 218 are the input
  reads. So every class matrix, split, night line, row, read and unit read here is exploration 033's, and the class matrices are the images'
  exploration 033 read there.
- **The night without replay in both jobs**: its input reads on the three paths' nine images are the same in job one and job two, 27 of 27,
  and so is the start image's; its rows, reads and unit reads are exploration 033's in both (section 1).
- **The splits as a check.** A split prints each kind's sum onto each readout in three parts, the kinds in the order A alone, A and C, C's
  own, B alone, B and D, D's own, so classes 0, 1, 4, 2, 3 and 5. On every image of the two jobs the input read's sum over each readout's
  units from each of those six classes is the split's sum of that kind onto that readout: 191 of 191 images, 2 292 sums.

## 1. Rows, reads and unit reads against exploration 033's

Each form's rows, A's reads, the pair's reads and the unit reads are set beside exploration 033's of the same form, path, checkpoint and
stream, by the stream (`spread`'s index), not by the order the job printed them:

| Job | Form | Rows that are exploration 033's | A's reads whose count, masks and whole line are exploration 033's | The pair's | Unit reads that are exploration 033's |
| :--- | :--- | :--- | :--- | :--- | :--- |
| Job one | Without replay | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job one | After block 6 alone | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job one | Plain on all four nights (H-47) | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job two | Without replay | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job two | After block 12 alone | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job two | After block 18 alone | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 |
| Job two | After block 24 alone | 15 of 15 | 240 of 240 | 240 of 240 | 240 of 240 |

Every row, read and unit read is exploration 033's: 105 of 105 rows, 1 680 of 1 680 of A's reads and 1 680 of 1 680 of the pair's with
exploration 033's count, masks and whole line, label aside, and 1 680 of 1 680 unit reads with its trials and sums. So A's count, each
stimulus's correct trials, each readout's and each unit's spikes and their differences under each form are exploration 033's, and its
sections 1 to 4 give them; this note does not print them again.

## 2. What each replayed night changed in the readouts' inputs

**By presynaptic group**, each night's change onto each readout's 459 units, each replayed night beside the night without replay's same
night; a single night's replayed night and the night without replay's same night ran on the same image:

| Night after block | Form | Its job | This night | Onto | A's kinds (classes 0, 1), M | B's kinds (2, 3), M | the odd places' excitatory (4, 6, 7), M | the even places' (5, 8, 9), M | other excitatory (10), M | inhibitory (11 to 13), M | All excitatory | Excitatory less inhibitory |
| :--- | :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | After block 6 alone | job one | replays | readout 1, the even places | +0.31 | −0.32 | −2.56 | −1.68 | −0.04 | +4.39 | −4.29 | −8.69 |
| 6 | After block 6 alone | job one | replays | readout 0, the odd places | −0.31 | +0.32 | −1.28 | −2.90 | −0.05 | +4.32 | −4.21 | −8.53 |
| 6 | Plain on all four nights (H-47) | job one | replays | readout 1, the even places | +0.31 | −0.32 | −2.56 | −1.68 | −0.04 | +4.39 | −4.29 | −8.69 |
| 6 | Plain on all four nights (H-47) | job one | replays | readout 0, the odd places | −0.31 | +0.32 | −1.28 | −2.90 | −0.05 | +4.32 | −4.21 | −8.53 |
| 6 | Without replay | job one | without replay | readout 1, the even places | −0.04 | −0.01 | +0.01 | −0.16 | −0.02 | +0.24 | −0.22 | −0.46 |
| 6 | Without replay | job one | without replay | readout 0, the odd places | +0.04 | +0.01 | +0.02 | −0.05 | −0.03 | +0.31 | −0.01 | −0.32 |
| 12 | After block 12 alone | job two | replays | readout 1, the even places | +0.30 | −0.28 | −2.44 | −1.68 | −0.04 | +4.48 | −4.14 | −8.62 |
| 12 | After block 12 alone | job two | replays | readout 0, the odd places | −0.30 | +0.28 | −1.25 | −2.56 | −0.05 | +4.43 | −3.89 | −8.32 |
| 12 | Plain on all four nights (H-47) | job one | replays | readout 1, the even places | +0.23 | −0.24 | −1.92 | −1.16 | −0.02 | +3.57 | −3.10 | −6.67 |
| 12 | Plain on all four nights (H-47) | job one | replays | readout 0, the odd places | −0.23 | +0.24 | −0.90 | −2.18 | −0.02 | +3.54 | −3.09 | −6.63 |
| 12 | Without replay | job one | without replay | readout 1, the even places | −0.01 | 0.00 | +0.17 | −0.03 | −0.02 | +0.24 | +0.11 | −0.12 |
| 12 | Without replay | job one | without replay | readout 0, the odd places | +0.01 | 0.00 | −0.05 | +0.01 | −0.02 | +0.26 | −0.05 | −0.31 |
| 18 | After block 18 alone | job two | replays | readout 1, the even places | +0.25 | −0.28 | −2.25 | −1.62 | −0.02 | +4.32 | −3.91 | −8.23 |
| 18 | After block 18 alone | job two | replays | readout 0, the odd places | −0.25 | +0.28 | −1.25 | −2.45 | −0.04 | +4.18 | −3.71 | −7.89 |
| 18 | Plain on all four nights (H-47) | job one | replays | readout 1, the even places | +0.12 | −0.17 | −1.56 | −0.58 | −0.01 | +2.98 | −2.21 | −5.19 |
| 18 | Plain on all four nights (H-47) | job one | replays | readout 0, the odd places | −0.12 | +0.17 | −0.49 | −1.47 | −0.02 | +2.88 | −1.93 | −4.81 |
| 18 | Without replay | job one | without replay | readout 1, the even places | −0.05 | −0.01 | +0.18 | +0.07 | 0.00 | +0.25 | +0.19 | −0.06 |
| 18 | Without replay | job one | without replay | readout 0, the odd places | +0.05 | +0.01 | +0.03 | +0.06 | −0.01 | +0.31 | +0.14 | −0.18 |
| 24 | After block 24 alone | job two | replays | readout 1, the even places | +0.23 | −0.30 | −2.26 | −1.46 | −0.03 | +4.39 | −3.82 | −8.21 |
| 24 | After block 24 alone | job two | replays | readout 0, the odd places | −0.23 | +0.30 | −0.95 | −2.45 | −0.04 | +4.22 | −3.37 | −7.59 |
| 24 | Plain on all four nights (H-47) | job one | replays | readout 1, the even places | +0.18 | −0.11 | −1.14 | −0.43 | −0.02 | +2.63 | −1.52 | −4.15 |
| 24 | Plain on all four nights (H-47) | job one | replays | readout 0, the odd places | −0.18 | +0.11 | −0.48 | −1.13 | −0.01 | +2.48 | −1.68 | −4.16 |
| 24 | Without replay | job one | without replay | readout 1, the even places | −0.02 | +0.02 | +0.16 | +0.13 | +0.01 | +0.34 | +0.30 | −0.03 |
| 24 | Without replay | job one | without replay | readout 0, the odd places | +0.02 | −0.02 | +0.13 | −0.12 | −0.01 | +0.36 | −0.01 | −0.37 |

Over each single replayed night the inhibitory units' input rose onto both readouts, by 4.18 to 4.48 M (the least after block 18 alone onto
readout 0, the most after block 12 alone onto readout 1), and the place groups' excitatory input fell, each group's more onto the other
readout than onto its own: the odd places' excitatory units lost 2.25 to 2.56 M onto readout 1 and 0.95 to 1.28 onto readout 0, the even
places' 2.45 to 2.90 onto readout 0 and 1.46 to 1.68 onto readout 1, and the other excitatory units 0.02 to 0.05. A's kinds gained 0.23 to
0.31 M onto readout 1 and lost as much onto readout 0, and B's kinds lost 0.28 to 0.32 onto readout 1 and gained as much onto readout 0:
each of their sums onto the two readouts together moved by 45 or less in the class matrix's units, a mean over the paths. All excitatory
input onto readout 1 fell by 4.29, 4.14, 3.91 and 3.82 M after block 6, 12, 18 and 24 alone, and onto readout 0 by 4.21, 3.89, 3.71 and
3.37. The night without replay's same nights moved each excitatory group by 0.18 M or less (the most its night after block 18 onto readout
1), and raised the inhibitory input onto the readouts by 0.24 to 0.36. H-47's plain nights after block 12, 18 and 24 moved the inhibitory
input and each place group's onto each readout less than its first did, the inhibitory input onto readout 1 by 3.57, 2.98 and 2.63 M where
its first night raised it by 4.39.

**By presynaptic class**, the same nights, in M:

| Night after block | Form | This night | Onto | 0: A alone | 1: A and C | 2: B alone | 3: B and D | 4: C's own | 5: D's own | 6: readout 0's pattern targets | 7: readout 0's other excitatory | 8: readout 1's pattern targets | 9: readout 1's other excitatory | 10: other excitatory | 11: inhibitory in readout 0 | 12: inhibitory in readout 1 | 13: inhibitory elsewhere |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | After block 6 alone | replays | readout 1, the even places | +0.119 | +0.190 | −0.113 | −0.211 | +0.037 | +0.012 | −1.758 | −0.835 | −1.180 | −0.515 | −0.039 | +2.198 | +2.196 | 0.000 |
| 6 | After block 6 alone | replays | readout 0, the odd places | −0.119 | −0.190 | +0.113 | +0.211 | −0.037 | −0.012 | −0.791 | −0.448 | −2.177 | −0.708 | −0.048 | +2.195 | +2.130 | 0.000 |
| 6 | Plain on all four nights (H-47) | replays | readout 1, the even places | +0.119 | +0.190 | −0.113 | −0.211 | +0.037 | +0.012 | −1.758 | −0.835 | −1.180 | −0.515 | −0.039 | +2.198 | +2.196 | 0.000 |
| 6 | Plain on all four nights (H-47) | replays | readout 0, the odd places | −0.119 | −0.190 | +0.113 | +0.211 | −0.037 | −0.012 | −0.791 | −0.448 | −2.177 | −0.708 | −0.048 | +2.195 | +2.130 | 0.000 |
| 6 | Without replay | without replay | readout 1, the even places | −0.037 | +0.001 | +0.037 | −0.048 | +0.005 | +0.025 | +0.057 | −0.054 | −0.085 | −0.097 | −0.024 | +0.098 | +0.141 | 0.000 |
| 6 | Without replay | without replay | readout 0, the odd places | +0.037 | −0.001 | −0.037 | +0.048 | −0.005 | −0.025 | −0.007 | +0.034 | +0.017 | −0.042 | −0.028 | +0.162 | +0.147 | 0.000 |
| 12 | After block 12 alone | replays | readout 1, the even places | +0.153 | +0.151 | −0.068 | −0.209 | +0.019 | +0.022 | −1.695 | −0.766 | −1.149 | −0.556 | −0.040 | +2.222 | +2.255 | 0.000 |
| 12 | After block 12 alone | replays | readout 0, the odd places | −0.153 | −0.151 | +0.068 | +0.209 | −0.019 | −0.022 | −0.759 | −0.474 | −1.897 | −0.639 | −0.049 | +2.265 | +2.168 | 0.000 |
| 12 | Plain on all four nights (H-47) | replays | readout 1, the even places | +0.114 | +0.117 | −0.064 | −0.180 | +0.036 | +0.032 | −1.276 | −0.676 | −0.752 | −0.439 | −0.017 | +1.734 | +1.832 | 0.000 |
| 12 | Plain on all four nights (H-47) | replays | readout 0, the odd places | −0.114 | −0.117 | +0.064 | +0.180 | −0.036 | −0.032 | −0.479 | −0.387 | −1.526 | −0.624 | −0.018 | +1.859 | +1.681 | 0.000 |
| 12 | Without replay | without replay | readout 1, the even places | −0.031 | +0.023 | +0.051 | −0.055 | −0.009 | +0.042 | +0.170 | +0.005 | +0.019 | −0.088 | −0.016 | +0.101 | +0.134 | 0.000 |
| 12 | Without replay | without replay | readout 0, the odd places | +0.031 | −0.023 | −0.051 | +0.055 | +0.009 | −0.042 | −0.012 | −0.044 | +0.050 | −0.003 | −0.019 | +0.135 | +0.126 | 0.000 |
| 18 | After block 18 alone | replays | readout 1, the even places | +0.089 | +0.159 | −0.093 | −0.182 | −0.004 | −0.008 | −1.477 | −0.767 | −1.162 | −0.449 | −0.019 | +2.085 | +2.234 | 0.000 |
| 18 | After block 18 alone | replays | readout 0, the odd places | −0.089 | −0.159 | +0.093 | +0.182 | +0.004 | +0.008 | −0.769 | −0.489 | −1.894 | −0.561 | −0.039 | +2.082 | +2.093 | 0.000 |
| 18 | Plain on all four nights (H-47) | replays | readout 1, the even places | +0.042 | +0.073 | −0.067 | −0.099 | −0.020 | +0.011 | −0.968 | −0.572 | −0.257 | −0.338 | −0.015 | +1.381 | +1.597 | 0.000 |
| 18 | Plain on all four nights (H-47) | replays | readout 0, the odd places | −0.042 | −0.073 | +0.067 | +0.099 | +0.020 | −0.011 | −0.189 | −0.317 | −1.010 | −0.450 | −0.020 | +1.497 | +1.385 | 0.000 |
| 18 | Without replay | without replay | readout 1, the even places | −0.060 | +0.010 | +0.029 | −0.039 | −0.012 | +0.026 | +0.306 | −0.114 | +0.117 | −0.074 | +0.001 | +0.092 | +0.156 | 0.000 |
| 18 | Without replay | without replay | readout 0, the odd places | +0.060 | −0.010 | −0.029 | +0.039 | +0.012 | −0.026 | +0.061 | −0.045 | +0.062 | +0.024 | −0.013 | +0.155 | +0.158 | 0.000 |
| 24 | After block 24 alone | replays | readout 1, the even places | +0.093 | +0.134 | −0.133 | −0.165 | +0.013 | +0.025 | −1.503 | −0.769 | −1.042 | −0.446 | −0.029 | +2.254 | +2.136 | 0.000 |
| 24 | After block 24 alone | replays | readout 0, the odd places | −0.093 | −0.134 | +0.133 | +0.165 | −0.013 | −0.025 | −0.480 | −0.461 | −1.889 | −0.537 | −0.037 | +2.137 | +2.085 | 0.000 |
| 24 | Plain on all four nights (H-47) | replays | readout 1, the even places | +0.058 | +0.118 | −0.017 | −0.095 | −0.006 | +0.023 | −0.623 | −0.508 | −0.181 | −0.271 | −0.022 | +1.288 | +1.340 | 0.000 |
| 24 | Plain on all four nights (H-47) | replays | readout 0, the odd places | −0.058 | −0.118 | +0.017 | +0.095 | +0.006 | −0.024 | −0.232 | −0.252 | −0.731 | −0.371 | −0.010 | +1.268 | +1.216 | 0.000 |
| 24 | Without replay | without replay | readout 1, the even places | −0.031 | +0.013 | +0.049 | −0.028 | +0.016 | +0.031 | +0.219 | −0.077 | +0.123 | −0.022 | +0.011 | +0.190 | +0.148 | 0.000 |
| 24 | Without replay | without replay | readout 0, the odd places | +0.031 | −0.013 | −0.049 | +0.028 | −0.016 | −0.031 | +0.136 | +0.009 | −0.128 | +0.042 | −0.015 | +0.173 | +0.186 | 0.000 |

Onto each readout the other readout's pattern targets lost the most of the excitatory classes over each single replayed night: readout 0's
pattern targets 1.48 to 1.76 M onto readout 1, and readout 1's 1.89 to 2.18 onto readout 0, where each readout's own pattern targets lost
1.04 to 1.18 onto readout 1 and 0.48 to 0.79 onto readout 0. The inhibitory units in readout 0 raised their input onto the readouts by 2.08
to 2.27 M and those in readout 1 by 2.09 to 2.26; class 13 holds none. Each of A's and B's four kinds, C's own and D's own lost onto one
readout what it gained onto the other: their sums onto the two readouts moved by 60 or less in the class matrix's units.

**By the readouts' classes**, each single night's replayed night and the night without replay's same night, by group, in M:

| Night after block | Form | This night | Onto the class | A's kinds (classes 0, 1) | B's kinds (2, 3) | the odd places' excitatory (4, 6, 7) | the even places' (5, 8, 9) | other excitatory (10) | inhibitory (11 to 13) |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | After block 6 alone | replays | D's own | +0.04 | −0.02 | −0.21 | −0.02 | 0.00 | +0.30 |
| 6 | After block 6 alone | replays | readout 1's pattern targets | +0.36 | −0.18 | −1.96 | −1.03 | −0.02 | +3.15 |
| 6 | After block 6 alone | replays | readout 1's other excitatory | 0.00 | −0.02 | −0.06 | −0.22 | −0.01 | +0.34 |
| 6 | After block 6 alone | replays | inhibitory in readout 1 | −0.09 | −0.10 | −0.32 | −0.41 | −0.01 | +0.61 |
| 6 | After block 6 alone | replays | C's own | −0.02 | +0.04 | +0.01 | −0.27 | 0.00 | +0.16 |
| 6 | After block 6 alone | replays | readout 0's pattern targets | −0.28 | +0.34 | −0.68 | −2.10 | −0.04 | +3.05 |
| 6 | After block 6 alone | replays | readout 0's other excitatory | −0.01 | 0.00 | −0.12 | −0.03 | 0.00 | +0.51 |
| 6 | After block 6 alone | replays | inhibitory in readout 0 | −0.01 | −0.05 | −0.49 | −0.49 | −0.01 | +0.60 |
| 6 | Without replay | without replay | D's own | 0.00 | +0.01 | −0.02 | −0.01 | 0.00 | −0.04 |
| 6 | Without replay | without replay | readout 1's pattern targets | −0.03 | −0.02 | −0.07 | −0.33 | −0.01 | +0.17 |
| 6 | Without replay | without replay | readout 1's other excitatory | 0.00 | +0.01 | −0.01 | −0.05 | 0.00 | +0.09 |
| 6 | Without replay | without replay | inhibitory in readout 1 | 0.00 | −0.01 | +0.12 | +0.23 | −0.01 | +0.03 |
| 6 | Without replay | without replay | C's own | −0.01 | 0.00 | −0.04 | 0.00 | 0.00 | −0.03 |
| 6 | Without replay | without replay | readout 0's pattern targets | +0.05 | −0.04 | −0.22 | −0.09 | −0.02 | +0.19 |
| 6 | Without replay | without replay | readout 0's other excitatory | −0.01 | 0.00 | +0.05 | −0.04 | 0.00 | +0.09 |
| 6 | Without replay | without replay | inhibitory in readout 0 | +0.02 | +0.05 | +0.23 | +0.08 | 0.00 | +0.05 |
| 12 | After block 12 alone | replays | D's own | +0.04 | −0.01 | −0.19 | +0.01 | 0.00 | +0.29 |
| 12 | After block 12 alone | replays | readout 1's pattern targets | +0.37 | −0.16 | −1.85 | −0.96 | −0.01 | +3.18 |
| 12 | After block 12 alone | replays | readout 1's other excitatory | 0.00 | −0.02 | −0.03 | −0.21 | −0.01 | +0.37 |
| 12 | After block 12 alone | replays | inhibitory in readout 1 | −0.11 | −0.09 | −0.37 | −0.52 | −0.02 | +0.64 |
| 12 | After block 12 alone | replays | C's own | −0.02 | +0.03 | +0.01 | −0.25 | 0.00 | +0.17 |
| 12 | After block 12 alone | replays | readout 0's pattern targets | −0.23 | +0.31 | −0.53 | −1.77 | −0.04 | +3.14 |
| 12 | After block 12 alone | replays | readout 0's other excitatory | −0.03 | 0.00 | −0.14 | −0.05 | 0.00 | +0.51 |
| 12 | After block 12 alone | replays | inhibitory in readout 0 | −0.03 | −0.06 | −0.59 | −0.50 | 0.00 | +0.62 |
| 12 | Without replay | without replay | D's own | −0.01 | +0.01 | −0.03 | −0.01 | 0.00 | −0.07 |
| 12 | Without replay | without replay | readout 1's pattern targets | −0.01 | −0.01 | −0.02 | −0.15 | 0.00 | +0.15 |
| 12 | Without replay | without replay | readout 1's other excitatory | 0.00 | −0.01 | +0.06 | −0.03 | 0.00 | +0.09 |
| 12 | Without replay | without replay | inhibitory in readout 1 | +0.01 | +0.01 | +0.16 | +0.17 | −0.01 | +0.06 |
| 12 | Without replay | without replay | C's own | −0.01 | −0.02 | −0.01 | +0.01 | 0.00 | −0.03 |
| 12 | Without replay | without replay | readout 0's pattern targets | +0.04 | −0.03 | −0.18 | −0.06 | −0.01 | +0.18 |
| 12 | Without replay | without replay | readout 0's other excitatory | −0.02 | 0.00 | +0.01 | −0.02 | 0.00 | +0.06 |
| 12 | Without replay | without replay | inhibitory in readout 0 | 0.00 | +0.05 | +0.13 | +0.08 | 0.00 | +0.05 |
| 18 | After block 18 alone | replays | D's own | +0.04 | −0.01 | −0.20 | +0.04 | 0.00 | +0.26 |
| 18 | After block 18 alone | replays | readout 1's pattern targets | +0.33 | −0.15 | −1.66 | −0.89 | −0.01 | +3.09 |
| 18 | After block 18 alone | replays | readout 1's other excitatory | 0.00 | 0.00 | −0.03 | −0.22 | −0.01 | +0.36 |
| 18 | After block 18 alone | replays | inhibitory in readout 1 | −0.12 | −0.11 | −0.35 | −0.55 | 0.00 | +0.62 |
| 18 | After block 18 alone | replays | C's own | −0.01 | +0.01 | +0.01 | −0.25 | 0.00 | +0.13 |
| 18 | After block 18 alone | replays | readout 0's pattern targets | −0.21 | +0.31 | −0.48 | −1.69 | −0.03 | +2.97 |
| 18 | After block 18 alone | replays | readout 0's other excitatory | −0.02 | 0.00 | −0.17 | 0.00 | 0.00 | +0.45 |
| 18 | After block 18 alone | replays | inhibitory in readout 0 | −0.01 | −0.04 | −0.62 | −0.50 | 0.00 | +0.62 |
| 18 | Without replay | without replay | D's own | −0.01 | 0.00 | −0.04 | 0.00 | 0.00 | −0.08 |
| 18 | Without replay | without replay | readout 1's pattern targets | −0.03 | −0.02 | +0.10 | −0.12 | 0.00 | +0.15 |
| 18 | Without replay | without replay | readout 1's other excitatory | 0.00 | +0.01 | +0.04 | +0.02 | 0.00 | +0.11 |
| 18 | Without replay | without replay | inhibitory in readout 1 | −0.01 | 0.00 | +0.08 | +0.16 | 0.00 | +0.07 |
| 18 | Without replay | without replay | C's own | 0.00 | 0.00 | −0.02 | 0.00 | 0.00 | −0.04 |
| 18 | Without replay | without replay | readout 0's pattern targets | +0.07 | −0.03 | −0.08 | −0.06 | −0.01 | +0.21 |
| 18 | Without replay | without replay | readout 0's other excitatory | −0.02 | 0.00 | −0.02 | −0.01 | 0.00 | +0.07 |
| 18 | Without replay | without replay | inhibitory in readout 0 | +0.01 | +0.04 | +0.15 | +0.13 | 0.00 | +0.08 |
| 24 | After block 24 alone | replays | D's own | 0.00 | −0.01 | −0.19 | 0.00 | 0.00 | +0.24 |
| 24 | After block 24 alone | replays | readout 1's pattern targets | +0.33 | −0.19 | −1.68 | −0.79 | −0.02 | +3.09 |
| 24 | After block 24 alone | replays | readout 1's other excitatory | 0.00 | −0.01 | −0.01 | −0.16 | −0.01 | +0.36 |
| 24 | After block 24 alone | replays | inhibitory in readout 1 | −0.10 | −0.08 | −0.37 | −0.52 | 0.00 | +0.70 |
| 24 | After block 24 alone | replays | C's own | −0.02 | +0.03 | +0.05 | −0.23 | 0.00 | +0.12 |
| 24 | After block 24 alone | replays | readout 0's pattern targets | −0.19 | +0.35 | −0.38 | −1.75 | −0.03 | +3.01 |
| 24 | After block 24 alone | replays | readout 0's other excitatory | 0.00 | 0.00 | −0.07 | +0.03 | 0.00 | +0.49 |
| 24 | After block 24 alone | replays | inhibitory in readout 0 | −0.02 | −0.08 | −0.55 | −0.50 | −0.01 | +0.61 |
| 24 | Without replay | without replay | D's own | −0.01 | +0.02 | −0.03 | +0.01 | 0.00 | −0.08 |
| 24 | Without replay | without replay | readout 1's pattern targets | −0.02 | −0.02 | +0.05 | −0.04 | +0.01 | +0.21 |
| 24 | Without replay | without replay | readout 1's other excitatory | 0.00 | +0.01 | +0.06 | −0.01 | 0.00 | +0.09 |
| 24 | Without replay | without replay | inhibitory in readout 1 | +0.02 | +0.01 | +0.08 | +0.17 | +0.01 | +0.12 |
| 24 | Without replay | without replay | C's own | −0.01 | −0.01 | 0.00 | −0.03 | 0.00 | −0.03 |
| 24 | Without replay | without replay | readout 0's pattern targets | +0.04 | −0.03 | −0.06 | −0.14 | −0.01 | +0.24 |
| 24 | Without replay | without replay | readout 0's other excitatory | −0.03 | 0.00 | +0.04 | −0.03 | 0.00 | +0.06 |
| 24 | Without replay | without replay | inhibitory in readout 0 | +0.02 | +0.01 | +0.16 | +0.08 | 0.00 | +0.09 |

Of each readout's inhibitory rise over each single replayed night its pattern targets received 70 to 72 per cent, 12 112 to 12 817 a unit,
and its inhibitory units 5 915 to 6 858 a unit, in the class matrix's units.

**The units that carried half of each readout's drop** right after each single replayed night (exploration 033, section 3), against the
rest of their readout: each set's spikes' difference a unit and its input difference a unit from each group, the mean over its units, in
the class matrix's units; other excitatory aside:

| Comparison | Trials | Readout | Units carrying half its drop (exploration 033) | Their spikes' difference a unit | The rest's | A's kinds: their input difference a unit | the rest's | B's kinds: theirs | the rest's | The odd places' excitatory: theirs | the rest's | The even places': theirs | the rest's | Inhibitory: theirs | the rest's | Excitatory less inhibitory: theirs | the rest's |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | 16 | −0.23 | −0.01 | −7 967 | +1 065 | +9 | −706 | −6 676 | −5 544 | −7 430 | −3 174 | +749 | +9 350 | −22 072 | −17 771 |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | 15 | −0.27 | −0.01 | −10 144 | −433 | +594 | +684 | −6 274 | −2 713 | −24 054 | −5 601 | +455 | +9 028 | −39 757 | −17 156 |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | 15 | −0.19 | −0.01 | +1 704 | +718 | −11 194 | −326 | −7 121 | −5 531 | −5 829 | −3 238 | +3 663 | +9 232 | −26 096 | −17 644 |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | 18 | −0.18 | −0.01 | −1 410 | −723 | −8 272 | +1 047 | −4 494 | −2 761 | −15 099 | −5 841 | +606 | +9 080 | −30 443 | −17 383 |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | 18 | −0.26 | −0.01 | −7 226 | +1 002 | −402 | −602 | −8 828 | −5 557 | −8 736 | −3 398 | +1 129 | +9 573 | −26 170 | −18 188 |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | 15 | −0.26 | −0.01 | −8 739 | −407 | −63 | +616 | −2 933 | −2 616 | −17 804 | −5 174 | +5 211 | +9 219 | −35 001 | −16 857 |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | 19 | −0.15 | −0.01 | +779 | +675 | −7 863 | −280 | −8 529 | −5 562 | −7 376 | −3 445 | +3 955 | +9 470 | −26 936 | −18 137 |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | 17 | −0.19 | −0.01 | −1 608 | −643 | −7 788 | +916 | −2 740 | −2 622 | −16 854 | −5 153 | +920 | +9 402 | −30 073 | −16 964 |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | 19 | −0.27 | −0.01 | −7 584 | +1 005 | −219 | −593 | −8 581 | −5 146 | −11 237 | −3 352 | +1 901 | +9 171 | −29 523 | −17 303 |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | 17 | −0.22 | −0.01 | −10 330 | −277 | +970 | +562 | −2 141 | −2 817 | −16 517 | −5 039 | +4 460 | +8 568 | −32 424 | −16 199 |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | 15 | −0.18 | −0.01 | +378 | +658 | −7 057 | −358 | −9 696 | −5 139 | −10 121 | −3 460 | +5 535 | +8 983 | −32 014 | −17 329 |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | 18 | −0.19 | −0.01 | −1 797 | −602 | −6 365 | +861 | −6 421 | −2 644 | −15 970 | −5 035 | +1 029 | +8 717 | −31 935 | −16 182 |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | 19 | −0.26 | −0.01 | −8 344 | +917 | +44 | −728 | −10 766 | −5 028 | −9 365 | −3 223 | +789 | +9 177 | −29 221 | −17 329 |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | 17 | −0.18 | −0.01 | −8 438 | −230 | +607 | +699 | −6 213 | −2 212 | −8 409 | −4 957 | +3 390 | +8 609 | −25 843 | −15 357 |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | 13 | −0.24 | −0.01 | +958 | +521 | −9 682 | −434 | −7 064 | −5 213 | −8 747 | −3 323 | +6 269 | +8 904 | −30 834 | −17 442 |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | 14 | −0.25 | −0.01 | −1 221 | −512 | −8 422 | +983 | −2 636 | −2 352 | −12 838 | −4 841 | +797 | +8 655 | −26 073 | −15 420 |

Right after each single replayed night, on A's trials the units that carried half of each readout's drop had an input difference from A's
kinds of −10 330 to −7 226 a unit, below 0 in 8 of 8 sets, where the rest of their readout had −433 to +1 065; on B's trials theirs from B's
kinds −11 194 to −6 365, below 0 in 8 of 8, and the rest's −434 to +1 047. Theirs from the trial's stimulus's kinds lay below the rest's in
16 of the 16 sets. Their excitatory input less inhibitory input difference read −39 757 to −22 072 a unit and the rest's −18 188 to −15 357,
theirs below the rest's in 16 of the 16, at 1.24 to 2.32 times the rest's; their excitatory input difference lay below the rest's in 16, and
their inhibitory input difference, +455 to +6 269 a unit against the rest's +8 568 to +9 573, above the rest's in 0. Their spikes'
difference a unit read −0.27 to −0.15 and the rest's −0.01 (exploration 033).

**The gain**, before and after each night:

| Night after block | Form | This night | Gain before the night | After it | Its change |
| :--- | :--- | :--- | ---: | ---: | :--- |
| 6 | After block 6 alone | replays | 1.694 | 1.632 | −0.062 |
| 6 | Plain on all four nights (H-47) | replays | 1.694 | 1.632 | −0.062 |
| 6 | Without replay | without replay | 1.694 | 1.727 | +0.033 |
| 12 | After block 12 alone | replays | 1.690 | 1.624 | −0.065 |
| 12 | Plain on all four nights (H-47) | replays | 1.699 | 1.643 | −0.057 |
| 12 | Without replay | without replay | 1.690 | 1.723 | +0.033 |
| 18 | After block 18 alone | replays | 1.683 | 1.622 | −0.061 |
| 18 | Plain on all four nights (H-47) | replays | 1.700 | 1.654 | −0.046 |
| 18 | Without replay | without replay | 1.683 | 1.721 | +0.037 |
| 24 | After block 24 alone | replays | 1.675 | 1.612 | −0.063 |
| 24 | Plain on all four nights (H-47) | replays | 1.698 | 1.656 | −0.042 |
| 24 | Without replay | without replay | 1.675 | 1.712 | +0.037 |

Each single replayed night lowered the gain, by 0.061 to 0.065 (the least after block 18 alone, the most after block 12 alone), where the
night without replay's same nights raised it, by 0.033 to 0.037; H-47's later plain nights lowered it by 0.042 to 0.057. The gain on the
image before the nights after block 6, 12, 18 and 24, which each single night's replayed night started from, was 1.694, 1.690, 1.683 and
1.675.

**The image each replayed night starts from**: a single night's image before its replayed night is the night without replay's image before
the same night, in M, with its gain:

| Image before the night after block | Onto | A's kinds (classes 0, 1), M | B's kinds (2, 3), M | the odd places' excitatory (4, 6, 7), M | the even places' (5, 8, 9), M | other excitatory (10), M | inhibitory (11 to 13), M | Its gain |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
| 6 | readout 1, the even places | +6.61 | +6.63 | +40.49 | +42.04 | +0.36 | +25.70 | 1.694 |
| 6 | readout 0, the odd places | +6.09 | +6.47 | +42.42 | +41.31 | +0.36 | +27.44 | 1.694 |
| 12 | readout 1, the even places | +6.44 | +6.67 | +40.43 | +41.91 | +0.34 | +26.22 | 1.690 |
| 12 | readout 0, the odd places | +6.26 | +6.44 | +42.51 | +41.22 | +0.34 | +27.92 | 1.690 |
| 18 | readout 1, the even places | +6.37 | +6.71 | +40.56 | +41.90 | +0.32 | +26.72 | 1.683 |
| 18 | readout 0, the odd places | +6.33 | +6.39 | +42.49 | +41.21 | +0.32 | +28.27 | 1.683 |
| 24 | readout 1, the even places | +6.26 | +6.71 | +40.69 | +41.96 | +0.32 | +27.31 | 1.675 |
| 24 | readout 0, the odd places | +6.44 | +6.40 | +42.57 | +41.28 | +0.30 | +28.77 | 1.675 |

| Image before the night after block | A's kinds onto readout 1, M | onto readout 0 | readout 1 less readout 0 | B's kinds onto readout 0 | onto readout 1 | readout 0 less readout 1 | A's margin |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | +6.61 | +6.09 | +0.52 | +6.47 | +6.63 | −0.16 | +0.36 |
| 12 | +6.44 | +6.26 | +0.17 | +6.44 | +6.67 | −0.23 | −0.06 |
| 18 | +6.37 | +6.33 | +0.04 | +6.39 | +6.71 | −0.32 | −0.28 |
| 24 | +6.26 | +6.44 | −0.19 | +6.40 | +6.71 | −0.31 | −0.50 |

On the image each replayed night starts from, A's kinds' input onto readout 1 less onto readout 0 read +0.52, +0.17, +0.04 and −0.19 M
before the nights after block 6, 12, 18 and 24, B's kinds' onto readout 0 less onto readout 1 −0.16, −0.23, −0.32 and −0.31, and A's margin
+0.36, −0.06, −0.28 and −0.50. The inhibitory input onto readout 1 was 25.70 M before the night after block 6 and 27.31 before the night
after block 24, onto readout 0 27.44 and 28.77; of A's and B's kinds' and each place group's input onto each readout, A's kinds' moved the
most between the two, by 0.35 M onto each readout: down onto readout 1 and up onto readout 0.

## 3. The inputs against the spikes

**The correlation** over each readout's 459 units of a unit's input difference from each group right after the replayed night with its
spikes' difference there (exploration 033); "–" where a group's input difference is the same on every unit:

| Comparison | Trials | Readout | Its spikes' difference (exploration 033) | A's kinds | B's kinds | The odd places' excitatory | The even places' | Other excitatory | Inhibitory | All excitatory | Excitatory less inhibitory |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | A's trials | readout 1, the even places | −7.45 | +0.60 | −0.03 | +0.12 | +0.13 | −0.12 | +0.17 | +0.28 | +0.12 |
| After block 6 alone less without replay, block 6 | A's trials | readout 0, the odd places | −7.92 | +0.63 | −0.01 | +0.08 | +0.38 | −0.15 | +0.15 | +0.43 | +0.28 |
| After block 6 alone less without replay, block 6 | B's trials | readout 1, the even places | −5.47 | −0.07 | +0.72 | +0.10 | +0.05 | −0.01 | +0.11 | +0.26 | +0.15 |
| After block 6 alone less without replay, block 6 | B's trials | readout 0, the odd places | −6.27 | +0.09 | +0.62 | +0.07 | +0.24 | +0.02 | +0.18 | +0.35 | +0.20 |
| After block 12 alone less without replay, block 12 | A's trials | readout 1, the even places | −9.19 | +0.61 | 0.00 | +0.12 | +0.18 | −0.05 | +0.20 | +0.34 | +0.15 |
| After block 12 alone less without replay, block 12 | A's trials | readout 0, the odd places | −7.73 | +0.60 | +0.01 | +0.06 | +0.28 | +0.04 | +0.09 | +0.36 | +0.23 |
| After block 12 alone less without replay, block 12 | B's trials | readout 1, the even places | −5.59 | +0.02 | +0.61 | +0.07 | +0.12 | −0.02 | +0.13 | +0.27 | +0.14 |
| After block 12 alone less without replay, block 12 | B's trials | readout 0, the odd places | −6.49 | +0.12 | +0.58 | +0.06 | +0.26 | +0.02 | +0.17 | +0.37 | +0.19 |
| After block 18 alone less without replay, block 18 | A's trials | readout 1, the even places | −10.19 | +0.62 | −0.01 | +0.14 | +0.24 | −0.03 | +0.18 | +0.39 | +0.22 |
| After block 18 alone less without replay, block 18 | A's trials | readout 0, the odd places | −7.23 | +0.70 | −0.04 | +0.03 | +0.22 | −0.04 | +0.10 | +0.31 | +0.20 |
| After block 18 alone less without replay, block 18 | B's trials | readout 1, the even places | −5.31 | −0.02 | +0.51 | +0.15 | +0.13 | −0.02 | +0.08 | +0.28 | +0.19 |
| After block 18 alone less without replay, block 18 | B's trials | readout 0, the odd places | −6.74 | +0.07 | +0.57 | +0.11 | +0.26 | +0.08 | +0.17 | +0.39 | +0.21 |
| After block 24 alone less without replay, block 24 | A's trials | readout 1, the even places | −9.75 | +0.61 | −0.04 | +0.21 | +0.22 | −0.02 | +0.19 | +0.39 | +0.21 |
| After block 24 alone less without replay, block 24 | A's trials | readout 0, the odd places | −6.03 | +0.60 | 0.00 | +0.14 | +0.08 | −0.01 | +0.09 | +0.27 | +0.16 |
| After block 24 alone less without replay, block 24 | B's trials | readout 1, the even places | −6.21 | −0.04 | +0.55 | +0.10 | +0.14 | −0.02 | +0.05 | +0.26 | +0.19 |
| After block 24 alone less without replay, block 24 | B's trials | readout 0, the odd places | −6.77 | +0.02 | +0.55 | +0.07 | +0.25 | +0.01 | +0.15 | +0.34 | +0.18 |

Right after each single replayed night, over the 16 sets the correlation of a unit's input difference from the trial's stimulus's kinds with
its spikes' difference read +0.51 to +0.72, 16 of the 16 above 0: A's kinds' on A's trials +0.60 to +0.70 and B's kinds' on B's trials +0.51
to +0.72; from the other stimulus's kinds −0.07 to +0.12. Of its excitatory input less inhibitory input difference it read +0.12 to +0.28
(after block 6 alone at block 6, readout 1 on A's trials the least, after block 6 alone at block 6, readout 0 on A's trials the most), 16 of
the 16 above 0; of its inhibitory input difference +0.05 to +0.20, 16 above 0; of its excitatory input difference +0.26 to +0.43, 16 above
0; of the odd places' excitatory +0.03 to +0.21 and of the even places' +0.05 to +0.38.

**A's kinds onto each readout**, each single night's difference right after its replayed night and H-47's plain night's at blocks 6 to 24,
beside the spikes' readout 1 less readout 0 on A's trials and readout 0 less readout 1 on B's in exploration 033, whose SEs are there:

| Comparison | A's kinds onto readout 1, its difference, M | onto readout 0 | readout 1 less readout 0 | B's kinds onto readout 0 | onto readout 1 | readout 0 less readout 1 | Spikes on A's trials: readout 1 less readout 0 (exploration 033) | On B's trials: readout 0 less readout 1 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | +0.34 | −0.34 | +0.69 | +0.31 | −0.31 | +0.63 | +0.48 | −0.80 |
| After block 12 alone less without replay, block 12 | +0.31 | −0.31 | +0.62 | +0.27 | −0.27 | +0.55 | −1.46 | −0.90 |
| After block 18 alone less without replay, block 18 | +0.30 | −0.30 | +0.60 | +0.26 | −0.26 | +0.53 | −2.96 | −1.43 |
| After block 24 alone less without replay, block 24 | +0.24 | −0.24 | +0.49 | +0.32 | −0.32 | +0.64 | −3.73 | −0.56 |
| Plain on all four nights less without replay, block 6 | +0.34 | −0.34 | +0.69 | +0.31 | −0.31 | +0.63 | +0.48 | −0.80 |
| Plain on all four nights less without replay, block 12 | +0.57 | −0.57 | +1.13 | +0.57 | −0.57 | +1.14 | +0.22 | −0.51 |
| Plain on all four nights less without replay, block 18 | +0.75 | −0.75 | +1.49 | +0.72 | −0.72 | +1.44 | −1.18 | −0.83 |
| Plain on all four nights less without replay, block 24 | +0.97 | −0.97 | +1.93 | +0.87 | −0.87 | +1.73 | −1.52 | −0.43 |

Right after each single replayed night A's kinds' input onto readout 1 lay above the night without replay's by 0.24 to 0.34 M and onto
readout 0 below it: readout 1's difference less readout 0's read +0.49 to +0.69 (after block 6 alone the most, after block 24 alone the
least). So after none of the four single nights did readout 1's units lose more input from A's kinds than readout 0's; after each they
gained more, where their spikes on A's trials, readout 1 less readout 0, read +0.48, −1.46, −2.96 and −3.73 after block 6, 12, 18 and 24
alone. B's kinds' onto readout 0 less onto readout 1 read +0.53 to +0.64. Under H-47's plain night A's kinds' readout 1 less readout 0 read
+1.13, +1.49 and +1.93 at blocks 12, 18 and 24.

**Readout 1 against readout 0**: each group's difference onto readout 1 less onto readout 0, and each readout's excitatory input less its
inhibitory input, each comparison's, beside the spikes' readout 1 less readout 0 on A's trials and readout 0 less readout 1 on B's
(exploration 033):

| Comparison | A's kinds (classes 0, 1): readout 1's difference less readout 0's, M | B's kinds (2, 3): readout 1's difference less readout 0's, M | the odd places' excitatory (4, 6, 7): readout 1's difference less readout 0's, M | the even places' (5, 8, 9): readout 1's difference less readout 0's, M | other excitatory (10): readout 1's difference less readout 0's, M | inhibitory (11 to 13): readout 1's difference less readout 0's, M | Excitatory less inhibitory: readout 1's difference | readout 0's | readout 1's less readout 0's | Spikes on A's trials: readout 1 less readout 0 (exploration 033) | On B's trials: readout 0 less readout 1 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | +0.69 | −0.63 | −1.26 | +1.32 | +0.01 | +0.14 | −8.23 | −8.21 | −0.01 | +0.48 | −0.80 |
| After block 12 alone less without replay, block 12 | +0.62 | −0.55 | −1.40 | +0.91 | +0.01 | +0.07 | −8.49 | −8.01 | −0.48 | −1.46 | −0.90 |
| After block 18 alone less without replay, block 18 | +0.60 | −0.53 | −1.15 | +0.82 | +0.01 | +0.21 | −8.17 | −7.71 | −0.46 | −2.96 | −1.43 |
| After block 24 alone less without replay, block 24 | +0.49 | −0.64 | −1.33 | +0.74 | −0.02 | +0.19 | −8.18 | −7.23 | −0.95 | −3.73 | −0.56 |
| Plain on all four nights less without replay, block 6 | +0.69 | −0.63 | −1.26 | +1.32 | +0.01 | +0.14 | −8.23 | −8.21 | −0.01 | +0.48 | −0.80 |
| Plain on all four nights less without replay, block 12 | +1.13 | −1.14 | −2.48 | +2.34 | 0.00 | +0.12 | −13.90 | −13.64 | −0.26 | +0.22 | −0.51 |
| Plain on all four nights less without replay, block 18 | +1.49 | −1.44 | −3.69 | +3.23 | −0.01 | +0.21 | −18.01 | −17.39 | −0.61 | −1.18 | −0.83 |
| Plain on all four nights less without replay, block 24 | +1.93 | −1.73 | −4.33 | +3.69 | −0.04 | +0.26 | −20.96 | −20.23 | −0.73 | −1.52 | −0.43 |
| Plain on all four nights less without replay, block 36 | +2.03 | −1.84 | −4.29 | +3.67 | −0.04 | +0.12 | −18.95 | −18.36 | −0.60 | +0.17 | +0.65 |
| After block 6 alone less without replay, block 36 | +0.60 | −0.50 | −1.18 | +0.82 | −0.03 | +0.03 | −4.73 | −4.41 | −0.32 | −0.41 | −0.37 |
| After block 12 alone less without replay, block 36 | +0.76 | −0.49 | −0.99 | +0.54 | −0.02 | −0.05 | −5.15 | −5.00 | −0.14 | −0.53 | −0.54 |
| After block 18 alone less without replay, block 36 | +0.61 | −0.48 | −0.82 | +0.72 | −0.01 | +0.21 | −5.52 | −5.34 | −0.18 | −1.69 | −0.56 |
| After block 24 alone less without replay, block 36 | +0.55 | −0.71 | −1.27 | +0.74 | −0.02 | +0.09 | −6.44 | −5.64 | −0.80 | −2.22 | +0.32 |

Right after each single replayed night readout 1's excitatory input less its inhibitory input fell further than readout 0's: by 0.01, 0.48,
0.46 and 0.95 M more after block 6, 12, 18 and 24 alone, where the spikes' readout 1 less readout 0 on A's trials read +0.48, −1.46, −2.96
and −3.73 and readout 0 less readout 1 on B's trials −0.80, −0.90, −1.43 and −0.56. The odd places' excitatory units lost 1.15 to 1.40 M
more onto readout 1 than onto readout 0, the even places' 0.74 to 1.32 more onto readout 0, and readout 1's inhibitory input rose by 0.07 to
0.21 more than readout 0's. At block 36 readout 1's excitatory less inhibitory input still lay 0.32, 0.14, 0.18 and 0.80 M further below
than readout 0's under after block 6, 12, 18 and 24 alone; under H-47's plain night readout 1's less readout 0's read −0.01, −0.26, −0.61,
−0.73, −0.60 at blocks 6, 12, 18, 24 and 36.

**By the readouts' classes**, each class's input difference from each group and its excitatory input less its inhibitory input, beside the
class's spikes' difference on each stimulus's trials with its SE (exploration 033, sections 3 and 4):

| Comparison | Class | How many units | A's kinds (classes 0, 1), M | B's kinds (2, 3), M | the odd places' excitatory (4, 6, 7), M | the even places' (5, 8, 9), M | inhibitory (11 to 13), M | Excitatory less inhibitory | Spikes on A's trials (exploration 033) | On B's trials |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 6 | D's own | 25 | +0.04 | −0.03 | −0.19 | −0.01 | +0.34 | −0.52 | −0.35 ± 0.03 | −0.53 ± 0.03 |
| After block 6 alone less without replay, block 6 | readout 1's pattern targets | 252 | +0.40 | −0.16 | −1.89 | −0.70 | +2.98 | −5.34 | −3.70 ± 0.12 | −2.75 ± 0.10 |
| After block 6 alone less without replay, block 6 | readout 1's other excitatory | 80 | 0.00 | −0.03 | −0.04 | −0.18 | +0.26 | −0.52 | −0.54 ± 0.04 | −0.91 ± 0.06 |
| After block 6 alone less without replay, block 6 | inhibitory in readout 1 | 102 | −0.10 | −0.09 | −0.44 | −0.64 | +0.58 | −1.84 | −2.86 ± 0.10 | −1.28 ± 0.10 |
| After block 6 alone less without replay, block 6 | C's own | 25 | 0.00 | +0.04 | +0.04 | −0.27 | +0.19 | −0.38 | −0.17 ± 0.02 | −0.19 ± 0.03 |
| After block 6 alone less without replay, block 6 | readout 0's pattern targets | 245 | −0.32 | +0.37 | −0.46 | −2.01 | +2.86 | −5.29 | −5.47 ± 0.18 | −2.73 ± 0.09 |
| After block 6 alone less without replay, block 6 | readout 0's other excitatory | 87 | +0.01 | 0.00 | −0.17 | +0.01 | +0.42 | −0.56 | −0.91 ± 0.05 | −0.73 ± 0.05 |
| After block 6 alone less without replay, block 6 | inhibitory in readout 0 | 102 | −0.02 | −0.10 | −0.71 | −0.58 | +0.55 | −1.97 | −1.36 ± 0.06 | −2.61 ± 0.08 |
| After block 12 alone less without replay, block 12 | D's own | 25 | +0.05 | −0.01 | −0.16 | +0.01 | +0.35 | −0.46 | −0.16 ± 0.03 | −0.38 ± 0.03 |
| After block 12 alone less without replay, block 12 | readout 1's pattern targets | 252 | +0.37 | −0.15 | −1.84 | −0.80 | +3.03 | −5.45 | −4.63 ± 0.13 | −2.87 ± 0.12 |
| After block 12 alone less without replay, block 12 | readout 1's other excitatory | 80 | 0.00 | −0.02 | −0.08 | −0.18 | +0.28 | −0.58 | −0.65 ± 0.05 | −0.81 ± 0.06 |
| After block 12 alone less without replay, block 12 | inhibitory in readout 1 | 102 | −0.11 | −0.09 | −0.53 | −0.68 | +0.58 | −2.00 | −3.75 ± 0.09 | −1.52 ± 0.12 |
| After block 12 alone less without replay, block 12 | C's own | 25 | −0.01 | +0.04 | +0.02 | −0.26 | +0.19 | −0.40 | −0.14 ± 0.03 | −0.12 ± 0.03 |
| After block 12 alone less without replay, block 12 | readout 0's pattern targets | 245 | −0.27 | +0.34 | −0.35 | −1.70 | +2.96 | −4.97 | −4.80 ± 0.18 | −2.74 ± 0.15 |
| After block 12 alone less without replay, block 12 | readout 0's other excitatory | 87 | −0.01 | 0.00 | −0.16 | −0.02 | +0.45 | −0.63 | −1.00 ± 0.08 | −0.91 ± 0.05 |
| After block 12 alone less without replay, block 12 | inhibitory in readout 0 | 102 | −0.03 | −0.11 | −0.72 | −0.58 | +0.57 | −2.01 | −1.79 ± 0.09 | −2.71 ± 0.14 |
| After block 18 alone less without replay, block 18 | D's own | 25 | +0.05 | −0.02 | −0.17 | +0.04 | +0.34 | −0.43 | −0.28 ± 0.03 | −0.32 ± 0.02 |
| After block 18 alone less without replay, block 18 | readout 1's pattern targets | 252 | +0.36 | −0.14 | −1.76 | −0.77 | +2.94 | −5.26 | −5.14 ± 0.11 | −2.73 ± 0.13 |
| After block 18 alone less without replay, block 18 | readout 1's other excitatory | 80 | 0.00 | 0.00 | −0.07 | −0.25 | +0.25 | −0.59 | −0.65 ± 0.04 | −0.64 ± 0.04 |
| After block 18 alone less without replay, block 18 | inhibitory in readout 1 | 102 | −0.11 | −0.11 | −0.43 | −0.71 | +0.54 | −1.90 | −4.12 ± 0.11 | −1.62 ± 0.10 |
| After block 18 alone less without replay, block 18 | C's own | 25 | −0.01 | +0.01 | +0.03 | −0.24 | +0.17 | −0.39 | −0.16 ± 0.02 | −0.22 ± 0.03 |
| After block 18 alone less without replay, block 18 | readout 0's pattern targets | 245 | −0.28 | +0.34 | −0.40 | −1.63 | +2.76 | −4.75 | −4.27 ± 0.13 | −3.19 ± 0.09 |
| After block 18 alone less without replay, block 18 | readout 0's other excitatory | 87 | +0.01 | 0.00 | −0.15 | 0.00 | +0.38 | −0.52 | −0.83 ± 0.07 | −0.84 ± 0.07 |
| After block 18 alone less without replay, block 18 | inhibitory in readout 0 | 102 | −0.02 | −0.08 | −0.76 | −0.64 | +0.55 | −2.05 | −1.97 ± 0.06 | −2.48 ± 0.09 |
| After block 24 alone less without replay, block 24 | D's own | 25 | +0.01 | −0.03 | −0.16 | 0.00 | +0.32 | −0.50 | −0.26 ± 0.02 | −0.49 ± 0.03 |
| After block 24 alone less without replay, block 24 | readout 1's pattern targets | 252 | +0.35 | −0.16 | −1.73 | −0.75 | +2.88 | −5.20 | −4.90 ± 0.12 | −2.97 ± 0.06 |
| After block 24 alone less without replay, block 24 | readout 1's other excitatory | 80 | 0.00 | −0.03 | −0.07 | −0.15 | +0.27 | −0.54 | −0.74 ± 0.06 | −1.09 ± 0.06 |
| After block 24 alone less without replay, block 24 | inhibitory in readout 1 | 102 | −0.12 | −0.10 | −0.45 | −0.69 | +0.58 | −1.94 | −3.85 ± 0.09 | −1.66 ± 0.09 |
| After block 24 alone less without replay, block 24 | C's own | 25 | 0.00 | +0.04 | +0.05 | −0.20 | +0.15 | −0.26 | −0.14 ± 0.02 | −0.16 ± 0.02 |
| After block 24 alone less without replay, block 24 | readout 0's pattern targets | 245 | −0.24 | +0.37 | −0.32 | −1.62 | +2.76 | −4.59 | −3.37 ± 0.13 | −3.46 ± 0.09 |
| After block 24 alone less without replay, block 24 | readout 0's other excitatory | 87 | +0.04 | 0.00 | −0.11 | +0.06 | +0.43 | −0.44 | −0.52 ± 0.03 | −0.90 ± 0.06 |
| After block 24 alone less without replay, block 24 | inhibitory in readout 0 | 102 | −0.04 | −0.09 | −0.71 | −0.58 | +0.52 | −1.94 | −2.00 ± 0.09 | −2.25 ± 0.09 |
| After block 6 alone less without replay, block 36 | D's own | 25 | +0.04 | −0.02 | −0.20 | −0.03 | +0.20 | −0.40 | +0.05 ± 0.02 | −0.19 ± 0.01 |
| After block 6 alone less without replay, block 36 | readout 1's pattern targets | 252 | +0.33 | −0.12 | −1.81 | −0.84 | +0.53 | −3.00 | −1.83 ± 0.06 | −1.53 ± 0.09 |
| After block 6 alone less without replay, block 36 | readout 1's other excitatory | 80 | 0.00 | −0.06 | +0.02 | −0.18 | +0.06 | −0.29 | −0.08 ± 0.06 | −0.59 ± 0.07 |
| After block 6 alone less without replay, block 36 | inhibitory in readout 1 | 102 | −0.08 | −0.05 | −0.36 | −0.56 | −0.01 | −1.04 | −1.58 ± 0.09 | +0.10 ± 0.08 |
| After block 6 alone less without replay, block 36 | C's own | 25 | −0.01 | +0.02 | +0.05 | −0.27 | +0.06 | −0.27 | −0.01 ± 0.01 | −0.06 ± 0.01 |
| After block 6 alone less without replay, block 36 | readout 0's pattern targets | 245 | −0.26 | +0.30 | −0.47 | −1.70 | +0.68 | −2.84 | −2.20 ± 0.18 | −0.80 ± 0.20 |
| After block 6 alone less without replay, block 36 | readout 0's other excitatory | 87 | +0.01 | 0.00 | −0.12 | +0.08 | +0.07 | −0.09 | −0.05 ± 0.14 | −0.58 ± 0.05 |
| After block 6 alone less without replay, block 36 | inhibitory in readout 0 | 102 | −0.04 | −0.07 | −0.63 | −0.53 | −0.06 | −1.21 | −0.78 ± 0.04 | −1.15 ± 0.13 |
| After block 12 alone less without replay, block 36 | D's own | 25 | +0.04 | −0.02 | −0.16 | +0.01 | +0.22 | −0.35 | +0.04 ± 0.01 | −0.11 ± 0.02 |
| After block 12 alone less without replay, block 36 | readout 1's pattern targets | 252 | +0.42 | −0.10 | −1.74 | −0.92 | +0.91 | −3.27 | −1.47 ± 0.33 | −1.04 ± 0.08 |
| After block 12 alone less without replay, block 36 | readout 1's other excitatory | 80 | 0.00 | −0.05 | −0.04 | −0.15 | +0.06 | −0.32 | −0.11 ± 0.08 | −0.25 ± 0.07 |
| After block 12 alone less without replay, block 36 | inhibitory in readout 1 | 102 | −0.09 | −0.08 | −0.41 | −0.62 | 0.00 | −1.21 | −1.81 ± 0.24 | +0.11 ± 0.17 |
| After block 12 alone less without replay, block 36 | C's own | 25 | −0.01 | +0.03 | +0.01 | −0.23 | +0.07 | −0.27 | −0.01 ± 0.01 | 0.00 ± 0.02 |
| After block 12 alone less without replay, block 36 | readout 0's pattern targets | 245 | −0.27 | +0.27 | −0.46 | −1.51 | +1.04 | −3.03 | −1.67 ± 0.15 | −0.49 ± 0.16 |
| After block 12 alone less without replay, block 36 | readout 0's other excitatory | 87 | −0.03 | 0.00 | −0.16 | +0.05 | +0.10 | −0.24 | −0.25 ± 0.10 | −0.46 ± 0.08 |
| After block 12 alone less without replay, block 36 | inhibitory in readout 0 | 102 | −0.06 | −0.05 | −0.75 | −0.54 | +0.04 | −1.45 | −0.89 ± 0.08 | −0.87 ± 0.09 |
| After block 18 alone less without replay, block 36 | D's own | 25 | +0.05 | −0.01 | −0.17 | +0.06 | +0.23 | −0.32 | +0.02 ± 0.01 | −0.04 ± 0.02 |
| After block 18 alone less without replay, block 36 | readout 1's pattern targets | 252 | +0.36 | −0.11 | −1.69 | −0.84 | +1.20 | −3.50 | −2.26 ± 0.18 | −1.11 ± 0.08 |
| After block 18 alone less without replay, block 36 | readout 1's other excitatory | 80 | 0.00 | −0.01 | −0.05 | −0.22 | +0.12 | −0.41 | −0.05 ± 0.05 | −0.13 ± 0.03 |
| After block 18 alone less without replay, block 36 | inhibitory in readout 1 | 102 | −0.11 | −0.10 | −0.37 | −0.64 | +0.08 | −1.30 | −2.20 ± 0.12 | −0.35 ± 0.05 |
| After block 18 alone less without replay, block 36 | C's own | 25 | −0.01 | +0.01 | +0.01 | −0.25 | +0.07 | −0.31 | 0.00 ± 0.01 | −0.01 ± 0.01 |
| After block 18 alone less without replay, block 36 | readout 0's pattern targets | 245 | −0.23 | +0.27 | −0.50 | −1.57 | +1.16 | −3.22 | −1.53 ± 0.09 | −0.99 ± 0.25 |
| After block 18 alone less without replay, block 36 | readout 0's other excitatory | 87 | −0.01 | 0.00 | −0.16 | +0.06 | +0.11 | −0.22 | −0.24 ± 0.13 | −0.53 ± 0.05 |
| After block 18 alone less without replay, block 36 | inhibitory in readout 0 | 102 | −0.05 | −0.04 | −0.80 | −0.61 | +0.09 | −1.58 | −1.02 ± 0.04 | −0.67 ± 0.18 |
| After block 24 alone less without replay, block 36 | D's own | 25 | +0.01 | −0.03 | −0.16 | −0.01 | +0.24 | −0.42 | −0.14 ± 0.02 | −0.29 ± 0.02 |
| After block 24 alone less without replay, block 36 | readout 1's pattern targets | 252 | +0.38 | −0.18 | −1.72 | −0.75 | +1.73 | −4.02 | −2.12 ± 0.28 | −1.54 ± 0.09 |
| After block 24 alone less without replay, block 36 | readout 1's other excitatory | 80 | 0.00 | −0.03 | −0.07 | −0.15 | +0.12 | −0.38 | −0.19 ± 0.04 | −0.57 ± 0.02 |
| After block 24 alone less without replay, block 36 | inhibitory in readout 1 | 102 | −0.11 | −0.11 | −0.44 | −0.69 | +0.25 | −1.62 | −2.05 ± 0.12 | −0.72 ± 0.11 |
| After block 24 alone less without replay, block 36 | C's own | 25 | 0.00 | +0.04 | +0.05 | −0.20 | +0.07 | −0.19 | +0.02 ± 0.02 | −0.05 ± 0.02 |
| After block 24 alone less without replay, block 36 | readout 0's pattern targets | 245 | −0.23 | +0.38 | −0.34 | −1.62 | +1.72 | −3.56 | −1.37 ± 0.09 | −1.41 ± 0.14 |
| After block 24 alone less without replay, block 36 | readout 0's other excitatory | 87 | +0.01 | 0.00 | −0.11 | +0.06 | +0.23 | −0.26 | −0.06 ± 0.05 | −0.66 ± 0.03 |
| After block 24 alone less without replay, block 36 | inhibitory in readout 0 | 102 | −0.05 | −0.07 | −0.71 | −0.57 | +0.22 | −1.63 | −0.87 ± 0.07 | −0.69 ± 0.05 |

Every class of both readouts lost excitatory input less inhibitory input right after each single replayed night. Readout 1's pattern targets
lost 5.34, 5.45, 5.26 and 5.20 M after block 6, 12, 18 and 24 alone, where their spikes on A's trials fell by 3.70, 4.63, 5.14 and 4.90; its
inhibitory units 1.84, 2.00, 1.90 and 1.94 M, their spikes 2.86, 3.75, 4.12 and 3.85; readout 0's pattern targets 5.29, 4.97, 4.75 and 4.59
M, their spikes on A's trials 5.47, 4.80, 4.27 and 3.37; its inhibitory units 1.97, 2.01, 2.05 and 1.94 M, their spikes 1.36, 1.79, 1.97 and
2.00.

**A's margin**, each of A's and B's kinds' input onto its stimulus's answer readout less onto the other, summed over the four kinds, its
change over each night, from the input read:

| Night after block | Form | This night | A's margin, its change over the night, M |
| :--- | :--- | :--- | ---: |
| 6 | After block 6 alone | replays | +1.26 |
| 6 | Plain on all four nights (H-47) | replays | +1.26 |
| 6 | Without replay | without replay | −0.05 |
| 12 | After block 12 alone | replays | +1.16 |
| 12 | Plain on all four nights (H-47) | replays | +0.95 |
| 12 | Without replay | without replay | −0.01 |
| 18 | After block 18 alone | replays | +1.05 |
| 18 | Plain on all four nights (H-47) | replays | +0.56 |
| 18 | Without replay | without replay | −0.08 |
| 24 | After block 24 alone | replays | +1.05 |
| 24 | Plain on all four nights (H-47) | replays | +0.58 |
| 24 | Without replay | without replay | −0.08 |

A's margin rose by 1.26, 1.16, 1.05 and 1.05 M over the replayed night after block 6, 12, 18 and 24 alone, and by 0.95, 0.56 and 0.58 over
H-47's later plain nights; the night without replay's nights moved it by 0.08 M or less. These are the changes exploration 033's section 5
reads from the class matrices, after block 6 alone from exploration 031's job, after block 12, 18 and 24 alone from exploration 032's and
H-47's two forms from exploration 020's, here from the input read.

## 4. The nights and blocks after the replayed night

Each single night's difference from the night without replay right after its replayed night, each later night's and each stretch's change
in it (the form's change less the night without replay's), and the difference at block 36, in M:

| Form | Onto | Its difference from the night without replay, and each stage's change in it | A's kinds (classes 0, 1), M | B's kinds (2, 3), M | the odd places' excitatory (4, 6, 7), M | the even places' (5, 8, 9), M | other excitatory (10), M | inhibitory (11 to 13), M |
| :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone | readout 1, the even places | right after its night (block 6) | +0.34 | −0.31 | −2.56 | −1.53 | −0.02 | +4.15 |
| After block 6 alone | readout 1, the even places | blocks 6 to 12, less without replay's | −0.02 | −0.02 | +0.01 | −0.02 | 0.00 | −0.92 |
| After block 6 alone | readout 1, the even places | the night after block 12, less without replay's | 0.00 | +0.02 | −0.01 | −0.02 | 0.00 | −0.39 |
| After block 6 alone | readout 1, the even places | blocks 12 to 18, less without replay's | +0.01 | 0.00 | 0.00 | 0.00 | 0.00 | −0.55 |
| After block 6 alone | readout 1, the even places | the night after block 18, less without replay's | +0.01 | +0.03 | +0.09 | −0.02 | −0.01 | −0.31 |
| After block 6 alone | readout 1, the even places | blocks 18 to 24, less without replay's | −0.02 | −0.03 | −0.01 | −0.02 | 0.00 | −0.40 |
| After block 6 alone | readout 1, the even places | the night after block 24, less without replay's | −0.03 | +0.04 | +0.13 | −0.01 | −0.02 | −0.26 |
| After block 6 alone | readout 1, the even places | blocks 24 to 36, less without replay's | +0.01 | +0.02 | 0.00 | +0.01 | 0.00 | −0.55 |
| After block 6 alone | readout 1, the even places | at block 36 | +0.30 | −0.25 | −2.35 | −1.60 | −0.04 | +0.78 |
| After block 6 alone | readout 0, the odd places | right after its night (block 6) | −0.34 | +0.31 | −1.30 | −2.85 | −0.02 | +4.02 |
| After block 6 alone | readout 0, the odd places | blocks 6 to 12, less without replay's | +0.02 | +0.02 | −0.01 | +0.02 | 0.00 | −0.85 |
| After block 6 alone | readout 0, the odd places | the night after block 12, less without replay's | 0.00 | −0.02 | +0.15 | +0.05 | +0.01 | −0.39 |
| After block 6 alone | readout 0, the odd places | blocks 12 to 18, less without replay's | −0.01 | 0.00 | 0.00 | 0.00 | 0.00 | −0.53 |
| After block 6 alone | readout 0, the odd places | the night after block 18, less without replay's | −0.01 | −0.03 | +0.01 | +0.18 | 0.00 | −0.31 |
| After block 6 alone | readout 0, the odd places | blocks 18 to 24, less without replay's | +0.02 | +0.03 | +0.01 | +0.02 | 0.00 | −0.40 |
| After block 6 alone | readout 0, the odd places | the night after block 24, less without replay's | +0.03 | −0.04 | −0.05 | +0.17 | 0.00 | −0.25 |
| After block 6 alone | readout 0, the odd places | blocks 24 to 36, less without replay's | −0.01 | −0.02 | +0.01 | −0.01 | 0.00 | −0.54 |
| After block 6 alone | readout 0, the odd places | at block 36 | −0.30 | +0.25 | −1.17 | −2.42 | −0.02 | +0.76 |
| After block 12 alone | readout 1, the even places | right after its night (block 12) | +0.31 | −0.27 | −2.61 | −1.66 | −0.02 | +4.24 |
| After block 12 alone | readout 1, the even places | blocks 12 to 18, less without replay's | +0.02 | −0.01 | +0.02 | +0.01 | 0.00 | −0.92 |
| After block 12 alone | readout 1, the even places | the night after block 18, less without replay's | −0.01 | +0.04 | +0.10 | −0.06 | −0.01 | −0.45 |
| After block 12 alone | readout 1, the even places | blocks 18 to 24, less without replay's | +0.03 | 0.00 | +0.02 | +0.01 | 0.00 | −0.59 |
| After block 12 alone | readout 1, the even places | the night after block 24, less without replay's | −0.03 | +0.03 | +0.09 | +0.02 | −0.01 | −0.32 |
| After block 12 alone | readout 1, the even places | blocks 24 to 36, less without replay's | +0.05 | −0.04 | +0.02 | 0.00 | 0.00 | −0.76 |
| After block 12 alone | readout 1, the even places | at block 36 | +0.38 | −0.25 | −2.35 | −1.69 | −0.05 | +1.19 |
| After block 12 alone | readout 0, the odd places | right after its night (block 12) | −0.31 | +0.27 | −1.21 | −2.56 | −0.03 | +4.17 |
| After block 12 alone | readout 0, the odd places | blocks 12 to 18, less without replay's | −0.02 | +0.01 | −0.01 | 0.00 | 0.00 | −0.82 |
| After block 12 alone | readout 0, the odd places | the night after block 18, less without replay's | +0.01 | −0.04 | −0.04 | +0.17 | 0.00 | −0.41 |
| After block 12 alone | readout 0, the odd places | blocks 18 to 24, less without replay's | −0.03 | 0.00 | −0.02 | 0.00 | 0.00 | −0.58 |
| After block 12 alone | readout 0, the odd places | the night after block 24, less without replay's | +0.03 | −0.03 | −0.07 | +0.16 | 0.00 | −0.33 |
| After block 12 alone | readout 0, the odd places | blocks 24 to 36, less without replay's | −0.05 | +0.04 | −0.01 | 0.00 | 0.00 | −0.79 |
| After block 12 alone | readout 0, the odd places | at block 36 | −0.38 | +0.25 | −1.36 | −2.23 | −0.03 | +1.25 |
| After block 18 alone | readout 1, the even places | right after its night (block 18) | +0.30 | −0.26 | −2.43 | −1.69 | −0.02 | +4.07 |
| After block 18 alone | readout 1, the even places | blocks 18 to 24, less without replay's | +0.01 | +0.03 | +0.02 | +0.02 | 0.00 | −0.98 |
| After block 18 alone | readout 1, the even places | the night after block 24, less without replay's | −0.04 | +0.03 | +0.10 | +0.02 | −0.01 | −0.45 |
| After block 18 alone | readout 1, the even places | blocks 24 to 36, less without replay's | +0.04 | −0.03 | +0.02 | 0.00 | 0.00 | −1.01 |
| After block 18 alone | readout 1, the even places | at block 36 | +0.30 | −0.24 | −2.28 | −1.65 | −0.03 | +1.64 |
| After block 18 alone | readout 0, the odd places | right after its night (block 18) | −0.30 | +0.26 | −1.28 | −2.51 | −0.03 | +3.86 |
| After block 18 alone | readout 0, the odd places | blocks 18 to 24, less without replay's | −0.01 | −0.03 | −0.02 | −0.02 | 0.00 | −0.95 |
| After block 18 alone | readout 0, the odd places | the night after block 24, less without replay's | +0.04 | −0.03 | −0.13 | +0.16 | +0.01 | −0.45 |
| After block 18 alone | readout 0, the odd places | blocks 24 to 36, less without replay's | −0.04 | +0.03 | −0.02 | 0.00 | 0.00 | −1.04 |
| After block 18 alone | readout 0, the odd places | at block 36 | −0.30 | +0.24 | −1.46 | −2.37 | −0.02 | +1.43 |
| After block 24 alone | readout 1, the even places | right after its night (block 24) | +0.24 | −0.32 | −2.42 | −1.60 | −0.04 | +4.05 |
| After block 24 alone | readout 1, the even places | blocks 24 to 36, less without replay's | +0.03 | −0.04 | +0.03 | 0.00 | 0.00 | −1.71 |
| After block 24 alone | readout 1, the even places | at block 36 | +0.28 | −0.36 | −2.39 | −1.59 | −0.04 | +2.34 |
| After block 24 alone | readout 0, the odd places | right after its night (block 24) | −0.24 | +0.32 | −1.08 | −2.33 | −0.02 | +3.86 |
| After block 24 alone | readout 0, the odd places | blocks 24 to 36, less without replay's | −0.03 | +0.04 | −0.03 | 0.00 | 0.00 | −1.61 |
| After block 24 alone | readout 0, the odd places | at block 36 | −0.28 | +0.36 | −1.11 | −2.34 | −0.02 | +2.25 |

Right after each single replayed night the inhibitory input onto each readout lay above the night without replay's by 3.86 to 4.24 M; by
block 36 by 0.76 to 2.34, having come back by 42 to 81 per cent of it (the least after block 24 alone onto readout 0, the most after block 6
alone onto readout 1). Each stretch of blocks after the replayed night lowered it, by 0.40 to 1.71 M, and each of the 6 later nights in the
single nights' forms lowered it onto both readouts too. All excitatory input onto each readout lay below the night without replay's by 3.36
to 4.25 M right after and by 3.39 to 4.10 at block 36: 6 of the eight came back, by 1 to 13 per cent, and 2 lay further below, by 1 to 2 per
cent (after block 18 alone onto readout 0 and after block 24 alone onto readout 0).

The night without replay's own nights and stretches, by group, onto each readout:

| Stage | Onto | A's kinds (classes 0, 1), M | B's kinds (2, 3), M | the odd places' excitatory (4, 6, 7), M | the even places' (5, 8, 9), M | other excitatory (10), M | inhibitory (11 to 13), M |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| the night after block 6 | readout 1, the even places | −0.04 | −0.01 | +0.01 | −0.16 | −0.02 | +0.24 |
| blocks 6 to 12 | readout 1, the even places | −0.14 | +0.05 | −0.06 | +0.04 | 0.00 | +0.28 |
| the night after block 6 | readout 0, the odd places | +0.04 | +0.01 | +0.02 | −0.05 | −0.03 | +0.31 |
| blocks 6 to 12 | readout 0, the odd places | +0.14 | −0.05 | +0.07 | −0.04 | 0.00 | +0.17 |
| the night after block 12 | readout 1, the even places | −0.01 | 0.00 | +0.17 | −0.03 | −0.02 | +0.24 |
| blocks 12 to 18 | readout 1, the even places | −0.06 | +0.05 | −0.04 | +0.01 | 0.00 | +0.27 |
| the night after block 12 | readout 0, the odd places | +0.01 | 0.00 | −0.05 | +0.01 | −0.02 | +0.26 |
| blocks 12 to 18 | readout 0, the odd places | +0.06 | −0.05 | +0.04 | −0.02 | 0.00 | +0.09 |
| the night after block 18 | readout 1, the even places | −0.05 | −0.01 | +0.18 | +0.07 | 0.00 | +0.25 |
| blocks 18 to 24 | readout 1, the even places | −0.06 | 0.00 | −0.05 | −0.01 | 0.00 | +0.34 |
| the night after block 18 | readout 0, the odd places | +0.05 | +0.01 | +0.03 | +0.06 | −0.01 | +0.31 |
| blocks 18 to 24 | readout 0, the odd places | +0.06 | 0.00 | +0.05 | +0.01 | 0.00 | +0.18 |
| the night after block 24 | readout 1, the even places | −0.02 | +0.02 | +0.16 | +0.13 | +0.01 | +0.34 |
| blocks 24 to 36 | readout 1, the even places | −0.12 | +0.08 | −0.08 | +0.01 | 0.00 | +0.58 |
| the night after block 24 | readout 0, the odd places | +0.02 | −0.02 | +0.13 | −0.12 | −0.01 | +0.36 |
| blocks 24 to 36 | readout 0, the odd places | +0.12 | −0.08 | +0.08 | −0.01 | 0.00 | +0.41 |

**Right after and at block 36**, each group's difference:

| Form | Onto | A's kinds: right after, M | at block 36 | B's kinds: right after, M | at block 36 | The odd places' excitatory: right after, M | at block 36 | The even places' excitatory: right after, M | at block 36 | Other excitatory: right after, M | at block 36 | Inhibitory: right after, M | at block 36 |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone | readout 1, the even places | +0.34 | +0.30 | −0.31 | −0.25 | −2.56 | −2.35 | −1.53 | −1.60 | −0.02 | −0.04 | +4.15 | +0.78 |
| After block 6 alone | readout 0, the odd places | −0.34 | −0.30 | +0.31 | +0.25 | −1.30 | −1.17 | −2.85 | −2.42 | −0.02 | −0.02 | +4.02 | +0.76 |
| After block 12 alone | readout 1, the even places | +0.31 | +0.38 | −0.27 | −0.25 | −2.61 | −2.35 | −1.66 | −1.69 | −0.02 | −0.05 | +4.24 | +1.19 |
| After block 12 alone | readout 0, the odd places | −0.31 | −0.38 | +0.27 | +0.25 | −1.21 | −1.36 | −2.56 | −2.23 | −0.03 | −0.03 | +4.17 | +1.25 |
| After block 18 alone | readout 1, the even places | +0.30 | +0.30 | −0.26 | −0.24 | −2.43 | −2.28 | −1.69 | −1.65 | −0.02 | −0.03 | +4.07 | +1.64 |
| After block 18 alone | readout 0, the odd places | −0.30 | −0.30 | +0.26 | +0.24 | −1.28 | −1.46 | −2.51 | −2.37 | −0.03 | −0.02 | +3.86 | +1.43 |
| After block 24 alone | readout 1, the even places | +0.24 | +0.28 | −0.32 | −0.36 | −2.42 | −2.39 | −1.60 | −1.59 | −0.04 | −0.04 | +4.05 | +2.34 |
| After block 24 alone | readout 0, the odd places | −0.24 | −0.28 | +0.32 | +0.36 | −1.08 | −1.11 | −2.33 | −2.34 | −0.02 | −0.02 | +3.86 | +2.25 |

From right after each single replayed night to block 36, A's and B's kinds' and each place group's difference onto each readout moved by
0.43 M or less (the most the even places' onto readout 0 after block 6 alone), where the inhibitory input's came back by 1.61 to 3.37 M.

**At block 36**, the units that carried half of each readout's drop there (exploration 033, section 4), against the rest:

| Comparison | Trials | Readout | Units carrying half its drop (exploration 033) | Their spikes' difference a unit | The rest's | A's kinds: their input difference a unit | the rest's | B's kinds: theirs | the rest's | The odd places' excitatory: theirs | the rest's | The even places': theirs | the rest's | Inhibitory: theirs | the rest's | Excitatory less inhibitory: theirs | the rest's |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | 7 | −0.26 | 0.00 | −4 378 | +730 | −157 | −551 | −11 750 | −5 018 | −9 752 | −3 396 | +135 | +1 725 | −26 172 | −10 055 |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | 6 | −0.27 | 0.00 | −24 614 | −334 | −960 | +566 | −654 | −2 568 | −10 382 | −5 207 | −2 111 | +1 695 | −34 553 | −9 273 |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | 4 | −0.32 | 0.00 | +2 958 | +631 | −11 874 | −446 | −18 795 | −5 001 | −17 605 | −3 369 | −16 420 | +1 860 | −28 896 | −10 137 |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | 6 | −0.23 | 0.00 | −94 | −659 | −7 603 | +653 | −5 037 | −2 510 | −8 967 | −5 226 | −8 335 | +1 777 | −12 974 | −9 559 |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | 7 | −0.25 | 0.00 | −6 632 | +941 | −27 | −543 | −18 749 | −4 914 | −16 911 | −3 471 | −4 475 | +2 712 | −37 844 | −10 800 |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | 5 | −0.29 | 0.00 | −25 234 | −557 | −1 126 | +553 | +1 030 | −3 017 | −6 439 | −4 837 | −1 271 | +2 762 | −30 500 | −10 682 |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | 3 | −0.28 | 0.00 | +1 579 | +821 | −12 366 | −457 | −18 907 | −5 034 | −14 955 | −3 602 | −3 314 | +2 641 | −41 336 | −11 014 |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | 3 | −0.30 | 0.00 | +222 | −833 | +500 | +535 | −5 430 | −2 956 | −19 855 | −4 756 | −12 826 | +2 820 | −11 738 | −10 892 |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | 8 | −0.28 | 0.00 | −4 922 | +759 | −362 | −521 | −11 017 | −4 852 | −22 618 | −3 248 | −2 285 | +3 671 | −36 634 | −11 598 |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | 6 | −0.24 | 0.00 | −18 991 | −418 | −5 | +525 | −1 889 | −3 191 | −9 416 | −5 102 | −403 | +3 160 | −29 898 | −11 390 |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | 4 | −0.23 | 0.00 | +1 551 | +652 | −12 262 | −415 | −15 659 | −4 866 | −10 988 | −3 521 | +775 | +3 592 | −38 133 | −11 805 |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | 6 | −0.20 | 0.00 | −691 | −660 | −1 676 | +547 | −3 361 | −3 172 | −12 652 | −5 059 | −6 036 | +3 234 | −12 212 | −11 625 |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | 10 | −0.24 | 0.00 | −10 727 | +854 | −768 | −778 | −12 166 | −5 043 | −15 930 | −3 197 | −730 | +5 229 | −38 861 | −13 482 |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | 6 | −0.20 | 0.00 | −17 938 | −372 | +45 | +787 | −5 261 | −2 383 | −4 937 | −5 089 | +413 | +4 961 | −28 503 | −12 066 |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | 6 | −0.27 | 0.00 | +941 | +597 | −14 560 | −595 | −12 461 | −5 102 | −9 545 | −3 394 | +2 733 | +5 131 | −38 357 | −13 713 |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | 5 | −0.28 | 0.00 | −1 516 | −592 | −1 362 | +801 | −8 588 | −2 353 | −25 447 | −4 863 | −2 113 | +4 979 | −34 731 | −12 034 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | 10 | −0.54 | −0.01 | −22 989 | +2 772 | −1 642 | −2 012 | −23 222 | −16 446 | −29 186 | −8 006 | −3 586 | +16 741 | −73 454 | −40 577 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | 8 | −0.67 | −0.01 | −56 374 | −1 250 | +4 | +2 039 | +835 | −7 388 | −35 514 | −16 117 | −4 582 | +16 409 | −86 467 | −39 172 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | 8 | −0.53 | −0.01 | +6 741 | +2 131 | −51 647 | −1 123 | −36 705 | −16 237 | −19 807 | −8 267 | +2 593 | +16 541 | −104 012 | −40 180 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | 7 | −0.53 | −0.01 | −11 781 | −2 063 | −4 686 | +2 107 | −19 414 | −7 056 | −94 355 | −15 249 | −4 861 | +16 367 | −122 174 | −38 724 |

At block 36, on A's trials the units that carried half of each readout's drop had an input difference from A's kinds of −56 374 to −4 378 a
unit, below 0 in 10 of 10 sets, where the rest of their readout had −1 250 to +2 772; on B's trials theirs from B's kinds −51 647 to +500,
below 0 in 9 of 10, and the rest's −1 123 to +2 107. Theirs from the trial's stimulus's kinds lay below the rest's in 20 of the 20 sets.
Their excitatory input less inhibitory input difference read −122 174 to −11 738 a unit and the rest's −40 577 to −9 273, theirs below the
rest's in 20 of the 20, at 1.05 to 3.75 times the rest's; their excitatory input difference lay below the rest's in 20, and their inhibitory
input difference, −16 420 to +2 733 a unit against the rest's +1 695 to +16 741, above the rest's in 0. Their spikes' difference a unit read
−0.67 to −0.20 and the rest's −0.01 to 0.00 (exploration 033).

| Comparison | Trials | Readout | Its spikes' difference (exploration 033) | A's kinds | B's kinds | The odd places' excitatory | The even places' | Other excitatory | Inhibitory | All excitatory | Excitatory less inhibitory |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| After block 6 alone less without replay, block 36 | A's trials | readout 1, the even places | −3.45 | +0.39 | −0.01 | +0.14 | +0.12 | −0.01 | +0.09 | +0.25 | +0.18 |
| After block 6 alone less without replay, block 36 | A's trials | readout 0, the odd places | −3.04 | +0.79 | +0.05 | 0.00 | +0.15 | −0.01 | +0.12 | +0.32 | +0.23 |
| After block 6 alone less without replay, block 36 | B's trials | readout 1, the even places | −2.21 | −0.08 | +0.56 | +0.19 | +0.22 | −0.01 | +0.27 | +0.36 | +0.18 |
| After block 6 alone less without replay, block 36 | B's trials | readout 0, the odd places | −2.58 | −0.01 | +0.48 | +0.03 | +0.10 | −0.04 | +0.18 | +0.18 | +0.05 |
| After block 12 alone less without replay, block 36 | A's trials | readout 1, the even places | −3.35 | +0.39 | 0.00 | +0.24 | +0.23 | 0.00 | +0.13 | +0.39 | +0.29 |
| After block 12 alone less without replay, block 36 | A's trials | readout 0, the odd places | −2.82 | +0.82 | +0.06 | +0.05 | +0.07 | 0.00 | +0.10 | +0.31 | +0.23 |
| After block 12 alone less without replay, block 36 | B's trials | readout 1, the even places | −1.28 | −0.07 | +0.45 | +0.16 | +0.18 | 0.00 | +0.12 | +0.28 | +0.19 |
| After block 12 alone less without replay, block 36 | B's trials | readout 0, the odd places | −1.82 | −0.01 | +0.36 | +0.01 | +0.17 | −0.02 | +0.15 | +0.19 | +0.08 |
| After block 18 alone less without replay, block 36 | A's trials | readout 1, the even places | −4.49 | +0.48 | −0.01 | +0.18 | +0.31 | −0.01 | +0.11 | +0.43 | +0.34 |
| After block 18 alone less without replay, block 36 | A's trials | readout 0, the odd places | −2.79 | +0.78 | +0.02 | +0.05 | +0.12 | −0.02 | +0.11 | +0.29 | +0.21 |
| After block 18 alone less without replay, block 36 | B's trials | readout 1, the even places | −1.64 | −0.02 | +0.55 | +0.18 | +0.17 | −0.01 | +0.10 | +0.32 | +0.24 |
| After block 18 alone less without replay, block 36 | B's trials | readout 0, the odd places | −2.20 | +0.03 | +0.40 | +0.07 | +0.15 | +0.03 | +0.16 | +0.23 | +0.12 |
| After block 24 alone less without replay, block 36 | A's trials | readout 1, the even places | −4.49 | +0.60 | −0.02 | +0.22 | +0.24 | −0.02 | +0.14 | +0.41 | +0.29 |
| After block 24 alone less without replay, block 36 | A's trials | readout 0, the odd places | −2.28 | +0.73 | +0.01 | +0.14 | +0.10 | −0.01 | +0.12 | +0.31 | +0.21 |
| After block 24 alone less without replay, block 36 | B's trials | readout 1, the even places | −3.12 | −0.03 | +0.58 | +0.10 | +0.14 | −0.01 | +0.09 | +0.27 | +0.19 |
| After block 24 alone less without replay, block 36 | B's trials | readout 0, the odd places | −2.80 | +0.04 | +0.46 | +0.07 | +0.24 | −0.01 | +0.11 | +0.30 | +0.21 |

At block 36, under each single night, over the 16 sets the correlation of a unit's input difference from the trial's stimulus's kinds with
its spikes' difference read +0.36 to +0.82, 16 of the 16 above 0: A's kinds' on A's trials +0.39 to +0.82 and B's kinds' on B's trials +0.36
to +0.58; from the other stimulus's kinds −0.08 to +0.06. Of its excitatory input less inhibitory input difference it read +0.05 to +0.34
(after block 6 alone at block 36, readout 0 on B's trials the least, after block 18 alone at block 36, readout 1 on A's trials the most), 16
of the 16 above 0; of its inhibitory input difference +0.09 to +0.27, 16 above 0; of its excitatory input difference +0.18 to +0.43, 16
above 0; of the odd places' excitatory 0.00 to +0.24 and of the even places' +0.07 to +0.31.

The gain after the night after block 24 and at block 36:

| Form | Gain after the night after block 24 | At block 36 |
| :--- | ---: | ---: |
| Without replay | 1.712 | 1.669 |
| After block 6 alone | 1.720 | 1.678 |
| Plain on all four nights (H-47) | 1.656 | 1.696 |
| After block 12 alone | 1.720 | 1.678 |
| After block 18 alone | 1.719 | 1.678 |
| After block 24 alone | 1.612 | 1.678 |

## 5. H-47's plain night on all four nights

Its difference from the night without replay at blocks 6 to 24, its nights so far together, and at block 36, in M:

| Block | Onto | A's kinds (classes 0, 1), M | B's kinds (2, 3), M | the odd places' excitatory (4, 6, 7), M | the even places' (5, 8, 9), M | other excitatory (10), M | inhibitory (11 to 13), M |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 6 | readout 1, the even places | +0.34 | −0.31 | −2.56 | −1.53 | −0.02 | +4.15 |
| 6 | readout 0, the odd places | −0.34 | +0.31 | −1.30 | −2.85 | −0.02 | +4.02 |
| 12 | readout 1, the even places | +0.57 | −0.57 | −4.64 | −2.67 | −0.02 | +6.56 |
| 12 | readout 0, the odd places | −0.57 | +0.57 | −2.16 | −5.02 | −0.02 | +6.45 |
| 18 | readout 1, the even places | +0.75 | −0.72 | −6.37 | −3.32 | −0.03 | +8.32 |
| 18 | readout 0, the odd places | −0.75 | +0.72 | −2.68 | −6.55 | −0.03 | +8.11 |
| 24 | readout 1, the even places | +0.97 | −0.87 | −7.64 | −3.88 | −0.06 | +9.49 |
| 24 | readout 0, the odd places | −0.97 | +0.87 | −3.31 | −7.56 | −0.02 | +9.23 |
| 36 | readout 1, the even places | +1.01 | −0.92 | −7.62 | −3.89 | −0.06 | +7.48 |
| 36 | readout 0, the odd places | −1.01 | +0.92 | −3.33 | −7.55 | −0.02 | +7.36 |

Under H-47's plain night the inhibitory input's and each place group's differences onto each readout grew with each night. Less the night
without replay's, at blocks 6, 12, 18, 24 and 36: the inhibitory input onto readout 1 read +4.15, +6.56, +8.32, +9.49, +7.48 M and onto
readout 0 +4.02, +6.45, +8.11, +9.23, +7.36; the odd places' excitatory input onto readout 1 −2.56, −4.64, −6.37, −7.64, −7.62, and the even
places' onto readout 0 −2.85, −5.02, −6.55, −7.56, −7.55; A's kinds' input onto readout 1 +0.34, +0.57, +0.75, +0.97, +1.01.

The units that carried half of each readout's drop under it at blocks 6 to 24 (exploration 033, section 3), against the rest:

| Comparison | Trials | Readout | Units carrying half its drop (exploration 033) | Their spikes' difference a unit | The rest's | A's kinds: their input difference a unit | the rest's | B's kinds: theirs | the rest's | The odd places' excitatory: theirs | the rest's | The even places': theirs | the rest's | Inhibitory: theirs | the rest's | Excitatory less inhibitory: theirs | the rest's |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | 16 | −0.23 | −0.01 | −7 967 | +1 065 | +9 | −706 | −6 676 | −5 544 | −7 430 | −3 174 | +749 | +9 350 | −22 072 | −17 771 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | 15 | −0.27 | −0.01 | −10 144 | −433 | +594 | +684 | −6 274 | −2 713 | −24 054 | −5 601 | +455 | +9 028 | −39 757 | −17 156 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | 15 | −0.19 | −0.01 | +1 704 | +718 | −11 194 | −326 | −7 121 | −5 531 | −5 829 | −3 238 | +3 663 | +9 232 | −26 096 | −17 644 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | 18 | −0.18 | −0.01 | −1 410 | −723 | −8 272 | +1 047 | −4 494 | −2 761 | −15 099 | −5 841 | +606 | +9 080 | −30 443 | −17 383 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | 14 | −0.37 | −0.01 | −12 777 | +1 672 | −790 | −1 253 | −14 893 | −9 956 | −16 542 | −5 490 | +1 125 | +14 717 | −46 127 | −29 779 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | 13 | −0.41 | −0.01 | −21 020 | −655 | +921 | +1 248 | −4 181 | −4 722 | −43 618 | −9 978 | +6 980 | +14 252 | −73 584 | −28 440 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | 13 | −0.29 | −0.01 | +1 333 | +1 229 | −20 053 | −690 | −11 830 | −10 056 | −9 499 | −5 720 | +2 690 | +14 641 | −42 555 | −29 920 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | 13 | −0.32 | −0.01 | −3 896 | −1 154 | −12 109 | +1 628 | −8 895 | −4 585 | −32 291 | −10 308 | +903 | +14 429 | −56 941 | −28 926 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | 15 | −0.45 | −0.01 | −17 811 | +2 280 | −985 | −1 583 | −19 095 | −13 696 | −22 613 | −6 714 | +1 005 | +18 696 | −61 510 | −38 479 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | 12 | −0.53 | −0.01 | −34 025 | −754 | +422 | +1 594 | −2 829 | −5 918 | −35 667 | −13 702 | +9 157 | +17 893 | −81 255 | −36 731 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | 11 | −0.41 | −0.01 | +3 710 | +1 572 | −30 294 | −858 | −23 499 | −13 636 | −12 629 | −7 101 | +5 392 | +18 430 | −68 104 | −38 523 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | 12 | −0.41 | −0.01 | −6 319 | −1 498 | −14 448 | +1 993 | −15 794 | −5 570 | −59 604 | −13 060 | +1 478 | +18 099 | −96 228 | −36 329 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | 14 | −0.55 | −0.02 | −23 723 | +2 919 | −628 | −1 925 | −24 145 | −16 400 | −22 788 | −7 993 | +176 | +21 317 | −71 460 | −44 862 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | 11 | −0.64 | −0.01 | −46 866 | −1 007 | +643 | +1 916 | −628 | −7 372 | −39 133 | −15 925 | −3 474 | +20 695 | −82 509 | −43 131 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | 10 | −0.53 | −0.01 | +5 135 | +2 039 | −48 473 | −848 | −39 192 | −16 134 | −15 540 | −8 286 | +8 336 | +20 947 | −106 407 | −44 320 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | 11 | −0.51 | −0.01 | −7 668 | −1 970 | −13 887 | +2 273 | −16 277 | −6 988 | −70 235 | −15 161 | −234 | +20 615 | −106 249 | −42 548 |

Under H-47's plain night at blocks 6 to 24, on A's trials the units that carried half of each readout's drop had an input difference from
A's kinds of −46 866 to −7 967 a unit, below 0 in 8 of 8 sets, where the rest of their readout had −1 007 to +2 919; on B's trials theirs
from B's kinds −48 473 to −8 272, below 0 in 8 of 8, and the rest's −858 to +2 273. Theirs from the trial's stimulus's kinds lay below the
rest's in 16 of the 16 sets. Their excitatory input less inhibitory input difference read −106 407 to −22 072 a unit and the rest's −44 862
to −17 156, theirs below the rest's in 16 of the 16, at 1.24 to 2.65 times the rest's; their excitatory input difference lay below the
rest's in 16, and their inhibitory input difference, −3 474 to +9 157 a unit against the rest's +9 028 to +21 317, above the rest's in 0.
Their spikes' difference a unit read −0.64 to −0.18 and the rest's −0.02 to −0.01 (exploration 033).

The correlation of the units' input difference with their spikes' difference under it at blocks 6 to 36:

| Comparison | Trials | Readout | Its spikes' difference (exploration 033) | A's kinds | B's kinds | The odd places' excitatory | The even places' | Other excitatory | Inhibitory | All excitatory | Excitatory less inhibitory |
| :--- | :--- | :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Plain on all four nights less without replay, block 6 | A's trials | readout 1, the even places | −7.45 | +0.60 | −0.03 | +0.12 | +0.13 | −0.12 | +0.17 | +0.28 | +0.12 |
| Plain on all four nights less without replay, block 6 | A's trials | readout 0, the odd places | −7.92 | +0.63 | −0.01 | +0.08 | +0.38 | −0.15 | +0.15 | +0.43 | +0.28 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 1, the even places | −5.47 | −0.07 | +0.72 | +0.10 | +0.05 | −0.01 | +0.11 | +0.26 | +0.15 |
| Plain on all four nights less without replay, block 6 | B's trials | readout 0, the odd places | −6.27 | +0.09 | +0.62 | +0.07 | +0.24 | +0.02 | +0.18 | +0.35 | +0.20 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 1, the even places | −9.92 | +0.58 | −0.02 | +0.13 | +0.17 | −0.07 | +0.19 | +0.31 | +0.14 |
| Plain on all four nights less without replay, block 12 | A's trials | readout 0, the odd places | −10.14 | +0.73 | +0.01 | +0.04 | +0.33 | −0.14 | +0.11 | +0.40 | +0.28 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 1, the even places | −7.39 | −0.04 | +0.79 | +0.08 | +0.10 | −0.03 | +0.13 | +0.32 | +0.18 |
| Plain on all four nights less without replay, block 12 | B's trials | readout 0, the odd places | −7.91 | +0.14 | +0.57 | +0.08 | +0.33 | −0.07 | +0.17 | +0.42 | +0.25 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 1, the even places | −13.12 | +0.61 | −0.02 | +0.13 | +0.21 | −0.04 | +0.20 | +0.35 | +0.17 |
| Plain on all four nights less without replay, block 18 | A's trials | readout 0, the odd places | −11.94 | +0.78 | +0.01 | +0.04 | +0.27 | −0.08 | +0.13 | +0.39 | +0.26 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 1, the even places | −8.71 | −0.06 | +0.79 | +0.11 | +0.09 | −0.02 | +0.12 | +0.33 | +0.20 |
| Plain on all four nights less without replay, block 18 | B's trials | readout 0, the odd places | −9.54 | +0.13 | +0.54 | +0.09 | +0.38 | −0.09 | +0.18 | +0.46 | +0.29 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 1, the even places | −14.75 | +0.58 | −0.02 | +0.14 | +0.21 | −0.02 | +0.20 | +0.36 | +0.18 |
| Plain on all four nights less without replay, block 24 | A's trials | readout 0, the odd places | −13.23 | +0.81 | 0.00 | +0.03 | +0.16 | −0.04 | +0.15 | +0.35 | +0.20 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 1, the even places | −10.20 | −0.05 | +0.80 | +0.11 | +0.11 | −0.02 | +0.13 | +0.35 | +0.21 |
| Plain on all four nights less without replay, block 24 | B's trials | readout 0, the odd places | −10.63 | +0.07 | +0.51 | +0.09 | +0.35 | −0.18 | +0.17 | +0.42 | +0.24 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 1, the even places | −10.26 | +0.55 | −0.02 | +0.15 | +0.23 | −0.03 | +0.20 | +0.35 | +0.18 |
| Plain on all four nights less without replay, block 36 | A's trials | readout 0, the odd places | −10.43 | +0.84 | +0.01 | +0.03 | +0.16 | −0.04 | +0.16 | +0.36 | +0.21 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 1, the even places | −7.88 | −0.06 | +0.81 | +0.12 | +0.11 | −0.02 | +0.14 | +0.35 | +0.22 |
| Plain on all four nights less without replay, block 36 | B's trials | readout 0, the odd places | −7.23 | +0.09 | +0.43 | +0.10 | +0.36 | −0.19 | +0.17 | +0.42 | +0.25 |

Under H-47's plain night at blocks 6 to 36, over the 20 sets the correlation of a unit's input difference from the trial's stimulus's kinds
with its spikes' difference read +0.43 to +0.84, 20 of the 20 above 0: A's kinds' on A's trials +0.55 to +0.84 and B's kinds' on B's trials
+0.43 to +0.81; from the other stimulus's kinds −0.07 to +0.14. Of its excitatory input less inhibitory input difference it read +0.12 to
+0.29 (H-47's plain night at block 6, readout 1 on A's trials the least, H-47's plain night at block 18, readout 0 on B's trials the most),
20 of the 20 above 0; of its inhibitory input difference +0.11 to +0.20, 20 above 0; of its excitatory input difference +0.26 to +0.46, 20
above 0; of the odd places' excitatory +0.03 to +0.15 and of the even places' +0.05 to +0.38.

## What it costs

The trial took 304 s and the two jobs 5 822 and 7 925 s, 97.0 and 132.1 minutes, at most 55.0 % of a job's 240 minutes; exploration 033's
took 304, 5 850 and 7 429 s. Nothing was cut: each job ran its forms on the three paths, every night watched, every read of A counted unit
by unit and every image's inputs read. The two logs are 11.0 and 13.5 MB, exploration 033's 6.6 and 7.8 MB; ADR-0249 put the input reads at
some 7 and 10 MB more.

## What it suggests

**On this cell each single replayed night moved both readouts' units' inputs in the same directions**: the inhibitory input onto both rose,
each place group's excitatory input fell, more onto the other readout than onto its own, A's kinds' input moved toward readout 1 and B's
toward readout 0, and the gain fell. Readout 1's units did not lose more input from A's kinds than readout 0's after any single night;
readout 1's excitatory input less inhibitory input fell further than readout 0's by 0.48, 0.46 and 0.95 M after the later single nights and
by 0.01 after the first. Unit by unit, right after each single replayed night the units that carried half of each drop had lost input from
the trial's stimulus's kinds, and the inhibitory rise had landed on them less than on the rest of their readout. By block 36 the inhibitory
difference had come back by 42 to 81 per cent and A's and B's kinds' and each place group's difference had moved by 0.43 M or less.

Questions this suggests for a later exploration or a round:
- What a unit's input change does to its drive: an input here is a sum of weights, and the presynaptic units' spikes, which weight it on
  each trial, are not read.
- Whether the units that carried half of each drop are the units with the most input from the trial's stimulus's kinds on the image before
  the night: those inputs are in these jobs' logs and are not read here against the units.
- What decides which units a replayed night's inhibitory rise lands on: in every set here it landed less on the units that carried half of
  the drop than on the rest, and a unit's inhibitory input difference correlated above 0 with its spikes' difference.
- What in the image each later night starts from makes the same replay take more of readout 1's excitatory input less inhibitory input than
  of readout 0's: before the nights after block 6, 12, 18 and 24 A's kinds' input onto readout 1 less onto readout 0 read +0.52, +0.17,
  +0.04 and −0.19 M, and the inhibitory input onto readout 1 was 25.70 M before the first and 27.31 before the last, onto readout 0 27.44
  and 28.77.
- Whether the inputs move the same way on other cells, seed 24's eighth image from the mirrored assignment among them.

## What did not run

- **Any other cell**, seed 24's eighth image from the mirrored assignment among them.
- **Replay on two of the four nights, or on every night but one.**
- **Any hold**, on any night.
- **Plain nights on both mappings and the awake gap.**
- **The pair's reads unit by unit**: they are read as before, their counts and masks set beside exploration 033's.
- **The units outside the readouts' sets**, their spikes and their inputs: the stimulus units, the other excitatory units and the inhibitory
  units elsewhere.
- **A unit's membrane, threshold or delays**: a unit's inputs are the weights onto it, not its drive, and the presynaptic units' spikes are
  not read.
- **Any path but H-47's three**, 0, 5 and 6.
- **A night split apart**, such as slow-wave sleep read apart from REM: a night's change is read whole, from the images before and after
  it.
- **The next day's reading of the pairings a night leaves** pending.
- **Any SE for an input**: an input is the mean of three images, one a path; the correlations carry none either.
- **The units that carried half of each single night's rise over H-47's plain night at block 36** (exploration 033, section 4): their
  inputs less the plain night's were not read.
- **A table of each of the 918 units' inputs**: the tables sum the readouts, the classes and the named sets; every unit's sums are in the
  jobs' logs.
