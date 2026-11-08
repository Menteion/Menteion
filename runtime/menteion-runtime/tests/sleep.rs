//! Brief 019's exit test (ADR-0037, ADR-0038; whitepaper §6.6, §8.8): sleep as the executor
//! composes it. The stage machine is stepped once per window on the window's cadence and
//! equals an oracle record stepped the same way, exactly, through a whole cycle (awake until
//! the onset threshold, slow-wave sleep, REM, slow-wave, awake); one tick short of a window
//! boundary nothing has moved. In slow-wave sleep a tagged ring of four units is fired
//! together on every ripple, its four synapses gain exactly the pair rule's amount per ripple
//! after the first (an oracle block fed the same spike ticks), an untagged ring's synapses do
//! not move and its units never fire. In REM the tag falls by one per ripple, a spent episode
//! is skipped, nothing is delivered and nothing fires; a wake between ticks returns the engine
//! to awake with its pressure kept, and a slow-wave engine whose ledger is spent replays
//! nothing. The whole loop is bit-identical on one and four workers; the ledger and the stage
//! round-trip through the image mid-sleep and a loaded engine continues alike; tagging is
//! refused for a full ledger, a missing unit and a bad pattern; the defaults never sleep.
//!
//! What this file does not show: that replay-driven consolidation transfers anything at the
//! reference scale (whitepaper hypothesis H-9). What it holds is the arithmetic: the ripple's
//! schedule, the pattern firing together, and the weights moving by the rule's amount.

#![deny(clippy::arithmetic_side_effects)]

use menteion_connectome::{MenteionFileHeader, SECTION_HOMEOSTASIS, SectionEntry, crc64};
use menteion_core::{
    FLAG_INHIBITORY, ISTDP_ALPHA_Q1_15, MODULATION_ONE_Q16, NO_SPIKE_ON_RECORD, Polarity, STP_MAX,
    STP_U, SYNAPSES_PER_BLOCK, SynapseBlock, THRESHOLD_BASE, spike_message,
};
use menteion_hippocampus::{Episode, HippocampalAttractorState, PATTERN_MAX, RIPPLE_SHIFT};
use menteion_homeostasis::{
    ACTIVITY_BIN_SHIFT, ACTIVITY_WINDOW_BINS, ACTIVITY_WINDOW_SHIFT, HomeostaticDrivePool,
    NIGHT_PHASE, PRESSURE_MAX_Q16, SLEEP_SHIFT_MAX, STAGE_AWAKE, STAGE_REM, STAGE_SWS,
};
use menteion_runtime::{
    Config, ConfigError, Executor, Image, NightHoldPart, TagError, WorkerReport,
};

const WINDOW: u64 = 1 << ACTIVITY_BIN_SHIFT.wrapping_add(ACTIVITY_WINDOW_SHIFT);
const RIPPLE: u64 = 1 << RIPPLE_SHIFT;
/// Two rings of four units: unit `i` fans out to `TARGETS[i]` through slot 0 of block `i`.
const TARGETS: [u32; 8] = [1, 2, 3, 0, 5, 6, 7, 4];
const WEIGHT: i16 = 8_000;

fn config(workers: usize, units: usize, blocks: usize, episodes: usize, sleep_shift: u8) -> Config {
    Config {
        workers,
        units,
        blocks,
        nodes_per_worker: 1 << 12,
        injector_capacity: 256,
        trace_capacity: 1 << 16,
        episodes,
        sleep_shift,
        ..Config::default()
    }
}

/// Every unit armed to fire.
fn arm(exec: &mut Executor<64>) {
    for unit in exec.units_mut() {
        unit.v_thresh = THRESHOLD_BASE;
        unit.stp_u_rel = STP_U;
        unit.stp_r_ves = STP_MAX;
    }
}

/// The two rings of `TARGETS`, one synapse per unit: `WEIGHT`, one tick of delay, basal.
fn wire_rings(exec: &mut Executor<64>) {
    {
        let blocks = exec.blocks_mut();
        for (i, &target) in TARGETS.iter().enumerate() {
            assert!(blocks[i].set_synapse(0, target, WEIGHT, 1, false));
        }
    }
    arm(exec);
    for (i, unit) in exec.units_mut().iter_mut().enumerate() {
        assert!(unit.set_first_block(i as u32));
    }
}

/// The image of `exec` with its homeostasis record patched, decoded under `config`: the way
/// a test puts an engine to sleep, since the stage and the pressure are the image's (§8.3).
fn reload_with(
    exec: &Executor<64>,
    config: Config,
    patch: impl Fn(&mut HomeostaticDrivePool),
) -> Executor<64> {
    let mut img = Image::encode(exec).expect("quiescent");
    let header = MenteionFileHeader::decode(img[0..64].try_into().unwrap());
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        if entry.kind == SECTION_HOMEOSTASIS {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            let mut pool = HomeostaticDrivePool::decode((&img[offset..][..64]).try_into().unwrap());
            patch(&mut pool);
            img[offset..][..length].copy_from_slice(&pool.encode());
            entry.crc64 = crc64(&img[offset..][..length]);
            img[at..][..64].copy_from_slice(&entry.encode());
            break;
        }
    }
    Image::decode::<64>(&img, config).expect("a well-formed record")
}

