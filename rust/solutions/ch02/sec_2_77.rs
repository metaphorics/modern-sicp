// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.77.

mod ex_2_77 {
    use std::cell::Cell;
    use std::rc::Rc;

    use ch02::sec_2_5::{
        contents, install_generic_arithmetic, make_complex_from_real_imag, type_tag,
    };
    use sicp_runtime::{Handler, Key, OpTable, SicpError, Value};

    /// The table key for an argument's tag list.
    fn tag_list_key(args: &[Value]) -> Result<Key, SicpError> {
        let mut key = Key::Nil;
        for a in args.iter().rev() {
            key = Key::pair(Key::Sym(type_tag(a)?), key);
        }
        Ok(key)
    }

    /// The section's `apply_generic`, instrumented to count every
    /// invocation so the trace question has a number attached.
    fn counting_apply_generic(
        table: &OpTable,
        count: &Cell<u32>,
        op: &str,
        args: &[Value],
    ) -> Result<Value, SicpError> {
        count.set(count.get() + 1);
        let Some(proc) = table.get(&Key::sym(op), &tag_list_key(args)?) else {
            return Err(SicpError::UserRaised {
                message: "No method for these types".into(),
                irritants: vec![Value::sym(op)],
            });
        };
        let bare: Result<Vec<Value>, SicpError> = args.iter().map(contents).collect();
        proc(&bare?)
    }

    const SELECTORS: [&str; 4] = ["real-part", "imag-part", "magnitude", "angle"];

    /// Alyssa's fix: the four selectors exported under `(complex)`. Each
    /// entry is the *generic* selector of the same name, so the inner
    /// rectangular or polar tag is peeled by one more dispatch — which is
    /// exactly what makes the count come out at two.
    pub fn install_complex_selectors(table: &Rc<OpTable>, count: &Rc<Cell<u32>>) {
        let complex_key = Key::pair(Key::sym("complex"), Key::Nil);
        for op in SELECTORS {
            let t = Rc::clone(table);
            let c = Rc::clone(count);
            let handler: Handler =
                Rc::new(move |args: &[Value]| counting_apply_generic(&t, &c, op, args));
            table.put(Key::sym(op), complex_key.clone(), handler);
        }
    }

    /// Evaluates the magnitude of the Figure 2.24 object and returns
    /// the answer with the number of `apply_generic` invocations.
    pub fn ex_2_77() -> Result<(String, u32), SicpError> {
        let table = Rc::new(OpTable::new());
        install_generic_arithmetic(&table)?;
        let count = Rc::new(Cell::new(0));
        install_complex_selectors(&table, &count);
        let z = make_complex_from_real_imag(&table, 3.0, 4.0)?;
        let value = counting_apply_generic(&table, &count, "magnitude", &[z])?;
        Ok((value.to_string(), count.get()))
    }
}

#[test]
fn ex_2_77() {
    let (value, count) = ex_2_77::ex_2_77().expect("magnitude works after the fix");
    assert_eq!(value, "5");
    assert_eq!(count, 2);
}
