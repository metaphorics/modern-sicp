// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1.4: running the evaluator as a program. The global
//! environment holds the primitive procedures under their
//! object-language names, and a driver loop reads forms, evaluates
//! them, and prints the results behind the book's prompts.

use ch04::sec_4_1::{driver_transcript, eval_program, run_program, setup_environment};
use sicp_runtime::print_value;

fn main() {
    // The global environment answers the primitives by name.
    let env = setup_environment();
    let values = eval_program(&env, "(+ 1 6)\n(+ 1 (* 2 3))\ncar").expect("runs");
    for value in &values {
        println!("{}", print_value(value));
    }
    // => 7
    // => 7
    // => #[primitive-procedure car]
    assert_eq!(
        values.iter().map(print_value).collect::<Vec<_>>(),
        vec!["7", "7", "#[primitive-procedure car]"]
    );

    // The driver loop's sample session: the book's append definition
    // and one call, prompts included.
    let transcript = driver_transcript(&[
        "(define (append x y) (if (null? x) y (cons (car x) (append (cdr x) y))))",
        "(append '(a b c) '(d e f))",
        "(cons 'x '(y z))",
    ]);
    println!("{transcript}");
    // => ;;; M-Eval input: (define (append x y) ...)
    // => ;;; M-Eval value: ok
    // => ;;; M-Eval input: (append '(a b c) '(d e f))
    // => ;;; M-Eval value: (a b c d e f)
    // => ;;; M-Eval input: (cons 'x '(y z))
    // => ;;; M-Eval value: (x y z)
    assert!(transcript.contains(";;; M-Eval value: (a b c d e f)"));
    assert!(transcript.ends_with(";;; M-Eval value: (x y z)\n"));

    // A whole program runs the same way with printer.md output: a
    // definition prints nothing, an error prints one line and stops.
    let output = run_program("(define (square x) (* x x))\n(square 6)\n(display 'done)\n");
    println!("{output}");
    // => 36
    // => done
    assert_eq!(output, "36\ndone");
}
