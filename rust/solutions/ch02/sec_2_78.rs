// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.78.

mod ex_2_78 {
    use std::rc::Rc;

    use ch02::sec_2_5::{apply_generic, contents, install_generic_arithmetic, type_tag};
    use sicp_runtime::{Handler, Key, OpTable, SchemeError, Value};

    /// The 2.4.2 trio before the modification: every datum is wrapped,
    /// even ordinary numbers, whose tag is the symbol `scheme-number`.
    pub fn attach_tag_wrapped(type_tag: &str, contents: Value) -> Value {
        Value::tagged(type_tag, contents)
    }

    /// The wrapped `type_tag`: answers the wrapper's tag verbatim.
    ///
    /// # Errors
    /// [`SchemeError::UserRaised`] when the datum is not wrapped.
    pub fn type_tag_wrapped(datum: &Value) -> Result<String, SchemeError> {
        match datum {
            Value::Tagged { tag, .. } => Ok(tag.as_ref().to_owned()),
            other => Err(SchemeError::UserRaised {
                message: "Bad tagged datum: TYPE-TAG".into(),
                irritants: vec![other.clone()],
            }),
        }
    }

    /// The wrapped `contents`: strips the single wrapper.
    ///
    /// # Errors
    /// [`SchemeError::UserRaised`] when the datum is not wrapped.
    pub fn contents_wrapped(datum: &Value) -> Result<Value, SchemeError> {
        match datum {
            Value::Tagged { data, .. } => Ok((**data).clone()),
            other => Err(SchemeError::UserRaised {
                message: "Bad tagged datum: CONTENTS".into(),
                irritants: vec![other.clone()],
            }),
        }
    }

    const SN2: fn() -> Key = || {
        Key::pair(
            Key::sym("scheme-number"),
            Key::pair(Key::sym("scheme-number"), Key::Nil),
        )
    };

    /// Installs a scheme-number package whose handlers wrap and unwrap
    /// like the pre-2.78 system did: the arithmetic sees bare numbers,
    /// the table sees tagged ones.
    pub fn install_scheme_number_wrapped(table: &OpTable) {
        // `apply_generic` strips the wrapper before the handler runs;
        // the wrapped trio is what makes the RESULT wrapped again.
        let bin = |f: fn(i128, i128) -> Result<i128, SchemeError>| {
            move |args: &[Value]| {
                let (Value::Int(x), Value::Int(y)) = (args[0].clone(), args[1].clone()) else {
                    return Err(SchemeError::TypeMismatch(
                        "wrapped package expects ordinary numbers".into(),
                    ));
                };
                f(x, y).map(|n| attach_tag_wrapped("scheme-number", Value::Int(n)))
            }
        };
        let add: Handler = Rc::new(bin(|a, b| a.checked_add(b).ok_or(SchemeError::Overflow)));
        let mul: Handler = Rc::new(bin(|a, b| a.checked_mul(b).ok_or(SchemeError::Overflow)));
        table.put(Key::sym("add"), SN2(), add);
        table.put(Key::sym("mul"), SN2(), mul);
    }

    /// The exercise, in three observations: the wrapped system wraps
    /// ordinary numbers and unwraps them per call; the modified trio
    /// leaves them bare; and the generic interface answers the same on
    /// both.
    pub fn ex_2_78() -> Result<(String, String, String), SchemeError> {
        // Before: an ordinary number is a wrapped tagged datum, and the
        // package unwraps it to do arithmetic.
        let wrapped_table = OpTable::new();
        install_scheme_number_wrapped(&wrapped_table);
        let three = attach_tag_wrapped("scheme-number", Value::Int(3));
        let four = attach_tag_wrapped("scheme-number", Value::Int(4));
        let sum = apply_generic(&wrapped_table, "add", &[three.clone(), four.clone()])?;
        let before = format!(
            "{} + {} = {}",
            type_tag_wrapped(&three)?,
            type_tag_wrapped(&four)?,
            sum
        );
        // The wrapped sum really is a tagged 7 underneath: the pair to
        // `type_tag_wrapped` above, proving the wrapper round-trips.
        if contents_wrapped(&sum)? != Value::Int(7) {
            return Err(SchemeError::TypeMismatch(
                "wrapped sum does not unwrap to 7".into(),
            ));
        }

        // After: the tag functions read the runtime's own variant, so an
        // ordinary number needs no wrapper at all.
        let bare = Value::Int(7);
        let after = format!("{} {}", type_tag(&bare)?, contents(&bare)?);

        // The generic interface treats the bare number like the wrapped
        // one: (mul 6 7) answers 42 without any wrapper in sight.
        let table = OpTable::new();
        install_generic_arithmetic(&table)?;
        let product = apply_generic(&table, "mul", &[Value::Int(6), Value::Int(7)])?;
        let generic = format!("{product} {}", product == Value::Int(42));

        Ok((before, after, generic))
    }
}

#[test]
fn ex_2_78() {
    let (before, after, generic) = ex_2_78::ex_2_78().expect("tag trio works");
    assert_eq!(before, "scheme-number + scheme-number = (scheme-number 7)");
    assert_eq!(after, "scheme-number 7");
    assert_eq!(generic, "42 true");
}
