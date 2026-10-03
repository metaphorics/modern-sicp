// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.45, one module and one test.

mod ex_2_45 {
    use ch02::sec_2_2::{CaptureSink, Frame, Painter, below, beside, wave};
    use std::rc::Rc;

    /// Exercise 2.45: `split`, the general splitting operation.
    ///
    /// The two arguments are the painter combinators: how to join the
    /// painter with the smaller pair, and how to join the pair
    /// internally. The self-referential recursive procedure becomes a
    /// named private recursive driver that the returned closure calls,
    /// because a bare closure cannot refer to itself by name.
    fn split(
        combine: impl Fn(&Painter, &Painter) -> Painter,
        place_pair: impl Fn(&Painter, &Painter) -> Painter,
    ) -> impl Fn(&Painter, u32) -> Painter {
        fn drive<C, P>(combine: &C, place_pair: &P, painter: &Painter, n: u32) -> Painter
        where
            C: Fn(&Painter, &Painter) -> Painter,
            P: Fn(&Painter, &Painter) -> Painter,
        {
            if n == 0 {
                return Rc::clone(painter);
            }
            let smaller = drive(combine, place_pair, painter, n - 1);
            combine(painter, &place_pair(&smaller, &smaller))
        }
        move |painter, n| drive(&combine, &place_pair, painter, n)
    }

    /// Paints a painter into a unit-square frame and renders the
    /// segments it draws.
    fn render_segments(painter: &Painter) -> String {
        let mut sink = CaptureSink::new();
        painter(&Frame::unit_square(), &mut sink);
        sink.to_text()
    }

    /// Exercise 2.45: split, a general splitting combinator
    ///
    /// Returns rendered segment sets of `right_split` and `up_split`
    /// for the `wave` painter at depth 1, each built by the general
    /// `split` combinator instead of by hand. The test pins them
    /// against the hand-built section definitions.
    pub fn ex_2_45() -> (String, String) {
        let right_split_via_split = split(beside, below);
        let up_split_via_split = split(below, beside);
        let painter = wave();
        (
            render_segments(&right_split_via_split(&painter, 1)),
            render_segments(&up_split_via_split(&painter, 1)),
        )
    }
}

#[test]
fn ex_2_45() {
    use ch02::sec_2_2::{CaptureSink, Frame, right_split, up_split, wave};

    let painter = wave();
    let mut right = CaptureSink::new();
    right_split(&painter, 1)(&Frame::unit_square(), &mut right);
    let mut up = CaptureSink::new();
    up_split(&painter, 1)(&Frame::unit_square(), &mut up);
    assert_eq!(ex_2_45::ex_2_45(), (right.to_text(), up.to_text()));
}
