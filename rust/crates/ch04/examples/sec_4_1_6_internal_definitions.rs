// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1.6: internal definitions. Definitions execute in
//! sequence, each extending the newest frame one name at a time; the
//! section's point is that this gives the same result as simultaneous
//! definition whenever the internal definitions come first and their
//! value expressions do not read the names being defined, as in the
//! mutually recursive `even?`/`odd?` procedures.

use ch04::sec_4_1::run_program;

fn main() {
    // Mutual recursion works, for the accidental reason the section
    // names: no call runs before both names are defined.
    let output = run_program(
        "(define (f x)\n  (define (even? n) (if (= n 0) true (odd? (- n 1))))\n  (define (odd? n) (if (= n 0) false (even? (- n 1))))\n  (even? x))\n(f 10)\n(f 7)\n",
    );
    println!("{output}");
    // => #t
    // => #f
    assert_eq!(output, "#t\n#f\n");

    // A name read before its define runs is simply unbound: the
    // sequential mechanism's one visible difference from simultaneous
    // definition.
    let output = run_program("(define (g) (define a (* b 2)) (define b 3) a)\n(g)\n");
    println!("{output}");
    // => Error: unbound variable: b
    assert!(output.starts_with("Error: unbound variable: b"));

    // Internal definitions mix freely with the body's other
    // expressions, each define extending the frame in order.
    let output = run_program(
        "(define (h x)\n  (define base 100)\n  (define (add-base n) (+ base n))\n  (define scaled (* x 2))\n  (add-base scaled))\n(h 5)\n",
    );
    println!("{output}");
    // => 110
    assert_eq!(output, "110\n");
}
