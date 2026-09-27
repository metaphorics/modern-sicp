// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.3: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_03 {
    /// A rectangle as two opposite corners: exercise 2.3's first
    /// representation.
    struct RectangleCorners {
        bottom_left: (f64, f64),
        top_right: (f64, f64),
    }

    impl RectangleCorners {
        fn width(&self) -> f64 {
            self.top_right.0 - self.bottom_left.0
        }

        fn height(&self) -> f64 {
            self.top_right.1 - self.bottom_left.1
        }
    }

    /// A rectangle as one corner plus its dimensions: exercise 2.3's
    /// second representation.
    struct RectangleDims {
        width: f64,
        height: f64,
    }

    /// The perimeter of a rectangle, in terms of `width` and `height`
    /// alone: the one procedure both representations share, above the
    /// abstraction barrier.
    fn perimeter(width: f64, height: f64) -> f64 {
        2.0 * (width + height)
    }

    /// The area of a rectangle, in terms of `width` and `height` alone.
    fn area(width: f64, height: f64) -> f64 {
        width * height
    }

    /// Exercise 2.3: two rectangle representations
    ///
    /// Returns the perimeter and area of a 4-by-3 rectangle computed
    /// through a corner-pair representation, then the same two numbers
    /// computed through a corner-plus-dimensions representation of the
    /// same rectangle, so the four numbers agree pairwise.
    pub fn ex_2_03() -> (f64, f64, f64, f64) {
        let by_corners = RectangleCorners {
            bottom_left: (0.0, 0.0),
            top_right: (4.0, 3.0),
        };
        let by_dims = RectangleDims {
            width: 4.0,
            height: 3.0,
        };
        (
            perimeter(by_corners.width(), by_corners.height()),
            area(by_corners.width(), by_corners.height()),
            perimeter(by_dims.width, by_dims.height),
            area(by_dims.width, by_dims.height),
        )
    }
}

#[test]
fn ex_2_03() {
    assert_eq!(ex_2_03::ex_2_03(), (14.0, 12.0, 14.0, 12.0));
}
