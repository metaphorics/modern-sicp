// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.48, one module and one test.

mod ex_2_48 {
    use ch02::sec_2_2::{Segment, Vect};

    /// Exercise 2.48: the segment data abstraction, on the section's
    /// `Segment` of two `Vect`s: the constructor packs the start and
    /// end vectors, the selectors take them back out, and nothing else
    /// in the picture language needs to know more.
    fn make_segment(start: Vect, end: Vect) -> Segment {
        Segment::new(start, end)
    }

    /// The book's `start-segment`.
    fn start_segment(segment: &Segment) -> Vect {
        segment.start
    }

    /// The book's `end-segment`.
    fn end_segment(segment: &Segment) -> Vect {
        segment.end
    }

    /// Exercise 2.48: directed line segments
    ///
    /// Returns the rendered start and end vectors selected back out of
    /// the segment from `(1, 2)` to `(3, 4)`.
    pub fn ex_2_48() -> (String, String) {
        let segment = make_segment(Vect::new(1.0, 2.0), Vect::new(3.0, 4.0));
        (
            start_segment(&segment).to_string(),
            end_segment(&segment).to_string(),
        )
    }
}

#[test]
fn ex_2_48() {
    assert_eq!(
        ex_2_48::ex_2_48(),
        ("(1, 2)".to_string(), "(3, 4)".to_string())
    );
}
