---
status: proposed
date: 2026-11-13
title: The design review planned
kind: decision
summary: Of the thirty-one entries of the design review of 2026-11-13, read against the tree, sixteen are planned in seven orders, three frame the plan, four wait for decisions of their own, six are deferred and two not adopted. Search by forks and a task's references come first; exploration 038 is paused.
decision-makers: Menteion maintainers
---

# ADR-0254: The design review planned

**In brief**
- **Decision.** Nothing is built by this ADR. The thirty-one entries of a design review written outside the tree on 2026-11-13 are numbered DR-01 to DR-31 and given a place each: sixteen are planned in seven orders, each to enter by a decision of its own; three frame the plan; four would move a standing decision and wait for an ADR of their own; six are deferred, each with what would bring it; two are not adopted as written. The first order is a search by forks with a task's references. M11's next step is a retention gate the host runs, and exploration 038 is paused. Four findings, F-87 to F-90, are recorded.
- **Why.** The maintainers asked that the review be taken into the plan, its claims checked against the tree first. Three readers checked every claim against `main` at `07ffcec`: most hold, and some that a proposal rests on do not.
- **Left open.** Each planned item's hypothesis, stopping rule and layout; where the search tool lives and how it runs; the order of this plan's orders 4 to 7 beside ADR-0187's M13, M14, M8's remainder and M15; the rungs of the ladder of tasks; a reference-platform machine; the review's literature, which it cites from memory.

## Context and Problem Statement

On 2026-11-13 the maintainers asked that the engine be improved after a design review written outside this tree that day, read against `07ffcec`. It has thirty-one entries in nine parts, each a problem, a proposed resolution and its rejected alternatives. It names five that would move a standing decision, six that use something the engine has and tissue lacks, and a suggested order: the measurement, a task's references and a search by forks first. Its literature is cited from memory.

The maintainers chose (2026-11-13): check its claims first, then plan it in one ADR as [ADR-0187](0187-the-upgrade-suite-planned.md) planned the upgrade suite; run a search by forks with a task's references as the first round; defer an admissible measurement, since no reference-platform machine is available; and pause exploration 038.

**What checking it against the tree finds.** Three readers checked each claim, file by file. Most hold: the plateau's and the threshold's constants, the free flag bit `0x10`, the 28-bit token and its 2^26 blocks, the record's lack of a reserved byte, the absence of any plasticity rule that reads the burst flag or the apical potential, and the lack of any search instrument. Those a proposal rests on and that do not hold:

- **The policy amendment's trial is not a retention gate.** It commits only when the two forks' behaviour hashes are equal and the cost fell (§8.18, [ADR-0031](0031-policy-amendment.md)); a night that changes weights fails it by construction. And an episode of the ledger names no mapping: whether a mapping is answered is the host's to read. A gate on a night's consolidation that the host runs, as H-47's harness forks its reads, moves no boundary; one the engine sets on its own amendments is what ADR-0187 does not adopt and §11.1 refuses ([ADR-0057](0057-the-inhibitory-rule-from-below-the-rail.md): "a score of its own choosing is not a test").
- **Slow adaptation needs a new rule.** The threshold's excess decays by `(excess >> shift).max(1)`, so one spike's step of 0.02 is gone in 13.1 ms at any shift of 10 or more, the present 12 among them; shifts of 16 to 18 change nothing below 1.0 (F-90).
- **Short-term plasticity is the presynaptic unit's.** Its state is in the unit's record, and `FLAG_FACILITATING` chooses between [ADR-0019](0019-short-term-plasticity.md)'s constants and the image's one facilitating class: a unit cannot send a depressing pathway to one target and a facilitating one to another.
- **The held-rule line ran six rounds, not four**: [ADR-0115](0115-the-assemblies-marked.md) found one usable cell and [ADR-0117](0117-the-region-measured.md) a usable region, neither robust, and the slow current's round ran no cell.
- **The engine's figures predate service on arrival.** The 11 ns turn, the lanes' split and the reach of 10^4 units were read before [ADR-0225](0225-a-unit-served-on-arrival-built.md); on H-45's arm that ADR read 167.8 M turns against 17.0 G catch-up steps.
- **A static synapse is not all seed**: a block holds the stored release that a delayed delivery reads back, which a token naming a unit and an ordinal cannot carry.
- **The plan's scale is the offline mode's** (Appendix A), not real time; §1.1 states bit-exact arithmetic, and quality goals 1 and 5 determinism and claims checkable in CI.
- **Others, smaller**: the workspace admits dependencies by TC-2's allow-list; among three answers H-30 to H-32 learned the first reversal and not every later one, while H-29 learned seven mappings of eight and failed only its mirrored arm's first reversal; a pairing writes the trace, not the weight; the apical potential leaks in about 10 ms while consolidation reads the modulator at the next presentation.

