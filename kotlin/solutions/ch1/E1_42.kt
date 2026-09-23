// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.42

package sicp.ch1.exercises

public fun <T> compose(
    f: (T) -> T,
    g: (T) -> T,
): (T) -> T = { x -> f(g(x)) }

private fun square(x: Long): Long = x * x

private fun inc(n: Long): Long = n + 1L

public fun ex_1_42(): Long = compose(::square, ::inc)(6L)
