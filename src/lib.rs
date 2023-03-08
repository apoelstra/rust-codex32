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

mod field;

/// Lowercase or uppercase (as applied to the bech32 alphabet)
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum Case {
    /// qpzr...
    Lower,
    /// QPZR...
    Upper,
}

