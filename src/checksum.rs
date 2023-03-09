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

//! Checksums
//!
//! Validates specific checksums
//!

use super::{Case, Error};
use crate::field::Fe;

/// An engine which consumes one GF32 character at a time, and produces
/// a residue modulo some generator
pub struct Engine {
    case: Option<Case>,
    generator: Vec<Fe>,
    residue: Vec<Fe>,
    target: Vec<Fe>,
}

impl Engine {
    // An engine which computes the normal codex32 checksum
    pub fn new_codex32_short() -> Engine {
        Engine {
            case: None,
            #[rustfmt::skip]
            generator: vec![
                Fe::S, Fe::S, Fe::C, Fe::M, Fe::L, Fe::E,
                Fe::E, Fe::E, Fe::Q, Fe::G, Fe::_3, Fe::M,
            ],
            #[rustfmt::skip]
            residue: vec![
                Fe::Q, Fe::Q, Fe::Q, Fe::Q, Fe::Q, Fe::Q,
                Fe::Q, Fe::Q, Fe::Q, Fe::Q, Fe::Q, Fe::Q,
                Fe::P,
            ],
            #[rustfmt::skip]
            target: vec![
                Fe::S, Fe::E, Fe::C, Fe::R, Fe::E, Fe::T,
                Fe::S, Fe::H, Fe::A, Fe::R, Fe::E, Fe::_3,
                Fe::_2,
            ],
        }
    }

    // An engine which computes the "long" codex32 checksum
    pub fn new_codex32_long() -> Engine {
        // hyk9x4hx4ef6e20p
        Engine {
            case: None,
            #[rustfmt::skip]
            generator: vec![
                Fe::H, Fe::Y, Fe::K, Fe::_9, Fe::X, Fe::_4,
                Fe::H, Fe::X, Fe::_4, Fe::E, Fe::F, Fe::_6,
                Fe::_2, Fe::_0,
            ],
            #[rustfmt::skip]
            residue: vec![
                Fe::Q, Fe::Q, Fe::Q, Fe::Q, Fe::Q, Fe::Q,
                Fe::Q, Fe::Q, Fe::Q, Fe::Q, Fe::Q, Fe::Q,
                Fe::Q, Fe::Q, Fe::P,
            ],
            #[rustfmt::skip]
            target: vec![
                Fe::S, Fe::E, Fe::C, Fe::R, Fe::E, Fe::T,
                Fe::S, Fe::H, Fe::A, Fe::R, Fe::E, Fe::_3,
                Fe::_2, Fe::E, Fe::X,
            ],
        }
    }

    /// Extracts the residue from a checksum engine
    pub fn into_residue(self) -> Vec<Fe> {
        self.residue
    }

    /// Determines whether the residue matches the target value
    /// for the checksum
    ///
    /// If you need the actual residue, e.g. for error correction,
    /// call the `into_residue` function (which will consume the
    /// engine).
    pub fn is_valid(&self) -> bool {
        self.residue == self.target
    }

    /// Initializes the checksum engine by loading an HRP into it
    pub fn input_hrp(&mut self, hrp: &str) -> Result<(), Error> {
        for ch in hrp.chars() {
            self.set_check_case(ch)?;
            self.input_fe(Fe::from_int(u32::from(ch) >> 5)?);
            self.input_fe(Fe::Q);
            self.input_fe(Fe::from_int(u32::from(ch) & 0x1f)?);
        }
        Ok(())
    }

    /// Adds a single character to the checksum engine
    pub fn input_char(&mut self, c: char) -> Result<(), Error> {
        self.set_check_case(c)?;
        self.input_fe(Fe::from_char(c)?);
        Ok(())
    }

    /// Adds an entire string to the engine, counting each character as a data character
    /// (not an HRP).
    pub fn input_data_str(&mut self, s: &str) -> Result<(), Error> {
        for ch in s.chars() {
            self.input_char(ch)?;
        }
        Ok(())
    }

    /// Helper function to check that the whole input has consistent case
    fn set_check_case(&mut self, c: char) -> Result<(), Error> {
        if !c.is_ascii() {
            Err(Error::InvalidChar(c))
        } else {
            let is_lower = c.is_ascii_lowercase();
            match (self.case, is_lower) {
                (Some(Case::Lower), true) | (Some(Case::Upper), false) => Ok(()),
                (Some(case @ Case::Lower), false) | (Some(case @ Case::Upper), true) => {
                    Err(Error::InvalidCase(case, c))
                }
                (None, true) => {
                    self.case = Some(Case::Lower);
                    Ok(())
                }
                (None, false) => {
                    self.case = Some(Case::Upper);
                    Ok(())
                }
            }
        }
    }

    /// Adds a single field element to the checksum engine
    ///
    /// This is where the real magic happens.
    fn input_fe(&mut self, e: Fe) {
        let res_len = self.residue.len(); // needed for borrowck
        // Store current coefficient of x^{n-1}, which will become
        // x^n (and get reduced)
        let xn = self.residue[res_len - 1];
        // Simply shift x^0 through x^{n-1} up one, and set x^0 to the new input
        for i in 0..res_len - 1 {
            self.residue[i + 1] = self.residue[i];
        }
        self.residue[0] = e;
        // Then reduce x^n mod the generator.
        for (i, ch) in self.generator.iter().enumerate() {
            self.residue[i] += *ch * xn;
        }
    }
}
