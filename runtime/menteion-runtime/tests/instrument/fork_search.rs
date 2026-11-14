//! The search by forks and the writer of the four stimulus–readout couplings (ADR-0255, brief
//! 088), a module the `inhibition` binary alone includes by path, so its tests run once.
//!
//! The search is a genetic algorithm in integers. It takes genes, each a named integer with
//! inclusive bounds and a start value; an objective, a whole-number score of a candidate's genes
//! on a stream, which a caller reads from a fork of its image; and P, G, E, M, S and a seed. Every
//! draw is `Lcg`'s from the seed, so a search repeats bit for bit, and it prints every candidate
//! it scores and the best's scores on the streams it never saw. It decides nothing: what it
//! finds is a reference, never a constant of the engine or of a later round.
//!
//! The writer scales the synapses one coupling counts by a factor in units of 1/256, and no other
//! synapse.

use crate::*;

// ----------------------------------------------------------------------------- the search

/// A gene: a named integer, its inclusive bounds and the value generation 0 starts from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Gene {
    pub(crate) name: &'static str,
    pub(crate) min: i32,
    pub(crate) max: i32,
    pub(crate) start: i32,
}

/// A search's sizes and seed: P candidates a generation, G generations, the E best kept, a step
/// of at most M either way, each candidate scored on S streams, and the seed of its `Lcg`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Breeding {
    pub(crate) population: usize,
    pub(crate) generations: usize,
    pub(crate) kept: usize,
    pub(crate) step: i32,
    pub(crate) streams: u32,
    pub(crate) seed: u64,
}

/// A candidate as it was scored: the generation and the index it was scored at, its genes, and
/// its score on each of the streams 0 to S − 1.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Scored {
    pub(crate) generation: usize,
    pub(crate) index: usize,
    pub(crate) genes: Vec<i32>,
    pub(crate) scores: Vec<i64>,
}

impl Scored {
    /// The sum of its scores, the rank's key.
    pub(crate) fn sum(&self) -> i64 {
        self.scores
            .iter()
            .fold(0i64, |sum, &s| sum.saturating_add(s))
    }
}

/// What a search did: every candidate scored, in the order scored; the best of the last
/// generation; its scores on the streams S to 2S − 1, which it never saw; and the lines printed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Searched {
    pub(crate) scored: Vec<Scored>,
    pub(crate) best: Scored,
    pub(crate) unseen: Vec<i64>,
    pub(crate) lines: Vec<String>,
}

