// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.46, one module and one test.

mod ex_2_46 {
    use ch02::sec_2_2::Vect;

    /// Exercise 2.46: the vector data abstraction. `make_vect` builds,
    /// `xcor_vect` and `ycor_vect` select, and the three operations are
    /// defined purely on those, so the representation could change
    /// under them without touching any caller. Coordinates are `f64`
    /// because vectors live in the unit square of the picture language.
    fn make_vect(x: f64, y: f64) -> Vect {
        Vect::new(x, y)
    }

    /// The book's `xcor-vect`.
    fn xcor_vect(v: &Vect) -> f64 {
        v.x
    }

    /// The book's `ycor-vect`.
    fn ycor_vect(v: &Vect) -> f64 {
        v.y
    }

    /// The book's `add-vect`.
    fn add_vect(v: &Vect, w: &Vect) -> Vect {
        make_vect(xcor_vect(v) + xcor_vect(w), ycor_vect(v) + ycor_vect(w))
    }

    /// The book's `sub-vect`.
    fn sub_vect(v: &Vect, w: &Vect) -> Vect {
        make_vect(xcor_vect(v) - xcor_vect(w), ycor_vect(v) - ycor_vect(w))
    }

    /// The book's `scale-vect`.
    fn scale_vect(s: f64, v: &Vect) -> Vect {
        make_vect(s * xcor_vect(v), s * ycor_vect(v))
    }

    /// Exercise 2.46: vector abstraction
    ///
    /// Returns the rendered `add_vect`, `sub_vect`, and `scale_vect`
    /// results for `(1, 2)` and `(3, 4)` with scalar 3, in that order.
    pub fn ex_2_46() -> (String, String, String) {
        let v = make_vect(1.0, 2.0);
        let w = make_vect(3.0, 4.0);
        (
            add_vect(&v, &w).to_string(),
            sub_vect(&v, &w).to_string(),
            scale_vect(3.0, &make_vect(2.0, 5.0)).to_string(),
        )
    }
}

#[test]
fn ex_2_46() {
    assert_eq!(
        ex_2_46::ex_2_46(),
        (
            "(4, 6)".to_string(),
            "(-2, -2)".to_string(),
            "(6, 15)".to_string()
        )
    );
}
