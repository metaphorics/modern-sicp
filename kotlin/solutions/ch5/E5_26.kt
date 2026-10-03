// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.26: report the explicit-control machine's stack
// depth and verify the book's relations: the tail-call controller reuses
// the caller frame on tail returns, so iterative maximum depth is
// independent of n while total pushes stay linear.

package sicp.ch5.solutions

/** The measured stack rows, fitted formulas, and relation observations;
 *  the tail-call controller holds iterative depth constant. */
public fun iterativeFactorialMeasurements(): List<String> {
    val ns = listOf(1, 2, 3, 4, 5, 6)
    val stats = ns.map { n -> measureOne(measuredCall(iterativeFactorialSource, "factorial(${n}L)")) }
    val rows = ns.zip(stats).map { (n, s) -> renderStats("iterative factorial", n, s) }
    val depths = stats.map { it.depth }
    val pushes = stats.map { it.pushes }
    val depthFit = fitLinear(ns, depths)
    val pushFit = fitLinear(ns, pushes)
    val independent = depths.distinct().size == 1
    val linear = pushFit != null && holdsLinear(ns, pushes, pushFit.first, pushFit.second)
    return rows +
        listOf(
            "maximum depth fitted: ${depthFit?.first ?: 0}n + ${depthFit?.second ?: 0}",
            "total pushes fitted: ${pushFit?.first ?: 0}n + ${pushFit?.second ?: 0}",
            "maximum depth independent of n: $independent",
            "total pushes linear in n: $linear",
        )
}
