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

//! codex32 Reference Implementation
//!
//! This project is a reference implementation of BIP-XXX "codex32", a project
//! by Leon Olson Curr and Pearlwort Snead to produce checksummed and secret-shared
//! BIP32 master seeds.
//!
//! References:
//!   * BIP-XXX <https://github.com/apoelstra/bips/blob/2023-02--volvelles/bip-0000.mediawiki>
//!   * The codex32 website <https://www.secretcodex32.com>
//!   * BIP-0173 "bech32" <https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki>
//!   * BIP-0032 "BIP 32" <https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki>
//!

// This is the shittiest lint ever and has literally never been correct when
// it has fired, and somehow in rust-bitcoin managed NOT to fire in the one
// case where it might've been useful.
// https://github.com/rust-bitcoin/rust-bitcoin/pull/1701
#![allow(clippy::suspicious_arithmetic_impl)]
// This one is also stupid but usually tolerable, though here we have a series
// of length checks and they want *one* of them to call is_empty() instead of
// len(), just to break symmetry.
#![allow(clippy::len_zero)]

mod checksum;
mod field;

use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// Tried a codex32 string of an illegal length
    InvalidLength(usize),
    /// Tried to decode a character which was not part of the bech32 alphabet,
    /// or, if in the HRP, was not ASCII.
    InvalidChar(char),
    /// Tried to decode a character but its case did not match the expected case
    InvalidCase(Case, char),
    /// String had an invalid checksum
    InvalidChecksum {
        /// Checksum we used, "long" or "short"
        checksum: &'static str,
        /// The string with the bad checksum
        string: String,
    },
    /// Error related to a single bech32 character
    Field(field::Error),
}

impl From<field::Error> for Error {
    fn from(e: field::Error) -> Error {
        Error::Field(e)
    }
}

/// Lowercase or uppercase (as applied to the bech32 alphabet)
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum Case {
    /// qpzr...
    Lower,
    /// QPZR...
    Upper,
}

/// A codex32 string, containing a valid checksum
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Codex32String(String);

impl fmt::Display for Codex32String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Codex32String {
    /// Construct a codex32 string from a not-yet-checksummed string
    pub fn from_unchecksummed_string(mut s: String) -> Result<Self, Error> {
        // Determine what checksum to use and extend the string
        let (len, mut checksum) = if s.len() < 81 {
            (13, checksum::Engine::new_codex32_short())
        } else {
            (15, checksum::Engine::new_codex32_short())
        };
        s.reserve_exact(len);

        // Split out the HRP
        let (hrp, real_string) = match s.rsplit_once('1') {
            Some((s1, s2)) => (s1, s2),
            None => ("", &s[..]),
        };
        // Compute the checksum
        checksum.input_hrp(hrp)?;
        checksum.input_data_str(real_string)?;
        for ch in checksum.into_residue() {
            s.push(ch.to_char());
        }
        Ok(Codex32String(s))
    }

    /// Construct a codex32 string from an already-checksummed string
    pub fn from_string(s: String) -> Result<Self, Error> {
        let (name, mut checksum) = if s.len() > 0 && s.len() < 94 {
            ("short", checksum::Engine::new_codex32_short())
        } else if s.len() > 95 && s.len() < 125 {
            ("long", checksum::Engine::new_codex32_long())
        } else {
            return Err(Error::InvalidLength(s.len()))
        };

        // Split out the HRP
        let (hrp, real_string) = match s.rsplit_once('1') {
            Some((s1, s2)) => (s1, s2),
            None => ("", &s[..]),
        };
        checksum.input_hrp(hrp)?;
        checksum.input_data_str(real_string)?;
        if !checksum.is_valid() {
            return Err(Error::InvalidChecksum {
                checksum: name,
                string: s,
            });
        }
        // Looks good, return
        Ok(Codex32String(s))
    }
}


