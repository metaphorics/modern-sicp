// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Shared runtime for the Rust edition: the seeded `random` every
//! probabilistic section runs on, and the edition's typed error.

mod random;

pub use random::Random;

/// The edition's typed error: every `error` in the book raises one of these
/// variants.
#[derive(Debug, thiserror::Error)]
pub enum SchemeError {
    /// `Random::new` rejects a zero seed because `xorshift64*` never leaves
    /// the zero state.
    #[error("random: the seed must be nonzero")]
    ZeroSeed,
}
