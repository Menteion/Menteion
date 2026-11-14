# What Mente may port from Menteion

Menteion's development is paused ([ADR-0258](adr/0258-development-paused.md)). Mente, a systems language, names a Menteion kernel as
its systems workload (its register's `success-workload` and `zero-cost-measured`, in `docs/design-review.md` of the Mente
repository). This document is for that port: the kernels that could be reproduced bit for bit, the tests that hold each, and the
mechanisms the whole engine asks of a language. It describes the tree; it decides nothing for Mente.

**The frozen tree.** Every path and test here is at commit `58228c9`, `main` after [ADR-0257](adr/0257-a-night-kept-by-the-hosts-probe.md)
merged. The closing round that paused development changed no file under `crates/`, `runtime/`, `benches/` or `testkit/`, and the tag
`menteion-paused-2026-11-15` marks its merge. The kernel files last changed on 2026-10-27 to 2026-11-02. A port is held to the tests
named here at that tag. During the pause a test that pins a kernel already in the tree, or a correction to this document, may still
merge by a pull request of its own (ADR-0258); this document then names it.

## What a port meets first

- **Menteion has no branchless kernel and claims none.** No file under `crates/`, `runtime/` or `benches/`, and no document, says
  branchless, branch-free, cmov or csel. `integrate` branches on data: the leak three ways on the sign
  (`crates/menteion-core/src/dynamics/membrane.rs`, `leak`), and a refractory selection and an early return. Mente's example
  `integrate_cell` (`examples/02_mechanical_sympathy_simd.mn` in Mente) is a simpler rule in its spirit: its leak `v - (v >> k)` has
  no one-LSB floor and no sign handling, one compartment and no refractory window. A port that must be `@branchless` needs an
  equivalent form of Menteion's rule, proved equal by the tests below.
- **Menteion names every overflow.** Plain `+ - * / %` is a Clippy error in every crate root (`clippy::arithmetic_side_effects`); each
  sum is `saturating_*`, `wrapping_*` or `checked_*`, and each narrowing `as` cast is justified by a comment. Mente traps on overflow
  and never wraps implicitly (its `spec/refinement-and-fixed-point.md`). Those sites are what `zero-cost-measured` counts; each
  kernel below gives its count.
- **Port `testkit/prop.rs` first** (173 lines, no dependencies): the MMIX LCG (`Lcg`), FNV-1a and the edge-biased lattices
  (`I32_LATTICE`, `U32_LATTICE`). Every property test draws its walks from it, so both sides can draw the same walks.
- **No floats.** `f32` and `f64` are a Clippy error (`clippy.toml`); Q16.16 lives in `i32`, widened to `i64` to multiply.

## Kernel candidates

| Kernel | Files | Code lines | What it asks of a language | Overflow sites | Fits Mente's |
| :--- | :--- | ---: | :--- | :--- | :--- |
| K1. Membrane integration | `crates/menteion-core/src/dynamics/membrane.rs`, `neuron.rs` | about 70 | Q16.16 in `i32` widened to `i64`; saturating add and sub; arithmetic shift right of negatives; `unsigned_abs`, `min`, `max`; `checked_sub` on `u16`; a 64-byte record aligned to 64 | 10 saturating, 1 checked, 12 casts | M3 (record), M5 (fixed point) |
| K2. Short-term plasticity and efficacy | `crates/menteion-core/src/dynamics/plasticity.rs`, `neuron.rs` | about 92 | Q0.8, Q1.15 and Q16.16 in one `i64` product; round to nearest; shift amounts clamped; `leading_zeros`; a loop over a computed bit count | 24 saturating, 31 casts | M5 |
| K3. STDP into eligibility, and three-factor consolidation | `crates/menteion-core/src/dynamics/synapse.rs` | about 250 | saturating `i16` traces; a wrapping `u32` tick difference read as signed; range patterns; `isqrt`; `const fn` evaluated at compile time | 25 saturating, 4 wrapping, 32 casts | M5 |
| K4. Two-tier timing wheel | `crates/menteion-core/src/dispatch/wheel.rs`, `cadence.rs` | 143 | const generics; a 4 195 336-byte value built without passing through the stack; a `const fn new` that asserts; power-of-two masks; a borrowed slice returned | few | M3, M4 |
| K5. Image codec: CRC-64/XZ, header and record bytes | `crates/menteion-connectome/src/lib.rs`, `crates/menteion-core/src/serial.rs` | about 350 | `u64` shift and xor; little-endian bytes in fixed arrays; `repr(C, align(64))` | none at risk | M1 to M3 |
| K6. Critic and modulator | `crates/menteion-neuromod/src/lib.rs` | 112 | generics over an iterator and a fold, unless rewritten over a slice; saturating `i64`; clamped shifts | few | M4 |

### K1. Membrane integration

- **Items**: `DendriticSuperNeuron::integrate` with `leak` and `add` (widen to `i64`, clamp to `i32`) and the constants above them.
  Beside it, optional: `SlowCurrent` and `integrate_slow`, `quiet_cannot_fire`, `quiet_catch_up` and `threshold_after`.
