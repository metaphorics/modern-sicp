// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.46: compiled Fibonacci stack use.

package sicp.ch5.solutions

/** Measures the evaluator, compiled program, and Figure 5.12 machine. */
public fun fibStackRatioTable(): List<String> {
    val ns = listOf(5, 8, 10)
    val interpreted = measure(fibSource, ns) { n -> "(fib $n)" }
    val compiled =
        ns.map { n ->
            lastStats(runCompiledMonitored(compiled = fibSource, driver = "(fib $n)"))
        }
    val special =
        ns.map { n ->
            val result = fibSim.run(mapOf("n" to HNum(n.toLong()), "val" to HNum(0))) as HandHalted
            Stats(result.saves, result.maxDepth)
        }
    return ns.indices.map { i ->
        val n = ns[i]
        val interpretedStats = interpreted[i]
        val compiledStats = compiled[i]
        val specialStats = special[i]
        "n=$n: interpreted pushes=${interpretedStats.pushes} depth=${interpretedStats.depth}; " +
            "compiled pushes=${compiledStats.pushes} depth=${compiledStats.depth}; " +
            "special pushes=${specialStats.pushes} depth=${specialStats.depth}; " +
            "compiled/interpreted=${ratio(compiledStats.pushes, interpretedStats.pushes)}/" +
            "${ratio(compiledStats.depth, interpretedStats.depth)}; " +
            "special/interpreted=${ratio(specialStats.pushes, interpretedStats.pushes)}/" +
            ratio(specialStats.depth, interpretedStats.depth)
    }
}

private fun ratio(
    numerator: Int,
    denominator: Int,
): String = "%.3f".format(java.util.Locale.ROOT, numerator.toDouble() / denominator)
