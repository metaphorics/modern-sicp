// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.27: the recursive factorial on the monitored
// stack, for comparison with 5.26. Both counters are linear in n, and the
// measured constants fill the book's table; the n = 5 row, 144 pushes at
// depth 28, is the very session the 5.4.4 prose quotes.

package sicp.ch5.solutions

/** The exercise's measurements for n = 1 to 6: the table, then the two
 *  fitted formulas with their per-point verification. */
public fun recursiveFactorialMeasurements(): List<String> {
    val ns = listOf(1, 2, 3, 4, 5, 6)
    val stats = measure(recursiveFactorialSource, ns) { n -> "(factorial $n)" }
    val depths = stats.map { it.depth }
    val pushes = stats.map { it.pushes }
    val table = ns.zip(stats) { n, s -> renderStats("recursive factorial", n, s) }
    val formulas =
        listOf(
            linearFitLine("maximum depth", ns, depths),
            linearFitLine("total pushes", ns, pushes),
        )
    return table + formulas
}
