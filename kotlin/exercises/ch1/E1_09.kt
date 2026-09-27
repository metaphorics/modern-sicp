// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.9

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.9, adapted for a host with no blanket tail-call guarantee:
 * each of two procedures adds two positive integers in terms of `inc`
 * (add one) and `dec` (subtract one); Kotlin's `Long.inc()` and
 * `Long.dec()` give exactly this contract. Using the substitution model,
 * trace the process each generates in evaluating `plusRecursive(4, 5)`
 * and `plusIterative(4, 5)`. Which self-call, if either, can the compiler
 * verify as `tailrec`? The statement lives in the section 1.2 chapter text.
 *
 * The scaffold returns the pair (plusRecursive(4, 5), plusIterative(4, 5)).
 */
public fun ex_1_09(): Pair<Long, Long> = throw PendingSolution()
