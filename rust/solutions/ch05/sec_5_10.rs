// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.10: a new surface syntax
//! installed as one translator feeding the untouched assembler.

use ch05::sec_5_2::{Fault, make_machine_from_datums, op};
use sicp_runtime::read_program;

use sicp_runtime::{Value, Value as V};

/// The book's GCD controller, for structural comparison with the
/// translation.
const GCD_BOOK_CONTROLLER: &str = "
test-b
  (test (op =) (reg b) (const 0))
  (branch (label gcd-done))
  (assign t (op rem) (reg a) (reg b))
  (assign a (reg b))
  (assign b (reg t))
  (goto (label test-b))
gcd-done";

/// The same controller in the new syntax: bare symbols name
/// registers, bare numbers are constants, `test` and `perform` name
/// their operation bare in head position, and `branch` and `goto`
/// take a bare label. Label lines are unchanged.
const GCD_NEW_CONTROLLER: &str = "
test-b
(test = b 0)
(branch gcd-done)
(assign t rem a b)
(assign a b)
(assign b t)
(goto test-b)
gcd-done";

/// The Fibonacci controller in the new syntax: the full stack
/// discipline survives the shallower grammar.
const FIB_NEW_CONTROLLER: &str = "
(assign continue (label fib-done))
fib-loop
(test < n 2)
(branch immediate-answer)
(save continue)
(assign continue (label afterfib-n-1))
(save n)
(assign n - n 1)
(goto fib-loop)
afterfib-n-1
(restore n)
(restore continue)
(assign n - n 2)
(save continue)
(assign continue (label afterfib-n-2))
(save val)
(goto fib-loop)
afterfib-n-2
(assign n val)
(restore val)
(restore continue)
(assign val + val n)
(goto (reg continue))
immediate-answer
(assign val n)
(goto (reg continue))
fib-done";

/// Translates a controller in the new syntax into the book's datums.
fn translate(controller: &str) -> Result<Vec<V>, Fault> {
    let lines = read_program(controller).map_err(|error| Fault::Parse(error.to_string()))?;
    Ok(lines.iter().map(translate_line).collect())
}

/// Translates one instruction line; labels and anything the new
/// syntax does not reshape pass through unchanged.
fn translate_line(datum: &V) -> V {
    let Ok(items) = datum.list_items() else {
        return datum.clone();
    };
    let Some(V::Sym(head)) = items.first() else {
        return datum.clone();
    };
    match head.as_ref() {
        "branch" | "goto" => translated_jump(&items),
        "assign" => translated_assign(&items),
        "test" | "perform" => V::list(
            std::iter::once(items[0].clone())
                .chain(std::iter::once(operation_head(&items[1])))
                .chain(items[2..].iter().map(operand))
                .collect(),
        ),
        _ => datum.clone(),
    }
}

/// Reshapes an assign: a bare operation name in head position with
/// operands (`(assign t rem a b)`) becomes an `(op ...)` source; a
/// lone bare symbol (`(assign n val)`) stays a register source; and
/// wrapped `(reg ...)`, `(const ...)`, `(label ...)`, and `(op ...)`
/// sources pass through with their operands translated.
fn translated_assign(items: &[V]) -> V {
    let source = &items[2];
    let translated = match (source, items.len()) {
        (V::Sym(_), 3) => operand(source),
        (V::Sym(name), _) => V::list(vec![V::sym("op"), V::sym(name)]),
        _ => operand(source),
    };
    V::list(
        items[..2]
            .iter()
            .cloned()
            .chain(std::iter::once(translated))
            .chain(items[3..].iter().map(operand))
            .collect(),
    )
}

/// Reshapes `(branch there)` into `(branch (label there))`; a
/// compound target such as `(reg continue)` is already book-shaped.
fn translated_jump(items: &[V]) -> V {
    match &items[1] {
        V::Sym(target) => V::list(vec![
            items[0].clone(),
            V::list(vec![V::sym("label"), V::sym(target)]),
        ]),
        _ => V::list(items.to_vec()),
    }
}

/// The head of a `test` or `perform` body: a bare operation name
/// becomes `(op name)`; an `(op ...)` list passes through.
fn operation_head(datum: &V) -> V {
    match datum {
        V::Sym(name) => V::list(vec![V::sym("op"), V::sym(name)]),
        other => other.clone(),
    }
}

/// One operand of the new syntax: a bare symbol names a register, a
/// self-evaluating datum is a constant, an `(op ...)` application is
/// translated recursively, and a wrapped `(reg ...)`, `(const ...)`,
/// or `(label ...)` passes through.
fn operand(datum: &V) -> V {
    match datum {
        V::Sym(name) => V::list(vec![V::sym("reg"), V::sym(name)]),
        V::Int(_) | V::Real(_) | V::Bool(_) | V::Str(_) => {
            V::list(vec![V::sym("const"), datum.clone()])
        }
        list @ (V::Pair(_) | V::Nil) => {
            let Ok(items) = list.list_items() else {
                return list.clone();
            };
            match items.first() {
                Some(V::Sym(tag)) if tag.as_ref() == "op" => V::list(
                    items[..2]
                        .iter()
                        .cloned()
                        .chain(items[2..].iter().map(operand))
                        .collect(),
                ),
                _ => list.clone(),
            }
        }
        other => other.clone(),
    }
}

fn gcd_operations() -> Vec<(&'static str, ch05::sec_5_2::OpHandler)> {
    vec![("=", op("=").unwrap()), ("rem", op("rem").unwrap())]
}

fn fib_operations() -> Vec<(&'static str, ch05::sec_5_2::OpHandler)> {
    vec![
        ("<", op("<").unwrap()),
        ("+", op("+").unwrap()),
        ("-", op("-").unwrap()),
    ]
}

mod ex_5_10 {
    //! Exercise 5.10: design a new syntax and modify the simulator to
    //! use it, changing only the syntax procedures.

    use super::*;

    /// The translation of the new-syntax GCD controller is
    /// indistinguishable from the book-syntax controller read
    /// directly: the assembler and the machine never learn a new
    /// syntax existed.
    #[test]
    fn ex_5_10_translation_matches_the_book_datums() {
        let translated = translate(GCD_NEW_CONTROLLER).unwrap();
        let book = read_program(GCD_BOOK_CONTROLLER).unwrap();
        assert_eq!(translated, book);
    }

    /// The translated machine runs: the GCD session of the text
    /// answers 2 on 206 and 40.
    #[test]
    fn ex_5_10_translated_gcd_runs() {
        let datums = translate(GCD_NEW_CONTROLLER).unwrap();
        let mut machine =
            make_machine_from_datums(&["a", "b", "t"], &gcd_operations(), &datums).unwrap();
        machine.set_register("a", Value::Int(206)).unwrap();
        machine.set_register("b", Value::Int(40)).unwrap();
        machine.start().unwrap();
        assert_eq!(machine.get_register("a").unwrap(), Value::Int(2));
    }

    /// The full Fibonacci controller in the new syntax runs on the
    /// untouched machine: 6 answers 8.
    #[test]
    fn ex_5_10_translated_fibonacci_runs() {
        let datums = translate(FIB_NEW_CONTROLLER).unwrap();
        let mut machine =
            make_machine_from_datums(&["n", "val", "continue"], &fib_operations(), &datums)
                .unwrap();
        machine.set_register("n", Value::Int(6)).unwrap();
        machine.start().unwrap();
        assert_eq!(machine.get_register("val").unwrap(), Value::Int(8));
    }
}
