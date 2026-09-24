// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.75.

mod ex_2_75 {
    use std::rc::Rc;

    use ch02::sec_2_4::{MessageObject, apply_generic_message_passing};
    use sicp_runtime::{SchemeError, Value};

    /// Exercise 2.75: `make-from-mag-ang` in message-passing style,
    /// analogous to the section's `make_from_real_imag_message_passing`.
    #[must_use]
    pub fn make_from_mag_ang_message_passing(r: f64, a: f64) -> MessageObject {
        Rc::new(move |op| match op {
            "magnitude" => Ok(Value::real(r)),
            "angle" => Ok(Value::real(a)),
            "real-part" => Ok(Value::real(r * a.cos())),
            "imag-part" => Ok(Value::real(r * a.sin())),
            other => Err(SchemeError::UserRaised {
                message: "Unknown op: MAKE-FROM-MAG-ANG".into(),
                irritants: vec![Value::sym(other)],
            }),
        })
    }

    /// Builds the polar number with magnitude 5 and the angle of
    /// `(3 . 4)`, then reads its magnitude and angle back through
    /// `apply_generic_message_passing`.
    pub fn ex_2_75() -> Result<(String, String), SchemeError> {
        let z = make_from_mag_ang_message_passing(5.0, 4.0f64.atan2(3.0));
        let magnitude = apply_generic_message_passing("magnitude", &z)?.to_string();
        let angle = apply_generic_message_passing("angle", &z)?.to_string();
        Ok((magnitude, angle))
    }

    #[cfg(test)]
    mod tests {
        use super::make_from_mag_ang_message_passing;
        use ch02::sec_2_4::apply_generic_message_passing;

        #[test]
        fn ex_2_75() {
            assert_eq!(
                super::ex_2_75(),
                Ok(("5".to_string(), "0.9272952180016122".to_string()))
            );
        }
        #[test]
        fn answers_real_and_imaginary_parts_too() {
            let z = make_from_mag_ang_message_passing(5.0, 4.0f64.atan2(3.0));
            let real_part = apply_generic_message_passing("real-part", &z)
                .expect("answered")
                .to_string()
                .parse::<f64>()
                .expect("a real number");
            assert!((real_part - 3.0).abs() < 1e-9);
        }

        #[test]
        fn unanswered_message_raises_the_books_error() {
            let z = make_from_mag_ang_message_passing(5.0, 0.0);
            let err = apply_generic_message_passing("rotation", &z).expect_err("unanswered");
            assert_eq!(err.to_string(), "Unknown op: MAKE-FROM-MAG-ANG rotation");
        }
    }
}