- **Record**: `DendriticSuperNeuron` (`neuron.rs`), `#[repr(C, align(64))]`, 64 bytes, asserted in a `const _` block. It holds two
  atomics, `mailbox_head_ptr: AtomicU64` at bytes 8 to 16 and `gate_state: AtomicU8` at byte 56, so it is a control record and not
  `Copy`. `integrate` writes plain fields only: `v_soma`, `v_basal`, `v_apical`, `v_thresh` (Q16.16 `i32`), two `u16` countdowns,
  `last_soma_spike_tick` and bit 0 of `flags`.
- **In and out**: `(&mut DendriticSuperNeuron, basal_q16: i32, apical_q16: i32, now_tick: u32) -> bool`, whether it fired.
- **Tests**: pinned values `a_soma_that_lands_exactly_on_a_positive_threshold_fires_and_one_lsb_short_does_not` and
  `the_threshold_steps_up_per_spike_and_decays_back_to_its_base`; 13 property tests in `membrane.rs`'s `mod prop`, among them
  `a_million_edge_biased_ticks_keep_every_invariant_and_never_panic`,
  `every_lattice_pair_drives_a_unit_for_a_thousand_ticks_without_a_panic`,
  `with_no_slow_potential_and_no_slow_input_the_slow_rule_is_integrate_bit_for_bit` with
  `the_slow_rule_is_adr_0122_s_by_an_i64_oracle_at_any_state` (an independent `i64` oracle), and
  `quiet_catch_up_is_integrate_with_no_input_bit_for_bit_and_counts_the_turns_served`. No hash pins `integrate` alone.
- **Depends on**: the record alone. **Difficulty**: low; medium with the catch-up, whose loops end by construction.

### K2. Short-term plasticity and synaptic efficacy

- **Items**: `stp_decay_factor_q16` (binary exponentiation over the bits), `relax_q0_8`, `step_stp`, `step_stp_class`, `StpClass`, and
  `synaptic_efficacy_q16` in `neuron.rs`.
- **In and out**: `stp_u_rel` and `stp_r_ves` (Q0.8 `u8` at bytes 58 and 59) and `elapsed_ticks: u32` to `(u8, u8)`; efficacy from
  `(i16 Q1.15, u8 Q0.8, u8 Q0.8)` to `i32` Q16.16.
- **Tests**: pinned values `full_weight_and_full_stp_is_just_below_one` (65 023), `most_negative_weight_stays_in_range` (−65 025),
  `half_weight_half_release_full_resource` (16 320) and `flooring_is_toward_negative_infinity` (−2); seven property tests, among
  them `the_class_s_step_is_tsodyks_markram_at_any_constants_by_an_i64_oracle`; one `#[ignore]`d test over all 2^32 efficacy inputs.
- **Depends on**: the record alone. **Difficulty**: low, and the natural test of mixed fixed-point formats.

### K3. STDP into eligibility, and three-factor consolidation

- **Items** in `synapse.rs`: `Polarity`, `window`, `depression_at`, `DepressionExponent`, `depression_magnitude` (`u64::isqrt`),
  `decayed`, `release` and `release_all`, `step_stdp_shaped` and `step_stdp_all_shaped`, `decay_eligibility`, `consolidate`,
  `consolidate_signed` and `consolidate_against`, with const asserts that call `istdp_alpha_q1_15` at compile time.
- **Record**: `SynapseBlock` (`neuron.rs`), 64 bytes, aligned to 64, `Copy`.
- **In and out**: `&mut SynapseBlock`, a `u32` tick, the targets' last spikes `[u32; 4]`, a `Polarity`, an `i16` alpha and an `i32`
  Q16.16 modulation, to traces or weights `[i16; 4]`.
- **Tests**: 21 unit tests, among them `pre_before_post_potentiates_and_post_before_pre_depresses_by_the_window`,
  `consolidation_moves_the_modulated_fraction_and_conserves_the_sum` and
  `stamps_pair_correctly_across_the_tick_wrap_and_a_stale_stamp_pairs_with_nothing`; seven property tests. No independent oracle
  holds `step_stdp`.
- **Depends on**: K2's `stp_decay_factor_q16`. **Difficulty**: medium.

### K4. Two-tier timing wheel

- **Items**: `FlatTimingWheel<const CAP: usize>` with `schedule`, `advance`, `cascade` and `push`; `WorkerWheel` is
  `FlatTimingWheel<2048>`, 4 195 336 bytes, asserted. `Cadence` in `cadence.rs`.
- **In and out**: a `u32` delay and a 28-bit token to `Result<(), ScheduleError>`; `advance()` returns `&[u32]`, valid to the next call.
- **Tests**: eight unit tests, among them `delivered_on_exactly_the_requested_advance` and
  `delivery_order_is_deterministic_across_both_rings`; the property test
  `every_token_is_due_exactly_once_at_exactly_its_tick_and_two_wheels_agree`.
- **Depends on**: nothing. The executor builds wheels on a thread with a large stack (F-34). **Difficulty**: low to medium.

