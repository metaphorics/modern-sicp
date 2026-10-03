// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.27: for comparison with exercise 5.26, the
// recursive factorial's stack behavior on the same monitored machine.
// Both the maximum depth and the total pushes are linear in n: each
// recursive level costs the evaluator a fixed package of saves. The
// fitted constants are measurement output of the shipped controller
// (whose continuation-frame design differs from the book's 5.4.1-5.4.4
// save points); the pinned observables are the two relations, verified
// on every measured point.

package sicp.ch5.solutions

/** The measurements, the fitted formulas as evidence, and the two
 *  relation verdicts. */
public fun recursiveFactorialMeasurements(): List<String> {
    val ns = listOf(1, 2, 3, 4, 5, 6)
    val stats = ns.map { n -> measureOne(measuredCall(recursiveFactorialSource, "factorial(${n}L)")) }
    val rows = ns.zip(stats).map { (n, s) -> renderStats("recursive factorial", n, s) }
    val depths = stats.map { it.depth }
    val pushes = stats.map { it.pushes }
    val depthFit = fitLinear(ns, depths)
    val pushFit = fitLinear(ns, pushes)
    val depthLinear = depthFit != null && holdsLinear(ns, depths, depthFit.first, depthFit.second)
    val pushLinear = pushFit != null && holdsLinear(ns, pushes, pushFit.first, pushFit.second)
    return rows +
        listOf(
            "maximum depth fitted: ${depthFit?.first ?: 0}n + ${depthFit?.second ?: 0}",
            "total pushes fitted: ${pushFit?.first ?: 0}n + ${pushFit?.second ?: 0}",
            "maximum depth linear in n: $depthLinear",
            "total pushes linear in n: $pushLinear",
        )
}
