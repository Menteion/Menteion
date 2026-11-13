---
status: proposed
date: 2026-11-13
title: The design review planned
kind: decision
summary: Of the thirty-one entries of the design review of 2026-11-13, read against the tree, sixteen are planned in seven orders, three frame the plan, four wait for decisions of their own and eight are deferred. Search by forks and a task's references come first; M11's exploration 038 is paused.
decision-makers: Menteion maintainers
---

# ADR-0254: The design review planned

**In brief**
- **Decision.** Nothing is built by this ADR. The thirty-one entries of a design review written outside the tree on 2026-11-13 are numbered DR-01 to DR-31 and given a place each: sixteen are planned in seven orders, each to enter by a decision of its own; three frame the plan; four would move a standing decision and wait for an ADR of their own; eight are deferred, each with what would bring it. The first round is a search by forks with a task's references. M11's next step is a retention gate the host runs, and exploration 038 is paused. Three findings, F-87 to F-89, are recorded.
- **Why.** The maintainers asked that the review be taken into the plan, its claims checked against the tree first. Three readers checked every claim against `main` at `07ffcec`: most hold, and some that a proposal rests on do not.
- **Left open.** Each planned item's hypothesis, stopping rule and layout; where the search tool lives and how it runs; whether the plan's order holds after each verdict; a reference-platform machine; the review's literature, which it cites from memory.

## Context and Problem Statement

On 2026-11-13 the maintainers asked that the engine be improved after a design review written outside this tree that day, read against `07ffcec`. It has thirty-one entries in nine parts, each a problem, a proposed resolution and its rejected alternatives. It names five that would move a standing decision, six that use something the engine has and tissue lacks, and a suggested order: the measurement, a task's references and a search by forks first. Its literature is cited from memory.

The maintainers chose (2026-11-13): check its claims first, then plan it in one ADR as [ADR-0187](0187-the-upgrade-suite-planned.md) planned the upgrade suite; run a search by forks with a task's references as the first round; defer an admissible measurement, since no reference-platform machine is available; and pause exploration 038.

**What checking it against the tree finds.** Three readers checked each claim, file by file. Most hold: the plateau's and the threshold's constants, the free flag bit `0x10`, the 28-bit token and its 2^26 blocks, the record's lack of a reserved byte, the absence of any rule that reads the burst flag or the apical potential, and the lack of any search instrument. Those a proposal rests on and that do not hold:

- **The policy amendment's trial is not a retention gate.** It commits only when the two forks' behaviour hashes are equal and the cost fell (§8.18, [ADR-0031](0031-policy-amendment.md)); a night that changes weights fails it by construction. And an episode of the ledger names no mapping: whether a mapping is answered is the host's to read. A gate on a night's consolidation that the host runs, as H-47's harness forks its reads, moves no boundary; one the engine sets on its own amendments is what ADR-0187 does not adopt and §11.1 refuses (ADR-0057).
- **Slow adaptation needs a new rule.** The threshold's excess decays by `(excess >> shift).max(1)`, so one spike's step of 0.02 is gone in 13.1 ms at any shift of 11 or more; shifts of 16 to 18 change nothing below 1.0.
- **Short-term plasticity is the presynaptic unit's**, its state and class in the unit's record, one class an image: a unit cannot send a depressing pathway to one target and a facilitating one to another.
- **The held-rule line ran six rounds, not four**; [ADR-0115](0115-the-assemblies-marked.md) found one usable cell and ADR-0117 a usable region, neither robust, and the slow current's round ran no cell.
- **The engine's figures predate service on arrival.** The 11 ns turn, the lanes' split and the reach of 10^4 units were read before [ADR-0225](0225-a-unit-served-on-arrival-built.md), after which most unit-ticks are a unit's catch-up, not a turn.
- **A static synapse is not all seed**: a block holds the stored release that a delayed delivery reads back, and the prior is one sequential walk, so no unit's fan-out can be generated alone.
- **The plan's scale is the offline mode's** (Appendix A), not real time, and §1.1 and quality goals 1 and 5 already state determinism and verification as the engine's aims.
- **Others, smaller**: the workspace admits dependencies by TC-2's allow-list; H-29 failed the first reversal, not the later ones; a pairing writes the trace, not the weight, and the zero sum changes weights too; the apical potential leaks in about 10 ms while consolidation reads the modulator at the next presentation.

