// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.39

package sicp.ch1.exercises

import kotlin.math.PI

// contFracIterative is exercise 1.37's public procedure, reused here from the same package.

/** Lambert's `N_1 = x`, `N_i = -x^2` for `i > 1`; `D_i = 2i - 1`. */
public fun tanCf(
    x: Double,
    k: Long,
): Double {
    fun n(i: Long): Double = if (i == 1L) x else -(x * x)

    fun d(i: Long): Double = 2.0 * i - 1.0
    return contFracIterative(::n, ::d, k)
}

public fun ex_1_39(): Double = tanCf(PI / 4.0, 10L)
