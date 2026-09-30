// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.4

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.4: `and` and `or` as derived forms of the kernel, evaluated
 * directly so the operands re-enter the full evaluator at every depth.
 * Short circuit is the specification: the first false operand stops `and`
 * at false, the first true value is `or`'s answer, and a later operand
 * that would fault never evaluates. `and()` is true, `or()` is false,
 * `and(1, 2, 3)` is 3, and `and(1, false, 1 / 0)` is false with no fault
 * raised.
 *
 * Expected: the and probe prints `true`, `true`, `3`, `false`; the or
 * probe prints `true`, `false`, `7`, `a`.
 */
public fun andExamplesTranscript(): String = throw PendingSolution()

/** The book's `or` examples on the same probe family. */
public fun orExamplesTranscript(): String = throw PendingSolution()

/** The short-circuit pins: `and(false, x)` is false and `or("b", "c")`
 * answers `b` without its second operand. => "false\nb\n" */
public fun shortCircuitTranscript(): String = throw PendingSolution()

/** The forms re-enter the whole evaluator, nested:
 * `and(or(false, 2), if(true, 40, 0), 1 + 1)` answers 2. => "2\n" */
public fun nestedFormsTranscript(): String = throw PendingSolution()
