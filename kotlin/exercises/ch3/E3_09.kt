// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.9

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.9: two functions for computing factorials, a recursive
 * version and an iterative one built on `factIter`. Draw the environment
 * structures created by evaluating `factorial(6)` with each version: the
 * recursive one has a stack of frames, one per pending multiplication,
 * each holding its own `n`; the iterative one carries its state in
 * parameters, and `tailrec` (in the solution) makes the compiler rewrite
 * the self-call as a loop that reuses one frame.
 */
public fun factorial(n: Long): Long = throw PendingSolution()

public fun factIter(
    product: Long,
    counter: Long,
    maxCount: Long,
): Long = throw PendingSolution()

public fun factorialIter(n: Long): Long = throw PendingSolution()
