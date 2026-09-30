// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
// Chapter 4, exercise 4.8

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.8: named `let`. The derived rewrite turns
 * `let loop(bindings) { body }` into a `GLetRec` that binds the loop's
 * lambda before the initial call, so the body's recursive self-calls
 * resolve the name in that recursive frame. The loop name stays local,
 * so an outer binding of the same name survives; a plain `let` still
 * evaluates.
 *
 * Expected: the book's named-let Fibonacci over 10 answers 55; the probe
 * with an outer name answers 1 from the loop and then reads 7 outside;
 * the plain-let probe answers 3.
 */
public fun namedLetFibonacciTranscript(): String = throw PendingSolution()

/** The loop name stays local to the wrapper frame. => "1\n7\n" */
public fun loopNameLocalTranscript(): String = throw PendingSolution()

/** Plain let still evaluates. => "3\n" */
public fun plainLetTranscript(): String = throw PendingSolution()
