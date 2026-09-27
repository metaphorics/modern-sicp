// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.21

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.21: recursion without `define`. Part (a): the statement's
 * expression applies an anonymous procedure that rebuilds `fact` on every
 * call from its own self-application, `(fact fact n)`, and so computes
 * factorials with no `define` and no name to recurse through; the same
 * trick drives fib, whose recursive step calls `(fb fb (- k 1))` and
 * `(fb fb (- k 2))`. Part (b) fills the statement's blanks for the mutual
 * even?/odd? pair: each procedure receives both procedures plus the count,
 * so the steps are `(od? ev? od? (- k 1))` and `(ev? ev? od? (- k 1))`.
 *
 * Expected answers: the statement's expression answers 3628800 on 10; the
 * fib analog answers 55 on 10; `(f 7)` answers `#f` and `(f 10)` answers
 * `#t`.
 */
public fun selfApplicationFactTranscript(): String = throw PendingSolution()

public fun selfApplicationFibTranscript(): String = throw PendingSolution()

public fun mutualEvenOddWithoutDefineTranscript(): String = throw PendingSolution()
