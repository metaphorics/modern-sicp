// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1.3: evaluator data structures. Frames bind names to
//! values, a procedure object carries its parameters, body, and the
//! environment of definition, and the four environment operations walk
//! the chain from the newest frame outwards.

use std::rc::Rc;

use ch04::sec_4_1::{
    define_variable_, eval_program, extend_environment, lookup_variable_value, set_variable_value_,
    setup_environment,
};
use sicp_runtime::{SchemeError, Symbol, Value, print_value};

fn main() {
    // A new frame over the global one, the book's `extend-environment`
    // with the parameters `x` and `y` bound to 3 and 4.
    let global = setup_environment();
    let frame = extend_environment(
        "demo",
        &[Symbol::from("x"), Symbol::from("y")],
        None,
        &[Value::int(3), Value::int(4)],
        &global,
    )
    .expect("extends");

    // Lookup walks the chain: x is in the new frame, and the frame
    // extends to the primitives beyond it.
    println!(
        "{}",
        print_value(&lookup_variable_value("x", &frame).expect("bound"))
    );
    // => 3
    assert_eq!(lookup_variable_value("x", &frame), Ok(Value::int(3)));
    println!(
        "{}",
        print_value(&lookup_variable_value("+", &frame).expect("bound"))
    );
    // => #[primitive-procedure +]
    assert!(matches!(
        lookup_variable_value("+", &frame),
        Ok(Value::Primitive { .. })
    ));

    // set! changes the nearest existing binding without creating one;
    // define creates or rebinds in the newest frame only.
    set_variable_value_("x", Value::int(30), &frame).expect("sets");
    define_variable_("y", Value::int(40), &frame).expect("defines");
    println!(
        "{}",
        print_value(&lookup_variable_value("x", &frame).expect("bound"))
    );
    // => 30
    assert_eq!(lookup_variable_value("x", &frame), Ok(Value::int(30)));

    // The global frame is untouched by the inner define of y.
    let Err(error) = lookup_variable_value("y", &global) else {
        panic!("y must stay unbound in the outer frame");
    };
    println!("{error}");
    // => unbound variable: y
    assert!(matches!(error, SchemeError::UnboundVariable(_)));

    // A compound procedure object prints by its definition name, its
    // environment part kept out of sight.
    let values = eval_program(&Rc::clone(&global), "(define (f x) x)\nf").expect("runs");
    println!("{}", print_value(&values[1]));
    // => #[compound-procedure f]
    assert_eq!(print_value(&values[1]), "#[compound-procedure f]");
}
