// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.29: monitoring the tree-recursive Fibonacci
// computation. The relations the exercise concludes with are that the
// maximum depth is linear in n, that the total pushes S(n) obey the
// recurrence S(n) = S(n-1) + S(n-2) + k with a constant k, and that the
// recurrence's closed form holds on every measured point. The fitted
// constants are measurement output of the shipped controller; the
// pinned observables are the three relations.

package sicp.ch5.solutions

/** Whether `S(n) = S(n-1) + S(n-2) + k` holds with one constant k over
 *  every measured point from the third on. */
private fun recurrenceHolds(
    pushes: List<Int>,
    k: Int,
): Boolean = (2 until pushes.size).all { index -> pushes[index] == pushes[index - 1] + pushes[index - 2] + k }

/** The measurements, the fitted formulas as evidence, and the three
 *  relation verdicts. */
public fun fibonacciStackMeasurements(): List<String> {
    val ns = (2..9).toList()
    val stats = ns.map { n -> measureOne(measuredCall(fibSource, "fib(${n}L)")) }
    val rows = ns.zip(stats).map { (n, s) -> renderStats("fib", n, s) }
    val depths = stats.map { it.depth }
    val pushes = stats.map { it.pushes }
    val depthFit = fitLinear(ns, depths)
    val depthLinear = depthFit != null && holdsLinear(ns, depths, depthFit.first, depthFit.second)
    val k = pushes[2] - pushes[1] - pushes[0]
    val recurrence = recurrenceHolds(pushes, k)
    val scale = (pushes[0] + k) / fib(3)
    val closed = (2 until ns.size).all { index -> pushes[index].toLong() == scale * fib(ns[index] + 1) - k }
    return rows +
        listOf(
            "maximum depth fitted: ${depthFit?.first ?: 0}n + ${depthFit?.second ?: 0}",
            "recurrence constant fitted: $k, scale fitted: $scale",
            "maximum depth linear in n: $depthLinear",
            "pushes recurrence with constant k: $recurrence",
            "pushes closed form over Fib: $closed",
        )
}