The readers also found four places where the tree disagrees with itself, recorded below as F-87 to F-90.

## Decision Drivers

- **The repository wins over the document** (principle 1): a proposal's premise is checked against the tree before it is planned.
- **Literature is a prior, not a verdict**, and one change a round: each item enters as a candidate for a measured round.
- **A rule says where it is enforced** (principle 5), and a standing decision moves only by an ADR of its own.
- **ADR-0187's frame**: done, planned in an order, deferred with a condition, not adopted.

## Considered Options

1. **The review sorted** into planned in an order, frames, standing decisions each for its own ADR, deferred with a condition, and not adopted.
2. **The review's own order taken as the plan.**
3. **Each entry an ADR of its own.**
4. **Nothing taken.**

## Decision Outcome

**Option 1. Nothing is built by this ADR.** The review stays outside the tree; this ADR cites it by its numbers and keys.

| DR | Key | DR | Key | DR | Key |
| :---: | :--- | :---: | :--- | :---: | :--- |
| 01 | engineered-advantages | 12 | dendritic-branches | 23 | layered-prior |
| 02 | stated-goal | 13 | hidden-layer-task | 24 | trial-gated-consolidation |
| 03 | technology-admission | 14 | burst-credit | 25 | prioritised-replay |
| 04 | admissible-measurement | 15 | unit-learning-signal | 26 | task-ladder |
| 05 | task-references | 16 | fork-credit | 27 | search-by-forks |
| 06 | tick-duration | 17 | fast-slow-weights | 28 | causal-trace |
| 07 | working-copy | 18 | plastic-delays | 29 | offline-training |
| 08 | block-index-width | 19 | structural-plasticity | 30 | mente-kernel-port |
| 09 | virtual-synapses | 20 | synapse-budget | 31 | freestanding-engine |
| 10 | plateau-class | 21 | gradient-information | | |
| 11 | adaptation-class | 22 | cellular-working-memory | | |

### Planned, in this order

Each item enters by a decision ADR of its own: an instrument by an ADR that builds it, a mechanism by one that writes a hypothesis and a stopping rule before any run. The DR named is a candidate, its layout derived again in that ADR.

| Order | What it asks | Candidates | Uses what the engine has and tissue lacks | What comes first |
| :---: | :--- | :--- | :--- | :--- |
| 1 | What the two-alternative task asks of a learner: its floor and its ceiling | DR-27, a search by forks in the tree, outside the tick loop; DR-05, a floor by the simplest learner and a ceiling by the search | exact repetition; fork and compare | none |
| 2 | Which path changed which weight, when | DR-28, a trace of every weight change, off by default and outside the image | exact repetition | none |
| 3 | M11: does a night kept only when the host's probe shows retention keep a mapping better than an ungated night and a night without replay, on what the probe did not see? | DR-24 as a gate the host runs; DR-25, the replay's order set by what the host tags and spends | fork and compare (DR-24); an exact ledger (DR-25) | order 1's references |
| 4 | A mapping no single layer learns | DR-13, its task; DR-23, a second generator with named populations and projections | — | order 1 |
| 5 | Credit beyond one layer, each rule read against the floor, the ceiling and the others | DR-16 first, then DR-14 (UPG-01), then DR-15 | fork and compare (DR-16) | order 4 |
| 6 | A rule held by a cell, not an assembly | DR-22, its candidates DR-10 and DR-11 | — | order 1; an ADR lifting the pause ADR-0126 read under ADR-0125's rule |
| 7 | What a synapse holds that changes what can be learned | DR-17, DR-18, DR-19, each its own round | an exact ledger (DR-19) | a task of the ladder with its references; for DR-19, order 4's task |

