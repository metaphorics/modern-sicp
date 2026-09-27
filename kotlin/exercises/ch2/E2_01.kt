// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.1

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.1: define a better version of `makeRat` that handles both
 * positive and negative arguments. `makeRat` should normalize the sign so
 * that if the rational number is positive, both the numerator and the
 * denominator are positive, and if the rational number is negative, only
 * the numerator is negative. The statement lives in the section 2.1
 * chapter text.
 *
 * The scaffold returns `makeRat(-3, -9)`, reduced and sign-normalized.
 */
public data class Rational(
    val numer: Long,
    val denom: Long,
)

public fun ex_2_01(): Rational = throw PendingSolution()
