// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1.1: the core of the evaluator. The `eval`/`apply`
//! interplay runs a small program: a definition binds a procedure, an
//! application evaluates the operator and the operands and calls
//! `apply_procedure`, an assignment rebinds, and a conditional branches
//! on the object language's truth.

use std::rc::Rc;

use ch04::sec_4_1::{Base, Evaluator, eval_program, setup_environment, text_of_quotation};
use sicp_runtime::{Env, Value, print_value, read, read_program};

fn main() {
    let env = setup_environment();

    // The core forms: define answers ok, the call answers the product,
    // set! rebinds, and a wrong-kind operand raises through the same
    // error channel.
    let values = eval_program(
        &env,
        "(define (square x) (* x x))\n(square 21)\n(square 1.5)\n(set! square 3)\n",
    )
    .expect("runs");
    for value in &values {
        println!("{}", print_value(value));
    }
    // => ok
    // => 441
    // => 2.25
    // => ok
    assert_eq!(
        values.iter().map(print_value).collect::<Vec<_>>(),
        vec!["ok", "441", "2.25", "ok"]
    );

    // Apply classifies its procedure: the primitive calls its handler,
    // the compound procedure extends its captured environment.
    let program = read_program("(define (twice f x) (apply f (list x x)))").expect("read");
    Base.eval(&program[0], &env).expect("defines");
    let call = read("(twice (lambda (a b) (+ a b)) 4)").expect("read");
    let answer = Base.eval(&call, &env).expect("applies");
    println!("{}", print_value(&answer));
    // => 8
    assert_eq!(answer, Value::int(8));

    // A quotation's value is its datum, unevaluated.
    let quoted = read("'(a b c)").expect("read");
    let datum = text_of_quotation(&quoted).expect("quoted");
    println!("{}", print_value(&datum));
    // => (a b c)
    assert_eq!(print_value(&datum), "(a b c)");

    // The evaluator's `if` tests the object language's truth: only the
    // explicit false object is false.
    let env2 = Env::child(&Rc::clone(&env));
    let program = read_program("(if 0 'zero-is-true 'zero-is-false)").expect("read");
    let answer = Base.eval(&program[0], &Rc::clone(&env2)).expect("runs");
    println!("{}", print_value(&answer));
    // => zero-is-true
    assert_eq!(print_value(&answer), "zero-is-true");
}
