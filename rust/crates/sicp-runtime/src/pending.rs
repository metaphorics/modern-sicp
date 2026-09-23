// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The typed pending marker of the scaffold contract (D28): every
//! unsolved exercise scaffold returns this instead of panicking, so the
//! failure names its origin and `just scaffold` surfaces pending work.

use std::fmt::{self, Display, Formatter};

/// The typed pending report of an unsolved scaffold: the body returns this
/// instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `1.1`.
    pub exercise: &'static str,
}

impl Pending {
    /// Builds the marker for `exercise`.
    #[must_use]
    pub fn new(exercise: &'static str) -> Self {
        Self { exercise }
    }
}

impl Display for Pending {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "exercise {} is pending", self.exercise)
    }
}

impl std::error::Error for Pending {}

#[cfg(test)]
mod tests {
    use super::Pending;

    #[test]
    fn marker_names_its_exercise() {
        let p = Pending::new("3.47");
        assert_eq!(p.exercise, "3.47");
        assert_eq!(p.to_string(), "exercise 3.47 is pending");
    }
}