/// The two rings asleep in `stage` with the regulation off, so the stage is what the image
/// says for as long as the test runs.
fn rings_asleep(workers: usize, stage: u8) -> Executor<64> {
    let mut exec = Executor::<64>::new(config(workers, 8, 8, 2, 0)).unwrap();
    wire_rings(&mut exec);
    reload_with(&exec, config(workers, 8, 8, 2, 0), |p| {
        p.sleep_stage = stage;
        p.sleep_pressure_q16 = PRESSURE_MAX_Q16;
    })
}

/// `(tick, unit)` of every spike, sorted.
fn spikes(reports: &[WorkerReport]) -> Vec<(u32, u32)> {
    let mut spikes: Vec<(u32, u32)> = reports
        .iter()
        .flat_map(|r| r.spikes.iter().map(|&(u, t)| (t, u)))
        .collect();
    spikes.sort_unstable();
    assert!(reports.iter().all(|r| r.dropped == 0), "wholly traced");
    spikes
}

/// The oracle's window: what the executor's tally does to a silent engine's record between
/// two window boundaries, then the sleep step.
fn oracle_window(oracle: &mut HomeostaticDrivePool, units: usize) {
    for _ in 0..ACTIVITY_WINDOW_BINS {
        assert!(oracle.close_bin().is_some());
    }
    oracle.regulate(units as u32);
    oracle.step_sleep();
}

#[test]
fn the_stage_machine_runs_on_the_window_cadence_exactly_as_the_record_s_rule_says() {
    const UNITS: usize = 16;
    let mut exec = Executor::<64>::new(config(1, UNITS, 0, 0, 5)).unwrap();
    let mut oracle = HomeostaticDrivePool {
        sleep_shift: 5,
        ..HomeostaticDrivePool::new()
    };
    assert_eq!(*exec.homeostasis(), oracle);
    // One tick short of the first boundary nothing has moved; at it, the phase and the
    // pressure have.
    exec.run(WINDOW - 1);
    assert_eq!(
        (
            exec.homeostasis().sleep_pressure_q16,
            exec.homeostasis().circadian_phase,
            exec.sleep_stage()
        ),
        (0, 0, STAGE_AWAKE)
    );
    exec.tick();
    oracle_window(&mut oracle, UNITS);
    assert_eq!(*exec.homeostasis(), oracle);
    assert_eq!(
        (
            exec.homeostasis().sleep_pressure_q16,
            exec.homeostasis().circadian_phase
        ),
        (PRESSURE_MAX_Q16 >> 5, 1),
        "a thirty-second of the gap, one phase step"
    );
    // A whole cycle, window by window, against the oracle: the stage changes at the windows
    // the rule says, with the pressure it says.
    let mut transitions = Vec::new();
    let mut window = 1u64;
    let mut previous = STAGE_AWAKE;
    while transitions.len() < 4 {
        exec.run(WINDOW);
        oracle_window(&mut oracle, UNITS);
        window = window.wrapping_add(1);
        assert_eq!(*exec.homeostasis(), oracle, "window {window}");
        let stage = exec.sleep_stage();
        if stage != previous {
            transitions.push((window, stage, exec.homeostasis().sleep_pressure_q16));
            previous = stage;
        }
        assert!(window < 200, "the cycle closes");
    }
    assert_eq!(
        transitions,
        vec![
            (66, STAGE_SWS, 57_462),
            (70, STAGE_REM, 33_684),
            (72, STAGE_SWS, 25_790),
            (73, STAGE_AWAKE, 22_567),
        ],
        "onset at 0.875 after 66 windows, four of slow-wave sleep, two of REM, one more of slow-wave, awake at 0.375"
    );
    assert_eq!(exec.homeostasis().circadian_phase, 73);
    assert_eq!(exec.hippocampus(), &HippocampalAttractorState::new());
    assert_eq!(exec.replays(), 0, "no episode: nothing to replay");
}

