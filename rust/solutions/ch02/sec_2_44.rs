// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.44, one module and one test.

mod ex_2_44 {
    use ch02::sec_2_2::{CaptureSink, Frame, Painter, below, beside, right_split, wave};
    use std::rc::Rc;

    /// Exercise 2.44: `up-split`, the mirror of the book's
    /// `right-split`: at depth 0 the painter itself, and below that the
    /// painter beside an `up-split` placed beside itself. The painter
    /// is a boxed frame-to-frame function, so the combinator is just a
    /// closure that captures the two painters and returns a new one.
    fn up_split(painter: &Painter, n: u32) -> Painter {
        if n == 0 {
            return Rc::clone(painter);
        }
        let smaller = up_split(painter, n - 1);
        below(painter, &beside(&smaller, &smaller))
    }

    /// Paints a painter into a unit-square frame and counts the
    /// segments its drawing produces.
    fn segment_count(painter: &Painter) -> usize {
        let mut sink = CaptureSink::new();
        painter(&Frame::unit_square(), &mut sink);
        sink.lines().len()
    }

    /// Exercise 2.44: up-split
    ///
    /// Returns the segment counts that `right_split` and the newly
    /// defined `up_split` each paint for the `wave` painter at depth 1,
    /// and the `wave` painter's own count, in that order. Both
    /// one-level splits paint three waves, confirming the mirrored
    /// construction draws the same amount in a different arrangement.
    pub fn ex_2_44() -> (usize, usize, usize) {
        let painter = wave();
        (
            segment_count(&right_split(&painter, 1)),
            segment_count(&up_split(&painter, 1)),
            segment_count(&painter),
        )
    }
}

#[test]
fn ex_2_44() {
    assert_eq!(ex_2_44::ex_2_44(), (54, 54, 18));
}
