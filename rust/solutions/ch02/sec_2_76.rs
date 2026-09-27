// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.76.

mod ex_2_76 {
    use std::rc::Rc;

    use ch02::sec_2_4::{
        MessageObject, apply_generic_message_passing, install_polar_package,
        install_rectangular_package, magnitude, make_from_real_imag_message_passing, real_part,
    };
    use sicp_runtime::{OpTable, SchemeError, Value};

    // --- Explicit dispatch: one closed enum, every operation a `match`. ---

    #[derive(Clone, Copy)]
    enum Z {
        Rect(f64, f64),
        Polar(f64, f64),
    }

    fn ed_real_part(z: Z) -> f64 {
        match z {
            Z::Rect(x, _) => x,
            Z::Polar(r, a) => r * a.cos(),
        }
    }

    fn ed_magnitude(z: Z) -> f64 {
        match z {
            Z::Rect(x, y) => (x * x + y * y).sqrt(),
            Z::Polar(r, _) => r,
        }
    }

    /// Serves the same probe through explicit dispatch: a third
    /// representation would add a `match` arm to both `ed_real_part` and
    /// `ed_magnitude` — two existing definitions edited. A new operation
    /// is one new function that already matches every variant, so it
    /// edits nothing existing.
    fn explicit_dispatch_probe() -> (f64, f64) {
        let z = Z::Rect(3.0, 4.0);
        (ed_real_part(z), ed_magnitude(z))
    }

    // --- Data-directed: the section's table, both packages installed. ---

    /// Serves the same probe through the operation table. Installing a
    /// third package would add table entries and edit nothing: a test
    /// below proves this by pointer identity, not just by absence of a
    /// diff.
    fn data_directed_probe() -> Result<(f64, f64), SchemeError> {
        let table = OpTable::new();
        install_rectangular_package(&table);
        install_polar_package(&table);
        let z = ch02::sec_2_4::make_from_real_imag(&table, 3.0, 4.0)?;
        let real = as_real(&real_part(&table, &z)?)?;
        let mag = as_real(&magnitude(&table, &z)?)?;
        Ok((real, mag))
    }

    fn as_real(v: &Value) -> Result<f64, SchemeError> {
        match v {
            Value::Real(x) => Ok(*x),
            other => Err(SchemeError::TypeMismatch(format!(
                "not a real number: {other}"
            ))),
        }
    }

    // --- Message passing: one object per representation. ---

    fn polar_message_object(r: f64, a: f64) -> MessageObject {
        Rc::new(move |op| match op {
            "real-part" => Ok(Value::real(r * a.cos())),
            "magnitude" => Ok(Value::real(r)),
            other => Err(SchemeError::UserRaised {
                message: "Unknown op".into(),
                irritants: vec![Value::sym(other)],
            }),
        })
    }

    /// Serves the same probe through message-passing objects. Adding a
    /// third representation is one new object (0 edits). Adding a new
    /// operation edits every existing object — here both the rectangular
    /// object from the section and [`polar_message_object`] would each
    /// gain a match arm.
    fn message_passing_probe() -> Result<(f64, f64), SchemeError> {
        let z = make_from_real_imag_message_passing(3.0, 4.0);
        let real = as_real(&apply_generic_message_passing("real-part", &z)?)?;
        let mag = as_real(&apply_generic_message_passing("magnitude", &z)?)?;
        Ok((real, mag))
    }

    /// Exercise 2.76: the same `(real-part, magnitude)` pair served three
    /// ways — explicit dispatch, data-directed programming, and message
    /// passing — to ground the comparison the exercise asks for.
    #[allow(
        clippy::type_complexity,
        reason = "one probe pair per strategy, matching the exercise's three-way comparison"
    )]
    pub fn ex_2_76() -> Result<((f64, f64), (f64, f64), (f64, f64)), SchemeError> {
        Ok((
            explicit_dispatch_probe(),
            data_directed_probe()?,
            message_passing_probe()?,
        ))
    }

    #[cfg(test)]
    mod tests {
        use super::{data_directed_probe, polar_message_object};
        use ch02::sec_2_4::{install_polar_package, install_rectangular_package, tag_list_key};
        use sicp_runtime::{Key, OpTable};

        #[test]
        fn ex_2_76() {
            assert_eq!(super::ex_2_76(), Ok(((3.0, 5.0), (3.0, 5.0), (3.0, 5.0))));
        }

        #[test]
        fn additive_install_leaves_existing_handlers_untouched() {
            // The data-directed organization's answer to "what does
            // adding a type cost": the pointer to an already-installed
            // handler is unchanged after a third package installs.
            let table = OpTable::new();
            install_rectangular_package(&table);
            let before = table
                .get(&Key::sym("real-part"), &tag_list_key(&["rectangular"]))
                .expect("rectangular installed");
            install_polar_package(&table);
            let after = table
                .get(&Key::sym("real-part"), &tag_list_key(&["rectangular"]))
                .expect("still installed");
            assert!(std::rc::Rc::ptr_eq(&before, &after));
        }

        #[test]
        fn message_passing_object_answers_its_two_operations() {
            let w = polar_message_object(5.0, 0.0);
            assert_eq!(w("magnitude").expect("answered").to_string(), "5");
            assert_eq!(w("real-part").expect("answered").to_string(), "5");
        }

        #[test]
        fn data_directed_probe_matches_the_explicit_one() {
            assert_eq!(data_directed_probe(), Ok((3.0, 5.0)));
        }

        #[test]
        fn explicit_dispatch_serves_the_polar_representation_too() {
            let w = super::Z::Polar(5.0, 0.0);
            assert!((super::ed_real_part(w) - 5.0).abs() < 1e-9);
            assert!((super::ed_magnitude(w) - 5.0).abs() < 1e-9);
        }
    }
}
