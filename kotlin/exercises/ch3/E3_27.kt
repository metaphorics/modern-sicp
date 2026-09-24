// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.27

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.27: memoization. `memoize` takes a function and returns a
 * memoized one: a local table keyed by the argument, consulted before
 * computing and filled afterward. `makeMemoFib` applies it to the
 * self-recursive Fibonacci lambda, so every recursive level consults
 * the one shared table and the exponential process of `fib` collapses
 * to a linear number of computes.
 */
public fun memoize(f: (Long) -> Long): (Long) -> Long = throw PendingSolution()

public fun makeMemoFib(): (Long) -> Long = throw PendingSolution()