/// The search over `genes` under `breeding` (ADR-0255). Generation 0 is the start genes and
/// P − 1 candidates drawn uniformly within the bounds. Each new candidate is scored on streams 0
/// to S − 1 and a generation is ranked by the sum, ties by the index in the generation, where the
/// kept come first in their rank and the children follow in the order made. The E best are kept
/// with their scores and not read again; each other place takes a child of two parents, each the
/// better ranked of two members drawn uniformly. Each gene comes from either parent with
/// probability 1/2, then moves with probability 1/4 by a step drawn uniformly from −M to M, held
/// within its bounds. After G generations the best of the last is read again on streams S to
/// 2S − 1. P + (G − 1)(P − E) candidates are scored, and each line is printed as it is made,
/// under `label`.
pub(crate) fn search_by_forks(
    genes: &[Gene],
    breeding: Breeding,
    label: &str,
    objective: &mut dyn FnMut(&[i32], u32) -> i64,
) -> Searched {
    let Breeding {
        population,
        generations,
        kept,
        step,
        streams,
        seed,
    } = breeding;
    assert!(
        population >= 2 && kept < population && generations >= 1 && streams >= 1 && step >= 0,
        "a search of at least two candidates, fewer kept, a generation, a stream and a step"
    );
    let unseen_end = streams
        .checked_mul(2)
        .expect("the unseen streams are numbered");
    let places = u32::try_from(population).expect("a population a draw reaches");
    let steps = u32::try_from(step)
        .ok()
        .and_then(|m| m.checked_mul(2))
        .and_then(|m| m.checked_add(1))
        .expect("a step a draw reaches");
    for gene in genes {
        assert!(
            gene.min <= gene.start && gene.start <= gene.max,
            "{}'s start {} within {}..={}",
            gene.name,
            gene.start,
            gene.min,
            gene.max
        );
        assert!(
            span(gene).is_some(),
            "{}'s bounds a draw reaches",
            gene.name
        );
    }
    let mut lines = Vec::new();
    let mut print = |line: String| {
        eprintln!("{line}");
        lines.push(line);
    };
    let names: Vec<&str> = genes.iter().map(|g| g.name).collect();
    let bounds: Vec<(i32, i32)> = genes.iter().map(|g| (g.min, g.max)).collect();
    let start: Vec<i32> = genes.iter().map(|g| g.start).collect();
    print(format!(
        "DUMP search {label} genes {names:?} bounds {bounds:?} start {start:?} P {population} G {generations} E {kept} M {step} S {streams} seed {seed:#018x}"
    ));
    let mut rng = Lcg::new(seed);
    let mut scored: Vec<Scored> = Vec::new();
    let mut score = |generation: usize, index: usize, candidate: Vec<i32>| {
        let scores: Vec<i64> = (0..streams).map(|s| objective(&candidate, s)).collect();
        let one = Scored {
            generation,
            index,
            genes: candidate,
            scores,
        };
        print(format!(
            "DUMP search {label} g {generation} i {index} genes {:?} scores {:?} sum {}",
            one.genes,
            one.scores,
            one.sum()
        ));
        scored.push(one.clone());
        one
    };
    let mut generation: Vec<Scored> = Vec::new();
    generation.push(score(0, 0, start.clone()));
    for index in 1..population {
        let drawn: Vec<i32> = genes.iter().map(|g| drawn_within(&mut rng, g)).collect();
        generation.push(score(0, index, drawn));
    }
    let mut ranked = ranked_of(generation);
    for g in 1..generations {
        let mut next: Vec<Scored> = ranked.iter().take(kept).cloned().collect();
        for index in kept..population {
            let first = tournament(&mut rng, places);
            let second = tournament(&mut rng, places);
            let mut child = Vec::new();
            for (j, gene) in genes.iter().enumerate() {
                let parent = if rng.below(2) == 0 { first } else { second };
                let mut value = ranked[parent].genes[j];
                if rng.below(4) == 0 {
                    let moved = i64::from(rng.below(steps)).saturating_sub(i64::from(step));
                    value = held_within(i64::from(value).saturating_add(moved), gene);
                }
                child.push(value);
            }
            next.push(score(g, index, child));
        }
        ranked = ranked_of(next);
    }
    let best = ranked.swap_remove(0);
    let unseen: Vec<i64> = (streams..unseen_end)
        .map(|s| objective(&best.genes, s))
        .collect();
    let unseen_sum = unseen.iter().fold(0i64, |sum, &s| sum.saturating_add(s));
    print(format!(
        "DUMP search {label} best g {} i {} genes {:?} scores {:?} unseen {unseen:?} sum {unseen_sum}",
        best.generation, best.index, best.genes, best.scores
    ));
    Searched {
        scored,
        best,
        unseen,
        lines,
    }
}

/// The count of values within a gene's bounds, when a draw reaches them all.
fn span(gene: &Gene) -> Option<u32> {
    let width = i64::from(gene.max)
        .saturating_sub(i64::from(gene.min))
        .saturating_add(1);
    u32::try_from(width).ok()
}

/// A value drawn uniformly within a gene's bounds.
fn drawn_within(rng: &mut Lcg, gene: &Gene) -> i32 {
    let span = span(gene).expect("bounds a draw reaches");
    held_within(
        i64::from(gene.min).saturating_add(i64::from(rng.below(span))),
        gene,
    )
}

/// A value held within a gene's bounds.
fn held_within(value: i64, gene: &Gene) -> i32 {
    let held = value.clamp(i64::from(gene.min), i64::from(gene.max));
    i32::try_from(held).expect("within an i32's bounds")
}

/// A generation in its rank: by the sum of the scores, the larger first, ties by the index in
/// the generation, which is the order it is given in.
fn ranked_of(generation: Vec<Scored>) -> Vec<Scored> {
    let mut ranked = generation;
    ranked.sort_by_key(|c| core::cmp::Reverse(c.sum()));
    ranked
}

/// A tournament of two: two places drawn uniformly from a ranked generation of `places`, the
/// better ranked taken.
fn tournament(rng: &mut Lcg, places: u32) -> usize {
    let a = rng.below(places);
    let b = rng.below(places);
    a.min(b) as usize
}

// ----------------------------------------------------------------------------- the writer

