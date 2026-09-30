// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.8

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.8: named `let`. The derived rewrite turns
 * `let loop(bindings) { body }` into a `GLam`/`GApp` pair: a wrapper
 * lambda binds the loop name to its own procedure and applies it to the
 * initializers, so the body's recursive self-calls resolve the name in
 * the wrapper frame. The loop name stays local -- an outer binding of the
 * same name survives -- and a plain `let` still evaluates.
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
