// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.15

package sicp.ch1.exercises

import kotlin.math.abs

private fun cube(x: Double): Double = x * x * x

/** Counts every application of `p` while answering part (a) of the statement. */
public fun ex_1_15(): Int {
    var pApplications = 0

    fun p(x: Double): Double {
        pApplications += 1
        return 3.0 * x - 4.0 * cube(x)
    }

    fun sine(angle: Double): Double = if (!(abs(angle) > 0.1)) angle else p(sine(angle / 3.0))
    sine(12.15)
    return pApplications
}