/// A weight scaled by `factor` in units of 1/256: the product widened to `i64`, rounded to the
/// nearest with a half away from zero, and held within ±32 767.
pub(crate) fn scaled_weight(weight: i16, factor: i32) -> i16 {
    let product = i64::from(weight).saturating_mul(i64::from(factor));
    let magnitude = product.unsigned_abs().saturating_add(128).wrapping_shr(8);
    let held = i64::try_from(magnitude)
        .unwrap_or(i64::MAX)
        .min(i64::from(i16::MAX));
    i16::try_from(held.saturating_mul(product.signum())).expect("held within ±32 767")
}

/// Scales each synapse `coupling` counts from `from` into `into` — from an excitatory unit of
/// `from` onto any unit of `into` — by `factor` in units of 1/256 (`scaled_weight`), and no
/// other synapse; returns the count written. At 256 it leaves the engine as it was.
pub(crate) fn scale_coupling(exec: &mut Engine, from: Set, into: Set, factor: i32) -> usize {
    let mut slots: Vec<(usize, usize)> = Vec::new();
    for unit in exec.units() {
        if unit.flags & FLAG_INHIBITORY != 0 || !from.contains(unit.id as u32) {
            continue;
        }
        for s in unit.fan_out(exec.blocks()) {
            if into.contains(s.target) {
                slots.push((s.block_idx as usize, usize::from(s.slot)));
            }
        }
    }
    let blocks = exec.blocks_mut();
    for &(block, slot) in &slots {
        let w = &mut blocks[block].weights_q1_15[slot];
        *w = scaled_weight(*w, factor);
    }
    slots.len()
}

/// The four couplings of a geometry scaled, in `couplings`' order — A→R0, A→R1, B→R0, B→R1 —
/// each by its factor in units of 1/256; returns the count written for each.
pub(crate) fn scale_couplings(exec: &mut Engine, sets: &[Set; 4], factors: [i32; 4]) -> [usize; 4] {
    let [a, b, r0, r1] = *sets;
    [
        scale_coupling(exec, a, r0, factors[0]),
        scale_coupling(exec, a, r1, factors[1]),
        scale_coupling(exec, b, r0, factors[2]),
        scale_coupling(exec, b, r1, factors[3]),
    ]
}

// ----------------------------------------------------------------------------- the tests

/// The sizes ADR-0255 fixes for the tests: P 16, G 10, E 4, M 16, S 3.
const TESTED: Breeding = Breeding {
    population: 16,
    generations: 10,
    kept: 4,
    step: 16,
    streams: 3,
    seed: 0,
};

/// A stream's noise on a candidate, 0 or 1: a read that moves from stream to stream.
fn noise(genes: &[i32], stream: u32) -> i64 {
    let key = genes
        .iter()
        .fold(0x0088_0255u64, |h, &g| mix64(h ^ u64::from(g as u32)));
    i64::from(mix64(key ^ u64::from(stream)) & 1 == 1)
}

/// Two genes in the box 0..=8, starting at its middle. A step of M = 16 is held at an edge most
/// times it moves in a box this small, so an inner best is reached through the draws and the
/// crossover, and from some seeds only: the tests fix one that reaches both bests by a child.
fn small_box() -> [Gene; 2] {
    let gene = |name| Gene {
        name,
        min: 0,
        max: 8,
        start: 4,
    };
    [gene("x"), gene("y")]
}

/// The seed the search reaches both bests from, each in a generation after the first.
const REACHING: u64 = 16;

/// The corner objective: four times the genes' sum and the stream's noise. Its best on any
/// streams is the box's top corner alone, since a step of one below it costs four on each stream
/// and the noise gives at most one.
fn corner(genes: &[i32], stream: u32) -> i64 {
    let sum = genes
        .iter()
        .fold(0i64, |s, &g| s.saturating_add(i64::from(g)));
    sum.saturating_mul(4).saturating_add(noise(genes, stream))
}

/// The point inside the box the inner objective is best at.
const INSIDE: [i32; 2] = [3, 5];

/// The inner objective: four times the squared distance from `INSIDE`, negated, and the stream's
/// noise. Its best is `INSIDE` alone, for the same reason as the corner's.
fn inner(genes: &[i32], stream: u32) -> i64 {
    let distance = genes.iter().zip(INSIDE).fold(0i64, |d, (&g, t)| {
        let off = i64::from(g).saturating_sub(i64::from(t));
        d.saturating_add(off.saturating_mul(off))
    });
    distance
        .saturating_mul(-4)
        .saturating_add(noise(genes, stream))
}