The readers also found three places where the tree disagrees with itself, recorded below as F-87 to F-89.

## Decision Drivers

- **The repository wins over the document** (principle 1): a proposal's premise is checked against the tree before it is planned.
- **Literature is a prior, not a verdict**, and one change a round: each item enters as a candidate for a measured round.
- **A rule says where it is enforced** (principle 5), and a standing decision moves only by an ADR of its own.
- **ADR-0187's frame**: done, planned in an order, deferred with a condition, not adopted.

## Considered Options

1. **The review sorted** into planned in an order, frames, standing decisions each for its own ADR, and deferred with a condition.
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
| 2 | Which rule changed which weight, when | DR-28, a trace of every weight change, off by default and outside the image | exact repetition | none |
| 3 | M11: does a night kept only when the host's probe shows retention keep a mapping better? | DR-24 as a gate the host runs; DR-25, the replay's order set by what the host tags and spends | fork and compare | order 1's references |
| 4 | A mapping no single layer learns | DR-13, its task; DR-23, a second generator with named populations and projections | — | order 1 |
| 5 | Credit beyond one layer, each rule read against the floor, the ceiling and the others | DR-16 first, then DR-14 (UPG-01), then DR-15 | fork and compare (DR-16) | order 4 |
| 6 | A rule held by a cell, not an assembly | DR-22, its candidates DR-10 and DR-11 | — | order 1; an ADR lifting ADR-0125's pause |
| 7 | What a synapse holds that changes what can be learned | DR-17, DR-18, DR-19, each its own round | — | order 4 passed |

