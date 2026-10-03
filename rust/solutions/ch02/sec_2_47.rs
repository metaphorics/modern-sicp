// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.47, one module and one test.

mod ex_2_47 {
    use ch02::sec_2_1::{Pair, car, cdr, cons};
    use ch02::sec_2_2::Vect;

    /// A frame whose three vectors sit in a fixed-size array: the
    /// edition's rendering of the book's list representation.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct FrameAsList([Vect; 3]);

    /// A frame built from nested pairs exactly as the book's second
    /// constructor writes it, the origin paired with the pair of edges, on
    /// the section 2.1 pair.
    #[derive(Debug, Clone)]
    pub struct FrameAsPair(Pair<Vect, Pair<Vect, Vect>>);

    /// The book's first constructor: the three-item list.
    pub fn make_frame_list(origin: Vect, edge1: Vect, edge2: Vect) -> FrameAsList {
        FrameAsList([origin, edge1, edge2])
    }

    /// The book's second constructor: the origin paired with the pair of edges.
    pub fn make_frame_pair(origin: Vect, edge1: Vect, edge2: Vect) -> FrameAsPair {
        FrameAsPair(cons(origin, cons(edge1, edge2)))
    }

    /// `origin-frame` for the list representation.
    pub fn origin_frame_list(frame: &FrameAsList) -> Vect {
        frame.0[0]
    }

    /// `edge1-frame` for the list representation.
    pub fn edge1_frame_list(frame: &FrameAsList) -> Vect {
        frame.0[1]
    }

    /// `edge2-frame` for the list representation.
    pub fn edge2_frame_list(frame: &FrameAsList) -> Vect {
        frame.0[2]
    }

    /// `origin-frame` for the pair representation.
    pub fn origin_frame_pair(frame: &FrameAsPair) -> Vect {
        car(&frame.0)
    }

    /// `edge1-frame` for the pair representation.
    pub fn edge1_frame_pair(frame: &FrameAsPair) -> Vect {
        car(&cdr(&frame.0))
    }

    /// `edge2-frame` for the pair representation.
    pub fn edge2_frame_pair(frame: &FrameAsPair) -> Vect {
        cdr(&cdr(&frame.0))
    }

    /// Renders one frame's three vectors on a line.
    fn render(origin: Vect, edge1: Vect, edge2: Vect) -> String {
        format!("{origin} {edge1} {edge2}")
    }

    /// Exercise 2.47: frame constructors
    ///
    /// Returns the rendered origin and edges extracted from a
    /// list-shaped frame constructor and from a cons-shaped one, both
    /// built for the unit square, in that order. The selectors differ
    /// per representation; everything that uses them does not.
    pub fn ex_2_47() -> (String, String) {
        let (origin, edge1, edge2) = (
            Vect::new(0.0, 0.0),
            Vect::new(1.0, 0.0),
            Vect::new(0.0, 1.0),
        );
        let as_list = make_frame_list(origin, edge1, edge2);
        let as_pair = make_frame_pair(origin, edge1, edge2);
        (
            render(
                origin_frame_list(&as_list),
                edge1_frame_list(&as_list),
                edge2_frame_list(&as_list),
            ),
            render(
                origin_frame_pair(&as_pair),
                edge1_frame_pair(&as_pair),
                edge2_frame_pair(&as_pair),
            ),
        )
    }
}

#[test]
fn ex_2_47() {
    assert_eq!(
        ex_2_47::ex_2_47(),
        (
            "(0, 0) (1, 0) (0, 1)".to_string(),
            "(0, 0) (1, 0) (0, 1)".to_string()
        )
    );
}
