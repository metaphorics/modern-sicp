// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

use crate::SchemeError;

/// The seeded `xorshift64*` generator (Vigna 2016) behind the book's
/// `random`; a fixed seed keeps every probabilistic interaction
/// deterministic and identical across the four editions.
pub struct Random {
    state: u64,
}

impl Random {
    /// Creates a generator from `seed`.
    ///
    /// # Errors
    /// Returns [`SchemeError::ZeroSeed`] when `seed` is zero, because
    /// `xorshift64*` maps zero to zero forever.
    pub fn new(seed: u64) -> Result<Self, SchemeError> {
        if seed == 0 {
            return Err(SchemeError::ZeroSeed);
        }
        Ok(Self { state: seed })
    }

    /// Returns the next 64-bit word of the stream.
    #[must_use]
    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Returns the book's `(random n)`: the next word reduced into `0..n`.
    ///
    /// # Panics
    /// Panics when `n` is zero, as `(random 0)` is an error in the book's
    /// Scheme.
    #[must_use]
    pub fn random(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

#[cfg(test)]
mod tests {
    use super::Random;
    use crate::SchemeError;

    #[test]
    fn seed_one_yields_the_edition_vector() {
        let mut rng = Random::new(1).expect("seed 1 is valid");
        let drawn: Vec<u64> = (0..5).map(|_| rng.random(1000)).collect();
        assert_eq!(drawn, [165, 517, 103, 413, 928]);
    }

    #[test]
    fn zero_seed_is_rejected() {
        assert!(matches!(Random::new(0), Err(SchemeError::ZeroSeed)));
    }
}