#[test]
fn slow_wave_sleep_replays_a_tagged_episode_on_every_ripple_and_consolidates_its_synapses() {
    let mut exec = rings_asleep(2, STAGE_SWS);
    assert_eq!(exec.sleep_stage(), STAGE_SWS);
    assert_eq!(exec.ticks(), 0);
    assert_eq!(exec.tag_episode(&[0, 1, 2, 3], 5), Ok(0));
    // Eight ripples, at ticks 0, 2 048, ..., 14 336; the last pattern's spikes land well
    // inside.
    exec.run(8 * RIPPLE);
    assert_eq!(exec.replays(), 8);
    assert_eq!(exec.depotentiations(), 0);
    let episode = exec.episodes()[0];
    assert_eq!((episode.replays, episode.tag), (8, 5), "counted, not spent");
    assert_eq!(
        exec.hippocampus().replay_hand,
        0,
        "one episode: the hand stays"
    );
    let weights: Vec<i16> = exec.blocks().iter().map(|b| b.weights_q1_15[0]).collect();
    assert_eq!(
        &weights[4..],
        &[WEIGHT; 4],
        "the untagged ring did not move"
    );
    let traces: Vec<i16> = exec
        .blocks()
        .iter()
        .map(|b| b.eligibility_q1_15[0])
        .collect();
    assert_eq!(traces, [0; 8], "consolidated at 1.0: nothing pending");
    assert_eq!(
        exec.homeostasis().sleep_stage,
        STAGE_SWS,
        "the regulation is off"
    );

    let spikes = spikes(&exec.shutdown());
    assert_eq!(spikes.len(), 32, "four units, eight ripples");
    let mut ripple_ticks = Vec::new();
    for (r, chunk) in spikes.chunks(4).enumerate() {
        let tick = chunk[0].0;
        assert!(
            chunk.iter().all(|&(t, _)| t == tick),
            "ripple {r}: the pattern fires together"
        );
        let units: Vec<u32> = chunk.iter().map(|&(_, u)| u).collect();
        assert_eq!(
            units,
            [0, 1, 2, 3],
            "ripple {r}: the tagged ring, nothing else"
        );
        let latency = tick as u64 % RIPPLE;
        assert!(
            (5..=20).contains(&latency),
            "ripple {r}: fired {latency} ticks after the ripple at {tick}"
        );
        assert_eq!(
            tick as u64 / RIPPLE,
            r as u64,
            "one ripple per cadence tick"
        );
        ripple_ticks.push(tick);
    }
    // The pair rule's amount, exactly: an oracle block fed the same spike ticks with the
    // target's last spike at the same tick (potentiation only), consolidated at 1.0.
    let mut oracle = SynapseBlock::new();
    assert!(oracle.set_synapse(0, 1, WEIGHT, 1, false));
    for &t in &ripple_ticks {
        oracle.step_stdp_all(
            t,
            [
                t,
                NO_SPIKE_ON_RECORD,
                NO_SPIKE_ON_RECORD,
                NO_SPIKE_ON_RECORD,
            ],
            Polarity::Excitatory,
            ISTDP_ALPHA_Q1_15,
        );
        oracle.consolidate_all(MODULATION_ONE_Q16, Polarity::Excitatory);
    }
    assert_eq!(&weights[..4], &[oracle.weights_q1_15[0]; 4]);
    assert!(
        weights[0] > WEIGHT + 7 * 115 && weights[0] < WEIGHT + 7 * 125,
        "seven pairings of 120 each after the first ripple: {}",
        weights[0]
    );
}

#[test]
fn rem_lowers_the_tag_per_ripple_skips_a_spent_episode_and_delivers_nothing() {
    let mut exec = rings_asleep(2, STAGE_REM);
    assert_eq!(exec.tag_episode(&[0, 1, 2, 3], 3), Ok(0));
    assert_eq!(exec.tag_episode(&[4, 5], 1), Ok(1));
    let tags = |exec: &Executor<64>| (exec.episodes()[0].tag, exec.episodes()[1].tag);
    // Ripple by ripple: the hand round robin, a spent episode skipped.
    let expected = [(2, 1), (2, 0), (1, 0), (0, 0), (0, 0)];
    for (r, &t) in expected.iter().enumerate() {
        exec.run(RIPPLE);
        assert_eq!(tags(&exec), t, "after ripple {r}");
    }
    assert_eq!(exec.depotentiations(), 4, "the fifth ripple found nothing");
    assert_eq!(exec.replays(), 0);
    assert!(exec.episodes().iter().all(|e| e.replays == 0));
    assert!(exec.hippocampus().replay_hand < 2);
    // A wake between ticks: awake from the next tick, the pressure kept, ripples inert.
    assert_eq!(exec.homeostasis().sleep_pressure_q16, PRESSURE_MAX_Q16);
    assert!(exec.wake());
    assert_eq!(exec.sleep_stage(), STAGE_AWAKE);
    assert!(!exec.wake(), "already awake");
    assert_eq!(exec.homeostasis().sleep_pressure_q16, PRESSURE_MAX_Q16);
    exec.run(2 * RIPPLE);
    assert_eq!(exec.depotentiations(), 4);
    // Back to slow-wave sleep with a spent ledger: nothing to replay.
    let mut sws = reload_with(&exec, config(2, 8, 8, 0, 0), |p| p.sleep_stage = STAGE_SWS);
    assert_eq!(sws.episodes().len(), 2);
    assert!(sws.episodes().iter().all(Episode::is_spent));
    sws.run(3 * RIPPLE);
    assert_eq!((sws.replays(), sws.depotentiations()), (0, 0));
    assert!(spikes(&sws.shutdown()).is_empty(), "nothing fired");
    assert!(
        spikes(&exec.shutdown()).is_empty(),
        "nothing fired in REM or awake"
    );
}

