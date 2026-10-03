// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.15

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

// Exercise 4.15: can `halts` be written in the evaluator? The bounded
// variant answers honestly what a real `halts` cannot: it carries a step
// budget, and a run that outlives its budget dies with the budget's typed
// fault instead of hanging. The book's dialogue -- define
// `runForever()` to call itself, define `try(p)` to run forever when
// `halts(p, p)` and answer `halts` otherwise, then call `try(try)` --
// runs with `halts` installed as a primitive the host controls, a stub
// whose canned answer the caller flips. With `false` the run terminates
// printing `halts` (the program halted though `halts` denied it); with
// `true` the run-forever branch spins until the thousand-step budget
// fires (the program ran on though `halts` affirmed it). Either canned
// answer is wrong about this input: the diagonal contradiction, stated
// with the budget as the witness instead of an infinite loop.

/** The book's halts dialogue on the bounded evaluator: `false` halts,
 * `true` exhausts the budget.
 * => "halts\n" / "BudgetExhausted after 1000 steps\n" */
public fun boundedTranscript(haltsAnswer: Boolean): String = throw PendingSolution()
