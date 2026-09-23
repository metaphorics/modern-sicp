// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.49, one module and one test.

mod ex_2_49 {
    use ch02::sec_2_2::{
        CaptureSink, Frame, Painter, Segment, Vect, segments_painter, wave_segments,
    };
    use std::path::Path;

    /// Builds a segment from raw coordinates, the shorthand the
    /// primitive painters below are written with.
    fn seg(start: (f64, f64), end: (f64, f64)) -> Segment {
        Segment::new(Vect::new(start.0, start.1), Vect::new(end.0, end.1))
    }

    /// Exercise 2.49a: the outline of the designated frame: the four
    /// sides of the unit square, given in frame coordinates so the
    /// frame coordinate map places them.
    pub fn outline_painter() -> Painter {
        segments_painter(vec![
            seg((0.0, 0.0), (1.0, 0.0)),
            seg((1.0, 0.0), (1.0, 1.0)),
            seg((1.0, 1.0), (0.0, 1.0)),
            seg((0.0, 1.0), (0.0, 0.0)),
        ])
    }

    /// Exercise 2.49b: an X connecting opposite corners.
    pub fn x_painter() -> Painter {
        segments_painter(vec![
            seg((0.0, 0.0), (1.0, 1.0)),
            seg((0.0, 1.0), (1.0, 0.0)),
        ])
    }

    /// Exercise 2.49c: a diamond connecting the midpoints of the sides.
    pub fn diamond_painter() -> Painter {
        segments_painter(vec![
            seg((0.5, 0.0), (1.0, 0.5)),
            seg((1.0, 0.5), (0.5, 1.0)),
            seg((0.5, 1.0), (0.0, 0.5)),
            seg((0.0, 0.5), (0.5, 0.0)),
        ])
    }

    /// Exercise 2.49d: the wave painter, built through
    /// `segments_painter` the way the exercise asks: the hand-drawn
    /// segment list becomes data the frame map transforms.
    pub fn wave_painter() -> Painter {
        segments_painter(wave_segments())
    }

    /// Counts the segments a painter draws in a unit-square frame.
    fn segment_count(painter: &Painter) -> usize {
        let mut sink = CaptureSink::new();
        painter(&Frame::unit_square(), &mut sink);
        sink.lines().len()
    }

    /// The directory holding the checked-in figure files, resolved from
    /// the crate so the test runs from any working directory.
    fn figures_dir() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../book/figures/generated/chap2")
    }

    /// The checked-in SVG for a figure must be byte-identical to what
    /// the painter renders now; otherwise the figure generator and the
    /// painters have drifted apart.
    #[track_caller]
    fn assert_figure_matches(name: &str, painter: &Painter) {
        let path = figures_dir().join(format!("{name}.std.svg"));
        let checked_in = std::fs::read_to_string(&path).unwrap_or_else(|_| {
            panic!("checked-in figure {name}.std.svg is missing; run the sec_2_2_4_picture_language example")
        });
        assert_eq!(
            checked_in,
            ch02::sec_2_2::render_svg(painter, 200),
            "figure {name} drifted from its painter"
        );
    }

    /// Exercise 2.49: primitive painters
    ///
    /// Returns the number of segments each primitive painter draws in a
    /// frame: the outline, the X, the diamond, and the wave, in that
    /// order.
    pub fn ex_2_49() -> [usize; 4] {
        [
            segment_count(&outline_painter()),
            segment_count(&x_painter()),
            segment_count(&diamond_painter()),
            segment_count(&wave_painter()),
        ]
    }

    /// The four primitives' SVGs match the checked-in figure files.
    #[test]
    fn primitive_figures_are_fresh() {
        assert_figure_matches("outline", &outline_painter());
        assert_figure_matches("cross", &x_painter());
        assert_figure_matches("diamond", &diamond_painter());
        assert_figure_matches("wave", &wave_painter());
    }
}

#[test]
fn ex_2_49() {
    assert_eq!(ex_2_49::ex_2_49(), [4, 2, 4, 18]);
}
