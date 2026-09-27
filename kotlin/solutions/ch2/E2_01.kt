// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.1

package sicp.ch2.exercises

public data class Rational(
    val numer: Long,
    val denom: Long,
)

private tailrec fun gcd(
    a: Long,
    b: Long,
): Long = if (b == 0L) a else gcd(b, a % b)

/** Moves the denominator's sign onto the numerator, then reduces by the gcd of the absolute values. */
public fun makeRat(
    n: Long,
    d: Long,
): Rational {
    val sign = if (d < 0L) -1L else 1L
    val g = gcd(kotlin.math.abs(n), kotlin.math.abs(d))
    return Rational(sign * n / g, sign * d / g)
}

public fun ex_2_01(): Rational = makeRat(-3L, -9L)