#[test]
fn the_loop_is_bit_identical_on_one_and_four_workers() {
    let outcome = |workers: usize| {
        let mut exec = Executor::<64>::new(config(workers, 8, 8, 2, 5)).unwrap();
        wire_rings(&mut exec);
        let mut exec = reload_with(&exec, config(workers, 8, 8, 2, 5), |p| {
            p.sleep_stage = STAGE_SWS;
            p.sleep_pressure_q16 = PRESSURE_MAX_Q16;
            p.circadian_phase = NIGHT_PHASE;
        });
        assert_eq!(exec.tag_episode(&[0, 1, 2, 3], 5), Ok(0));
        assert_eq!(exec.tag_episode(&[4, 6], 40), Ok(1));
        let mut stages = Vec::new();
        for _ in 0..5 {
            exec.run(WINDOW);
            stages.push(exec.sleep_stage());
        }
        let homeostasis = *exec.homeostasis();
        let hippocampus = *exec.hippocampus();
        let episodes = exec.episodes().to_vec();
        let units: Vec<[u8; 64]> = exec.units().iter().map(|u| u.encode()).collect();
        let blocks = exec.blocks().to_vec();
        let counters = (exec.replays(), exec.depotentiations());
        let spikes = spikes(&exec.shutdown());
        (
            stages,
            homeostasis,
            hippocampus,
            episodes,
            units,
            blocks,
            counters,
            spikes,
        )
    };
    let one = outcome(1);
    let four = outcome(4);
    assert_eq!(
        one.0,
        vec![STAGE_SWS, STAGE_SWS, STAGE_SWS, STAGE_REM, STAGE_REM],
        "four windows of slow-wave sleep, then REM"
    );
    assert_eq!(one.1, four.1, "the homeostasis record");
    assert_eq!(one.2, four.2, "the hippocampal record");
    assert_eq!(one.3, four.3, "the ledger");
    assert_eq!(one.4, four.4, "the unit arenas");
    assert_eq!(one.5, four.5, "the synapse arenas");
    assert_eq!(one.6, four.6, "the counters");
    assert_eq!(one.7, four.7, "the spike trains");
    let ripples_per_window = WINDOW / RIPPLE;
    assert_eq!(
        one.6,
        (4 * ripples_per_window, 45),
        "every ripple of four windows replayed one of the two episodes; REM spent both"
    );
    assert!(one.3.iter().all(Episode::is_spent));
    assert_eq!(
        one.7.len(),
        (2 * ripples_per_window * 4 + 2 * ripples_per_window * 2) as usize,
        "one spike per unit per replay, the two episodes in turn"
    );
}

#[test]
fn the_ledger_and_the_stage_round_trip_through_the_image_mid_sleep_and_a_loaded_engine_continues_alike()
 {
    let mut exec = Executor::<64>::new(config(2, 8, 8, 4, 5)).unwrap();
    wire_rings(&mut exec);
    let mut exec = reload_with(&exec, config(2, 8, 8, 4, 5), |p| {
        p.sleep_stage = STAGE_SWS;
        p.sleep_pressure_q16 = PRESSURE_MAX_Q16;
        p.circadian_phase = NIGHT_PHASE;
    });
    assert_eq!(exec.tag_episode(&[0, 1, 2, 3], 9), Ok(0));
    assert_eq!(exec.tag_episode(&[7, 4], 2), Ok(1));
    // Five bins into the third window, a hundred ticks after a ripple, once the pattern's
    // spikes and their deliveries have landed: the state an image is written from.
    exec.run(2 * WINDOW + 5 * (1 << ACTIVITY_BIN_SHIFT) + 100);
    assert!(exec.is_quiescent());
    let written = (
        *exec.homeostasis(),
        *exec.hippocampus(),
        exec.episodes().to_vec(),
    );
    assert_eq!(written.0.sleep_stage, STAGE_SWS);
    assert!(written.0.window_bins > 0, "mid-window");
    assert!(written.2[0].replays > 50 && written.2[1].replays > 50);
    assert_eq!(written.1.episodes, 2);
    let image = Image::encode(&exec).unwrap();
    // Loaded under a configuration that says no sleep and no room: the image's shift, stage,
    // pressure and ledger are the engine's.
    let mut loaded = Image::decode::<64>(&image, config(2, 8, 0, 0, 0)).unwrap();
    assert_eq!(*loaded.homeostasis(), written.0);
    assert_eq!(*loaded.hippocampus(), written.1);
    assert_eq!(loaded.episodes(), &written.2[..]);
    assert_eq!(loaded.homeostasis().sleep_shift, 5);
    assert_eq!(loaded.ticks(), exec.ticks());
    assert_eq!(loaded.episode_room(), 0);
    assert_eq!(loaded.tag_episode(&[5], 1), Err(TagError::LedgerFull));
    // Both go on through the rest of the cycle: the same replays, the same REM, the same
    // weights.
    for _ in 0..3 {
        exec.run(WINDOW);
        loaded.run(WINDOW);
        assert_eq!(loaded.homeostasis(), exec.homeostasis());
        assert_eq!(loaded.hippocampus(), exec.hippocampus());
        assert_eq!(loaded.episodes(), exec.episodes());
    }
    assert_eq!(exec.sleep_stage(), STAGE_REM, "REM by the fifth window");
    assert_eq!(loaded.blocks(), exec.blocks());
    assert_eq!(
        loaded
            .units()
            .iter()
            .map(|u| u.encode())
            .collect::<Vec<_>>(),
        exec.units().iter().map(|u| u.encode()).collect::<Vec<_>>()
    );
    assert!(
        exec.blocks()[0].weights_q1_15[0] > WEIGHT + 5_000,
        "a hundred and sixty replays potentiated the ring: {}",
        exec.blocks()[0].weights_q1_15[0]
    );
}

