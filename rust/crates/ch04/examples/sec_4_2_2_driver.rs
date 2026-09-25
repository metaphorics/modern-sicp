// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.2

//! Section 4.2.2/4.2.3: the lazy driver loop. Each interaction is the
//! book's L-Eval session: `actual-value` forces every printed answer,
//! the 4.2.3 procedural pairs make streams and lists identical, and
//! `solve` runs its integrator over the delayed spine.

use ch04::eval_support::{Lazy, lazy_driver_transcript, with_eval_stack};

const TRY_SESSION: &[&str] = &["(define (try a b) (if (= a 0) 1 b))", "(try 0 (/ 1 0))"];

const UNLESS_SESSION: &[&str] = &[
    "(define (unless condition usual-value exceptional-value) \
     (if condition exceptional-value usual-value))",
    "(unless (= 0 0) (/ 1 0) (begin (display \"exception: returning 0\") 0))",
];

const LAZY_LIST_SESSION: &[&str] = &[
    "(define (cons x y) (lambda (m) (m x y)))",
    "(define (car z) (z (lambda (p q) p)))",
    "(define (cdr z) (z (lambda (p q) q)))",
    "(define (list-ref items n) \
     (if (= n 0) (car items) (list-ref (cdr items) (- n 1))))",
    "(define (map proc items) \
     (if (null? items) '() (cons (proc (car items)) (map proc (cdr items)))))",
    "(define (scale-list items factor) \
     (map (lambda (x) (* x factor)) items))",
    "(define (add-lists list1 list2) \
     (cond ((null? list1) list2) ((null? list2) list1) \
     (else (cons (+ (car list1) (car list2)) \
     (add-lists (cdr list1) (cdr list2))))))",
    "(define ones (cons 1 ones))",
    "(define integers (cons 1 (add-lists ones integers)))",
    "(list-ref integers 17)",
];

const SOLVE_SESSION: &[&str] = &[
    "(define (cons x y) (lambda (m) (m x y)))",
    "(define (car z) (z (lambda (p q) p)))",
    "(define (cdr z) (z (lambda (p q) q)))",
    "(define (list-ref items n) \
     (if (= n 0) (car items) (list-ref (cdr items) (- n 1))))",
    "(define (map proc items) \
     (if (null? items) '() (cons (proc (car items)) (map proc (cdr items)))))",
    "(define (scale-list items factor) \
     (map (lambda (x) (* x factor)) items))",
    "(define (add-lists list1 list2) \
     (cond ((null? list1) list2) ((null? list2) list1) \
     (else (cons (+ (car list1) (car list2)) \
     (add-lists (cdr list1) (cdr list2))))))",
    "(define (integral integrand initial-value dt) \
     (define int (cons initial-value (add-lists (scale-list integrand dt) int))) int)",
    "(define (solve f y0 dt) (define y (integral dy y0 dt)) (define dy (map f y)) y)",
    "(list-ref (solve (lambda (x) x) 1 0.001) 1000)",
];

fn main() {
    // Each session runs on the 256 MiB worker stack: forcing a long
    // chain of memoized thunks is host non-tail recursion, and the
    // worker is the 4.1 postponement that keeps the book's 1000-step
    // `solve` inside its budget.
    // The section's first interaction: the armed operand never runs.
    let transcript = with_eval_stack(move || lazy_driver_transcript(&Lazy, TRY_SESSION));
    println!("{transcript}");
    // => ;;; L-Eval input: (define (try a b) (if (= a 0) 1 b))
    // => ;;; L-Eval value: ok
    // => ;;; L-Eval input: (try 0 (/ 1 0))
    // => ;;; L-Eval value: 1
    assert!(transcript.ends_with(";;; L-Eval value: 1\n"));

    // `unless` at `b` = 0: the display side effect precedes the value.
    // The object program's `display` writes no newline, so the driver's
    // value prompt follows the displayed text directly.
    let transcript = lazy_driver_transcript(&Lazy, UNLESS_SESSION);
    println!("{transcript}");
    // => exception: returning 0
    // => ;;; L-Eval value: 0
    assert!(transcript.contains("exception: returning 0;;; L-Eval value: 0\n"));

    // Streams as lazy lists: the car is delayed as well as the cdr, so
    // `ones` and `integers` define without looping.
    let transcript = lazy_driver_transcript(&Lazy, LAZY_LIST_SESSION);
    println!("{transcript}");
    // => ;;; L-Eval value: 18
    assert!(transcript.ends_with(";;; L-Eval value: 18\n"));

    // `solve` as originally intended in 3.5.4: no explicit delay. The
    // edition prints the host's full precision; the book's 2.716924 is
    // the rounded form.
    let transcript = with_eval_stack(move || lazy_driver_transcript(&Lazy, SOLVE_SESSION));
    println!("{transcript}");
    // => ;;; L-Eval value: 2.716923932235896
    assert!(transcript.ends_with(";;; L-Eval value: 2.716923932235896\n"));
}