#[test]
fn the_search_reaches_the_best_at_a_corner_and_inside_a_small_box_from_its_seed() {
    let genes = small_box();
    let reaching = Breeding {
        seed: REACHING,
        ..TESTED
    };
    let mut read = |g: &[i32], s: u32| corner(g, s);
    let found = search_by_forks(&genes, reaching, "corner", &mut read);
    assert_eq!(found.best.genes, vec![8; 2], "the top corner");
    assert!(found.best.generation > 0, "reached by a child");
    let mut read = |g: &[i32], s: u32| inner(g, s);
    let found = search_by_forks(&genes, reaching, "inside", &mut read);
    assert_eq!(found.best.genes, INSIDE.to_vec(), "the point inside");
    assert!(found.best.generation > 0, "reached by a child");
    let unseen: Vec<i64> = (3..6).map(|s| inner(&INSIDE, s)).collect();
    assert_eq!(found.unseen, unseen, "the best read on streams 3 to 5");
    assert_eq!(
        found.best.scores,
        (0..3).map(|s| inner(&INSIDE, s)).collect::<Vec<_>>()
    );
    let last = found.lines.last().expect("the best's line");
    assert_eq!(
        last,
        &format!(
            "DUMP search inside best g {} i {} genes {INSIDE:?} scores {:?} unseen {unseen:?} sum {}",
            found.best.generation,
            found.best.index,
            found.best.scores,
            unseen.iter().fold(0i64, |sum, &s| sum.saturating_add(s))
        )
    );
}

#[test]
fn one_seed_prints_the_same_table_twice_and_another_seed_another() {
    let genes = small_box();
    let mut read = |g: &[i32], s: u32| inner(g, s);
    let once = search_by_forks(&genes, TESTED, "seed", &mut read);
    let twice = search_by_forks(&genes, TESTED, "seed", &mut read);
    assert_eq!(
        once, twice,
        "the same seed, the same search and the same lines"
    );
    let other = Breeding { seed: 1, ..TESTED };
    let another = search_by_forks(&genes, other, "seed", &mut read);
    assert_ne!(once.lines, another.lines, "another seed, another table");
    assert_eq!(once.lines.len(), another.lines.len());
    assert_eq!(
        once.lines[0],
        "DUMP search seed genes [\"x\", \"y\"] bounds [(0, 8), (0, 8)] start [4, 4] P 16 G 10 E 4 M 16 S 3 seed 0x0000000000000000"
    );
    for (line, one) in once.lines[1..].iter().zip(&once.scored) {
        assert_eq!(
            line,
            &format!(
                "DUMP search seed g {} i {} genes {:?} scores {:?} sum {}",
                one.generation,
                one.index,
                one.genes,
                one.scores,
                one.scores
                    .iter()
                    .fold(0i64, |sum, &s| sum.saturating_add(s))
            ),
            "every candidate scored is printed in the order scored"
        );
    }
}

