// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme program sqrt in SICP section 1.1.7
//
// Chapter 5, exercise 5.3: the square-root machine of 1.1.7 twice. Stage
// one assumes `good-enough?` and `improve` are available as primitives;
// stage two expands both in terms of the arithmetic operations, keeping
// only `abs` primitive (the book counts it among the arithmetic of 1.1.7).
// Each controller ends with `(perform (op print) (reg guess))` so the
// design's answer becomes one printed line at `sqrt-done`.

package sicp.ch5.solutions

import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.NO_POSITION
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.MachineOp
import sicp.runtime.Perform
import sicp.runtime.Stmt
import sicp.runtime.Test

private fun asReal(value: GValue): Double =
    when (value) {
        is GValue.VDouble -> value.value
        is GValue.VLong -> value.value.toDouble()
        else -> error("the sqrt machine works on reals, found ${render(value)}")
    }

/** The Newton test of 1.1.7 as a primitive: is the guess good enough for
 *  the 0.001 tolerance? */
private val goodEnoughDevice: MachineOp =
    { args ->
        if (args.size != 2) raise(GuestError.UnassignedRead(NO_POSITION))
        val guess = asReal(args[0])
        GValue.VBool(kotlin.math.abs(guess * guess - asReal(args[1])) < 0.001)
    }

/** The Newton improvement step of 1.1.7 as a primitive. */
private val improveDevice: MachineOp =
    { args ->
        if (args.size != 2) raise(GuestError.UnassignedRead(NO_POSITION))
        val guess = asReal(args[0])
        GValue.VDouble((guess + asReal(args[1]) / guess) / 2.0)
    }

/** Stage one: two registers, `guess` (started at 1.0) and `x`; the two
 *  primitives sit in the operations table. */
public val sqrtStageOneController: List<Stmt> =
    listOf(
        Label("sqrt-loop"),
        Test(opCond("good-enough?", reg("guess"), reg("x"))),
        Branch("sqrt-done"),
        Assign("guess", opSrc("improve", reg("guess"), reg("x"))),
        Goto(GotoTarget.Lbl("sqrt-loop")),
        Label("sqrt-done"),
        Perform(opAct("print", reg("guess"))),
    )

/** Stage two: `good-enough?` becomes three assigns computing
 *  `abs(guess * guess - x)` into the temporary `t` and one test against
 *  `(const 0.001)`; `improve` becomes `(x / guess) + guess` divided by
 *  `(const 2)`. An operation's inputs are registers and constants only,
 *  so the intermediates must live in `t`. */
public val sqrtStageTwoController: List<Stmt> =
    listOf(
        Label("sqrt-loop"),
        Assign("t", opSrc("*", reg("guess"), reg("guess"))),
        Assign("t", opSrc("-", reg("t"), reg("x"))),
        Assign("t", opSrc("abs", reg("t"))),
        Test(opCond("<", reg("t"), constV(0.001))),
        Branch("sqrt-done"),
        Assign("t", opSrc("/", reg("x"), reg("guess"))),
        Assign("t", opSrc("+", reg("t"), reg("guess"))),
        Assign("guess", opSrc("/", reg("t"), constV(2.0))),
        Goto(GotoTarget.Lbl("sqrt-loop")),
        Label("sqrt-done"),
        Perform(opAct("print", reg("guess"))),
    )

/** Stage one's devices: the shared arithmetic plus the two primitives the
 *  stage assumes. */
private val stageOneDevices: Map<String, MachineOp> =
    machineArithmetic + mapOf("good-enough?" to goodEnoughDevice, "improve" to improveDevice)

/** One stage's transcript for [x]: the single printed line at `sqrt-done`. */
private fun sqrtTranscript(
    controller: List<Stmt>,
    registers: Set<String>,
    devices: Map<String, MachineOp>,
    x: Double,
): String {
    val out = StringBuilder()
    val machine =
        freshMachine(
            registers,
            devices + ("print" to printOp(out)),
            controller,
            mapOf("guess" to GValue.VDouble(1.0), "x" to GValue.VDouble(x)),
        )
    runToHalt(machine)
    return out.toString()
}

/** Stage one on [x]: the transcript with the two primitives assumed. */
public fun sqrtStageOneTranscript(x: Double): String = sqrtTranscript(sqrtStageOneController, setOf("guess", "x"), stageOneDevices, x)

/** Stage two on [x]: the transcript with only arithmetic operations. */
public fun sqrtStageTwoTranscript(x: Double): String =
    sqrtTranscript(sqrtStageTwoController, setOf("guess", "x", "t"), machineArithmetic, x)

/** Stage one then stage two on x = 2 and x = 9; the two transcripts agree,
 *  which is the point of the two-stage design. */
public fun sqrtMachineTranscripts(): List<String> =
    listOf(2.0, 9.0).flatMap { x -> listOf(sqrtStageOneTranscript(x), sqrtStageTwoTranscript(x)) }
