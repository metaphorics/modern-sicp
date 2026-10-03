// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.24

package sicp.ch4.solutions

import sicp.ch4.Analyzed
import sicp.ch4.Direct

// Exercise 4.24: what analysis saves, measured in counters instead of
// wall-clock time. Re-analyzing before every call climbs both counters
// together; analyzing once and running three times climbs the run
// counter alone. The same analyze-once program runs under both
// engines, and both agree on every answer and counter: the lesson is
// the saved analyses, and the engines concur on them.

/** Three calls with a fresh analysis each, then three calls on one. */
internal val SAVINGS_SOURCE: String =
    """
fun analyzeCounted(name: String, init: GExpr, body: GExpr, env: GFrame): GExpr {
    val noted = gEval(GSet("analyses", GAdd(GVar("analyses"), GNum(1L))), env)
    return GApp(GLam(name, body), init)
}

fun runCounted(form: GExpr, env: GFrame): GValue? {
    val noted = gEval(GSet("runs", GAdd(GVar("runs"), GNum(1L))), env)
    return gEval(form, env)
}

fun freshCounters(): GFrame =
    GFrame(mutableMapOf<String, GValue>("analyses" to GNumV(0L), "runs" to GNumV(0L)), null)
    """.trimIndent()

/** The analyze-once program both engines run. */
internal val SAVINGS_PROGRAM: String =
    KERNEL_SOURCE + "\n" + SAVINGS_SOURCE + "\n" +
        """
fun main() {
    val env = freshCounters()
    val form = analyzeCounted("a", GNum(2L), GAdd(GVar("a"), GNum(5L)), env)
    println(renderValue(runCounted(form, env)))
    println(renderValue(runCounted(form, env)))
    println(renderValue(runCounted(form, env)))
    println(renderValue(gEval(GVar("analyses"), env)))
    println(renderValue(gEval(GVar("runs"), env)))
}
        """.trimIndent()

/** Re-analysis climbs with the runs; one analysis serves all three.
 * => "7\n7\n7\n3\n3\n7\n7\n7\n1\n3\n" */
public fun analysisSavingsTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + SAVINGS_SOURCE + "\n" +
                """
fun main() {
    val first = freshCounters()
    println(renderValue(runCounted(analyzeCounted("a", GNum(2L), GAdd(GVar("a"), GNum(5L)), first), first)))
    println(renderValue(runCounted(analyzeCounted("a", GNum(2L), GAdd(GVar("a"), GNum(5L)), first), first)))
    println(renderValue(runCounted(analyzeCounted("a", GNum(2L), GAdd(GVar("a"), GNum(5L)), first), first)))
    println(renderValue(gEval(GVar("analyses"), first)))
    println(renderValue(gEval(GVar("runs"), first)))
    val second = freshCounters()
    val form = analyzeCounted("a", GNum(2L), GAdd(GVar("a"), GNum(5L)), second)
    println(renderValue(runCounted(form, second)))
    println(renderValue(runCounted(form, second)))
    println(renderValue(runCounted(form, second)))
    println(renderValue(gEval(GVar("analyses"), second)))
    println(renderValue(gEval(GVar("runs"), second)))
}
                """.trimIndent(),
        ),
    )

/** The analyzed engine agrees on every answer and counter.
 * => "7\n7\n7\n1\n3\n" */
public fun analyzedEngineTranscript(): String = outcomeText(Analyzed.run(SAVINGS_PROGRAM))
