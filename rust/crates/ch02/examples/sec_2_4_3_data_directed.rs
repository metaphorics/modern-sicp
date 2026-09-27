// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.4.3

//! Section 2.4.3: data-directed programming. The two representation
//! packages install their procedures into the operation table under
//! `(rectangular)` and `(polar)`, `apply_generic` looks the combination
//! up, and adding a representation means adding table entries — no
//! existing procedure changes.

use ch02::sec_2_4::{
    add_complex, angle, apply_generic, attach_tag, contents, imag_part, install_polar_package,
    install_rectangular_package, magnitude, make_from_mag_ang, make_from_real_imag, mul_complex,
    real_part, tag_list_key,
};
use sicp_runtime::{Key, OpTable};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let table = OpTable::new();
    install_rectangular_package(&table);
    install_polar_package(&table);

    // The table is directly readable: Ben's real-part is the entry under
    // ("real-part", (rectangular)).
    let Some(ben_real_part) = table.get(&Key::sym("real-part"), &tag_list_key(&["rectangular"]))
    else {
        return Err("rectangular package is not installed".into());
    };
    let z = make_from_real_imag(&table, 3.0, 4.0)?;
    let z_contents = contents(&z)?;
    let contents_slice = std::slice::from_ref(&z_contents);
    println!("{}", ben_real_part(contents_slice)?);
    // => 3
    assert_eq!(ben_real_part(contents_slice)?.to_string(), "3");

    // The same lookup misses into the absent option for an uninstalled
    // combination.
    println!(
        "{}",
        table
            .get(&Key::sym("real-part"), &tag_list_key(&["spherical"]))
            .is_none()
    );
    // => true
    assert!(
        table
            .get(&Key::sym("real-part"), &tag_list_key(&["spherical"]))
            .is_none()
    );

    // The generic selectors go through apply_generic: tags of the
    // arguments, table lookup, handler applied to the contents.
    let w = make_from_mag_ang(&table, 5.0, 0.927_295_218_001_612_2)?;
    println!("{}", real_part(&table, &w)?);
    // => 3.0000000000000004
    assert!((real_part(&table, &w)?.to_string().parse::<f64>()? - 3.0).abs() < 1e-9);
    println!("{}", imag_part(&table, &w)?);
    // => 3.9999999999999996
    assert!((imag_part(&table, &w)?.to_string().parse::<f64>()? - 4.0).abs() < 1e-9);
    println!("{}", magnitude(&table, &z)?);
    // => 5
    assert_eq!(magnitude(&table, &z)?.to_string(), "5");
    println!("{}", angle(&table, &z)?);
    // => 0.9272952180016122
    assert_eq!(angle(&table, &z)?.to_string(), "0.9272952180016122");

    // The constructors, too, come out of the table.
    let sum = add_complex(&table, &z, &w)?;
    println!("{sum}");
    // => (rectangular (6 . 8))
    assert_eq!(sum.to_string(), "(rectangular (6 . 8))");
    let product = mul_complex(&table, &z, &z)?;
    println!("{}", magnitude(&table, &product)?);
    // => 25
    assert_eq!(magnitude(&table, &product)?.to_string(), "25");

    // A tag with no installed package reaches apply_generic's error path,
    // which names the operation and the tag list.
    let sphere = attach_tag(
        "spherical",
        sicp_runtime::Value::list(vec![sicp_runtime::Value::real(3.0)]),
    );
    let err = apply_generic(&table, "magnitude", std::slice::from_ref(&sphere))
        .expect_err("no spherical package");
    println!("{err}");
    // => No method for these types: APPLY-GENERIC (magnitude (spherical))
    assert_eq!(
        err.to_string(),
        "No method for these types: APPLY-GENERIC (magnitude (spherical))"
    );

    Ok(())
}
