// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.22

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.22: analyzing `let`. Analysis translates the form once
// into the kernel application the evaluator already runs; execution
// runs that translation on demand. The analysis counter climbs when
// the form is analyzed, the run counter climbs per execution, and the
// probe pins the separation: one analysis serves two runs.

// Exercise 4.22: one analysis serves every execution.

/** Analysis translates once; execution runs the translation per call. */
internal val ANALYZE_LET_SOURCE: String =
    """
fun analyzeLet(name: String, init: GExpr, body: GExpr, env: GFrame): GExpr {
    val noted = gEval(GSet("analyses", GAdd(GVar("analyses"), GNum(1L))), env)
    return GApp(GLam(name, body), init)
}

fun runAnalyzed(form: GExpr, env: GFrame): GValue? {
    val noted = gEval(GSet("runs", GAdd(GVar("runs"), GNum(1L))), env)
    return gEval(form, env)
}
    """.trimIndent()

/** One analysis, two executions, each answering 7. => "7\n7\n1\n2\n" */
public fun analyzedLetTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + ANALYZE_LET_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("analyses" to GNumV(0L), "runs" to GNumV(0L)), null)
    val form = analyzeLet("a", GNum(2L), GAdd(GVar("a"), GNum(5L)), env)
    println(renderValue(runAnalyzed(form, env)))
    println(renderValue(runAnalyzed(form, env)))
    println(renderValue(gEval(GVar("analyses"), env)))
    println(renderValue(gEval(GVar("runs"), env)))
}
                """.trimIndent(),
        ),
    )
