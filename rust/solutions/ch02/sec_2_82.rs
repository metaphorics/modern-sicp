// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.82.

mod ex_2_82 {
    use ch02::sec_2_4::{imag_part_dispatch, real_part_dispatch};
    use ch02::sec_2_5::{
        contents, install_generic_arithmetic, make_complex_from_real_imag, make_rational,
        put_coercion, type_tag,
    };
    use sicp_runtime::{Handler, Key, OpTable, SicpError, Value};
    use std::rc::Rc;

    fn tag_list_key(args: &[Value]) -> Result<Key, SicpError> {
        let mut key = Key::Nil;
        for a in args.iter().rev() {
            key = Key::pair(Key::Sym(type_tag(a)?), key);
        }
        Ok(key)
    }

    fn no_method(op: &str) -> SicpError {
        SicpError::UserRaised {
            message: "No method for these types".into(),
            irritants: vec![Value::sym(op)],
        }
    }

    /// The generalization the exercise asks for: on a miss, try each
    /// distinct argument type as a target and coerce every argument into
    /// it; the first target that takes all the arguments wins.
    fn multi_apply_generic(
        table: &OpTable,
        coercions: &OpTable,
        op: &str,
        args: &[Value],
    ) -> Result<Value, SicpError> {
        if let Some(proc) = table.get(&Key::sym(op), &tag_list_key(args)?) {
            let bare: Result<Vec<Value>, SicpError> = args.iter().map(contents).collect();
            return proc(&bare?);
        }
        let mut targets: Vec<String> = Vec::new();
        for a in args {
            let t = type_tag(a)?.as_ref().to_owned();
            if !targets.contains(&t) {
                targets.push(t);
            }
        }
        for target in &targets {
            let Some(coerced) = coerce_all(coercions, args, target) else {
                continue;
            };
            if coerced.len() == args.len() {
                return multi_apply_generic(table, coercions, op, &coerced);
            }
        }
        Err(no_method(op))
    }

    /// Coerces every argument to `target`, or answers `None` when any
    /// argument's type has no coercion path there.
    fn coerce_all(coercions: &OpTable, args: &[Value], target: &str) -> Option<Vec<Value>> {
        let mut out = Vec::with_capacity(args.len());
        for a in args {
            let t = type_tag(a).ok()?;
            if t.as_ref() == target {
                out.push(a.clone());
                continue;
            }
            let coerce = coercions.get(&Key::sym(t.as_ref()), &Key::sym(target))?;
            out.push(coerce(std::slice::from_ref(a)).ok()?);
        }
        Some(out)
    }

    /// The three-argument complex sum the demo table offers.
    fn install_add3(table: &OpTable) {
        let key = Key::pair(
            Key::sym("complex"),
            Key::pair(
                Key::sym("complex"),
                Key::pair(Key::sym("complex"), Key::Nil),
            ),
        );
        let add3: Handler = Rc::new(|args: &[Value]| {
            let re = real_of(&args[0])? + real_of(&args[1])? + real_of(&args[2])?;
            let im = imag_of(&args[0])? + imag_of(&args[1])? + imag_of(&args[2])?;
            Ok(Value::tagged(
                "complex",
                ch02::sec_2_4::rect_make_from_real_imag_tagged(re, im),
            ))
        });
        table.put(Key::sym("add"), key, add3);
    }

    // The handler's arguments are the bare (rect- or polar-tagged)
    // complex contents; the 2.4.2 dispatch selectors read their parts.
    fn real_of(z: &Value) -> Result<f64, SicpError> {
        let v = real_part_dispatch(z)?;
        let Value::Real(x) = v else {
            return Err(SicpError::TypeMismatch("complex part".into()));
        };
        Ok(x)
    }

    fn imag_of(z: &Value) -> Result<f64, SicpError> {
        let v = imag_part_dispatch(z)?;
        let Value::Real(x) = v else {
            return Err(SicpError::TypeMismatch("complex part".into()));
        };
        Ok(x)
    }

    /// The demo: a mixed triple added through the strategy, and the case
    /// the strategy cannot reach.
    pub fn ex_2_82() -> Result<(String, String, String), SicpError> {
        let table = Rc::new(OpTable::new());
        install_generic_arithmetic(&table)?;
        install_add3(&table);
        let coercions = OpTable::new();
        for from in ["integer", "real"] {
            put_coercion(
                &coercions,
                from,
                "complex",
                Rc::new(|args: &[Value]| ch02::sec_2_5::number_to_complex(&args[0])),
            );
        }
        put_coercion(
            &coercions,
            "rational",
            "complex",
            Rc::new(|args: &[Value]| {
                let r = contents(&args[0])?;
                let Value::Pair(cell) = &r else {
                    return Err(SicpError::TypeMismatch("rational".into()));
                };
                let (Value::Int(n), Value::Int(d)) =
                    (cell.car.borrow().clone(), cell.cdr.borrow().clone())
                else {
                    return Err(SicpError::TypeMismatch("rational".into()));
                };
                #[expect(
                    clippy::cast_precision_loss,
                    reason = "the inexact promotion the rational-to-complex coercion performs"
                )]
                let x = n as f64 / d as f64;
                Ok(Value::tagged(
                    "complex",
                    ch02::sec_2_4::rect_make_from_real_imag_tagged(x, 0.0),
                ))
            }),
        );

        // 1 + 1/2 + (1 + i): every argument reaches `complex`, and the
        // three-way entry answers.
        let triple = multi_apply_generic(
            &table,
            &coercions,
            "add",
            &[
                Value::Int(1),
                make_rational(&table, 1, 2)?,
                make_complex_from_real_imag(&table, 1.0, 1.0)?,
            ],
        )?;

        // The limit: 1 + 1/2 has no (integer, rational) entry, and
        // both single-target attempts fail because neither argument's own
        // type is `complex` — the common supertype is not among the
        // targets the strategy ever tries.
        let limited = multi_apply_generic(
            &table,
            &coercions,
            "add",
            &[Value::Int(1), make_rational(&table, 1, 2)?],
        )
        .expect_err("the strategy cannot find the common supertype");
        // The failure must be the dispatch miss the strategy's story is
        // about, not a crash inside some handler.
        if !matches!(&limited, SicpError::UserRaised { .. }) {
            return Err(SicpError::TypeMismatch(
                "the limit must fail as a dispatch miss".into(),
            ));
        }

        // A direct mixed-type entry, had one been written, would have
        // been found by the first lookup — the strategy never skips an
        // exact match, only ever fails to invent a third type.
        let direct = multi_apply_generic(
            &table,
            &coercions,
            "add",
            &[
                make_complex_from_real_imag(&table, 2.0, 3.0)?,
                Value::Int(4),
            ],
        )?;

        Ok((triple.to_string(), limited.to_string(), direct.to_string()))
    }
}

#[test]
fn ex_2_82() {
    let (triple, _limited, direct) = ex_2_82::ex_2_82().expect("2.82 answers");
    assert_eq!(triple, "(complex (rectangular (2.5 . 1)))");
    assert_eq!(direct, "(complex (rectangular (6 . 3)))");
}
