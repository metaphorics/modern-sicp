// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.3

//! Section 4.3.1: `amb` and search. Each interaction is the book's
//! driver session: `amb` picks alternatives depth-first, `require`
//! rejects a line of attack by failing, and `try-again` at the driver
//! resumes the deepest pending choice for the next answer.

use ch04::eval_support::amb_session;

const LIBRARY_LINES: &[&str] = &[
    "(define (require p) (if (not p) (amb)))",
    "(define (an-element-of items) \
     (require (not (null? items))) \
     (amb (car items) (an-element-of (cdr items))))",
    "(define (an-integer-starting-from n) \
     (amb n (an-integer-starting-from (+ n 1))))",
];

fn main() {
    // The six possible values of the section's two-choice expression,
    // then the exhaustion the book's driver reports.
    let tries = ["try-again"; 7];
    let session = amb_session(
        &LIBRARY_LINES
            .iter()
            .copied()
            .chain(["(list (amb 1 2 3) (amb 'a 'b))"])
            .chain(tries)
            .collect::<Vec<_>>(),
    );
    println!("{session}");
    // => ;;; Amb-Eval value: (1 a) ... (3 b)
    for value in ["(1 a)", "(1 b)", "(2 a)", "(2 b)", "(3 a)", "(3 b)"] {
        assert!(session.contains(&format!(";;; Amb-Eval value: {value}\n")));
    }
    assert!(session.contains(
        ";;; There are no more values of\n(list (amb 1 2 3) (amb (quote a) (quote b)))\n"
    ));

    // The book's driver sample: each try-again yields the next pair
    // whose sum is prime, then the search runs dry and a new problem
    // answers afresh.
    let session = {
        let mut lines: Vec<&str> = LIBRARY_LINES.to_vec();
        lines.push(
            "(define (prime? n) \
             (define (smallest-divisor test) \
             (if (> (* test test) n) n \
             (if (= (remainder n test) 0) test \
             (smallest-divisor (+ test 1))))) \
             (= (smallest-divisor 2) n))",
        );
        lines.push(
            "(define (prime-sum-pair list1 list2) \
             (let ((a (an-element-of list1)) (b (an-element-of list2))) \
             (require (prime? (+ a b))) \
             (list a b)))",
        );
        lines.push("(prime-sum-pair '(1 3 5 8) '(20 35 110))");
        lines.push("try-again");
        lines.push("try-again");
        lines.push("try-again");
        lines.push("(prime-sum-pair '(19 27 30) '(11 36 58))");
        amb_session(&lines)
    };
    println!("{session}");
    assert!(session.contains(";;; Amb-Eval value: (3 20)\n"));
    assert!(session.contains(";;; Amb-Eval value: (3 110)\n"));
    assert!(session.contains(";;; Amb-Eval value: (8 35)\n"));
    assert!(session.contains(
        ";;; There are no more values of\n(prime-sum-pair (quote (1 3 5 8)) (quote (20 35 110)))\n"
    ));
    assert!(session.contains(";;; Amb-Eval value: (30 11)\n"));

    // Infinite ranges: each try-again draws the next integer.
    let session = {
        let mut lines: Vec<&str> = LIBRARY_LINES.to_vec();
        lines.push("(an-integer-starting-from 3)");
        lines.push("try-again");
        lines.push("try-again");
        amb_session(&lines)
    };
    println!("{session}");
    assert!(session.ends_with(";;; Amb-Eval value: 5\n"));
}
