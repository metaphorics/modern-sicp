// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.3: the square-root machine of 1.1.7 twice. Stage
// one assumes `good-enough?` and `improve` are available as primitives;
// stage two expands both in terms of the arithmetic operations, keeping
// only `abs` primitive (the book counts it among the arithmetic of 1.1.7).
// Each controller ends with `(perform (op print) (reg guess))` so the
// design's answer becomes one printed line at `sqrt-done`.

package sicp.ch5.solutions

import kotlin.math.abs

private fun asReal(value: HandVal): Double =
    when (value) {
        is HReal -> value.value
        else -> error("the sqrt machine works on reals, found ${render(value)}")
    }

/** The Newton test of 1.1.7 as a primitive: is the guess good enough for
 *  the 0.001 tolerance? */
private fun goodEnough(
    guess: HandVal,
    x: HandVal,
): HandVal {
    val g = asReal(guess)
    val v = asReal(x)
    return HBool(abs(g * g - v) < 0.001)
}

/** The Newton improvement step of 1.1.7 as a primitive. */
private fun improve(
    guess: HandVal,
    x: HandVal,
): HandVal {
    val g = asReal(guess)
    val v = asReal(x)
    return HReal((g + v / g) / 2.0)
}

/** Stage one: two registers, `guess` (started at 1.0) and `x`; the two
 *  primitives sit in the operations table. */
public val sqrtStageOneController: List<HandInstruction> =
    listOf(
        HLabelDef("sqrt-loop"),
        HTest("good-enough?", listOf(argReg("guess"), argReg("x"))),
        HBranch("sqrt-done"),
        HAssignOp("guess", "improve", listOf(argReg("guess"), argReg("x"))),
        HGotoLabel("sqrt-loop"),
        HLabelDef("sqrt-done"),
        HPerform("print", listOf(argReg("guess"))),
    )

/** Stage two: `good-enough?` becomes three assigns computing
 *  `abs(guess * guess - x)` into the temporary `t` and one test against
 *  `(const 0.001)`; `improve` becomes `(x / guess) + guess` divided by
 *  `(const 2)`. An operation's inputs are registers and constants only,
 *  so the intermediates must live in `t`. */
public val sqrtStageTwoController: List<HandInstruction> =
    listOf(
        HLabelDef("sqrt-loop"),
        HAssignOp("t", "*", listOf(argReg("guess"), argReg("guess"))),
        HAssignOp("t", "-", listOf(argReg("t"), argReg("x"))),
        HAssignOp("t", "abs", listOf(argReg("t"))),
        HTest("<", listOf(argReg("t"), argReal(0.001))),
        HBranch("sqrt-done"),
        HAssignOp("t", "/", listOf(argReg("x"), argReg("guess"))),
        HAssignOp("t", "+", listOf(argReg("t"), argReg("guess"))),
        HAssignOp("guess", "/", listOf(argReg("t"), argReal(2.0))),
        HGotoLabel("sqrt-loop"),
        HLabelDef("sqrt-done"),
        HPerform("print", listOf(argReg("guess"))),
    )

/** Stage one's operations table: the shared arithmetic plus the two
 *  primitives the stage assumes. */
private val stageOneOps: Map<String, HandOp> =
    handArithOps +
        mapOf(
            "good-enough?" to handOp2 { guess, x -> goodEnough(guess, x) },
            "improve" to handOp2 { guess, x -> improve(guess, x) },
        )

/** One stage's transcript for [x]: the single printed line at `sqrt-done`. */
private fun sqrtTranscript(
    controller: List<HandInstruction>,
    ops: Map<String, HandOp>,
    x: Double,
): String {
    val out = StringBuilder()
    val sim = HandSim(controller, ops + mapOf("print" to handPrintOp(out)))
    sim.run(mapOf("guess" to HReal(1.0), "x" to HReal(x)))
    return out.toString()
}

/** Stage one on [x]: the transcript with the two primitives assumed. */
public fun sqrtStageOneTranscript(x: Double): String = sqrtTranscript(sqrtStageOneController, stageOneOps, x)

/** Stage two on [x]: the transcript with only arithmetic operations. */
public fun sqrtStageTwoTranscript(x: Double): String = sqrtTranscript(sqrtStageTwoController, handArithOps, x)

/** Stage one then stage two on x = 2 and x = 9; the two transcripts agree,
 *  which is the point of the two-stage design. */
public fun sqrtMachineTranscripts(): List<String> =
    listOf(2.0, 9.0).flatMap { x -> listOf(sqrtStageOneTranscript(x), sqrtStageTwoTranscript(x)) }