#[test]
fn tagging_is_refused_for_a_full_ledger_a_missing_unit_or_a_bad_pattern_and_the_defaults_never_sleep()
 {
    let mut none = Executor::<64>::new(config(1, 4, 0, 0, 0)).unwrap();
    assert_eq!(none.tag_episode(&[0], 1), Err(TagError::LedgerFull));
    assert_eq!(none.episode_room(), 0);
    let mut one = Executor::<64>::new(config(1, 4, 0, 1, 0)).unwrap();
    assert_eq!(one.episode_room(), 1);
    assert_eq!(one.tag_episode(&[0, 4], 1), Err(TagError::NoSuchUnit));
    assert_eq!(one.tag_episode(&[], 1), Err(TagError::InvalidPattern));
    assert_eq!(one.tag_episode(&[1, 1], 1), Err(TagError::InvalidPattern));
    assert_eq!(one.tag_episode(&[1], 0), Err(TagError::InvalidPattern));
    let thirteen: Vec<u32> = (0..PATTERN_MAX as u32 + 1).map(|i| i % 4).collect();
    assert_eq!(one.tag_episode(&thirteen, 1), Err(TagError::InvalidPattern));
    assert!(one.episodes().is_empty(), "nothing was appended");
    one.run(7);
    assert_eq!(one.tag_episode(&[3, 0], 2), Ok(0));
    assert_eq!(one.episodes()[0].tagged_tick, 7, "tagged at this tick");
    assert_eq!(one.episodes()[0].pattern(), &[3, 0]);
    assert_eq!(one.tag_episode(&[1], 1), Err(TagError::LedgerFull));
    assert_eq!(one.episode_room(), 0);
    assert!(matches!(
        Executor::<64>::new(config(1, 1, 0, 0, SLEEP_SHIFT_MAX + 1)),
        Err(ConfigError::SleepShiftOutOfRange)
    ));
    assert!(Executor::<64>::new(config(1, 1, 0, 0, SLEEP_SHIFT_MAX)).is_ok());
    assert!(matches!(
        Executor::<64>::new(config(1, 1, 0, u32::MAX as usize, 0)),
        Err(ConfigError::TooManyEpisodes)
    ));
    // A ripple takes two nodes per unit of the widest pattern on worker 0: a pool below that
    // is refused while the ledger has room, and is not a ledger's business without one.
    assert!(matches!(
        Executor::<64>::new(Config {
            nodes_per_worker: 2 * PATTERN_MAX - 1,
            ..config(1, 4, 0, 1, 0)
        }),
        Err(ConfigError::TooFewNodes)
    ));
    assert!(
        Executor::<64>::new(Config {
            nodes_per_worker: 2 * PATTERN_MAX,
            ..config(1, 4, 0, 1, 0)
        })
        .is_ok()
    );
    assert!(
        Executor::<64>::new(Config {
            nodes_per_worker: 1,
            ..config(1, 4, 0, 0, 0)
        })
        .is_ok()
    );
    assert_eq!(
        (Config::default().sleep_shift, Config::default().episodes),
        (0, 0)
    );
    // The defaults: two windows on, the phase advanced and nothing else moved.
    let mut quiet = Executor::<64>::new(Config {
        units: 1,
        ..Config::default()
    })
    .unwrap();
    quiet.run(2 * WINDOW);
    assert_eq!(
        *quiet.homeostasis(),
        HomeostaticDrivePool {
            circadian_phase: 2,
            ..HomeostaticDrivePool::new()
        }
    );
    assert_eq!(quiet.sleep_stage(), STAGE_AWAKE);
    assert_eq!(quiet.hippocampus(), &HippocampalAttractorState::new());
    assert!(!quiet.wake());
}

