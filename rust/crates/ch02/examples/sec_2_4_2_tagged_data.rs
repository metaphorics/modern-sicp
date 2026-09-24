// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.4.2

//! Section 2.4.2: tagged data. Both representations coexist in one
//! system; each number carries its tag, and the 2.4.2 selectors dispatch
//! on it explicitly before 2.4.3 moves the dispatch into the table.

use ch02::sec_2_4::{
    angle_dispatch, attach_tag, contents, imag_part_dispatch, is_polar, is_rectangular,
    magnitude_dispatch, polar_make_from_mag_ang_tagged, polar_make_from_real_imag_tagged,
    real_part_dispatch, rect_make_from_mag_ang_tagged, rect_make_from_real_imag_tagged, type_tag,
};
use sicp_runtime::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // attach-tag/type-tag/contents are the runtime's tagged-datum
    // constructors and accessors.
    let z = rect_make_from_real_imag_tagged(3.0, 4.0);
    println!("{z}");
    // => (rectangular (3 . 4))
    assert_eq!(z.to_string(), "(rectangular (3 . 4))");

    println!("{}", is_rectangular(&z));
    // => true
    assert!(is_rectangular(&z));
    println!("{}", is_polar(&z));
    // => false
    assert!(!is_polar(&z));

    println!("{}", type_tag(&z)?.as_ref());
    // => rectangular
    assert_eq!(type_tag(&z)?.as_ref(), "rectangular");

    println!("{}", contents(&z)?);
    // => (3 . 4)
    assert_eq!(contents(&z)?.to_string(), "(3 . 4)");

    // Attaching a tag to the same pair the other way, to show the tag is
    // ordinary list structure, is attach_tag itself:
    let bare = attach_tag("rectangular", contents(&z)?);
    assert_eq!(bare, z);

    // Alyssa's numbers, tagged polar.
    let w = polar_make_from_mag_ang_tagged(5.0, 0.927_295_218_001_612_2);
    println!("{w}");
    // => (polar (5 . 0.9272952180016122))
    assert_eq!(w.to_string(), "(polar (5 . 0.9272952180016122))");
    assert!(is_polar(&w));

    // Ben's other constructor converts polar input to his representation,
    // as the book's 2.4.1 listing already did.
    let z2 = rect_make_from_mag_ang_tagged(1.0, std::f64::consts::FRAC_PI_2);
    println!("{z2}");
    // => (rectangular (0.00000000000000006123233995736766 . 1))
    assert_eq!(
        z2.to_string(),
        "(rectangular (0.00000000000000006123233995736766 . 1))"
    );
    let w2 = polar_make_from_real_imag_tagged(3.0, 4.0);
    println!("{w2}");
    // => (polar (5 . 0.9272952180016122))
    assert_eq!(w2.to_string(), "(polar (5 . 0.9272952180016122))");

    // The 2.4.2 generic selectors check the tag, strip it, and call the
    // representation's procedure on the bare contents.
    println!("{}", real_part_dispatch(&w)?);
    // => 3.0000000000000004
    assert!((real_part_dispatch(&w)?.to_string().parse::<f64>()? - 3.0).abs() < 1e-9);
    println!("{}", imag_part_dispatch(&w)?);
    // => 3.9999999999999996
    assert!((imag_part_dispatch(&w)?.to_string().parse::<f64>()? - 4.0).abs() < 1e-9);
    println!("{}", magnitude_dispatch(&z)?);
    // => 5
    assert_eq!(magnitude_dispatch(&z)?.to_string(), "5");
    println!("{}", angle_dispatch(&z)?);
    // => 0.9272952180016122
    assert_eq!(angle_dispatch(&z)?.to_string(), "0.9272952180016122");

    // A datum with no tag is bad in the eyes of both selectors.
    let untagged = Value::Pair(sicp_runtime::cons_cell(Value::real(3.0), Value::real(4.0)));
    let err = type_tag(&untagged).expect_err("untagged");
    println!("{err}");
    // => Bad tagged datum: TYPE-TAG (3 . 4)
    assert_eq!(err.to_string(), "Bad tagged datum: TYPE-TAG (3 . 4)");
    let err = magnitude_dispatch(&untagged).expect_err("untagged");
    assert!(err.to_string().starts_with("Unknown type: MAGNITUDE"));

    Ok(())
}