- **Order 1** decides where the tool lives (a module of the harness or a crate that is not a state crate, which changes §4's count of such crates) and how it runs under [ADR-0184](0184-the-runs-off-the-developer-machine.md). A reference is reported beside a new reading; no standing verdict is read again.
- **Order 2** adds no branch, lock or syscall to the tick loop (§8.11): a trace is taken on a second run, as the spike train's ring and `trace_capacity` already are. It names the paths that move a weight: consolidation, the signed consolidation and the zero sum's rounds.
- **Order 3** replaces exploration 038 as M11's next decision. The probe and the mappings are the host's; the engine's rules and the policy amendment do not change.
- **Order 5**: DR-16 is run by the host, at quiescent points, its weight writes part of the run's input. DR-14 needs a stamp of whether the target's last spike was a burst and a home for its slow average; the facilitating pathway it asks for needs short-term plasticity per synapse or a decision against it. DR-15 waits for a design that carries the signal to consolidation, since the apical potential leaks in about 10 ms.
- **Order 6** needs DR-11's decay floor changed and the classes a section of their own, since the modulator section has no reserved byte left. ADR-0125 and ADR-0126's other route, the rule held in the symbolic layer, is weighed beside it.
- **Order 7**: DR-18 meets the loader's rule that the critic's window equals the shortest delay, and the prior's delay bands are not in the image; DR-19 needs a slot quiescent while it is retargeted and amends [ADR-0011](0011-epoch-based-reclamation.md)'s reclamation.

### Frames of the plan

- **DR-26, the ladder of tasks**: the learning line's tasks are ordered as a ladder, each with its references (order 1). Its gate on the crates is not adopted, since it would reorder M8's remainder, M14 and M15; the units a rung is read again at are decided rung by rung.
- **DR-21, gradients**: no exact gradient is computed inside the engine. Gradient information may enter approximated locally (DR-14, DR-15), estimated by perturbation across forks (DR-16), or computed outside as a ceiling (DR-29, deferred), each by its own round; a learning round says which it uses.
- **DR-01, what the engine has and tissue lacks**: each planned item names the engineered capability it uses, the table's fourth column. A capability is stated as a mechanism and a reading, never as a comparison with tissue or another system.

### Each by an ADR of its own, when a round needs it

- **DR-03, the admission of a technology**: when a round would add a technology or a dependency, or a port is weighed. Its premises are part overstated: the doctrine also covers the toolchain and the platform and admitted `criterion` below 1.0 (ADR-0014), a technology can acquire its two years, and the MSRV gate (ADR-0009) and `.spec-brief.json` are not named.
- **DR-06, the tick's duration**, and **DR-07, a working copy of the integrated fields**: after an admissible measurement (DR-04). Both read figures from before ADR-0225; the shortest delay is already known, 100 ticks, and images under another time base are new networks.
- **DR-29, a trainer outside the workspace**: a maintainers' decision on a second implementation of the integer rules that no differential test holds.

### Deferred, with what would bring each

- **DR-04, an admissible measurement**: a machine of §7.1's reference platform, which the project does not have on 2026-11-13. M7 stays open.
- **DR-08, the token's width; DR-09, synapses not stored; DR-20's count**: a task whose ceiling rises with the count of synapses ([H-7](../research/H-7.md), T-2). DR-09's premise fails on the stored release and the sequential prior.
- **DR-12, dendritic branches**: a task whose ceiling the two-compartment unit cannot reach, read by order 1's references.
- **DR-02, a nearer stated goal**: not adopted as written, since the plan's scale is the offline mode's and §1.1 already names determinism and verification; DR-26's ladder may carry a nearer goal.
- **DR-30, a kernel in another language**: that language's milestones and DR-03; TC-1 says the engine is written in Rust.
- **DR-31, a freestanding engine**: not scheduled, as Appendix C's longer-horizon directions.

### Findings

- **F-87**: §8.6 said the arenas are allocated from huge pages, with no label; no code in the tree maps huge pages. **Resolved** here: §8.6 marks it Specified.
- **F-88**: §2.3 said the tick's duration is a configuration parameter; it is a constant of the build, `TICK_NS`, which the image's header records and the loader holds. **Resolved** here: §2.3 says so.
- **F-89**: ADR-0187 and Appendix C credit M13's imagined rollout as "the lookahead ADR-0099 ordered"; ADR-0099's lookahead is the executor's synchronisation once per minimum delay, a deferred lever (ADR-0105). **Resolved** in Appendix C's M13 row; ADR-0187's body stands as accepted.

## Consequences

- **Good**: every entry of the review has a place, with its premise checked; the next decisions have an order, and the first needs no machine the project lacks.
- **Good**: M11's next step is a mechanism read on the host's side, not another dissection.
- **Bad**: seven planned orders are many rounds, and each verdict may reorder them.
- **Bad**: the review's layouts and figures are not trusted; each round derives its own again.
- **Neutral**: no file under `crates/`, `runtime/`, `benches/` or `scripts/` changes.

## Alternatives considered and why rejected

- **Option 2, the review's order as the plan**: several of its premises do not hold in the tree, and it puts an admissible measurement first, which waits for a machine.
- **Option 3, an ADR for each entry**: thirty-one ADRs, most of which would say "deferred".
- **Option 4, nothing taken**: the review names questions the tree's own ADRs left open, credit beyond one layer and a task's references among them.

## Confirmation

- Whitepaper: §2.3 and §8.6 corrected, §11's F-87 to F-89, Appendix C's M11 and M13 rows and a paragraph naming this plan; section 9's row.
- `docs/STATUS.md`, `docs/adr/README.md`, `CHANGELOG.md`.
- `npm run spec` holds the links, the tables and the version.
