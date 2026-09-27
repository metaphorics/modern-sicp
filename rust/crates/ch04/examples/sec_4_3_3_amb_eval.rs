// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.3

//! Section 4.3.3: the engine the section's implementation listings
//! describe. The choice frames, the undo trail, and the driver protocol
//! each show their book behavior: a `set!` on a branch rolls back when
//! the branch fails, `permanent-set!` survives the same unwind, and the
//! driver's try-again protocol ends every problem with the book's
//! exhaustion report.

use ch04::eval_support::{AMB_SEED, Amb, SchemeError, run_amb, setup_amb_environment};

fn main() {
    // A set! made on a branch lands on the undo trail: the failed
    // branch's mutations roll back when the search unwinds through it.
    let undoing = r"
(define (require p) (if (not p) (amb)))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define count 0)
(define log '())
(let ((x (an-element-of '(1 2))))
  (set! count (+ count 1))
  (set! log (cons x log))
  (require (= x 1))
  x)";
    let (values, _) = run_amb(&Amb::new(AMB_SEED).expect("the seed is nonzero"), undoing)
        .expect("the program runs");
    println!(
        "set! answer: {}",
        sicp_runtime::print_value(values.last().expect("a value"))
    );
    // => set! answer: 1
    assert_eq!(
        sicp_runtime::print_value(values.last().expect("a value")),
        "1"
    );

    // The same search resumed: the whole failed branch unwinds, so the
    // count reads 0 again -- every set! on the path rolled back.
    let resumed = with_worker(move || {
        let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
        let env = setup_amb_environment();
        let mut report: Vec<String> = Vec::new();
        for form in sicp_runtime::read_program(undoing).expect("parses") {
            let _ = amb.run_form(&form, &env);
        }
        report.push(exhausted(&amb));
        let probe = amb.run("count", &env).expect("count is defined");
        report.push(sicp_runtime::print_value(&probe));
        report
    });
    println!("set! trail: {resumed:?}");
    // => set! trail: ["exhausted", "0"]
    assert_eq!(resumed, vec!["exhausted".to_owned(), "0".to_owned()]);

    // Exercise 4.51's permanent-set! skips the trail: the failed
    // branch's increment survives the unwind, and the probe reads both
    // trials -- the answer's and the failed retry's.
    let permanent = r"
(define (require p) (if (not p) (amb)))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define count 0)
(let ((x (an-element-of '(1 2))))
  (permanent-set! count (+ count 1))
  (require (= x 1))
  x)";
    let survived = with_worker(move || {
        let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
        let env = setup_amb_environment();
        let mut report: Vec<String> = Vec::new();
        for form in sicp_runtime::read_program(permanent).expect("parses") {
            let _ = amb.run_form(&form, &env);
        }
        report.push(exhausted(&amb));
        let probe = amb.run("count", &env).expect("count is defined");
        report.push(sicp_runtime::print_value(&probe));
        report
    });
    println!("permanent-set! trail: {survived:?}");
    // => permanent-set! trail: ["exhausted", "2"]
    assert_eq!(survived, vec!["exhausted".to_owned(), "2".to_owned()]);

    // The driver's try-again with no problem in flight reports exactly
    // that, and an (amb) with no choices fails immediately.
    let driver = with_worker(move || {
        let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
        let env = setup_amb_environment();
        let no_problem = matches!(amb.try_again(), Err(SchemeError::Backtrack));
        let empty = matches!(amb.run("(amb)", &env), Err(SchemeError::Backtrack));
        vec![no_problem, empty]
    });
    println!("driver protocol: {driver:?}");
    // => driver protocol: [true, true]
    assert_eq!(driver, vec![true, true]);
    let _ = resumed;
}

/// Whether the problem in flight has no answer left, the driver's
/// exhaustion signal.
fn exhausted(amb: &Amb) -> String {
    match amb.try_again() {
        Err(SchemeError::Backtrack) => String::from("exhausted"),
        _ => String::from("answer"),
    }
}

/// Runs `job` on the 256 MiB worker stack the 4.1 substrate provides:
/// the search nests the host stack a few hundred evaluation frames
/// deep, and the worker is the edition's stated bound.
fn with_worker<T: Send + 'static>(job: impl FnOnce() -> T + Send + 'static) -> T {
    ch04::sec_4_1::with_eval_stack(job)
}