// ---------------------------------------- the slow-wave baseline and the ledger spent (ADR-0235)

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../testkit/prop.rs"
));

/// The replay drive's message, given by hand: two of them, as a ripple gives each unit (ADR-0038).
const DRIVE: u32 = spike_message(0x0001_4000, false);

/// The two rings in `stage` with the regulation off and no episode, the image's baseline at
/// `baseline_q16` and the slow-wave baseline at `slow_wave`.
fn rings_under(stage: u8, baseline_q16: i32, slow_wave: Option<i32>) -> Executor<64> {
    let built = Config {
        modulation_baseline_q16: baseline_q16,
        ..config(1, 8, 8, 2, 0)
    };
    let mut exec = Executor::<64>::new(built.clone()).unwrap();
    wire_rings(&mut exec);
    let under = Config {
        slow_wave_baseline_q16: slow_wave,
        ..built
    };
    reload_with(&exec, under, |p| {
        p.sleep_stage = stage;
        p.sleep_pressure_q16 = PRESSURE_MAX_Q16;
    })
}

/// What an engine holds: its units, its synapses, its homeostasis record, its counters and its
/// spikes.
type Held = (
    Vec<[u8; 64]>,
    Vec<SynapseBlock>,
    HomeostaticDrivePool,
    (u64, u64),
    Vec<(u32, u32)>,
);

/// `ripples` ripples' ticks of `exec`, the first ring given the replay drive by hand at the start
/// of each, so that it fires and pairs in every stage: what the engine then holds.
fn driven(mut exec: Executor<64>, ripples: u32) -> Held {
    let inject = exec.injector();
    for _ in 0..ripples {
        for unit in 0..4u32 {
            for _ in 0..2 {
                inject.inject(unit, DRIVE).unwrap();
            }
        }
        exec.run(RIPPLE);
    }
    let units = exec.units().iter().map(|u| u.encode()).collect();
    let blocks = exec.blocks().to_vec();
    let homeostasis = *exec.homeostasis();
    let counters = (exec.replays(), exec.depotentiations());
    (
        units,
        blocks,
        homeostasis,
        counters,
        spikes(&exec.shutdown()),
    )
}

/// The slow-wave baseline over the lattice (ADR-0235): an engine that sets it at `b` and whose
/// own baseline is zero is, bit for bit, the engine that sets none and whose own baseline is `b`
/// in slow-wave sleep and zero in REM and awake; set at the engine's own baseline, it is the
/// engine as it was in every stage. In slow-wave sleep a whole one moves synapses the engine's
/// zero leaves where they were, so the first comparison is not of two engines that do nothing.
#[test]
fn a_slow_wave_tick_consolidates_under_the_slow_wave_baseline_and_every_other_tick_under_the_engine_s()
 {
    let mut lcg = Lcg::new(0x0235);
    let mut baselines: Vec<i32> = I32_LATTICE
        .iter()
        .copied()
        .filter(|b| (0..=MODULATION_ONE_Q16).contains(b))
        .collect();
    for _ in 0..3 {
        baselines.push(lcg.below(MODULATION_ONE_Q16 as u32 + 1) as i32);
    }
    for &b in &baselines {
        for stage in [STAGE_SWS, STAGE_REM, STAGE_AWAKE] {
            let engine_s = if stage == STAGE_SWS { b } else { 0 };
            assert_eq!(
                driven(rings_under(stage, 0, Some(b)), 3),
                driven(rings_under(stage, engine_s, None), 3),
                "{b} {stage}: in the baseline's place in slow-wave sleep alone"
            );
            assert_eq!(
                driven(rings_under(stage, b, Some(b)), 3),
                driven(rings_under(stage, b, None), 3),
                "{b} {stage}: at the engine's own, the engine as it was"
            );
        }
    }
    let whole = driven(rings_under(STAGE_SWS, 0, Some(MODULATION_ONE_Q16)), 3);
    let zero = driven(rings_under(STAGE_SWS, 0, None), 3);
    assert_ne!(whole.1, zero.1, "a whole baseline consolidates");
}

