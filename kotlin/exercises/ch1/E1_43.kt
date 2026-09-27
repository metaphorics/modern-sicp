// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.43

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.43: if `f` is a numerical function and `n` is a positive
 * integer, the `n`-th repeated application of `f` is the function whose
 * value at `x` is `f(f(...(f(x))...))`. Write a procedure `repeated`
 * that takes a procedure computing `f` and a positive integer `n` and
 * returns the procedure computing the `n`-th repeated application of
 * `f`, so `repeated(::square, 2)(5)` is 625. Hint: `compose` from
 * exercise 1.42 may help. The statement lives in the section 1.3
 * chapter text.
 *
 * The scaffold returns `repeated(square, 2)(5)`.
 */
public fun ex_1_43(): Double = throw PendingSolution()
