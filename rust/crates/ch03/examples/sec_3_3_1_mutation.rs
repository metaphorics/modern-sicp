// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.3.1

//! Section 3.3.1: mutable list structure — building a pair through its
//! mutators, the effect of `set-car!` and `set-cdr!`, `append!` against
//! `append`, a cycle, and the sharing that `set-to-wow!` exposes.

use std::rc::Rc;

use ch03::sec_3_3::{append_bang, make_cycle};
use sicp_runtime::Value;
use sicp_runtime::{Pair, cons_cell, eq_pair, set_car, set_cdr};

/// The book's `append` of 2.2.1, over the mutable pairs: it copies the
/// spine of `x`, so `x` keeps its old tail.
fn append_value(x: &Value, y: &Value) -> Value {
    match x {
        Value::Pair(cell) => {
            let rest = append_value(&cell.cdr.borrow(), y);
            Value::Pair(cons_cell(cell.car.borrow().clone(), rest))
        }
        _ => y.clone(),
    }
}

/// The book's `set-to-wow!`: replaces the `car` of the pair that `x`'s
/// first pair names, and returns `x` for printing.
fn set_to_wow(x: &Pair) -> Pair {
    let car_value = x.car.borrow().clone();
    if let Value::Pair(inner) = car_value {
        set_car(&inner, Value::sym("wow"));
    }
    Rc::clone(x)
}

/// The first pair of a `Value`, for the shapes the section mutates.
fn head_pair(v: &Value) -> Pair {
    let Value::Pair(pair) = v else {
        return cons_cell(v.clone(), Value::Nil);
    };
    Rc::clone(pair)
}

#[expect(
    clippy::similar_names,
    reason = "z1_car/z1_cdr and z2_car/z2_cdr name the two halves of the same pair; that similarity is the point of the transcript"
)]
fn main() {
    // The book's footnote way of building a pair: `cons_cell` allocates
    // the two cells, and the mutators fill them in.
    let new = cons_cell(Value::Nil, Value::Nil);
    set_car(&new, Value::sym("a"));
    set_cdr(&new, Value::sym("b"));
    let answer = Value::Pair(new);
    println!("{answer}");
    // => (a . b)
    assert_eq!(answer.to_string(), "(a . b)");

    // `x` is ((a b) c d) and `y` is (e f), as in Figure 3.12.
    let x = Value::list(vec![
        Value::list(vec![Value::sym("a"), Value::sym("b")]),
        Value::sym("c"),
        Value::sym("d"),
    ]);
    let y = Value::list(vec![Value::sym("e"), Value::sym("f")]);
    println!("{x}");
    // => ((a b) c d)
    assert_eq!(x.to_string(), "((a b) c d)");

    // (set-car! x y) replaces the car pointer of x's first pair, so x
    // now prints as ((e f) c d) and the old (a b) chain is detached.
    set_car(&head_pair(&x), y.clone());
    println!("{x}");
    // => ((e f) c d)
    assert_eq!(x.to_string(), "((e f) c d)");

    // Starting over: (set-cdr! x y) replaces the cdr pointer instead, so
    // x now prints as ((a b) e f).
    let x = Value::list(vec![
        Value::list(vec![Value::sym("a"), Value::sym("b")]),
        Value::sym("c"),
        Value::sym("d"),
    ]);
    set_cdr(&head_pair(&x), y.clone());
    println!("{x}");
    // => ((a b) e f)
    assert_eq!(x.to_string(), "((a b) e f)");

    // `append` copies; `append!` mutates. After the copying append, the
    // cdr of x is still (b); after append!, it is (b c d).
    let x = Value::list(vec![Value::sym("a"), Value::sym("b")]);
    let y = Value::list(vec![Value::sym("c"), Value::sym("d")]);
    let z = append_value(&x, &y);
    println!("{z}");
    // => (a b c d)
    assert_eq!(z.to_string(), "(a b c d)");
    let cdr_x = head_pair(&x).cdr.borrow().clone();
    println!("{cdr_x}");
    // => (b)
    assert_eq!(cdr_x.to_string(), "(b)");

    let w = append_bang(&head_pair(&x), &head_pair(&y));
    println!("{}", Value::Pair(Rc::clone(&w)));
    // => (a b c d)
    assert_eq!(Value::Pair(w).to_string(), "(a b c d)");
    let cdr_x = head_pair(&x).cdr.borrow().clone();
    println!("{cdr_x}");
    // => (b c d)
    assert_eq!(cdr_x.to_string(), "(b c d)");

    // make-cycle splices the last pair's cdr back to the first, so
    // (last-pair z) would never return. A bounded walk sees the loop:
    // three steps over (a b c) land back on the first pair.
    let z = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);
    let first = head_pair(&z);
    make_cycle(&first);
    let mut cursor = Rc::clone(&first);
    for _ in 0..3 {
        let next = cursor.cdr.borrow().clone();
        cursor = match next {
            Value::Pair(cell) => cell,
            _ => break,
        };
    }
    assert!(eq_pair(&cursor, &first));

    // Sharing: z1's car and cdr name one pair; z2's name two separate
    // pairs that happen to hold equal lists. The default printer shows
    // the same text for both.
    let x = Value::list(vec![Value::sym("a"), Value::sym("b")]);
    let z1 = cons_cell(x.clone(), x);
    println!("{}", Value::Pair(Rc::clone(&z1)));
    // => ((a b) a b)
    let x_fresh = Value::list(vec![Value::sym("a"), Value::sym("b")]);
    let y_fresh = Value::list(vec![Value::sym("a"), Value::sym("b")]);
    let z2 = cons_cell(x_fresh, y_fresh);
    println!("{}", Value::Pair(Rc::clone(&z2)));
    // => ((a b) a b)
    assert_eq!(Value::Pair(Rc::clone(&z1)).to_string(), "((a b) a b)");
    assert_eq!(Value::Pair(Rc::clone(&z2)).to_string(), "((a b) a b)");

    // set-to-wow! on z1 changes both halves, because they are one pair;
    // on z2 it changes only the car half.
    let after = set_to_wow(&z1);
    println!("{}", Value::Pair(Rc::clone(&after)));
    // => ((wow b) wow b)
    assert_eq!(
        Value::Pair(Rc::clone(&after)).to_string(),
        "((wow b) wow b)"
    );
    let after = set_to_wow(&z2);
    println!("{}", Value::Pair(Rc::clone(&after)));
    // => ((wow b) a b)
    assert_eq!(Value::Pair(after).to_string(), "((wow b) a b)");

    // The pointer test sees what the printer cannot: z1's two halves
    // are the same object, z2's are not.
    let z1_car = z1.car.borrow().clone();
    let z1_cdr = z1.cdr.borrow().clone();
    println!(
        "{}",
        Value::Bool(eq_pair(&head_pair(&z1_car), &head_pair(&z1_cdr)))
    );
    // => #t
    assert!(eq_pair(&head_pair(&z1_car), &head_pair(&z1_cdr)));
    let z2_car = z2.car.borrow().clone();
    let z2_cdr = z2.cdr.borrow().clone();
    println!(
        "{}",
        Value::Bool(eq_pair(&head_pair(&z2_car), &head_pair(&z2_cdr)))
    );
    // => #f
    assert!(!eq_pair(&head_pair(&z2_car), &head_pair(&z2_cdr)));
}
