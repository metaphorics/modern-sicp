// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.32

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.32: the extra laziness. Under the section's procedural pairs
 * both `cons` slots delay, so `(car (cons 7 (/ 1 0)))` answers 7 and the
 * danger fires only when the `cdr` is demanded; the chapter-3 stream shape
 * computes its head at construction. The self-referential `ones` closes in
 * one step under delayed construction and dies unbound under the strict
 * constructor.
 *
 * Expected answers: `7` then `Error: division by zero`; the eager
 * constructor fails at construction; `(car ones)` answers 1; the strict
 * `ones` fails `Error: unbound variable: ones`.
 */
public fun lazyPairSlotsTranscript(): String = throw PendingSolution()

public fun eagerConstructorTranscript(): String = throw PendingSolution()

public fun onesOneStepTranscript(): String = throw PendingSolution()

public fun strictOnesTranscript(): String = throw PendingSolution()