/// The ledger spent (ADR-0235), at every priority of the lattice: a ring tagged and spent before
/// a night is never replayed, and a ring tagged after the call replays on every ripple, as it
/// does in a ledger of its own, its synapses and every spike bit for bit; with no call the two
/// replay in turn.
#[test]
fn a_night_after_the_ledger_is_spent_replays_what_is_tagged_after_and_nothing_before() {
    for priority in U8_LATTICE.iter().copied().filter(|&p| p > 0) {
        let mut spent = rings_asleep(1, STAGE_SWS);
        assert_eq!(spent.tag_episode(&[0, 1, 2, 3], priority), Ok(0));
        spent.spend_episodes();
        assert!(spent.episodes()[0].is_spent());
        assert_eq!(spent.tag_episode(&[4, 5, 6, 7], priority), Ok(1));
        let mut alone = rings_asleep(1, STAGE_SWS);
        assert_eq!(alone.tag_episode(&[4, 5, 6, 7], priority), Ok(0));
        spent.run(4 * RIPPLE);
        alone.run(4 * RIPPLE);
        assert_eq!((spent.replays(), alone.replays()), (4, 4), "{priority}");
        let (first, second) = (spent.episodes()[0], spent.episodes()[1]);
        assert_eq!((first.replays, first.tag), (0, 0), "{priority}");
        assert_eq!((second.replays, second.tag), (4, priority), "{priority}");
        assert_eq!(spent.blocks(), alone.blocks(), "{priority}: the synapses");
        assert_eq!(
            spikes(&spent.shutdown()),
            spikes(&alone.shutdown()),
            "{priority}: the spikes"
        );
    }
    let mut kept = rings_asleep(1, STAGE_SWS);
    assert_eq!(kept.tag_episode(&[0, 1, 2, 3], 5), Ok(0));
    assert_eq!(kept.tag_episode(&[4, 5, 6, 7], 5), Ok(1));
    kept.run(4 * RIPPLE);
    let replays = kept
        .episodes()
        .iter()
        .map(|e| e.replays)
        .collect::<Vec<_>>();
    assert_eq!(replays, [2, 2], "unspent, the two in turn");
}

// ------------------------------------------------------------ exploration 027, on its branch

/// The night's longest run in windows before it wakes, for the bound of `through_the_night`.
const FANNED_WINDOWS_MAX: u32 = 64;

/// The pressure's time constant of `fanned`'s night, short so that the night wakes in a few
/// windows.
const FANNED_SHIFT: u8 = 1;

/// A unit and its group, as exploration 027's hold by target takes them.
type Grouped = (u32, u8);

/// Exploration 027's groups (ADR-0242): unit 0 a held source of group 1, and the targets of its
/// block: unit 1 of group 1, its own, and units 2 and 5 of group 2, the other; unit 3 of none.
const FANNED_SOURCES: [(u32, u8); 1] = [(0, 1)];
const FANNED_TARGETS: [(u32, u8); 3] = [(1, 1), (2, 2), (5, 2)];

/// The slots of unit 0's block each part holds: across, those onto units 2 and 5; within, those
/// onto units 1 and 3.
const ACROSS_SLOTS: [usize; 2] = [1, 3];
const WITHIN_SLOTS: [usize; 2] = [0, 2];

/// The two rings with three more synapses in unit 0's block, onto units 2, 3 and 5, each
/// `WEIGHT`, one tick of delay, basal: asleep in slow-wave sleep at full pressure under
/// `FANNED_SHIFT`, so that the night runs to its end, with the first ring and unit 5 tagged.
fn fanned() -> Executor<64> {
    let mut exec = Executor::<64>::new(config(1, 8, 8, 2, 0)).unwrap();
    wire_rings(&mut exec);
    {
        let block = &mut exec.blocks_mut()[0];
        for (slot, target) in [(1usize, 2u32), (2, 3), (3, 5)] {
            assert!(block.set_synapse(slot, target, WEIGHT, 1, false));
        }
    }
    let mut exec = reload_with(&exec, config(1, 8, 8, 2, 0), |p| {
        p.sleep_stage = STAGE_SWS;
        p.sleep_pressure_q16 = PRESSURE_MAX_Q16;
        p.sleep_shift = FANNED_SHIFT;
    });
    assert_eq!(exec.tag_episode(&[0, 1, 2, 3, 5], 5), Ok(0));
    exec
}

/// What the engine holds after its night, window by window until the window that wakes it,
/// with what exploration 025's and exploration 027's holds cleared at the night's last tick.
fn through_the_night(mut exec: Executor<64>) -> (Held, [(u64, u64, i64, i64); 2]) {
    for _ in 0..FANNED_WINDOWS_MAX {
        if exec.sleep_stage() == STAGE_AWAKE {
            break;
        }
        exec.run(WINDOW);
    }
    assert_eq!(exec.sleep_stage(), STAGE_AWAKE, "the night ended");
    let cleared = [
        exec.night_hold_sources_cleared(),
        exec.night_hold_part_cleared(),
    ];
    let units = exec.units().iter().map(|u| u.encode()).collect();
    let blocks = exec.blocks().to_vec();
    let homeostasis = *exec.homeostasis();
    let counters = (exec.replays(), exec.depotentiations());
    let held = (
        units,
        blocks,
        homeostasis,
        counters,
        spikes(&exec.shutdown()),
    );
    (held, cleared)
}

