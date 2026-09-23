// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.2: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_02 {
    /// A point in the plane: exercise 2.2's `make-point`, `x-point`, and
    /// `y-point`.
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn new(x: f64, y: f64) -> Self {
            Point { x, y }
        }
    }

    /// A line segment between two points: exercise 2.2's `make-segment`,
    /// `start-segment`, and `end-segment`.
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Segment {
        start: Point,
        end: Point,
    }

    impl Segment {
        fn new(start: Point, end: Point) -> Self {
            Segment { start, end }
        }

        /// The point whose coordinates are the average of the segment's
        /// two endpoints: exercise 2.2's `midpoint-segment`.
        fn midpoint(&self) -> Point {
            Point::new(
                f64::midpoint(self.start.x, self.end.x),
                f64::midpoint(self.start.y, self.end.y),
            )
        }
    }

    /// Renders a point as `(x,y)`: exercise 2.2's `print-point`.
    fn print_point(p: Point) -> String {
        format!("({},{})", p.x, p.y)
    }

    /// Exercise 2.2: line segments built from points
    ///
    /// Returns the midpoint's `x` and `y` coordinates of the segment
    /// from `(2, 3)` to `(6, 9)`, and its `print-point`-style rendering,
    /// in that order.
    pub fn ex_2_02() -> (f64, f64, String) {
        let segment = Segment::new(Point::new(2.0, 3.0), Point::new(6.0, 9.0));
        let mid = segment.midpoint();
        (mid.x, mid.y, print_point(mid))
    }
}

#[test]
fn ex_2_02() {
    assert_eq!(ex_2_02::ex_2_02(), (4.0, 6.0, "(4,6)".to_string()));
}
