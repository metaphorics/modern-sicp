// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.45: compiled factorial stack use.

package sicp.ch5.solutions

/** Measures the evaluator, compiled program, and hand-tailored factorial machine. */
public fun stackRatioTable(): List<String> {
    val ns = listOf(5, 10, 20)
    val interpreted = measure(recursiveFactorialSource, ns) { n -> "(factorial $n)" }
    val compiled =
        ns.map { n ->
            lastStats(runCompiledMonitored(compiled = recursiveFactorialSource, driver = "(factorial $n)"))
        }
    val special =
        ns.map { n ->
            val result = recursiveFactorialSim.run(mapOf("n" to HNum(n.toLong()), "val" to HNum(0))) as HandHalted
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
    } +
        listOf(
            "Compiler improvements: propagate register needs more precisely and open-code the recursive call/return path, as the hand controller avoids evaluator dispatch and redundant saves.",
        )
}

private fun ratio(
    numerator: Int,
    denominator: Int,
): String = "%.3f".format(java.util.Locale.ROOT, numerator.toDouble() / denominator)
