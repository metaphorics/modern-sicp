// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.21

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.21: recursion without a named binding. Part (a): the
 * statement's expression applies an anonymous procedure that rebuilds
 * `fact` on every call from its own self-application, and so computes
 * factorials with no name to recurse through; the same trick drives fib,
 * whose recursive step calls the two self-applications. Part (b) fills
 * the statement's blanks for the mutual even?/odd? pair: each procedure
 * receives both procedures plus the count, so each step calls its
 * partner with the decremented count.
 *
 * Expected answers: the factorial expression answers 3628800 on 10; the
 * fib analog answers 55 on 10; the mutual probe answers `false` on 7 and
 * `true` on 10.
 */
public fun selfApplicationFactTranscript(): String = throw PendingSolution()

/** The fib analog of the trick. => "55\n" */
public fun selfApplicationFibTranscript(): String = throw PendingSolution()

/** The mutual even?/odd? pair without named bindings.
 * => "false\ntrue\n" */
public fun mutualEvenOddWithoutDefineTranscript(): String = throw PendingSolution()
