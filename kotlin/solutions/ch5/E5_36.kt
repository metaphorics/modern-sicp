// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.36: the compiler's operand evaluation order is
// right to left: `construct-arglist` reverses the operand codes, so the
// last operand's value initializes `argl` and each earlier operand
// conses onto it. The order is determined in that one reverse; the
// `leftToRight` configuration turns it off. The observable is a
// compiled `(list (record 1) (record 2))` with `record` bound to a
// logging primitive. The efficiency answer is measured, not argued:
// both orders emit the same number of instructions, so the code size is
// unaffected by the choice.

package sicp.ch5.solutions

import sicp.ch5.CompilerConfig
import sicp.ch5.EvaluatorFault
import sicp.ch5.ObjectPrimitive

private val orderSource: String = "(list (record 1) (record 2))"

/** Runs the compiled order program with `record` logging, and answers
 *  the log in order. */
private fun recordedOrder(cfg: CompilerConfig): List<String> {
    val log = mutableListOf<String>()
    val record: ObjectPrimitive =
        { args ->
            if (args.size != 1) raise(EvaluatorFault("record needs one argument"))
            log.add(args[0].toString())
            args[0]
        }
    runCompiled(cfg, orderSource, "", extraPrimitives = mapOf("record" to record))
    return log
}

/** The exercise's runs: the default records `2 1` (right to left), the
 *  reordered configuration records `1 2`, and the instruction counts
 *  are equal. */
public fun operandOrderRuns(): List<String> {
    val logRight = recordedOrder(CompilerConfig())
    val logLeft = recordedOrder(CompilerConfig(leftToRight = true))
    val countRight = compileCounts(CompilerConfig(), orderSource).first
    val countLeft = compileCounts(CompilerConfig(leftToRight = true), orderSource).first
    return listOf(
        "default order: ${logRight.joinToString(" ")}",
        "left-to-right order: ${logLeft.joinToString(" ")}",
        "instruction counts: $countRight = $countLeft: ${countRight == countLeft}",
    )
}
