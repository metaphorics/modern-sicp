// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.37

package sicp.ch1.exercises

import kotlin.math.abs

/** (a): the recursive process, one call per term from the innermost `D_k` outward. */
public fun contFrac(
    n: (Long) -> Double,
    d: (Long) -> Double,
    k: Long,
): Double {
    fun term(i: Long): Double = if (i > k) 0.0 else n(i) / (d(i) + term(i + 1L))
    return term(1L)
}

/** (b): the iterative process, folding from the innermost term (index `k`) back to the first. */
public fun contFracIterative(
    n: (Long) -> Double,
    d: (Long) -> Double,
    k: Long,
): Double {
    tailrec fun iter(
        i: Long,
        result: Double,
    ): Double = if (i == 0L) result else iter(i - 1L, n(i) / (d(i) + result))
    return iter(k, 0.0)
}

private const val ONE_OVER_PHI = 0.6180339887498948

private fun smallestAccurateK(): Long {
    var k = 1L
    while (abs(contFrac({ 1.0 }, { 1.0 }, k) - ONE_OVER_PHI) >= 0.0001) k += 1L
    return k
}

public fun ex_1_37(): Triple<Long, Double, Double> {
    val k = smallestAccurateK()
    return Triple(k, contFrac({ 1.0 }, { 1.0 }, k), contFracIterative({ 1.0 }, { 1.0 }, k))
}