#[test]
fn no_gene_leaves_its_bounds_and_the_count_scored_is_p_plus_g_less_one_times_p_less_e() {
    // Bounds narrower than a step, one on either side of zero and one a single value, so that
    // every step is held and the draws reach both edges.
    let genes = [
        Gene {
            name: "low",
            min: -5,
            max: 3,
            start: -5,
        },
        Gene {
            name: "high",
            min: 7,
            max: 12,
            start: 12,
        },
        Gene {
            name: "one",
            min: -2,
            max: -2,
            start: -2,
        },
    ];
    for breeding in [
        TESTED,
        Breeding {
            population: 5,
            generations: 3,
            kept: 0,
            step: 0,
            streams: 1,
            seed: 7,
        },
        Breeding {
            population: 2,
            generations: 4,
            kept: 1,
            step: 40,
            streams: 2,
            seed: 9,
        },
    ] {
        let mut calls: Vec<(Vec<i32>, u32)> = Vec::new();
        let mut read = |g: &[i32], s: u32| {
            for (value, gene) in g.iter().zip(&genes) {
                assert!(
                    gene.min <= *value && *value <= gene.max,
                    "{value} in {gene:?}"
                );
            }
            calls.push((g.to_vec(), s));
            i64::from(g[0]).saturating_add(i64::from(s))
        };
        let found = search_by_forks(&genes, breeding, "bounds", &mut read);
        let Breeding {
            population: p,
            generations: g,
            kept: e,
            streams: s,
            ..
        } = breeding;
        let last = g.saturating_sub(1);
        let children = p.saturating_sub(e);
        let count = p.saturating_add(last.saturating_mul(children));
        assert_eq!(found.scored.len(), count, "{breeding:?}");
        assert_eq!(
            found.lines.len(),
            count.saturating_add(2),
            "the head, each candidate and the best"
        );
        let s = s as usize;
        assert_eq!(
            calls.len(),
            count.saturating_add(1).saturating_mul(s),
            "the kept are not read again"
        );
        for (at, reads) in calls.chunks(s).enumerate() {
            for (k, (candidate, stream)) in reads.iter().enumerate() {
                if at < count {
                    assert_eq!(candidate, &found.scored[at].genes);
                    assert_eq!(*stream as usize, k, "a candidate reads streams 0 to S − 1");
                } else {
                    assert_eq!(candidate, &found.best.genes);
                    assert_eq!(
                        *stream as usize,
                        s.saturating_add(k),
                        "the best reads streams S to 2S − 1"
                    );
                }
            }
        }
        for one in &found.scored {
            assert!(one.index < p && one.generation < g);
        }
        assert_eq!(
            found.scored[0].genes,
            vec![-5, 12, -2],
            "generation 0 starts from the start genes"
        );
        for generation in 0..g {
            let indices: Vec<usize> = found
                .scored
                .iter()
                .filter(|c| c.generation == generation)
                .map(|c| c.index)
                .collect();
            let first = if generation == 0 { 0 } else { e };
            assert_eq!(
                indices,
                (first..p).collect::<Vec<_>>(),
                "generation {generation} scores its new places in order"
            );
        }
    }
}

#[test]
fn a_tie_ranks_by_the_index_so_the_kept_come_first_and_the_start_is_best() {
    let genes = small_box();
    let mut read = |_: &[i32], _: u32| 7i64;
    let found = search_by_forks(&genes, TESTED, "tie", &mut read);
    assert_eq!(
        found.best.genes,
        vec![4; 2],
        "the start, index 0 of generation 0"
    );
    assert_eq!((found.best.generation, found.best.index), (0, 0));
    assert_eq!(found.unseen, vec![7; 3]);
    // Each generation's children take the places after the kept, in the order made.
    for one in found.scored.iter().filter(|c| c.generation > 0) {
        assert!(one.index >= TESTED.kept);
    }
}

#[test]
fn the_best_is_the_first_scored_of_the_largest_sum_since_the_kept_keep_their_order() {
    // The score is one gene, so many candidates tie. The best is always kept, a tie ranks the
    // kept before the children and the kept in their rank, so the best of the last generation
    // is the candidate scored first among those with the largest sum of all.
    let genes = small_box();
    for (j, seed) in [(0usize, 0u64), (1, 5), (0, 11)] {
        let mut read = |g: &[i32], s: u32| i64::from(g[j]).saturating_add(noise(g, s));
        let breeding = Breeding { seed, ..TESTED };
        let found = search_by_forks(&genes, breeding, "kept", &mut read);
        let largest = found.scored.iter().map(Scored::sum).max().expect("scored");
        let first = found
            .scored
            .iter()
            .find(|c| c.sum() == largest)
            .expect("the largest");
        assert_eq!(&found.best, first, "gene {j}, seed {seed}");
        assert!(
            found.scored.iter().filter(|c| c.sum() == largest).count() > 1
                || found.best.generation > 0,
            "a tie or a child, so the order is read"
        );
    }
}

#[test]
fn a_weight_scales_by_256ths_rounded_half_away_from_zero_and_held_within_the_rail() {
    for (weight, factor, scaled) in [
        (0i16, 332, 0i16),
        (12_000, 256, 12_000),
        (1, 128, 1),
        (-1, 128, -1),
        (3, 128, 2),
        (-3, 128, -2),
        (383, 1, 1),
        (384, 1, 2),
        (-383, 1, -1),
        (-384, 1, -2),
        (12_001, 128, 6_001),
        (100, -256, -100),
        (7_000, 332, 9_078),
        (32_767, 332, 32_767),
        (-32_767, 332, -32_767),
        (i16::MIN, 256, -32_767),
        (i16::MIN, i32::MAX, -32_767),
    ] {
        assert_eq!(
            scaled_weight(weight, factor),
            scaled,
            "{weight} × {factor}/256"
        );
    }
}

