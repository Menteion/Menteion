# Where the work stands

Every round rewrites this page whole, and it holds no history ([ADR-0197](adr/0197-one-place-for-each-kind-of-fact.md)).
[ADR-0246](adr/0246-the-first-nights-replay-on-twelve-cells.md) wrote it on 2026-11-10, after exploration 030.

## The engine

- **It runs.** The records with their layout assertions, the update rules, synaptic fan-out with three-factor STDP, the executor on a pool of workers that each own a range of the unit arena, the `.menteion` image (format 26, its magic `MENTEION`), the criticality controller, sleep with the episodic ledger and its replay, and the symbolic layer are Implemented ([§1.6](WHITEPAPER.md#16-implementation-status-at-a-glance)).
- **It serves a unit on arrival** ([ADR-0225](adr/0225-a-unit-served-on-arrival-built.md)): a unit that cannot fire with no input sleeps off the schedule and is caught up bit for bit at its next message or when the arena is read. Per-tick service stays the model and the tests' reference, `Executor::per_tick`.
- **Its default** for an image no derivation has settled is `Config::reads_its_targets()`: `Config::default()` with the modulation at rest 0.5, the gain 1.75, the read of 128 windows, both targets by it, and the probe, so the rate's rule runs at the step the engine makes ([ADR-0196](adr/0196-the-engines-default.md), [ADR-0205](adr/0205-the-probe-in-the-engines-default.md)). `Config::default()` stays the bare engine every pin runs.
- **It can keep its addresses and read a delay** ([ADR-0208](adr/0208-a-reward-past-the-trial-its-address-kept-built.md), [ADR-0217](adr/0217-a-delay-the-engine-reads-built.md)): a reward that names its trial's age consolidates under the address kept, and with the delay read the engine moves once to the age that leads. Unset, told, or on time with no move, every run is the run it was.
- **It can hold a stimulus's couplings** ([ADR-0228](adr/0228-a-zero-sum-at-the-stimulus-built.md)): `Executor::set_zero_sum` scales each group back to a value the host gives, and `Config::depression_exponent` sets the excitatory depression's exponent. Neither is in the image.
- **It has a name for running a settled image** ([ADR-0233](adr/0233-a-running-configuration-built.md)): `Config::runs_a_settled_image()` is the default with a quarter's exponent. The host reads each group's value once with `Executor::coupling_sum` and sets the groups at each load; the name without them is a state no run has read.
- **It can sleep on a baseline of its own and replay only what a host tags** ([ADR-0235](adr/0235-learning-that-sleep-keeps-built.md)): `Config::slow_wave_baseline_q16` takes the baseline's place in slow-wave sleep, `Executor::spend_episodes` spends the ledger, and `Task::trial_presenting` presents a stimulus to the units a host gives. Each unset, every run is the run it was; the baseline is in no name and not in the image. The replay's drive arrives scaled by the gain, 4.375 at 1.75 (F-47 resolved).

## What it learns

On the reference network at 1 024 units, ADR-0044's prior from seed 22, unless a row says otherwise.
Every hypothesis, with its report, is in [docs/research/](research/README.md).

| Asked | Read | Hypotheses |
| :--- | :--- | :--- |
| The two-alternative task: learned, revised through three reversals with the couplings bounded, under a reward right 7 times in 8 | yes, under H-25's configuration | H-13 to H-28 |
| A choice among three answers | no: the first mapping is learned, not every revision, and the line stops | H-29 to H-32 |
| The task on seeds 23 and 24, and at 2 048 units | yes | H-33, H-34 |
| A reward right 3 times in 4, with the gain held at 1.75 on the earlier networks | no: 21 mappings of 24 learned over three networks | H-35, H-36 |
| The criticality controller's step | no: every step takes the network to 24.4 Hz a unit | H-37 |
| The gain moved by a rate the controller holds, then a target and the inhibitory target the engine reads | yes, on three networks | H-38 to H-41 |
| A step the engine makes from its own loop | yes, on three networks: step = 1/61 to 1/60 | [H-42](research/H-42.md) |
| A reward one or two trials late, its address kept; and right 3 times in 4 on the default | yes, on three networks | [H-43](research/H-43.md), [H-44](research/H-44.md) |
| A delay of one or two trials the engine is not told | yes, on three networks: one move to the true age, every later mapping 118 to 127 of 128 | [H-45](research/H-45.md) |
| The couplings over sixteen schedules, under a zero sum at the stimulus and a quarter's exponent | yes, on three networks: highest coupling 1.157 to 1.215 | [H-46](research/H-46.md) |
| A mapping a conflicting one takes, kept more by four nights with replay than four without | no at step 5: yes in ten cells of twelve, by +2.2 to +19.6 at block 36; not in one cell each on seeds 23 and 24, at −4.9 and +1.8; the pair learned and A's highest coupling at most 1.223 in every cell | [H-47](research/H-47.md) |

**The learning configuration** is H-25's ([ADR-0146](adr/0146-the-tag-the-address-already-is.md)): the excitatory synapses under the reward's gate with the signed gate set, the inhibitory ones under a baseline of their own, a critic of the engine's own over the shortest synaptic delay after a reward, and the reward's address drawn by the engine from that window's counts onto the readout its selection chose.

Per trial the host gives the reward's sign; under a delay it names the trial's age, or, with the delay read, the engine reads it. The readout sets are the body's interface, and so, under the zero sum, are its groups.

## Asked, read as no, and kept unset

- The rule held by the network is paused, with the facilitating class and the slow current ([ADR-0126](adr/0126-the-gate-raised-measured.md)); the address's neural target side is closed, with the hold and the released delivery (ADR-0146).
- The whole punishment is a parameter of the image no configuration sets (ADR-0155, ADR-0156); the exploration an option of the task no run sets (ADR-0163, ADR-0164); and the controller's step under its estimate is zero in every run that learns (ADR-0077, ADR-0177).
- The kept addresses, the delay read, the zero sum's groups and the slow-wave baseline are in no default or name; the exponent is in the running configuration alone.
- Of the speed line ([ADR-0105](adr/0105-the-speed-line-closed.md)), the sweep and service on arrival are kept, and the lanes reverted.

## What does not exist

- Every subsystem's real dynamics beyond these rules, the per-unit gain of §8.8, and a reserved byte in the modulator section.
- The lookahead ADR-0099 ordered, the `mmap` path, the shared-memory mappings, core pinning, the tool broker, standardising apart, and the real-time mode's reach, measured on the reference platform.
- A delay that changes during a run or passes two trials, a run past sixteen schedules, and couplings whose shape has settled.
- A public call that lays H-25's learning configuration on an image, so no host outside the tests reaches a settled image.
- Chained episodes compiled in sleep (UPG-04), and a night whose replay reaches A's mapping on every network.

## The next decision

[ADR-0246](adr/0246-the-first-nights-replay-on-twelve-cells.md) reads exploration 030 and writes no hypothesis, and M11 stays open.
1. M11: exploration 031 runs once ADR-0246 merges. It reads one form beside the night without replay on all twelve of H-47's cells, on a branch of its own: H-47's plain night's replay after block 6 alone with the later nights without replay, which on seed 23's first mapping from the mirrored assignment was the only one of exploration 030's eight forms above the night without replay beyond 2 SE at block 36, on B's trials. Whether a hypothesis follows it is the next decision.
2. The schedule's sweep of 2026-11-11 at 20:17Z is the next to record, by the first ADR that merges after it ends.
3. A read of the arena on every tick catches every asleep unit up a tick, so a test that reads it so takes longer ([ADR-0225](adr/0225-a-unit-served-on-arrival-built.md)): what such a reading should pay is not decided.
