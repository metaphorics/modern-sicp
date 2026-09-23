// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.2.4

//! Section 2.2.4: the picture language, and this edition's figure
//! generator. Painters are functions from a frame to the segments they
//! draw; rendering them through the SVG sink writes the deterministic
//! line-art files the book references under `figures/generated/chap2/`.
//! The generated figures are checked in, so the book builds without
//! running this example.

use ch02::sec_2_2::{
    Frame, Painter, Segment, Vect, below, beside, corner_split, flip_horiz, flip_vert, identity_op,
    painter_op, render_svg, right_split, rotate_90, rotate_180, segments_painter, square_limit,
    square_of_four, transform_painter, wave, wave_segments,
};
use std::path::Path;
use std::rc::Rc;

// --- primitive painters (exercise 2.49) ---------------------------------

fn seg(start: (f64, f64), end: (f64, f64)) -> Segment {
    Segment::new(Vect::new(start.0, start.1), Vect::new(end.0, end.1))
}

fn outline() -> Painter {
    segments_painter(vec![
        seg((0.0, 0.0), (1.0, 0.0)),
        seg((1.0, 0.0), (1.0, 1.0)),
        seg((1.0, 1.0), (0.0, 1.0)),
        seg((0.0, 1.0), (0.0, 0.0)),
    ])
}

fn cross() -> Painter {
    segments_painter(vec![
        seg((0.0, 0.0), (1.0, 1.0)),
        seg((0.0, 1.0), (1.0, 0.0)),
    ])
}

fn diamond() -> Painter {
    segments_painter(vec![
        seg((0.5, 0.0), (1.0, 0.5)),
        seg((1.0, 0.5), (0.5, 1.0)),
        seg((0.5, 1.0), (0.0, 0.5)),
        seg((0.0, 0.5), (0.5, 0.0)),
    ])
}

fn wave_with_smile() -> Painter {
    let v = Vect::new;
    let mut segments = wave_segments();
    segments.push(Segment::new(v(0.40, 0.58), v(0.47, 0.53)));
    segments.push(Segment::new(v(0.47, 0.53), v(0.54, 0.58)));
    segments_painter(segments)
}

// --- the 2.51 pair of below constructions, and the 2.52 variants --------

fn below_direct(painter1: &Painter, painter2: &Painter) -> Painter {
    let bottom = transform_painter(
        painter1,
        Vect::new(0.0, 0.0),
        Vect::new(1.0, 0.0),
        Vect::new(0.0, 0.5),
    );
    let top = transform_painter(
        painter2,
        Vect::new(0.0, 0.5),
        Vect::new(1.0, 0.5),
        Vect::new(0.0, 1.0),
    );
    Rc::new(move |frame: &Frame, sink: &mut dyn ch02::sec_2_2::Sink| {
        bottom(frame, sink);
        top(frame, sink);
    })
}

fn rotate_270_direct(painter: &Painter) -> Painter {
    transform_painter(
        painter,
        Vect::new(0.0, 1.0),
        Vect::new(0.0, 0.0),
        Vect::new(1.0, 1.0),
    )
}

fn below_rotated(painter1: &Painter, painter2: &Painter) -> Painter {
    rotate_90(&beside(
        &rotate_270_direct(painter1),
        &rotate_270_direct(painter2),
    ))
}

fn corner_split_variant(painter: &Painter, n: u32) -> Painter {
    if n == 0 {
        return Rc::clone(painter);
    }
    beside(
        &below(painter, &ch02::sec_2_2::up_split(painter, n - 1)),
        &below(
            &right_split(painter, n - 1),
            &corner_split_variant(painter, n - 1),
        ),
    )
}

fn square_limit_variant(painter: &Painter, n: u32) -> Painter {
    (square_of_four(
        painter_op(rotate_180),
        painter_op(|p| flip_vert(p)),
        identity_op(),
        painter_op(|p| flip_horiz(p)),
    ))(&corner_split_variant(painter, n))
}

// --- the figure generator -----------------------------------------------

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let painter = wave();

    // The book builds `wave4` from `wave` in two stages: beside wave
    // with its vertical flip, then stack that pair over itself.
    let wave2 = beside(&painter, &flip_vert(&painter));
    let wave4 = below(&wave2, &wave2);

    // Probes for the below constructions: a low stroke and a high one.
    let low = segments_painter(vec![seg((0.1, 0.2), (0.4, 0.3))]);
    let high = segments_painter(vec![seg((0.1, 0.7), (0.4, 0.8))]);

    let figures: Vec<(&str, String)> = vec![
        ("wave", render_svg(&painter, 200)),
        ("wave4", render_svg(&wave4, 200)),
        ("outline", render_svg(&outline(), 200)),
        ("cross", render_svg(&cross(), 200)),
        ("diamond", render_svg(&diamond(), 200)),
        ("wave_smile", render_svg(&wave_with_smile(), 200)),
        ("below_direct", render_svg(&below_direct(&low, &high), 200)),
        ("below_rotate", render_svg(&below_rotated(&low, &high), 200)),
        ("corner_split", render_svg(&corner_split(&painter, 4), 200)),
        (
            "corner_split_variant",
            render_svg(&corner_split_variant(&painter, 4), 200),
        ),
        ("square_limit", render_svg(&square_limit(&painter, 4), 200)),
        (
            "square_limit_variant",
            render_svg(&square_limit_variant(&painter, 4), 200),
        ),
    ];

    // Every SVG is deterministic markup, so a regenerated figure is
    // byte-identical to the checked-in one and the solutions' freshness
    // tests hold.
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../book/figures/generated/chap2");
    std::fs::create_dir_all(&dir)?;
    for (name, svg) in &figures {
        std::fs::write(dir.join(format!("{name}.std.svg")), svg)?;
        println!("wrote figures/generated/chap2/{name}.std.svg");
    }

    assert_eq!(figures.len(), 12);
    assert!(figures.iter().all(|(_, svg)| svg.contains("<line")));
    assert_eq!(render_svg(&wave4, 200), render_svg(&wave4, 200));

    // The rotate composition of `below` paints exactly what the direct
    // transform paints; see exercises 2.51 and 2.51a.
    assert_eq!(
        render_svg(&below_direct(&low, &high), 200),
        render_svg(&below_rotated(&low, &high), 200)
    );

    println!("all figures rendered");
    // => ok
    Ok(())
}
