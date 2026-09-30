// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.23

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.23: Alyssa's looping sequence analyzer. Analyzing a
// sequence must build the combination without running it; Alyssa's
// version evaluates every form during analysis, so the effects land
// at analysis time and again at every run. The text version answers
// nothing until run: the counters pin which analyzer ran what when.

// Exercise 4.23: analysis must build, never execute.

/** The text combination against Alyssa's execute-during-analysis. */
internal val SEQUENCE_SOURCE: String =
    """
fun toStatements(forms: List<GExpr>): List<GStmt> {
    var out: List<GStmt> = emptyList()
    var index = 0
    while (index < forms.size) {
        val one: List<GStmt> = listOf(GExprStmt(forms.get(index)))
        out = out + one
        index = index + 1
    }
    return out
}

fun analyzeSequenceText(forms: List<GExpr>): GExpr = GBlock(toStatements(forms))

fun analyzeSequenceAlyssa(forms: List<GExpr>, env: GFrame): GExpr {
    var index = 0
    while (index < forms.size) {
        val value = gEval(forms.get(index), env)
        index = index + 1
    }
    return GBlock(toStatements(forms))
}

fun countingForms(): List<GExpr> =
    listOf(GSet("log", GAdd(GVar("log"), GNum(10L))), GSet("log", GAdd(GVar("log"), GNum(20L))))
    """.trimIndent()

/** The text analyzer answers nothing until run. => "0\n30\n" */
public fun textSequenceTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + SEQUENCE_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("log" to GNumV(0L)), null)
    val combo = analyzeSequenceText(countingForms())
    println(renderValue(gEval(GVar("log"), env)))
    val ran = gEval(combo, env)
    println(renderValue(gEval(GVar("log"), env)))
}
                """.trimIndent(),
        ),
    )

/** Alyssa's analyzer runs the forms during analysis, then again per run.
 * => "30\n60\n" */
public fun alyssaSequenceTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + SEQUENCE_SOURCE + "\n" +
                """
fun main() {
    val env = GFrame(mutableMapOf<String, GValue>("log" to GNumV(0L)), null)
    val combo = analyzeSequenceAlyssa(countingForms(), env)
    println(renderValue(gEval(GVar("log"), env)))
    val ran = gEval(combo, env)
    println(renderValue(gEval(GVar("log"), env)))
}
                """.trimIndent(),
        ),
    )
