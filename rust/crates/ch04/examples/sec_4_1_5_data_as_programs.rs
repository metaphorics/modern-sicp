// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1.5: data as programs. The evaluator is fed the
//! definition of `factorial` as data and emulates the factorial
//! machine; the same evaluator runs any other machine description we
//! feed it, which is what makes it a universal machine.

use ch04::sec_4_1::run_program;

fn main() {
    // The factorial program as the description of a machine: feed it
    // to the evaluator and the evaluator computes factorials.
    let output = run_program(
        "(define (factorial n)\n  (if (= n 1)\n      1\n      (* (factorial (- n 1)) n)))\n(factorial 6)\n(factorial 10)\n",
    );
    println!("{output}");
    // => 720
    // => 3628800
    assert_eq!(output, "720\n3628800\n");

    // The same evaluator emulates a different machine from data: the
    // Fibonacci machine of section 1.2.2.
    let output = run_program(
        "(define (fib n)\n  (cond ((= n 0) 0)\n        ((= n 1) 1)\n        (else (+ (fib (- n 1)) (fib (- n 2))))))\n(fib 12)\n",
    );
    println!("{output}");
    // => 144
    assert_eq!(output, "144\n");

    // And the object language's own data structures are the host's:
    // a quoted expression is data the evaluator can take apart.
    let output = run_program("(car '(factorial 6))\n(cdr '(factorial 6))\n");
    println!("{output}");
    // => factorial
    // => (6)
    assert_eq!(output, "factorial\n(6)\n");
}
