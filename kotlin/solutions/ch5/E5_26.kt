// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.26: the monitored stack explores the evaluator's
// tail-recursion property with the iterative factorial of 1.2.1. Every
// measurement is one evaluator run on the monitored driver of 5.4.4, the
// driver initializing the stack once per interaction, so each call's
// counters are its own. The maximum depth comes out the same for every n
// (part a's answer: the process is iterative, and the controller keeps
// nothing on the stack across the recursive tail call), and the total
// pushes fit the linear formula the exercise asks for, with the fitted
// constants verified on every measured point (part b).

package sicp.ch5.solutions

/** The exercise's measurements for n = 1 to 6: the table, then the two
 *  answers, each number machine-run output. */
public fun iterativeFactorialMeasurements(): List<String> {
    val ns = listOf(1, 2, 3, 4, 5, 6)
    val stats = measure(iterativeFactorialSource, ns) { n -> "(factorial $n)" }
    val depths = stats.map { it.depth }
    val pushes = stats.map { it.pushes }
    val depthConstant = depths.distinct().singleOrNull()
    val table = ns.zip(stats) { n, s -> renderStats("iterative factorial", n, s) }
    val answers =
        listOf(
            "maximum depth: $depthConstant, independent of n = ${depthConstant != null}",
            linearFitLine("total pushes", ns, pushes),
        )
    return table + answers
}