### K5. Image codec

- **Items**: `Crc64` and `crc64` (bitwise, no table), `MenteionFileHeader` with `FORMAT_VERSION = 26`, `SectionEntry`, and the record
  bytes `DendriticSuperNeuron::encode`/`decode` and `SynapseBlock::encode`/`decode` (`serial.rs`, atomics loaded as plain values). The
  whole loader, `runtime/menteion-runtime/src/image.rs`, uses `std`, `Vec` and the file system and is a larger port.
- **Tests**: `the_crc_matches_the_published_check_value_and_streams` (`crc64(b"123456789")` is `0x995D_C9BB_DF19_39FA`, an external
  value); `a_header_round_trips_and_validates`; five round trips in `serial.rs`; 27 tests in `runtime/menteion-runtime/tests/image.rs`.
- **Difficulty**: low for the state crates' part; medium to high for the whole loader.

### K6. Critic and modulator

- **Items**: `NeuromodulatorState` (16 bytes, aligned to 16) with `modulation`, `signed_modulation`, `reward` and `decay_dopamine`;
  `ValueCritic` with `value_q16`, `error_q16`, `received_q16` and `step`.
- **Tests**: 10 unit tests and four property tests, two against `i128` oracles. **Difficulty**: low, but not a hot loop.

### Compositions, once single kernels hold

- **C1. The single-threaded oscillator** (`crates/menteion-core/tests/oscillator.rs`, core crate only): the wheel, the mailbox, the
  gate's claim, `integrate`, `release` and fan-out, with periods pinned at 1 527, 3 793 and 2 433 ticks. It needs atomics with
  sequentially consistent and acquire-release orderings.
- **C2. The T-1 network**, the strongest bit-for-bit target the tree has:
  `the_random_network_hashes_to_the_pinned_value_on_every_architecture` (`runtime/menteion-runtime/tests/differential.rs`). 128 units
  and 128 blocks run 20 000 ticks on one worker, and the CRC-64 of their bytes and spikes is pinned at `0x6c27858ece2dd412`
  (`PINNED_ARENA_HASH`), on x86-64 and on AArch64. It needs K1 to K5 and the executor's phase order; within 20 000 ticks the gain's
  regulation, sleep and the ripple do not run.

**The order a port could take**: K1 with K2 at Mente's M5; K4 and K5 at M3 and M4; then C1, and C2 at M7.

## What the whole engine asks of a language

| Mechanism | Label | Where it is, or that it is not |
| :--- | :--- | :--- |
| 64-byte records aligned to 64, `#[repr(C)]`, size asserted at compile time | Implemented | every state crate's `const _` blocks |
| Q16.16 in `i32`, `i64` products, saturating state arithmetic, no floats, arithmetic only by name | Implemented | `clippy.toml`; `#![deny(clippy::arithmetic_side_effects)]` in every crate root |
| `no_std` state crates with no heap and no dependencies | Implemented | all 32 state crates; `npm run spec:deps`; spec-guard's absence directives |
| No allocation after start-up | Implemented | `runtime/menteion-runtime/tests/no_alloc.rs`, a counting `GlobalAlloc` |
| The turn gate, a control record with atomics | Implemented | `neuron.rs`: a sequentially consistent compare-and-swap, acquire-release turns |
| The mailbox, a lock-free stack, and node pools | Implemented | `neuron.rs`, `runtime/menteion-runtime/src/pool.rs` |
| A worker pool, each owning a range of the arena, three phases a tick | Implemented | `runtime/menteion-runtime/src/executor.rs` |
| A spin barrier that yields after 1 024 spins, the loop's only system call | Implemented | `runtime/menteion-runtime/src/barrier.rs` |
| A bounded multi-producer ring for injection | Implemented | `runtime/menteion-runtime/src/injector.rs` |
| The timing wheel and cadence | Implemented | K4 |
| The image format, its header and version (26), CRC-64/XZ | Implemented | K5 |
| One `unsafe` invariant: a `&mut` to a record never overlaps another reference to it | Implemented ([ADR-0023](adr/0023-executor.md)) | `runtime/menteion-runtime/src/arena.rs`; `unsafe_code = "forbid"` elsewhere |
| A run identical on one, two and four workers | Implemented | `runtime/menteion-runtime/tests/differential.rs` |
| A run identical on x86-64 and AArch64 at 20 000 ticks | Implemented; the 10^6-tick image form is a Target | C2 |
| Abort on a broken invariant | Implemented | `executor.rs`, `std::process::abort` |
| Core pinning | Specified | not in the code |
| `mmap`, huge pages | Specified (F-87) | not in the code; the loader copies |
| A seccomp filter on the workers | Specified | not in the code |
| Epoch-based reclamation | Specified ([ADR-0011](adr/0011-epoch-based-reclamation.md), F-93) | not in the code |

The executor's work-stealing deques of ADR-0023 were replaced by a schedule bitmap ([ADR-0100](adr/0100-the-sweep-without-the-gate.md));
the runtime crate's description still names them (F-92).
