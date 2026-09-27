// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.43a

package sicp.ch1.exercises

// compose is exercise 1.42's public procedure, reused here from the same package.

/**
 * [repeatedLog] composes by squaring: `n`-fold repetition of `f` splits into two
 * `(n / 2)`-fold repetitions composed together when `n` is even, and one `f` peeled off
 * plus an `(n - 1)`-fold repetition when `n` is odd, so the recursion depth (and the number
 * of `compose` calls) grows with `log2(n)`, the same trick `fastExpt` used for `b^n` in
 * section 1.2.4.
 */
public fun <T> repeatedLog(
    f: (T) -> T,
    n: Long,
): (T) -> T =
    when {
        n == 1L -> f
        n % 2L == 0L -> repeatedLog(f, n / 2L).let { half -> compose(half, half) }
        else -> compose(f, repeatedLog(f, n - 1L))
    }

private fun square(x: Double): Double = x * x

public fun ex_1_43a(): Double = repeatedLog(::square, 2L)(5.0)
