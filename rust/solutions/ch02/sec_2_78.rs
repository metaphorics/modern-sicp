// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.78.

mod ex_2_78 {
    use std::rc::Rc;

    use ch02::sec_2_5::{apply_generic, install_generic_arithmetic};
    use sicp_runtime::{Handler, Key, OpTable, SicpError, Value};

    /// The 2.4.2 trio before the modification: every datum is wrapped,
    /// even ordinary numbers, whose tag is the symbol `integer`.
    pub fn attach_tag_wrapped(type_tag: &str, contents: Value) -> Value {
        Value::tagged(type_tag, contents)
    }

    /// The wrapped `type_tag`: answers the wrapper's tag verbatim.
    ///
    /// # Errors
    /// [`SicpError::UserRaised`] when the datum is not wrapped.
    pub fn type_tag_wrapped(datum: &Value) -> Result<String, SicpError> {
        match datum {
            Value::Tagged { tag, .. } => Ok(tag.as_ref().to_owned()),
            other => Err(SicpError::UserRaised {
                message: "Bad tagged datum: type_tag".into(),
                irritants: vec![other.clone()],
            }),
        }
    }

    /// The wrapped `contents`: strips the single wrapper.
    ///
    /// # Errors
    /// [`SicpError::UserRaised`] when the datum is not wrapped.
    pub fn contents_wrapped(datum: &Value) -> Result<Value, SicpError> {
        match datum {
            Value::Tagged { data, .. } => Ok((**data).clone()),
            other => Err(SicpError::UserRaised {
                message: "Bad tagged datum: contents".into(),
                irritants: vec![other.clone()],
            }),
        }
    }

    const II: fn() -> Key = || {
        Key::pair(
            Key::sym("integer"),
            Key::pair(Key::sym("integer"), Key::Nil),
        )
    };

    /// Installs a wrapped integer package whose handlers wrap and
    /// unwrap like the pre-2.78 system did: the arithmetic sees bare
    /// numbers, the table sees tagged ones.
    pub fn install_integer_wrapped(table: &OpTable) {
        // `apply_generic` strips the wrapper before the handler runs;
        // the wrapped trio is what makes the result wrapped again.
        let bin = |f: fn(i128, i128) -> Result<i128, SicpError>| {
            move |args: &[Value]| {
                let (Value::Int(x), Value::Int(y)) = (args[0].clone(), args[1].clone()) else {
                    return Err(SicpError::TypeMismatch(
                        "wrapped package expects plain integers".into(),
                    ));
                };
                f(x, y).map(|n| attach_tag_wrapped("integer", Value::Int(n)))
            }
        };
        let add: Handler = Rc::new(bin(|a, b| a.checked_add(b).ok_or(SicpError::Overflow)));
        let mul: Handler = Rc::new(bin(|a, b| a.checked_mul(b).ok_or(SicpError::Overflow)));
        table.put(Key::sym("add"), II(), add);
        table.put(Key::sym("mul"), II(), mul);
    }

    /// The exercise, in three observations: the wrapped system's sum is
    /// a tagged datum; the modified trio leaves an ordinary number bare;
    /// and the generic interface answers the same on bare numbers.
    pub fn ex_2_78() -> Result<(Value, Value, Value), SicpError> {
        // Before: an ordinary number is a wrapped tagged datum, and the
        // package unwraps it to do arithmetic.
        let wrapped_table = OpTable::new();
        install_integer_wrapped(&wrapped_table);
        let three = attach_tag_wrapped("integer", Value::Int(3));
        let four = attach_tag_wrapped("integer", Value::Int(4));
        let sum = apply_generic(&wrapped_table, "add", &[three, four])?;
        // The wrapped sum really is a tagged 7 underneath: the wrapper
        // round-trips through the stripped contents.
        if contents_wrapped(&sum)? != Value::Int(7) {
            return Err(SicpError::TypeMismatch(
                "wrapped sum does not unwrap to 7".into(),
            ));
        }

        // After: the tag functions read the runtime's own variant, so an
        // ordinary number needs no wrapper at all.
        let bare = Value::Int(7);

        // The generic interface treats the bare number like the wrapped
        // one: `mul` of 6 and 7 answers 42 without any wrapper in sight.
        let table = OpTable::new();
        install_generic_arithmetic(&table)?;
        let product = apply_generic(&table, "mul", &[Value::Int(6), Value::Int(7)])?;

        Ok((sum, bare, product))
    }
}

#[test]
fn ex_2_78() {
    use ch02::sec_2_5::{contents, type_tag};
    use sicp_runtime::Value;

    let (sum, bare, product) = ex_2_78::ex_2_78().expect("tag trio works");
    // The pre-modification system wraps ordinary numbers: the sum is a
    // tagged datum whose wrapper is the `integer` tag and whose contents
    // are the plain 7.
    assert!(matches!(&sum, Value::Tagged { .. }));
    assert_eq!(ex_2_78::type_tag_wrapped(&sum).unwrap(), "integer");
    assert_eq!(ex_2_78::contents_wrapped(&sum).unwrap(), Value::Int(7));
    // The modified trio leaves ordinary numbers bare: a bare number
    // answers its own domain tag and its own contents.
    assert!(matches!(&bare, Value::Int(7)));
    assert_eq!(type_tag(&bare).unwrap().as_ref(), "integer");
    assert_eq!(contents(&bare).unwrap(), bare);
    // The generic interface answers the same on bare numbers.
    assert_eq!(product, Value::Int(42));
}