- **Order 1** decides where the tool lives, a module of `runtime/menteion-runtime/tests` or a crate that is not a state crate (which changes §1.6's count of such crates), and how it runs under [ADR-0184](0184-the-runs-off-the-developer-machine.md). A reference is reported beside a new reading; no standing verdict is read again.
- **Order 2** is off by default, as `trace_capacity` and the spike train are. §8.11 allows no branch, lock or syscall in the tick loop, and those two branch in the turn, so its ADR says how the trace meets that rule: a replica outside the engine over a second run, as [ADR-0072](0072-what-the-trace-is-made-of.md)'s oracle replays the pair rule over the train, or a build of its own. It names the paths that move a synaptic weight inside the engine: consolidation, the signed consolidation, the zero sum's rounds and the critic's value weights, and the host's writes beside them.
- **Order 3** is M11's next decision, a new hypothesis that reopens nothing of H-47's step 8. The probe and the mappings are the host's; the engine's rules and the policy amendment do not change. The review counted DR-24 among its five that move a standing decision; run by the host it moves none, and a gate the engine sets on its own amendments is not adopted. Exploration 038 ([ADR-0253](0253-what-the-night-writes-on-the-other-ten-cells.md)) is paused, and order 3's ADR says whether it runs, is dropped or is replaced. UPG-04, chained episodes compiled in sleep, stays M11's named candidate beside it.
- **Order 4**: DR-23 is a generator reproducible from its parameters. It sets aside ADR-0187's candidate for another generator, UPG-24's laminar and routing records, which stay unbuilt until a round must store what a generator cannot regenerate.
- **Order 5**: DR-16 is run by the host, at quiescent points, its weight writes part of the run's input. DR-14 needs a stamp of whether the target's last spike was a burst and a home for its slow average; the facilitating pathway it asks for needs short-term plasticity per synapse or a decision against it. DR-15 waits for a design that carries the signal to consolidation, since the apical potential leaks in about 10 ms.
- **Order 6** needs DR-11's decay floor changed and the classes a section of their own, since the modulator section has no reserved byte left. [ADR-0111](0111-a-rule-held-by-the-network.md)'s other route, the rule held in the symbolic layer, which ADR-0125 and ADR-0126 name, is weighed beside it.
- **Order 7**: DR-18 meets the loader's rule that the critic's window equals the shortest delay, and the prior's delay bands are not in the image; DR-19 needs a slot quiescent while it is retargeted and amends [ADR-0011](0011-epoch-based-reclamation.md)'s reclamation.

**Beside ADR-0187's order.** Orders 1 to 3 come before ADR-0187's remaining milestones, M13 (fourth), M14 (fifth), M8's remainder (sixth) and M15 (seventh), since order 3 is M11's step and M13 and M14 wait on M11. After order 3's verdict, an ADR orders this plan's orders 4 to 7 beside those milestones.

### Frames of the plan

These describe how the plan's rounds are to be read. No check enforces them; each round's ADR says how it meets them.
- **DR-26, the ladder of tasks**: the learning line's tasks are taken as a ladder, each with its references (order 1). Later ADRs define the rungs one by one; the review's rungs are candidates: several alternatives with reversals, the line H-29 to H-32 stopped; a mapping no single layer learns (order 4); a delayed match; a sequence; a small control task in fixed point; and a standard spiking data set.
- **DR-21, gradients**: the plan computes no exact gradient inside the engine. Gradient information may enter approximated locally (DR-14, DR-15), estimated by perturbation across forks (DR-16), or computed outside as a ceiling (DR-29, below); a learning round says which it uses.
- **DR-01, what the engine has and tissue lacks**: each planned item names the engineered capability it uses, the table's fourth column. A capability is stated as a mechanism and a reading, never as a comparison with tissue or another system.

### Each by an ADR of its own, when a round needs it

- **DR-03, the admission of a technology**: when a round would add a technology or a dependency, or a port is weighed. Its premises are partly overstated: the doctrine also covers the toolchain and the platform and admitted `criterion` below 1.0 (ADR-0014), a technology can acquire its two years, and the MSRV gate (ADR-0009) and `.spec-brief.json` are not named.
- **DR-06, the tick's duration**, and **DR-07, a working copy of the integrated fields**: after an admissible measurement (DR-04). Both read figures from before ADR-0225; the shortest delay is already known, 100 ticks, and images under another time base are new networks.
- **DR-29, a trainer outside the workspace**: a maintainers' decision, since it would be a second model of the engine's integer rules that no test in the tree holds to the engine.

### Deferred, with what would bring each

- **DR-04, an admissible measurement**: a machine of §7.1's reference platform, which the project does not have on 2026-11-13. M7 stays open.
- **DR-08, the token's width; DR-09, synapses not stored; DR-20's count**: a task whose ceiling rises with the count of synapses ([H-7](../research/H-7.md), T-2). DR-09's premise fails on the stored release; its counter-based generator would make new networks.
- **DR-12, dendritic branches**: a task of the ladder whose ceiling the two-compartment unit cannot reach.
- **DR-30, a kernel in another language**: that language's milestones and DR-03; TC-1 says the engine is written in Rust.
- **DR-31, a freestanding engine**: an admissible measurement that shows the host's share of tail latency (DR-04), and a freestanding target for the engine's language.

### Not adopted

- **DR-02, a nearer stated goal, as written**: the plan's scale is the offline mode's, not real time, and the engine's aims are stated in §1.1 and the quality goals; the ladder (DR-26) may carry a nearer goal.
- **DR-26's gate on the crates**, which would reorder M8's remainder, M14 and M15.
- **A gate or a replay priority the engine sets on its own amendments**, as ADR-0187.

### Findings

- **F-87**: §8.6 said the arenas are allocated from huge pages, with no label; no code in the tree maps huge pages, which §7.1's reference platform and T-2's protocol count as host tuning and §8.7's Specified `mmap` path names. **Resolved** here: §8.6 marks it Specified.
- **F-88**: §2.3 said the tick's duration is a configuration parameter; the fine tick is a constant of the build, `TICK_NS`, which the image's header records and the loader refuses another of ([ADR-0033](0033-tick-duration-in-the-header.md)). **Resolved** here: §2.3 says so.
- **F-89**: ADR-0187 and Appendix C credit M13's imagined rollout as "the lookahead ADR-0099 ordered", and ADR-0187's last alternative names "the lookahead" among the questions its ADRs left open; ADR-0099's lookahead is the executor's synchronisation once per minimum delay, a deferred lever (ADR-0105). **Resolved** in Appendix C's M13 row; ADR-0187's body stands as accepted.
- **F-90**: the threshold's decay is given as $2^{-12}$ of its excess, τ ≈ 41 ms (§5.2.1, `membrane.rs`); with the one-LSB floor one spike's step of 0.02 decays linearly in 13.1 ms, and the exponential holds only above an excess of 0.0625. **Open**: corrected with the round that takes DR-11 (order 6).

## Consequences

- **Good**: every entry of the review has a place, with its premise checked; the next decisions have an order, and the first needs no machine the project lacks.
- **Good**: M11's next step is a mechanism read on the host's side, not another dissection.
- **Bad**: seven planned orders are many rounds, and each verdict may reorder them; how orders 4 to 7 sit beside ADR-0187's milestones is decided later.
- **Bad**: the review's layouts and figures are not trusted; each round derives its own again.
- **Neutral**: no file under `crates/`, `runtime/`, `benches/` or `scripts/` changes.

## Alternatives considered and why rejected

- **Option 2, the review's order as the plan**: several of its premises do not hold in the tree, and it puts an admissible measurement first, which waits for a machine.
- **Option 3, an ADR for each entry**: thirty-one ADRs, most of which would say "deferred".
- **Option 4, nothing taken**: the review names a question the tree left open, credit beyond one layer ([ADR-0187](0187-the-upgrade-suite-planned.md) deferred UPG-01 for want of a hidden layer), and instruments the explorations have lacked.

## Confirmation

- Whitepaper: §2.3 and §8.6 corrected, each with a directive; §11's F-87 to F-90; Appendix C's M11 and M13 rows and a paragraph naming this plan; section 9's row.
- `docs/STATUS.md`, `docs/adr/README.md`, `CHANGELOG.md`.
- `npm run spec` holds the links, the tables, the directives and the version.
