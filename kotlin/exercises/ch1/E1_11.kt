// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.11

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.11: a function `f` is defined by the rule that `f(n) = n` if
 * `n < 3` and `f(n) = f(n - 1) + 2 f(n - 2) + 3 f(n - 3)` if `n >= 3`.
 * Write a procedure that computes `f` by means of a recursive process, and
 * one that computes `f` by means of an iterative process. The statement
 * lives in the section 1.2 chapter text.
 *
 * The scaffold returns the pair (recursive f(n), iterative f(n)).
 */
public fun ex_1_11(n: Long): Pair<Long, Long> = throw PendingSolution()
