// Rust Codex32 Library and Reference Implementation
// Written in 2023 by
//   Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Correction Tables
//!
//! This is a simple utility that prints out a sorted list of incorrect residues
//! for low numbers of errors.
//!

use codex32::Fe;
use std::collections::BTreeMap;
use std::fmt;

const HRP: &str = "ms";
const SHARE_LEN: usize = 48;

/// An error in a share (not an error in this library!)
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Error {
    position: usize,
    diff: Fe,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+{} @ {:2}", self.diff.to_char().to_ascii_uppercase(), self.position)
    }
}

fn engine_to_residue(engine: codex32::ChecksumEngine) -> String {
    let mut residue = engine.into_residue();
    residue[0] += Fe::S;
    residue[1] += Fe::E;
    residue[2] += Fe::C;
    residue[3] += Fe::R;
    residue[4] += Fe::E;
    residue[5] += Fe::T;
    residue[6] += Fe::S;
    residue[7] += Fe::H;
    residue[8] += Fe::A;
    residue[9] += Fe::R;
    residue[10] += Fe::E;
    residue[11] += Fe::_3;
    residue[12] += Fe::_2;

    residue
        .into_iter()
        .map(Fe::to_char)
        .map(|c| c.to_ascii_uppercase())
        .collect()
}


fn main() {
    assert!(SHARE_LEN <= 93); // for now don't bother supporting long strings
    assert!(SHARE_LEN > HRP.len()); // for now don't bother supporting long strings

    let offset = HRP.len() + 1;

    // Add a 0 mask over the bits that would represent the HRP
    let mut engine = codex32::ChecksumEngine::new_codex32_short();
    engine.force_residue_to_zero();

    let mut residues = BTreeMap::new();
    // Singles
    for i in 0..SHARE_LEN - offset - 13 {
        for err in Fe::iter_alpha() {
            if err == Fe::Q {
                continue;
            }

            let mut engine = engine.clone();
            for scan in 0..SHARE_LEN - offset {
                if scan == i {
                    engine.input_fe(err);
                } else {
                    engine.input_fe(Fe::Q);
                }
            }
            residues.insert(
                engine_to_residue(engine),
                vec![
                    Error {
                        position: i,
                        diff: err,
                    },
                ]
            );
        }
    }
    // Doubles
    for i in 0..SHARE_LEN - offset {
        for j in i + 1..SHARE_LEN - offset {
            for err1 in Fe::iter_alpha() {
                if err1 == Fe::Q {
                    continue;
                }
                for err2 in Fe::iter_alpha() {
                    if err2 == Fe::Q {
                        continue;
                    }
                    let mut engine = engine.clone();
                    for scan in 0..SHARE_LEN - offset {
                        if scan == i {
                            engine.input_fe(err1);
                        } else if scan == j {
                            engine.input_fe(err2);
                        } else {
                            engine.input_fe(Fe::Q);
                        }
                    }
                    residues.insert(
                        engine_to_residue(engine),
                        vec![
                            Error {
                                position: i,
                                diff: err1,
                            },
                            Error {
                                position: j,
                                diff: err2,
                            },
                        ]
                    );
                }
            }
        }
    }

    for (res, errs) in &residues {
        print!("{res}: ");
        print!("{}", errs[0]);
        for more in &errs[1..] {
            print!(", {}", more);
        }
        println!();
    }
    println!("Total: {} possibilities", residues.len());
}

