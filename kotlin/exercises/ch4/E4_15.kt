// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.15

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

// Exercise 4.15: can `halts?` be written in the evaluator? The solution's
// `Bounded` evaluator answers honestly what a real `halts?` cannot: it
// carries a step budget in `step`, and a run that outlives its budget
// dies with the typed `MachineFault` instead of hanging. The book's
// dialogue -- `(define (run-forever) (run-forever))`,
// `(define (try p) (if (halts? p p) (run-forever) 'halts))`,
// `(try try)` -- runs with `halts?` installed as a primitive the host
// controls, a stub whose canned answer the caller flips. With `#f` the
// run terminates printing `halts` (the program halted though `halts?`
// denied it); with `#t` the `'run-forever` branch spins until the budget
// fires (the program ran on though `halts?` affirmed it). Either canned
// answer is wrong about this input: the diagonal contradiction, stated
// with the budget as the witness instead of an infinite loop.

/** The book's halts? dialogue on the bounded evaluator: `#f` halts,
 * `#t` exhausts the budget. => "halts\n" / the machine-fault line. */
public fun boundedTranscript(haltsAnswer: Boolean): String = throw PendingSolution()
