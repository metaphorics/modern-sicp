// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.29: the stack of the tree-recursive Fibonacci on
// the monitored evaluator, and the two formulas. (a) The maximum depth
// grows by a fixed increment per n: the linear space the 1.2.2 argument
// needs, reported as `depth = inc * n + c` with the increment verified on
// every step. (b) The pushes satisfy the recurrence the hint asks for,
// `S(n) = S(n-1) + S(n-2) + k` with one constant k on every measured n,
// and the closed form `S(n) = a * Fib(n+1) + b`, with `a` and `b` fitted
// from the measurements and verified on every point.

package sicp.ch5.solutions

import kotlin.math.abs

/** [fib] as a closed-form oracle; the measured `a` divides the last
 *  difference exactly, so the integer fit is the real one. */
private fun closedFormLine(
    ns: List<Int>,
    pushes: List<Int>,
): String {
    val a = (pushes.last() - pushes[pushes.size - 2]) / (fib(ns.last() + 1) - fib(ns[ns.size - 2] + 1))
    val b = pushes.first() - a * fib(ns.first() + 1)
    val holds = ns.zip(pushes).all { (n, p) -> a * fib(n + 1) + b == p.toLong() }
    val sign = if (b < 0) "-" else "+"
    return "S(n) = $a * Fib(n+1) $sign ${abs(b)}, holds on every measured n: $holds"
}

/** The exercise's measurements for n = 2 to 9: the table, then the three
 *  formula lines, each constant derived from the runs. */
public fun fibonacciStackMeasurements(): List<String> {
    val ns = listOf(2, 3, 4, 5, 6, 7, 8, 9)
    val stats = measure(fibSource, ns) { n -> "(fib $n)" }
    val depths = stats.map { it.depth }
    val pushes = stats.map { it.pushes }
    val table = ns.zip(stats) { n, s -> renderStats("fib", n, s) }

    val depthSteps = depths.zipWithNext().map { (a, b) -> b - a }
    val increment = depthSteps.first()
    val depthConstant = depths.first() - increment * ns.first()
    val depthSign = if (depthConstant < 0) "-" else "+"
    val depthLine =
        "maximum depth = ${increment}n $depthSign ${abs(depthConstant)} " +
            "(every step $increment), linear: ${depthSteps.all { it == increment }}"

    val recurrenceResiduals =
        (0..pushes.size - 3).map { i -> pushes[i + 2] - pushes[i + 1] - pushes[i] }
    val k = recurrenceResiduals.first()
    val recurrenceLine =
        "S(n) = S(n-1) + S(n-2) + $k, k constant: ${recurrenceResiduals.all { it == k }}"

    return table + listOf(depthLine, recurrenceLine, closedFormLine(ns, pushes))
}
