//! The tables H-47's round pinned, a row a line (ADR-0227): the twin's from explorations 018's
//! and 019's logs at the protocol's commit, and the rest from its 24 jobs.

use crate::*;

/// H-47's verdict, as read from its 24 jobs.
pub(crate) const NIGHTS_1024: Option<NightsVerdict> = None;

/// The step of ADR-0234's stopping rule the verdict reaches.
pub(crate) const NIGHTS_STOP_1024: Option<u8> = None;

/// ADR-0234's five predicted readings, as read.
pub(crate) const NIGHTS_READINGS_READ_1024: Option<[bool; 5]> = None;

/// The first mapping's image's CRC-64, `[network][arm]`, as its jobs wrote it.
pub(crate) const NIGHTS_FIRST_CRC: [[Option<u64>; 2]; 3] = [[None; 2]; 3];

/// Each form's rows, `[network][start][form]`, path by path and checkpoint by checkpoint.
pub(crate) const NIGHTS_ROWS: [[[&[NightsRow]; 4]; 4]; 3] = [[[&[]; 4]; 4]; 3];

/// The twin of H-47's step 2 (ADR-0234): each group of its lines, their count and the FNV-1a of
/// their text sorted and joined by line feeds, label aside, from exploration 018's run
/// 41367828454 (job `arm-0`) and exploration 019's run 41408442255 (job `arm-0`), a reference and
/// not evidence (ADR-0193).
pub(crate) const NIGHTS_TWIN: [(&str, u32, u64); 17] = [
    ("groups", 1, 0x8c43_c15e_406f_3d42),
    ("head", 1, 0xee3d_5f63_89d7_914d),
    ("no-replay blocks 6", 2, 0xb586_f70e_7084_4ad5),
    ("no-replay blocks 6 first", 16, 0xab02_8d2f_ee5a_273e),
    ("no-replay blocks 6 second", 16, 0xff16_acaa_a370_a081),
    ("no-replay gap 6", 33, 0xafc7_47f3_79ed_25a5),
    ("pins", 22, 0xe378_37bb_6239_1647),
    ("plain-first blocks 12", 2, 0x8fa4_3efb_ea59_3cda),
    ("plain-first blocks 12 first", 16, 0xc39d_96a7_a626_c475),
    ("plain-first blocks 12 run", 1, 0xe3b5_2a15_12b1_18b2),
    ("plain-first blocks 12 second", 16, 0x57c1_eed6_2ebd_0054),
    ("plain-first blocks 6", 2, 0x1a91_c920_4c02_b014),
    ("plain-first blocks 6 first", 16, 0xdbf1_5eac_456e_4de1),
    ("plain-first blocks 6 second", 16, 0xf212_3f63_99c0_4b78),
    ("plain-first gap 12", 33, 0x7b17_0f1e_331a_ff12),
    ("plain-first gap 6", 33, 0x3cfe_1fce_b87f_4406),
    ("the groups", 1, 0xd0ab_dbc5_35d8_a2ee),
];