/// A small engine for the writer: the instrument's network at 256 units, as synthesized, and
/// the geometry's four sets on it.
fn small_engine() -> (Engine, [Set; 4], Vec<u8>) {
    let exec = at_gain(&prior(256), config(256, 1, BASELINE_Q16), gain(256));
    let image = Image::encode(&exec).expect("quiescent");
    (exec, geometry(256, rotation(256)), image)
}

/// A fork of the small engine's image.
fn forked(image: &[u8]) -> Engine {
    served(Image::decode::<2048>(image, config(256, 1, BASELINE_Q16)).expect("an image"))
}

/// The blocks `exec` would hold had each synapse from an excitatory unit of `from` into `into`
/// been scaled by `factor`/256 by hand, a half away from zero, and no other: the arithmetic of
/// the rounding written out on a weight at or above zero, which every excitatory one is.
fn by_hand(exec: &Engine, from: Set, into: Set, factor: i32) -> (Vec<SynapseBlock>, usize) {
    let mut blocks = exec.blocks().to_vec();
    let mut count = 0usize;
    for unit in exec.units() {
        if unit.flags & FLAG_INHIBITORY != 0 || !from.contains(unit.id as u32) {
            continue;
        }
        for s in unit.fan_out(exec.blocks()) {
            if into.contains(s.target) {
                assert!(s.weight_q1_15 >= 0, "an excitatory weight");
                let exact = i64::from(s.weight_q1_15).saturating_mul(i64::from(factor));
                let rounded = exact.saturating_add(128).wrapping_div(256);
                blocks[s.block_idx as usize].weights_q1_15[usize::from(s.slot)] =
                    i16::try_from(rounded.min(32_767)).expect("on the rail at most");
                count = count.saturating_add(1);
            }
        }
    }
    (blocks, count)
}

#[test]
fn the_writer_leaves_the_image_at_256_and_writes_each_coupling_s_synapses_and_no_other() {
    let (mut exec, sets, image) = small_engine();
    let [a, b, r0, r1] = sets;
    let pairs = [(a, r0), (a, r1), (b, r0), (b, r1)];
    for (from, into) in pairs {
        assert!(scale_coupling(&mut exec, from, into, 256) > 0);
    }
    assert_eq!(
        Image::encode(&exec).expect("quiescent"),
        image,
        "at 256 the image byte for byte"
    );
    let image_couplings = couplings(&exec, &sets);
    for factor in [128, 332] {
        for (k, &(from, into)) in pairs.iter().enumerate() {
            let mut fork = forked(&image);
            let (wanted, count) = by_hand(&fork, from, into, factor);
            assert!(count > 0, "the coupling has synapses");
            assert_eq!(scale_coupling(&mut fork, from, into, factor), count);
            assert_eq!(
                fork.blocks(),
                &wanted[..],
                "each synapse as the rounding says"
            );
            let mut hand = forked(&image);
            hand.blocks_mut().copy_from_slice(&wanted);
            assert_eq!(
                Image::encode(&fork).expect("quiescent"),
                Image::encode(&hand).expect("quiescent"),
                "nothing else in the image moves"
            );
            let read = couplings(&fork, &sets);
            for (j, (&now, &was)) in read.iter().zip(&image_couplings).enumerate() {
                if j == k {
                    assert_ne!(now, was, "the coupling scaled by {factor}");
                } else {
                    assert_eq!(now, was, "coupling {j} untouched");
                }
            }
        }
    }
}

#[test]
fn the_four_couplings_scale_each_by_its_own_factor() {
    let (mut exec, sets, image) = small_engine();
    let counts = scale_couplings(&mut exec, &sets, [256; 4]);
    assert!(counts.iter().all(|&c| c > 0));
    assert_eq!(Image::encode(&exec).expect("quiescent"), image);
    let factors = [128, 332, 200, 300];
    let written = scale_couplings(&mut exec, &sets, factors);
    assert_eq!(written, counts, "every coupling's synapses, once");
    let mut fork = forked(&image);
    let [a, b, r0, r1] = sets;
    for (k, (from, into)) in [(a, r0), (a, r1), (b, r0), (b, r1)].into_iter().enumerate() {
        let (wanted, count) = by_hand(&fork, from, into, factors[k]);
        assert_eq!(count, counts[k]);
        fork.blocks_mut().copy_from_slice(&wanted);
    }
    assert_eq!(
        exec.blocks(),
        fork.blocks(),
        "the four, each by its own factor"
    );
}
