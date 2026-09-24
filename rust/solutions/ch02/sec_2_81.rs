// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.81.

mod ex_2_81 {
    use ch02::sec_2_5::{
        contents, install_generic_arithmetic, make_complex_from_real_imag, put_coercion, type_tag,
    };
    use sicp_runtime::{Handler, Key, OpTable, SchemeError, Value};
    use std::rc::Rc;

    fn tag_list_key(args: &[Value]) -> Result<Key, SchemeError> {
        let mut key = Key::Nil;
        for a in args.iter().rev() {
            key = Key::pair(Key::Sym(type_tag(a)?), key);
        }
        Ok(key)
    }

    fn no_method(op: &str, args: &[Value]) -> SchemeError {
        let mut irritants = vec![Value::sym(op)];
        irritants.extend(args.iter().cloned());
        SchemeError::UserRaised {
            message: "No method for these types".into(),
            irritants,
        }
    }

    /// The section's `apply_generic`, capped at `max_depth` coercions so
    /// Louis's infinite recursion is observable instead of hanging.
    fn louis_apply_generic(
        table: &OpTable,
        coercions: &OpTable,
        op: &str,
        args: &[Value],
        depth: u32,
    ) -> Result<Value, SchemeError> {
        if depth == 0 {
            return Err(SchemeError::UserRaised {
                message: "recursion did not terminate: Louis's self-coercion loop".into(),
                irritants: vec![],
            });
        }
        if let Some(proc) = table.get(&Key::sym(op), &tag_list_key(args)?) {
            let bare: Result<Vec<Value>, SchemeError> = args.iter().map(contents).collect();
            return proc(&bare?);
        }
        if let [a1, a2] = args {
            let (t1, t2) = (type_tag(&args[0])?, type_tag(&args[1])?);
            if let Some(coerce) = coercions.get(&Key::sym(t1.as_ref()), &Key::sym(t2.as_ref())) {
                let a1 = coerce(std::slice::from_ref(a1))?;
                return louis_apply_generic(table, coercions, op, &[a1, a2.clone()], depth - 1);
            }
            if let Some(coerce) = coercions.get(&Key::sym(t2.as_ref()), &Key::sym(t1.as_ref())) {
                let a2 = coerce(std::slice::from_ref(a2))?;
                return louis_apply_generic(table, coercions, op, &[a1.clone(), a2], depth - 1);
            }
        }
        Err(no_method(op, args))
    }

    /// Part (c): the same dispatch, but same-type arguments give up
    /// immediately instead of trying to coerce them into themselves.
    fn fixed_apply_generic(
        table: &OpTable,
        coercions: &OpTable,
        op: &str,
        args: &[Value],
    ) -> Result<Value, SchemeError> {
        // Direct entries always win, as before; the fix only skips the
        // coercion attempt when the arguments already share a type.
        if let Some(proc) = table.get(&Key::sym(op), &tag_list_key(args)?) {
            let bare: Result<Vec<Value>, SchemeError> = args.iter().map(contents).collect();
            return proc(&bare?);
        }
        if let [a1, _a2] = args {
            let t1 = type_tag(a1)?;
            let t2 = type_tag(&args[1])?;
            if t1 == t2 {
                return Err(no_method(op, args));
            }
        }
        louis_apply_generic(table, coercions, op, args, 8)
    }

    /// Louis's identity coercions plus the `exp` operation, which only
    /// the scheme-number package offers.
    pub fn install(table: &OpTable, coercions: &OpTable) {
        put_coercion(
            coercions,
            "scheme-number",
            "scheme-number",
            Rc::new(|args: &[Value]| Ok(args[0].clone())),
        );
        put_coercion(
            coercions,
            "complex",
            "complex",
            Rc::new(|args: &[Value]| Ok(args[0].clone())),
        );
        let key = Key::pair(
            Key::sym("scheme-number"),
            Key::pair(Key::sym("scheme-number"), Key::Nil),
        );
        let exp: Handler = Rc::new(|args: &[Value]| {
            let (Value::Int(b), Value::Int(e)) = (&args[0], &args[1]) else {
                return Err(SchemeError::TypeMismatch("exp: integers".into()));
            };
            let e = u32::try_from(*e).map_err(|_| SchemeError::Overflow)?;
            b.checked_pow(e)
                .map(Value::Int)
                .ok_or(SchemeError::Overflow)
        });
        table.put(Key::sym("exp"), key, exp);
    }

    /// The three answers: what Louis's coercions do to a missing
    /// same-type operation, what the fixed dispatch does, and that `exp`
    /// still answers on two ordinary numbers.
    pub fn ex_2_81() -> Result<(String, String, i128), SchemeError> {
        let table = OpTable::new();
        install_generic_arithmetic(&table)?;
        let coercions = OpTable::new();
        install(&table, &coercions);

        let z1 = make_complex_from_real_imag(&table, 1.0, 0.0)?;
        let z2 = make_complex_from_real_imag(&table, 1.0, 1.0)?;

        // (a): with the identity coercions installed, each "fix" coerces
        // the arguments into the type they already have, so the same miss
        // recurses forever; the depth cap turns that into an error.
        let louis = louis_apply_generic(&table, &coercions, "exp", &[z1.clone(), z2.clone()], 64)
            .expect_err("does not terminate");
        // (b)/(c): without self-coercion the miss is reported at once.
        let fixed = fixed_apply_generic(&table, &coercions, "exp", &[z1, z2])
            .expect_err("no exp for complex");
        // The operation itself works where it is installed.
        let Value::Int(eight) =
            fixed_apply_generic(&table, &coercions, "exp", &[Value::Int(2), Value::Int(3)])?
        else {
            return Err(SchemeError::TypeMismatch("exp 2 3".into()));
        };

        Ok((louis.to_string(), fixed.to_string(), eight))
    }
}

#[test]
fn ex_2_81() {
    let (louis, fixed, eight) = ex_2_81::ex_2_81().expect("2.81 answers");
    assert!(louis.contains("did not terminate"), "Louis's loop: {louis}");
    assert!(
        fixed.contains("No method for these types"),
        "fixed dispatch: {fixed}"
    );
    assert_eq!(eight, 8);
}
