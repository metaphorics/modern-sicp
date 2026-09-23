// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.50, one module and one test.

mod ex_2_50 {
    use ch02::sec_2_2::{
        CaptureSink, Frame, Painter, Segment, Vect, segments_painter, transform_painter,
    };

    /// Exercise 2.50: `flip-horiz`, built directly on
    /// `transform_painter`: the new frame runs the unit square's
    /// horizontal axis backwards.
    pub fn flip_horiz_direct(painter: &Painter) -> Painter {
        transform_painter(
            painter,
            Vect::new(1.0, 0.0),
            Vect::new(0.0, 0.0),
            Vect::new(1.0, 1.0),
        )
    }

    /// Exercise 2.50: counterclockwise rotation by 180 degrees, built
    /// directly: both axes run backwards.
    pub fn rotate_180_direct(painter: &Painter) -> Painter {
        transform_painter(
            painter,
            Vect::new(1.0, 1.0),
            Vect::new(0.0, 1.0),
            Vect::new(1.0, 0.0),
        )
    }

    /// Exercise 2.50: counterclockwise rotation by 270 degrees, built
    /// directly: the frame origin moves to the top left corner and the
    /// edges trade roles.
    pub fn rotate_270_direct(painter: &Painter) -> Painter {
        transform_painter(
            painter,
            Vect::new(0.0, 1.0),
            Vect::new(0.0, 0.0),
            Vect::new(1.0, 1.0),
        )
    }

    /// A one-segment probe painter, deliberately asymmetric so each
    /// transform moves it somewhere different. The coordinates are
    /// dyadic (halves, quarters, eighths), so the flipped and rotated
    /// values print exactly.
    fn probe_painter() -> Painter {
        let probe = Segment::new(Vect::new(0.25, 0.5), Vect::new(0.75, 0.875));
        segments_painter(vec![probe])
    }

    /// Paints a painter into a unit-square frame and renders the
    /// segments it draws.
    fn render_segments(painter: &Painter) -> String {
        let mut sink = CaptureSink::new();
        painter(&Frame::unit_square(), &mut sink);
        sink.to_text()
    }

    /// Exercise 2.50: flip-horiz and the rotations
    ///
    /// Returns the rendered segment set of the probe segment
    /// `(0.25, 0.5)` to `(0.75, 0.875)` under `flip_horiz`,
    /// `rotate_180`, and `rotate_270`, each built directly with
    /// `transform_painter`.
    pub fn ex_2_50() -> (String, String, String) {
        let probe = probe_painter();
        (
            render_segments(&flip_horiz_direct(&probe)),
            render_segments(&rotate_180_direct(&probe)),
            render_segments(&rotate_270_direct(&probe)),
        )
    }

    /// The direct 180 construction agrees with the composed one the
    /// book's footnote offers (`compose flip-vert flip-horiz`, the
    /// section module's `rotate_180`).
    #[test]
    fn direct_180_agrees_with_composed() {
        use ch02::sec_2_2::rotate_180;
        let probe = probe_painter();
        assert_eq!(
            render_segments(&rotate_180_direct(&probe)),
            render_segments(&rotate_180(&probe))
        );
    }
}

#[test]
fn ex_2_50() {
    assert_eq!(
        ex_2_50::ex_2_50(),
        (
            "(0.75, 0.5) -> (0.25, 0.875)".to_string(),
            "(0.75, 0.5) -> (0.25, 0.125)".to_string(),
            "(0.5, 0.75) -> (0.875, 0.25)".to_string()
        )
    );
}