/// `fanned` with exploration 027's hold set on `part`.
fn fanned_holding(part: NightHoldPart) -> Executor<64> {
    let mut exec = fanned();
    assert!(exec.set_night_hold_by_target(&FANNED_SOURCES, &FANNED_TARGETS, part));
    exec
}

/// Exploration 027's hold by target (ADR-0242), on its branch alone. Unset, set and unset again,
/// or refused, the night is the night as it was, bit for bit, and clears nothing; the two parts
/// together are exploration 025's hold of the same source, bit for bit, what they cleared with
/// them; each part alone leaves its slots at their first weights with their traces cleared, and
/// the other part's slots move as in the night with nothing held.
#[test]
fn a_hold_by_target_holds_its_part_slot_by_slot_and_unset_is_the_night_as_it_was() {
    let (plain, plain_cleared) = through_the_night(fanned());
    assert_eq!(
        plain_cleared,
        [(0, 0, 0, 0); 2],
        "nothing held, nothing cleared"
    );
    let moved: Vec<usize> = (0..SYNAPSES_PER_BLOCK)
        .filter(|&slot| plain.1[0].weights_q1_15[slot] != WEIGHT)
        .collect();
    assert_eq!(
        moved,
        [0, 1, 2, 3],
        "the night moves every slot of unit 0's block"
    );

    for part in [
        NightHoldPart::Across,
        NightHoldPart::Within,
        NightHoldPart::Both,
    ] {
        let mut unset = fanned();
        assert!(unset.set_night_hold_by_target(&FANNED_SOURCES, &FANNED_TARGETS, part));
        assert!(unset.set_night_hold_by_target(&[], &FANNED_TARGETS, part));
        assert_eq!(
            through_the_night(unset),
            (plain.clone(), plain_cleared),
            "{part:?} set and unset: the night as it was"
        );
    }

    // Each refusal changes nothing, so the night runs with nothing held.
    let refused: [(&[Grouped], &[Grouped]); 7] = [
        (&[(0, 0)], &FANNED_TARGETS),
        (&[(8, 1)], &FANNED_TARGETS),
        (&[(0, 1), (0, 1)], &FANNED_TARGETS),
        (&[(7, 1)], &FANNED_TARGETS),
        (&FANNED_SOURCES, &[(8, 1)]),
        (&FANNED_SOURCES, &[(1, 0)]),
        (&FANNED_SOURCES, &[(1, 1), (1, 2)]),
    ];
    for (i, &(sources, targets)) in refused.iter().enumerate() {
        let mut exec = fanned();
        // Unit 7 inhibitory for the fourth: an inhibitory source is refused.
        exec.units_mut()[7].flags |= FLAG_INHIBITORY;
        let mut beside = fanned();
        beside.units_mut()[7].flags |= FLAG_INHIBITORY;
        assert!(
            !exec.set_night_hold_by_target(sources, targets, NightHoldPart::Both),
            "refusal {i}"
        );
        assert_eq!(
            through_the_night(exec),
            through_the_night(beside),
            "refusal {i}: nothing changed"
        );
    }

    let (both, both_cleared) = through_the_night(fanned_holding(NightHoldPart::Both));
    let mut whole = fanned();
    assert!(whole.set_night_hold_sources(&[0]));
    let (sources, sources_cleared) = through_the_night(whole);
    assert_eq!(
        both, sources,
        "both parts: exploration 025's hold, bit for bit"
    );
    assert_eq!(
        (both_cleared[1], both_cleared[0]),
        (sources_cleared[0], (0, 0, 0, 0)),
        "both parts clear what exploration 025's hold clears"
    );
    assert_eq!(both_cleared[1].0, 4, "every slot of unit 0's block");
    assert_ne!(both.1[0], plain.1[0], "the hold holds something");

    for (part, held, other) in [
        (NightHoldPart::Across, ACROSS_SLOTS, WITHIN_SLOTS),
        (NightHoldPart::Within, WITHIN_SLOTS, ACROSS_SLOTS),
    ] {
        let (night, cleared) = through_the_night(fanned_holding(part));
        let block = night.1[0];
        for slot in held {
            assert_eq!(
                (block.weights_q1_15[slot], block.eligibility_q1_15[slot]),
                (WEIGHT, 0),
                "{part:?}: slot {slot} at its first weight, its trace cleared"
            );
        }
        for slot in other {
            assert_ne!(
                block.weights_q1_15[slot], WEIGHT,
                "{part:?}: slot {slot} of the other part moved"
            );
        }
        assert_eq!(
            (cleared[0], cleared[1].0),
            ((0, 0, 0, 0), 2),
            "{part:?}: its two slots cleared, and nothing of exploration 025's"
        );
    }
}
