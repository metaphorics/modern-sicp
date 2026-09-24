// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1.2: representing expressions. The evaluator classifies a
//! form by its head symbol, the way `tagged-list?` does in the book,
//! and the derived-expression rewrite turns a `cond` into a nest of
//! `if` expressions before the evaluator ever sees it.

use ch04::sec_4_1::{
    cond_to_if, eval_program, is_assignment, is_cond, is_quoted, setup_environment,
};
use sicp_runtime::{print_value, read};

fn main() {
    // The syntax predicates recognize the special forms by their tags.
    let quoted = read("(quote (a b))").expect("read");
    let assignment = read("(set! x 3)").expect("read");
    let cond = read("(cond ((> x 0) x) ((= x 0) (display 'zero) 0) (else (- x)))").expect("read");
    println!(
        "quoted={} assignment={} cond={}",
        is_quoted(&quoted),
        is_assignment(&assignment),
        is_cond(&cond)
    );
    // => quoted=true assignment=true cond=true
    assert!(is_quoted(&quoted));
    assert!(is_assignment(&assignment));
    assert!(is_cond(&cond));

    // The derived-expression rewrite the section shows.
    let rewritten = cond_to_if(&cond).expect("rewrites");
    println!("{}", print_value(&rewritten));
    // => (if (> x 0) x (if (= x 0) (begin (display (quote zero)) 0) (- x)))
    assert_eq!(
        print_value(&rewritten),
        "(if (> x 0) x (if (= x 0) (begin (display (quote zero)) 0) (- x)))"
    );

    // The rewrite evaluates the same as the original: one branch runs.
    let env = setup_environment();
    let program =
        read("(let ((x 0)) (cond ((> x 0) 'pos) ((= x 0) 'zero) (else 'neg)))").expect("read");
    let answer = ch04::sec_4_1::eval(&program, &env).expect("evaluates");
    println!("{}", print_value(&answer));
    // => zero
    assert_eq!(print_value(&answer), "zero");

    // Evaluating the rewritten form directly gives the same answer.
    let program = read("(let ((x 0)) (if (> x 0) 'pos (if (= x 0) 'zero 'neg)))").expect("read");
    let answer = ch04::sec_4_1::eval(&program, &env).expect("evaluates");
    println!("{}", print_value(&answer));
    // => zero
    assert_eq!(print_value(&answer), "zero");
    assert!(eval_program(&env, "(if #f #f #f)").is_ok());
}
