// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.43

package sicp.ch1.exercises

// compose is exercise 1.42's public procedure, reused here from the same package.

/** Generic over `T`, so exercise 1.44 can reuse the same shape with `T = (Double) -> Double`. */
public fun <T> repeated(
    f: (T) -> T,
    n: Long,
): (T) -> T = if (n == 1L) f else compose(f, repeated(f, n - 1L))

private fun square(x: Double): Double = x * x

public fun ex_1_43(): Double = repeated(::square, 2L)(5.0)
