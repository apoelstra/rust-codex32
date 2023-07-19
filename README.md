# rust-codex32

[![Build Status]][actions] [![Latest Version]][crates.io] [![Rustc Version 1.48+]][rustc]

[Build Status]: https://img.shields.io/github/actions/workflow/status/apoelstra/rust-codex32/ci.yml?branch=master
[actions]: https://github.com/apoelstra/rust-codex32/actions?query=branch%3Amaster
[Latest Version]: https://img.shields.io/crates/v/codex32.svg
[crates.io]: https://crates.io/crates/codex32
[Rustc Version 1.48+]: https://img.shields.io/badge/rustc-1.48+-lightgray.svg
[rustc]: https://blog.rust-lang.org/2020/11/19/Rust-1.48.html


Provides functionality for [codex32/BIP93](https://github.com/bitcoin/bips/blob/master/bip-0093.mediawiki)
master seeds, for the Rust programming language.

**codex32** is a scheme for managing BIP32 master seeds (commonly derived from
12 or 24 "seed words") without the use of electronic computers. It relies on
by-hand computation using paper computers "volvelles", worksheets, and patience.
More information can be found at the [codex32 website](https://www.secretcodex32.com), linked above.

This library serves as a reference implementation of codex32, and should also
be usable by wallet projects who wish to support the import of codex32 seeds.
It supports, or will support:

* Converting 16-to-64-byte seeds to BIP-93-compliant seed string (encoded as a "share" with index `S`), and back.
* Splitting seeds into a set of shares for distribution.
* Recovering seeds from sufficiently many shares.
* Generating and verifying BIP93 checksums for share data.
* Detecting and correcting errors in BIP93 strings.

## Contributing

Contributions are welcome, though as of July 2023, the library is slated to be largely
rewritten as a wrapper around [rust-bech32](https://github.com/rust-bitcoin/rust-bech32),
as soon as that library's API is overhauled to support the use of arbitrary BCH checksums.

The current state of this library is pretty rough and it may not be worthwhile to improve
it until that rewrite has arrived.

