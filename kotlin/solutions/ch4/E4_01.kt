// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.1

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.1: operand evaluation order. The host fixes the order --
// operands evaluate left to right -- so the question the book leaves open
// has a definite answer here. The exercise still matters: the kernel's
// [listOfValues] is the order made explicit in code (one value per operand
// expression, stopping at the first failure), and the right-to-left
// variant shows exactly what the host had been deciding. The probe is the
// book's device: two `note` recorders that stamp a shared first/second
// cell, so the printed stamps pin which operand ran first. The opening
// pair shows the kernel's dotted rendering on the way through.

/** A recorder: stamps the first cell still holding 0, else the second. */
internal val NOTE_SOURCE: String =
    """
fun note(n: Long): GExpr =
    GIf(
        GEq(GVar("first"), GNum(0L)),
        GSet("first", GNum(n)),
        GSet("second", GNum(n)),
    )
    """.trimIndent()

/** The right-to-left operand walk: last operand first. */
internal val RIGHT_TO_LEFT_SOURCE: String =
    """
fun listOfValuesRight(operands: List<GExpr>, env: GFrame): List<GValue>? {
    var out: List<GValue> = emptyList()
    var index = operands.size - 1
    while (index >= 0) {
        val value = gEval(operands.get(index), env) ?: return null
        out = listOf(value) + out
        index = index - 1
    }
    return out
}
    """.trimIndent()

/** A fresh recorder frame: both stamps start at 0. */
internal val RECORDER_SOURCE: String =
    """
fun recorder(): GFrame = GFrame(mutableMapOf<String, GValue>("first" to GNumV(0L), "second" to GNumV(0L)), null)
    """.trimIndent()

/** Left to right: the kernel walk stamps 1 then 2.
 * => "(1 . 2)\n1\n2\n" */
public fun leftToRightTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + NOTE_SOURCE + "\n" + RECORDER_SOURCE + "\n" +
                """
fun main() {
    val env = recorder()
    println(renderValue(gEval(GConstruct("Pair", listOf(GNum(1L), GNum(2L))), env)))
    listOfValues(listOf(note(1L), note(2L)), env)
    println(renderValue(gEval(GVar("first"), env)))
    println(renderValue(gEval(GVar("second"), env)))
}
                """.trimIndent(),
        ),
    )

/** Right to left: the variant walk stamps 2 then 1.
 * => "(1 . 2)\n2\n1\n" */
public fun rightToLeftTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + NOTE_SOURCE + "\n" + RECORDER_SOURCE + "\n" + RIGHT_TO_LEFT_SOURCE + "\n" +
                """
fun main() {
    val env = recorder()
    println(renderValue(gEval(GConstruct("Pair", listOf(GNum(1L), GNum(2L))), env)))
    listOfValuesRight(listOf(note(1L), note(2L)), env)
    println(renderValue(gEval(GVar("first"), env)))
    println(renderValue(gEval(GVar("second"), env)))
}
                """.trimIndent(),
        ),
    )
