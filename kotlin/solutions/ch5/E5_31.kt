// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.31: which of the compiler's saves are
// superfluous for four combinations. The compiler answers by
// construction: each compilation emits the saves its register analysis
// demands, and the analysis here reads them back off the emitted
// statements -- a `save`/`restore` pair whose register survives the pair
// intact is exactly the superfluous save the exercise asks about. The
// combinations are bare calls of the section's two procedures `f` and
// `g`, written in guest source and compiled through the section's own
// compiler.

package sicp.ch5.solutions

import sicp.ch5.Compiler

/** One probe program: the combination as `probe`'s body beside the
 *  procedures it names. */
private fun probeSource(
    procedures: String,
    body: String,
): String =
    """
    $procedures

    fun probe(): String {
        return $body
    }

    fun main() {
        println(probe())
    }
    """.trimIndent()

/** The four combinations of the exercise, in order. */
private val combinations: List<Pair<String, String>> =
    listOf(
        "(a)" to
            probeSource(
                """
                fun f(first: String, second: String): String {
                    return first
                }
                """.trimIndent(),
                """f("x", "y")""",
            ),
        "(b)" to
            probeSource(
                """
                fun f(): (String, String) -> String {
                    return { first: String, second: String -> first }
                }
                """.trimIndent(),
                """f()("x", "y")""",
            ),
        "(c)" to
            probeSource(
                """
                val y: String = "y"

                fun g(value: String): String {
                    return value
                }

                fun f(first: String, second: String): String {
                    return first
                }
                """.trimIndent(),
                """f(g("x"), y)""",
            ),
        "(d)" to
            probeSource(
                """
                fun g(value: String): String {
                    return value
                }

                fun f(first: String, second: String): String {
                    return first
                }
                """.trimIndent(),
                """f(g("x"), "y")""",
            ),
    )

/** The report: one line per combination, naming the saves the analysis
 *  found superfluous (the saved register survives the pair), beside the
 *  structural verdict every well-formed compilation carries and the
 *  agreement of each compiled run with its direct run. */
public fun superfluousSaves(): List<String> {
    val report = mutableListOf<String>()
    var paired = true
    var agrees = true
    for ((name, source) in combinations) {
        val stmts = compiledStatements(source)
        val pairs = savePairs(stmts)
        paired = paired && pairs.none { it.restoreIndex <= it.saveIndex }
        val superfluous = pairs.filterNot { it.clobbered }.map { it.register }
        val checked = admitProgram(source)
        val direct = outputLines(sicp.ch4.Direct.run(checked))
        val compiled = outputLines(Compiler.compileAndRun(checked))
        agrees = agrees && direct == compiled
        report.add("$name saves ${pairs.size}, superfluous: ${superfluous.joinToString(" ").ifEmpty { "none" }}")
    }
    report.add("every save pairs with one restore: $paired")
    report.add("compiled and direct runs agree: $agrees")
    return report
}
