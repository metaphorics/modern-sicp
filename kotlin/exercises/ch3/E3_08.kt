// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.8

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.8: When we defined the evaluation model in 1.1.3, we said
 * that the first step in evaluating an expression is to evaluate its
 * subexpressions, but we never specified the order. Kotlin does specify
 * it: a function's arguments are evaluated strictly left to right, and
 * `a + b` desugars to `a.plus(b)`, so the receiver `a` is always
 * evaluated before the argument `b`. Because of assignment, the order in
 * which arguments are evaluated can make a difference to the result, and
 * this exercise makes that difference witness Kotlin's own guarantee
 * instead of an implementation's unspecified choice the way SICP's
 * original does for Scheme.
 *
 * Write `makeF`, returning a function `f` that closes over one mutable
 * cell such that `f(0) + f(1)` always evaluates to 0, and `f(1) + f(0)`
 * always evaluates to 1, deterministically, on every conformant Kotlin
 * implementation.
 *
 * The scaffold's `f` always returns its argument unchanged, so both sums
 * equal 1 either way and prove nothing about evaluation order.
 */
public fun makeF(): (Int) -> Int = throw PendingSolution()
