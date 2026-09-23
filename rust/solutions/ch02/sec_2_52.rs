// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.52, one module and one test.

mod ex_2_52 {
    use ch02::sec_2_2::{
        CaptureSink, Frame, Painter, Segment, Vect, below, beside, corner_split, flip_horiz,
        flip_vert, identity_op, painter_op, right_split, rotate_180, segments_painter,
        square_limit, square_of_four, up_split, wave, wave_segments,
    };
    use std::rc::Rc;

    /// Exercise 2.52a: a wave with a smile. Two short segments join the
    /// primitive `wave` segment list, so every derived pattern inherits
    /// the change from the lowest level of the design.
    pub fn wave_with_smile() -> Painter {
        let v = Vect::new;
        let mut segments = wave_segments();
        segments.push(Segment::new(v(0.40, 0.58), v(0.47, 0.53)));
        segments.push(Segment::new(v(0.47, 0.53), v(0.54, 0.58)));
        segments_painter(segments)
    }

    /// Exercise 2.52b: a corner split that branches with one copy of
    /// each split image. Where the book's `corner-split` places an
    /// `up-split` result beside the painter and a `right-split` result
    /// below it and then recurses on the corner, this arrangement puts
    /// the painter, one `up-split`, one `right-split`, and the
    /// recursive corner into the four quadrants directly.
    pub fn corner_split_variant(painter: &Painter, n: u32) -> Painter {
        if n == 0 {
            return Rc::clone(painter);
        }
        beside(
            &below(painter, &up_split(painter, n - 1)),
            &below(
                &right_split(painter, n - 1),
                &corner_split_variant(painter, n - 1),
            ),
        )
    }

    fn flip_vert_op() -> impl Fn(&Painter) -> Painter {
        |p: &Painter| flip_vert(p)
    }

    fn flip_horiz_op() -> impl Fn(&Painter) -> Painter {
        |p: &Painter| flip_horiz(p)
    }

    /// Exercise 2.52c: a square limit over the variant corner split,
    /// assembled in a different pattern from the book's arrangement.
    pub fn square_limit_variant(painter: &Painter, n: u32) -> Painter {
        (square_of_four(
            painter_op(rotate_180),
            painter_op(flip_vert_op()),
            identity_op(),
            painter_op(flip_horiz_op()),
        ))(&corner_split_variant(painter, n))
    }

    fn render_segments(painter: &Painter) -> String {
        let mut sink = CaptureSink::new();
        painter(&Frame::unit_square(), &mut sink);
        sink.to_text()
    }

    /// Exercise 2.52: square-limit variations
    ///
    /// Returns the segment count of the smiled wave, the segment count
    /// of the single-copy corner split at depth 1, and a verdict that
    /// the rearranged square limit paints a different segment set from
    /// the book's arrangement.
    pub fn ex_2_52() -> (usize, usize, bool) {
        let base = wave();
        let smiled = wave_with_smile();
        let smiled_count = {
            let mut sink = CaptureSink::new();
            smiled(&Frame::unit_square(), &mut sink);
            sink.lines().len()
        };

        let variant = corner_split_variant(&base, 1);
        let variant_count = {
            let mut sink = CaptureSink::new();
            variant(&Frame::unit_square(), &mut sink);
            sink.lines().len()
        };

        let book_limit = square_limit(&base, 2);
        let new_limit = square_limit_variant(&base, 2);
        let differs = render_segments(&book_limit) != render_segments(&new_limit);

        (smiled_count, variant_count, differs)
    }

    /// The checked-in 2.52 figure files are fresh, as are the
    /// book-listing figures of the square-limit family that the
    /// example generator writes.
    #[test]
    fn square_limit_figures_are_fresh() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../book/figures/generated/chap2");
        let base = wave();
        let smiled = wave_with_smile();
        let book_corner = corner_split(&base, 4);
        let book_limit = square_limit(&base, 4);
        let variant_corner = corner_split_variant(&base, 4);
        let variant_limit = square_limit_variant(&base, 4);
        for (name, painter) in [
            ("wave_smile", &smiled),
            ("corner_split", &book_corner),
            ("corner_split_variant", &variant_corner),
            ("square_limit", &book_limit),
            ("square_limit_variant", &variant_limit),
        ] {
            let checked_in = std::fs::read_to_string(dir.join(format!("{name}.std.svg")))
                .unwrap_or_else(|_| {
                    panic!(
                        "checked-in figure {name}.std.svg is missing; run the sec_2_2_4_picture_language example"
                    )
                });
            assert_eq!(
                checked_in,
                ch02::sec_2_2::render_svg(painter, 200),
                "figure {name} drifted"
            );
        }
    }
}

#[test]
fn ex_2_52() {
    assert_eq!(ex_2_52::ex_2_52(), (20, 72, true));
}
